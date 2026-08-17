//! ✏️ Writer play app — the `edit` mode: the single-window jack/text-editor authoring layout.

use crate::editor::writer::modes::edit::windows::main;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WRITER_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::writer::create_writer_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WRITER_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`. A single full-width window (writer has exactly one window kind).
pub fn layout() -> WindowLayout {
    create_default_layout(&[main::WRITER_PLAY_WINDOW_KIND.into()], "row", Some(&[100.0]), Some(&["Jack".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_the_main_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::WRITER_PLAY_WINDOW_KIND), "layout must reference the main window kind: {json}");
    }
}
//#endregion 🧪️Tests
