//! 🧪️ The one cad-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
//! instead of re-deriving a store/dispatch/render scaffold of its own.

use semio_s_artifact_cad_cad_editor::*;
use semio_s_artifact_cad_cad_commands::{camera::set_camera, camera::set_projection, camera::set_projection_param};
use semio_s_artifact_cad_cad_commands::contribution::set_contributions;
use semio_s_artifact_cad_cad_commands::{engagement::engagement_abort, engagement::engagement_input, engagement::engagement_possible_select, engagement::engagement_repeat_last, engagement::engagement_submit, engagement::world_pointer_down, engagement::world_pointer_move};
use semio_s_artifact_cad_cad_commands::{io::import_cad_file, io::load_raw_request, io::save_current, io::save_in_play, io::save_selected};
use semio_s_artifact_cad_cad_commands::model_definition::set_active_example;
use semio_s_artifact_cad_cad_commands::{node::add_node, node::rename_node, node::set_node_selection};
use semio_s_artifact_cad_cad_commands::{object::add_object, object::delete_object, object::duplicate_object, object::patch_object, object::patch_selection};
use semio_s_artifact_cad_cad_commands::{reference::patch_cad_play_reference, reference::reference_hover, reference::set_reference_hidden, reference::set_reference_locked, reference::set_reference_selection};
use semio_s_artifact_cad_cad_commands::{sun::set_sun_azimuth, sun::set_sun_elevation, sun::set_sun_intensity, sun::toggle_sun};
use semio_s_artifact_cad_cad_commands::{transform::apply_transformation, transform::rotate_selection, transform::scale_selection, transform::translate_selection};
use semio_s_artifact_cad_cad_commands::utility::set_dislocate_option;
use semio_s_artifact_cad_cad_config::{cad_sun_config_to_world, CadConfig, CadConfigMutation, CadDislocateOptions, SetEdit, SetContributionsEdit};
use semio_s_artifact_cad_cad_interaction::{self, apply_event, can_commit, keyed_transitions, resolve_interaction_key, start_session, CadEngagementScratch};
use semio_s_artifact_cad_cad_modes as edit;
use semio_s_artifact_cad_cad_transform::{cad_import_object_emit, CadPaneModels, CadToolEntry, CadToolLeaf, CadTransformRecord};
use semio_s_artifact_cad_cad_modes::window_transient;
use semio_s_artifact_cad_cad_transient::CadWorldWindowTransient;
use semio_s_artifact_cad_cad_modes::{windows::building, windows::energy, windows::shape, windows::structure_classic};
use semio_s_artifact_cad_cad_panels::{catalogue, document, inspection};
use semio_s_artifact_cad_cad_kit::{editor::cad::terminology::cad_is_de_locale, editor::cad::terminology::cad_labels};
use semio_s_artifact_cad_cad_mutations::CadMutation;
use semio_s_artifact_cad_cad_model::{geometry::cad_object_from_model_element, geometry::model_element_from_cad_object, geometry::objects_from_model_snapshot, geometry::CadObject, geometry::CAD_OBJECT_PSET};
use semio_s_artifact_cad_cad_io::{export_solids_as, CadSolidExport, CAD_SOLID_EXPORT_DIALECT_STEP};
use semio_s_artifact_cad_cad_inferences::{cad_brep_kernel, cad_camera_projection_config, ensure_object_solid_handle, forest_play_scene, next_cad_id, CAD_EXAMPLE_FOREST_LEFT, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC};
use semio_s_artifact_cad_cad_model::{snapshot::artifact_kind, snapshot::CadCamera, snapshot::CadPaneId, snapshot::CadSnapshot, snapshot::CadWorkingScene, snapshot::CAD_DOCUMENT_SCHEMA};
use semio_s_artifact_cad_cad_play::CadComposedPanes;
use {semio_s_artifact_stdio_semio_model::schema::mutations::remove_element::RemoveElement, semio_s_artifact_stdio_semio_model::schema::mutations::set_element::SetElement, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation};
use semio_s_artifact_stdio_semio_model::schema::mutations::insert_element::InsertElement;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelElement;
use semio_framework::kernel::Effect;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::tree_item as framework_tree_item;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::world3d_camera_projection_json;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ChildContentView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::Menu;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::WorldSunConfig;
use semio_framework_3d_brep_engine::{Brep, GeometryHandle};
use semio_framework_plugin::app::{ArtifactEditor, Dialect, Editor};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_pack_json::Value;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use protocol::{ColdMutation, Mutation};
use semio_framework_plugin::{ActionMeta, HistoryView, PluginApp, UiMenuRef, VcsArtifactApp};

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
///
/// 🪪️ MOUNTED on [`TEST_INSTANCE`] — the same id [`meta`] stamps on every `ActionMeta`. An unbound
/// app refuses every typed command with `interactive-job.live-instance` ("does not belong to the
/// mounted live app instance"), which is what the host would do to a command for a dead window.
/// Every caller must therefore settle each dispatch ([`settle`]) and [`close`] the app before it
/// drops, exactly as the host's own close loop does.
pub async fn new_app() -> CadFixtureApp {
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members::<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(cad_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(TEST_INSTANCE).await;
    CadFixtureApp(app, scope)
}

/// 🪪️ The one live instance id this crate's mounted fixtures run on.
pub const TEST_INSTANCE: u32 = 1;

/// 📐️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the app's paid close.
pub struct CadFixtureApp(pub VcsArtifactApp<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

impl std::ops::Deref for CadFixtureApp {
    type Target = VcsArtifactApp<EditorApp<CadPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CadFixtureApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 🔁️ Drives the host's bounded continuation + ACK protocol to quiescence after a dispatch.
pub async fn settle(app: &mut CadFixtureApp) {
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("cad fixture publication: {fault:?}"));
}

/// 🔚️ Runs the exact terminal-empty close loop; a fixture store that drops without it panics with
/// `artifact store reached Drop without its exact terminal-empty shallow-shell witness`.
pub fn close(app: &mut CadFixtureApp) {
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 🪟️ The four cad pane windows as `ShellHost` mounts them. `window_engagements`/`window_measures`
/// project ONLY the window instances the view carries, so a `ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)` yields an empty
/// map however many window kinds the app declares.
pub fn four_pane_view() -> ViewModel {
    ViewModel {
        window_instances: CAD_PANE_WINDOW_KIND_IDS.iter().map(|kind| semio_framework_plugin::ViewWindowInstance { id: (*kind).to_string(), window_kind_id: (*kind).to_string() }).collect(),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

/// 🪟️ The four world-window kinds, coarsest pane first.
pub const CAD_PANE_WINDOW_KIND_IDS: [&str; 4] = [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID];

/// ✏️ Adapts `create_cad_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `context::assert_declared_actions_bridge_to_commands` still expects —
/// framework test context gap, not modifiable here (`🧰️framework/**` is outside this packet's lease).
pub fn cad_app_manifest_for_tests() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_cad_app(), examples: Vec::new() }
}

semio_framework_plugin::history_edit_acceptance_law!("cad", CadPlayApp, cad_app_manifest_for_tests, "../..");
semio_framework_plugin::composed_reload_law!("cad", CadPlayApp, cad_app_manifest_for_tests, "../..");
semio_framework_plugin::composed_child_history_law!("cad", CadPlayApp, cad_app_manifest_for_tests, [("translateSelection", r#"{"objectIds":["object-hexagonal-cut-concrete-forest-left"],"dx":1.5,"dy":0.0,"dz":0.0}"#)]);

pub fn empty_history() -> HistoryView {
    HistoryView::empty()
}

/// 🔀️ Keeps the legacy test-harness call shape while exercising the production action bridge —
/// `cad_command_from_action` speaks `DslValue`, so this bridges the `pack::json::Value`-shaped
/// test-harness `args` via `protocol::json::to_dsl_value` right at the call site.
pub fn command_from_action(action: &str, args: Option<&Value>) -> CadCommand {
    cad_command_from_action(action, args.map(semio_framework_pack_json::to_dsl_value).as_ref()).unwrap_or_else(|error| panic!("command_from_action: {error:?}"))
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
    drive_result(app, scene, action, args, config).expect("cad command handled")
}

/// 🪪️ Production-dispatch harness answering the command's own refusal.
#[allow(clippy::needless_pass_by_value)]
pub fn drive_result(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
    drive_engagement(app, scene, action, args, config, &CadWorldWindowTransient::default()).map(|(emit, _)| emit)
}

/// 🫧️ One command over `config` in a world window whose engagement transient is `transient` (design §17.4); answers
/// the emit and the window's engagement state afterwards — unchanged when the command published none.
#[allow(clippy::needless_pass_by_value)]
pub fn drive_engagement(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig, transient: &CadWorldWindowTransient) -> Result<(Emit<CadMutation, CadConfigMutation>, CadWorldWindowTransient), Fault> {
    let _ = app;
    let history = empty_history();
    let doc = ArtifactView::new(scene, &history);
    let cfg = ConfigView { snapshot: config, window: None };
    let command = command_from_action(action, args.as_ref());
    let mut ctx = CadDispatchCtx::new(CadInteractionSnapshot::default(), None, transient.clone());
    let emit = command.dispatch(&doc, &cfg, &mut ctx)?;
    Ok((emit, ctx.next_window_transient.unwrap_or_else(|| transient.clone())))
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
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let mut ctx = CadDispatchCtx::new(CadInteractionSnapshot::default(), Some(view_state), CadWorldWindowTransient::default());
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
        next = protocol::apply_diff_cold(operation.diff(&next).diff(), &next).expect("valid mutation diff");
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
    cad_runtime_from(&config_after(emit, base), &CadWorldWindowTransient::default())
}

/// 🎛️ A render view of `scene` with no live child: every pane reads the genesis content its child id names.
pub fn view(scene: CadSnapshot, runtime: CadPlayRuntime) -> CadPlayView {
    CadPlayView::of(&scene, &ChildContentView::EMPTY, runtime, CadInteractionSnapshot::default())
}

/// 🕹️ [`view`] with a live `"cad"` interaction.
pub fn view_with_interaction(scene: CadSnapshot, runtime: CadPlayRuntime, interaction: CadInteractionSnapshot) -> CadPlayView {
    CadPlayView::of(&scene, &ChildContentView::EMPTY, runtime, interaction)
}

/// 🪆️ The render view of the mounted app's document composed with its live children.
pub fn live_view(app: &CadFixtureApp) -> CadPlayView {
    CadPlayView::of(&app.snapshot().expect("snapshot"), &app.test_child_content_view(), CadPlayRuntime::default(), CadInteractionSnapshot::default())
}

/// 🏛️ `pane`'s model as the mounted app's child store holds it now.
pub async fn pane_model(app: &CadFixtureApp, pane: CadPaneId) -> semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot {
    use store::{snapshot::ArtifactPack, SpaceMember, ReferenceSpaceMember};
    let parent = app.snapshot().expect("CAD parent snapshot");
    let child = semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&parent, pane).unwrap_or_else(|| panic!("{pane:?} pane composes a model child"));
    let member = app.child_store(semio_s_artifact_cad_cad_model::snapshot::cad_pane_model_slot(pane), &child.child_id).await.unwrap_or_else(|| panic!("{pane:?} pane model member is live"));
    semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot::decode_pack(&member.document_pack_bytes().await.expect("pane model pack")).expect("pane model snapshot")
}

/// 🧱️ `pane`'s objects as the mounted app's child store holds them now.
pub async fn pane_objects(app: &CadFixtureApp, pane: CadPaneId) -> Vec<CadObject> {
    objects_from_model_snapshot(&pane_model(app, pane).await)
}

/// 🧩️ Every history row that lists composed-member mutations, oldest first.
pub async fn member_rows(app: &mut CadFixtureApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    let mut rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.mutations.iter().any(|mutation| mutation.store.is_some())).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 🪟️ The action metadata of a command dispatched in the open world window `window_id` of kind `window_kind_id`.
pub fn window_meta(window_id: &str, window_kind_id: &str) -> ActionMeta {
    ActionMeta {
        view_state: Some(ViewModel {
            window_id: Some(window_id.into()),
            active_window_kind_id: Some(window_kind_id.into()),
            window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: window_id.into(), window_kind_id: window_kind_id.into() }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        }),
        ..meta("local")
    }
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
