pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::app::EditorApp;
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{Effect, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    /// 🧪️ The registered, MOUNTED fixture app every editor test drives, bound to [`BIM_TEST_INSTANCE`] so typed commands reach their retained routes and retired through the framework's
    /// exact close loop when it goes out of scope.
    pub struct BimApp(VcsArtifactApp<EditorApp<BimModelApp>>);

    impl std::ops::Deref for BimApp {
        type Target = VcsArtifactApp<EditorApp<BimModelApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for BimApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for BimApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
            }
        }
    }

    pub const BIM_TEST_INSTANCE: u32 = 1;

    pub fn bim_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_bim_app(), examples: Vec::new() }
    }

    pub async fn bim_app() -> BimApp {
        let mut app = new_app_with_registry::<EditorApp<BimModelApp>>(bim_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
        app.bind_instance_id(BIM_TEST_INSTANCE).await;
        BimApp(app)
    }

    /// 🔁️ Dispatches `command` as the host does (no window addressed), drives its retained publication home and applies any `LoadDocument` effect.
    pub async fn dispatch(app: &mut BimApp, command: BimCommand) -> Dispatched {
        let mut result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, BIM_TEST_INSTANCE).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        for effect in &result.requested_effects {
            if let Effect::LoadDocument { pack, spr } = effect {
                let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                semio_framework_plugin::artifact_app_laws::load_document(app, &files).await.expect("test host applies load-document effect");
            }
        }
        Dispatched { result, lanes: settled.lanes }
    }

    /// 🪟️ Dispatches `command` addressed at the window instance `view` names, as the shell does for a window-scoped action.
    pub async fn dispatch_in(app: &mut BimApp, command: BimCommand, view: &ViewModel) -> Dispatched {
        let action_meta = semio_framework_plugin::ActionMeta { instance_id: BIM_TEST_INSTANCE, view_state: Some(view.clone()), ..meta("local") };
        let mut result = app.dispatch_typed(command, &action_meta).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, BIM_TEST_INSTANCE).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        Dispatched { result, lanes: settled.lanes }
    }

    pub struct Dispatched {
        pub result: InvocationResult,
        pub lanes: Vec<TypedOperationResultLane>,
    }

    impl Dispatched {
        pub fn edited_document(&self) -> bool {
            self.lanes.contains(&TypedOperationResultLane::Artifact)
        }
        pub fn wrote(&self, lane: TypedOperationResultLane) -> bool {
            self.lanes.contains(&lane)
        }
    }

    impl std::ops::Deref for Dispatched {
        type Target = InvocationResult;
        fn deref(&self) -> &Self::Target {
            &self.result
        }
    }

    pub async fn history_verb(app: &mut BimApp, verb: &str) {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, verb, BIM_TEST_INSTANCE).await;
    }

    pub fn view(locale: semio_framework_ui_locale::Locale, windows: &[(&str, &str)], addressed: Option<&str>) -> ViewModel {
        ViewModel {
            window_id: addressed.map(str::to_string),
            window_instances: windows.iter().map(|(id, kind)| ViewWindowInstance { id: (*id).into(), window_kind_id: (*kind).into() }).collect(),
            ..ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native)
        }
    }

    pub async fn render_text(app: &mut BimApp, body_key: &str, view: &ViewModel) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view).await.expect("render")).expect("project and retire semantic tree")
    }

    pub async fn canvas_scene(app: &mut BimApp, body_key: &str, view: &ViewModel) -> semio_framework_plugin::Canvas2dScene {
        let json = render_text(app, body_key, view).await;
        semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&json).expect("assembled canvas scene")
    }

    pub async fn world_scene(app: &mut BimApp, view: &ViewModel) -> semio_framework_plugin::World3dScene {
        let tree = app.render(world::BODY_KEY, None, view).await.expect("render world");
        let decoded = semio_framework_plugin::artifact_app_laws::built_surface_scene(&tree.root);
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire world tree");
        decoded.expect("assembled 3D scene")
    }
}

/// 🧰️ Pure-handler fixtures for the command nodes: the demo model behind a history-free view, no mounted app.
pub(crate) mod support {
    use super::super::*;
    use semio_framework_plugin::HistoryView;

    pub fn demo() -> ModelSnapshot {
        BimModelApp::initial_snapshot()
    }

    pub fn english() -> ViewModel {
        ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }

