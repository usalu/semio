/// 🪶️ Verifies the authenticated original-media group after reopening the native SQLite database.
#[semio_framework_async_macros::async_test]
async fn original_media_group_survives_sqlite_restart() {
    use protocol::atomic_hub::{AtomicSocketEdit, AtomicWalEdit};
    use std::time::Duration;
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../🗿️artifact/⚙️engine/🧪️tests/🔬️unit/📬️atomic-media-replay/🔣️original-group.json"
    )).unwrap();
    let root = tempdir("original-media-sqlite").await;
    let path = root.join("db.sqlite3");
    let document = protocol::ArtifactId("sqlite-original-media-document".into());
    let author = fixture["actor"].as_str().unwrap();
    let edit_id = fixture["edit"].as_str().unwrap();
    let rows = fixture["rows"].as_u64().unwrap() as usize;
    let bytes_per_row = fixture["bytesPerRow"].as_u64().unwrap() as usize;
    let ids: Vec<_> = (0..rows).map(|index| format!("{edit_id}-{index}")).collect();
    let mut events = Vec::with_capacity(rows);
    for (index, id) in ids.iter().enumerate() {
        let dependencies: Vec<_> = index.checked_sub(1).into_iter().map(|prior| ids[prior].as_str()).collect();
        let mut row = envelope(id, &dependencies, author, &document, &[]).await;
        let schema = protocol::SchemaId("fixture.original-media.v1".into());
        let payload = protocol::CausalPayload::try_from_slice(&vec![b'a' + index as u8; bytes_per_row]).unwrap();
        row.diff = protocol::ArtifactDiff { schema: schema.clone(), payload };
        row.inverse = protocol::InverseMutation { schema, payload: protocol::CausalPayload::try_from_slice(&[]).unwrap() };
        row.transaction = Some(protocol::TransactionRef { id: edit_id.to_owned(), tool: fixture["tool"].as_str().unwrap().to_owned() });
        events.push(row);
    }
    let group = AtomicWalEdit { edit_id: edit_id.to_owned(), rows };
    let frames = group.relay(&events, &[]).unwrap();
    assert!(frames.len() > 1);
    let verify = |actor: &str| {
        let mut socket = AtomicSocketEdit::default();
        let mut result = None;
        for frame in &frames {
            result = socket.push(1, &frame[4..], &document.0, actor)?;
        }
        result.ok_or("original SQLite group did not complete")
    };
    assert!(verify(fixture["otherActor"].as_str().unwrap()).is_err());
    let batch = || db_artifact_contract::CommandBatch::from_verified_atomic(verify(author).unwrap(), &document).unwrap();
    let options = || db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() };

    let frontier = {
        let pool = test_worker_pool();
        let storage = db_storage_sqlite::SqliteStorage::open(pool.clone(), &path).await.unwrap();
        let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Prod), Arc::new(db_storage::DbBackend::Sqlite(storage))).await.unwrap();
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let receipt = handle.submit(batch(), options()).await.unwrap().unwrap();
        assert_eq!(receipt.frontier.head_seq, rows as u64);
        assert_eq!(receipt.frontier.commit_seq, fixture["commits"].as_u64().unwrap());
        assert_eq!(handle.submit(batch(), options()).await.unwrap().unwrap().frontier, receipt.frontier);
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(Duration::from_secs(30))).await.unwrap();
        drop(database);
        pool.shutdown().unwrap();
        receipt.frontier
    };

    let pool = test_worker_pool();
    let storage = db_storage_sqlite::SqliteStorage::open(pool.clone(), &path).await.unwrap();
    let mut reopened = Database::open(pool.clone(), DbConfig::for_profile(Profile::Prod), Arc::new(db_storage::DbBackend::Sqlite(storage))).await.unwrap();
    let handle = reopened.ensure_document(&document).await.unwrap();
    assert_eq!(handle.frontier().await.unwrap(), frontier);
    let core = to_core_document_id(&document).await;
    let storage = reopened.storage().await;
    let wal = storage.wal().await;
    let state = db_sync::replay_sync_state(&wal, core.clone()).await.unwrap();
    let mut interior = db_durability::Frontier::genesis(core);
    interior.head_seq = fixture["catchupFrom"].as_u64().unwrap();
    let replay = db_sync::grouped_frontier_frames(&state, &interior, protocol::ActorId("fixture-hub".into())).await.unwrap();
    let [protocol::ServerFrame::Commands { envelopes, frontier, .. }] = replay.as_slice() else { panic!("expected one grouped SQLite replay"); };
    assert_eq!(frontier.head_edit_id, edit_id);
    assert_eq!(group.relay(envelopes, &[]).unwrap(), frames);
    drop(wal);
    drop(storage);
    drop(handle);
    reopened.shutdown(&DatabaseShutdownControl::for_timeout(Duration::from_secs(30))).await.unwrap();
    drop(reopened);
    pool.shutdown().unwrap();
    std::fs::remove_dir_all(root).unwrap();
}
