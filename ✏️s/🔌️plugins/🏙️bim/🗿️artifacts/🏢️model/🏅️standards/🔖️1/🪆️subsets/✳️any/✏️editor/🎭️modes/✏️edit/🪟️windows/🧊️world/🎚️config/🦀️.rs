//! 🎚️ Persisted local state for one exact BIM world window: its orbit camera, projection preset, storey visibility and the section plane.

use crate::editor::bim::kit::window_config;

window_config! {
    window: super::WINDOW_KIND_ID,
    schema: "bim.world-window.config",
    envelope: "s.bim.model.world-window.config",
    extension: "bimworldwindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️world/🎚️config",
    display: "Set BIM World Window Configuration",
    type BimWorldWindowConfig, BimWorldWindowConfigDiff, BimWorldWindowConfigMutation, BimWorldWindowConfigOwner;
    #[dsl(block)]
    camera: store::Viewport3dOrbit = super::INITIAL_CAMERA;
    #[dsl(block)]
    projection: semio_framework_plugin::WorldProjectionConfig = semio_framework_plugin::WorldProjectionConfig::default();
    isolated_storey: String = String::new();
    hidden_storeys: Vec<String> = Vec::new();
    section_enabled: bool = false;
    section_axis: String = "z".to_string();
    section_offset: f64 = 1.2;
    framed: bool = false;
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
