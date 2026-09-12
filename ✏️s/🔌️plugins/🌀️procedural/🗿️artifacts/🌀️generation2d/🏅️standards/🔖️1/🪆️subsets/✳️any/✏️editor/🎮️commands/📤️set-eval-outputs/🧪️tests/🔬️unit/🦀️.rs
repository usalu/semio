use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_eval_outputs_does_not_mutate_the_document() {
    let mut app = app().await;
    let before = snapshot_read(&app);
    dispatch(&mut app, Generation2dCommand::SetEvalOutputs(SetEvalOutputs { outputs_json: "{}".into() })).await;
    let after = snapshot_read(&app);
    let unchanged = after == before;
    close(app);
    assert!(unchanged, "setEvalOutputs writes only the retained evaluation session");
}
