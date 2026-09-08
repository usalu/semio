
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_model_window_only() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(model::LOWPOLY_PLAY_WINDOW_MAIN));
}
