//! 📐️ 📐️ Forms play app commands command — `patch-vector-field`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::parse_value_json;
use crate::artifacts::forms::schema::update_block_operation;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shell
async fn patch_vector_field(spec: &FormsSnapshot, question_id: &str, field_key: &str, field: &str, raw_value: &Value) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        if let Some(entry) = fields.iter_mut().find(|item| item.key == field_key) {
            match field {
                "label" => entry.label = raw_value.as_str().map(str::to_string),
                "value" => entry.value = raw_value.as_f64(),
                _ => {}
            }
        }
        question.fields = Some(fields);
    })
}
//#endregion 🔖️Shell




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-vector-field")]
pub struct PatchVectorField {
    pub question_id: String,
    pub field_key: String,
    pub field: String,
    pub value_json: String,
}

pub async fn handle(payload: &PatchVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let raw_value = parse_value_json(&payload.value_json);
    match patch_vector_field(doc.snapshot, &payload.question_id, &payload.field_key, &payload.field, &raw_value) {
        Some(operation) => Ok(Emit::amend(vec![operation], format!("patch-vector:{}:{}:{}", payload.question_id, payload.field_key, payload.field))),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{dispatch, forms_app};
    use crate::editor::forms::FormsCommand;
    use crate::editor::forms::commands::add_vector_field::AddVectorField;
    use PatchVectorField;
    use crate::editor::forms::commands::remove_vector_field::RemoveVectorField;

    async fn vector_question_id(app: &mut crate::editor::forms::testkit::FormsApp) -> String {
        dispatch(app, FormsCommand::AddQuestion(crate::editor::forms::commands::add_question::AddQuestion { kind: "vector".into(), step_id: None }));
        crate::artifacts::forms::schema::flatten_questions(&app.snapshot().expect("projection")).into_iter().map(|(_, question)| question).find(|question| question.kind == "vector").expect("vector question").id
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_vector_field_updates_the_named_component() {
        let mut app = forms_app();
        let question_id = vector_question_id(&mut app);
        dispatch(&mut app, FormsCommand::PatchVectorField(PatchVectorField { question_id: question_id.clone(), field_key: "x".into(), field: "value".into(), value_json: "5.0".into() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        let x = question.fields.as_ref().expect("fields").iter().find(|field| field.key == "x").expect("x field");
        assert_eq!(x.value, Some(5.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_and_remove_vector_field_round_trip() {
        let mut app = forms_app();
        let question_id = vector_question_id(&mut app);
        dispatch(&mut app, FormsCommand::AddVectorField(AddVectorField { question_id: question_id.clone(), field_key: "w".into() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        assert!(question.fields.as_ref().expect("fields").iter().any(|field| field.key == "w"));
        dispatch(&mut app, FormsCommand::RemoveVectorField(RemoveVectorField { question_id: question_id.clone(), field_key: "w".into() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        assert!(question.fields.as_ref().expect("fields").iter().all(|field| field.key != "w"));
    }
}
//#endregion 🧪️Tests
