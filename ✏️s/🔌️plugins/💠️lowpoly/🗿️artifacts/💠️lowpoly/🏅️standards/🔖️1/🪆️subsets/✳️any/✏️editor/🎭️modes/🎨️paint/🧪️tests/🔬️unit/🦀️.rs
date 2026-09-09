use super::*;

#[semio_framework_async_macros::async_test]
async fn the_named_layout_lists_both_paint_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(model::LOWPOLY_PLAY_WINDOW_MAIN) && json.contains(uv::LOWPOLY_PLAY_WINDOW_UV), "layout must reference both window kinds: {json}");
}

#[semio_framework_async_macros::async_test]
async fn the_mode_binds_directly_to_its_named_layout() {
    assert_eq!(definition().layout_id.as_deref(), Some(LOWPOLY_PLAY_LAYOUT_PAINT));
}
