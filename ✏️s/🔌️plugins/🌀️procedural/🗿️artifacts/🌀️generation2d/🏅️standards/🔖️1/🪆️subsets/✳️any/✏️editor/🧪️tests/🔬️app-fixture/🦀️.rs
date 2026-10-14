//! 🧪️ The registered generation2d editor app fixture every editor route law drives: construction, dispatch, render and the retirement of the owners a law opens.
use semio_s_artifact_procedural_2d_mode_edit::windows::flow;
use semio_s_artifact_procedural_2d_mode_edit::windows::preview;
use semio_s_artifact_procedural_2d_mode_edit as edit;
use semio_s_artifact_procedural_2d_mode_generate::windows::generations;
use semio_s_artifact_procedural_2d_mode_generate as generate;
use semio_s_artifact_procedural_2d_editor_panels::document;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Fault;
use semio_s_artifact_procedural_2d_editor::*;
use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Generation2dApp = VcsArtifactApp<EditorApp<Generation2dPlayApp>>;

/// 🧪️ The ONE app fixture. This app publishes `bounded_first_step_tool_proofs!` factories, so the
/// registryless `context::new_app` cannot satisfy the framework's tool-proof catalog: `migrated_tool_ids`
/// reads an EMPTY `AppActionRegistry` while the generated catalog lists all 21 rows, and every bounded
/// proof is rejected with `interactive-job.catalog-authority` before the app is even constructed. Every
/// fixture therefore goes through the registry variant, exactly as the `🧊️generation3d` sibling does
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.3).
pub async fn app() -> Generation2dApp {
    app_with_registry().await
}