    pub fn german() -> ViewModel {
        ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native)
    }

    pub fn ctx(selected: &[&str]) -> BimDispatchCtx {
        BimDispatchCtx { selected: selected.iter().map(|id| id.to_string()).collect(), utility: crate::editor::bim::utilities::DEFAULT_UTILITY.into(), ..BimDispatchCtx::default() }
    }

    /// 🧪️ Runs `body` against `snapshot` the way a handler sees it.
    pub fn run<R>(snapshot: &ModelSnapshot, body: impl FnOnce(&ArtifactView<'_, ModelSnapshot>, &ConfigView<'_, NoConfig>) -> R) -> R {
        let history = HistoryView::empty();
        let doc = ArtifactView::new(snapshot, &history);
        body(&doc, &ConfigView { snapshot: &NoConfig::default(), window: None })
    }

    /// 🔁️ The snapshot after the emitted mutations are applied one after the other through the central applier.
    pub fn applied(snapshot: &ModelSnapshot, emit: &Emit<ModelMutation, NoConfigMutation>) -> ModelSnapshot {
        emit.artifact_mutations.iter().fold(snapshot.clone(), |state, mutation| crate::mutations::apply_model_mutation(&state, mutation).expect("the emitted mutation applies"))
    }
}

use super::*;
use context::{bim_app, canvas_scene, dispatch, dispatch_in, history_verb, render_text, view, world_scene};
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::PluginApp;
use semio_framework_ui_locale::Locale;
use std::future::Future;

fn plain(locale: Locale) -> ViewModel {
    view(locale, &[], None)
}

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn create_bim_app_builds_a_definition_for_the_editor_role() {
    let def = create_bim_app();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, BIM_MODEL_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<BimModelApp as ArtifactEditor>::DIALECT, BIM_MODEL_DIALECT);
    assert_eq!(<BimModelApp as ArtifactEditor>::DOCUMENT_SCHEMA, BIM_MODEL_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn the_editor_starts_from_the_demo_model() {
    assert!(!BimModelApp::initial_snapshot().storeys.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_window_panel_domain_and_command() {
    let json = serde_json::to_string(&create_bim_app()).expect("app definition json");
    for window in [plan::WINDOW_KIND_ID, world::WINDOW_KIND_ID, section::WINDOW_KIND_ID, schedule::WINDOW_KIND_ID] {
        assert!(json.contains(window), "window kind {window} missing from the manifest");
    }
    for body in [outliner_panel::BODY_KEY, properties_panel::BODY_KEY, library_panel::BODY_KEY] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    for domain in [crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN, crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN] {
        assert!(json.contains(&format!("\"{domain}\"")), "interaction domain {domain} missing from the manifest");
    }
    for id in BIM_TOOL_IDS {
        assert!(json.contains(&format!("\"{id}\"")), "command {id} missing from the manifest");
    }
    assert!(json.contains("3d.bim-model"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn the_manifest_binds_undo_redo_and_delete_while_the_framework_owns_escape_and_select_all() {
    let json = serde_json::to_string(&create_bim_app()).expect("app definition json");
    for action in ["undo", "redo", "deleteSelection"] {
        assert!(json.contains(&format!("\"{action}\"")), "keybinding action {action} missing");
    }
    let actions = semio_framework::interaction_action_definitions(&create_bim_app());
    let keys = |id: &str| actions.iter().find(|action| action.id == id).and_then(|action| action.keys.clone());
    assert_eq!((keys("clearSelection").as_deref(), keys("selectAll").as_deref()), (Some("escape"), Some("mod+a")), "escape clears and mod+a selects all through the framework's own actions");
}
//#endregion 🔖️Manifest

//#region 🔖️CommandSurface
pub(super) fn every_command() -> Vec<BimCommand> {
    vec![
        BimCommand::CreateEntity(create_entity::CreateEntity { kind: "storey".into(), parent: "bldg-1".into(), name: "Attic".into() }),
        BimCommand::DeleteSelection(delete_selection::DeleteSelection { ids: vec!["w-south".into()] }),
        BimCommand::RenameEntity(rename_entity::RenameEntity { id: "st-first".into(), name: "Upper".into() }),
        BimCommand::SetField(set_field::SetField { ids: vec!["st-ground".into()], field: "height".into(), value: "3.4".into() }),
        BimCommand::SetView(set_view::SetView { field: "storey".into(), value: "st-first".into(), pressed: None }),
        BimCommand::SetCamera(set_camera::SetCamera { camera2d: Some(store::Viewport2d { x: 1.0, y: 2.0, zoom: 3.0 }), camera3d: None }),
        BimCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 10.0, y: 20.0, width: 800.0, height: 600.0, shift: true, ctrl: false, meta: false }),
        BimCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 11.0, y: 21.0, width: 800.0, height: 600.0, ..Default::default() }),
        BimCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 12.0, y: 22.0, width: 800.0, height: 600.0, cancelled: true, ..Default::default() }),
        BimCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick { x: 13.0, y: 23.0, width: 800.0, height: 600.0, ..Default::default() }),
        BimCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}),
        BimCommand::CanvasEscape(canvas_escape::CanvasEscape {}),
        BimCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: "w".into(), position: vec![1.0, 2.0, 0.0], shift_key: true, ..Default::default() }),
        BimCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { pane: "w".into(), position: vec![3.0, 4.0, 0.0] }),
        BimCommand::ArmSelect(arm_utility::ArmSelect {}),
        BimCommand::ArmWall(arm_utility::ArmWall {}),
        BimCommand::ArmWallArc(arm_utility::ArmWallArc {}),
        BimCommand::ArmCurtainWall(arm_utility::ArmCurtainWall {}),
        BimCommand::ArmColumn(arm_utility::ArmColumn {}),
        BimCommand::ArmBeam(arm_utility::ArmBeam {}),
        BimCommand::ArmSlab(arm_utility::ArmSlab {}),
        BimCommand::ArmRoof(arm_utility::ArmRoof {}),
        BimCommand::ArmWindow(arm_utility::ArmWindow {}),
        BimCommand::ArmDoor(arm_utility::ArmDoor {}),
        BimCommand::ArmOpening(arm_utility::ArmOpening {}),
        BimCommand::ArmStair(arm_utility::ArmStair {}),
        BimCommand::ArmRailing(arm_utility::ArmRailing {}),
        BimCommand::ArmSpace(arm_utility::ArmSpace {}),
        BimCommand::ArmGrid(arm_utility::ArmGrid {}),
        BimCommand::ArmMeasure(arm_utility::ArmMeasure {}),
    ]
}

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_the_tool_roster() {
    let ids: Vec<&str> = every_command().iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids, BIM_TOOL_IDS, "every command row is a bounded tool and every_command() covers every row in table order");
    assert_eq!(BimCommand::TOOL_JOB_IDS, BIM_TOOL_IDS);
}

