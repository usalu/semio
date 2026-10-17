use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::World3dScene;
#[allow(unused_imports)]
use crate::{active_shot, is_transparent_shooting_background};
#[allow(unused_imports)]
use crate::{shooting_asset_scale, ShootingAsset, ShootingCamera, ShootingShot, ShootingSnapshot};
#[allow(unused_imports)]
use semio_framework_pack_json::json;
#[allow(unused_imports)]
use semio_framework_pack_json::Value;
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
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_plugin::WorldSunConfig;
#[allow(unused_imports)]
use std::collections::HashSet;
use crate::viewer::shooting::modes::view::windows::scene::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world_3d_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, BODY_KEY);
    assert!(matches!(def.surface_kind, SurfaceKind::World3d));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let node = render(&snapshot).expect("viewer scene");
    let scene: World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assembled scene");
    assert!(scene.meshes_json.contains("/mesh/🧊️base.glb"));
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer scene");
}
