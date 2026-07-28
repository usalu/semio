//! 🗄️ `db_sync` — server side of `protocol_wire`: frontier exchange, missing-command transfer,
//! snapshot bootstrap, and resume tokens for a document replica ((re)connecting to the hub over
//! `protocol::{ClientFrame, ServerFrame}`). Frozen contract:
//! `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`); wire types frozen in `.repo/🎫/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/
//! contract.md` `## Amendment` §`protocol_wire`.
//!
//! 🎯 Design choice (dependency scope): per the contract's dep table this crate depends on
//! `db_core`, `db_wal`, `db_storage`, `protocol` only — no `db_document` (still a stub as of this
//! wave, and per the contract's hard dependency rule this crate never interprets operation
//! semantics anyway). Every function below therefore derives a document's sync state directly
//! from its WAL via `db_wal::replay_document` rather than consulting a live document actor — the
//! authoritative source once `db_document` lands. This crate's replay-derived `Frontier` is a
//! faithful stand-in (see `//#region 🔖ReplicaState`'s doc for exactly which fields are derived
//! vs. placeholder), not a shortcut: every WAL record this crate touches is decoded and verified
//! the same way `db_wal`'s own recovery path does.
//!
//! 🎯 Design choice (`ResumeToken` receive path): `db_core::ResumeToken` exposes `encode(&Frontier)
//! -> ResumeToken` and `ResumeToken::decode(&self) -> Frontier`, but no public constructor from an
//! arbitrary wire string (its inner field is private to `db_core`) — so this crate cannot
//! reconstruct a `ResumeToken` from `protocol::ClientFrame::Hello.resume_token: Option<String>` to
//! call its type-safe `decode`. Rather than duplicating `db_core`'s private parsing logic (a
//! frozen, un-editable crate this wave), this crate uses `Hello.frontier:
//! Option<protocol::RuntimeFrontierSummary>` — a separate, always-decodable field on the very same
//! frame — as the authoritative source of "where is the replica" on the receive path. The
//! `resume_token` this crate ISSUES (`issue_resume_token`, on the send path, `Welcome.resume_token`)
//! is fully real: `db_core::ResumeToken::encode` is public and exercised end to end.

//#region 🔖Codec
/// @emoji ✉️ This crate's own convention for `db_wal::WalRecord::Command`'s payload bytes:
/// `protocol_causal::encode_envelope`'s binary record — the same primitive codec `protocol_wire`
/// uses for `ClientFrame::Commands`/`ServerFrame::Commands`, so a WAL command's bytes are
/// byte-identical to its on-wire form (M-C's "communication AND storage both binary"). `db_wal`
/// itself never interprets these bytes (per the contract, no crate below `db_document` does);
/// this crate is the first one that needs to read a command's bytes back out semantically (to
/// relay it as a typed `protocol::OperationEnvelope` in a `ServerFrame::Commands`), so it is the
/// natural place to fix this convention. Once `db_document` lands it becomes the writer of these
/// bytes; this codec is the seam it should reuse rather than inventing a second one.
pub fn encode_command_envelope(envelope: &protocol::OperationEnvelope) -> Vec<u8> {
    let mut out = Vec::new();
    protocol::encode_envelope(envelope, &mut out);
    out
}

/// @emoji 📖 Inverse of `encode_command_envelope`. Validates the byte length against
/// `db_core::DbLimits::default().max_command_bytes` BEFORE decoding anything sized by it (mirrors
/// `pack_core`'s stated invariant), then maps a decode failure to `DbError::Corrupt` rather than
/// leaking `protocol::ProtocolError`.
pub fn decode_command_envelope(bytes: &[u8]) -> Result<protocol::OperationEnvelope, db_core::DbError> {
    db_core::check_len(bytes.len() as u64, db_core::DbLimits::default().max_command_bytes, "wal_command_envelope")?;
    let mut pos = 0usize;
    let envelope = protocol::decode_envelope(bytes, &mut pos).map_err(|error| db_core::DbError::Corrupt(format!("malformed wal command envelope: {error}")))?;
    Ok(envelope)
}
//#endregion 🔖Codec

