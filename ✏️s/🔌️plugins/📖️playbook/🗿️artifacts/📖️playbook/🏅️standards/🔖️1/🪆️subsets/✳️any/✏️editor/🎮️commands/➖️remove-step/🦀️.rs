//! ➖️ Playbook play app command — `remove-step`: removes one step from the `flow` child, bridging the chain over it.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::playbook_child_leaves_emit;
use crate::op::PlaybookMutation;
use crate::{playbook_flow_content, playbook_remove_step_leaves, PlaybookSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub step_id: String,
}

pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    if payload.step_id.is_empty() {
        return Ok(Emit::default());
    }
    let content = playbook_flow_content(doc.snapshot, &doc.children)?;
    Ok(playbook_child_leaves_emit(doc.snapshot, playbook_remove_step_leaves(&content, &payload.step_id)?))
}
