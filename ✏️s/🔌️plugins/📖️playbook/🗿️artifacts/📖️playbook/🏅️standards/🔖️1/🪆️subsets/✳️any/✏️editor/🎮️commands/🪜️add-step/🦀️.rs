//! 🪜️ Playbook play app command — `add-step`: appends one empty step to the `flow` child (node + chain edge, one child edit).

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::{playbook_child_leaves_emit, playbook_minted_id};
use crate::op::PlaybookMutation;
use crate::{playbook_add_step_leaves, playbook_flow_content, playbook_step_order, PlaybookSnapshot, PlaybookStep};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-step")]
pub struct AddStep {}

pub fn handle(_payload: &AddStep, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let content = playbook_flow_content(doc.snapshot, &doc.children)?;
    let step = PlaybookStep { id: playbook_minted_id(doc, "step")?, title: format!("Step {}", playbook_step_order(&content).len() + 1), description: None, blocks: Vec::new() };
    Ok(playbook_child_leaves_emit(doc.snapshot, playbook_add_step_leaves(&content, &step)?))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
