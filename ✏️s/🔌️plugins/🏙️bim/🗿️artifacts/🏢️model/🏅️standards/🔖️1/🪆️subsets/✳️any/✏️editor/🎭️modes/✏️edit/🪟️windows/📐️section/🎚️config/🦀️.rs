//! 🎚️ Persisted local state for one exact BIM section window: the section line in plan coordinates, how deep it looks and its navigation.

use crate::editor::bim::kit::window_config;

window_config! {
    window: super::WINDOW_KIND_ID,
    schema: "bim.section-window.config",
    envelope: "s.bim.model.section-window.config",
    extension: "bimsectionwindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️section/🎚️config",
    display: "Set BIM Section Window Configuration",
    type BimSectionWindowConfig, BimSectionWindowConfigMutation, BimSectionWindowConfigOwner;
    start_x: f64 = 0.0;
    start_y: f64 = 0.0;
    end_x: f64 = 10.0;
    end_y: f64 = 0.0;
    depth: f64 = 5.0;
    framed: bool = false;
    #[dsl(block)]
    viewport: store::Viewport2d = store::Viewport2d { x: 5.0, y: -1.5, zoom: 40.0 };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
