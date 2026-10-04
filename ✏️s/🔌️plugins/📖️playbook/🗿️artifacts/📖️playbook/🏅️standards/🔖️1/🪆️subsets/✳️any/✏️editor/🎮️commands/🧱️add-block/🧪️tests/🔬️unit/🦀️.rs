use super::*;
use crate::editor::playbook::commands::move_block::MoveBlock;
use crate::editor::playbook::commands::remove_block::RemoveBlock;
use crate::editor::playbook::unit_tests::context::{dispatch, live_spec, playbook_app};
use crate::editor::playbook::PlaybookCommand;
use AddBlock;

/// 🧱️ The block lands in the first step of the chain and is published on the `flow` child, never in the parent document.
#[semio_framework_async_macros::async_test]
async fn add_block_action_appends_block() {
    let mut app = playbook_app().await;
    let parent = app.snapshot().expect("projection");
    dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None })).await;
    let steps = live_spec(&app).await.steps;
    assert_eq!(steps[0].blocks.len(), 1);
    assert_eq!(steps[0].blocks[0].kind, "text");
    assert_eq!(app.snapshot().expect("projection"), parent, "a block edit never touches the parent document");
}

#[semio_framework_async_macros::async_test]
async fn add_block_into_a_named_step_and_a_missing_one() {
    let mut app = playbook_app().await;
    dispatch(&mut app, PlaybookCommand::AddStep(crate::editor::playbook::commands::add_step::AddStep {})).await;
    let second = live_spec(&app).await.steps[1].id.clone();
    dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "number".into(), step_id: Some(second.clone()) })).await;
    let steps = live_spec(&app).await.steps;
    assert!(steps[0].blocks.is_empty());
    assert_eq!(steps[1].blocks[0].kind, "number");
}

/// 🕹️ A deleted block's id, if selected, is pruned by the framework's own `revalidate_interaction_state_after_document_change`
/// against `interaction_topology` (`interaction_topology_covers_every_step_and_block`).
#[semio_framework_async_macros::async_test]
async fn remove_block_action_removes_it_from_the_document() {
    let mut app = playbook_app().await;
    dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None })).await;
    let steps = live_spec(&app).await.steps;
    let (step_id, block_id) = (steps[0].id.clone(), steps[0].blocks[0].id.clone());
    dispatch(&mut app, PlaybookCommand::RemoveBlock(RemoveBlock { step_id, block_id })).await;
    assert!(live_spec(&app).await.steps[0].blocks.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn move_block_relocates_between_steps() {
    let mut app = playbook_app().await;
    dispatch(&mut app, PlaybookCommand::AddStep(crate::editor::playbook::commands::add_step::AddStep {})).await;
    dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None })).await;
    let steps = live_spec(&app).await.steps;
    let (from_step_id, to_step_id, block_id) = (steps[0].id.clone(), steps[1].id.clone(), steps[0].blocks[0].id.clone());
    dispatch(&mut app, PlaybookCommand::MoveBlock(MoveBlock { block_id: block_id.clone(), from_step_id, to_step_id, index: 0 })).await;
    let steps = live_spec(&app).await.steps;
    assert!(steps[0].blocks.is_empty());
    assert_eq!(steps[1].blocks[0].id, block_id);
}
