//! 🧪️ Verifies one authenticated original edit through durable submit, replay, catch-up and restart.

use db_artifact_contract::{CommandBatch, SubmitOptions};
use db_artifact_engine::{ArtifactEngine, ArtifactEngineConfig, ArtifactLedgerPoint, artifact_ledger_tail};
use db_durability::DurabilityClass;
use protocol::atomic_hub::{AtomicSocketEdit, client_atomic_part};
use std::sync::{Arc, atomic::AtomicBool};

fn fixture() -> protocol::VerifiedAtomicEdit {
    let parts: &[&[u8]] = &[
        include_bytes!("🗑️generated/raw-hub-atomic/0"), include_bytes!("🗑️generated/raw-hub-atomic/1"),
        include_bytes!("🗑️generated/raw-hub-atomic/2"), include_bytes!("🗑️generated/raw-hub-atomic/3"),
        include_bytes!("🗑️generated/raw-hub-atomic/4"), include_bytes!("🗑️generated/raw-hub-atomic/5"),
        include_bytes!("🗑️generated/raw-hub-atomic/6"), include_bytes!("🗑️generated/raw-hub-atomic/7"),
    ];
    let mut owner = AtomicSocketEdit::default();
    for input in parts {
        let (id, part) = client_atomic_part(input).unwrap().unwrap();
        if let Some(edit) = owner.push(id, part, "fixture-document", "fixture-actor").unwrap() { return edit; }
    }
    panic!("fixture never sealed")
}

#[semio_framework_async_macros::async_test]
async fn original_group_replays_from_one_durable_boundary() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let document = protocol::ArtifactId("fixture-db-document".into());
    let core_document = db_artifact_contract::to_core_document_id(&document).await;
    let options = || SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() };
    let mut engine = ArtifactEngine::create_retained(document.clone(), storage.clone(), ArtifactEngineConfig::default(), 0).await.unwrap();
    let unadmitted = fixture().into_parts().1;
    assert!(CommandBatch::new(unadmitted).await.is_err());
    let batch = || CommandBatch::from_verified_atomic(fixture(), &document).unwrap();
    let committed = engine.submit(batch(), options(), 1).await.unwrap();
    assert_eq!(committed.frontier.head_seq, 15);
    assert_eq!(committed.frontier.commit_seq, 1);
    assert_eq!(engine.submit(batch(), options(), 2).await.unwrap().frontier, committed.frontier);
    let point = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    assert_eq!(point.head_edit_id, Some(protocol::MutationId("edit-png-82".into())));
    std::future::poll_fn(|cx| engine.poll_close(cx)).await.unwrap();
    drop(engine);

    let (mut engine, _) = ArtifactEngine::open_retained(document, storage.clone(), ArtifactEngineConfig::default(), 3).await.unwrap();
    let recovered = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    assert_eq!(point, recovered);
    let wal = storage.wal().await;
    let tail = artifact_ledger_tail(&wal, &core_document, &ArtifactLedgerPoint::genesis(), &point, Arc::new(AtomicBool::new(false))).await.unwrap();
    assert_eq!(tail.len(), 1);
    assert_eq!(tail[0].original_edit.as_ref().unwrap().edit_id, "edit-png-82");
    let png = include_bytes!("🗑️generated/large-picked/large-picked.png");
    assert!(tail[0].envelopes.iter().flat_map(|row| row.diff.payload.iter()).eq(png.iter().copied()));
    let state = db_sync::replay_sync_state(&wal, core_document.clone()).await.unwrap();
    let mut interior = db_durability::Frontier::genesis(core_document);
    interior.head_seq = 9;
    let frames = db_sync::grouped_frontier_frames(&state, &interior, protocol::ActorId("fixture-hub".into())).await.unwrap();
    let [protocol::ServerFrame::Commands { envelopes, frontier, .. }] = frames.as_slice() else { panic!("expected one atomic replay frame"); };
    assert_eq!(frontier.head_edit_id, "edit-png-82");
    assert_eq!(envelopes, &tail[0].envelopes);
    drop(wal);
    std::future::poll_fn(|cx| engine.poll_close(cx)).await.unwrap();
}
