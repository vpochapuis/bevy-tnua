//! # Tnua - A Character Controller for Bevy.
//!
//! Tnua ("motion" in Hebrew) is a floating character controller, which means that instead of
//! constantly touching the ground the character floats above it, which makes many aspects of the
//! motion control simpler.
//!
//! Tnua can use [Rapier](https://rapier.rs/) or [XPBD](https://github.com/Jondolf/bevy_xpbd), and
//! supports both the 2D and 3D versions of both with integration crates:
//!
//! * For Rapier 2D, add the [bevy-tnua-rapier2d](https://crates.io/crates/bevy-tnua-rapier2d) crate.
//! * For Rapier 3D, add the [bevy-tnua-rapier3d](https://crates.io/crates/bevy-tnua-rapier3d) crate.
//! * For XPBD 2D, add the [bevy-tnua-xpbd2d](https://crates.io/crates/bevy-tnua-xpbd2d) crate.
//! * For XPBD 3D, add the [bevy-tnua-xpbd3d](https://crates.io/crates/bevy-tnua-xpbd3d) crate.
//! * Third party integration crates. Such crates should depend on
//!   [bevy-tnua-physics-integration-layer](https://crates.io/crates/bevy-tnua-physics-integration-layer)
//!   and not the main bevy-tnua crate.
//!
//! Each physics integration crate has basic usage instructions for adding it in its documentation.
//!
//! In addition to the physics integration plugin, the
//! [`TnuaControllerPlugin`](prelude::TnuaControllerPlugin) should also be added.
//!
//! A Tnua controlled character must have a dynamic rigid body, everything from
//! `Tnua<physics-backend>IOBundle` (e.g. - for Rapier 3D, use `TnuaRapier3dIOBundle1), and
//! everything from [`TnuaControllerBundle`](prelude::TnuaControllerBundle):
//! ```no_run
//! # use bevy::prelude::*;
//! # // Not importing from Rapier because there are two versions and the default features does not
//! # // enable either:
//! # type TnuaRapier3dIOBundle = ();
//! # #[derive(Component)]
//! # enum RigidBody { Dynamic }
//! # use bevy_tnua::prelude::*;
//! # let mut commands: Commands = panic!();
//! # let mut cmd = commands.spawn_empty();
//! cmd.insert(RigidBody::Dynamic);
//! cmd.insert(TnuaRapier3dIOBundle::default()); // this one depends on the physics backend
//! cmd.insert(TnuaControllerBundle::default());
//! ```
//! Typically though it'd also include a `Collider`.
//!
//! ## Optional But Recommended
//!
//! * Tnua, by default, casts a single ray to the ground. This can be a problem when the character
//!   stands on a ledge, because the ray may be past the ledge while the character's collider
//!   isn't. To avoid that, use `Tnua<physics-backend>SensorShape` (e.g. - for Rapier 3D, use
//!   `TnuaRapier3dSensorShape`) to replace the ray with a shape that resembles the collider. It is
//!   better to use a shape a little bit smaller than the collider, so that when the character
//!   presses against a wall Tnua won't think it should be lifted up when the casted shape hits
//!   that wall.
//! * Tnua will apply forces to keep the character upright, but it is also possible to lock
//!   rotation so that there would be no tilting at all. This is done by Tnua itself - it has to be
//!   done by the physics engine. Both Rapier and XPBD can do it using a component called
//!   `LockedAxes`. When using it in 3D in combination of rotation controls (such as
//!   [`TnuaBuiltinWalk::desired_forward`](builtins::TnuaBuiltinWalk::desired_forward)) make sure
//!   to only lock the X and Z axess, so that Tnua could rotate the character around the Y axis.
//!
//! ## Controlling the Character
//!
//! To control the character, update the [`TnuaController`](prelude::TnuaController) (added via tha
//! [`TnuaControllerBundle`](prelude::TnuaControllerBundle)) by feeding it a [basis](TnuaBasis) and
//! zero or more [actions](TnuaAction). For some of the advanced features to work, the system that
//! does this needs to be placed inside the [`TnuaUserControlsSystemSet`] system set.
//!
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy_tnua::prelude::*;
//! # use bevy_tnua::math::Vector3;
//! # #[derive(Component)]
//! # struct PlayerInputComponent;
//! # impl PlayerInputComponent {
//! # fn direction_vector(&self) -> Vector3 { Vector3::ZERO }
//! # fn jump_pressed(&self) -> bool { false }
//! # }
//! fn player_control_system(mut query: Query<(
//!     &mut TnuaController,
//!     &PlayerInputComponent,  // not part of Tnua - defined in user code
//! )>) {
//!     for (mut controller, player_input) in query.iter_mut() {
//!         controller.basis(TnuaBuiltinWalk {
//!             // Move in the direction the player entered, at a speed of 10.0:
//!             desired_velocity: player_input.direction_vector() * 10.0,
//!
//!             // Turn the character in the movement direction:
//!             desired_forward: player_input.direction_vector(),
//!             
//!             // Must be larger than the height of the entity's center from the bottom of its
//!             // collider, or else the character will not float and Tnua will not work properly:
//!             float_height: 2.0,
//!
//!             // TnuaBuiltinWalk has many other fields that can be configured:
//!             ..Default::default()
//!         });
//!
//!         if player_input.jump_pressed() {
//!             // The jump action must be fed as long as the player holds the button.
//!             controller.action(TnuaBuiltinJump {
//!                 // The full height of the jump, if the player does not release the button:
//!                 height: 4.0,
//!
//!                 // TnuaBuiltinJump too has other fields that can be configured:
//!                 ..Default::default()
//!             });
//!         }
//!     }
//! }
//! ```
//! Refer to the documentation of [`TnuaController`](prelude::TnuaController) for more information,
//! but essentially the _basis_ controls the general movement and the _action_ is something
//! special (jump, dash, crouch, etc.)
//!
//! ## Motion Based Animation
//!
//! [`TnuaController`](crate::prelude::TnuaController) can also be used to retreive data that can
//! be used to decide which animation to play. A useful helper for that is [`TnuaAnimatingState`].
mod animating_helper;
mod basis_action_traits;
pub mod builtins;
pub mod control_helpers;
pub mod controller;
mod util;
pub use animating_helper::{TnuaAnimatingState, TnuaAnimatingStateDirective};
pub use basis_action_traits::{
    DynamicAction, DynamicBasis, TnuaAction, TnuaActionContext, TnuaActionInitiationDirective,
    TnuaActionLifecycleDirective, TnuaActionLifecycleStatus, TnuaBasis, TnuaBasisContext,
};

pub mod prelude {
    pub use crate::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};
    pub use crate::controller::{TnuaController, TnuaControllerBundle, TnuaControllerPlugin};
    pub use crate::{TnuaAction, TnuaPipelineStages, TnuaUserControlsSystemSet};
}

pub use bevy_tnua_physics_integration_layer::data_for_backends::*;
pub use bevy_tnua_physics_integration_layer::*;

use bevy::prelude::*;

/// The user controls should be applied in this system set.
#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct TnuaUserControlsSystemSet;
