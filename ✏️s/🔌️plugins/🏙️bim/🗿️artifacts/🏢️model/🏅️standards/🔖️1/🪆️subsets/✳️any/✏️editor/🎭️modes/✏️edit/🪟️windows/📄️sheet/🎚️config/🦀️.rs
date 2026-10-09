//! 🎚️ Persisted local state for one exact BIM sheet window: which sheet it shows and its navigation, in paper millimetres from the top left corner of the sheet. The paper, the viewports and the title block are authored in the sheet.

use crate::editor::bim::kit::window_config;

window_config! {
    window: super::WINDOW_KIND_ID,
    schema: "bim.sheet-window.config",
    envelope: "s.bim.model.sheet-window.config",
    extension: "bimsheetwindowcfg",
    owner_path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📄️sheet/🎚️config",
    display: "Set BIM Sheet Window Configuration",
    type BimSheetWindowConfig, BimSheetWindowConfigDiff, BimSheetWindowConfigMutation, BimSheetWindowConfigOwner;
    sheet: String = String::new();
    framed: bool = false;
    #[dsl(block)]
    viewport: store::Viewport2d = store::Viewport2d { x: 210.0, y: 148.5, zoom: 1.5 };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
