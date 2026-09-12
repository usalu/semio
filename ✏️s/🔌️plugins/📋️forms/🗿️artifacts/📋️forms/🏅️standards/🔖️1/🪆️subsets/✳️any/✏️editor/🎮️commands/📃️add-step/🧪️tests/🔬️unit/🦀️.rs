use super::*;
use crate::editor::forms::commands::move_step::MoveStep;
use crate::editor::forms::commands::patch_step::PatchStep;
use crate::editor::forms::commands::remove_step::RemoveStep;
use crate::editor::forms::commands::update_form::UpdateForm;
use crate::editor::forms::unit_tests::context::{dispatch, forms_app};
use crate::editor::forms::FormsCommand;
use AddStep;

#[semio_framework_async_macros::async_test]
async fn add_step_action_appends_step() {
    let mut app = forms_app().await;
    let before = forms_steps(&app.snapshot().expect("projection")).len();
    dispatch(&mut app, FormsCommand::AddStep(AddStep {})).await;
    assert_eq!(forms_steps(&app.snapshot().expect("projection")).len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn patch_step_updates_title_and_description() {
    let mut app = forms_app().await;
    let step_id = forms_steps(&app.snapshot().expect("projection"))[0].id.clone();
    dispatch(&mut app, FormsCommand::PatchStep(PatchStep { step_id, field: "title".into(), value: "Renamed".into() })).await;
    assert_eq!(forms_steps(&app.snapshot().expect("projection"))[0].title, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn remove_and_move_step_actions() {
    let mut app = forms_app().await;
    dispatch(&mut app, FormsCommand::AddStep(AddStep {})).await;
    let last_step_id = forms_steps(&app.snapshot().expect("projection")).last().unwrap().id.clone();
    dispatch(&mut app, FormsCommand::MoveStep(MoveStep { step_id: last_step_id.clone(), index: 0 })).await;
    assert_eq!(forms_steps(&app.snapshot().expect("projection"))[0].id, last_step_id);
    dispatch(&mut app, FormsCommand::RemoveStep(RemoveStep { step_id: last_step_id.clone() })).await;
    assert!(forms_steps(&app.snapshot().expect("projection")).iter().all(|step| step.id != last_step_id));
}

#[semio_framework_async_macros::async_test]
async fn update_form_action_sets_title() {
    let mut app = forms_app().await;
    dispatch(&mut app, FormsCommand::UpdateForm(UpdateForm { title: "My Form".into() })).await;
    assert_eq!(app.snapshot().expect("projection").title.as_deref(), Some("My Form"));
}
