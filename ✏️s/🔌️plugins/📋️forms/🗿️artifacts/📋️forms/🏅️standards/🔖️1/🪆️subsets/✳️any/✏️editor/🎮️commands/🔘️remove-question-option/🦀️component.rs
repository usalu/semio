//! 🔘️ 🔘️ Forms play app commands command — `remove-question-option`.

use crate::artifacts::forms::schema::update_block_operation;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shell
async fn remove_question_option(spec: &FormsSnapshot, question_id: &str, option_value: &str) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.retain(|entry| entry.value != option_value);
        question.options = Some(options);
    })
}
//#endregion 🔖️Shell

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-question-option")]
pub struct RemoveQuestionOption {
    pub question_id: String,
    pub option_value: String,
}

pub async fn handle(payload: &RemoveQuestionOption, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    match remove_question_option(doc.snapshot, &payload.question_id, &payload.option_value) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}
