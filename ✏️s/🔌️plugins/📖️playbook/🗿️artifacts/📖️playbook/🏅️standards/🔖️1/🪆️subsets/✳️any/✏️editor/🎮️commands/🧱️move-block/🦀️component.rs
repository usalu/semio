//! 🧱️ 🧱️ Playbook play app commands command — `move-block`.

use crate::artifacts::playbook::op::{move_block_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-block")]
pub struct MoveBlock {
    pub block_id: String,
    pub from_step_id: String,
    pub to_step_id: String,
    pub index: usize,
}

pub fn handle(payload: &MoveBlock, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![move_block_operation(&payload.block_id, &payload.from_step_id, &payload.to_step_id, payload.index)]))
}
