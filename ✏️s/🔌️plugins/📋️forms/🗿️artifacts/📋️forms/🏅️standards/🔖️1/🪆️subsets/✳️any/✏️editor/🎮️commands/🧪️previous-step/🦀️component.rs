//! 🧪️ 🧪️ Forms play app commands command — `previous-step`.

use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "previous-step")]
pub struct PreviousStep {}

pub async fn handle(_payload: &PreviousStep, _doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::config(vec![FormsConfigMutation::SetStepIndex { index: cfg.snapshot.current_step_index.saturating_sub(1) }]))
}
