pub(crate) mod context {
    //! 🧪️ The one cad-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
    //! instead of re-deriving a store/dispatch/render scaffold of its own.
    use super::super::*;
    use protocol::{Mutation, MutationDiff};
    use semio_framework_plugin::app::EditorApp;
    use semio_framework_plugin::{ActionMeta, HistoryView, UiMenuRef, VcsArtifactApp};
    
    pub fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::artifact_app_laws::meta(actor)
    }
    
    /// ✏️ `CadPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime `ArtifactApp`
    /// — `EditorApp<CadPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
    /// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<CadPlayApp>` builds it.
    ///
    /// 🧾️ It carries the real `AppActionRegistry`: `with_registry_on_bus` joins
    /// `EditorApp<CadPlayApp>`'s `bounded_first_step_tool_proofs!` roster against the registry's
    /// `Migrated` tool ids (`AppActionRegistry::validate_tool_job_rows`), so the registry-LESS
    /// `artifact_app_laws::new_app` — whose empty registry declares nothing — panics at construction
    /// with `interactive-job.catalog-authority` … `generated_migrated=false`, `migrated={}`. A
    /// registry-less wrapper could not dispatch anything anyway (`admit_command_wire_with_proof`
    /// refuses every verb that has no manifest declaration).
    ///
    /// 🧩️ The roster is `SemioMembers`: `CadPlayApp::genesis_child_pack` derives an
    /// `s.stdio.semio@v1/model` child for every composed pane, and a `NoMembers` store can never open
    /// that dialect (`derived child dialect … is not declared by this app's member roster`).
    pub async fn new_app() -> VcsArtifactApp<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers> {
        semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members::<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(cad_app_manifest_for_tests).await
    }
    
    /// ✏️ Adapts `create_cad_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::assert_declared_actions_bridge_to_commands` still expects —
    /// framework test context gap, not modifiable here (`🧰️framework/**` is outside this packet's lease).
    pub fn cad_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_cad_app(), examples: Vec::new() }
    }
    
    pub fn empty_history() -> HistoryView {
        HistoryView::empty()
    }
    
    /// 🔀️ Keeps the legacy test-harness call shape while exercising the production action bridge —
    /// `cad_command_from_action` speaks `DslValue`, so this bridges the `pack::json::Value`-shaped
    /// test-harness `args` via `protocol::json::to_dsl_value` right at the call site.
    pub fn command_from_action(action: &str, args: Option<&Value>) -> CadCommand {
        cad_command_from_action(action, args.map(json::to_dsl_value).as_ref()).unwrap_or_else(|error| panic!("command_from_action: {error:?}"))
    }
    
    /// 🕹️ Drives one action against a bare `CadPlayApp` (unwrapped, config defaulted) so tests can
    /// inspect the emitted document/config operations directly.
    pub fn drive(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>) -> Emit<CadMutation, CadConfigMutation> {
        drive_with_config(app, scene, action, args, &CadConfig::default())
    }
    
    /// 🧪️ `args` stays owned so every ported test keeps the pre-migration `(action id, json!(..))`
    /// call shape verbatim; `command_from_action` only ever reads it.
    ///
    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): dispatches straight through
    /// `CadCommand::dispatch` instead of the `ArtifactApp::handle` trait method — `handle`'s
    /// `interaction: &semio_framework_plugin::app::InteractionView<'_>` parameter has `pub(crate)`
    /// fields in that crate, so this crate's own tests cannot construct one; `dispatch` only needs
    /// the app-owned `CadDispatchCtx` (whose `interaction: CadInteractionSnapshot` field IS plain
    /// and cad-owned), so tests build that by hand and skip the adaptation `handle` exists for.
    #[allow(clippy::needless_pass_by_value)]
    pub fn drive_with_config(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig) -> Emit<CadMutation, CadConfigMutation> {
        let operation = CadPreviewOperationIdentity { app_instance_id: 1, parent_document_id: "cad-test-document".into(), operation_id: 1, operation_generation: 1, canonical_base_revision: "00".repeat(32) };
        drive_with_operation(app, scene, action, args, config, Some(operation)).expect("cad command handled")
    }
    
    /// 🪪️ Production-dispatch harness with an explicit public operation identity, including the
    /// missing-context case used by fail-closed transition fixtures.
    #[allow(clippy::needless_pass_by_value)]
    pub fn drive_with_operation(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig, preview_operation: Option<CadPreviewOperationIdentity>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let _ = app;
        let history = empty_history();
        let doc = ArtifactView::new(scene, &history);
        let cfg = ConfigView { snapshot: config, window: None };
        let command = command_from_action(action, args.as_ref());
        let mut ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation, view_state: None };
        command.dispatch(&doc, &cfg, &mut ctx)
    }
    
    /// 🪟️ Dispatches one command with a host-authenticated concrete CAD window instance.
    pub fn drive_in_window(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig, window_id: &str, window_kind_id: &str) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let _ = app;
        let history = empty_history();
        let doc = ArtifactView::new(scene, &history);
        let cfg = ConfigView { snapshot: config, window: None };
        let command = command_from_action(action, args.as_ref());
        let view_state = ViewModel {
            window_id: Some(window_id.into()),
            active_window_kind_id: Some(window_kind_id.into()),
            window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: window_id.into(), window_kind_id: window_kind_id.into() }],
            ..ViewModel::default()
        };
        let mut ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation: None, view_state: Some(view_state) };
        command.dispatch(&doc, &cfg, &mut ctx)
    }
    
    pub fn render_direct(_app: &CadPlayApp, body_key: &str, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
        let cfg = ConfigView { snapshot: config, window: None };
        CadPlayApp::render(body_key, doc, &cfg, view_state).map(|tree| tree.root)
    }
    
    pub fn window_measures_direct(_app: &CadPlayApp, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let cfg = ConfigView { snapshot: config, window: None };
        CadPlayApp::window_measures(doc, &cfg, view_state)
    }
    
    pub fn context_menu_direct(_app: &CadPlayApp, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let cfg = ConfigView { snapshot: config, window: None };
        let request = ContextMenuRequest { menu: UiMenuRef { id: "world3d".into(), args: None }, surface: None, window_instance_id: None, point: None };
        CadPlayApp::context_menu(&request, doc, &cfg, view_state, registry)
    }
    
    /// 🧮️ Folds a list of `CadMutation`s onto a scene via the core `Mutation`/`MutationDiff` impls —
    /// mirrors what the wrapping `VcsArtifactApp` store does when it dispatches the emitted operations.
    pub fn apply_mutations(scene: &CadSnapshot, operations: &[CadMutation]) -> CadSnapshot {
        let mut next = scene.clone();
        for operation in operations {
            next = operation.diff(&next).diff().apply(&next).expect("valid mutation diff");
        }
        next
    }
    
    /// 🧮️ `apply_mutations`'s config-targeted twin — folds an `Emit`'s `config_mutations` onto a base
    /// `CadConfig` (mirrors what `VcsArtifactApp`'s config store does when it dispatches them).
    pub fn config_after(emit: &Emit<CadMutation, CadConfigMutation>, base: &CadConfig) -> CadConfig {
        let mut next = base.clone();
        for operation in &emit.config_mutations {
            next = operation.diff(&next).diff().clone();
        }
        next
    }
    
    /// 🧮️ `config_after` plus the `CadConfig -> CadPlayRuntime` boundary conversion — the direct
    /// replacement for the pre-B1 `app.runtime.borrow()` most tests below inspected after `drive(..)`.
    pub fn runtime_after(emit: &Emit<CadMutation, CadConfigMutation>, base: &CadConfig) -> CadPlayRuntime {
        cad_runtime_from_config(&config_after(emit, base))
    }
    
    pub fn view(scene: CadSnapshot, runtime: CadPlayRuntime) -> CadPlayView {
        CadPlayView { document: scene, runtime, interaction: CadInteractionSnapshot::default() }
    }

    pub fn view_with_interaction(scene: CadSnapshot, runtime: CadPlayRuntime, interaction: CadInteractionSnapshot) -> CadPlayView {
        CadPlayView { document: scene, runtime, interaction }
    }

    /// 🌲️ The forest document as an interaction-less render view.
    pub fn forest_view() -> CadPlayView {
        view(forest_play_scene(), CadPlayRuntime::default())
    }

    pub fn selecting(ids: &[&str]) -> CadInteractionSnapshot {
        CadInteractionSnapshot { granularity: edit::CAD_WORLD_PICK_GRANULARITY.into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: ids.first().map(|id| id.to_string()), hovered_ids: Vec::new() }
    }

    /// 🚚️ Depth-first concatenation of one paged text carrier — the inverse of `paged_text_carrier`.
    fn packed_text(node: &semio_framework_plugin::BuiltNode, payload: &mut String) {
        if let semio_framework_plugin::plugin_app_close_prelude::Component::Text(props) = &node.component {
            payload.push_str(&props.packed_payload());
        }
        for child in node.children.iter() {
            packed_text(child, payload);
        }
    }

    /// 🚚️ The world-3d `lane` payload a rendered scene surface publishes beside its spine.
    pub fn scene_lane(node: &semio_framework_plugin::BuiltNode, lane: &str) -> String {
        let suffix = format!(".{lane}");
        let carrier = node.children.iter().find(|child| child.key.as_str().ends_with(&suffix)).unwrap_or_else(|| panic!("scene surface must publish the {lane} lane"));
        let mut payload = String::new();
        packed_text(carrier, &mut payload);
        payload
    }
}

use context::*;
use super::*;
use crate::standards::v1::subsets::any::io::scene_from_spatial_payload;
use crate::standards::v1::subsets::any::schema::inferences::{
    align_mesh_to_host_snapshot_centroid, default_document, object_mesh_data, run_derive_from_geometry, CAD_CONCRETE_FOREST_REFERENCE_URL, CAD_DEFAULT_TYPOLOGY_EXTENT, CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX, CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX,
    CAD_FOREST_REFERENCE_PLANE_Z, CAD_FOREST_REFERENCE_WIDTH_WORLD, CAD_FOREST_REFERENCE_Y_OFFSET_RATIO,
};
use crate::{empty_cad_snapshot, CadNode, CAD_PLAY_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ActionKind, AppActionRegistry, EditorApp, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
use store::{Backbone, BackboneMessage, MemoryBackbone};

//#region 🔖️Fixtures
/// ⚖️ One value per `app_commands!` row (plus a `None`-everywhere twin for every row with
/// `Option` fields) — the closed set the wire laws below iterate. Captured from the
/// pre-consolidation `CadCommand` enum, ticket
/// `26/08/05/CAD-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`.
pub(crate) fn every_command() -> Vec<CadCommand> {
    vec![
        CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }),
        CadCommand::AddObject(add_object::AddObject { typology: None }),
        CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: Some("1.5".into()), delta: Some(2.5) }),
        CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: None }),
        CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: vec!["object-1".into(), "object-2".into()], field: "label".into(), value: Some("Renamed".into()), delta: Some(0.25) }),
        CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: Vec::new(), field: "label".into(), value: None, delta: None }),
        CadCommand::DeleteObject(delete_object::DeleteObject { object_id: "object-1".into() }),
        CadCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: "object-1".into() }),
        CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }),
        CadCommand::RenameNode(rename_node::RenameNode { node_id: "node-1".into(), value: "Renamed".into() }),
        CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec!["object-1".into()], dx: 1.0, dy: -2.0, dz: 3.5 }),
        CadCommand::RotateSelection(rotate_selection::RotateSelection { object_ids: vec!["object-1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.57 }),
        CadCommand::ScaleSelection(scale_selection::ScaleSelection { object_ids: vec!["object-1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
        CadCommand::ApplyTransformation(apply_transformation::ApplyTransformation { qid: "spatial.shape.from_geometry".into() }),
        CadCommand::ImportCadFile(import_cad_file::ImportCadFile { name: "triangle.obj".into(), payload: "data:model/obj;base64,AAAA".into() }),
        CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), field: "widthWorld".into(), value: Some("8".into()), delta: Some(0.5) }),
        CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), field: "hidden".into(), value: None, delta: None }),
        CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: Some("shape".into()) }),
        CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: None }),
        CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-cut-concrete-forest-left".into() }),
        CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: Some("shape".into()), surface_id: Some("cad.play.scene3d/shape".into()), x: Some(1.0), y: Some(2.0), z: Some(3.0) }),
        CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: None, surface_id: None, x: None, y: None, z: None }),
        CadCommand::SetCamera(set_camera::SetCamera { pane: Some("cad.play.scene3d/building".into()), camera: CadCamera::default() }),
        CadCommand::SetCamera(set_camera::SetCamera { pane: None, camera: CadCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], zoom: 2.0, fov: 60.0, ..CadCamera::default() } }),
        CadCommand::SetProjection(set_projection::SetProjection { pane: Some("cad.play.scene3d/shape".into()), field: Some("orthographicView".into()), value_str: Some("top".into()), value_num: Some(12.5), param: Some("fov".into()) }),
        CadCommand::SetProjection(set_projection::SetProjection { pane: None, field: None, value_str: None, value_num: None, param: None }),
        CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam { pane: Some("cad.play.scene3d/shape".into()), field: Some("fov".into()), value_str: Some("x".into()), value_num: Some(45.0), param: Some("fov".into()) }),
        CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam { pane: None, field: None, value_str: None, value_num: None, param: None }),
        CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: Some("building".into()), option: "rotate".into(), pressed: Some(false) }),
        CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: None, option: "move".into(), pressed: None }),
        CadCommand::SetNodeSelection(set_node_selection::SetNodeSelection { node_ids: vec!["node-1".into(), "node-2".into()] }),
        CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: Some("shape".into()), model_definition_id: Some("spatial.shape".into()), reference_id: Some("ref-1".into()) }),
        CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: None, model_definition_id: None, reference_id: None }),
        CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: Some("ref-1".into()) }),
        CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: None }),
        CadCommand::EngagementInput(engagement_input::EngagementInput { value: "SetHeight2.5".into(), pane: Some("shape".into()) }),
        CadCommand::EngagementInput(engagement_input::EngagementInput { value: String::new(), pane: None }),
        CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: Some("shape".into()), possible_id: "primitive.box".into() }),
        CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: None, possible_id: "primitive.box".into() }),
        CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: Some("shape".into()) }),
        CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: None }),
        CadCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
        CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: Some(3.0), y: Some(4.0), z: Some(0.0) }),
        CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: None, y: None, z: None }),
        CadCommand::ToggleSun(toggle_sun::ToggleSun {}),
        CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
        CadCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
        CadCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 0.85 }),
        CadCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        CadCommand::SaveSelected(save_selected::SaveSelected {}),
        CadCommand::SaveInPlay(save_in_play::SaveInPlay {}),
        CadCommand::SaveCurrent(save_current::SaveCurrent { format: Some("step".into()) }),
        CadCommand::SaveCurrent(save_current::SaveCurrent { format: None }),
        CadCommand::LoadRawRequest(load_raw_request::LoadRawRequest {}),
    ]
}

/// 🧪️ The shell's example picker reaches the production bridge and produces the typed command
/// that replaces the CAD document instead of falling through to the framework-only action path.
#[semio_framework_async_macros::async_test]
async fn production_action_bridge_loads_the_declared_example() {
    let command = <CadPlayApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&json::to_dsl_value(&json!({ "exampleId": CAD_EXAMPLE_FOREST_LEFT })))).expect("declared example action");
    assert!(matches!(command, CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id }) if example_id == CAD_EXAMPLE_FOREST_LEFT));
    let contributions = <CadPlayApp as ArtifactEditor>::command_from_action("setContributions", Some(&json::to_dsl_value(&json!({ "json": "[{\"id\":\"cad\"}]" })))).expect("declared host command");
    assert!(matches!(contributions, CadCommand::SetContributions(set_contributions::SetContributions { json }) if json == "[{\"id\":\"cad\"}]"));
    assert!(<CadPlayApp as ArtifactEditor>::command_from_action("notACadAction", None).is_err());
}

