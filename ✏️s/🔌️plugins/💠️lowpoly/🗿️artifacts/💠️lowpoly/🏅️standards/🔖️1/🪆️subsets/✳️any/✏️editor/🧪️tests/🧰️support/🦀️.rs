//! 🧰️ The one shared app test harness — every test target of the family mounts it as `context`.

use semio_s_artifact_lowpoly_lowpoly_editor::*;
use semio_s_artifact_lowpoly_lowpoly_commands::{paint::LowpolyPaintTick, paint::lowpoly_paint_tick};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::lowpoly_paint_drive;
use semio_s_artifact_lowpoly_lowpoly_mut_paint::{apply_paint_stroke::ApplyPaintStroke, edit_paint_layer::EditPaintLayer};
use semio_framework_tool_machine::GesturePhase;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText;
use semio_s_artifact_lowpoly_lowpoly_commands::{add_primitive, camera, chrome, engagement, document, media, mesh_edit, object, paint, patch_object, selection, sun, transform, utility, uv};
use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation, SetActiveObjectEdit, SetPaintUtilityEdit, SetUtilityParamsEdit, SetPaintColorEdit, SetEngagementInputEdit, SetSunEdit};
use semio_s_artifact_lowpoly_lowpoly_modes::{self, editor::lowpoly::modes::paint as paint_mode};
use semio_s_artifact_lowpoly_lowpoly_panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, layers as layers_panel};
use semio_s_artifact_lowpoly_lowpoly_view::{editor::lowpoly::session::LowpolyScratch, editor::lowpoly::session::LowpolyTransient, editor::lowpoly::session::LowpolyTransientMutation};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::LowpolyLabels;
use semio_s_artifact_lowpoly_lowpoly_view::{resolve_active_object_id, selection_from_interaction, selection_from_state, utility_param_f64, LowpolyView, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::artifact_kind, snapshot::LowpolyObject, snapshot::LowpolySnapshot, snapshot::LOWPOLY_DOCUMENT_SCHEMA};
use protocol::Mutation;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_ui_locale::LabelText;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
use semio_framework_plugin::WindowEngagementOption;
use semio_framework_plugin::WindowEngagementPossible;
use semio_framework_plugin::WindowEngagementStatus;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;
use camera::set_camera;
use chrome::toggle_show_edges;
use engagement::{engagement_input, engagement_submit};
use document::{replace_snapshot_json, set_snapshot_json};
use media::{export_mesh, import_mesh_file, load_mesh_request};
use object::{delete_selection, duplicate_object};
use mesh_edit::{bevel, decimate, dissolve, extrude, flip_faces, inset, loop_cut, merge, mirror, snap, subdivide, toggle_smooth, triangulate};
use paint::{add_paint_layer, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, fill_bucket, paint_at, paint_fill, paint_sample, paint_stroke};
use selection::{set_active_object, set_active_paint_layer};
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use transform::{rotate_selection, scale_selection, translate_selection};
use utility::set_utility_param;
use uv::{clear_seam, mark_uv_seam, unwrap_active};
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry, project_and_retire_fixture_tree};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

/// 🪪️ The one live instance every fixture app binds — a registry-backed app refuses typed
/// commands until it knows its instance (`interactive-job.live-instance`).
pub const INSTANCE: u32 = 1;

/// 🔁️ The settle loop's maintenance grant. `settle_registered_typed_operation` pages maintenance at
/// exactly one 4 KiB page; a lowpoly document carrying mesh content is larger than that, so its
/// close cursor never receives a grant it can act on and the operation never retires (measured
/// 2026-09-17: a 2 299-byte snapshot settles, a 4 240-byte one hangs).
const SETTLE_GRANT_BYTES: usize = 1 << 20;

/// 🧪️ A registry-backed, instance-bound app that closes its stores on drop (the store drop witness
/// panics otherwise). Derefs to the framework app, so every `PluginApp` call reads as before.
pub struct LowpolyApp(pub VcsArtifactApp<EditorApp<LowpolyPlayApp>>);

