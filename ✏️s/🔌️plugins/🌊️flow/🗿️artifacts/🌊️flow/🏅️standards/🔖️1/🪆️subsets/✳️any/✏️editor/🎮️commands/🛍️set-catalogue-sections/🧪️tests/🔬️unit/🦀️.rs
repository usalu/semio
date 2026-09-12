use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn setting_catalogue_sections_emits_no_artifact_mutations() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::SetCatalogueSections(SetCatalogueSections { sections_json: "[]".into() })).await;
    assert!(result.mutations.is_empty());
}
