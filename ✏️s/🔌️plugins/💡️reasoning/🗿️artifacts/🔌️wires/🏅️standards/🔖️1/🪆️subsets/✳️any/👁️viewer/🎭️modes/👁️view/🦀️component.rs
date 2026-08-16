//! 👁️ Wires viewer — the `view` mode: the sole window layout (the read-only WIRES canvas, full width).

use crate::viewer::wires::modes::view::windows::canvas;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WIRES_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::wires::create_wires_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WIRES_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The viewer's default window layout — this mode is the app's `default_mode_id`, so its layout IS
/// the app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[canvas::WIRES_VIEW_WINDOW_CANVAS.into()], "row", Some(&[100.0]), Some(&["Canvas".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_canvas_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(canvas::WIRES_VIEW_WINDOW_CANVAS), "layout must reference the canvas window kind: {json}");
    }
}
//#endregion 🧪️Tests
