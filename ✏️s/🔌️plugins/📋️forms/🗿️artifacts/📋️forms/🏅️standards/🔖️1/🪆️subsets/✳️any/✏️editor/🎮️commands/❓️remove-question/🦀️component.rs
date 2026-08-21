//! ❓️ ❓️ Forms play app commands command — `remove-question`.

use crate::artifacts::forms::schema::locate_question;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-question")]
pub struct RemoveQuestion {
    pub question_id: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer prunes the deleted question
// out of a config-owned selection list here — the framework's own `revalidate_interaction_state_after_
// document_change` prunes the "fields" domain's selection against `interaction_topology` after every
// document dispatch, so a deleted question's id is pruned automatically.
pub async fn handle(payload: &RemoveQuestion, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
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
