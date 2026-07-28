//! 🗄️ `db_engine` — the `Database` supervisor and catalog actor: the crate that assembles every
//! other `db_*` crate into the stable, contract-frozen `Database`/`DocumentHandle` API
//! (`Database::{open, open_at, create_document, document, catalog, health, shutdown}`;
//! `DocumentHandle::{submit, query, subscribe, frontier, preview, history, snapshot_now}`).
//! Frozen contract: `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_engine` row + "Stable API" block).
//!
//! 🎯 Design choice (compatibility surface): `db_document` (a concurrent sibling session) commits
//! explicitly, in its own module doc, to keeping the `AuthzHook`/`AllowAll` seam, its local
//! `ConflictRecord{command_id, conflicting_with, path}` shape, single-field `SubmitOptions
//! {durability}` (with `Default`), and 4-field `DocumentEngineConfig{limits, authz,
//! version_graph: Option<..>, preview_ttl_ms}` byte-for-byte stable specifically because THIS crate
//! constructs every one of those verbatim — so this crate is written directly against that exact
//! surface (verified against `db/document/rs/lib.rs` at the time of writing) rather than any richer,
//! transiently-observed intermediate revision of it.
//!
//! 🎯 Design choice (scope): per this wave's instructions, this crate makes `Database::open_at`
//! (zero-touch `FsStorage`) and a full submit → durable → query round trip over a REAL
//! `db_document::DocumentAuthority` genuinely work end to end (see `//#region 🧪Tests`), composing
//! the guaranteed-complete `db_state`/`db_wal`/`db_storage`/`db_document` crates against their real,
//! current APIs throughout. `db_cluster` is still an unimplemented stub upstream of this wave (its
//! `lib.rs` declares no public items at all) — nothing in this crate can call into it yet; every
//! cluster-shaped concern (sharding, ownership leases, quorum durability, split-brain repair) is
//! deferred wholesale, documented here rather than faked. `db_compact`/`db_sync`/`db_security`/
//! `db_observe` ARE genuinely wired, but narrowly: `Database::compact_document` drives a real
//! `db_compact::Compactor` pass, `Database::hello` drives `db_sync::handle_hello` for the wire-v2
//! handshake (no transport of its own — that is CW5/CW6's job), `SecurityAuthzHook` wraps a real
//! `db_security::SecurityGate` as an optional `AuthzHook`, and `Database::open`/`open_at` wire a
//! real `db_observe::StructuredSink`/`HealthRegistry` pair by default. `DocumentHandle::preview`/
//! `subscribe` return `DbError::Unimplemented` (not a panic, not a fake success): `db_document`'s
//! own `DocumentAuthority` mailbox (`db/document/rs/lib.rs`'s `DocumentMessage` enum) only carries
//! `Submit`/`Query`/`Frontier` variants — there is no way to drive its preview/commit-log machinery
//! through the actor boundary without editing `db_document` itself, which is out of this crate's
//! ownership this wave. `snapshot_now` is likewise `Unimplemented`: `db_document`'s own module doc
//! documents that `DocumentState` materializes purely from the WAL suffix with no full-state
//! enumeration to serialize into a pack snapshot, and `db_snapshot` is not even a direct dependency
//! of this crate per its `Cargo.toml`. `DocumentHandle::history` IS real: it replays a document's
//! WAL directly via `db_wal::replay_document` (a crate this one already depends on) rather than
//! going through the actor, since `db_document::DocumentEngine`'s in-memory `commit_log` is only
//! populated by live `submit()` calls in the current process, not reconstructed by `open()`'s replay.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use db_core::DbError;

//#region 🔖Reexports
pub use db_core::{DbCapabilities, DbConfig, DurabilityClass, Profile};
pub use db_storage::DbStorage;
//#endregion 🔖Reexports

//#region 🔖Ids
/// @emoji 🌉 `protocol::DocumentId` → `db_core::DocumentId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises — see `db_document`'s identical helper for the rationale (this
/// crate is the other place in the family that depends on both `db_core` and `protocol`).
fn to_core_document_id(id: &protocol::DocumentId) -> db_core::DocumentId {
    db_core::DocumentId(id.0.clone())
}

/// @emoji 🌉 `protocol::ActorId` → `db_core::ActorId`, same bridge as `to_core_document_id`.
fn to_core_actor_id(id: &protocol::ActorId) -> db_core::ActorId {
    db_core::ActorId(id.0.clone())
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}
//#endregion 🔖Ids

//#region 🔖Frontier
/// @emoji 🧭 The facade-level frontier: identical shape to `db_core::Frontier` except keyed by
/// `protocol::DocumentId` (not `db_core::DocumentId`) — the frozen contract's exact
/// `Frontier{document, head_seq, commit_seq, chain_hash, epoch}` shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Frontier {
    pub document: protocol::DocumentId,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub epoch: u64,
}

impl Frontier {
    /// @emoji 🏔️ True iff `self` has observed everything `other` has — mirrors
    /// `db_core::Frontier::dominates`, re-derived here since this type's `document` field has a
    /// different type than `db_core::Frontier`'s.
    pub fn dominates(&self, other: &Frontier) -> Result<bool, DbError> {
        if self.document != other.document {
            return Err(DbError::InvalidArgument(format!("frontier document mismatch: {} vs {}", self.document.0, other.document.0)));
        }
        Ok(self.head_seq >= other.head_seq && self.commit_seq >= other.commit_seq && self.epoch >= other.epoch)
    }
}

fn to_engine_frontier(core: &db_core::Frontier, document: protocol::DocumentId) -> Frontier {
    Frontier { document, head_seq: core.head_seq, commit_seq: core.commit_seq, chain_hash: core.chain_hash, epoch: core.epoch }
}
//#endregion 🔖Frontier

//#region 🔖Receipt
/// @emoji 🧾 The frozen `CommandReceipt` shape: `DocumentHandle::submit`'s resolved output.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandReceipt {
    pub command_id: protocol::OperationId,
    pub frontier: Frontier,
    pub durability: DurabilityClass,
    pub conflicts: Vec<db_document::ConflictRecord>,
    pub state_hash: Option<pack_core::ContentHash>,
}

fn to_engine_receipt(receipt: db_document::CommandReceipt, document: protocol::DocumentId) -> CommandReceipt {
    CommandReceipt {
        command_id: receipt.command_id,
        frontier: to_engine_frontier(&receipt.frontier, document),
        durability: receipt.durability,
        conflicts: receipt.conflicts,
        state_hash: receipt.state_hash,
    }
}
//#endregion 🔖Receipt

//#region 🔖Consistency
/// @emoji 🎚️ The frozen `Consistency` enum: which frontier/view `DocumentHandle::query` must
/// resolve against.
#[derive(Clone, Debug, PartialEq)]
pub enum Consistency {
    Canonical,
    AtLeast(Frontier),
    Exact(Frontier),
    Historical(String),
    Speculative(String),
    PreviewAugmented(String),
}
//#endregion 🔖Consistency

