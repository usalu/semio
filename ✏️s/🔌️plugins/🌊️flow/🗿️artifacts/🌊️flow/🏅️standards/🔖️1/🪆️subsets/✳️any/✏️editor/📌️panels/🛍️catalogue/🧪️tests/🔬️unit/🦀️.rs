use super::*;
use crate::editor::flow::testkit::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn flow_widget_drag_json_wraps_descriptor_under_drag_mime() {
    let descriptor = flow_widget_descriptor("neuron", Some("math.add"));
    let drag = flow_widget_drag_json(&descriptor);
    assert!(drag.get(FLOW_WIDGET_DRAG_MIME).is_some());
}

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_module_operators() {
    let mut app = flow_app().await;
    let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("flow-play-catalogue.math"), "expected math module section: {json}");
    assert!(json.contains("math.add"), "expected math.add operator: {json}");
}

#[semio_framework_async_macros::async_test]
async fn catalogue_items_export_flow_widget_drag_payload() {
    let mut app = flow_app().await;
    let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains(FLOW_WIDGET_DRAG_MIME), "missing drag mime: {json}");
    assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
}

#[semio_framework_async_macros::async_test]
async fn every_built_in_extension_is_listed_in_the_installed_section() {
    let mut app = flow_app().await;
    let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE).await;
    for (id, ..) in FLOW_AUTOMATIONS {
        assert!(json.contains(&format!("flow-play-extensions.{id}")), "extension {id} missing: {json}");
    }
}
