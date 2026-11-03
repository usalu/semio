//! 🔎️ App-surface inspection panels: the checks window, the properties inspector, the outliner, the selection mirror, the catalogue and inspection panels, the check parameters, bulk edit, host routes, view steps, shell hooks and shortcut dispatch; the panels crate re-exports them.

#![allow(async_fn_in_trait, unused_imports)]

include!("../../📋️prelude/🦀️.rs");

pub use semio_s_plugin_norm_app_surface_foundation::*;
pub use semio_s_plugin_norm_app_surface_live_core::*;
pub use semio_s_plugin_norm_app_surface_visual_chrome::*;
pub use semio_s_plugin_norm_app_surface_viewport3d as viewport3d;

#[path = "../../🪟️visual-windows/✅️checks/🦀️.rs"]
mod checks_window;
pub use checks_window::*;

#[path = "../../🌲️outliner/🦀️.rs"]
mod outliner;
pub use outliner::*;

#[path = "../../🪄️selection-mirror/🦀️.rs"]
mod selection_mirror;
pub use selection_mirror::*;

#[path = "../../🪟️visual-windows/🧾️properties/🦀️.rs"]
mod properties;
pub use properties::*;

#[path = "../../⌨️shortcut-dispatch/🦀️.rs"]
mod shortcut_dispatch;
pub use shortcut_dispatch::*;

#[path = "../../🏳️check-parameters/🦀️.rs"]
mod check_parameters;
pub use check_parameters::*;

#[path = "../../🎹️view-steps/🦀️.rs"]
mod view_steps;
pub use view_steps::*;

#[path = "../../🔗️check-entity-link/🦀️.rs"]
mod check_entity_link;
pub use check_entity_link::*;

#[path = "../../🧰️entity-editing/🧭️host-routes/🦀️.rs"]
mod host_routes;
pub use host_routes::*;

#[path = "../../🧰️entity-editing/🍽️context-menu/🦀️.rs"]
mod context_menu;
pub use context_menu::*;

#[path = "../../🧰️entity-editing/🧭️interaction-topology/🦀️.rs"]
mod interaction_topology;
pub use interaction_topology::*;

#[path = "../../🧰️entity-editing/🗃️bulk-edit/🦀️.rs"]
mod bulk_edit;
pub use bulk_edit::*;

#[path = "../../🔌️shell-hooks/🦀️.rs"]
mod shell_hooks;
pub use shell_hooks::*;
