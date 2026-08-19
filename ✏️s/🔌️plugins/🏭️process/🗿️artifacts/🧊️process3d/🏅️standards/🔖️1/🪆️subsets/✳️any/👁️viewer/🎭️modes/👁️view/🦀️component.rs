//! 👁️ Process 3D viewer — the `view` mode: a single full-pane Workpiece window, the read-only
//! counterpart of the editor's single-window `edit` mode.

use crate::viewer::process3d::modes::view::windows::workpiece;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PROCESS3D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::process3d::create_process3d_viewer`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: PROCESS3D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Workpiece window — the read-only viewer has no chrome to allocate around it.
pub async fn layout() -> WindowLayout {
    create_default_layout(&[workpiece::PROCESS3D_VIEW_WINDOW_MAIN.into()], "row", None, Some(&["Workpiece".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_default_layout_references_the_workpiece_window() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(workpiece::PROCESS3D_VIEW_WINDOW_MAIN), "layout must reference the workpiece window kind: {json}");
    }
}
//#endregion 🧪️Tests
