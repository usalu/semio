
use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;
use crate::editor::gis2d::testkit::{app, main_window_measures, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_gis_map_scene() {
    let mut app = app().await;
    assert!(render_body(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await.contains("tiled-map"));
}

#[semio_framework_async_macros::async_test]
async fn render_canvas_uses_absolute_tile_urls_when_env_set() {
    unsafe { std::env::set_var("SEMIO_ASSET_BASE_URL", "http://127.0.0.1:6141") };
    let mut app = app().await;
    let json = render_body(&mut app, GIS2D_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains("http://127.0.0.1:6141/osm/{z}/{x}/{y}.png"));
    assert!(json.contains("http://127.0.0.1:6141/vt/{z}/{x}/{y}.pbf"));
    unsafe { std::env::remove_var("SEMIO_ASSET_BASE_URL") };
}

#[semio_framework_async_macros::async_test]
async fn the_window_collects_every_option_node_exactly_once() {
    let config = Gis2dConfig::default();
    let measures = window_measures(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default()));
    assert_eq!(measures.len(), 5, "3 selects + the layers and layer-weights groups");
    let mut app = app().await;
    assert_eq!(main_window_measures(&mut app).await.len(), measures.len(), "the app routes the same set under the window id");
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_tiled_map_surface_to_the_composite_body() {
    let definition = definition();
    assert_eq!(definition.id, GIS2D_PLAY_WINDOW_MAIN);
    assert_eq!(definition.body_key, GIS2D_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::TiledMap));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
