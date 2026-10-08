use super::*;
use crate::editor::vcs::unit_tests::context::{app, dispatch};
use crate::editor::vcs::VcsCommand;

#[semio_framework_async_macros::async_test]
async fn change_counter_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::ChangeCounter(ChangeCounter { value: 7 }));
}

#[semio_framework_async_macros::async_test]
async fn change_counter_command_updates_exactly_its_field() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot");
    let result = dispatch(&mut instance, VcsCommand::ChangeCounter(ChangeCounter { value: 7 })).await;
    assert!(result.edited_document(), "changeCounter must publish on the document lane");
    let after = instance.snapshot().expect("materialize snapshot");
    assert_eq!(after.counter, 7);
    let mut expected = before;
    expected.counter = 7;
    assert_eq!(after, expected);
}
