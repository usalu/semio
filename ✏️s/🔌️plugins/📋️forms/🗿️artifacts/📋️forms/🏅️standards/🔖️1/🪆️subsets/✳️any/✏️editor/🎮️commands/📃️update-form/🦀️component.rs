//! 📃️ 📃️ Forms play app commands command — `update-form`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use crate::artifacts::forms::schema::create_form_id;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot, FormStep};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "update-form")]
pub struct UpdateForm {
    pub title: String,
}

pub fn handle(payload: &UpdateForm, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit {
        artifact_mutations: vec![FormMutation::ChangeFormTitle(crate::artifacts::forms::mutations::change_form_title::mutation::ChangeFormTitle { new_title: Some(payload.title.clone()).filter(|title| !title.is_empty()) })],
        coalesce_key: Some("change-form-title".into()),
        ..Default::default()
    })
}
