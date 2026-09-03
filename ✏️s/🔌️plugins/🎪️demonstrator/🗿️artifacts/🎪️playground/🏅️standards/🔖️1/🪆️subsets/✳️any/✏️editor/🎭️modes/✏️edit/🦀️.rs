//! ✏️ Playground editor — the `edit` mode: a single window over the document's one `schema` field.

use crate::editor::playground::modes::edit::windows::main;
use semio_framework_plugin::{create_stack_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PLAYGROUND_EDIT_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::playground::create_playground_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PLAYGROUND_EDIT_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One window, one layout slot.
pub fn layout() -> WindowLayout {
    create_stack_layout(&[main::WINDOW_KIND_ID.into()], Some(&["Schema".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_edit_layout_lists_the_one_window() {
        let json = dsl::os_pack::json::to_json_string(&layout());
        assert!(json.contains(main::WINDOW_KIND_ID), "layout must reference the main window kind: {json}");
    }
}
//#endregion 🧪️Tests
