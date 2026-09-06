//! 📜️ VCS play app — the history window: the checkpoint/alternative swimlane graph.

use crate::editor::vcs::VCS_PLAY_APP_ID;
use semio_framework_plugin::{scene_surface, BuiltNode, HistoryView, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const VCS_PLAY_WINDOW_HISTORY: &str = "vcs-history";
pub const VCS_PLAY_BODY_HISTORY: &str = "vcs.play.history";
const VCS_PLAY_SURFACE_HISTORY: &str = "vcs.play.history";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::vcs::create_vcs_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: VCS_PLAY_WINDOW_HISTORY.into(),
        label: LocalizedLabel::native("History", "Verlauf"),
        body_key: VCS_PLAY_BODY_HISTORY.into(),
        surface_kind: SurfaceKind::GraphTimeline,
        icon_id: "git-branch".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_vcs_app`'s `.window_kind_interactions(..)` call (the
        // "history" domain — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(history: &HistoryView) -> UiAssemblyResult<BuiltNode> {
    let scene = semio_framework_ui_scene::GraphTimelineScene { columns_json: dsl::json::to_json_string(&history.columns) };
    scene_surface(VCS_PLAY_SURFACE_HISTORY, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::GraphTimeline, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_history_scene() {
        let mut instance = app().await;
        let json = render_body(&mut instance, VCS_PLAY_BODY_HISTORY).await;
        assert!(json.contains("graph-timeline"), "missing graph-timeline surface kind: {json}");
        assert!(json.contains("lane"), "missing lane field in history columns: {json}");
        assert!(!json.contains("\"table\""), "history must not fall back to a generic table: {json}");
    }
}
//#endregion 🧪️Tests
