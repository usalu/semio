//! 🧲️ App-surface live core: the live lane payloads and runtime, direct manipulation, the edit rules and the live command dispatch, layered over the foundation crate.

#![allow(async_fn_in_trait, unused_imports)]

include!("../📋️prelude/🦀️.rs");

pub use semio_s_plugin_norm_app_surface_foundation::*;

#[path = "../🫧️live-lanes/🦀️.rs"]
mod live_lanes;
pub use live_lanes::*;

#[path = "../✏️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::*;

#[path = "../🕹️direct-manipulation/🦀️.rs"]
pub mod direct_manipulation;

#[path = "../⌨️command-args-and-dispatch/🦀️.rs"]
mod command_args_and_dispatch;
pub use command_args_and_dispatch::*;