//#region 🔖ReplicaState
/// @emoji 🧾 One document's currently-retained sync state, replayed fresh from its WAL — the
/// shared input every negotiation function below works from.
///
/// 🎯 Design choice (`Frontier` field derivation, since `db_document` doesn't exist yet to supply
/// an authoritative one): `head_seq` = count of `WAL_COMMAND` records replayed (genesis = 0);
/// `commit_seq` = count of `WAL_TX_COMMIT` records replayed; `chain_hash` = a replay-derived
/// content chain, `blake3(digest_1 || .. || digest_k)` where `digest_i = blake3(command_i's raw WAL
/// bytes)` — the same shape `protocol::verify_slice`'s `slice_content_chain` uses, chosen because
/// `db_wal` does not expose a public accessor for a segment's real commit `chain_hash` (see
/// `db_wal`'s own `SegmentWriter::tip_chain_hash`, which is private); `epoch` is always `0` here —
/// cluster fencing epochs are `db_cluster`'s concern, unreachable without a `CatalogStorage` scoped
/// to this specific document's shard, which this crate's inputs don't carry.
#[derive(Clone, Debug, PartialEq)]
pub struct DocumentSyncState {
    pub frontier: db_core::Frontier,
    pub commands: Vec<protocol::OperationEnvelope>,
    /// @emoji 🚧 The lowest `head_seq` this crate can still serve via tail (missing-command)
    /// transfer — the `head_seq` of the most recent `WAL_SNAPSHOT_PUB` record replayed, or `0` if
    /// none (nothing has ever been compacted away). A replica behind this floor needs
    /// `decide_bootstrap`'s snapshot path instead.
    pub floor_head_seq: u64,
}

/// @emoji 🔁 Replays `document`'s entire currently-retained WAL via `db_wal::replay_document` and
/// derives its `DocumentSyncState` — see the struct's doc for exactly how each field is derived.
pub fn replay_sync_state(storage: &dyn db_storage::WalStorage, document: db_core::DocumentId) -> Result<DocumentSyncState, db_core::DbError> {
    let records = db_wal::replay_document(storage, &document)?;
    let mut commands = Vec::new();
    let mut command_digests: Vec<[u8; 32]> = Vec::new();
    let mut commit_seq = 0u64;
    let mut floor_head_seq = 0u64;
    for record in &records {
        match record {
            db_wal::WalRecord::Command(bytes) => {
                commands.push(decode_command_envelope(bytes)?);
                command_digests.push(*blake3::hash(bytes).as_bytes());
            }
            db_wal::WalRecord::TxCommit { .. } => commit_seq += 1,
            // 🎯 Overwritten on every occurrence rather than max()'d: `WalRecord`s replay in
            // on-disk (chronological) order, so the last one seen is always the most recent.
            db_wal::WalRecord::SnapshotPub { frontier, .. } => floor_head_seq = frontier.head_seq,
            _ => {}
        }
    }
    let head_seq = commands.len() as u64;
    let chain_hash = fold_content_chain(&command_digests);
    let frontier = db_core::Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 };
    Ok(DocumentSyncState { frontier, commands, floor_head_seq })
}

/// @emoji 🔐 Folds per-command digests into one combined digest — see `DocumentSyncState`'s doc
/// for the derivation this implements. `[0u8; 32]` for an empty document, matching
/// `db_core::Frontier::genesis`'s all-zero `chain_hash`.
fn fold_content_chain(digests: &[[u8; 32]]) -> [u8; 32] {
    if digests.is_empty() {
        return [0u8; 32];
    }
    let mut concat = Vec::with_capacity(digests.len() * 32);
    for digest in digests {
        concat.extend_from_slice(digest);
    }
    *blake3::hash(&concat).as_bytes()
}
//#endregion 🔖ReplicaState

//#region 🔖Frontier
/// @emoji ➖ `db_core::FrontierDelta::between`, re-exposed under this crate's own name for
/// discoverability — frontier-delta computation is this crate's stated responsibility, so
/// `db_sync::frontier_delta` is the expected first stop even though the primitive itself lives in
/// `db_core`.
pub fn frontier_delta(from: &db_core::Frontier, to: &db_core::Frontier) -> Result<db_core::FrontierDelta, db_core::DbError> {
    db_core::FrontierDelta::between(from, to)
}

/// @emoji 🌉 `db_core::Frontier` -> `protocol::RuntimeFrontierSummary` (the wire-frame shape
/// `ServerFrame::{Welcome, Commands, Ack}.*frontier` fields carry). `head_edit_id` has no
/// `db_core::Frontier` counterpart (see `DocumentSyncState`'s doc); callers pass whatever they
/// consider the frontier's tip identity (`state_frontier_summary` below supplies the natural
/// choice: the last replayed command's `operation_id`).
pub fn to_frontier_summary(frontier: &db_core::Frontier, head_edit_id: String) -> protocol::RuntimeFrontierSummary {
    protocol::RuntimeFrontierSummary {
        document_id: protocol::DocumentId(frontier.document.0.clone()),
        head_edit_ordinal: frontier.head_seq,
        head_edit_id,
        last_commit_seq: frontier.commit_seq,
        chain_hash: frontier.chain_hash,
    }
}

