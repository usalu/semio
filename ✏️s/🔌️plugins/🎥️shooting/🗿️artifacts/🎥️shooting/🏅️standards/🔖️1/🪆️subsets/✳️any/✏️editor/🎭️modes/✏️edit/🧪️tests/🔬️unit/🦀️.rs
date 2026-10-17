use crate::{editor::shooting::modes::edit::windows::icon, editor::shooting::modes::edit::windows::scene};
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
use crate::editor::shooting::modes::edit::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_both_edit_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(scene::SHOOTING_PLAY_WINDOW_SCENE) && json.contains(icon::SHOOTING_PLAY_WINDOW_ICON), "layout must reference both window kinds: {json}");
}
