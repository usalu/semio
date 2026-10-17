use crate::editor::gis3d::modes::view::windows::terrain;
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowLayout;
use crate::editor::gis3d::modes::view::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_single_terrain_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(terrain::GIS3D_PLAY_WINDOW_MAIN));
}

#[semio_framework_async_macros::async_test]
async fn the_mode_is_the_apps_only_and_default_mode() {
    assert_eq!(definition().id, GIS3D_PLAY_MODE_VIEW);
}
