pub(crate) mod context {
    use super::super::*;
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
        let mut app = new_app_with_registry::<EditorApp<Generation2dPlayApp>>(generation2d_manifest_for_tests).await;
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
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1).await.expect("Generation2d dispatched operation settles");
        result
    }
    
    /// 🧹️ Walks the fixture app to its terminal-empty ownership witness — a registered app owns an
    /// `ArtifactStore` and a fixed owner registry, both of which reject a bare drop.
    pub fn close(mut app: Generation2dApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
    }
    
    pub async fn render(app: &mut Generation2dApp, body_key: &str) -> String {
        render_with_view(app, body_key, &ViewModel::default()).await
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
        let view = ViewModel { window_instances: roster, ..Default::default() };
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
        let view = ViewModel { window_instances: roster, ..Default::default() };
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
                effects.extend(Box::pin(settle_registered_typed_operation(app, action_meta.instance_id)).await.expect("the run's driver turns settle").effects);
            }
            let dispatches: Vec<(String, Option<dsl::DslValue>)> = std::mem::take(&mut effects).into_iter().filter_map(|effect| match effect {
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
                    Box::pin(app.handle_action(&action, args.as_ref(), action_meta)).await.unwrap_or_else(|fault| panic!("{action} dispatch: {fault:?}"));
                    continue;
                }
                if action == "flowEvalTick" {
                    receipt.hops += 1;
                    receipt.hop_windows.push(args.as_ref().and_then(|args| args.get("windowId")).and_then(dsl::DslValue::as_str).unwrap_or_default().to_string());
                }
                let arguments = match args.as_ref() {
                    Some(dsl::DslValue::Object(entries)) => entries.iter().cloned().collect(),
                    _ => std::collections::BTreeMap::new(),
                };
                let app_id = app.app_id().await.to_string();
                let invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: String::new(), app_id }, command_id: action.clone() }, arguments };
                Box::pin(app.handle_command(&invocation, None, action_meta)).await.unwrap_or_else(|fault| panic!("the shell redispatches the run's {action} hop: {fault:?}"));
                let hop = Box::pin(settle_registered_typed_operation(app, action_meta.instance_id)).await.expect("retained publication");
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
            active_alternative_id: None,
            current_checkpoint_id: None,
            commands: Vec::new(),
            command_filter: semio_framework_plugin::app::HistoryCommandFilter::default(),
        }
    }
}

use super::*;
use crate::editor::generation2d::unit_tests::context::{app, app_with_registry, close, snapshot_read};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip;
use semio_framework_plugin::PluginApp;

fn production_initial_snapshot(label: &str) -> Generation2dSnapshot {
    let mut snapshot = Generation2dSnapshot::default();
    snapshot.host_snapshot.schema = label.into();
    for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
        snapshot.host_snapshot.widgets.push(Widget::InputNote { id: id.into(), text: text.into() });
    }
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
    snapshot.host_snapshot.layout.insert("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 });
    snapshot.host_snapshot.layout.insert("clear-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
        snapshot.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() });
    }
    snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("rename-generation".into());
    snapshot
}

fn production_mutations() -> Vec<Generation2dMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::*;
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("production".into()))),
        ),
    );
    vec![
        create_widget(0, Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true }),
        replace_widget(Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() }),
        delete_widget("delete-target".into()),
        connect_synapse(0, semio_framework_artifact_flow_flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() }),
        replace_synapse(semio_framework_artifact_flow_flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() }),
        disconnect_synapse("disconnect-synapse".into()),
        move_widget("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 31.0, y: -17.0 }),
        clear_widget_layout("clear-target".into()),
        update_camera(semio_framework_artifact_flow_flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 }),
        change_schema("flow.host_snapshot.production-retained".into()),
        create_generation(semio_framework_artifact_playbook_playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: Default::default() }),
        delete_generation("delete-generation".into()),
        rename_generation("rename-generation".into(), "After Rename".into()),
        change_generation_value("change-generation".into(), "deep-answer".into(), serde_json::json!({"object": {"array": [1.0, false, "retained"]}}).into()),
    ]
}

fn production_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::new();
    value.try_reserve_exact(bytes.len() * 2).expect("P2 production hex preflight");
    for byte in bytes {
        value.push(char::from(DIGITS[usize::from(byte >> 4)]));
        value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    value
}

/// 📸️ Every owned production projection this law holds is a CLOSING read — `fixture.layout` is a
/// live `OrderedMap` root that aborts the process on a bare drop.
fn production_read(snapshot: Generation2dSnapshot) -> crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead {
    crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead::new(snapshot)
}

