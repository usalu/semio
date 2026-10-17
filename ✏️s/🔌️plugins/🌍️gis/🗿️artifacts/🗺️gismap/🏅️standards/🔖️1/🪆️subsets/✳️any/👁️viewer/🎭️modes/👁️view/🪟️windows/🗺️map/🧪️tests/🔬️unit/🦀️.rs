use semio_framework_plugin::ActionKind;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TiledMapScene;
#[allow(unused_imports)]
use crate::tile_coordinates::gis_policy::gis_map_scene_descriptor_json;
#[allow(unused_imports)]
use crate::GisMapSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
#[allow(unused_imports)]
use semio_framework_plugin::scene_surface;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
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
use crate::viewer::gismap::modes::view::windows::map::{GIS_MAP_VIEW_DEFAULT_CAMERA_JSON};
use crate::viewer::gismap::modes::view::windows::map::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tiled_map_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.surface_kind, SurfaceKind::TiledMap);
}

/// ⚖️ LAW: the read-only map window declares the host's camera verb, with its `camera` argument, as a
/// `Migrated` view action — undeclared, the shell dropped every pan (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_map_window_declares_the_hosts_camera_verb() {
    let def = definition();
    let action = def.actions.iter().find(|action| action.id == SET_CAMERA_ACTION_ID).expect("the viewer map window declares setCamera");
    assert!(matches!(action.kind, ActionKind::View), "a camera is a view action, never a document mutation");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(action.args.iter().any(|arg| arg.id == "camera"), "the host's camera argument survives effective_action_args");
}

/// ⚖️ LAW: an unpanned window publishes the default camera the host fits to the world; a retained
/// camera is published exactly.
#[semio_framework_async_macros::async_test]
async fn render_publishes_the_default_camera_until_the_window_retains_one() {
    let document = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let unpanned = semio_framework_plugin::artifact_app_laws::built_surface_scene::<TiledMapScene>(&render(&document, None).expect("render")).expect("tiled map scene");
    assert_eq!(unpanned.camera_json, GIS_MAP_VIEW_DEFAULT_CAMERA_JSON);
    let window = config::GisMapViewerWindowConfig { camera: config::GisMapViewerCamera { x: 12.0, y: -4.5, zoom: 2.0 } };
    let panned = semio_framework_plugin::artifact_app_laws::built_surface_scene::<TiledMapScene>(&render(&document, Some(&window)).expect("render")).expect("tiled map scene");
    assert_eq!(panned.camera_json, window.camera.scene_camera_json());
}
