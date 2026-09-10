use super::*;
use crate::editor::generation3d::commands::{add_widget, patch_flow_widgets};
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use semio_framework_artifact_flow_flow::Widget;
use crate::editor::generation3d::testkit;

#[semio_framework_async_macros::async_test]
async fn add_widget_action_appends_widget() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = testkit::snapshot(&app).fixture.widgets.len();
    dispatch(&mut app, Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None })).await;
    assert!(testkit::snapshot(&app).fixture.widgets.len() > before);
}

#[semio_framework_async_macros::async_test]
async fn patch_flow_widgets_edits_slider_value() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) })).await;
    let value = testkit::snapshot(&app).fixture.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, .. } if id == "height" => Some(*value),
        _ => None,
    });
    assert_eq!(value, Some(9.5));
}

#[semio_framework_async_macros::async_test]
async fn patch_flow_widgets_recomputes_preview_geometry() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before_fixture = testkit::snapshot(&app).fixture.clone();
    let mut before_session = FlowEvalSession::new();
    let mut before_host = semio_framework_os_flow::flow_host_with_session(&before_fixture, &before_session);
    before_session.sync(&before_host);
    while before_session.tick(&mut before_host) {}
    before_host.retire_cold();
    let before_eval = before_session.eval_json().to_string();
    let (before_meshes, _) = crate::editor::generation3d::preview_payload_from_eval(&before_eval, &before_fixture, &Generation3dConfig::default());

    dispatch(&mut app, Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) })).await;
    let after_fixture = testkit::snapshot(&app).fixture.clone();
    let mut after_session = FlowEvalSession::new();
    let mut after_host = semio_framework_os_flow::flow_host_with_session(&after_fixture, &after_session);
    after_session.sync(&after_host);
    while after_session.tick(&mut after_host) {}
    after_host.retire_cold();
    let after_eval = after_session.eval_json().to_string();
    let (after_meshes, _) = crate::editor::generation3d::preview_payload_from_eval(&after_eval, &after_fixture, &Generation3dConfig::default());

    assert_ne!(before_eval, after_eval, "slider mutation must invalidate the evaluated flow");
    assert_ne!(before_meshes, after_meshes, "slider mutation must change the tessellated preview mesh");
    // 🧹️ Both owners reject a live drop: `FlowEvalSession` demands the explicit close boundary its
    // production owner walks, and a cloned `FlowFixture` carries the fail-closed
    // `OrderedMap<WidgetLayout>` root (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    testkit::retire_flow_eval_session(after_session);
    testkit::retire_flow_eval_session(before_session);
    after_fixture.retire_cold();
    before_fixture.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn remove_widget_action_deletes_by_id() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    assert!(testkit::snapshot(&app).fixture.widgets.iter().any(|widget| crate::widget_id(widget) == "sides"));
    dispatch(&mut app, Generation3dCommand::RemoveWidget(RemoveWidget { widget_id: "sides".into() })).await;
    assert!(!testkit::snapshot(&app).fixture.widgets.iter().any(|widget| crate::widget_id(widget) == "sides"));
}
