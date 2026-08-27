//! ✏️ Flow play app — the `edit` mode: the default two-window authoring layout (graph + compiled DSL).

use crate::editor::flow::modes::edit::windows::{compiled, main};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const FLOW_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::flow::create_flow_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: FLOW_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[main::FLOW_PLAY_WINDOW_MAIN.into(), compiled::FLOW_PLAY_WINDOW_COMPILED.into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "DSL".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(main::FLOW_PLAY_WINDOW_MAIN) && json.contains(compiled::FLOW_PLAY_WINDOW_COMPILED), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
