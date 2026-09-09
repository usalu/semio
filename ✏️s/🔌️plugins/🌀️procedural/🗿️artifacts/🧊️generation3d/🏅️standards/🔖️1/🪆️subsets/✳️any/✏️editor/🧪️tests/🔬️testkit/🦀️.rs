use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{ActionMeta, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

/// ✏️ `Generation3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Generation3dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Generation3dPlayApp>` builds it.
pub type Generation3dApp = VcsArtifactApp<EditorApp<Generation3dPlayApp>>;

/// ✏️ Adapts `create_generation3d_app`'s `AppDefinition` (contract §2.4) into the
/// `App { definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` /
/// `testkit::new_app_with_registry` still expect — framework testkit gap, not modifiable here
/// (`🧰️framework/**` is outside this packet's lease).
pub fn generation3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_generation3d_app(), examples: Vec::new() }
}

pub async fn app() -> Generation3dApp {
    let mut app = new_app::<EditorApp<Generation3dPlayApp>>().await;
    app.bind_instance_id(1).await;
    app
}

pub async fn app_with_registry() -> Generation3dApp {
    let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    app
}

pub async fn dispatch(app: &mut Generation3dApp, command: Generation3dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub fn preview_views(left: &str, right: &str) -> (ViewModel, ViewModel) {
    let roster = vec![
        ViewWindowInstance { id: left.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
        ViewWindowInstance { id: right.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
    ];
    let view = ViewModel { window_instances: roster, ..Default::default() };
    (view.for_window_instance(left).expect("left Generation3d preview window"), view.for_window_instance(right).expect("right Generation3d preview window"))
}

pub async fn dispatch_with_view(app: &mut Generation3dApp, command: Generation3dCommand, view_state: ViewModel) -> Result<InvocationResult, semio_framework_plugin::Fault> {
    app.dispatch_typed(command, &ActionMeta { view_state: Some(view_state), ..meta("local") }).await
}

pub async fn render(app: &mut Generation3dApp, body_key: &str) -> String {
    let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
    render_with_view(app, body_key, &view).await
}

pub async fn render_with_view(app: &mut Generation3dApp, body_key: &str, view_state: &ViewModel) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
}

/// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
/// drains in production — a test has to do that draining itself.
pub async fn drain_flow_eval_ticks(app: &mut Generation3dApp) {
    let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
    drain_flow_eval_ticks_with_view(app, &view).await;
}

pub async fn drain_flow_eval_ticks_with_view(app: &mut Generation3dApp, view: &ViewModel) {
    app.pending_effects().await;
    for _ in 0..1000 {
        let result = dispatch_with_view(app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}), view.clone()).await.expect("flowEvalTick");
        if !result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")) {
            return;
        }
    }
    panic!("flowEvalTick chain did not converge within 1000 ticks");
}

/// 🧹️ `FlowEvalSession` rejects a live drop (`🌊️flow/🖥️host/🦀️.rs`'s `Drop` +
/// `live_session_drop_is_rejected_without_recursive_payload_destruction`), so a test that owns one
/// must walk it across the close boundary itself — the same `begin_close` + granted `close_step`
/// loop `FlowInstanceOperationOwner::maintenance_step` runs in production.
pub fn retire_flow_eval_session(mut session: FlowEvalSession) {
    session.begin_close();
    for _ in 0..1_000_000 {
        match session.close_step(1, 65_536) {
            semio_framework_job::InteractiveJobCloseStep::Pending { .. } => continue,
            semio_framework_job::InteractiveJobCloseStep::Complete => return,
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("a positive close grant must never block the evaluation session"),
        }
    }
    panic!("the evaluation session did not reach terminal-empty under a positive close grant");
}

/// 📜️ The empty `HistoryView` a command-handler unit test hands `ArtifactView::new` — built here once
/// because `HistoryView` (`🧰️framework/…/🔌️plugin/🦀️.rs`) derives no `Default`.
pub fn empty_history_view() -> semio_framework_plugin::HistoryView {
    semio_framework_plugin::HistoryView {
        columns: Vec::new(),
        can_undo: false,
        can_redo: false,
        active_alternative_id: None,
        current_checkpoint_id: None,
        commands: Vec::new(),
        command_filter: semio_framework_plugin::app::HistoryCommandFilter::default(),
    }
}