fn production_semantic_digest(snapshot: &Generation2dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation2d.production-law.semantic");
    digest.observe(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(snapshot));
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation2dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let mutations = production_mutations();
    assert_eq!(mutations.len(), 14, "production ingress carries every P2 mutation variant including clear-widget-layout");
    let mut mutation_hex = Vec::new();
    mutation_hex.try_reserve_exact(mutations.len()).expect("P2 production mutation owner preflight");
    for mutation in &mutations {
        mutation_hex.push(production_hex(&crate::standards::v1::subsets::any::schema::mutations::binary::encode_op(mutation).expect("P2 production mutation encoding")));
    }
    let mut expected = production_initial_snapshot(label);
    crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_apply_retained_mutations_for_test(&mut expected, &mutations);
    let expected_digest = production_semantic_digest(&expected);
    crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_retire_mutations_cold(mutations);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GENERATION_2D_SCHEMA,
        "id": "generation2d-production-mounted-law",
        "vcs": {
            "initialSnapshot": production_hex(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(&snapshot)),
            "edits": [{
                "id": "generation2d-production-all14-edit",
                "actor": "generation2d-production-law",
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
    .expect("schema-first P2 production fixture envelope");
    snapshot.retire_cold();
    (wire, expected, expected_digest)
}

/// 🔐️ Releases the process-global publication lease on UNWIND too. The lease table is a fixed
/// 4-slot registry (`GENERATION2D_PUBLICATION_SLOTS`), so a law that panics mid-drive strands a slot
/// and every later `generation2d_admit_publication_authority` anywhere in the binary then fails
/// `generation2d-publication.saturated` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
struct ProductionLease(semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle);

impl Drop for ProductionLease {
    fn drop(&mut self) {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_release_publication_authority(self.0.operation, self.0.generation);
    }
}

fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation2dPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P2 production ingress credits");
    crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_admit_publication_authority(handle.operation, handle.generation, handle.generation.0, handle.generation.0, handle.generation.0, crate::standards::v1::subsets::any::schema::mutations::binary::Generation2dPublicationCredits { maximum_items: 8_192, maximum_output_pages: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION2D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION2D_MOUNTED_CONTROL_CREDITS })
    .expect("P2 production publication authority");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P2 production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P2 production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("P2 production envelope seal"));
    handle
}

fn drive_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation2dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..300_000 {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0)
            .expect("P2 authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P2 production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("P2 production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("P2 production envelope load did not reach terminal");
}

/// 🔐️ LAW: non-empty P2D2 canonical ingress reaches the real VCS maintenance replacement,
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
    let handle = admit_production_envelope(&mut accepted, &wire);
    let lease = ProductionLease(handle);
    assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let snapshot = production_read(accepted.snapshot().expect("accepted P2 production snapshot"));
    let expected = production_read(expected);
    assert_eq!(&*snapshot, &*expected, "real maintenance must publish all P2 snapshot and all-14 replay fields");
    assert_eq!(production_semantic_digest(&snapshot), expected_digest);
    assert!(snapshot.host_snapshot.layout.contains_key("move-target"));
    assert!(!snapshot.host_snapshot.layout.contains_key("clear-target"), "2D-only clear-widget-layout must survive retained replay");
    assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P2 terminal ACK"));
    assert!(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_release_publication_authority(handle.operation, handle.generation));
    drop(lease);
    close(accepted);

    use crate::standards::v1::subsets::any::schema::mutations::binary::Generation2dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
    for (hostile, expected_code) in [
        (Missing, "generation2d-publication.authority-missing"),
        (WrongOperation, "generation2d-publication.wrong-operation"),
        (WrongGeneration, "generation2d-publication.wrong-generation"),
        (WrongBase, "generation2d-publication.wrong-base"),
        (WrongParent, "generation2d-publication.wrong-parent"),
    ] {
        let mut app = app_with_registry().await;
        let last_valid = production_read(app.snapshot().expect("last-valid P2 snapshot"));
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (wire, _, _) = production_envelope_wire("rejected-production-candidate");
        let handle = admit_production_envelope(&mut app, &wire);
        let lease = ProductionLease(handle);
        crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_take_publication_hostile_observed(handle.operation), Some(expected_code));
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = production_read(app.snapshot().expect("last-valid P2 snapshot after rejected candidate"));
        assert_eq!(production_semantic_digest(&retained), last_valid_digest);
        assert_eq!(&*retained, &*last_valid);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P2 terminal ACK after candidate retirement"));
        assert!(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_release_publication_authority(handle.operation, handle.generation));
        drop(lease);
        close(app);
    }
}

