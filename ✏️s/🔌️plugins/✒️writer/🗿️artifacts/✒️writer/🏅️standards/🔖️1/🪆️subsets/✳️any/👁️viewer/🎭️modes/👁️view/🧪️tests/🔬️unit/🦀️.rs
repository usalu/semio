use crate::viewer::writer::modes::view::windows::main;
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
use crate::viewer::writer::modes::view::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_main_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(main::WRITER_VIEW_WINDOW_KIND), "layout must reference the main window kind: {json}");
}
