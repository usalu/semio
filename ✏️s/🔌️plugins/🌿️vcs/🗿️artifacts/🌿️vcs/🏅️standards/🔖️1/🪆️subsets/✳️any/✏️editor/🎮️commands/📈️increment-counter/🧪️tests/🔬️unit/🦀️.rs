use super::*;
use crate::editor::vcs::unit_tests::context::{app, dispatch};
use crate::editor::vcs::VcsCommand;

#[semio_framework_async_macros::async_test]
async fn increment_counter_action_updates_projection() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot").counter;
    // 🧾️ A mounted app publishes AFTER it answers, so the immediate answer carries no mutations —
    // the settled receipt's `Artifact` lane is the witness that the document edit landed.
    let result = dispatch(&mut instance, VcsCommand::IncrementCounter(IncrementCounter {})).await;
    assert!(result.edited_document(), "incrementCounter must publish on the document lane");
    assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
}
