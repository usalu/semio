use super::*;
use crate::editor::playbook::commands::move_step::MoveStep;
use crate::editor::playbook::commands::remove_step::RemoveStep;
use crate::editor::playbook::commands::update_playbook::UpdatePlaybook;
use crate::editor::playbook::unit_tests::context::{dispatch, history_verb, live_spec, playbook_app};
use crate::editor::playbook::PlaybookCommand;
use AddStep;

#[semio_framework_async_macros::async_test]
async fn add_step_action_appends_step() {
    let mut app = playbook_app().await;
    let before = live_spec(&app).await.steps.len();
    dispatch(&mut app, PlaybookCommand::AddStep(AddStep {})).await;
    let steps = live_spec(&app).await.steps;
    assert_eq!(steps.len(), before + 1);
    assert_eq!(steps.last().expect("appended step").title, format!("Step {}", before + 1));
}

/// ↔️ A move rewires the chain and an undo of a middle remove restores the order (the chain, not the node vector, is the order).
#[semio_framework_async_macros::async_test]
async fn remove_and_move_step_actions_keep_the_chain_order_through_undo() {
    let mut app = playbook_app().await;
    dispatch(&mut app, PlaybookCommand::AddStep(AddStep {})).await;
    dispatch(&mut app, PlaybookCommand::AddStep(AddStep {})).await;
    let ids: Vec<String> = live_spec(&app).await.steps.into_iter().map(|step| step.id).collect();
    dispatch(&mut app, PlaybookCommand::MoveStep(MoveStep { step_id: ids[2].clone(), index: 0 })).await;
    let moved: Vec<String> = live_spec(&app).await.steps.into_iter().map(|step| step.id).collect();
    assert_eq!(moved, vec![ids[2].clone(), ids[0].clone(), ids[1].clone()]);
    dispatch(&mut app, PlaybookCommand::RemoveStep(RemoveStep { step_id: ids[0].clone() })).await;
    let removed: Vec<String> = live_spec(&app).await.steps.into_iter().map(|step| step.id).collect();
    assert_eq!(removed, vec![ids[2].clone(), ids[1].clone()]);
    history_verb(&mut app, "undo").await;
    let restored: Vec<String> = live_spec(&app).await.steps.into_iter().map(|step| step.id).collect();
    assert_eq!(restored, moved, "undo of a middle remove restores the step at its chain position");
}

#[semio_framework_async_macros::async_test]
async fn remove_step_with_empty_id_is_a_no_op() {
    let mut app = playbook_app().await;
    let before = live_spec(&app).await.steps.len();
    dispatch(&mut app, PlaybookCommand::RemoveStep(RemoveStep { step_id: String::new() })).await;
    assert_eq!(live_spec(&app).await.steps.len(), before);
}

/// ⚖️ Every committed title is its own edit: two dispatches are two undo steps, never a coalesced burst that would merge
/// separate renames into one history row.
#[semio_framework_async_macros::async_test]
async fn every_committed_title_is_one_undo_step() {
    let mut app = playbook_app().await;
    for title in ["Draft", "Recipe"] {
        dispatch(&mut app, PlaybookCommand::UpdatePlaybook(UpdatePlaybook { value: title.into() })).await;
    }
    assert_eq!(app.snapshot().expect("projection").title.as_deref(), Some("Recipe"));
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("projection").title.as_deref(), Some("Draft"), "one undo restores the previous committed title");
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("projection").title, None);
}