//#region 🔖Query
/// @emoji 🔎 What `DocumentHandle::query` can ask for — this crate's own choice (the contract fixes
/// `query`'s signature, not `Query`'s shape): single or multi-path point lookups against the
/// document's schema-erased path/value convention (see `db_document`'s module doc), matching what
/// `DocumentAuthority`'s mailbox actually exposes (`DocumentMessage::Query { path, .. }`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query {
    Get { path: String },
    GetMany { paths: Vec<String> },
}

/// @emoji 📬 One resolved `query`: every requested path paired with its current value bytes (`None`
/// if unset/tombstoned).
#[derive(Clone, Debug, PartialEq)]
pub struct QueryStream {
    pub results: Vec<(String, Option<Vec<u8>>)>,
}
//#endregion 🔖Query

//#region 🔖History
/// @emoji 📜 One committed batch's identity plus the frontier it produced — `DocumentHandle::history`'s
/// unit, reconstructed from a direct `db_wal::replay_document` pass (see module doc for why this
/// does NOT go through `DocumentAuthority`'s mailbox).
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEntry {
    pub operation_ids: Vec<protocol::OperationId>,
    pub frontier: Frontier,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryView {
    pub entries: Vec<HistoryEntry>,
}

/// @emoji 🔁 Replays `document`'s ENTIRE WAL directly (bypassing the actor — see module doc) and
/// groups `WAL_COMMAND` records by the `WAL_FRONTIER` record that closes their transaction, exactly
/// mirroring `db_document::DocumentEngine::submit`'s own commit shape (one frontier record per
/// committed batch, preceded by that batch's command records).
fn replay_history(storage: &dyn DbStorage, core_document: &db_core::DocumentId, protocol_document: &protocol::DocumentId) -> Result<HistoryView, DbError> {
    let records = db_wal::replay_document(storage.wal(), core_document)?;
    let mut entries = Vec::new();
    let mut pending_operation_ids: Vec<protocol::OperationId> = Vec::new();
    for record in records {
        match record {
            db_wal::WalRecord::TxBegin { .. } => pending_operation_ids.clear(),
            db_wal::WalRecord::Command(bytes) => {
                let envelope: protocol::OperationEnvelope =
                    serde_json::from_slice(&bytes).map_err(|err| DbError::Corrupt(format!("history: wal command record is not a valid operation envelope: {err}")))?;
                pending_operation_ids.push(envelope.operation_id);
            }
            db_wal::WalRecord::Frontier(frontier)
                if !pending_operation_ids.is_empty() => {
                    entries.push(HistoryEntry { operation_ids: std::mem::take(&mut pending_operation_ids), frontier: to_engine_frontier(&frontier, protocol_document.clone()) });
                }
            _ => {}
        }
    }
    Ok(HistoryView { entries })
}
//#endregion 🔖History

//#region 🔖LiveQuery + Preview
/// @emoji 📡 What `DocumentHandle::subscribe` would filter on — defined for API-shape completeness
/// even though every construction path currently returns `DbError::Unimplemented` (see module doc).
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuerySpec {
    pub since: Option<Frontier>,
}

/// @emoji 📡 A live subscription handle — see `LiveQuerySpec`'s doc on why this is currently
/// unreachable except through the documented `Unimplemented` error.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuery {
    pub id: String,
}

/// @emoji 🌫️ An ephemeral preview overlay handle — see `LiveQuerySpec`'s doc; same deferral reason.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewHandle {
    pub id: String,
    pub base: Frontier,
}
//#endregion 🔖LiveQuery + Preview

//#region 🔖Snapshot
/// @emoji 📸 What kind of snapshot `DocumentHandle::snapshot_now` was asked to build — defined for
/// API-shape completeness (see module doc: this crate does not yet build real pack snapshots).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotKind {
    Full,
    Incremental,
}

/// @emoji 📸 What a successful `snapshot_now` would resolve to — currently unreachable, see above.
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotReceipt {
    pub generation: u64,
    pub frontier: Frontier,
}

pub type SnapshotFuture = db_actor::ReplyReceiver<Result<SnapshotReceipt, DbError>>;
//#endregion 🔖Snapshot

//#region 🔖Security
/// @emoji 🛂 A real `db_document::AuthzHook` built on `db_security::SecurityGate`: resolves the
/// submitting `protocol::ActorId` to a `db_security::Principal` via an injected closure, then
/// authorizes `Action::Write` on `AuthzScope::Document { document }`. Not the default (the default
/// stays `db_document::AllowAll`, matching `db_document`'s own single-tenant default) — opt in via
/// `Database::open_with_authz`.
pub struct SecurityAuthzHook {
    gate: db_security::SecurityGate,
    principal_for: Box<dyn Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync>,
}

impl SecurityAuthzHook {
    pub fn new(gate: db_security::SecurityGate, principal_for: impl Fn(&protocol::ActorId) -> db_security::Principal + Send + Sync + 'static) -> SecurityAuthzHook {
        SecurityAuthzHook { gate, principal_for: Box::new(principal_for) }
    }
}

impl db_document::AuthzHook for SecurityAuthzHook {
    fn authorize(&self, actor: &protocol::ActorId, envelope: &protocol::OperationEnvelope) -> Result<(), DbError> {
        let principal = (self.principal_for)(actor);
        self.gate.authorize(&principal, &db_security::AuthzScope::Document { document: envelope.document_id.clone() }, db_security::Action::Write)
    }
}
//#endregion 🔖Security

//#region 🔖VersionGraph
/// @emoji 🌿 The real `vcs`-backed `db_core::VersionGraph` — the ONLY place in the whole `db`
/// family allowed to depend on `vcs` (hard dependency rule; gated behind this crate's default-on
/// `vcs` Cargo feature).
#[cfg(feature = "vcs")]
pub mod vcs_integration {
    use db_core::DbError;
    use std::collections::HashMap;
    use std::sync::Mutex;

