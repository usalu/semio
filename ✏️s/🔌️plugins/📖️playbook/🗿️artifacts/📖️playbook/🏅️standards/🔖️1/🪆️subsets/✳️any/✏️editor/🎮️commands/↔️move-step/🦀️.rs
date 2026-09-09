//! 🪜️ 🪜️ Playbook play app commands command — `move-step`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::op::{move_step_operation, PlaybookMutation};
use crate::PlaybookSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
