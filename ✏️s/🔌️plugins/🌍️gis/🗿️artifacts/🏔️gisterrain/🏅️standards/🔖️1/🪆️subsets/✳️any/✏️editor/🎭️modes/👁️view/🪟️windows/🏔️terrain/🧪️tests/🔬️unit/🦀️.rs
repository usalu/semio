use semio_framework_plugin::SurfaceKind;
use serde_json::json;
#[allow(unused_imports)]
use crate::schema::terrain_descriptor::TerrainDescriptorJson;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::build_terrain_scene_json;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::fixture_descriptor;
#[allow(unused_imports)]
use crate::GisTerrainSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
#[allow(unused_imports)]
use semio_framework_plugin::scene_surface;
#[allow(unused_imports)]
use semio_framework_plugin::world3d_selection_json;
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_plugin::World3dScene;
#[allow(unused_imports)]
use semio_framework_surface::terrain::projection;
#[allow(unused_imports)]
use serde_json::Value;
use crate::editor::gis3d::modes::view::windows::terrain::*;
use crate::editor::gis3d::unit_tests::context::{app, close, render as render_body};

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