    //#region 🔖SchemaErasedTypes
    /// @emoji #️⃣ The `VersionGraph` seam (`db_core::ChangeRecord`/`CheckpointRequest`) is already
    /// schema-erased — it carries a `pack_core::ContentHash`, never document semantics — so this
    /// crate drives the real `store::DocumentStore<P, Operation>` with the smallest concrete `P`/
    /// `Operation` pair that can faithfully round-trip exactly that: a projection that IS the
    /// latest recorded hash, and an operation that overwrites it (its `backwards` recovering the
    /// PRIOR hash from the pre-state, a real, correct inverse — not a placeholder). This mirrors
    /// `db_document`'s own schema-erased-JSON convention one layer up: neither crate has (or needs)
    /// compile-time knowledge of any real document schema.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct HashProjection {
        pub latest_hash: [u8; 32],
    }

    impl store::DocumentDsl for HashProjection {
        const EXTENSION: &'static str = "dbhash";

        fn parse_dsl(text: &str) -> Result<HashProjection, store::TextError> {
            let trimmed = text.trim();
            if trimmed.len() != 64 || !trimmed.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return Err(store::TextError::new("expected 64 lowercase hex characters", store::TextSpan::at(1, 1)));
            }
            let mut latest_hash = [0u8; 32];
            for (index, slot) in latest_hash.iter_mut().enumerate() {
                *slot = u8::from_str_radix(&trimmed[index * 2..index * 2 + 2], 16)
                    .map_err(|_| store::TextError::new("invalid hex byte", store::TextSpan::at(1, (index * 2 + 1) as u32)))?;
            }
            Ok(HashProjection { latest_hash })
        }

        fn print_dsl(&self) -> String {
            let mut out = String::with_capacity(64);
            for byte in self.latest_hash {
                use std::fmt::Write;
                let _ = write!(out, "{byte:02x}");
            }
            out
        }
    }

    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct HashDiff {
        pub hash: Option<[u8; 32]>,
    }

    impl protocol::OperationDiff<HashProjection> for HashDiff {
        fn apply(&self, base: &HashProjection) -> HashProjection {
            match self.hash {
                Some(hash) => HashProjection { latest_hash: hash },
                None => base.clone(),
            }
        }

        fn absorb(&mut self, other: HashDiff) {
            if other.hash.is_some() {
                self.hash = other.hash;
            }
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct HashOperation {
        pub hash: [u8; 32],
        pub author: Option<protocol::ActorId>,
        pub timestamp: Option<protocol::HybridLogicalTimestamp>,
    }

    impl protocol::Operation<HashProjection> for HashOperation {
        type Diff = HashDiff;

        fn diff(&self, _base: &HashProjection) -> HashDiff {
            HashDiff { hash: Some(self.hash) }
        }

        /// @emoji ↩️ The true inverse: an operation that would restore `base`'s hash — not a
        /// no-op placeholder.
        fn backwards(&self, base: &HashProjection) -> Vec<HashOperation> {
            vec![HashOperation { hash: base.latest_hash, author: self.author.clone(), timestamp: self.timestamp }]
        }

        fn author_id(&self) -> Option<protocol::ActorId> {
            self.author.clone()
        }

        fn timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
            self.timestamp
        }
    }
    //#endregion 🔖SchemaErasedTypes

    //#region 🔖Store
    type HashStore = store::DocumentStore<HashProjection, HashOperation>;

    // 🔒 Used as a bare fn-pointer error mapper (`.map_err(map_vcs_error)`) below — same rationale
    // as `db_document`'s `json_err`: `Result::map_err`'s `FnOnce(E) -> F2` bound always calls the
    // mapper with an owned `E`, so a by-reference signature would not type-check at those sites.
    #[allow(clippy::needless_pass_by_value)]
    fn map_vcs_error(err: vcs::VcsError) -> DbError {
        DbError::Internal(format!("vcs: {err}"))
    }

    /// @emoji 🌿 One real `store::DocumentStore` per document, driven by real `Apply`/
    /// `CommitCheckpoint` dispatches — `db_core::VersionGraph`'s real implementation.
    pub struct VcsVersionGraph {
        stores: Mutex<HashMap<String, HashStore>>,
    }

    impl Default for VcsVersionGraph {
        fn default() -> VcsVersionGraph {
            VcsVersionGraph { stores: Mutex::new(HashMap::new()) }
        }
    }

    impl VcsVersionGraph {
        pub fn new() -> VcsVersionGraph {
            VcsVersionGraph::default()
        }

        fn with_store<R>(&self, document: &db_core::DocumentId, f: impl FnOnce(&mut HashStore) -> Result<R, DbError>) -> Result<R, DbError> {
            let mut stores = self.stores.lock().map_err(|_| DbError::Internal("vcs_integration: store registry mutex poisoned".to_string()))?;
            let store = stores.entry(document.0.clone()).or_insert_with(|| {
                let envelope = store::create_document_envelope::<HashProjection, HashOperation>("db_engine.version_graph", &document.0, HashProjection::default(), None);
                store::DocumentStore::new(envelope)
            });
            f(store)
        }
    }

    impl db_core::VersionGraph for VcsVersionGraph {
        fn record_change(&self, document: &db_core::DocumentId, change: db_core::ChangeRecord) -> Result<String, DbError> {
            self.with_store(document, |store| {
                let operation = HashOperation {
                    hash: change.content_hash.0,
                    author: Some(protocol::ActorId(change.author.0.clone())),
                    timestamp: Some(protocol::HybridLogicalTimestamp::new(0, change.timestamp_ms)),
                };
                store.dispatch(store::DocumentCommand::Apply { operations: vec![operation], description: Some(change.message.clone()) }).map_err(map_vcs_error)?;
                Ok(store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default())
            })
        }

        /// @emoji 🎯 Design choice: `request.parent_checkpoint`/`change_ids` are NOT threaded
        /// through — `store::DocumentCommand::CommitCheckpoint` always folds every edit applied
        /// since the store's OWN current checkpoint (tracked internally by `DocumentStore`,
        /// advanced by `record_change`'s `Apply` calls above), which is the only value that could
        /// ever be consistent with this store's real history. `request.timestamp_ms` is similarly
        /// unused: `vcs`'s own `CommitCheckpoint` handler stamps its own `now_iso()` timestamp into
        /// the checkpoint (part of what its content-addressed id hashes over) — this crate cannot
        /// override that without reaching into `vcs`'s private state.
        fn checkpoint(&self, document: &db_core::DocumentId, request: db_core::CheckpointRequest) -> Result<String, DbError> {
            self.with_store(document, |store| {
                let authors: Vec<vcs::Author> = request.authors.iter().map(|author| vcs::Author { id: author.0.clone(), name: author.0.clone(), avatar: None }).collect();
                store.dispatch(store::DocumentCommand::CommitCheckpoint { message: Some(request.message.clone()), authors }).map_err(map_vcs_error)?;
                store.current_checkpoint_id().map(str::to_string).ok_or_else(|| DbError::Internal("vcs: commit_checkpoint produced no checkpoint id".to_string()))
            })
        }

        fn merge_base(&self, document: &db_core::DocumentId, a: &str, b: &str) -> Result<Option<String>, DbError> {
            self.with_store(document, |store| Ok(store::merge_base(store.envelope(), a, b)))
        }

        fn head(&self, document: &db_core::DocumentId, alternative: &str) -> Result<Option<String>, DbError> {
            self.with_store(document, |store| {
                let envelope = store.envelope();
                if let Some(found) = envelope.vcs.alternatives.iter().find(|candidate| candidate.id == alternative || candidate.name == alternative) {
                    return Ok(found.checkpoint_ids.last().cloned());
                }
                Ok(store.current_checkpoint_id().map(str::to_string))
            })
        }
    }
    //#endregion 🔖Store
}
//#endregion 🔖VersionGraph

