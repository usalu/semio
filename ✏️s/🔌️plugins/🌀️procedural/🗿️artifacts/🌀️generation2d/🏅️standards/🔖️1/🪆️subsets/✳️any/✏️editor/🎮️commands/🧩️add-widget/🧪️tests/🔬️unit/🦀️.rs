use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn add_widget_emits_op_and_grows_document() {
    let mut app = app().await;
    let before = snapshot_read(&app).host_snapshot.widgets.len();
    dispatch(&mut app, Generation2dCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, format: None, action: None, x: None, y: None })).await;
    let after = snapshot_read(&app).host_snapshot.widgets.len();
    close(app);
    assert_eq!(after, before + 1);
}

#[semio_framework_async_macros::async_test]
async fn add_widget_inserts_neuron_export_and_action() {
    let mut app = app().await;
    dispatch(&mut app, Generation2dCommand::AddWidget(AddWidget { kind: "neuron".into(), neuron_kind: Some("math.add".into()), format: None, action: None, x: None, y: None })).await;
    dispatch(&mut app, Generation2dCommand::AddWidget(AddWidget { kind: "outputExport".into(), neuron_kind: None, format: Some("svg".into()), action: None, x: None, y: None })).await;
    dispatch(&mut app, Generation2dCommand::AddWidget(AddWidget { kind: "outputAction".into(), neuron_kind: None, format: None, action: Some("log".into()), x: None, y: None })).await;
    let neuron = snapshot_read(&app).host_snapshot.widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } if neuron_kind == "math.add"));
    let export = snapshot_read(&app).host_snapshot.widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::OutputExport { format, .. } if format == "svg"));
    let action = snapshot_read(&app).host_snapshot.widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::OutputAction { action, .. } if action == "log"));
    close(app);
    assert!(neuron, "neuronKind math.add is inserted");
    assert!(export, "export format svg is inserted");
    assert!(action, "action log is inserted");
}
