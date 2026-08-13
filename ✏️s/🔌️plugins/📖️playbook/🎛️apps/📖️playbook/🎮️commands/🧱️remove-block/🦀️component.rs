//! 🧱️ 🧱️ Playbook play app commands command — `remove-block`.

use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::schema::default_block;
use crate::artifacts::playbook::op::{add_block_operation, move_block_operation, remove_block_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

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
