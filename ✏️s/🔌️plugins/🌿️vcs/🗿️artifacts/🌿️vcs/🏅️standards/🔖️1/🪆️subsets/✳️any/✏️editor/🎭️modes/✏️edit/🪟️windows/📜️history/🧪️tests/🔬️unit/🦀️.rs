use super::*;
use crate::editor::vcs::unit_tests::context::{app, render as render_body};

/// 🗄️ The history window is a PACKED `graph-timeline` surface now — its `HistoryColumn[]` travels as
/// the scene doc's binary `columnsJson`, not as literal text in the projected tree — so the lane
/// assertion reads the decoded scene instead of grepping the projection.
#[semio_framework_async_macros::async_test]
async fn renders_history_scene() {
    let mut instance = app().await;
    let json = render_body(&mut instance, VCS_PLAY_BODY_HISTORY).await;
    assert!(json.contains("graph-timeline"), "missing graph-timeline surface kind: {json}");
    assert!(!json.contains("\"table\""), "history must not fall back to a generic table: {json}");
    let scene: GraphTimelineScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&json).expect("packed graph-timeline scene");
    let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("history columns json");
    assert!(!columns.is_empty(), "the seeded history renders at least one checkpoint column");
    assert!(columns.iter().all(|column| column.get("lane").and_then(serde_json::Value::as_u64).is_some()), "every history column carries its swimlane: {}", scene.columns_json);
}
