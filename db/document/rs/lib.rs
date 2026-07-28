//! 🗄️ `db_document` — the document authority actor and its command pipeline: admit → dedupe →
//! base-resolve → authz → deps → validate → conflict → execute → WAL append → durability →
//! publish → project → vcs → preview-reconcile → receipt. Composes `db_state` (materialized
//! overlay), `db_wal` (durability), `db_storage` (the pluggable substrate), and `protocol`
//! (`OperationEnvelope`/`OperationDiff`) into `DocumentEngine`, the crate's central type, plus a
//! thin `db_actor`-mailbox wrapper (`DocumentAuthority`) around it. Frozen contract:
//! `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_document` row).
//!
//! 🎯 Design choice (compatibility surface — re-checked against `db_engine`'s live state, which
//! was itself being revised concurrently while this file was authored): `db_engine`'s
//! `document_engine_config` now builds `DocumentEngineConfig{limits, security, version_graph,
//! emit, ..DocumentEngineConfig::default()}` (a real `db_security::SecurityGate` baked directly
//! in, `version_graph: Arc<dyn VersionGraph>` non-optional, defaulted `..` spread for the rest),
//! `CommandReceipt.conflicts: Vec<db_conflict::ConflictRecord>` (the real type), and
//! `DocumentAuthority::run_query_blocking(query, consistency)` (two arguments, `db_query::
//! Consistency`-aware). This revision matches that shape exactly: `security`/`emit` are real
//! `DocumentEngineConfig` fields, `version_graph` is required (`db_core::NullVersionGraph` is the
//! "no vcs" default rather than `Option::None`), `submit`'s conflict step is a genuine
//! `db_conflict::ConflictDetector` fed by retained recent-commit `TouchedSet` history (not a local
//! last-writer stand-in), and `query`/`RunQuery`/`run_query_blocking` take a `db_query::
//! Consistency` and resolve it via `db_query::resolve_consistency` + `db_index::
//! IndexConsistencyResolver`. `AuthzHook`/`AllowAll` are kept defined (unused in the hot `submit`
//! path now that `security` supersedes them) purely because they are still a public, documented
//! extension seam and cost nothing to keep — a caller may still hand-roll one. Because
//! `DocumentEngineConfig`'s new fields are absorbed via `..Default::default()` at every call site
//! observed, `db_projection` registration is also wired in as a `projections` factory field (see
//! `🔖Engine`'s doc for why a factory, not a stored engine).
//!
//! 🎯 Design choice (diff convention, unchanged): `protocol::OperationEnvelope`'s `diff`/`inverse`
//! payloads are schema-erased `serde_json::Value`s — `db_document` has no compile-time knowledge of
//! any concrete document schema, so it adopts one generic convention for BOTH: a JSON *object*
//! whose keys are `db_state`-style `/`-segmented paths and whose values are either the new JSON
//! value to set at that path, or JSON `null` as an explicit tombstone (path deleted). This is a
//! real, documented limitation, not a workaround: a legitimate application-level `null` value is
//! indistinguishable from a delete under this convention. `envelope_from_operation` (new this
//! revision) is the generic ingestion boundary that actually exercises `protocol::Operation`/
//! `OperationDiff` (`diff`/`apply`/`operation_id`/`dependencies`/`author_id`/`timestamp`) to build
//! an envelope in this same convention from a typed operation — the one place in the `db` family
//! below `db_document` allowed to interpret operation semantics at all (per the contract's hard
//! dependency rule).

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use db_core::DbError;
use protocol::OperationDiff as _;

//#region 🔖Ids
/// @emoji 🌉 `protocol::DocumentId` → `db_core::DocumentId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises (see `db_core::DocumentId`'s own doc for the rationale: this
/// crate is the first one in the family that depends on both `db_core` and `protocol` and so is
/// where the bridge is actually exercised).
fn to_core_document_id(id: &protocol::DocumentId) -> db_core::DocumentId {
    db_core::DocumentId(id.0.clone())
}

/// @emoji 🌉 `protocol::ActorId` → `db_core::ActorId`, same bridge as `to_core_document_id`.
fn to_core_actor_id(id: &protocol::ActorId) -> db_core::ActorId {
    db_core::ActorId(id.0.clone())
}

// 🔒 Used as a bare fn-pointer error mapper (`.map_err(json_err)`) throughout this file —
// `Result::map_err`'s `FnOnce(E) -> F2` bound always calls the mapper with an owned `E`, so a
// by-reference signature would not type-check at any of those call sites despite the function
// body itself only borrowing `err` to format it.
#[allow(clippy::needless_pass_by_value)]
fn json_err(err: serde_json::Error) -> DbError {
    DbError::InvalidArgument(format!("db_document json error: {err}"))
}
//#endregion 🔖Ids

//#region 🔖Command
/// @emoji 📦 One atomically-submitted group of causally-related operations against a single
/// document — the unit `DocumentEngine::submit` accepts. Every envelope must target the same
/// `document_id` (checked at construction, and again against the engine's own document at submit
/// time); the batch's `command_id` (for dedupe/the returned `CommandReceipt`) is its LAST
/// envelope's `operation_id`, since `OperationEnvelope` has no separate batch-level id of its own.
pub struct CommandBatch {
    pub envelopes: Vec<protocol::OperationEnvelope>,
}

impl CommandBatch {
    /// @emoji 🏗️ Builds a batch, rejecting an empty one or one whose envelopes disagree on
    /// `document_id`.
    pub fn new(envelopes: Vec<protocol::OperationEnvelope>) -> Result<CommandBatch, DbError> {
        let first = envelopes.first().ok_or_else(|| DbError::InvalidArgument("command batch must contain at least one operation".to_string()))?;
        let document_id = first.document_id.clone();
        if envelopes.iter().any(|envelope| envelope.document_id != document_id) {
            return Err(DbError::InvalidArgument("every envelope in a command batch must target the same document".to_string()));
        }
        Ok(CommandBatch { envelopes })
    }
}

/// @emoji 🎚️ Per-submit durability override — see `db_core::DurabilityClass`'s doc for the
/// strength ordering `DocumentWal::submit` honors.
#[derive(Clone, Copy, Debug)]
pub struct SubmitOptions {
    pub durability: db_core::DurabilityClass,
}

impl Default for SubmitOptions {
    fn default() -> Self {
        SubmitOptions { durability: db_core::DurabilityClass::Memory }
    }
}
//#endregion 🔖Command

//#region 🔖Diff
/// @emoji 🧮 Flattens a diff/inverse JSON object into `(path, Some(value) | None)` pairs per this
/// module's generic path-value convention (see module doc). Errors if `value` is not a JSON
/// object — this crate's own schema-erased documents have no other shape it can interpret.
fn entries_from_value(value: &serde_json::Value) -> Result<Vec<(String, Option<serde_json::Value>)>, DbError> {
    let object = value
        .as_object()
        .ok_or_else(|| DbError::InvalidArgument("diff/inverse payload must be a JSON object of path -> value".to_string()))?;
    Ok(object.iter().map(|(path, entry)| (path.clone(), if entry.is_null() { None } else { Some(entry.clone()) })).collect())
}

/// @emoji ➡️ Entries for an envelope's forward diff.
fn diff_entries(diff: &protocol::DocumentDiff) -> Result<Vec<(String, Option<serde_json::Value>)>, DbError> {
    entries_from_value(&diff.payload)
}

/// @emoji ↩️ Entries for an envelope's inverse diff (the `undo` pipeline's source).
fn inverse_entries(inverse: &protocol::InverseOperation) -> Result<Vec<(String, Option<serde_json::Value>)>, DbError> {
    entries_from_value(&inverse.inverse_diff)
}

/// @emoji 🧮 The inverse of `entries_from_value` — rebuilds a JSON object from path-value pairs,
/// `None` becoming an explicit `null` tombstone. Used by `undo` to construct a compensating
/// envelope's diff/inverse payloads.
fn entries_to_value(entries: &[(String, Option<serde_json::Value>)]) -> serde_json::Value {
    let mut object = serde_json::Map::with_capacity(entries.len());
    for (path, value) in entries {
        object.insert(path.clone(), value.clone().unwrap_or(serde_json::Value::Null));
    }
    serde_json::Value::Object(object)
}

/// @emoji 👣 The `TouchedSet` a set of entries would write — shared by `DocumentState::
/// apply_entries` and preview publishing.
fn entries_touched(entries: &[(String, Option<serde_json::Value>)]) -> db_state::TouchedSet {
    let mut touched = db_state::TouchedSet::new();
    for (path, _) in entries {
        touched.record(db_state::TouchedRegion::write(path.clone()));
    }
    touched
}
//#endregion 🔖Diff

