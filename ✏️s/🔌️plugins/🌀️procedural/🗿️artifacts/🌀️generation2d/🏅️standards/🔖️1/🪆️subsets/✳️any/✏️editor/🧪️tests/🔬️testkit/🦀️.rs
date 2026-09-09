use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Generation2dApp = VcsArtifactApp<EditorApp<Generation2dPlayApp>>;

/// 🧪️ The ONE app fixture. This app publishes `bounded_first_step_tool_proofs!` factories, so the
/// registryless `testkit::new_app` cannot satisfy the framework's tool-proof catalog: `migrated_tool_ids`
/// reads an EMPTY `AppActionRegistry` while the generated catalog lists all 21 rows, and every bounded
/// proof is rejected with `interactive-job.catalog-authority` before the app is even constructed. Every
/// fixture therefore goes through the registry variant, exactly as the `🧊️generation3d` sibling does
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.3).
pub async fn app() -> Generation2dApp {
    app_with_registry().await
}

pub async fn app_with_registry() -> Generation2dApp {
    let mut app = new_app_with_registry::<EditorApp<Generation2dPlayApp>>(generation2d_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    app
}

/// 📸️ `PluginApp::snapshot` hands back an OWNED projection whose `fixture.layout` is a live
/// `OrderedMap` root — the bundled 2d default document carries layout entries, so a bare
/// `app.snapshot().expect(..)` temporary panics on drop. Every read goes through this closing read.
pub fn snapshot_read(app: &Generation2dApp) -> crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead {
    crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead::new(app.snapshot().expect("Generation2d fixture app snapshot"))
}

/// 🕹️ Dispatch AND settle. Every generation2d action is `InteractiveJobClassification::Migrated`, so
/// `dispatch_typed` only ENQUEUES a retained job — the document, config and transient lanes are
/// published when the host drives the ladder. `settle_registered_typed_operation` is that host loop.
pub async fn dispatch(app: &mut Generation2dApp, command: Generation2dCommand) -> InvocationResult {
    let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
    semio_framework_plugin::testkit::settle_registered_typed_operation(app, 1).await.expect("Generation2d dispatched operation settles");
    result
}

/// 🧹️ Walks the fixture app to its terminal-empty ownership witness — a registered app owns an
/// `ArtifactStore` and a fixed owner registry, both of which reject a bare drop.
pub fn close(mut app: Generation2dApp) {
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut app);
}

pub async fn render(app: &mut Generation2dApp, body_key: &str) -> String {
    render_with_view(app, body_key, &ViewModel::default()).await
}

/// 🌍️ The localized twin of [`render`] — labels resolve off `ViewModel::locale`, so a translation
/// law has to hand the renderer the locale it is asserting.
pub async fn render_with_view(app: &mut Generation2dApp, body_key: &str, view_state: &ViewModel) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
}

/// ✏️ Adapts `create_generation2d_app`'s `AppDefinition` (contract §2.4) into the `App {
/// definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still
/// expects — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn generation2d_manifest_for_testkit() -> App {
    App { definition: create_generation2d_app(), examples: Vec::new() }
}

/// 🧹️ `FlowEvalSession` rejects a live drop, so a test that owns one must walk it across the close
/// boundary itself — the same `begin_close` + granted `close_step` loop
/// `Generation2dInstanceOperationOwner::maintenance_step` runs in production.
pub fn retire_flow_eval_session(mut session: FlowEvalSession) {
    close_flow_session(&mut session);
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
