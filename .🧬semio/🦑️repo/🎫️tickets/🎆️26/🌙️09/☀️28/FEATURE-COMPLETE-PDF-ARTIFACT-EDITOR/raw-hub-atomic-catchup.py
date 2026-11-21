"""📚 Stages transaction-aware Hub hello replay without exposing constituent source rows."""

import hashlib
from pathlib import Path

HELLO = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🎞️hello-replay-tail-and-execution/🦀️.rs")
REPLAY = HELLO.parent.parent / "🪞️replica-sync-protocol/🦀️.rs"
SYNC = HELLO.parent.parent / "🦀️.rs"
DB = SYNC.parent.parent / "🦀️.rs"


def apply(root, pre, post):
    def edit(path, anchor, next_value, expected=1):
        body = (root / path).read_bytes()
        pre[str(path)] = hashlib.sha256(body).hexdigest()
        value = post.get(str(path), body.decode("utf8"))
        assert value.count(anchor) == expected, (path, anchor, value.count(anchor))
        post[str(path)] = value.replace(anchor, next_value, 1 if expected != 1 else -1)

    edit(REPLAY, "    pub group: protocol::atomic_hub::AtomicWalEdit,",
         "    pub group: protocol::atomic_hub::AtomicWalEdit,\n    pub frontier: Frontier,\n    pub prior_frontier: Frontier,")
    edit(REPLAY, "            let start = commands.len();\n            let mut group = None;",
         """            let start = commands.len();
            let prior_frontier = Frontier {
                document: document.clone(), head_seq: start as u64, commit_seq,
                chain_hash: if start == 0 { [0; 32] } else { *chain.clone().finalize().as_bytes() },
                epoch: 0,
            };
            let mut group = None;""")
    edit(REPLAY, "                atomic_edits.push(AtomicReplaySpan { range: start..commands.len(), group });",
         """                let frontier = Frontier {
                    document: document.clone(), head_seq: commands.len() as u64,
                    commit_seq: commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("sync replay commit sequence"))?,
                    chain_hash: *chain.clone().finalize().as_bytes(),
                    epoch: 0,
                };
                atomic_edits.push(AtomicReplaySpan { range: start..commands.len(), group, frontier, prior_frontier });""")
    edit(REPLAY, "    let head_edit_id = state.commands.last().map(|envelope| envelope.mutation_id.0.clone()).unwrap_or_default();",
         """    let head_edit_id = state.atomic_edits.last().filter(|span| span.range.end == state.commands.len()).map(|span| span.group.edit_id.clone())
        .or_else(|| state.commands.last().map(|envelope| envelope.mutation_id.0.clone())).unwrap_or_default();""")
    edit(REPLAY, "/// 📨️ Wraps " + chr(96) + "envelopes" + chr(96),
         """/// 🧱️ Splits the replay cut only on a stored grouped edit boundary.
pub async fn grouped_frontier_frames(
    state: &ArtifactSyncState, replica: &Frontier, origin: protocol::ActorId,
) -> Result<Vec<protocol::ServerFrame>, DbError> {
    if replica.document != state.frontier.document || replica.head_seq > state.frontier.head_seq {
        return Err(DbError::InvalidArgument("invalid grouped frontier".into()));
    }
    if replica.head_seq < state.floor_head_seq {
        return Err(DbError::Unavailable("grouped frontier is behind the retained WAL floor".into()));
    }
    let mut position = replica.head_seq as usize;
    let mut frames = Vec::new();
    for span in &state.atomic_edits {
        if span.range.end <= position { continue; }
        if position < span.range.start {
            let last = state.commands[span.range.start - 1].mutation_id.0.clone();
            let frontier = to_frontier_summary(&span.prior_frontier, last).await;
            let envelopes = state.commands[position..span.range.start].to_vec();
            frames.push(protocol::ServerFrame::Commands { envelopes, origin: origin.clone(), frontier });
        }
        let frontier = if span.range.end == state.commands.len() { &state.frontier } else { &span.frontier };
        let frontier = to_frontier_summary(frontier, span.group.edit_id.clone()).await;
        let envelopes = state.commands[span.range.clone()].to_vec();
        frames.push(protocol::ServerFrame::Commands { envelopes, origin: origin.clone(), frontier });
        position = span.range.end;
    }
    if position < state.commands.len() {
        frames.push(commands_server_frame(state, state.commands[position..].to_vec(), origin).await);
    }
    Ok(frames)
}

/// 📨️ Wraps """ + chr(96) + "envelopes" + chr(96))
    edit(SYNC, "    replay_sync_state, state_frontier_summary,",
         "    grouped_frontier_frames, replay_sync_state, state_frontier_summary,")
    edit(DB, "    pub use db_sync::handle_frontier_advertise;",
         "    pub use db_sync::{from_frontier_summary, grouped_frontier_frames, handle_frontier_advertise, replay_sync_state};")
    anchor = "                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::SnapshotPub { frontier, .. })) => floor_head_seq = frontier.head_seq,"
    edit(HELLO, anchor, """                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Outbox(bytes))) => {
                        if let Some(group) = super::replica_sync_protocol::decode_atomic_wal_descriptor(bytes)? {
                            head_edit_id = group.edit_id;
                        }
                    }
""" + anchor)
    old = """            loop {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
                decode_control.renew_step()?;
                database_sync_hello_control(&cancelled, &expired)?;
                match transaction.next_record_step() {
                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Command(bytes))) => {
                        head_seq = head_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello tail head sequence"))?;
                        chain.update(&bytes.hash());
                        if head_seq > replica_head {
                            let envelope = decode_wal_command(bytes, &mut decode_control)?;
                            head_edit_id.clone_from(&envelope.mutation_id.0);
                            envelopes.push(envelope);
                        }
                    }
                    Ok(db_wal_log::WalCommittedRecordStep::Record(_)) => {}"""
    edit(HELLO, old, """            let transaction_start = head_seq;
            let prior_hash = *chain.clone().finalize().as_bytes();
            let might_reach_replica = transaction_start.saturating_add(protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES as u64) > replica_head;
            let mut transaction_rows = Vec::new();
            let mut atomic_edit = None;
            loop {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
                decode_control.renew_step()?;
                database_sync_hello_control(&cancelled, &expired)?;
                match transaction.next_record_step() {
                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Command(bytes))) => {
                        head_seq = head_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello tail head sequence"))?;
                        chain.update(&bytes.hash());
                        if might_reach_replica || head_seq > replica_head {
                            transaction_rows.push((head_seq, decode_wal_command(bytes, &mut decode_control)?));
                        }
                    }
                    Ok(db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Outbox(bytes))) => {
                        if let Some(group) = super::replica_sync_protocol::decode_atomic_wal_descriptor(bytes)? {
                            if atomic_edit.replace(group).is_some() {
                                return Err(DbError::Corrupt("hello tail repeats an atomic WAL descriptor".into()));
                            }
                        }
                    }
                    Ok(db_wal_log::WalCommittedRecordStep::Record(_)) => {}""")
    anchor = """            if envelopes.len() >= DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES || (head_seq >= end_head_seq && !envelopes.is_empty()) {
                let frame_commit_seq = if head_seq >= end_head_seq { end_commit_seq } else { commit_seq };
                let frontier = (head_seq, std::mem::take(&mut head_edit_id), frame_commit_seq, *chain.finalize().as_bytes());
                DatabaseSyncHelloTailSlotFree(&handoff).await;
                database_sync_hello_control(&cancelled, &expired)?;
                handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page = Some((std::mem::take(&mut envelopes), frontier));
            }"""
    edit(HELLO, anchor, """            if let Some(group) = atomic_edit {
                if head_seq <= replica_head { continue; }
                if !envelopes.is_empty() {
                    let prior = (transaction_start, std::mem::take(&mut head_edit_id), commit_seq - 1, prior_hash);
                    DatabaseSyncHelloTailSlotFree(&handoff).await;
                    database_sync_hello_control(&cancelled, &expired)?;
                    handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page = Some((std::mem::take(&mut envelopes), prior));
                }
                let rows: Vec<_> = transaction_rows.into_iter().map(|(_, row)| row).collect();
                if !group.admits(&rows) {
                    return Err(DbError::Corrupt("hello tail cannot recover the whole authored edit".into()));
                }
                let frame_commit_seq = if head_seq >= end_head_seq { end_commit_seq } else { commit_seq };
                let frontier = (head_seq, group.edit_id, frame_commit_seq, *chain.finalize().as_bytes());
                DatabaseSyncHelloTailSlotFree(&handoff).await;
                database_sync_hello_control(&cancelled, &expired)?;
                handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page = Some((rows, frontier));
            } else {
                for (ordinal, row) in transaction_rows {
                    if ordinal <= replica_head { continue; }
                    if row.transaction.as_ref().is_some_and(|tx| protocol::atomic_group::is_atomic_backbone_tool(&tx.tool)) {
                        return Err(DbError::Corrupt("hello tail lost its atomic WAL descriptor".into()));
                    }
                    head_edit_id.clone_from(&row.mutation_id.0);
                    envelopes.push(row);
                }
                if envelopes.len() >= DATABASE_SYNC_HELLO_TAIL_FRAME_ENVELOPES || (head_seq >= end_head_seq && !envelopes.is_empty()) {
                    let frame_commit_seq = if head_seq >= end_head_seq { end_commit_seq } else { commit_seq };
                    let frontier = (head_seq, std::mem::take(&mut head_edit_id), frame_commit_seq, *chain.finalize().as_bytes());
                    DatabaseSyncHelloTailSlotFree(&handoff).await;
                    database_sync_hello_control(&cancelled, &expired)?;
                    handoff.lock().unwrap_or_else(std::sync::PoisonError::into_inner).page = Some((std::mem::take(&mut envelopes), frontier));
                }
            }""")