//#region 🔖️CommandSurface
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    assert_eq!(GENERATION2D_BOUNDED_TOOL_IDS.len(), 21);
    assert_eq!(GENERATION2D_CONTRIBUTIONS_TOOL_IDS.len(), 1);
    assert_eq!(<Generation2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 22, "both factories' proofs, aggregated");
    assert_eq!(Generation2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 21);
    assert_eq!(Generation2dContributionsJobFactory::PUBLICATION_CONTRACTS.len(), 1);
    assert!(GENERATION2D_CONTRIBUTIONS_TOOL_IDS.iter().all(|tool_id| !GENERATION2D_BOUNDED_TOOL_IDS.contains(tool_id)), "a tool id may be owned by exactly one factory");
    assert!(GENERATION2D_CONTRIBUTIONS_RAW_BYTES > GENERATION2D_RETAINED_RAW_BYTES, "the contributions route exists precisely because the gesture quota cannot carry it");
    assert_eq!(generation2d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation2d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert!(GENERATION2D_BOUNDED_TOOL_IDS.iter().all(|tool_id| Generation2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    for migrated in ["nodeGraphEdit", "moveMediaNode", "addWidget", "removeWidget", "connectMediaPorts", "reorganize", "setEvalOutputs"] {
        assert!(GENERATION2D_BOUNDED_TOOL_IDS.contains(&migrated), "{migrated} must own an exact retained reducer route");
        assert!(<Generation2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().iter().any(|proof| proof.tool_id() == migrated), "{migrated} must carry its bounded first-step proof row");
    }
    assert!(
        every_command().iter().all(|command| GENERATION2D_BOUNDED_TOOL_IDS.contains(&command.command_id()) || GENERATION2D_CONTRIBUTIONS_TOOL_IDS.contains(&command.command_id())),
        "every declared command routes through one of the two retained ladders"
    );
}

async fn drive_preview_operation(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation2dPlayApp>>) -> Result<(u64, u64, u64), String> {
    use semio_framework_plugin::app::TypedOperationResultLane;
    let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1).await.map_err(|error| format!("{error:?}"))?;
    let count = |wanted: TypedOperationResultLane| receipt.lanes.iter().filter(|lane| **lane == wanted).count() as u64;
    Ok((count(TypedOperationResultLane::Artifact), count(TypedOperationResultLane::Config), count(TypedOperationResultLane::Transient)))
}

/// ⚖️ LAW: a generation command authors its document and selection lanes and NOTHING else — no tick loop
/// and no transient of its own. The generate preview's evaluation is the `previewEval` run's, so the
/// command carries exactly one run start on its own emit for the attached generate preview, and the run
/// publishes the evaluation into the ONE app transient every generate preview window renders.
#[semio_framework_async_macros::async_test]
async fn a_generation_command_carries_the_run_start_and_the_run_publishes_the_shared_generate_preview() {
    let mut app = app_with_registry().await;
    let result: Result<(), String> = async {
        let (generations_view, previews) = context::generate_shell_views();
        let before_generations = snapshot_read(&app).generation.as_state().generations.len();
        let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(generations_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("preview-owner") };
        app.dispatch_typed(Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "rect".into() }), &action_meta).await.map_err(|error| format!("{error:?}"))?;
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        let settled = context::drive_preview_run(&mut app, &action_meta, &[]).await;
        let before_transient = app.ephemeral_snapshot().await.transient_generation;
        app.dispatch_typed(Generation2dCommand::AddGeneration(add_generation::AddGeneration {}), &action_meta).await.map_err(|error| format!("{error:?}"))?;
        let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        let count = |wanted: semio_framework_plugin::app::TypedOperationResultLane| receipt.lanes.iter().filter(|lane| **lane == wanted).count();
        let lanes = (count(semio_framework_plugin::app::TypedOperationResultLane::Artifact), count(semio_framework_plugin::app::TypedOperationResultLane::Config), count(semio_framework_plugin::app::TypedOperationResultLane::Transient));
        if lanes != (1, 1, 0) {
            return Err(format!("addGeneration must publish its artifact and selection lanes only, got {lanes:?}"));
        }
        let carried = context::run_actions(&receipt.effects);
        if carried != vec![semio_framework_plugin::TOOL_RUN_START_ACTION_ID.to_string()] {
            return Err(format!("addGeneration must carry exactly one run start, got {carried:?} (the boot run settled as {settled:?})"));
        }
        if app.ephemeral_snapshot().await.transient_generation != before_transient {
            return Err("the gesture itself must publish no transient".into());
        }
        let run = context::drive_preview_run(&mut app, &action_meta, &receipt.effects).await;
        if !run.hop_windows.iter().any(|window| window == "preview-a" || window == "preview-b") {
            return Err(format!("the run must evaluate the generate previews: {run:?}"));
        }
        if run.state.as_deref() != Some("finalized") && run.state.as_deref() != Some("complete") {
            return Err(format!("the run must settle: {run:?}"));
        }
        if app.ephemeral_snapshot().await.transient_generation == before_transient {
            return Err("the run must publish the generate preview into the app transient".into());
        }
        if snapshot_read(&app).generation.as_state().generations.len() != before_generations + 1 {
            return Err("addGeneration did not preserve its document behavior".into());
        }
        let mut rendered = Vec::new();
        for context_view in &previews {
            let tree = app.render(generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW, None, context_view).await.map_err(|error| format!("{error:?}"))?;
            rendered.push(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?);
        }
        if rendered[0] != rendered[1] {
            return Err("generation windows did not consume the same app-transient preview".into());
        }
        if rendered[0].contains(generation2d_labels(&previews[0]).preview_hint.as_str()) {
            return Err(format!("the generate preview still shows its hint after the run evaluated: {}", rendered[0]));
        }
        if include_str!("../../🎚️config/🧬️schema/🔣️.json").contains("generationPreviewText") {
            return Err("config schema still owns computed preview output".into());
        }
        Ok(())
    }
    .await;
    close(app);
    result.expect("Generation2d generate preview run");
}

