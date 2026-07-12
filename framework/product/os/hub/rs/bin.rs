mod header {
    // 🧲Header
    // OS hub v2 — SQLite-backed VFS + per-document op-log actors with duplex WebSocket sync.
    // CQRS split: op appends are causally ordered (OpDag) and never version-gated; only whole-envelope
    // snapshot replacement keeps optimistic concurrency (CAS → Conflict).
}

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, mpsc, oneshot};
use semio_framework_core::{HubClientFrame, HubServerFrame, OpDag, OpEnvelope, PresencePeer};
use uuid::Uuid;

//#region 🔖Storage
const SCHEMA: &str = "\
CREATE TABLE IF NOT EXISTS node (
    id TEXT PRIMARY KEY,
    parent_id TEXT REFERENCES node(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS document (
    id TEXT PRIMARY KEY,
    schema TEXT NOT NULL,
    snapshot TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS document_op (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    version INTEGER NOT NULL,
    actor TEXT,
    envelope TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS session (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    client_name TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS share_token (
    token TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
";

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeRow {
    id: String,
    parent_id: Option<String>,
    name: String,
    kind: String,
}

/// @emoji 🗄️ SQLite-backed hub persistence.
/// One `rusqlite::Connection` behind a `Mutex` (chosen over a dedicated command thread: rusqlite
/// calls are synchronous and short, the hub's concurrency is modest, and the guard is never held
/// across an `.await`, so no future is made non-`Send`).
#[derive(Clone)]
struct HubStorage {
    conn: Arc<Mutex<Connection>>,
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn node_from_row(row: &rusqlite::Row) -> rusqlite::Result<NodeRow> {
    Ok(NodeRow {
        id: row.get(0)?,
        parent_id: row.get(1)?,
        name: row.get(2)?,
        kind: row.get(3)?,
    })
}

impl HubStorage {
    fn open(path: &str) -> Self {
        let conn = Connection::open(path).expect("open sqlite");
        conn.execute_batch(SCHEMA).expect("bootstrap schema");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().expect("hub storage lock")
    }

    /// @emoji 🌱 Seeds a default document plus a `Documents/default` VFS entry on a fresh database.
    fn seed(&self) {
        self.ensure_document("default");
        let node_count: i64 = self
            .lock()
            .query_row("SELECT COUNT(*) FROM node", [], |row| row.get(0))
            .unwrap_or(0);
        if node_count == 0 {
            let folder = self.create_node(None, "Documents", "folder");
            self.create_node(Some(&folder.id), "default", "document");
        }
    }

    /// @emoji 📄 Loads a document, seeding a fresh empty snapshot on first open (open-on-demand).
    fn ensure_document(&self, id: &str) -> DocumentRecord {
        let conn = self.lock();
        let existing = conn
            .query_row(
                "SELECT schema, snapshot, version FROM document WHERE id = ?1",
                [id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, i64>(2)?,
                    ))
                },
            )
            .optional()
            .expect("query document");
        if let Some((schema, snapshot, version)) = existing {
            let snapshot = serde_json::from_str(&snapshot).unwrap_or_else(|_| default_snapshot());
            return DocumentRecord {
                schema,
                snapshot,
                version,
            };
        }
        let snapshot = default_snapshot();
        let schema = snapshot
            .get("schema")
            .and_then(|value| value.as_str())
            .unwrap_or("s.studio/v1")
            .to_string();
        conn.execute(
            "INSERT INTO document (id, schema, snapshot, version) VALUES (?1, ?2, ?3, 0)",
            rusqlite::params![id, schema, snapshot.to_string()],
        )
        .expect("insert document");
        DocumentRecord {
            schema,
            snapshot,
            version: 0,
        }
    }

    fn save_document(&self, id: &str, schema: &str, snapshot: &Value, version: i64) {
        self.lock()
            .execute(
                "INSERT INTO document (id, schema, snapshot, version) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(id) DO UPDATE SET schema = ?2, snapshot = ?3, version = ?4",
                rusqlite::params![id, schema, snapshot.to_string(), version],
            )
            .expect("save document");
    }

    /// @emoji ➕ Appends one op, deduping by op id via `INSERT OR IGNORE`. Returns whether a row was written.
    fn insert_op(&self, document_id: &str, version: i64, envelope: &OpEnvelope) -> bool {
        let payload = serde_json::to_string(envelope).unwrap_or_default();
        let changed = self
            .lock()
            .execute(
                "INSERT OR IGNORE INTO document_op (id, document_id, version, actor, envelope, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    envelope.id.0,
                    document_id,
                    version,
                    envelope.actor.0,
                    payload,
                    now_ms()
                ],
            )
            .expect("insert op");
        changed > 0
    }

    fn load_ops(&self, document_id: &str) -> Vec<(i64, OpEnvelope)> {
        let conn = self.lock();
        let mut stmt = conn
            .prepare("SELECT version, envelope FROM document_op WHERE document_id = ?1 ORDER BY version ASC")
            .expect("prepare load_ops");
        let rows = stmt
            .query_map([document_id], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })
            .expect("query load_ops");
        rows.filter_map(|row| row.ok())
            .filter_map(|(version, envelope)| {
                serde_json::from_str(&envelope).ok().map(|envelope| (version, envelope))
            })
            .collect()
    }

    fn list_nodes(&self, parent: Option<&str>) -> Vec<NodeRow> {
        let conn = self.lock();
        match parent {
            Some(parent) => {
                let mut stmt = conn
                    .prepare("SELECT id, parent_id, name, kind FROM node WHERE parent_id = ?1 ORDER BY name")
                    .expect("prepare list_nodes");
                let rows = stmt.query_map([parent], node_from_row).expect("query list_nodes");
                rows.filter_map(|row| row.ok()).collect()
            }
            None => {
                let mut stmt = conn
                    .prepare("SELECT id, parent_id, name, kind FROM node WHERE parent_id IS NULL ORDER BY name")
                    .expect("prepare list_nodes");
                let rows = stmt.query_map([], node_from_row).expect("query list_nodes");
                rows.filter_map(|row| row.ok()).collect()
            }
        }
    }

    fn create_node(&self, parent_id: Option<&str>, name: &str, kind: &str) -> NodeRow {
        let id = Uuid::now_v7().to_string();
        self.lock()
            .execute(
                "INSERT INTO node (id, parent_id, name, kind) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, parent_id, name, kind],
            )
            .expect("insert node");
        NodeRow {
            id,
            parent_id: parent_id.map(|value| value.to_string()),
            name: name.to_string(),
            kind: kind.to_string(),
        }
    }

    fn create_share_token(&self, document_id: &str) -> String {
        let token = Uuid::now_v7().to_string();
        self.lock()
            .execute(
                "INSERT INTO share_token (token, document_id, created_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![token, document_id, now_ms()],
            )
            .expect("insert share token");
        token
    }

    fn document_has_tokens(&self, document_id: &str) -> bool {
        self.lock()
            .query_row(
                "SELECT COUNT(*) FROM share_token WHERE document_id = ?1",
                [document_id],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0)
            > 0
    }

    fn token_valid(&self, document_id: &str, token: &str) -> bool {
        self.lock()
            .query_row(
                "SELECT COUNT(*) FROM share_token WHERE document_id = ?1 AND token = ?2",
                rusqlite::params![document_id, token],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0)
            > 0
    }

    /// @emoji 🔐 Tokenless documents are open (dev default); once any token is issued a valid bearer is required.
    fn authorized(&self, document_id: &str, token: Option<&str>) -> bool {
        if !self.document_has_tokens(document_id) {
            return true;
        }
        match token {
            Some(token) => self.token_valid(document_id, token),
            None => false,
        }
    }
}
//#endregion 🔖Storage

//#region 🔖DocumentActor
struct DocumentRecord {
    schema: String,
    snapshot: Value,
    version: i64,
}

struct AppendedOp {
    version: i64,
    op_id: String,
    is_new: bool,
}

struct SubscribeReply {
    receiver: broadcast::Receiver<HubServerFrame>,
    version: i64,
    envelope: Value,
    presence: Vec<PresencePeer>,
    backlog: Vec<OpEnvelope>,
}

/// @emoji 📬 Mailbox messages for a {@link DocumentActor}.
enum DocMsg {
    Subscribe {
        since_version: i64,
        reply: oneshot::Sender<SubscribeReply>,
    },
    AppendOps {
        envelopes: Vec<OpEnvelope>,
        origin: String,
        reply: oneshot::Sender<Vec<AppendedOp>>,
    },
    PutEnvelope {
        version: i64,
        envelope: Value,
        reply: oneshot::Sender<Result<i64, i64>>,
    },
    GetDocument {
        reply: oneshot::Sender<(Value, i64)>,
    },
    GetEnvelope {
        reply: oneshot::Sender<(Value, i64)>,
    },
    OpsSince {
        since: i64,
        reply: oneshot::Sender<Vec<(i64, OpEnvelope)>>,
    },
    PresenceUpdate {
        peer: PresencePeer,
    },
    PresenceLeave {
        actor: String,
    },
}

/// @emoji 🎛️ Cheap clonable handle to a document's actor mailbox.
#[derive(Clone)]
struct DocumentHandle {
    tx: mpsc::Sender<DocMsg>,
}

impl DocumentHandle {
    async fn subscribe(&self, since_version: i64) -> Option<SubscribeReply> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(DocMsg::Subscribe { since_version, reply })
            .await
            .ok()?;
        rx.await.ok()
    }

    async fn append_ops(&self, envelopes: Vec<OpEnvelope>, origin: String) -> Vec<AppendedOp> {
        let (reply, rx) = oneshot::channel();
        if self
            .tx
            .send(DocMsg::AppendOps { envelopes, origin, reply })
            .await
            .is_err()
        {
            return Vec::new();
        }
        rx.await.unwrap_or_default()
    }

    async fn put_envelope(&self, version: i64, envelope: Value) -> Result<i64, i64> {
        let (reply, rx) = oneshot::channel();
        if self
            .tx
            .send(DocMsg::PutEnvelope { version, envelope, reply })
            .await
            .is_err()
        {
            return Err(-1);
        }
        rx.await.unwrap_or(Err(-1))
    }

    async fn get_document(&self) -> Option<(Value, i64)> {
        let (reply, rx) = oneshot::channel();
        self.tx.send(DocMsg::GetDocument { reply }).await.ok()?;
        rx.await.ok()
    }

    async fn get_envelope(&self) -> Option<(Value, i64)> {
        let (reply, rx) = oneshot::channel();
        self.tx.send(DocMsg::GetEnvelope { reply }).await.ok()?;
        rx.await.ok()
    }

    async fn ops_since(&self, since: i64) -> Vec<(i64, OpEnvelope)> {
        let (reply, rx) = oneshot::channel();
        if self.tx.send(DocMsg::OpsSince { since, reply }).await.is_err() {
            return Vec::new();
        }
        rx.await.unwrap_or_default()
    }

    async fn presence_update(&self, peer: PresencePeer) {
        let _ = self.tx.send(DocMsg::PresenceUpdate { peer }).await;
    }

    async fn presence_leave(&self, actor: String) {
        let _ = self.tx.send(DocMsg::PresenceLeave { actor }).await;
    }
}

/// @emoji 🎭 One actor per open document: owns the `OpDag`, the log version counter, the in-memory op
/// cache, the presence roster, and the per-document broadcast fan-out. All persistence goes through
/// {@link HubStorage}.
struct DocumentActor {
    document_id: String,
    storage: HubStorage,
    schema: String,
    snapshot: Value,
    version: i64,
    dag: OpDag,
    ops: Vec<(i64, OpEnvelope)>,
    seen: HashSet<String>,
    presence: HashMap<String, PresencePeer>,
    broadcast: broadcast::Sender<HubServerFrame>,
}

impl DocumentActor {
    fn load(document_id: String, storage: HubStorage) -> Self {
        let record = storage.ensure_document(&document_id);
        let ops = storage.load_ops(&document_id);
        let mut dag = OpDag::new();
        let mut seen = HashSet::new();
        for (_, envelope) in &ops {
            let _ = dag.insert(envelope.clone());
            seen.insert(envelope.id.0.clone());
        }
        let (broadcast, _) = broadcast::channel(256);
        Self {
            document_id,
            storage,
            schema: record.schema,
            snapshot: record.snapshot,
            version: record.version,
            dag,
            ops,
            seen,
            presence: HashMap::new(),
            broadcast,
        }
    }

    async fn run(mut self, mut rx: mpsc::Receiver<DocMsg>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                DocMsg::Subscribe {
                    since_version,
                    reply,
                } => {
                    let backlog = self
                        .ops
                        .iter()
                        .filter(|(version, _)| *version > since_version)
                        .map(|(_, envelope)| envelope.clone())
                        .collect();
                    let _ = reply.send(SubscribeReply {
                        receiver: self.broadcast.subscribe(),
                        version: self.version,
                        envelope: self.snapshot.clone(),
                        presence: self.presence.values().cloned().collect(),
                        backlog,
                    });
                }
                DocMsg::AppendOps {
                    envelopes,
                    origin,
                    reply,
                } => {
                    let mut appended = Vec::new();
                    let mut fresh = Vec::new();
                    for envelope in envelopes {
                        let op_id = envelope.id.0.clone();
                        if self.seen.contains(&op_id) {
                            appended.push(AppendedOp {
                                version: self.version,
                                op_id,
                                is_new: false,
                            });
                            continue;
                        }
                        let inserted = self.storage.insert_op(&self.document_id, self.version + 1, &envelope);
                        if !inserted {
                            self.seen.insert(op_id.clone());
                            appended.push(AppendedOp {
                                version: self.version,
                                op_id,
                                is_new: false,
                            });
                            continue;
                        }
                        self.version += 1;
                        let _ = self.dag.insert(envelope.clone());
                        self.seen.insert(op_id.clone());
                        self.ops.push((self.version, envelope.clone()));
                        appended.push(AppendedOp {
                            version: self.version,
                            op_id,
                            is_new: true,
                        });
                        fresh.push(envelope);
                    }
                    if !fresh.is_empty() {
                        self.storage
                            .save_document(&self.document_id, &self.schema, &self.snapshot, self.version);
                        let _ = self.broadcast.send(HubServerFrame::Ops {
                            version: self.version,
                            envelopes: fresh,
                            origin,
                        });
                    }
                    let _ = reply.send(appended);
                }
                DocMsg::PutEnvelope {
                    version,
                    envelope,
                    reply,
                } => {
                    if version != self.version {
                        let _ = reply.send(Err(self.version));
                        continue;
                    }
                    self.version += 1;
                    self.apply_envelope(&envelope);
                    self.storage
                        .save_document(&self.document_id, &self.schema, &self.snapshot, self.version);
                    let _ = self.broadcast.send(HubServerFrame::SnapshotReplaced {
                        version: self.version,
                        envelope: self.snapshot.clone(),
                    });
                    let _ = reply.send(Ok(self.version));
                }
                DocMsg::GetDocument { reply } => {
                    let _ = reply.send((self.snapshot.clone(), self.version));
                }
                DocMsg::GetEnvelope { reply } => {
                    let _ = reply.send((self.envelope_view(), self.version));
                }
                DocMsg::OpsSince { since, reply } => {
                    let rows = self
                        .ops
                        .iter()
                        .filter(|(version, _)| *version > since)
                        .map(|(version, envelope)| (*version, envelope.clone()))
                        .collect();
                    let _ = reply.send(rows);
                }
                DocMsg::PresenceUpdate { peer } => {
                    self.presence.insert(peer.actor.clone(), peer);
                    let _ = self.broadcast.send(HubServerFrame::Presence {
                        peers: self.presence.values().cloned().collect(),
                    });
                }
                DocMsg::PresenceLeave { actor } => {
                    if self.presence.remove(&actor).is_some() {
                        let _ = self.broadcast.send(HubServerFrame::Presence {
                            peers: self.presence.values().cloned().collect(),
                        });
                    }
                }
            }
        }
    }

    /// @emoji 🔁 Merges an incoming envelope's `vcs` block into the durable snapshot (structural replace).
    fn apply_envelope(&mut self, envelope: &Value) {
        if let Some(obj) = envelope.as_object() {
            if obj.get("vcs").is_some() {
                if let Some(snapshot) = self.snapshot.as_object_mut() {
                    if let Some(vcs) = obj.get("vcs") {
                        snapshot.insert("vcs".into(), vcs.clone());
                    }
                    if let Some(schema) = obj.get("schema").and_then(|value| value.as_str()) {
                        self.schema = schema.to_string();
                        snapshot.insert("schema".into(), Value::String(schema.into()));
                    }
                    if let Some(id) = obj.get("id").and_then(|value| value.as_str()) {
                        snapshot.insert("id".into(), Value::String(id.into()));
                    }
                    if let Some(backbone) = obj.get("backbone") {
                        snapshot.insert("backbone".into(), backbone.clone());
                    }
                    return;
                }
            }
        }
        self.snapshot = envelope.clone();
    }

    fn envelope_view(&self) -> Value {
        self.snapshot
            .get("vcs")
            .cloned()
            .map(|vcs| {
                serde_json::json!({
                    "schema": self.schema,
                    "id": self.snapshot.get("id").cloned().unwrap_or_else(|| Value::String(self.document_id.clone())),
                    "vcs": vcs,
                    "backbone": self.snapshot.get("backbone").cloned(),
                })
            })
            .unwrap_or_else(|| self.snapshot.clone())
    }
}

