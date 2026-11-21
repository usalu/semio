"""📟 Stages the mounted original document port and source-owned wake/close routing."""

import hashlib
import json
from pathlib import Path
import runpy

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
REACTOR = PLUGIN / "⚛️reactor"
OPS = REACTOR / "🏃️runtime/🧰️ops/📄️document/🦀️.rs"
PUMP = OPS.with_name("📥️atomic-receive.rs")
APP = REACTOR / "🔌️app/🦀️.rs"
APP_WORK = APP.with_name("📟️atomic-document.rs")
INSTANCE = PLUGIN / "🧵️instance/🔌️plugin-app-impl/🦀️.rs"
REGISTRY = REACTOR / "🏃️runtime/📥️staging/♻️reactor-lifecycle/🗃️instance-registry/🦀️.rs"
CLEANUP = REGISTRY.parent.with_name("🧯️cleanup-pumps") / "🦀️.rs"
TURN = REACTOR / "🔄️turn/🌀️poll-kernel-turn/🦀️.rs"
EVENT = REACTOR / "🔄️turn/📨️event-dispatch/🦀️.rs"
STORE = REACTOR.parents[0].parent / "🏪️store/⚙️engine/📥️lifted/🏪️store/↩️supersede-undo-redo-and-backbone-pump/🦀️.rs"
STORE_DRIVER = STORE.parent.with_name("📑️delegating-operations-and-publication") / "🦀️.rs"
STORE_FACADE = STORE.parents[4] / "🦀️.rs"
OUT = TICKET / "🗑️generated/raw-reactor-inbound-overlay.json"


def prepare_atomic_store(source):
    entry = "pub async fn ingest_verified_atomic_edit<P, M>("
    prepared = source.replace(entry, """pub async fn ingest_verified_atomic_edit<P, M>(
    store: &mut StoreParts<'_, P, M>, incoming: &VerifiedAtomicEdit,
) -> Result<AtomicStoreReceive, VcsError>
where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
    prepare_verified_atomic_edit(store, incoming).await?.commit(store, incoming.edit_id()).await
}

enum AtomicStoreCandidate<M> {
    AwaitingDependencies,
    AlreadyApplied(semio_os_spr::MergeReport),
    Ready { dag: MutationDag, edits: Vec<Edit<M>>, transitions: Vec<MutationEnvelope>, clock: semio_os_spr::HybridLogicalTimestamp },
}

/// 🔏️ Owns the complete private replay candidate until the Store can commit its one authored edit.
pub struct AtomicStorePreparation<M>(AtomicStoreCandidate<M>);

impl<M> AtomicStorePreparation<M> {
    pub fn will_commit(&self) -> bool { matches!(self.0, AtomicStoreCandidate::Ready { .. }) }

    pub fn discard<P>(self, store: &mut StoreParts<'_, P, M>) -> Result<(), VcsError>
    where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
        if let AtomicStoreCandidate::Ready { dag, edits, transitions, .. } = self.0 {
            retire_scratch_edits::<P, M>(edits);
            retire_uncommitted(store, dag, transitions)?;
        }
        Ok(())
    }

    pub async fn commit<P>(self, store: &mut StoreParts<'_, P, M>, authored: &str) -> Result<AtomicStoreReceive, VcsError>
    where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
        match self.0 {
            AtomicStoreCandidate::AwaitingDependencies => Ok(AtomicStoreReceive::AwaitingDependencies),
            AtomicStoreCandidate::AlreadyApplied(report) => Ok(AtomicStoreReceive::Finished(report)),
            AtomicStoreCandidate::Ready { dag, edits, transitions, clock } => {
                let report = commit_ready_remote_batch(store, dag, edits, transitions, Some(authored)).await?;
                if report.accepted { store.log.clock = clock; }
                Ok(AtomicStoreReceive::Finished(report))
            }
        }
    }
}

/// 🧩️ Checks the entire causal group before changing a live transaction or publishing history.
pub async fn prepare_verified_atomic_edit<P, M>(""")
    old = ") -> Result<AtomicStoreReceive, VcsError>\nwhere P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {\n    ensure_durable_group_idle(store)?;"
    new = old.replace("AtomicStoreReceive", "AtomicStorePreparation<M>")
    assert prepared.count(old) == 1
    prepared = prepared.replace(old, new)
    edits = [
        ("return Ok(AtomicStoreReceive::Finished(semio_os_spr::MergeReport {", "return Ok(AtomicStorePreparation(AtomicStoreCandidate::AlreadyApplied(semio_os_spr::MergeReport {"),
        ("            replayed: Vec::new(), worst: None, conflict: None,\n        }));", "            replayed: Vec::new(), worst: None, conflict: None,\n        })));"),
        ("        return Ok(AtomicStoreReceive::AwaitingDependencies);", "        return Ok(AtomicStorePreparation(AtomicStoreCandidate::AwaitingDependencies));"),
        ("""    let report = commit_ready_remote_batch(store, candidate, batch, transitions, Some(incoming.edit_id())).await?;
    if report.accepted { store.log.clock = clock; }
    Ok(AtomicStoreReceive::Finished(report))""",
         "    Ok(AtomicStorePreparation(AtomicStoreCandidate::Ready { dag: candidate, edits: batch, transitions, clock }))"),
    ]
    for prior, replacement in edits:
        assert prepared.count(prior) == 1, prior
        prepared = prepared.replace(prior, replacement)
    return prepared

