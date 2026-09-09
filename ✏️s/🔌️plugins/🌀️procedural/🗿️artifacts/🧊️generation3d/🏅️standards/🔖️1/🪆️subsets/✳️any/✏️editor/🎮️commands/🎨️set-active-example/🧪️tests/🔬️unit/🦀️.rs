use super::*;
use crate::editor::generation3d::testkit::{app, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn set_active_example_via_string_action_loads_fixture() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app_with_registry().await;
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_FILLET }).into()), &semio_framework_plugin::testkit::meta("local")).await.expect("set example");
    let projection = app.snapshot().expect("snapshot");
    assert!(projection.fixture.widgets.iter().any(|widget| crate::widget_id(widget).contains("fillet") || matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind.contains("fillet") || neuron_kind.contains("box"))));
}

#[semio_framework_async_macros::async_test]
async fn unknown_example_id_is_a_no_op() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Generation3dCommand::SetActiveExample(SetActiveExample { example_id: "not-a-real-example".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot"), before);
}
