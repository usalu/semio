//! 🏪 Replays real PDF graph and placement operations as one edit through the staged Store bundle.

use crate::native::png_source::{PNG_GRAPH_CHUNK_BYTES, PngGraphExtent, PngGraphSource, PngPlane, append_hex};
use crate::standards::v1_7::subsets::base::schema::snapshot::{
    ObjRef, PdfImage, PdfImageBody, PdfOp, PdfPage, PdfSnapshot,
};
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::{
    AppendPageContent, InsertObject, PdfMutation, SetImage,
};
use causal_source_overlay::binary::encode_envelopes;
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant, paged::ByteReadSource, retained_clone::retained_clone_grant_admits};
use spr_source_overlay::{
    ActorId, ArtifactDiff, ArtifactId, CausalPayload, ColdMutation, Edit, HybridLogicalTimestamp, InverseMutation,
    Mutation, MutationEnvelope, MutationMeta, MutationId, OpBinary, SchemaId, TransactionRef, UndoPolicy, VerifiedAtomicEdit, decode_verified_atomic_backbone,
    atomic_group::{ATOMIC_BACKBONE_TOOL, ATOMIC_EDIT_PREFIX_BYTES, AtomicBackbonePart, AtomicBackboneSections, AtomicBackboneSender},
};
use store_bundle_overlay::AtomicStoreReceive;
use semio_os_store_engine::lifted_store::ArtifactStore;
use store_operation_wire_overlay::{ArtifactOperationTargets, ArtifactOperationWire,
    original_edit::{ArtifactOriginalEditMessage, OriginalEditWireHeaders}};
use std::{future::Future, task::{Context, Poll, Waker}};

const INPUT: &[u8] = include_bytes!("🗑️generated/large-picked/large-picked.png");
const DOCUMENT: &str = "atomic-pdf-instance";
const ACTOR: &str = spr_source_overlay::LOCAL_ACTOR_ID;
const ID: &str = "edit-original-png";
const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 16, maximum_copy_bytes: 128 << 10,
    maximum_capacity_bytes: 128 << 10, maximum_release_bytes: 128 << 10, maximum_depth: 64 };
const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 28, maximum_copy_bytes: 128 << 20,
    maximum_capacity_bytes: 64 << 20, maximum_release_bytes: 64 << 20, ..TURN };

fn block<T: Future>(future: T) -> T::Output {
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("cold Store test yielded"),
    }
}

fn open_store(envelope: semio_os_store_space_schema::ArtifactEnvelope<PdfSnapshot, PdfMutation>)
    -> ArtifactStore<PdfSnapshot, PdfMutation> {
    use semio_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication;
    let mut store = block(ArtifactStore::new::<ArtifactSnapshotPublication>(
        envelope, ActorId(ACTOR.into()),
    )).unwrap();
    semio_os_store_space::lifted_store::install_unscheduled_catalog(
        &mut store, semio_os_store_engine_owners::funded_bounded_artifact_store_owners(),
    ).unwrap();
    store
}

