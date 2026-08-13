//! 🪜️ 🪜️ Playbook play app commands command — `update-playbook`.

use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::artifacts::playbook::op::{add_step_operation, change_title_operation, move_step_operation, remove_step_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "update-playbook")]
pub struct UpdatePlaybook {
    pub value: String,
}

pub fn handle(payload: &UpdatePlaybook, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    Ok(Emit::amend(vec![change_title_operation(Some(payload.value.clone()).filter(|title| !title.is_empty()))], "playbook.title"))
}
