
use super::*;
use crate::editor::vcs::VcsCommand;
use crate::editor::vcs::commands::{edit, text_edit};
use crate::editor::vcs::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn vcs_demo_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::PatchSnapshot(PatchSnapshot { field: "title".into(), value: "Renamed".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::TextEdit(text_edit::TextEdit { text: "{}".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::Edit(edit::Edit { text: "{}".into() }));
}

#[semio_framework_async_macros::async_test]
async fn vcs_demo_command_op_binary_agrees_with_text() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&VcsCommand::PatchSnapshot(PatchSnapshot { field: "counter".into(), value: "3".into() }));
}

#[semio_framework_async_macros::async_test]
async fn text_edit_action_persists_projection_changes() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot");
    let mut edited = before.clone();
    edited.title = "Edited via JSON".into();
    edited.counter = before.counter + 41;
    edited.tags.push("edited-in-place".into());
    let text = serde_json::to_string_pretty(&edited).unwrap();
    let result = dispatch(&mut instance, VcsCommand::TextEdit(text_edit::TextEdit { text })).await;
    assert!(!result.mutations.is_empty());
    let after = instance.snapshot().expect("materialize snapshot");
    assert_eq!(after.title, "Edited via JSON");
    assert_eq!(after.counter, before.counter + 41);
    assert!(after.tags.contains(&"edited-in-place".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn edit_action_is_alias_for_text_edit() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot");
    let mut edited = before;
    edited.status = "reviewed".into();
    let text = serde_json::to_string(&edited).unwrap();
    let result = dispatch(&mut instance, VcsCommand::Edit(edit::Edit { text })).await;
    assert!(!result.mutations.is_empty());
    assert_eq!(instance.snapshot().expect("materialize snapshot").status, "reviewed");
}