//#region 🔖Bridge
/// @emoji 🌉 The generic ingestion boundary: builds an `OperationEnvelope` (in this crate's own
/// path-value diff convention) from a typed `protocol::Operation<P>` against a serializable
/// projection `P`, writing the whole post-state at `path`. Genuinely exercises `Operation`/
/// `OperationDiff`'s trait methods (`diff`/`apply`/`operation_id`/`dependencies`/`author_id`/
/// `timestamp`) — see the module doc's design-choice note on why this crate is allowed to. A
/// caller wanting per-sub-path granularity builds the JSON object directly via
/// `CommandBatch::new`/`entries_to_value` instead of this whole-projection convenience.
pub fn envelope_from_operation<P, Op>(
    document: protocol::DocumentId,
    path: &str,
    op: &Op,
    base: &P,
    default_actor: protocol::ActorId,
    default_operation_id: protocol::OperationId,
    default_timestamp: protocol::HybridLogicalTimestamp,
) -> Result<protocol::OperationEnvelope, DbError>
where
    P: serde::Serialize,
    Op: protocol::Operation<P>,
{
    let diff = op.diff(base);
    let post = diff.apply(base);
    let mut forward = serde_json::Map::with_capacity(1);
    forward.insert(path.to_string(), serde_json::to_value(&post).map_err(json_err)?);
    let mut backward = serde_json::Map::with_capacity(1);
    backward.insert(path.to_string(), serde_json::to_value(base).map_err(json_err)?);
    let schema = std::any::type_name::<Op>().to_string();
    Ok(protocol::OperationEnvelope {
        operation_id: op.operation_id().unwrap_or(default_operation_id),
        document_id: document,
        actor: op.author_id().unwrap_or(default_actor),
        dependencies: op.dependencies(),
        diff: protocol::DocumentDiff { schema: schema.clone(), payload: serde_json::Value::Object(forward) },
        inverse: protocol::InverseOperation { schema, inverse_diff: serde_json::Value::Object(backward) },
        timestamp: op.timestamp().unwrap_or(default_timestamp),
    })
}
//#endregion 🔖Bridge

//#region 🔖Conflict
/// @emoji ⚔️ One detected write/write intersection between the currently-executing operation and
/// an earlier operation it did not declare as a `dependency`. Detected directly off
/// `db_state::TouchedRegion::path_intersects` (see `DocumentState::apply_entries`). Resolution
/// policy is last-writer-wins (the conflicting write still applies) — recorded for the caller's
/// visibility only, never rejected, since every commit in a single-document-actor pipeline is
/// already fully serialized by the time this runs. Kept a local, path-granular shape (rather than
/// `db_conflict::ConflictRecord`, which reports per conflicting COMMAND PAIR) because it is
/// `db_engine`'s own frozen `CommandReceipt.conflicts` element type — see module doc.
/// `preview_conflicts` (below) is the real, additive `db_conflict::ConflictDetector` integration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictRecord {
    pub command_id: protocol::OperationId,
    pub conflicting_with: protocol::OperationId,
    pub path: String,
}

/// @emoji ⚔️ Builds the `db_conflict::CommandTouch` `envelope` would produce, without applying it —
/// shared by `submit`'s recent-history bookkeeping and `preview_conflicts`'s real `db_conflict::
/// ConflictDetector` use. `conflict_rule` defaults uniformly to `Merge(LwwRegister)`: a raw
/// `OperationEnvelope` carries no per-operation `ConflictRule` of its own (that lives on
/// `protocol::Operation`, one layer up — see `envelope_from_operation`'s doc).
fn command_touch(envelope: &protocol::OperationEnvelope, touched: &db_state::TouchedSet) -> db_conflict::CommandTouch {
    let touch = db_conflict::CommandTouch::new(
        envelope.operation_id.clone(),
        envelope.actor.clone(),
        db_conflict::CommandKind::from(envelope.diff.schema.as_str()),
        protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister),
        envelope.timestamp,
    );
    touched.regions.iter().fold(touch, |touch, region| touch.touch(region.clone()))
}
//#endregion 🔖Conflict

//#region 🔖Receipt
/// @emoji 🧾 What `DocumentEngine::submit` returns: the committed batch's identity, the document's
/// new `Frontier`, the durability actually requested, any detected conflicts, and the post-commit
/// state's content hash. Mirrors the `db` facade's frozen `CommandReceipt` shape, except `frontier`
/// is `db_core::Frontier` (this crate's own internal currency) rather than the facade's
/// `protocol::DocumentId`-keyed twin — the facade converts via `to_core_document_id`'s inverse at
/// its own boundary (see module doc's bridge note).
#[derive(Clone, Debug, PartialEq)]
pub struct CommandReceipt {
    pub command_id: protocol::OperationId,
    pub frontier: db_core::Frontier,
    pub durability: db_core::DurabilityClass,
    pub conflicts: Vec<ConflictRecord>,
    pub state_hash: Option<pack::ContentHash>,
}

/// @emoji 📤 One committed operation's opaque effect bytes, queued for downstream
/// replication/notification (`db_sync`/`db_engine`'s concern to actually drain and ship — this
/// crate only accumulates and hands them out via `DocumentEngine::drain_outbox`).
#[derive(Clone, Debug)]
pub struct OutboxEntry {
    pub operation_id: protocol::OperationId,
    pub bytes: Vec<u8>,
}

/// @emoji 📣 One commit's live-query-relevant summary — `DocumentEngine::commit_log` accumulates
/// these so a poll-based subscriber can diff its last-seen index against the log to discover what
/// changed, without this crate needing an actual push/subscribe transport of its own. `db_query`'s
/// `LiveQuery` (see `🔖Query`, new this revision) is the push-shaped sibling of this same signal.
#[derive(Clone, Debug)]
pub struct CommitNotification {
    pub frontier: db_core::Frontier,
    pub operation_ids: Vec<protocol::OperationId>,
    pub touched: db_state::TouchedSet,
}
//#endregion 🔖Receipt

//#region 🔖State
/// @emoji 🏗️ A document's materialized state: a flat `db_state::PMap` from path to raw value
/// bytes, plus a per-path last-writer map for `submit`'s local, path-granular conflict detection
/// (see `🔖Conflict`'s doc on why this stays local rather than `db_conflict`-backed). `values` uses
/// `PMap` (not a mutable `HashMap`) specifically so `content_hash` — the `Frontier.chain_hash`
/// source — is a real content-addressed digest of the whole state, not an incidental byte count,
/// and so `PMap::iter` gives `snapshot_now`/`query` a cheap, complete enumeration.
struct DocumentState {
    values: db_state::PMap<String, Vec<u8>>,
    last_writer: db_state::PMap<String, protocol::OperationId>,
}

impl DocumentState {
    fn new() -> DocumentState {
        DocumentState { values: db_state::PMap::new(), last_writer: db_state::PMap::new() }
    }

    fn get(&self, path: &str) -> Option<Vec<u8>> {
        self.values.get(&path.to_string()).cloned()
    }

    fn content_hash(&self) -> pack::ContentHash {
        self.values.content_hash()
    }

    /// @emoji ✍️ Applies one envelope's flattened path-value entries, returning the new state, the
    /// `TouchedSet` it wrote, and any conflicts (a path whose last writer is neither `operation_id`
    /// itself nor a declared `dependencies` member).
    fn apply_entries(
        &self,
        operation_id: &protocol::OperationId,
        dependencies: &[protocol::OperationId],
        entries: &[(String, Option<serde_json::Value>)],
    ) -> Result<(DocumentState, db_state::TouchedSet, Vec<ConflictRecord>), DbError> {
        let mut values = self.values.clone();
        let mut last_writer = self.last_writer.clone();
        let mut touched = db_state::TouchedSet::new();
        let mut conflicts = Vec::new();
        for (path, value) in entries {
            if let Some(previous_writer) = self.last_writer.get(path) {
                if previous_writer != operation_id && !dependencies.contains(previous_writer) {
                    conflicts.push(ConflictRecord {
                        command_id: operation_id.clone(),
                        conflicting_with: previous_writer.clone(),
                        path: path.clone(),
                    });
                }
            }
            match value {
                Some(json) => {
                    let bytes = serde_json::to_vec(json).map_err(json_err)?;
                    values = values.insert(path.clone(), bytes);
                }
                None => values = values.remove(path),
            }
            touched.record(db_state::TouchedRegion::write(path.clone()));
            last_writer = last_writer.insert(path.clone(), operation_id.clone());
        }
        Ok((DocumentState { values, last_writer }, touched, conflicts))
    }
}
//#endregion 🔖State

//#region 🔖Hooks
/// @emoji 🛂 The authorization seam `DocumentEngine::submit` calls once per envelope, before
/// executing it — kept as its own narrow trait (rather than a direct `db_security` dependency) so a
/// real deployment supplies whatever backend it wants at `DocumentEngineConfig` construction time.
/// `db_engine`'s `SecurityAuthzHook` is the real `db_security::SecurityGate`-backed implementation.
pub trait AuthzHook: Send + Sync {
    fn authorize(&self, actor: &protocol::ActorId, envelope: &protocol::OperationEnvelope) -> Result<(), DbError>;
}

/// @emoji 🟢 The default `AuthzHook`: authorizes everything. Correct for a single-tenant/test
/// deployment with no authorization policy configured; a real multi-tenant deployment must supply
/// its own hook.
#[derive(Clone, Copy, Default, Debug)]
pub struct AllowAll;

impl AuthzHook for AllowAll {
    fn authorize(&self, _actor: &protocol::ActorId, _envelope: &protocol::OperationEnvelope) -> Result<(), DbError> {
        Ok(())
    }
}
//#endregion 🔖Hooks