/// 🚪️ The React shell announces the first example by dispatching `setActiveExample` through the
/// retained typed-operation lane. It used to be classified `BatchOnlyPendingRewrite`, which the host
/// refuses outright (`dispatch-failed … interactive-job classification BatchOnlyPendingRewrite`), and
/// the whole-document load it publishes then met the trait-default initializer refusal
/// (`artifact-store.persisted-initializer-refused`). Every offered example must be admitted, publish
/// ONE `LoadDocument`, settle `Ready` through the host's archive door and become the live document.
#[semio_framework_async_macros::async_test]
async fn every_example_load_is_admitted_and_settles_through_the_host_document_archive_door() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    let definition = create_cad_app();
    let declared = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).filter(|action| action.id == "setActiveExample").map(|action| action.semantics.execution.interactive_job).chain(definition.commands.iter().filter(|command| command.id == "setActiveExample").map(|command| command.semantics.execution.interactive_job)).collect::<Vec<_>>();
    assert!(!declared.is_empty() && declared.iter().all(|classification| *classification == InteractiveJobClassification::Migrated), "setActiveExample must be a live interactive job: {declared:?}");
    const INSTANCE: u32 = 7;
    for (archive_id, example_id) in [(91_u64, CAD_EXAMPLE_FOREST_LEFT), (92, crate::examples::demo::ID), (93, "")] {
        let mut app = semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members::<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(cad_app_manifest_for_tests).await;
        app.bind_instance_id(INSTANCE).await;
        app.dispatch_typed(CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_framework_plugin::ActionMeta { actor: "fixture".into(), instance_id: INSTANCE, view_state: None }).await.expect("setActiveExample is admitted");
        let mut loaded = None;
        let mut terminal = false;
        for _ in 0..100_000 {
            PluginApp::maintenance_step(&mut app, 1, 4_096).expect("maintenance step");
            app.advance_typed_operation_publication().await.expect("publication");
            if let Some(page) = app.take_typed_operation_result_page(INSTANCE) {
                assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                terminal |= page.lane == TypedOperationResultLane::Terminal;
                assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge"));
            }
            if let Some(semio_framework::kernel::Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
                assert!(loaded.is_none(), "one example switch is one whole-document load");
                loaded = Some((pack, spr));
            }
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            if !app.has_pending_typed_operations() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(terminal, "{example_id:?}: the retained operation reaches its terminal page");
        let (parent_pack, parent_spr) = loaded.expect("an example publishes one whole-document load");
        let expected = <CadSnapshot as store::ArtifactPack>::decode_pack(&parent_pack).expect("the load carries a CAD document");
        PluginApp::begin_document_archive_load(&mut app, archive_id, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
        let mut status = None;
        for _ in 0..1_000_000 {
            let polled = PluginApp::poll_document_archive_load(&mut app, archive_id).await.expect("archive status");
            if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
                status = Some(polled);
                break;
            }
            PluginApp::maintenance_step(&mut app, 1, 4_096).expect("archive maintenance step");
            std::thread::yield_now();
        }
        let status = status.expect("archive load reaches a terminal state");
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{example_id:?}: {}", String::from_utf8_lossy(&status.fault));
        PluginApp::acknowledge_document_archive_load(&mut app, archive_id).expect("archive acknowledgement");
        let live = app.snapshot().expect("loaded snapshot");
        assert_eq!(live, expected, "{example_id:?} became the live document");
        for pane in CadPaneId::all() {
            let Some(child) = crate::cad_pane_model(&live, pane) else { continue };
            assert!(app.child_store(crate::cad_pane_model_slot(pane), &child.child_id).await.is_some(), "{example_id:?}: the genesis-derived {pane:?} model member is live after the load");
            let scene = crate::cad_pane_local_scene(&live, pane).unwrap_or_else(|| panic!("{example_id:?}: the loaded {pane:?} pane resolves its materialization"));
            assert!(!crate::cad_scene_pane_objects(&scene, pane).is_empty(), "{example_id:?}: the loaded {pane:?} pane renders real objects");
        }
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
    }
}

/// 🛂️ An example id the app does not ship is refused by name — never a silent no-op that leaves the
/// previous document standing while the shell's picker claims the switch happened.
#[test]
fn an_unknown_example_is_refused_by_name() {
    let Err(fault) = drive_with_operation(&CadPlayApp, &forest_play_scene(), "setActiveExample", Some(json!({ "exampleId": "not-an-example" })), &CadConfig::default(), None) else { panic!("an unknown example must be refused") };
    assert!(fault.message.starts_with("cad.example.unknown"), "{fault:?}");
}

#[test]
fn host_contributions_resolve_to_the_event_sourced_config_lane() {
    let mutation = <CadPlayApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&json::to_dsl_value(&json!({ "json": "[{\"id\":\"cad\"}]" })))).expect("host configuration").expect("CAD contribution mutation");
    assert_eq!(mutation, CadConfigMutation::SetContributions { json: "[{\"id\":\"cad\"}]".into() });
    assert_eq!(<CadPlayApp as ArtifactEditor>::host_configuration_mutation("setActiveExample", None).expect("non-host action"), None);
    assert!(<CadPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some());
    assert!(<CadPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
    let factory = CadRetainedCommandJobFactory::new("s.cad.cad@1/*#editor");
    let expected_keys = CAD_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new("s.cad.cad@1/*#editor", *tool_id)).collect::<Vec<_>>();
    assert_eq!(ToolJobFactory::keys(&factory), expected_keys);
    assert_eq!(ToolJobFactory::payload_schema_id(&factory), CAD_RETAINED_COMMAND_SCHEMA);
    assert_eq!(ToolJobFactory::classification(&factory), InteractiveJobClassification::Migrated);
    assert_eq!(<CadRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, CAD_RETAINED_PUBLICATION_CONTRACTS);
    assert_eq!(<CadPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), CAD_RETAINED_TOOL_IDS.len());
}

#[semio_framework_async_macros::async_test]
async fn retained_cad_presence_close_empty_lanes_have_exact_owners() {
    let fixture: Value = json::parse(include_str!("../../👥️presence/🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
    let maximum_items = fixture["grant"]["maximumItems"].as_u64().unwrap() as usize;
    let maximum_bytes = fixture["grant"]["maximumBytes"].as_u64().unwrap() as usize;
    let envelope = store::create_document_envelope::<NoDraft, NoDraftMutation>("draft.empty", "cad-draft-close", NoDraft::default(), None);
    let mut draft = store::DraftStore::new(envelope).await.unwrap();
    draft.install_document_store_owners_exact(<CadPlayApp as ArtifactEditor>::build_draft_store_owners().unwrap());
    let mut disposer = <CadPlayApp as ArtifactEditor>::build_draft_store_disposer().unwrap();
    for turn in 0..100_000 {
        match disposer.close_step(&mut draft, maximum_items, maximum_bytes).unwrap() {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= maximum_items && released_bytes <= maximum_bytes),
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("empty CAD draft close blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh CAD fixture unexpectedly awaits external input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => break,
        }
        assert!(turn < 99_999);
    }
    assert!(disposer.terminal_is_empty(&draft));
    let mut transient = store::TransientStore::new(semio_framework_plugin::NoTransient::default());
    let mut disposer = <CadPlayApp as ArtifactEditor>::build_transient_store_disposer().unwrap();
    assert_eq!(disposer.close_step(&mut transient, 0, maximum_bytes).unwrap(), semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(disposer.close_step(&mut transient, maximum_items, maximum_bytes).unwrap(), semio_framework_plugin::PluginCloseStep::Complete);
    assert!(disposer.terminal_is_empty(&transient));
    eprintln!("[DEBUG] CAD exact NoDraft and NoTransient owners completed under 1-item/4096-byte grants");
}

#[semio_framework_async_macros::async_test]
async fn retained_factory_proofs_activate_the_real_cad_manifest_and_close_under_the_production_grant() {
    let fixture: Value = json::parse(include_str!("../../../🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("CAD activation fixture");
    let activation = &fixture["activation"];
    let controller = activation["controller"].as_str().expect("controller");
    let bus = semio_framework::ActionBus::new();
    let definition = create_cad_app();
    let host_route = fixture["routes"].as_array().unwrap().iter().find(|route| route["id"] == "setContributions").unwrap();
    assert_eq!(host_route["disposition"], "migrated");
    let host_command = definition.commands.iter().find(|command| command.id == host_route["id"].as_str().unwrap()).expect("host command declaration");
    assert_eq!(host_command.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    let registry = AppActionRegistry::from_definition(&definition);
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>::with_registry_on_bus(EditorApp::<CadPlayApp>::default(), registry, bus.clone()).await;
    assert_eq!(app.app_id().await, controller);
    assert_eq!(<CadPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), activation["proofRows"].as_u64().expect("proof rows") as usize);
    let mut admitted = std::collections::BTreeSet::new();
    for tool_id in CAD_RETAINED_TOOL_IDS {
        let admission = bus.admit_exact_wire(controller, *tool_id, CAD_RETAINED_COMMAND_SCHEMA, &[]).expect("real CAD factory is live before proof validation");
        assert_eq!(admission.factory_type_id, std::any::TypeId::of::<CadRetainedCommandJobFactory>());
        assert_eq!(admission.factory_type_name, std::any::type_name::<CadRetainedCommandJobFactory>());
        assert!(admitted.insert(*tool_id));
    }
    assert!(admitted.contains(activation["injectedTool"].as_str().expect("injected tool")));
    let maximum_items = activation["closeItems"].as_u64().expect("close items") as usize;
    let maximum_bytes = activation["closeBytes"].as_u64().expect("close bytes") as usize;
    let mut complete = false;
    for _ in 0..100_000 {
        match app.close_step(maximum_items, maximum_bytes).expect("bounded CAD close") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= maximum_items && released_bytes <= maximum_bytes);
            }
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("CAD constructor close blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh CAD fixture unexpectedly awaits external input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => {
                complete = true;
                break;
            }
        }
    }
    assert!(complete && app.close_terminal_is_empty(), "the real mounted CAD owner must reach its empty terminal shell");
    eprintln!("[DEBUG] CAD activation joined {} exact app factory rows and completed bounded close", admitted.len());
}

#[test]
fn retained_config_store_preparation_is_bounded_exact_and_reversible() {
    let base = CadConfig::default();
    let mut next = base.clone();
    next.selected_node_ids.push("node-retained".into());
    let mutation = CadConfigMutation::Snapshot { config: Box::new(next.clone()) };
    let footprint = admit_cad_config_mutation(&mutation).expect("bounded CAD config mutation");
    let (post, inverse, forward) = prepare_cad_config(&base, mutation.clone()).expect("exact CAD config preparation");
    assert_eq!(post, next);
    assert_eq!(forward, mutation);
    assert_eq!(inverse, vec![CadConfigMutation::Snapshot { config: Box::new(base.clone()) }]);
    // 🧺️ `work_items` counts staged ROWS: the forward plus every inverse row — declaring fewer fail-closes
    // the gesture in `ArtifactStore::fold_batch_item` (`batched item candidate failed its exact fixed fold contract`).
    assert_eq!(footprint.work_items, 1 + inverse.len(), "config footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::ArtifactStoreOneItemFootprint::for_one_invertible_item(footprint.retained_bytes));
    let oversized = CadConfigMutation::SetContributions { json: "x".repeat(CAD_CONFIG_STORE_MAXIMUM_BYTES + 1) };
    assert!(admit_cad_config_mutation(&oversized).is_err());
}

/// 🧺️ The one-invertible-item declaration (`admit_cad_artifact_mutation`) is only exact while every
/// `CadMutation` inverts to at most one row against the forest document.
#[test]
fn every_cad_mutation_inverse_fits_the_one_invertible_item_footprint() {
    let base = forest_play_scene();
    let history = empty_history();
    let doc = ArtifactView::new(&base, &history);
    let config = CadConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let operation = CadPreviewOperationIdentity { app_instance_id: 1, parent_document_id: "cad-test-document".into(), operation_id: 1, operation_generation: 1, canonical_base_revision: "00".repeat(32) };
    // 🚪️ The I/O commands mint envelopes/effects the bare dispatch harness cannot retire; the
    // document-mutating vocabulary this law guards lives in the node/reference/transform commands.
    let io_commands = ["importCadFile", "saveSelected", "saveInPlay", "saveCurrent", "loadRawRequest", "setActiveExample"];
    for command in every_command().into_iter().filter(|command| !io_commands.contains(&command.command_id())) {
        let mut ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation: Some(operation.clone()), view_state: None };
        let Ok(emit) = command.dispatch(&doc, &cfg, &mut ctx) else { continue };
        for mutation in &emit.artifact_mutations {
            let inverse = <CadMutation as protocol::Mutation<CadSnapshot>>::inverse(mutation, &base);
            assert!(inverse.len() <= 1, "{} inverts to {} rows; the artifact lane declares one invertible item", command.command_id(), inverse.len());
        }
    }
}

#[test]
fn retained_artifact_store_preparation_is_bounded_exact_and_reversible() {
    let base = empty_cad_snapshot();
    let node = CadNode { id: "node-retained".into(), label: "Retained".into(), kind: "group".into() };
    let mutation = CadMutation::CreateNode(crate::mutations::create_node::CreateNode { node: node.clone() });
    let footprint = admit_cad_artifact_mutation(&mutation).expect("bounded CAD Artifact mutation");
    let (post, inverse, forward) = prepare_cad_artifact(&base, mutation.clone()).expect("exact CAD Artifact preparation");
    assert_eq!(post.nodes, vec![node]);
    assert_eq!(forward, mutation);
    assert_eq!(footprint.work_items, 1 + inverse.len(), "artifact footprint must cover forward + inverse rows");
    assert_eq!(footprint, store::ArtifactStoreOneItemFootprint::for_one_invertible_item(footprint.retained_bytes));
    let mut restored = post;
    for operation in inverse {
        let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&operation, &restored);
        restored = protocol::MutationDiff::apply(outcome.diff(), &restored).expect("exact inverse");
    }
    assert_eq!(restored, base);
}

#[test]
fn retained_route_fixture_matches_the_exact_owner_manifest_and_laws() {
    let fixture: Value = json::parse(include_str!("../../../🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("CAD retained route fixture");
    let routes = fixture.get("routes").and_then(Value::as_array).expect("route array");
    let route_ids = routes.iter().map(|route| route.get("id").and_then(Value::as_str).expect("route id")).collect::<std::collections::BTreeSet<_>>();
    let command_ids = every_command().iter().map(CadCommand::command_id).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(routes.len(), command_ids.len());
    assert_eq!(route_ids, command_ids);
    assert_eq!(fixture.get("admittedRoutes"), Some(&Value::Array(CAD_RETAINED_TOOL_IDS.iter().map(|id| Value::from(*id)).collect())));
    assert_eq!(fixture.pointer("/limits/closePageBytes").and_then(Value::as_u64), Some(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES as u64));
    assert_eq!(fixture.get("laws"), Some(&json!(["ownerLocal", "progress", "cancel", "freshness", "ackBeforeClose", "incrementalClose", "terminalEmpty"])));
    assert!(routes.iter().all(|route| {
        let id = route.get("id").and_then(Value::as_str);
        let disposition = route.get("disposition").and_then(Value::as_str);
        let blocker = route.get("blocker").and_then(Value::as_str);
        if id.is_some_and(|id| CAD_RETAINED_TOOL_IDS.contains(&id)) {
            disposition == Some("migrated") && blocker == Some("none")
        } else {
            disposition == Some("batchOnlyPendingRewrite") && blocker.is_some_and(|blocker| blocker != "none")
        }
    }));
    let manifest = create_cad_app();
    for tool_id in CAD_RETAINED_TOOL_IDS {
        let mut declarations = 0;
        for commands in std::iter::once(&manifest.commands).chain(manifest.modes.iter().map(|mode| &mode.commands)) {
            let matches = commands.iter().filter(|command| command.id == *tool_id).collect::<Vec<_>>();
            if matches.is_empty() {
                continue;
            }
            assert_eq!(matches.len(), 1, "{tool_id} requires exactly one declaration per command scope");
            assert_eq!(matches[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id}");
            declarations += 1;
        }
        for window in &manifest.window_kinds {
            let actions = semio_framework::window_kind_actions(&manifest, window).into_iter().filter(|action| action.id == *tool_id).collect::<Vec<_>>();
            if actions.is_empty() {
                continue;
            }
            assert_eq!(actions.len(), 1, "{tool_id} requires exactly one declaration per window scope");
            assert_eq!(actions[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id}");
            declarations += 1;
        }
        assert!(declarations > 0, "{tool_id} requires a manifest command or window declaration");
    }
    assert_eq!(manifest.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&manifest, window)).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).map(|action| action.semantics.execution.interactive_job), Some(InteractiveJobClassification::Migrated));
    assert!(manifest.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&manifest, window)).filter(|action| route_ids.contains(action.id.as_str())).all(|action| {
        let expected = if CAD_RETAINED_TOOL_IDS.contains(&action.id.as_str()) { InteractiveJobClassification::Migrated } else { InteractiveJobClassification::BatchOnlyPendingRewrite };
        action.semantics.execution.interactive_job == expected
    }));
}