impl std::ops::Deref for LowpolyApp {
    type Target = VcsArtifactApp<EditorApp<LowpolyPlayApp>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LowpolyApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for LowpolyApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
        }
    }
}

/// 🧪️ `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` (framework test context,
/// unchanged for this ticket) still take `fn() -> App` — `create_lowpoly_app` now returns
/// `AppDefinition` (contract §2.4). This tiny local wrapper is the documented bridge (pilot report
/// `📓️w2-cad-report.md` recipe step 7), not a framework fix owed by this packet.
pub fn lowpoly_manifest_for_tests() -> App {
    App { definition: create_lowpoly_app(), examples: Vec::new() }
}

semio_framework_plugin::history_edit_acceptance_law!("lowpoly", LowpolyPlayApp, lowpoly_manifest_for_tests, "../..");

/// 🧪️ The registry-backed app every unit test builds on. It used to be the registry-less
/// `new_app`, which fails closed since lowpoly's 47 tools became `Migrated` with exact factories: an
/// empty registry has no migrated ids, so the tool-proof catalog rejects the first factory
/// (`interactive-job.catalog-authority`, 25 tests, ticket 26/08/29/LOWPOLY-END-TO-END, 2026-09-17).
pub async fn app() -> LowpolyApp {
    app_with_registry().await
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn app_with_registry() -> LowpolyApp {
    let mut app = new_app_with_registry::<EditorApp<LowpolyPlayApp>>(lowpoly_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(INSTANCE).await;
    LowpolyApp(app)
}

/// 🪟️ The Model window's view state — what the React shell stamps on every dispatch, and what the
/// window-scoped verbs (`setCamera`, the select toggles) key their config mutations off.
pub fn action_meta() -> semio_framework_plugin::ActionMeta {
    let id = edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN;
    let mut action = meta("local");
    action.view_state = Some(ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: id.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
    action
}

/// 🎯️ Dispatches one typed command through the retained route and settles its typed operation, so
/// the snapshot a test reads next is the committed one.
pub async fn dispatch(app: &mut LowpolyApp, command: LowpolyCommand) -> InvocationResult {
    let verb = command.command_id().to_string();
    let result = app.0.dispatch_typed(command, &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{verb} refused: {fault:?}"));
    settle(&mut app.0, &verb).await;
    result
}

/// 🕹️ Dispatches one string action exactly as the React shell does (`handle_action` →
/// `command_from_action` → retained job → settle). Framework-reserved verbs (`interactionSelect`,
/// `undo`, `redo`) return an admission receipt; their tool job only lands through the reserved settle.
pub async fn act(app: &mut LowpolyApp, action: &str, args: serde_json::Value) {
    let args = semio_framework_value::DslValue::from(&args);
    let result = app.0.handle_action(action, Some(&args), &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{action} refused: {fault:?}"));
    if matches!(action, "interactionSelect" | "interactionHover" | "undo" | "redo") {
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, result).await.unwrap_or_else(|fault| panic!("{action} did not settle its reserved job: {fault:?}"));
    }
    settle(&mut app.0, action).await;
}

/// 🚫️ Dispatches a command the app must REFUSE and returns the refusal's text — either the dispatch
/// itself faults, or the retained reducer publishes a fault page while the operation settles.
pub async fn dispatch_refused(app: &mut LowpolyApp, command: LowpolyCommand) -> String {
    let verb = command.command_id().to_string();
    if let Err(fault) = app.0.dispatch_typed(command, &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await {
        return fault.message;
    }
    settle_faults(&mut app.0, &verb).await.into_iter().next().unwrap_or_else(|| panic!("{verb} was admitted and settled without a refusal"))
}

/// 🔁️ `settle`, but a fault page is collected instead of asserted away.
async fn settle_faults(app: &mut VcsArtifactApp<EditorApp<LowpolyPlayApp>>, action: &str) -> Vec<String> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    let mut faults = Vec::new();
    while app.has_pending_typed_operations() {
        assert!(std::time::Instant::now() < deadline, "{action} did not settle");
        let _ = app.maintenance_step(1, SETTLE_GRANT_BYTES).unwrap_or_else(|fault| panic!("{action} maintenance: {fault:?}"));
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));
        while let Some(page) = app.take_typed_operation_result_page(INSTANCE) {
            if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                faults.push(String::from_utf8_lossy(page.bytes()).to_string());
            }
            assert!(app.acknowledge_typed_operation_result(page.token).unwrap_or(false), "{action} rejected its result ACK");
        }
        while app.take_typed_operation_effect().is_some() {}
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while app.take_typed_operation_completion().await.unwrap_or(None).is_some() {}
        while let Some(reply) = app.take_local_interaction_query_reply() {
            if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                assert!(app.acknowledge_local_interaction_query(&token), "{action} rejected its local-interaction ACK");
            }
        }
        std::thread::yield_now();
    }
    faults
}

