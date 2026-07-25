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

use semio_framework_core::{HubClientFrame, HubServerFrame, OperationEnvelope, PresencePeer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::{broadcast, mpsc};
use vcs::{reconcile_alternative, BackboneMessage, ChannelBackbone, ChannelBackboneRemote, DocumentVcsStore, Operation, StudioConflict};

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
    /// @emoji ⚠️ A structural conflict (external divergence with local pending operations / hub CAS reject).
    Conflict(StudioConflict),
}
//#endregion 🔖Protocol

//#region 🔖Endpoints
/// @emoji 🧱 Core wire types used only when reconstructing an {@link OperationEnvelope} from a stored edit,
/// which happens exclusively on the native folder path.
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_core::{ActorId, DocumentDiff, DocumentId, DocumentVersion, InverseOperation, OperationId, PayloadHash, SchemaId, SchemaVersion, UndoPolicy};

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

//#region 🔖SyncSession
/// @emoji 🔁 Pairs a document's vcs store with the causal DAG that reconciles remote envelopes into
/// it. Extended into the actor world via {@link SyncSession::attach}: it holds the actor command
/// channel and event stream, drains status on {@link SyncSession::tick}, and delegates store IO.
pub struct SyncSession<P, Operation>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Operation: Clone + serde::Serialize + serde::de::DeserializeOwned + Operation<P>,
{
    pub store: DocumentVcsStore<P, Operation>,
    cmd_tx: Option<mpsc::UnboundedSender<DocumentActorMsg>>,
    events: Option<broadcast::Receiver<DocumentEvent>>,
    status: DocumentSyncStatus,
}

