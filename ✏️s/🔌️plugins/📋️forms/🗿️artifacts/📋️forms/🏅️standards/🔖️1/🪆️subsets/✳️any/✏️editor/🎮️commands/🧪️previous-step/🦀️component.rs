//! 🧪️ 🧪️ Forms play app commands command — `previous-step`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::{effective_try_values, parse_value_json, reset_try_config_mutations, try_values_json_text, try_values_map};
use crate::artifacts::forms::schema::can_advance;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "previous-step")]
pub struct PreviousStep {}

pub fn handle(_payload: &PreviousStep, _doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::config(vec![FormsConfigMutation::SetStepIndex { index: cfg.snapshot.current_step_index.saturating_sub(1) }]))
}
