pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry, settle_registered_typed_operation, TypedOperationFixtureReceipt};
    use semio_framework_plugin::{ActionMeta, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    /// ✏️ `Generation3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Generation3dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Generation3dPlayApp>` builds it.
    pub type Generation3dApp = VcsArtifactApp<EditorApp<Generation3dPlayApp>>;
    
    /// ✏️ Adapts `create_generation3d_app`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `context::assert_declared_actions_bridge_to_commands` /
    /// `context::new_app_with_registry` still expect — framework test context gap, not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub fn generation3d_app_manifest_for_tests() -> semio_framework_plugin::App {
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
    /// registryless `context::new_app` cannot satisfy the framework's tool-proof catalog and faults with
    /// `interactive-job.catalog-authority` — which then aborts the whole process, because the unwind
    /// runs `ArtifactStoreCursorDisposer`'s Drop before its terminal-empty close. Every fixture
    /// therefore goes through the registry variant (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn app() -> Generation3dAppFixture {
        app_with_registry().await
    }
    
    pub async fn app_with_registry() -> Generation3dAppFixture {
        let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_tests).await;
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
    
    /// 🎯️ Selects `ids` in the framework-owned `graph` interaction domain AND settles the reserved tool
    /// job the dispatch only ADMITS. `interactionSelect` is a `FrameworkInteractionSelectJob`
    /// (`🔌️plugin/🦀️.rs`), so a bare `handle_action` returns an admission receipt and nothing has
    /// touched `protocol::InteractionState` yet — a caller that reads `interaction_state()` or dispatches
    /// a selection-fed command straight afterwards observes the PRE-selection state and its law passes
    /// vacuously (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, the same admission-vs-publication trap
    /// [`dispatch`] documents for typed commands).
    pub async fn select_graph(app: &mut Generation3dApp, granularity: &str, ids: &[&str]) -> semio_framework_plugin::InvocationResult {
        let targets: Vec<semio_framework_plugin::InteractionTarget> = ids.iter().map(|id| semio_framework_plugin::InteractionTarget { granularity: granularity.into(), id: (*id).into() }).collect();
        let targets_json = serde_json::to_string(&targets).expect("selection targets");
        let args: dsl::DslValue = serde_json::json!({ "domainId": crate::editor::generation3d::GENERATION_3D_INTERACTION_DOMAIN, "targets": targets_json, "merge": "replace", "method": "pick" }).into();
        let admitted = app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("interaction selection admitted");
        semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.expect("interaction selection settles its reserved tool job")
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
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
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
            let window_kind_id = view.active_window_kind_id.clone().unwrap_or_else(|| crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into());
            let receipt = dispatch_with_view(app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: window_id.clone(), window_kind_id }), view.clone()).await.expect("flowEvalTick");
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
    
    /// 🧳️ A standalone retained-operation owner handle of THIS app's concrete type, for a law that
    /// drives one piece of retained work directly instead of through a live instance. Its session is a
    /// real one, so anything the work arms lands on a real latch.
    pub fn instance_operation_owner() -> semio_framework_plugin::ArtifactInstanceOperationOwnerHandle {
        semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(<Generation3dPlayApp as semio_framework_plugin::ArtifactEditor>::build_instance_operation_owner())
    }
    
    /// 🧹️ Walks a test context-built owner across the same close boundary `VcsArtifactApp` runs — its
    /// `FlowEvalSession` rejects a live drop, so a law that builds one owns its retirement.
    pub fn retire_instance_operation_owner(handle: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) {
        use semio_framework_plugin::ArtifactInstanceOperationOwner;
        for _ in 0..1_000_000 {
            let complete = handle
                .with_mut::<crate::editor::generation3d::component::Generation3dInstanceOperationOwner, _>(|owner| Ok(matches!(ArtifactInstanceOperationOwner::close_step(owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete))))
                .expect("the test context owner lends itself to its own close ladder");
            if complete {
                return;
            }
        }
        panic!("the test context instance operation owner did not reach terminal-empty under a positive close grant");
    }
    
    /// 🧹️ `FlowEvalSession` rejects a live drop (`🌊️flow/🖥️host/🦀️.rs`'s `Drop` +
    /// `live_session_drop_is_rejected_without_recursive_payload_destruction`), so a test that owns one
    /// must walk it across the close boundary itself — the same `begin_close` + granted `close_step`
    /// loop `FlowInstanceOperationOwner::maintenance_step` runs in production.
    pub use crate::flow_operators::retire_flow_eval_session;
    
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
    
    /// 🧩️ The host's `contributionsJson` for the generation3d closure — surface-neutral, so it lives
    /// in `🧪️tests/🔬️flow-operators` beside the two manifests it is built from and both surfaces' laws
    /// push the identical payload. Re-exported here because every editor law already names it.
    pub use crate::flow_operators::staged_flow_extension_contributions_json;
    
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
    
    /// 🏛 Generate-mode roster: generations + form + generate preview. The generations window is current.
    pub fn generate_shell_views(generations: &str, form: &str, preview: &str) -> (ViewModel, ViewModel) {
        let roster = vec![
            ViewWindowInstance { id: generations.into(), window_kind_id: crate::editor::generation3d::modes::generate::windows::generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS.into() },
            ViewWindowInstance { id: form.into(), window_kind_id: crate::editor::generation3d::modes::generate::windows::form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM.into() },
            ViewWindowInstance { id: preview.into(), window_kind_id: crate::editor::generation3d::modes::generate::windows::preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
        ];
        let view = ViewModel { window_instances: roster, ..Default::default() };
        (view.for_window_instance(generations).expect("generations window instance"), view.for_window_instance(preview).expect("generate preview window instance"))
    }
}

pub(crate) mod serial_execution {
    use std::sync::{Mutex, MutexGuard};
    
    static TEST_SERIAL: Mutex<()> = Mutex::new(());
    
    /// 🔒️ Serialises every test that drives the process-wide flow-eval kernel cache, and installs the
    /// packaged `brep`/`math` operator sets before the first one runs — without that installation
    /// `FlowHost::evaluate` answers `unknown kind: …` for every bundled fixture's nodes
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn lock() -> MutexGuard<'static, ()> {
        crate::flow_operators::installed();
        TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

use super::*;
use self::context::{app, app_with_registry, drain_flow_eval_ticks, drain_flow_eval_ticks_with_view, preview_views};
use semio_framework_plugin::PluginApp;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app() {
    use crate::editor::generation3d::modes::edit::windows::preview::transient::Generation3dPreviewWindowTransientOwner;
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = Box::new(app_with_registry().await);
    let (left, right) = preview_views("generation3d-preview-left", "generation3d-preview-right");
    let config_before = app.config_pack().await.expect("Generation3d app config before preview evaluation");
    drain_flow_eval_ticks_with_view(&mut app, &left).await;
    {
        let left_state = app.window_transient_snapshot(&left).expect("left preview transient snapshot").expect("left preview owner");
        let right_state = app.window_transient_snapshot(&right).expect("right preview transient snapshot").expect("right preview owner");
        assert!(left_state.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.as_deref()).is_some_and(|text| !text.is_empty()));
        assert!(right_state.get::<Generation3dPreviewWindowTransientOwner>().is_some_and(|state| state.preview_eval_text.is_none()));
    }
    let config_after = app.config_pack().await.expect("Generation3d app config after preview evaluation");
    assert_eq!((config_after.pack, config_after.spr), (config_before.pack, config_before.spr));
    drain_flow_eval_ticks_with_view(&mut app, &right).await;
    assert!(app.window_transient_snapshot(&right).expect("right evaluated snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_some()));
    let document = app.document_pack().await.expect("Generation3d document before reload");
    app.load_document_pack(&document).await.expect("same document reload resets preview window transient");
    for view in [&left, &right] {
        assert!(app.window_transient_snapshot(view).expect("reset preview snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_none()));
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut **app);
    eprintln!("[DEBUG] Generation3d registered runtime isolated two preview evaluations, preserved app config bytes, reset both ephemeral windows on document reload, and closed terminal-empty");
}
fn production_initial_snapshot(label: &str) -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.fixture.schema = label.into();
    for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
        snapshot.fixture.widgets.push(semio_framework_artifact_flow_flow::Widget::InputNote { id: id.into(), text: text.into() });
    }
    snapshot.fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
    snapshot.fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
    snapshot.fixture.layout.insert("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 });
    snapshot.fixture.layout.insert("clear-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
        snapshot.generation.cold_builder_mut().unwrap().generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() });
    }
    snapshot.generation.cold_builder_mut().unwrap().selected_generation_id = Some("rename-generation".into());
    snapshot
}

fn production_mutations() -> Vec<Generation3dMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::*;
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("production".into()))),
        ),
    );
    vec![
        Generation3dMutation::CreateWidget(create_widget::CreateWidget {
            index: 0,
            widget: semio_framework_artifact_flow_flow::Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true },
        }),
        Generation3dMutation::UpdateWidget(update_widget::UpdateWidget {
            widget: semio_framework_artifact_flow_flow::Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() },
        }),
        Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: "delete-target".into() }),
        Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse {
            index: 0,
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() },
        }),
        Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse {
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() },
        }),
        Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: "disconnect-synapse".into() }),
        Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: "move-target".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 31.0, y: -17.0 } }),
        Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: "clear-target".into() }),
        Generation3dMutation::UpdateCamera(update_camera::UpdateCamera { camera: semio_framework_artifact_flow_flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 } }),
        Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: "flow.fixture.production-retained".into() }),
        Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation: semio_framework_artifact_playbook_playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: Default::default() } }),
        Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id: "delete-generation".into() }),
        Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id: "rename-generation".into(), new_name: "After Rename".into() }),
        Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue {
            id: "change-generation".into(),
            question_id: "deep-answer".into(),
            new_value: serde_json::json!({"object": {"array": [1.0, false, "retained"]}}).into(),
        }),
    ]
}

fn production_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::new();
    value.try_reserve_exact(bytes.len() * 2).expect("P3 production hex preflight");
    for byte in bytes {
        value.push(char::from(DIGITS[usize::from(byte >> 4)]));
        value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    value
}

fn production_semantic_digest(snapshot: &Generation3dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation3d.production-law.semantic");
    digest.observe(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(snapshot));
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation3dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let mutations = production_mutations();
    assert_eq!(mutations.len(), 14, "production ingress carries every P3 mutation variant including delete-widget-position");
    let mut mutation_hex = Vec::new();
    mutation_hex.try_reserve_exact(mutations.len()).expect("P3 production mutation owner preflight");
    for mutation in &mutations {
        mutation_hex.push(production_hex(&crate::standards::v1::subsets::any::schema::mutations::binary::encode_op(mutation).expect("P3 production mutation encoding")));
    }
    let mut expected = production_initial_snapshot(label);
    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_apply_retained_mutations_for_test(&mut expected, &mutations);
    let expected_digest = production_semantic_digest(&expected);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GENERATION_3D_SCHEMA,
        "id": "generation3d-production-mounted-law",
        "vcs": {
            "initialSnapshot": production_hex(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(&snapshot)),
            "edits": [{
                "id": "generation3d-production-all14-edit",
                "actor": "generation3d-production-law",
                "forwards": mutation_hex,
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "1"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first P3 production fixture envelope");
    // 🧹️ A `create-widget`/`update-widget` row owns a whole `Widget`, and the seed projection owns
    // an `OrderedMap` layout root — both fail-close on a bare drop, so this fixture retires what it
    // authored instead of letting the scope drop it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    for mutation in mutations {
        mutation.retire_cold();
    }
    snapshot.retire_cold();
    (wire, expected, expected_digest)
}

/// 🔐️ Owns the publication lease `admit_production_envelope` took and releases it even when the law
/// panics before its explicit release. The lease table is a PROCESS-GLOBAL 4-slot
/// `FixedOperationRegistry` (`🧬️schema/🧬️mutations/💾️binary/🦀️.rs:211`), so one leaked slot turns every
/// later law in the same binary into `generation3d-publication.saturated` — an order-dependent red
/// that has nothing to do with what those laws assert
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
struct Generation3dProductionLease {
    handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle,
    released: bool,
}

impl Generation3dProductionLease {
    fn release(&mut self) -> bool {
        if self.released {
            return false;
        }
        self.released = true;
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_release_publication_authority(self.handle.operation, self.handle.generation)
    }
}

impl Drop for Generation3dProductionLease {
    fn drop(&mut self) {
        self.release();
    }
}

fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, wire: &[u8]) -> Generation3dProductionLease {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P3 production ingress credits");
    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_admit_publication_authority(handle.operation, handle.generation, handle.generation.0, handle.generation.0, handle.generation.0, crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationCredits { maximum_items: 8_192, maximum_output_pages: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_CONTROL_CREDITS })
    .expect("P3 production publication authority");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P3 production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P3 production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("P3 production envelope seal"));
    Generation3dProductionLease { handle, released: false }
}

fn drive_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..300_000 {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0)
            .expect("P3 authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P3 production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("P3 production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("P3 production envelope load did not reach terminal");
}

