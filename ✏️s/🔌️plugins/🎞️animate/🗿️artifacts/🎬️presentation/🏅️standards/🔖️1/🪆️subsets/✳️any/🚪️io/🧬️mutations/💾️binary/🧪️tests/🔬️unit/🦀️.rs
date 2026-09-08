
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::standards::v1::subsets::any::schema::mutations::{create_tile, replace_tiles};
use store::{ArtifactCommand, os_store::test_support};

struct PresentationProjectionFixtureTarget {
    value: Option<PresentationSnapshot>,
}

impl PresentationProjectionAdoptionTarget for PresentationProjectionFixtureTarget {
    fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot> {
        if self.value.is_some() {
            return Err(value);
        }
        self.value = Some(value);
        Ok(())
    }
}

struct PresentationProjectionBackpressureTarget {
    reject_once: bool,
    value: Option<PresentationSnapshot>,
}

impl PresentationProjectionAdoptionTarget for PresentationProjectionBackpressureTarget {
    fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot> {
        if std::mem::take(&mut self.reject_once) || self.value.is_some() {
            return Err(value);
        }
        self.value = Some(value);
        Ok(())
    }
}

fn presentation_envelope_test_pages(snapshot_hex: &str) -> store::OwnedSchemaDecodePages {
    let json = format!("{{\"schema\":\"{PRESENTATION_DOCUMENT_SCHEMA}\",\"id\":\"deck-1\",\"vcs\":{{\"initialSnapshot\":\"{snapshot_hex}\",\"edits\":[],\"changes\":[],\"checkpoints\":[],\"alternatives\":[]}},\"editMessages\":[],\"conflicts\":[]}}");
    presentation_envelope_json_test_pages(&json)
}

fn presentation_envelope_json_test_pages(json: &str) -> store::OwnedSchemaDecodePages {
    let chunks = json.as_bytes().chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: json.len() }).expect("exact test page credits");
    for chunk in chunks {
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded test page")).unwrap_or_else(|_| panic!("pre-admitted page"));
    }
    pages.seal().expect("sealed test pages");
    pages
}

fn close_presentation_snapshot(value: PresentationSnapshot) {
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&PresentationFreshSnapshotRetirementFactory, value);
    assert!(matches!(retirement.close_step(1, PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES), Ok(store::SnapshotRetirementStep::Pending { .. })));
    assert_eq!(retirement.close_step(1, PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES).expect("fixture retirement"), store::SnapshotRetirementStep::Complete);
    assert!(retirement.terminal_is_empty());
    drop(retirement);
}

fn close_presentation_pages(mut pages: store::OwnedSchemaDecodePages) {
    while pages.close_take_page().is_some() {}
    assert!(pages.terminal_is_empty());
    drop(pages);
}

fn close_presentation_registry(registry: &mut PresentationEnvelopeMaterializeRegistry, pool: &semio_framework_job::WorkerPool) {
    for _ in 0..100_000 {
        if registry.close_next_step(pool, 1, PRESENTATION_ENVELOPE_SNAPSHOT_PACK_BYTES).expect("bounded registry close") == store::SnapshotRetirementStep::Complete {
            assert!(registry.terminal_is_empty());
            return;
        }
    }
    panic!("Presentation registry did not reach terminal empty within the fixed fixture ceiling");
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = PresentationMutation::ReplaceTiles(replace_tiles::ReplaceTiles { new_tiles: Vec::new() });
    test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn envelope_helpers_round_trip() {
    let snapshot = empty_presentation_snapshot();
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let operation = semio_framework_job::OperationId(7001);
    let generation = semio_framework_job::Generation(3);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
    let mut target = PresentationProjectionFixtureTarget { value: None };
    for _ in 0..10_000 {
        match registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") {
            PresentationEnvelopeMaterializeHandleStep::Pending | PresentationEnvelopeMaterializeHandleStep::Progress => {}
            PresentationEnvelopeMaterializeHandleStep::Ready => {
                assert!(registry.try_publish_to(operation, generation, &mut target).expect("completion lock is uncontended"));
                break;
            }
            outcome => panic!("valid Presentation envelope caller produced {outcome:?}"),
        }
    }
    assert!(registry.terminal_is_empty());
    drop(registry);
    let deck = target.value.take().expect("typed projection published exactly once");
    assert_eq!(deck.schema, PRESENTATION_DOCUMENT_SCHEMA);
    assert!(crate::presentation_working_scene(&deck).1.is_empty());
    close_presentation_snapshot(deck);
}

