//! 📃️ 📃️ Forms play app commands command — `remove-step`.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::reset_try_config_mutations;
use crate::artifacts::forms::schema::create_form_id;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot, FormStep};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub step_id: String,
}

pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    if payload.step_id.is_empty() {
        return Ok(Emit::default());
    }
    let spec = doc.snapshot;
    let config = cfg.snapshot;
    let removed_ids: Vec<String> = forms_steps(spec).iter().filter(|step| step.id == payload.step_id).flat_map(|step| step.blocks.iter().map(|question| question.id.clone())).collect();
    let mut config_mutations = reset_try_config_mutations();
    config_mutations.push(FormsConfigMutation::SetSelection { ids: config.selected_ids.iter().filter(|id| !removed_ids.contains(id)).cloned().collect() });
    Ok(Emit { artifact_mutations: vec![FormMutation::DeleteStep(crate::artifacts::forms::mutations::delete_step::mutation::DeleteStep { id: payload.step_id.clone() })], config_mutations, ..Default::default() })
}