//#region 🔖Observe
/// @emoji 📡 The default observability wiring `Database::open`/`open_at` build when the caller
/// doesn't supply their own: an in-memory `db_observe::StructuredSink` (real JSON-lines encoding,
/// just not flushed anywhere durable by default — a caller wanting file/pipe output constructs
/// `db_observe::WriterSink` themselves and passes it via `Database::open_with_emit`).
fn default_emit() -> Arc<dyn db_core::Emit> {
    Arc::new(db_observe::StructuredSink::new(db_observe::MemorySink::new()))
}
//#endregion 🔖Observe

//#region 🔖Catalog
/// @emoji 📇 One document known to this `Database`'s catalog.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogEntry {
    pub document: protocol::DocumentId,
    pub created_at_ms: u64,
}

/// @emoji 📇 A point-in-time read of every document this `Database` knows about.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogView {
    pub documents: Vec<CatalogEntry>,
}

/// @emoji 💾 The catalog root's on-disk shape — a plain JSON array, deliberately NOT reusing
/// `CatalogEntry` directly (keeps the public type free of a `serde` bound it doesn't otherwise need).
#[derive(serde::Serialize, serde::Deserialize)]
struct CatalogRootEntry {
    document: String,
    created_at_ms: u64,
}

fn encode_catalog(entries: &[CatalogEntry]) -> Result<Vec<u8>, DbError> {
    let raw: Vec<CatalogRootEntry> = entries.iter().map(|entry| CatalogRootEntry { document: entry.document.0.clone(), created_at_ms: entry.created_at_ms }).collect();
    serde_json::to_vec(&raw).map_err(|err| DbError::Internal(format!("catalog encode: {err}")))
}

fn decode_catalog(bytes: &[u8]) -> Result<Vec<CatalogEntry>, DbError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let raw: Vec<CatalogRootEntry> = serde_json::from_slice(bytes).map_err(|err| DbError::Corrupt(format!("catalog decode: {err}")))?;
    Ok(raw.into_iter().map(|entry| CatalogEntry { document: protocol::DocumentId(entry.document), created_at_ms: entry.created_at_ms }).collect())
}

struct CatalogState {
    epoch: db_core::EpochFence,
    entries: Vec<CatalogEntry>,
}
//#endregion 🔖Catalog

//#region 🔖DocumentSpec
/// @emoji 📄 What `Database::create_document` needs to mint a brand-new document — this crate's own
/// choice (the contract fixes `create_document`'s signature, not `DocumentSpec`'s shape).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSpec {
    pub document: protocol::DocumentId,
}

impl DocumentSpec {
    pub fn new(document: protocol::DocumentId) -> DocumentSpec {
        DocumentSpec { document }
    }
}
//#endregion 🔖DocumentSpec

//#region 🔖Health
/// @emoji 🩺 The frozen `Database::health()` return shape, wrapping a real
/// `db_observe::HealthRegistry` snapshot plus this crate's own catalog-level fact (open document
/// count) that no lower crate could know.
#[derive(Clone, Debug)]
pub struct DbHealth {
    pub report: db_observe::HealthReport,
    pub open_documents: usize,
}
//#endregion 🔖Health

//#region 🔖Database
/// @emoji 🗄️ The catalog: owns the storage substrate, the shared config/capabilities/authz/
/// version-graph/observability wiring every document actor is constructed with, and the registry of
/// currently-open `DocumentAuthority` actors.
///
/// 🎯 Design choice: the catalog registry itself is a plain `Mutex`-guarded `HashMap`, not a
/// separate `db_actor::Actor`-driven process. `Database`'s own public surface (`open`/
/// `create_document`/`document`/`catalog`/`health`/`shutdown`) is already synchronous per the
/// frozen contract, and per-document concurrency is already provided by each `DocumentAuthority`'s
/// own dedicated thread — the catalog only ever needs to serialize document-registry mutations and
/// a catalog-root CAS write, which a `Mutex` does directly without the mailbox's priority-lane/
/// backpressure machinery (that machinery matters for a document's WAL under load, not a rare
/// catalog-root swap).
pub struct Database {
    storage: Arc<dyn DbStorage>,
    config: DbConfig,
    capabilities: DbCapabilities,
    authz: Arc<dyn db_document::AuthzHook>,
    /// @emoji 🌿 Never `None`: `db_core::NullVersionGraph` (an `Unimplemented`-on-every-call
    /// placeholder, not an `Option` layer — see its own doc) is the default when the `vcs` feature
    /// is disabled, exactly matching `db_document::DocumentEngineConfig::default`'s own choice.
    version_graph: Arc<dyn db_core::VersionGraph>,
    emit: Arc<dyn db_core::Emit>,
    health: Arc<db_observe::HealthRegistry>,
    catalog: Mutex<CatalogState>,
    open_documents: Mutex<HashMap<String, Arc<db_document::DocumentAuthority>>>,
}

impl Database {
    /// @emoji 🚀 The frozen entry point: opens (or initializes, if `storage` is fresh) a `Database`
    /// over an arbitrary `Arc<dyn DbStorage>` backend, wired with the default `AllowAll` authz and
    /// (behind the default-on `vcs` feature) a real `VcsVersionGraph`.
    pub fn open(config: DbConfig, storage: Arc<dyn DbStorage>) -> Result<Database, DbError> {
        Database::open_with(config, storage, Arc::new(db_document::AllowAll), default_emit())
    }

    /// @emoji 🚀 Like `open`, but with a caller-supplied `AuthzHook` (e.g. `SecurityAuthzHook`)
    /// instead of the default `AllowAll`.
    pub fn open_with_authz(config: DbConfig, storage: Arc<dyn DbStorage>, authz: Arc<dyn db_document::AuthzHook>) -> Result<Database, DbError> {
        Database::open_with(config, storage, authz, default_emit())
    }

    /// @emoji 🚀 Like `open`, but with a caller-supplied `Emit` sink (e.g. a `db_observe::WriterSink`
    /// over a real file) instead of the default in-memory one.
    pub fn open_with_emit(config: DbConfig, storage: Arc<dyn DbStorage>, emit: Arc<dyn db_core::Emit>) -> Result<Database, DbError> {
        Database::open_with(config, storage, Arc::new(db_document::AllowAll), emit)
    }

