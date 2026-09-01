//! 🪜️ 🪜️ Playbook play app commands command — `update-playbook`.

use crate::artifacts::playbook::op::{change_title_operation, PlaybookMutation};
use crate::artifacts::playbook::PlaybookSnapshot;
use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "update-playbook")]
pub struct UpdatePlaybook {
    pub value: String,
}

pub fn handle(payload: &UpdatePlaybook, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    Ok(Emit::amend(vec![change_title_operation(Some(payload.value.clone()).filter(|title| !title.is_empty()))], "playbook.title"))
}
