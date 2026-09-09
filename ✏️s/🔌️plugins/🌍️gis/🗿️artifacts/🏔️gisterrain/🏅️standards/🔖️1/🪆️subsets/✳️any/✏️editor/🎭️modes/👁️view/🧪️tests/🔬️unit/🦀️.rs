use super::*;

#[semio_framework_async_macros::async_test]
async fn the_default_layout_lists_the_single_terrain_window() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    assert!(json.contains(terrain::GIS3D_PLAY_WINDOW_MAIN));
}

#[semio_framework_async_macros::async_test]
async fn the_mode_is_the_apps_only_and_default_mode() {
    assert_eq!(definition().id, GIS3D_PLAY_MODE_VIEW);
}