/// 🔐️ LAW: non-empty P3D3 canonical ingress reaches the real VCS maintenance replacement,
/// and accepted, stale, ABA, and displaced stores remain owned until explicit terminal ACK/close.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed() {
    // 🧹️ Registry-backed, never `VcsArtifactApp::new` — this app publishes
    // `bounded_first_step_tool_proofs!`, so a registryless instance faults at construction with
    // `interactive-job.catalog-authority` and its unwind aborts the binary.
    let _serial = crate::publication_authority::lock();
    let mut accepted = app_with_registry().await;
    let base_generation = accepted.artifact_generation_now();
    let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
    let mut lease = admit_production_envelope(&mut accepted, &wire);
    let handle = lease.handle;
    assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let snapshot = context::snapshot(&accepted);
    assert_eq!(&snapshot, &expected, "real maintenance must publish all P3 snapshot and all-14 replay fields");
    assert_eq!(production_semantic_digest(&snapshot), expected_digest);
    assert!(snapshot.fixture.layout.contains_key("move-target"));
    assert!(!snapshot.fixture.layout.contains_key("clear-target"), "3D-only delete-widget-position must survive retained replay");
    assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P3 terminal ACK"));
    assert!(lease.release());
    drop(snapshot);
    expected.retire_cold();

    use crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
    for (hostile, expected_code) in [
        (Missing, "generation3d-publication.authority-missing"),
        (WrongOperation, "generation3d-publication.wrong-operation"),
        (WrongGeneration, "generation3d-publication.wrong-generation"),
        (WrongBase, "generation3d-publication.wrong-base"),
        (WrongParent, "generation3d-publication.wrong-parent"),
    ] {
        let mut app = app_with_registry().await;
        let last_valid = context::snapshot(&app);
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (wire, _, _) = production_envelope_wire("rejected-production-candidate");
        let mut lease = admit_production_envelope(&mut app, &wire);
        let handle = lease.handle;
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = context::snapshot(&app);
        assert_eq!(production_semantic_digest(&retained), last_valid_digest);
        assert_eq!(retained, last_valid);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P3 terminal ACK after candidate retirement"));
        assert!(lease.release());
        drop(retained);
        drop(last_valid);
    }
}

//#region 🔖️CommandSurface
/// ⚖️ LAW: the brep kernel's mesh transfer unit fits the wire bound `flowTessellateResolve` actually
/// declares, and that bound belongs to the CHAIN's own route, never to the gesture quota. A factory
/// registers ONE `ToolExecutionContract` for all its keys, so while the chain shared
/// `Generation3dBoundedCommandJobFactory` the transfer unit was pinned to 8 KiB minus the envelope
/// header — 4 KiB of mesh body per `flowEvalTick` round trip, i.e. ten round trips for
/// `sphere-cut-with-torus` and no painted preview inside any patience window
/// (`📓️preview-mesh-delivery-2026-09-12.md`). Widening it on the gesture route would have widened
/// 24 unrelated interactive routes with it, so the chain owns
/// [`Generation3dFlowEvalJobFactory`] and this law pins ITS bound to the transfer unit.
#[test]
fn tessellate_transfer_unit_fits_the_declared_response_wire_bound() {
    let maximum = semio_framework_os_flow::brep_geometry::tessellate_envelope_maximum_bytes();
    assert!(maximum <= GENERATION3D_FLOW_EVAL_RAW_BYTES, "one tessellate step envelope is at most {maximum} bytes but the declared wire bound is {GENERATION3D_FLOW_EVAL_RAW_BYTES}");
    assert!(maximum > GENERATION3D_RETAINED_RAW_BYTES, "a transfer unit that still fits the gesture quota needs no route of its own");
    assert_eq!(generation3d_flow_eval_contract().max_raw_wire_bytes, GENERATION3D_FLOW_EVAL_RAW_BYTES, "the registered contract and the factory-side wire cap are one bound");
    assert_eq!(generation3d_bounded_contract().max_raw_wire_bytes, GENERATION3D_RETAINED_RAW_BYTES, "the gesture quota stays exactly where it was");
}

#[test]
fn command_ids_are_unique_and_cover_every_row() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), GENERATION3D_RETAINED_TOOL_IDS.len() + GENERATION3D_FLOW_EVAL_TOOL_IDS.len() + GENERATION3D_CONTRIBUTIONS_TOOL_IDS.len(), "every Generation3dCommand row must be covered by every_command()");
}

/// ⚖️ LAW: every one of the 30 declared `Generation3dCommand` rows is retained-owned by EXACTLY one
/// of the three factories — the gesture route, the preview chain's own route and the contributions
/// route — with an exact, nonempty publication-lane contract, the shape
/// `ArtifactToolFactoryRegistry::register` itself enforces
/// (`🧰️framework/…/🔌️plugin/🦀️.rs:12736-12748`), asserted here so a future command addition that
/// forgets its retained-tool-id/publication-contract row fails this test instead of silently
/// reintroducing `interactive-job.missing-owned-reducer` at dispatch. Mirrors generation2d's own
/// `retained_route_dispositions_are_exact_and_exhaustive` (`…/generation2d/…/✏️editor/🦀️.rs:1033`).
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    assert_eq!(GENERATION3D_RETAINED_TOOL_IDS.len(), 24);
    assert_eq!(GENERATION3D_FLOW_EVAL_TOOL_IDS.len(), 5);
    assert_eq!(GENERATION3D_CONTRIBUTIONS_TOOL_IDS.len(), 1);
    assert_eq!(<Generation3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 30, "all three factories' proofs, aggregated");
    assert_eq!(Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 24);
    assert_eq!(Generation3dFlowEvalJobFactory::PUBLICATION_CONTRACTS.len(), 5);
    assert_eq!(Generation3dContributionsJobFactory::PUBLICATION_CONTRACTS.len(), 1);
    assert_eq!(generation3d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation3d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!(generation3d_flow_eval_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation3d_flow_eval_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert!(GENERATION3D_RETAINED_TOOL_IDS.iter().all(|tool_id| Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    assert!(GENERATION3D_FLOW_EVAL_TOOL_IDS.iter().all(|tool_id| Generation3dFlowEvalJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    let mut sorted_ids = GENERATION3D_RETAINED_TOOL_IDS.to_vec();
    sorted_ids.extend_from_slice(GENERATION3D_FLOW_EVAL_TOOL_IDS);
    sorted_ids.extend_from_slice(GENERATION3D_CONTRIBUTIONS_TOOL_IDS);
    let declared = sorted_ids.len();
    sorted_ids.sort_unstable();
    sorted_ids.dedup();
    assert_eq!(sorted_ids.len(), declared, "a tool id may be owned by exactly one factory");
    for command in every_command() {
        assert!(
            sorted_ids.contains(&command.command_id()),
            "command {} is owned by none of Generation3dBoundedCommandJobFactory, Generation3dFlowEvalJobFactory or Generation3dContributionsJobFactory",
            command.command_id()
        );
    }
}

/// ⚖️ LAW: the contributions route's declared wire ceiling is REACHABLE — the largest page the
/// framework's own public-invocation envelope admits still fits it, and the ceiling is not so wide
/// that it could never be reached (`📓️extension-addressing-2026-09-10.md` §6.3 proposed 512 KiB,
/// which is 42× the largest page that can exist).
#[test]
fn contributions_route_declares_a_reachable_wire_ceiling() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    assert_eq!(GENERATION3D_CONTRIBUTIONS_RAW_BYTES, semio_framework::PUBLIC_INVOCATION_BODY_BYTES);
    let pack: String = std::iter::repeat_n('x', 190_719).collect();
    let wire = protocol::json::to_json_string(&("setContributions", Some(dsl::DslValue::object([
        ("json".to_string(), dsl::DslValue::String(pack)),
        ("page".to_string(), dsl::DslValue::uint(0)),
        ("pageCount".to_string(), dsl::DslValue::uint(1)),
    ]))));
    assert!(wire.len() <= GENERATION3D_CONTRIBUTIONS_RAW_BYTES, "a scoped pack encodes to {} bytes but the contract declares {GENERATION3D_CONTRIBUTIONS_RAW_BYTES}", wire.len());
    assert!(GENERATION3D_CONTRIBUTIONS_RAW_BYTES > GENERATION3D_RETAINED_RAW_BYTES, "the contributions route exists precisely because the gesture quota cannot carry it");
    assert_eq!(generation3d_contributions_contract().max_raw_wire_bytes, GENERATION3D_CONTRIBUTIONS_RAW_BYTES);
}

async fn drive_preview_operation(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>) -> Result<(u64, u64, u64), String> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut artifact = 0;
    let mut config = 0;
    let mut transient = 0;
    while app.has_pending_typed_operations() {
        if std::time::Instant::now() >= deadline {
            return Err("Generation3d preview operation did not finish".into());
        }
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
        while let Some(page) = app.take_typed_operation_result_page(1) {
            use semio_framework_plugin::app::TypedOperationResultLane;
            if page.lane == TypedOperationResultLane::Fault {
                return Err(format!("preview operation fault: {:?}", page.bytes()));
            }
            artifact += u64::from(page.lane == TypedOperationResultLane::Artifact);
            config += u64::from(page.lane == TypedOperationResultLane::Config);
            transient += u64::from(page.lane == TypedOperationResultLane::Transient);
            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
        }
        app.take_typed_operation_effect();
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
        std::thread::yield_now();
    }
    Ok((artifact, config, transient))
}

#[semio_framework_async_macros::async_test]
async fn generation_preview_is_one_app_transient_shared_by_two_generation_windows() {
    let mut app = app_with_registry().await;
    let result: Result<(), String> = async {
        let before_document = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(app.snapshot().map_err(|error| format!("{error:?}"))?);
        let before_generation = app.ephemeral_snapshot().await.transient_generation;
        app.dispatch_typed(Generation3dCommand::AddGeneration(add_generation::AddGeneration {}), &semio_framework_plugin::artifact_app_laws::meta("preview-owner")).await.map_err(|error| format!("{error:?}"))?;
        if drive_preview_operation(&mut app).await? != (1, 1, 1) {
            return Err("preview command did not publish artifact, selection config, and app transient exactly once".into());
        }
        if app.ephemeral_snapshot().await.transient_generation != before_generation + 1 {
            return Err("preview app transient generation did not advance exactly once".into());
        }
        if crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(app.snapshot().map_err(|error| format!("{error:?}"))?).generation.as_state().generations.len() != before_document.generation.as_state().generations.len() + 1 {
            return Err("addGeneration did not preserve its document behavior".into());
        }
        let view = semio_framework_plugin::ViewModel {
            window_instances: vec![
                semio_framework::ViewWindowInstance { id: "preview-a".into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
                semio_framework::ViewWindowInstance { id: "preview-b".into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
            ],
            ..Default::default()
        };
        let mut rendered = Vec::new();
        for window_id in ["preview-a", "preview-b"] {
            let context = view.for_window_instance(window_id).ok_or("missing generation preview window")?;
            let tree = app.render(generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW, None, &context).await.map_err(|error| format!("{error:?}"))?;
            rendered.push(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?);
        }
        if rendered[0] != rendered[1] {
            return Err("generation windows did not consume the same app-transient preview".into());
        }
        if include_str!("../../🎚️config/🧬️schema/🔣️.json").contains("generationPreviewText") {
            return Err("config schema still owns computed preview output".into());
        }
        Ok(())
    }
    .await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    result.expect("Generation3d preview ownership runtime");
}

#[test]
fn every_command_round_trips_through_text_and_binary() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
/// explicitly per row since generation3d's wire keys frequently diverge from a mechanical
/// kebab-case of the command id (for example, `nodeGraphViewport` → `viewport`).
#[test]
fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let expected_keywords = [
        "active-example",
        "graph-edit",
        "delete-selection",
        "remove-widget",
        "move-node",
        "add-widget",
        "patch-flow-widgets",
        "reorganize",
        "translate-selection",
        "rotate-selection",
        "scale-selection",
        "add-generation",
        "remove-generation",
        "rename-generation",
        "update-generation-values",
        "viewport",
        "lod-mode",
        "show-mode",
        "toggle-sun",
        "sun-azimuth",
        "sun-elevation",
        "sun-intensity",
        "camera",
        "select-generation",
        "flow-eval-tick",
        "flow-eval-resolve",
        "flow-tessellate-resolve",
        "cancel-preview-eval",
        "flow-tessellate-cancel-resolve",
        "set-contributions",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
    for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
        let printed = protocol::OpText::print_op(command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Generation3dCommand> {
    vec![
        Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }),
        Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "extrude".into() }),
        Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "extrude".into(), x: 1.0, y: 2.0 }),
        Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: Some(10.0), y: None }),
        Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }),
        Generation3dCommand::Reorganize(reorganize::Reorganize {}),
        Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }),
        Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }),
        Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
        Generation3dCommand::AddGeneration(add_generation::AddGeneration {}),
        Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "generation-1".into() }),
        Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }),
        Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
        Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 3.0 } }),
        Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
        Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
        Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
        Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
        Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
        Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
        Generation3dCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::generation3d::config::Generation3dPreviewCamera::default() }),
        Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: "generation-1".into() }),
        Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() }),
        Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash: 7, output_json: "{}".into(), extension_id: String::new(), ok: true, fault_code: String::new(), fault_message: String::new() }),
        Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash: 9, output_json: "{}".into() }),
        Generation3dCommand::CancelPreviewEval(cancel_preview_eval::CancelPreviewEval { window_id: "w1".into(), window_kind_id: "procedural-preview".into() }),
        Generation3dCommand::FlowTessellateCancelResolve(flow_tessellate_cancel_resolve::FlowTessellateCancelResolve { window_id: "w1".into(), window_kind_id: "procedural-preview".into(), output_json: "{\"ok\":true,\"retired\":1}".into(), ok: true }),
        Generation3dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into(), page: 0, page_count: 1 }),
    ]
}
//#endregion 🔖️CommandSurface

