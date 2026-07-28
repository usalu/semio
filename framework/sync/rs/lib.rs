//! 🔁 Local-first sync actor layer: a schema-agnostic per-document backbone actor that runs all IO
//! (persist, hub WebSocket sync, file watching) off the UI thread, plus the causal {@link SyncSession}
//! that feeds remote {@link OperationEnvelope}s into a document's vcs edit timeline.
//!
//! # Threading model
//! - **Native** (wgpu native host, tests): {@link DocumentHost::open} spawns a dedicated `std::thread`
//!   running a current-thread tokio runtime; the actor `select!`s over the store's outbound queue, a
//!   hub WebSocket, a `notify` file watcher, and reconnect/debounce timers.
//! - **Browser wgpu build** (`wasm32-unknown-unknown`): the actor runs on `wasm_bindgen_futures::
//!   spawn_local` with a `web_sys::WebSocket` hub transport (no threads, no filesystem). The
//!   production browser shell instead uses a TS twin (`backbone-worker.ts`, WS-E); this wasm actor
//!   keeps the crate coherent for a future in-wasm host.
//! - **WASI-P2 plugins never link this crate** — inside the sandbox a store attaches vcs's pure
//!   `PortBackbone` (an in-memory queue relayed to the host). This actor is a host-side concern only.

use protocol::{AckStage, ApplyOutcome, Bootstrap, ClientFrame, Lane, ServerFrame, decode_server_frame, encode_client_frame};
use semio_framework_core::{ActorId, DocumentDiff, DocumentVersion, InverseOperation, OperationEnvelope, OperationId, PayloadHash, PresencePeer, SchemaId, SchemaVersion, UndoPolicy};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::{broadcast, mpsc};
use vcs::{reconcile_alternative, BackboneMessage, ChannelBackbone, ChannelBackboneRemote, DocumentVcsStore, StudioConflict};

//#region 🔖Errors
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SyncError {
    #[error("vcs error: {0}")]
    Vcs(String),
    #[error("actor error: {0}")]
    Actor(String),
}
//#endregion 🔖Errors

//#region 🔖Protocol
/// @emoji 🗃️ A durable place a document synchronizes with. A document may bind to several at once
/// (folder-only, hub-only, or both); the actor treats each as an independent peer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PersistenceBinding {
    /// @emoji 📁 Local canonical store. A directory uses the multi-document `folder://` sqlite store;
    /// a `*.json` path uses the single-blob `file://` export format.
    Folder { path: std::path::PathBuf },
    /// @emoji ☁️ A hub node reachable over WebSocket
    /// (`remote://host:port` → `ws://host:port/studios/{studio_id}/documents/{id}/ws`).
    Hub {
        base_url: String,
        studio_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        token: Option<String>,
    },
}

/// @emoji 🧾 Everything {@link DocumentHost::open} needs to spawn one document's actor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentActorConfig {
    pub document_id: String,
    pub schema: String,
    pub bindings: Vec<PersistenceBinding>,
    /// @emoji 👁️ Watch the folder binding for external edits (other processes writing the file).
    #[serde(default)]
    pub watch_external: bool,
    /// @emoji 🖋️ The authoring actor id used for hub `Hello`/presence and operation origin filtering.
    pub actor: String,
}

/// @emoji 📨 Caller → actor control messages, sent on the {@link DocumentChannels} command channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentActorMsg {
    /// @emoji ⬆️ Wakes the actor to drain the store's outbound operations promptly. `envelopes` is a
    /// direct-injection fallback used only when no store is attached to the channel (empty = pure wake).
    LocalOperations { envelopes: Vec<OperationEnvelope> },
    /// @emoji 📸 Same as {@link LocalOperations} for a full-envelope snapshot (structural commands / seeding).
    LocalSnapshot { envelope_json: String },
    /// @emoji 📡 Broadcasts this peer's presence/selection to the hub.
    PresenceHeartbeat { peer: PresencePeer },
    /// @emoji 👻 Publishes an ephemeral, best-effort UI-state blob on the hub's uncredited preview
    /// lane (`protocol_wire::ClientFrame::PreviewPublish`) — e.g. a drag ghost or live cursor;
    /// `seq` is a per-`key` monotone counter so a receiver can drop stale-arriving previews.
    PublishPreview { key: String, seq: u64, payload: Vec<u8> },
    /// @emoji 🔄 Forces an immediate re-read + diff of the folder binding (test/manual poke hook).
    ExternalChanged,
    /// @emoji ✂️ Flushes any pending outbound operations, then stops the actor.
    Detach,
}

/// @emoji 📶 Connection state of a document's remote (hub) transport.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RemoteState {
    Detached,
    Connecting,
    Live { peer_count: usize },
    Backoff { retry_in_ms: u64 },
}

/// @emoji 🚦 Snapshot of a document's sync health for status badges.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSyncStatus {
    pub persisted: bool,
    pub pendingOperations: usize,
    pub remote: RemoteState,
}

impl Default for DocumentSyncStatus {
    fn default() -> Self {
        Self { persisted: false, pendingOperations: 0, remote: RemoteState::Detached }
    }
}

/// @emoji 📬 Actor → subscriber events, delivered on the broadcast channel from {@link DocumentHost::subscribe}.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentEvent {
    /// @emoji 🕸️ Remote operations (hub fan-out or appended external edits) — also pushed into the store's
    /// inbound queue so `store.tick()` materializes them.
    RemoteOperations { envelopes: Vec<OperationEnvelope> },
    /// @emoji 📸 The whole envelope was replaced (divergent external history / hub snapshot swap).
    SnapshotReplaced { envelope_json: String },
    /// @emoji 🚦 Sync status changed.
    Status(DocumentSyncStatus),
    /// @emoji 📡 The presence roster changed.
    Presence { peers: Vec<PresencePeer> },
    /// @emoji 👻 A peer published an ephemeral preview blob (`protocol_wire::ServerFrame::Preview`)
    /// on the uncredited, loss-tolerant preview lane — the counterpart of
    /// {@link DocumentActorMsg::PublishPreview}.
    Preview { actor: String, key: String, seq: u64, payload: Vec<u8> },
    /// @emoji 📮 The hub's terminal disposition for one outbound `Commands` batch
    /// (`protocol_wire::ServerFrame::Ack`'s `Applied` stage) — accepted as-is, transformed against
    /// concurrent history (the transformed envelope is already delivered as a
    /// {@link DocumentEvent::RemoteOperations} replacing the speculative local one), or rejected
    /// (the speculative local head is rolled back via {@link rollback_envelope} before this fires).
    CommandOutcome { batch_id: u64, outcome: CommandAckOutcome },
    /// @emoji ⚠️ A structural conflict (external divergence with local pending operations / hub CAS reject).
    Conflict(StudioConflict),
}

/// @emoji ⚖️ The client-side twin of `protocol_wire::ApplyOutcome`, minus the `Transformed`
/// envelope payload (already delivered separately as {@link DocumentEvent::RemoteOperations} by
/// the time this fires — see {@link DocumentEvent::CommandOutcome}).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CommandAckOutcome {
    Accepted,
    Transformed,
    Rejected { reason: String },
}
//#endregion 🔖Protocol

//#region 🔖Endpoints
// 🧱 Most `semio_framework_core` id/diff/inverse types this region and 🔖WireBridge below build
// envelopes from are imported once, unconditionally, at the top of the file (both the native
// folder-reconstruction path here and the cross-target wire bridge need them). `DocumentId` is the
// one exception: only this native-only region names it explicitly (the wire bridge never spells the
// type, just moves values through it), so it stays a native-only import to avoid an unused-import
// warning on the wasm32 build.
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_core::DocumentId;

/// @emoji 📇 Edit ids present in an envelope's `vcs.edits` array (schema-agnostic JSON read).
#[cfg(not(target_arch = "wasm32"))]
fn envelope_edit_ids(value: Option<&Value>) -> Vec<String> {
    value.and_then(|v| v.get("vcs")).and_then(|v| v.get("edits")).and_then(|e| e.as_array()).map(|edits| edits.iter().filter_map(|edit| edit.get("id").and_then(|id| id.as_str()).map(String::from)).collect()).unwrap_or_default()
}

/// @emoji 📜 The `vcs.edits` entries of an envelope, as raw JSON values.
#[cfg(not(target_arch = "wasm32"))]
fn envelope_edits(value: &Value) -> Vec<Value> {
    value.get("vcs").and_then(|v| v.get("edits")).and_then(|e| e.as_array()).cloned().unwrap_or_default()
}

/// @emoji 📦 Rebuilds an {@link OperationEnvelope} from a stored `Edit` JSON so an appended external edit can
/// flow through the store's causal DAG (`ingest_remote` → `edit_from_operation_envelope`). Mirrors vcs's
/// `operation_envelope_from_edit` field-for-field.
#[cfg(not(target_arch = "wasm32"))]
fn operation_envelope_from_stored_edit(schema: &str, document_id: &str, edit: Value) -> OperationEnvelope {
    let id = edit.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let actor = edit.get("actor").and_then(|v| v.as_str()).unwrap_or("external").to_string();
    let sequence = edit.get("sequenceNumber").and_then(|v| v.as_i64()).unwrap_or(0);
    let backwards = edit.get("backwards").cloned().unwrap_or_else(|| Value::Array(Vec::new()));
    let payload_hash = semio_framework_hash::hash_bytes(&serde_json::to_vec(&edit).unwrap_or_default());
    OperationEnvelope {
        id: OperationId(id.clone()),
        actor: ActorId(actor),
        document: DocumentId(document_id.to_string()),
        schema_version: SchemaVersion(schema.to_string()),
        deps: Vec::new(),
        payload_hash: PayloadHash(payload_hash),
        diff: DocumentDiff { schema_id: SchemaId(schema.to_string()), payload: edit },
        inverse: InverseOperation {
            target_operation: OperationId(id),
            inverse_diff: DocumentDiff { schema_id: SchemaId(schema.to_string()), payload: serde_json::json!({ "backwards": backwards }) },
            base_version: DocumentVersion(sequence.max(0) as u64),
            dependencies: Vec::new(),
            undo_policy: UndoPolicy::ExactBaseOnly,
        },
    }
}

/// @emoji 🔗 Derives a hub WebSocket URL: `remote://host:port` (or `http(s)://`, `ws(s)://`) →
/// `ws(s)://host:port/studios/{studio_id}/documents/{document_id}/ws`.
fn hub_ws_url(base_url: &str, studio_id: &str, document_id: &str) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map(|(_, rest)| rest).unwrap_or(base_url).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    format!("{scheme}://{authority}/studios/{studio_id}/documents/{document_id}/ws")
}
//#endregion 🔖Endpoints

//#region 🔖WireBridge
/// @emoji 🌉 Converts between this actor's local, schema-agnostic {@link OperationEnvelope}
/// (`semio_framework_core`, the shape `vcs::DocumentVcsStore`/`ChannelBackbone` speak) and
/// `protocol_causal::OperationEnvelope` (the shape `protocol_wire::ClientFrame::Commands`/
/// `ServerFrame::Commands` carry on the wire). `ActorId`/`DocumentId`/`OperationId` are literally
/// the same type on both sides (`semio_framework_core` re-exports them from `protocol_core`
/// verbatim — see that re-export's doc comment), so only `schema_version`/`payload_hash` (local-
/// only, recomputed on receipt exactly like {@link operation_envelope_from_stored_edit} already
/// does) and `inverse.{target_operation,base_version,dependencies,undo_policy}` (no wire
/// counterpart — `protocol_causal::InverseOperation` is deliberately simpler, see its frozen-
/// contract doc comment) need real bridging.
fn to_wire_envelope(envelope: &OperationEnvelope, timestamp: protocol::HybridLogicalTimestamp) -> protocol::OperationEnvelope {
    protocol::OperationEnvelope {
        operation_id: envelope.id.clone(),
        document_id: envelope.document.clone(),
        actor: envelope.actor.clone(),
        dependencies: envelope.deps.clone(),
        diff: protocol::DocumentDiff { schema: envelope.diff.schema_id.0.clone(), payload: envelope.diff.payload.clone() },
        inverse: protocol::InverseOperation { schema: envelope.inverse.inverse_diff.schema_id.0.clone(), inverse_diff: envelope.inverse.inverse_diff.payload.clone() },
        timestamp,
    }
}

