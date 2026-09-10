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
    let window_id = view.window_id.clone().expect("a drained tick view is narrowed to one preview window");
    app.pending_effects(Some(view)).await;
    for _ in 0..1000 {
        let receipt = dispatch_with_view(app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: window_id.clone() }), view.clone()).await.expect("flowEvalTick");
        let answered = crate::brep_extension::settle(app, meta("local").instance_id).await;
        let rearmed = receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick"));
        if !rearmed && answered == 0 {
            return;
        }
    }
    panic!("flowEvalTick chain did not converge within 1000 ticks");
}

/// 🏛️ The SHELL's own roster: the flow window is current (that is the window a served boot focuses,
/// and the window an unaddressed `Effect::DispatchAction` would be redispatched under), with the
/// preview window attached alongside it. Every window-scoped route has to find its own window in
/// here rather than assume it is the current one.
pub fn shell_views(flow: &str, preview: &str) -> (ViewModel, ViewModel) {
    let roster = vec![
        ViewWindowInstance { id: flow.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::flow::GENERATION_3D_PLAY_WINDOW_MAIN.into() },
        ViewWindowInstance { id: preview.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
    ];
    let view = ViewModel { window_instances: roster, ..Default::default() };
    (view.for_window_instance(flow).expect("flow window instance"), view.for_window_instance(preview).expect("preview window instance"))
}

/// 🔁️ The REAL served chain, end to end: nothing is hand-addressed here. `pending_effects` arms the
/// first tick off the host's attached-window roster, and every following tick is the `action`+`args`
/// of an `Effect::DispatchAction` the app itself emitted, replayed through `PluginApp::handle_action`
/// under the SHELL's current window — the flow window — exactly the way `ShellHost` feeds
/// `requestedEffects` back. So the tick's window address, its wire decode and the retained route's
/// own preflight are all exercised, which a hand-built preview-addressed command skips entirely
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// 🏛️ Redispatches one armed `Effect::DispatchAction` exactly the way `makeEffectDispatchOne`
/// (`🛠️ShellHelpers/🟦️.tsx`) does: an action id the app declares as a COMMAND re-enters the typed
/// command channel with the shell's own live view attached, never the scoped action channel — which
/// is why `handle_action("flowEvalTick", …)` is answered with `interactive-job.unknown-key`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn dispatch_effect_command(app: &mut Generation3dApp, command_id: &str, args: Option<&dsl::DslValue>, action_meta: &ActionMeta) -> Result<(), semio_framework_plugin::Fault> {
    use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
    let arguments = match args {
        Some(dsl::DslValue::Object(entries)) => entries.iter().cloned().collect(),
        _ => std::collections::BTreeMap::new(),
    };
    let app_id = app.app_id().await.to_string();
    let invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: String::new(), app_id }, command_id: command_id.to_string() }, arguments };
    app.handle_command(&invocation, None, action_meta).await.map(|_| ())
}

pub async fn drain_armed_flow_eval_ticks(app: &mut Generation3dApp, shell_view: &ViewModel) -> usize {
    let armed = app.pending_effects(Some(shell_view)).await;
    drain_armed_flow_eval_ticks_from(app, shell_view, &armed).await
}

/// 🔁️ The same drain, started from effects some OTHER route already armed — the shape a law needs
/// when the claim under test is that a route re-arms a chain nothing else would
/// (`setContributions` installing a registry the first evaluation ran without), rather than that
/// `pending_effects` arms the first one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn drain_armed_flow_eval_ticks_from(app: &mut Generation3dApp, shell_view: &ViewModel, initial: &[Effect]) -> usize {
    let action_meta = ActionMeta { view_state: Some(shell_view.clone()), ..meta("local") };
    let mut armed = armed_ticks(initial);
    let mut ticks = 0;
    for _ in 0..1000 {
        let Some(args) = armed.pop() else { return ticks };
        dispatch_effect_command(app, "flowEvalTick", args.as_ref(), &action_meta).await.expect("the shell redispatches an armed flowEvalTick effect");
        let receipt = settle_registered_typed_operation(app, action_meta.instance_id).await.expect("retained publication");
        assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "an armed flowEvalTick faulted in the retained job ladder: args={args:?}");
        ticks += 1;
        arm(&mut armed, armed_ticks(&receipt.effects));
        arm(&mut armed, armed_ticks(&crate::brep_extension::settle_with_meta(app, action_meta.instance_id, &action_meta).await.effects));
    }
    panic!("the armed flowEvalTick chain did not converge within 1000 dispatches");
}