#[semio_framework_async_macros::async_test]
async fn retained_presentation_envelope_materializes_populated_history_in_order() {
    let snapshot = empty_presentation_snapshot();
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let mutation_value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&PresentationMutation::ReplaceTiles(replace_tiles::ReplaceTiles { new_tiles: Vec::new() })));
    let mutation = dsl::os_pack::json::to_string(&mutation_value);
    let json = format!(
        "{{\"schema\":\"{PRESENTATION_DOCUMENT_SCHEMA}\",\"id\":\"deck-history\",\"vcs\":{{\"initialSnapshot\":\"{hex}\",\"edits\":[{{\"id\":\"edit-1\",\"forwards\":[{mutation}],\"inverse\":[],\"sequenceNumber\":1,\"startedAt\":\"1\"}}],\"changes\":[],\"checkpoints\":[],\"alternatives\":[]}},\"editMessages\":[],\"conflicts\":[]}}"
    );
    let operation = semio_framework_job::OperationId(7007);
    let generation = semio_framework_job::Generation(9);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_json_test_pages(&json)).unwrap_or_else(|_| panic!("populated retained caller was pre-admitted"));
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
    let mut target = PresentationProjectionFixtureTarget { value: None };
    for _ in 0..20_000 {
        match registry.maintenance_step(operation, generation, generation, &pool).expect("exact populated caller") {
            PresentationEnvelopeMaterializeHandleStep::Pending | PresentationEnvelopeMaterializeHandleStep::Progress => {}
            PresentationEnvelopeMaterializeHandleStep::Ready => {
                assert!(registry.try_publish_to(operation, generation, &mut target).expect("populated output publication"));
                break;
            }
            outcome => panic!("populated Presentation history produced {outcome:?}"),
        }
    }
    assert!(registry.terminal_is_empty());
    drop(registry);
    let deck = target.value.take().expect("populated history published exactly once");
    assert!(crate::presentation_working_scene(&deck).1.is_empty());
    close_presentation_snapshot(deck);
}

