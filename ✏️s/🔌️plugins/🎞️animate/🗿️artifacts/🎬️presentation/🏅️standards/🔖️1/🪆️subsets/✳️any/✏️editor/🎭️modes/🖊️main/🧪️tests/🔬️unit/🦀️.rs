use crate::editor::animate::modes::main::windows::tile_editor;
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
use crate::editor::animate::modes::main::*;

#[test]
fn the_default_layout_lists_the_tile_editor_window() {
    let json = semio_framework_pack_json::to_json_string(&layout());
    assert!(json.contains(tile_editor::PRESENTATION_PLAY_WINDOW_MAIN), "layout must reference the tile-editor window kind: {json}");
}
