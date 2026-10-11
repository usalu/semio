//! 🔘️ 🔘️ Forms play app commands command — `add-question-option`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::schema::{create_form_id, update_block_operations};
use crate::{op::FormMutation, FormQuestionOption, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shell
fn add_question_option(spec: &FormsSnapshot, question_id: &str, label: &str) -> Option<Vec<FormMutation>> {
    let value = create_form_id("opt");
    update_block_operations(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.push(FormQuestionOption { value, label: label.into() });
        question.options = Some(options);
    })
}
//#endregion 🔖️Shell

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-question-option")]
pub struct AddQuestionOption {
    pub question_id: String,
    pub label: String,
}

pub fn handle(payload: &AddQuestionOption, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    match add_question_option(doc.snapshot, &payload.question_id, &payload.label) {
        Some(operations) => Ok(Emit::mutations(operations)),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