/// ⚖️ LAW: the one-action spot check above is not enough — this is the framework's own harness,
/// which walks EVERY action this app's window kinds render, stages each one's declared args the way
/// the host does, and skips the framework-injected ids. It is what catches the next
/// `setActiveExample`: chrome that declares an action no command row backs.
#[semio_framework_async_macros::async_test]
async fn every_rendered_action_bridges_through_the_framework_harness() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<CadPlayApp>>(cad_app_manifest_for_tests).await;
}

/// ⚖️ Text and binary are two projections of the same command, and every printed line starts with
/// that row's wire keyword — the guard that a command decomposition cannot silently rename a row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_own_wire_keyword() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = protocol::OpText::print_op(&command);
        let keyword = printed.split_whitespace().next().unwrap_or_default().to_string();
        assert!(!keyword.is_empty(), "a command must print a leading wire keyword: {printed:?}");
        let decoded = <CadCommand as protocol::OpText>::parse_op(&printed).expect("re-parse");
        assert_eq!(decoded, command, "printed line must re-parse to the same command: {printed:?}");
    }
}

/// 🔒️ Wire-format pin: the exact bytes of rows whose `Option` fields make `None`/`Some` distinct
/// wire cases, copied out of the pre-consolidation baseline dump.
#[test]

fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &CadCommand| -> String { protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect() };
    assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) })), "0100011b7370617469616c2e73686170652e7072696d69746976652e626f7801000600");
    assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: None })), "01000000");
    assert_eq!(
        hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: Some("1.5".into()), delta: Some(2.5) })),
        "01010303312e35086f626a6563742d31086f726967696e2e780400060101060202060003050000000000000440"
    );
    assert_eq!(hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: None })), "010102086f626a6563742d31086f726967696e2e7802000600010601");
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `SetHover`/`WorldPick` and their
    // byte pins are DELETED (those commands no longer exist); every OTHER row's ordinal shifted
    // too (this enum's binary encoding is a plain row-position ordinal — greenfield, no
    // back-compat expected), so the tail-of-enum pins (`EngagementAbort`/`ToggleSun`/
    // `SaveSelected`/`LoadRawRequest`) are dropped rather than hand-recomputed; the exact-wire-key
    // guard for every row (including these) already lives in
    // `every_command_round_trips_text_and_binary_under_its_own_wire_keyword` above.
}

#[semio_framework_async_macros::async_test]
async fn forest_example_uses_per_object_brep_meshes() {
    let scene = forest_working_scene();
    let view = forest_view();
    let json = edit::world_instances_json(&scene.building_objects, &view);
    assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
    let meshes: Vec<serde_json::Value> = serde_json::from_str(&edit::world_meshes_json(&scene.building_objects, scene.building_geometry.as_ref())).expect("meshes json");
    assert!(scene.building_objects.len() > 5);
    assert_eq!(meshes.len(), scene.building_objects.iter().filter(|object| object.visible).count(), "one inline tessellation per visible object");
    for (object, mesh) in scene.building_objects.iter().filter(|object| object.visible).zip(&meshes) {
        assert_eq!(mesh["id"].as_str(), Some(object.id.as_str()), "the instance's meshId must resolve to its own solid, never a shared placeholder kind");
        assert!(mesh.get("kind").is_none(), "a CAD solid is authored geometry, not a renderer-derivable kind");
        assert!(mesh["data"]["positions"].as_array().is_some_and(|positions| positions.len() >= 9), "inline tessellation must carry real triangles");
    }
    assert!(!json.contains("🧊️hexagonal-cut-concrete-forest-left.glb"));
    assert!(scene.building_objects.iter().all(|object| object.solid_handle.is_some()));
}

/// 🗄️ The mesh lane is remembered per materialized scene: a second render of the same `Arc` answers
/// the same bytes without tessellating, an object edit (a different digest) re-tessellates, and a
/// re-minted scene (the same objects under a new `Arc`) is a miss by construction.
#[semio_framework_async_macros::async_test]
async fn mesh_lane_is_cached_per_materialized_scene() {
    let scene = std::sync::Arc::new(forest_working_scene());
    let objects = &scene.building_objects;
    let geometry = scene.building_geometry.as_ref();
    let started = std::time::Instant::now();
    let first = edit::world_meshes_json_cached(CadPaneId::Building, Some(&scene), objects, geometry);
    let cold = started.elapsed();
    let started = std::time::Instant::now();
    let second = edit::world_meshes_json_cached(CadPaneId::Building, Some(&scene), objects, geometry);
    let warm = started.elapsed();
    assert_eq!(first, second);
    assert_eq!(first, edit::world_meshes_json(objects, geometry), "the cached lane is byte-identical to a fresh tessellation");
    assert!(warm < cold / 4 || warm.as_millis() < 2, "a warm lane never tessellates (cold {cold:?}, warm {warm:?})");
    // ✏️ An edit to a tessellation-relevant field misses: a wider object is a different lane.
    let mut widened = objects.clone();
    widened[0].extent = Some([9.0, 9.0, 9.0]);
    widened[0].solid_handle = None;
    widened[0].primitives.clear();
    let edited = edit::world_meshes_json_cached(CadPaneId::Building, Some(&scene), &widened, geometry);
    assert_ne!(edited, first);
    assert_eq!(edited, edit::world_meshes_json(&widened, geometry));
    // 🪆️ The same objects under a re-minted scene are a miss (a new `Arc` is a new geometry identity)
    // and then a hit again on their own allocation.
    let reminted = std::sync::Arc::new(forest_working_scene());
    let again = edit::world_meshes_json_cached(CadPaneId::Building, Some(&reminted), &reminted.building_objects, reminted.building_geometry.as_ref());
    assert_eq!(again, first);
    let started = std::time::Instant::now();
    let _ = edit::world_meshes_json_cached(CadPaneId::Building, Some(&reminted), &reminted.building_objects, reminted.building_geometry.as_ref());
    assert!(started.elapsed() < cold / 4 || started.elapsed().as_millis() < 2);
    // 🚫️ A pane without a materialized scene renders the fallback roster and is never cached.
    let fallback = edit::world_meshes_json_cached(CadPaneId::Shape, None, &[], None);
    assert!(fallback.contains(CAD_FALLBACK_MESH_KIND));
}

#[semio_framework_async_macros::async_test]
async fn url_backed_objects_share_one_mesh_reference_beside_inline_solids() {
    let scene = forest_working_scene();
    let mut objects = scene.building_objects.clone();
    objects[0].mesh_url = Some("/🧊️assets/column.glb".into());
    objects[1].mesh_url = Some("/🧊️assets/column.glb".into());
    let meshes: Vec<serde_json::Value> = serde_json::from_str(&edit::world_meshes_json(&objects, scene.building_geometry.as_ref())).expect("meshes json");
    let url_entries: Vec<&serde_json::Value> = meshes.iter().filter(|mesh| mesh.get("url").is_some()).collect();
    assert_eq!(url_entries.len(), 1, "one shared reference per asset url");
    assert_eq!(url_entries[0]["id"].as_str(), Some("mesh:column"));
    assert_eq!(meshes.len(), 1 + objects.iter().filter(|object| object.visible).count() - 2, "url-backed objects add no inline tessellation");
    let instances: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(&objects, &forest_view())).expect("instances json");
    assert_eq!(instances[0]["meshId"].as_str(), Some("mesh:column"));
    assert_eq!(instances[2]["meshId"].as_str(), Some(objects[2].id.as_str()));
}

/// 🎨️ An instance's wire colour is its TYPOLOGY's resolved style, not a hardcoded blue/grey premix:
/// both hosts layer selection/hover on top of it as separate booleans, so a premixed colour double-
/// paints on one target and loses the typology on both (ticket 26/09/17 packet W2f).
#[semio_framework_async_macros::async_test]
async fn world_instances_carry_their_typology_colour_not_a_selection_premix() {
    use crate::editor::cad::engine::typology::resolve_typology_style;
    let scene = forest_working_scene();
    let objects = &scene.building_objects;
    let instances: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(objects, &forest_view())).expect("instances json");
    let visible: Vec<&crate::standards::v1::subsets::any::io::geometry_import::CadObject> = objects.iter().filter(|object| object.visible).collect();
    assert_eq!(instances.len(), visible.len());
    for (object, instance) in visible.iter().zip(&instances) {
        assert_eq!(instance["color"].as_str(), Some(resolve_typology_style(&object.typology).color.as_str()), "{}", object.typology);
    }
    assert!(!instances.iter().any(|instance| instance["color"].as_str() == Some("#3b82f6")), "no selection premix survives on the colour lane");
    let selected = view(forest_play_scene(), CadPlayRuntime::default());
    let with_selection: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(objects, &selected)).expect("instances json");
    assert_eq!(with_selection[0]["color"], instances[0]["color"], "selection never rewrites the colour lane");
}

/// 🧲️ The geometry pick overlay rides the `engagementPreview` lane only while the live session's
/// state accepts a selection, and it is capped so a forest-scale pane can never overrun the lane.
#[semio_framework_async_macros::async_test]
async fn the_pick_overlay_is_bounded_and_only_published_while_a_selection_is_accepted() {
    use crate::editor::cad::engine::interaction::{accepts_selection, start_session};
    let scene = forest_working_scene();
    let document = forest_play_scene();
    let idle = edit::build_world_scene_for_pane(&view(document, CadPlayRuntime::default()), CadPaneId::Building, "cad.play.scene3d/building", None, Default::default()).expect("idle scene");
    let idle_scene: semio_framework_plugin::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: idle }).expect("projected")).expect("world scene");
    assert!(idle_scene.engagement_preview_json.is_none(), "no session, no overlay");

    let session = start_session("feature.extrudeWire", CadPaneId::Building).expect("extrude-wire session");
    assert!(accepts_selection(&session), "extrudeWire opens on its `selectWire` selection state");
    assert!(!accepts_selection(&start_session("primitive.box", CadPaneId::Shape).expect("box session")), "a construction interaction that starts on a ground pick publishes no overlay");

    let overlay = edit::pick_target_preview_items(&scene.building_objects, scene.building_geometry.as_ref(), CadPaneId::Building);
    assert!(!overlay.is_empty(), "a selection state publishes the pick overlay");
    assert!(overlay.len() <= edit::CAD_PICK_OVERLAY_ITEM_BUDGET, "the overlay stays inside its budget, got {}", overlay.len());
    for item in &overlay {
        let kind = item.get("kind").and_then(protocol::DslValue::as_str).expect("item kind");
        assert!(matches!(kind, "point" | "segment"), "{kind} is not a pick-overlay wire kind");
        assert!(item.get("role").and_then(protocol::DslValue::as_str).is_some_and(|role| role.contains(':')), "every item carries its `kind:id` pick key as its role");
    }
    assert!(overlay.iter().any(|item| item.get("kind").and_then(protocol::DslValue::as_str) == Some("segment")), "a solid pane draws its members as segments");
    assert!(edit::pick_target_preview_items(&scene.building_objects, None, CadPaneId::Building).is_empty(), "a pane with no kernel geometry offers no overlay");
}

#[semio_framework_async_macros::async_test]
async fn world_fit_revision_follows_the_document_not_object_poses() {
    let document = forest_play_scene();
    let scene = forest_working_scene();
    let base = edit::world_fit_revision(&document, CadPaneId::Building, &scene.building_objects);
    let mut moved = scene.building_objects.clone();
    moved[0].origin = [12.0, -3.0, 0.5];
    assert_eq!(edit::world_fit_revision(&document, CadPaneId::Building, &moved), base, "a transform edit must not re-arm the auto-fit");
    assert_ne!(edit::world_fit_revision(&document, CadPaneId::Energy, &scene.energy_objects), base, "each pane frames its own content");
    let mut other = document.clone();
    other.id = "another-document".into();
    assert_ne!(edit::world_fit_revision(&other, CadPaneId::Building, &scene.building_objects), base, "opening another document frames again");
}

