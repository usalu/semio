use crate::editor::sequence::unit_tests::context::{dispatch_in_script, new_app, render_in, script_window_meta};
use crate::editor::sequence::SequenceCommand;

use super::run_command::Run;
use super::stop_command::Stop;

/// 🏃️ `run`/`stop` publish the last run result as the SCRIPT window's own window transient, so both
/// the dispatch and the read have to be addressed at that window instance — an unaddressed dispatch
/// is refused with `sequence-script-window-view-required`, and an unaddressed render sees no
/// transient at all.
#[semio_framework_async_macros::async_test]
async fn run_stores_result_and_renders_in_script() {
    let mut app = new_app().await;
    dispatch_in_script(&mut app, SequenceCommand::Run(Run {})).await;
    let view = script_window_meta().view_state.expect("script window view");
    assert!(render_in(&mut app, crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT, &view).await.contains("run result"));
}

#[semio_framework_async_macros::async_test]
async fn stop_command_clears_last_run_result() {
    let mut app = new_app().await;
    dispatch_in_script(&mut app, SequenceCommand::Run(Run {})).await;
    dispatch_in_script(&mut app, SequenceCommand::Stop(Stop {})).await;
    let view = script_window_meta().view_state.expect("script window view");
    assert!(!render_in(&mut app, crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT, &view).await.contains("run result"));
}
