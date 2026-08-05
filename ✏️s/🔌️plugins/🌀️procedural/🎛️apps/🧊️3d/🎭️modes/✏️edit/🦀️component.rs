//! ✏️ Procedural3d play app — the `edit` mode: the default two-window authoring layout (flow graph +
//! 3D preview).

use crate::apps::procedural3d::modes::edit::windows::{flow, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PROCEDURAL_3D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PROCEDURAL_3D_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub fn layout() -> WindowLayout {
    create_default_layout(&[flow::PROCEDURAL_3D_PLAY_WINDOW_MAIN.into(), preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(flow::PROCEDURAL_3D_PLAY_WINDOW_MAIN) && json.contains(preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
    }
}
//#endregion 🧪️Tests
