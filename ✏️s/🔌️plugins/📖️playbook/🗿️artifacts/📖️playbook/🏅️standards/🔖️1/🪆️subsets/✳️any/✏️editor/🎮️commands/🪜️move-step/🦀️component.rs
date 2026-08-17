//! 🪜️ 🪜️ Playbook play app commands command — `move-step`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::op::{move_step_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub step_id: String,
    pub index: usize,
}

pub fn handle(payload: &MoveStep, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    if payload.step_id.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![move_step_operation(&payload.step_id, payload.index)]))
}