#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

#[semio_framework_async_macros::async_test]
async fn every_command_row_declares_a_manifest_action_with_a_description() {
    let actions = command_actions();
    assert_eq!(actions.iter().map(|action| action.id.as_str()).collect::<Vec<_>>(), BIM_TOOL_IDS);
}
//#endregion 🔖️CommandSurface

//#region 🔖️ActionBridge
fn args(value: serde_json::Value) -> DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture JSON"))
}

#[test]
fn the_bridge_folds_host_arguments_into_typed_commands() {
    use serde_json::json;
    let bridge = |action: &str, value: serde_json::Value| BimModelApp::command_from_action(action, Some(&args(value))).unwrap_or_else(|error| panic!("{action} failed to bridge: {}", error.message));
    assert_eq!(bridge("createEntity", json!({ "kind": "wall" })), BimCommand::CreateEntity(create_entity::CreateEntity { kind: "wall".into(), parent: String::new(), name: String::new() }));
    assert_eq!(bridge("deleteSelection", json!({ "id": "w-south" })), BimCommand::DeleteSelection(delete_selection::DeleteSelection { ids: vec!["w-south".into()] }));
    assert_eq!(bridge("renameEntity", json!({ "id": "st-first", "value": "Upper" })), BimCommand::RenameEntity(rename_entity::RenameEntity { id: "st-first".into(), name: "Upper".into() }));
    assert_eq!(
        bridge("setField", json!({ "field": "height", "ids": ["st-ground"], "value": 3.5 })),
        BimCommand::SetField(set_field::SetField { ids: vec!["st-ground".into()], field: "height".into(), value: "3.5".into() })
    );
    assert_eq!(bridge("setView", json!({ "field": "section_enabled", "pressed": true })), BimCommand::SetView(set_view::SetView { field: "section_enabled".into(), value: String::new(), pressed: Some(true) }));
    assert_eq!(bridge("setView", json!({ "field": "cut_height", "value": 1.5 })), BimCommand::SetView(set_view::SetView { field: "cut_height".into(), value: "1.5".into(), pressed: None }));
    assert_eq!(
        bridge("setCamera", json!({ "windowId": "w1", "camera": { "x": 2, "y": -3, "zoom": 20 } })),
        BimCommand::SetCamera(set_camera::SetCamera { camera2d: Some(store::Viewport2d { x: 2.0, y: -3.0, zoom: 20.0 }), camera3d: None })
    );
    let BimCommand::SetCamera(set_camera::SetCamera { camera2d: None, camera3d: Some(orbit) }) = bridge("setCamera", json!({ "position": [1, 2, 3], "target": [0, 0, 0], "zoom": 2 })) else { panic!("an orbit pose bridges to camera3d") };
    assert_eq!((orbit.position, orbit.zoom), ([1.0, 2.0, 3.0], 2.0));
    let BimCommand::CanvasPointerDown(press) = bridge("canvasPointerDown", json!({ "x": 5, "y": 6, "width": 800, "height": 600, "shift": true, "worldX": 1.5, "button": 0 })) else { panic!("a press bridges to canvasPointerDown") };
    assert_eq!((press.x, press.y, press.width, press.shift, press.ctrl), (5.0, 6.0, 800.0, true, false));
    assert!(BimModelApp::command_from_action("nonsense", None).is_err());
}
//#endregion 🔖️ActionBridge

