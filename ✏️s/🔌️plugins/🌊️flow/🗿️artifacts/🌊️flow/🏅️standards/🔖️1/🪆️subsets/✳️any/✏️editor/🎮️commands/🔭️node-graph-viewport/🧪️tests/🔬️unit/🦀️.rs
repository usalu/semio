use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, render, settle, FlowApp};
use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};
use serde_json::{json, Value};

async fn node_graph_scene(app: &mut FlowApp) -> ui_wgpu::wgpu::NodeGraphScene {
    semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&render(app, FLOW_PLAY_BODY_MAIN).await).expect("rendered nodeGraph scene")
}

async fn preview_off_ids(app: &mut FlowApp) -> Value {
    node_graph_scene(app).await.preview_off_json.as_deref().and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
}

#[semio_framework_async_macros::async_test]
async fn set_preview_off_toggles_ids_on_and_off_the_scene() {
    use crate::editor::flow::commands::set_preview_off;
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: true })).await;
    settle(&mut app).await;
    assert_eq!(preview_off_ids(&mut app).await, json!(["slider"]));
    dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: false })).await;
    settle(&mut app).await;
    assert_eq!(preview_off_ids(&mut app).await, Value::Null, "an empty preview-off set is omitted from the scene");
}

#[semio_framework_async_macros::async_test]
async fn node_graph_viewport_moves_the_camera() {
    let mut app = flow_app().await;
    let before = node_graph_scene(&mut app).await.viewport;
    dispatch(&mut app, FlowCommand::NodeGraphViewport(NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 30.0, y: -12.0, zoom: 2.0 } })).await;
    settle(&mut app).await;
    let after = node_graph_scene(&mut app).await.viewport.expect("Flow main scene publishes its authored viewport");
    assert_ne!(before, Some(after), "the retained viewport publication changes the authored camera");
    assert_eq!((after.x, after.y, after.zoom), (30.0, -12.0, 2.0));
}