//#region 🔖Engine
/// @emoji ⚙️ Construction-time configuration for one `DocumentEngine`. Field shape is FROZEN for
/// this wave (see module doc): `db_engine` constructs this as a 4-field struct literal with no
/// `..Default::default()` spread, so a new required field here would be a breaking change to a
/// sibling crate this session does not own.
pub struct DocumentEngineConfig {
    pub limits: db_core::DbLimits,
    /// @emoji 🛂 Deprecated-in-spirit extension seam, kept defined (see module doc): `submit` now
    /// authorizes through `security` instead. A caller with an existing `AuthzHook` impl can still
    /// call it manually; `DocumentEngine` itself no longer does.
    pub authz: Arc<dyn AuthzHook>,
    /// @emoji 🔐 The real authz/dedupe/DoS-budget gate `submit` calls once per envelope — see
    /// `db_security::SecurityGate::admit_command`'s doc. Keyed per-envelope by a `Principal`
    /// synthesized from that envelope's own `actor` (a permissive `"member"` role, `"default"`
    /// tenant) — `SubmitOptions` stays durability-only (see its doc) so this crate's dedupe/authz
    /// story does not require a caller to separately authenticate every submit call.
    pub security: db_security::SecurityGate,
    /// @emoji 🌿 The `vcs` seam (see `db_core::VersionGraph`'s doc) — `db_core::NullVersionGraph`
    /// (the default) answers every call `Unimplemented` rather than requiring an `Option` layer;
    /// only `db_engine` behind the `vcs` feature wires a real implementation in.
    pub version_graph: Arc<dyn db_core::VersionGraph>,
    pub emit: Arc<dyn db_core::Emit>,
    pub preview_ttl_ms: u64,
    /// @emoji 🧬 Projection factory: `submit`'s project step registers a fresh
    /// `db_projection::ProjectionEngine` from this on every call it needs one (see `🔖Engine`'s doc
    /// for why a factory rather than a stored, already-built engine — `db_projection::
    /// ProjectionEngine::new`'s borrowed-`IndexStorage` + owned-`Vec<Box<dyn ErasedProjection>>`
    /// shape does not compose with `DocumentEngine` owning its storage as `Arc<dyn DbStorage>`
    /// without becoming self-referential). Defaults to no projections registered.
    pub projections: Arc<dyn Fn() -> Vec<Box<dyn db_projection::ErasedProjection>> + Send + Sync>,
}

impl Default for DocumentEngineConfig {
    fn default() -> Self {
        let limits = db_core::DbLimits::default();
        let policy = db_security::RoleBasedPolicy::new().with_grant(db_security::Grant::allow("member", &["**"], &[db_security::Action::Read, db_security::Action::Write]));
        DocumentEngineConfig {
            preview_ttl_ms: limits.max_preview_ttl_ms,
            limits,
            authz: Arc::new(AllowAll),
            security: db_security::SecurityGate::new(
                policy,
                db_security::ReplayGuard::new(60_000, 4_096),
                db_security::BudgetRegistry::new(100_000, 100_000),
                Arc::new(db_core::NullEmit),
            ),
            version_graph: Arc::new(db_core::NullVersionGraph),
            emit: Arc::new(db_core::NullEmit),
            projections: Arc::new(Vec::new),
        }
    }
}

/// @emoji 🎭 The document authority's real, synchronous pipeline: one open document's WAL,
/// materialized state, causal dependency bookkeeping, previews, and outbox — everything
/// `DocumentAuthority` (the `db_actor`-mailbox wrapper below) drives from its own dedicated
/// thread. Deliberately NOT `Send` (see `DocumentAuthority`'s doc for why: `DocumentState` embeds
/// `db_state::PMap`, which is `Rc`-based) — usable directly, single-threaded, wherever a mailbox
/// isn't needed (e.g. this crate's own tests).
pub struct DocumentEngine {
    document: db_core::DocumentId,
    protocol_document: protocol::DocumentId,
    storage: Arc<dyn db_storage::DbStorage>,
    wal: db_wal::DocumentWal,
    state: DocumentState,
    vcs_head: Option<String>,
    applied: HashMap<String, protocol::OperationEnvelope>,
    applied_receipts: HashMap<String, CommandReceipt>,
    actor_seq: HashMap<String, u64>,
    frontier: db_core::Frontier,
    outbox: Vec<OutboxEntry>,
    commit_log: Vec<CommitNotification>,
    previews: db_preview::PreviewStore,
    recent_touches: VecDeque<db_conflict::CommandTouch>,
    live_queries: HashMap<u64, db_query::LiveQuery>,
    next_live_query_id: u64,
    config: DocumentEngineConfig,
}

const MAX_RECENT_TOUCHES: usize = 256;

