
use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_window_kinds() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(inputs::WINDOW_INPUTS), "layout must reference the inputs window kind: {json}");
    assert!(json.contains(results::WINDOW_RESULTS), "layout must reference the results window kind: {json}");
}

#[semio_framework_async_macros::async_test]
async fn the_mode_is_the_apps_default() {
    assert_eq!(definition().id, crate::app_surface::MODE_EDIT);
}
