//! 🧱️ 🧱️ Playbook play app commands command — `add-block`.

use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::schema::default_block;
use crate::artifacts::playbook::op::{add_block_operation, move_block_operation, remove_block_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub kind: String,
    pub step_id: Option<String>,
}

pub fn handle(payload: &AddBlock, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let steps = spec.steps();
    let Some(step_id) = payload.step_id.clone().or_else(|| steps.first().map(|step| step.id.clone())) else {
        return Ok(Emit::default());
    };
    let block_id = format!("block-{}", steps.iter().map(|step| step.blocks.len()).sum::<usize>() + 1);
    Ok(Emit { artifact_mutations: vec![add_block_operation(&step_id, default_block(block_id.clone(), &payload.kind), None)], config_mutations: vec![PlaybookConfigMutation::SetSelectedIds { ids: vec![block_id] }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use AddBlock;
    use crate::apps::playbook::testkit::{dispatch, playbook_app, playbook_app_with_registry};
    use crate::apps::playbook::PlaybookCommand;
    use move_block::MoveBlock;
    use remove_block::RemoveBlock;

    #[test]
    fn add_block_action_appends_and_selects_block() {
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
        dispatch(&mut app, PlaybookCommand::AddStep(crate::apps::playbook::commands::add_step::AddStep {}));
        dispatch(&mut app, PlaybookCommand::AddBlock(AddBlock { kind: "text".into(), step_id: None }));
        let projection = app.snapshot().expect("materialize projection");
        assert_eq!(projection.steps()[0].blocks.last().unwrap().kind, "text", "kind default materialized from the registry");
    }

    #[test]
    fn remove_block_clears_it_from_selection() {
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
        dispatch(&mut app, PlaybookCommand::AddStep(crate::apps::playbook::commands::add_step::AddStep {}));
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
