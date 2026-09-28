#!/usr/bin/env python3
"""⚖ H14 14c (c)(d)(e) laws: db_index run-id layout with levels + owned appends fold into levels + a fold beyond one read
credit stops; db_artifact index written after the receipt and refilled from the WAL on reopen, bounded receipts,
> 70 k single-envelope commits stay writable with bounded runs (fs). Idempotent; `--dry-run` reports only."""
import sys

INDEX_TESTS = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🧪️tests/🔬️unit/🦀️.rs"
ARTIFACT_TESTS = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {INDEX_TESTS: open(INDEX_TESTS, encoding="utf-8").read(), ARTIFACT_TESTS: open(ARTIFACT_TESTS, encoding="utf-8").read()}
problems, states = [], []


def edit(path, old, new, label):
    text = files[path]
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


edit(INDEX_TESTS, """#[semio_framework_async_macros::async_test]
async fn run_id_round_trips_kind_sequence_and_entry_count_for_every_kind() {
    for kind in IndexKind::ALL {
        for sequence in [0u64, 1, SEQUENCE_MASK] {
            for entries in [1usize, 2, MAX_RUN_ENTRIES as usize] {
                let run_id = make_run_id(kind, sequence, entries).expect("make_run_id");
                assert_eq!(namespace_of_run_id(run_id), run_namespace(kind));
                assert_eq!(sequence_of_run_id(run_id), sequence);
                assert_eq!(entries_of_run_id(run_id), entries as u64);
                assert!(run_id < 1 << 63, "a run id stays a positive SQL integer");
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn run_ids_order_by_kind_then_sequence() {
    let older = make_run_id(IndexKind::Command, 7, MAX_RUN_ENTRIES as usize).unwrap();
    let newer = make_run_id(IndexKind::Command, 8, 1).unwrap();
    let other_kind = make_run_id(IndexKind::ActorSeq, 0, 1).unwrap();
    assert!(older < newer && newer < other_kind);
}

#[semio_framework_async_macros::async_test]
async fn run_id_rejects_sequence_overflow_and_entry_counts_outside_one_run() {
    assert!(matches!(make_run_id(IndexKind::Command, SEQUENCE_MASK + 1, 1), Err(DbError::LimitExceeded(_))));
    assert!(matches!(make_run_id(IndexKind::Command, 0, 0), Err(DbError::LimitExceeded(_))));
    assert!(matches!(make_run_id(IndexKind::Command, 0, MAX_RUN_ENTRIES as usize + 1), Err(DbError::LimitExceeded(_))));
}
//#endregion 🔖️IndexKind
""", """#[semio_framework_async_macros::async_test]
async fn run_id_round_trips_kind_sequence_level_and_entry_count_for_every_kind() {
    for kind in IndexKind::ALL {
        for sequence in [0u64, 1, SEQUENCE_MASK] {
            for entries in [1usize, 2, MAX_RUN_ENTRIES as usize] {
                let run_id = make_run_id(kind, sequence, 0, entries).expect("make_run_id");
                assert_eq!(namespace_of_run_id(run_id), run_namespace(kind));
                assert_eq!(sequence_of_run_id(run_id), sequence);
                assert_eq!(level_of_run_id(run_id), 0);
                assert_eq!(entries_of_run_id(run_id), entries as u64);
                assert!(run_id < 1 << 63, "a run id stays a positive SQL integer");
            }
            for level in 1..=RUN_LEVEL_MAX {
                let run_id = make_run_id(kind, sequence, level, level_capacity(level) as usize).expect("make_run_id");
                assert_eq!(sequence_of_run_id(run_id), sequence);
                assert_eq!(level_of_run_id(run_id), level);
                assert_eq!(entries_of_run_id(run_id), MAX_RUN_ENTRIES << (2 * level), "a folded run declares its level's capacity");
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn run_ids_order_by_kind_then_sequence_then_level() {
    let older = make_run_id(IndexKind::Command, 7, 0, MAX_RUN_ENTRIES as usize).unwrap();
    let folded = make_run_id(IndexKind::Command, 7, 1, 4 * MAX_RUN_ENTRIES as usize).unwrap();
    let newer = make_run_id(IndexKind::Command, 8, 0, 1).unwrap();
    let other_kind = make_run_id(IndexKind::ActorSeq, 0, 0, 1).unwrap();
    assert!(older < folded && folded < newer && newer < other_kind, "a folded run sorts right after its oldest input and before every newer run");
}

#[semio_framework_async_macros::async_test]
async fn run_id_rejects_sequence_overflow_levels_and_entry_counts_outside_a_run() {
    assert!(matches!(make_run_id(IndexKind::Command, SEQUENCE_MASK + 1, 0, 1), Err(DbError::LimitExceeded(_))));
    assert!(matches!(make_run_id(IndexKind::Command, 0, 0, 0), Err(DbError::LimitExceeded(_))));
    assert!(matches!(make_run_id(IndexKind::Command, 0, 0, MAX_RUN_ENTRIES as usize + 1), Err(DbError::LimitExceeded(_))));
    assert!(matches!(make_run_id(IndexKind::Command, 0, RUN_LEVEL_MAX + 1, 1), Err(DbError::LimitExceeded(_))));
    assert!(matches!(make_run_id(IndexKind::Command, 0, RUN_LEVEL_MAX, level_capacity(RUN_LEVEL_MAX) as usize + 1), Err(DbError::LimitExceeded(_))));
}
//#endregion 🔖️IndexKind

//#region 🔖️OwnedRuns
async fn append_seq_runs(index: &CommandIndex<'_, MemoryStorage>, runs: &mut OwnedRuns, from: u64, full_runs: u64) {
    for run in 0..full_runs {
        let first = from + run * MAX_RUN_ENTRIES;
        let entries: Vec<(u64, RecordLocation)> = (first..first + MAX_RUN_ENTRIES).map(|seq| (seq, RecordLocation { segment: seq / 1_000, offset: seq, len: 1 })).collect();
        index.record_owned_run(runs, &entries).await.expect("owned append");
    }
}

/// ⚖️ An append-only owner's appends never list and fold every four full runs of one level into one run of the next, so
/// 147 full runs (9 408 entries) end as exactly the base-4 digits of 147 — two level-3, one level-2 and three level-0
/// runs — whose ids storage lists exactly as the owner knows them, oldest first, and every entry still resolves.
#[semio_framework_async_macros::async_test]
async fn owned_appends_fold_full_runs_into_levels_and_every_entry_still_resolves() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId::from("doc-levels");
    let index = CommandIndex::new(&storage, document.clone()).await;
    let mut runs = index.owned_runs().await.expect("owned runs");
    assert!(runs.is_empty());
    append_seq_runs(&index, &mut runs, 1, 147).await;
    let handle = IndexHandle::new(&storage, document.clone(), IndexKind::Command).await;
    let mut control = control();
    let listed = handle.kind_run_ids(&mut control).await.expect("listing");
    assert_eq!(listed, runs.ids, "storage holds exactly the runs the owner knows");
    let shape: Vec<(u64, u32)> = listed.iter().map(|id| (sequence_of_run_id(*id), level_of_run_id(*id))).collect();
    assert_eq!(shape, vec![(0, 3), (64, 3), (128, 2), (144, 0), (145, 0), (146, 0)]);
    handle.verify(&mut control).await.expect("every folded run verifies");
    assert_eq!(index.indexed_through().await.expect("indexed through"), 147 * 64);
    for seq in (1..=147 * 64).step_by(97).chain([1, 4_096, 4_097, 9_216, 9_408]) {
        assert_eq!(index.lookup(seq).await.expect("lookup"), Some(RecordLocation { segment: seq / 1_000, offset: seq, len: 1 }), "seq {seq}");
    }
    assert_eq!(index.lookup(9_409).await.expect("lookup"), None);
    let mut reopened = index.owned_runs().await.expect("owned runs again");
    append_seq_runs(&index, &mut reopened, 147 * 64 + 1, 1).await;
    let refolded: Vec<(u64, u32)> = handle.kind_run_ids(&mut control).await.expect("listing").iter().map(|id| (sequence_of_run_id(*id), level_of_run_id(*id))).collect();
    assert_eq!(refolded, vec![(0, 3), (64, 3), (128, 2), (144, 1)], "a re-listed owner folds its newest four level-0 runs on the next append");
}

/// ⚖️ A fold whose inputs exceed one read operation's credit writes nothing: the runs stay as they were, readable, and the
/// owner stops folding at that level instead of re-reading them on every later append.
#[semio_framework_async_macros::async_test]
async fn a_fold_beyond_one_read_credit_is_skipped_and_its_level_becomes_the_ceiling() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId::from("doc-wide");
    let handle = IndexHandle::new(&storage, document.clone(), IndexKind::Projection).await;
    let mut control = control();
    let mut runs = handle.owned_runs(&mut control).await.expect("owned runs");
    let value = vec![7u8; 2_048];
    for run in 0..8u64 {
        let keys: Vec<[u8; 8]> = (0..MAX_RUN_ENTRIES).map(|entry| (run * MAX_RUN_ENTRIES + entry).to_be_bytes()).collect();
        let entries: Vec<(&[u8], &[u8])> = keys.iter().map(|key| (&key[..], &value[..])).collect();
        handle.append_owned_sorted_run(&mut runs, &entries, &mut control).await.expect("wide append");
    }
    let listed = handle.kind_run_ids(&mut control).await.expect("listing");
    assert_eq!(listed.len(), 8, "four 128 KiB runs exceed one read credit: nothing folds");
    assert!(listed.iter().all(|id| level_of_run_id(*id) == 0));
    assert_eq!(runs.fold_ceiling, 0);
    let key = (5 * MAX_RUN_ENTRIES + 3).to_be_bytes();
    let found = handle.get(&retained(&key).await, &mut control).await.expect("get").expect("present");
    assert_eq!(read_retained(&found).await, value);
}
//#endregion 🔖️OwnedRuns
""", "index run-id + owned laws")