#[semio_framework_async_macros::async_test]
async fn retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack() {
    let operation = semio_framework_job::OperationId(7002);
    let generation = semio_framework_job::Generation(4);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_test_pages("00")).unwrap_or_else(|_| panic!("sealed malformed pages remain retained"));
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
    for _ in 0..10_000 {
        if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Fault {
            break;
        }
    }
    assert!(registry.fault(operation, generation).expect("exact fault owner").is_some());
    assert_eq!(registry.close_step(operation, generation, &pool, 0, 0).expect("zero grant preserves the exact fault owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    close_presentation_registry(&mut registry, &pool);
    assert!(registry.terminal_is_empty());
    drop(registry);
}

#[semio_framework_async_macros::async_test]
async fn retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output() {
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
    let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let operation = semio_framework_job::OperationId(7003);
    let generation = semio_framework_job::Generation(5);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
    registry.cancel(operation, generation).expect("exact live caller");
    for _ in 0..10_000 {
        if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Cancelled {
            break;
        }
    }
    assert_eq!(registry.close_step(operation, generation, &pool, 0, 0).expect("zero grant preserves the exact cancelled job"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    close_presentation_registry(&mut registry, &pool);
    assert!(registry.terminal_is_empty());
    drop(registry);
}

#[semio_framework_async_macros::async_test]
async fn retained_presentation_envelope_registry_preserves_collision_capacity_and_exact_rejected_pages() {
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
    let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let generation = semio_framework_job::Generation(9);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    for index in 0..PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY {
        registry.try_submit(semio_framework_job::OperationId(8_000 + index as u64), generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("every fixed registry slot admits exactly once"));
    }
    let collision = semio_framework_job::OperationId(8_000 + PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY as u64);
    let pages = presentation_envelope_test_pages(&hex);
    let rejected_page_count = pages.page_count();
    let rejected_byte_count = pages.byte_count();
    let (fault, pages) = registry.try_submit(collision, generation, pages).expect_err("capacity +1 returns the exact caller pages");
    assert_eq!(fault, PresentationEnvelopeMaterializeRegistryFault::Capacity);
    assert_eq!(pages.page_count(), rejected_page_count);
    assert_eq!(pages.byte_count(), rejected_byte_count);
    close_presentation_pages(pages);
    let duplicate = semio_framework_job::OperationId(8_000);
    let (fault, pages) = registry.try_submit(duplicate, generation, presentation_envelope_test_pages(&hex)).expect_err("duplicate operation never replaces its live owner");
    assert_eq!(fault, PresentationEnvelopeMaterializeRegistryFault::Collision);
    close_presentation_pages(pages);
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
    close_presentation_registry(&mut registry, &pool);
    drop(registry);
}

#[semio_framework_async_macros::async_test]
async fn retained_presentation_envelope_wrong_owner_abort_and_interrupted_close_preserve_live_slot() {
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
    let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let operation = semio_framework_job::OperationId(8_050);
    let generation = semio_framework_job::Generation(13);
    let wrong_generation = semio_framework_job::Generation(14);
    let wrong_operation = semio_framework_job::OperationId(operation.0 + PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY as u64);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("one exact retained caller is admitted"));
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));

    assert_eq!(registry.cancel(operation, wrong_generation), Err(PresentationEnvelopeMaterializeRegistryFault::Stale));
    assert_eq!(registry.cancel(wrong_operation, generation), Err(PresentationEnvelopeMaterializeRegistryFault::Stale));
    assert!(registry.close_step(operation, wrong_generation, &pool, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).is_err());
    assert!(registry.fault(operation, generation).is_ok(), "wrong-owner probes preserve the exact live slot");

    registry.cancel(operation, generation).expect("the exact owner may still abort");
    assert_eq!(registry.close_step(operation, generation, &pool, 0, 0).expect("zero-grant interrupted close"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    close_presentation_registry(&mut registry, &pool);
    assert!(registry.terminal_is_empty());
    drop(registry);
}

#[semio_framework_async_macros::async_test]
async fn retained_presentation_envelope_publication_retries_backpressure_exactly_once() {
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
    let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let operation = semio_framework_job::OperationId(8_100);
    let generation = semio_framework_job::Generation(10);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
    for _ in 0..10_000 {
        if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Ready {
            break;
        }
    }
    let mut target = PresentationProjectionBackpressureTarget { reject_once: true, value: None };
    assert!(!registry.try_publish_to(operation, generation, &mut target).expect("first publication preserves backpressure owner"));
    assert_eq!(registry.maintenance_step(operation, generation, generation, &pool).expect("backpressured caller remains exact"), PresentationEnvelopeMaterializeHandleStep::Ready);
    assert!(registry.try_publish_to(operation, generation, &mut target).expect("second publication atomically succeeds"));
    assert!(registry.terminal_is_empty());
    close_presentation_snapshot(target.value.take().expect("one exact output publication"));
    drop(registry);
}

#[semio_framework_async_macros::async_test]
async fn retained_presentation_envelope_stale_generation_cancels_and_unpublished_output_closes() {
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(&empty_presentation_snapshot());
    let hex = pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let operation = semio_framework_job::OperationId(8_200);
    let generation = semio_framework_job::Generation(11);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("sealed fixed-page caller"));
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::InteractiveNative, 1));
    for _ in 0..10_000 {
        let step = registry.maintenance_step(operation, generation, semio_framework_job::Generation(12), &pool).expect("exact stale caller remains retained");
        if matches!(step, PresentationEnvelopeMaterializeHandleStep::Cancelled | PresentationEnvelopeMaterializeHandleStep::Fault) {
            break;
        }
    }
    close_presentation_registry(&mut registry, &pool);
    assert!(registry.terminal_is_empty());
    drop(registry);

    let operation = semio_framework_job::OperationId(8_201);
    let mut registry = PresentationEnvelopeMaterializeRegistry::new();
    registry.try_submit(operation, generation, presentation_envelope_test_pages(&hex)).unwrap_or_else(|_| panic!("second sealed fixed-page caller"));
    for _ in 0..10_000 {
        if registry.maintenance_step(operation, generation, generation, &pool).expect("exact live caller") == PresentationEnvelopeMaterializeHandleStep::Ready {
            break;
        }
    }
    close_presentation_registry(&mut registry, &pool);
    assert!(registry.terminal_is_empty());
    drop(registry);
}

#[semio_framework_async_macros::async_test]
async fn presentation_deck_materializes() {
    let mut store = PresentationStore::new(create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, "animate-presentation", empty_presentation_snapshot(), None)).await.expect("valid artifact store fixture");
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![PresentationMutation::CreateTile(create_tile::CreateTile { index: 0, tile: crate::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
            description: None,
        })
        .await
        .expect("apply");
    assert_eq!(crate::presentation_working_scene(&store.snapshot().expect("projection")).1.len(), 1);
}

//#region 🔖️DocumentTextTests
#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_with_operation_applied() {
    let mut store = PresentationStore::new(create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, "animate-presentation", crate::default_presentation_snapshot(), None)).await.expect("valid artifact store fixture");
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![PresentationMutation::CreateTile(create_tile::CreateTile { index: 0, tile: crate::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
            description: None,
        })
        .await
        .expect("apply");
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
}
//#endregion 🔖️DocumentTextTests
