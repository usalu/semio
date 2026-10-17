use crate::viewer::flow::modes::view::windows::main;
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
use crate::viewer::flow::modes::view::*;

#[semio_framework_async_macros::async_test]
async fn the_layout_lists_the_single_view_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::WINDOW_KIND_ID), "layout must reference the main window kind: {json}");
}