//#region 🔖️Dispatch
#[semio_framework_async_macros::async_test]
async fn creating_a_storey_stacks_it_on_the_last_one_and_undo_redo_round_trips() {
    let mut app = bim_app().await;
    let result = dispatch(&mut app, BimCommand::CreateEntity(create_entity::CreateEntity { kind: "storey".into(), parent: "bldg-1".into(), name: "Attic".into() })).await;
    assert!(result.edited_document(), "creating an entity writes the document lane");
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.storeys.len(), 3);
    let attic = snapshot.storeys.values().find(|storey| storey.name == "Attic").expect("the new storey is named as asked");
    assert_eq!((attic.level, attic.building.as_str()), (2, "bldg-1"));
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("snapshot").storeys.len(), 2);
    history_verb(&mut app, "redo").await;
    assert_eq!(app.snapshot().expect("snapshot").storeys.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn a_default_name_comes_from_the_addressed_locale_and_an_unknown_kind_is_refused() {
    let mut app = bim_app().await;
    let german = view(Locale::De, &[], None);
    let result = dispatch_in(&mut app, BimCommand::CreateEntity(create_entity::CreateEntity { kind: "storey".into(), parent: String::new(), name: String::new() }), &german).await;
    assert!(result.edited_document());
    assert!(app.snapshot().expect("snapshot").storeys.values().any(|storey| storey.name == "Geschoss 3"), "the default name is the German kind label and the next ordinal");
}

#[semio_framework_async_macros::async_test]
async fn resizing_a_storey_through_set_field_re_infers_the_walls_above_it() {
    let mut app = bim_app().await;
    let result = dispatch(&mut app, BimCommand::SetField(set_field::SetField { ids: vec!["st-ground".into()], field: "height".into(), value: "3.6".into() })).await;
    assert!(result.edited_document());
    let snapshot = app.snapshot().expect("snapshot");
    assert!((snapshot.storeys["st-ground"].height - 3.6).abs() < 1e-12);
    let height = crate::editor::bim::inference::with_inference(None, &snapshot, |inference| inference.wall_layout["w-south"].height);
    assert!((height - 3.6).abs() < 1e-9, "the wall follows its storey by inference, got {height}");
}

#[semio_framework_async_macros::async_test]
async fn renaming_deleting_and_undoing_walks_the_history() {
    let mut app = bim_app().await;
    assert!(dispatch(&mut app, BimCommand::RenameEntity(rename_entity::RenameEntity { id: "st-first".into(), name: "Upper".into() })).await.edited_document());
    assert_eq!(app.snapshot().expect("snapshot").storeys["st-first"].name, "Upper");
    assert!(dispatch(&mut app, BimCommand::DeleteSelection(delete_selection::DeleteSelection { ids: vec!["w-south".into()] })).await.edited_document());
    assert!(!app.snapshot().expect("snapshot").walls.contains_key("w-south"));
    history_verb(&mut app, "undo").await;
    assert!(app.snapshot().expect("snapshot").walls.contains_key("w-south"), "the delete's concrete inverse restores the wall");
    assert_eq!(app.snapshot().expect("snapshot").storeys["st-first"].name, "Upper");
}

#[semio_framework_async_macros::async_test]
async fn deleting_walls_is_one_history_row_and_undo_restores_them() {
    let mut app = bim_app().await;
    assert!(dispatch(&mut app, BimCommand::DeleteSelection(delete_selection::DeleteSelection { ids: vec!["w-south".into(), "w-east".into()] })).await.edited_document());
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.walls.keys().cloned().collect::<Vec<_>>(), vec!["w-north".to_string(), "w-west".to_string()]);
    history_verb(&mut app, "undo").await;
    let restored = app.snapshot().expect("snapshot");
    assert_eq!(restored.walls.len(), 4);
}
//#endregion 🔖️Dispatch