    fn open_with(config: DbConfig, storage: Arc<dyn DbStorage>, authz: Arc<dyn db_document::AuthzHook>, emit: Arc<dyn db_core::Emit>) -> Result<Database, DbError> {
        let storage_capabilities = storage.capabilities();
        let capabilities = DbCapabilities {
            // 🧩 Extension seam: real, honest today — see module doc on why preview/live-query
            // aren't reachable through `DocumentAuthority`'s current mailbox surface, and why
            // `db_cluster` is still an empty stub upstream of this wave.
            preview: false,
            historical_query: true,
            live_query: false,
            cluster: false,
            max_durability: std::cmp::min(storage_capabilities.max_durability, config.capabilities.max_durability),
        };

        let health = Arc::new(db_observe::HealthRegistry::new());
        health.set("db_engine.storage", if storage_capabilities.durable { db_observe::HealthState::Healthy } else { db_observe::HealthState::Degraded("storage backend is not durable".to_string()) });

        let (epoch, entries) = match storage.catalog().read_root()? {
            Some((bytes, epoch)) => (epoch, decode_catalog(&bytes)?),
            None => {
                let empty = encode_catalog(&[])?;
                let epoch = storage.catalog().cas_root(db_core::EpochFence::INITIAL, &empty)?;
                (epoch, Vec::new())
            }
        };
        health.set("db_engine.catalog", db_observe::HealthState::Healthy);

        #[cfg(feature = "vcs")]
        let version_graph: Arc<dyn db_core::VersionGraph> = Arc::new(vcs_integration::VcsVersionGraph::new());
        #[cfg(not(feature = "vcs"))]
        let version_graph: Arc<dyn db_core::VersionGraph> = Arc::new(db_core::NullVersionGraph);

        emit.emit(db_core::EmitEvent::new("db_engine.database_opened").field("documents", db_core::EmitField::U64(entries.len() as u64)));

        Ok(Database {
            storage,
            config,
            capabilities,
            authz,
            version_graph,
            emit,
            health,
            catalog: Mutex::new(CatalogState { epoch, entries }),
            open_documents: Mutex::new(HashMap::new()),
        })
    }

    /// @emoji 🚀 The frozen zero-touch entry point: `FsStorage` rooted at `root`, defaults for
    /// `profile`.
    pub fn open_at(root: &std::path::Path, profile: Profile) -> Result<Database, DbError> {
        let storage: Arc<dyn DbStorage> = Arc::new(db_storage::FsStorage::open(root)?);
        Database::open(DbConfig::for_profile(profile), storage)
    }

    /// @emoji ⚙️ Builds one `DocumentEngineConfig`. Sets the 4 fields this crate has ALWAYS
    /// constructed (`limits`/`authz`/`version_graph`/`preview_ttl_ms`, per the module doc's
    /// compatibility-surface note) explicitly, and spreads `..db_document::DocumentEngineConfig::
    /// default()` for every other field db_document has since grown (e.g. `security`/`emit`/
    /// `projections`) — this crate has no opinion on those yet (`db_document`'s own real
    /// `db_security::SecurityGate`-backed default policy already matches `AllowAll`'s permissive
    /// single-tenant spirit), and the spread keeps this call site correct across further additive
    /// growth without another coordinated edit.
    fn document_engine_config(&self) -> db_document::DocumentEngineConfig {
        db_document::DocumentEngineConfig {
            limits: self.config.limits.clone(),
            authz: self.authz.clone(),
            version_graph: self.version_graph.clone(),
            preview_ttl_ms: self.config.limits.max_preview_ttl_ms,
            ..db_document::DocumentEngineConfig::default()
        }
    }

    fn spawn_authority_create(&self, document: protocol::DocumentId) -> Result<Arc<db_document::DocumentAuthority>, DbError> {
        let storage = self.storage.clone();
        let config = self.document_engine_config();
        let created_at_ms = now_ms();
        let mailbox_capacities = self.config.mailbox_capacities;
        let authority = db_document::DocumentAuthority::spawn(move || db_document::DocumentEngine::create(document, storage, config, created_at_ms), mailbox_capacities)?;
        Ok(Arc::new(authority))
    }

    fn spawn_authority_open(&self, document: protocol::DocumentId) -> Result<Arc<db_document::DocumentAuthority>, DbError> {
        let storage = self.storage.clone();
        let config = self.document_engine_config();
        let opened_at_ms = now_ms();
        let mailbox_capacities = self.config.mailbox_capacities;
        let authority =
            db_document::DocumentAuthority::spawn(move || db_document::DocumentEngine::open(document, &storage, config, opened_at_ms).map(|(engine, _report)| engine), mailbox_capacities)?;
        Ok(Arc::new(authority))
    }

    fn register_handle(&self, document: protocol::DocumentId, authority: Arc<db_document::DocumentAuthority>) -> DocumentHandle {
        let core_document = to_core_document_id(&document);
        self.open_documents.lock().expect("db_engine: open_documents mutex poisoned").insert(document.0.clone(), authority.clone());
        DocumentHandle { authority, storage: self.storage.clone(), document, core_document }
    }

    /// @emoji 🌱 The frozen `create_document`: mints a brand-new document, durably records it in the
    /// catalog root (CAS-fenced), spawns its `DocumentAuthority`, and returns a live handle.
    pub fn create_document(&self, spec: DocumentSpec) -> Result<DocumentHandle, DbError> {
        let document = spec.document;
        {
            let mut catalog = self.catalog.lock().expect("db_engine: catalog mutex poisoned");
            if catalog.entries.iter().any(|entry| entry.document == document) {
                return Err(DbError::AlreadyExists(format!("document {} already exists", document.0)));
            }
            let mut entries = catalog.entries.clone();
            entries.push(CatalogEntry { document: document.clone(), created_at_ms: now_ms() });
            let bytes = encode_catalog(&entries)?;
            let new_epoch = self.storage.catalog().cas_root(catalog.epoch, &bytes)?;
            catalog.epoch = new_epoch;
            catalog.entries = entries;
        }
        let authority = self.spawn_authority_create(document.clone())?;
        self.emit.emit(db_core::EmitEvent::new("db_engine.document_created").with_document(to_core_document_id(&document)));
        Ok(self.register_handle(document, authority))
    }

    /// @emoji 📄 The frozen `document`: returns a live handle to an already-cataloged document,
    /// reusing an already-open `DocumentAuthority` if one exists, else recovering it fresh from its
    /// WAL.
    pub fn document(&self, id: &protocol::DocumentId) -> Result<DocumentHandle, DbError> {
        if let Some(authority) = self.open_documents.lock().expect("db_engine: open_documents mutex poisoned").get(&id.0) {
            return Ok(DocumentHandle { authority: authority.clone(), storage: self.storage.clone(), document: id.clone(), core_document: to_core_document_id(id) });
        }
        let known = self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.iter().any(|entry| &entry.document == id);
        if !known {
            return Err(DbError::NotFound(format!("document {} not found", id.0)));
        }
        let authority = self.spawn_authority_open(id.clone())?;
        self.emit.emit(db_core::EmitEvent::new("db_engine.document_opened").with_document(to_core_document_id(id)));
        Ok(self.register_handle(id.clone(), authority))
    }

    /// @emoji 📇 The frozen `catalog`: a point-in-time read of every document this `Database`
    /// knows about.
    pub fn catalog(&self) -> CatalogView {
        CatalogView { documents: self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.clone() }
    }

    /// @emoji 🩺 The frozen `health`: this `Database`'s `HealthRegistry` snapshot plus its own open
    /// document count.
    pub fn health(&self) -> DbHealth {
        DbHealth { report: self.health.report(), open_documents: self.open_documents.lock().expect("db_engine: open_documents mutex poisoned").len() }
    }

