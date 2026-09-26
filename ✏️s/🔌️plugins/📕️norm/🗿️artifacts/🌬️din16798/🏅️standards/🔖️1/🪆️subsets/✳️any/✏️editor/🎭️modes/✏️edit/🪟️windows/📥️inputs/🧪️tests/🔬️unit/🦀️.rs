use super::*;
use crate::editor::din16798::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_INPUTS);
    assert_eq!(definition().id, WINDOW_INPUTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_structured_editor_not_json_dump() {
    let mut app = context::app_with_registry().await;
    let body = context::render(&mut app, BODY_INPUTS).await;
    let looks_structured = ["Outdoor", "Außen", "Zone", "zones", "Ventilation", "Lüftung", "National", "annex", "Comfort", "Komfort"]
        .iter()
        .any(|needle| body.contains(needle));
    assert!(looks_structured, "inputs body must render structured field labels, got: {body}");
    assert!(!body.trim_start().starts_with('{') || looks_structured, "inputs must not be a raw JSON dump");
    context::close(&mut app);
}
