use super::*;
use crate::editor::vcs::unit_tests::context::{app, dispatch};
use crate::editor::vcs::VcsCommand;

#[semio_framework_async_macros::async_test]
async fn rename_vcs_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::RenameVcs(RenameVcs { title: "Renamed".into() }));
}

#[semio_framework_async_macros::async_test]
async fn rename_vcs_command_updates_exactly_its_field() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot");
    let result = dispatch(&mut instance, VcsCommand::RenameVcs(RenameVcs { title: "Renamed".into() })).await;
    assert!(result.edited_document(), "renameVcs must publish on the document lane");
    let after = instance.snapshot().expect("materialize snapshot");
    assert_eq!(after.title, "Renamed");
    let mut expected = before;
    expected.title = "Renamed".into();
    assert_eq!(after, expected);
}