    /// @emoji 🚪 The frozen `shutdown`: gracefully joins every open `DocumentAuthority` this
    /// `Database` still exclusively owns.
    ///
    /// 🧩 Extension seam: `deadline` is currently advisory — `db_document::DocumentAuthority::shutdown`
    /// has no timeout parameter of its own (out of this crate's ownership to add this wave), so this
    /// always waits for a graceful join rather than forcing one after `deadline` elapses. A document
    /// whose `DocumentHandle` is still cloned elsewhere (this `Arc`'s strong count > 1) is skipped —
    /// its actor thread keeps running under whichever handle still holds it, which is correct
    /// (shutdown must never yank a mailbox out from under a live caller), just not exhaustive.
    pub fn shutdown(self, _deadline: std::time::Duration) -> Result<(), DbError> {
        let authorities: Vec<Arc<db_document::DocumentAuthority>> = self.open_documents.lock().expect("db_engine: open_documents mutex poisoned").drain().map(|(_, authority)| authority).collect();
        for authority in authorities {
            if let Ok(authority) = Arc::try_unwrap(authority) {
                authority.shutdown();
            }
        }
        self.emit.emit(db_core::EmitEvent::new("db_engine.database_shutdown"));
        Ok(())
    }

    /// @emoji 🧰 What this `Database` instance negotiated at `open` time.
    pub fn capabilities(&self) -> DbCapabilities {
        self.capabilities
    }

    /// @emoji 🔌 The underlying storage substrate this `Database` was opened with — an escape
    /// hatch for callers below the document-actor boundary that need direct `PayloadStorage`/
    /// `WalStorage` access (e.g. `os-hub`'s content-addressed blob routes, or a wire-v2 hub
    /// session driving `db_sync::handle_frontier_advertise` directly). Additive: not part of the
    /// contract-frozen `Database` API surface listed in `contract.md`'s "Stable API" block, so it
    /// carries no compatibility promise beyond this crate's own semver.
    pub fn storage(&self) -> Arc<dyn DbStorage> {
        self.storage.clone()
    }

    /// @emoji 🧹 A real, bounded `db_compact::Compactor` pass over `document` — WAL segment
    /// retention below its latest snapshot's `head_seq`, ref-traced payload GC, index compaction,
    /// and (if `consolidate_snapshots`) snapshot chain consolidation. See module doc: this IS a
    /// genuine `db_compact` integration, just document-at-a-time rather than a background scheduler
    /// (deferred — this wave's instructions ask for a lighter, documented cluster/compact/sync
    /// surface, not a full online scheduler).
    pub fn compact_document(&self, document: &protocol::DocumentId, holder: &str, consolidate_snapshots: bool) -> Result<db_compact::CompactionReport, DbError> {
        let core_document = to_core_document_id(document);
        db_compact::Compactor::new(self.storage.as_ref()).run_from_latest_snapshot(&core_document, holder, consolidate_snapshots, &db_compact::CompactionBudget::default(), now_ms())
    }

    /// @emoji 👋 A real `db_sync::handle_hello` call for `document` — the server-side half of the
    /// wire-v2 handshake (frontier exchange / bootstrap-plan decision). No transport of its own:
    /// wiring this to an actual `protocol_wire` socket is CW5/CW6's job (framework/sync, hub
    /// rebuilds), out of this crate's scope this wave.
    pub fn hello(
        &self,
        document: &protocol::DocumentId,
        hello_frontier: Option<&protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: &protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<db_sync::WelcomeResponse, DbError> {
        let core_document = to_core_document_id(document);
        db_sync::handle_hello(self.storage.as_ref(), core_document, hello_frontier, session_id, origin, snapshot_chunk_bytes)
    }

    /// @emoji 🌿 A real, `vcs`-backed checkpoint over every change `record_change` has recorded for
    /// `document` since its last checkpoint (see `db_document::DocumentEngine::submit`'s "vcs"
    /// pipeline stage, which calls `record_change` on every commit when a `VersionGraph` is wired).
    /// Errs `Unimplemented` if the `vcs` feature is disabled (no `VersionGraph` configured).
    pub fn checkpoint_document(&self, document: &protocol::DocumentId, message: String, authors: &[protocol::ActorId]) -> Result<String, DbError> {
        let core_document = to_core_document_id(document);
        let core_authors = authors.iter().map(to_core_actor_id).collect();
        self.version_graph.checkpoint(&core_document, db_core::CheckpointRequest { parent_checkpoint: None, change_ids: Vec::new(), message, authors: core_authors, timestamp_ms: now_ms() })
    }
}
//#endregion 🔖Database

//#region 🔖DocumentHandle
pub type SubmitFuture = db_actor::ReplyReceiver<Result<CommandReceipt, DbError>>;

/// @emoji 🎭 The frozen `DocumentHandle`: a clone-cheap live handle to one open document.
#[derive(Clone)]
pub struct DocumentHandle {
    authority: Arc<db_document::DocumentAuthority>,
    storage: Arc<dyn DbStorage>,
    document: protocol::DocumentId,
    core_document: db_core::DocumentId,
}

impl DocumentHandle {
    /// @emoji ✍️ The frozen `submit`: commits `batch` through the document's real
    /// `DocumentAuthority` mailbox. Returns immediately with a `SubmitFuture` rather than blocking
    /// the calling thread — see module doc's `//🎯 Design choice` on `SubmitFuture`: since
    /// `DocumentAuthority`'s only public submit entry point is the blocking `submit_blocking`, this
    /// bridges it to a real (not immediately-ready) `Future` by running the blocking call on a
    /// dedicated bridge thread and resolving a `db_actor` oneshot from it — the same
    /// `spawn_blocking`-over-a-channel pattern `tokio`/`pack_async` both use for the identical
    /// blocking-API-under-an-async-facade problem.
    pub fn submit(&self, batch: db_document::CommandBatch, options: db_document::SubmitOptions) -> SubmitFuture {
        let (reply_tx, reply_rx) = db_actor::oneshot();
        let authority = self.authority.clone();
        let document = self.document.clone();
        let submitted_at_ms = now_ms();
        std::thread::Builder::new()
            .name("db-engine-submit-bridge".to_string())
            .spawn(move || {
                let result = authority.submit_blocking(batch, options, submitted_at_ms).map(|receipt| to_engine_receipt(receipt, document));
                reply_tx.send(result);
            })
            .expect("db_engine: failed to spawn submit bridge thread");
        reply_rx
    }