/// ⚖️ LAW (brief: start → complete → finalize, abort leaves the document byte-identical): the read-only
/// `previewEval` run started by the edit preview's first poll settles, is finalized (a read-only run is
/// finalized by the framework as soon as it completes) into NO document edit,
/// and an abort raised while it is running leaves the document pack, the config pack and the history
/// exactly as they were.
#[semio_framework_async_macros::async_test]
async fn the_preview_eval_run_finalizes_nothing_and_an_abort_leaves_the_document_byte_identical() {
    let mut app = app_with_registry().await;
    let result: Result<(), String> = async {
        let (flow_view, _) = context::edit_shell_views();
        let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
        app.dispatch_typed(Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "rect".into() }), &action_meta).await.map_err(|error| format!("{error:?}"))?;
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        let document = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
        let config = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        let history = app.artifact_generation_now();
        let run = context::drive_preview_run(&mut app, &action_meta, &[]).await;
        if run.hops == 0 || run.state.as_deref() != Some("finalized") {
            return Err(format!("the run must hop, complete and finalize: {run:?}"));
        }
        let after = (app.document_pack().await.map_err(|error| format!("{error:?}"))?, app.config_pack().await.map_err(|error| format!("{error:?}"))?, app.artifact_generation_now());
        if (after.0.pack.as_slice(), after.1.pack.as_slice(), after.2) != (document.pack.as_slice(), config.pack.as_slice(), history) {
            return Err("a finalized read-only run must author no document, config or history entry".into());
        }
        app.dispatch_typed(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None }), &action_meta).await.map_err(|error| format!("{error:?}"))?;
        let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        let edited = (app.document_pack().await.map_err(|error| format!("{error:?}"))?, app.artifact_generation_now());
        for effect in &receipt.effects {
            if let Effect::DispatchAction { action, args, .. } = effect {
                app.handle_action(action, args.as_ref(), &action_meta).await.map_err(|error| format!("{action}: {error:?}"))?;
            }
        }
        let view = app.tool_run_ledger().view().ok_or("the gesture's run start must open a run")?;
        app.handle_action(
            semio_framework_plugin::TOOL_RUN_ABORT_ACTION_ID,
            Some(&dsl::DslValue::object([
                (semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID.to_string(), dsl::DslValue::String(view.identity.id.run.to_string())),
                (semio_framework_tool_run::TOOL_RUN_ARG_GENERATION.to_string(), dsl::DslValue::uint(u64::from(view.identity.generation))),
            ])),
            &action_meta,
        )
        .await
        .map_err(|error| format!("toolRunAbort: {error:?}"))?;
        let aborted = context::drive_preview_run(&mut app, &action_meta, &[]).await;
        if aborted.state.as_deref() != Some("aborted") {
            return Err(format!("the abort must end the run aborted and the poll must not restart it: {aborted:?}"));
        }
        let after_abort = (app.document_pack().await.map_err(|error| format!("{error:?}"))?, app.artifact_generation_now());
        if (after_abort.0.pack.as_slice(), after_abort.0.spr.as_slice(), after_abort.1) != (edited.0.pack.as_slice(), edited.0.spr.as_slice(), edited.1) {
            return Err("an aborted read-only run must leave the document byte-identical".into());
        }
        app.handle_action(semio_framework_tool_run::TOOL_RUN_DISMISS_ACTION_ID, Some(&dsl::DslValue::object([(semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID.to_string(), dsl::DslValue::String(view.identity.id.run.to_string()))])), &action_meta).await.map_err(|error| format!("toolRunDismiss: {error:?}"))?;
        Ok(())
    }
    .await;
    close(app);
    result.expect("Generation2d previewEval lifecycle");
}

/// 🧳️ A standalone instance owner handle with the demo's edit preview attached through the real
/// `pending_effects` law, retired through its own close ladder by [`retire_owner`].
fn demo_owner(window: (&str, &'static str)) -> semio_framework_plugin::ArtifactInstanceOperationOwnerHandle {
    let handle = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(<Generation2dPlayApp as ArtifactEditor>::build_instance_operation_owner());
    handle
        .with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
            let (sessions, link) = owner.parts()?;
            let windows = [(window.0, window.1, PreviewEvalTarget::Document)];
            crate::preview_eval::preview_eval_run_effects(sessions, link, &windows, &Default::default(), None, true);
            Ok(())
        })
        .expect("the demo owner attaches its preview");
    handle
}

fn retire_owner(handle: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) {
    use semio_framework_plugin::ArtifactInstanceOperationOwner;
    for _ in 0..1_000_000 {
        if handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| Ok(matches!(ArtifactInstanceOperationOwner::close_step(owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete)))).expect("the owner lends itself to its close ladder") {
            return;
        }
    }
    panic!("the demo owner did not reach terminal-empty");
}

fn close_job(job: &mut crate::preview_eval::PreviewEvalRunJob<Generation2dInstanceOperationOwner>) {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};
    job.begin_close();
    for _ in 0..1_000 {
        if matches!(job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Complete) {
            assert!(job.terminal_is_empty());
            return;
        }
    }
    panic!("the run job close never completed");
}