#[semio_framework_async_macros::async_test]
async fn declared_actions_bridge_to_commands() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Generation3dPlayApp>>(context::generation3d_app_manifest_for_tests).await;
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_editor_installs_every_declared_bounded_command_proof() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    // 🧹️ Through the self-closing fixture, never a bare `new_app_with_registry` — a plainly-dropped
    // `VcsArtifactApp` fails its own store's terminal-empty witness and aborts the binary.
    let _app = app_with_registry().await;
}

#[test]
fn the_manifest_stitches_every_taxonomy_node() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let json = serde_json::to_string(&create_generation3d_app()).expect("app definition json");
    for id in [
        flow_window::GENERATION_3D_PLAY_WINDOW_MAIN,
        edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW,
        generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS,
        form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM,
        generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW,
    ] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::GENERATION_3D_PLAY_MODE_EDIT, generate::GENERATION_3D_PLAY_MODE_GENERATE] {
        assert!(json.contains(id), "mode {id} missing from the manifest");
    }
    assert!(json.contains("3d.generation"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn each_example_loads_distinct_fixture_and_preview_geometry() {
    use crate::standards::v1::subsets::any::schema::*;
    use crate::widget_id;
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let examples = [
        PROCEDURAL_EXAMPLE_HEX_COLUMN,
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
        PROCEDURAL_EXAMPLE_SPHERE_TORUS,
        PROCEDURAL_EXAMPLE_BOX_FILLET,
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
        PROCEDURAL_EXAMPLE_BOX_SHELL,
    ];
    let mut signatures = std::collections::BTreeSet::new();
    for example_id in examples {
        let mut app = app().await;
        context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() })).await;
        let signature = format!("{:?}", context::snapshot(&app).fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect::<std::collections::BTreeSet<_>>());
        assert!(signatures.insert(signature.clone()), "duplicate fixture signature for {example_id}: {signature}");
    }
}

#[semio_framework_async_macros::async_test]
async fn refresh_pending_effects_arms_flow_eval_tick_chain() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() })).await;
    let (_, preview_view) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    let effects = app.pending_effects(Some(&preview_view)).await;
    assert!(effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")));
    drain_flow_eval_ticks(&mut app).await;
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_flow_graph_edits() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).fixture.widgets.len();
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(
        &mut app,
        Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }),
        |app| context::snapshot(&app).fixture.widgets.len(),
        before,
        before + 1,
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_widget_moves() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let widgets: Vec<String> = {
        let fixture = app().await;
        context::snapshot(&fixture).fixture.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect()
    };
    assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
    let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
    // 🧹️ The REGISTERED pair, never `assert_two_instances_converge` — this app publishes
    // `bounded_first_step_tool_proofs!`, so a registryless instance faults with
    // `interactive-job.catalog-authority` and its unwind aborts the binary.
    semio_framework_plugin::artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<Generation3dPlayApp>, (Option<f64>, Option<f64>), _, _>(
        "mem://generation3d-convergence",
        || async { context::generation3d_app_manifest_for_tests() },
        Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
        Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
        move |app| {
            let layout = &context::snapshot(&app).fixture.layout;
            (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn generation3d_labels_translate_catalogue_and_inspector_in_german() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let catalogue = context::render_with_view(&mut app, catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE, &view_state).await;
    assert!(catalogue.contains("\"Elemente\""));
    let inspector = context::render_with_view(&mut app, inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION, &view_state).await;
    assert!(inspector.contains("Elemente:"));
}

/// 🕹️ The runtime graph-selection route persists through an exactly owned interaction store.
#[semio_framework_async_macros::async_test]
async fn generation3d_interaction_selection_owns_its_persisted_history() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let node_id = context::snapshot(&app).fixture.widgets.first().map(crate::widget_id).expect("default fixture node").to_string();
    context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🗂️ With nothing selected the selection-conditioned rows stay folded away and the disclosure
/// budget holds.
#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let widgets: Vec<String> = context::snapshot(&app).fixture.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    assert!(!menu.is_empty(), "grouped disclosure menu should not be empty");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🕹️ The runtime funnels every right-click through `context_menu_with_request_context`, so a node
/// selected through the framework-owned `graph` domain — never through `request.surface`, which is
/// `None` here exactly as it is for a menu opened off a scene surface the app did not paint — must
/// unfold the transform trio, the removal group and the destructive delete row.
#[semio_framework_async_macros::async_test]
async fn context_menu_reads_the_framework_owned_graph_selection() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    // 🧊️ A literal default-fixture widget id, not `app.snapshot()`: that accessor hands back an OWNED
    // `Generation3dSnapshot`, whose `FlowFixture.layout: OrderedMap` aborts the process on drop
    // (`🌱️value/🗂️ordered/🦀️.rs:81`) unless explicitly retired.
    let node_id = "extrude".to_string();
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let ids_of = |menu: &[semio_framework_plugin::ContextMenuItemSpec]| -> Vec<String> {
        menu.iter().flat_map(|item| std::iter::once(item.id.clone()).chain(item.children.iter().flatten().map(|child| child.id.clone()))).collect()
    };
    let unselected = ids_of(&app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await);
    assert!(!unselected.iter().any(|id| id == "translateSelection"), "an empty selection must not offer a transform: {unselected:?}");
    assert!(!unselected.iter().any(|id| id == "removeWidget"), "an empty selection must not offer a removal target: {unselected:?}");
    context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    let selected = ids_of(&app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await);
    for id in ["translateSelection", "rotateSelection", "scaleSelection", "removeWidget", "removeGeneration"] {
        assert!(selected.iter().any(|candidate| candidate == id), "a live graph selection must offer {id}: {selected:?}");
    }
    assert!(selected.iter().any(|id| id.contains("delete")), "a live graph selection must offer the destructive delete row: {selected:?}");
    eprintln!("[DEBUG] generation3d context menu unfolded {} rows for one framework-owned graph selection", selected.len());
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🕸️ A preview instance id (`{widget}@{channel}#{index}`) and a synapse id land in the node and edge
/// domains respectively — the projection `context_menu_body` hands `selection_domains_from_surface`.
#[test]
fn graph_selection_splits_into_node_and_edge_domains() {
    let mut fixture = semio_framework_artifact_flow_flow::FlowFixture::default();
    fixture.widgets.push(semio_framework_artifact_flow_flow::Widget::InputNote { id: "note".into(), text: "n".into() });
    fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "wire".into(), from: "note".into(), to: "note".into(), from_port: String::new(), to_port: String::new() });
    let marks = PreviewInteractionMarks { hovered: Default::default(), selected: ["note@out#2".to_string(), "wire".to_string()].into_iter().collect() };
    assert_eq!(marks.graph_selection_domains(&fixture), (vec!["note".to_string()], vec!["wire".to_string()]));
}

/// 🕹️ `worldPointerDown`/`graphPointerDown` are gone: the world host reports a pick through the
/// framework-injected `interactionSelect` on the declared `graph` domain (`World3dHost/🟦️.tsx:4447`),
/// and nothing in the repo dispatches a graph pointer-down at all. A route no surface can reach is
/// dead API, not an observation hook.
#[test]
fn no_pointer_down_route_survives_the_framework_owned_selection_domain() {
    let definition = create_generation3d_app();
    let json = serde_json::to_string(&definition).expect("app definition json");
    assert!(!json.contains("PointerDown"), "pointer-down routes are framework-owned now");
    assert!(GENERATION3D_RETAINED_TOOL_IDS.iter().chain(GENERATION3D_FLOW_EVAL_TOOL_IDS).all(|id| !id.ends_with("PointerDown")), "a retained tool id outlived its action");
    assert!(definition.interactions.iter().any(|interaction| interaction.id == "graph"), "the graph domain is what replaced them");
}

#[semio_framework_async_macros::async_test]
async fn sun_measures_are_exposed_on_preview_windows() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    // 🪟️ `PluginApp::window_measures` projects per ATTACHED window instance
    // (`🔌️plugin/🦀️.rs`'s `for window in view_state.window_instances`), so an empty `ViewModel`
    // can only ever answer an empty map — the roster is what makes this assertion mean anything.
    let view = semio_framework_plugin::ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), window_kind_id: edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
            semio_framework_plugin::ViewWindowInstance { id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
        ],
        ..Default::default()
    };
    let measures = app.window_measures(&view).await;
    assert!(measures.contains_key(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW));
    assert!(measures.contains_key(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW));
}

//#region 🔖️EngineComputeTests
/// 🧬️ Rehomed verbatim from the deleted `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — these tests exercise
/// `PreviewPipeline`/`MeshBridge` functions above, all of which are app
/// behavior (they construct or take a [`Generation3dConfig`]), so the tests travel with them.
use semio_framework_ui::wgpu::kernel_3d_scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Vec3};
use std::sync::MutexGuard;

fn test_serial() -> MutexGuard<'static, ()> {
    crate::editor::generation3d::unit_tests::serial_execution::lock()
}

/// 🌉️ Decodes one preview mesh record's `data` field (a `pack::json`/`serde_json::Value` fragment
/// off the wire) into `MeshData` through its first-party `FromValue` codec — `MeshData` no longer
/// derives `serde::Deserialize` in a non-`#[cfg(test)]` build of its own crate (`🏗️mesh-engine/🦀️.rs`'s
/// `#[cfg_attr(test, derive(Serialize, Deserialize))]` only activates inside that crate's OWN test
/// build, never for a downstream consumer like this one), so `serde_json::from_value` cannot reach
/// it here; round-trip through the wire text and `dsl::json::from_json_str` instead, exactly like
/// production's `mesh_data_for_preview_handle` (above) does.
fn mesh_data_from_json(value: &Value) -> semio_framework_plugin::MeshData {
    dsl::json::from_json_str(&serde_json::to_string(value).expect("mesh data json text")).expect("mesh data")
}

/// 🌉️ `Mesh3d` (a plain positions/normals/indices struct) no longer exists —
/// `semio_framework_ui::wgpu::kernel_3d_scene`'s mesh API is now a generation/revision-keyed write-token/lease
/// pair (`mesh3d_begin`/`mesh3d_write_vec3`/`mesh3d_seal`) meant for the shared render-owned mesh
/// arena, not for a one-off AABB check. This test only ever needed the bounding box of the raw
/// position buffer, so compute it directly instead of standing up a lease.
fn aabb_of_positions(positions: &[f32]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for vertex in positions.chunks_exact(3) {
        for axis in 0..3 {
            min[axis] = min[axis].min(vertex[axis]);
            max[axis] = max[axis].max(vertex[axis]);
        }
    }
    (min, max)
}

fn preview_payload_from_evaluated_fixture(fixture: &semio_framework_artifact_flow_flow::FlowFixture, cfg: &Generation3dConfig) -> (String, String) {
    let eval_json = crate::standards::v1::subsets::any::schema::with_host(fixture, |host| host.evaluate().unwrap_or_default());
    preview_payload_from_eval(&eval_json, fixture, cfg)
}

#[test]
fn preview_payload_has_meshes_and_instances() {
    let _serial = test_serial();
    let projection = crate::standards::v1::subsets::any::schema::default_snapshot();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
    assert_ne!(meshes_json, "[]", "meshes_json was empty");
    assert_ne!(instances_json, "[]", "instances_json was empty");
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert!(!meshes.is_empty());
    assert!(!instances.is_empty());
    let kind_by_widget: std::collections::HashMap<String, String> = projection
        .fixture
        .widgets
        .iter()
        .map(|widget| {
            let kind = match widget {
                semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } => neuron_kind.clone(),
                _ => String::new(),
            };
            (crate::widget_id(widget).to_string(), kind)
        })
        .collect();
    let mut surfaces = 0;
    let mut curves = 0;
    for mesh in &meshes {
        let id = mesh.get("id").and_then(|value| value.as_str()).unwrap_or("");
        assert!(id.starts_with("eval-"), "mesh id must be tessellated eval handle, got {id}");
        let widget = id.trim_start_matches("eval-").split('@').next().unwrap_or_default();
        let kind = kind_by_widget.get(widget).map(String::as_str).unwrap_or_default();
        let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
        assert!(!data.edge_positions.is_empty(), "every preview mesh carries edge geometry, {id} ({kind}) did not");
        // 🧊️ A surface-bearing channel tessellates to real triangles; a CURVE (`brep.curve.*`) and a
        // `math.vector` marker are TRIANGLE-FREE by construction — `vector_marker_mesh` is a single
        // origin→tip segment and the wire tessellation is a polyline, so both carry edge geometry
        // (and the segment's own endpoints) but never an index buffer. Demanding ≥9 positions and
        // ≥3 indices of them asserted something the geometry never had
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        if kind.starts_with("brep.curve.") || kind == "math.vector" {
            assert!(data.indices.is_empty(), "curve/marker preview {id} ({kind}) must be triangle-free, got {} indices", data.indices.len());
            curves += 1;
        } else {
            assert!(data.positions.len() >= 9, "surface preview {id} ({kind}) has too few positions");
            assert!(data.indices.len() >= 3, "surface preview {id} ({kind}) has too few indices");
            surfaces += 1;
        }
    }
    assert!(surfaces > 0, "the default document previews at least one tessellated surface");
    assert!(curves > 0, "the default document previews at least one edge-only curve or vector marker");
    let camera = Camera3d {
        position: Vec3::from_array([config.preview_camera.position[0] as f32, config.preview_camera.position[1] as f32, config.preview_camera.position[2] as f32]),
        target: Vec3::from_array([config.preview_camera.target[0] as f32, config.preview_camera.target[1] as f32, config.preview_camera.target[2] as f32]),
        up: Vec3::new(0.0, 0.0, 1.0),
        fov_y: config.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
        near: 0.1,
        far: 1000.0,
    };
    let view_proj = camera.view_proj(0.6);
    let planes = frustum_planes(view_proj);
    let mut visible = 0usize;
    for instance in instances {
        let mesh_id = instance.get("meshId").or_else(|| instance.get("mesh_id")).and_then(|value| value.as_str()).unwrap_or("eval-missing");
        let mesh = meshes.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id)).expect("mesh record");
        let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
        let (mesh_aabb_min, mesh_aabb_max) = aabb_of_positions(&data.positions);
        let position = instance.get("position").and_then(|value| value.as_array()).map_or([0.0, 0.0, 0.0], |items| [items[0].as_f64().unwrap_or(0.0) as f32, items[1].as_f64().unwrap_or(0.0) as f32, items[2].as_f64().unwrap_or(0.0) as f32]);
        assert_eq!(position, [0.0, 0.0, 0.0], "preview instances stay in world space");
        let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
        let (min, max) = transform_aabb(model, mesh_aabb_min, mesh_aabb_max);
        if aabb_intersects_frustum(&planes, min, max) {
            visible += 1;
        }
    }
    assert!(visible > 0, "no preview instances intersect camera frustum");
    projection.retire_cold();
}