#[semio_framework_async_macros::async_test]
async fn quad_panes_each_populate_distinct_objects() {
    let scene = forest_working_scene();
    assert!(!scene.objects.is_empty(), "shape pane");
    assert!(!scene.building_objects.is_empty(), "building pane");
    assert!(!scene.energy_objects.is_empty(), "energy pane");
    assert!(!scene.structure_classic_objects.is_empty(), "structure classic pane");
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_cut_concrete_forest_not_placeholder_box() {
    let scene = CadPlayApp::initial_snapshot();
    assert_eq!(scene.id, CAD_EXAMPLE_FOREST_LEFT);
    assert_eq!(scene.nodes.first().map(|node| node.label.as_str()), Some("Concrete Forest Left"), "must not be the placeholder 'Model' node");
    let working = forest_working_scene();
    assert_ne!(working.objects.first().map(|object| object.id.as_str()), Some("object-box-1"));
    assert!(!working.building_objects.is_empty(), "building pane must not be the empty default placeholder");
    assert!(!working.energy_objects.is_empty(), "energy pane must not be the empty default placeholder");
    assert!(!working.structure_classic_objects.is_empty(), "structure pane must not be the empty default placeholder");
    assert!(working.objects.iter().all(|object| object.solid_handle.is_some()));
}

#[semio_framework_async_macros::async_test]
async fn forest_energy_world_mesh_survives_scene_roundtrip() {
    let scene = forest_working_scene();
    let roundtrip: CadWorkingScene = json::from_json_str(&json::to_json_string(&scene)).expect("deserialize");
    let object = roundtrip.energy_objects.first().expect("energy object");
    let mesh = object_mesh_data(object, roundtrip.energy_geometry.as_ref());
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(min_z > 2.5, "energy world mesh min z {min_z}");
    let slab = roundtrip.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
    let slab_mesh = object_mesh_data(slab, roundtrip.structure_classic_geometry.as_ref());
    let slab_min_z = slab_mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(slab_min_z > 2.5, "structure world mesh min z {slab_min_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_references_use_xy_ground_plane_and_z_up() {
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference");
    assert_eq!(reference.origin[2], CAD_FOREST_REFERENCE_PLANE_Z, "reference must stay on the CAD ground datum");
    assert!((reference.origin[0] - (-9.7)).abs() < 1e-9, "reference x {} should be base + 50% width (right)", reference.origin[0]);
    let expected_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * (0.5 + CAD_FOREST_REFERENCE_Y_OFFSET_RATIO);
    assert!((reference.origin[1] - expected_y).abs() < 1e-9, "reference CAD y {} should be centered then moved +20% forward on the world plane", reference.origin[1]);
    let centered_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.5;
    assert!(((reference.origin[1] - centered_y) - CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.2).abs() < 1e-9, "the requested offset must affect CAD y only");
    assert_eq!(CAD_FOREST_REFERENCE_Y_OFFSET_RATIO, 0.2);
    assert!(reference.locked, "example references default locked like puzzle 3d");
    assert_eq!(reference.width_world, 28.6);
}

/// 🖼️ The `/cad-assets` route is the static-dir row in `📦️packages/🦀️rust/Cargo.toml` rooted at
/// `📚️examples/🖼️assets`; a url that misses a file there is answered by the dev server's SPA
/// fallback (`index.html`, 200) and the reference plane silently never paints.
#[semio_framework_async_macros::async_test]
async fn forest_reference_url_names_a_file_under_the_served_assets_root() {
    let relative = CAD_CONCRETE_FOREST_REFERENCE_URL.strip_prefix("/cad-assets/").expect("reference url rides the /cad-assets static-dir route");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets").join(relative);
    assert!(path.is_file(), "reference asset {} must exist under the served assets root", path.display());
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference");
    assert_eq!(reference.source_url, CAD_CONCRETE_FOREST_REFERENCE_URL);
    let published = edit::world_references_json(&scene, CadPaneId::Energy).expect("references lane");
    assert!(published.contains(CAD_CONCRETE_FOREST_REFERENCE_URL), "the world-3d references lane must carry the served url");
}

#[semio_framework_async_macros::async_test]
async fn align_mesh_to_host_snapshot_centroid_corrects_drifted_surface() {
    let scene = forest_working_scene();
    let geometry = scene.energy_geometry.as_ref().expect("energy geometry");
    let object = scene.energy_objects.first().expect("energy object");
    let mut mesh = object_mesh_data(object, Some(geometry));
    for vertex in mesh.positions.as_chunks_mut::<3>().0 {
        vertex[2] = 0.0;
    }
    align_mesh_to_host_snapshot_centroid(&mut mesh, geometry, &object.primitives);
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(min_z > 2.5, "aligned mesh min z {min_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_surface_meshes_fall_back_to_typology_extent_without_pane_geometry() {
    // ⚠️ CORRECTED (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    // wave G4): this test used to assert the mesh stayed at its authored height even with no
    // `CadGeometry` in hand — that only worked because `cad_brep_kernel()` was a process-global
    // `BrepEngineHost` singleton, so `energy.solid_handle` (minted by an EARLIER, already-dropped
    // call to `forest_pane_bundle`) still resolved in whatever kernel `object_mesh_data` happened
    // to reach. `origin` on fixture-derived `CadObject`s is always `[0,0,0]`
    // (`objects_from_host_snapshot_model`) — the authored height lived ONLY in the solid's own vertex
    // data, addressed by that handle. A `cad_brep_kernel()` is now a fresh, local `Brep::new()`
    // per call (doctrine tier-(d): never outlives the call that built it), so a handle from a
    // different call is honestly unresolvable, and — exactly like `mesh_from_glb`'s documented
    // gap elsewhere in this codebase — meshing falls back to the typology's default extent box
    // at the kernel's local origin instead of silently fabricating a placement it cannot know.
    let scene = forest_working_scene();
    let energy = scene.energy_objects.first().expect("energy object");
    let energy_mesh = object_mesh_data(energy, None);
    assert!(!energy_mesh.positions.is_empty(), "energy mesh must still be real geometry, just typology-shaped");
    let slab = scene.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
    let slab_mesh = object_mesh_data(slab, None);
    assert!(!slab_mesh.positions.is_empty(), "structure slab mesh must still be real geometry, just typology-shaped");
}

#[semio_framework_async_macros::async_test]
async fn cad_artifact_schema_matches_domain() {
    let scene = empty_cad_snapshot();
    assert_eq!(scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn default_example_and_forest_scene_parse_as_projections() {
    let default_json = json::to_json_string(&default_document());
    let default_scene: CadSnapshot = json::from_json_str(&default_json).unwrap();
    assert_eq!(default_scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
    let forest_json = json::to_json_string(&forest_play_scene());
    let forest_scene: CadSnapshot = json::from_json_str(&forest_json).unwrap();
    assert_eq!(forest_scene.id, CAD_EXAMPLE_FOREST_LEFT);
    assert!(!forest_working_scene().building_objects.is_empty());
}
//#endregion 🔖️Fixtures
//#region 🔖️Render
#[semio_framework_async_macros::async_test]
async fn renders_world_scene_for_each_pane() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let view_state = ViewModel::default();
    for body_key in [shape::BODY_KEY, building::BODY_KEY, energy::BODY_KEY, structure_classic::BODY_KEY] {
        let node = render_direct(&app, body_key, &doc, &CadConfig::default(), &view_state).expect("CAD UI assembly");
        let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = &node.component else { panic!("body {body_key} should render a surface") };
        assert_eq!(props.doc_schema.as_str(), "world-3d@1", "body {body_key} should render a world-3d scene");
        let lanes: Vec<&str> = node.children.iter().map(|child| child.key.as_str()).collect();
        for lane in ["meshes", "instances", "selection", "references", "environment", "fit"] {
            assert!(lanes.iter().any(|key| key.ends_with(&format!(".{lane}"))), "body {body_key} must publish the {lane} lane beside the spine, got {lanes:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn world_scene_binds_the_cad_interaction_domain_at_object_granularity() {
    let scene = forest_play_scene();
    let view = CadPlayView { document: scene, runtime: CadPlayRuntime::default(), interaction: CadInteractionSnapshot::default() };
    let node = edit::build_world_scene_for_pane(&view, CadPaneId::Shape, shape::SURFACE_ID, None, CadDislocateOptions::default()).expect("scene surface");
    assert_eq!(node.key.as_str(), shape::SURFACE_ID);
    let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = &node.component else { panic!("world-3d surface node") };
    let world: semio_framework_plugin::World3dScene = semio_framework_ui_scene::decode(props).expect("world-3d spine");
    assert_eq!(world.domain_id.as_deref(), Some(CAD_INTERACTION_DOMAIN));
    assert_eq!(world.domain_granularity_id.as_deref(), Some(edit::CAD_WORLD_PICK_GRANULARITY));
}

/// 🛡️ Anti-regression guard for the "four empty windows" defect: `forest_play_document` must
/// actually populate `shape_model`/`building_model`/`energy_model`/`structure_classic_model`
/// (via `cad_document_pane_bundle`, carried as each handle's `ArtifactChild::local_owner`), and
/// `build_world_scene_for_pane`'s own pane resolver (`edit::cad_pane_working_scene`/
/// `edit::cad_pane_working_objects`) must read real objects back out of them instead of the
/// hardcoded empty slice the defect shipped with. Checks the exact `instances_json` string
/// `build_world_scene_for_pane` feeds `MeshWindowKit::render` — the built scene's world-3d
/// payload the defect left permanently empty — for every pane, not just via the lower-level
/// `world_instances_json(&scene.building_objects, ..)` shortcut `forest_example_uses_per_object_brep_meshes` uses.
#[semio_framework_async_macros::async_test]
async fn forest_example_world_scene_has_non_empty_instances_for_every_pane() {
    let document = forest_play_scene();
    for pane in CadPaneId::all() {
        let working_scene = edit::cad_pane_working_scene(&document, pane).unwrap_or_else(|| panic!("pane {pane:?} must resolve a local-owner working scene"));
        let (objects, _geometry) = edit::cad_pane_working_objects(&working_scene, pane);
        assert!(!objects.is_empty(), "pane {pane:?} must have real objects, not the empty-defect slice");
        let instances_json = edit::world_instances_json(objects, &view(document.clone(), CadPlayRuntime::default()));
        assert_ne!(instances_json, "[]", "pane {pane:?} instances_json must not be empty");
        let meshes_json = edit::world_meshes_json(objects, _geometry);
        assert!(meshes_json.contains("\"data\""), "pane {pane:?} must inline its solids' tessellation into meshesJson");
        assert!(!meshes_json.contains("\"kind\""), "pane {pane:?} must not degrade authored solids to placeholder kinds");
    }
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_one_window_scoped_dislocate_utility() {
    let definition = create_cad_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, vec![CAD_DISLOCATE_UTILITY_ID]);
    // 🧰️ The framework auto-injects `setActiveUtility` as a View action once utilities are declared —
    // cad must NOT also declare it as an Mutation.
    let set_active_utility = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility auto-injected");
    assert_eq!(set_active_utility.kind, ActionKind::View);
    // 🚦️ Transform utilities gate the action panel while active (the default) — cad declares no
    // passive `allows_actions_while_active` view utilities.
    assert!(definition.utilities.iter().all(|utility| !utility.allows_actions_while_active));
    // 🧭️ Every world-3d pane owns its own Dislocate utility activation.
    for window in &definition.window_kinds {
        let refs: Vec<&str> = window.utilities.iter().map(|utility_ref| utility_ref.as_str()).collect();
        assert_eq!(refs, vec![CAD_DISLOCATE_UTILITY_ID], "window {} utilities", window.id);
    }
}

/// 🧱️ The manifest stitch: every window kind / panel tab the taxonomy nodes export lands in the
/// built `AppDefinition` with the same id, body key, surface kind and (empty) manifest measures the
/// pre-consolidation scalar `.window_kind(..)`/`.panel_tab(..)` calls produced — measures stay
/// config-derived per frame via `ArtifactApp::window_measures`, never frozen into the manifest.
#[semio_framework_async_macros::async_test]
async fn manifest_stitches_every_taxonomy_node_with_its_pre_migration_shape() {
    let definition = create_cad_app();
    let windows: Vec<(&str, &str)> = definition.window_kinds.iter().map(|window| (window.id.as_str(), window.body_key.as_str())).collect();
    assert_eq!(windows, vec![(shape::WINDOW_KIND_ID, shape::BODY_KEY), (building::WINDOW_KIND_ID, building::BODY_KEY), (energy::WINDOW_KIND_ID, energy::BODY_KEY), (structure_classic::WINDOW_KIND_ID, structure_classic::BODY_KEY),]);
    for window in definition.window_kinds.iter() {
        assert_eq!(window.surface_kind, ui_wgpu::wgpu::SurfaceKind::World3d, "window {} surface kind", window.id);
        assert!(window.options.measures.is_empty(), "window {} must not freeze measures into the manifest", window.id);
    }
    let modes: Vec<&str> = definition.modes.iter().map(|mode| mode.id.as_str()).collect();
    assert_eq!(modes, vec![edit::CAD_PLAY_MODE_EDIT]);
    assert_eq!(definition.default_mode_id, edit::CAD_PLAY_MODE_EDIT);
    // 🕰️ The framework appends its own history tab after the app-declared ones.
    let panels: Vec<(&str, Option<&str>)> = definition.panel_tabs.iter().map(|tab| (tab.id(), tab.body_key.as_deref())).take(3).collect();
    assert_eq!(
        panels,
        vec![
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID, Some(document::CAD_PLAY_BODY_ARTIFACT)),
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID, Some(catalogue::CAD_PLAY_BODY_CATALOGUE)),
            (semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID, Some(inspection::CAD_PLAY_BODY_PROPERTIES)),
        ]
    );
    let layout_json = json::to_json_string(&edit::layout());
    for window_kind_id in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
        assert!(layout_json.contains(window_kind_id), "default quad layout must place {window_kind_id}: {layout_json}");
    }
    assert_eq!(definition.artifact_kinds.iter().map(|kind| kind.id.as_str()).collect::<Vec<_>>(), vec!["3d.cad"]);
}

/// 🖼️ A reference pick is app-owned runtime state, and it clears the framework `"cad"` object
/// selection through the sanctioned `InteractionWrite` lane so the inspection panel can show it.
#[semio_framework_async_macros::async_test]
async fn reference_pick_selects_the_reference_and_clears_the_cad_domain() {
    let scene = forest_play_scene();
    let selected = forest_working_scene().building_objects[0].id.clone();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let command = command_from_action("setReferenceSelection", Some(&json!({ "modelDefinitionId": CAD_MODEL_DEFINITION_ENERGY, "referenceId": "ref-concrete-forest" })));
    let mut ctx = CadDispatchCtx { interaction: selecting(&[selected.as_str()]), preview_operation: None, view_state: None };
    let emit = command.dispatch(&doc, &cfg, &mut ctx).expect("reference pick");
    assert!(emit.artifact_mutations.is_empty(), "a reference pick is never a document edit");
    let runtime = runtime_after(&emit, &config);
    assert_eq!(runtime.selected_reference_model_definition_id.as_deref(), Some(CAD_MODEL_DEFINITION_ENERGY));
    assert_eq!(runtime.selected_reference_id.as_deref(), Some("ref-concrete-forest"));
    assert_eq!(emit.interaction_writes.len(), 1);
    assert_eq!(emit.interaction_writes[0].domain, CAD_INTERACTION_DOMAIN);
    assert_eq!(emit.interaction_writes[0].merge, protocol::MergeMode::Subtractive, "an empty replace is a no-op; the live ids are subtracted");
    assert_eq!(emit.interaction_writes[0].targets.iter().map(|target| target.id.as_str()).collect::<Vec<_>>(), vec![selected.as_str()]);
    let mut idle = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation: None, view_state: None };
    assert!(command.dispatch(&doc, &cfg, &mut idle).expect("reference pick").interaction_writes.is_empty(), "nothing selected, nothing to subtract");
    let view = view(scene, runtime);
    let panel = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: inspection::build_properties_panel(&view, cad_labels(&ViewModel::default()), None, &semio_framework_plugin::TreeWindows::unhosted()).expect("panel") }).expect("projection");
    assert!(panel.contains("cad-play-inspector.reference.width.grow"), "{panel}");
}

#[semio_framework_async_macros::async_test]
async fn internal_and_plumbing_actions_excluded_from_palette() {
    let definition = create_cad_app();
    let hidden_actions = [
        "patchCadPlayReference",
        "engagementSubmit",
        "setNodeSelection",
        "setReferenceSelection",
        "referenceHover",
        "engagementInput",
        "engagementPossibleSelect",
        "engagementRepeatLast",
        "engagementAbort",
        "worldPointerDown",
        "worldPointerMove",
        "setDislocateOption",
    ];
    for action_id in hidden_actions {
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("action {action_id} missing from manifest"));
        assert!(!action.in_palette, "internal action {action_id} must have in_palette: false");
    }

    let palette_user_actions = ["addObject", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection"];
    for action_id in palette_user_actions {
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("user action {action_id} missing from manifest"));
        assert!(action.in_palette, "user action {action_id} must have in_palette: true");
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_input_and_possible_engagements_present() {
    let mut app = new_app().await;
    let engagements = app.window_engagements(&ViewModel::default()).await;
    let shape = engagements.get(shape::WINDOW_KIND_ID).expect("shape engagement");
    assert!(shape.input.is_some());
    assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
}

/// 🕹️ The interaction-view threading law for window chrome: the HUD's `cad-status` row counts the
/// live `"cad"` domain selection `window_engagements_with_request_context` threads in, so it can never
/// disagree with what the world scene paints. Before this was threaded the row read `0` forever.
#[semio_framework_async_macros::async_test]
async fn the_engagement_hud_counts_the_threaded_cad_selection() {
    let labels = cad_labels(&ViewModel::default());
    let count_of = |interaction: CadInteractionSnapshot| -> String {
        let view = view_with_interaction(forest_play_scene(), CadPlayRuntime::default(), interaction);
        let engagement = shape::engagement(&view, labels);
        engagement.status.expect("cad status rows").into_iter().find(|row| row.id == "cad-status").expect("cad-status row").text
    };
    assert!(count_of(CadInteractionSnapshot::default()).starts_with('0'), "an empty cad domain must report zero selected");
    let selected = CadInteractionSnapshot { granularity: "object".into(), ids: vec!["1".into(), "2".into()], anchor_id: None, hovered_ids: Vec::new() };
    assert!(count_of(selected).starts_with('2'), "the HUD must count the threaded cad selection");
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_registered_for_all_four_panes() {
    let mut app = new_app().await;
    let engagements = app.window_engagements(&ViewModel::default()).await;
    for window_kind in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
        assert!(engagements.contains_key(window_kind), "missing engagement for {window_kind}");
    }
}

#[semio_framework_async_macros::async_test]
async fn forest_example_includes_reference_overlay() {
    let scene = forest_play_scene();
    let references = edit::world_references_json(&scene, CadPaneId::Shape).expect("references");
    assert!(references.contains("ref-concrete-forest"));
}

#[semio_framework_async_macros::async_test]
async fn typology_extent_derives_from_authored_geometry() {
    let scene = forest_working_scene();
    let column = scene.building_objects.iter().find(|object| object.typology == "building.building.column").expect("column object");
    let extent = column.extent.expect("column extent derived from geometry");
    assert!(extent[2] > 0.05, "authored column height should be measurable");
    assert_ne!(extent, CAD_DEFAULT_TYPOLOGY_EXTENT, "should differ from the universal fallback");
}
//#endregion 🔖️Render
//#region 🔖️ViewModel
#[semio_framework_async_macros::async_test]
async fn gumball_config_fields_present_regardless_of_dislocate_activation() {
    let selection = edit::world_selection_json(&view(default_document(), CadPlayRuntime::default()), CadPaneId::Shape, &[], Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(selection.contains("\"transformMode\":\"transform\""));
    assert!(selection.contains("\"moveAxes\":true"));
    assert!(selection.contains("\"rotate\":true"));
    assert!(selection.contains("\"scaleAxes\":false"));
    assert!(selection.contains("\"gumballActive\":false"), "no selection, no gumball");
    assert!(!selection.contains("\"gumballTarget\""));
}

/// 🕹️ The live `"cad"` domain reaches the world scene: selected/hovered ids ride the selection lane
/// and every instance carries its own flags, and the Dislocate gumball shows for a live selection.
#[semio_framework_async_macros::async_test]
async fn live_cad_selection_reaches_the_world_scene_and_arms_the_gumball() {
    let scene = forest_working_scene();
    let first = scene.building_objects[0].id.clone();
    let second = scene.building_objects[1].id.clone();
    let mut interaction = selecting(&[first.as_str()]);
    interaction.hovered_ids = vec![second.clone()];
    let view = view_with_interaction(forest_play_scene(), CadPlayRuntime::default(), interaction);
    let selection = edit::world_selection_json(&view, CadPaneId::Building, &scene.building_objects, Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(selection.contains(&format!("\"ids\":[\"{first}\"]")), "selection lane carries the domain's ids: {selection}");
    assert!(selection.contains(&format!("\"hoveredId\":\"{second}\"")), "selection lane carries the domain's hover: {selection}");
    assert!(selection.contains("\"gumballActive\":true"), "dislocate + selection arms the gumball: {selection}");
    let other_pane = edit::world_selection_json(&view, CadPaneId::Energy, &scene.energy_objects, Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(other_pane.contains("\"ids\":[]"), "a building selection never leaks into the energy pane's lane: {other_pane}");
    assert!(other_pane.contains("\"hoveredId\":null"), "a building hover never leaks into the energy pane's lane: {other_pane}");
    let instances: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(&scene.building_objects, &view)).expect("instances json");
    assert_eq!(instances[0]["selected"], serde_json::Value::Bool(true));
    assert_eq!(instances[0]["hovered"], serde_json::Value::Bool(false));
    assert_eq!(instances[1]["selected"], serde_json::Value::Bool(false));
    assert_eq!(instances[1]["hovered"], serde_json::Value::Bool(true));
    let idle = edit::world_selection_json(&view, CadPaneId::Building, &scene.building_objects, None, CadDislocateOptions::default());
    assert!(idle.contains("\"gumballActive\":false"), "no dislocate utility, no gumball even with a selection");
}

/// 🎥️ Camera edits target the host-authenticated exact window and never app or document state.
#[semio_framework_async_macros::async_test]
async fn set_camera_writes_config_not_mutations() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let emit = drive_in_window(&app, &scene, "setCamera", Some(json!({ "surfaceId": "cad.play.scene3d/building", "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 2.0, "fov": 60.0 } })), &CadConfig::default(), "building-right", building::WINDOW_KIND_ID).expect("addressed camera command");
    assert!(emit.artifact_mutations.is_empty(), "setCamera must not emit a VCS operation");
    assert!(emit.config_mutations.is_empty(), "setCamera must not mutate app configuration");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "building-right");
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), building::WINDOW_KIND_ID);
}

/// 🖼️ The four panes' reference overlays share one reference id; only the pane whose model
/// definition the app-owned reference selection names paints it selected.
#[semio_framework_async_macros::async_test]
async fn reference_selection_paints_only_its_own_pane() {
    let runtime = CadPlayRuntime { selected_reference_model_definition_id: Some(CAD_MODEL_DEFINITION_ENERGY.into()), selected_reference_id: Some("ref-concrete-forest".into()), ..CadPlayRuntime::default() };
    let view = view(forest_play_scene(), runtime);
    let energy = edit::world_selection_json(&view, CadPaneId::Energy, &[], None, CadDislocateOptions::default());
    assert!(energy.contains("\"referenceSelectedId\":\"ref-concrete-forest\""), "{energy}");
    let building = edit::world_selection_json(&view, CadPaneId::Building, &[], None, CadDislocateOptions::default());
    assert!(!building.contains("referenceSelectedId"), "{building}");
}

#[semio_framework_async_macros::async_test]
async fn gumball_inactive_without_selection() {
    let selection = edit::world_selection_json(&view(default_document(), CadPlayRuntime::default()), CadPaneId::Shape, &[], Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
    assert!(selection.contains("\"gumballActive\":false"));
    assert!(!selection.contains("\"gumballTarget\""));
}

#[semio_framework_async_macros::async_test]
async fn active_utility_flows_from_the_host_view_into_scene() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let view_state = ViewModel { active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()), ..ViewModel::default() };
    let node = render_direct(&app, shape::BODY_KEY, &doc, &config, &view_state).expect("CAD UI assembly");
    let selection = scene_lane(&node, "selection");
    assert!(selection.contains(r#""transformMode":"transform""#), "render sources Dislocate from ViewModel.active_utility_id: {selection}");
}

#[semio_framework_async_macros::async_test]
async fn dislocate_utility_is_scoped_by_each_window_view_context() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let shape_view = ViewModel { active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()), ..ViewModel::default() };
    let building_view = ViewModel { active_utility_id: None, ..ViewModel::default() };
    let shape = scene_lane(&render_direct(&app, shape::BODY_KEY, &doc, &config, &shape_view).expect("CAD UI assembly"), "selection");
    let building = scene_lane(&render_direct(&app, building::BODY_KEY, &doc, &config, &building_view).expect("CAD UI assembly"), "selection");
    assert!(shape.contains(r#""gumballActive":false"#), "the interaction-less render path has no selection to arm: {shape}");
    assert!(shape.contains(r#""transformMode":"transform""#));
    assert!(building.contains(r#""gumballActive":false"#));
    assert!(!building.contains(r#""transformMode":"transform""#));
}

#[semio_framework_async_macros::async_test]
async fn context_menu_resolves_labels_from_the_registry() {
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `context_menu` is no longer
    // selection-gated — `ArtifactApp::context_menu` has no `InteractionView` parameter, so it
    // can no longer tell whether anything is selected (see its own doc comment); it always
    // shows the transform/duplicate/delete section now.
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let registry = AppActionRegistry::from_definition(&create_cad_app());
    let config = CadConfig::default();

    let view_state = ViewModel::default();
    let items = context_menu_direct(&app, &doc, &config, &view_state, &registry);
    assert!(items.iter().any(|item| item.id == "translateSelection" && item.label.is_some()), "labels must resolve from the registry: {items:?}");
    assert!(items.iter().any(|item| item.id == "deleteObject" && item.destructive == Some(true)), "deleteObject must be marked destructive: {items:?}");
}

/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
/// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
/// and the destructive `deleteObject` action stays the trailing item.
#[semio_framework_async_macros::async_test]
async fn context_menu_is_grouped_and_keeps_delete_object_last() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let registry = AppActionRegistry::from_definition(&create_cad_app());
    let config = CadConfig::default();

    let view_state = ViewModel::default();
    let items = context_menu_direct(&app, &doc, &config, &view_state, &registry);

    assert!(items.len() <= 9, "top-level context menu should stay progressively disclosed: {items:?}");
    assert_eq!(items.last().map(|item| item.id.as_str()), Some("deleteObject"), "deleteObject must stay the trailing item: {items:?}");
    assert_eq!(items.last().and_then(|item| item.destructive), Some(true), "trailing deleteObject must be marked destructive: {items:?}");
}

/// 🎛️ Dislocate preferences target an exact window instance.
#[semio_framework_async_macros::async_test]
async fn dislocate_move_and_rotate_options_are_per_pane() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let emit = drive_in_window(&app, &scene, "setDislocateOption", Some(json!({ "pane": "building", "option": "rotate", "pressed": false })), &CadConfig::default(), "building-left", building::WINDOW_KIND_ID).expect("addressed utility command");
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "building-left");
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), building::WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn engagement_hud_no_longer_carries_utility_switcher_options() {
    let mut app = new_app().await;
    let engagements = app.window_engagements(&ViewModel::default()).await;
    for engagement in engagements.values() {
        assert!(engagement.options.is_none(), "utility switching now lives in the framework utility bar, not the engagement HUD");
    }
}

#[semio_framework_async_macros::async_test]
async fn utility_switch_has_no_plugin_command_or_config_lane() {
    assert!(<CadPlayApp as ArtifactEditor>::command_from_action(SET_ACTIVE_UTILITY_ACTION_ID, None).is_err());
    assert!(!CAD_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
    assert!(!every_command().iter().any(|command| command.command_id() == SET_ACTIVE_UTILITY_ACTION_ID));
}

#[semio_framework_async_macros::async_test]
async fn sun_measures_registered_for_all_four_panes_and_default_off() {
    let app = CadPlayApp::default();
    let base_config = CadConfig::default();
    assert!(!edit::windows::config::CadWorldWindowConfig::default().sun.enabled, "sun must be off by default");
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let view_state = ViewModel {
        window_id: Some("shape-sun".into()),
        active_window_kind_id: Some(shape::WINDOW_KIND_ID.into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "shape-sun".into(), window_kind_id: shape::WINDOW_KIND_ID.into() }],
        ..ViewModel::default()
    };
    let measures = window_measures_direct(&app, &doc, &base_config, &view_state);
    assert!(measures.contains_key("shape-sun"), "missing exact-window sun measures");
    let emit = drive_in_window(&app, &scene, "toggleSun", None, &base_config, "shape-sun", shape::WINDOW_KIND_ID).expect("addressed sun command");
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "shape-sun");
}

// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `worldPick`/`setHover`/`setSelection`
// and their round-trip tests are DELETED, not migrated — mesh object/vertex/edge/face
// selection AND hover are now the framework-owned `"cad"` interaction domain, dispatched
// through the auto-injected `interactionSelect`/`interactionHover` verbs (never app-declared)
// and tested once, centrally, by the framework's own `semio-framework-plugin` suite.

//#endregion 🔖️ViewModel
//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn add_object_action_seeds_an_empty_pane_with_a_composed_model_child() {
    // 🪆️ 2026-09-16: `addObject` is a real mutation now. `default_document()` holds no model child
    // at all, so this is the empty-pane arm of the re-materialization seam — the created object
    // mints the pane's composed `s.stdio.semio.model` child rather than dropping on the floor.
    // Selection stays out of scope (framework-owned, unreachable from `handle()`).
    let app = CadPlayApp::default();
    let scene = default_document();
    assert!(scene.shape_model.is_none(), "this arm needs a pane with no composed child yet");
    let emit = drive(&app, &scene, "addObject", Some(json!({ "typology": "building.building.column" })));
    assert_eq!(emit.artifact_mutations.len(), 1, "addObject emits one bounded parent op");
    let after = apply_mutations(&scene, &emit.artifact_mutations);
    assert!(after.shape_model.is_some(), "the first object mints the pane's composed model child");
    let objects = cad_pane_objects(&after, CadPaneId::Shape);
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].typology, "building.building.column");
}

#[semio_framework_async_macros::async_test]
async fn add_object_through_wrapper_grows_the_composed_pane() {
    // ⚠️ Test-harness debt, not app behaviour: `artifact_app_laws::new_app` is registry-less, so
    // this dispatch fails its tool-proof catalog before reaching the reducer (see the ticket's own
    // "Registryless testkit::new_app Unusable" note). The assertion below is what it must prove
    // once that harness gains a registry; the no-op claim it used to make is simply wrong now.
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot");
    app.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &meta("local")).await.expect("add object dispatch");
    let after = app.snapshot().expect("snapshot");
    assert_ne!(json::to_json_string(&before), json::to_json_string(&after), "addObject must re-mint the addressed pane's composed model child");
}

#[semio_framework_async_macros::async_test]
async fn derive_transformation_populates_energy_pane() {
    // ⚠️ `apply_transformation_mutations` is a documented no-op pending the child-dispatch seam
    // (see its own doc comment in this file) — this instead exercises the real derive algorithm
    // directly (`run_derive_from_geometry`), the pure function `applyTransformation` will call
    // once that seam exists. `make_object_for_typology` already built and dropped its own local
    // `cad_brep_kernel()` internally to mint the object's solid handle; `cad_brep_kernel()` is a
    // fresh `Brep::new()` per call (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    // wave G4 — no shared lock, no reentrancy concern), so the kernel built HERE for
    // `run_derive_from_geometry` is its own independent instance.
    let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let derived = run_derive_from_geometry(&mut kernel, &[object], "energy");
    assert!(!derived.is_empty());
    assert!(derived.iter().any(|object| object.typology.starts_with("energy.energy.")));
}

#[semio_framework_async_macros::async_test]
async fn forest_transformation_uses_live_shape_pane() {
    // ⚠️ CORRECTED (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    // wave G4): this test used to derive from `forest_working_scene().objects` and compare
    // against a single live box, relying on the forest fixture's `solid_handle`s resolving into
    // whatever kernel `run_derive_from_geometry` reached — only true under the deleted
    // process-global `BrepEngineHost` singleton. `solid_for_object` (the derive's per-object
    // solid builder) has never applied `object.origin` — fixture objects carry `origin:
    // [0,0,0]` regardless (`objects_from_host_snapshot_model`) — so once a handle stops resolving it
    // falls back to an extent+typology-only box built at the kernel's local origin, and two
    // fixture objects sharing that origin fuse into a materially different (and no longer
    // fixture-distinguishing) hull. The real, still-true property — output tracks LIVE INPUT,
    // not a memoized static result — is verified here directly against extent, the one field
    // that DOES still flow through `solid_for_object`'s fallback path honestly.
    let live_box = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let box_derived = run_derive_from_geometry(&mut kernel, &[live_box], "energy");
    assert!(!box_derived.is_empty(), "a live box must derive at least a hull");

    let live_wall = make_object_for_typology("building.building.wall", 0, CadPaneId::Shape);
    let mut kernel = cad_brep_kernel();
    let wall_derived = run_derive_from_geometry(&mut kernel, &[live_wall], "energy");
    assert!(!wall_derived.is_empty(), "a live wall panel must derive at least a hull");

    let wall_typologies: Vec<&str> = wall_derived.iter().map(|object| object.typology.as_str()).collect();
    let box_typologies: Vec<&str> = box_derived.iter().map(|object| object.typology.as_str()).collect();
    assert_ne!(
        wall_typologies, box_typologies,
        "a thin wall panel and a cube must classify their dominant faces differently, proving the derive tracks the LIVE input's real shape, not a memoized result:\n  box:  {box_typologies:?}\n  wall: {wall_typologies:?}"
    );
}

#[semio_framework_async_macros::async_test]
async fn save_selected_emits_download_effect() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let emit = drive_in_window(&app, &scene, "saveSelected", None, &config, "cad-shape-secondary", shape::WINDOW_KIND_ID).expect("addressed CAD window export");
    assert!(emit.artifact_mutations.is_empty(), "export must not mutate the document");
    assert_eq!(emit.effects.len(), 1);
    match &emit.effects[0] {
        Effect::DownloadMediaExport { filename, data, .. } => {
            assert_eq!(filename, "cad.selected.spatial.dsl");
            assert!(!data.contains("activeModelDefinitionId"));
            assert!(data.contains("spatial.shape"));
        }
        other => panic!("expected DownloadMediaExport, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn load_raw_request_emits_file_open_effect() {
    let app = CadPlayApp::default();
    let emit = drive(&app, &default_document(), "loadRawRequest", None);
    match &emit.effects[0] {
        Effect::RequestFileOpen { import_action, read_as, .. } => {
            assert_eq!(import_action, "importCadFile");
            assert_eq!(read_as.as_deref(), Some("dataUrl"));
        }
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
}
//#endregion 🔖️Operations
//#region 🔖️Engagement
#[semio_framework_async_macros::async_test]
async fn engagement_starts_box_interaction_session() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let runtime = runtime_after(&emit, &config);
    assert!(runtime.engagement_session.is_some());
}

#[semio_framework_async_macros::async_test]
async fn world_pointer_move_updates_live_preview_without_committing_or_emitting_mutations() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let config = config_after(&emit, &config);

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
    assert!(emit.artifact_mutations.is_empty(), "a pointer move must not emit any document operation");
    let runtime = runtime_after(&emit, &config);
    let session = runtime.engagement_session.as_ref().expect("session still active");
    assert_eq!(session.state, "first_corner", "pointer.move must not change state");
    assert_eq!(session.context.get("cursor"), Some(&json::to_dsl_value(&json!([3.0, 4.0, 0.0]))));
}

//#region 🔖️GesturePreview
fn preview_operation(app_instance_id: u32) -> CadPreviewOperationIdentity {
    CadPreviewOperationIdentity { app_instance_id, parent_document_id: "cad-preview-document".into(), operation_id: 41, operation_generation: 3, canonical_base_revision: "cd".repeat(32) }
}

fn persisted_preview_stamp(config: &CadConfig) -> CadPreviewStamp {
    CadPreviewStamp { operation: json::from_json_str(config.engagement_preview_operation_json.as_ref().expect("persisted operation identity")).expect("valid persisted operation identity"), generation: config.engagement_preview_generation }
}

fn spatial_scene_import_args() -> Value {
    let file_text = json!({
        "schema": "spatial.model",
        "revision": 1,
        "modelDefinitionId": "spatial.shape",
        "objects": [{
            "id": "object-preview-transition",
            "label": "Preview transition",
            "typology": "spatial.shape.primitive.box",
            "visible": true,
            "locked": false,
            "origin": [0.0, 0.0, 0.0],
            "primitives": []
        }]
    })
    .to_string();
    json!({ "payload": file_text, "name": "preview-transition.spatial.json" })
}

/// 🔬️ CW7 preview-law seam: `CadPlayApp::gesture_preview` reads `CadEngagementScratch` only, never
/// `CadSnapshot`/`CadMutation` — driven through the real `worldPointerMove` handler (the natural
/// per-tick gesture handler) via the existing `drive` helper, config threaded explicitly across
/// calls (the pure `CadPlayApp` no longer holds any of this state itself).
#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_none_without_a_live_engagement_session() {
    let app = CadPlayApp::default();
    assert!(app.gesture_preview(&CadConfig::default()).is_none(), "no live engagement session, nothing to preview");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_reflects_the_live_rubber_band_preview_and_clears_on_abort() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let config = config_after(&emit, &config);

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
    let config = config_after(&emit, &config);
    let first = app.gesture_preview(&config).expect("a live engagement session is previewable");
    let value: Value = json::parse_bytes(&first.payload).expect("payload is valid json");
    assert_eq!(value["context"]["cursor"], json!([3.0, 4.0, 0.0]));

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [5.0, 6.0, 0.0] })), &config);
    let config = config_after(&emit, &config);
    let second = app.gesture_preview(&config).expect("still live mid-gesture");
    assert_eq!(second.stamp.operation, first.stamp.operation);
    assert_eq!(second.stamp.generation, first.stamp.generation + 1, "the persisted preview generation advances exactly once per changed checkpoint");
    assert!(second.is_fresher_than(&first.stamp));
    let value_after_second: Value = json::parse_bytes(&second.payload).expect("payload is valid json");
    assert_eq!(value_after_second["context"]["cursor"], json!([5.0, 6.0, 0.0]), "preview tracks the live cursor, not the gesture start");

    let emit = drive_with_config(&app, &scene, "engagementAbort", None, &config);
    let config = config_after(&emit, &config);
    assert!(app.gesture_preview(&config).is_none(), "the engagement session was aborted: nothing left to preview");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_a_pure_read_never_mutating_the_engagement_session() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    let config = config_after(&emit, &config);
    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &config);
    let config = config_after(&emit, &config);
    let session_before = config.engagement_session_json.clone();
    let first = app.gesture_preview(&config);
    let second = app.gesture_preview(&config);
    assert_eq!(first, second, "equal checkpoint reads keep the exact same freshness stamp");
    assert_eq!(config.engagement_session_json, session_before, "gesture_preview must never mutate the live engagement session it reads");
}