/// ⚖️ LAW (`⏯️preview-eval-run.json` `demoRun`): the run job over the bundled demo — every hop performed by
/// the real `flowEvalTick` hop core, nothing contributed — traces every node it observes with its verdict,
/// counts one hop per unit of fuel, settles with the declared settle step, and no single `drive_step`
/// overruns the interactive ceiling.
#[test]
fn the_demo_run_job_traces_every_node_settles_and_stays_under_the_interactive_ceiling() {
    use semio_framework_job::{Generation, InteractiveStage, OperationId, StepBudget, StepOutcome};
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunTick, ToolRunTraceOp};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⏯️preview-eval-run.json")).expect("run fixture");
    let row = &fixture["demoRun"];
    let expected = &row["expected"];
    let (window_id, kind) = (row["window"]["id"].as_str().unwrap(), edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW);
    assert_eq!(row["window"]["kind"].as_str(), Some(kind));
    let handle = demo_owner((window_id, kind));
    let port = semio_framework_plugin::ToolRunJobPort::default();
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]);
    let mut job = crate::preview_eval::PreviewEvalRunJob::<Generation2dInstanceOperationOwner>::new(handle.clone(), port.clone(), identity).expect("the run job attaches");
    let snapshot = crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead::new(crate::standards::v1::subsets::any::schema::default_snapshot());
    let history = context::empty_history_view();
    let config = Generation2dConfig::default();
    let tick = Generation2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: window_id.into(), window_kind_id: kind.into() });
    let (mut sequence, mut worst_us, mut hops_performed) = (0_u64, 0_u64, 0_usize);
    let mut ticks: Vec<ToolRunTick> = Vec::new();
    let settled = loop {
        let started = semio_framework_job::default_now_us().expect("clock");
        let mut verdict = None;
        let outcome = semio_framework_job::drive_step(&mut job, "generation2d.preview-eval.run.test", OperationId(1), Generation(0), InteractiveStage::InteractiveStep, StepBudget::new(1, started + semio_framework_job::INTERACTIVE_LANE_WALL_US * 8), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut verdict);
        worst_us = worst_us.max(semio_framework_job::default_now_us().expect("clock").saturating_sub(started));
        match outcome {
            StepOutcome::Yield => {}
            StepOutcome::PreviewReady(mut payload) => {
                let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("tick page").to_vec()).collect();
                while !payload.terminal_is_empty() {
                    let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                }
                ticks.push(ToolRunTick::decode(&bytes).expect("tick decodes"));
            }
            other => break other,
        }
        let armed = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| Ok(owner.parts()?.0.document.window_tick_is_armed(window_id))).expect("owner");
        if armed {
            hops_performed += 1;
            let doc = ArtifactView::new(&*snapshot, &history);
            let cfg = ConfigView { snapshot: &config, window: None };
            let (emit, transient) = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| generation2d_flow_eval_hop(owner, &tick, &doc, &cfg)).expect("the hop runs");
            assert!(emit.extension_invocations.is_empty() && transient.is_empty(), "an uncontributed document target parks nothing and publishes no transient");
        }
        assert!(ticks.len() < 10_000, "the run must settle");
    };
    let mut final_nodes: std::collections::BTreeMap<u64, &'static str> = std::collections::BTreeMap::new();
    let mut verdicts = std::collections::BTreeSet::new();
    for op in ticks.iter().flat_map(|tick| tick.trace.iter()).flat_map(|page| page.ops.iter()) {
        if let ToolRunTraceOp::Upsert { key, verdict, reason, .. } = op {
            verdicts.insert(format!("{verdict:?}"));
            final_nodes.insert(*key, crate::preview_eval::PreviewEvalRunReason::ALL[usize::from(*reason)].id());
        }
    }
    let expected_nodes: std::collections::BTreeMap<u64, &str> = expected["finalNodes"].as_object().unwrap().iter().map(|(node, reason)| (crate::preview_eval::preview_eval_node_entity(node), reason.as_str().unwrap())).collect();
    let settle_reasons: Vec<&str> = ticks.iter().flat_map(|tick| tick.steps.iter()).map(|step| crate::preview_eval::PreviewEvalRunReason::ALL[usize::from(step.reason)].id()).collect();
    let hops_counted = ticks.last().and_then(|tick| tick.progress.as_ref()).and_then(|progress| progress.counters.iter().find(|counter| counter.counter == crate::preview_eval::PreviewEvalRunCounter::Hops.index())).map_or(0, |counter| counter.value);
    println!("[STATS] demoRun ticks={} hops_performed={hops_performed} hops_counted={hops_counted} worst_drive_step_us={worst_us} verdicts={verdicts:?} steps={settle_reasons:?}", ticks.len());
    assert!(matches!(settled, StepOutcome::Complete(_)), "the run completes: {:?}", std::mem::discriminant(&settled));
    assert_eq!(final_nodes, expected_nodes, "every node's final traced verdict");
    assert_eq!(settle_reasons.last().copied(), expected["settleReason"].as_str(), "the settle step");
    assert!(hops_counted >= expected["hopsAtLeast"].as_u64().unwrap() && hops_counted as usize == hops_performed, "one counted hop per performed hop");
    assert!(worst_us <= expected["ceilingMicros"].as_u64().unwrap(), "worst drive_step {worst_us} µs overran the interactive ceiling");
    close_job(&mut job);
    drop(snapshot);
    retire_owner(&handle);
}

/// ⚖️ LAW: closing an UNSETTLED run job — the host-driven abort — quiesces the sessions it was running: its
/// armed window owes nothing afterwards, so no poll restarts what the user stopped.
#[test]
fn closing_an_unsettled_run_job_quiesces_its_window() {
    use semio_framework_job::{Generation, InteractiveStage, OperationId, StepBudget};
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity};
    let window_id = "preview-1";
    let handle = demo_owner((window_id, edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW));
    let port = semio_framework_plugin::ToolRunJobPort::default();
    let mut job = crate::preview_eval::PreviewEvalRunJob::<Generation2dInstanceOperationOwner>::new(handle.clone(), port, ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32])).expect("the run job attaches");
    let mut sequence = 0;
    let mut verdict = None;
    let started = semio_framework_job::default_now_us().expect("clock");
    let outcome = semio_framework_job::drive_step(&mut job, "generation2d.preview-eval.close.test", OperationId(1), Generation(0), InteractiveStage::InteractiveStep, StepBudget::new(1, started + semio_framework_job::INTERACTIVE_LANE_WALL_US * 8), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut verdict);
    if let semio_framework_job::StepOutcome::PreviewReady(mut payload) = outcome {
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
    }
    let armed = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| Ok(owner.parts()?.0.document.window_tick_is_armed(window_id))).expect("owner");
    assert!(armed, "the first step dispatches the first hop");
    close_job(&mut job);
    let (owed, still_armed) = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
        let (sessions, _) = owner.parts()?;
        Ok((sessions.document.window_tick_owed(window_id), sessions.document.window_tick_is_armed(window_id)))
    }).expect("owner");
    assert_eq!((owed, still_armed), (false, false), "an aborted run leaves its window quiescent");
    retire_owner(&handle);
}