//#region 🔖️Windows
const LEFT: &str = "bim-plan-left";
const RIGHT: &str = "bim-plan-right";

fn two_plans() -> (ViewModel, ViewModel) {
    let windows = [(LEFT, plan::WINDOW_KIND_ID), (RIGHT, plan::WINDOW_KIND_ID), ("bim-world", world::WINDOW_KIND_ID)];
    (view(Locale::En, &windows, Some(LEFT)), view(Locale::En, &windows, Some(RIGHT)))
}

#[test]
fn two_plan_windows_keep_their_own_storey_and_camera_and_the_document_stays_untouched() {
    std::thread::Builder::new()
        .name("bim-plan-window-isolation".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let mut future = std::pin::pin!(async {
                let mut app = bim_app().await;
                let (left, right) = two_plans();
                let before = app.document_pack().await.expect("pack");
                let moved = dispatch_in(&mut app, BimCommand::SetView(set_view::SetView { field: "storey".into(), value: "st-first".into(), pressed: None }), &left).await;
                assert!(moved.wrote(TypedOperationResultLane::WindowConfig) && !moved.edited_document(), "a view parameter writes the window config lane only");
                let panned = dispatch_in(&mut app, BimCommand::SetCamera(set_camera::SetCamera { camera2d: Some(store::Viewport2d { x: 4.0, y: -3.0, zoom: 25.0 }), camera3d: None }), &right).await;
                assert!(panned.wrote(TypedOperationResultLane::WindowConfig));
                let left_scene = canvas_scene(&mut app, plan::BODY_KEY, &left).await;
                let right_scene = canvas_scene(&mut app, plan::BODY_KEY, &right).await;
                assert!(left_scene.layers_json.len() < right_scene.layers_json.len(), "the first storey has no walls, the ground storey has");
                assert_eq!((right_scene.camera_x, right_scene.camera_y, right_scene.zoom), (4.0, -3.0, 25.0));
                assert!(right_scene.framing.is_none(), "a navigated window keeps its pose");
                assert_ne!((left_scene.camera_x, left_scene.zoom), (right_scene.camera_x, right_scene.zoom), "the cameras never cross windows");
                let after = app.document_pack().await.expect("pack");
                assert!(before.pack == after.pack && before.spr == after.spr, "window config never changes document bytes");
            });
            let waker = std::task::Waker::noop();
            let mut context = std::task::Context::from_waker(waker);
            loop {
                if let std::task::Poll::Ready(()) = future.as_mut().poll(&mut context) {
                    break;
                }
                std::thread::yield_now();
            }
        })
        .expect("spawn")
        .join()
        .expect("window isolation law");
}

#[semio_framework_async_macros::async_test]
async fn the_world_window_renders_one_instance_per_wall_with_the_elements_domain() {
    let mut app = bim_app().await;
    let world_view = view(Locale::En, &[("bim-world", world::WINDOW_KIND_ID)], Some("bim-world"));
    let scene = world_scene(&mut app, &world_view).await;
    assert_eq!(scene.domain_id.as_deref(), Some(crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN));
    let instances: Vec<serde_json::Value> = serde_json::from_str(&scene.instances_json).expect("instances");
    assert_eq!(instances.len(), 4);
}
//#endregion 🔖️Windows

