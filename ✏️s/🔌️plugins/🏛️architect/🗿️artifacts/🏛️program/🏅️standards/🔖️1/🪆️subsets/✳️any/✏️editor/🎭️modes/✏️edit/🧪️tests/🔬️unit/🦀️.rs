
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_four_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    for window in [adjacency::ARCHITECT_WINDOW_ADJACENCY, graph::ARCHITECT_WINDOW_GRAPH, register::ARCHITECT_WINDOW_REGISTER, report::ARCHITECT_WINDOW_REPORT] {
        assert!(json.contains(window), "layout must reference {window}: {json}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_mode_is_the_pencil_edit_mode() {
    let definition = definition();
    assert_eq!(definition.id, ARCHITECT_MODE_EDIT);
    assert_eq!(definition.icon_id.as_str(), "pencil");
}