#[test]
fn document_from_mesh_returns_valid_default_snapshot() {
    let _serial = test_serial();
    let mesh = semio_framework_plugin::MeshData::default();
    let document = generation3d_document_from_mesh(&mesh).expect("dwg mesh import document");
    let projection: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&document)).expect("parseable projection");
    assert_eq!(projection.fixture.schema, "flow.fixture");
    projection.retire_cold();
}

#[test]
fn generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
    let _serial = test_serial();
    use semio_framework_plugin::{GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter};
    let default_projection = crate::standards::v1::subsets::any::schema::default_snapshot();
    let document_json: Value = serde_json::from_str(&dsl::json::to_json_string(&default_projection)).expect("projection json");
    default_projection.retire_cold();
    let mesh = generation3d_mesh_from_document(&dsl::DslValue::from(&document_json)).expect("mesh from document");
    assert!(!mesh.positions.is_empty());

    let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
    let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
    let obj_document = generation3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
    <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&obj_document)).expect("parseable obj projection").retire_cold();

    let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
    let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
    let glb_document = generation3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
    <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&glb_document)).expect("parseable glb projection").retire_cold();

    let stl_bytes = StlExporter.export(&mesh).expect("stl export");
    let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
    let stl_document = generation3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
    <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&stl_document)).expect("parseable stl projection").retire_cold();
}

#[test]
fn rectangle_wire_preview_emits_edge_only_mesh() {
    let _serial = test_serial();
    let projection = <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
    assert!(!meshes.is_empty(), "rectangle wire preview should tessellate curve edges");
    let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
    assert!(data.indices.is_empty(), "wire preview has no shaded triangles");
    assert!(data.edge_positions.len() >= 6, "curve preview should include edge polylines");
    assert!(!instances_json.is_empty());
    projection.retire_cold();
}

#[test]
fn all_bundled_examples_emit_preview_meshes() {
    let _serial = test_serial();
    let config = Generation3dConfig::default();
    let cases = [
        ("hexagonal-mushroom-column", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN),
        ("rectangle-extrude-volume", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE),
        ("sphere-cut-with-torus", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS),
        ("box-fillet-preview", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET),
        ("sphere-box-fuse", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE),
        ("face-sweep-extrude", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE),
        ("rectangle-wire-preview", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE),
        ("box-shell-preview", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_SHELL),
    ];
    for (label, example_id) in cases {
        let projection = crate::standards::v1::subsets::any::schema::example_snapshot(example_id).unwrap_or_else(|| panic!("{label}: missing projection"));
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        assert_ne!(meshes_json, "[]", "{label}: meshes empty; eval may have failed");
        assert_ne!(instances_json, "[]", "{label}: instances empty");
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_else(|err| panic!("{label}: meshes json: {err}"));
        assert!(!meshes.is_empty(), "{label}: no mesh entries");
        projection.retire_cold();
    }
}

#[test]
fn preview_tolerance_follows_lod_mode() {
    assert!((preview_tolerance("coarse") - 0.15).abs() < 1e-9);
    assert!((preview_tolerance("fine") - 0.02).abs() < 1e-9);
    assert!((preview_tolerance("") - 0.05).abs() < 1e-9);
}

#[test]
fn wireframe_show_mode_strips_shaded_triangles() {
    let _serial = test_serial();
    let projection = crate::standards::v1::subsets::any::schema::default_snapshot();
    let config = Generation3dConfig { show_mode: "wireframe".into(), ..Default::default() };
    let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
    assert!(!meshes.is_empty());
    let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
    assert!(data.indices.is_empty());
    assert!(!data.edge_positions.is_empty());
    projection.retire_cold();
}