/// @emoji 🌉 The inverse of {@link to_wire_envelope}: rebuilds a full local envelope from one that
/// crossed the wire, recomputing `payload_hash`/`base_version` the same way
/// {@link operation_envelope_from_stored_edit} does (this actor's payloads are always edit-shaped
/// JSON carrying their own `sequenceNumber`/`backwards`), and defaulting `undo_policy` to
/// `ExactBaseOnly` (the only policy this schema-agnostic actor ever assigns, mirrored from that
/// same function).
fn from_wire_envelope(envelope: protocol::OperationEnvelope) -> OperationEnvelope {
    let schema = envelope.diff.schema;
    let sequence = envelope.diff.payload.get("sequenceNumber").and_then(|value| value.as_i64()).unwrap_or(0).max(0) as u64;
    let payload_hash = semio_framework_hash::hash_bytes(&serde_json::to_vec(&envelope.diff.payload).unwrap_or_default());
    OperationEnvelope {
        id: envelope.operation_id.clone(),
        actor: envelope.actor,
        document: envelope.document_id,
        schema_version: SchemaVersion(schema.clone()),
        deps: envelope.dependencies,
        payload_hash: PayloadHash(payload_hash),
        diff: DocumentDiff { schema_id: SchemaId(schema), payload: envelope.diff.payload },
        inverse: InverseOperation {
            target_operation: envelope.operation_id,
            inverse_diff: DocumentDiff { schema_id: SchemaId(envelope.inverse.schema), payload: envelope.inverse.inverse_diff },
            base_version: DocumentVersion(sequence),
            dependencies: Vec::new(),
            undo_policy: UndoPolicy::ExactBaseOnly,
        },
    }
}

/// @emoji ↩️ Synthesizes a local "undo" envelope from a speculative envelope's own precomputed
/// `inverse`, so a hub `Ack::Applied::{Rejected,Transformed}` outcome can roll back (or replace)
/// the local speculative head without a second round trip. This actor stays `serde_json::Value`-
/// typed end to end (never touches `vcs`/`protocol_command`'s typed `Operation`/`OperationDiff`
/// trait machinery — see the crate doc), so "the inverse machinery" it uses is simply replaying the
/// envelope's own already-computed `InverseOperation` diff as a synthetic remote operation, the
/// same path {@link DocumentActor::deliver_remote_operations} already uses for any other remote edit.
fn rollback_envelope(envelope: &OperationEnvelope) -> OperationEnvelope {
    let undo_id = OperationId(format!("{}~undo", envelope.id.0));
    OperationEnvelope {
        id: undo_id.clone(),
        actor: envelope.actor.clone(),
        document: envelope.document.clone(),
        schema_version: envelope.schema_version.clone(),
        deps: vec![envelope.id.clone()],
        payload_hash: PayloadHash(semio_framework_hash::hash_bytes(&serde_json::to_vec(&envelope.inverse.inverse_diff.payload).unwrap_or_default())),
        diff: envelope.inverse.inverse_diff.clone(),
        inverse: InverseOperation { target_operation: undo_id, inverse_diff: envelope.diff.clone(), base_version: envelope.inverse.base_version, dependencies: Vec::new(), undo_policy: envelope.inverse.undo_policy },
    }
}

/// @emoji 📡 `PresencePeer` -> the schema-erased JSON `protocol_wire::ClientFrame::Presence` carries.
fn presence_to_json(peer: &PresencePeer) -> Value {
    serde_json::to_value(peer).unwrap_or(Value::Null)
}

/// @emoji 📡 The inverse of {@link presence_to_json}, for `ServerFrame::Presence`'s peer roster.
fn presence_from_json(value: &Value) -> Option<PresencePeer> {
    serde_json::from_value(value.clone()).ok()
}

/// @emoji ⏰ Millisecond wall-clock reads for {@link next_timestamp}: `SystemTime` natively,
/// `js_sys::Date` in the browser wasm build (no `SystemTime` there).
#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// @emoji 🧮 A stable, deterministic `u64` seed for an actor id string, for
/// `protocol::HybridLogicalTimestamp::actor` (which is `u64`-shaped; this actor's own id is a
/// free-form `String`).
fn actor_seed(actor: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    actor.hash(&mut hasher);
    hasher.finish()
}

/// @emoji ⏰ Advances `counter` and stamps a fresh {@link protocol::HybridLogicalTimestamp} for an
/// outbound wire envelope. Wire-only metadata: `semio_framework_core::OperationEnvelope` carries no
/// timestamp field, so this never needs to round-trip back through {@link from_wire_envelope}.
fn next_timestamp(seed: u64, counter: &mut u64) -> protocol::HybridLogicalTimestamp {
    *counter = counter.wrapping_add(1);
    protocol::HybridLogicalTimestamp { actor: seed, physical_ms: now_ms(), logical: *counter }
}
//#endregion 🔖WireBridge

//#region 🔖SyncSession
/// @emoji 🔁 Pairs a document's vcs store with the causal DAG that reconciles remote envelopes into
/// it. Extended into the actor world via {@link SyncSession::attach}: it holds the actor command
/// channel and event stream, drains status on {@link SyncSession::tick}, and delegates store IO.
pub struct SyncSession<P, Operation>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Operation: Clone + serde::Serialize + serde::de::DeserializeOwned + vcs::Operation<P>,
{
    pub store: DocumentVcsStore<P, Operation>,
    cmd_tx: Option<mpsc::UnboundedSender<DocumentActorMsg>>,
    events: Option<broadcast::Receiver<DocumentEvent>>,
    status: DocumentSyncStatus,
}

impl<P, Operation> SyncSession<P, Operation>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Operation: Clone + serde::Serialize + serde::de::DeserializeOwned + vcs::Operation<P>,
{
    pub fn new(store: DocumentVcsStore<P, Operation>) -> Self {
        Self { store, cmd_tx: None, events: None, status: DocumentSyncStatus::default() }
    }

    /// @emoji 🔌 Attaches this session's store to a document actor: the actor's `ChannelBackbone` end
    /// is wired into the store, and the command/event channels are retained for wake + status.
    pub fn attach(&mut self, channels: DocumentChannels, events: broadcast::Receiver<DocumentEvent>) -> Result<(), SyncError> {
        self.store.attach_backbone(Box::new(channels.channel_backbone)).map_err(|error| SyncError::Vcs(error.to_string()))?;
        self.cmd_tx = Some(channels.cmd_tx);
        self.events = Some(events);
        Ok(())
    }

    /// @emoji ✂️ Detaches from the actor (asking it to flush + stop) and unbinds the store's backbone.
    pub fn detach(&mut self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(DocumentActorMsg::Detach);
        }
        self.store.detach_backbone();
        self.cmd_tx = None;
        self.events = None;
    }

    /// @emoji 🔔 Nudges the actor to drain the store's outbound queue without waiting for its poll tick.
    pub fn wake(&self) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(DocumentActorMsg::LocalOperations { envelopes: Vec::new() });
        }
    }

    /// @emoji 👻 Publishes an ephemeral preview blob on the hub's preview lane. See
    /// {@link DocumentActorMsg::PublishPreview}.
    pub fn publish_preview(&self, key: String, seq: u64, payload: Vec<u8>) {
        if let Some(cmd_tx) = &self.cmd_tx {
            let _ = cmd_tx.send(DocumentActorMsg::PublishPreview { key, seq, payload });
        }
    }

    /// @emoji 📥 Drains any buffered sync status, then pumps the store's inbound backbone queue into
    /// the edit timeline (delegating to `store.tick()`/`pump()`).
    pub fn tick(&mut self) -> Result<bool, SyncError> {
        if let Some(events) = &mut self.events {
            while let Ok(event) = events.try_recv() {
                if let DocumentEvent::Status(status) = &event {
                    self.status = status.clone();
                }
            }
        }
        self.store.tick().map_err(|error| SyncError::Vcs(error.to_string()))
    }

    /// @emoji 🚦 The latest sync status seen on the event stream (updated by {@link SyncSession::tick}).
    pub fn status(&self) -> DocumentSyncStatus {
        self.status.clone()
    }

    /// @emoji 🕸️ Feeds a remote envelope through the store's causal DAG, materializing it (and any
    /// now-unblocked dependents) into the edit timeline. Kept for direct/test injection.
    pub fn receive(&mut self, envelope: semio_framework_core::OperationEnvelope) -> Result<(), SyncError> {
        self.store.ingest_remote(envelope).map_err(|error| SyncError::Vcs(error.to_string()))
    }

    pub fn reconcile_branch(&mut self, alternative_name: &str, message: Option<String>, authors: Vec<vcs::Author>) -> Result<String, SyncError> {
        let mut envelope = self.store.envelope().clone();
        let alternative_id = reconcile_alternative(&mut envelope, alternative_name, message, authors).map_err(|error| SyncError::Vcs(error.to_string()))?;
        let applied = self.store.applied_edit_ids().to_vec();
        let redo = self.store.redo_edit_ids().to_vec();
        self.store.set_state(envelope, applied, redo);
        Ok(alternative_id)
    }
}
//#endregion 🔖SyncSession

//#region 🔖Host
/// @emoji 🎛️ The channels {@link DocumentHost::open} hands back to a caller: attach `channel_backbone`
/// to your `DocumentVcsStore`, and send control messages (or wakes) on `cmd_tx`.
pub struct DocumentChannels {
    pub cmd_tx: mpsc::UnboundedSender<DocumentActorMsg>,
    /// @emoji 🔗 The store-side backbone end. The caller owns store attachment:
    /// `store.attach_backbone(Box::new(channels.channel_backbone))`.
    pub channel_backbone: ChannelBackbone,
}

struct OpenDocument {
    cmd_tx: mpsc::UnboundedSender<DocumentActorMsg>,
    events: broadcast::Sender<DocumentEvent>,
    #[cfg(not(target_arch = "wasm32"))]
    join: Option<std::thread::JoinHandle<()>>,
}

/// @emoji 🏛️ Registry of open per-document actors. One `DocumentHost` per host process (wgpu native,
/// tests, or the browser wgpu build) owns every open document's actor + event fan-out.
#[derive(Clone, Default)]
pub struct DocumentHost {
    inner: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, OpenDocument>>>,
}

impl DocumentHost {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji 🚀 Spawns (or replaces) the actor for `config.document_id` and returns the channels the
    /// caller wires into its store. Idempotent per id: opening an already-open id closes the old actor.
    pub fn open(&self, config: DocumentActorConfig) -> DocumentChannels {
        let document_id = config.document_id.clone();
        self.close(&document_id);
        let (channel_backbone, remote) = ChannelBackbone::pair(&format!("actor://{document_id}"));
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (event_tx, _event_rx) = broadcast::channel(256);
        #[cfg(not(target_arch = "wasm32"))]
        let join = spawn_actor(config, remote, cmd_rx, event_tx.clone());
        #[cfg(target_arch = "wasm32")]
        spawn_actor(config, remote, cmd_rx, event_tx.clone());
        let entry = OpenDocument {
            cmd_tx: cmd_tx.clone(),
            events: event_tx,
            #[cfg(not(target_arch = "wasm32"))]
            join,
        };
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(document_id, entry);
        DocumentChannels { cmd_tx, channel_backbone }
    }

    /// @emoji 📬 A fresh event receiver for `document_id`. If the document is not open the receiver's
    /// sender is dropped, so it simply reports closed.
    pub fn subscribe(&self, document_id: &str) -> broadcast::Receiver<DocumentEvent> {
        let guard = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match guard.get(document_id) {
            Some(document) => document.events.subscribe(),
            None => {
                let (_tx, rx) = broadcast::channel(1);
                rx
            }
        }
    }

    /// @emoji 🔔 Sends a control message to a document's actor (e.g. a presence heartbeat or a wake).
    pub fn send(&self, document_id: &str, message: DocumentActorMsg) {
        if let Some(document) = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(document_id) {
            let _ = document.cmd_tx.send(message);
        }
    }