#[test]
fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), GENERATION2D_BOUNDED_TOOL_IDS.len() + GENERATION2D_CONTRIBUTIONS_TOOL_IDS.len(), "every Generation2dCommand row must be covered by every_command()");
}

#[test]
fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
/// where the two vocabularies genuinely diverge. This is what a missing `#[dsl(keyword = ..)]` on a
/// payload struct silently breaks (the record prints with no keyword at all and fails to re-parse).
#[test]
fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_keywords = [
        "node-graph-edit",
        "move-media-node",
        "add-widget",
        "remove-widget",
        "connect-media-ports",
        "reorganize",
        "add-generation",
        "remove-generation",
        "rename-generation",
        "update-generation-values",
        "node-graph-viewport",
        "set-show-mode",
        "generate",
        "set-eval-outputs",
        "canvas-pointer-down",
        "canvas-pointer-move",
        "canvas-pointer-up",
        "canvas-wheel",
        "select-generation",
        "flow-eval-tick",
        "flow-eval-resolve",
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
pub(super) fn every_command() -> Vec<Generation2dCommand> {
    vec![
        Generation2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
        Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None }),
        Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "n1".into() }),
        Generation2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
        Generation2dCommand::Reorganize(reorganize::Reorganize {}),
        Generation2dCommand::AddGeneration(add_generation::AddGeneration {}),
        Generation2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "g1".into() }),
        Generation2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "g1".into(), name: "Copy".into() }),
        Generation2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
        Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d::default() }),
        Generation2dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wire".into() }),
        Generation2dCommand::Generate(enter_generate::Generate {}),
        Generation2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: "{}".into() }),
        Generation2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}),
        Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}),
        Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
        Generation2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {}),
        Generation2dCommand::SelectGeneration(select_generation::SelectGeneration { id: Some("g1".into()) }),
        Generation2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: "preview-1".into(), window_kind_id: edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW.into() }),
        Generation2dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { window_id: "preview-1".into(), window_kind_id: edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW.into(), node_hash: 7, output_json: "{}".into(), ..Default::default() }),
        Generation2dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into(), page: 0, page_count: 1 }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[test]
fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_generation2d_app()).expect("app definition json");
    for id in [
        flow_window::GENERATION2D_PLAY_WINDOW_MAIN,
        edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW,
        generations::GENERATION2D_PLAY_WINDOW_GENERATIONS,
        form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM,
        generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW,
    ] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::GENERATION2D_PLAY_MODE_EDIT, generate::GENERATION2D_PLAY_MODE_GENERATE] {
        assert!(json.contains(id), "mode {id} missing from the manifest");
    }
    for body in [document_panel::GENERATION2D_PLAY_BODY_ARTIFACT, catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE, inspection_panel::GENERATION2D_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("2d.generation"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn declared_actions_bridge_to_commands() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Generation2dPlayApp>>(context::generation2d_manifest_for_tests).await;
}

/// 🎥️ LAW: `nodeGraphViewport` declares no args, so the shell stages nothing for it. An absent `viewport`
/// decodes to the identity camera instead of refusing the bridge, while a present but malformed one still
/// faults. The generic bridge law only exercises the absent half.
#[test]
fn node_graph_viewport_decodes_an_absent_viewport_as_identity_and_refuses_a_malformed_one() {
    let decode = |args: dsl::DslValue| <Generation2dPlayApp as ArtifactEditor>::command_from_action("nodeGraphViewport", Some(&args));
    let identity = semio_framework_os_kernel::Viewport2d::default();
    assert_eq!(decode(dsl::DslValue::Object(Vec::new())).expect("an argless invocation bridges"), Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: identity }));
    let authored = semio_framework_os_kernel::Viewport2d { x: 12.0, y: -8.0, zoom: 2.0 };
    assert_eq!(
        decode(dsl::DslValue::object([("viewport".into(), protocol::ToValue::to_value(&authored))])).expect("an authored viewport bridges"),
        Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: authored })
    );
    let refused = decode(dsl::DslValue::object([("viewport".into(), dsl::DslValue::object([("x".into(), dsl::DslValue::float(1.0)), ("y".into(), dsl::DslValue::float(2.0)), ("zoom".into(), dsl::DslValue::float(0.0))]))]));
    assert!(refused.is_err(), "a zoom the graph cannot express must fault instead of being replaced by the identity camera");
}

/// 🧩️ `addWidget` is a MIGRATED interactive job now, so `dispatch_typed` only ENQUEUES it — the
/// document grows once the retained ladder is driven to its publication, exactly as the runtime
/// drives it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn add_widget_materializes_declared_kind_default_into_an_operation() {
    let mut app = app_with_registry().await;
    let result: Result<(usize, usize), String> = async {
        let before = snapshot_read(&app).host_snapshot.widgets.len();
        app.dispatch_typed(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None }), &semio_framework_plugin::artifact_app_laws::meta("local"))
            .await
            .map_err(|error| format!("{error:?}"))?;
        let (artifact, _, _) = drive_preview_operation(&mut app).await?;
        if artifact != 1 {
            return Err(format!("addWidget must publish its artifact lane exactly once, got {artifact}"));
        }
        Ok((before, snapshot_read(&app).host_snapshot.widgets.len()))
    }
    .await;
    close(app);
    let (before, after) = result.expect("Generation2d addWidget retained runtime");
    assert_eq!(after, before + 1);
}

