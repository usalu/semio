
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_all_three_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::SEQUENCE_PLAY_WINDOW_MAIN) && json.contains(script::SEQUENCE_PLAY_WINDOW_SCRIPT) && json.contains(compiled::SEQUENCE_PLAY_WINDOW_COMPILED), "layout must reference all three window kinds: {json}");
}
