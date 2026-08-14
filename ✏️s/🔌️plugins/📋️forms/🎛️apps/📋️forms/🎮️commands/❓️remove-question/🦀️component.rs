//! ❓️ ❓️ Forms play app commands command — `remove-question`.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::{parse_value_json, reset_try_config_mutations};
use crate::artifacts::forms::schema::{create_form_id, locate_question, update_block_operation, value_to_dsl};
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormQuestion, FormsSnapshot, FormVectorField};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-question")]
pub struct RemoveQuestion {
    pub question_id: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the deleted question
// out of a config-owned selection list here — the framework's own `revalidate_interaction_state_after_
// document_change` prunes the "fields" domain's selection against `interaction_topology` after every
// document dispatch, so a deleted question's id is pruned automatically.
pub fn handle(payload: &RemoveQuestion, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let Some(location) = locate_question(spec, &payload.question_id) else {
        return Ok(Emit::default());
    };
    Ok(Emit {
        artifact_mutations: vec![FormMutation::DeleteBlock(crate::artifacts::forms::mutations::delete_block::mutation::DeleteBlock { step_id: location.step_id, id: payload.question_id.clone() })],
        config_mutations: reset_try_config_mutations(),
        ..Default::default()
    })
}