impl DocumentEngine {
    /// @emoji 🌱 Creates a brand-new document: a genesis WAL (segment 0) and an empty state.
    /// Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    pub fn create(
        document: protocol::DocumentId,
        storage: Arc<dyn db_storage::DbStorage>,
        config: DocumentEngineConfig,
        now_ms: u64,
    ) -> Result<DocumentEngine, DbError> {
        let core_id = to_core_document_id(&document);
        let wal = db_wal::DocumentWal::create(storage.wal(), core_id.clone(), db_wal::GroupCommitPolicy::default(), now_ms)?;
        Ok(DocumentEngine::assemble(document, core_id, storage, wal, None, config))
    }

    /// @emoji 🚑 Materializes a document as initial ⊕ latest `db_snapshot` generation ⊕ WAL suffix
    /// (this revision adds the snapshot half — the prior revision was WAL-suffix-only): loads the
    /// latest snapshot's `DocumentState` (if any) as the starting point, opens/recovers the WAL
    /// (per `db_wal::DocumentWal::open`), then replays only the `WAL_COMMAND` records committed
    /// AFTER the snapshot's own `head_seq` (a full-from-genesis replay when there is no snapshot
    /// yet).
    pub fn open(
        document: protocol::DocumentId,
        storage: &Arc<dyn db_storage::DbStorage>,
        config: DocumentEngineConfig,
        now_ms: u64,
    ) -> Result<(DocumentEngine, MaterializeReport), DbError> {
        let core_id = to_core_document_id(&document);
        let mut report = MaterializeReport::default();

        let mut state = DocumentState::new();
        let mut applied_head_seq = 0u64;
        let mut vcs_head = None;
        let snapshot_manager = db_snapshot::SnapshotManager::new(storage.snapshot());
        if let Some((generation, descriptor)) = snapshot_manager.load_latest(&core_id)? {
            report.from_snapshot = true;
            report.snapshot_generation = Some(generation);
            let combined = snapshot_manager.materialize_chain(&core_id, generation)?;
            let handle = db_snapshot::open_latest(&combined)?;
            for hash in &descriptor.roots {
                let page_bytes = db_snapshot::read_page(&combined, &handle, *hash)?;
                for (path, value) in decode_state_page(&page_bytes)? {
                    state.values = match value {
                        Some(bytes) => state.values.insert(path, bytes),
                        None => state.values.remove(&path),
                    };
                }
            }
            applied_head_seq = descriptor.head_seq;
            vcs_head = descriptor.vcs_head;
        }

        let (wal, wal_recovery) = db_wal::DocumentWal::open(storage.wal(), core_id.clone(), db_wal::GroupCommitPolicy::default(), now_ms)?;
        report.torn_tail_bytes = wal_recovery.torn_tail_bytes;
        let mut engine = DocumentEngine::assemble(document, core_id.clone(), storage.clone(), wal, vcs_head, config);
        engine.state = state;
        engine.frontier.head_seq = applied_head_seq;

        let records = db_wal::replay_document(storage.wal(), &core_id)?;
        let mut batch_ids: HashSet<String> = HashSet::new();
        let mut seen: u64 = 0;
        for record in records {
            match record {
                db_wal::WalRecord::TxBegin { .. } => batch_ids.clear(),
                db_wal::WalRecord::Command(bytes) => {
                    let envelope: protocol::OperationEnvelope =
                        serde_json::from_slice(&bytes).map_err(|err| DbError::Corrupt(format!("wal command record is not a valid operation envelope: {err}")))?;
                    seen += 1;
                    batch_ids.insert(envelope.operation_id.0.clone());
                    if seen <= applied_head_seq {
                        // Already folded into the loaded snapshot — replay the causal bookkeeping
                        // (`applied`) but not the state mutation itself.
                        engine.applied.insert(envelope.operation_id.0.clone(), envelope);
                        continue;
                    }
                    let (touched, _conflicts, _) = engine.apply_one(&envelope, &batch_ids)?;
                    let touch = command_touch(&envelope, &touched);
                    if engine.recent_touches.len() >= MAX_RECENT_TOUCHES {
                        engine.recent_touches.pop_front();
                    }
                    engine.recent_touches.push_back(touch);
                    report.commands_replayed += 1;
                }
                // 🩹 The authoritative post-commit frontier was written verbatim at commit time
                // (see `submit`'s final record) — replaying it directly avoids recomputing
                // `head_seq`/`commit_seq` from scratch and guarantees the reopened engine agrees
                // exactly with what was durable, even if this crate's own bookkeeping ever changes.
                db_wal::WalRecord::Frontier(frontier) => engine.frontier = frontier,
                _ => {}
            }
        }
        Ok((engine, report))
    }

    fn assemble(
        protocol_document: protocol::DocumentId,
        core_id: db_core::DocumentId,
        storage: Arc<dyn db_storage::DbStorage>,
        wal: db_wal::DocumentWal,
        vcs_head: Option<String>,
        config: DocumentEngineConfig,
    ) -> DocumentEngine {
        let preview_budgets = db_preview::PreviewBudgets { default_ttl_ms: config.preview_ttl_ms, max_ttl_ms: config.preview_ttl_ms, ..db_preview::PreviewBudgets::default() };
        DocumentEngine {
            document: core_id.clone(),
            protocol_document,
            storage,
            wal,
            state: DocumentState::new(),
            vcs_head,
            applied: HashMap::new(),
            applied_receipts: HashMap::new(),
            actor_seq: HashMap::new(),
            frontier: db_core::Frontier::genesis(core_id.clone()),
            outbox: Vec::new(),
            commit_log: Vec::new(),
            previews: db_preview::PreviewStore::new(core_id, preview_budgets),
            recent_touches: VecDeque::new(),
            live_queries: HashMap::new(),
            next_live_query_id: 0,
            config,
        }
    }

    /// @emoji ✅🚫 Dependency + dedupe + execute for one envelope, shared by `submit` (before the
    /// WAL write) and `open`'s replay (after it). `batch_ids` is the set of operation ids already
    /// seen earlier in the SAME transaction (a multi-envelope batch may reference its own earlier
    /// members as dependencies). Returns `(touched, conflicts, applied_now)`; `applied_now` is
    /// `false` (with empty touched/conflicts) if `envelope.operation_id` was already applied in an
    /// earlier commit — the per-envelope half of this crate's dedupe law.
    fn apply_one(&mut self, envelope: &protocol::OperationEnvelope, batch_ids: &HashSet<String>) -> Result<(db_state::TouchedSet, Vec<ConflictRecord>, bool), DbError> {
        if self.applied.contains_key(&envelope.operation_id.0) {
            return Ok((db_state::TouchedSet::new(), Vec::new(), false));
        }
        for dependency in &envelope.dependencies {
            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) {
                return Err(DbError::InvalidArgument(format!(
                    "operation {} depends on unseen operation {}",
                    envelope.operation_id.0, dependency.0
                )));
            }
        }
        let entries = diff_entries(&envelope.diff)?;
        db_core::check_len(entries.len() as u64, self.config.limits.max_batch_commands as u64, "db_document::diff_entries")?;
        let (new_state, touched, conflicts) = self.state.apply_entries(&envelope.operation_id, &envelope.dependencies, &entries)?;
        self.state = new_state;
        self.applied.insert(envelope.operation_id.0.clone(), envelope.clone());
        let actor_seq = self.actor_seq.entry(envelope.actor.0.clone()).or_insert(0);
        *actor_seq += 1;
        Ok((touched, conflicts, true))
    }

    /// @emoji 🚦 The full command pipeline: admit → dedupe → base-resolve/deps → authz → validate →
    /// conflict → execute → WAL append → durability → publish → project → vcs →
    /// preview-reconcile → live-query notify → receipt.
    // 🔒 `batch` is taken by value deliberately, not just because the current body happens not to
    // move it: submitting a batch is the one place in this API where the caller's copy is
    // logically spent (it becomes part of durable history), matching `DocumentMessage::Submit`'s
    // owned payload on the actor-mailbox side one level up. Keeping ownership here leaves room for
    // a future zero-copy WAL append that moves `batch.envelopes` directly into the write path.
    #[allow(clippy::needless_pass_by_value)]
    pub fn submit(&mut self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        // admit
        db_core::check_len(batch.envelopes.len() as u64, self.config.limits.max_batch_commands as u64, "db_document::batch_commands")?;
        for envelope in &batch.envelopes {
            if envelope.document_id != self.protocol_document {
                return Err(DbError::InvalidArgument(format!(
                    "envelope targets document {:?} but this actor owns {:?}",
                    envelope.document_id, self.protocol_document
                )));
            }
        }
        let command_id = batch.envelopes.last().expect("CommandBatch::new guarantees at least one envelope").operation_id.clone();

        // dedupe (whole-batch, keyed by the batch's designated command_id)
        if let Some(cached) = self.applied_receipts.get(&command_id.0) {
            return Ok(cached.clone());
        }

        let mut batch_ids: HashSet<String> = HashSet::new();
        let mut records: Vec<db_wal::WalRecord> = Vec::new();
        let mut touched_all = db_state::TouchedSet::new();
        let mut conflicts_all: Vec<ConflictRecord> = Vec::new();
        let mut newly_applied: Vec<(protocol::OperationEnvelope, db_state::TouchedSet)> = Vec::new();

        for envelope in &batch.envelopes {
            // authz: the `AuthzHook` seam (defaults to `AllowAll`; `db_engine`'s `SecurityAuthzHook`
            // wraps a real `db_security::SecurityGate` here).
            self.config.authz.authorize(&envelope.actor, envelope)?;

            // authz (defense in depth): the newer, real `db_security::SecurityGate` gate, keyed by a
            // permissive principal synthesized from the envelope's own actor (see
            // `DocumentEngineConfig::security`'s doc) — additive, does not replace `authz` above.
            let principal = db_security::Principal::new(envelope.actor.clone(), db_security::TenantId::from("default"), vec!["member".to_string()]);
            self.config.security.admit_command(
                &principal,
                &db_security::TenantId::from("default"),
                &envelope.document_id,
                &envelope.diff.schema,
                &envelope.actor,
                &envelope.operation_id,
                now_ms,
            )?;

            let envelope_bytes = serde_json::to_vec(envelope).map_err(json_err)?;
            db_core::check_len(envelope_bytes.len() as u64, self.config.limits.max_command_bytes, "db_document::envelope_bytes")?;

            // base-resolve/deps + validate + conflict + execute
            let (touched, conflicts, applied_now) = self.apply_one(envelope, &batch_ids)?;
            batch_ids.insert(envelope.operation_id.0.clone());
            if !applied_now {
                continue;
            }

            // Bookkeeping for `preview_conflicts`'s real, additive `db_conflict::ConflictDetector`
            // integration (see its own doc) — `submit`'s own returned `ConflictRecord`s stay this
            // crate's original path-granular last-writer detection above (see `🔖Conflict`'s doc).
            let touch = command_touch(envelope, &touched);
            if self.recent_touches.len() >= MAX_RECENT_TOUCHES {
                self.recent_touches.pop_front();
            }
            self.recent_touches.push_back(touch);

            for region in &touched.regions {
                touched_all.record(region.clone());
            }
            conflicts_all.extend(conflicts);
            newly_applied.push((envelope.clone(), touched));

            let diff_bytes = serde_json::to_vec(&envelope.diff).map_err(json_err)?;
            let inverse_bytes = serde_json::to_vec(&envelope.inverse).map_err(json_err)?;
            records.push(db_wal::WalRecord::Command(envelope_bytes.clone()));
            records.push(db_wal::WalRecord::Diff(diff_bytes));
            records.push(db_wal::WalRecord::Inverse(inverse_bytes));
            records.push(db_wal::WalRecord::Outbox(envelope_bytes.clone()));
            self.outbox.push(OutboxEntry { operation_id: envelope.operation_id.clone(), bytes: envelope_bytes });
        }

        if newly_applied.is_empty() {
            // Every envelope in this (re-)submitted batch was already durable individually — a
            // full no-op commit, per-envelope half of the dedupe law (see `apply_one`'s doc).
            let receipt = CommandReceipt {
                command_id,
                frontier: self.frontier.clone(),
                durability: options.durability,
                conflicts: Vec::new(),
                state_hash: Some(self.state.content_hash()),
            };
            self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone());
            return Ok(receipt);
        }

        // publish: compute + WAL-append the new frontier in the same transaction as its commands
        let new_frontier = db_core::Frontier {
            document: self.document.clone(),
            head_seq: self.frontier.head_seq + newly_applied.len() as u64,
            commit_seq: self.frontier.commit_seq + 1,
            chain_hash: self.state.content_hash().0,
            epoch: self.frontier.epoch,
        };
        records.push(db_wal::WalRecord::Frontier(new_frontier.clone()));

        // WAL append + durability (DocumentWal::submit wraps `records` in its own TxBegin/TxCommit)
        self.wal.submit(self.storage.wal(), &records, options.durability, now_ms)?;
        self.frontier = new_frontier.clone();

        // publish: durable indices
        let command_index = db_index::CommandIndex::new(self.storage.index(), self.document.clone());
        let inverse_index = db_index::InverseIndex::new(self.storage.index(), self.document.clone());
        let actor_seq_index = db_index::ActorSeqIndex::new(self.storage.index(), self.document.clone());
        db_index::FrontierIndex::new(self.storage.index(), self.document.clone()).record(&new_frontier)?;
        let base_seq = self.frontier.head_seq - newly_applied.len() as u64;
        for (offset, (envelope, _)) in newly_applied.iter().enumerate() {
            let seq = base_seq + offset as u64 + 1;
            let location = db_index::RecordLocation { segment: self.wal.active_segment_index(), offset: seq, len: 1 };
            command_index.record(seq, location)?;
            inverse_index.record(seq, location)?;
            let core_actor = to_core_actor_id(&envelope.actor);
            let actor_seq = *self.actor_seq.get(&envelope.actor.0).unwrap_or(&0);
            actor_seq_index.record(&core_actor, actor_seq, seq)?;
        }

        // project: run every registered projection over each newly-applied envelope
        let projection_classes = (self.config.projections)();
        if !projection_classes.is_empty() {
            let engine = db_projection::ProjectionEngine::new(self.storage.index(), self.document.clone(), projection_classes)?;
            for (offset, (envelope, touched)) in newly_applied.iter().enumerate() {
                engine.apply_envelope(base_seq + offset as u64 + 1, envelope, touched)?;
            }
        }

        // preview-reconcile
        self.previews.reconcile_with(&db_preview::LandedCommand { frontier: new_frontier.clone(), touched: touched_all.clone() }, &db_preview::DbConflictOracle::default());
        self.commit_log.push(CommitNotification {
            frontier: new_frontier.clone(),
            operation_ids: newly_applied.iter().map(|(envelope, _)| envelope.operation_id.clone()).collect(),
            touched: touched_all,
        });

        // vcs (best-effort: this crate never blocks a commit on the vcs seam's outcome; a disabled
        // vcs feature supplies `db_core::NullVersionGraph`, whose `Unimplemented` is tolerated here)
        for (envelope, _) in &newly_applied {
            match self.config.version_graph.record_change(
                &self.document,
                db_core::ChangeRecord {
                    parent: None,
                    content_hash: self.state.content_hash(),
                    author: to_core_actor_id(&envelope.actor),
                    message: format!("operation {}", envelope.operation_id.0),
                    timestamp_ms: now_ms,
                },
            ) {
                Ok(_) | Err(DbError::Unimplemented(_)) => {}
                Err(other) => return Err(other),
            }
        }

        // live-query notify
        let _ = self.refresh_live_queries();

        // receipt
        let receipt =
            CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash()) };
        self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone());
        Ok(receipt)
    }

    /// @emoji ↩️ The inverse-undo pipeline: looks up `target`'s already-applied envelope, flips its
    /// `inverse` into a compensating envelope's `diff` (and vice versa, so the compensating
    /// envelope's OWN inverse can re-undo the undo), and submits it as a fresh, ordinary command
    /// depending on `target` — undo is just another commit, not a WAL rewrite. This is the crate's
    /// "inverse undo, using protocol's inverse-operation machinery".
    pub fn undo(&mut self, target: &protocol::OperationId, undo_operation_id: protocol::OperationId, actor: protocol::ActorId, now_ms: u64) -> Result<CommandReceipt, DbError> {
        let original = self.applied.get(&target.0).cloned().ok_or_else(|| DbError::NotFound(format!("operation {} not found for undo", target.0)))?;
        let undo_diff_entries = inverse_entries(&original.inverse)?;
        let redo_inverse_entries = diff_entries(&original.diff)?;
        let compensating = protocol::OperationEnvelope {
            operation_id: undo_operation_id,
            document_id: self.protocol_document.clone(),
            actor,
            dependencies: vec![target.clone()],
            diff: protocol::DocumentDiff { schema: original.inverse.schema.clone(), payload: entries_to_value(&undo_diff_entries) },
            inverse: protocol::InverseOperation { schema: original.diff.schema, inverse_diff: entries_to_value(&redo_inverse_entries) },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
        };
        self.submit(CommandBatch::new(vec![compensating])?, SubmitOptions::default(), now_ms)
    }

    pub fn get(&self, path: &str) -> Option<Vec<u8>> {
        self.state.get(path)
    }

    pub fn frontier(&self) -> db_core::Frontier {
        self.frontier.clone()
    }

    pub fn commit_log(&self) -> &[CommitNotification] {
        &self.commit_log
    }

    /// @emoji 📤 Hands out (and clears) every effect queued since the last drain.
    pub fn drain_outbox(&mut self) -> Vec<OutboxEntry> {
        std::mem::take(&mut self.outbox)
    }

    //#region 🔖Snapshot
    /// @emoji 📸 Publishes a new `db_snapshot` generation of the whole current `DocumentState` —
    /// new this revision; the counterpart `open` reads back to accelerate materialization.
    pub fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        let entries: Vec<(String, Option<Vec<u8>>)> = self.state.values.iter().map(|(path, bytes)| (path.clone(), Some(bytes.clone()))).collect();
        let page = db_state::Page::new(encode_state_page(&entries));
        let snapshot_manager = db_snapshot::SnapshotManager::new(self.storage.snapshot());
        let origin =
            if snapshot_manager.load_latest(&self.document)?.is_some() { db_snapshot::SnapshotOrigin::Incremental } else { db_snapshot::SnapshotOrigin::FullBaseline };
        let body = db_snapshot::SnapshotBody {
            head_seq: self.frontier.head_seq,
            commit_seq: self.frontier.commit_seq,
            epoch: self.frontier.epoch,
            chain_hash: self.frontier.chain_hash,
            protocol_version: 1,
            vcs_head: self.vcs_head.clone(),
            base_pack_hash: None,
            roots: vec![page.hash],
            created_at_ms: now_ms,
        };
        snapshot_manager.publish(&self.document, origin, &[page], body)
    }
    //#endregion 🔖Snapshot

    //#region 🔖Query
    /// @emoji 🔎 One-shot query over the document's current materialized state, resolved under
    /// `consistency` via `db_index`'s `CommitIndex`/`FrontierIndex`. `StateQuerySource` always reads
    /// the CURRENT canonical state regardless of what `consistency` resolved to — a true
    /// point-in-time replay is `db_engine`'s documented deferred extension (see its own module doc).
    // 🔒 Mirrors `submit`'s ownership rationale above: `DocumentMessage::RunQuery` already owns
    // `query`/`consistency` one level up on the actor-mailbox side, and query descriptors are
    // small, cheap-to-move value types — taking them by reference here would only push the
    // ownership question onto every caller for no benefit.
    #[allow(clippy::needless_pass_by_value)]
    pub fn query(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        let resolver = db_query::IndexConsistencyResolver {
            commits: db_index::CommitIndex::new(self.storage.index(), self.document.clone()),
            frontiers: db_index::FrontierIndex::new(self.storage.index(), self.document.clone()),
        };
        // A fresh document has no recorded frontier yet; canonical reads still succeed via the
        // in-memory frontier, so only consult the resolver for modes that truly need the index.
        if !matches!(consistency, db_query::Consistency::Canonical) {
            db_query::resolve_consistency(&consistency, &resolver)?;
        }
        let source = StateQuerySource(&self.state.values);
        db_query::execute(&query, &source, None, &db_query::QueryLimits::default())
    }

    /// @emoji 📡 Registers a live query, returning its subscription id — new this revision.
    pub fn subscribe(&mut self, spec: db_query::LiveQuerySpec) -> u64 {
        let id = self.next_live_query_id;
        self.next_live_query_id += 1;
        self.live_queries.insert(id, db_query::LiveQuery::new(spec));
        id
    }

    pub fn unsubscribe(&mut self, id: u64) {
        self.live_queries.remove(&id);
    }

    /// @emoji 📡 Live-query notify: re-evaluates every registered live query and returns what
    /// changed. Called automatically at the end of `submit`; also callable directly.
    pub fn refresh_live_queries(&mut self) -> Vec<(u64, db_query::QueryDiff)> {
        let source = StateQuerySource(&self.state.values);
        let limits = db_query::QueryLimits::default();
        let mut diffs = Vec::new();
        for (id, live_query) in self.live_queries.iter_mut() {
            if let Ok(diff) = live_query.refresh(&source, None, &limits) {
                if !diff.added.is_empty() || !diff.removed.is_empty() || !diff.updated.is_empty() {
                    diffs.push((*id, diff));
                }
            }
        }
        diffs
    }
    //#endregion 🔖Query

    //#region 🔖Advisory
    /// @emoji 🔮 Advisory-only, real `db_conflict::ConflictDetector` integration: runs `batch`'s
    /// envelopes' touched regions against recent commit history WITHOUT executing anything, using
    /// the family's real bloom-filter/kind-matrix machinery — a caller (e.g. a UI) can call this
    /// before `submit` to preview likely conflicts. `submit`'s own returned `ConflictRecord`s stay
    /// this crate's original path-granular last-writer detection (see `🔖Conflict`'s doc on why
    /// `db_conflict`'s per-command-pair shape can't replace it without breaking `db_engine`'s frozen
    /// `CommandReceipt.conflicts` element type).
    pub fn preview_conflicts(&self, batch: &CommandBatch) -> Result<Vec<db_conflict::ConflictRecord>, DbError> {
        let mut probe: Vec<db_conflict::CommandTouch> = self.recent_touches.iter().cloned().collect();
        for envelope in &batch.envelopes {
            let entries = diff_entries(&envelope.diff)?;
            let touched = entries_touched(&entries);
            probe.push(command_touch(envelope, &touched));
        }
        Ok(db_conflict::ConflictDetector::new().detect(&probe))
    }
    //#endregion 🔖Advisory

    //#region 🔖Preview
    /// @emoji 🌫️ Publishes a new preview overlaying the CURRENT committed state — never durable
    /// (never touches the WAL), per the contract's preview law. Backed by a real
    /// `db_preview::PreviewStore` this revision (previously a local, minimal stand-in).
    pub fn publish_preview(&mut self, entries: &[(String, Option<serde_json::Value>)], now_ms: u64) -> Result<db_preview::PreviewId, DbError> {
        let touched = entries_touched(entries);
        let envelope = protocol::OperationEnvelope {
            operation_id: protocol::OperationId(format!("preview-{}", entries.len())),
            document_id: self.protocol_document.clone(),
            actor: protocol::ActorId("preview".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff { schema: "db_document.preview".to_string(), payload: entries_to_value(entries) },
            inverse: protocol::InverseOperation { schema: "db_document.preview".to_string(), inverse_diff: serde_json::Value::Object(serde_json::Map::new()) },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
        };
        self.previews.publish(db_preview::PublishPreviewRequest {
            document: self.document.clone(),
            actor: db_core::ActorId("preview".to_string()),
            key: format!("preview-{now_ms}"),
            base: self.frontier.clone(),
            envelope,
            touched,
            ttl_ms: None,
            now_ms,
        })
    }

    /// @emoji 🌫️ The value a preview would show at `path`: the preview's own diff if it touches
    /// `path`, else falling through to the committed state.
    pub fn preview_get(&self, id: &db_preview::PreviewId, path: &str) -> Result<Option<Vec<u8>>, DbError> {
        let preview = self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?;
        for (entry_path, value) in diff_entries(&preview.envelope.diff)? {
            if entry_path == path {
                return match value {
                    Some(json) => Ok(Some(serde_json::to_vec(&json).map_err(json_err)?)),
                    None => Ok(None),
                };
            }
        }
        Ok(self.state.get(path))
    }

    pub fn preview_status(&self, id: &db_preview::PreviewId) -> Result<db_preview::PreviewState, DbError> {
        Ok(self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?.state)
    }

    pub fn withdraw_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.withdraw(id)
    }

    pub fn reject_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.reject(id)
    }

    pub fn expire_previews(&mut self, now_ms: u64) -> Vec<db_preview::PreviewId> {
        self.previews.sweep_expired(now_ms)
    }
    //#endregion 🔖Preview
}
//#endregion 🔖Engine

