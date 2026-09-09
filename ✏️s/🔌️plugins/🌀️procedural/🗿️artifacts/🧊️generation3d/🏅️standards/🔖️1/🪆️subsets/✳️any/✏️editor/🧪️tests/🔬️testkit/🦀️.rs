use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry, settle_registered_typed_operation, TypedOperationFixtureReceipt};
use semio_framework_plugin::{ActionMeta, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

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

/// 🧹️ A live app fixture that CLOSES itself. `VcsArtifactApp`'s `ArtifactStore` owns an
/// `ArtifactStoreCursorDisposer` whose `Drop` asserts terminal-empty ownership, and the document
/// snapshot behind it owns `OrderedMap` roots that reject a bare drop — so a plainly-dropped fixture
/// double-panics and aborts the whole test binary. Dereferences to the app, and drains the exact
/// retained close ladder (`PluginApp::close_step`) on the way out, which is the same law the runtime
/// uses (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub struct Generation3dAppFixture(Generation3dApp);

impl std::ops::Deref for Generation3dAppFixture {
    type Target = Generation3dApp;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Generation3dAppFixture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Generation3dAppFixture {
    fn drop(&mut self) {
        for _ in 0..1_000_000 {
            if self.0.close_terminal_is_empty() {
                return;
            }
            if self.0.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).is_err() {
                break;
            }
        }
        assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "Generation3d app fixture did not reach its terminal-empty close witness");
    }
}

/// 🧪️ The ONE app fixture. This app publishes `bounded_first_step_tool_proofs!` factories, so the
/// registryless `testkit::new_app` cannot satisfy the framework's tool-proof catalog and faults with
/// `interactive-job.catalog-authority` — which then aborts the whole process, because the unwind
/// runs `ArtifactStoreCursorDisposer`'s Drop before its terminal-empty close. Every fixture
/// therefore goes through the registry variant (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn app() -> Generation3dAppFixture {
    app_with_registry().await
}

pub async fn app_with_registry() -> Generation3dAppFixture {
    let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    Generation3dAppFixture(app)
}

/// 📸️ Reads the live projection into a self-retiring [`Generation3dSnapshotRead`] — never
/// `app.snapshot()` directly, whose owned `Generation3dSnapshot` aborts the binary on a bare drop.
pub fn snapshot(app: &Generation3dApp) -> crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead {
    crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(app.snapshot().expect("snapshot"))
}

/// 🎯️ Dispatches a typed command AND completes its bounded host publication protocol. Every one of
/// this app's 28 tools is a RETAINED job (`GENERATION3D_RETAINED_TOOL_IDS`), so `dispatch_typed`
/// only returns an admission receipt — `{"operationId","generation"}` — and the mutation does not
/// reach the store until the host advances maintenance/publication turns and acknowledges the
/// result page. Reading `snapshot(&app)` straight after a bare `dispatch_typed` therefore observes
/// the PRE-command document, which is why a whole class of editor-command tests passed vacuously
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn dispatch(app: &mut Generation3dApp, command: Generation3dCommand) -> TypedOperationFixtureReceipt {
    dispatch_with_view_meta(app, command, meta("local")).await.expect("dispatch")
}

pub fn preview_views(left: &str, right: &str) -> (ViewModel, ViewModel) {
    let roster = vec![
        ViewWindowInstance { id: left.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
        ViewWindowInstance { id: right.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
    ];
    let view = ViewModel { window_instances: roster, ..Default::default() };
    (view.for_window_instance(left).expect("left Generation3d preview window"), view.for_window_instance(right).expect("right Generation3d preview window"))
}

pub async fn dispatch_with_view(app: &mut Generation3dApp, command: Generation3dCommand, view_state: ViewModel) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin::Fault> {
    dispatch_with_view_meta(app, command, ActionMeta { view_state: Some(view_state), ..meta("local") }).await
}

/// 🔁️ Completes whatever retained publication is already in flight — the settle half of
/// [`dispatch`], for a caller that entered through `PluginApp::handle_action` instead.
pub async fn settle(app: &mut Generation3dApp) -> TypedOperationFixtureReceipt {
    settle_registered_typed_operation(app, meta("local").instance_id).await.expect("retained publication")
}

async fn dispatch_with_view_meta(app: &mut Generation3dApp, command: Generation3dCommand, action_meta: ActionMeta) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin::Fault> {
    app.dispatch_typed(command, &action_meta).await?;
    settle_registered_typed_operation(app, action_meta.instance_id).await
}

pub async fn render(app: &mut Generation3dApp, body_key: &str) -> String {
    let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
    render_with_view(app, body_key, &view).await
}

pub async fn render_with_view(app: &mut Generation3dApp, body_key: &str, view_state: &ViewModel) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
}

/// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
/// drains in production — a test has to do that draining itself. It also declares its geometry work
/// through `Emit::extension_invocations`, which only the SHELL answers in production: every
/// preview handle is tessellated inside the `brep` extension, and until the answer comes back the
/// session holds no mesh pack. Both halves are driven here, so the chain converges on a REAL
/// preview instead of on an empty one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn drain_flow_eval_ticks(app: &mut Generation3dApp) {
    let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
    drain_flow_eval_ticks_with_view(app, &view).await;
}

pub async fn drain_flow_eval_ticks_with_view(app: &mut Generation3dApp, view: &ViewModel) {
    app.pending_effects().await;
    for _ in 0..1000 {
        let receipt = dispatch_with_view(app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}), view.clone()).await.expect("flowEvalTick");
        let answered = crate::brep_extension::settle(app, meta("local").instance_id).await;
        let rearmed = receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick"));
        if !rearmed && answered == 0 {
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