pub async fn app_with_registry() -> Generation2dApp {
    let mut app = new_app_with_registry::<EditorApp<Generation2dPlayApp>>(generation2d_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(1).await;
    app
}

/// 📸️ `PluginApp::snapshot` hands back an OWNED projection whose `fixture.layout` is a live
/// `OrderedMap` root — the bundled 2d default document carries layout entries, so a bare
/// `app.snapshot().expect(..)` temporary panics on drop. Every read goes through this closing read.
pub fn snapshot_read(app: &Generation2dApp) -> semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshotRead {
    semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshotRead::new(app.snapshot().expect("Generation2d fixture app snapshot"))
}

/// 🕹️ Dispatch AND settle. Every generation2d action is `InteractiveJobClassification::Migrated`, so
/// `dispatch_typed` only ENQUEUES a retained job — the document, config and transient lanes are
/// published when the host drives the ladder. `settle_registered_typed_operation` is that host loop.
pub async fn dispatch(app: &mut Generation2dApp, command: Generation2dCommand) -> InvocationResult {
    let result = app.dispatch_typed(command, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("dispatch");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("Generation2d dispatched operation settles");
    result
}

/// 🧹️ Walks the fixture app to its terminal-empty ownership witness — a registered app owns an
/// `ArtifactStore` and a fixed owner registry, both of which reject a bare drop.
pub fn close(mut app: Generation2dApp) {
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}


/// 🎯️ Selects `ids` in the framework-owned `graph` domain — the twin of generation3d's helper.
/// `interactionSelect` is a `FrameworkInteractionSelectJob`, so a bare `handle_action` only
/// admits; settle the reserved tool job before any selection-fed read (context menu, delete).
pub async fn select_graph(app: &mut Generation2dApp, granularity: &str, ids: &[&str]) -> InvocationResult {
    let targets: Vec<semio_framework_plugin::InteractionTarget> = ids.iter().map(|id| semio_framework_plugin::InteractionTarget { granularity: granularity.into(), id: (*id).into() }).collect();
    let targets_json = serde_json::to_string(&targets).expect("selection targets");
    let args: semio_framework_value::DslValue = serde_json::json!({ "domainId": semio_s_artifact_procedural_2d_model::GENERATION2D_INTERACTION_DOMAIN, "targets": targets_json, "merge": "replace", "method": "pick" }).into();
    let admitted = app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("interaction selection admitted");
    semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.expect("interaction selection settles its reserved tool job")
}

pub async fn render(app: &mut Generation2dApp, body_key: &str) -> String {
    render_with_view(app, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await
}

/// 🌍️ The localized twin of [`render`] — labels resolve off `ViewModel::locale`, so a translation
/// law has to hand the renderer the locale it is asserting.
pub async fn render_with_view(app: &mut Generation2dApp, body_key: &str, view_state: &ViewModel) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
}

/// ✏️ Adapts `create_generation2d_app`'s `AppDefinition` (contract §2.4) into the `App {
/// definition, examples }` shape `context::assert_declared_actions_bridge_to_commands` still
/// expects — framework test context gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn generation2d_manifest_for_tests() -> App {
    App { definition: create_generation2d_app(), examples: Vec::new() }
}

semio_framework_plugin::history_edit_acceptance_law!("procedural", Generation2dPlayApp, generation2d_manifest_for_tests, "../..");

/// 🧹️ `FlowEvalSession` rejects a live drop, so a test that owns one must walk it across the close
/// boundary itself — the same `begin_close` + granted `close_step` loop
/// `Generation2dInstanceOperationOwner::maintenance_step` runs in production.
pub fn retire_flow_eval_session(mut session: FlowEvalSession) {
    close_flow_session(&mut session);
}

/// 🏛️ The edit layout's shell roster — flow window current, edit preview attached — and the preview's own view.
pub fn edit_shell_views() -> (ViewModel, ViewModel) {
    let roster = vec![
        semio_framework::ViewWindowInstance { id: "flow-1".into(), window_kind_id: flow_window::GENERATION2D_PLAY_WINDOW_MAIN.into() },
        semio_framework::ViewWindowInstance { id: "preview-1".into(), window_kind_id: edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW.into() },
    ];
    let view = ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    (view.for_window_instance("flow-1").expect("flow window instance"), view.for_window_instance("preview-1").expect("preview window instance"))
}

/// 🏛️ The generate layout's shell roster with TWO generate previews — generations window current —
/// and each preview's own view.
pub fn generate_shell_views() -> (ViewModel, Vec<ViewModel>) {
    let roster = vec![
        semio_framework::ViewWindowInstance { id: "generations-1".into(), window_kind_id: generations::GENERATION2D_PLAY_WINDOW_GENERATIONS.into() },
        semio_framework::ViewWindowInstance { id: "preview-a".into(), window_kind_id: generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
        semio_framework::ViewWindowInstance { id: "preview-b".into(), window_kind_id: generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
    ];
    let view = ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    (view.for_window_instance("generations-1").expect("generations window instance"), ["preview-a", "preview-b"].iter().map(|id| view.for_window_instance(id).expect("generate preview window instance")).collect())
}

/// 🧾️ What one driven `previewEval` run did: its hops and their windows, the run actions fed back and
/// the state it rests in.
#[derive(Clone, Debug, Default)]
pub struct PreviewRunReceipt {
    pub hops: usize,
    pub hop_windows: Vec<String>,
    pub run_actions: Vec<String>,
    pub state: Option<String>,
}

/// 🚦️ The framework run actions an effect list carries.
pub fn run_actions(effects: &[semio_framework_plugin::Effect]) -> Vec<String> {
    effects.iter().filter_map(|effect| match effect {
        semio_framework_plugin::Effect::DispatchAction { action, .. } if semio_framework_plugin::is_tool_run_action_id(action) => Some(action.clone()),
        _ => None,
    }).collect()
}

/// 🔁️ The served `previewEval` loop, nothing hand-addressed: `pending_effects` off the shell roster,
/// every run action fed back through `PluginApp::handle_action`, every hop the run's port hands the
/// host redispatched as the typed command the shell sends. The demo operators are uncontributed in a
/// test binary, so no hop parks an extension invocation. Stops once nothing is owed or pending and
/// the run rests. `action_meta` names the shell view the roster is read off and the bound instance.
pub async fn drive_preview_run(app: &mut Generation2dApp, action_meta: &semio_framework_plugin::ActionMeta, initial: &[semio_framework_plugin::Effect]) -> PreviewRunReceipt {
    use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
    use semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation;
    let shell_view = action_meta.view_state.as_ref();
    let mut receipt = PreviewRunReceipt::default();
    let mut effects: Vec<semio_framework_plugin::Effect> = initial.to_vec();
    for _ in 0..100_000 {
        effects.extend(Box::pin(app.pending_effects(shell_view)).await);
        if app.has_pending_typed_operations() {
            effects.extend(Box::pin(settle_registered_typed_operation(app, action_meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).await.expect("the run's driver turns settle").effects);
        }
        let dispatches: Vec<(String, Option<semio_framework_value::DslValue>)> = std::mem::take(&mut effects).into_iter().filter_map(|effect| match effect {
            semio_framework_plugin::Effect::DispatchAction { action, args, .. } => Some((action, args)),
            _ => None,
        }).collect();
        receipt.state = app.tool_run_presence().map(|presence| presence.state.wire_name().to_string());
        if dispatches.is_empty() && !app.has_pending_typed_operations() && receipt.state.as_deref().is_none_or(|state| matches!(state, "complete" | "aborted" | "faulted" | "finalized")) {
            return receipt;
        }
        for (action, args) in dispatches {
            if semio_framework_plugin::is_tool_run_action_id(&action) {
                receipt.run_actions.push(action.clone());
                Box::pin(app.handle_action(&action, args.as_ref(), action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).await.unwrap_or_else(|fault| panic!("{action} dispatch: {fault:?}"));
                continue;
            }
            if action == "flowEvalTick" {
                receipt.hops += 1;
                receipt.hop_windows.push(args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string());
            }
            let arguments = match args.as_ref() {
                Some(semio_framework_value::DslValue::Object(entries)) => entries.iter().cloned().collect(),
                _ => std::collections::BTreeMap::new(),
            };
            let app_id = app.app_id().await.to_string();
            let invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: String::new(), app_id }, command_id: action.clone() }, arguments };
            Box::pin(app.handle_command(&invocation, None, action_meta)).await.unwrap_or_else(|fault| panic!("the shell redispatches the run's {action} hop: {fault:?}"));
            let hop = Box::pin(settle_registered_typed_operation(app, action_meta.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).await.expect("retained publication");
            assert!(!hop.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "the run's {action} hop faulted: args={args:?}");
            effects.extend(hop.effects);
        }
    }
    panic!("the previewEval run did not settle within 100 000 host turns; last state {:?}", receipt.state);
}

/// 📜️ The empty `HistoryView` a command-handler unit test hands `ArtifactView::new` — built here once
/// because `HistoryView` (`🧰️framework/…/🔌️plugin/🦀️.rs`) derives no `Default`.
pub fn empty_history_view() -> semio_framework_plugin::HistoryView {
    semio_framework_plugin::HistoryView {
        columns: Vec::new(),
        can_undo: false,
        can_redo: false,
        active_alternative_id: None, alternatives: Vec::new(),
        current_checkpoint_id: None,
        commands: Vec::new(),
        command_filter: semio_framework_plugin::app::HistoryCommandFilter::default(),
    }
}