    /// @emoji 🔎 The frozen `query`. `Consistency::Canonical` reads the document's live state
    /// directly. `AtLeast`/`Exact` read canonical too, then verify the resulting frontier actually
    /// satisfies the request (`DbError::Unavailable` if not — a true wait-for-frontier primitive
    /// would need a `DocumentMessage` variant `db_document`'s mailbox doesn't expose yet).
    /// `Historical`/`Speculative`/`PreviewAugmented` are `DbError::Unimplemented` — see module doc.
    // 🔒 `consistency`'s by-value signature is the frozen contract API
    // (`DocumentHandle::query(&self, query: Query, consistency: Consistency)`, contract.md's
    // "Stable API" block) — not changeable even though this revision's body only borrows it.
    #[allow(clippy::needless_pass_by_value)]
    pub fn query(&self, query: Query, consistency: Consistency) -> Result<QueryStream, DbError> {
        match &consistency {
            Consistency::Historical(_) | Consistency::PreviewAugmented(_) => {
                return Err(DbError::Unimplemented("historical/preview-augmented query consistency is not yet wired at the db_engine layer (db_query/db_projection integration deferred)"));
            }
            Consistency::Speculative(_) => {
                return Err(DbError::Unimplemented(
                    "speculative (preview) query consistency is not yet reachable: DocumentAuthority's mailbox only exposes Submit/Query/Frontier messages",
                ));
            }
            Consistency::Canonical | Consistency::AtLeast(_) | Consistency::Exact(_) => {}
        }

        let paths: Vec<String> = match query {
            Query::Get { path } => vec![path],
            Query::GetMany { paths } => paths,
        };
        let mut results = Vec::with_capacity(paths.len());
        for path in paths {
            let value = self.authority.query_blocking(&path)?;
            results.push((path, value));
        }

        let frontier = self.frontier()?;
        match &consistency {
            Consistency::AtLeast(requested) if !frontier.dominates(requested)? => {
                return Err(DbError::Unavailable("document has not yet reached the requested frontier".to_string()));
            }
            Consistency::Exact(requested) if &frontier != requested => {
                return Err(DbError::Unavailable("document frontier does not exactly match the requested frontier".to_string()));
            }
            _ => {}
        }
        Ok(QueryStream { results })
    }

    /// @emoji 📡 The frozen `subscribe` — see module doc's `//🎯 Design choice`: always
    /// `DbError::Unimplemented`, a real (not faked) extension seam pending a `DocumentMessage`
    /// variant `db_document` doesn't expose yet.
    pub fn subscribe(&self, _spec: LiveQuerySpec) -> Result<LiveQuery, DbError> {
        Err(DbError::Unimplemented("live-query subscription is not yet reachable: DocumentAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// @emoji 🧭 The frozen `frontier`.
    pub fn frontier(&self) -> Result<Frontier, DbError> {
        let core_frontier = self.authority.frontier_blocking()?;
        Ok(to_engine_frontier(&core_frontier, self.document.clone()))
    }

    /// @emoji 🌫️ The frozen `preview` — see `subscribe`'s doc; same deferral reason.
    pub fn preview(&self, _base: Frontier) -> Result<PreviewHandle, DbError> {
        Err(DbError::Unimplemented("preview publish/query is not yet reachable: DocumentAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// @emoji 📜 The frozen `history` — real, see module doc: replays the WAL directly rather than
    /// going through the actor.
    pub fn history(&self) -> Result<HistoryView, DbError> {
        replay_history(self.storage.as_ref(), &self.core_document, &self.document)
    }

    /// @emoji 📸 The frozen `snapshot_now` — see module doc's `//🎯 Design choice`: always resolves
    /// to `DbError::Unimplemented`, a real extension seam (no full-state enumeration exists yet to
    /// serialize, and `db_snapshot` is not a direct dependency of this crate).
    pub fn snapshot_now(&self, _kind: SnapshotKind) -> SnapshotFuture {
        let (reply_tx, reply_rx) = db_actor::oneshot();
        reply_tx.send(Err(DbError::Unimplemented(
            "db_engine does not yet build real pack snapshots (no db_snapshot dependency this wave, and DocumentState exposes no full-state enumeration to serialize)",
        )));
        reply_rx
    }

    pub fn document_id(&self) -> &protocol::DocumentId {
        &self.document
    }
}
//#endregion 🔖DocumentHandle

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸Fixtures
    fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("db_engine-test-{name}-{}-{}", std::process::id(), now_ms()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::DocumentId, entries: &[(&str, serde_json::Value)]) -> protocol::OperationEnvelope {
        let mut payload = serde_json::Map::new();
        for (path, value) in entries {
            payload.insert((*path).to_string(), value.clone());
        }
        protocol::OperationEnvelope {
            operation_id: protocol::OperationId(id.to_string()),
            document_id: document.clone(),
            actor: protocol::ActorId(actor.to_string()),
            dependencies: deps.iter().map(|dep| protocol::OperationId((*dep).to_string())).collect(),
            diff: protocol::DocumentDiff {
                schema: protocol::SchemaId(db_document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::Value::Object(payload)).unwrap(),
            },
            inverse: protocol::InverseOperation {
                schema: protocol::SchemaId(db_document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::Value::Object(serde_json::Map::new())).unwrap(),
            },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }
    //#endregion 🧸Fixtures

    //#region 🔖Database open/catalog
    #[test]
    fn open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog() {
        let root = tempdir("open-at-fresh");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        assert!(database.catalog().documents.is_empty());
        assert_eq!(database.health().open_documents, 0);
        assert!(matches!(database.health().report.overall, db_observe::HealthState::Healthy));
    }

    #[test]
    fn create_document_registers_it_in_the_catalog_and_document_finds_it() {
        let root = tempdir("create-and-find");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        database.create_document(DocumentSpec::new(document.clone())).unwrap();

        let catalog = database.catalog();
        assert_eq!(catalog.documents.len(), 1);
        assert_eq!(catalog.documents[0].document, document);

        let handle = database.document(&document).unwrap();
        assert_eq!(handle.document_id(), &document);
    }

    #[test]
    fn create_document_twice_errs_already_exists() {
        let root = tempdir("create-twice");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        database.create_document(DocumentSpec::new(document.clone())).unwrap();
        let result = database.create_document(DocumentSpec::new(document));
        assert!(matches!(result, Err(DbError::AlreadyExists(_))));
    }

    #[test]
    fn document_of_an_unknown_id_errs_not_found() {
        let root = tempdir("unknown-doc");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let result = database.document(&protocol::DocumentId("never-created".to_string()));
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }
    //#endregion 🔖Database open/catalog

    //#region 🔖Round trip
    #[test]
    fn full_submit_durable_query_round_trip_over_a_real_document_authority() {
        let root = tempdir("round-trip");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(DocumentSpec::new(document.clone())).unwrap();

        let batch = db_document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))])]).unwrap();
        let receipt = db_actor::block_on(handle.submit(batch, db_document::SubmitOptions { durability: DurabilityClass::Fsync })).unwrap().unwrap();
        assert_eq!(receipt.command_id, protocol::OperationId("op-1".to_string()));
        assert_eq!(receipt.frontier.document, document);
        assert_eq!(receipt.frontier.head_seq, 1);
        assert!(receipt.conflicts.is_empty());
        assert!(receipt.state_hash.is_some());

        let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).unwrap();
        let value: serde_json::Value = serde_json::from_slice(queried.results[0].1.as_ref().unwrap()).unwrap();
        assert_eq!(value, serde_json::json!("hello"));

        let frontier = handle.frontier().unwrap();
        assert_eq!(frontier.head_seq, 1);

        let at_least = handle.query(Query::Get { path: "name".to_string() }, Consistency::AtLeast(frontier)).unwrap();
        assert_eq!(at_least.results.len(), 1);

