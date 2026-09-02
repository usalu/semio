//! 📃️ 📃️ Forms play app commands command — `patch-step`.

use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-step")]
pub struct PatchStep {
    pub step_id: String,
    pub field: String,
    pub value: String,
}

/// ✏️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: `FormMutation` has no whole-step-replace
/// variant (the old `UpdateStep{step}` is banned `SetSnapshot`-shaped vocabulary at the
/// per-collection scale) — emits the granular `RenameStep`/`ChangeStepDescription` verb the field
/// actually maps onto instead of building a whole replacement `FormStep`.
pub async fn handle(payload: &PatchStep, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    if !forms_steps(spec).iter().any(|step| step.id == payload.step_id) {
        return Ok(Emit::default());
    }
    let mutation = match payload.field.as_str() {
        "title" => FormMutation::RenameStep(crate::artifacts::forms::mutations::rename_step::mutation::RenameStep { id: payload.step_id.clone(), new_title: payload.value.clone() }),
        "description" => FormMutation::ChangeStepDescription(crate::artifacts::forms::mutations::change_step_description::mutation::ChangeStepDescription {
            id: payload.step_id.clone(),
            new_description: Some(payload.value.clone()).filter(|description| !description.is_empty()),
        }),
        _ => return Ok(Emit::default()),
    };
    Ok(Emit { artifact_mutations: vec![mutation], config_mutations: reset_try_config_mutations(), coalesce_key: Some(format!("patch-step:{}:{}", payload.step_id, payload.field)), ..Default::default() })
}