//#region 🔖QuerySource
/// @emoji 🚰 The `db_query::QuerySource` this crate supplies over its own `DocumentState`: one row
/// per stored path, `{"path": <path>, "value": <text-or-bytes>}`.
struct StateQuerySource<'a>(&'a db_state::PMap<String, Vec<u8>>);

fn path_row_value(path: &str, bytes: &[u8]) -> db_query::Value {
    let mut map = std::collections::BTreeMap::new();
    map.insert("path".to_string(), db_query::Value::Text(path.to_string()));
    match std::str::from_utf8(bytes) {
        Ok(text) => {
            map.insert("value".to_string(), db_query::Value::Text(text.to_string()));
        }
        Err(_) => {
            map.insert("value".to_string(), db_query::Value::Bytes(bytes.to_vec()));
        }
    }
    db_query::Value::Map(map)
}

impl<'a> db_query::QuerySource for StateQuerySource<'a> {
    fn scan(&self) -> Box<dyn Iterator<Item = (db_query::RowId, db_query::Value)> + '_> {
        Box::new(self.0.iter().enumerate().map(|(index, (path, bytes))| (db_query::RowId(index as u64), path_row_value(path, bytes))))
    }
}
//#endregion 🔖QuerySource

//#region 🔖Snapshot
/// @emoji 📸 This crate's own snapshot page convention (opaque to `db_snapshot`/`db_storage`): the
/// whole `DocumentState` as of the snapshot's frontier, one entry per stored path.
fn encode_state_page(entries: &[(String, Option<Vec<u8>>)]) -> Vec<u8> {
    let mut writer = pack::ByteWriter::new();
    writer.write_varint_u64(entries.len() as u64);
    for (path, value) in entries {
        writer.write_varint_u64(path.len() as u64);
        writer.write_bytes(path.as_bytes());
        match value {
            Some(bytes) => {
                writer.write_u8(1);
                writer.write_varint_u64(bytes.len() as u64);
                writer.write_bytes(bytes);
            }
            None => writer.write_u8(0),
        }
    }
    writer.into_bytes()
}

