use super::*;
use crate::editor::flow::testkit::{dispatch, flow_app, render};
use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_rejects_unknown_and_accepts_known() {
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::SetLodMode(SetLodMode { value: "bogus".into() })).await;
    dispatch(&mut app, FlowCommand::SetLodMode(SetLodMode { value: "micro".into() })).await;
    let json = render(&mut app, FLOW_PLAY_BODY_MAIN).await;
    assert!(json.contains("\\\"forcedLabel\\\":\\\"micro\\\"") || json.contains("\"forcedLabel\":\"micro\""));
}

#[semio_framework_async_macros::async_test]
async fn default_runtime_enables_proximity_distance() {
    let mut app = flow_app().await;
    let json = render(&mut app, FLOW_PLAY_BODY_MAIN).await;
    assert!(json.contains("proximityDistance") && !json.contains(r#""proximityDistance":0"#));
}

#[semio_framework_async_macros::async_test]
async fn set_proximity_distance_updates_scene_lod_json() {
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::SetProximityDistance(crate::editor::flow::commands::set_proximity_distance::SetProximityDistance { value: 96.0 })).await;
    assert!(render(&mut app, FLOW_PLAY_BODY_MAIN).await.contains("96"));
}

#[semio_framework_async_macros::async_test]
async fn negative_proximity_distances_clamp_to_zero() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::SetProximityDistance(crate::editor::flow::commands::set_proximity_distance::SetProximityDistance { value: -10.0 })).await;
    assert!(result.mutations.is_empty(), "a view command emits no document operations");
    assert!(!render(&mut app, FLOW_PLAY_BODY_MAIN).await.contains("-10"));
}
