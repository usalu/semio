use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn add_widget_emits_op_and_grows_document() {
    let mut app = app().await;
    let before = snapshot_read(&app).fixture.widgets.len();
    dispatch(&mut app, Generation2dCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None })).await;
    let after = snapshot_read(&app).fixture.widgets.len();
    close(app);
    assert_eq!(after, before + 1);
}
