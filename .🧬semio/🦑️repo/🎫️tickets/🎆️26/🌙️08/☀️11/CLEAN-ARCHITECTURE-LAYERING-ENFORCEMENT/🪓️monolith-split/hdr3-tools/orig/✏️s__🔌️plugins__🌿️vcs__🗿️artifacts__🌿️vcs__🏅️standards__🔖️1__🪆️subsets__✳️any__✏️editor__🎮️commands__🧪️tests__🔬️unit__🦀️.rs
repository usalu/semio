//! 🧪️ Aggregated unit tests of the `commands` leaves, one module per leaf.

mod increment_counter {
use super::super::increment_counter::*;
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
}

mod no_operation {
use super::super::no_operation::*;
use crate::editor::vcs::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel};
use crate::editor::vcs::VcsCommand;

#[semio_framework_async_macros::async_test]
async fn vcs_demo_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::NoMutation(NoMutation {}));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: Vec::new() }));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: false }));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {}));
}
}

mod rename_vcs {
use super::super::rename_vcs::*;
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
}

mod change_counter {
use super::super::change_counter::*;
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
}

mod change_status {
use super::super::change_status::*;
use crate::editor::vcs::unit_tests::context::{app, dispatch};
use crate::editor::vcs::VcsCommand;

#[semio_framework_async_macros::async_test]
async fn change_status_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::ChangeStatus(ChangeStatus { status: "reviewed".into() }));
}

#[semio_framework_async_macros::async_test]
async fn change_status_command_updates_exactly_its_field() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot");
    let result = dispatch(&mut instance, VcsCommand::ChangeStatus(ChangeStatus { status: "reviewed".into() })).await;
    assert!(result.edited_document(), "changeStatus must publish on the document lane");
    let after = instance.snapshot().expect("materialize snapshot");
    assert_eq!(after.status, "reviewed");
    let mut expected = before;
    expected.status = "reviewed".into();
    assert_eq!(after, expected);
}
}

mod change_notes {
use super::super::change_notes::*;
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
}
