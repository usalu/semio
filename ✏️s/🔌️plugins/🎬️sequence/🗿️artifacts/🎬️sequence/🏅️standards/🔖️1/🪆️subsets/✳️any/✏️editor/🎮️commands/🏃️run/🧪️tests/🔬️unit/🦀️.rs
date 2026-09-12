use crate::editor::sequence::unit_tests::context::{dispatch, new_app, render};
use crate::editor::sequence::SequenceCommand;

use super::run_command::Run;
use super::stop_command::Stop;

#[semio_framework_async_macros::async_test]
async fn run_stores_result_and_renders_in_script() {
    let mut app = new_app().await;
    dispatch(&mut app, SequenceCommand::Run(Run {})).await;
    assert!(render(&mut app, crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT).await.contains("run result"));
}

#[semio_framework_async_macros::async_test]
async fn stop_command_clears_last_run_result() {
    let mut app = new_app().await;
    dispatch(&mut app, SequenceCommand::Run(Run {})).await;
    dispatch(&mut app, SequenceCommand::Stop(Stop {})).await;
    assert!(!render(&mut app, crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT).await.contains("run result"));
}
