//! 📃️ 📃️ Forms play app commands command — `move-step`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub step_id: String,
    pub index: u64,
}

pub fn handle(payload: &MoveStep, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    if payload.step_id.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit {
        artifact_mutations: vec![FormMutation::ReorderStep(crate::artifacts::forms::mutations::reorder_step::mutation::ReorderStep { id: payload.step_id.clone(), to_index: payload.index as usize })],
        config_mutations: reset_try_config_mutations(),
        ..Default::default()
    })
}