fn spawn_document_actor(document_id: String, storage: HubStorage) -> DocumentHandle {
    let (tx, rx) = mpsc::channel(256);
    let actor = DocumentActor::load(document_id, storage);
    tokio::spawn(actor.run(rx));
    DocumentHandle { tx }
}
//#endregion 🔖DocumentActor

//#region 🔖State
#[derive(Clone)]
struct HubState {
    storage: HubStorage,
    actors: Arc<DashMap<String, DocumentHandle>>,
    admin_token: Option<String>,
}

impl HubState {
    /// @emoji 🗂️ Returns the document's actor, spawning it lazily on first access (open-on-demand).
    fn actor(&self, document_id: &str) -> DocumentHandle {
        if let Some(existing) = self.actors.get(document_id) {
            return existing.clone();
        }
        self.actors
            .entry(document_id.to_string())
            .or_insert_with(|| spawn_document_actor(document_id.to_string(), self.storage.clone()))
            .clone()
    }
}

fn bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(|value| value.to_string())
}

fn default_snapshot() -> Value {
    serde_json::json!({
        "schema": "s.studio/v1",
        "id": "default",
        "name": "Studio",
        "vcs": {
            "initialProjection": {
                "programs": [],
                "activeProgramId": null,
                "activeAlternativeId": null,
                "appInstances": [],
                "mediaGraph": { "schema": "s.media-graph", "nodes": [], "edges": [] }
            },
            "operations": [],
            "checkpoints": [],
            "alternatives": []
        }
    })
}
//#endregion 🔖State