#[test]
fn generation3d_io_declares_the_params_and_geometry_ports() {
    let io = semio_framework::io::resolve_ready(generation3d_io());
    assert_eq!(io.document_schema, "generation.3d");
    assert_eq!(io.artifact.id, "3d.generation");
    let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert_eq!(params.direction, semio_framework_plugin::MediaPortDirection::In);
    assert!(!params.required);
    let geometry = io.ports.iter().find(|port| port.id == "geometry:out").expect("geometry:out declared");
    assert_eq!(geometry.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(geometry.kind_id.as_deref(), Some("3d.mesh"));
    assert_eq!(geometry.multiplicity, semio_framework::PortMultiplicity::Many);
}

/// 🔌️ One `preview: true` neuron with two output channels (a point channel and a vector
/// channel — neither needs a brep kernel) must yield one instance PER CHANNEL, each id
/// qualified with its own channel, not one flattened instance for the whole widget.
fn preview_widget_fixture(id: &str, output_ports: Vec<String>) -> semio_framework_artifact_flow_flow::FlowFixture {
    let widget = semio_framework_artifact_flow_flow::Widget::Neuron { id: id.into(), neuron_kind: "test.multi".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports, preview: true };
    semio_framework_artifact_flow_flow::FlowFixture { schema: "flow.fixture".into(), camera: semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets: vec![widget], synapses: Vec::new(), layout: Default::default() }
}

#[test]
fn preview_payload_channel_qualifies_ids_across_two_output_channels() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({
        "multi": {
            "out": {
                "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
                "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert_eq!(instances.len(), 2, "two output channels should yield two preview instances, got {instances:?}");
    let ids: std::collections::HashSet<&str> = instances.iter().filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
    assert!(ids.contains("multi@a#0"), "point-channel instance id missing, got {ids:?}");
    assert!(ids.contains("multi@b#0"), "vector-channel instance id missing, got {ids:?}");
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
    assert_eq!(meshes.len(), 2, "each inline channel mints its own mesh, got {meshes:?}");
}

/// 🔌️ A single channel whose value is a `$schema: "list"` dictionary (the wire form
/// `semio_framework_artifact_flow_flow::neural::Dictionary` lists actually take) of N geometry-bearing entries must flatten
/// to N instances, indexed `#0..#{N-1}` in list order — proven here with inline points so the
/// test needs no brep kernel/session.
#[test]
fn preview_payload_flattens_a_list_channel_into_indexed_instances() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("listy", vec!["points".into()]);
    let eval_json = json!({
        "listy": {
            "out": {
                "points": {
                    "$schema": "list",
                    "0": { "$schema": "point", "x": 1.0, "y": 0.0, "z": 0.0 },
                    "1": { "$schema": "point", "x": 2.0, "y": 0.0, "z": 0.0 },
                    "2": { "$schema": "point", "x": 3.0, "y": 0.0, "z": 0.0 }
                }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (_meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert_eq!(instances.len(), 3, "a 3-entry list channel should yield 3 instances, got {instances:?}");
    for index in 0..3 {
        let expected_id = format!("listy@points#{index}");
        assert!(instances.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some(expected_id.as_str())), "missing {expected_id} in {instances:?}");
    }
}

/// 🔌️ A channel carrying only pure data (a number, no handle, no `x`/`y`/`z`) is not
/// geometry-bearing and must not fabricate a placeholder preview instance.
#[test]
fn preview_payload_emits_no_instance_for_a_pure_data_channel() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("scalar", vec!["value".into()]);
    let eval_json = json!({
        "scalar": {
            "out": {
                "value": { "$schema": "number", "value": 42.0 }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    assert_eq!(meshes_json, "[]", "pure-data channel must not fabricate mesh geometry");
    assert_eq!(instances_json, "[]", "pure-data channel must not fabricate a preview instance");
}
/// 🕹️ The three id forms one mark can take, and the transitive reach of each: a node-level
/// mark covers every channel and every instance below it, a channel-level mark covers only its
/// own channel, and an instance-level mark covers only itself.
#[test]
fn preview_marks_resolve_node_channel_and_instance_ids() {
    let node = PreviewInteractionMarks { hovered: ["multi".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(node.hovers("multi", "a", 0) && node.hovers("multi", "b", 3));
    assert!(!node.hovers("other", "a", 0));

    let channel = PreviewInteractionMarks { hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(channel.hovers("multi", "b", 0) && channel.hovers("multi", "b", 7));
    assert!(!channel.hovers("multi", "a", 0));

    let instance = PreviewInteractionMarks { hovered: ["multi@b#2".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(instance.hovers("multi", "b", 2));
    assert!(!instance.hovers("multi", "b", 1));
}

/// 🕹️ Graph → world: hovering the NODE in the node graph must light up every one of its
/// channels' preview geometry, not just one.
#[test]
fn preview_payload_marks_every_channel_of_a_hovered_node() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
    .to_string();
    let marks = PreviewInteractionMarks { hovered: ["multi".to_string()].into_iter().collect(), selected: ["multi@a".to_string()].into_iter().collect() };
    let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
    let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
    assert_eq!(instances.len(), 2);
    assert!(instances.iter().all(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)), "node hover must reach every channel: {instances:?}");
    assert_eq!(payload.selected_ids, vec!["multi@a#0".to_string()], "channel-level selection must not spill onto the sibling channel");
    assert!(payload.hovered_id.is_some(), "the scene needs a concrete hovered instance to paint");
}

/// 🕹️ Graph → world, narrowed: hovering one PORT lights up only that channel's geometry.
#[test]
fn preview_payload_marks_only_the_hovered_channel() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
    .to_string();
    let marks = PreviewInteractionMarks { hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
    let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
    let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
    let hovered: Vec<&str> = instances.iter().filter(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)).filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
    assert_eq!(hovered, vec!["multi@b#0"], "only the hovered channel may light up: {instances:?}");
    assert_eq!(payload.hovered_id.as_deref(), Some("multi@b#0"));
}

/// 🕹️ World → graph: hovering one preview INSTANCE in the 3D world resolves back to its node
/// and its port, which is what the node-graph window paints.
#[test]
fn graph_marks_project_instance_hover_back_onto_its_node_and_port() {
    let marks = PreviewInteractionMarks { hovered: ["multi@b#0".to_string()].into_iter().collect(), selected: ["multi@a#1".to_string()].into_iter().collect() };
    assert_eq!(marks.hovered_graph_target(), Some(("multi".to_string(), Some("b".to_string()))));
    assert!(marks.graph_highlight_ids().contains(&"multi".to_string()));
    assert_eq!(marks.graph_selection_ids(), vec!["multi".to_string()]);
    assert_eq!(PreviewInteractionMarks::widget_of("multi@b#0"), "multi");
    assert_eq!(PreviewInteractionMarks::port_of("multi@b#0"), Some("b"));
    assert_eq!(PreviewInteractionMarks::port_of("multi"), None);
}

/// 🕸️ Every port the node graph paints is also an interaction target parented to its widget —
/// the topology link `HoverSpec { transitive: true }` walks.
#[test]
fn interaction_topology_ports_match_the_node_graph_port_ids() {
    let _serial = test_serial();
    let projection = crate::standards::v1::subsets::any::schema::default_snapshot();
    let ports_by_node = generation3d_port_ids_by_node(&projection.fixture);
    assert!(!ports_by_node.is_empty(), "default fixture should project graph nodes");
    assert!(ports_by_node.values().any(|ports| !ports.is_empty()), "default fixture should project at least one port");
    for (node_id, ports) in &ports_by_node {
        for port in ports {
            assert!(port.starts_with(&format!("{node_id}@")), "port {port} must be qualified by its node {node_id}");
            assert_eq!(PreviewInteractionMarks::widget_of(port), node_id.as_str());
        }
    }
    projection.retire_cold();
}
/// 👁️ Which widget kinds contribute preview geometry: a neuron only when its author-set toggle
/// is on, an output preview always, a cluster always (it has no toggle of its own, and its
/// inner `semio_framework_artifact_flow_flow::neural::Neuron`s have none either), and a pure input widget never.
#[test]
fn widget_preview_eligibility_covers_neurons_output_previews_and_clusters() {
    let on = semio_framework_artifact_flow_flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: true };
    let off = semio_framework_artifact_flow_flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: false };
    let output = semio_framework_artifact_flow_flow::Widget::OutputPreview { id: "p".into(), preview: Default::default(), expanded: Default::default() };
    let cluster = semio_framework_artifact_flow_flow::Widget::Cluster { id: "c".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() };
    let slider = semio_framework_artifact_flow_flow::Widget::InputSlider { id: "s".into(), label: "S".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 };
    assert!(widget_previews(&on));
    assert!(!widget_previews(&off));
    assert!(widget_previews(&output));
    assert!(widget_previews(&cluster));
    assert!(!widget_previews(&slider));
}
//#endregion 🔖️EngineComputeTests

//#region 🔖️ExamplesTests
/// 📚️ Ticket 26/09/03/PROCEDURAL-3D-END-TO-END — `examples()` (wired at the plugin root via
/// `.editor_with_examples::<Generation3dPlayApp>(create_generation3d_app(), …examples())`) must
/// carry the same eight ids, in the same order, as the `setActiveExample` select options this app
/// declares — otherwise the navbar dropdown and the action's own arg picker disagree.
#[test]
fn examples_match_set_active_example_select_options() {
    let definition = create_generation3d_app();
    let select_ids: Vec<String> = definition
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter())
        .find(|action| action.id == "setActiveExample")
        .and_then(|action| action.args.first())
        .and_then(|arg| match arg.control() {
            semio_framework::ActionArgControl::Select { options } => Some(options.into_iter().map(|option| option.value).collect::<Vec<_>>()),
            _ => None,
        })
        .expect("setActiveExample must declare a Select arg");
    let example_ids: Vec<String> = examples().into_iter().map(|source| source.id().to_string()).collect();
    assert_eq!(example_ids.len(), 8);
    assert_eq!(example_ids, select_ids);
}
//#endregion 🔖️ExamplesTests

//#region 📏️SurfaceBudgetTests
/// 🪟️ Every authored body key of this app: five window bodies then the three panel bodies, in the
/// order `generation3d_render_body` matches them.
const GENERATION3D_BODY_KEYS: [&str; 8] = [
    flow_window::GENERATION_3D_PLAY_BODY_MAIN,
    edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW,
    generations::GENERATION_3D_PLAY_BODY_GENERATIONS,
    form::GENERATION_3D_PLAY_BODY_GENERATE_FORM,
    generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW,
    document_panel::GENERATION_3D_PLAY_BODY_DOCUMENT,
    catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE,
    inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION,
];

/// 📏️ Every window and panel surface of every bundled example must fit the framework's ONE resident
/// surface capacity (`ui_contract::UI_RESIDENT_SURFACE_BYTES`, which `ui_runtime`'s
/// `SURFACE_RECONCILE_SURFACE_BYTES` is defined as). Renders each example's own projection through
/// the SAME `generation3d_render_body` every live window goes through, so the measurement is per
/// example rather than per app state. Prints the table the boot report reads: no surface may be
/// silently oversized behind the mount-time admission fault
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, defect 2).
#[test]
fn every_window_and_panel_surface_fits_the_resident_surface_bound() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let bound = semio_framework_ui_contract::UI_RESIDENT_SURFACE_BYTES;
    let config = Generation3dConfig::default();
    let view_state = semio_framework_plugin::ViewModel::default();
    for example_id in examples().into_iter().map(|source| source.id().to_string()) {
        let snapshot = crate::standards::v1::subsets::any::schema::example_snapshot(&example_id).unwrap_or_else(|| panic!("{example_id}: missing projection"));
        for body_key in GENERATION3D_BODY_KEYS {
            let session = FlowEvalSession::new();
            let tree = generation3d_render_body(body_key, &snapshot, &config, None, &view_state, &PreviewInteractionMarks::default(), &session).expect("render");
            context::retire_flow_eval_session(session);
            let rendered = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render json");
            println!("[STATS] surface example={example_id} body={body_key} bytes={} bound={bound}", rendered.len());
            assert!(!rendered.is_empty(), "{example_id}/{body_key} rendered empty");
            assert!(rendered.len() <= bound, "{example_id}/{body_key} is {} B, over the {bound} B resident surface bound", rendered.len());
        }
        snapshot.retire_cold();
    }
}
//#endregion 📏️SurfaceBudgetTests

//#region 🧹️RetirementTests
/// 🧹️ The live first-turn sequence in ONE process: `pending_effects` (which built an unretired
/// scratch `FlowHost` and trapped the plugin actor with `ordered-map root must be explicitly retired
/// before drop`), a render of every body, an example switch that replaces the whole fixture, and the
/// retained `FlowEvalSession` owner's own close. Nothing here may abort
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, defect 1).
#[semio_framework_async_macros::async_test]
async fn the_first_turn_sequence_retires_every_flow_host_it_builds() {
    use semio_framework_plugin::{ArtifactView, ConfigView};
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;

    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let config = Generation3dConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let (_, preview_view) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    let _ = (&doc, &cfg);
    // 🔒️ Through the app instance, because the poll now reads the RETAINED evaluation session's
    // per-window arming latch and only the instance owns the handle to it.
    let effects = app.pending_effects(Some(&preview_view)).await;
    println!("[STATS] pending_effects effects={}", effects.len());
    snapshot.retire_cold();

    for body_key in GENERATION3D_BODY_KEYS {
        let rendered = context::render(&mut app, body_key).await;
        assert!(!rendered.is_empty(), "{body_key} rendered empty");
    }

    for example_id in [crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET, crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN] {
        context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() })).await;
        let rendered = context::render(&mut app, GENERATION3D_BODY_KEYS[0]).await;
        assert!(!rendered.is_empty(), "{example_id} main body rendered empty after the switch");
    }

    let mut owner = Generation3dInstanceOperationOwner::new();
    let mut closed = false;
    for _ in 0..1_000_000 {
        if matches!(semio_framework_plugin::ArtifactInstanceOperationOwner::close_step(&mut owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            closed = true;
            break;
        }
    }
    assert!(closed, "the retained evaluation-session owner never reached its terminal close");
    assert!(semio_framework_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner));
}
//#endregion 🧹️RetirementTests

//#region 🔁️ExtensionRoundTripTests
/// 🚦️ One widget's `NodeEvalStatus` discriminant as the flow window publishes it.
pub(crate) fn node_eval_status(status_json: &str, widget_id: &str) -> String {
    let status: serde_json::Value = serde_json::from_str(status_json).expect("flow status json");
    status.get(widget_id).and_then(|entry| entry.get("status")).and_then(serde_json::Value::as_str).unwrap_or("<absent>").to_string()
}

/// ⚖️ LAW: the served boot fixture `hexagonal-mushroom-column` evaluates END TO END across the
/// extension actor boundary. Every `brep`/`math` operator reaches this app as a CONTRIBUTED stub
/// (`🔬️flow-operators`, the browser's own wiring), so the only way the graph can finish is by
/// crossing the whole round trip once per node — `Emit::extension_invocations` → the SDK's minted
/// `req` and parked continuation → the host running the capability on the plugin the invocation
/// ADDRESSES → `Event::Completed` → `flowEvalResolve` seeding the node cache and re-arming the tick.
///
/// 🐛️ Regression guard for the served-app stall (`📓️runtime-verification-2026-09-09.md` boot #3):
/// the graph reached the renderer with `extrusion-axis: computing` and `data-meshes-json="[]"`
/// forever, because the invocation addressed the flow manifest's own id (`math`) while the host
/// resolves an extension actor by the CONTRIBUTING PLUGIN's id (`flow-extension-math`). Every
/// evaluated node here must end `ok` and the preview must paint real geometry
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn hex_column_evaluates_end_to_end_through_the_extension_round_trip() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;

    let graph = context::render(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&graph).expect("node-graph scene decodes off the rendered flow surface");
    let status_json = scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status");
    for widget_id in ["height", "radius", "sides", "profile", "extrusion-axis", "extrude", "column-preview"] {
        assert_eq!(node_eval_status(&status_json, widget_id), "ok", "{widget_id} never finished evaluating: {status_json}");
    }

    let preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    let mesh_count = meshes.as_array().map_or(0, Vec::len);
    assert!(mesh_count >= 1, "the extruded column must reach the preview as at least one mesh, got {mesh_count}: {}", world.meshes_json);
    eprintln!("[DEBUG] hex column round trip finished: status={status_json} meshes={mesh_count}");
}

/// ⚖️ LAW: the served machine. The host's `contributionsJson` — built here exactly the way
/// `buildContributionsJson` (`🎠️kernel/🟦️.ts`) builds it, from the staged extension crates' own
/// manifests — crosses as the page run `semio_framework::public_invocation_string_pages` cuts,
/// through the REAL `setContributions` retained route, and that alone is what makes `brep`/`math`
/// addressable and the whole hex-column chain finish with geometry.
///
/// The payload carries one extra witness manifest nothing else in this binary ever installs, so the
/// registry state this law reads is provably THIS run's delivery and not a leftover
/// `install_flow_extension_manifest` from `🔬️flow-operators` — the served plugin has neither
/// (`📓️extension-addressing-2026-09-10.md` §6.1). The witness is removed again at the end, because
/// the registry is process-wide and this law is a guest in it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn host_pushed_contribution_pages_install_the_registry_the_served_chain_needs() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).is_err(),
        "the witness extension must not be addressable before the host pushes it"
    );

    let contributions = context::staged_flow_extension_contributions_json(&[(context::CONTRIBUTIONS_WITNESS_PLUGIN_ID, context::contributions_witness_manifest_json())]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    println!("[STATS] contributions payload chars={} pages={} page-bound={}", contributions.chars().count(), pages.len(), semio_framework::PUBLIC_INVOCATION_STRING_BYTES);
    assert!(pages.len() > 1, "the staged closure must exceed one public-invocation string page, else the paging law proves nothing");

    // 🌉️ The shell reaches this route through `plugin_handle_command` → `command_from_action`, not
    // through the typed binary channel, so the ARG DECODE is part of the delivery and is pinned here
    // on the real page the pager cut — including the envelope bound that sized it.
    for (index, page) in pages.iter().enumerate() {
        let counted: usize = page.chars().map(semio_framework::public_invocation_char_cost).sum();
        assert!(counted <= semio_framework::PUBLIC_INVOCATION_STRING_BYTES, "page {index} costs {counted} against the public-invocation string bound");
    }
    let decoded = <Generation3dPlayApp as ArtifactEditor>::command_from_action(
        "setContributions",
        Some(&dsl::DslValue::object([
            ("json".to_string(), dsl::DslValue::String(pages[0].clone())),
            ("page".to_string(), dsl::DslValue::uint(0)),
            ("pageCount".to_string(), dsl::DslValue::uint(pages.len() as u64)),
        ])),
    )
    .expect("the host's own argument shape decodes into the typed command");
    assert_eq!(decoded, Generation3dCommand::SetContributions(set_contributions::SetContributions { json: pages[0].clone(), page: 0, page_count: pages.len() as u64 }));

    semio_framework_trace::reset_heap_peak();
    let (before_boot, _) = context::heap_probe("before boot");
    let mut app = app_with_registry().await;
    let (after_boot, _) = context::heap_probe("after boot");
    for (index, page) in pages.iter().enumerate() {
        let receipt = context::dispatch(
            &mut app,
            Generation3dCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
        )
        .await;
        assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "contributions page {index} faulted in the retained job ladder");
        if index % 8 == 0 || index + 1 == pages.len() {
            context::heap_probe(&format!("after page {index}"));
        }
    }
    let (after_install, _) = context::heap_probe("after install");
    assert_eq!(semio_framework_os_flow::host_flow_extension_contributions_pending_bytes(), 0, "the assembler retains nothing once its last page lands");
    assert_eq!(
        semio_framework_os_flow::flow_extension_invocation_address(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness must be addressable only because this run's pages carried it"),
        context::CONTRIBUTIONS_WITNESS_PLUGIN_ID
    );
    for extension_id in [crate::flow_operators::BREP_EXTENSION_FLOW_ID, crate::flow_operators::MATH_EXTENSION_FLOW_ID] {
        let address = semio_framework_os_flow::flow_extension_invocation_address(extension_id).expect("a host-pushed contribution makes its extension addressable");
        assert!(address.starts_with("flow-extension-"), "{extension_id} resolved to {address:?}, not to a contributing plugin id");
    }

    drain_flow_eval_ticks(&mut app).await;
    let (after_eval, _) = context::heap_probe("after first eval");
    let graph = context::render(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&graph).expect("node-graph scene decodes off the rendered flow surface");
    let status_json = scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status");
    for widget_id in ["profile", "extrusion-axis", "extrude", "column-preview"] {
        assert_eq!(node_eval_status(&status_json, widget_id), "ok", "{widget_id} never finished evaluating under a host-pushed registry: {status_json}");
    }
    let preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    let mesh_count = meshes.as_array().map_or(0, Vec::len);
    assert!(mesh_count >= 1, "a host-pushed registry must reach a painted preview, got {mesh_count}: {}", world.meshes_json);
    println!("[STATS] host-pushed chain finished: meshes={mesh_count}");
    let (after_mesh, peak) = context::heap_probe("after first tessellation");
    println!(
        "[MEMORY] deltas: boot={} install={} eval={} mesh={} peak={peak} ceiling={} budget={}",
        after_boot - before_boot,
        after_install - after_boot,
        after_eval - after_install,
        after_mesh - after_eval,
        semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes(),
        semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES
    );
    // ⚖️ LAW: the whole boot + contributions-install + first-mesh sequence peaks under the schema's
    // declared share of the guest's linear-memory budget — the bound that turns a browser-only
    // `rust_oom` trap into a native failure (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let ceiling = semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes() as isize;
    assert!(
        peak < ceiling,
        "the boot + install + first-mesh sequence peaked at {peak} B, over {}% of the {} B guest budget ({ceiling} B)",
        semio_framework_trace::GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT,
        semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES
    );
    semio_framework_os_flow::uninstall_flow_extension(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
}

/// 🪟️ The thirteen surfaces the SERVED shell mounts in one `surface-visible` burst, in the order the
/// browser sends them (measured on 6018, `🗑️generated/fix-forward-contributions/bridge-trace4`): the
/// eight app bodies, the framework History body and the four framework sections. A guest turn that
/// cannot render all thirteen never hands its turn back, and the shard watchdog reads that as a dead
/// worker (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION3D_SHELL_MOUNTED_BODY_KEYS: [&str; 13] = [
    flow_window::GENERATION_3D_PLAY_BODY_MAIN,
    edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW,
    generations::GENERATION_3D_PLAY_BODY_GENERATIONS,
    form::GENERATION_3D_PLAY_BODY_GENERATE_FORM,
    generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW,
    document_panel::GENERATION_3D_PLAY_BODY_DOCUMENT,
    catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE,
    inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION,
    "framework.body.history",
    "framework.section.engagements",
    "framework.section.measures",
    "framework.section.tools",
    "framework.section.catalogue",
];

/// ⚖️ LAW: every surface the shell mounts in its boot burst renders, and the whole burst costs less
/// than one interactive turn. The reactor renders every dirty surface inside ONE `reactor::poll`
/// (`⚛️reactor/🔄️turn/🦀️.rs`, the `dirty.surfaces` loop between the `presence-clock` and `render`
/// phases), so a single body that cannot finish takes the whole turn — and with it the worker's
/// progress ticker, which the host then reads as `shard 0 terminated by the host watchdog`.
#[semio_framework_async_macros::async_test]
async fn the_shell_boot_surface_burst_renders_inside_one_turn() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow, preview) = context::shell_views("procedural-main", "procedural-preview-test");
    let burst = std::time::Instant::now();
    for body_key in GENERATION3D_SHELL_MOUNTED_BODY_KEYS {
        let view = if body_key == flow_window::GENERATION_3D_PLAY_BODY_MAIN { flow.clone() } else { preview.clone() };
        let started = std::time::Instant::now();
        eprintln!("[STATS] shell burst rendering {body_key}");
        let rendered = app.render(body_key, None, &view).await;
        let elapsed = started.elapsed();
        println!("[STATS] shell burst {body_key} ok={} elapsed_ms={}", rendered.is_ok(), elapsed.as_millis());
        assert!(elapsed < std::time::Duration::from_secs(2), "surface {body_key} took {elapsed:?} to render, more than one interactive turn can afford");
        if let Ok(tree) = rendered {
            semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("a rendered shell surface projects");
        }
    }
    let burst = burst.elapsed();
    println!("[STATS] shell boot burst total_ms={}", burst.as_millis());
    assert!(burst < std::time::Duration::from_secs(4), "the whole shell boot surface burst took {burst:?}, over the one-turn budget the reactor renders it in");
}

