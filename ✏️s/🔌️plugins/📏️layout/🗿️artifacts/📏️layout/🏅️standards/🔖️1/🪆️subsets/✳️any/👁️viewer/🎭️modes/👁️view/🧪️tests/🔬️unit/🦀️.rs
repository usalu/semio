use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_references_the_preview_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(preview::WINDOW_KIND_ID), "layout must reference the preview window kind: {json}");
}
