//! 🎚️ Persisted local state for one exact BIM section window: which section or elevation view it shows and its navigation. The plane and the depth are authored in the view.

use crate::editor::bim::kit::window_config;

window_config! {
    window: super::WINDOW_KIND_ID,
    schema: "bim.section-window.config",
    envelope: "s.bim.model.section-window.config",
    extension: "bimsectionwindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️section/🎚️config",
    display: "Set BIM Section Window Configuration",
    type BimSectionWindowConfig, BimSectionWindowConfigDiff, BimSectionWindowConfigMutation, BimSectionWindowConfigOwner;
    view: String = String::new();
    framed: bool = false;
    selected_options: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    workset_visibility: std::collections::BTreeMap<String, bool> = std::collections::BTreeMap::new();
    #[dsl(block)]
    viewport: store::Viewport2d = store::Viewport2d { x: 5.0, y: -1.5, zoom: 40.0 };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