    /// @emoji ✂️ Stops a document's actor (flushing pending outbound operations first) and, on native, joins
    /// its thread.
    pub fn close(&self, document_id: &str) {
        let document = self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(document_id);
        if let Some(document) = document {
            let _ = document.cmd_tx.send(DocumentActorMsg::Detach);
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(join) = document.join {
                let _ = join.join();
            }
        }
    }

    /// @emoji 🧹 Ids of every currently-open document.
    pub fn open_documents(&self) -> Vec<String> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).keys().cloned().collect()
    }
}

impl Drop for DocumentHost {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.inner) > 1 {
            return;
        }
        for document_id in self.open_documents() {
            self.close(&document_id);
        }
    }
}
//#endregion 🔖Host

//#region 🔖NativeActor
#[cfg(not(target_arch = "wasm32"))]
mod native_actor {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};
    use std::time::Duration;
    use tokio::time::Instant;
    use tokio_tungstenite::tungstenite::Message;

    type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
    type WsSink = futures_util::stream::SplitSink<WsStream, Message>;
    type WsRead = futures_util::stream::SplitStream<WsStream>;

    struct HubConn {
        write: WsSink,
        read: WsRead,
    }

    /// @emoji 📁 A folder/file binding's storage driver, keyed for multi-document sqlite or single
    /// pack-backed blob. `Pack` (was `Text`) stores the actor's whole-envelope JSON as a REAL pack
    /// file + real `.ops` text + a DSL mirror via `vcs::FolderTextStorage::write_pack`/`read_pack` —
    /// this actor stays `serde_json::Value`-typed end to end (it drains/relays generic backbone
    /// messages, never a concrete `Projection`/`Operation`), so the `schema` string is how it reaches a
    /// concrete codec: `vcs::document_codec(schema)` looks up the `vcs::DocumentCodec` a real app
    /// registered (`register_document_codec_for_app`, wave 2) and does the pack↔envelope-JSON bridging
    /// on this actor's behalf. Replaces the old bug where `.dsl` held a raw envelope-JSON dump and
    /// `.ops` was always written empty (see `read`/`write` below — a missing codec is now a hard
    /// error, never a silent JSON-in-`.dsl` fallback).
    enum FolderEndpoint {
        Sqlite { storage: vcs::FolderSqliteStorage, document_id: String, schema: String },
        Pack { storage: vcs::FolderTextStorage, document_id: String, extension: String, schema: String },
    }

    impl FolderEndpoint {
        /// @emoji 📖 `Ok(None)` = nothing persisted yet; `Ok(Some(json))` = the envelope, resolved
        /// pack-first (falling back to the DSL mirror for hand-authored/imported documents with no
        /// `.pack` file yet); `Err` = a real failure (missing codec registration for `schema`, or a
        /// pack/dsl decode error) — never silently degrades to a raw dump.
        fn read(&self) -> Result<Option<String>, String> {
            match self {
                FolderEndpoint::Sqlite { storage, document_id, .. } => storage.read(document_id).map_err(|error| error.to_string()),
                FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                    let Some(codec) = vcs::document_codec(schema) else {
                        return Err(format!("no document codec registered for schema {schema:?}"));
                    };
                    if let Some(pack_files) = storage.read_pack(document_id, extension).map_err(|error| error.to_string())? {
                        return (codec.parse)(&pack_files.pack, &pack_files.ops).map(Some).map_err(|error| error.to_string());
                    }
                    match storage.read(document_id, extension).map_err(|error| error.to_string())? {
                        Some(text_files) => (codec.parse_dsl)(&text_files.dsl, &text_files.ops).map(Some).map_err(|error| error.to_string()),
                        None => Ok(None),
                    }
                }
            }
        }

        /// @emoji ✍️ Persists `json` (the whole envelope). `Err` on a missing codec, same hard-error
        /// rule as `read`.
        fn write(&self, json: &str) -> Result<(), String> {
            match self {
                FolderEndpoint::Sqlite { storage, document_id, schema } => storage.write(document_id, schema, json).map_err(|error| error.to_string()),
                FolderEndpoint::Pack { storage, document_id, extension, schema } => {
                    let Some(codec) = vcs::document_codec(schema) else {
                        return Err(format!("no document codec registered for schema {schema:?}"));
                    };
                    let (pack_files, dsl_mirror) = (codec.print)(json).map_err(|error| error.to_string())?;
                    storage.write_pack(document_id, extension, &pack_files, &dsl_mirror).map_err(|error| error.to_string())
                }
            }
        }
    }

    /// @emoji 🎭 One document's backbone actor: drains the store's outbound queue to persist + relay,
    /// ingests hub/file changes back into the store, and keeps subscribers current with status/events.
    pub(super) struct DocumentActor {
        document_id: String,
        schema: String,
        actor: String,
        remote: ChannelBackboneRemote,
        events: broadcast::Sender<DocumentEvent>,
        cmd_rx: mpsc::UnboundedReceiver<DocumentActorMsg>,
        folder: Option<FolderEndpoint>,
        folder_watch_path: Option<PathBuf>,
        watch_external: bool,
        hub_base_url: Option<String>,
        hub_studio_id: Option<String>,
        hub_token: Option<String>,
        hub: Option<HubConn>,
        /// @emoji 🏔️ Last frontier the hub reported (`Welcome.server_frontier` / `Commands.frontier` /
        /// `Ack.frontier`) — the wire-v2 replacement for the old `hub_version: i64` counter.
        server_frontier: Option<protocol::RuntimeFrontierSummary>,
        /// @emoji 🎟️ The hub's last `Welcome.resume_token`, echoed back on the next `Hello` after a
        /// reconnect so the hub can resume rather than replay from scratch.
        resume_token: Option<String>,
        backoff_ms: u64,
        reconnect_at: Option<Instant>,
        /// @emoji 🧺 Outbound `Commands` batches awaiting an `Ack`, keyed by `batch_id`, so `Rejected`/
        /// `Transformed` can roll back exactly the envelopes that batch sent.
        pending_batches: std::collections::HashMap<u64, Vec<OperationEnvelope>>,
        next_batch_id: u64,
        /// @emoji ⏰ This actor's `HybridLogicalTimestamp` seed (derived from `actor`) + logical tick
        /// counter, for {@link next_timestamp} on every outbound wire envelope.
        hlc_seed: u64,
        hlc_counter: u64,
        current_envelope: Option<Value>,
        known_edit_ids: HashSet<String>,
        last_written_hash: Option<String>,
        remote_state: RemoteState,
        last_status: Option<DocumentSyncStatus>,
        watcher: Option<notify::RecommendedWatcher>,
        fs_rx: Option<mpsc::UnboundedReceiver<()>>,
        fs_deadline: Option<Instant>,
    }

    impl DocumentActor {
        pub(super) fn new(config: DocumentActorConfig, remote: ChannelBackboneRemote, cmd_rx: mpsc::UnboundedReceiver<DocumentActorMsg>, events: broadcast::Sender<DocumentEvent>) -> Self {
            let mut folder = None;
            let mut folder_watch_path = None;
            let mut hub_base_url = None;
            let mut hub_studio_id = None;
            let mut hub_token = None;
            for binding in &config.bindings {
                match binding {
                    PersistenceBinding::Folder { path } => {
                        if folder.is_none() {
                            folder = Some(build_folder_endpoint(path, &config.document_id, &config.schema));
                            folder_watch_path = Some(folder_watch_path_for(path));
                        }
                    }
                    PersistenceBinding::Hub { base_url, studio_id, token } => {
                        if hub_base_url.is_none() {
                            hub_base_url = Some(base_url.clone());
                            hub_studio_id = Some(studio_id.clone());
                            hub_token = token.clone();
                        }
                    }
                }
            }
            let hlc_seed = actor_seed(&config.actor);
            Self {
                document_id: config.document_id,
                schema: config.schema,
                actor: config.actor,
                remote,
                events,
                cmd_rx,
                folder,
                folder_watch_path,
                watch_external: config.watch_external,
                hub_base_url,
                hub_studio_id,
                hub_token,
                hub: None,
                server_frontier: None,
                resume_token: None,
                backoff_ms: 500,
                reconnect_at: None,
                pending_batches: std::collections::HashMap::new(),
                next_batch_id: 0,
                hlc_seed,
                hlc_counter: 0,
                current_envelope: None,
                known_edit_ids: HashSet::new(),
                last_written_hash: None,
                remote_state: RemoteState::Detached,
                last_status: None,
                watcher: None,
                fs_rx: None,
                fs_deadline: None,
            }
        }

        pub(super) async fn run(mut self) {
            self.setup();
            self.try_connect_hub().await;
            let mut poll = tokio::time::interval(Duration::from_millis(25));
            poll.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                let reconnect_at = self.reconnect_at;
                let fs_deadline = self.fs_deadline;
                tokio::select! {
                    biased;
                    cmd = self.cmd_rx.recv() => {
                        match cmd {
                            None => { break; }
                            Some(message) => {
                                if self.handle_cmd(message).await {
                                    break;
                                }
                            }
                        }
                    }
                    message = hub_next(&mut self.hub), if self.hub.is_some() => {
                        self.on_hub_message(message).await;
                    }
                    changed = fs_next(&mut self.fs_rx), if self.fs_rx.is_some() => {
                        if changed.is_some() {
                            self.fs_deadline = Some(Instant::now() + Duration::from_millis(200));
                        }
                    }
                    _ = sleep_opt(reconnect_at), if reconnect_at.is_some() => {
                        self.reconnect_at = None;
                        self.try_connect_hub().await;
                    }
                    _ = sleep_opt(fs_deadline), if fs_deadline.is_some() => {
                        self.fs_deadline = None;
                        self.handle_external_change();
                    }
                    _ = poll.tick() => {
                        self.drain_and_relay().await;
                        self.emit_status_if_changed();
                    }
                }
            }
        }

        /// @emoji 🌱 Seeds persistence state from any already-stored envelope and installs the file watcher.
        fn setup(&mut self) {
            if let Some(json) = self.folder.as_ref().and_then(|folder| folder.read().ok().flatten()) {
                if let Ok(value) = serde_json::from_str::<Value>(&json) {
                    self.known_edit_ids = envelope_edit_ids(Some(&value)).into_iter().collect();
                    self.current_envelope = Some(value);
                    self.last_written_hash = Some(semio_framework_hash::hash_bytes(json.as_bytes()));
                }
            }
            if self.watch_external {
                if let Some(watch_path) = self.folder_watch_path.clone() {
                    if let Some((watcher, fs_rx)) = install_watcher(&watch_path) {
                        self.watcher = Some(watcher);
                        self.fs_rx = Some(fs_rx);
                    }
                }
            }
        }

        /// @emoji 📨 Handles a caller control message. Returns `true` when the actor should stop.
        async fn handle_cmd(&mut self, message: DocumentActorMsg) -> bool {
            match message {
                DocumentActorMsg::LocalOperations { envelopes } => {
                    let drained = self.drain_and_relay().await;
                    if !drained && !envelopes.is_empty() {
                        self.persist_operations(&envelopes);
                        self.relay_operations_to_hub(&envelopes).await;
                    }
                    false
                }
                DocumentActorMsg::LocalSnapshot { envelope_json } => {
                    let drained = self.drain_and_relay().await;
                    if !drained {
                        self.persist_snapshot(&envelope_json);
                    }
                    false
                }
                DocumentActorMsg::PresenceHeartbeat { peer } => {
                    self.send_client_frame(ClientFrame::Presence { peer: presence_to_json(&peer) }, Lane::Preview).await;
                    false
                }
                DocumentActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_client_frame(ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                    false
                }
                DocumentActorMsg::ExternalChanged => {
                    self.handle_external_change();
                    false
                }
                DocumentActorMsg::Detach => {
                    self.drain_and_relay().await;
                    true
                }
            }
        }

        /// @emoji 📤 Drains the store's outbound queue, persisting + relaying each message. Returns
        /// whether anything was drained.
        async fn drain_and_relay(&mut self) -> bool {
            let messages = self.remote.drain().unwrap_or_default();
            let drained = !messages.is_empty();
            for message in messages {
                match message {
                    BackboneMessage::Operations { envelopes } => {
                        self.persist_operations(&envelopes);
                        self.relay_operations_to_hub(&envelopes).await;
                    }
                    BackboneMessage::Snapshot { envelope_json } => {
                        self.persist_snapshot(&envelope_json);
                        // 📸 No client -> hub whole-envelope push exists in wire v2
                        // (`protocol_wire::ClientFrame` has no snapshot-put variant — only
                        // causally-ordered `Commands`; the hub -> client snapshot direction is
                        // `Bootstrap::Snapshot`/`SnapshotChunk`/`SnapshotDone`, download-only). The
                        // folder binding above still persists this snapshot; relaying a structural
                        // snapshot to the hub is a CW6+ hub-rebuild concern (documented deferral in
                        // the CW5 report, not a bug in this actor).
                    }
                    BackboneMessage::Ack { .. } => {}
                }
            }
            drained
        }

        //#region 🔖Folder
        /// @emoji ✍️ Persists the current envelope JSON to the folder binding and records the content
        /// hash for self-write suppression. A write failure (e.g. no `vcs::DocumentCodec` registered
        /// for this document's schema — see `FolderEndpoint::write`) is swallowed here the same way
        /// every other best-effort path in this actor already is, but deliberately does NOT record
        /// `last_written_hash` on failure — a false "persisted" mark would make `handle_external_change`
        /// mistake the still-stale on-disk content for a self-write and ignore a real external change.
        fn persist_write(&mut self, json: &str) {
            let Some(folder) = self.folder.as_ref() else { return };
            if folder.write(json).is_ok() {
                self.last_written_hash = Some(semio_framework_hash::hash_bytes(json.as_bytes()));
            }
        }

        /// @emoji 📸 Records a full-envelope snapshot as the canonical persisted state.
        fn persist_snapshot(&mut self, envelope_json: &str) {
            if self.folder.is_none() {
                return;
            }
            if let Ok(value) = serde_json::from_str::<Value>(envelope_json) {
                self.known_edit_ids = envelope_edit_ids(Some(&value)).into_iter().collect();
                self.current_envelope = Some(value);
            }
            self.persist_write(envelope_json);
        }

        /// @emoji ➕ Appends locally-applied operations to the persisted envelope's `vcs.edits` (append-only),
        /// keeping the on-disk copy coherent so self-writes are never mistaken for external edits.
        fn persist_operations(&mut self, envelopes: &[OperationEnvelope]) {
            if self.folder.is_none() {
                return;
            }
            let Some(mut value) = self.current_envelope.clone() else { return };
            if let Some(edits) = value.get_mut("vcs").and_then(|vcs| vcs.get_mut("edits")).and_then(|edits| edits.as_array_mut()) {
                for envelope in envelopes {
                    if self.known_edit_ids.insert(envelope.id.0.clone()) {
                        edits.push(envelope.diff.payload.clone());
                    }
                }
            }
            let json = serde_json::to_string(&value).unwrap_or_default();
            self.current_envelope = Some(value);
            self.persist_write(&json);
        }

        /// @emoji 👁️ Re-reads the folder binding and classifies the change: append-only → `RemoteOperations`,
        /// divergence → `SnapshotReplaced`, divergence with local pending operations → `Conflict`. Self-writes
        /// (content hash match) are ignored.
        fn handle_external_change(&mut self) {
            let Some(json) = self.folder.as_ref().and_then(|folder| folder.read().ok().flatten()) else { return };
            let hash = semio_framework_hash::hash_bytes(json.as_bytes());
            if self.last_written_hash.as_deref() == Some(hash.as_str()) {
                return;
            }
            let Ok(value) = serde_json::from_str::<Value>(&json) else { return };
            let file_ids: HashSet<String> = envelope_edit_ids(Some(&value)).into_iter().collect();
            let lost: Vec<String> = self.known_edit_ids.difference(&file_ids).cloned().collect();
            let new_ids: HashSet<String> = file_ids.difference(&self.known_edit_ids).cloned().collect();

            if lost.is_empty() && !new_ids.is_empty() {
                let appended: Vec<OperationEnvelope> =
                    envelope_edits(&value).into_iter().filter(|edit| edit.get("id").and_then(|id| id.as_str()).map(|id| new_ids.contains(id)).unwrap_or(false)).map(|edit| operation_envelope_from_stored_edit(&self.schema, &self.document_id, edit)).collect();
                self.known_edit_ids.extend(new_ids);
                self.current_envelope = Some(value);
                self.last_written_hash = Some(hash);
                self.deliver_remote_operations(appended);
            } else if !lost.is_empty() {
                if !self.pending_batches.is_empty() {
                    self.emit(DocumentEvent::Conflict(StudioConflict { kind: "externalDivergence".into(), uri: format!("folder://{}", self.document_id), message: "external history diverged while local operations are pending".into() }));
                } else {
                    self.known_edit_ids = file_ids;
                    self.current_envelope = Some(value);
                    self.last_written_hash = Some(hash);
                    self.deliver_snapshot(json);
                }
            }
        }
        //#endregion 🔖Folder

        //#region 🔖Hub
        async fn try_connect_hub(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else { return };
            let studio_id = self.hub_studio_id.clone().unwrap_or_default();
            let token = self.hub_token.clone();
            let url = hub_ws_url(&base_url, &studio_id, &self.document_id);
            self.set_remote_state(RemoteState::Connecting);
            match tokio_tungstenite::connect_async(url).await {
                Ok((stream, _response)) => {
                    let (write, read) = stream.split();
                    self.hub = Some(HubConn { write, read });
                    self.backoff_ms = 500;
                    let hello = ClientFrame::Hello {
                        wire_version: 1,
                        protocol_version: 1,
                        schema: self.schema.clone(),
                        // 🔖 No schema pack hashing wired into this client-side actor yet (db/pack
                        // integration is a CW6+ hub-rebuild concern) — the hub is JSON-only until
                        // then anyway, so this placeholder is never validated this wave.
                        pack_schema_hash: [0u8; 32],
                        actor: ActorId(self.actor.clone()),
                        token,
                        resume_token: self.resume_token.clone(),
                        frontier: self.server_frontier.clone(),
                    };
                    self.send_client_frame(hello, Lane::Command).await;
                }
                Err(_error) => {
                    self.schedule_reconnect();
                }
            }
        }

        fn schedule_reconnect(&mut self) {
            let retry = self.backoff_ms;
            self.set_remote_state(RemoteState::Backoff { retry_in_ms: retry });
            self.reconnect_at = Some(Instant::now() + Duration::from_millis(retry));
            self.backoff_ms = (self.backoff_ms * 2).min(30_000);
        }

        async fn on_hub_message(&mut self, message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>) {
            match message {
                Some(Ok(Message::Binary(bytes))) => {
                    if let Ok((_lane, frame)) = decode_server_frame(&bytes) {
                        self.on_hub_frame(frame);
                    }
                }
                Some(Ok(Message::Ping(payload))) => {
                    self.send_raw(Message::Pong(payload)).await;
                }
                Some(Ok(_)) => {}
                Some(Err(_)) | None => {
                    self.hub = None;
                    self.schedule_reconnect();
                }
            }
        }

        fn on_hub_frame(&mut self, frame: ServerFrame) {
            match frame {
                ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                    self.resume_token = Some(resume_token);
                    self.server_frontier = Some(server_frontier);
                    // 📡 `Welcome` no longer carries a presence roster (wire v2 splits it into its own
                    // `ServerFrame::Presence`) — `peer_count` is corrected once that frame arrives.
                    self.set_remote_state(RemoteState::Live { peer_count: 0 });
                    match bootstrap {
                        Bootstrap::None | Bootstrap::Tail => {}
                        Bootstrap::Snapshot { .. } => {
                            // 📦 Pack-based snapshot bootstrap: no client-side pack decoder wired into
                            // this actor this wave (db/pack integration is a CW6+ hub-rebuild concern)
                            // — accepted and ignored rather than erroring; catch-up instead relies on
                            // the hub's follow-up `Commands` frame(s) once CW6 lands.
                        }
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {
                    // 📦 See the `Bootstrap::Snapshot` note above — accepted and ignored.
                }
                ServerFrame::Commands { envelopes, origin, frontier } => {
                    self.server_frontier = Some(frontier);
                    if origin != ActorId(self.actor.clone()) {
                        let converted: Vec<OperationEnvelope> = envelopes.into_iter().map(from_wire_envelope).collect();
                        self.persist_operations(&converted);
                        self.deliver_remote_operations(converted);
                    }
                }
                ServerFrame::Ack { batch_id, stages, frontier } => {
                    self.server_frontier = Some(frontier);
                    self.handle_ack(batch_id, stages);
                }
                ServerFrame::Preview { actor, key, seq, payload } => {
                    if actor != ActorId(self.actor.clone()) {
                        self.emit(DocumentEvent::Preview { actor: actor.0, key, seq, payload });
                    }
                }
                ServerFrame::Presence { peers } => {
                    let peers: Vec<PresencePeer> = peers.iter().filter_map(presence_from_json).collect();
                    self.set_remote_state(RemoteState::Live { peer_count: peers.len() });
                    self.emit(DocumentEvent::Presence { peers });
                }
                ServerFrame::CreditGrant { .. } => {
                    // 🪙 Command-lane credit-based flow control: no client-side backpressure
                    // implemented this wave (scope is frame plumbing, not congestion control) —
                    // accepted and ignored.
                }
                ServerFrame::Error { code, message } => {
                    self.emit(DocumentEvent::Conflict(StudioConflict { kind: code, uri: self.hub_base_url.clone().unwrap_or_default(), message }));
                }
            }
        }

        /// @emoji 📮 Resolves one outbound `Commands` batch's terminal `Applied` stage: `Accepted`
        /// just clears the pending batch; `Transformed`/`Rejected` both roll back the speculative
        /// local head first (via {@link rollback_envelope}, replayed as remote operations), and
        /// `Transformed` then delivers the hub's replacement envelope the same way.
        fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
            for stage in stages {
                let AckStage::Applied { outcome } = stage else { continue };
                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                match *outcome {
                    ApplyOutcome::Accepted => {
                        self.emit(DocumentEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                    }
                    ApplyOutcome::Transformed { envelope } => {
                        let rollbacks: Vec<OperationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.persist_operations(&rollbacks);
                        self.deliver_remote_operations(rollbacks);
                        let converted = from_wire_envelope(*envelope);
                        self.persist_operations(std::slice::from_ref(&converted));
                        self.deliver_remote_operations(vec![converted]);
                        self.emit(DocumentEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    }
                    ApplyOutcome::Rejected { reason } => {
                        let rollbacks: Vec<OperationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.persist_operations(&rollbacks);
                        self.deliver_remote_operations(rollbacks);
                        self.emit(DocumentEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Rejected { reason } });
                    }
                }
            }
            self.emit_status_if_changed();
        }

        async fn relay_operations_to_hub(&mut self, envelopes: &[OperationEnvelope]) {
            if self.hub.is_none() || envelopes.is_empty() {
                return;
            }
            let batch_id = self.next_batch_id;
            self.next_batch_id = self.next_batch_id.wrapping_add(1);
            let wire_envelopes: Vec<protocol::OperationEnvelope> = envelopes.iter().map(|envelope| to_wire_envelope(envelope, next_timestamp(self.hlc_seed, &mut self.hlc_counter))).collect();
            self.pending_batches.insert(batch_id, envelopes.to_vec());
            self.send_client_frame(ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command).await;
            self.emit_status_if_changed();
        }

        async fn send_client_frame(&mut self, frame: ClientFrame, lane: Lane) {
            let bytes = encode_client_frame(&frame, lane);
            self.send_raw(Message::Binary(bytes.into())).await;
        }

        async fn send_raw(&mut self, message: Message) {
            let mut failed = false;
            if let Some(conn) = self.hub.as_mut() {
                if conn.write.send(message).await.is_err() {
                    failed = true;
                }
            }
            if failed {
                self.hub = None;
                self.schedule_reconnect();
            }
        }
        //#endregion 🔖Hub

        //#region 🔖Deliver
        /// @emoji 🕸️ Pushes remote operations into the store's inbound queue and notifies subscribers.
        fn deliver_remote_operations(&mut self, envelopes: Vec<OperationEnvelope>) {
            if envelopes.is_empty() {
                return;
            }
            let _ = self.remote.push(BackboneMessage::Operations { envelopes: envelopes.clone() });
            self.emit(DocumentEvent::RemoteOperations { envelopes });
        }

        /// @emoji 📸 Pushes a full-envelope snapshot into the store's inbound queue and notifies subscribers.
        fn deliver_snapshot(&mut self, envelope_json: String) {
            let _ = self.remote.push(BackboneMessage::Snapshot { envelope_json: envelope_json.clone() });
            self.emit(DocumentEvent::SnapshotReplaced { envelope_json });
        }

        fn emit(&self, event: DocumentEvent) {
            let _ = self.events.send(event);
        }

        fn status(&self) -> DocumentSyncStatus {
            DocumentSyncStatus { persisted: self.last_written_hash.is_some() || self.server_frontier.is_some(), pendingOperations: self.pending_batches.values().map(Vec::len).sum(), remote: self.remote_state.clone() }
        }

        fn set_remote_state(&mut self, state: RemoteState) {
            self.remote_state = state;
            self.emit_status_if_changed();
        }

        fn emit_status_if_changed(&mut self) {
            let status = self.status();
            if self.last_status.as_ref() != Some(&status) {
                self.last_status = Some(status.clone());
                self.emit(DocumentEvent::Status(status));
            }
        }
        //#endregion 🔖Deliver
    }

    /// @emoji 🔀 A binding path with a file extension addresses one document's text blob directly
    /// (`Text`, generalizing the deleted single-file `FileJsonStorage` beyond `.json`); an extensionless
    /// directory path is the canonical multi-document sqlite store (`Sqlite`).
    fn build_folder_endpoint(path: &std::path::Path, document_id: &str, schema: &str) -> FolderEndpoint {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(extension) => {
                let folder = path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
                FolderEndpoint::Pack {
                    storage: vcs::FolderTextStorage::new(folder),
                    document_id: document_id.to_string(),
                    extension: extension.to_string(),
                    schema: schema.to_string(),
                }
            }
            None => FolderEndpoint::Sqlite { storage: vcs::FolderSqliteStorage::new(path.to_path_buf()), document_id: document_id.to_string(), schema: schema.to_string() },
        }
    }

    /// @emoji 📍 The on-disk path a folder binding writes to: the `<document_id>.<extension>` text blob
    /// itself, or the multi-document sqlite db under `<folder>/.semio/documents.db`.
    fn folder_watch_path_for(path: &Path) -> PathBuf {
        if path.extension().is_some() {
            path.to_path_buf()
        } else {
            path.join(".semio").join("documents.db")
        }
    }

    /// @emoji 👁️ Installs a `notify` watcher over the binding's on-disk directory, forwarding raw
    /// change events into an async channel (debounced by the actor's 200ms deadline).
    fn install_watcher(watch_path: &Path) -> Option<(notify::RecommendedWatcher, mpsc::UnboundedReceiver<()>)> {
        use notify::Watcher;
        let watch_root = watch_path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| watch_path.to_path_buf());
        let _ = std::fs::create_dir_all(&watch_root);
        let (tx, rx) = mpsc::unbounded_channel();
        let mut watcher = notify::recommended_watcher(move |result: Result<notify::Event, notify::Error>| {
            if result.is_ok() {
                let _ = tx.send(());
            }
        })
        .ok()?;
        watcher.watch(&watch_root, notify::RecursiveMode::NonRecursive).ok()?;
        Some((watcher, rx))
    }

    async fn hub_next(conn: &mut Option<HubConn>) -> Option<Result<Message, tokio_tungstenite::tungstenite::Error>> {
        match conn {
            Some(conn) => conn.read.next().await,
            None => std::future::pending().await,
        }
    }

    async fn fs_next(rx: &mut Option<mpsc::UnboundedReceiver<()>>) -> Option<()> {
        match rx {
            Some(rx) => rx.recv().await,
            None => std::future::pending().await,
        }
    }

    async fn sleep_opt(deadline: Option<Instant>) {
        match deadline {
            Some(deadline) => tokio::time::sleep_until(deadline).await,
            None => std::future::pending().await,
        }
    }

    /// @emoji 🚀 Spawns a dedicated OS thread running a current-thread tokio runtime that drives the actor.
    pub(super) fn spawn_actor(config: DocumentActorConfig, remote: ChannelBackboneRemote, cmd_rx: mpsc::UnboundedReceiver<DocumentActorMsg>, events: broadcast::Sender<DocumentEvent>) -> Option<std::thread::JoinHandle<()>> {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().ok()?;
        std::thread::Builder::new()
            .name(format!("sync-actor-{}", config.document_id))
            .spawn(move || {
                let actor = DocumentActor::new(config, remote, cmd_rx, events);
                runtime.block_on(actor.run());
            })
            .ok()
    }
}

