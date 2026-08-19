//! ✏️ VCS play app — the `edit` mode: the app's only mode, a two-window authoring layout (editor + history).

use crate::editor::vcs::modes::edit::windows::{editor, history};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const VCS_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::vcs::create_vcs_app`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: VCS_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub async fn layout() -> WindowLayout {
    create_default_layout(&[editor::VCS_PLAY_WINDOW_EDITOR.into(), history::VCS_PLAY_WINDOW_HISTORY.into()], "row", Some(&[30.0, 70.0]), Some(&["Editor".into(), "History".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(editor::VCS_PLAY_WINDOW_EDITOR) && json.contains(history::VCS_PLAY_WINDOW_HISTORY), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
