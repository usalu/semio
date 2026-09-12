use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn reorganize_keeps_every_widget() {
    let mut app = flow_app().await;
    let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
    dispatch(&mut app, FlowCommand::Reorganize(Reorganize {})).await;
    assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before);
}
