//! ✏️ Process 3d play app — the `edit` mode: the single-window workpiece authoring layout.

use crate::apps::process3d::modes::edit::windows::workpiece;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PROCESS3D_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::process3d::create_process3d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PROCESS3D_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into()], "row", None, Some(&["Workpiece".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_references_the_workpiece_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN), "layout must reference the workpiece window kind: {json}");
    }
}
//#endregion 🧪️Tests