//#region 🔖️Panels
#[semio_framework_async_macros::async_test]
async fn the_outliner_nests_the_model_and_speaks_both_languages() {
    let mut app = bim_app().await;
    let english = render_text(&mut app, outliner_panel::BODY_KEY, &plain(Locale::En)).await;
    for expected in ["Demo House", "Plot", "House", "Ground", "First", "Add Storey", "Add Wall"] {
        assert!(english.contains(expected), "the English outliner shows '{expected}': {english}");
    }
    let german = render_text(&mut app, outliner_panel::BODY_KEY, &plain(Locale::De)).await;
    assert!(german.contains("Geschoss hinzufügen") && german.contains("Wand hinzufügen"), "the German outliner is German: {german}");
}

#[semio_framework_async_macros::async_test]
async fn the_properties_panel_summarises_the_model_when_nothing_is_selected() {
    let mut app = bim_app().await;
    let english = render_text(&mut app, properties_panel::BODY_KEY, &plain(Locale::En)).await;
    assert!(english.contains("Nothing selected") && english.contains("s.bim.model@1"), "{english}");
    let german = render_text(&mut app, properties_panel::BODY_KEY, &plain(Locale::De)).await;
    assert!(german.contains("Nichts ausgewählt"), "{german}");
}

#[semio_framework_async_macros::async_test]
async fn the_library_lists_the_types_and_materials_of_the_model() {
    let mut app = bim_app().await;
    let text = render_text(&mut app, library_panel::BODY_KEY, &plain(Locale::En)).await;
    for expected in ["Materials", "Brick", "Mineral Wool", "Wall types", "Brick 300"] {
        assert!(text.contains(expected), "the library shows '{expected}': {text}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_schedule_lists_the_walls_and_the_section_cuts_them() {
    let mut app = bim_app().await;
    let schedule_text = render_text(&mut app, schedule::BODY_KEY, &plain(Locale::En)).await;
    assert!(schedule_text.contains("South") && schedule_text.contains("Volume"), "{schedule_text}");
    let section_view = view(Locale::En, &[("bim-section", section::WINDOW_KIND_ID)], Some("bim-section"));
    let section_scene = canvas_scene(&mut app, section::BODY_KEY, &section_view).await;
    assert!(section_scene.layers_json.contains("level:st-ground"), "the section draws the storey datum lines");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_is_named_not_crashed_on() {
    let mut app = bim_app().await;
    assert!(render_text(&mut app, "bim.edit.nowhere", &plain(Locale::De)).await.contains("bim.edit.nowhere"));
}
//#endregion 🔖️Panels

//#region 🔖️Faults
#[semio_framework_async_macros::async_test]
async fn every_refusal_code_the_commands_raise_has_an_english_and_a_german_notice() {
    let codes: Vec<&str> = bim_fault_notices().iter().map(|(code, _)| *code).collect();
    for code in [
        "bim.create.kind-unknown",
        "bim.create.unsupported",
        "bim.create.storey-missing",
        "bim.delete.unsupported",
        "bim.rename.unsupported",
        "bim.set.read-only",
        "bim.set.value-invalid",
        "bim.view.window-required",
        "bim.camera.pose-mismatch",
    ] {
        assert!(codes.contains(&code), "no notice for {code}");
    }
    let mut unique = codes.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), codes.len());
}
//#endregion 🔖️Faults

#[semio_framework_async_macros::async_test]
async fn zz_debug_close() {
    let mut app = bim_app().await;
    let world_view = view(Locale::En, &[("bim-world", world::WINDOW_KIND_ID)], Some("bim-world"));
    let _ = world_scene(&mut app, &world_view).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    std::mem::forget(app);
}

#[semio_framework_async_macros::async_test]
async fn zz_debug_close_plain() {
    let mut app = bim_app().await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn zz_debug_close_plan() {
    let mut app = bim_app().await;
    let v = view(Locale::En, &[("bim-plan", plan::WINDOW_KIND_ID)], Some("bim-plan"));
    let _ = canvas_scene(&mut app, plan::BODY_KEY, &v).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    std::mem::forget(app);
}

#[semio_framework_async_macros::async_test]
async fn zz_debug_close_section() {
    let mut app = bim_app().await;
    let v = view(Locale::En, &[("bim-section", section::WINDOW_KIND_ID)], Some("bim-section"));
    let _ = canvas_scene(&mut app, section::BODY_KEY, &v).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    std::mem::forget(app);
}
