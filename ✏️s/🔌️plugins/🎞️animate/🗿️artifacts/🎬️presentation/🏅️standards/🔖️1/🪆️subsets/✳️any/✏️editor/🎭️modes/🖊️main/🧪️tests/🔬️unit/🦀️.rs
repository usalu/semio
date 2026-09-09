use super::*;

#[test]
fn the_default_layout_lists_the_tile_editor_window() {
    let json = dsl::os_pack::json::to_json_string(&layout());
    assert!(json.contains(tile_editor::PRESENTATION_PLAY_WINDOW_MAIN), "layout must reference the tile-editor window kind: {json}");
}
