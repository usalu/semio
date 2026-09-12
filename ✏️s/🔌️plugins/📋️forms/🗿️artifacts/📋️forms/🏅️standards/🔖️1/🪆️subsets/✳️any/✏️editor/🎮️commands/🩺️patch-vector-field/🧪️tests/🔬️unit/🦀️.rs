use super::*;
use crate::editor::forms::commands::add_vector_field::AddVectorField;
use crate::editor::forms::commands::remove_vector_field::RemoveVectorField;
use crate::editor::forms::unit_tests::context::{dispatch, forms_app};
use crate::editor::forms::FormsCommand;
use PatchVectorField;

async fn vector_question_id(app: &mut crate::editor::forms::unit_tests::context::FormsApp) -> String {
    dispatch(app, FormsCommand::AddQuestion(crate::editor::forms::commands::add_question::AddQuestion { kind: "vector".into(), step_id: None })).await;
    crate::schema::flatten_questions(&app.snapshot().expect("projection")).into_iter().map(|(_, question)| question).find(|question| question.kind == "vector").expect("vector question").id
}

#[semio_framework_async_macros::async_test]
async fn patch_vector_field_updates_the_named_component() {
    let mut app = forms_app().await;
    let question_id = vector_question_id(&mut app).await;
    dispatch(&mut app, FormsCommand::PatchVectorField(PatchVectorField { question_id: question_id.clone(), field_key: "x".into(), field: "value".into(), value_json: "5.0".into() })).await;
    let spec = app.snapshot().expect("projection");
    let (_, question) = crate::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
    let x = question.fields.as_ref().expect("fields").iter().find(|field| field.key == "x").expect("x field");
    assert_eq!(x.value, Some(5.0));
}

#[semio_framework_async_macros::async_test]
async fn add_and_remove_vector_field_round_trip() {
    let mut app = forms_app().await;
    let question_id = vector_question_id(&mut app).await;
    dispatch(&mut app, FormsCommand::AddVectorField(AddVectorField { question_id: question_id.clone(), field_key: "w".into() })).await;
    let spec = app.snapshot().expect("projection");
    let (_, question) = crate::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
    assert!(question.fields.as_ref().expect("fields").iter().any(|field| field.key == "w"));
    dispatch(&mut app, FormsCommand::RemoveVectorField(RemoveVectorField { question_id: question_id.clone(), field_key: "w".into() })).await;
    let spec = app.snapshot().expect("projection");
    let (_, question) = crate::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
    assert!(question.fields.as_ref().expect("fields").iter().all(|field| field.key != "w"));
}