/// @emoji 🌉 Inverse bridge direction: `protocol::RuntimeFrontierSummary` -> `db_core::Frontier`,
/// the primitive `handle_hello`/`handle_frontier_advertise` use to turn a replica's advertised
/// wire frontier into something `missing_commands`/`decide_bootstrap` can compare against a
/// `DocumentSyncState`. `epoch` is always `0` (see `DocumentSyncState`'s doc: `RuntimeFrontierSummary`
/// carries no cluster-fencing epoch at all).
pub fn from_frontier_summary(summary: &protocol::RuntimeFrontierSummary) -> db_core::Frontier {
    db_core::Frontier {
        document: db_core::DocumentId(summary.document_id.0.clone()),
        head_seq: summary.head_edit_ordinal,
        commit_seq: summary.last_commit_seq,
        chain_hash: summary.chain_hash,
        epoch: 0,
    }
}

/// @emoji 🌉 `state`'s own frontier as a `RuntimeFrontierSummary`, with `head_edit_id` filled from
/// the last replayed command's `operation_id` (empty string for a genesis document with no
/// commands yet).
pub fn state_frontier_summary(state: &DocumentSyncState) -> protocol::RuntimeFrontierSummary {
    let head_edit_id = state.commands.last().map(|envelope| envelope.operation_id.0.clone()).unwrap_or_default();
    to_frontier_summary(&state.frontier, head_edit_id)
}
//#endregion 🔖Frontier

//#region 🔖MissingCommands
/// @emoji 📦 The missing-command-transfer primitive: every command `state` holds strictly after
/// `replica`'s `head_seq`, in replay order — what `db_sync` ships a reconnecting/catching-up
/// replica via `ServerFrame::Commands`.
///
/// 🎯 Design choice (why not `protocol::extract_range`/`RecordSlice`): that primitive walks a
/// `.spr` stream for `protocol_core::REC_EDIT`-kind frames — the shape `protocol_history`'s
/// history-log format uses. `db_wal`'s WAL segments are also `.spr` containers but frame commands
/// under the family's own `WAL_COMMAND` (`0x44`) record kind in the `0x40..=0x4F` extension range
/// (see `db_wal`'s `//#region 🔖RecordKinds`), never `REC_EDIT` — so `extract_range` structurally
/// cannot find them. This function is this crate's `WAL_COMMAND`-shaped analog, built the same
/// way (a linear ordinal-indexed slice) but over `DocumentSyncState::commands`, which is already
/// the fully-decoded, ordinal-indexed sequence `replay_sync_state` produced.
pub fn missing_commands(state: &DocumentSyncState, replica: &db_core::Frontier) -> Result<Vec<protocol::OperationEnvelope>, db_core::DbError> {
    if replica.document != state.frontier.document {
        return Err(db_core::DbError::InvalidArgument(format!(
            "frontier document mismatch: replica {} vs server {}",
            replica.document, state.frontier.document
        )));
    }
    if replica.head_seq > state.frontier.head_seq {
        return Err(db_core::DbError::InvalidArgument(format!(
            "replica frontier is ahead of the server: replica head_seq {} > server head_seq {}",
            replica.head_seq, state.frontier.head_seq
        )));
    }
    if replica.head_seq < state.floor_head_seq {
        return Err(db_core::DbError::Unavailable(format!(
            "replica head_seq {} is behind the retained WAL floor {}; snapshot bootstrap is required",
            replica.head_seq, state.floor_head_seq
        )));
    }
    Ok(state.commands[replica.head_seq as usize..].to_vec())
}

/// @emoji 📨 Wraps `envelopes` (typically `missing_commands`' result) as a `ServerFrame::Commands`
/// stamped with `state`'s current frontier — `origin` is the relaying actor identity the caller
/// (the hub session layer, which owns its own actor identity) supplies; this crate has no opinion
/// on it beyond passing it through.
pub fn commands_server_frame(state: &DocumentSyncState, envelopes: Vec<protocol::OperationEnvelope>, origin: protocol::ActorId) -> protocol::ServerFrame {
    protocol::ServerFrame::Commands { envelopes, origin, frontier: state_frontier_summary(state) }
}
//#endregion 🔖MissingCommands