#[semio_framework_async_macros::async_test]
async fn add_widget_undo_redo_round_trip() {
    let mut app = app().await;
    let before = snapshot_read(&app).host_snapshot.widgets.len();
    assert_undo_redo_round_trip(&mut app, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None }), |app| snapshot_read(app).host_snapshot.widgets.len(), before, before + 1).await;
    close(app);
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_widget_moves() {
    let fixture = app().await;
    let widgets: Vec<String> = snapshot_read(&fixture).host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    close(fixture);
    assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
    let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
    // 🧹️ The REGISTERED pair, never `assert_two_instances_converge` — this app publishes
    // `bounded_first_step_tool_proofs!`, so a registryless instance faults with
    // `interactive-job.catalog-authority` and its unwind aborts the binary.
    semio_framework_plugin::artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<Generation2dPlayApp>, (Option<f64>, Option<f64>), _, _>(
        "mem://generation2d-convergence",
        || async { crate::editor::generation2d::unit_tests::context::generation2d_manifest_for_tests() },
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
        move |app| {
            let projection = snapshot_read(app);
            (projection.host_snapshot.layout.get(&w0).map(|entry| entry.x), projection.host_snapshot.layout.get(&w1).map(|entry| entry.x))
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::generation2d::unit_tests::context::render;
    let mut app = app().await;
    let rendered = render(&mut app, "generation2d.play.nope").await;
    close(app);
    assert!(rendered.contains("Unknown body"), "{rendered}");
}
//#endregion 🔖️CrossCutting

//#region 📏️SurfaceBudgetTests
/// 🗂️ Every authored body key, in the order `generation2d_render_body` matches them.
const GENERATION2D_BODY_KEYS: [&str; 8] = [
    flow_window::GENERATION2D_PLAY_BODY_MAIN,
    edit_preview::GENERATION2D_PLAY_BODY_PREVIEW,
    generations::GENERATION2D_PLAY_BODY_GENERATIONS,
    form::GENERATION2D_PLAY_BODY_GENERATE_FORM,
    generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW,
    document_panel::GENERATION2D_PLAY_BODY_ARTIFACT,
    catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE,
    inspection_panel::GENERATION2D_PLAY_BODY_INSPECTION,
];

/// 📏️ The generation2d twin of generation3d's surface-bound law. Every window and panel body must fit
/// the framework's ONE resident surface capacity (`ui_contract::UI_RESIDENT_SURFACE_BYTES`, which
/// `ui_runtime`'s `SURFACE_RECONCILE_SURFACE_BYTES` is defined as). The registered operator catalogue is
/// APP-STATIC and rides the reserved `framework.section.catalogue` surface, so no body may grow with the
/// installed operator set — the exact failure that made the node-graph window unrenderable
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
#[semio_framework_async_macros::async_test]
async fn every_window_and_panel_surface_fits_the_resident_surface_bound() {
    use crate::editor::generation2d::unit_tests::context::render;
    let bound = semio_framework_ui_contract::UI_RESIDENT_SURFACE_BYTES;
    let mut app = app_with_registry().await;
    for body_key in GENERATION2D_BODY_KEYS {
        let rendered = render(&mut app, body_key).await;
        println!("[STATS] surface body={body_key} bytes={} bound={bound}", rendered.len());
        assert!(!rendered.is_empty(), "{body_key} rendered empty");
        assert!(rendered.len() <= bound, "{body_key} is {} B, over the {bound} B resident surface bound", rendered.len());
    }
    close(app);
}
//#endregion 📏️SurfaceBudgetTests

//#region 🔖️ContextMenuTests
/// 🕹️ `context_menu` no longer has anything to dispatch a selection command WITH (`setSelection`
/// is deleted — selection is the framework's `graph` interaction domain now) and `context_menu`
/// itself carries no `InteractionView` to read it back even if it did (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, same discovered gap as `render`), so the
/// destructive `delete-selection` row — conditioned on a real selection — never appears; this test
/// now only pins the disclosure budget.
#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_disclosure_budget() {
    let mut app = app_with_registry().await;
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let items = app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await;
    close(app);
    assert!(items.len() <= 9, "top-level menu rows (leaves + groups + separator) must stay within disclosure budget, got {}", items.len());
    assert!(items.iter().all(|item| item.id != "delete-selection"), "no interaction data at context_menu time means delete-selection cannot appear");
}
//#endregion 🔖️ContextMenuTests

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn export_drawing_out_returns_vector_media() {
    let mut app = app().await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("drawing:out")).expect("export drawing:out");
    close(app);
    assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
}

#[semio_framework_async_macros::async_test]
async fn export_document_out_returns_flow_media() {
    let mut app = app().await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("artifact:out")).expect("export document:out");
    close(app);
    assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Flow });
    assert!(matches!(media.payload, semio_framework_plugin::MediaPayload::Structured { schema, .. } if schema == GENERATION_2D_SCHEMA));
}