const MAX_STATE_PAGE_ENTRIES: u64 = 10_000_000;
const MAX_STATE_PAGE_PATH_BYTES: u64 = 4_096;
const MAX_STATE_PAGE_VALUE_BYTES: u64 = 256 * 1024 * 1024;

/// @emoji 🗺️ One decoded state page: `(path, value_bytes)` pairs, `None` marking a tombstoned path.
type StatePageEntries = Vec<(String, Option<Vec<u8>>)>;

fn decode_state_page(bytes: &[u8]) -> Result<StatePageEntries, DbError> {
    let mut reader = pack::ByteReader::new(bytes);
    let count = reader.read_varint_u64()?;
    db_core::check_len(count, MAX_STATE_PAGE_ENTRIES, "db_document::snapshot_page_entries")?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let path_len = reader.read_varint_u64()?;
        db_core::check_len(path_len, MAX_STATE_PAGE_PATH_BYTES, "db_document::snapshot_page_path")?;
        let path_bytes = reader.read_bytes(path_len as usize)?.to_vec();
        let path = String::from_utf8(path_bytes).map_err(|_| DbError::Corrupt("snapshot page path is not valid utf-8".to_string()))?;
        let value = if reader.read_u8()? == 1 {
            let len = reader.read_varint_u64()?;
            db_core::check_len(len, MAX_STATE_PAGE_VALUE_BYTES, "db_document::snapshot_page_value")?;
            Some(reader.read_bytes(len as usize)?.to_vec())
        } else {
            None
        };
        entries.push((path, value));
    }
    Ok(entries)
}

/// @emoji 📋 What `DocumentEngine::open` did to materialize state — "initial ⊕ snapshot ⊕ WAL
/// suffix" made observable. New this revision (was `db_wal::WalRecoveryReport` alone before).
#[derive(Clone, Debug, Default)]
pub struct MaterializeReport {
    pub from_snapshot: bool,
    pub snapshot_generation: Option<u64>,
    pub torn_tail_bytes: u64,
    pub commands_replayed: u64,
}
//#endregion 🔖Snapshot

//#region 🔖Actor
/// @emoji 📨 A message crossing `DocumentAuthority`'s mailbox — deliberately `Send` (unlike
/// `DocumentEngine` itself, see `DocumentAuthority`'s doc).
pub enum DocumentMessage {
    Submit { batch: CommandBatch, options: SubmitOptions, now_ms: u64, reply: db_actor::ReplySender<Result<CommandReceipt, DbError>> },
    Query { path: String, reply: db_actor::ReplySender<Option<Vec<u8>>> },
    Frontier { reply: db_actor::ReplySender<db_core::Frontier> },
    /// @emoji 🔎 Additive this revision — `db_engine`'s current `DocumentHandle::query` goes
    /// through `Query { path, .. }` above and never constructs this variant, so adding it is safe.
    RunQuery { query: db_query::Query, consistency: db_query::Consistency, reply: db_actor::ReplySender<Result<db_query::QueryResult, DbError>> },
    SnapshotNow { now_ms: u64, reply: db_actor::ReplySender<Result<u64, DbError>> },
    DrainOutbox { reply: db_actor::ReplySender<Vec<OutboxEntry>> },
}