//#region 🔖Bootstrap
/// @emoji 🚀 How a (re)connecting replica should be caught up, decided by `decide_bootstrap` —
/// the pre-wire-encoding twin of `protocol::Bootstrap` (kept separate so this crate's core
/// decision logic stays testable without constructing full `ServerFrame`s; `build_welcome` below
/// lowers it to the wire shape).
#[derive(Clone, Debug, PartialEq)]
pub enum BootstrapPlan {
    /// @emoji ✅ The replica is already fully caught up — nothing to send.
    None,
    /// @emoji 🚚 The replica is within the retained WAL floor: ship it the missing commands
    /// directly, no snapshot needed.
    Tail { envelopes: Vec<protocol::OperationEnvelope> },
    /// @emoji 📸 The replica is behind the retained WAL floor (or brand new against a compacted
    /// document): ship it a whole snapshot generation first.
    Snapshot { generation: u64, bytes: Vec<u8>, pack_hash: [u8; 32] },
}

/// @emoji 🧭 Decides `BootstrapPlan` for `replica` (`None` meaning a totally fresh replica with no
/// prior frontier at all) against `state`, consulting `snapshots` only when the replica's
/// `head_seq` has fallen behind `state.floor_head_seq`.
pub fn decide_bootstrap(
    state: &DocumentSyncState,
    snapshots: &dyn db_storage::SnapshotStorage,
    replica: Option<&db_core::Frontier>,
) -> Result<BootstrapPlan, db_core::DbError> {
    let replica_head_seq = replica.map_or(0, |frontier| frontier.head_seq);
    if replica_head_seq >= state.floor_head_seq {
        let missing = match replica {
            Some(frontier) => missing_commands(state, frontier)?,
            None => state.commands.clone(),
        };
        return Ok(if missing.is_empty() { BootstrapPlan::None } else { BootstrapPlan::Tail { envelopes: missing } });
    }
    let generation = snapshots.latest_generation(&state.frontier.document)?.ok_or_else(|| {
        db_core::DbError::Unavailable(format!(
            "replica head_seq {replica_head_seq} is behind the retained WAL floor {} and no snapshot generation is available",
            state.floor_head_seq
        ))
    })?;
    let bytes = snapshots.read_generation(&state.frontier.document, generation)?;
    let pack_hash = *blake3::hash(&bytes).as_bytes();
    Ok(BootstrapPlan::Snapshot { generation, bytes, pack_hash })
}
//#endregion 🔖Bootstrap

//#region 🔖ResumeToken
/// @emoji 🎫 Issues a fresh resume token for `frontier` — the send-path half of resume tokens (see
/// module doc for why the receive path uses `Hello.frontier` instead). `Welcome.resume_token` is
/// always populated from this.
pub fn issue_resume_token(frontier: &db_core::Frontier) -> Result<String, db_core::DbError> {
    Ok(db_core::ResumeToken::encode(frontier)?.as_str().to_string())
}
//#endregion 🔖ResumeToken

//#region 🔖Hello
/// @emoji 🚀 What `handle_hello` produces: the `Welcome` frame itself, plus whatever follow-up
/// frames the chosen bootstrap needs (a single `Commands` frame for `Tail`; one `SnapshotChunk`
/// per chunk plus a trailing `SnapshotDone` for a non-inlined `Snapshot`; none for `None` or an
/// inlined `Snapshot`, whose bytes already travel inside `Welcome.bootstrap`).
#[derive(Clone, Debug, PartialEq)]
pub struct WelcomeResponse {
    pub welcome: protocol::ServerFrame,
    pub follow_up: Vec<protocol::ServerFrame>,
}

/// @emoji 🏗️ Lowers a `BootstrapPlan` to the wire `protocol::Bootstrap` shape plus its follow-up
/// frames. A `Snapshot` whose bytes fit within `snapshot_chunk_bytes` is inlined directly into
/// `Bootstrap::Snapshot.inline` (no follow-up frames); a larger one is chunked instead — this
/// crate's own choice of threshold behavior, since the contract fixes `Bootstrap::Snapshot`'s two
/// shapes but not when to prefer one over the other.
fn lower_bootstrap_plan(plan: &BootstrapPlan, state: &DocumentSyncState, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> (protocol::Bootstrap, Vec<protocol::ServerFrame>) {
    match plan {
        BootstrapPlan::None => (protocol::Bootstrap::None, Vec::new()),
        BootstrapPlan::Tail { envelopes } => (protocol::Bootstrap::Tail, vec![commands_server_frame(state, envelopes.clone(), origin.clone())]),
        BootstrapPlan::Snapshot { bytes, pack_hash, .. } => {
            if bytes.len() <= snapshot_chunk_bytes {
                (protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: Some(bytes.clone()) }, Vec::new())
            } else {
                let chunks: Vec<&[u8]> = bytes.chunks(snapshot_chunk_bytes).collect();
                let mut follow_up: Vec<protocol::ServerFrame> = chunks
                    .iter()
                    .enumerate()
                    .map(|(seq, chunk)| protocol::ServerFrame::SnapshotChunk { seq: seq as u32, bytes: chunk.to_vec() })
                    .collect();
                follow_up.push(protocol::ServerFrame::SnapshotDone { seq_count: chunks.len() as u32 });
                (protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: None }, follow_up)
            }
        }
    }
}

