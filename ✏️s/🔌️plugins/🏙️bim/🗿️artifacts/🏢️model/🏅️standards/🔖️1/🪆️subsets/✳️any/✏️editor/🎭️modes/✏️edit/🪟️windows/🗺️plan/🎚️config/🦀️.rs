//! 🎚️ Persisted local state for one exact BIM plan window: which storey it shows, its navigation and the cut height of its display.

use crate::editor::bim::kit::window_config;

window_config! {
    window: super::WINDOW_KIND_ID,
    schema: "bim.plan-window.config",
    envelope: "s.bim.model.plan-window.config",
    extension: "bimplanwindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️plan/🎚️config",
    display: "Set BIM Plan Window Configuration",
    type BimPlanWindowConfig, BimPlanWindowConfigMutation, BimPlanWindowConfigOwner;
    storey: String = String::new();
    cut_height: f64 = 1.2;
    framed: bool = false;
    #[dsl(block)]
    viewport: store::Viewport2d = store::Viewport2d { x: 0.0, y: 0.0, zoom: 40.0 };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
