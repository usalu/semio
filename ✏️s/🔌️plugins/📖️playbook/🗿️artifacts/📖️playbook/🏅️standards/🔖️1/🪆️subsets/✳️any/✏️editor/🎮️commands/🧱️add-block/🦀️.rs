//! 🧱️ 🧱️ Playbook play app commands command — `add-block`.

use crate::op::{add_block_operation, PlaybookMutation};
use crate::schema::default_block;
use crate::PlaybookSnapshot;
use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub kind: String,
    pub step_id: Option<String>,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new block used to also become the
// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
// forms' `add-question`/note's `add-block`).
pub fn handle(payload: &AddBlock, doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    let step_id = payload.step_id.as_deref().filter(|value| !value.is_empty()).unwrap_or("s").to_string();
    let block_id = format!("block-op-{}", doc.operation()?.operation_id);
    Ok(Emit { artifact_mutations: vec![add_block_operation(&step_id, default_block(block_id, &payload.kind), None)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
