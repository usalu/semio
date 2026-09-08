
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_builder_window() {
    let json = protocol::json::to_json_string(&layout());
    assert!(json.contains(builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER), "layout must reference the builder window kind: {json}");
}
