//! 📃️ 📃️ Forms play app commands command — `remove-step`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub step_id: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the removed step's
// questions out of a config-owned selection list here — the framework's own
// `revalidate_interaction_state_after_document_change` prunes the "fields" domain's selection against
// `interaction_topology` after every document dispatch, so deleted ids are pruned automatically.
pub async fn handle(payload: &RemoveStep, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    if payload.step_id.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![FormMutation::DeleteStep(crate::artifacts::forms::mutations::delete_step::mutation::DeleteStep { id: payload.step_id.clone() })], config_mutations: reset_try_config_mutations(), ..Default::default() })
}
