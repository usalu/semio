//! ✏️ Sequence play app — the `edit` mode: the app's only mode, a three-window authoring layout
//! (graph canvas + compiled script + DSL).

use crate::apps::sequence::modes::edit::windows::{compiled, main, script};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const SEQUENCE_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::sequence::create_sequence_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: SEQUENCE_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS
/// the app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(
        &[main::SEQUENCE_PLAY_WINDOW_MAIN.into(), script::SEQUENCE_PLAY_WINDOW_SCRIPT.into(), compiled::SEQUENCE_PLAY_WINDOW_COMPILED.into()],
        "row",
        Some(&[50.0, 25.0, 25.0]),
        Some(&["Sequence".into(), "Script".into(), "DSL".into()]),
    )
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_all_three_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::SEQUENCE_PLAY_WINDOW_MAIN) && json.contains(script::SEQUENCE_PLAY_WINDOW_SCRIPT) && json.contains(compiled::SEQUENCE_PLAY_WINDOW_COMPILED), "layout must reference all three window kinds: {json}");
    }
}
//#endregion 🧪️Tests
