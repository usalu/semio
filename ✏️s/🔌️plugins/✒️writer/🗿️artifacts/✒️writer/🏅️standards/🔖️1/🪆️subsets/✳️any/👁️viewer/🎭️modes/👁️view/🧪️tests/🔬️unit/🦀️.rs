use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_main_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::WRITER_VIEW_WINDOW_KIND), "layout must reference the main window kind: {json}");
}