//#region 🔖Rest
#[derive(Serialize)]
struct DocumentResponse {
    snapshot: Value,
    version: i64,
}

#[derive(Serialize)]
struct EnvelopeResponse {
    envelope: Value,
    version: i64,
}

#[derive(Deserialize)]
struct PutEnvelopeRequest {
    version: i64,
    envelope: Value,
}

#[derive(Serialize)]
struct PutEnvelopeResponse {
    version: i64,
}

#[derive(Deserialize)]
struct AppendOpRequest {
    envelope: OpEnvelope,
}

#[derive(Serialize)]
struct AppendOpResponse {
    version: i64,
}

#[derive(Serialize)]
struct OpSinceRow {
    version: i64,
    envelope: OpEnvelope,
}

#[derive(Deserialize)]
struct SinceQuery {
    since: Option<i64>,
}

#[derive(Deserialize)]
struct NodesQuery {
    parent: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateNodeRequest {
    parent_id: Option<String>,
    name: String,
    kind: String,
}

#[derive(Serialize)]
struct ShareResponse {
    token: String,
}

async fn list_nodes(Query(query): Query<NodesQuery>, State(state): State<HubState>) -> Json<Vec<NodeRow>> {
    Json(state.storage.list_nodes(query.parent.as_deref()))
}

async fn create_node(State(state): State<HubState>, Json(body): Json<CreateNodeRequest>) -> Json<NodeRow> {
    Json(state.storage.create_node(body.parent_id.as_deref(), &body.name, &body.kind))
}

async fn get_document(
    Path(document_id): Path<String>,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Result<Json<DocumentResponse>, StatusCode> {
    if !state.storage.authorized(&document_id, bearer(&headers).as_deref()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let (snapshot, version) = state
        .actor(&document_id)
        .get_document()
        .await
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(DocumentResponse { snapshot, version }))
}

async fn get_envelope(
    Path(document_id): Path<String>,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Result<Json<EnvelopeResponse>, StatusCode> {
    if !state.storage.authorized(&document_id, bearer(&headers).as_deref()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let (envelope, version) = state
        .actor(&document_id)
        .get_envelope()
        .await
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(EnvelopeResponse { envelope, version }))
}

async fn put_envelope(
    Path(document_id): Path<String>,
    headers: HeaderMap,
    State(state): State<HubState>,
    Json(body): Json<PutEnvelopeRequest>,
) -> Result<Json<PutEnvelopeResponse>, StatusCode> {
    if !state.storage.authorized(&document_id, bearer(&headers).as_deref()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    match state.actor(&document_id).put_envelope(body.version, body.envelope).await {
        Ok(version) => Ok(Json(PutEnvelopeResponse { version })),
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

async fn append_op(
    Path(document_id): Path<String>,
    headers: HeaderMap,
    State(state): State<HubState>,
    Json(body): Json<AppendOpRequest>,
) -> Result<Json<AppendOpResponse>, StatusCode> {
    if !state.storage.authorized(&document_id, bearer(&headers).as_deref()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let origin = body.envelope.actor.0.clone();
    let appended = state.actor(&document_id).append_ops(vec![body.envelope], origin).await;
    let version = appended.last().map(|op| op.version).unwrap_or(0);
    Ok(Json(AppendOpResponse { version }))
}

async fn get_ops_since(
    Path(document_id): Path<String>,
    Query(query): Query<SinceQuery>,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Result<Json<Vec<OpSinceRow>>, StatusCode> {
    if !state.storage.authorized(&document_id, bearer(&headers).as_deref()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let rows = state
        .actor(&document_id)
        .ops_since(query.since.unwrap_or(0))
        .await
        .into_iter()
        .map(|(version, envelope)| OpSinceRow { version, envelope })
        .collect();
    Ok(Json(rows))
}

async fn create_share(
    Path(document_id): Path<String>,
    headers: HeaderMap,
    State(state): State<HubState>,
) -> Result<Json<ShareResponse>, StatusCode> {
    match state.admin_token.as_deref() {
        Some(expected) => {
            if bearer(&headers).as_deref() != Some(expected) {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        None => return Err(StatusCode::FORBIDDEN),
    }
    Ok(Json(ShareResponse {
        token: state.storage.create_share_token(&document_id),
    }))
}
//#endregion 🔖Rest

//#region 🔖WebSocket
async fn document_ws(
    ws: WebSocketUpgrade,
    Path(document_id): Path<String>,
    State(state): State<HubState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, document_id, state))
}

fn encode(frame: &HubServerFrame) -> Message {
    Message::Text(serde_json::to_string(frame).unwrap_or_default().into())
}

async fn handle_ws(socket: WebSocket, document_id: String, state: HubState) {
    let (mut sender, mut receiver) = socket.split();

    let hello = match receiver.next().await {
        Some(Ok(Message::Text(text))) => serde_json::from_str::<HubClientFrame>(&text).ok(),
        _ => None,
    };
    let (actor, token, since_version) = match hello {
        Some(HubClientFrame::Hello { actor, token, since_version }) => (actor, token, since_version),
        _ => {
            let _ = sender
                .send(encode(&HubServerFrame::Error {
                    message: "expected hello frame".into(),
                }))
                .await;
            return;
        }
    };
    if !state.storage.authorized(&document_id, token.as_deref()) {
        let _ = sender
            .send(encode(&HubServerFrame::Error {
                message: "unauthorized".into(),
            }))
            .await;
        return;
    }

    let handle = state.actor(&document_id);
    let sub = match handle.subscribe(since_version).await {
        Some(sub) => sub,
        None => return,
    };
    let mut broadcast_rx = sub.receiver;
    let welcome = HubServerFrame::Welcome {
        version: sub.version,
        envelope: if since_version == 0 { Some(sub.envelope) } else { None },
        presence: sub.presence,
        backlog: sub.backlog,
    };
    if sender.send(encode(&welcome)).await.is_err() {
        return;
    }

    let mut presence_joined = false;
    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<HubClientFrame>(&text) {
                            Ok(HubClientFrame::Ops { envelopes }) => {
                                let appended = handle.append_ops(envelopes, actor.clone()).await;
                                for op in appended {
                                    if op.is_new {
                                        if sender.send(encode(&HubServerFrame::Ack { op_id: op.op_id, version: op.version })).await.is_err() {
                                            handle.presence_leave(actor.clone()).await;
                                            return;
                                        }
                                    }
                                }
                            }
                            Ok(HubClientFrame::PutEnvelope { version, envelope }) => {
                                if let Err(current) = handle.put_envelope(version, envelope).await {
                                    if sender.send(encode(&HubServerFrame::Conflict { message: format!("stale version; current {current}") })).await.is_err() {
                                        handle.presence_leave(actor.clone()).await;
                                        return;
                                    }
                                }
                            }
                            Ok(HubClientFrame::Presence { peer }) => {
                                presence_joined = true;
                                handle.presence_update(peer).await;
                            }
                            Ok(HubClientFrame::Bye) => break,
                            Ok(HubClientFrame::Hello { .. }) | Err(_) => {}
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            event = broadcast_rx.recv() => {
                match event {
                    Ok(frame) => {
                        if sender.send(encode(&frame)).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
    if presence_joined {
        handle.presence_leave(actor).await;
    }
}
//#endregion 🔖WebSocket

//#region 🔖Main
fn router(state: HubState) -> Router {
    Router::new()
        .route("/nodes", get(list_nodes).post(create_node))
        .route("/documents/{id}", get(get_document))
        .route("/documents/{id}/envelope", get(get_envelope).put(put_envelope))
        .route("/documents/{id}/ops", post(append_op).get(get_ops_since))
        .route("/documents/{id}/share", post(create_share))
        .route("/documents/{id}/ws", get(document_ws))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let port: u16 = std::env::var("OS_HUB_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(6070);
    let db_path = std::env::var("OS_HUB_DB").unwrap_or_else(|_| "./.semio/hub.db".into());
    if db_path != ":memory:" {
        if let Some(parent) = std::path::Path::new(&db_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let storage = HubStorage::open(&db_path);
    storage.seed();
    let admin_token = std::env::var("OS_HUB_ADMIN_TOKEN").ok().filter(|value| !value.is_empty());
    let state = HubState {
        storage,
        actors: Arc::new(DashMap::new()),
        admin_token,
    };
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("os-hub listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, router(state)).await.expect("serve");
}
//#endregion 🔖Main

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    use semio_framework_core::{
        ActorId, DocumentDiff, DocumentId, DocumentVersion, InverseOperation, OperationId, PayloadHash,
        SchemaId, SchemaVersion, UndoPolicy,
    };
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message as WsMessage;

    fn temp_db_path() -> String {
        std::env::temp_dir()
            .join(format!("os-hub-test-{}.db", Uuid::now_v7()))
            .to_string_lossy()
            .into_owned()
    }

    fn memory_state() -> HubState {
        let storage = HubStorage::open(":memory:");
        storage.seed();
        HubState {
            storage,
            actors: Arc::new(DashMap::new()),
            admin_token: None,
        }
    }

    fn file_state(path: &str) -> HubState {
        let storage = HubStorage::open(path);
        storage.seed();
        HubState {
            storage,
            actors: Arc::new(DashMap::new()),
            admin_token: None,
        }
    }

    fn sample_envelope(id: &str) -> OpEnvelope {
        OpEnvelope {
            id: OperationId(id.into()),
            actor: ActorId("actor-1".into()),
            document: DocumentId("default".into()),
            schema_version: SchemaVersion("test.v1".into()),
            deps: Vec::new(),
            payload_hash: PayloadHash("hash".into()),
            diff: DocumentDiff {
                schema_id: SchemaId("diff.v1".into()),
                payload: serde_json::json!({ "value": id }),
            },
            inverse: InverseOperation {
                target_operation: OperationId(id.into()),
                inverse_diff: DocumentDiff {
                    schema_id: SchemaId("diff.v1".into()),
                    payload: serde_json::json!({}),
                },
                base_version: DocumentVersion(0),
                dependencies: Vec::new(),
                undo_policy: UndoPolicy::ExactBaseOnly,
            },
        }
    }

    async fn spawn_server(state: HubState) -> SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = router(state);
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        addr
    }

    async fn next_server_frame<S>(ws: &mut S) -> HubServerFrame
    where
        S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
    {
        loop {
            match ws.next().await {
                Some(Ok(WsMessage::Text(text))) => {
                    return serde_json::from_str::<HubServerFrame>(text.as_str()).expect("server frame")
                }
                Some(Ok(_)) => continue,
                other => panic!("expected text frame, got {other:?}"),
            }
        }
    }

    fn client_text(frame: &HubClientFrame) -> WsMessage {
        WsMessage::Text(serde_json::to_string(frame).unwrap().into())
    }

    // 🔬 WS duplex fan-out: A's op reaches B over its own socket.
    #[tokio::test]
    async fn ws_duplex_fan_out() {
        let addr = spawn_server(memory_state()).await;
        let url = format!("ws://{addr}/documents/default/ws");

        let (mut a, _) = connect_async(&url).await.unwrap();
        a.send(client_text(&HubClientFrame::Hello {
            actor: "A".into(),
            token: None,
            since_version: 0,
        }))
        .await
        .unwrap();
        assert!(matches!(next_server_frame(&mut a).await, HubServerFrame::Welcome { .. }));

        let (mut b, _) = connect_async(&url).await.unwrap();
        b.send(client_text(&HubClientFrame::Hello {
            actor: "B".into(),
            token: None,
            since_version: 0,
        }))
        .await
        .unwrap();
        assert!(matches!(next_server_frame(&mut b).await, HubServerFrame::Welcome { .. }));

        a.send(client_text(&HubClientFrame::Ops {
            envelopes: vec![sample_envelope("op-1")],
        }))
        .await
        .unwrap();

        // B must observe the op fanned out with origin "A".
        loop {
            match next_server_frame(&mut b).await {
                HubServerFrame::Ops { version, envelopes, origin } => {
                    assert_eq!(version, 1);
                    assert_eq!(envelopes.len(), 1);
                    assert_eq!(envelopes[0].id.0, "op-1");
                    assert_eq!(origin, "A");
                    break;
                }
                HubServerFrame::Presence { .. } => continue,
                other => panic!("unexpected frame on B: {other:?}"),
            }
        }
    }

    // 🔬 Persistence round-trip: ops survive a full server/state teardown against the same sqlite file.
    #[tokio::test]
    async fn persistence_round_trip_from_file() {
        let path = temp_db_path();
        {
            let state = file_state(&path);
            let handle = state.actor("default");
            for id in ["op-1", "op-2", "op-3"] {
                handle.append_ops(vec![sample_envelope(id)], "actor-1".into()).await;
            }
        }
        // Rebuild fresh state + actors against the same db file.
        let reopened = file_state(&path);
        let ops = reopened.actor("default").ops_since(0).await;
        assert_eq!(ops.len(), 3);
        assert_eq!(ops.iter().map(|(_, e)| e.id.0.clone()).collect::<Vec<_>>(), vec!["op-1", "op-2", "op-3"]);
        let _ = std::fs::remove_file(&path);
    }

    // 🔬 Op-id dedupe: the same envelope appended twice yields one row and one new append.
    #[tokio::test]
    async fn op_id_dedupe() {
        let state = memory_state();
        let handle = state.actor("default");
        let first = handle.append_ops(vec![sample_envelope("dup")], "actor-1".into()).await;
        let second = handle.append_ops(vec![sample_envelope("dup")], "actor-1".into()).await;
        assert!(first[0].is_new);
        assert!(!second[0].is_new);
        assert_eq!(state.storage.load_ops("default").len(), 1);
        assert_eq!(handle.ops_since(0).await.len(), 1);
    }

    // 🔬 Snapshot CAS: a stale-version envelope replace is rejected without mutating state.
    #[tokio::test]
    async fn snapshot_cas_conflict() {
        let state = memory_state();
        let handle = state.actor("default");
        let (_, version) = handle.get_document().await.unwrap();
        assert_eq!(version, 0);
        let envelope = serde_json::json!({ "schema": "s.studio/v1", "id": "default", "vcs": { "operations": [] } });
        assert_eq!(handle.put_envelope(0, envelope.clone()).await, Ok(1));
        // Stale base (0) now conflicts; current is 1.
        assert_eq!(handle.put_envelope(0, envelope.clone()).await, Err(1));
        let (_, after) = handle.get_document().await.unwrap();
        assert_eq!(after, 1, "state not corrupted by rejected CAS");
    }

    // 🔬 Op append never 409s on version mismatch (the bug fix): two "concurrent" appends both succeed.
    #[tokio::test]
    async fn op_append_never_version_conflicts() {
        let state = memory_state();
        let handle = state.actor("default");
        // Bump the structural version so a legacy client's base assumption (0) would mismatch.
        let envelope = serde_json::json!({ "schema": "s.studio/v1", "id": "default", "vcs": { "operations": [] } });
        assert_eq!(handle.put_envelope(0, envelope).await, Ok(1));
        // Both appends succeed regardless of any base-version assumption.
        let a = handle.append_ops(vec![sample_envelope("concurrent-a")], "A".into()).await;
        let b = handle.append_ops(vec![sample_envelope("concurrent-b")], "B".into()).await;
        assert!(a[0].is_new && a[0].version == 2);
        assert!(b[0].is_new && b[0].version == 3);
        assert_eq!(handle.ops_since(0).await.len(), 2);
    }

    // 🔬 REST op append assigns and returns an incrementing version.
    #[tokio::test]
    async fn rest_append_increments_version() {
        let state = memory_state();
        let response = append_op(
            Path("default".into()),
            HeaderMap::new(),
            State(state.clone()),
            Json(AppendOpRequest { envelope: sample_envelope("op-1") }),
        )
        .await
        .expect("append");
        assert_eq!(response.0.version, 1);
    }

    // 🔬 GET /ops?since= filters by assigned version.
    #[tokio::test]
    async fn rest_ops_since_filters() {
        let state = memory_state();
        let handle = state.actor("default");
        handle.append_ops(vec![sample_envelope("op-1")], "actor-1".into()).await;
        handle.append_ops(vec![sample_envelope("op-2")], "actor-1".into()).await;
        let all = get_ops_since(
            Path("default".into()),
            Query(SinceQuery { since: None }),
            HeaderMap::new(),
            State(state.clone()),
        )
        .await
        .unwrap();
        assert_eq!(all.0.len(), 2);
        let newer = get_ops_since(
            Path("default".into()),
            Query(SinceQuery { since: Some(1) }),
            HeaderMap::new(),
            State(state.clone()),
        )
        .await
        .unwrap();
        assert_eq!(newer.0.len(), 1);
        assert_eq!(newer.0[0].envelope.id.0, "op-2");
    }

    // 🔬 VFS nodes are durable and creatable.
    #[tokio::test]
    async fn nodes_create_and_list() {
        let state = memory_state();
        let created = create_node(
            State(state.clone()),
            Json(CreateNodeRequest { parent_id: None, name: "Projects".into(), kind: "folder".into() }),
        )
        .await;
        let child = create_node(
            State(state.clone()),
            Json(CreateNodeRequest {
                parent_id: Some(created.0.id.clone()),
                name: "sketch".into(),
                kind: "document".into(),
            }),
        )
        .await;
        let children = list_nodes(
            Query(NodesQuery { parent: Some(created.0.id.clone()) }),
            State(state.clone()),
        )
        .await;
        assert_eq!(children.0.len(), 1);
        assert_eq!(children.0[0].id, child.0.id);
    }

    // 🔬 Auth-lite: issuing a share token closes an otherwise-open document.
    #[tokio::test]
    async fn share_token_gates_access() {
        let storage = HubStorage::open(":memory:");
        storage.seed();
        let admin = HubState {
            storage: storage.clone(),
            actors: Arc::new(DashMap::new()),
            admin_token: Some("admin-secret".into()),
        };
        // Open before any token is issued.
        assert!(storage.authorized("guarded", None));
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, "Bearer admin-secret".parse().unwrap());
        let share = create_share(Path("guarded".into()), headers, State(admin.clone()))
            .await
            .expect("share");
        // Now closed to tokenless access, open with the minted token.
        assert!(!storage.authorized("guarded", None));
        assert!(storage.authorized("guarded", Some(&share.0.token)));
        // Wrong admin bearer is rejected.
        let mut bad = HeaderMap::new();
        bad.insert(axum::http::header::AUTHORIZATION, "Bearer nope".parse().unwrap());
        assert!(create_share(Path("guarded".into()), bad, State(admin)).await.is_err());
    }
}
//#endregion 🔖Tests
