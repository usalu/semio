//! 👁️ Writer viewer — the `view` mode: a single full-pane read-only text window, the read-only
//! counterpart of the editor's `edit` mode.

use crate::viewer::writer::modes::view::windows::main;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WRITER_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::writer::create_writer_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WRITER_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-width window — the read-only viewer has no quadrant layout to allocate.
pub fn layout() -> WindowLayout {
    create_default_layout(&[main::WRITER_VIEW_WINDOW_KIND.into()], "row", Some(&[100.0]), Some(&["Text".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_main_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::WRITER_VIEW_WINDOW_KIND), "layout must reference the main window kind: {json}");
    }
}
//#endregion 🧪️Tests