#[semio_framework_async_macros::async_test]
async fn production_transition_authority_routes_engagement_utility_and_import_without_noop_increment() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let operation = preview_operation(1);
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };

    let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("ordinary engagement transition");
    let started = config_after(&started_emit, &base);
    let started_stamp = persisted_preview_stamp(&started);
    assert!(started.engagement_session_json.is_some());
    assert_eq!(started_stamp.operation, operation);
    assert_eq!(started_stamp.generation, base.engagement_preview_generation + 1);

    let utility_emit = drive_with_operation(&app, &scene, "engagementAbort", None, &started, Some(operation.clone())).expect("engagement cancellation transition");
    let utility_cleared = config_after(&utility_emit, &started);
    let utility_stamp = persisted_preview_stamp(&utility_cleared);
    assert!(utility_cleared.engagement_session_json.is_none());
    assert!(utility_stamp.is_fresher_than(&started_stamp));
    assert_eq!(utility_stamp.generation, started_stamp.generation + 1);

    let noop_emit = drive_with_operation(&app, &scene, "engagementAbort", None, &utility_cleared, Some(operation.clone())).expect("same checkpoint cancellation");
    let noop = config_after(&noop_emit, &utility_cleared);
    assert_eq!(noop.engagement_session_json, utility_cleared.engagement_session_json);
    assert_eq!(persisted_preview_stamp(&noop), utility_stamp, "a non-session config change must not advance the preview generation");

    let input_emit = drive_with_operation(&app, &scene, "engagementInput", Some(json!({ "value": "b", "pane": "shape" })), &noop, Some(operation.clone())).expect("engagement input");
    let with_input = config_after(&input_emit, &noop);
    assert_eq!(persisted_preview_stamp(&with_input), utility_stamp, "input-only config must preserve the stamp");
    let restarted_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &with_input, Some(operation.clone())).expect("restart engagement");
    let restarted = config_after(&restarted_emit, &with_input);
    let restarted_stamp = persisted_preview_stamp(&restarted);
    assert!(restarted_stamp.is_fresher_than(&utility_stamp));

    let import_emit = drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &restarted, Some(operation)).expect("scene import clear transition");
    let import_cleared = config_after(&import_emit, &restarted);
    let import_stamp = persisted_preview_stamp(&import_cleared);
    assert!(import_cleared.engagement_session_json.is_none());
    assert!(import_stamp.is_fresher_than(&restarted_stamp));
    assert_eq!(import_stamp.generation, restarted_stamp.generation + 1);
    assert!(matches!(import_emit.effects.first(), Some(Effect::LoadDocument { .. })));

    let example_emit = drive_with_operation(&app, &scene, "setActiveExample", Some(json!({ "exampleId": "" })), &restarted, Some(preview_operation(1))).expect("active example clear transition");
    let example_cleared = config_after(&example_emit, &restarted);
    let example_stamp = persisted_preview_stamp(&example_cleared);
    assert!(example_cleared.engagement_session_json.is_none());
    assert!(example_stamp.is_fresher_than(&restarted_stamp));
    assert_eq!(example_stamp.generation, restarted_stamp.generation + 1);
    assert!(matches!(example_emit.effects.first(), Some(Effect::LoadDocument { .. })));
}