def stage():
    stage = runpy.run_path(str(TICKET / "raw-store-inbox-stage.py"))
    stage["stage"]()
    data = json.loads(stage["OUT"].read_text(encoding="utf8"))
    pre, post = data["preimages"], data["candidate"]

    def edit(path, anchor, value):
        body = (ROOT / path).read_bytes()
        pre[str(path)] = hashlib.sha256(body).hexdigest()
        current = post.get(str(path), body.decode("utf8"))
        assert current.count(anchor) == 1, (path, anchor, current.count(anchor))
        post[str(path)] = current.replace(anchor, value)

    assert not (ROOT / PUMP).exists()
    pre[str(PUMP)] = None
    post[str(PUMP)] = (TICKET / "raw-reactor-inbound-pump.rs").read_text(encoding="utf8")
    edit(OPS, "extern crate semio_os_kernel as protocol;", """extern crate semio_os_kernel as protocol;

#[path = "📥️atomic-receive.rs"]
mod atomic_receive;
pub use atomic_receive::{advance_atomic_document_backbone, prepare_atomic_document_turn, retains_atomic_document_backbone, retire_atomic_document_backbone};""")
    edit(REGISTRY, "    pub channel: Option<store::os_store::backbone::ActorBackboneChannelOwner>,",
         "    pub channel: Option<store::os_store::backbone::ActorBackboneChannelOwner>,\n    pub atomic: store::os_store::backbone::atomic_transport::AtomicDocumentBackboneWork,")
    edit(REGISTRY, "    pub document_backbones: RefCell<RuntimeInstanceRegistry<RuntimeDocumentBackboneBindingV1>>,",
         "    pub document_backbones: RefCell<RuntimeInstanceRegistry<RuntimeDocumentBackboneBindingV1>>,\n    pub document_backbone_turn_cursor: Cell<usize>,")
    edit(REGISTRY, "            document_backbones: RefCell::new(RuntimeInstanceRegistry::new()),",
         "            document_backbones: RefCell::new(RuntimeInstanceRegistry::new()),\n            document_backbone_turn_cursor: Cell::new(0),")
    edit(OPS, "RuntimeDocumentBackboneBindingV1 { generation: command.binding_generation, uri: Some(command.uri.clone()), channel: Some(owner) }",
         "RuntimeDocumentBackboneBindingV1 { generation: command.binding_generation, uri: Some(command.uri.clone()), channel: Some(owner), atomic: Default::default() }")
    edit(CLEANUP, "    let generation = checked_runtime_close_generation(runtime.close_generation.get())?;",
         """    if document_backbones.get(instance_id).is_some_and(|binding| !binding.atomic.terminal()) {
        return Err(plugin_internal_fault("document backbone original receive must retire before native instance removal"));
    }
    let generation = checked_runtime_close_generation(runtime.close_generation.get())?;""")
    edit(APP, "    async fn detach_backbone(&mut self) -> Result<(), Fault>;",
         """    /// 🔌️ Advances only the original group held by the addressed document receiver.
    async fn advance_original_atomic_backbone(
        &mut self,
        _inbox: &mut semio_os_kernel::os_store::backbone::atomic_transport::AtomicBackboneInbox,
        _original: &mut Option<Vec<u8>>,
        _binding: [u8; 32],
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<Option<protocol::MergeReport>, semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("plugin.atomic-backbone-unsupported"), "this app has no original atomic document receiver").into())
    }
    fn original_atomic_outbound_pending(&self) -> bool { false }
    fn advance_original_atomic_outbound(
        &mut self,
        _delivery: &mut Option<semio_os_kernel::os_store::backbone::atomic_transport::AtomicBackboneDelivery>,
        _binding: [u8; 32],
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<(), semio_framework_job::OriginalTurnFailure> { Ok(()) }
    async fn detach_backbone(&mut self) -> Result<(), Fault>;""")
    prefix = (ROOT / APP).read_text(encoding="utf8").splitlines()[0]
    edit(APP, prefix, prefix + '\n\n#[path = "📟️atomic-document.rs"]\npub mod atomic_document;')
    pre[str(APP_WORK)] = None
    post[str(APP_WORK)] = (TICKET / "raw-reactor-backbone-turn.rs").read_text(encoding="utf8")
    edit(INSTANCE, "    async fn detach_backbone(&mut self) -> Result<(), Fault> {",
         (TICKET / "raw-reactor-inbound-app-method.rs").read_text(encoding="utf8") + "\n    async fn detach_backbone(&mut self) -> Result<(), Fault> {")
    edit(STORE_FACADE, "pub use semio_os_store_engine::lifted_store::{ArtifactStore,",
         "pub use semio_os_store_engine::AtomicStoreReceive;\npub use semio_os_store_engine::lifted_store::{ArtifactStore,")
    edit(STORE, """    /// 📥 Passes a complete authenticated multipart edit through one Store history publication.""",
         """    /// 📥 Passes a verified group through one Store history publication.""")
    edit(STORE, """        if self.log.envelope.open_transaction.is_some() {
            return Ok(semio_os_store_engine_bundle::AtomicStoreReceive::AwaitingDependencies);
        }
        semio_os_store_engine_bundle::ingest_verified_atomic_edit(&mut self.parts(), group).await""",
         """        let prepared = semio_os_store_engine_bundle::prepare_verified_atomic_edit(&mut self.parts(), group).await?;
        let open = if prepared.will_commit() {
            match self.lift_open_transaction().await {
                Ok(open) => open,
                Err(error) => { prepared.discard(&mut self.parts())?; return Err(error); }
            }
        } else { None };
        let report = prepared.commit(&mut self.parts(), group.edit_id()).await;
        if let Some((open, edit)) = open { self.restore_open_transaction(open, edit).await?; }
        report""")
    edit(STORE, """    /// 🛟️ The sole public remote write gate ([`Self::ingest_remote_merge`]). While a tool transaction is open its
    /// unannounced edit is lifted off the applied tail for the merge and folded back onto the merged head after it, so a
    /// remote edit never reorders, quarantines or degrades it and it stays the tail its commit announces.""",
         """    /// 🛟️ Folds a remote edit before restoring any open local tool transaction to the applied tail.""")
    edit(STORE, """    /// 🪞️ Delegates to [`semio_os_store_engine_bundle::ingest_remote_merge`]: feeds one remote envelope through the causal DAG and merges it into the edit timeline.""",
         """    /// 🪞️ Folds one remote envelope through the causal edit timeline.""")
    edit(STORE, """    /// 📥️ Pumps every queued inbound message from the attached backbone into the timeline.""",
         """    /// 📥️ Folds the attached backbone's queued messages.""")
    edit(STORE, """    /// 🪆️ A member lane names a document this store does not own; only the composing app can
    /// resolve `(slot, child_id)` to a live member, so the message waits for its owner.
    ///
    /// A store never consumes acks (they flow store→actor); drain and ignore any that echo back.""",
         """    /// 🪆️ Folds this document's queue, leaving member lanes to their owning app.
    /// Drops echoed acknowledgements, which only flow Store to actor.""")
    bundle = STORE.parents[3] / "🧳️bundle"
    edit(bundle / "🦀️.rs", 
         "pub use atomic_ingest::{AtomicStoreReceive, ingest_verified_atomic_edit};",
         "pub use atomic_ingest::{AtomicStoreReceive, ingest_verified_atomic_edit, prepare_verified_atomic_edit};")
    ingest = bundle / "🧩️atomic-ingest.rs"
    before = (ROOT / ingest).read_text(encoding="utf8")
    pre[str(ingest)] = hashlib.sha256(before.encode("utf8")).hexdigest()
    post[str(ingest)] = prepare_atomic_store(before)
    edit(STORE_DRIVER, """    /// 📤️ Exposes only the completed next physical frame through the authenticated attached endpoint.
    pub async fn send_ready_atomic_outbound(
        &mut self, binding: &[u8; 32],
        delivery: &mut semio_os_store_backbone::atomic_transport::AtomicBackboneDelivery,
    ) -> Result<bool, VcsError> {
        delivery.check_binding(binding).map_err(VcsError::InverseRefused)?;
        let Some(backbone) = self.wire.backbone.as_mut() else { return Ok(false); };
        delivery.send_ready(backbone).await
    }""", """    /// 📤️ Advances and donates the exact owned wire to the authenticated actor Port.
    pub fn advance_bound_atomic_outbound(
        &mut self, binding: [u8; 32], delivery: &mut Option<semio_os_store_backbone::atomic_transport::AtomicBackboneDelivery>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<Option<RetainedCloneProgress>, VcsError> {
        if let (Some(port), Some(owner)) = (self.wire.backbone.as_ref(), delivery.as_mut().filter(|owner| owner.needs_actor_wire())) {
            owner.check_binding(&binding).map_err(VcsError::InverseRefused)?;
            return owner.advance_actor_wire(port, effect).map(Some);
        }
        self.advance_atomic_outbound(binding, delivery, effect).map_err(VcsError::InverseRefused)
    }""")
    edit(EVENT, "                let output = semio_plugin_runtime_ops::ops_document::plugin_receive_document_backbone",
         """                if semio_os_kernel::os_store::backbone::atomic_transport::AtomicBackboneInbox::recognizes_hot(&payload) {
                    semio_plugin_runtime_ops::ops_document::retains_atomic_document_backbone(runtime, &uri, payload)?;
                    continue;
                }
                let output = semio_plugin_runtime_ops::ops_document::plugin_receive_document_backbone""")
    edit(OPS, "pub async fn plugin_handle_document_backbone_binding<PA:",
         """/// 🔓️ Settles a matching retire command's paid ingress custody before the exact existing binding closes.
pub fn prepare_atomic_document_detach<PA: PluginApp>(runtime: &PluginRuntime<PA>, source_instance: &str, payload: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    use semio_plugin_actor_host::document_backbone_binding::{decide_document_backbone_binding_v1, decode_document_backbone_binding_command_v1, DocumentBackboneBindingDecisionV1};
    let Some(command) = decode_document_backbone_binding_command_v1(payload).map_err(plugin_internal_fault)? else { return Ok(true); };
    if source_instance.parse::<u32>().ok() != Some(command.instance_id) { return Ok(true); }
    if !runtime.guest_lifetimes.try_borrow().map_err(|_| plugin_internal_fault("lifecycle authority busy"))?
        .get(command.instance_id).is_some_and(|slot| slot.cell.is_live()) { return Ok(true); }
    let valid = {
        let bindings = runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
        let state = bindings.get(command.instance_id).map(RuntimeDocumentBackboneBindingV1::state).unwrap_or_default();
        matches!(decide_document_backbone_binding_v1(&state, &command), DocumentBackboneBindingDecisionV1::Retire(_))
    };
    if !valid { return Ok(true); }
    retire_atomic_document_backbone(runtime, command.instance_id, cx)
}
pub async fn plugin_handle_document_backbone_binding<PA:""")
    edit(TURN, "    let focus = admit_lifecycle_focus(runtime, events)?;",
         """    let (prior_port_retired, offer_atomic_outbound) =
        semio_plugin_runtime_ops::ops_document::prepare_atomic_document_turn(runtime, events.as_ref(), cx)?;
    let focus = if prior_port_retired { admit_lifecycle_focus(runtime, events)? } else { None };""")
    edit(TURN, "    let selected_event = select_turn_event(runtime, events, cx)?;",
         "    let selected_event = if prior_port_retired { select_turn_event(runtime, events, cx)? } else { None };")
    edit(TURN, "    trace_turn_phase_retention(\"ingress\");",
         """    let atomic_document_work = if prior_port_retired {
        let (pending, output) = semio_plugin_runtime_ops::ops_document::advance_atomic_document_backbone(runtime, offer_atomic_outbound, cx).await?;
        if let Some(output) = output {
            dirty_document_surfaces(runtime, output.instance_id, &mut dirty).await?;
            for frame in output.frames { route_app_frame(output.instance_id, &frame, &mut effects); }
            effects.extend(output.effects);
        }
        pending
    } else { true };

    trace_turn_phase_retention("ingress");""")
    edit(TURN, "    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };",
         "    let status = if more_work || atomic_document_work { TurnStatus::MoreWork } else { TurnStatus::Idle };")
    runpy.run_path(str(TICKET / "raw-reactor-inbound-room.py"))["stage"](ROOT, edit)
    OUT.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(post)} original document backbone source scheduler postimages")


if __name__ == "__main__":
    stage()
