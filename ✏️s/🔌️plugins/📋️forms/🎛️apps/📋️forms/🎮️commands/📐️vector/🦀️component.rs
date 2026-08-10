//! 📐️ Forms play app commands — vector-field lifecycle (patch / add / remove), for the `vector` question
//! kind's per-component fields.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::parse_value_json;
use crate::artifacts::forms::engine::update_block_operation;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot, FormVectorField};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shell
fn patch_vector_field(spec: &FormsSnapshot, question_id: &str, field_key: &str, field: &str, raw_value: &Value) -> Option<FormMutation> {
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

fn add_vector_field(spec: &FormsSnapshot, question_id: &str, key: &str) -> Option<FormMutation> {
    let location = crate::artifacts::forms::engine::locate_question(spec, question_id)?;
    if location.question.fields.iter().flatten().any(|entry| entry.key == key) {
        return None;
    }
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.push(FormVectorField { key: key.into(), label: Some(key.into()), value: Some(0.0) });
        question.fields = Some(fields);
    })
}

fn remove_vector_field(spec: &FormsSnapshot, question_id: &str, field_key: &str) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.retain(|entry| entry.key != field_key);
        question.fields = Some(fields);
    })
}
//#endregion 🔖️Shell

//#region 🔖️PatchVectorField
pub mod patch_vector_field {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-vector-field")]
    pub struct PatchVectorField {
        pub question_id: String,
        pub field_key: String,
        pub field: String,
        pub value_json: String,
    }

    pub fn handle(payload: &PatchVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let raw_value = parse_value_json(&payload.value_json);
        match patch_vector_field(doc.snapshot, &payload.question_id, &payload.field_key, &payload.field, &raw_value) {
            Some(operation) => Ok(Emit::amend(vec![operation], format!("patch-vector:{}:{}:{}", payload.question_id, payload.field_key, payload.field))),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchVectorField

//#region 🔖️AddVectorField
pub mod add_vector_field {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-vector-field")]
    pub struct AddVectorField {
        pub question_id: String,
        pub field_key: String,
    }

    pub fn handle(payload: &AddVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        match add_vector_field(doc.snapshot, &payload.question_id, &payload.field_key) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️AddVectorField

//#region 🔖️RemoveVectorField
pub mod remove_vector_field {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-vector-field")]
    pub struct RemoveVectorField {
        pub question_id: String,
        pub field_key: String,
    }

    pub fn handle(payload: &RemoveVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        match remove_vector_field(doc.snapshot, &payload.question_id, &payload.field_key) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RemoveVectorField

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app};
    use crate::apps::forms::FormsCommand;
    use add_vector_field::AddVectorField;
    use patch_vector_field::PatchVectorField;
    use remove_vector_field::RemoveVectorField;

    fn vector_question_id(app: &mut crate::apps::forms::testkit::FormsApp) -> String {
        dispatch(app, FormsCommand::AddQuestion(crate::apps::forms::commands::question::add_question::AddQuestion { kind: "vector".into(), step_id: None }));
        crate::artifacts::forms::engine::flatten_questions(&app.snapshot().expect("projection")).into_iter().map(|(_, question)| question).find(|question| question.kind == "vector").expect("vector question").id
    }

    #[test]
    fn patch_vector_field_updates_the_named_component() {
        let mut app = forms_app();
        let question_id = vector_question_id(&mut app);
        dispatch(&mut app, FormsCommand::PatchVectorField(PatchVectorField { question_id: question_id.clone(), field_key: "x".into(), field: "value".into(), value_json: "5.0".into() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::engine::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        let x = question.fields.as_ref().expect("fields").iter().find(|field| field.key == "x").expect("x field");
        assert_eq!(x.value, Some(5.0));
    }

    #[test]
    fn add_and_remove_vector_field_round_trip() {
        let mut app = forms_app();
        let question_id = vector_question_id(&mut app);
        dispatch(&mut app, FormsCommand::AddVectorField(AddVectorField { question_id: question_id.clone(), field_key: "w".into() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::engine::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        assert!(question.fields.as_ref().expect("fields").iter().any(|field| field.key == "w"));
        dispatch(&mut app, FormsCommand::RemoveVectorField(RemoveVectorField { question_id: question_id.clone(), field_key: "w".into() }));
        let spec = app.snapshot().expect("projection");
        let (_, question) = crate::artifacts::forms::engine::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
        assert!(question.fields.as_ref().expect("fields").iter().all(|field| field.key != "w"));
    }
}
//#endregion 🧪️Tests
