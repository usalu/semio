//! 🖊️ Animate presentation app — the `main` mode: the single tile-editor authoring layout.

use crate::editor::animate::modes::main::windows::tile_editor;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PRESENTATION_PLAY_MODE_MAIN: &str = "main";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::animate::create_animate_presentation_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PRESENTATION_PLAY_MODE_MAIN.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[tile_editor::PRESENTATION_PLAY_WINDOW_MAIN.into()], "stack", None, Some(&["Tile editor".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_tile_editor_window() {
        let json = dsl::os_pack::json::to_json_string(&layout());
        assert!(json.contains(tile_editor::PRESENTATION_PLAY_WINDOW_MAIN), "layout must reference the tile-editor window kind: {json}");
    }
}
//#endregion 🧪️Tests
