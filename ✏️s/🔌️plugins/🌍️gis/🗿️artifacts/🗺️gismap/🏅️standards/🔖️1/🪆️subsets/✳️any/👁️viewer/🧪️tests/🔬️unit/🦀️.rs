use super::*;

#[semio_framework_async_macros::async_test]
async fn create_gismap_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_gismap_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, GISMAP_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<GisMapViewer as ArtifactViewer>::DIALECT, GISMAP_DIALECT);
}

//#region 🧭️ViewerCamera
fn map_view(window_kind_id: &str) -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel { window_id: Some("map-1".into()), window_instances: vec![semio_framework::ViewWindowInstance { id: "map-1".into(), window_kind_id: window_kind_id.into() }], ..Default::default() }
}

/// ⚖️ LAW: the read-only surface declares and owns the host's camera verb — undeclared, the shell
/// dropped every pan with `no window kind declares it` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_viewer_declares_and_owns_the_camera_verb() {
    let def = create_gismap_viewer();
    let window = def.window_kinds.iter().find(|window| window.id == map::WINDOW_KIND_ID).expect("the map window is declared");
    assert!(window.actions.iter().any(|action| action.id == map::SET_CAMERA_ACTION_ID), "{:?}", window.actions.iter().map(|action| action.id.as_str()).collect::<Vec<_>>());
    assert_eq!(GIS_MAP_VIEW_TOOL_IDS, &[map::SET_CAMERA_ACTION_ID]);
    assert_eq!(<GisMapViewCommand as protocol::OpBinary>::TOOL_JOB_IDS, GIS_MAP_VIEW_TOOL_IDS);
    for window in &def.window_kinds {
        for action in semio_framework::window_kind_actions(&def, window) {
            assert!(!matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "viewer window {} declares the mutating action {}", window.id, action.id);
        }
    }
}

