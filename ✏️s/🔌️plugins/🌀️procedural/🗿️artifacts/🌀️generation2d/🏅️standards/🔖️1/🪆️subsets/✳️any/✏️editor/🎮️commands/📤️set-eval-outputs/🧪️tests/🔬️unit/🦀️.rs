
use super::*;
use crate::editor::generation2d::Generation2dCommand;
use crate::editor::generation2d::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn set_eval_outputs_does_not_mutate_the_document() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Generation2dCommand::SetEvalOutputs(SetEvalOutputs { outputs_json: "{}".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot"), before);
}
