//! 🔘️ 🔘️ Forms play app commands command — `remove-question-option`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::schema::update_block_operations;
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shell
fn remove_question_option(spec: &FormsSnapshot, question_id: &str, option_value: &str) -> Option<Vec<FormMutation>> {
    update_block_operations(spec, question_id, |question| {
        *question = crate::editor::forms::questions::patch_choice(question, option_value, "remove", &semio_framework_pack_json::Value::Null).expect("removing a choice cannot fail");
    })
}
//#endregion 🔖️Shell

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-question-option")]
pub struct RemoveQuestionOption {
    pub question_id: String,
    pub option_value: String,
}

pub fn handle(payload: &RemoveQuestionOption, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    match remove_question_option(doc.snapshot, &payload.question_id, &payload.option_value) {
        Some(operations) => Ok(Emit::mutations(operations)),
        None => Ok(Emit::default()),
    }
}