#[semio_framework_async_macros::async_test]
async fn production_transition_authority_isolates_two_app_aba_sequences() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let operations = [preview_operation(1), preview_operation(2)];
    let mut previews = Vec::new();

    for operation in operations {
        let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("start app-local engagement");
        let started = config_after(&started_emit, &base);
        let a_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &started, Some(operation.clone())).expect("A");
        let at_a = config_after(&a_emit, &started);
        let first_a = app.gesture_preview(&at_a).expect("first A preview");
        let b_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &at_a, Some(operation.clone())).expect("B");
        let at_b = config_after(&b_emit, &at_a);
        let a_again_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &at_b, Some(operation)).expect("A again");
        let at_a_again = config_after(&a_again_emit, &at_b);
        let second_a = app.gesture_preview(&at_a_again).expect("second A preview");
        assert_eq!(first_a.payload, second_a.payload);
        assert_eq!(second_a.stamp.generation, first_a.stamp.generation + 2);
        assert_ne!(first_a.stamp, second_a.stamp);
        previews.push(second_a);
    }

    assert_eq!(previews[0].payload, previews[1].payload);
    assert_eq!(previews[0].stamp.generation, previews[1].stamp.generation);
    assert_ne!(previews[0].stamp.operation, previews[1].stamp.operation);
    assert!(!previews[0].is_fresher_than(&previews[1].stamp));
    assert!(!previews[1].is_fresher_than(&previews[0].stamp));
}

#[semio_framework_async_macros::async_test]
async fn production_transition_exhaustion_and_missing_context_fail_before_checkpoint_persistence() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let operation = preview_operation(9);
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("start live engagement");
    let started = config_after(&started_emit, &base);
    let mut at_max = started.clone();
    at_max.engagement_preview_generation = CAD_PREVIEW_GENERATION_MAX;
    let checkpoint_before = at_max.engagement_session_json.clone();
    let operation_before = at_max.engagement_preview_operation_json.clone();

    assert!(drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &at_max, Some(operation.clone())).is_err());
    assert!(drive_with_operation(&app, &scene, "setActiveExample", Some(json!({ "exampleId": "" })), &at_max, Some(operation.clone())).is_err());
    assert!(drive_with_operation(&app, &scene, "engagementAbort", None, &at_max, Some(operation)).is_err());
    assert_eq!(at_max.engagement_session_json, checkpoint_before, "failed commands cannot persist their cleared checkpoint");
    assert_eq!(at_max.engagement_preview_generation, CAD_PREVIEW_GENERATION_MAX);
    assert_eq!(at_max.engagement_preview_operation_json, operation_before);

    assert!(drive_with_operation(&app, &scene, "engagementAbort", None, &started, None).is_err());
    assert!(drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &started, None).is_err());
    let mut bypass = cad_runtime_from_config(&started);
    bypass.engagement_session = None;
    assert!(snapshot_of(&bypass, &started).is_err(), "ordinary snapshots must reject session-transition bypasses");
    assert_eq!(started.engagement_session_json, checkpoint_before);
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_rejects_aba_collision_and_cross_app_stamps_and_survives_restart() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base);
    let started = config_after(&emit, &base);

    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &started);
    let at_a = config_after(&emit, &started);
    let preview_a = app.gesture_preview(&at_a).expect("A preview");
    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &at_a);
    let at_b = config_after(&emit, &at_a);
    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &at_b);
    let at_a_again = config_after(&emit, &at_b);
    let preview_a_again = app.gesture_preview(&at_a_again).expect("second A preview");
    assert_eq!(preview_a.payload, preview_a_again.payload, "fixture must exercise A → B → A");
    assert_eq!(preview_a_again.stamp.generation, preview_a.stamp.generation + 2);
    assert_ne!(preview_a.stamp, preview_a_again.stamp, "ABA payload equality cannot reproduce a freshness stamp");

    let restarted: CadConfig = json::from_json_str(&json::to_json_string(&at_a_again)).expect("cold reopen config");
    assert_eq!(app.gesture_preview(&restarted).expect("reopened preview").stamp, preview_a_again.stamp);

    let mut other_app = restarted.clone();
    other_app.engagement_preview_operation_json =
        Some(json::to_json_string(&CadPreviewOperationIdentity { app_instance_id: 2, parent_document_id: "cad-test-document".into(), operation_id: 1, operation_generation: 1, canonical_base_revision: "00".repeat(32) }));
    let collision = app.gesture_preview(&other_app).expect("other app preview");
    assert_eq!(collision.stamp.generation, preview_a_again.stamp.generation, "forced finite-generation collision fixture");
    assert_ne!(collision.stamp.operation, preview_a_again.stamp.operation);
    assert!(!collision.is_fresher_than(&preview_a_again.stamp), "freshness requires exact operation identity, not only a colliding counter");
}

#[semio_framework_async_macros::async_test]
async fn preview_generation_cross_surface_domain_round_trips_max_and_rejects_plus_one() {
    let app = CadPlayApp::default();
    let operation = CadPreviewOperationIdentity { app_instance_id: 7, parent_document_id: "cad-max-document".into(), operation_id: 11, operation_generation: 13, canonical_base_revision: "ab".repeat(32) };
    let at_max = CadConfig { engagement_session_json: Some("{}".into()), engagement_preview_operation_json: Some(json_string_of(&operation)), engagement_preview_generation: CAD_PREVIEW_GENERATION_MAX, ..CadConfig::default() };
    let mut encoded = protocol::ToValue::to_value(&at_max);
    let decoded = <CadConfig as protocol::FromValue>::from_value(encoded.clone()).expect("maximum generation deserializes exactly");
    assert_eq!(decoded.engagement_preview_generation, CAD_PREVIEW_GENERATION_MAX);
    assert_eq!(app.gesture_preview(&decoded).expect("maximum generation remains previewable").stamp.generation, CAD_PREVIEW_GENERATION_MAX);

    let plus_one = i64::from(CAD_PREVIEW_GENERATION_MAX) + 1;
    if let protocol::DslValue::Object(fields) = &mut encoded {
        for (key, value) in fields.iter_mut() {
            if key == "engagementPreviewGeneration" {
                *value = protocol::DslValue::int(plus_one);
            }
        }
    }
    assert!(<CadConfig as protocol::FromValue>::from_value(encoded).is_err(), "maximum + 1 must fail before entering persisted config");

    let mut runtime = cad_runtime_from_config(&decoded);
    runtime.engagement_session = None;
    let ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation: Some(operation), view_state: None };
    assert!(preview_transition_snapshot_of(&runtime, &decoded, &ctx).is_err(), "incrementing the maximum generation must fail closed");

    let json_schema: Value = json::parse(include_str!("../../🎚️config/🧬️schema/🔣️.json")).expect("CAD config JSON descriptor");
    let generation_schema = &json_schema["properties"]["engagementPreviewGeneration"];
    assert_eq!(generation_schema["minimum"], json!(0));
    assert_eq!(generation_schema["maximum"], json!(CAD_PREVIEW_GENERATION_MAX));
    assert!(include_str!("../../🎚️config/🧬️schema/🛰️.proto").contains("int32 engagement_preview_generation = 11;"));
    assert!(include_str!("../../🎚️config/🧬️schema/🔗️.graphql").contains("engagementPreviewGeneration: Int!"));
    assert!(include_str!("../../🎚️config/🧬️schema/🟦️.ts").contains("engagementPreviewGeneration: number;"));
    assert!(include_str!("../../🎚️config/🧬️schema/🦀️.rs").contains("engagement_preview_generation: i32"));
}
//#endregion 🔖️GesturePreview

#[semio_framework_async_macros::async_test]
async fn engagement_repeat_last_restarts_the_last_finalized_interaction() {
    let app = CadPlayApp::default();
    let mut scene = default_document();
    let mut config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    config = config_after(&emit, &config);
    assert!(cad_runtime_from_config(&config).engagement_session.is_some());

    // 📦️box.json's default boxMode is "point" (length/width prompt); select diagonal mode (key
    // "d") to reach the classic two-corner-click flow.
    config.engagement_input = "d".into();
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    config = config_after(&emit, &config);

    for position in [json!([0.0, 0.0, 0.0]), json!([2.0, 3.0, 0.0])] {
        let emit = drive_with_config(&app, &scene, "worldPointerDown", Some(json!({ "pane": "shape", "position": position })), &config);
        scene = apply_mutations(&scene, &emit.artifact_mutations);
        config = config_after(&emit, &config);
    }

    config.engagement_input = "SetHeight2.5".into();
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    config = config_after(&emit, &config);

    // 📦️box.json's `set.height` only records the height (state stays first_corner_height); an
    // explicit `confirm` (Enter) is needed to reach `ready`, box's commit.fromStates.
    config.engagement_input = "Confirm".into();
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    scene = apply_mutations(&scene, &emit.artifact_mutations);
    config = config_after(&emit, &config);
    let runtime = cad_runtime_from_config(&config);
    assert!(runtime.engagement_session.is_none(), "box should have committed");
    assert_eq!(runtime.last_finalized_interaction_id.as_deref(), Some("primitive.box"));

    let emit = drive_with_config(&app, &scene, "engagementRepeatLast", Some(json!({ "pane": "shape" })), &config);
    let runtime = runtime_after(&emit, &config);
    let session = runtime.engagement_session.as_ref().expect("repeat-last should start a session");
    assert_eq!(session.interaction_id, "primitive.box");
}
//#endregion 🔖️Engagement
//#region 🔖️Import
#[semio_framework_async_macros::async_test]
async fn import_spatial_modelspace_round_trips() {
    let payload = json!({
        "schema": "spatial.modelspace",
        "revision": 1,
        "activeModelDefinitionId": "spatial.shape",
        "models": [{
            "id": "spatial.shape",
            "model": {
                "schema": "spatial.model",
                "revision": 1,
                "objects": [{
                    "id": "object-imported",
                    "label": "Imported",
                    "typology": "spatial.shape.primitive.box",
                    "visible": true,
                    "locked": false,
                    "origin": [1.0, 2.0, 3.0],
                    "primitives": []
                }]
            }
        }]
    });
    let scene = scene_from_spatial_payload(&json::to_dsl_value(&payload)).expect("scene");
    assert!(scene.shape_model.is_some(), "a real imported object must mint a shape-model child");
}