edit(ARTIFACT_TESTS, """//#region 🔖️Outbox + CommitLog
#[semio_framework_async_macros::async_test]
async fn outbox_and_commit_log_accumulate_and_outbox_drains() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    assert_eq!(engine.commit_log().await.len(), 1);
    assert_eq!(engine.commit_log().await[0].operation_ids, vec![protocol::MutationId("op-1".to_string())]);

    let drained = engine.drain_outbox().await;
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].mutation_id, protocol::MutationId("op-1".to_string()));
    assert!(engine.drain_outbox().await.is_empty(), "drain must clear the outbox");
}
//#endregion 🔖️Outbox + CommitLog
""", """//#region 🔖️IndexBacklog + Receipts
async fn single_envelope_commit(engine: &mut ArtifactEngine, index: u64, durability: DurabilityClass) -> CommandReceipt {
    let id = format!("op-{index}");
    engine.submit(CommandBatch::new(vec![envelope(&id, &[], "alice", &[]).await]).await.unwrap(), SubmitOptions { durability, ..Default::default() }, index).await.unwrap()
}

async fn close_engine(engine: &mut ArtifactEngine) {
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

/// ⚖️ A commit's receipt never waits for index I/O: 300 acknowledged single-envelope commits leave no index run behind
/// (the runner writes them after replying) — the state a crash after the Ack and before the index write leaves. The
/// reopened document refills its backlog from the WAL, its maintenance writes every full run, and later commits continue
/// the same runs: every indexed seq resolves to its WAL position, nothing twice, nothing missing.
#[semio_framework_async_macros::async_test]
async fn index_runs_follow_the_receipt_and_a_reopen_refills_them_from_the_wal() {
    let storage = storage().await;
    let before = {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        for index in 1..=300 {
            single_envelope_commit(&mut engine, index, DurabilityClass::Fsync).await;
        }
        let frontier = engine.frontier().await;
        close_engine(&mut engine).await;
        frontier
    };
    let core = to_core_document_id(&document_id().await).await;
    let index_facet = storage.index().await;
    assert_eq!(db_index::CommandIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 0, "no receipt waited for an index run");
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 0);

    let (mut reopened, _) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 301).unwrap();
    assert_eq!(reopened.frontier().await, before);
    reopened.maintain_index().await.unwrap();
    let commands = db_index::CommandIndex::new(&index_facet, core.clone()).await;
    assert_eq!(commands.indexed_through().await.unwrap(), 256, "four full runs from the WAL suffix; the partial tail waits");
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 256);
    for index in 301..=400 {
        single_envelope_commit(&mut reopened, index, DurabilityClass::Fsync).await;
        reopened.maintain_index().await.unwrap();
    }
    assert_eq!(commands.indexed_through().await.unwrap(), 384);
    for seq in [1u64, 64, 65, 256, 257, 300, 301, 384] {
        assert_eq!(commands.lookup(seq).await.unwrap().map(|location| location.offset), Some(seq), "seq {seq}");
    }
    assert_eq!(commands.lookup(385).await.unwrap(), None);
    let mut control = db_index::IndexHandle::new(&index_facet, core.clone(), db_index::IndexKind::Command).await.operation_control(65_536).unwrap();
    db_index::IndexHandle::new(&index_facet, core, db_index::IndexKind::Command).await.verify(&mut control).await.unwrap();
    close_engine(&mut reopened).await;
}

/// ⚖️ The engine keeps the receipts of its newest `APPLIED_RECEIPTS_MAX` batches only; a whole-batch resend inside the window
/// answers its original receipt, an older one is answered by the per-envelope dedupe without a new commit.
#[semio_framework_async_macros::async_test]
async fn applied_receipts_keep_a_bounded_window_and_older_resends_stay_idempotent() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let mut last = None;
    for index in 1..=(APPLIED_RECEIPTS_MAX as u64 + 10) {
        last = Some(single_envelope_commit(&mut engine, index, DurabilityClass::Memory).await);
    }
    assert_eq!(engine.applied_receipts.len(), APPLIED_RECEIPTS_MAX);
    assert_eq!(engine.applied_receipt_order.len(), APPLIED_RECEIPTS_MAX);
    let frontier = engine.frontier().await;
    let newest = single_envelope_commit(&mut engine, APPLIED_RECEIPTS_MAX as u64 + 10, DurabilityClass::Memory).await;
    assert_eq!(Some(newest), last, "a resend inside the window answers its original receipt");
    let oldest = single_envelope_commit(&mut engine, 1, DurabilityClass::Memory).await;
    assert_eq!(oldest.frontier, frontier, "a resend beyond the window commits nothing");
    assert_eq!(engine.frontier().await, frontier);
    assert_eq!(engine.applied_receipts.len(), APPLIED_RECEIPTS_MAX);
    close_engine(&mut engine).await;
}

/// ⚖️ A document stays writable far past the old 4 096-run listing ceiling (4 runs per 64 single-envelope commits: every
/// commit failed after ~65.5 k of them): 70 016 single-envelope commits on the filesystem backend, each followed by the
/// runner's index maintenance, all commit; the four kinds hold a few hundred runs, every indexed seq resolves.
#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn seventy_thousand_single_envelope_commits_stay_writable_with_bounded_index_runs() {
    const COMMITS: u64 = 70_016;
    let dir = std::env::temp_dir().join(format!("db_artifact_seventy_thousand_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let storage = StdArc::new(db_storage::DbBackend::Fs(db_storage::FsStorage::open(crate::db_storage::db_io_test_pool(), &dir).await.unwrap()));
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    for index in 1..=COMMITS {
        single_envelope_commit(&mut engine, index, DurabilityClass::Memory).await;
        engine.maintain_index().await.unwrap_or_else(|error| panic!("index maintenance after commit {index}: {error}"));
    }
    assert_eq!(engine.frontier().await.head_seq, COMMITS);
    let core = to_core_document_id(&document_id().await).await;
    let index_facet = storage.index().await;
    let mut runs = 0usize;
    for kind in [db_index::IndexKind::Command, db_index::IndexKind::Inverse, db_index::IndexKind::ActorSeq, db_index::IndexKind::Frontier] {
        let handle = db_index::IndexHandle::new(&index_facet, core.clone(), kind).await;
        let mut control = handle.operation_control(1_000_000).unwrap();
        runs += handle.stats(&mut control).await.unwrap().run_count;
    }
    assert!(runs <= 300, "{runs} runs for {COMMITS} commits");
    let commands = db_index::CommandIndex::new(&index_facet, core.clone()).await;
    assert_eq!(commands.indexed_through().await.unwrap(), COMMITS);
    for seq in [1u64, 4_096, 65_536, COMMITS] {
        assert_eq!(commands.lookup(seq).await.unwrap().map(|location| location.offset), Some(seq), "seq {seq}");
    }
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core).await.indexed_through().await.unwrap(), COMMITS);
    close_engine(&mut engine).await;
    drop(engine);
    drop(index_facet);
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}
//#endregion 🔖️IndexBacklog + Receipts
""", "artifact laws")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
