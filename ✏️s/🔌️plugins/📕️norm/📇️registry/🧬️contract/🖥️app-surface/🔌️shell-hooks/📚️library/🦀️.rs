//! 🔌️ Host shell integration: keyboard routes, menu visibility and device hooks.

#![allow(async_fn_in_trait, unused_imports)]

include!("../../📋️prelude/🦀️.rs");

pub use semio_s_plugin_norm_app_surface_foundation::*;
pub use semio_s_plugin_norm_app_surface_live_core::*;
pub use semio_s_plugin_norm_app_surface_visual_chrome::*;
pub use semio_s_plugin_norm_app_surface_viewport3d as viewport3d;
pub use semio_s_plugin_norm_app_surface_interaction_topology::*;

#[path = "../🦀️.rs"]
mod shell_hooks;
pub use shell_hooks::*;

#[path = "../../🧰️entity-editing/🧭️host-routes/🦀️.rs"]
mod host_routes;
pub use host_routes::*;

#[path = "../../⌨️shortcut-dispatch/🦀️.rs"]
mod shortcut_dispatch;
pub use shortcut_dispatch::*;

#[path = "../🎹️view-actions/🦀️.rs"]
mod view_actions;
pub use view_actions::*;

#[path = "../👁️menu-visibility/🦀️.rs"]
mod menu_visibility;
pub use menu_visibility::*;
