use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(editor::VCS_PLAY_WINDOW_EDITOR) && json.contains(history::VCS_PLAY_WINDOW_HISTORY), "layout must reference both window kinds: {json}");
}
