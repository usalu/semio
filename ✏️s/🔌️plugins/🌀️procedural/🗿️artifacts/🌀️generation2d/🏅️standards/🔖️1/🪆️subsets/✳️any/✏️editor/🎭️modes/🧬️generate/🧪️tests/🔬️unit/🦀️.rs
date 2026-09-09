use super::*;

#[test]
fn the_generate_layout_lists_all_three_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(generations::GENERATION2D_PLAY_WINDOW_GENERATIONS));
    assert!(json.contains(form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM));
    assert!(json.contains(preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW));
}
