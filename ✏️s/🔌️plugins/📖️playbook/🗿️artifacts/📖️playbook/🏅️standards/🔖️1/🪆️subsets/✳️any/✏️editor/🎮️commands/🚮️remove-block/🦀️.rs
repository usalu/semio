//! 🚮️ Playbook play app command — `remove-block`: removes one block from its step (one absolute `blocksJson` set on the `flow`
//! child). A selection naming it is pruned by the framework's `revalidate_interaction_state_after_document_change`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::playbook_child_leaves_emit;
use crate::op::PlaybookMutation;
use crate::{playbook_flow_content, playbook_remove_block_leaves, PlaybookSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-block")]
pub struct RemoveBlock {
    pub step_id: String,
    pub block_id: String,
}

pub fn handle(payload: &RemoveBlock, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    if payload.step_id.is_empty() || payload.block_id.is_empty() {
        return Ok(Emit::default());
    }
    let content = playbook_flow_content(doc.snapshot, &doc.children)?;
    Ok(playbook_child_leaves_emit(doc.snapshot, playbook_remove_block_leaves(&content, &payload.step_id, &payload.block_id)?))
}
