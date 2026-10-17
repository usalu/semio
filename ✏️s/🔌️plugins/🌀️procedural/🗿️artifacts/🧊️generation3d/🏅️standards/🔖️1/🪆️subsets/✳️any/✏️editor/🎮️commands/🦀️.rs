//! 🎮️ Generation3d editor commands: the command payloads not split into their own crates, and the mesh-selection edits.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "➕️add-generation/🦀️.rs"]
pub mod add_generation;
#[path = "🔺️flow-tessellate-resolve/🦀️.rs"]
pub mod flow_tessellate_resolve;
#[path = "🧯️flow-tessellate-cancel-resolve/🦀️.rs"]
pub mod flow_tessellate_cancel_resolve;
#[path = "🧬️generation/🦀️.rs"]
pub mod generation;
#[path = "🔭️node-graph-viewport/🦀️.rs"]
pub mod node_graph_viewport;
#[path = "🗑️remove-generation/🦀️.rs"]
pub mod remove_generation;
#[path = "🏷️rename-generation/🦀️.rs"]
pub mod rename_generation;
#[path = "🗺️reorganize/🦀️.rs"]
pub mod reorganize;
#[path = "🎯️select-generation/🦀️.rs"]
pub mod select_generation;
#[path = "📷️set-camera/🦀️.rs"]
pub mod set_camera;
#[path = "🔁️cycle-show-mode/🦀️.rs"]
pub mod cycle_show_mode;
#[path = "👁️set-show-mode/🦀️.rs"]
pub mod set_show_mode;
#[path = "🌄️set-sun-elevation/🦀️.rs"]
pub mod set_sun_elevation;
#[path = "🌞️toggle-sun/🦀️.rs"]
pub mod toggle_sun;
#[path = "🎚️update-generation-values/🦀️.rs"]
pub mod update_generation_values;

pub use semio_s_artifact_procedural_3d_editor_commands_2::{flow_eval_resolve, flow_eval_release, import_document_request, patch_flow_widgets, remove_widget, set_active_example, cycle_lod_mode, set_lod_mode, set_sun_azimuth, set_sun_intensity, translate_selection, edit_mesh_selection, knife_mesh_selection};
