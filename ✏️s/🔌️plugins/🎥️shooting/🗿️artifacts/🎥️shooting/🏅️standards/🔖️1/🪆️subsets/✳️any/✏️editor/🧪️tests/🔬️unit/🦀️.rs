pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::app::EditorApp;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::{Effect, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowMeasure};

    /// 🧪️ The registered, MOUNTED fixture app every editor test drives — bound to
    /// [`SHOOTING_TEST_INSTANCE`] so typed commands reach their retained routes, and retired through the
    /// framework's exact close loop when it goes out of scope (the store's `Drop` demands the
    /// terminal-empty witness). `ShootingPlayApp` implements the AUTHORING trait `ArtifactEditor`;
    /// `EditorApp<ShootingPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
    /// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<ShootingPlayApp>` builds it.
    pub struct ShootingApp(VcsArtifactApp<EditorApp<ShootingPlayApp>>);

    impl std::ops::Deref for ShootingApp {
        type Target = VcsArtifactApp<EditorApp<ShootingPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for ShootingApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for ShootingApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
            }
        }
    }

    /// 🧪️ The instance id every `meta("local")` dispatch is stamped with.
    pub const SHOOTING_TEST_INSTANCE: u32 = 1;

    /// ✏️ Adapts `create_shooting_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` still
    /// expect — framework test context gap, not modifiable here (`🧰️framework/**` is outside this packet's
    /// lease).
    pub fn shooting_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_shooting_app(), examples: Vec::new() }
    }

    /// 🧪️ Registry-backed and bound — the bare `new_app` carries an empty `AppActionRegistry`, and
    /// `tool_job_registration` admits a `bounded_first_step_tool_proofs!` row only when its id is
    /// `Migrated` in the LIVE registry, so a bare instance faults at construction with
    /// `interactive-job.catalog-authority` now that every verb is a bounded tool.
    pub async fn shooting_app() -> ShootingApp {
        let mut app = new_app_with_registry::<EditorApp<ShootingPlayApp>>(shooting_app_manifest_for_tests).await;
        app.bind_instance_id(SHOOTING_TEST_INSTANCE).await;
        ShootingApp(app)
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn shooting_app_with_registry() -> ShootingApp {
        shooting_app().await
    }

    /// 🔁️ A mounted app answers before its retained typed operation has published: drive it home the
    /// way the plugin host's continuation does, fold the settled receipt's effects into the answer and
    /// apply any `LoadDocument` the way the host would.
    pub async fn dispatch(app: &mut ShootingApp, command: ShootingCommand) -> Dispatched {
        let mut result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, SHOOTING_TEST_INSTANCE).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        for effect in &result.requested_effects {
            if let Effect::LoadDocument { pack, spr } = effect {
                let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                app.load_document_pack(&files).await.expect("test host applies load-document effect");
            }
        }
        Dispatched { result, lanes: settled.lanes }
    }

    /// 🧾️ A settled dispatch: the immediate answer plus the store lanes the retained publication
    /// actually wrote (a mounted app publishes AFTER answering, so `result.mutations` is always empty).
    pub struct Dispatched {
        pub result: InvocationResult,
        pub lanes: Vec<TypedOperationResultLane>,
    }

    impl Dispatched {
        /// 📝️ Whether the settled publication wrote the document lane.
        pub fn edited_document(&self) -> bool {
            self.lanes.contains(&TypedOperationResultLane::Artifact)
        }
    }

    impl std::ops::Deref for Dispatched {
        type Target = InvocationResult;
        fn deref(&self) -> &Self::Target {
            &self.result
        }
    }

    /// ↩️ Runs a framework-reserved history verb (`undo`/`redo`) and settles its publication.
    pub async fn history_verb(app: &mut ShootingApp, verb: &str) {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, verb, SHOOTING_TEST_INSTANCE).await;
    }

    pub async fn render(app: &mut ShootingApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("project and retire semantic tree")
    }
    
    pub async fn world_scene(app: &mut ShootingApp) -> semio_framework_plugin::World3dScene {
        let tree = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewModel::default()).await.expect("render scene");
        let decoded = semio_framework_plugin::artifact_app_laws::built_surface_scene(&tree.root);
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire scene tree");
        decoded.expect("assembled 3D scene")
    }
    
    pub async fn icon_scene(app: &mut ShootingApp) -> semio_framework_plugin::IconRenderScene {
        let tree = app.render(SHOOTING_PLAY_BODY_ICON, None, &ViewModel::default()).await.expect("render icon");
        let decoded = match &tree.root.component {
            semio_framework_plugin::Component::Surface(props) => semio_framework_ui_scene::decode(props),
            _ => panic!("icon surface"),
        };
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire icon tree");
        decoded.expect("packed icon scene")
    }
    
    /// 🪟️ The two default-layout panes — window chrome (`window_engagements`/`window_measures`) is keyed
    /// by window INSTANCE and only projected for instances the view state carries.
    pub fn view(locale: semio_framework_plugin::Locale) -> ViewModel {
        ViewModel {
            locale,
            window_id: Some(SHOOTING_PLAY_WINDOW_SCENE.into()),
            window_instances: vec![
                ViewWindowInstance { id: SHOOTING_PLAY_WINDOW_SCENE.into(), window_kind_id: SHOOTING_PLAY_WINDOW_SCENE.into() },
                ViewWindowInstance { id: SHOOTING_PLAY_WINDOW_ICON.into(), window_kind_id: SHOOTING_PLAY_WINDOW_ICON.into() },
            ],
            ..Default::default()
        }
    }

    pub async fn scene_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
        app.window_measures(&view(semio_framework_plugin::Locale::En)).await.get(SHOOTING_PLAY_WINDOW_SCENE).cloned().expect("scene window measures")
    }
    
    pub async fn icon_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
        app.window_measures(&view(semio_framework_plugin::Locale::En)).await.get(SHOOTING_PLAY_WINDOW_ICON).cloned().expect("icon window measures")
    }
}

