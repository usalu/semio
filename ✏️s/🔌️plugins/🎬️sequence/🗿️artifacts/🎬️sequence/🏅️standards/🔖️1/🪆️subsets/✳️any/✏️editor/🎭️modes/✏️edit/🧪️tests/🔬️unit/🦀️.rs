use crate::editor::sequence::modes::edit::windows::{compiled, main, script};
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
use crate::editor::sequence::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_all_three_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::SEQUENCE_PLAY_WINDOW_MAIN) && json.contains(script::SEQUENCE_PLAY_WINDOW_SCRIPT) && json.contains(compiled::SEQUENCE_PLAY_WINDOW_COMPILED), "layout must reference all three window kinds: {json}");
}
