use super::*;
use crate::editor::shooting::testkit::{dispatch, shooting_app, shooting_app_with_registry, ShootingApp};
use semio_framework_plugin::app::EditorApp;
use semio_framework_plugin::testkit;
use semio_framework_plugin::{Effect, PluginApp};
use serde_json::{json, Value};

fn default_camera(position: [f64; 3]) -> crate::ShootingCamera {
    crate::ShootingCamera { position, target: [0.0, 0.0, 0.0], zoom: 1.0, fov: 50.0, up: None, projection: None }
}

//#region 🧵️RetainedCatalogOracle
#[derive(Debug, PartialEq, Eq)]
struct ShootingRetainedCatalogSummary {
    routes: usize,
    bounded: usize,
    resumable: usize,
    migrated: usize,
    fail_closed: usize,
    unique: bool,
    route_ids: std::collections::BTreeSet<String>,
    bounded_ids: std::collections::BTreeSet<String>,
    host_only_ids: std::collections::BTreeSet<String>,
}

trait ShootingRetainedCatalogOracle {
    fn summarize(&self, fixture: &str) -> ShootingRetainedCatalogSummary;
}

struct SerdeJsonShootingRetainedCatalogOracle;

impl ShootingRetainedCatalogOracle for SerdeJsonShootingRetainedCatalogOracle {
    fn summarize(&self, fixture: &str) -> ShootingRetainedCatalogSummary {
        let document: Value = serde_json::from_str(fixture).expect("language-neutral retained catalog fixture");
        let routes = document.get("routes").and_then(Value::as_array).expect("routes array");
        let bounded = routes.iter().filter(|route| route.get("execution").and_then(Value::as_str) == Some("bounded")).count();
        let resumable = routes.iter().filter(|route| route.get("execution").and_then(Value::as_str) == Some("resumable")).count();
        let migrated = routes.iter().filter(|route| route.get("admission").and_then(Value::as_str) == Some("migrated")).count();
        let fail_closed = routes.iter().filter(|route| route.get("admission").and_then(Value::as_str) == Some("failClosed")).count();
        let route_ids = routes.iter().filter_map(|route| route.get("id").and_then(Value::as_str).map(str::to_string)).collect::<std::collections::BTreeSet<_>>();
        let bounded_ids = routes.iter().filter(|route| route.get("execution").and_then(Value::as_str) == Some("bounded")).filter_map(|route| route.get("id").and_then(Value::as_str).map(str::to_string)).collect::<std::collections::BTreeSet<_>>();
        let host_only_ids = document
            .get("publicationContracts")
            .and_then(Value::as_array)
            .expect("publication contracts array")
            .iter()
            .filter(|contract| contract.get("lanes").and_then(Value::as_array).is_some_and(|lanes| lanes.as_slice() == [Value::String("hostOnly".into())]))
            .filter_map(|contract| contract.get("toolId").and_then(Value::as_str).map(str::to_string))
            .collect::<std::collections::BTreeSet<_>>();
        ShootingRetainedCatalogSummary { routes: routes.len(), bounded, resumable, migrated, fail_closed, unique: route_ids.len() == routes.len(), route_ids, bounded_ids, host_only_ids }
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_command_catalog_matches_the_serde_json_oracle() {
    let oracle = SerdeJsonShootingRetainedCatalogOracle.summarize(include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json"));
    let mut command_ids = every_command().iter().map(|command| shooting_command_id(command).to_string()).collect::<std::collections::BTreeSet<_>>();
    command_ids.insert("exportActiveShot".to_string());
    let bounded_ids = SHOOTING_BOUNDED_TOOL_IDS.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let host_only_ids = <ShootingCommandJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .filter(|contract| contract.lanes == [semio_framework_plugin::ArtifactToolPublicationLane::HostOnly])
        .map(|contract| contract.tool_id.to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let subject = ShootingRetainedCatalogSummary {
        routes: command_ids.len(),
        bounded: bounded_ids.len(),
        resumable: command_ids.difference(&bounded_ids).count(),
        migrated: bounded_ids.len(),
        fail_closed: command_ids.difference(&bounded_ids).count(),
        unique: bounded_ids.len() == SHOOTING_BOUNDED_TOOL_IDS.len() && bounded_ids.is_subset(&command_ids),
        route_ids: command_ids.clone(),
        bounded_ids: bounded_ids.clone(),
        host_only_ids,
    };
    assert_eq!(oracle, ShootingRetainedCatalogSummary { routes: 39, bounded: 2, resumable: 37, migrated: 2, fail_closed: 37, unique: true, route_ids: command_ids, bounded_ids: bounded_ids.clone(), host_only_ids: bounded_ids });
    assert_eq!(subject, oracle);
}

#[semio_framework_async_macros::async_test]
async fn retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures() {
    let fixture = include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json");
    let expected = SHOOTING_BOUNDED_TOOL_IDS.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let wrong_lane = fixture.replacen("\"hostOnly\"", "\"artifact\"", 1);
    let wrong_tool = fixture.replacen("\"loadRequest\"", "\"forgedRequest\"", 1);
    assert_ne!(SerdeJsonShootingRetainedCatalogOracle.summarize(&wrong_lane).host_only_ids, expected);
    assert_ne!(SerdeJsonShootingRetainedCatalogOracle.summarize(&wrong_tool).host_only_ids, expected);
}
//#endregion 🧵️RetainedCatalogOracle

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
/// hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_across_every_row() {
    let ids: Vec<&str> = every_command().iter().map(shooting_command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 38, "every ShootingCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<ShootingCommand> {
    vec![
        ShootingCommand::ImportSnapshotJson(import_snapshot_json::ImportSnapshotJson { json: "{\"schema\":\"shooting.shooting\"}".into() }),
        ShootingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "base-icon".into() }),
        ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some("s1".into()) }),
        ShootingCommand::SetActiveAsset(set_active_asset::SetActiveAsset { asset_id: Some("a1".into()) }),
        ShootingCommand::SetShotCamera(set_shot_camera::SetShotCamera { shot_id: "s1".into(), camera: default_camera([1.0, 2.0, 3.0]) }),
        ShootingCommand::SaveCamera(save_camera::SaveCamera {}),
        ShootingCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
        ShootingCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
        ShootingCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 2.4 }),
        ShootingCommand::SetAmbientIntensity(set_ambient_intensity::SetAmbientIntensity { value: 1.15 }),
        ShootingCommand::SetMaterialRoughness(set_material_roughness::SetMaterialRoughness { value: 0.5 }),
        ShootingCommand::SetShadowEnabled(set_shadow_enabled::SetShadowEnabled { value: true }),
        ShootingCommand::ToggleSun(toggle_sun::ToggleSun { value: false }),
        ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Overview".into() }),
        ShootingCommand::SetActiveShotFormat(set_active_shot_format::SetActiveShotFormat { value: "png".into() }),
        ShootingCommand::SetActiveShotShape(set_active_shot_shape::SetActiveShotShape { value: "ellipse".into() }),
        ShootingCommand::PatchShots(patch_shots::PatchShots { shot_ids: vec!["s1".into(), "s2".into()], field: "label".into(), value: "Hero".into() }),
        ShootingCommand::PatchAssets(patch_assets::PatchAssets { asset_ids: vec!["a1".into()], field: "name".into(), value: "Renamed".into() }),
        ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "rectangle".into() }),
        ShootingCommand::AddAsset(add_asset::AddAsset { format: "glb".into() }),
        ShootingCommand::ImportAsset(import_asset::ImportAsset { payload: "data:model/gltf-binary;base64,AAA=".into(), name: Some("Imported".into()) }),
        ShootingCommand::ResetSnapshot(reset_snapshot::ResetSnapshot {}),
        ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec!["a1".into(), "a2".into()], dx: 1.0, dy: -2.0, dz: 3.5 }),
        ShootingCommand::RotateSelection(rotate_selection::RotateSelection { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }),
        ShootingCommand::ScaleSelection(scale_selection::ScaleSelection { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
        ShootingCommand::SetCamera(set_camera::SetCamera { camera: default_camera([9.0, 9.0, 9.0]) }),
        ShootingCommand::LoadSavedCamera(load_saved_camera::LoadSavedCamera { id: "cam1".into() }),
        ShootingCommand::SetCameraDraftLabel(set_camera_draft_label::SetCameraDraftLabel { value: "Hero".into() }),
        ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(true) }),
        ShootingCommand::SetShotSelection(set_shot_selection::SetShotSelection { shot_ids: vec!["s1".into()] }),
        ShootingCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {}),
        ShootingCommand::WorldPointerMove(world_pointer_move::WorldPointerMove {}),
        ShootingCommand::SaveDownload(save_download::SaveDownload {}),
        ShootingCommand::LoadRequest(load_request::LoadRequest {}),
        ShootingCommand::ImportAssetRequest(import_asset_request::ImportAssetRequest {}),
        ShootingCommand::ExportShots(export_shots::ExportShots { all: true }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_shooting_app()).expect("app definition json");
    for id in [SHOOTING_PLAY_WINDOW_SCENE, SHOOTING_PLAY_WINDOW_ICON] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for body in [SHOOTING_PLAY_BODY_DOCUMENT, SHOOTING_PLAY_BODY_CATALOGUE, SHOOTING_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("2d.shooting"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_scopes_transform_gumball_and_actions_are_declared() {
    let definition = create_shooting_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["move", "rotate", "scale"], "gumball utilities declared in registry order");
    assert!(definition.utilities.iter().all(|utility| utility.group.as_deref() == Some("transform")), "one exclusive transform group");
    let scene = definition.window_kinds.iter().find(|window| window.id == SHOOTING_PLAY_WINDOW_SCENE).expect("scene window");
    let scoped: Vec<&str> = scene.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(scoped, ["move", "rotate", "scale"], "utilities scoped to the scene window kind");
    for command in ["loadRequest", "importAssetRequest", "saveDownload", "exportActiveShot", "exportAllShots", "resetFixture", "saveCamera"] {
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == command), "registry declares {command}");
    }
    let mut app = shooting_app().await;
    let engagements = app.window_engagements(&semio_framework_plugin::ViewModel::default()).await;
    assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].options.is_none(), "the gumball selector moved to the host-derived utility bar");
    assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].status.as_ref().unwrap()[0].text.contains("assets"));
    assert!(engagements[SHOOTING_PLAY_WINDOW_ICON].status.as_ref().unwrap()[0].text.contains("256×256"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: replaces the deleted
/// `world_pick_is_declared_as_a_view_action_and_emits_no_operations` test — asset pick/select is the
/// framework-injected `interactionSelect` verb now (no app-declared action id), asserted here
/// instead of a bespoke `worldPick` action.
#[semio_framework_async_macros::async_test]
async fn interaction_select_is_reachable_as_a_framework_injected_action_under_registry_enforcement() {
    let mut app = shooting_app_with_registry().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let targets = serde_json::to_string(&serde_json::json!([{ "granularity": "asset", "id": asset_id }])).unwrap();
    app.handle_action("interactionSelect", Some(&dsl::os_pack::json::to_dsl_value(&dsl::json!({ "domainId": SHOOTING_INTERACTION_DOMAIN, "targets": targets.as_str(), "merge": "replace" }))), &testkit::meta("local")).await.expect("interactionSelect");
}

#[semio_framework_async_macros::async_test]
async fn assets_interaction_domain_is_declared_and_scoped_to_the_scene_window() {
    let definition = create_shooting_app();
    let domain = definition.interactions.iter().find(|interaction| interaction.id == SHOOTING_INTERACTION_DOMAIN).expect("assets interaction domain declared");
    assert_eq!(domain.granularities.len(), 1);
    assert_eq!(domain.granularities[0].id, "asset");
    assert!(matches!(domain.hierarchy, HierarchyProvider::Flat));
    let scene = definition.window_kinds.iter().find(|window| window.id == SHOOTING_PLAY_WINDOW_SCENE).expect("scene window");
    assert!(scene.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == SHOOTING_INTERACTION_DOMAIN));
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Locale
#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_native_english_by_default() {
    let mut app = shooting_app().await;
    let document_json = crate::editor::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT).await;
    assert!(document_json.contains("Shots"));
    assert!(document_json.contains("Assets"));
    let catalogue_json = crate::editor::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_CATALOGUE).await;
    assert!(catalogue_json.contains("Add Shot"));
    assert!(catalogue_json.contains("SVG Rectangle"));
    let engagements = app.window_engagements(&semio_framework_plugin::ViewModel::default()).await;
    assert_eq!(engagements[SHOOTING_PLAY_WINDOW_SCENE].input.as_ref().unwrap().placeholder.as_deref(), Some("Camera label"));
    assert_eq!(engagements[SHOOTING_PLAY_WINDOW_ICON].input.as_ref().unwrap().placeholder.as_deref(), Some("Shot label"));
}

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_native_german() {
    let mut app = shooting_app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let document_json = testkit::project_and_retire_fixture_tree(app.render(SHOOTING_PLAY_BODY_DOCUMENT, None, &view_state).await.expect("render document")).expect("retire document tree");
    assert!(document_json.contains("Aufnahmen"));
    assert!(document_json.contains("Objekte"));
    let engagements = app.window_engagements(&view_state).await;
    assert_eq!(engagements[SHOOTING_PLAY_WINDOW_SCENE].input.as_ref().unwrap().placeholder.as_deref(), Some("Kamera-Bezeichnung"));
    assert_eq!(engagements[SHOOTING_PLAY_WINDOW_ICON].input.as_ref().unwrap().placeholder.as_deref(), Some("Aufnahme-Bezeichnung"));
}
//#endregion 🔖️Locale

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = shooting_app().await;
    testkit::assert_undo_redo_round_trip(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "png".into(), shape: "rectangle".into() }), |app| app.snapshot().expect("snapshot").shots.len(), 2, 3).await;
}

