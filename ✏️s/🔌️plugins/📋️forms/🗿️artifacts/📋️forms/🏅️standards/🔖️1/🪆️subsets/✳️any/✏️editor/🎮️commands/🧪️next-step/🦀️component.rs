//! 🧪️ 🧪️ Forms play app commands command — `next-step`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::effective_try_values;
use crate::artifacts::forms::schema::can_advance;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "next-step")]
pub struct NextStep {}

pub async fn handle(_payload: &NextStep, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let config = cfg.snapshot;
    let index = config.current_step_index as usize;
    let steps = forms_steps(spec);
    if index + 1 < steps.len() {
        let step = &steps[index];
        let values = effective_try_values(spec, config);
        if can_advance(step, &values) {
            return Ok(Emit::config(vec![FormsConfigMutation::SetStepIndex { index: config.current_step_index + 1 }]));
        }
    }
    Ok(Emit::default())
}
