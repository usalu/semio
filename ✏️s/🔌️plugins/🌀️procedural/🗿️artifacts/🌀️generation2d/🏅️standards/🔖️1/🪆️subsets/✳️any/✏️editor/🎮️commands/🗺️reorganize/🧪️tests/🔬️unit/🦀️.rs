use super::*;
use crate::editor::generation2d::commands::node_graph_viewport;
use crate::editor::generation2d::testkit::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn reorganize_emits_operations() {
    let mut app = app().await;
    let placements = |app: &crate::editor::generation2d::testkit::Generation2dApp| -> Vec<(String, u64, u64)> {
        snapshot_read(app).fixture.layout.iter().map(|(id, layout)| (id.clone(), layout.x.to_bits(), layout.y.to_bits())).collect()
    };
    let before = placements(&app);
    dispatch(&mut app, Generation2dCommand::Reorganize(Reorganize {})).await;
    let after = placements(&app);
    close(app);
    assert_ne!(before, after);
}

#[semio_framework_async_macros::async_test]
async fn node_graph_viewport_sets_camera() {
    let mut app = app().await;
    dispatch(&mut app, Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: dsl::json::to_json_string(&semio_framework_artifact_flow_flow::CameraJson { x: 1.0, y: 2.0, zoom: 3.0 }) })).await;
    close(app);
}
