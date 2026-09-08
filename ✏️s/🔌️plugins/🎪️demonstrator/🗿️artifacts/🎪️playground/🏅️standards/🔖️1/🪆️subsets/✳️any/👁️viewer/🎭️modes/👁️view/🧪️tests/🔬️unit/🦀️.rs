
use super::*;

#[test]
fn the_view_layout_lists_the_one_read_only_window() {
    let json = dsl::os_pack::json::to_json_string(&layout());
    assert!(json.contains(main::WINDOW_KIND_ID), "layout must reference the main window kind: {json}");
}
