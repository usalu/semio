//! 🧱️ 🧱️ Playbook play app commands command — `add-block`.

use crate::artifacts::playbook::op::{add_block_operation, PlaybookMutation};
use crate::artifacts::playbook::schema::default_block;
use crate::artifacts::playbook::PlaybookSnapshot;
use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub kind: String,
    pub step_id: Option<String>,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new block used to also become the
// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
// forms' `add-question`/note's `add-block`).
pub fn handle(payload: &AddBlock, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let step_id = payload.step_id.as_deref().filter(|value| !value.is_empty()).unwrap_or("s").to_string();
    let block_id = format!("block-op-{}", doc.operation()?.operation_id);
    Ok(Emit { artifact_mutations: vec![add_block_operation(&step_id, default_block(block_id, &payload.kind), None)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::playbook::commands::move_block::MoveBlock;
    use crate::editor::playbook::commands::remove_block::RemoveBlock;
    use crate::editor::playbook::testkit::{dispatch, playbook_app, playbook_app_with_registry};
    use crate::editor::playbook::PlaybookCommand;
    use AddBlock;

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new block is no longer
    /// auto-selected by this command (selection is framework-owned now) — only the document edit itself.
    #[semio_framework_async_macros::async_test]
    async fn add_block_action_appends_block() {
        let mut app = playbook_app().await;
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None })).await;
        let projection = app.snapshot().expect("projection");
        let steps = projection.steps();
        assert_eq!(steps[0].blocks.len(), 1);
        assert_eq!(steps[0].blocks[0].kind, "text");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_block_materializes_declared_kind_default() {
        let mut app = playbook_app_with_registry().await;
        dispatch(&mut app, PlaybookCommand::AddStep(crate::editor::playbook::commands::add_step::AddStep {})).await;
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None })).await;
        let projection = app.snapshot().expect("materialize projection");
        assert_eq!(projection.steps()[0].blocks.last().unwrap().kind, "text", "kind default materialized from the registry");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no config-owned selection to check
    /// anymore — a deleted block's id, if selected, is pruned by the framework's own
    /// `revalidate_interaction_state_after_document_change` against `interaction_topology`, covered by
    /// `interaction_topology_covers_every_step_and_block` in the app root's own tests.
    #[semio_framework_async_macros::async_test]
    async fn remove_block_action_removes_it_from_the_document() {
        let mut app = playbook_app().await;
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None })).await;
        let steps = app.snapshot().expect("projection").steps();
        let step_id = steps[0].id.clone();
        let block_id = steps[0].blocks[0].id.clone();
        dispatch(&mut app, PlaybookCommand::RemoveBlock(RemoveBlock { step_id, block_id })).await;
        assert!(app.snapshot().expect("projection").steps()[0].blocks.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn move_block_relocates_between_steps() {
        let mut app = playbook_app().await;
        dispatch(&mut app, PlaybookCommand::AddStep(crate::editor::playbook::commands::add_step::AddStep {})).await;
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None })).await;
        let projection = app.snapshot().expect("projection");
        let steps = projection.steps();
        let from_step_id = steps[0].id.clone();
        let to_step_id = steps[1].id.clone();
        let block_id = steps[0].blocks[0].id.clone();
        dispatch(&mut app, PlaybookCommand::MoveBlock(MoveBlock { block_id: block_id.clone(), from_step_id, to_step_id, index: 0 })).await;
        let projection = app.snapshot().expect("projection");
        let steps = projection.steps();
        assert!(steps[0].blocks.is_empty());
        assert_eq!(steps[1].blocks[0].id, block_id);
    }
}
//#endregion 🧪️Tests
