//! 🎚️ Persisted local state for one exact BIM plan window: which plan view it shows and its navigation. The storey and the cut height are authored in the view.

use crate::editor::bim::kit::path;
use crate::editor::bim::kit::window_config;

use crate::editor::bim::kit::{OptionSelection, WorksetVisibility};

window_config! {
    window: crate::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID,
    schema: "bim.plan-window.config",
    envelope: "s.bim.model.plan-window.config",
    extension: "bimplanwindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️plan/🎚️config",
    display: "Set BIM Plan Window Configuration",
    type BimPlanWindowConfig, BimPlanWindowConfigDiff, BimPlanWindowConfigMutation, BimPlanWindowConfigOwner;
    view: String = String::new();
    framed: bool = false;
    selected_options: Vec<OptionSelection> = Vec::new();
    workset_visibility: Vec<WorksetVisibility> = Vec::new();
    #[dsl(block)]
    viewport: store::Viewport2d = store::Viewport2d { x: 0.0, y: 0.0, zoom: 40.0 };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
