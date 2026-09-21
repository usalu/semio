use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn reorganize_keeps_every_widget() {
    let mut app = flow_app().await;
    let before_scene = app.snapshot().expect("snapshot").to_host_snapshot();
    let before = before_scene.widgets.len();
    before_scene.retire_cold();
    dispatch(&mut app, FlowCommand::Reorganize(Reorganize {})).await;
    settle(&mut app).await;
    let after_scene = app.snapshot().expect("snapshot").to_host_snapshot();
    assert_eq!(after_scene.widgets.len(), before);
    after_scene.retire_cold();
}
