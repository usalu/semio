
use super::*;
use crate::editor::generation2d::Generation2dCommand;
use crate::editor::generation2d::commands::node_graph_viewport;
use crate::editor::generation2d::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn reorganize_emits_operations() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").fixture;
    dispatch(&mut app, Generation2dCommand::Reorganize(Reorganize {})).await;
    let after = app.snapshot().expect("snapshot").fixture;
    assert_ne!(before.layout, after.layout);
}

#[semio_framework_async_macros::async_test]
async fn node_graph_viewport_sets_camera() {
    let mut app = app().await;
    dispatch(&mut app, Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: dsl::json::to_json_string(&semio_framework_artifact_flow_flow::CameraJson { x: 1.0, y: 2.0, zoom: 3.0 }) })).await;
}
