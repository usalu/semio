
use super::*;
use crate::editor::forms::FormsCommand;
use crate::editor::forms::commands::remove_question_option::RemoveQuestionOption;
use crate::editor::forms::testkit::{dispatch, forms_app};
use AddQuestionOption;

async fn single_or_multi_question_id(app: &mut crate::editor::forms::testkit::FormsApp) -> String {
    dispatch(app, FormsCommand::AddQuestion(crate::editor::forms::commands::add_question::AddQuestion { kind: "single".into(), step_id: None })).await;
    crate::schema::flatten_questions(&app.snapshot().expect("projection")).into_iter().map(|(_, question)| question).find(|question| question.kind == "single").expect("single question").id
}

#[semio_framework_async_macros::async_test]
async fn add_and_remove_question_option_round_trip() {
    let mut app = forms_app().await;
    let question_id = single_or_multi_question_id(&mut app).await;
    dispatch(&mut app, FormsCommand::AddQuestionOption(AddQuestionOption { question_id: question_id.clone(), label: "New option".into() })).await;
    let spec = app.snapshot().expect("projection");
    let (_, question) = crate::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
    let added = question.options.as_ref().expect("options").iter().find(|option| option.label == "New option").expect("added option").value.clone();
    dispatch(&mut app, FormsCommand::RemoveQuestionOption(RemoveQuestionOption { question_id: question_id.clone(), option_value: added.clone() })).await;
    let spec = app.snapshot().expect("projection");
    let (_, question) = crate::schema::flatten_questions(&spec).into_iter().find(|(_, question)| question.id == question_id).expect("question");
    assert!(question.options.as_ref().expect("options").iter().all(|option| option.value != added));
}