#[semio_framework_async_macros::async_test]
async fn import_params_in_patches_matching_input_slider() {
    let mut app = app().await;
    crate::editor::generation2d::unit_tests::context::dispatch(&mut app, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None })).await;
    let slider_id = snapshot_read(&app)
        .host_snapshot
        .widgets
        .iter()
        .find_map(|widget| match widget {
            Widget::InputSlider { id, .. } => Some(id.clone()),
            _ => None,
        })
        .expect("just-added input slider");
    let media = semio_framework_plugin::Media {
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: serde_json::json!({ slider_id.clone(): 42.0 }).to_string() },
    };
    app.import_media("params:in", media, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("import params");
    let value = snapshot_read(&app).host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, .. } if id == &slider_id => Some(*value),
        _ => None,
    });
    close(app);
    assert_eq!(value, Some(42.0));
}

/// 🔐️ LAW: the app-owned resumable importer is the ONLY inbound media route, and it fails closed
/// on every payload it does not own — an unknown port, a non-structured payload and a non-object
/// JSON root all reject without publishing an operation.
#[semio_framework_async_macros::async_test]
async fn import_media_fails_closed_off_its_own_params_in_contract() {
    let structured = |json: &str| semio_framework_plugin::Media {
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: json.into() },
    };
    for (port, media) in [
        ("params:out", structured("{}")),
        ("params:in", structured("[1, 2]")),
        ("params:in", structured("not json")),
        (
            "params:in",
            semio_framework_plugin::Media {
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                payload: semio_framework_plugin::MediaPayload::Binary { format_kind: "bin".into(), blob_hash: String::new() },
            },
        ),
    ] {
        let mut app = app().await;
        let before = snapshot_read(&app).host_snapshot.widgets.len();
        let rejected = app.import_media(port, media, &semio_framework_plugin::artifact_app_laws::meta("local")).await;
        let after = snapshot_read(&app).host_snapshot.widgets.len();
        close(app);
        assert!(rejected.is_err(), "generation2d import must reject '{port}' instead of publishing");
        assert_eq!(after, before, "a rejected import must leave the document untouched");
    }
}

/// 🔐️ LAW: an import whose keys match no `InputSlider` publishes nothing and leaves every widget
/// exactly as it was — unmatched ids and non-numeric values are skipped, never faulted.
#[semio_framework_async_macros::async_test]
async fn import_params_skips_unmatched_keys_and_non_numeric_values() {
    let mut app = app().await;
    let before: Vec<String> = snapshot_read(&app).host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    let media = semio_framework_plugin::Media {
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: serde_json::json!({ "no-such-widget": 1.0, "another": "text" }).to_string() },
    };
    app.import_media("params:in", media, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("import params");
    let after: Vec<String> = snapshot_read(&app).host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    close(app);
    assert_eq!(after, before);
}

#[semio_framework_async_macros::async_test]
async fn media_ports_declare_params_in_and_drawing_out() {
    let ports = <Generation2dPlayApp as ArtifactEditor>::media_ports().await;
    assert!(ports.iter().any(|port| port.id == "artifact:in"));
    assert!(ports.iter().any(|port| port.id == "artifact:out"));
    let params_in = ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert_eq!(params_in.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
    let drawing_out = ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
    assert_eq!(drawing_out.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
    assert_eq!(drawing_out.kind_id.as_deref(), Some("2d.drawing"));
}

#[test]
fn generation2d_io_declares_the_params_and_drawing_ports() {
    let io = semio_framework::io::resolve_ready(generation2d_io());
    assert_eq!(io.artifact_schema, "generation.2d");
    let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert!(!params.required);
    let drawing = io.ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
    assert_eq!(drawing.kind_id.as_deref(), Some("2d.drawing"));
}
//#endregion 🔖️PortTests

//#region 📇️WindowActionLawTests
/// 📇️ THE window-kind action law, generation2d's half (ticket 26/09/09/PROCEDURAL-3D-END-TO-END) —
/// the exact twin of generation3d's `every_emitted_action_is_declared_on_its_window_kind`, kept
/// assertion-for-assertion identical so the two apps cannot drift.
///
/// Every action id a rendered `UiNode` binding of a window emits must appear in that window kind's
/// `WindowKindDefinition.actions` — that list is what `ShellHost`'s `declaredAction` gate reads before
/// it will call `plugin.handleAction`
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5691`) —
/// and no window may declare a window-scoped action only some other window emits, which is what makes
/// `.window_kind_action_refs(...)` mean anything against `build_definition`'s unowned-action fallback
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`).
///
/// The two generations are dispatched, not hand-built: the tree only emits
/// `selectGeneration`/`renameGeneration`/`removeGeneration` and the form only emits
/// `updateGenerationValues` once a roster exists.
#[semio_framework_async_macros::async_test]
async fn every_emitted_action_is_declared_on_its_window_kind() {
    let definition = create_generation2d_app();
    let windows: Vec<(String, String, std::collections::BTreeSet<String>)> =
        definition.window_kinds.iter().map(|kind| (kind.id.clone(), kind.body_key.clone(), kind.actions.iter().map(|action| action.id.clone()).collect())).collect();
    assert_eq!(windows.len(), 5, "generation2d declares five window kinds");
    let mut app = app_with_registry().await;
    for _ in 0..2 {
        crate::editor::generation2d::unit_tests::context::dispatch(&mut app, Generation2dCommand::AddGeneration(add_generation::AddGeneration {})).await;
    }
    let mut emitted: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = Default::default();
    for (kind_id, body_key, _) in &windows {
        let projection = crate::editor::generation2d::unit_tests::context::render(&mut app, body_key).await;
        emitted.insert(kind_id.clone(), crate::emitted_action_ids(&projection));
    }
    close(app);
    let window_scoped: std::collections::BTreeSet<String> = emitted.values().flatten().cloned().collect();
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