use super::*;
use context::{dispatch, shooting_app, shooting_app_with_registry, ShootingApp};
use semio_framework_plugin::app::EditorApp;
use semio_framework_plugin::artifact_app_laws;
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
        let migrated = routes.iter().filter(|route| route.get("admission").and_then(Value::as_str) == Some("Migrated")).count();
        let fail_closed = routes.iter().filter(|route| route.get("admission").and_then(Value::as_str) == Some("FailClosed")).count();
        let route_ids = routes.iter().filter_map(|route| route.get("id").and_then(Value::as_str).map(str::to_string)).collect::<std::collections::BTreeSet<_>>();
        let bounded_ids = routes.iter().filter(|route| route.get("execution").and_then(Value::as_str) == Some("bounded")).filter_map(|route| route.get("id").and_then(Value::as_str).map(str::to_string)).collect::<std::collections::BTreeSet<_>>();
        let host_only_ids = document
            .get("publicationContracts")
            .and_then(Value::as_array)
            .expect("publication contracts array")
            .iter()
            .filter(|contract| contract.get("lanes").and_then(Value::as_array).is_some_and(|lanes| lanes.as_slice() == [Value::String("HostOnly".into())]))
            .filter_map(|contract| contract.get("toolId").and_then(Value::as_str).map(str::to_string))
            .collect::<std::collections::BTreeSet<_>>();
        ShootingRetainedCatalogSummary { routes: routes.len(), bounded, resumable, migrated, fail_closed, unique: route_ids.len() == routes.len(), route_ids, bounded_ids, host_only_ids }
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_command_catalog_matches_the_serde_json_oracle() {
    let oracle = SerdeJsonShootingRetainedCatalogOracle.summarize(include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json"));
    let mut command_ids = every_command().iter().map(|command| command.command_id().to_string()).collect::<std::collections::BTreeSet<_>>();
    command_ids.insert("setActiveUtility".to_string());
    let bounded_ids = SHOOTING_BOUNDED_TOOL_IDS.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let host_only_ids = <ShootingCommandJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .filter(|contract| contract.lanes == [ArtifactToolPublicationLane::HostOnly])
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
        host_only_ids: host_only_ids.clone(),
    };
    assert_eq!(oracle, ShootingRetainedCatalogSummary { routes: 38, bounded: 37, resumable: 1, migrated: 37, fail_closed: 1, unique: true, route_ids: command_ids, bounded_ids, host_only_ids });
    assert_eq!(subject, oracle);
}

