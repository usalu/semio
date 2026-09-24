use super::*;

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
    let document = crate::schema::default_document();
    let unpanned = semio_framework_plugin::artifact_app_laws::built_surface_scene::<TiledMapScene>(&render(&document, None).expect("render")).expect("tiled map scene");
    assert_eq!(unpanned.camera_json, GIS_MAP_VIEW_DEFAULT_CAMERA_JSON);
    let window = config::GisMapViewerWindowConfig { camera: config::GisMapViewerCamera { x: 12.0, y: -4.5, zoom: 2.0 } };
    let panned = semio_framework_plugin::artifact_app_laws::built_surface_scene::<TiledMapScene>(&render(&document, Some(&window)).expect("render")).expect("tiled map scene");
    assert_eq!(panned.camera_json, window.camera.scene_camera_json());
}
