//! 📃️ 📃️ Forms play app commands command — `update-form`.

use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "update-form")]
pub struct UpdateForm {
    pub title: String,
}

pub async fn handle(payload: &UpdateForm, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit {
        artifact_mutations: vec![FormMutation::ChangeFormTitle(crate::artifacts::forms::mutations::change_form_title::mutation::ChangeFormTitle { new_title: Some(payload.title.clone()).filter(|title| !title.is_empty()) })],
        coalesce_key: Some("change-form-title".into()),
        ..Default::default()
    })
}