/// 🎥️ `SetCamera` is config-only — dragging the viewport camera through several ticks must never
/// create a VCS edit/undo step on the DOCUMENT store at all.
#[semio_framework_async_macros::async_test]
async fn camera_drag_never_creates_a_document_undo_step() {
    let mut app = shooting_app().await;
    for position in [[1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0]] {
        dispatch(&mut app, ShootingCommand::SetCamera(set_camera::SetCamera { camera: default_camera(position) })).await;
    }
    async fn camera_position(app: &mut ShootingApp) -> Value {
        let scene = crate::editor::shooting::testkit::world_scene(app).await;
        let camera: Value = serde_json::from_str(&scene.camera_json).unwrap();
        camera["position"].clone()
    }
    assert_eq!(camera_position(&mut app).await, json!([3.0, 0.0, 0.0]), "config camera reflects the last drag tick");
    app.handle_action("undo", None, &testkit::meta("local")).await.expect("undo (no-op: nothing on the document store to undo)");
    assert_eq!(camera_position(&mut app).await, json!([3.0, 0.0, 0.0]), "document undo has nothing to revert — the drag never touched the document");
}

/// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
/// apply DISJOINT edits, and exchanging operations over a `MemoryBackbone` converges both sides to
/// contain BOTH edits.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    testkit::assert_two_instances_converge::<EditorApp<ShootingPlayApp>, (String, [f64; 3])>(
        "mem://shooting-convergence",
        ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Renamed By A".into() }),
        ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec!["base".into()], dx: 5.0, dy: 6.0, dz: 7.0 }),
        |app| {
            let snapshot = app.snapshot().expect("snapshot");
            (crate::standards::v1::subsets::any::schema::active_shot(&snapshot).unwrap().label.clone(), snapshot.assets[0].origin)
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_shooting() {
    testkit::assert_ingest_idempotent::<EditorApp<ShootingPlayApp>, String>(ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero".into() }), |app| {
        crate::standards::v1::subsets::any::schema::active_shot(&app.snapshot().expect("snapshot")).unwrap().label.clone()
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = shooting_app().await;
    assert!(crate::editor::shooting::testkit::render(&mut app, "shooting.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️CrossCutting

//#region 🔖️Io
#[semio_framework_async_macros::async_test]
async fn shooting_io_mirrors_the_declared_artifact_kind() {
    let io = shooting_io();
    assert_eq!(io.document_schema, "shooting.scene");
    assert_eq!(io.artifact.id, "2d.shooting");
    // 🗂️ `AppIo` has no `export_stdio_kinds`/`import_stdio_kinds` string peer (see `shooting_io`'s
    // doc comment) — the real format list lives on `artifact_kind()` instead, asserted below.
    assert_eq!(io.export_formats.len(), 0);
    assert_eq!(io.import_formats.len(), 0);
    let kind = crate::artifact_kind();
    assert_eq!(kind.export_stdio_kinds, kind.import_stdio_kinds);
    assert!(kind.export_stdio_kinds.iter().any(|kind| kind == "stdio.svg"));
    assert!(kind.export_stdio_kinds.iter().any(|kind| kind == "stdio.png"));
}

/// 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe:
/// `photos:out` is declared, optional/`Many`, and pinned to the `2d.image` kind.
#[semio_framework_async_macros::async_test]
async fn shooting_io_declares_the_photos_out_port() {
    let io = shooting_io();
    let port = io.ports.iter().find(|port| port.id == "photos:out").expect("photos:out declared");
    assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(port.kind_id.as_deref(), Some("2d.image"));
    assert!(!port.required);
    assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
    assert_eq!(port.media_type.class, MediaClass::TwoD);
    assert_eq!(port.media_type.form, MediaForm::Raster);
}

/// 🖼️ `shooting_photo_media` renders the same scene as `exportActiveShot`'s PNG (base64, non-empty).
#[semio_framework_async_macros::async_test]
async fn shooting_photo_media_exports_a_raster_2d_image() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let media = shooting_photo_media(&snapshot).expect("photo export succeeds");
    assert_eq!(media.media_type.class, MediaClass::TwoD);
    assert_eq!(media.media_type.form, MediaForm::Raster);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "2d.image");
            assert!(!json.is_empty());
        }
        MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
    }
}
//#endregion 🔖️Io

//#region 🔖️Export
#[semio_framework_async_macros::async_test]
async fn export_import_and_download_operations() {
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {})).await;
    match &result.requested_effects[0] {
        Effect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "importSnapshotJson"),
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
    let result = dispatch(&mut app, ShootingCommand::SaveDownload(save_download::SaveDownload {})).await;
    match &result.requested_effects[0] {
        Effect::DownloadMediaExport { filename, data, .. } => {
            assert_eq!(filename, "shooting.shooting.ops");
            let round_trip: ShootingSnapshot = dsl::os_pack::from_json_str(data).unwrap();
            assert_eq!(round_trip.schema, SHOOTING_DOCUMENT_SCHEMA);
        }
        other => panic!("expected DownloadMediaExport, got {other:?}"),
    }
}
//#endregion 🔖️Export
