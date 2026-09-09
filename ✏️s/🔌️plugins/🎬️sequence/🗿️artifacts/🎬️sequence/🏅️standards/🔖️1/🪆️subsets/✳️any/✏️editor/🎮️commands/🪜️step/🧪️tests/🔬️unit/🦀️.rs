use crate::editor::sequence::testkit::{dispatch, new_app, new_app_with_registry_wired, select_steps};
use crate::editor::sequence::SequenceCommand;

use super::add_step::AddStep;
use super::delete_selection::DeleteSelection;
use super::remove_step::RemoveStep;

#[semio_framework_async_macros::async_test]
async fn add_step_command_appends_step() {
    let mut app = new_app().await;
    dispatch(&mut app, SequenceCommand::AddStep(AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 })).await;
    assert!(app.snapshot().expect("projection").to_fixture().steps.len() > 2);
}

#[semio_framework_async_macros::async_test]
async fn remove_step_command_deletes_step() {
    let mut app = new_app().await;
    let step_id = app.snapshot().expect("projection").to_fixture().steps[0].id.clone();
    dispatch(&mut app, SequenceCommand::RemoveStep(RemoveStep { id: step_id.clone() })).await;
    assert!(app.snapshot().expect("projection").to_fixture().steps.iter().all(|step| step.id != step_id));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: end-to-end proof the "steps"
/// domain's live selection actually drives `deleteSelection` — selects `step-1` via the
/// framework's real `interactionSelect` action (`select_steps`, the only way a downstream crate
/// can populate a genuine `InteractionView`), then confirms `deleteSelection` removes exactly
/// that step.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_live_selected_step() {
    let mut app = new_app_with_registry_wired().await;
    select_steps(&mut app, &["step-1"]).await;
    dispatch(&mut app, SequenceCommand::DeleteSelection(DeleteSelection {})).await;
    assert!(!app.snapshot().expect("projection").to_fixture().steps.iter().any(|step| step.id == "step-1"), "selected step must be deleted");
}
