
use super::*;
use crate::editor::flow::testkit::{FlowApp, dispatch, flow_app, render};
use crate::editor::flow::{FLOW_PLAY_BODY_MAIN, FlowCommand};
use serde_json::{Value, json};

async fn preview_off_ids(app: &mut FlowApp) -> Value {
    let rendered: Value = serde_json::from_str(&render(app, FLOW_PLAY_BODY_MAIN).await).expect("render json");
    rendered.pointer("/nodeGraph/previewOffJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
}

#[semio_framework_async_macros::async_test]
async fn set_preview_off_toggles_ids_on_and_off_the_scene() {
    use crate::editor::flow::commands::set_preview_off;
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: true })).await;
    assert_eq!(preview_off_ids(&mut app).await, json!(["slider"]));
    dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: false })).await;
    assert_eq!(preview_off_ids(&mut app).await, Value::Null, "an empty preview-off set is omitted from the scene");
}

#[semio_framework_async_macros::async_test]
async fn node_graph_viewport_moves_the_camera() {
    let mut app = flow_app().await;
    let before = render(&mut app, FLOW_PLAY_BODY_MAIN).await;
    dispatch(&mut app, FlowCommand::NodeGraphViewport(NodeGraphViewport { camera: CameraJson { x: 30.0, y: -12.0, zoom: 2.0 } })).await;
    assert_ne!(before, render(&mut app, FLOW_PLAY_BODY_MAIN).await);
}