/// @emoji 🎭 A live handle to one document's authority actor, running the `db_actor` mailbox on a
/// dedicated OS thread.
///
/// 🎯 Design choice (why this does NOT implement `db_actor::Actor`): `db_actor::Actor: Send +
/// 'static`, but `DocumentEngine` structurally cannot be `Send` — `DocumentState` embeds
/// `db_state::PMap`, which is `Rc`-based (`db_state`'s own module doc: cheap `O(1)` clone via
/// reference-count bump, deliberately not thread-safe). Rather than fight that (e.g. by
/// re-wrapping every persistent structure in `Arc`, undermining `db_state`'s whole design), this
/// type embraces it: `spawn` takes a `Send` CONSTRUCTION CLOSURE (not a pre-built engine), builds
/// the `DocumentEngine` on its own dedicated thread, and never moves it again — only
/// `DocumentMessage`s (themselves `Send`) ever cross the mailbox boundary. That is exactly the
/// isolation property the actor pattern exists to provide; it just isn't expressible through
/// `db_actor::Actor`'s particular trait shape for a `!Send` actor body, so this crate builds
/// directly on `db_actor`'s lower-level mailbox primitives (`mailbox`/`Address`/`Receiver`/
/// `oneshot`/`block_on`) instead.
pub struct DocumentAuthority {
    address: db_actor::Address<DocumentMessage>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl DocumentAuthority {
    /// @emoji 🚀 Spawns the actor thread, builds the engine there via `build`, and blocks until
    /// that construction succeeds or fails — a caller never holds a `DocumentAuthority` whose
    /// engine failed to open.
    pub fn spawn(build: impl FnOnce() -> Result<DocumentEngine, DbError> + Send + 'static, capacities: db_core::MailboxCapacities) -> Result<DocumentAuthority, DbError> {
        let (address, receiver) = db_actor::mailbox::<DocumentMessage>(capacities);
        let (ready_tx, ready_rx) = db_actor::oneshot::<Result<(), DbError>>();
        let handle = std::thread::Builder::new()
            .name("db-document-actor".to_string())
            .spawn(move || {
                let mut engine = match build() {
                    Ok(engine) => engine,
                    Err(err) => {
                        ready_tx.send(Err(err));
                        return;
                    }
                };
                ready_tx.send(Ok(()));
                while let Some(envelope) = receiver.recv_blocking() {
                    match envelope.payload {
                        DocumentMessage::Submit { batch, options, now_ms, reply } => reply.send(engine.submit(batch, options, now_ms)),
                        DocumentMessage::Query { path, reply } => reply.send(engine.get(&path)),
                        DocumentMessage::Frontier { reply } => reply.send(engine.frontier()),
                        DocumentMessage::RunQuery { query, consistency, reply } => reply.send(engine.query(query, consistency)),
                        DocumentMessage::SnapshotNow { now_ms, reply } => reply.send(engine.snapshot_now(now_ms)),
                        DocumentMessage::DrainOutbox { reply } => reply.send(engine.drain_outbox()),
                    }
                }
            })
            .expect("db_document: failed to spawn document actor thread");

        match db_actor::block_on(ready_rx) {
            Ok(Ok(())) => Ok(DocumentAuthority { address, handle: Some(handle) }),
            Ok(Err(err)) => Err(err),
            Err(_) => Err(DbError::Closed),
        }
    }

    pub fn submit_blocking(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        self.address.ask_blocking(db_core::Priority::Command, |reply| DocumentMessage::Submit { batch, options, now_ms, reply })?
    }

    pub fn query_blocking(&self, path: &str) -> Result<Option<Vec<u8>>, DbError> {
        let path = path.to_string();
        self.address.ask_blocking(db_core::Priority::Query, |reply| DocumentMessage::Query { path, reply })
    }

    pub fn frontier_blocking(&self) -> Result<db_core::Frontier, DbError> {
        self.address.ask_blocking(db_core::Priority::Query, |reply| DocumentMessage::Frontier { reply })
    }

    pub fn run_query_blocking(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        self.address.ask_blocking(db_core::Priority::Query, |reply| DocumentMessage::RunQuery { query, consistency, reply })?
    }

    pub fn snapshot_now_blocking(&self, now_ms: u64) -> Result<u64, DbError> {
        self.address.ask_blocking(db_core::Priority::Command, |reply| DocumentMessage::SnapshotNow { now_ms, reply })?
    }

    pub fn drain_outbox_blocking(&self) -> Result<Vec<OutboxEntry>, DbError> {
        self.address.ask_blocking(db_core::Priority::Query, |reply| DocumentMessage::DrainOutbox { reply })
    }

    /// @emoji 🚪 Closes the mailbox and joins the actor thread — graceful shutdown.
    pub fn shutdown(mut self) {
        self.address.close();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
//#endregion 🔖Actor

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc as StdArc;

    fn storage() -> StdArc<dyn db_storage::DbStorage> {
        StdArc::new(db_storage::MemoryStorage::new())
    }

    fn document_id() -> protocol::DocumentId {
        protocol::DocumentId("doc-1".to_string())
    }

    fn envelope(id: &str, deps: &[&str], actor: &str, entries: &[(&str, serde_json::Value)]) -> protocol::OperationEnvelope {
        let mut payload = serde_json::Map::new();
        for (path, value) in entries {
            payload.insert((*path).to_string(), value.clone());
        }
        protocol::OperationEnvelope {
            operation_id: protocol::OperationId(id.to_string()),
            document_id: document_id(),
            actor: protocol::ActorId(actor.to_string()),
            dependencies: deps.iter().map(|dep| protocol::OperationId((*dep).to_string())).collect(),
            diff: protocol::DocumentDiff { schema: "generic".to_string(), payload: serde_json::Value::Object(payload) },
            inverse: protocol::InverseOperation { schema: "generic".to_string(), inverse_diff: serde_json::Value::Object(serde_json::Map::new()) },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }

    //#region 🔖Command
    #[test]
    fn command_batch_rejects_empty_and_mixed_documents() {
        assert!(CommandBatch::new(Vec::new()).is_err());
        let mut mismatched = envelope("op-2", &[], "alice", &[("x", serde_json::json!(1))]);
        mismatched.document_id = protocol::DocumentId("other-doc".to_string());
        let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]), mismatched]);
        assert!(batch.is_err());
    }
    //#endregion 🔖Command

    //#region 🔖Bridge
    mod bridge {
        use super::*;

        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        struct Counter(i64);

        #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
        struct AddDiff(i64);

        impl protocol::OperationDiff<Counter> for AddDiff {
            fn apply(&self, base: &Counter) -> Counter {
                Counter(base.0 + self.0)
            }
            fn absorb(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        struct Add(i64);

        impl protocol::Operation<Counter> for Add {
            type Diff = AddDiff;
            fn diff(&self, _base: &Counter) -> Self::Diff {
                AddDiff(self.0)
            }
            fn backwards(&self, _base: &Counter) -> Vec<Self> {
                vec![Add(-self.0)]
            }
        }

        #[test]
        fn envelope_from_operation_uses_operation_and_diff_traits() {
            let base = Counter(10);
            let op = Add(5);
            let envelope = envelope_from_operation(
                document_id(),
                "counter",
                &op,
                &base,
                protocol::ActorId("alice".to_string()),
                protocol::OperationId("op-add-1".to_string()),
                protocol::HybridLogicalTimestamp::new(1, 0),
            )
            .unwrap();
            let entries = diff_entries(&envelope.diff).unwrap();
            assert_eq!(entries.len(), 1);
            let (path, value) = &entries[0];
            assert_eq!(path, "counter");
            let new_value: Counter = serde_json::from_value(value.clone().unwrap()).unwrap();
            assert_eq!(new_value.0, 15);
        }
    }
    //#endregion 🔖Bridge

    //#region 🔖Engine submit + materialize + WAL replay
    #[test]
    fn submit_persists_to_wal_and_updates_materialized_state_and_frontier() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();

        let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))])]).unwrap();
        let receipt = engine.submit(batch, SubmitOptions { durability: db_core::DurabilityClass::Fsync }, 1).unwrap();

        assert_eq!(receipt.command_id, protocol::OperationId("op-1".to_string()));
        assert_eq!(receipt.frontier.head_seq, 1);
        assert_eq!(receipt.frontier.commit_seq, 1);
        assert!(receipt.conflicts.is_empty());
        assert!(receipt.state_hash.is_some());

        let stored = engine.get("name").unwrap();
        let value: serde_json::Value = serde_json::from_slice(&stored).unwrap();
        assert_eq!(value, serde_json::json!("hello"));
        assert_eq!(engine.frontier().head_seq, 1);
    }

    #[test]
    fn open_replays_the_wal_and_reconstructs_state_and_frontier_identically() {
        let storage = storage();
        {
            let mut engine = DocumentEngine::create(document_id(), storage.clone(), DocumentEngineConfig::default(), 0).unwrap();
            let batch1 = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))])]).unwrap();
            engine.submit(batch1, SubmitOptions { durability: db_core::DurabilityClass::Fsync }, 1).unwrap();
            let batch2 = CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("count", serde_json::json!(2))])]).unwrap();
            engine.submit(batch2, SubmitOptions { durability: db_core::DurabilityClass::Fsync }, 2).unwrap();
        }

        let (reopened, report) = DocumentEngine::open(document_id(), &storage, DocumentEngineConfig::default(), 3).unwrap();
        assert_eq!(report.torn_tail_bytes, 0);
        assert_eq!(reopened.frontier().head_seq, 2);
        assert_eq!(reopened.frontier().commit_seq, 2);

        let name: serde_json::Value = serde_json::from_slice(&reopened.get("name").unwrap()).unwrap();
        assert_eq!(name, serde_json::json!("hello"));
        let count: serde_json::Value = serde_json::from_slice(&reopened.get("count").unwrap()).unwrap();
        assert_eq!(count, serde_json::json!(2));
    }

    #[test]
    fn materialize_from_snapshot_plus_wal_suffix_matches_full_replay() {
        let storage = storage();
        {
            let mut engine = DocumentEngine::create(document_id(), storage.clone(), DocumentEngineConfig::default(), 0).unwrap();
            for i in 0..3 {
                let key = format!("path-{i}");
                let value = format!("value-{i}");
                let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))])]).unwrap();
                engine.submit(batch, SubmitOptions { durability: db_core::DurabilityClass::Fsync }, i).unwrap();
            }
            engine.snapshot_now(10).unwrap();
            for i in 3..6 {
                let key = format!("path-{i}");
                let value = format!("value-{i}");
                let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))])]).unwrap();
                engine.submit(batch, SubmitOptions { durability: db_core::DurabilityClass::Fsync }, i).unwrap();
            }
        }

        let (reopened, report) = DocumentEngine::open(document_id(), &storage, DocumentEngineConfig::default(), 20).unwrap();
        assert!(report.from_snapshot);
        assert_eq!(report.commands_replayed, 3);
        assert_eq!(reopened.frontier().head_seq, 6);
        for i in 0..6 {
            let value: serde_json::Value = serde_json::from_slice(&reopened.get(&format!("path-{i}")).unwrap()).unwrap();
            assert_eq!(value, serde_json::json!(format!("value-{i}")));
        }
    }

    #[test]
    fn deletion_via_json_null_tombstones_a_path() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        engine
            .submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))])]).unwrap(), SubmitOptions::default(), 0)
            .unwrap();
        assert!(engine.get("x").is_some());
        engine
            .submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("x", serde_json::Value::Null)])]).unwrap(), SubmitOptions::default(), 1)
            .unwrap();
        assert!(engine.get("x").is_none());
    }
    //#endregion 🔖Engine submit + materialize + WAL replay

    //#region 🔖Deps + Dedupe
    #[test]
    fn submit_rejects_an_envelope_whose_dependency_was_never_applied() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        let batch = CommandBatch::new(vec![envelope("op-2", &["op-1-never-applied"], "alice", &[("x", serde_json::json!(1))])]).unwrap();
        let result = engine.submit(batch, SubmitOptions::default(), 0);
        assert!(matches!(result, Err(DbError::InvalidArgument(_))));
        assert!(engine.get("x").is_none(), "a rejected batch must not have partially applied");
    }

    #[test]
    fn resubmitting_the_same_batch_returns_the_cached_receipt_without_advancing_the_frontier() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        let batch = || CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))])]).unwrap();

        let first = engine.submit(batch(), SubmitOptions::default(), 0).unwrap();
        let frontier_after_first = engine.frontier();
        let second = engine.submit(batch(), SubmitOptions::default(), 1).unwrap();

        assert_eq!(first, second);
        assert_eq!(engine.frontier(), frontier_after_first, "a deduped resubmit must not move the frontier");
    }
    //#endregion 🔖Deps + Dedupe

    //#region 🔖Conflict
    #[test]
    fn concurrent_write_to_the_same_path_without_a_dependency_is_recorded_as_a_conflict() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))])]).unwrap(), SubmitOptions::default(), 0).unwrap();

        // op-2 writes the same path but does NOT declare op-1 as a dependency: a real concurrent
        // write from `op-2`'s author's point of view.
        let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))])]).unwrap(), SubmitOptions::default(), 1).unwrap();
        assert_eq!(receipt.conflicts.len(), 1);
        assert_eq!(receipt.conflicts[0].conflicting_with, protocol::OperationId("op-1".to_string()));
        assert_eq!(receipt.conflicts[0].path, "x");
        // Last-writer-wins: the conflicting write still applies.
        let x: serde_json::Value = serde_json::from_slice(&engine.get("x").unwrap()).unwrap();
        assert_eq!(x, serde_json::json!(2));
    }

    #[test]
    fn declaring_the_prior_writer_as_a_dependency_avoids_the_conflict() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))])]).unwrap(), SubmitOptions::default(), 0).unwrap();
        let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "bob", &[("x", serde_json::json!(2))])]).unwrap(), SubmitOptions::default(), 1).unwrap();
        assert!(receipt.conflicts.is_empty());
    }

    #[test]
    fn preview_conflicts_uses_real_db_conflict_detector_against_recent_history() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))])]).unwrap(), SubmitOptions::default(), 0).unwrap();

        let probe = CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))])]).unwrap();
        let conflicts = engine.preview_conflicts(&probe).unwrap();
        assert!(!conflicts.is_empty(), "db_conflict must detect the same-path intersection against recent history");
    }
    //#endregion 🔖Conflict

    //#region 🔖Undo
    #[test]
    fn undo_applies_the_recorded_inverse_and_produces_a_fresh_commit() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        let original = protocol::OperationEnvelope {
            operation_id: protocol::OperationId("op-1".to_string()),
            document_id: document_id(),
            actor: protocol::ActorId("alice".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff { schema: "generic".to_string(), payload: serde_json::json!({ "x": 1 }) },
            inverse: protocol::InverseOperation { schema: "generic".to_string(), inverse_diff: serde_json::json!({ "x": null }) },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        };
        engine.submit(CommandBatch::new(vec![original]).unwrap(), SubmitOptions::default(), 0).unwrap();
        assert!(engine.get("x").is_some());

        let receipt = engine
            .undo(&protocol::OperationId("op-1".to_string()), protocol::OperationId("op-1-undo".to_string()), protocol::ActorId("alice".to_string()), 1)
            .unwrap();
        assert_eq!(receipt.frontier.head_seq, 2);
        assert!(engine.get("x").is_none(), "undo must have applied the recorded inverse (delete x)");
    }

    #[test]
    fn undo_of_an_unknown_operation_errs_not_found() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        let result = engine.undo(&protocol::OperationId("never-applied".to_string()), protocol::OperationId("undo-1".to_string()), protocol::ActorId("alice".to_string()), 0);
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }
    //#endregion 🔖Undo

    //#region 🔖Preview
    #[test]
    fn preview_is_never_durable_and_a_conflicting_commit_supersedes_it() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();

        let preview_id = engine.publish_preview(&[("y".to_string(), Some(serde_json::json!("preview-value")))], 0).unwrap();
        assert_eq!(engine.preview_status(&preview_id).unwrap(), db_preview::PreviewState::Active);
        let preview_value: serde_json::Value = serde_json::from_slice(&engine.preview_get(&preview_id, "y").unwrap().unwrap()).unwrap();
        assert_eq!(preview_value, serde_json::json!("preview-value"));
        assert!(engine.get("y").is_none(), "a preview must never be visible in committed state");

        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("y", serde_json::json!("committed-value"))])]).unwrap(), SubmitOptions::default(), 1).unwrap();
        assert_eq!(engine.preview_status(&preview_id).unwrap(), db_preview::PreviewState::Superseded, "an intersecting real commit must supersede the preview");

        let committed: serde_json::Value = serde_json::from_slice(&engine.get("y").unwrap()).unwrap();
        assert_eq!(committed, serde_json::json!("committed-value"));
    }

    #[test]
    fn preview_withdraw_and_expire_transitions() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();

        let withdrawn_id = engine.publish_preview(&[("a".to_string(), Some(serde_json::json!(1)))], 0).unwrap();
        engine.withdraw_preview(&withdrawn_id).unwrap();
        assert_eq!(engine.preview_status(&withdrawn_id).unwrap(), db_preview::PreviewState::Withdrawn);

        let dummy_id = db_preview::PreviewId("does-not-exist".to_string());
        assert!(matches!(engine.preview_status(&dummy_id), Err(DbError::NotFound(_))));
    }
    //#endregion 🔖Preview

    //#region 🔖Security
    #[test]
    fn security_gate_rejects_a_principal_denied_by_its_policy() {
        // An empty `RoleBasedPolicy` (no grants at all) denies every action, per its own doc — a
        // default-deny policy, matching `db_engine`'s own equivalent test of the same gate.
        let security = db_security::SecurityGate::new(
            db_security::RoleBasedPolicy::new(),
            db_security::ReplayGuard::new(60_000, 1_024),
            db_security::BudgetRegistry::new(100_000, 100_000),
            Arc::new(db_core::NullEmit),
        );
        let storage = storage();
        let config = DocumentEngineConfig { security, ..DocumentEngineConfig::default() };
        let mut engine = DocumentEngine::create(document_id(), storage, config, 0).unwrap();
        let result = engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("x", serde_json::json!(1))])]).unwrap(), SubmitOptions::default(), 0);
        assert!(matches!(result, Err(DbError::Unauthorized(_))));
        assert!(engine.get("x").is_none());
    }
    //#endregion 🔖Security

    //#region 🔖Query
    #[test]
    fn query_finds_a_committed_row_by_path() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("greeting", serde_json::json!("hello"))])]).unwrap(), SubmitOptions::default(), 0).unwrap();

        let query = db_query::Query::new().filter(db_query::Predicate::Eq(db_query::Path::empty().push_field("path"), db_query::Value::Text("greeting".to_string())));
        let result = engine.query(query, db_query::Consistency::Canonical).unwrap();
        assert_eq!(result.rows.len(), 1);
    }

    #[test]
    fn live_query_refresh_reports_no_further_diff_right_after_submit_already_refreshed_it() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        let id = engine.subscribe(db_query::LiveQuerySpec { query: db_query::Query::new(), consistency: db_query::Consistency::Canonical });
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))])]).unwrap(), SubmitOptions::default(), 0).unwrap();
        let diffs = engine.refresh_live_queries();
        assert!(diffs.is_empty());
        engine.unsubscribe(id);
    }
    //#endregion 🔖Query

    //#region 🔖Outbox + CommitLog
    #[test]
    fn outbox_and_commit_log_accumulate_and_outbox_drains() {
        let storage = storage();
        let mut engine = DocumentEngine::create(document_id(), storage, DocumentEngineConfig::default(), 0).unwrap();
        engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))])]).unwrap(), SubmitOptions::default(), 0).unwrap();

        assert_eq!(engine.commit_log().len(), 1);
        assert_eq!(engine.commit_log()[0].operation_ids, vec![protocol::OperationId("op-1".to_string())]);

        let drained = engine.drain_outbox();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].operation_id, protocol::OperationId("op-1".to_string()));
        assert!(engine.drain_outbox().is_empty(), "drain must clear the outbox");
    }
    //#endregion 🔖Outbox + CommitLog

    //#region 🔖Actor
    #[test]
    fn document_authority_submits_and_queries_over_the_mailbox_from_a_dedicated_thread() {
        let storage = storage();
        let document = document_id();
        let authority = DocumentAuthority::spawn(move || DocumentEngine::create(document, storage, DocumentEngineConfig::default(), 0), db_core::MailboxCapacities::uniform(16))
            .unwrap();

        let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hi"))])]).unwrap();
        let receipt = authority.submit_blocking(batch, SubmitOptions::default(), 0).unwrap();
        assert_eq!(receipt.frontier.head_seq, 1);

        let queried: serde_json::Value = serde_json::from_slice(&authority.query_blocking("name").unwrap().unwrap()).unwrap();
        assert_eq!(queried, serde_json::json!("hi"));

        let frontier = authority.frontier_blocking().unwrap();
        assert_eq!(frontier.head_seq, 1);

        let generation = authority.snapshot_now_blocking(1).unwrap();
        assert_eq!(generation, 0);

        authority.shutdown();
    }

    #[test]
    fn document_authority_spawn_propagates_a_build_failure_synchronously() {
        let result = DocumentAuthority::spawn(|| Err(DbError::InvalidArgument("boom".to_string())), db_core::MailboxCapacities::uniform(4));
        assert!(matches!(result, Err(DbError::InvalidArgument(_))));
    }
    //#endregion 🔖Actor
}
//#endregion 🧪Tests
