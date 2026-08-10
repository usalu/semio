//! 🧱️ Playbook play app commands — block lifecycle (add / remove / move) within a step.

use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::engine::default_block;
use crate::artifacts::playbook::op::{add_block_operation, move_block_operation, remove_block_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddBlock
pub mod add_block {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-block")]
    pub struct AddBlock {
        pub kind: String,
        pub step_id: Option<String>,
    }

    pub fn handle(payload: &AddBlock, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
        let spec = doc.snapshot;
        let Some(step_id) = payload.step_id.clone().or_else(|| spec.steps.first().map(|step| step.id.clone())) else {
            return Ok(Emit::default());
        };
        let block_id = format!("block-{}", spec.steps.iter().map(|step| step.blocks.len()).sum::<usize>() + 1);
        Ok(Emit { artifact_mutations: vec![add_block_operation(&step_id, default_block(block_id.clone(), &payload.kind), None)], config_mutations: vec![PlaybookConfigMutation::SetSelectedIds { ids: vec![block_id] }], ..Default::default() })
    }
}
//#endregion 🔖️AddBlock

//#region 🔖️RemoveBlock
pub mod remove_block {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-block")]
    pub struct RemoveBlock {
        pub step_id: String,
        pub block_id: String,
    }

    pub fn handle(payload: &RemoveBlock, _doc: &ArtifactView<'_, PlaybookSnapshot>, cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
        if payload.step_id.is_empty() || payload.block_id.is_empty() {
            return Ok(Emit::default());
        }
        let config = cfg.snapshot;
        let remaining: Vec<String> = config.selected_ids.iter().filter(|id| **id != payload.block_id).cloned().collect();
        Ok(Emit { artifact_mutations: vec![remove_block_operation(&payload.step_id, &payload.block_id)], config_mutations: vec![PlaybookConfigMutation::SetSelectedIds { ids: remaining }], ..Default::default() })
    }
}
//#endregion 🔖️RemoveBlock

//#region 🔖️MoveBlock
pub mod move_block {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-block")]
    pub struct MoveBlock {
        pub block_id: String,
        pub from_step_id: String,
        pub to_step_id: String,
        pub index: usize,
    }

    pub fn handle(payload: &MoveBlock, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![move_block_operation(&payload.block_id, &payload.from_step_id, &payload.to_step_id, payload.index)]))
    }
}
//#endregion 🔖️MoveBlock

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use add_block::AddBlock;
    use crate::apps::playbook::testkit::{dispatch, playbook_app, playbook_app_with_registry};
    use crate::apps::playbook::PlaybookCommand;
    use move_block::MoveBlock;
    use remove_block::RemoveBlock;

    #[test]
    fn add_block_action_appends_and_selects_block() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let projection = app.snapshot().expect("projection");
        assert_eq!(projection.steps[0].blocks.len(), 1);
        assert_eq!(projection.steps[0].blocks[0].kind, "text");
    }

    #[test]
    fn add_block_materializes_declared_kind_default() {
        let mut app = playbook_app_with_registry();
        dispatch(&mut app, PlaybookCommand::AddStep(crate::apps::playbook::commands::step::add_step::AddStep {}));
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let projection = app.snapshot().expect("materialize projection");
        assert_eq!(projection.steps[0].blocks.last().unwrap().kind, "text", "kind default materialized from the registry");
    }

    #[test]
    fn remove_block_clears_it_from_selection() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let step_id = app.snapshot().expect("projection").steps[0].id.clone();
        let block_id = app.snapshot().expect("projection").steps[0].blocks[0].id.clone();
        dispatch(&mut app, PlaybookCommand::RemoveBlock(RemoveBlock { step_id, block_id }));
        assert!(app.snapshot().expect("projection").steps[0].blocks.is_empty());
    }

    #[test]
    fn move_block_relocates_between_steps() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddStep(crate::apps::playbook::commands::step::add_step::AddStep {}));
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let projection = app.snapshot().expect("projection");
        let from_step_id = projection.steps[0].id.clone();
        let to_step_id = projection.steps[1].id.clone();
        let block_id = projection.steps[0].blocks[0].id.clone();
        dispatch(&mut app, PlaybookCommand::MoveBlock(MoveBlock { block_id: block_id.clone(), from_step_id, to_step_id, index: 0 }));
        let projection = app.snapshot().expect("projection");
        assert!(projection.steps[0].blocks.is_empty());
        assert_eq!(projection.steps[1].blocks[0].id, block_id);
    }
}
//#endregion 🧪️Tests
