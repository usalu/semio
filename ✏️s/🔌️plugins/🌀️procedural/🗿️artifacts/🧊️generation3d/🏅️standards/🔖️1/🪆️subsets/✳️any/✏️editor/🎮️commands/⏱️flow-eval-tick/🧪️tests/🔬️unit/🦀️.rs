use super::*;
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;

#[semio_framework_async_macros::async_test]
async fn flow_eval_tick_does_not_panic_with_nothing_pending() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::FlowEvalTick(FlowEvalTick {})).await;
}