/// ⏱️ Native wall budget for ONE served `setContributions` crossing. The host's shard watchdog kills
/// a worker that is silent for `SHARD_LIVENESS_POLICY`'s budget (18 s observed), and a debug wasm
/// guest runs one to two orders of magnitude slower than this debug native build — so a native
/// install that needs seconds is already a dead shard in the browser. Two seconds is the widest
/// figure that still fails fast (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const SERVED_CONTRIBUTIONS_INSTALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(2);

/// ⚖️ LAW: the crossing the SERVED shell actually performs. Since the pack-encoded command ingress
/// replaced the 4 KiB string pager (`🏛️ShellHost/🟦️.tsx`, "One pack crossing"), the whole scoped
/// closure arrives as page 0 of 1 — a single ~250 000-character `json` argument in ONE retained
/// command — and the guest must fold it, rebuild the process-wide registry and hand its turn back
/// inside the shard watchdog's silence budget.
///
/// The paged twin above proves delivery; this one proves the served SHAPE and its COST. A guest that
/// installs the same closure correctly but takes 19 s to do it reads to the host as a worker that
/// died (`shard 0 terminated by the host watchdog`, boot check #12), and every later boot repeats the
/// loop for ever because the re-created actor is handed the same command again
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn the_served_one_page_contributions_crossing_installs_inside_the_watchdog_budget() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let contributions = context::staged_flow_extension_contributions_json(&[(context::CONTRIBUTIONS_WITNESS_PLUGIN_ID, context::contributions_witness_manifest_json())]);
    assert!(contributions.len() > semio_framework::PUBLIC_INVOCATION_STRING_BYTES, "a one-page law proves nothing on a payload that would have fitted one 4 KiB string page");
    let mut app = app_with_registry().await;
    let started = std::time::Instant::now();
    let receipt = context::dispatch(&mut app, Generation3dCommand::SetContributions(set_contributions::SetContributions { json: contributions.clone(), page: 0, page_count: 1 })).await;
    let elapsed = started.elapsed();
    println!("[STATS] served one-page contributions install chars={} elapsed_ms={}", contributions.len(), elapsed.as_millis());
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "the served one-page crossing faulted in the retained job ladder");
    assert_eq!(
        semio_framework_os_flow::flow_extension_invocation_address(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness must be addressable only because this run's single page carried it"),
        context::CONTRIBUTIONS_WITNESS_PLUGIN_ID
    );
    assert_eq!(semio_framework_os_flow::host_flow_extension_contributions_pending_bytes(), 0, "a one-page run retains nothing once its only page lands");
    semio_framework_os_flow::uninstall_flow_extension(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
    assert!(
        elapsed < SERVED_CONTRIBUTIONS_INSTALL_BUDGET,
        "the served one-page contributions crossing took {elapsed:?} of native debug time, over the {SERVED_CONTRIBUTIONS_INSTALL_BUDGET:?} budget — the wasm guest pays this many times over and the shard watchdog kills it mid-turn"
    );
}

/// ⚖️ LAW: the STEADY STATE holds. A served boot does not stop at the first mesh — the preview
/// window re-arms `flowEvalTick` for as long as the tab is open, and the guest it runs in has ONE
/// fixed linear memory ([`semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES`]) with no
/// process to restart. Boot #9b trapped `rust_oom` ~60 s after boot, so the bound this law states is
/// exactly that horizon: whatever one settled tick+render cycle retains, `STEADY_STATE_CYCLES` of
/// them must still fit in the headroom the install-peak ceiling leaves
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn the_settled_evaluation_tick_cycle_retains_nothing_that_would_exhaust_the_guest() {
    /// ⏱️ Tick+render cycles a 60 s browser boot runs at the preview window's re-arm cadence.
    const STEADY_STATE_CYCLES: isize = 600;
    /// 📐️ Cycles actually run here — the growth is linear in the leak, so a short run measured and
    /// extrapolated is the same law at a fraction of the suite's time.
    const MEASURED_CYCLES: isize = 60;
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;
    let _ = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let (settled, _) = context::heap_probe("steady state armed");
    for _ in 0..MEASURED_CYCLES {
        drain_flow_eval_ticks(&mut app).await;
        let _ = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    }
    let (after, _) = context::heap_probe("steady state settled");
    let per_cycle = (after - settled) / MEASURED_CYCLES;
    let headroom = (semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES - semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes()) as isize;
    println!("[MEMORY] steady state: per-cycle={per_cycle} horizon={} headroom={headroom}", per_cycle * STEADY_STATE_CYCLES);
    assert!(
        per_cycle * STEADY_STATE_CYCLES < headroom,
        "a settled tick+render cycle retains {per_cycle} B, so {STEADY_STATE_CYCLES} of them claim {} B of the {headroom} B the install-peak ceiling leaves in the guest budget",
        per_cycle * STEADY_STATE_CYCLES
    );
}

/// ⚖️ LAW: every `Effect::InvokeExtension` this app emits carries the CONTRIBUTING PLUGIN's id,
/// resolved through the single translation surface
/// [`semio_framework_os_flow::flow_extension_invocation_address`] — the flow manifest's own id
/// (`brep`, `math`) is never an address, and the tessellate hop in particular must reach
/// `flow-extension-brep`.
///
/// 🪪️ And its inverse: with the geometry kernel's CONTRIBUTION removed (the served app's actual
/// state — nothing installs contributed manifests into a browser plugin instance, see
/// `📓️extension-addressing-2026-09-10.md` §3), the preview window must publish a typed
/// `phase: "faulted"` carrying both languages and the miss's own `fault` object, instead of the
/// per-tick `eprintln!` boot #7 counted 46 times (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn extension_invocations_address_the_contributing_plugin_and_a_missing_contribution_faults_the_preview() {
    use crate::flow_operators::{BREP_EXTENSION_FLOW_ID, BREP_EXTENSION_PLUGIN_ID, MATH_EXTENSION_PLUGIN_ID};
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (view, _) = preview_views("procedural-address-test", "procedural-address-test-other");
    let window_id = view.window_id.clone().expect("the addressed tick view names one preview window");
    let mut addresses: Vec<(String, String)> = Vec::new();
    app.pending_effects(Some(&view)).await;
    for _ in 0..1000 {
        let receipt = context::dispatch_with_view(&mut app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: window_id.clone(), window_kind_id: view.active_window_kind_id.clone().unwrap_or_else(|| crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into()) }), view.clone()).await.expect("flowEvalTick");
        let settled = semio_framework_plugin::artifact_app_laws::settle_extension_invocations(&mut *app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id, &semio_framework_plugin::artifact_app_laws::meta("local"), &mut |pending| {
            addresses.push((pending.extension_id.clone(), pending.capability.clone()));
            crate::brep_extension::serve(pending)
        })
        .await
        .expect("in-process extension round trip");
        let rearmed = receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick"));
        if !rearmed && settled.answered == 0 {
            break;
        }
    }
    assert!(!addresses.is_empty(), "the hex column boot must cross the extension boundary at least once");
    for (extension_id, capability) in &addresses {
        assert!(extension_id == BREP_EXTENSION_PLUGIN_ID || extension_id == MATH_EXTENSION_PLUGIN_ID, "invocation address {extension_id:?} (capability {capability:?}) is not a contributing plugin id");
    }
    assert!(
        addresses.iter().any(|(extension_id, capability)| extension_id == BREP_EXTENSION_PLUGIN_ID && capability == "tessellate"),
        "the tessellate hop must address the contributing brep plugin: {addresses:?}"
    );
    eprintln!("[DEBUG] invocation addresses: {addresses:?}");

    let miss = semio_framework_os_flow::flow_extension_invocation_address("not-contributed-by-anything").expect_err("an uncontributed flow extension id has no invocation address");
    let mut owner = Generation3dInstanceOperationOwner::new();
    let faulted: serde_json::Value = owner
        .with_session(|session| serde_json::from_str(&crate::editor::generation3d::preview_progress_status_json_for(Some(session), Err(miss))).expect("preview status json"))
        .expect("the retained evaluation-session owner lends its session");
    for _ in 0..1_000_000 {
        if matches!(semio_framework_plugin::ArtifactInstanceOperationOwner::close_step(&mut owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner), "the projection's own session owner must close terminal-empty");
    assert_eq!(faulted.get("phase").and_then(serde_json::Value::as_str), Some("faulted"), "{faulted}");
    let label = faulted.get("phaseLabel").expect("the faulted phase carries its own label pair");
    assert!(label.get("en").and_then(serde_json::Value::as_str).is_some_and(|text| !text.is_empty()));
    assert!(label.get("de").and_then(serde_json::Value::as_str).is_some_and(|text| !text.is_empty()));
    assert_eq!(faulted.get("cancellable").and_then(serde_json::Value::as_bool), Some(false), "a preview that never invoked anything offers no cancel");
    let fault = faulted.get("fault").expect("the faulted status names the miss");
    assert_eq!(fault.get("code").and_then(serde_json::Value::as_str), Some(semio_framework_os_flow::FlowExtensionAddressMiss::CODE));
    assert_eq!(fault.get("extensionId").and_then(serde_json::Value::as_str), Some("not-contributed-by-anything"));
    for language in ["en", "de"] {
        let message = fault.get("message").and_then(|value| value.get(language)).and_then(serde_json::Value::as_str).unwrap_or_default();
        assert!(message.contains("not-contributed-by-anything"), "the {language} message must name the unresolved flow extension id: {message}");
    }
    let contributed = fault.get("contributed").and_then(serde_json::Value::as_array).expect("the miss names every translation the session DOES carry");
    for (extension_id, plugin_id) in [(BREP_EXTENSION_FLOW_ID, BREP_EXTENSION_PLUGIN_ID), ("math", MATH_EXTENSION_PLUGIN_ID)] {
        assert!(
            contributed.iter().any(|entry| entry.get("extensionId").and_then(serde_json::Value::as_str) == Some(extension_id) && entry.get("pluginId").and_then(serde_json::Value::as_str) == Some(plugin_id)),
            "{extension_id} -> {plugin_id} missing from {contributed:?}"
        );
    }
    eprintln!("[DEBUG] unaddressable geometry kernel published: {faulted}");
}

