//! 📜️ VCS play app — the history window: the checkpoint/alternative swimlane graph.

use crate::apps::vcs::VCS_PLAY_APP_ID;
use semio_framework_plugin::{build_graph_timeline_scene, GraphTimelineScene, HistoryView, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const VCS_PLAY_WINDOW_HISTORY: &str = "vcs-history";
pub const VCS_PLAY_BODY_HISTORY: &str = "vcs.play.history";
const VCS_PLAY_SURFACE_HISTORY: &str = "vcs.play.history";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::vcs::create_vcs_app`.
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
pub fn render(history: &HistoryView) -> UiNode {
    build_graph_timeline_scene(VCS_PLAY_SURFACE_HISTORY, VCS_PLAY_APP_ID, GraphTimelineScene { columns_json: serde_json::to_string(&history.columns).unwrap_or_else(|_| "[]".into()) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::testkit::{app, render as render_body};

    #[test]
    fn renders_history_scene() {
        let mut instance = app();
        let json = render_body(&mut instance, VCS_PLAY_BODY_HISTORY);
        assert!(json.contains("graph-timeline"), "missing graph-timeline surface kind: {json}");
        assert!(json.contains("lane"), "missing lane field in history columns: {json}");
        assert!(!json.contains("\"table\""), "history must not fall back to a generic table: {json}");
    }
}
//#endregion 🧪️Tests
