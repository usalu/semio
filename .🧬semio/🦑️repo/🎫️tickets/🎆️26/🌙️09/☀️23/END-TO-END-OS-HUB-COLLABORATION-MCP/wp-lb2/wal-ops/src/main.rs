//! 📼️ LB2 ticket-local reader: every COMMITTED command of one hub document's WAL, decoded with the db's own
//! `decode_wal_command` and classified with the replication crate's `is_history_transition`, as JSON lines —
//! the hub-authoritative census the conflict probe's "the hub accepted only the winner" criterion counts
//! (domain operations vs history transitions, per actor). Read-only: `FsStorage` over `<db-root>`, never a writer.
//! usage: lb2-wal-ops <db-root> <wal-document-id>

use db::storage::WalStorage as _;

/// 🧾️ One command inside a committed WAL transaction.
struct Committed {
    id: String,
    actor: String,
    transition: bool,
    target: Vec<String>,
}

/// 🪶️ Closes one replayed record exactly (its owner must reach terminal before drop).
fn close_record(mut record: db::wal::WalRecord) {
    while record.close_step().expect("wal record closes") {}
}

async fn census(root: &std::path::Path, document: &str) -> Result<Vec<Committed>, db::DbError> {
    let pool = std::sync::Arc::new(semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let storage = match db::storage::FsStorage::open(pool, root).await {
        Ok(storage) => storage,
        Err(mut rejected) => loop {
            match rejected.retry_close().await {
                Ok(cause) => return Err(cause),
                Err(retained) => rejected = retained,
            }
        },
    };
    let document = db::ids::ArtifactId(document.to_string());
    let mut segments = storage.list_segments(&document).await?;
    while segments.close_step() {}
    drop(segments);
    let control = db::wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(60), 10_000_000)?;
    let mut decode = db::wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(60), 10_000_000)?;
    let mut cursor = db::wal::replay_document(&storage, &document, control).await?;
    let mut open: Vec<Committed> = Vec::new();
    let mut committed = Vec::new();
    loop {
        match cursor.next_step().await? {
            db::wal::WalReplayStep::Record(record) => {
                match &record {
                    db::wal::WalRecord::TxBegin { .. } | db::wal::WalRecord::TxAbort { .. } => open.clear(),
                    db::wal::WalRecord::TxCommit { .. } => committed.append(&mut open),
                    db::wal::WalRecord::Command(bytes) => {
                        let envelope = db::sync::decode_wal_command(bytes, &mut decode)?;
                        open.push(Committed { id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.clone(), transition: protocol::is_history_transition(&envelope), target: envelope.target.clone() });
                    }
                    _ => {}
                }
                close_record(record);
            }
            db::wal::WalReplayStep::Yield => continue,
            db::wal::WalReplayStep::Done => break,
        }
    }
    while cursor.close_owner_step()? {}
    drop(cursor);
    storage.close().await?;
    Ok(committed)
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [root, document] = args.as_slice() else {
        eprintln!("usage: lb2-wal-ops <db-root> <wal-document-id>");
        std::process::exit(2);
    };
    match semio_framework_async::block_on(census(std::path::Path::new(root), document)) {
        Ok(rows) => {
            for row in rows {
                let target = row.target.iter().map(|segment| format!("{segment:?}")).collect::<Vec<_>>().join(",");
                println!("{{\"id\":{:?},\"actor\":{:?},\"transition\":{},\"target\":[{target}]}}", row.id, row.actor, row.transition);
            }
        }
        Err(error) => {
            eprintln!("lb2-wal-ops: {error}");
            std::process::exit(1);
        }
    }
}
