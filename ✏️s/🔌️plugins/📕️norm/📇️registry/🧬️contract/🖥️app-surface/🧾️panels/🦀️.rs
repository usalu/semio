//! 🧾️ App-surface panels: the checks window, the properties inspector, the outliner, the selection mirror, the catalogue and inspection panels and the viewport toolbar with its lane overlays, layered over the foundation, viewport, visual-chrome and live-core crates.

#![allow(async_fn_in_trait, unused_imports)]

include!("../📋️prelude/🦀️.rs");

pub use semio_s_plugin_norm_app_surface_foundation::*;
pub use semio_s_plugin_norm_app_surface_live_core::*;
pub use semio_s_plugin_norm_app_surface_visual_chrome::*;
pub use semio_s_plugin_norm_app_surface_viewport3d as viewport3d;
pub use semio_s_plugin_norm_app_surface_panels_inspection::*;

#[path = "../📚️catalogue-and-inspection/🦀️.rs"]
mod catalogue_and_inspection;
pub use catalogue_and_inspection::*;

#[path = "../🗂️design-case-surface/🦀️.rs"]
mod design_case_surface;
pub use design_case_surface::*;

#[path = "../📡️node-contract/🦀️.rs"]
mod node_contract;
pub use node_contract::*;

#[path = "../🧰️entity-editing/🖐️touch-menu/🦀️.rs"]
mod touch_menu_window;
pub use touch_menu_window::*;

#[path = "../🪟️visual-windows/🧊️viewport3d/🫧️lane-overlays/🦀️.rs"]
pub mod lane_overlays;

#[path = "../🪟️visual-windows/🧊️viewport3d/🧰️toolbar/🦀️.rs"]
pub mod toolbar;

#[cfg(test)]
#[path = "../🧪️tests/🔬️viewport-panels/🦀️.rs"]
mod viewport_panel_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️inspection/🦀️.rs"]
mod inspection_tests;
