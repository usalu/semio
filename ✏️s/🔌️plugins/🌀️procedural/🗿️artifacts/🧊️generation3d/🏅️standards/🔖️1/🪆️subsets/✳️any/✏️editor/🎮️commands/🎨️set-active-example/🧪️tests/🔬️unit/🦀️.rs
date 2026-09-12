use super::*;
use crate::editor::generation3d::unit_tests::context::{app, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::PluginApp;
use crate::editor::generation3d::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn set_active_example_via_string_action_loads_fixture() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_FILLET }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("set example");
    context::settle(&mut app).await;
    let projection = context::snapshot(&app);
    assert!(projection.fixture.widgets.iter().any(|widget| crate::widget_id(widget).contains("fillet") || matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind.contains("fillet") || neuron_kind.contains("box"))));
}

#[semio_framework_async_macros::async_test]
async fn unknown_example_id_is_a_no_op() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    dispatch(&mut app, Generation3dCommand::SetActiveExample(SetActiveExample { example_id: "not-a-real-example".into() })).await;
    assert_eq!(context::snapshot(&app), before);
}