#[semio_framework_async_macros::async_test]
async fn import_cad_file_action_accepts_spatial_json_text_string_payload() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let file_text = json!({
        "schema": "spatial.model",
        "revision": 1,
        "modelDefinitionId": "spatial.shape",
        "objects": [{
            "id": "object-loaded",
            "label": "Loaded",
            "typology": "spatial.shape.primitive.box",
            "visible": true,
            "locked": false,
            "origin": [1.0, 2.0, 3.0],
            "primitives": []
        }]
    })
    .to_string();
    let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": file_text, "name": "cad.spatial.json" })));
    // 🌱️ Whole-document replace is not an in-history mutation (SEMANTIC-MUTATIONS-OVERHAUL
    // retired `SetSnapshot`) — a spatial JSON string payload now surfaces as a
    // `Effect::LoadDocument` carrying the replacement document's pack bytes.
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("importCadFile must emit a LoadDocument effect for a spatial JSON string payload") else {
        panic!("expected a LoadDocument effect");
    };
    let next = <CadSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(next.shape_model.is_some(), "a real imported object must mint a shape-model child");
}

#[semio_framework_async_macros::async_test]
async fn import_cad_file_action_imports_obj_by_extension() {
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `import_cad_object_by_extension`
    // now returns a `SemioModelElement` — composing it into the document needs the same
    // child-dispatch seam as `commands/🧱️object/component.rs` (see `import_cad_file::handle`'s
    // own doc comment). Documented no-op. 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
    // (26/08/14): auto-selecting the imported object is no longer reachable from `handle()`
    // either (selection is framework-owned) — this now only asserts the document-write gap.
    let app = CadPlayApp::default();
    let scene = default_document();
    let obj_text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
    let obj_data_url = format!("data:model/obj;base64,{}", base64_codec::base64_standard_encode(obj_text));
    let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })));
    assert!(emit.artifact_mutations.is_empty(), "importCadFile's document write is a documented no-op until the child-dispatch seam lands");
    assert!(emit.config_mutations.is_empty(), "importCadFile no longer touches config once selection moved to the framework");
}
//#endregion 🔖️Import
//#region 🔖️History
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_added_node_through_generic_helper() {
    // ⚠️ `AddObject` is a documented no-op pending the child-dispatch seam (see
    // `commands/🧱️object/component.rs`'s module doc) — this exercises the generic
    // `assert_undo_redo_round_trip` test context helper (distinct from `undo_redo_round_trips_added_node_through_wrapper`
    // below, which drives the manual add/undo/redo dance) against the real `AddNode` command.
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot").nodes.len();
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(&mut app, CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1).await;
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_added_node_through_wrapper() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("snapshot").nodes.len();
    app.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).await.expect("add node");
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
    let undo = app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before);
    app.handle_action("redo", None, &meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn coalesced_translate_drag_is_a_single_undo_step() {
    // ⚠️ Test-harness debt, not app behaviour: `artifact_app_laws::new_app` is registry-less, so
    // every dispatch below fails its tool-proof catalog before reaching the reducer. What the law
    // asserts is still correct for an id no pane owns: `translateSelection` resolves the addressed
    // objects out of their pane's materialization and emits nothing when there are none, so three
    // coalesced ticks against a stranger id leave the document exactly where it started. The
    // seam's real behaviour is proved against the demo document by
    // `translate_selection_moves_the_object_and_the_rendered_instance`.
    let mut app = new_app().await;
    let before = json::to_json_string(&app.snapshot().expect("snapshot"));
    for _ in 0..3 {
        app.dispatch_typed(CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec!["object-box-1".into()], dx: 1.0, dy: 0.0, dz: 0.0 }), &meta("local")).await.expect("translate tick");
    }
    let after = json::to_json_string(&app.snapshot().expect("snapshot"));
    assert_eq!(before, after, "an id no pane materializes has nothing to move");
}
//#endregion 🔖️History
//#region 🔖️Convergence
/// 🧪️ The definitional merge proof: two instances start from the SAME base projection, apply
/// DISJOINT edits (A translates object A, B patches object B's label), and after exchanging operations
/// over a `MemoryBackbone` both converge to contain BOTH edits — impossible under whole-document
/// `setDocument` snapshots.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    // 🪆️ `PatchObject` is real again (2026-09-16), but it re-mints the addressed pane's composed
    // model child from that child's in-process materialization — which `default_document()` has
    // none of, and which no backbone frame carries either. `RenameNode` is a parent-document
    // mutation with no such dependency, so it is what this convergence law rides on.
    let mut base = default_document();
    base.nodes = vec![CadNode { id: "node-a".into(), label: "A".into(), kind: "solid".into() }, CadNode { id: "node-b".into(), label: "B".into(), kind: "solid".into() }];
    let node_a = base.nodes[0].id.clone();
    let node_b = base.nodes[1].id.clone();
    let base_envelope = store::create_document_envelope::<CadSnapshot, CadMutation>(CAD_DOCUMENT_SCHEMA, "cad-play", base, None);
    let base_files = store::print_document_pack(&base_envelope).await.expect("print document pack");

    let mut instance_a = new_app().await;
    let mut instance_b = new_app().await;
    instance_a.load_document_pack(&base_files).await.expect("load a");
    instance_b.load_document_pack(&base_files).await.expect("load b");
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://cad-convergence", "mem://cad-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    // A renames node A.
    instance_a.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_a.clone(), value: "Renamed By A".into() }), &meta("actor-a")).await.expect("a renames node a");

    // B renames node B — a disjoint edit that must survive alongside A's.
    instance_b.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_b.clone(), value: "Renamed By B".into() }), &meta("actor-b")).await.expect("b renames node b");

    // A neutral history command always pumps inbound operations before doing its own work.
    instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");

    let scene_a = instance_a.snapshot().expect("projection a");
    let scene_b = instance_b.snapshot().expect("projection b");

    let label_a_in_a = scene_a.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
    let label_a_in_b = scene_b.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
    let label_b_in_a = scene_a.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();
    let label_b_in_b = scene_b.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();

    assert_eq!(label_a_in_a, "Renamed By A", "instance A keeps its own edit");
    assert_eq!(label_a_in_b, "Renamed By A", "instance B converges on A's edit");
    assert_eq!(label_b_in_a, "Renamed By B", "instance A converges on B's edit");
    assert_eq!(label_b_in_b, "Renamed By B", "instance B keeps its own edit");
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_cad() {
    let mut sender = new_app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://cad-doc", "mem://cad-doc").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
    sender.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).await.expect("add node");

    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(operations);
        }
    }
    assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
    let operations = envelopes;

    let mut receiver = new_app().await;
    let nodes_before = receiver.snapshot().expect("snapshot").nodes.len();
    receiver.ingest_operations(&operations).await.expect("ingest once");
    receiver.ingest_operations(&operations).await.expect("ingest twice");
    assert_eq!(receiver.snapshot().expect("snapshot").nodes.len(), nodes_before + 1, "feeding the same operation twice must not double-apply");
}
//#endregion 🔖️Convergence


//#region 🪆️ObjectMutationSeam
/// 🪆️ The pane's live objects, read back through the exact accessor the world scene and the document
/// tree use — a mutation that does not land here has not re-materialized anything.
fn shape_objects(document: &CadSnapshot) -> Vec<crate::standards::v1::subsets::any::io::geometry_import::CadObject> {
    cad_pane_objects(document, CadPaneId::Shape)
}

/// 🥽️ The `instances` lane one pane's world-3d surface publishes — the rendered transform.
fn shape_instances_lane(document: &CadSnapshot) -> String {
    let envelope = view(document.clone(), CadPlayRuntime::default());
    let node = edit::build_world_scene_for_pane(&envelope, CadPaneId::Shape, "cad.play.scene3d/shape", None, CadDislocateOptions::default()).expect("shape world scene");
    scene_lane(&node, "instances")
}

/// ▶️ Law (a): `translateSelection` on the demo document changes the selected object's transform in
/// the composed child AND the rendered instance transform after re-materialization. This is the
/// gumball-snaps-back defect the 2026-09-16 CAD end-to-end report left open.
#[semio_framework_async_macros::async_test]
async fn translate_selection_moves_the_object_and_the_rendered_instance() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let before = shape_objects(&scene);
    let target = before.first().expect("the demo document materializes shape objects").clone();
    let emit = drive(&app, &scene, "translateSelection", Some(json!({ "objectIds": [target.id.clone()], "dx": 1.5, "dy": -2.25, "dz": 0.5 })));
    assert!(!emit.artifact_mutations.is_empty(), "translateSelection must emit a real document operation");

    let after = apply_mutations(&scene, &emit.artifact_mutations);
    let moved = shape_objects(&after).into_iter().find(|object| object.id == target.id).expect("the object survives the move");
    assert_eq!(moved.origin, [target.origin[0] + 1.5, target.origin[1] - 2.25, target.origin[2] + 0.5], "the composed child carries the new origin");
    assert_ne!(after.shape_model.as_ref().map(|child| child.child_id.clone()), scene.shape_model.as_ref().map(|child| child.child_id.clone()), "the pane's child handle is re-minted");

    let lane = shape_instances_lane(&after);
    let position = format!("\"position\":[{},{},{}]", moved.origin[0], moved.origin[1], moved.origin[2]);
    assert!(lane.contains(&position), "the rendered instance must carry the moved pose: looked for {position} in {lane}");
}

/// ▶️ Law (b): `addObject` yields a new rendered instance in the addressed pane.
#[semio_framework_async_macros::async_test]
async fn add_object_yields_a_new_rendered_instance() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let before = shape_objects(&scene).len();
    let emit = drive_in_window(&app, &scene, "addObject", Some(json!({ "typology": "spatial.shape.primitive.box" })), &CadConfig::default(), "cad-window-shape", shape::WINDOW_KIND_ID).expect("addObject handled");
    assert_eq!(emit.artifact_mutations.len(), 1, "addObject is one bounded parent op");

    let after = apply_mutations(&scene, &emit.artifact_mutations);
    let objects = shape_objects(&after);
    assert_eq!(objects.len(), before + 1, "the pane materializes one more object");
    let created = objects.last().expect("the created object is appended");
    assert!(shape_instances_lane(&after).contains(created.id.as_str()), "the created object is a rendered instance");
}

/// ↩️ Law (c): undo restores. Every object gesture's ops are ordinary in-history `CadMutation`s, so
/// the existing history mechanism inverts them with nothing app-specific — applied newest-first, the
/// inverses bring the demo document back exactly.
#[semio_framework_async_macros::async_test]
async fn object_mutations_invert_back_to_the_demo_document() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let target = shape_objects(&scene).first().expect("shape objects").id.clone();
    let gestures = [
        drive(&app, &scene, "translateSelection", Some(json!({ "objectIds": [target.clone()], "dx": 2.0, "dy": 0.0, "dz": 0.0 }))),
        drive(&app, &scene, "scaleSelection", Some(json!({ "objectIds": [target.clone()], "sx": 2.0, "sy": 2.0, "sz": 2.0 }))),
        drive(&app, &scene, "deleteObject", Some(json!({ "objectId": target.clone() }))),
    ];
    for emit in gestures {
        assert!(!emit.artifact_mutations.is_empty(), "every object gesture emits a real operation");
        let mut forward = scene.clone();
        let mut inverses: Vec<CadMutation> = Vec::new();
        for mutation in &emit.artifact_mutations {
            inverses.extend(protocol::Mutation::inverse(mutation, &forward));
            forward = protocol::MutationDiff::apply(protocol::Mutation::diff(mutation, &forward).diff(), &forward).expect("gesture applies");
        }
        assert_ne!(forward, scene, "the gesture moved the document");
        inverses.reverse();
        let restored = apply_mutations(&forward, &inverses);
        assert_eq!(restored, scene, "undo must restore the demo document");
        assert_eq!(shape_objects(&restored), shape_objects(&scene), "undo must restore the materialized objects too");
    }
}
//#endregion 🪆️ObjectMutationSeam

//#region 🧩️Contributions
/// 🧩️ Law (e): a contributed `cad.computer` pack is accepted by `setContributions` — the payload
/// survives onto the config lane AND the app's own reader decodes it into a real pack. The four
/// `cad-extension-*` plugins wired into the demonstrator closure on 2026-09-16 push exactly this shape.
#[semio_framework_async_macros::async_test]
async fn a_contributed_cad_computer_pack_is_accepted_by_set_contributions() {
    let contributions = json::to_json_string(&protocol::DslValue::Array(vec![
        protocol::DslValue::object([
            ("pluginId".to_string(), protocol::DslValue::String("cad-extension-aec-building".into())),
            (
                "topicContribution".to_string(),
                protocol::DslValue::object([
                    ("topic".to_string(), protocol::DslValue::String("cad.computer".into())),
                    (
                        "payload".to_string(),
                        protocol::DslValue::object([
                            ("appId".to_string(), protocol::DslValue::String("cad-play".into())),
                            ("moduleId".to_string(), protocol::DslValue::String("aec.building".into())),
                            ("computersJson".to_string(), protocol::DslValue::String("[{\"id\":\"aec.building.storey-count\"}]".into())),
                        ]),
                    ),
                ]),
            ),
        ]),
        // 🧩️ A second entry on another topic: a mixed host payload must still install cad's own share.
        protocol::DslValue::object([
            ("pluginId".to_string(), protocol::DslValue::String("flow".into())),
            (
                "topicContribution".to_string(),
                protocol::DslValue::object([("topic".to_string(), protocol::DslValue::String("flow.extension".into())), ("payload".to_string(), protocol::DslValue::object([]))]),
            ),
        ]),
    ]));

    let accepted = crate::standards::v1::subsets::any::schema::inferences::validate_cad_computer_contributions(&contributions);
    assert_eq!(accepted.len(), 1, "exactly the cad.computer pack addressed to cad-play is accepted: {accepted:?}");
    assert_eq!(accepted[0].module_id, "aec.building");
    assert!(accepted[0].computers_json.contains("aec.building.storey-count"), "the contributor's own computer roster survives verbatim");

    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let emit = drive(&app, &scene, "setContributions", Some(json!({ "json": contributions.clone() })));
    let config = config_after(&emit, &CadConfig::default());
    assert_eq!(config.contributions_json, contributions, "setContributions stores the pack on the config lane");
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::validate_cad_computer_contributions(&config.contributions_json).len(), 1, "the stored payload is still an accepted pack");
}

/// 🧩️ The REAL host pack the four `cad-extension-*` plugins push — a Rust mirror of
/// `shippedCadComputerContributionsJson` (`✏️editor/⚙️engine/🏃️runtime/🟦️.ts:51`), module ids and
/// `computersJson` rosters verbatim.
fn shipped_cad_computer_contributions() -> String {
    let entry = |plugin_id: &str, module_id: &str, computers_json: &str| {
        protocol::DslValue::object([
            ("pluginId".to_string(), protocol::DslValue::String(plugin_id.into())),
            (
                "topicContribution".to_string(),
                protocol::DslValue::object([
                    ("topic".to_string(), protocol::DslValue::String("cad.computer".into())),
                    (
                        "payload".to_string(),
                        protocol::DslValue::object([
                            ("appId".to_string(), protocol::DslValue::String("cad-play".into())),
                            ("moduleId".to_string(), protocol::DslValue::String(module_id.into())),
                            ("computersJson".to_string(), protocol::DslValue::String(computers_json.into())),
                        ]),
                    ),
                ]),
            ),
        ])
    };
    json::to_json_string(&protocol::DslValue::Array(vec![
        entry("cad-extension-spatial-shape", "spatial-shape", "{\"modelDefinitionIds\":[\"spatial.shape\"],\"statComputers\":[\"spatial.shape.geometry\"],\"propertyComputers\":[\"spatial.shape.volume\"],\"importProfiles\":[],\"transformationAppliers\":[]}"),
        entry("cad-extension-aec-building", "aec-building", "{\"modelDefinitionIds\":[\"aec.building\"],\"statComputers\":[],\"propertyComputers\":[],\"importProfiles\":[{\"modelDefinitionId\":\"aec.building\",\"layerTypology\":{},\"fallbackTypology\":\"building.building.slab\"}],\"transformationAppliers\":[]}"),
        entry("cad-extension-aec-building-energy", "aec-building-energy", "{\"modelDefinitionIds\":[\"aec.building.energy\"],\"statComputers\":[\"energy.demand\"],\"propertyComputers\":[\"energy.heatedvolume\"],\"importProfiles\":[],\"transformationAppliers\":[]}"),
        entry("cad-extension-aec-building-structure", "aec-building-structure", "{\"modelDefinitionIds\":[\"aec.building.structure\"],\"statComputers\":[\"structure.stability\"],\"propertyComputers\":[],\"importProfiles\":[],\"transformationAppliers\":[\"aec.building.structure/from_building\"]}"),
    ]))
}

