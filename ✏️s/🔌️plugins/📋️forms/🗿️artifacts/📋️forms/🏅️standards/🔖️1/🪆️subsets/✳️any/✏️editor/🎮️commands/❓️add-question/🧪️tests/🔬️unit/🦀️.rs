use super::*;
use crate::editor::forms::commands::drop_question_kind::DropQuestionKind;
use crate::editor::forms::commands::move_question::MoveQuestion;
use crate::editor::forms::commands::patch_questions::PatchQuestions;
use crate::editor::forms::commands::remove_question::RemoveQuestion;
use crate::editor::forms::testkit::{dispatch, forms_app};
use crate::editor::forms::FormsCommand;
use AddQuestion;

#[semio_framework_async_macros::async_test]
async fn add_question_action_appends_question() {
    let mut app = forms_app().await;
    dispatch(&mut app, FormsCommand::AddQuestion(AddQuestion { kind: "text".into(), step_id: None })).await;
    assert!(crate::schema::flatten_questions(&app.snapshot().expect("projection")).iter().any(|(_, question)| question.kind == "text"));
}

#[semio_framework_async_macros::async_test]
async fn add_question_undo_redo_round_trip() {
    let mut app = forms_app().await;
    let before = crate::schema::flatten_questions(&app.snapshot().expect("projection")).len();
    semio_framework_plugin::testkit::assert_undo_redo_round_trip(
        &mut app,
        FormsCommand::AddQuestion(AddQuestion { kind: "text".into(), step_id: None }),
        |app| crate::schema::flatten_questions(&app.snapshot().expect("projection")).len(),
        before,
        before + 1,
    )
    .await;
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the dropped question is no longer
/// auto-selected by this command (selection is framework-owned now) — only the document edit itself.
#[semio_framework_async_macros::async_test]
async fn drop_question_kind_inserts_the_question() {
    let mut app = forms_app().await;
    let step_id = forms_steps(&app.snapshot().expect("projection"))[0].id.clone();
    dispatch(&mut app, FormsCommand::DropQuestionKind(DropQuestionKind { kind: "slider".into(), target_id: crate::schema::forms_play_step_tree_id(&step_id), drop_position: "inside".into() })).await;
    let spec = app.snapshot().expect("projection");
    assert!(forms_steps(&spec)[0].blocks.iter().any(|question| question.kind == "slider"));
}

#[semio_framework_async_macros::async_test]
async fn inspector_patch_updates_required() {
    let mut app = forms_app().await;
    dispatch(&mut app, FormsCommand::SetActiveExample(crate::editor::forms::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
    let name_id = forms_steps(&app.snapshot().expect("projection"))[0].blocks[0].id.clone();
    dispatch(&mut app, FormsCommand::PatchQuestions(PatchQuestions { question_ids: vec![name_id], field: "required".into(), value_json: "false".into(), param_key: None })).await;
    let spec = app.snapshot().expect("projection");
    assert!(!forms_steps(&spec)[0].blocks[0].required.unwrap_or(true));
}

#[semio_framework_async_macros::async_test]
async fn remove_question_removes_it_from_the_document() {
    let mut app = forms_app().await;
    let question_id = forms_steps(&app.snapshot().expect("projection"))[0].blocks[0].id.clone();
    dispatch(&mut app, FormsCommand::RemoveQuestion(RemoveQuestion { question_id: question_id.clone() })).await;
    assert!(crate::schema::flatten_questions(&app.snapshot().expect("projection")).iter().all(|(_, question)| question.id != question_id));
}

#[semio_framework_async_macros::async_test]
async fn move_question_relocates_it_to_the_target_step() {
    let mut app = forms_app().await;
    dispatch(&mut app, FormsCommand::AddStep(crate::editor::forms::commands::add_step::AddStep {})).await;
    let spec = app.snapshot().expect("projection");
    let steps = forms_steps(&spec);
    let question_id = steps[0].blocks[0].id.clone();
    let target_step_id = steps.last().unwrap().id.clone();
    dispatch(&mut app, FormsCommand::MoveQuestion(MoveQuestion { question_id: question_id.clone(), to_step_id: target_step_id.clone(), target_id: None, position: "inside".into(), index: None })).await;
    let moved = app.snapshot().expect("projection");
    assert!(forms_steps(&moved).iter().find(|step| step.id == target_step_id).expect("target step").blocks.iter().any(|question| question.id == question_id));
}
