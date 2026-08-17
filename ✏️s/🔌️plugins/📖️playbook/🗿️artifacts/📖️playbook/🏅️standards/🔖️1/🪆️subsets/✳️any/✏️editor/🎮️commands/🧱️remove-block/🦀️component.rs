//! 🧱️ 🧱️ Playbook play app commands command — `remove-block`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::op::{remove_block_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-block")]
pub struct RemoveBlock {
    pub step_id: String,
    pub block_id: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the deleted block out
// of a config-owned selection list here — the framework's own `revalidate_interaction_state_after_
// document_change` prunes the "blocks" domain's selection against `interaction_topology` after every
// document dispatch, so a deleted block's id is pruned automatically.
pub fn handle(payload: &RemoveBlock, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    if payload.step_id.is_empty() || payload.block_id.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![remove_block_operation(&payload.step_id, &payload.block_id)], ..Default::default() })
}