/// 🔁️ The fixture's own settle loop, at a REAL host grant (see `SETTLE_GRANT_BYTES`). Returns the
/// number of result pages the operation published.
pub async fn settle(app: &mut VcsArtifactApp<EditorApp<LowpolyPlayApp>>, action: &str) -> usize {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    let mut lanes = 0;
    while app.has_pending_typed_operations() {
        assert!(std::time::Instant::now() < deadline, "{action} did not settle");
        let _ = app.maintenance_step(1, SETTLE_GRANT_BYTES).unwrap_or_else(|fault| panic!("{action} maintenance: {fault:?}"));
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));
        while let Some(page) = app.take_typed_operation_result_page(INSTANCE) {
            let lane = page.lane;
            let fault = (lane == semio_framework_plugin::app::TypedOperationResultLane::Fault).then(|| String::from_utf8_lossy(page.bytes()).to_string());
            assert!(app.acknowledge_typed_operation_result(page.token).unwrap_or(false), "{action} rejected its result ACK");
            assert!(fault.is_none(), "{action} publication fault: {}", fault.unwrap_or_default());
            lanes += 1;
        }
        while app.take_typed_operation_effect().is_some() {}
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while app.take_typed_operation_completion().await.unwrap_or(None).is_some() {}
        while let Some(reply) = app.take_local_interaction_query_reply() {
            if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                assert!(app.acknowledge_local_interaction_query(&token), "{action} rejected its local-interaction ACK");
            }
        }
        std::thread::yield_now();
    }
    lanes
}

/// 🧾️ How many document mutations the session command log has recorded — the retained route
/// commits through the typed operation, so an `InvocationResult` no longer carries the mutation
/// and the log is what a test counts.
pub async fn committed_edits(app: &mut LowpolyApp) -> usize {
    app.0.history_snapshot().await.expect("history snapshot").upserts.iter().filter(|entry| entry.kind == "mutation" && entry.applied).count()
}

/// 🖼️ Renders one body and projects it to JSON through the fixture observer, which also retires the
/// paged children a serde projection cannot carry (`BuiltChildren requires retained page transport`).
pub async fn render(app: &mut LowpolyApp, body_key: &str) -> String {
    let tree = app.0.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
/// injected `interactionSelect` verb, dispatched against the "mesh" domain declared on this app.
/// `object_id`/`face_id` address the same row id the Document panel tree renders (see
/// `🧭️view/🦀️.rs`'s `🔖️MeshDomain` region).
pub async fn select_face(app: &mut LowpolyApp, object_id: &str, face_id: u32) {
    select(app, &[("face", &semio_s_artifact_lowpoly_lowpoly_view::document_target_row_id(object_id, "face", face_id))]).await;
}

/// 🕹️ A replace-merge pick of `targets` (`(granularity, row id)`) in the mesh domain.
pub async fn select(app: &mut LowpolyApp, targets: &[(&str, &str)]) {
    let targets: Vec<serde_json::Value> = targets.iter().map(|(granularity, id)| serde_json::json!({ "granularity": granularity, "id": id })).collect();
    act(app, "interactionSelect", serde_json::json!({ "domainId": MESH_INTERACTION_DOMAIN, "targets": serde_json::to_string(&targets).expect("targets"), "merge": "replace", "method": "pick" })).await;
}
