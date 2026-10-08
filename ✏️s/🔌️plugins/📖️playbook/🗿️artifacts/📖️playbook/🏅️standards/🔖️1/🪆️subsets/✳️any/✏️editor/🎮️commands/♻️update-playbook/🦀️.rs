//! 🪜️ 🪜️ Playbook play app commands command — `update-playbook`.

use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::schema::mutations::{change_title_operation, PlaybookMutation};
use crate::PlaybookSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "update-playbook")]
pub struct UpdatePlaybook {
    pub value: String,
}

pub fn handle(payload: &UpdatePlaybook, _doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![change_title_operation(Some(payload.value.clone()).filter(|title| !title.is_empty()))]))
}
