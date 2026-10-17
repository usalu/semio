use crate::viewer::animate::modes::view::windows::tile_editor;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutRoot;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutStackNode;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayoutWindowNode;
use crate::viewer::animate::modes::view::*;

#[test]
fn the_view_layout_lists_the_tile_editor_window() {
    let json = semio_framework_pack_json::to_json_string(&layout());
    assert!(json.contains(tile_editor::WINDOW_KIND_ID), "layout must reference the tile-editor window kind: {json}");
}
