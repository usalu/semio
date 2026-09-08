
use super::*;

#[test]
fn the_view_layout_lists_the_tile_editor_window() {
    let json = dsl::os_pack::json::to_json_string(&layout());
    assert!(json.contains(tile_editor::WINDOW_KIND_ID), "layout must reference the tile-editor window kind: {json}");
}
