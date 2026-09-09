use super::*;
use crate::editor::vcs::testkit::{app, dispatch};
use crate::editor::vcs::VcsCommand;

#[semio_framework_async_macros::async_test]
async fn increment_counter_action_updates_projection() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot").counter;
    let result = dispatch(&mut instance, VcsCommand::IncrementCounter(IncrementCounter {})).await;
    assert_eq!(result.mutations.len(), 1);
    assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
}