#[semio_framework_async_macros::async_test]
async fn retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures() {
    let fixture = include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json");
    let expected = ["importSnapshotJson", "setActiveExample", "resetFixture", "worldPointerDown", "worldPointerMove", "saveDownload", "loadRequest", "importAssetRequest", "exportActiveShot", "exportAllShots"].iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(SerdeJsonShootingRetainedCatalogOracle.summarize(fixture).host_only_ids, expected);
    let wrong_lane = fixture.replacen("\"HostOnly\"", "\"Artifact\"", 1);
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
    let ids: Vec<&str> = every_command().iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 37, "every ShootingCommand row must be covered by every_command()");
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
        ShootingCommand::ExportActiveShot(export_active_shot::ExportActiveShot {}),
        ShootingCommand::ExportAllShots(export_all_shots::ExportAllShots {}),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ActionBridge
fn action_args(value: &Value) -> DslValue {
    dsl::os_pack::json_to_dsl_value(&dsl::os_pack::json::parse(&value.to_string()).expect("fixture JSON"))
}

/// 🐫️ The shell's spelling of the payload keys.
fn camel_case_keys(value: &DslValue) -> DslValue {
    match value {
        DslValue::Object(entries) => DslValue::Object(
            entries
                .iter()
                .map(|(key, value)| {
                    let mut camel = String::new();
                    let mut upper = false;
                    for ch in key.chars() {
                        if ch == '_' { upper = true; } else if upper { camel.push(ch.to_ascii_uppercase()); upper = false; } else { camel.push(ch); }
                    }
                    (camel, value.clone())
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

/// 🌉️ Every command row the shells reach by action id must decode through `command_from_action`
/// (camelCase host keys → the payloads' own snake_case `FromValue` names), and its `command_id` must
/// round-trip — the boundary that was missing entirely before ticket 26/09/16/SHOOTING-PLUGIN-END-TO-END
/// (every shell action was refused as "not framework-reserved").
#[test]
fn command_from_action_round_trips_every_command_id() {
    for command in every_command() {
        let id = command.command_id();
        let args = dsl::ToValue::to_value(&command);
        let payload = match &args {
            DslValue::Object(entries) if entries.len() == 1 => entries[0].1.clone(),
            other => other.clone(),
        };
        let bridged = ShootingPlayApp::command_from_action(id, Some(&camel_case_keys(&payload))).unwrap_or_else(|error| panic!("action {id} failed to bridge: {}", error.message));
        assert_eq!(bridged.command_id(), id, "command_id mismatch for action {id}");
    }
    assert!(ShootingPlayApp::command_from_action("nonsense", None).is_err());
}

/// 🎛️ The host control contracts (`value`/`pressed`/gumball `ids`/viewport `camera`) reach the rows.
#[test]
fn command_from_action_bridges_host_control_contracts() {
    let args = |json: Value| action_args(&json);
    assert_eq!(ShootingPlayApp::command_from_action("setActiveShot", Some(&args(json!({ "value": "s2" })))).expect("select bridge"), ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some("s2".into()) }));
    assert_eq!(ShootingPlayApp::command_from_action("toggleSun", Some(&args(json!({ "pressed": true })))).expect("toggle bridge"), ShootingCommand::ToggleSun(toggle_sun::ToggleSun { value: true }));
    assert_eq!(ShootingPlayApp::command_from_action("setCenterModel", Some(&args(json!({ "pressed": false })))).expect("toggle bridge"), ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(false) }));
    assert_eq!(ShootingPlayApp::command_from_action("setSunAzimuth", Some(&args(json!({ "value": 90 })))).expect("slider bridge"), ShootingCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }));
    assert_eq!(
        ShootingPlayApp::command_from_action("patchShots", Some(&args(json!({ "field": "width", "shotIds": ["s1"], "value": 512 })))).expect("inspector bridge"),
        ShootingCommand::PatchShots(patch_shots::PatchShots { shot_ids: vec!["s1".into()], field: "width".into(), value: "512".into() })
    );
    assert_eq!(ShootingPlayApp::command_from_action("addShot", Some(&args(json!({ "format": "svg", "shape": "ellipse" })))).expect("catalogue bridge"), ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() }));
    assert_eq!(ShootingPlayApp::command_from_action("addShot", None).expect("defaults bridge"), ShootingCommand::AddShot(add_shot::AddShot { format: "png".into(), shape: "rectangle".into() }));
    assert_eq!(
        ShootingPlayApp::command_from_action("translateSelection", Some(&args(json!({ "mode": "object", "ids": ["a1"], "dx": 1, "dy": 0, "dz": -2.5 })))).expect("gumball bridge"),
        ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec!["a1".into()], dx: 1.0, dy: 0.0, dz: -2.5 })
    );
    let ShootingCommand::SetCamera(set_camera::SetCamera { camera }) = ShootingPlayApp::command_from_action("setCamera", Some(&args(json!({ "windowId": "w1", "camera": { "position": [1, 2, 3], "target": [0, 0, 0], "zoom": 2 } })))).expect("viewport bridge") else {
        panic!("setCamera must bridge to SetCamera");
    };
    assert_eq!(camera.position, [1.0, 2.0, 3.0]);
    assert_eq!(camera.zoom, 2.0);
    assert_eq!(camera.fov, crate::default_fov());
    assert_eq!(ShootingPlayApp::command_from_action("loadSavedCamera", Some(&args(json!({ "id": "cam1" })))).expect("possible engagement bridge"), ShootingCommand::LoadSavedCamera(load_saved_camera::LoadSavedCamera { id: "cam1".into() }));
    assert_eq!(ShootingPlayApp::command_from_action("setShotSelection", Some(&args(json!({ "shotIds": ["s1", "s2"] })))).expect("tree bridge"), ShootingCommand::SetShotSelection(set_shot_selection::SetShotSelection { shot_ids: vec!["s1".into(), "s2".into()] }));
    assert_eq!(ShootingPlayApp::command_from_action("exportAllShots", None).expect("export bridge"), ShootingCommand::ExportAllShots(export_all_shots::ExportAllShots {}));
}
//#endregion 🔖️ActionBridge

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_shooting_app()).expect("app definition json");
    for id in [SHOOTING_PLAY_WINDOW_SCENE, SHOOTING_PLAY_WINDOW_ICON] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for body in [SHOOTING_PLAY_BODY_ARTIFACT, SHOOTING_PLAY_BODY_CATALOGUE, SHOOTING_PLAY_BODY_INSPECTION] {
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
        assert!(
            definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).any(|action| action.id == command),
            "registry declares {command}"
        );
    }
    let mut app = shooting_app().await;
    let engagements = app.window_engagements(&context::view(semio_framework_plugin::Locale::En)).await;
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
    app.handle_action("interactionSelect", Some(&dsl::os_pack::json::to_dsl_value(&dsl::json!({ "domainId": SHOOTING_INTERACTION_DOMAIN, "targets": targets.as_str(), "merge": "replace" }))), &artifact_app_laws::meta("local")).await.expect("interactionSelect");
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
    let document_json = context::render(&mut app, SHOOTING_PLAY_BODY_ARTIFACT).await;
    assert!(document_json.contains("Shots"));
    assert!(document_json.contains("Assets"));
    let catalogue_json = context::render(&mut app, SHOOTING_PLAY_BODY_CATALOGUE).await;
    assert!(catalogue_json.contains("Add Shot"));
    assert!(catalogue_json.contains("SVG Rectangle"));
    let engagements = app.window_engagements(&context::view(semio_framework_plugin::Locale::En)).await;
    assert_eq!(engagements[SHOOTING_PLAY_WINDOW_SCENE].input.as_ref().unwrap().placeholder.as_deref(), Some("Camera label"));
    assert_eq!(engagements[SHOOTING_PLAY_WINDOW_ICON].input.as_ref().unwrap().placeholder.as_deref(), Some("Shot label"));
}

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_native_german() {
    let mut app = shooting_app().await;
    let view_state = context::view(semio_framework_plugin::Locale::De);
    let document_json = artifact_app_laws::project_and_retire_fixture_tree(app.render(SHOOTING_PLAY_BODY_ARTIFACT, None, &view_state).await.expect("render document")).expect("retire document tree");
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
    artifact_app_laws::assert_undo_redo_round_trip(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "png".into(), shape: "rectangle".into() }), |app| app.snapshot().expect("snapshot").shots.len(), 2, 3).await;
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
        let scene = context::world_scene(app).await;
        let camera: Value = serde_json::from_str(&scene.camera_json).unwrap();
        camera["position"].clone()
    }
    assert_eq!(camera_position(&mut app).await, json!([3.0, 0.0, 0.0]), "config camera reflects the last drag tick");
    context::history_verb(&mut app, "undo").await;
    assert_eq!(camera_position(&mut app).await, json!([3.0, 0.0, 0.0]), "document undo has nothing to revert — the drag never touched the document");
}

/// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
/// apply DISJOINT edits, and exchanging operations over a `MemoryBackbone` converges both sides to
/// contain BOTH edits.
///
/// 🧹️ The REGISTERED pair: shooting publishes bounded tool proofs, so a registry-less `paired_apps`
/// instance faults in the `interactive-job.catalog-authority` proof join (`generated_migrated=false`,
/// `migrated={}`) while it is constructed, before any edit lands.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<ShootingPlayApp>, (String, [f64; 3]), _, _>(
        "mem://shooting-convergence",
        || async { context::shooting_app_manifest_for_tests() },
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
    artifact_app_laws::assert_registered_ingest_idempotent::<EditorApp<ShootingPlayApp>, String, _, _>(
        || async { context::shooting_app_manifest_for_tests() },
        ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero".into() }),
        |app| crate::standards::v1::subsets::any::schema::active_shot(&app.snapshot().expect("snapshot")).unwrap().label.clone(),
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = shooting_app().await;
    assert!(context::render(&mut app, "shooting.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️CrossCutting

//#region 🔖️Io
#[semio_framework_async_macros::async_test]
async fn shooting_io_mirrors_the_declared_artifact_kind() {
    let io = shooting_io();
    assert_eq!(io.artifact_schema, "shooting.scene");
    assert_eq!(io.artifact.id, "2d.shooting");
    // 🗂️ `AppIo` has no `export_stdio_kinds`/`import_stdio_kinds` string peer (see `shooting_io`'s
    // doc comment) — the real format list lives on `artifact_kind()` instead, asserted below.
    assert_eq!(io.export_formats.len(), 0);
    assert_eq!(io.import_formats.len(), 0);
    let kind = crate::artifact_kind();
    assert_eq!(kind.export_stdio_kinds, kind.import_stdio_kinds);
    assert_eq!(kind.export_stdio_kinds, vec!["stdio.json".to_string()], "rendered shots leave through `photos:out`; a scene document is not an image file");
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

//#region 🔖️ExampleLoad
/// 🚪️ The browser host answers `setActiveExample` by handing the published `LoadDocument` back through
/// the document archive door. Without a retained initialization job the store refused the envelope with
/// `artifact-store.persisted-initializer-refused`; every offered example must settle `Ready` and become
/// the live document.
#[semio_framework_async_macros::async_test]
async fn every_example_load_settles_through_the_host_document_archive_door() {
    use set_active_example::{SetActiveExample, SHOOTING_EXAMPLE_DEFAULT_ID, SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT};
    for (archive_id, example_id) in [(91_u64, SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT), (92, SHOOTING_EXAMPLE_DEFAULT_ID)] {
        let mut app = shooting_app().await;
        app.dispatch_typed(ShootingCommand::SetActiveExample(SetActiveExample { example_id: example_id.into() }), &artifact_app_laws::meta("local")).await.expect("setActiveExample is admitted");
        let mut loaded = None;
        for _ in 0..100_000 {
            PluginApp::maintenance_step(&mut *app, 1, 4_096).expect("maintenance step");
            app.advance_typed_operation_publication().await.expect("publication");
            if let Some(page) = app.take_typed_operation_result_page(context::SHOOTING_TEST_INSTANCE) {
                assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge"));
            }
            if let Some(Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
                loaded = Some((pack, spr));
            }
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            if !app.has_pending_typed_operations() {
                break;
            }
            std::thread::yield_now();
        }
        let (parent_pack, parent_spr) = loaded.expect("an example publishes one whole-document load");
        let expected = <ShootingSnapshot as store::ArtifactPack>::decode_pack(&parent_pack).expect("the load carries a shooting document");
        PluginApp::begin_document_archive_load(&mut *app, archive_id, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
        let mut status = None;
        for _ in 0..1_000_000 {
            let polled = PluginApp::poll_document_archive_load(&mut *app, archive_id).await.expect("archive status");
            if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
                status = Some(polled);
                break;
            }
            PluginApp::maintenance_step(&mut *app, 1, 4_096).expect("archive maintenance step");
            std::thread::yield_now();
        }
        let status = status.expect("archive load reaches a terminal state");
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{example_id}: {}", String::from_utf8_lossy(&status.fault));
        PluginApp::acknowledge_document_archive_load(&mut *app, archive_id).expect("archive acknowledgement");
        assert_eq!(app.snapshot().expect("loaded snapshot"), expected, "{example_id} became the live document");
    }
}
//#endregion 🔖️ExampleLoad
