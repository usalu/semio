//! 🚚️ Playbook play app command — `move-block`: moves one block within or across steps (one `remove-node-param` and one position-exact
//! `set-node-param` row, one `flow` child edit).

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::playbook_child_leaves_emit;
use crate::op::PlaybookMutation;
use crate::{playbook_flow_content, playbook_move_block_leaves, PlaybookSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "move-block")]
pub struct MoveBlock {
    pub block_id: String,
    pub from_step_id: String,
    pub to_step_id: String,
    pub index: usize,
}

pub fn handle(payload: &MoveBlock, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let content = playbook_flow_content(doc.snapshot, &doc.children)?;
    Ok(playbook_child_leaves_emit(doc.snapshot, playbook_move_block_leaves(&content, &payload.block_id, &payload.from_step_id, &payload.to_step_id, payload.index)?))
}
