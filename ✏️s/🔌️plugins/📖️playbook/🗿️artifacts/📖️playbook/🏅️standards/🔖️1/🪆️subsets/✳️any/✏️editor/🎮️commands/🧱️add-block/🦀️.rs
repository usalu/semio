//! 🧱️ Playbook play app command — `add-block`: appends a blank block of a kind to a step (one concrete `set-node-param` row on the
//! `flow` child).

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::{playbook_child_leaves_emit, playbook_minted_id};
use crate::op::PlaybookMutation;
use crate::schema::default_block;
use crate::{playbook_add_block_leaves, playbook_flow_content, playbook_step_order, PlaybookSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub kind: String,
    pub step_id: Option<String>,
}

/// 🧱️ The block lands in the named step, else in the first step of the chain. Selection is framework-owned `InteractionState`
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), never set by an app command's `Emit`.
pub fn handle(payload: &AddBlock, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let content = playbook_flow_content(doc.snapshot, &doc.children)?;
    let step_id = match payload.step_id.as_deref().filter(|value| !value.is_empty()) {
        Some(step_id) => step_id.to_string(),
        None => playbook_step_order(&content).first().map(|id| id.to_string()).unwrap_or_default(),
    };
    let block = default_block(playbook_minted_id(doc, "block")?, &payload.kind);
    Ok(playbook_child_leaves_emit(doc.snapshot, playbook_add_block_leaves(&content, &step_id, block, None)?))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
