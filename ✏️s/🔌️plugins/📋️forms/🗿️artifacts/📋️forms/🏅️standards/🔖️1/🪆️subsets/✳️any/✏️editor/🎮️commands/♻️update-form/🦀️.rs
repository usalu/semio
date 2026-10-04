//! 📃️ 📃️ Forms play app commands command — `update-form`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "update-form")]
pub struct UpdateForm {
    pub title: String,
}

pub fn handle(payload: &UpdateForm, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![FormMutation::ChangeFormTitle(crate::mutations::change_form_title::mutation::ChangeFormTitle { new_title: Some(payload.title.clone()).filter(|title| !title.is_empty()) })]))
}
