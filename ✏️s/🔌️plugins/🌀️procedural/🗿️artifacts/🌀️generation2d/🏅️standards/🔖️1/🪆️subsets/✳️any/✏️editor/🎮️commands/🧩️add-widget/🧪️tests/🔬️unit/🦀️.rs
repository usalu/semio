use super::*;
use crate::editor::generation2d::testkit::{app, dispatch};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn add_widget_emits_op_and_grows_document() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").fixture.widgets.len();
    dispatch(&mut app, Generation2dCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None })).await;
    assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before + 1);
}