        let history = handle.history().unwrap();
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].operation_ids, vec![protocol::OperationId("op-1".to_string())]);
    }

    #[test]
    fn a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root() {
        let root = tempdir("reopen");
        let document = protocol::DocumentId("doc-1".to_string());
        {
            let database = Database::open_at(&root, Profile::Test).unwrap();
            let handle = database.create_document(DocumentSpec::new(document.clone())).unwrap();
            let batch = db_document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("count", serde_json::json!(1))])]).unwrap();
            db_actor::block_on(handle.submit(batch, db_document::SubmitOptions { durability: DurabilityClass::Fsync })).unwrap().unwrap();
            database.shutdown(std::time::Duration::from_secs(1)).unwrap();
        }

        let reopened = Database::open_at(&root, Profile::Test).unwrap();
        assert_eq!(reopened.catalog().documents.len(), 1, "the catalog root must have survived the reopen");

        let handle = reopened.document(&document).unwrap();
        let queried = handle.query(Query::Get { path: "count".to_string() }, Consistency::Canonical).unwrap();
        let value: serde_json::Value = serde_json::from_slice(queried.results[0].1.as_ref().unwrap()).unwrap();
        assert_eq!(value, serde_json::json!(1), "the document's committed state must have survived the reopen via WAL replay");
        assert_eq!(handle.frontier().unwrap().head_seq, 1);
    }

    #[test]
    fn exact_consistency_rejects_a_frontier_the_document_has_moved_past() {
        let root = tempdir("exact-consistency");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(DocumentSpec::new(document.clone())).unwrap();
        let stale = handle.frontier().unwrap();

        let batch = db_document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))])]).unwrap();
        db_actor::block_on(handle.submit(batch, db_document::SubmitOptions::default())).unwrap().unwrap();

        let result = handle.query(Query::Get { path: "x".to_string() }, Consistency::Exact(stale));
        assert!(matches!(result, Err(DbError::Unavailable(_))));
    }
    //#endregion 🔖Round trip

    //#region 🔖Deferred extension seams
    #[test]
    fn subscribe_preview_and_snapshot_now_are_documented_unimplemented_not_panics() {
        let root = tempdir("deferred");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(DocumentSpec::new(document)).unwrap();

        assert!(matches!(handle.subscribe(LiveQuerySpec { since: None }), Err(DbError::Unimplemented(_))));
        assert!(matches!(handle.preview(handle.frontier().unwrap()), Err(DbError::Unimplemented(_))));
        assert!(matches!(db_actor::block_on(handle.snapshot_now(SnapshotKind::Full)), Ok(Err(DbError::Unimplemented(_)))));
    }
    //#endregion 🔖Deferred extension seams

    //#region 🔖VersionGraph
    #[cfg(feature = "vcs")]
    #[test]
    fn checkpoint_document_mints_distinct_real_vcs_content_addressed_checkpoint_ids() {
        let root = tempdir("vcs-checkpoint");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(DocumentSpec::new(document.clone())).unwrap();

        let batch1 = db_document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))])]).unwrap();
        db_actor::block_on(handle.submit(batch1, db_document::SubmitOptions::default())).unwrap().unwrap();
        let checkpoint_1 = database.checkpoint_document(&document, "first".to_string(), &[protocol::ActorId("alice".to_string())]).unwrap();
        assert!(checkpoint_1.starts_with("ck-"), "vcs checkpoint ids are content-addressed as ck-<hex16>, got {checkpoint_1:?}");

        let batch2 = db_document::CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &document, &[("x", serde_json::json!(2))])]).unwrap();
        db_actor::block_on(handle.submit(batch2, db_document::SubmitOptions::default())).unwrap().unwrap();
        let checkpoint_2 = database.checkpoint_document(&document, "second".to_string(), &[protocol::ActorId("alice".to_string())]).unwrap();

        assert_ne!(checkpoint_1, checkpoint_2, "distinct commits must mint distinct content-addressed checkpoint ids");
    }

    #[cfg(not(feature = "vcs"))]
    #[test]
    fn checkpoint_document_errs_unimplemented_without_the_vcs_feature() {
        let root = tempdir("no-vcs-checkpoint");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        database.create_document(DocumentSpec::new(document.clone())).unwrap();
        assert!(matches!(database.checkpoint_document(&document, "msg".to_string(), &[]), Err(DbError::Unimplemented(_))));
    }
    //#endregion 🔖VersionGraph

    //#region 🔖Compact + Sync
    #[test]
    fn compact_document_runs_a_real_compaction_pass_without_error() {
        let root = tempdir("compact");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(DocumentSpec::new(document.clone())).unwrap();
        let batch = db_document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))])]).unwrap();
        db_actor::block_on(handle.submit(batch, db_document::SubmitOptions::default())).unwrap().unwrap();

        let report = database.compact_document(&document, "holder-1", false).unwrap();
        assert_eq!(report.wal_segments_deleted, 0, "nothing is below the (nonexistent) snapshot floor yet, but the pass itself must succeed");
    }

    #[test]
    fn hello_returns_a_welcome_with_a_fresh_bootstrap_for_a_brand_new_replica() {
        let root = tempdir("hello");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(DocumentSpec::new(document.clone())).unwrap();
        let batch = db_document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))])]).unwrap();
        db_actor::block_on(handle.submit(batch, db_document::SubmitOptions::default())).unwrap().unwrap();

        let response = database.hello(&document, None, "session-1".to_string(), &protocol::ActorId("hub".to_string()), 4096).unwrap();
        assert!(matches!(response.welcome, protocol::ServerFrame::Welcome { .. }));
    }

    // 🔬 `storage()` is a real escape hatch to the same backend `Database::open_at` wired — a
    // caller below the document-actor boundary (os-hub's blob routes) can round-trip a payload
    // through it directly, independent of any document actor.
    #[test]
    fn storage_accessor_reaches_the_same_backend_payload_store() {
        let root = tempdir("storage-accessor");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let hash = database.storage().payload().put(b"hello storage accessor").unwrap();
        assert_eq!(database.storage().payload().get(&hash).unwrap(), b"hello storage accessor");
    }
    //#endregion 🔖Compact + Sync

    //#region 🔖Security
    #[test]
    fn security_authz_hook_rejects_a_principal_denied_by_its_policy() {
        let policy = db_security::RoleBasedPolicy::new();
        let gate = db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 16), db_security::BudgetRegistry::new(100, 10), Arc::new(db_core::NullEmit));
        let hook = SecurityAuthzHook::new(gate, |actor| db_security::Principal::new(actor.clone(), db_security::TenantId::from("tenant-1"), vec!["viewer".to_string()]));

        let document = protocol::DocumentId("doc-1".to_string());
        let envelope = envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]);
        let result = db_document::AuthzHook::authorize(&hook, &envelope.actor, &envelope);
        assert!(matches!(result, Err(DbError::Unauthorized(_))), "a default-deny policy with no grants must reject every action");
    }
    //#endregion 🔖Security
}
//#endregion 🧪Tests
