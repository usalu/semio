
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_canvas_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(canvas::WIRES_VIEW_WINDOW_CANVAS), "layout must reference the canvas window kind: {json}");
}