#[cfg(not(target_arch = "wasm32"))]
use native_actor::spawn_actor;
//#endregion 🔖NativeActor

//#region 🔖WasmActor
/// @emoji 🌐 Browser wgpu build: the actor runs on `spawn_local` with a `web_sys::WebSocket` hub
/// transport. No filesystem, so folder bindings are ignored (the browser uses the dev-middleware
/// SSE watch instead, wired by WS-E's TS twin). Kept coherent so a future in-wasm host can link it.
#[cfg(target_arch = "wasm32")]
mod wasm_actor {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{BinaryType, MessageEvent, WebSocket};

    struct WasmActor {
        document_id: String,
        schema: String,
        actor: String,
        remote: ChannelBackboneRemote,
        events: broadcast::Sender<DocumentEvent>,
        hub_base_url: Option<String>,
        hub_studio_id: Option<String>,
        hub_token: Option<String>,
        ws: Option<WebSocket>,
        server_frontier: Option<protocol::RuntimeFrontierSummary>,
        resume_token: Option<String>,
        pending_batches: std::collections::HashMap<u64, Vec<OperationEnvelope>>,
        next_batch_id: u64,
        hlc_seed: u64,
        hlc_counter: u64,
        incoming_tx: mpsc::UnboundedSender<Vec<u8>>,
        _closures: Vec<Closure<dyn FnMut(MessageEvent)>>,
        _open_closures: Vec<Closure<dyn FnMut()>>,
    }

