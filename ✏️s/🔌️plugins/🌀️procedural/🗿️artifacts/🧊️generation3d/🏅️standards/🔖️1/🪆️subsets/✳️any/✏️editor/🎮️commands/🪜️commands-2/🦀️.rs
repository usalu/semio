//! 🪜️ Generation3d editor commands: the command payloads not split into their own crates, and the mesh-selection edits (part 2).
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../✅️flow-eval-resolve/🦀️.rs"]
pub mod flow_eval_resolve;
#[path = "../🔓️flow-eval-release/🦀️.rs"]
pub mod flow_eval_release;
#[path = "../📂️import-document-request/🦀️.rs"]
pub mod import_document_request;
#[path = "../🩹️patch-flow-widgets/🦀️.rs"]
pub mod patch_flow_widgets;
#[path = "../➖️remove-widget/🦀️.rs"]
pub mod remove_widget;
#[path = "../🎨️set-active-example/🦀️.rs"]
pub mod set_active_example;
#[path = "../🔁️cycle-lod-mode/🦀️.rs"]
pub mod cycle_lod_mode;
#[path = "../🔬️set-lod-mode/🦀️.rs"]
pub mod set_lod_mode;
#[path = "../🧭️set-sun-azimuth/🦀️.rs"]
pub mod set_sun_azimuth;
#[path = "../🔆️set-sun-intensity/🦀️.rs"]
pub mod set_sun_intensity;
#[path = "../↔️translate-selection/🦀️.rs"]
pub mod translate_selection;
#[path = "../🥽️edit-mesh-selection/🦀️.rs"]
pub mod edit_mesh_selection;
#[path = "../🔪️knife-mesh-selection/🦀️.rs"]
pub mod knife_mesh_selection;
