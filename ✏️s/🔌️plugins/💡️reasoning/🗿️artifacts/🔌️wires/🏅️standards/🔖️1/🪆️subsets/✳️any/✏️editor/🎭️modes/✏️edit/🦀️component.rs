//! ✏️ Wires play app — the `edit` mode: the sole window layout (the WIRES canvas, full width).

use crate::editor::wires::modes::edit::windows::canvas;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WIRES_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::wires::create_wires_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WIRES_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[canvas::WIRES_PLAY_WINDOW_CANVAS.into()], "row", Some(&[100.0]), Some(&["Canvas".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_canvas_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(canvas::WIRES_PLAY_WINDOW_CANVAS), "layout must reference the canvas window kind: {json}");
    }
}
//#endregion 🧪️Tests