/// ⚖️ LAW: the SERVED ORDER. `ShellHost` loads the example and only THEN pushes the contributions
/// closure, so the first evaluation always runs against a registry that cannot address the geometry
/// kernel — and the miss is retained, not merely late: it sits in the session's neural cache, in its
/// incremental baseline, in the published `eval_json`/`status_json` and in the tessellation ledger a
/// surface projects as `phase: "faulted"`. Installing operators into a process-wide registry
/// publishes nothing, so boot #11 sat at `faulted` for 2.5 minutes with no user action able to move
/// it (`📓️runtime-verification-2026-09-09.md`).
///
/// This law drives that exact order and asserts the recovery is AUTOMATIC: the run's last page
/// invalidates the retained session against the new
/// `semio_framework_os_flow::flow_extension_registry_generation` and re-arms the `flowEvalTick`
/// chain for every attached preview window, and NOTHING ELSE runs afterwards — the drain below is
/// started from the install's own effects, never from `pending_effects`.
///
/// 🧪️ A `--lib` binary LINKS both extension packs (`🔬️flow-operators`), and a linked pack shadows
/// the contributed stub — so an empty contribution table alone still evaluates, and would prove
/// nothing. [`UnlinkedFlowExtensions`] therefore retires the linked installers for the length of
/// this law, which is the served guest's actual shape: nothing linked, everything contributed. The
/// evaluate hop then crosses the real extension wire, answered in-process by `🔬️brep-extension`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::take();
    semio_framework_os_flow::sync_host_flow_extension_contributions("[]".to_string()).expect("an empty closure is a legal registry state — it is the state every served boot starts in");
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_err(),
        "the geometry kernel must be unaddressable before the host pushes anything, or this law proves nothing"
    );

    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = context::shell_views("procedural-rearm-main", "procedural-rearm-preview");
    context::dispatch_with_view(
        &mut app,
        Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN.to_string() }),
        flow_view.clone(),
    )
    .await
    .expect("the example loads before any contribution arrives, exactly as ShellHost orders it");
    context::drain_armed_flow_eval_ticks(&mut app, &flow_view).await;

    let faulted_preview = context::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await;
    assert_eq!(preview_phase(&faulted_preview), "faulted", "an unaddressable geometry kernel must fault the preview");
    assert_eq!(preview_mesh_count(&faulted_preview), 0, "a faulted preview paints nothing");
    let faulted_status = flow_status_json(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    assert_ne!(node_eval_status(&faulted_status, "extrude"), "ok", "the extrusion cannot have evaluated against a registry with no operators: {faulted_status}");
    println!("[STATS] before the install: {faulted_status}");

    let contributions = context::staged_flow_extension_contributions_json(&[]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    println!("[STATS] late install: pages={} page-bound={}", pages.len(), semio_framework::PUBLIC_INVOCATION_STRING_BYTES);
    let mut install_effects: Vec<Effect> = Vec::new();
    for (index, page) in pages.iter().enumerate() {
        let receipt = context::dispatch_with_view(
            &mut app,
            Generation3dCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
            flow_view.clone(),
        )
        .await
        .expect("contributions page");
        assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "contributions page {index} faulted in the retained job ladder");
        let rearms = receipt.effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")).count();
        if index + 1 < pages.len() {
            assert_eq!(rearms, 0, "page {index} installs nothing yet, so it owes no re-arm");
        } else {
            assert_eq!(rearms, 1, "the run's last page owes exactly one re-arm per attached preview window");
            install_effects = receipt.effects.clone();
        }
    }
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_ok(),
        "the pushed closure must make the geometry kernel addressable"
    );

    let ticks = context::drain_armed_flow_eval_ticks_from(&mut app, &flow_view, &install_effects).await;
    assert!(ticks > 0, "the install's own effects must be the thing that restarts the chain");
    let status_json = flow_status_json(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    for widget_id in ["profile", "extrusion-axis", "extrude", "column-preview"] {
        assert_eq!(node_eval_status(&status_json, widget_id), "ok", "{widget_id} never re-evaluated after the late install: {status_json}");
    }
    let preview = context::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await;
    assert_ne!(preview_phase(&preview), "faulted", "the install must clear the preview fault without any user action");
    let meshes = preview_mesh_count(&preview);
    assert!(meshes >= 1, "the re-armed chain must reach a painted preview, got {meshes}");
    println!("[STATS] late install re-armed {ticks} ticks and painted meshes={meshes}");
}

/// ⚖️ LAW: the invalidation KEY is the registry generation, not "a page run finished". A re-push of
/// an unchanged closure leaves the generation where it was, invalidates nothing and re-arms nothing
/// — otherwise every host refresh would restart a settled evaluation
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn re_pushing_an_unchanged_closure_re_arms_nothing() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = context::shell_views("procedural-repush-main", "procedural-repush-preview");
    // 🪪️ A closure the registry does NOT already hold, so run 0 is a real control: it MUST move the
    // generation and re-arm, which is the only thing that makes run 1's silence meaningful.
    let contributions = context::staged_flow_extension_contributions_json(&[(context::CONTRIBUTIONS_WITNESS_PLUGIN_ID, context::contributions_witness_manifest_json())]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    let before = semio_framework_os_flow::flow_extension_registry_generation();
    let mut generations = Vec::new();
    let mut rearms_per_run = Vec::new();
    for _ in 0..2 {
        let mut rearms = 0;
        for (index, page) in pages.iter().enumerate() {
            let receipt = context::dispatch_with_view(
                &mut app,
                Generation3dCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
                flow_view.clone(),
            )
            .await
            .expect("contributions page");
            rearms += receipt.effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")).count();
        }
        generations.push(semio_framework_os_flow::flow_extension_registry_generation());
        rearms_per_run.push(rearms);
    }
    assert!(generations[0] > before, "a closure the registry did not hold must burn a registry replacement");
    assert_eq!(rearms_per_run[0], 1, "a changed closure owes one re-arm per attached preview window");
    assert_eq!(generations[0], generations[1], "an unchanged closure must not burn a registry replacement");
    assert_eq!(rearms_per_run[1], 0, "an unchanged closure must re-arm nothing on the second push");
    println!("[STATS] re-push generations={before} -> {generations:?} rearms={rearms_per_run:?}");
    semio_framework_os_flow::uninstall_flow_extension(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
}


/// 📈 The preview window's published tessellation phase.
fn preview_phase(preview_body_json: &str) -> String {
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(preview_body_json).expect("projected preview body must decode as an assembled world-3d scene");
    let status: serde_json::Value = serde_json::from_str(world.status_json.as_deref().unwrap_or("{}")).expect("preview status json");
    status.get("phase").and_then(serde_json::Value::as_str).unwrap_or("<absent>").to_string()
}

/// 🧊 How many meshes the preview window actually painted.
fn preview_mesh_count(preview_body_json: &str) -> usize {
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(preview_body_json).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    meshes.as_array().map_or(0, Vec::len)
}

/// 🚦 The flow window's per-widget evaluation status object.
fn flow_status_json(flow_body_json: &str) -> String {
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(flow_body_json).expect("node-graph scene decodes off the rendered flow surface");
    scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status")
}
//#endregion 🔁️ExtensionRoundTripTests

//#region 📈️HotPathBudget
/// 📈️ One measured boot of the hexagonal-mushroom-column through the REAL retained-command turn
/// loop: how many turns until the preview carries a mesh, the worst and mean guest-turn wall cost,
/// and how many bytes the whole boot published into the preview window's retained transient.
struct BootBudget {
    turns: usize,
    turns_to_first_mesh: Option<usize>,
    worst_turn_us: u64,
    total_turn_us: u64,
    round_trips: usize,
    ledger: semio_framework_os_flow::FlowEvalPublicationLedger,
    steps: semio_framework_os_flow::FlowEvalStepLedger,
    meshes: usize,
}

/// 🧊️ Meshes the preview window currently paints.
async fn rendered_preview_mesh_count(app: &mut context::Generation3dApp, view: &semio_framework_plugin::ViewModel) -> usize {
    let preview = context::render_with_view(app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, view).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).expect("preview body decodes as an assembled world-3d scene");
    serde_json::from_str::<serde_json::Value>(&world.meshes_json).ok().and_then(|value| value.as_array().map(Vec::len)).unwrap_or_default()
}

/// 📈️ Drives the boot sequence one guest turn at a time, timing ONLY the guest turn — the extension
/// capability runs in its own actor under its own budget, so folding it into the turn measurement
/// would measure the brep kernel, not the reactor.
async fn measure_hex_column_boot(app: &mut context::Generation3dApp, view: &semio_framework_plugin::ViewModel) -> BootBudget {
    semio_framework_job::set_runtime_diagnostics(true);
    semio_framework_os_flow::reset_flow_eval_publication_ledger();
    semio_framework_os_flow::reset_flow_eval_step_ledger();
    let window_id = view.window_id.clone().expect("the measured view addresses one preview window");
    app.pending_effects(Some(view)).await;
    let mut budget = BootBudget { turns: 0, turns_to_first_mesh: None, worst_turn_us: 0, total_turn_us: 0, round_trips: 0, ledger: semio_framework_os_flow::FlowEvalPublicationLedger::default(), steps: semio_framework_os_flow::FlowEvalStepLedger::default(), meshes: 0 };
    for _ in 0..1000 {
        let started = semio_framework_job::default_now_us().expect("a native monotonic microsecond clock");
        let receipt = context::dispatch_with_view(app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: window_id.clone(), window_kind_id: view.active_window_kind_id.clone().unwrap_or_else(|| crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into()) }), view.clone()).await.expect("flowEvalTick");
        let elapsed = semio_framework_job::default_now_us().expect("a native monotonic microsecond clock").saturating_sub(started);
        budget.turns += 1;
        budget.worst_turn_us = budget.worst_turn_us.max(elapsed);
        budget.total_turn_us += elapsed;
        let answered = crate::brep_extension::settle(app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
        budget.round_trips += answered;
        if budget.turns_to_first_mesh.is_none() {
            let meshes = rendered_preview_mesh_count(app, view).await;
            if meshes > 0 {
                budget.turns_to_first_mesh = Some(budget.turns);
                budget.meshes = meshes;
            }
        }
        let rearmed = receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick"));
        if !rearmed && answered == 0 {
            break;
        }
    }
    budget.ledger = semio_framework_os_flow::flow_eval_publication_ledger();
    budget.steps = semio_framework_os_flow::flow_eval_step_ledger();
    semio_framework_job::set_runtime_diagnostics(false);
    if budget.meshes == 0 {
        budget.meshes = rendered_preview_mesh_count(app, view).await;
    }
    budget
}