/// @emoji 🏗️ Builds the full `WelcomeResponse` for `plan` against `state`. `snapshot_chunk_bytes`
/// must be non-zero (validated before `lower_bootstrap_plan` could otherwise divide the snapshot
/// into a runaway number of zero-progress chunks).
pub fn build_welcome(state: &DocumentSyncState, plan: &BootstrapPlan, session_id: String, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<WelcomeResponse, db_core::DbError> {
    if snapshot_chunk_bytes == 0 {
        return Err(db_core::DbError::InvalidArgument("snapshot_chunk_bytes must be non-zero".to_string()));
    }
    let resume_token = issue_resume_token(&state.frontier)?;
    let (bootstrap, follow_up) = lower_bootstrap_plan(plan, state, origin, snapshot_chunk_bytes);
    let welcome = protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier: state_frontier_summary(state), bootstrap };
    Ok(WelcomeResponse { welcome, follow_up })
}

/// @emoji 👋 The top-level entry point for a `protocol::ClientFrame::Hello`: replays `document`'s
/// current sync state, decides a bootstrap plan against `hello_frontier` (the replica's advertised
/// `RuntimeFrontierSummary`, `None` for a totally fresh replica — see module doc for why this
/// crate reads `Hello.frontier` rather than decoding `Hello.resume_token`), and lowers it to a
/// `WelcomeResponse`.
pub fn handle_hello(
    storage: &dyn db_storage::DbStorage,
    document: db_core::DocumentId,
    hello_frontier: Option<&protocol::RuntimeFrontierSummary>,
    session_id: String,
    origin: &protocol::ActorId,
    snapshot_chunk_bytes: usize,
) -> Result<WelcomeResponse, db_core::DbError> {
    let state = replay_sync_state(storage.wal(), document)?;
    let replica = hello_frontier.map(from_frontier_summary);
    let plan = decide_bootstrap(&state, storage.snapshot(), replica.as_ref())?;
    build_welcome(&state, &plan, session_id, origin, snapshot_chunk_bytes)
}