/// ⚖️ LAW: the camera route joins its exact registered factory. A registry-backed viewer PANICS at
/// construction (`interactive-job.catalog-authority` / `-incomplete`) whenever the roster, the
/// publication contract, the `Migrated` classification and `TOOL_JOB_IDS` disagree, and every
/// declared lane is a window-config lane.
#[semio_framework_async_macros::async_test]
async fn the_camera_route_joins_its_exact_retained_factory_on_the_window_config_lane() {
    use semio_framework_plugin::{ArtifactOwnedToolJobFactory, PluginApp};
    let contracts = <GisMapViewCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0].lanes, &[ArtifactToolPublicationLane::WindowConfig], "a viewer may never name the artifact lane");
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_gismap_viewer());
    let mut app = semio_framework_plugin::VcsArtifactApp::<ViewerApp<GisMapViewer>, semio_s_artifact_stdio_semio::SemioMembers>::with_registry(ViewerApp::<GisMapViewer>::default(), registry).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
    assert!(app.close_terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_camera_gesture_becomes_an_addressed_window_config_write_and_nothing_else() {
    let command = GisMapViewCommand::SetCamera { camera: "{\"x\":1250.5,\"y\":-830.25,\"zoom\":3.5}".into() };
    let emit = camera_emit(&command, Some(&map_view(map::WINDOW_KIND_ID))).expect("the camera is admissible");
    assert_eq!(emit.window_config_mutations.len(), 1, "one addressed window-config write");
    assert!(emit.artifact_mutations.is_empty(), "a viewer NEVER emits a document mutation");
    assert!(emit.config_mutations.is_empty());
    assert!(emit.effects.is_empty());
    assert_eq!(emit.coalesce_key.as_deref(), Some("gis.map.viewer.camera:map-1"), "a burst of debounced pan ticks collapses per window instance");
}

#[semio_framework_async_macros::async_test]
async fn a_camera_gesture_without_a_concrete_map_window_is_refused_rather_than_written_anywhere() {
    let command = GisMapViewCommand::SetCamera { camera: "{\"x\":1.0,\"y\":1.0,\"zoom\":1.0}".into() };
    assert!(camera_emit(&command, None).is_err(), "a camera is addressed at ONE window instance");
    assert!(camera_emit(&command, Some(&map_view("gis2d-view-other"))).is_err(), "a camera addressed at another window kind must not write the map pane");
    assert!(camera_emit(&GisMapViewCommand::default(), Some(&map_view(map::WINDOW_KIND_ID))).is_err(), "an empty camera is a refusal, never a silent no-op");
    assert!(camera_emit(&GisMapViewCommand::SetCamera { camera: "{\"x\":1.0,\"y\":1.0,\"zoom\":0.0}".into() }, Some(&map_view(map::WINDOW_KIND_ID))).is_err(), "a zero zoom is not a camera");
}

/// ⚖️ LAW: the args bridge takes the host's `camera` object (the `TiledMapHost` sends `{x,y,zoom}`) or
/// its JSON text, canonicalizes it, and refuses a foreign action.
#[semio_framework_async_macros::async_test]
async fn the_args_bridge_canonicalizes_the_hosts_camera_and_refuses_a_foreign_action() {
    for raw in ["{\"surfaceId\":\"gis2d.view.composite\",\"camera\":{\"x\":12.0,\"y\":-4.5,\"zoom\":2.0}}", "{\"camera\":\"{\\\"x\\\":12.0,\\\"y\\\":-4.5,\\\"zoom\\\":2.0}\"}"] {
        let args = dsl::json::from_json_str::<dsl::DslValue>(raw).expect("args");
        let command = command_from_action(map::SET_CAMERA_ACTION_ID, Some(&args)).expect("the bridge accepts the host's camera");
        assert_eq!(command, GisMapViewCommand::SetCamera { camera: map::config::GisMapViewerCamera { x: 12.0, y: -4.5, zoom: 2.0 }.scene_camera_json() });
        assert_eq!(GisMapViewer::command_id(&command), map::SET_CAMERA_ACTION_ID);
    }
    assert!(command_from_action("addFeature", None).is_err(), "the viewer has no document verb to bridge to");
}

#[semio_framework_async_macros::async_test]
async fn the_view_command_round_trips_through_its_binary_codec() {
    use protocol::OpBinary;
    let command = GisMapViewCommand::SetCamera { camera: "{\"x\":1.0,\"y\":2.0,\"zoom\":3.0}".into() };
    assert_eq!(GisMapViewCommand::decode_op(&command.encode_op().expect("encode")).expect("decode"), command);
}
/// ⚖️ LAW: a pan dispatched exactly as the `TiledMapHost` sends it (`{surfaceId, camera:{x,y,zoom}}`
/// from one map window) settles through the viewer's retained route into THAT window's config, and the
/// window's next render publishes the retained camera.
#[semio_framework_async_macros::async_test]
async fn a_dispatched_pan_is_retained_by_its_window_and_rendered_back() {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app_with_members::<ViewerApp<GisMapViewer>, semio_s_artifact_stdio_semio::SemioMembers, _>(async { semio_framework_plugin::App { definition: create_gismap_viewer(), examples: Vec::new() } }).await;
    let view = map_view(map::WINDOW_KIND_ID);
    let mut meta = semio_framework_plugin::artifact_app_laws::meta("local");
    meta.view_state = Some(view.clone());
    let args = dsl::json::from_json_str::<dsl::DslValue>("{\"surfaceId\":\"gis2d.view.composite\",\"camera\":{\"x\":12.0,\"y\":-4.5,\"zoom\":2.0}}").expect("args");
    app.handle_action(map::SET_CAMERA_ACTION_ID, Some(&args), &meta).await.expect("the pan is admitted");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("the pan settles");
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(map::BODY_KEY, None, &view).await.expect("render")).expect("projection");
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::TiledMapScene>(&projection).expect("tiled map scene");
    assert_eq!(scene.camera_json, map::config::GisMapViewerCamera { x: 12.0, y: -4.5, zoom: 2.0 }.scene_camera_json());
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🧭️ViewerCamera