impl<P, Operation> SyncSession<P, Operation>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Operation: Clone + serde::Serialize + serde::de::DeserializeOwned + Operation<P>,
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

    /// @emoji 📁 A folder/file binding's storage driver, keyed for multi-document sqlite or single blob.
    enum FolderEndpoint {
        Sqlite { storage: vcs::FolderSqliteStorage, document_id: String, schema: String },
        Json { storage: vcs::FileJsonStorage },
    }

    impl FolderEndpoint {
        fn read(&self) -> Option<String> {
            match self {
                FolderEndpoint::Sqlite { storage, document_id, .. } => storage.read(document_id).ok().flatten(),
                FolderEndpoint::Json { storage } => storage.read().ok().flatten(),
            }
        }

        fn write(&self, json: &str) {
            match self {
                FolderEndpoint::Sqlite { storage, document_id, schema } => {
                    let _ = storage.write(document_id, schema, json);
                }
                FolderEndpoint::Json { storage } => {
                    let _ = storage.write(json);
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
        hub_version: i64,
        backoff_ms: u64,
        reconnect_at: Option<Instant>,
        pending_hub: HashSet<String>,
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
                hub_version: 0,
                backoff_ms: 500,
                reconnect_at: None,
                pending_hub: HashSet::new(),
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
            if let Some(json) = self.folder.as_ref().and_then(|folder| folder.read()) {
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
                        self.relay_snapshot_to_hub(&envelope_json).await;
                    }
                    false
                }
                DocumentActorMsg::PresenceHeartbeat { peer } => {
                    self.send_client_frame(HubClientFrame::Presence { peer }).await;
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
                        self.relay_snapshot_to_hub(&envelope_json).await;
                    }
                    BackboneMessage::Ack { .. } => {}
                }
            }
            drained
        }

        //#region 🔖Folder
        /// @emoji ✍️ Persists the current envelope JSON to the folder binding and records the content
        /// hash for self-write suppression.
        fn persist_write(&mut self, json: &str) {
            let Some(folder) = self.folder.as_ref() else { return };
            folder.write(json);
            self.last_written_hash = Some(semio_framework_hash::hash_bytes(json.as_bytes()));
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
            let Some(json) = self.folder.as_ref().and_then(|folder| folder.read()) else { return };
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
                if !self.pending_hub.is_empty() {
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
                    let hello = HubClientFrame::Hello { actor: self.actor.clone(), token, since_version: self.hub_version };
                    self.send_client_frame(hello).await;
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
                Some(Ok(Message::Text(text))) => {
                    if let Ok(frame) = serde_json::from_str::<HubServerFrame>(text.as_str()) {
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

        fn on_hub_frame(&mut self, frame: HubServerFrame) {
            match frame {
                HubServerFrame::Welcome { version, envelope, presence, backlog } => {
                    self.hub_version = version;
                    self.set_remote_state(RemoteState::Live { peer_count: presence.len() });
                    if let Some(envelope) = envelope {
                        self.deliver_snapshot(envelope.to_string());
                    }
                    if !backlog.is_empty() {
                        self.persist_operations(&backlog);
                        self.deliver_remote_operations(backlog);
                    }
                    self.emit(DocumentEvent::Presence { peers: presence });
                }
                HubServerFrame::Operations { version, envelopes, origin } => {
                    self.hub_version = version;
                    if origin != self.actor {
                        self.persist_operations(&envelopes);
                        self.deliver_remote_operations(envelopes);
                    }
                }
                HubServerFrame::SnapshotReplaced { version, envelope } => {
                    self.hub_version = version;
                    self.deliver_snapshot(envelope.to_string());
                }
                HubServerFrame::Presence { peers } => {
                    self.set_remote_state(RemoteState::Live { peer_count: peers.len() });
                    self.emit(DocumentEvent::Presence { peers });
                }
                HubServerFrame::Ack { operation_id, version } => {
                    self.hub_version = version;
                    self.pending_hub.remove(&operation_id);
                    self.emit_status_if_changed();
                }
                HubServerFrame::Conflict { message } => {
                    self.emit(DocumentEvent::Conflict(StudioConflict { kind: "hubCas".into(), uri: self.hub_base_url.clone().unwrap_or_default(), message }));
                }
                HubServerFrame::Error { .. } => {}
            }
        }

        async fn relay_operations_to_hub(&mut self, envelopes: &[OperationEnvelope]) {
            if self.hub.is_none() || envelopes.is_empty() {
                return;
            }
            for envelope in envelopes {
                self.pending_hub.insert(envelope.id.0.clone());
            }
            self.send_client_frame(HubClientFrame::Operations { envelopes: envelopes.to_vec() }).await;
            self.emit_status_if_changed();
        }

        async fn relay_snapshot_to_hub(&mut self, envelope_json: &str) {
            if self.hub.is_none() {
                return;
            }
            if let Ok(envelope) = serde_json::from_str::<Value>(envelope_json) {
                let version = self.hub_version;
                self.send_client_frame(HubClientFrame::PutEnvelope { version, envelope }).await;
            }
        }

        async fn send_client_frame(&mut self, frame: HubClientFrame) {
            let json = serde_json::to_string(&frame).unwrap_or_default();
            self.send_raw(Message::Text(json.into())).await;
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
            DocumentSyncStatus { persisted: self.last_written_hash.is_some() || self.hub_version > 0, pendingOperations: self.pending_hub.len(), remote: self.remote_state.clone() }
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

    fn build_folder_endpoint(path: &std::path::Path, document_id: &str, schema: &str) -> FolderEndpoint {
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            FolderEndpoint::Json { storage: vcs::FileJsonStorage::new(path.to_path_buf()) }
        } else {
            FolderEndpoint::Sqlite { storage: vcs::FolderSqliteStorage::new(path.to_path_buf()), document_id: document_id.to_string(), schema: schema.to_string() }
        }
    }

    /// @emoji 📍 The on-disk path a folder binding writes to: the `*.json` blob itself, or the
    /// multi-document sqlite db under `<folder>/.semio/documents.db`.
    fn folder_watch_path_for(path: &Path) -> PathBuf {
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
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
    use std::collections::HashSet;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{MessageEvent, WebSocket};

    struct WasmActor {
        document_id: String,
        actor: String,
        remote: ChannelBackboneRemote,
        events: broadcast::Sender<DocumentEvent>,
        hub_base_url: Option<String>,
        hub_studio_id: Option<String>,
        hub_token: Option<String>,
        ws: Option<WebSocket>,
        hub_version: i64,
        pending_hub: HashSet<String>,
        incoming_tx: mpsc::UnboundedSender<String>,
        _closures: Vec<Closure<dyn FnMut(MessageEvent)>>,
        _open_closures: Vec<Closure<dyn FnMut()>>,
    }

    impl WasmActor {
        fn connect(&mut self) {
            let Some(base_url) = self.hub_base_url.clone() else { return };
            let studio_id = self.hub_studio_id.clone().unwrap_or_default();
            let url = hub_ws_url(&base_url, &studio_id, &self.document_id);
            let Ok(ws) = WebSocket::new(&url) else { return };

            let incoming = self.incoming_tx.clone();
            let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
                if let Some(text) = event.data().as_string() {
                    let _ = incoming.send(text);
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            self._closures.push(onmessage);

            let hello = HubClientFrame::Hello { actor: self.actor.clone(), token: self.hub_token.clone(), since_version: self.hub_version };
            let hello_json = serde_json::to_string(&hello).unwrap_or_default();
            let ws_for_open = ws.clone();
            let onopen = Closure::wrap(Box::new(move || {
                let _ = ws_for_open.send_with_str(&hello_json);
            }) as Box<dyn FnMut()>);
            ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            self._open_closures.push(onopen);

            self.ws = Some(ws);
        }

        fn send_frame(&self, frame: &HubClientFrame) {
            if let Some(ws) = &self.ws {
                let _ = ws.send_with_str(&serde_json::to_string(frame).unwrap_or_default());
            }
        }

        fn drain_and_relay(&mut self) -> bool {
            let messages = self.remote.drain().unwrap_or_default();
            let drained = !messages.is_empty();
            for message in messages {
                match message {
                    BackboneMessage::Operations { envelopes } => {
                        for envelope in &envelopes {
                            self.pending_hub.insert(envelope.id.0.clone());
                        }
                        self.send_frame(&HubClientFrame::Operations { envelopes });
                    }
                    BackboneMessage::Snapshot { envelope_json } => {
                        if let Ok(envelope) = serde_json::from_str::<Value>(&envelope_json) {
                            self.send_frame(&HubClientFrame::PutEnvelope { version: self.hub_version, envelope });
                        }
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
                        for envelope in &envelopes {
                            self.pending_hub.insert(envelope.id.0.clone());
                        }
                        self.send_frame(&HubClientFrame::Operations { envelopes });
                    }
                }
                DocumentActorMsg::LocalSnapshot { envelope_json } => {
                    if !self.drain_and_relay() {
                        if let Ok(envelope) = serde_json::from_str::<Value>(&envelope_json) {
                            self.send_frame(&HubClientFrame::PutEnvelope { version: self.hub_version, envelope });
                        }
                    }
                }
                DocumentActorMsg::PresenceHeartbeat { peer } => {
                    self.send_frame(&HubClientFrame::Presence { peer });
                }
                DocumentActorMsg::ExternalChanged | DocumentActorMsg::Detach => {}
            }
        }

        fn on_text(&mut self, text: &str) {
            let Ok(frame) = serde_json::from_str::<HubServerFrame>(text) else { return };
            match frame {
                HubServerFrame::Welcome { version, envelope, presence, backlog } => {
                    self.hub_version = version;
                    if let Some(envelope) = envelope {
                        self.deliver_snapshot(envelope.to_string());
                    }
                    if !backlog.is_empty() {
                        self.deliver_remote_operations(backlog);
                    }
                    let _ = self.events.send(DocumentEvent::Presence { peers: presence });
                }
                HubServerFrame::Operations { version, envelopes, origin } => {
                    self.hub_version = version;
                    if origin != self.actor {
                        self.deliver_remote_operations(envelopes);
                    }
                }
                HubServerFrame::SnapshotReplaced { version, envelope } => {
                    self.hub_version = version;
                    self.deliver_snapshot(envelope.to_string());
                }
                HubServerFrame::Presence { peers } => {
                    let _ = self.events.send(DocumentEvent::Presence { peers });
                }
                HubServerFrame::Ack { operation_id, version } => {
                    self.hub_version = version;
                    self.pending_hub.remove(&operation_id);
                }
                HubServerFrame::Conflict { message } => {
                    let _ = self.events.send(DocumentEvent::Conflict(StudioConflict { kind: "hubCas".into(), uri: self.hub_base_url.clone().unwrap_or_default(), message }));
                }
                HubServerFrame::Error { .. } => {}
            }
        }

        fn deliver_remote_operations(&self, envelopes: Vec<OperationEnvelope>) {
            if envelopes.is_empty() {
                return;
            }
            let _ = self.remote.push(BackboneMessage::Operations { envelopes: envelopes.clone() });
            let _ = self.events.send(DocumentEvent::RemoteOperations { envelopes });
        }

        fn deliver_snapshot(&self, envelope_json: String) {
            let _ = self.remote.push(BackboneMessage::Snapshot { envelope_json: envelope_json.clone() });
            let _ = self.events.send(DocumentEvent::SnapshotReplaced { envelope_json });
        }
    }

    pub(super) fn spawn_actor(config: DocumentActorConfig, remote: ChannelBackboneRemote, mut cmd_rx: mpsc::UnboundedReceiver<DocumentActorMsg>, events: broadcast::Sender<DocumentEvent>) {
        let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<String>();
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
        let mut actor = WasmActor {
            document_id: config.document_id,
            actor: config.actor,
            remote,
            events,
            hub_base_url,
            hub_studio_id,
            hub_token,
            ws: None,
            hub_version: 0,
            pending_hub: HashSet::new(),
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
                    text = incoming_rx.recv() => {
                        match text {
                            Some(text) => actor.on_text(&text),
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
    /// @emoji 📬 A raw `HubServerFrame` delivered as if received over the hub WebSocket. Driven by
    /// WS-E's TS twin; the folder-only Rust harness skips these.
    HubFrame { frame: HubServerFrame },
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
    use vcs::{create_document_vcs_envelope, operation_envelope_from_edit, Edit, OperationDiff};

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
        /// @emoji 🧪 A minimal in-process hub speaking the real `HubClientFrame`/`HubServerFrame`
        /// protocol, so the hub endpoint is exercised end-to-end without linking WS-C's `os-hub` bin.
        struct MockHub {
            log: Arc<Mutex<Vec<(i64, OperationEnvelope)>>>,
            broadcast: tokio_broadcast::Sender<HubServerFrame>,
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
            let since_version = match read.next().await {
                Some(Ok(WsMessage::Text(text))) => match serde_json::from_str::<HubClientFrame>(text.as_str()) {
                    Ok(HubClientFrame::Hello { since_version, .. }) => since_version,
                    _ => return,
                },
                _ => return,
            };
            let (version, backlog) = {
                let log = hub.log.lock().await;
                let version = log.last().map(|(v, _)| *v).unwrap_or(0);
                let backlog: Vec<OperationEnvelope> = log.iter().filter(|(v, _)| *v > since_version).map(|(_, envelope)| envelope.clone()).collect();
                (version, backlog)
            };
            let welcome = HubServerFrame::Welcome { version, envelope: None, presence: Vec::new(), backlog };
            if write.send(WsMessage::Text(serde_json::to_string(&welcome).unwrap().into())).await.is_err() {
                return;
            }
            let mut broadcast_rx = hub.broadcast.subscribe();
            loop {
                tokio::select! {
                    incoming = read.next() => {
                        match incoming {
                            Some(Ok(WsMessage::Text(text))) => {
                                match serde_json::from_str::<HubClientFrame>(text.as_str()) {
                                    Ok(HubClientFrame::Operations { envelopes }) => {
                                        for envelope in envelopes {
                                            let (assigned, origin) = {
                                                let mut log = hub.log.lock().await;
                                                let next = log.last().map(|(v, _)| *v).unwrap_or(0) + 1;
                                                log.push((next, envelope.clone()));
                                                (next, envelope.actor.0.clone())
                                            };
                                            let ack = HubServerFrame::Ack { operation_id: envelope_id(&envelope), version: assigned };
                                            let _ = write.send(WsMessage::Text(serde_json::to_string(&ack).unwrap().into())).await;
                                            let _ = hub.broadcast.send(HubServerFrame::Operations {
                                                version: assigned,
                                                envelopes: vec![envelope],
                                                origin,
                                            });
                                        }
                                    }
                                    Ok(HubClientFrame::Bye) | Err(_) => {}
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
                                if write.send(WsMessage::Text(serde_json::to_string(&frame).unwrap().into())).await.is_err() {
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

        fn envelope_id(envelope: &OperationEnvelope) -> String {
            envelope.id.0.clone()
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
                DocumentEvent::Conflict(_) => "conflict",
            }
        }
    }
    //#endregion 🧪Actor
}
