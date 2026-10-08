use super::*;
use crate::editor::vcs::unit_tests::context::{app, dispatch};
use crate::editor::vcs::VcsCommand;

#[semio_framework_async_macros::async_test]
async fn change_notes_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::ChangeNotes(ChangeNotes { notes: "Jotted".into() }));
}

#[semio_framework_async_macros::async_test]
async fn change_notes_command_updates_exactly_its_field() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot");
    let result = dispatch(&mut instance, VcsCommand::ChangeNotes(ChangeNotes { notes: "Jotted".into() })).await;
    assert!(result.edited_document(), "changeNotes must publish on the document lane");
    let after = instance.snapshot().expect("materialize snapshot");
    assert_eq!(after.notes, "Jotted");
    let mut expected = before;
    expected.notes = "Jotted".into();
    assert_eq!(after, expected);
}