/// 🔁️ Coalesces re-arms the way a shell effect queue does — a tick that both re-arms itself and
/// parks an extension continuation would otherwise double the queue on every hop.
fn arm(armed: &mut Vec<Option<dsl::DslValue>>, more: Vec<Option<dsl::DslValue>>) {
    for entry in more {
        if !armed.contains(&entry) {
            armed.push(entry);
        }
    }
}

fn armed_ticks(effects: &[Effect]) -> Vec<Option<dsl::DslValue>> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == "flowEvalTick" => Some(args.clone()),
            _ => None,
        })
        .collect()
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

/// 🧩️ The host's `contributionsJson` for the generation3d closure, built the way
/// `buildContributionsJson` (`🎠️kernel/🟦️.ts`) builds it: one `{pluginId, topicContribution}` entry
/// per `flow.extension` topic contribution the staged extension plugins declare, JSON-encoded as an
/// array. The two manifests come from the extension crates themselves, so this is the SAME payload
/// the served shell pushes — not a hand-written stand-in.
pub fn staged_flow_extension_contributions_json(extra: &[(&str, String)]) -> String {
    let entries: Vec<semio_framework::manifest::ProgramContributionEntry> = [
        (crate::flow_operators::BREP_EXTENSION_PLUGIN_ID, crate::flow_operators::resolve_ready(semio_s_plugin_flow_extension_brep::extension_manifest_json())),
        (crate::flow_operators::MATH_EXTENSION_PLUGIN_ID, semio_s_plugin_flow_extension_math::extension_manifest_json()),
    ]
    .into_iter()
    .chain(extra.iter().map(|(plugin_id, manifest_json)| (*plugin_id, manifest_json.clone())))
    .map(|(plugin_id, manifest_json)| semio_framework::manifest::ProgramContributionEntry {
        plugin_id: plugin_id.to_string(),
        topic_contribution: Some(semio_framework::manifest::TopicContribution::new("flow.extension", dsl::DslValue::object([("manifestJson".to_string(), dsl::DslValue::String(manifest_json))]))),
    })
    .collect();
    protocol::json::to_json_string(&entries)
}

/// 🪪️ A contributed `flow.extension` manifest that carries no operators at all — the witness a
/// delivery law adds to the payload so that "the registry now holds what the host pushed" is a claim
/// about THIS run, never about a manifest some earlier `install_flow_extension_manifest` left behind.
pub const CONTRIBUTIONS_WITNESS_EXTENSION_ID: &str = "contributions-delivery-witness";
pub const CONTRIBUTIONS_WITNESS_PLUGIN_ID: &str = "flow-extension-contributions-delivery-witness";

pub fn contributions_witness_manifest_json() -> String {
    protocol::json::to_json_string(&dsl::DslValue::object([
        ("schema".to_string(), dsl::DslValue::String("flow.extension".into())),
        ("id".to_string(), dsl::DslValue::String(CONTRIBUTIONS_WITNESS_EXTENSION_ID.into())),
        ("name".to_string(), dsl::DslValue::String("Contributions Delivery Witness".into())),
        ("version".to_string(), dsl::DslValue::String("1.0.0".into())),
        ("activationEvents".to_string(), dsl::DslValue::Array(vec![dsl::DslValue::String("onStartup".into())])),
        (
            "contributes".to_string(),
            dsl::DslValue::object([
                ("schemas".to_string(), dsl::DslValue::Array(Vec::new())),
                ("operators".to_string(), dsl::DslValue::Array(Vec::new())),
                ("widgets".to_string(), dsl::DslValue::Array(Vec::new())),
                ("commands".to_string(), dsl::DslValue::Array(Vec::new())),
                ("settings".to_string(), dsl::DslValue::Array(Vec::new())),
            ]),
        ),
    ]))
}

//#region 🧮️HeapWitness
/// 🧮️ This test binary's allocator. The guest this artifact ships into runs in ONE fixed
/// `GUEST_LINEAR_MEMORY_MAXIMUM_BYTES` linear memory with ONE allocator and no threads, so an owner
/// the install path never frees is a hard `rust_oom` trap in a browser tab and nothing at all in a
/// native suite — the only way a law can state the bound is to weigh the heap itself
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[global_allocator]
static GENERATION3D_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

/// 📸️ One labelled reading of the process heap, printed as `[MEMORY]` so a lane can read the whole
/// boot → page → install → eval → tessellation sequence off one run's output.
pub fn heap_probe(label: &str) -> (isize, isize) {
    let retained = semio_framework_trace::retained_heap_bytes();
    let peak = semio_framework_trace::peak_heap_bytes();
    println!("[MEMORY] {label}: retained={retained} peak={peak}");
    (retained, peak)
}
//#endregion 🧮️HeapWitness
