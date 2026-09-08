
use super::*;

#[test]
fn the_default_layout_lists_both_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(semio_framework_os_flow::GENERATION_3D_PLAY_WINDOW_MAIN) && json.contains(preview::GENERATION_3D_PLAY_WINDOW_PREVIEW));
}
