use super::*;
use crate::editor::gis3d::testkit::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_the_world_3d_terrain_scene() {
    let mut app = app().await;
    let json = render_body(&mut app, GIS3D_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains("world-3d"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_fixture_pins_reach_the_scene_as_world_instances() {
    let mut app = app().await;
    let json = render_body(&mut app, GIS3D_PLAY_BODY_COMPOSITE).await;
    assert!(json.contains("p_institut_de_botanique_ulg_liege"));
    assert!(json.contains("pin"));
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_world3d_surface_to_the_composite_body() {
    let definition = definition();
    assert_eq!(definition.id, GIS3D_PLAY_WINDOW_MAIN);
    assert_eq!(definition.body_key, GIS3D_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
}
