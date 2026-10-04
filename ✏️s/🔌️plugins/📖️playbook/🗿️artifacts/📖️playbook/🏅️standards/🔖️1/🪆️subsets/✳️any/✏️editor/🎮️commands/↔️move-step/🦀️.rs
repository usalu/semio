//! ↔️ Playbook play app command — `move-step`: moves one step by rewiring the `flow` child's chain (node identity kept).

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::playbook_child_leaves_emit;
use crate::op::PlaybookMutation;
use crate::{playbook_flow_content, playbook_move_step_leaves, PlaybookSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub step_id: String,
    pub index: usize,
}

pub fn handle(payload: &MoveStep, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    if payload.step_id.is_empty() {
        return Ok(Emit::default());
    }
    let content = playbook_flow_content(doc.snapshot, &doc.children)?;
    Ok(playbook_child_leaves_emit(doc.snapshot, playbook_move_step_leaves(&content, &payload.step_id, payload.index)?))
}
