use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, render, settle, FlowApp};
use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

async fn lod(app: &mut FlowApp) -> serde_json::Value {
    let rendered = render(app, FLOW_PLAY_BODY_MAIN).await;
    let scene: ui_wgpu::wgpu::NodeGraphScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&rendered).expect("rendered nodeGraph scene");
    serde_json::from_str(scene.lod_json.as_deref().expect("nodeGraph LOD payload")).expect("nodeGraph LOD JSON")
}

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_rejects_unknown_and_accepts_known() {
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::SetLodMode(SetLodMode { value: "bogus".into() })).await;
    settle(&mut app).await;
    dispatch(&mut app, FlowCommand::SetLodMode(SetLodMode { value: "micro".into() })).await;
    settle(&mut app).await;
    assert_eq!(lod(&mut app).await["forcedLabel"], "micro");
}

#[semio_framework_async_macros::async_test]
async fn default_runtime_enables_proximity_distance() {
    let mut app = flow_app().await;
    assert_eq!(lod(&mut app).await["proximityDistance"], 48.0);
}

#[semio_framework_async_macros::async_test]
async fn set_proximity_distance_updates_scene_lod_json() {
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::SetProximityDistance(crate::editor::flow::commands::set_proximity_distance::SetProximityDistance { value: 96.0 })).await;
    settle(&mut app).await;
    assert_eq!(lod(&mut app).await["proximityDistance"], 96.0);
}

#[semio_framework_async_macros::async_test]
async fn negative_proximity_distances_clamp_to_zero() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::SetProximityDistance(crate::editor::flow::commands::set_proximity_distance::SetProximityDistance { value: -10.0 })).await;
    assert!(result.mutations.is_empty(), "a view command emits no document operations");
    settle(&mut app).await;
    assert_eq!(lod(&mut app).await["proximityDistance"], 0.0);
}
