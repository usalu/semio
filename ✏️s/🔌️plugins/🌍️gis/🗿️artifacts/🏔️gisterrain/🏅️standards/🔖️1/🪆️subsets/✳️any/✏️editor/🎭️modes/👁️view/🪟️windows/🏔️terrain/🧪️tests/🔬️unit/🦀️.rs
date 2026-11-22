use semio_plugin::SurfaceKind;
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
use semio_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
#[allow(unused_imports)]
use semio_plugin::scene_surface;
#[allow(unused_imports)]
use semio_plugin::world3d_selection_json;
#[allow(unused_imports)]
use semio_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_plugin::World3dScene;
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
async fn the_fixture_features_reach_the_terrain_scene() {
    let mut app = app().await;
    let json = render_body(&mut app, GIS3D_PLAY_BODY_COMPOSITE).await;
    let scene = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<World3dScene>(&json).expect("terrain scene");
    let terrain: Value = serde_json::from_str(scene.terrain_json.as_deref().expect("terrain lane")).expect("terrain JSON");
    assert_eq!(terrain["overlays"]["positions"][0]["id"], "p_institut_de_botanique_ulg_liege");
    assert_eq!(terrain["overlays"]["routes"][0]["id"], "r_sart_tilman");
    assert_eq!(terrain["overlays"]["regions"][0]["id"], "g_sart_tilman");
    assert!(terrain.get("tileUrlTemplate").is_none());
    let document = app.snapshot().expect("terrain snapshot");
    let mesh = semio_s_artifact_gis_gisterrain_inferences::gis_terrain_mesh_from_snapshot(&document).expect("export mesh");
    let primitive = &mesh.meshes[0].primitives[0];
    let mut session = semio_framework_surface::terrain::TerrainSessionCore::default();
    session.set_project_origin(terrain["projectOriginLon"].as_f64().unwrap(), terrain["projectOriginLat"].as_f64().unwrap());
    session.set_exaggeration(document.exaggeration);
    assert!(session.set_elevation_grid_json(&terrain["elevationGrid"].to_string()));
    let viewport_mesh: Value = serde_json::from_str(&session.terrain_tile_mesh_json(0, 0, 0)).expect("viewport mesh");
    let positions = viewport_mesh["positions"].as_array().expect("viewport positions");
    assert_eq!(positions.len(), primitive.positions.len() * 3);
    for (point, xyz) in primitive.positions.iter().zip(positions.chunks_exact(3)) {
        for (expected, actual) in [point.x, point.y, point.z].into_iter().zip(xyz) {
            assert_eq!(expected as f32, actual.as_f64().unwrap() as f32);
        }
    }
    assert_eq!(viewport_mesh["indices"], serde_json::json!(primitive.indices));
    let draped: Value = serde_json::from_str(&session.terrain_overlay_tile_json(0, 0, 0, &terrain["overlays"].to_string())).expect("draped GIS features");
    assert!(!draped["positions"].as_array().expect("draped positions").is_empty());
    assert!(!draped["routes"][0]["segments"].as_array().expect("draped paths").is_empty());
    assert!(!draped["regions"][0]["segments"].as_array().expect("draped boundaries").is_empty());
    assert_eq!(scene.instances_json, "[]");
    close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_world3d_surface_to_the_composite_body() {
    let definition = definition();
    assert_eq!(definition.id, GIS3D_PLAY_WINDOW_MAIN);
    assert_eq!(definition.body_key, GIS3D_PLAY_BODY_COMPOSITE);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
}

#[semio_framework_async_macros::async_test]
async fn editor_utilities_and_the_data_panel_project_the_mounted_artifact() {
    use crate::editor::gis3d::{create_gis3d_app, terrain_data, unit_tests::context};
    let declaration = create_gis3d_app();
    for (id, _, _, _) in UTILITIES {
        assert!(declaration.utilities.iter().any(|utility| utility.id == id && utility.allows_actions_while_active));
    }
    let mut mounted = app().await;
    let mut view = context::main_window_view();
    view.active_utility_id = Some("terrainSmooth".into());
    view.active_utility_by_window_id.insert(GIS3D_PLAY_WINDOW_MAIN.into(), view.active_utility_id.clone());
    let body = context::render_at(&mut mounted, GIS3D_PLAY_BODY_COMPOSITE, &view).await;
    let scene = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<World3dScene>(&body).expect("terrain editor scene");
    let interaction: Value = serde_json::from_str(scene.interaction_json.as_deref().expect("interaction lane")).unwrap();
    assert_eq!(interaction["activeUtility"], "terrainSmooth");
    assert_eq!(interaction["terrainBrush"]["action"], "editElevation");
    assert!(interaction.get("terrainMovePosition").is_none());
    view.active_utility_id = Some("terrainMovePosition".into());
    view.active_utility_by_window_id.insert(GIS3D_PLAY_WINDOW_MAIN.into(), view.active_utility_id.clone());
    let body = context::render_at(&mut mounted, GIS3D_PLAY_BODY_COMPOSITE, &view).await;
    let scene = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<World3dScene>(&body).expect("terrain editor scene");
    let interaction: Value = serde_json::from_str(scene.interaction_json.as_deref().expect("interaction lane")).unwrap();
    assert_eq!(interaction["terrainMovePosition"]["action"], "moveOverlayPosition");
    assert!(interaction.get("terrainBrush").is_none());
    let sections = view.for_panel();
    for section in [semio_framework::UiRefreshSection::Measures, semio_framework::UiRefreshSection::Tools] {
        let body = context::render_at(&mut mounted, section.body_key(), &sections).await;
        assert!(body.contains(section.body_key()));
    }
    let panel = context::render(&mut mounted, terrain_data::BODY).await;
    assert!(panel.contains("33 × 33"));
    assert!(panel.contains("r_sart_tilman"));
    assert!(panel.contains("Elevation Range"));
    close(&mut mounted);
}
