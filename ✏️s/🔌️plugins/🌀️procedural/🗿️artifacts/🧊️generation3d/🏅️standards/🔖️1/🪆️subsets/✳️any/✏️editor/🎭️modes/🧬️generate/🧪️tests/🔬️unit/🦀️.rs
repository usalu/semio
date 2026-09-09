use super::*;

#[test]
fn the_generate_layout_lists_all_three_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS));
    assert!(json.contains(form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM));
    assert!(json.contains(preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW));
}