/// ⚖️ LAW: the REAL four-extension `cad.computer` pack the demonstrator's koordinator pane receives
/// is ADMITTED by the retained config envelope, lands on the config lane through the production
/// `setContributions` dispatch, and decodes back into all four computer modules. Unlike sourcing's
/// 96-byte filter-text envelope, cad already prices `SetContributions` on its own
/// `CAD_CONFIG_STORE_MAXIMUM_BYTES` lane — this law pins that the real pack fits it, and that the
/// `CAD_CONFIG_STORE_MAXIMUM_BYTES` lane — this law pins that the real pack fits it.
///
/// 🏁️ The one-page (4 KiB) close budget this law used to pin as well was the framework's retained-
/// config close livelock, fixed 2026-09-16 in `close_retained_fields_step`
/// (`📓️fix-2026-09-16-kernel-capability-contributions-and-close-cliff.md` §2); a 65 536-byte retained
/// config now reaches terminal-empty, so the declared lane is what bounds this pack and nothing else.
#[semio_framework_async_macros::async_test]
async fn the_shipped_cad_computer_pack_is_admitted_by_the_retained_config_envelope() {
    let contributions = shipped_cad_computer_contributions();
    assert!(contributions.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES, "the real pack is {} bytes against a {}-byte config lane", contributions.len(), CAD_CONFIG_STORE_MAXIMUM_BYTES);

    let mutation = CadConfigMutation::SetContributions { json: contributions.clone() };
    assert!(admit_cad_config_mutation(&mutation).is_ok(), "the retained config store must admit the real host pack");
    assert!(prepare_cad_config(&CadConfig::default(), mutation).is_ok(), "the whole preparation must admit the real host pack");
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let emit = drive(&app, &scene, "setContributions", Some(json!({ "json": contributions.clone() })));
    let config = config_after(&emit, &CadConfig::default());
    assert_eq!(config.contributions_json, contributions, "the real pack survives verbatim onto the config lane");
    let accepted = crate::standards::v1::subsets::any::schema::inferences::validate_cad_computer_contributions(&config.contributions_json);
    assert_eq!(
        accepted.iter().map(|pack| pack.module_id.clone()).collect::<Vec<_>>(),
        vec!["spatial-shape".to_string(), "aec-building".to_string(), "aec-building-energy".to_string(), "aec-building-structure".to_string()],
        "every cad-extension-* module installs, in host order"
    );
    assert!(accepted.iter().all(|pack| pack.computers_json.contains("modelDefinitionIds")), "each module's own computer roster survives verbatim");
    // 🖼️ `render_body` runs `validate_cad_computer_contributions` on every body assembly
    // (`✏️editor/🦀️.rs:1278`), so a pack that made the panel refuse would surface here.
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    assert!(render_direct(&app, shape::BODY_KEY, &doc, &config, &ViewModel::default()).is_ok(), "the shape panel still assembles with the whole pack installed");
}

/// 🧩️ The pack the DEMONSTRATOR's koordinator pane actually receives: the demonstrator consumes
/// `cad.computer`, `process.machines` and `sourcing.module`, so the host hands this app all three
/// topics in ONE crossing and the guest ignores what is not addressed to it.
fn demonstrator_contributions_pack() -> String {
    let mut entries: Vec<protocol::DslValue> = serde_json::from_str::<Vec<serde_json::Value>>(&shipped_cad_computer_contributions())
        .expect("the shipped cad pack parses")
        .into_iter()
        .map(|value| protocol::json::from_json_str::<protocol::DslValue>(&value.to_string()).expect("entry"))
        .collect();
    for (plugin_id, topic, app_id, bulk) in [
        ("process-extension-wood", "process.machines", "process3d-play", 4_705usize),
        ("process-extension-metal", "process.machines", "process3d-play", 4_011),
        ("process-extension-robotic", "process.machines", "process3d-play", 4_010),
        ("process-extension-concrete", "process.machines", "process3d-play", 3_311),
        ("sourcing-module-beams", "sourcing.module", "sourcing-curation", 780),
        ("sourcing-module-windows", "sourcing.module", "sourcing-curation", 669),
        ("sourcing-module-slabs", "sourcing.module", "sourcing-curation", 589),
    ] {
        entries.push(protocol::DslValue::object([
            ("pluginId".to_string(), protocol::DslValue::String(plugin_id.into())),
            (
                "topicContribution".to_string(),
                protocol::DslValue::object([
                    ("topic".to_string(), protocol::DslValue::String(topic.into())),
                    (
                        "payload".to_string(),
                        protocol::DslValue::object([
                            ("appId".to_string(), protocol::DslValue::String(app_id.into())),
                            ("moduleId".to_string(), protocol::DslValue::String(plugin_id.into())),
                            // 📐️ The REAL bulk string of that extension, measured off the built dev manifests 2026-09-16.
                            ("bulkJson".to_string(), protocol::DslValue::String("m".repeat(bulk))),
                        ]),
                    ),
                ]),
            ),
        ]));
    }
    json::to_json_string(&protocol::DslValue::Array(entries))
}

/// ⚖️ LAW: the REAL demonstrator pack crosses this app's registered `setContributions` admission and
/// is retained whole.
///
/// 🏁️ Before the per-app admission, cad priced `setContributions` on the 8 KiB gesture envelope every
/// retained tool shares, and the live push died with `typed command raw JSON exceeds its registered
/// retained-page admission` the moment the host stopped cutting capability packs to `[]`
/// (2026-09-16, ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS).
#[semio_framework_async_macros::async_test]
async fn the_real_demonstrator_pack_is_admitted_by_the_registered_contributions_wire() {
    let pack = demonstrator_contributions_pack();
    let wire = json::to_json_string(&("setContributions", protocol::DslValue::object([("json".to_string(), protocol::DslValue::String(pack.clone()))])));
    println!("[STATS] cad demonstrator pack packChars={} wireChars={}", pack.len(), wire.len());
    assert!(pack.len() > CAD_RETAINED_RAW_BYTES, "the real pack is past the gesture envelope — that is why the app declares its own contributions wire");
    assert!(wire.len() <= semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, "the real pack's command wire ({} B) must fit the registered admission", wire.len());
    assert!(pack.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES, "the real pack ({} B) must fit the retained config lane", pack.len());
    let mutation = CadConfigMutation::SetContributions { json: pack.clone() };
    assert!(admit_cad_config_mutation(&mutation).is_ok());
    assert!(prepare_cad_config(&CadConfig::default(), mutation).is_ok());
    let accepted = crate::standards::v1::subsets::any::schema::inferences::validate_cad_computer_contributions(&pack);
    assert_eq!(accepted.len(), 4, "only the four cad.computer modules install; the foreign topics are ignored, never refused");
}
//#endregion 🧩️Contributions

//#region 🔖️EngagementSubmit
/// ⏎️ The shell's Enter/Space on an empty action line during a session is the state's `confirm`:
/// box's `first_corner_height` accepts the typed height, which reaches the `ready` commit state.
#[semio_framework_async_macros::async_test]
async fn empty_submit_during_a_session_fires_the_state_confirm_and_commits() {
    let document = empty_cad_snapshot();
    let mut runtime = CadPlayRuntime::default();
    assert!(start_interaction_session(&mut runtime, CadPaneId::Shape, "primitive.box"));
    {
        let session = runtime.engagement_session.as_mut().expect("session");
        assert!(apply_event(session, "pointer.down", Some(&protocol::DslValue::Array(vec![protocol::DslValue::float(0.0), protocol::DslValue::float(0.0), protocol::DslValue::float(0.0)]))));
        assert!(apply_event(session, "pointer.down", Some(&protocol::DslValue::Array(vec![protocol::DslValue::float(2.0), protocol::DslValue::float(3.0), protocol::DslValue::float(0.0)]))));
        assert_eq!(session.state, "first_corner_height");
    }
    runtime.engagement_input = "2".into();
    assert!(engagement_submit_mutations(&document, &mut runtime, CadPaneId::Shape).is_empty(), "the typed height is a scalar entry, not a commit");
    assert_eq!(runtime.engagement_session.as_ref().map(|session| session.state.as_str()), Some("first_corner_height"));
    assert!(runtime.engagement_input.is_empty(), "a consumed scalar entry clears the published line");
    // 🏁️ `ready` is the spec's `commit.fromStates` entry, so accepting the height commits at once.
    let ops = engagement_submit_mutations(&document, &mut runtime, CadPaneId::Shape);
    assert_eq!(ops.len(), 1, "accepting the height reaches `ready` and commits exactly one box: {ops:?}");
    assert!(matches!(ops[0], CadMutation::CreateObject(_)));
    assert!(runtime.engagement_session.is_none(), "the committed session is closed");
    assert_eq!(runtime.engagement_step, "Committed 1 object(s)");
    // 🛑️ Without a session the empty line stays the idle no-op.
    assert!(engagement_submit_mutations(&document, &mut runtime, CadPaneId::Shape).is_empty());
    assert_eq!(runtime.engagement_step, "Idle");
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️EngagementCoalescing
/// 🧵️ One engagement is one history item: every non-committing step (keystroke, start, pointer
/// move, pick, abort) amends under `CAD_ENGAGEMENT_COALESCE_KEY`; the committing pick is a described
/// document edit with the objects on the artifact lane and no coalesce key.
#[semio_framework_async_macros::async_test]
async fn engagement_steps_coalesce_into_one_history_item_until_the_commit() {
    use crate::editor::cad::commands::engagement::CAD_ENGAGEMENT_COALESCE_KEY;
    let app = CadPlayApp::default();
    let scene = empty_cad_snapshot();
    let mut config = CadConfig::default();
    let amended = |emit: &Emit<CadMutation, CadConfigMutation>, what: &str| {
        assert_eq!(emit.coalesce_key.as_deref(), Some(CAD_ENGAGEMENT_COALESCE_KEY), "{what} amends the running engagement item");
        assert!(emit.artifact_mutations.is_empty(), "{what} touches no document");
        assert_eq!(emit.config_mutations.len(), 1, "{what} republishes the session once");
    };
    for (index, character) in ["B", "Bo", "Box"].iter().enumerate() {
        let emit = drive_with_config(&app, &scene, "engagementInput", Some(json!({ "pane": "shape", "value": character })), &config);
        amended(&emit, &format!("keystroke {index}"));
        config = config_after(&emit, &config);
    }
    let emit = drive_with_config(&app, &scene, "engagementPossibleSelect", Some(json!({ "pane": "shape", "possibleId": "primitive.box" })), &config);
    amended(&emit, "starting the interaction");
    config = config_after(&emit, &config);
    let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "position": [0.5, 0.5, 0.0] })), &config);
    amended(&emit, "a pointer move");
    config = config_after(&emit, &config);
    for point in [[0.0, 0.0, 0.0], [2.0, 3.0, 0.0]] {
        let emit = drive_with_config(&app, &scene, "worldPointerDown", Some(json!({ "pane": "shape", "position": point })), &config);
        amended(&emit, "a non-committing pick");
        config = config_after(&emit, &config);
    }
    let emit = drive_with_config(&app, &scene, "engagementInput", Some(json!({ "pane": "shape", "value": "2" })), &config);
    amended(&emit, "the height keystroke");
    config = config_after(&emit, &config);
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    amended(&emit, "applying the typed height");
    config = config_after(&emit, &config);
    // ⏎️ The empty line is `confirm`: `ready` is the box's commit state.
    let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
    assert_eq!(emit.coalesce_key, None, "the commit is its own described document edit");
    assert_eq!(emit.artifact_mutations.len(), 1, "one box lands: {:?}", emit.artifact_mutations);
    assert!(matches!(emit.artifact_mutations[0], CadMutation::CreateObject(_)));
    let runtime = runtime_after(&emit, &config);
    assert!(runtime.engagement_session.is_none());
    assert_eq!(runtime.engagement_step, "Committed 1 object(s)");
}
//#endregion 🔖️EngagementCoalescing

//#region 🔖️InteractionScope
/// 🎯️ A hover on the `cad` domain repaints the four world bodies and nothing else — never the
/// framework's `Full` fallback (every window body, every panel, the rails) on pointer motion; a
/// selection adds the Inspection and Artifact panels plus the HUD count; a verb on a foreign domain
/// is handed back to the framework.
#[semio_framework_async_macros::async_test]
async fn cad_interaction_scope_keeps_hover_to_the_world_bodies() {
    use semio_framework::kernel::UiDirtyScope;
    use semio_framework_plugin::InteractionVerb;
    let Some(UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels }) = cad_interaction_scope(InteractionVerb::Hover, &[CAD_INTERACTION_DOMAIN]) else {
        panic!("hover answers a partial scope");
    };
    assert_eq!(window_bodies, CAD_WORLD_BODY_KEYS.map(str::to_string).to_vec());
    assert!(panel_bodies.is_empty() && !utilities && !tools && !engagements && !measures && !labels);
    let Some(UiDirtyScope::Partial { panel_bodies, engagements, .. }) = cad_interaction_scope(InteractionVerb::Select, &[CAD_INTERACTION_DOMAIN]) else {
        panic!("select answers a partial scope");
    };
    assert_eq!(panel_bodies, vec![inspection::CAD_PLAY_BODY_PROPERTIES.to_string(), document::CAD_PLAY_BODY_ARTIFACT.to_string()]);
    assert!(engagements);
    assert_eq!(cad_interaction_scope(InteractionVerb::SetGranularity, &[CAD_INTERACTION_DOMAIN]), Some(UiDirtyScope::None));
    assert_eq!(cad_interaction_scope(InteractionVerb::Hover, &["vortex"]), None);
    assert_eq!(cad_interaction_scope(InteractionVerb::Hover, &[]), None);
}
//#endregion 🔖️InteractionScope

//#region 🔖️RenderCostProbe
/// 🩺️ Where a warm window render spends its time (a probe, printed with `--nocapture`; the only
/// assertion is that a warm render is cheaper than a cold one).
#[semio_framework_async_macros::async_test]
async fn world_scene_render_cost_probe() {
    let view = forest_view();
    let options = CadDislocateOptions::default();
    let cold = std::time::Instant::now();
    let _ = edit::build_world_scene_for_pane(&view, CadPaneId::Building, "cad.play.scene3d/building", None, options).expect("scene");
    let cold = cold.elapsed();
    let warm = std::time::Instant::now();
    for _ in 0..10 {
        let _ = edit::build_world_scene_for_pane(&view, CadPaneId::Building, "cad.play.scene3d/building", None, options).expect("scene");
    }
    let warm = warm.elapsed() / 10;
    let scene = crate::cad_pane_local_scene(&view.document, CadPaneId::Building).expect("scene");
    let (objects, geometry) = edit::cad_pane_working_objects(&scene, CadPaneId::Building);
    let lane = std::time::Instant::now();
    let meshes = edit::world_meshes_json_cached(CadPaneId::Building, Some(&scene), objects, geometry);
    let lane = lane.elapsed();
    let carrier = std::time::Instant::now();
    let _ = semio_framework_plugin::paged_text_carrier("meshes", &meshes);
    let carrier = carrier.elapsed();
    eprintln!("[DEBUG] world scene render: cold {cold:?} warm {warm:?} (meshes lane {} bytes: cached lookup {lane:?}, paged carrier {carrier:?})", meshes.len());
    assert!(warm < cold);
}
//#endregion 🔖️RenderCostProbe
