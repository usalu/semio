use crate::editor::shooting::modes::edit::windows::scene::options;
use semio_framework_pack_json::json;
use semio_framework_pack_json::{parse, Value};
use semio_framework_plugin::SurfaceKind;
#[allow(unused_imports)]
use crate::editor::shooting::config::ShootingConfig;
#[allow(unused_imports)]
use crate::editor::shooting::terminology::ShootingLabels;
#[allow(unused_imports)]
use crate::is_transparent_shooting_background;
#[allow(unused_imports)]
use crate::{shooting_asset_scale, ShootingAsset, ShootingShot, ShootingSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::world3d_mesh_id_from_url;
#[allow(unused_imports)]
use semio_framework_plugin::world3d_meshes_json_from_kinds_and_urls;
#[allow(unused_imports)]
use semio_framework_plugin::world3d_scene;
#[allow(unused_imports)]
use semio_framework_plugin::world3d_selection_json;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagementInput;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagementPossible;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagementStatus;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_plugin::World3dScene;
#[allow(unused_imports)]
use semio_framework_plugin::WorldSunConfig;
#[allow(unused_imports)]
use std::collections::HashSet;
use crate::editor::shooting::modes::edit::windows::scene::*;
use crate::editor::shooting::unit_tests::context::{scene_window_measures, shooting_app};

#[semio_framework_async_macros::async_test]
async fn renders_world_model_scene() {
    let mut app = shooting_app().await;
    let scene = crate::editor::shooting::unit_tests::context::world_scene(&mut app).await;
    let environment: Value = parse(scene.environment_json.as_deref().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(environment["sun"]["azimuth"], json!(45.0));
    assert_eq!(environment["material"]["roughness"], json!(1.0));
    let frame: Value = parse(scene.frame_json.as_deref().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(frame["width"], json!(256));
    assert_eq!(frame["shape"], json!("rectangle"));
    let fit: Value = parse(scene.fit_json.as_deref().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(fit["enabled"], json!(true));
    let camera: Value = parse(&scene.camera_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(camera["zoom"], json!(1.0));
    assert_eq!(camera["projection"], json!("perspective"));
}

#[semio_framework_async_macros::async_test]
async fn model_scene_uses_asset_mesh_urls() {
    let mut app = shooting_app().await;
    let scene = crate::editor::shooting::unit_tests::context::world_scene(&mut app).await;
    assert!(scene.meshes_json.contains("mesh:🧊️base"));
    assert!(scene.meshes_json.contains("/mesh/🧊️base.glb"));
}

#[semio_framework_async_macros::async_test]
async fn window_measures_surface_eight_scene_measures() {
    let mut app = shooting_app().await;
    let measures = scene_window_measures(&mut app).await;
    assert_eq!(measures.len(), 8);
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world_3d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, SHOOTING_PLAY_BODY_SCENE);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
