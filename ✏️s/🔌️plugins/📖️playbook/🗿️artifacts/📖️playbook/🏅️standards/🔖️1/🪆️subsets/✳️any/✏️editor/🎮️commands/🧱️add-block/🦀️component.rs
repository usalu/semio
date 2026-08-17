//! 🧱️ 🧱️ Playbook play app commands command — `add-block`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::schema::default_block;
use crate::artifacts::playbook::op::{add_block_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    let spec = doc.snapshot;
    let steps = spec.steps();
    let Some(step_id) = payload.step_id.clone().or_else(|| steps.first().map(|step| step.id.clone())) else {
        return Ok(Emit::default());
    };
    let block_id = format!("block-{}", steps.iter().map(|step| step.blocks.len()).sum::<usize>() + 1);
    Ok(Emit { artifact_mutations: vec![add_block_operation(&step_id, default_block(block_id, &payload.kind), None)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use AddBlock;
    use crate::editor::playbook::testkit::{dispatch, playbook_app, playbook_app_with_registry};
    use crate::editor::playbook::PlaybookCommand;
    use crate::editor::playbook::commands::move_block::MoveBlock;
    use crate::editor::playbook::commands::remove_block::RemoveBlock;

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new block is no longer
    /// auto-selected by this command (selection is framework-owned now) — only the document edit itself.
    #[test]
    fn add_block_action_appends_block() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let projection = app.snapshot().expect("projection");
        let steps = projection.steps();
        assert_eq!(steps[0].blocks.len(), 1);
        assert_eq!(steps[0].blocks[0].kind, "text");
    }

    #[test]
    fn add_block_materializes_declared_kind_default() {
        let mut app = playbook_app_with_registry();
        dispatch(&mut app, PlaybookCommand::AddStep(crate::editor::playbook::commands::add_step::AddStep {}));
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let projection = app.snapshot().expect("materialize projection");
        assert_eq!(projection.steps()[0].blocks.last().unwrap().kind, "text", "kind default materialized from the registry");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no config-owned selection to check
    /// anymore — a deleted block's id, if selected, is pruned by the framework's own
    /// `revalidate_interaction_state_after_document_change` against `interaction_topology`, covered by
    /// `interaction_topology_covers_every_step_and_block` in the app root's own tests.
    #[test]
    fn remove_block_action_removes_it_from_the_document() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let steps = app.snapshot().expect("projection").steps();
        let step_id = steps[0].id.clone();
        let block_id = steps[0].blocks[0].id.clone();
        dispatch(&mut app, PlaybookCommand::RemoveBlock(RemoveBlock { step_id, block_id }));
        assert!(app.snapshot().expect("projection").steps()[0].blocks.is_empty());
    }

    #[test]
    fn move_block_relocates_between_steps() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddStep(crate::editor::playbook::commands::add_step::AddStep {}));
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let projection = app.snapshot().expect("projection");
        let steps = projection.steps();
        let from_step_id = steps[0].id.clone();
        let to_step_id = steps[1].id.clone();
        let block_id = steps[0].blocks[0].id.clone();
        dispatch(&mut app, PlaybookCommand::MoveBlock(MoveBlock { block_id: block_id.clone(), from_step_id, to_step_id, index: 0 }));
        let projection = app.snapshot().expect("projection");
        let steps = projection.steps();
        assert!(steps[0].blocks.is_empty());
        assert_eq!(steps[1].blocks[0].id, block_id);
    }
}
//#endregion 🧪️Tests
