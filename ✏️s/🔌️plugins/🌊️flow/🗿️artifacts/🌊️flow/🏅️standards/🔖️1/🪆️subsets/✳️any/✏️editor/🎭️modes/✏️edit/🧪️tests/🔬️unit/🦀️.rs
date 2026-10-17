use crate::editor::flow::base::FLOW_PLAY_WINDOW_MAIN;
use crate::editor::flow::modes::edit::windows::compiled;
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main;
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
use crate::editor::flow::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(FLOW_PLAY_WINDOW_MAIN) && json.contains(compiled::FLOW_PLAY_WINDOW_COMPILED), "layout must reference both window kinds: {json}");
}