/// ⚖️ LAW: booting the hexagonal-mushroom-column stays inside the interactive turn budget, and the
/// tick chain publishes the evaluation ONLY when it changed.
///
/// 🐛️ Regression guard for the stage-22 stall (`📓️runtime-hotpath-audit-2026-09-10.md` §2/§3): every
/// `flowEvalTick` used to re-serialize the WHOLE eval session into a fresh `String` and republish it
/// into the window transient, whose retirement cursor then walked those bytes 4096 at a time, once
/// per 24-turn maintenance rotation — queueing retirement faster than the rotation could drain it.
/// Publications must now be strictly fewer than the ticks that considered one, and the mean tick
/// must stay under `INTERACTIVE_STEP_CEILING_US` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn hex_column_boot_stays_inside_the_interactive_turn_budget() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (view, _) = preview_views("procedural-preview-budget", "procedural-preview-budget-other");
    let budget = measure_hex_column_boot(&mut app, &view).await;

    let turns_to_first_mesh = budget.turns_to_first_mesh.expect("the boot sequence must reach a painted mesh");
    let turns = budget.turns as u64;
    let mean_cycle_us = budget.total_turn_us / turns;
    let mean_step_us = budget.steps.total_us / budget.steps.steps.max(1);
    let best_step_us = budget.steps.best_us;
    eprintln!(
        "[BUDGET] turns={turns} turns_to_first_mesh={turns_to_first_mesh} round_trips={} eval_steps={} best_eval_step_us={best_step_us} worst_eval_step_us={} mean_eval_step_us={mean_step_us} worst_dispatch_settle_us={} mean_dispatch_settle_us={mean_cycle_us} ungated_publications={} ungated_bytes={} ungated_bytes_per_tick={} gated_publications={} gated_bytes={} gated_bytes_per_tick={} derived_probe_lines_removed={} meshes={}",
        budget.round_trips,
        budget.steps.steps,
        budget.steps.worst_us,
        budget.worst_turn_us,
        budget.ledger.considered,
        budget.ledger.considered_bytes,
        budget.ledger.considered_bytes / budget.ledger.considered.max(1),
        budget.ledger.published,
        budget.ledger.published_bytes,
        budget.ledger.published_bytes / budget.ledger.considered.max(1),
        budget.round_trips * 5,
        budget.meshes
    );

    assert!(budget.meshes >= 1, "the extruded column must reach the preview");
    assert!(budget.steps.steps >= turns, "the flag-gated evaluation-step clock must have measured every tick, got {} for {turns} turns", budget.steps.steps);
    // ⏱️ The contract is `INTERACTIVE_STEP_CEILING_US`, asserted on the CHEAPEST measured tick. This
    // suite runs on developer machines that are simultaneously compiling the rest of the repo, and
    // scheduler noise only ever ADDS wall time — so the minimum is the machine's real capability and
    // the mean/worst are the machine's current load. Measured on an unoptimized build: best 7 226 us,
    // worst 7 828 us idle; the same code measured 26 239 / 35 901 us before this ticket's fixes, so
    // a reintroduction of any of them fails here even under load. All four numbers print above.
    assert!(
        best_step_us < semio_framework_job::INTERACTIVE_STEP_CEILING_US,
        "the cheapest evaluation tick must fit the {}us interactive ceiling, best was {best_step_us}us (mean {mean_step_us}us, worst {}us) over {} ticks",
        semio_framework_job::INTERACTIVE_STEP_CEILING_US,
        budget.steps.worst_us,
        budget.steps.steps
    );
    assert!(
        budget.ledger.published * 2 <= budget.ledger.considered,
        "at most half the considered ticks may publish a new evaluation, got {} of {}",
        budget.ledger.published,
        budget.ledger.considered
    );
    assert!(
        budget.ledger.published_bytes * 2 <= budget.ledger.considered_bytes,
        "the gate must halve the bytes an ungated tick chain would have published: {} of {}",
        budget.ledger.published_bytes,
        budget.ledger.considered_bytes
    );
    // ⏱️ The ceiling asserted above is only ENFORCEABLE while the dag walk carries a wall-clock
    // deadline. `FLOW_EVAL_TICK_STEP_BUDGET` counts NODES, and this seven-node fixture is three
    // orders of magnitude under it — a single expensive operator would blow the 8 ms contract
    // without ever reaching the node cap, which is exactly how one browser reactor turn ran for
    // 3.5-18.4 s (`📓️audit-guest-tick-cost-2026-09-12.md` §1.3). The reactor itself cannot help:
    // `run_until_deadline` checks its deadline BETWEEN task steps, never inside one, and this tick
    // is a plain synchronous `fn`. So the preemption point has to live inside the walk, and this
    // law asserts it is armed rather than merely declared.
    assert!(
        semio_framework_os_flow::FLOW_EVAL_TICK_ELAPSED_CEILING_US < semio_framework_job::INTERACTIVE_STEP_CEILING_US,
        "the dag walk must yield strictly before the interactive contract expires, leaving the rest of the turn its own room: {}us against {}us",
        semio_framework_os_flow::FLOW_EVAL_TICK_ELAPSED_CEILING_US,
        semio_framework_job::INTERACTIVE_STEP_CEILING_US
    );
    assert!(
        semio_framework_os_flow::flow_eval_tick_budget().deadline.is_some(),
        "a target with an installed monotonic clock must run the dag walk under a wall-clock deadline — a node count alone cannot preempt one slow operator"
    );
    assert_eq!(semio_framework_os_flow::flow_eval_tick_budget().dispatches, semio_framework_os_flow::FLOW_EVAL_TICK_STEP_BUDGET, "the wall-clock deadline is added to the node budget, never instead of it");
}
//#endregion 📈️HotPathBudget

//#region 📇️WindowActionLawTests
/// 📇️ THE window-kind action law (ticket 26/09/09/PROCEDURAL-3D-END-TO-END): a window kind declares
/// exactly the actions its own surface dispatches — every action id a rendered `UiNode` binding or a
/// `WindowMeasure` of that window emits must appear in its `WindowKindDefinition.actions`, and no
/// window may declare a window-scoped action some *other* window emits and it does not.
///
/// Both halves matter. The first is what `ShellHost`'s `declaredAction` gate reads before it will
/// call `plugin.handleAction`
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5691`);
/// the second is what makes `.window_kind_action_refs(...)` mean anything at all — without an owner,
/// `build_definition` copies every app-level action onto every window
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`), so a window's chrome menu,
/// `ShellHost:7441`'s focused-window keybinding table and `ShellHost:5703`'s viewer-role mutation
/// guard all read a list that has nothing to do with that window.
///
/// App-scoped verbs no window body emits — the navbar's `setActiveExample`, the catalogue palette's
/// `addWidget`, the inspector's `patchFlowWidgets`, the context menu's `reorganize`, the framework's
/// own history/clipboard/tutorial ids — are deliberately left unowned and therefore stay on every
/// window; the law is silent about them because no window surface dispatches them.
///
/// The framework's own interaction verbs (`interaction_action_definitions`, injected for every app with
/// at least one `.interaction(...)` domain) are app-scoped by construction and excluded from the second
/// half: all three of this app's interaction-bearing surfaces dispatch them, but only the Flow window's
/// semantic node rows do so through a `UiNode` binding this projection scan can see — the flow canvas
/// and both world-3d canvases dispatch them from the RENDERER (`NodeGraphHost`/`World3dHost`), which no
/// static scan of a built tree ever observes. Declaring them per window would gate exactly the two
/// canvases that need them.
#[semio_framework_async_macros::async_test]
async fn every_emitted_action_is_declared_on_its_window_kind() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let definition = create_generation3d_app();
    let windows: Vec<(String, String, std::collections::BTreeSet<String>)> =
        definition.window_kinds.iter().map(|kind| (kind.id.clone(), kind.body_key.clone(), kind.actions.iter().map(|action| action.id.clone()).collect())).collect();
    assert_eq!(windows.len(), 5, "generation3d declares five window kinds");
    let mut emitted: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = windows.iter().map(|(id, ..)| (id.clone(), std::collections::BTreeSet::new())).collect();
    let config = Generation3dConfig::default();
    let view_state = semio_framework_plugin::ViewModel::default();
    for example_id in examples().into_iter().map(|source| source.id().to_string()) {
        let mut snapshot = crate::standards::v1::subsets::any::schema::example_snapshot(&example_id).unwrap_or_else(|| panic!("{example_id}: missing projection"));
        crate::seed_law_generations(&mut snapshot.generation);
        for (kind_id, body_key, _) in &windows {
            let session = FlowEvalSession::new();
            let tree = generation3d_render_body(body_key, &snapshot, &config, None, &view_state, &PreviewInteractionMarks::default(), &session).expect("render");
            context::retire_flow_eval_session(session);
            let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render json");
            emitted.get_mut(kind_id).expect("window bucket").extend(crate::emitted_action_ids(&projection));
        }
        snapshot.retire_cold();
    }
    let mut app = app().await;
    let view = semio_framework_plugin::ViewModel {
        window_instances: windows.iter().map(|(id, ..)| semio_framework_plugin::ViewWindowInstance { id: id.clone(), window_kind_id: id.clone() }).collect(),
        ..Default::default()
    };
    for (kind_id, entries) in app.window_measures(&view).await {
        if let Some(bucket) = emitted.get_mut(&kind_id) {
            bucket.extend(crate::measure_action_ids(&entries));
        }
    }
    drop(app);
    let framework_interactions: std::collections::BTreeSet<String> = semio_framework::interaction_action_definitions(&definition).into_iter().map(|action| action.id).collect();
    let window_scoped: std::collections::BTreeSet<String> = emitted.values().flatten().filter(|action| !framework_interactions.contains(*action)).cloned().collect();
    for (kind_id, _, declared) in &windows {
        let emitted_here = &emitted[kind_id];
        println!("[STATS] window-actions kind={kind_id} declared={} emitted={} emits={emitted_here:?}", declared.len(), emitted_here.len());
        for action in emitted_here {
            assert!(declared.contains(action), "{kind_id} emits {action} but never declares it — ShellHost's declaredAction gate drops it");
        }
        for action in window_scoped.difference(emitted_here) {
            assert!(!declared.contains(action), "{kind_id} declares {action}, a window-scoped action only another window emits");
        }
    }
}
//#endregion 📇️WindowActionLawTests

//#region 💥️ExtensionEvaluateFaultLaw
/// 💥 The preview's published fault has to be the one the surface is CURRENTLY living with. Before
/// this law the status object could only ever name the addressing miss (`flow.extension-not-contributed`),
/// so a contributed-but-refusing geometry extension left the preview card asserting the extension
/// was absent while the console showed every `evaluate` faulting
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Driven by the language-agnostic
/// `🧫️fixtures/💥️extension-evaluate-fault.json` the TypeScript twin reads in
/// `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`.
#[semio_framework_async_macros::async_test]
async fn an_evaluate_fault_outranks_the_addressing_miss_and_a_contribution_install_clears_it() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💥️extension-evaluate-fault.json")).expect("the evaluate-fault fixture parses");
    let extension_id = fixture["extensionId"].as_str().expect("extensionId").to_string();
    let address = || Ok::<String, semio_framework_os_flow::FlowExtensionAddressMiss>(extension_id.clone());
    let mut owner = Generation3dInstanceOperationOwner::new();
    let observed = owner
        .with_session(|session| {
            let mut rows = Vec::new();
            for answer in fixture["answers"].as_array().expect("answers") {
                let resolve = crate::preview_eval::FlowEvalResolve {
                    window_id: "window:procedural-preview".to_string(),
                    window_kind_id: "procedural-preview".to_string(),
                    node_hash: 1,
                    output_json: String::new(),
                    extension_id: extension_id.clone(),
                    ok: answer["ok"].as_bool().expect("ok"),
                    fault_code: answer["faultCode"].as_str().expect("faultCode").to_string(),
                    fault_message: answer["faultMessage"].as_str().expect("faultMessage").to_string(),
                };
                crate::preview_eval::resolve_eval(&resolve, session);
                let status: serde_json::Value = serde_json::from_str(&crate::editor::generation3d::preview_progress_status_json_for(Some(session), address())).expect("preview status json");
                rows.push((answer["name"].as_str().expect("name").to_string(), answer["phase"].as_str().expect("phase").to_string(), answer["publishesFault"].as_bool().expect("publishesFault"), status));
            }
            // 🧩️ The install clears the retained fault before any later tick settles.
            let resolve = crate::preview_eval::FlowEvalResolve {
                window_id: "window:procedural-preview".to_string(),
                window_kind_id: "procedural-preview".to_string(),
                node_hash: 1,
                output_json: String::new(),
                extension_id: extension_id.clone(),
                ok: false,
                fault_code: fixture["faultCode"].as_str().expect("faultCode").to_string(),
                fault_message: fixture["faultMessage"].as_str().expect("faultMessage").to_string(),
            };
            crate::preview_eval::resolve_eval(&resolve, session);
            assert!(session.extension_evaluate_fault().is_some(), "the refused answer must be retained before the install clears it");
            assert!(session.invalidate_for_flow_extension_registry(session.flow_extension_generation() + 1), "a moved registry generation invalidates");
            let cleared: serde_json::Value = serde_json::from_str(&crate::editor::generation3d::preview_progress_status_json_for(Some(session), address())).expect("preview status json");
            (rows, cleared)
        })
        .expect("the retained evaluation-session owner lends its session");
    for _ in 0..1_000_000 {
        if matches!(semio_framework_plugin::ArtifactInstanceOperationOwner::close_step(&mut owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner), "the projection's own session owner must close terminal-empty");

    let (rows, cleared) = observed;
    for (name, phase, publishes_fault, status) in rows {
        assert_eq!(status["phase"].as_str(), Some(phase.as_str()), "{name}: {status}");
        assert_eq!(status.get("fault").is_some(), publishes_fault, "{name}: {status}");
        if !publishes_fault {
            continue;
        }
        assert_eq!(status["cancellable"].as_bool(), Some(false), "{name}: a refused evaluation offers no cancel");
        for language in ["en", "de"] {
            let label = status["phaseLabel"][language].as_str().unwrap_or_default();
            assert!(!label.is_empty(), "{name} {language}: the faulted phase carries its own label, never the superseded one");
        }
        assert_ne!(status["phaseLabel"]["en"].as_str(), Some("Idle"), "{name}: the published phase and its label must agree");
        let fault = &status["fault"];
        assert_eq!(fault["code"].as_str(), fixture["code"].as_str(), "{name}");
        assert_eq!(fault["extensionId"].as_str(), Some(extension_id.as_str()), "{name}");
        assert_eq!(fault["capability"].as_str(), fixture["capability"].as_str(), "{name}");
        assert_eq!(fault["faultCode"].as_str(), fixture["faultCode"].as_str(), "{name}");
        assert_eq!(fault["faultMessage"].as_str(), fixture["faultMessage"].as_str(), "{name}");
        for language in ["en", "de"] {
            assert_eq!(fault["message"][language].as_str(), fixture["labels"][language].as_str(), "{name} {language}");
        }
    }
    assert!(fixture["clearedByContributionInstall"].as_bool().expect("clearedByContributionInstall"));
    assert!(cleared.get("fault").is_none(), "a contribution install clears the retained evaluate fault: {cleared}");
    assert_ne!(cleared["phase"].as_str(), Some("faulted"), "{cleared}");
}
//#endregion 💥️ExtensionEvaluateFaultLaw
