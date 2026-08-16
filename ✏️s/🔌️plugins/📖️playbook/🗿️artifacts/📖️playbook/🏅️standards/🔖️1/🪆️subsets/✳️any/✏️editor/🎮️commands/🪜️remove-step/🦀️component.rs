//! 🪜️ 🪜️ Playbook play app commands command — `remove-step`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::op::{add_step_operation, change_title_operation, move_step_operation, remove_step_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub step_id: String,
}

pub fn handle(payload: &RemoveStep, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    if payload.step_id.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![remove_step_operation(&payload.step_id)]))
}
