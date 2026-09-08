
use super::*;
use crate::editor::flow::FlowCommand;
use crate::editor::flow::modes::generate::windows::{form, generations};
use crate::editor::flow::testkit::{dispatch, flow_app, render};

#[semio_framework_async_macros::async_test]
async fn adding_a_generation_populates_the_form_and_emits_no_artifact_mutations() {
    let mut app = flow_app().await;
    assert!(render(&mut app, form::FLOW_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"), "the form starts empty");
    let result = dispatch(&mut app, FlowCommand::AddGeneration(AddGeneration {})).await;
    assert!(result.mutations.is_empty(), "generations are config state, never document operations");
    assert!(render(&mut app, generations::FLOW_PLAY_BODY_GENERATIONS).await.contains("selectGeneration"), "the new generation lands in the list");
}

#[semio_framework_async_macros::async_test]
async fn removing_an_unknown_generation_is_a_no_operation() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::RemoveGeneration(crate::editor::flow::modes::generate::commands::remove_generation::RemoveGeneration { id: "nope".into() })).await;
    assert!(result.mutations.is_empty());
}