/// @emoji 📡 Mid-session catch-up: a connected replica sends `ClientFrame::FrontierAdvertise`
/// (e.g. after a period of being caught up passively via broadcast, to confirm its position) and
/// the hub replies with whatever commands it's still missing, or `None` if it's already current.
pub fn handle_frontier_advertise(
    storage: &dyn db_storage::WalStorage,
    document: db_core::DocumentId,
    advertised: &protocol::RuntimeFrontierSummary,
    origin: protocol::ActorId,
) -> Result<Option<protocol::ServerFrame>, db_core::DbError> {
    let state = replay_sync_state(storage, document)?;
    let replica = from_frontier_summary(advertised);
    let missing = missing_commands(&state, &replica)?;
    Ok(if missing.is_empty() { None } else { Some(commands_server_frame(&state, missing, origin)) })
}
//#endregion 🔖Hello

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_core::DocumentId;
    use db_storage::MemoryStorage;
    use db_wal::{DocumentWal, GroupCommitPolicy, WalRecord};

    //#region 🧸Fixtures
    fn sample_envelope(id: &str, seq: u64) -> protocol::OperationEnvelope {
        protocol::OperationEnvelope {
            operation_id: protocol::OperationId(id.to_string()),
            document_id: protocol::DocumentId("doc-1".to_string()),
            actor: protocol::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff { schema: protocol::SchemaId("diff.v1".to_string()), payload: seq.to_le_bytes().to_vec() },
            inverse: protocol::InverseOperation { schema: protocol::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
        }
    }

    /// @emoji 🧸 Creates `document`'s WAL in `storage` and submits `count` sample commands
    /// (ids `"op-0".."op-{count-1}"`), each `Fsync`-durable so replay sees them immediately.
    fn seed_wal(storage: &MemoryStorage, document: &DocumentId, count: u64) {
        let mut wal = DocumentWal::create(storage, document.clone(), GroupCommitPolicy::default(), 0).unwrap();
        for i in 0..count {
            let envelope = sample_envelope(&format!("op-{i}"), i);
            let bytes = encode_command_envelope(&envelope);
            wal.submit(storage, &[WalRecord::Command(bytes)], db_core::DurabilityClass::Fsync, i).unwrap();
        }
    }

    /// @emoji 🧸 Reopens `document`'s WAL and appends one `SnapshotPub` marker covering `frontier`.
    fn publish_snapshot_marker(storage: &MemoryStorage, document: &DocumentId, generation: u64, frontier: db_core::Frontier) {
        let (mut wal, _report) = DocumentWal::open(storage, document.clone(), GroupCommitPolicy::default(), 1000).unwrap();
        wal.submit(storage, &[WalRecord::SnapshotPub { generation, frontier }], db_core::DurabilityClass::Fsync, 1000).unwrap();
    }
    //#endregion 🧸Fixtures

    //#region 🔖Codec
    #[test]
    fn command_envelope_round_trips_through_encode_decode() {
        let envelope = sample_envelope("op-1", 7);
        let bytes = encode_command_envelope(&envelope);
        assert_eq!(decode_command_envelope(&bytes).unwrap(), envelope);
    }

    #[test]
    fn decode_command_envelope_rejects_malformed_bytes_without_panicking() {
        assert!(matches!(decode_command_envelope(b"not json"), Err(db_core::DbError::Corrupt(_))));
    }
    //#endregion 🔖Codec

    //#region 🔖ReplicaState
    #[test]
    fn replay_sync_state_derives_frontier_and_ordered_commands() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 3);

        let state = replay_sync_state(&storage, document).unwrap();
        assert_eq!(state.frontier.head_seq, 3);
        assert_eq!(state.frontier.commit_seq, 3);
        assert_eq!(state.floor_head_seq, 0);
        assert_eq!(state.commands.len(), 3);
        assert_eq!(state.commands[0].operation_id.0, "op-0");
        assert_eq!(state.commands[2].operation_id.0, "op-2");
    }

    #[test]
    fn replay_sync_state_on_empty_document_is_genesis() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 0);

        let state = replay_sync_state(&storage, document).unwrap();
        assert_eq!(state.frontier.head_seq, 0);
        assert_eq!(state.frontier.chain_hash, [0u8; 32]);
        assert!(state.commands.is_empty());
    }

    #[test]
    fn replay_sync_state_tracks_the_latest_snapshot_pub_as_the_floor() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 5);
        let floor_frontier = db_core::Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [1u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 1, floor_frontier);

        let state = replay_sync_state(&storage, document).unwrap();
        assert_eq!(state.floor_head_seq, 2);
        assert_eq!(state.frontier.head_seq, 5, "the marker itself carries no commands");
    }
    //#endregion 🔖ReplicaState

    //#region 🔖Frontier
    #[test]
    fn frontier_delta_reports_the_command_gap_and_rejects_backwards() {
        let document: DocumentId = "doc-1".into();
        let from = db_core::Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [0u8; 32], epoch: 0 };
        let to = db_core::Frontier { document, head_seq: 5, commit_seq: 5, chain_hash: [9u8; 32], epoch: 0 };

        let delta = frontier_delta(&from, &to).unwrap();
        assert_eq!(delta.commands, 3);
        assert!(!delta.is_empty());
        assert!(frontier_delta(&to, &from).is_err(), "a delta only ever moves forward");
    }

    #[test]
    fn frontier_summary_bridges_round_trip() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 2);
        let state = replay_sync_state(&storage, document).unwrap();

        let summary = state_frontier_summary(&state);
        assert_eq!(summary.head_edit_ordinal, 2);
        assert_eq!(summary.head_edit_id, "op-1");
        assert_eq!(summary.last_commit_seq, state.frontier.commit_seq);
        assert_eq!(summary.chain_hash, state.frontier.chain_hash);

        let bridged_back = from_frontier_summary(&summary);
        assert_eq!(bridged_back.head_seq, state.frontier.head_seq);
        assert_eq!(bridged_back.commit_seq, state.frontier.commit_seq);
        assert_eq!(bridged_back.chain_hash, state.frontier.chain_hash);
        assert_eq!(bridged_back.document, state.frontier.document);
    }
    //#endregion 🔖Frontier

    //#region 🔖MissingCommands
    #[test]
    fn missing_commands_transfer_round_trip_from_genesis() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 4);
        let state = replay_sync_state(&storage, document.clone()).unwrap();

        let replica_frontier = db_core::Frontier::genesis(document);
        let missing = missing_commands(&state, &replica_frontier).unwrap();
        assert_eq!(missing, state.commands, "a genesis replica is missing every command");

        // "Applying" the transfer catches the replica up to the server's frontier exactly.
        let caught_up = state.frontier.clone();
        assert!(missing_commands(&state, &caught_up).unwrap().is_empty());
    }

    #[test]
    fn missing_commands_transfer_round_trip_for_a_partially_caught_up_replica() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 3);
        let first_state = replay_sync_state(&storage, document.clone()).unwrap();
        let replica_frontier = first_state.frontier;

        // More commands land on the server after the replica already caught up once.
        {
            let (mut wal, _report) = DocumentWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100).unwrap();
            for i in 3..6u64 {
                let envelope = sample_envelope(&format!("op-{i}"), i);
                wal.submit(&storage, &[WalRecord::Command(encode_command_envelope(&envelope))], db_core::DurabilityClass::Fsync, i).unwrap();
            }
        }

        let second_state = replay_sync_state(&storage, document).unwrap();
        let missing = missing_commands(&second_state, &replica_frontier).unwrap();
        assert_eq!(missing.len(), 3);
        assert_eq!(missing[0].operation_id.0, "op-3");
        assert_eq!(missing[2].operation_id.0, "op-5");
    }

    #[test]
    fn missing_commands_rejects_document_mismatch_and_a_replica_ahead_of_server() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 2);
        let state = replay_sync_state(&storage, document).unwrap();

        let other_document = db_core::Frontier::genesis("doc-2".into());
        assert!(matches!(missing_commands(&state, &other_document), Err(db_core::DbError::InvalidArgument(_))));

        let ahead = db_core::Frontier { head_seq: 99, ..state.frontier.clone() };
        assert!(matches!(missing_commands(&state, &ahead), Err(db_core::DbError::InvalidArgument(_))));
    }

    #[test]
    fn missing_commands_rejects_a_replica_behind_the_retained_floor() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 5);
        let floor_frontier = db_core::Frontier { document: document.clone(), head_seq: 3, commit_seq: 3, chain_hash: [2u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 1, floor_frontier);
        let state = replay_sync_state(&storage, document.clone()).unwrap();

        let too_far_behind = db_core::Frontier { document, head_seq: 1, commit_seq: 1, chain_hash: [0u8; 32], epoch: 0 };
        assert!(matches!(missing_commands(&state, &too_far_behind), Err(db_core::DbError::Unavailable(_))));
    }
    //#endregion 🔖MissingCommands

    //#region 🔖Bootstrap
    #[test]
    fn decide_bootstrap_serves_tail_for_a_fresh_replica_within_the_floor() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 3);
        let state = replay_sync_state(&storage, document).unwrap();

        let plan = decide_bootstrap(&state, &storage, None).unwrap();
        assert_eq!(plan, BootstrapPlan::Tail { envelopes: state.commands });
    }

    #[test]
    fn decide_bootstrap_reports_none_for_an_already_caught_up_replica() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 3);
        let state = replay_sync_state(&storage, document).unwrap();

        let plan = decide_bootstrap(&state, &storage, Some(&state.frontier)).unwrap();
        assert_eq!(plan, BootstrapPlan::None);
    }

    #[test]
    fn decide_bootstrap_serves_snapshot_when_a_generation_is_available_below_the_floor() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 5);
        let floor_frontier = db_core::Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 7, floor_frontier);
        db_storage::SnapshotStorage::write_generation(&storage, &document, 7, b"snapshot-bytes").unwrap();
        let state = replay_sync_state(&storage, document.clone()).unwrap();

        let stale_replica = db_core::Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
        let plan = decide_bootstrap(&state, &storage, Some(&stale_replica)).unwrap();
        match plan {
            BootstrapPlan::Snapshot { generation, bytes, pack_hash } => {
                assert_eq!(generation, 7);
                assert_eq!(bytes, b"snapshot-bytes");
                assert_eq!(pack_hash, *blake3::hash(b"snapshot-bytes").as_bytes());
            }
            other => panic!("expected a Snapshot plan, got {other:?}"),
        }
    }

    #[test]
    fn decide_bootstrap_reports_unavailable_when_below_floor_with_no_snapshot() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 5);
        let floor_frontier = db_core::Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 7, floor_frontier);
        let state = replay_sync_state(&storage, document.clone()).unwrap();

        let stale_replica = db_core::Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
        assert!(matches!(decide_bootstrap(&state, &storage, Some(&stale_replica)), Err(db_core::DbError::Unavailable(_))));
    }
    //#endregion 🔖Bootstrap

    //#region 🔖ResumeToken
    #[test]
    fn issue_resume_token_produces_the_documented_v1_wire_format() {
        let document: DocumentId = "doc-1".into();
        let frontier = db_core::Frontier { document, head_seq: 4, commit_seq: 4, chain_hash: [5u8; 32], epoch: 0 };
        let token = issue_resume_token(&frontier).unwrap();
        assert!(token.starts_with("v1|doc-1|4|4|0|"));
    }
    //#endregion 🔖ResumeToken

    //#region 🔖Hello
    #[test]
    fn handle_hello_bootstraps_a_fresh_replica_via_tail_and_issues_a_resume_token() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 3);

        let response = handle_hello(&storage, document, None, "session-1".to_string(), &protocol::ActorId("hub".to_string()), 64 * 1024).unwrap();
        let protocol::ServerFrame::Welcome { bootstrap, server_frontier, resume_token, .. } = &response.welcome else {
            panic!("expected a Welcome frame");
        };
        assert_eq!(*bootstrap, protocol::Bootstrap::Tail);
        assert_eq!(server_frontier.head_edit_ordinal, 3);
        assert!(!resume_token.is_empty());
        assert_eq!(response.follow_up.len(), 1);
        match &response.follow_up[0] {
            protocol::ServerFrame::Commands { envelopes, .. } => assert_eq!(envelopes.len(), 3),
            other => panic!("expected a Commands follow-up frame, got {other:?}"),
        }
    }

    #[test]
    fn handle_hello_reports_no_follow_up_for_an_already_caught_up_replica() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 2);
        let state = replay_sync_state(&storage, document.clone()).unwrap();
        let hello_frontier = state_frontier_summary(&state);

        let response = handle_hello(&storage, document, Some(&hello_frontier), "session-2".to_string(), &protocol::ActorId("hub".to_string()), 64 * 1024).unwrap();
        let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
            panic!("expected a Welcome frame");
        };
        assert_eq!(*bootstrap, protocol::Bootstrap::None);
        assert!(response.follow_up.is_empty());
    }

    #[test]
    fn handle_hello_chunks_a_snapshot_larger_than_the_requested_chunk_size() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 4);
        let floor_frontier = db_core::Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 9, floor_frontier);
        let big_snapshot = vec![7u8; 10];
        db_storage::SnapshotStorage::write_generation(&storage, &document, 9, &big_snapshot).unwrap();

        let stale_hello_frontier = protocol::RuntimeFrontierSummary { document_id: protocol::DocumentId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] };
        let response = handle_hello(&storage, document, Some(&stale_hello_frontier), "session-3".to_string(), &protocol::ActorId("hub".to_string()), 4).unwrap();

        let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
            panic!("expected a Welcome frame");
        };
        assert!(matches!(bootstrap, protocol::Bootstrap::Snapshot { inline: None, .. }));
        // 10 bytes chunked at 4 bytes/chunk -> 3 chunks (4, 4, 2), plus one SnapshotDone.
        assert_eq!(response.follow_up.len(), 4);
        assert!(matches!(response.follow_up[3], protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
    }

    #[test]
    fn handle_hello_rejects_zero_snapshot_chunk_bytes() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 1);
        assert!(matches!(handle_hello(&storage, document, None, "s".to_string(), &protocol::ActorId("hub".to_string()), 0), Err(db_core::DbError::InvalidArgument(_))));
    }

    #[test]
    fn handle_frontier_advertise_relays_missing_commands_and_none_when_caught_up() {
        let storage = MemoryStorage::new();
        let document: DocumentId = "doc-1".into();
        seed_wal(&storage, &document, 2);
        let first_state = replay_sync_state(&storage, document.clone()).unwrap();
        let replica_summary = state_frontier_summary(&first_state);

        {
            let (mut wal, _report) = DocumentWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100).unwrap();
            let envelope = sample_envelope("op-2", 2);
            wal.submit(&storage, &[WalRecord::Command(encode_command_envelope(&envelope))], db_core::DurabilityClass::Fsync, 100).unwrap();
        }

        let frame = handle_frontier_advertise(&storage, document.clone(), &replica_summary, protocol::ActorId("hub".to_string())).unwrap();
        match frame {
            Some(protocol::ServerFrame::Commands { envelopes, .. }) => {
                assert_eq!(envelopes.len(), 1);
                assert_eq!(envelopes[0].operation_id.0, "op-2");
            }
            other => panic!("expected a Commands frame, got {other:?}"),
        }

        let up_to_date_state = replay_sync_state(&storage, document.clone()).unwrap();
        let up_to_date_summary = state_frontier_summary(&up_to_date_state);
        assert!(handle_frontier_advertise(&storage, document, &up_to_date_summary, protocol::ActorId("hub".to_string())).unwrap().is_none());
    }
    //#endregion 🔖Hello
}
//#endregion 🧪Tests
