//! 🔘️ Forms play app commands — question option lifecycle (patch / add / remove), for `single`/`multi`
//! question kinds.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::parse_value_json;
use crate::artifacts::forms::engine::{create_form_id, update_block_operation};
use crate::artifacts::forms::{op::FormMutation, FormQuestionOption, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shell
fn patch_question_option(spec: &FormsSnapshot, question_id: &str, option_value: &str, field: &str, raw_value: &Value) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        if let Some(option) = options.iter_mut().find(|entry| entry.value == option_value) {
            if field == "label" {
                option.label = raw_value.as_str().unwrap_or("").to_string();
            }
        }
        question.options = Some(options);
    })
}

fn add_question_option(spec: &FormsSnapshot, question_id: &str, label: &str) -> Option<FormMutation> {
    let value = create_form_id("opt");
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.push(FormQuestionOption { value, label: label.into() });
        question.options = Some(options);
    })
}

fn remove_question_option(spec: &FormsSnapshot, question_id: &str, option_value: &str) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.retain(|entry| entry.value != option_value);
        question.options = Some(options);
    })
}
//#endregion 🔖️Shell

//#region 🔖️PatchQuestionOptions
pub mod patch_question_options {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-question-options")]
    pub struct PatchQuestionOptions {
        pub question_ids: Vec<String>,
        pub option_value: String,
        pub field: String,
        pub value_json: String,
    }

    pub fn handle(payload: &PatchQuestionOptions, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let spec = doc.snapshot;
        let raw_value = parse_value_json(&payload.value_json);
        let operations: Vec<FormMutation> = payload.question_ids.iter().filter_map(|question_id| patch_question_option(spec, question_id, &payload.option_value, &payload.field, &raw_value)).collect();
        if operations.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::amend(operations, format!("patch-option:{}:{}", payload.option_value, payload.field)))
    }
}
//#endregion 🔖️PatchQuestionOptions

//#region 🔖️AddQuestionOption
pub mod add_question_option {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-question-option")]
    pub struct AddQuestionOption {
        pub question_id: String,
        pub label: String,
    }

    pub fn handle(payload: &AddQuestionOption, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        match add_question_option(doc.snapshot, &payload.question_id, &payload.label) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️AddQuestionOption

//#region 🔖️RemoveQuestionOption
pub mod remove_question_option {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-question-option")]
    pub struct RemoveQuestionOption {
        pub question_id: String,
        pub option_value: String,
    }

    pub fn handle(payload: &RemoveQuestionOption, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        match remove_question_option(doc.snapshot, &payload.question_id, &payload.option_value) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RemoveQuestionOption

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app};
    use crate::apps::forms::FormsCommand;
    use add_question_option::AddQuestionOption;
    use remove_question_option::RemoveQuestionOption;

    fn single_or_multi_question_id(app: &mut crate::apps::forms::testkit::FormsApp) -> String {
        dispatch(app, FormsCommand::AddQuestion(crate::apps::forms::commands::question::add_question::AddQuestion { kind: "single".into(), step_id: None }));
        crate::artifacts::forms::engine::flatten_questions(&app.snapshot().expect("projection")).into_iter().map(|(_, question)| question).find(|question| question.kind == "single").expect("single question").id
    }

    #[test]
    fn add_and_remove_question_option_round_trip() {
        let mut app = forms_app();
        let question_id = single_or_multi_question_id(&mut app);
        dispatch(&mut app, FormsCommand::AddQuestionOption(AddQuestionOption { question_id: question_id.clone(), label: "New option".into() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::engine::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        let added = question.options.as_ref().expect("options").iter().find(|option| option.label == "New option").expect("added option").value.clone();
        dispatch(&mut app, FormsCommand::RemoveQuestionOption(RemoveQuestionOption { question_id: question_id.clone(), option_value: added.clone() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::engine::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        assert!(question.options.as_ref().expect("options").iter().all(|option| option.value != added));
    }
}
//#endregion 🧪️Tests
