//! 🎚️ Persisted local state for one exact BIM world window: its orbit camera, projection preset, storey visibility, phase filter (`all`, `existing`, `new`, `demolished`, `temporary`; empty means all) and the section plane.

use crate::{window_config, OptionSelection, WorksetVisibility};

pub const WINDOW_KIND_ID: &str = "bim-edit-world";

/// 🎥️ The camera of a window that has not framed its content yet.
pub const INITIAL_CAMERA: store::Viewport3dOrbit = store::Viewport3dOrbit { position: [14.0, -14.0, 10.0], target: [4.0, 3.0, 1.5], zoom: 1.0, up: None };

window_config! {
    window: WINDOW_KIND_ID,
    schema: "bim.world-window.config",
    envelope: "s.bim.model.world-window.config",
    extension: "bimworldwindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️world/🎚️config",
    display: "Set BIM World Window Configuration",
    type BimWorldWindowConfig, BimWorldWindowConfigDiff, BimWorldWindowConfigMutation, BimWorldWindowConfigOwner;
    #[dsl(block)]
    camera: store::Viewport3dOrbit = INITIAL_CAMERA;
    #[dsl(block)]
    projection: semio_plugin::WorldProjectionConfig = semio_plugin::WorldProjectionConfig::default();
    isolated_storey: String = String::new();
    hidden_storeys: Vec<String> = Vec::new();
    isolated_elements: Vec<String> = Vec::new();
    section_box: Vec<f64> = Vec::new();
    view_phase: String = String::new();
    section_enabled: bool = false;
    section_axis: String = "z".to_string();
    section_offset: f64 = 1.2;
    framed: bool = false;
    selected_options: Vec<OptionSelection> = Vec::new();
    workset_visibility: Vec<WorksetVisibility> = Vec::new();
    energy_overlay: bool = false;
    structural_overlay: bool = false;
    energy_mode: String = "u_value".to_string();
}
