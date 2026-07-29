//! ⏱️ Db crate family benchmarks — command-submit, recovery/snapshot-restore, and preview-vs-commit
//! hot paths, over a real `db_document::DocumentEngine` backed by `db_storage::MemoryStorage`.
//! Three groups, per this wave's instructions: `command_submit` (small commands must not scale
//! with existing document size), `recovery` (open cost vs. WAL depth, and snapshot ⊕ suffix vs.
//! full-from-genesis replay), and `preview_commit` (publishing previews must never delay a real
//! commit — `db_preview`'s "never durable" law, structurally exercised by
//! `db_testkit::assert_preview_never_durable`, has a latency-shaped counterpart here). Run via
//! `nx run @db/testkit:bench` (`bun ./script.ts bench`).

use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use db_testkit::WorkloadGen;
use std::sync::Arc;

//#region 🔖Fixtures
fn single_envelope(document: &protocol::DocumentId, index: usize) -> protocol::OperationEnvelope {
    let mut payload = serde_json::Map::with_capacity(1);
    payload.insert(format!("path-{index}"), serde_json::json!(index));
    protocol::OperationEnvelope {
        operation_id: protocol::OperationId(format!("bench-op-{index}")),
        document_id: document.clone(),
        actor: protocol::ActorId("bench-actor".to_string()),
        dependencies: Vec::new(),
        diff: protocol::DocumentDiff {
            schema: protocol::SchemaId(db_document::DB_PATHMAP_SCHEMA.to_string()),
            payload: serde_json::to_vec(&serde_json::Value::Object(payload)).unwrap_or_default(),
        },
        inverse: protocol::InverseOperation {
            schema: protocol::SchemaId(db_document::DB_PATHMAP_SCHEMA.to_string()),
            payload: serde_json::to_vec(&serde_json::Value::Object(serde_json::Map::new())).unwrap_or_default(),
        },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

fn populated_engine(document: protocol::DocumentId, existing_paths: usize) -> (protocol::DocumentId, Arc<dyn db_storage::DbStorage>, db_document::DocumentEngine) {
    let storage: Arc<dyn db_storage::DbStorage> = Arc::new(db_storage::MemoryStorage::new());
    let mut engine = db_document::DocumentEngine::create(document.clone(), storage.clone(), db_document::DocumentEngineConfig::default(), 0).expect("create");
    for (i, envelope) in WorkloadGen::new(1).disjoint_batch(&document, existing_paths).into_iter().enumerate() {
        let batch = db_document::CommandBatch::new(vec![envelope]).expect("batch");
        engine.submit(batch, db_document::SubmitOptions { durability: db_core::DurabilityClass::Fsync }, i as u64).expect("seed submit");
    }
    (document, storage, engine)
}

fn committed_wal(document: protocol::DocumentId, op_count: usize, snapshot_after: Option<usize>) -> (protocol::DocumentId, Arc<dyn db_storage::DbStorage>) {
    let storage: Arc<dyn db_storage::DbStorage> = Arc::new(db_storage::MemoryStorage::new());
    {
        let mut engine = db_document::DocumentEngine::create(document.clone(), storage.clone(), db_document::DocumentEngineConfig::default(), 0).expect("create");
        for (i, envelope) in WorkloadGen::new(2).disjoint_batch(&document, op_count).into_iter().enumerate() {
            let batch = db_document::CommandBatch::new(vec![envelope]).expect("batch");
            engine.submit(batch, db_document::SubmitOptions { durability: db_core::DurabilityClass::Fsync }, i as u64).expect("submit");
            if Some(i + 1) == snapshot_after {
                engine.snapshot_now((i + 1) as u64).expect("snapshot_now");
            }
        }
    }
    (document, storage)
}

/// @emoji 🌫️ A document with a small seeded base plus `preview_count` published preview overlays —
/// the fixture `bench_preview_commit` times a real commit against, to reveal whether preview volume
/// leaks into commit latency.
fn engine_with_previews(document: protocol::DocumentId, preview_count: usize) -> (protocol::DocumentId, Arc<dyn db_storage::DbStorage>, db_document::DocumentEngine) {
    let storage: Arc<dyn db_storage::DbStorage> = Arc::new(db_storage::MemoryStorage::new());
    let mut engine = db_document::DocumentEngine::create(document.clone(), storage.clone(), db_document::DocumentEngineConfig::default(), 0).expect("create");
    for (i, envelope) in WorkloadGen::new(4).disjoint_batch(&document, 5).into_iter().enumerate() {
        let batch = db_document::CommandBatch::new(vec![envelope]).expect("batch");
        engine.submit(batch, db_document::SubmitOptions { durability: db_core::DurabilityClass::Fsync }, i as u64).expect("seed submit");
    }
    for i in 0..preview_count {
        engine.publish_preview(&[(format!("preview-path-{i}"), Some(serde_json::json!(i)))], (100 + i) as u64).expect("publish_preview");
    }
    (document, storage, engine)
}
//#endregion 🔖Fixtures

//#region 🔖CommandSubmit
/// @emoji 🎯 Target invariant: a single small command's submit cost must not scale with how many
/// paths the document already has — the group's per-`existing_paths` timings should stay flat.
fn bench_command_submit(c: &mut Criterion) {
    let mut group = c.benchmark_group("command_submit");
    for existing in [10usize, 100, 1_000] {
        group.bench_with_input(BenchmarkId::from_parameter(existing), &existing, |b, &existing| {
            b.iter_batched(
                || populated_engine(protocol::DocumentId(format!("bench-submit-{existing}")), existing),
                |(document, _storage, mut engine)| {
                    let envelope = single_envelope(&document, existing);
                    let batch = db_document::CommandBatch::new(vec![envelope]).expect("batch");
                    black_box(engine.submit(batch, db_document::SubmitOptions { durability: db_core::DurabilityClass::Fsync }, existing as u64).expect("submit"));
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}
//#endregion 🔖CommandSubmit

//#region 🔖Recovery
/// @emoji 🎯 Target invariants: `open` cost tracks catalog + snapshot root + WAL tail (not the
/// whole history once a snapshot exists), so `full_replay`'s per-`op_count` timings should grow
/// with WAL depth while `snapshot_plus_suffix` stays close to the (fixed-size) suffix alone
/// regardless of how much history precedes the snapshot.
fn bench_recovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("recovery");
    for op_count in [10usize, 100, 500] {
        group.bench_with_input(BenchmarkId::new("full_replay", op_count), &op_count, |b, &op_count| {
            b.iter_batched(
                || committed_wal(protocol::DocumentId(format!("bench-recover-{op_count}")), op_count, None),
                |(document, storage)| {
                    black_box(db_document::DocumentEngine::open(document, &storage, db_document::DocumentEngineConfig::default(), 0).expect("open"));
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.bench_function("snapshot_plus_suffix_of_1000_with_10_tail", |b| {
        b.iter_batched(
            || committed_wal(protocol::DocumentId("bench-recover-snap".to_string()), 1_000, Some(990)),
            |(document, storage)| {
                black_box(db_document::DocumentEngine::open(document, &storage, db_document::DocumentEngineConfig::default(), 0).expect("open"));
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}
//#endregion 🔖Recovery

//#region 🔖PreviewCommit
/// @emoji 🎯 Target invariant: publishing previews must never delay a real commit — the group's
/// per-`preview_count` timings for committing ONE additional durable command should stay flat
/// regardless of how many preview overlays already sit on the document, since the preview overlay
/// lives entirely outside the WAL-append/commit path.
fn bench_preview_commit(c: &mut Criterion) {
    let mut group = c.benchmark_group("preview_commit");
    for preview_count in [0usize, 50, 500] {
        group.bench_with_input(BenchmarkId::from_parameter(preview_count), &preview_count, |b, &preview_count| {
            b.iter_batched(
                || engine_with_previews(protocol::DocumentId(format!("bench-preview-commit-{preview_count}")), preview_count),
                |(document, _storage, mut engine)| {
                    let envelope = single_envelope(&document, 10_000 + preview_count);
                    let batch = db_document::CommandBatch::new(vec![envelope]).expect("batch");
                    black_box(engine.submit(batch, db_document::SubmitOptions { durability: db_core::DurabilityClass::Fsync }, 999).expect("commit submit"));
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}
//#endregion 🔖PreviewCommit

criterion_group!(benches, bench_command_submit, bench_recovery, bench_preview_commit);
criterion_main!(benches);
