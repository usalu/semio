#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use crate::{FlowMutation, FlowSnapshot};
#[allow(unused_imports)]
use flow::FlowEvalSession;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::flow::commands::set_catalogue_sections::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn setting_catalogue_sections_emits_no_artifact_mutations() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::SetCatalogueSections(SetCatalogueSections { sections_json: "[]".into() })).await;
    assert!(result.mutations.is_empty());
    settle(&mut app).await;
}