    impl WasmActor {
        fn connect(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else { return };
            let studio_id = self.hub_studio_id.clone().unwrap_or_default();
            let url = hub_ws_url(&base_url, &studio_id, &self.document_id);
            let Ok(ws) = WebSocket::new(&url) else { return };
            ws.set_binary_type(BinaryType::Arraybuffer);

            let incoming = self.incoming_tx.clone();
            let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
                if let Some(buffer) = event.data().dyn_ref::<js_sys::ArrayBuffer>() {
                    let _ = incoming.send(js_sys::Uint8Array::new(buffer).to_vec());
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            self._closures.push(onmessage);

            let hello = ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: self.schema.clone(),
                // 🔖 See the native actor's matching note in `try_connect_hub` — no client-side
                // schema pack hashing wired this wave, the hub is JSON-only until CW6 anyway.
                pack_schema_hash: [0u8; 32],
                actor: ActorId(self.actor.clone()),
                token: self.hub_token.clone(),
                resume_token: self.resume_token.clone(),
                frontier: self.server_frontier.clone(),
            };
            let mut hello_bytes = encode_client_frame(&hello, Lane::Command);
            let ws_for_open = ws.clone();
            let onopen = Closure::wrap(Box::new(move || {
                let _ = ws_for_open.send_with_u8_array(&mut hello_bytes);
            }) as Box<dyn FnMut()>);
            ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            self._open_closures.push(onopen);

            self.ws = Some(ws);
        }

        fn send_frame(&self, frame: &ClientFrame, lane: Lane) {
            if let Some(ws) = &self.ws {
                let mut bytes = encode_client_frame(frame, lane);
                let _ = ws.send_with_u8_array(&mut bytes);
            }
        }

        /// @emoji 🧺 Builds + sends one `Commands` batch, tracking it in `pending_batches` for
        /// {@link WasmActor::handle_ack}. Mirrors the native actor's `relay_operations_to_hub`.
        fn relay_operations(&mut self, envelopes: &[OperationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
            let batch_id = self.next_batch_id;
            self.next_batch_id = self.next_batch_id.wrapping_add(1);
            let wire_envelopes: Vec<protocol::OperationEnvelope> = envelopes.iter().map(|envelope| to_wire_envelope(envelope, next_timestamp(self.hlc_seed, &mut self.hlc_counter))).collect();
            self.pending_batches.insert(batch_id, envelopes.to_vec());
            self.send_frame(&ClientFrame::Commands { batch_id, envelopes: wire_envelopes }, Lane::Command);
        }

        fn drain_and_relay(&mut self) -> bool {
            let messages = self.remote.drain().unwrap_or_default();
            let drained = !messages.is_empty();
            for message in messages {
                match message {
                    BackboneMessage::Operations { envelopes } => {
                        self.relay_operations(&envelopes);
                    }
                    BackboneMessage::Snapshot { .. } => {
                        // 📸 No client -> hub whole-envelope push in wire v2 — see the native actor's
                        // matching note in `drain_and_relay` (native_actor module, above).
                    }
                    BackboneMessage::Ack { .. } => {}
                }
            }
            drained
        }

        fn handle_cmd(&mut self, message: DocumentActorMsg) {
            match message {
                DocumentActorMsg::LocalOperations { envelopes } => {
                    let drained = self.drain_and_relay();
                    if !drained && !envelopes.is_empty() {
                        self.relay_operations(&envelopes);
                    }
                }
                DocumentActorMsg::LocalSnapshot { .. } => {
                    self.drain_and_relay();
                }
                DocumentActorMsg::PresenceHeartbeat { peer } => {
                    self.send_frame(&ClientFrame::Presence { peer: presence_to_json(&peer) }, Lane::Preview);
                }
                DocumentActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_frame(&ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview);
                }
                DocumentActorMsg::ExternalChanged | DocumentActorMsg::Detach => {}
            }
        }

        fn on_binary(&mut self, bytes: &[u8]) {
            let Ok((_lane, frame)) = decode_server_frame(bytes) else { return };
            match frame {
                ServerFrame::Welcome { session_id: _, resume_token, server_frontier, bootstrap } => {
                    self.resume_token = Some(resume_token);
                    self.server_frontier = Some(server_frontier);
                    match bootstrap {
                        Bootstrap::None | Bootstrap::Tail => {}
                        // 📦 See the native actor's matching `Bootstrap::Snapshot` note — no
                        // client-side pack decoder wired this wave, accepted and ignored.
                        Bootstrap::Snapshot { .. } => {}
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {}
                ServerFrame::Commands { envelopes, origin, frontier } => {
                    self.server_frontier = Some(frontier);
                    if origin != ActorId(self.actor.clone()) {
                        let converted: Vec<OperationEnvelope> = envelopes.into_iter().map(from_wire_envelope).collect();
                        self.deliver_remote_operations(converted);
                    }
                }
                ServerFrame::Ack { batch_id, stages, frontier } => {
                    self.server_frontier = Some(frontier);
                    self.handle_ack(batch_id, stages);
                }
                ServerFrame::Preview { actor, key, seq, payload } => {
                    if actor != ActorId(self.actor.clone()) {
                        let _ = self.events.send(DocumentEvent::Preview { actor: actor.0, key, seq, payload });
                    }
                }
                ServerFrame::Presence { peers } => {
                    let peers: Vec<PresencePeer> = peers.iter().filter_map(presence_from_json).collect();
                    let _ = self.events.send(DocumentEvent::Presence { peers });
                }
                ServerFrame::CreditGrant { .. } => {}
                ServerFrame::Error { code, message } => {
                    let _ = self.events.send(DocumentEvent::Conflict(StudioConflict { kind: code, uri: self.hub_base_url.clone().unwrap_or_default(), message }));
                }
            }
        }

        /// @emoji 📮 Mirrors the native actor's `handle_ack` — see its doc comment.
        fn handle_ack(&mut self, batch_id: u64, stages: Vec<AckStage>) {
            for stage in stages {
                let AckStage::Applied { outcome } = stage else { continue };
                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                match *outcome {
                    ApplyOutcome::Accepted => {
                        let _ = self.events.send(DocumentEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Accepted });
                    }
                    ApplyOutcome::Transformed { envelope } => {
                        let rollbacks: Vec<OperationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.deliver_remote_operations(rollbacks);
                        let converted = from_wire_envelope(*envelope);
                        self.deliver_remote_operations(vec![converted]);
                        let _ = self.events.send(DocumentEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Transformed });
                    }
                    ApplyOutcome::Rejected { reason } => {
                        let rollbacks: Vec<OperationEnvelope> = sent.iter().rev().map(rollback_envelope).collect();
                        self.deliver_remote_operations(rollbacks);
                        let _ = self.events.send(DocumentEvent::CommandOutcome { batch_id, outcome: CommandAckOutcome::Rejected { reason } });
                    }
                }
            }
        }