fn close(store: &mut ArtifactStore<PdfSnapshot, PdfMutation>) {
    println!("[DEBUG] Starting paid Store close at generation {}", store.generation());
    drop(store.detach_backbone().unwrap());
    let mut wallet = AuthoringWallet::new(
        RetainedCloneGrant { maximum_items: 1 << 30, maximum_copy_bytes: 1 << 30,
            maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, ..TURN }, 1 << 34,
    );
    let mut idle = 0;
    let mut repeated = None;
    for turn in 0..(8 << 20) {
        if store.close_owned_terminal_is_empty() {
            println!("[DEBUG] Store original close reached terminal-empty after {turn} funded turns");
            return;
        }
        let demand = store.close_owned_demands(4096).unwrap();
        let funded = RetainedCloneGrant { maximum_items: TURN.maximum_items,
            maximum_copy_bytes: demand.copy_bytes.max(4096), maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let (step, _) = wallet.run_settled(funded, 1 << 16, |effect| store.close_owned_store_step(effect)).unwrap().unwrap();
        let progress = step.result.unwrap().progress();
        if turn != 0 && turn % 1_000_000 == 0 {
            println!("[DEBUG] Store generation {} retirement turn {turn}: {demand:?} {progress:?}", store.generation());
        }
        if repeated == Some(demand) && progress == Default::default() { idle += 1; } else { idle = 0; }
        repeated = Some(demand);
        assert!(idle < 16, "Store paid close stalled at turn {turn}, next {demand:?}, grant {funded:?}");
        wallet.post(progress).unwrap();
    }
    panic!("large PDF Store did not return its original paid owners");
}

fn sealed(points: &[MutationEnvelope], wallet: &mut AuthoringWallet) -> VerifiedAtomicEdit {
    let raw = encode_envelopes(points);
    let mut prefix = [0; ATOMIC_EDIT_PREFIX_BYTES];
    let end = AtomicBackboneSender::edit_prefix(ID, &mut prefix).unwrap();
    let message: Vec<_> = prefix[..end].iter().chain(raw.iter()).copied().collect();
    seal_wire(&message, wallet)
}

fn seal_wire<S: ByteReadSource>(message: &S, wallet: &mut AuthoringWallet) -> VerifiedAtomicEdit {
    let mut sender = AtomicBackboneSender::begin(message).unwrap();
    let mut receiver = None;
    while !sender.terminal_is_empty() {
        let mut outbound = None;
        let turn = wallet.run(TURN, 1 << 17, |effect| sender.advance(message, &mut outbound, effect)).unwrap().unwrap();
        wallet.post(turn.output.unwrap()).unwrap();
        let Some(frame) = outbound else { continue; };
        let (part, _) = AtomicBackbonePart::parse(&frame).unwrap();
        let receiver = receiver.get_or_insert_with(|| AtomicBackboneSections::new(part.key).unwrap());
        let mut owned = Some(CausalPayload::try_from_slice(&frame).unwrap());
        let turn = wallet.run(TURN, 4096, |effect| receiver.admit(part, &mut owned, effect)).unwrap().unwrap();
        wallet.post(turn.output.unwrap().1).unwrap();
        assert!(owned.is_none());
    }
    let mut receiver = receiver.unwrap();
    let mut digest = receiver.complete().unwrap();
    let group = loop {
        let result = wallet.run(TURN, 4096, |effect| digest.advance(&receiver, 4096, effect)).unwrap().unwrap().output.unwrap();
        wallet.post(result.1).unwrap();
        if let Some(bytes) = result.0 { break decode_verified_atomic_backbone(&bytes, DOCUMENT, ACTOR).unwrap(); }
    };
    while !receiver.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 1 << 16, |effect| receiver.close(4096, effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
    group
}

fn whole_document(_: &PdfMutation) -> Option<&dyn ArtifactOperationTargets> { None }

fn original_message(events: &[MutationEnvelope], wallet: &mut AuthoringWallet) -> CausalPayload {
    let forwards = events.iter().map(|event| PdfMutation::decode_op(&event.diff.payload.to_vec_owner()).unwrap()).collect();
    let inverse: Vec<_> = events.iter().map(|event| PdfMutation::decode_op(&event.inverse.payload.to_vec_owner()).unwrap()).collect();
    let mutate = |event: &MutationEnvelope| MutationMeta { mutation_id: Some(event.mutation_id.clone()),
        dependencies: event.dependencies.clone(), base_version: 0, author_id: Some(event.actor.clone()),
        timestamp: event.timestamp, undo_policy: UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None,
        label: None, group_id: None, origin: Default::default(), transaction: event.transaction.clone() };
    let mut edit = Some(Box::new(Edit { id: ID.into(), actor: Some(ACTOR.into()), line: None, forwards, inverse: inverse.into(),
        mutation_meta: events.iter().map(mutate).collect(), verb: Some("pick-image".into()),
        sequence_number: 0, started_at: String::new(), finished_at: None }));
    let wire = ArtifactOperationWire::semantic::<PdfSnapshot>().with_original_target(whole_document);
    let (mut owner, progress) = ArtifactOriginalEditMessage::admit(&mut edit, wire, TURN).unwrap().unwrap();
    wallet.post(progress).unwrap();
    let headers = OriginalEditWireHeaders { document: DOCUMENT, schema: &events[0].diff.schema.0, observed: None };
    let mut total = 0;
    let mut work = 0;
    let mut multi_item_turns = 0;
    let mut items_combined = 0;
    let mut effects = 0;
    let mut idle = 0;
    let mut repeated = None;
    for index in 0..1_048_576 {
        if owner.ready() { break; }
        let mut remaining = RetainedCloneGrant { maximum_copy_bytes: 4096, ..TURN };
        for _ in 0..64 {
            if owner.ready() { break; }
            let demand = owner.demands(4096).unwrap();
            if !retained_clone_grant_admits(remaining, demand) { break; }
            let grant = RetainedCloneGrant { maximum_items: if demand.capacity_bytes == 0 { remaining.maximum_items } else { 1 },
                maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes,
                maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            let result = wallet.run_invoked(grant, 1 << 16, |effect| owner.advance(headers, effect)).unwrap().unwrap();
            effects += 1;
            work += result.performed;
            let receipt = result.output.unwrap().progress();
            assert!(receipt.fits(remaining));
            remaining.maximum_items -= receipt.copied_items;
            remaining.maximum_copy_bytes -= receipt.copied_bytes;
            remaining.maximum_capacity_bytes -= receipt.retained_capacity_bytes;
            remaining.maximum_release_bytes -= receipt.released_bytes;
            total += receipt.copied_bytes;
            if receipt.copied_items > 1 {
                multi_item_turns += 1;
                items_combined += receipt.copied_items;
            }
            let signature = (demand, receipt);
            if repeated == Some(signature) && receipt.copied_bytes <= 14 { idle += 1; } else { idle = 0; }
            repeated = Some(signature);
            assert!(idle < 64, "original edit wire stalled at turn {index}, copied {total}, next {demand:?}, receipt {receipt:?}, wallet {:?}",
                wallet.remaining());
            wallet.post(receipt).unwrap();
            if receipt == Default::default() { break; }
        }
        if owner.ready() {
            println!("[DEBUG] Original edit encoder: {} Store turns, {effects} quoted effects, {work} native work units, {total} quoted copies, {items_combined} items in {multi_item_turns} coalesced effects", index + 1);
        }
    }
    assert!(owner.ready(), "original edit wire did not finish after {total} copied bytes");
    let (returned, wire, progress) = owner.take(TURN).unwrap().unwrap();
    wallet.post(progress).unwrap();
    assert_eq!(returned.id, ID);
    owner.begin_close();
    while !owner.terminal_is_empty() {
        let turn = wallet.run_invoked(TURN, 1 << 16, |effect| owner.close_step(4096, effect)).unwrap().unwrap();
        wallet.post(turn.output.unwrap().progress()).unwrap();
    }
    wire
}

fn operations(before: &PdfSnapshot, token: PngGraphExtent) -> (Vec<MutationEnvelope>, PdfSnapshot) {
    let mut ops: Vec<_> = INPUT.chunks(PNG_GRAPH_CHUNK_BYTES).enumerate().map(|(index, source)| {
        let mut hex = String::with_capacity(source.len() * 2);
        append_hex(&mut hex, source);
        PdfMutation::InsertObject(InsertObject { id: ObjRef { num: 1 + index as u32, gen: 0 },
            value: token.part(index, hex).unwrap(), index: None, admitted_stream_roles: None })
    }).collect();
    let mut color = PdfImage::rgb8("Color", 521, 257, Vec::new());
    color.body = PdfImageBody::Artifact { reference: PngPlane::Rgb.reference(token.digest) };
    color.soft_mask = Some("Mask".into());
    let mut mask = PdfImage::gray8("Mask", 521, 257, Vec::new());
    mask.body = PdfImageBody::Artifact { reference: PngPlane::Alpha.reference(token.digest) };
    ops.extend([PdfMutation::SetImage(SetImage { image: mask, index: None }),
        PdfMutation::SetImage(SetImage { image: color, index: None }),
        PdfMutation::AppendPageContent(AppendPageContent { index: 0, content: vec![
            PdfOp::Save, PdfOp::Transform { matrix: [90., 0., 0., 48., 12., 20.] },
            PdfOp::PaintXObject { name: "Color".into() }, PdfOp::Restore,
        ] })]);
    let mut after = before.clone();
    let schema = SchemaId(before.schema.clone());
    let mut envelopes = Vec::new();
    for (index, operation) in ops.into_iter().enumerate() {
        let forward = operation.encode_op().unwrap();
        let inverse = ColdMutation::inverse(&operation, &after).unwrap();
        after = semio_os_vcs::apply_mutation(&after, &operation).unwrap().0;
        assert_eq!(inverse.len(), 1);
        let meta = |bytes: &[u8]| CausalPayload::try_from_slice(bytes).unwrap();
        envelopes.push(MutationEnvelope {
            mutation_id: MutationId(format!("{ID}/{index}")),
            document_id: ArtifactId(DOCUMENT.into()), actor: ActorId(ACTOR.into()),
            dependencies: (index > 0).then(|| MutationId(format!("{ID}/{}", index - 1))).into_iter().collect(),
            observed: None,
            target: Mutation::conflict_target(&operation),
            diff: ArtifactDiff { schema: schema.clone(), payload: meta(&forward) },
            inverse: InverseMutation { schema: schema.clone(), payload: meta(&inverse[0].encode_op().unwrap()) },
            timestamp: HybridLogicalTimestamp { actor: 73, physical_ms: 1040, logical: index as u64 },
            transaction: Some(TransactionRef { id: "atomic-pick".into(), tool: format!("s.stdio.pdf#{ATOMIC_BACKBONE_TOOL}") }),
            verb: Some("pick-image".into()), line: None,
        });
    }
    (envelopes, after)
}

pub fn run() {
    let mut seed = PdfSnapshot::default();
    seed.pages.push(PdfPage::new(128., 128.));
    seed.pages.push(PdfPage::new(96., 96.));
    let mut hash = semio_framework_hash::Sha256::new();
    hash.update(INPUT);
    let token = PngGraphExtent { digest: hash.finalize(), length: INPUT.len() };
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 28);
    let envelope = semio_os_store_document::create_document_envelope::<_, PdfMutation>(&seed.schema, DOCUMENT, seed.clone(), None,
        &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut receiver = open_store(envelope);
    use semio_os_store_space_schema::ArtifactCommand;
    let dispatch = semio_os_store_test_support::dispatch_test_command;
    let local = PdfMutation::AppendPageContent(AppendPageContent {
        index: 1, content: vec![PdfOp::Save, PdfOp::Restore],
    });
    let local_id = "fixture-open-tool";
    let draft = semio_os_vcs::apply_mutation(&seed, &local).unwrap().0;
    block(dispatch(&mut receiver, ArtifactCommand::AppendTransaction {
        mutations: vec![local.clone()], transaction: TransactionRef { id: local_id.into(), tool: "fixture.draft".into() },
    })).unwrap();
    let open = receiver.envelope().open_transaction.clone().unwrap();
    assert_eq!(open.transaction.id, local_id);
    assert_eq!(receiver.snapshot().unwrap(), draft);
    let generation = receiver.generation();
    let (events, expected) = operations(&seed, token);
    assert_eq!(events.len(), 18);
    let mut waiting = events.clone();
    waiting[0].dependencies.push(MutationId("missing-predecessor".into()));
    let incomplete = sealed(&waiting, &mut wallet);
    assert!(matches!(block(receiver.ingest_original_atomic(&incomplete)).unwrap(), AtomicStoreReceive::AwaitingDependencies));
    assert_eq!(receiver.snapshot().unwrap(), draft);
    assert_eq!(receiver.generation(), generation);
    assert_eq!(receiver.envelope().open_transaction.as_ref().unwrap().edit_id, open.edit_id);
    assert_eq!(receiver.replay_edits().len(), 1);
    assert!(!receiver.replay_edits().iter().any(|edit| edit.id == ID));

    let complete = if std::env::args().any(|arg| arg == "binary") {
        sealed(&events, &mut wallet)
    } else {
        let authored_wire = original_message(&events, &mut wallet);
        assert!(authored_wire.len() > 262_144);
        seal_wire(&authored_wire, &mut wallet)
    };
    let outcome = block(receiver.ingest_original_atomic(&complete)).unwrap();
    assert!(matches!(outcome, AtomicStoreReceive::Finished(report) if report.accepted));
    assert_eq!(receiver.envelope().open_transaction.as_ref().unwrap().edit_id, open.edit_id);
    assert_eq!(receiver.snapshot().unwrap(), semio_os_vcs::apply_mutation(&expected, &local).unwrap().0);
    assert_eq!(receiver.replay_edits().iter().filter(|edit| edit.id == ID).count(), 1);
    assert_eq!(receiver.replay_edits().iter().last().unwrap().id, open.edit_id);
    block(dispatch(&mut receiver, ArtifactCommand::AbortTransaction { transaction_id: local_id.into() })).unwrap();
    assert!(receiver.envelope().open_transaction.is_none());
    let actual = receiver.snapshot().unwrap();
    assert_eq!(actual, expected);
    assert_eq!(receiver.replay_edits().len(), 1);
    println!("[DEBUG] Authenticated group committed once under the local open transaction, while an unresolved gap left its exact Store revision and local draft intact");
    assert_eq!(receiver.replay_edits().iter().next().unwrap().id, ID);
    assert_eq!(receiver.replay_edits().iter().next().unwrap().forwards.len(), 18);
    let (_, source) = PngGraphSource::resolve(&actual.objects, &PngPlane::Alpha.reference(token.digest)).unwrap();
    assert_eq!(source.byte_len(), INPUT.len());
    let mut comparison = vec![0; INPUT.len()];
    assert_eq!(source.read_bytes(0, &mut comparison), INPUT.len());
    assert_eq!(comparison, INPUT);
    println!("[DEBUG] Store accepted one 18-operation PDF edit and reconstructed every source byte");
    let archive = block(semio_os_store_text_io::text_format::print_document_pack(receiver.envelope())).unwrap();
    println!("[DEBUG] Printed Store archive {}+{} bytes", archive.pack.len(), archive.spr.len());
    assert!(archive.pack.len() + archive.spr.len() < spr_source_overlay::DOCUMENT_ARCHIVE_MAXIMUM_BYTES);
    let parsed = block(semio_os_store_text_io::text_format::parse_document_pack::<PdfSnapshot, PdfMutation>(
        &archive.pack, &archive.spr, &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    )).unwrap();
    println!("[DEBUG] Replayed Store archive into its original PDF graph");
    assert_eq!(parsed.snapshot, actual);
    let mut reopened = open_store(parsed.into_envelope());
    println!("[DEBUG] Reopened snapshot and history as one live Store owner");
    assert_eq!(reopened.snapshot().unwrap(), actual);
    assert_eq!(reopened.replay_edits().len(), 1);
    block(dispatch(&mut reopened, ArtifactCommand::Undo)).unwrap();
    println!("[DEBUG] Undid the grouped original image");
    assert_eq!(reopened.snapshot().unwrap(), seed);
    block(dispatch(&mut reopened, ArtifactCommand::Redo)).unwrap();
    println!("[DEBUG] Redid the grouped original image");
    assert_eq!(reopened.snapshot().unwrap(), actual);
    close(&mut reopened);
    close(&mut receiver);
    println!("[DEBUG] Exactly one Store edit committed all 473683 PNG bytes, mask/color, and placement through archive reopen and undo/redo");
}
