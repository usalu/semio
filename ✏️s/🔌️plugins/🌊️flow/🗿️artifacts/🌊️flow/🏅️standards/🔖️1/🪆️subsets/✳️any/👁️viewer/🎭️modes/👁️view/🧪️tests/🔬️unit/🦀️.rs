use super::*;

#[semio_framework_async_macros::async_test]
async fn the_layout_lists_the_single_view_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::WINDOW_KIND_ID), "layout must reference the main window kind: {json}");
}