        fn deliver_remote_operations(&self, envelopes: Vec<OperationEnvelope>) {
            if envelopes.is_empty() {
                return;
            }
            let _ = self.remote.push(BackboneMessage::Operations { envelopes: envelopes.clone() });
            let _ = self.events.send(DocumentEvent::RemoteOperations { envelopes });
        }
    }

    pub(super) fn spawn_actor(config: DocumentActorConfig, remote: ChannelBackboneRemote, mut cmd_rx: mpsc::UnboundedReceiver<DocumentActorMsg>, events: broadcast::Sender<DocumentEvent>) {
        let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let mut hub_base_url = None;
        let mut hub_studio_id = None;
        let mut hub_token = None;
        for binding in &config.bindings {
            if let PersistenceBinding::Hub { base_url, studio_id, token } = binding {
                if hub_base_url.is_none() {
                    hub_base_url = Some(base_url.clone());
                    hub_studio_id = Some(studio_id.clone());
                    hub_token = token.clone();
                }
            }
        }
        let hlc_seed = actor_seed(&config.actor);
        let mut actor = WasmActor {
            document_id: config.document_id,
            schema: config.schema,
            actor: config.actor,
            remote,
            events,
            hub_base_url,
            hub_studio_id,
            hub_token,
            ws: None,
            server_frontier: None,
            resume_token: None,
            pending_batches: std::collections::HashMap::new(),
            next_batch_id: 0,
            hlc_seed,
            hlc_counter: 0,
            incoming_tx,
            _closures: Vec::new(),
            _open_closures: Vec::new(),
        };
        wasm_bindgen_futures::spawn_local(async move {
            actor.connect();
            loop {
                tokio::select! {
                    cmd = cmd_rx.recv() => {
                        match cmd {
                            None => break,
                            Some(DocumentActorMsg::Detach) => { actor.drain_and_relay(); break; }
                            Some(message) => actor.handle_cmd(message),
                        }
                    }
                    bytes = incoming_rx.recv() => {
                        match bytes {
                            Some(bytes) => actor.on_binary(&bytes),
                            None => break,
                        }
                    }
                }
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_actor::spawn_actor;
//#endregion 🔖WasmActor

//#region 🔖Fixtures
/// @emoji 🎬 A scripted actor test vector shared by cargo test (here) and vitest (WS-E's TS twin).
/// Each fixture drives inbound events at a document actor and asserts the resulting `DocumentEvent`
/// sequence and the final persisted envelope edit ids. See `framework/sync/fixtures/README.md`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorFixture {
    pub name: String,
    pub schema: String,
    pub document_id: String,
    /// @emoji 📥 Inbound stimulus applied to the actor, in order.
    pub inbound: Vec<FixtureInbound>,
    /// @emoji 📤 The `DocumentEvent` variant tags expected on the subscriber channel, in order.
    pub expected_events: Vec<String>,
    /// @emoji 📇 Edit ids expected in the document's timeline after replay.
    pub expected_edit_ids: Vec<String>,
}

/// @emoji 📥 One scripted inbound stimulus: either a hub server frame or an external folder edit.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FixtureInbound {
    /// @emoji 📬 A raw `protocol_wire::ServerFrame` delivered as if received over the hub
    /// WebSocket. Driven by `backbone-worker.ts`'s TS fallback vitest harness; the folder-only
    /// Rust harness skips these.
    HubFrame { frame: ServerFrame },
    /// @emoji 📁 An external folder edit: append these edit JSON objects to `vcs.edits` out-of-band.
    ExternalEdits { edits: Vec<Value> },
    /// @emoji ♻️ An external whole-envelope rewrite (divergent history): replace the stored envelope.
    ReplaceEnvelope { envelope: Value },
}

/// @emoji 📂 Loads every `*.json` fixture from `framework/sync/fixtures/`.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_fixtures(dir: &std::path::Path) -> Vec<ActorFixture> {
    let mut fixtures = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return fixtures };
    let mut paths: Vec<std::path::PathBuf> = entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json")).collect();
    paths.sort();
    for path in paths {
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(fixture) = serde_json::from_str::<ActorFixture>(&text) {
                fixtures.push(fixture);
            }
        }
    }
    fixtures
}
//#endregion 🔖Fixtures

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use vcs::{create_document_vcs_envelope, operation_envelope_from_edit, Edit, Operation, OperationDiff};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoProjection {
        n: i32,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl OperationDiff<DemoProjection> for DemoDiff {
        fn apply(&self, projection: &DemoProjection) -> DemoProjection {
            DemoProjection { n: self.n.unwrap_or(projection.n) }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "operation")]
    enum DemoOperation {
        SetN { n: i32 },
    }

    impl Operation<DemoProjection> for DemoOperation {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                DemoOperation::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![DemoOperation::SetN { n: projection.n }]
        }
    }

    fn sample_operation_envelope(edit_id: &str, n: i32) -> semio_framework_core::OperationEnvelope {
        let edit = Edit {
            id: edit_id.into(),
            actor: None,
            forwards: vec![DemoOperation::SetN { n }],
            backwards: vec![DemoOperation::SetN { n: 0 }],
            operation_meta: Vec::new(),
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "0".into(),
            finished_at: None,
        };
        let placeholder: vcs::DocumentVcsEnvelope<DemoProjection, DemoOperation> = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        operation_envelope_from_edit(&placeholder, &edit, Vec::new()).expect("operation envelope")
    }

    //#region 🧪SyncSession
    #[test]
    fn receive_materializes_remote_envelope_into_the_edit_timeline() {
        let envelope: vcs::DocumentVcsEnvelope<DemoProjection, DemoOperation> = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let store = DocumentVcsStore::new(envelope);
        let mut session = SyncSession::new(store);
        session.receive(sample_operation_envelope("edit-1", 5)).expect("receive");
        assert_eq!(session.store.projection().expect("projection").n, 5);
        assert_eq!(session.store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    fn receive_buffers_out_of_order_envelopes_until_dependencies_arrive() {
        let envelope: vcs::DocumentVcsEnvelope<DemoProjection, DemoOperation> = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let store = DocumentVcsStore::new(envelope);
        let mut session = SyncSession::new(store);
        let mut second = sample_operation_envelope("edit-2", 9);
        second.deps = vec![semio_framework_core::OperationId("edit-1".into())];
        session.receive(second).expect("receive second first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 0, "buffered until edit-1 arrives");
        session.receive(sample_operation_envelope("edit-1", 5)).expect("receive first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 2, "both edits now applied");
        assert_eq!(session.store.projection().expect("projection").n, 9);
    }
    //#endregion 🧪SyncSession

    //#region 🧪Helpers
    #[test]
    fn hub_ws_url_derives_ws_endpoint_from_remote_uri() {
        assert_eq!(hub_ws_url("remote://host:6070", "studio-1", "doc-1"), "ws://host:6070/studios/studio-1/documents/doc-1/ws");
        assert_eq!(hub_ws_url("https://hub.example.com", "studio-1", "doc-2"), "wss://hub.example.com/studios/studio-1/documents/doc-2/ws");
        assert_eq!(hub_ws_url("ws://127.0.0.1:5000/prefix", "studio-1", "d"), "ws://127.0.0.1:5000/studios/studio-1/documents/d/ws");
    }
    //#endregion 🧪Helpers

    //#region 🧪WireBridge
    #[test]
    fn wire_bridge_round_trips_identity_and_diff_through_protocol_causal() {
        let envelope = sample_operation_envelope("edit-1", 5);
        let wire = to_wire_envelope(&envelope, protocol::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 });
        assert_eq!(wire.operation_id, envelope.id);
        assert_eq!(wire.actor, envelope.actor);
        assert_eq!(wire.document_id, envelope.document);
        assert_eq!(wire.diff.payload, envelope.diff.payload);

        let recovered = from_wire_envelope(wire);
        assert_eq!(recovered.id, envelope.id);
        assert_eq!(recovered.actor, envelope.actor);
        assert_eq!(recovered.document, envelope.document);
        assert_eq!(recovered.diff.payload, envelope.diff.payload);
        assert_eq!(recovered.inverse.inverse_diff.payload, envelope.inverse.inverse_diff.payload);
    }

    #[test]
    fn rollback_envelope_synthesizes_an_undo_from_the_original_inverse() {
        let envelope = sample_operation_envelope("edit-1", 5);
        let rollback = rollback_envelope(&envelope);
        assert_eq!(rollback.deps, vec![envelope.id.clone()], "the undo depends on the operation it undoes");
        assert_eq!(rollback.diff.payload, envelope.inverse.inverse_diff.payload, "the undo's forward diff IS the original's inverse");
        assert_ne!(rollback.id, envelope.id, "the undo gets its own operation id");
    }

    /// @emoji 🎬 Canonical wire-frame byte fixtures shared with `backbone-worker.ts`'s vitest suite
    /// (`framework/product/os/core/js/backbone-worker.ts` `WireBridge` region / `index.ts`'s
    /// `encodeClientFrame`/`decodeServerFrame` twins) — both sides decode the exact same committed
    /// bytes under `framework/sync/fixtures/wire/`, proving `protocol_wire`'s lane+varint+JSON codec
    /// round-trips identically across Rust and TS. Regenerated deterministically by this test (every
    /// value below is a fixed constant, never a clock/random read) rather than hand-authored, so a
    /// `protocol_wire` field-order/shape change fails loudly here instead of silently diverging from
    /// the TS twin.
    #[test]
    fn wire_fixtures_stay_byte_identical_across_rust_and_ts() {
        let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/wire");
        std::fs::create_dir_all(&fixtures_dir).expect("fixtures dir");

        let hello = ClientFrame::Hello {
            wire_version: 1,
            protocol_version: 1,
            schema: "demo/v1".to_string(),
            pack_schema_hash: [7u8; 32],
            actor: protocol::ActorId("actor-1".to_string()),
            token: Some("token-1".to_string()),
            resume_token: None,
            frontier: None,
        };
        let hello_bytes = encode_client_frame(&hello, Lane::Command);
        std::fs::write(fixtures_dir.join("client-hello.bin"), &hello_bytes).expect("write client-hello.bin");
        let (lane, decoded) = protocol::decode_client_frame(&hello_bytes).expect("decode client-hello.bin");
        assert_eq!(lane, Lane::Command);
        assert_eq!(decoded, hello);

        let wire_envelope = protocol::OperationEnvelope {
            operation_id: protocol::OperationId("op-1".to_string()),
            document_id: protocol::DocumentId("doc-1".to_string()),
            actor: protocol::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff { schema: "demo/v1".to_string(), payload: serde_json::json!({"n": 5, "sequenceNumber": 1}) },
            inverse: protocol::InverseOperation { schema: "demo/v1".to_string(), inverse_diff: serde_json::json!({"n": 0}) },
            timestamp: protocol::HybridLogicalTimestamp { actor: 42, physical_ms: 1000, logical: 0 },
        };
        let commands = ClientFrame::Commands { batch_id: 1, envelopes: vec![wire_envelope] };
        let commands_bytes = encode_client_frame(&commands, Lane::Command);
        std::fs::write(fixtures_dir.join("client-commands.bin"), &commands_bytes).expect("write client-commands.bin");
        let (lane, decoded) = protocol::decode_client_frame(&commands_bytes).expect("decode client-commands.bin");
        assert_eq!(lane, Lane::Command);
        assert_eq!(decoded, commands);

        let frontier = protocol::RuntimeFrontierSummary { document_id: protocol::DocumentId("doc-1".to_string()), head_edit_ordinal: 1, head_edit_id: "op-1".to_string(), last_commit_seq: 1, chain_hash: [9u8; 32] };
        let welcome = ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail };
        let welcome_bytes = protocol::encode_server_frame(&welcome, Lane::Command);
        std::fs::write(fixtures_dir.join("server-welcome.bin"), &welcome_bytes).expect("write server-welcome.bin");
        let (lane, decoded) = decode_server_frame(&welcome_bytes).expect("decode server-welcome.bin");
        assert_eq!(lane, Lane::Command);
        assert_eq!(decoded, welcome);

        let ack = ServerFrame::Ack { batch_id: 1, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier };
        let ack_bytes = protocol::encode_server_frame(&ack, Lane::Command);
        std::fs::write(fixtures_dir.join("server-ack.bin"), &ack_bytes).expect("write server-ack.bin");
        let (lane, decoded) = decode_server_frame(&ack_bytes).expect("decode server-ack.bin");
        assert_eq!(lane, Lane::Command);
        assert_eq!(decoded, ack);
    }
    //#endregion 🧪WireBridge

    //#region 🧪Helpers

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn op_envelope_from_stored_edit_round_trips_through_ingest() {
        let edit_json = serde_json::json!({
            "id": "ext-1",
            "actor": "peer",
            "forwards": [{ "operation": "SetN", "n": 42 }],
            "backwards": [{ "operation": "SetN", "n": 0 }],
            "sequenceNumber": 3,
            "startedAt": "0"
        });
        let envelope = operation_envelope_from_stored_edit("demo/v1", "demo", edit_json);
        assert_eq!(envelope.id.0, "ext-1");
        let recovered: Edit<DemoOperation> = serde_json::from_value(envelope.diff.payload.clone()).expect("recover edit");
        assert_eq!(recovered.forwards, vec![DemoOperation::SetN { n: 42 }]);
    }
    //#endregion 🧪Helpers

    //#region 🧪Actor
    #[cfg(not(target_arch = "wasm32"))]
    mod actor_tests {
        use super::*;
        use futures_util::{SinkExt, StreamExt};
        use protocol::{decode_client_frame, encode_server_frame};
        use std::sync::Arc;
        use std::time::Duration;
        use tokio::sync::{broadcast as tokio_broadcast, Mutex};
        use tokio_tungstenite::tungstenite::Message as WsMessage;

        fn demo_envelope(document_id: &str) -> vcs::DocumentVcsEnvelope<DemoProjection, DemoOperation> {
            create_document_vcs_envelope("demo/v1", document_id, DemoProjection { n: 0 }, None)
        }

        async fn wait_for_event(events: &mut broadcast::Receiver<DocumentEvent>, mut predicate: impl FnMut(&DocumentEvent) -> bool) -> DocumentEvent {
            let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
            loop {
                match tokio::time::timeout_at(deadline, events.recv()).await {
                    Ok(Ok(event)) => {
                        if predicate(&event) {
                            return event;
                        }
                    }
                    Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => continue,
                    other => panic!("no matching event before deadline: {other:?}"),
                }
            }
        }

        // 🔬 External folder edit → RemoteOperations event + the store timeline grows on tick().
        #[tokio::test]
        async fn folder_external_edit_delivers_remote_operations() {
            let dir = tempfile::tempdir().expect("tempdir");
            let host = DocumentHost::new();
            let channels = host.open(DocumentActorConfig { document_id: "doc-a".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() });
            let mut events = host.subscribe("doc-a");
            let mut store = DocumentVcsStore::new(demo_envelope("doc-a"));
            store.attach_backbone(Box::new(channels.channel_backbone)).expect("attach");

            // A local apply establishes a persisted edit on disk.
            store.dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply");
            channels.cmd_tx.send(DocumentActorMsg::LocalOperations { envelopes: Vec::new() }).expect("wake");

            // Wait until the actor has persisted the local edit to the folder db.
            let storage = vcs::FolderSqliteStorage::new(dir.path().to_path_buf());
            let stored = loop {
                if let Some(json) = storage.read("doc-a").expect("read") {
                    if json.contains("\"edits\":[{") {
                        break json;
                    }
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            };

            // Out-of-band: append a foreign edit directly to the stored envelope.
            let mut value: serde_json::Value = serde_json::from_str(&stored).expect("parse");
            let external_edit = serde_json::json!({
                "id": "external-1",
                "actor": "peer",
                "forwards": [{ "operation": "SetN", "n": 42 }],
                "backwards": [{ "operation": "SetN", "n": 1 }],
                "sequenceNumber": 9,
                "startedAt": "0"
            });
            value["vcs"]["edits"].as_array_mut().unwrap().push(external_edit);
            storage.write("doc-a", "demo/v1", &serde_json::to_string(&value).unwrap()).expect("out-of-band write");

            // Deterministically poke the actor to re-read (notify also wired, but timing-independent here).
            channels.cmd_tx.send(DocumentActorMsg::ExternalChanged).expect("poke");

            let event = wait_for_event(&mut events, |event| matches!(event, DocumentEvent::RemoteOperations { .. })).await;
            match event {
                DocumentEvent::RemoteOperations { envelopes } => {
                    assert_eq!(envelopes.len(), 1);
                    assert_eq!(envelopes[0].id.0, "external-1");
                }
                other => panic!("expected RemoteOperations, got {other:?}"),
            }

            // The store ingests the pushed operation on tick(); the timeline grows and projection updates.
            store.tick().expect("tick");
            assert_eq!(store.envelope().vcs.edits.len(), 2, "external edit joined the timeline");
            assert_eq!(store.projection().expect("projection").n, 42);
            host.close("doc-a");
        }

        //#region 🔖MockHub
        /// @emoji 🧪 A minimal in-process hub speaking the real, binary `protocol_wire::ClientFrame`/
        /// `ServerFrame` protocol, so the hub endpoint is exercised end-to-end without linking a real
        /// `db`-backed hub (that's CW6's job — this mock never touches `db`). Ordinal-indexed log,
        /// mirroring `db_sync`'s replica-catch-up shape (`Hello.frontier` -> filtered backlog ->
        /// `Welcome` then a follow-up `Commands`), but with a placeholder `chain_hash`/`resume_token`
        /// (this mock has no durable log to derive a real chain hash from).
        struct MockHub {
            log: Arc<Mutex<Vec<(u64, protocol::OperationEnvelope)>>>,
            broadcast: tokio_broadcast::Sender<ServerFrame>,
        }

        fn mock_frontier(ordinal: u64) -> protocol::RuntimeFrontierSummary {
            protocol::RuntimeFrontierSummary { document_id: DocumentId("mock".to_string()), head_edit_ordinal: ordinal, head_edit_id: format!("edit-{ordinal}"), last_commit_seq: ordinal, chain_hash: [0u8; 32] }
        }

        async fn spawn_mock_hub() -> (std::net::SocketAddr, Arc<MockHub>) {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind");
            let addr = listener.local_addr().expect("addr");
            let (broadcast, _rx) = tokio_broadcast::channel(256);
            let hub = Arc::new(MockHub { log: Arc::new(Mutex::new(Vec::new())), broadcast });
            let accept_hub = hub.clone();
            tokio::spawn(async move {
                loop {
                    let Ok((stream, _)) = listener.accept().await else { break };
                    let conn_hub = accept_hub.clone();
                    tokio::spawn(async move {
                        if let Ok(ws) = tokio_tungstenite::accept_async(stream).await {
                            mock_hub_connection(ws, conn_hub).await;
                        }
                    });
                }
            });
            (addr, hub)
        }

        async fn mock_hub_connection(ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, hub: Arc<MockHub>) {
            let (mut write, mut read) = ws.split();
            // Expect Hello first.
            let requested_ordinal = match read.next().await {
                Some(Ok(WsMessage::Binary(bytes))) => match decode_client_frame(&bytes) {
                    Ok((_, ClientFrame::Hello { frontier, .. })) => frontier.map_or(0, |frontier| frontier.head_edit_ordinal),
                    _ => return,
                },
                _ => return,
            };
            let (frontier, backlog) = {
                let log = hub.log.lock().await;
                let ordinal = log.last().map_or(0, |(ordinal, _)| *ordinal);
                let backlog: Vec<protocol::OperationEnvelope> = log.iter().filter(|(ordinal, _)| *ordinal > requested_ordinal).map(|(_, envelope)| envelope.clone()).collect();
                (mock_frontier(ordinal), backlog)
            };
            let welcome = ServerFrame::Welcome { session_id: "mock-session".to_string(), resume_token: "mock-resume".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail };
            if write.send(WsMessage::Binary(encode_server_frame(&welcome, Lane::Command).into())).await.is_err() {
                return;
            }
            if !backlog.is_empty() {
                let commands = ServerFrame::Commands { envelopes: backlog, origin: ActorId("hub-backlog".to_string()), frontier: frontier.clone() };
                if write.send(WsMessage::Binary(encode_server_frame(&commands, Lane::Command).into())).await.is_err() {
                    return;
                }
            }
            let mut broadcast_rx = hub.broadcast.subscribe();
            loop {
                tokio::select! {
                    incoming = read.next() => {
                        match incoming {
                            Some(Ok(WsMessage::Binary(bytes))) => {
                                match decode_client_frame(&bytes) {
                                    Ok((_, ClientFrame::Commands { batch_id, envelopes })) => {
                                        let mut assigned_frontier = frontier.clone();
                                        for envelope in envelopes {
                                            let (ordinal, origin) = {
                                                let mut log = hub.log.lock().await;
                                                let next = log.last().map_or(0, |(ordinal, _)| *ordinal) + 1;
                                                log.push((next, envelope.clone()));
                                                (next, envelope.actor.clone())
                                            };
                                            assigned_frontier = mock_frontier(ordinal);
                                            let _ = hub.broadcast.send(ServerFrame::Commands { envelopes: vec![envelope], origin, frontier: assigned_frontier.clone() });
                                        }
                                        let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: assigned_frontier };
                                        let _ = write.send(WsMessage::Binary(encode_server_frame(&ack, Lane::Command).into())).await;
                                    }
                                    Ok((_, ClientFrame::PreviewPublish { key, seq, payload })) => {
                                        // 👻 Best-effort fan-out on the uncredited preview lane — this mock
                                        // hub doesn't track per-connection actor identity beyond `Hello`, so
                                        // it stamps a fixed sentinel origin (fine for the round-trip test
                                        // this drives, which only asserts the *other* peer receives it).
                                        let _ = hub.broadcast.send(ServerFrame::Preview { actor: ActorId("mock-hub-peer".to_string()), key, seq, payload });
                                    }
                                    Ok((_, ClientFrame::Bye)) | Err(_) => {}
                                    Ok(_) => {}
                                }
                            }
                            Some(Ok(WsMessage::Close(_))) | None | Some(Err(_)) => break,
                            Some(Ok(_)) => {}
                        }
                    }
                    frame = broadcast_rx.recv() => {
                        match frame {
                            Ok(frame) => {
                                if write.send(WsMessage::Binary(encode_server_frame(&frame, Lane::Command).into())).await.is_err() {
                                    break;
                                }
                            }
                            Err(tokio_broadcast::error::RecvError::Lagged(_)) => {}
                            Err(tokio_broadcast::error::RecvError::Closed) => break,
                        }
                    }
                }
            }
        }
        //#endregion 🔖MockHub

        // 🔬 Two DocumentHosts converge through a hub: A's operation fans out to B, whose store materializes it.
        #[tokio::test]
        async fn two_hosts_converge_through_hub() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = DocumentHost::new();
            let channels_a = host_a.open(DocumentActorConfig {
                document_id: "shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), studio_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "A".into(),
            });
            let mut store_a = DocumentVcsStore::new(demo_envelope("shared"));
            store_a.attach_backbone(Box::new(channels_a.channel_backbone)).expect("attach a");

            let host_b = DocumentHost::new();
            let channels_b = host_b.open(DocumentActorConfig {
                document_id: "shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), studio_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "B".into(),
            });
            let mut events_b = host_b.subscribe("shared");
            let mut store_b = DocumentVcsStore::new(demo_envelope("shared"));
            store_b.attach_backbone(Box::new(channels_b.channel_backbone)).expect("attach b");

            // Give both actors time to connect + Hello.
            tokio::time::sleep(Duration::from_millis(300)).await;

            store_a.dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![DemoOperation::SetN { n: 7 }], description: None }).expect("apply on a");
            channels_a.cmd_tx.send(DocumentActorMsg::LocalOperations { envelopes: Vec::new() }).expect("wake a");

            let event = wait_for_event(&mut events_b, |event| matches!(event, DocumentEvent::RemoteOperations { .. })).await;
            match event {
                DocumentEvent::RemoteOperations { envelopes } => assert_eq!(envelopes.len(), 1),
                other => panic!("expected RemoteOperations on B, got {other:?}"),
            }
            store_b.tick().expect("tick b");
            assert_eq!(store_b.projection().expect("projection b").n, 7, "B converged on A's operation");

            host_a.close("shared");
            host_b.close("shared");
        }

        // 🔬 Reconnect with `since` catch-up: after A appends operations while B is offline, B reconnects and
        // its Welcome backlog carries only the operations it missed.
        #[tokio::test]
        async fn reconnect_since_catch_up_replays_backlog() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = DocumentHost::new();
            let channels_a = host_a.open(DocumentActorConfig {
                document_id: "catchup".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), studio_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "A".into(),
            });
            let mut store_a = DocumentVcsStore::new(demo_envelope("catchup"));
            store_a.attach_backbone(Box::new(channels_a.channel_backbone)).expect("attach a");
            tokio::time::sleep(Duration::from_millis(300)).await;

            // A applies two operations while nobody else is connected.
            for n in [3, 4] {
                store_a.dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![DemoOperation::SetN { n }], description: None }).expect("apply on a");
                channels_a.cmd_tx.send(DocumentActorMsg::LocalOperations { envelopes: Vec::new() }).expect("wake a");
                tokio::time::sleep(Duration::from_millis(80)).await;
            }

            // B connects fresh (since_version 0) and its Welcome backlog replays both operations.
            let host_b = DocumentHost::new();
            let channels_b =
                host_b.open(DocumentActorConfig { document_id: "catchup".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, studio_id: "studio-1".into(), token: None }], watch_external: false, actor: "B".into() });
            let mut events_b = host_b.subscribe("catchup");
            let mut store_b = DocumentVcsStore::new(demo_envelope("catchup"));
            store_b.attach_backbone(Box::new(channels_b.channel_backbone)).expect("attach b");

            let event = wait_for_event(&mut events_b, |event| matches!(event, DocumentEvent::RemoteOperations { .. })).await;
            if let DocumentEvent::RemoteOperations { envelopes } = event {
                assert_eq!(envelopes.len(), 2, "backlog replays both missed operations");
            }
            store_b.tick().expect("tick b");
            assert_eq!(store_b.envelope().vcs.edits.len(), 2, "B caught up on the full backlog");
            assert_eq!(store_b.projection().expect("projection b").n, 4);

            host_a.close("catchup");
            host_b.close("catchup");
        }

        // 🔬 Detach drains the outbox: an operation applied right before close still reaches the hub (and B).
        #[tokio::test]
        async fn detach_drains_pending_outbound_operations() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            // Observer B stays connected to witness A's last operation.
            let host_b = DocumentHost::new();
            let channels_b = host_b.open(DocumentActorConfig {
                document_id: "drain".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), studio_id: "studio-1".into(), token: None }],
                watch_external: false,
                actor: "B".into(),
            });
            let mut events_b = host_b.subscribe("drain");
            let mut store_b = DocumentVcsStore::new(demo_envelope("drain"));
            store_b.attach_backbone(Box::new(channels_b.channel_backbone)).expect("attach b");

            let host_a = DocumentHost::new();
            let channels_a =
                host_a.open(DocumentActorConfig { document_id: "drain".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, studio_id: "studio-1".into(), token: None }], watch_external: false, actor: "A".into() });
            let mut store_a = DocumentVcsStore::new(demo_envelope("drain"));
            store_a.attach_backbone(Box::new(channels_a.channel_backbone)).expect("attach a");
            tokio::time::sleep(Duration::from_millis(300)).await;

            store_a.dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![DemoOperation::SetN { n: 5 }], description: None }).expect("apply on a");
            // Immediately close A without waiting for the poll tick: Detach must flush the outbox first.
            host_a.close("drain");

            let event = wait_for_event(&mut events_b, |event| matches!(event, DocumentEvent::RemoteOperations { .. })).await;
            if let DocumentEvent::RemoteOperations { envelopes } = event {
                assert_eq!(envelopes.len(), 1, "the operation applied before detach was not lost");
            }
            store_b.tick().expect("tick b");
            assert_eq!(store_b.projection().expect("projection b").n, 5);
            host_b.close("drain");
        }

        // 🔬 The mock hub always Acks `Accepted` — confirms the new `ServerFrame::Ack` ->
        // `DocumentEvent::CommandOutcome` wiring actually fires (not just that it compiles).
        #[tokio::test]
        async fn command_outcome_accepted_fires_after_hub_ack() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");
            let host = DocumentHost::new();
            let channels =
                host.open(DocumentActorConfig { document_id: "outcome".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, studio_id: "studio-1".into(), token: None }], watch_external: false, actor: "A".into() });
            let mut events = host.subscribe("outcome");
            let mut store = DocumentVcsStore::new(demo_envelope("outcome"));
            store.attach_backbone(Box::new(channels.channel_backbone)).expect("attach");
            tokio::time::sleep(Duration::from_millis(300)).await;

            store.dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply");
            channels.cmd_tx.send(DocumentActorMsg::LocalOperations { envelopes: Vec::new() }).expect("wake");

            let event = wait_for_event(&mut events, |event| matches!(event, DocumentEvent::CommandOutcome { .. })).await;
            match event {
                DocumentEvent::CommandOutcome { outcome, .. } => assert_eq!(outcome, CommandAckOutcome::Accepted),
                other => panic!("expected CommandOutcome, got {other:?}"),
            }
            host.close("outcome");
        }

        // 🔬 `SyncSession::publish_preview` -> `ClientFrame::PreviewPublish` -> the mock hub's
        // preview-lane fan-out -> `ServerFrame::Preview` -> `DocumentEvent::Preview` on another peer.
        #[tokio::test]
        async fn publish_preview_round_trips_through_hub() {
            let (addr, _hub) = spawn_mock_hub().await;
            let base_url = format!("ws://{addr}");

            let host_a = DocumentHost::new();
            let channels_a =
                host_a.open(DocumentActorConfig { document_id: "preview".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), studio_id: "studio-1".into(), token: None }], watch_external: false, actor: "A".into() });

            let host_b = DocumentHost::new();
            host_b.open(DocumentActorConfig { document_id: "preview".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, studio_id: "studio-1".into(), token: None }], watch_external: false, actor: "B".into() });
            let mut events_b = host_b.subscribe("preview");
            tokio::time::sleep(Duration::from_millis(300)).await;

            channels_a.cmd_tx.send(DocumentActorMsg::PublishPreview { key: "cursor".into(), seq: 1, payload: vec![1, 2, 3] }).expect("publish preview");

            let event = wait_for_event(&mut events_b, |event| matches!(event, DocumentEvent::Preview { .. })).await;
            match event {
                DocumentEvent::Preview { key, seq, payload, .. } => {
                    assert_eq!(key, "cursor");
                    assert_eq!(seq, 1);
                    assert_eq!(payload, vec![1, 2, 3]);
                }
                other => panic!("expected Preview, got {other:?}"),
            }
            host_a.close("preview");
            host_b.close("preview");
        }

        // 🔬 Shared fixtures replay: each fixture's inbound stimuli produce the expected DocumentEvent
        // sequence and final timeline. The same fixtures drive WS-E's vitest harness against the TS twin.
        #[tokio::test]
        async fn fixtures_replay_matches_expected_events() {
            let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures");
            let fixtures = load_fixtures(&fixtures_dir);
            assert!(!fixtures.is_empty(), "expected fixtures in {fixtures_dir:?}");
            for fixture in fixtures {
                replay_fixture(&fixture).await;
            }
        }

        async fn replay_fixture(fixture: &ActorFixture) {
            let dir = tempfile::tempdir().expect("tempdir");
            let host = DocumentHost::new();
            let channels =
                host.open(DocumentActorConfig { document_id: fixture.document_id.clone(), schema: fixture.schema.clone(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() });
            let mut events = host.subscribe(&fixture.document_id);
            let mut store = DocumentVcsStore::new(create_document_vcs_envelope::<DemoProjection, DemoOperation>(&fixture.schema, &fixture.document_id, DemoProjection { n: 0 }, None));
            store.attach_backbone(Box::new(channels.channel_backbone)).expect("attach");
            let storage = vcs::FolderSqliteStorage::new(dir.path().to_path_buf());
            // Wait for the seed snapshot to land on disk.
            loop {
                if storage.read(&fixture.document_id).expect("read").is_some() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }

            // Lockstep: apply each stimulus, then wait for its paired expected event before the next
            // (removes any write/poke race). Folder-replayable fixtures pair inbound 1:1 with events.
            assert_eq!(fixture.inbound.len(), fixture.expected_events.len(), "fixture {} must pair each inbound stimulus with one expected event", fixture.name);
            let mut observed: Vec<String> = Vec::new();
            for (inbound, expected) in fixture.inbound.iter().zip(fixture.expected_events.iter()) {
                match inbound {
                    FixtureInbound::ExternalEdits { edits } => {
                        let stored = storage.read(&fixture.document_id).expect("read").expect("some");
                        let mut value: Value = serde_json::from_str(&stored).expect("parse");
                        let array = value["vcs"]["edits"].as_array_mut().expect("edits array");
                        for edit in edits {
                            array.push(edit.clone());
                        }
                        storage.write(&fixture.document_id, &fixture.schema, &serde_json::to_string(&value).unwrap()).expect("write");
                        channels.cmd_tx.send(DocumentActorMsg::ExternalChanged).expect("poke");
                    }
                    FixtureInbound::ReplaceEnvelope { envelope } => {
                        storage.write(&fixture.document_id, &fixture.schema, &serde_json::to_string(envelope).unwrap()).expect("replace write");
                        channels.cmd_tx.send(DocumentActorMsg::ExternalChanged).expect("poke");
                    }
                    FixtureInbound::HubFrame { .. } => {
                        panic!("fixture {} uses a HubFrame stimulus not supported by the Rust harness", fixture.name);
                    }
                }
                let event = wait_for_event(&mut events, |event| document_event_tag(event) == expected.as_str()).await;
                observed.push(document_event_tag(&event).to_string());
                store.tick().expect("tick");
            }
            assert_eq!(&observed, &fixture.expected_events, "fixture {} event sequence", fixture.name);
            let timeline_ids: Vec<String> = store.envelope().vcs.edits.iter().map(|edit| edit.id.clone()).collect();
            for expected_id in &fixture.expected_edit_ids {
                assert!(timeline_ids.contains(expected_id), "fixture {} expected edit id {expected_id} in timeline {timeline_ids:?}", fixture.name);
            }
            host.close(&fixture.document_id);
        }

        fn document_event_tag(event: &DocumentEvent) -> &'static str {
            match event {
                DocumentEvent::RemoteOperations { .. } => "remoteOperations",
                DocumentEvent::SnapshotReplaced { .. } => "snapshotReplaced",
                DocumentEvent::Status(_) => "status",
                DocumentEvent::Presence { .. } => "presence",
                DocumentEvent::Preview { .. } => "preview",
                DocumentEvent::CommandOutcome { .. } => "commandOutcome",
                DocumentEvent::Conflict(_) => "conflict",
            }
        }
    }
    //#endregion 🧪Actor
}
