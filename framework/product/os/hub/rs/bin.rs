mod header {
    // 🧲Header
    // OS hub v2 — pluggable-storage VFS + per-document op-log actors with duplex WebSocket sync.
    // CQRS split: op appends are causally ordered (OpDag) and never version-gated; only whole-envelope
    // snapshot replacement keeps optimistic concurrency (CAS → Conflict). Persistence is behind
    // {@link HubStorage} (os-hub-storage) — sqlite today, postgres/neo4j are sibling backends.
}

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use os_hub_storage::model::{BlobRecord, DocumentRecord, NodeRecord, StudioRole};
use os_hub_storage::HubStorage;
use os_hub_storage_sqlite::SqliteStorage;
use semio_framework_core::{HubClientFrame, HubServerFrame, OpDag, OpEnvelope, PresencePeer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, mpsc, oneshot};

//#region ⚠️ Errors
/// @emoji 🧯 Top-level startup error — the only fallible paths outside a per-document actor are
/// picking/connecting the storage backend and binding the HTTP listener.
#[derive(Debug, thiserror::Error)]
enum HubError {
    #[error(transparent)]
    Storage(#[from] os_hub_storage::error::StorageError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unknown OS_HUB_STORAGE_BACKEND: {0}")]
    UnknownStorageBackend(String),
}
//#endregion ⚠️ Errors

fn now_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

//#region 🔖DocumentActor
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
    Subscribe { since_version: i64, reply: oneshot::Sender<SubscribeReply> },
    AppendOps { envelopes: Vec<OpEnvelope>, origin: String, reply: oneshot::Sender<Vec<AppendedOp>> },
    PutEnvelope { version: i64, envelope: Value, reply: oneshot::Sender<Result<i64, i64>> },
    GetDocument { reply: oneshot::Sender<(Value, i64)> },
    GetEnvelope { reply: oneshot::Sender<(Value, i64)> },
    OpsSince { since: i64, reply: oneshot::Sender<Vec<(i64, OpEnvelope)>> },
    PresenceUpdate { peer: PresencePeer },
    PresenceLeave { actor: String },
}

/// @emoji 🎛️ Cheap clonable handle to a document's actor mailbox.
#[derive(Clone)]
struct DocumentHandle {
    tx: mpsc::Sender<DocMsg>,
}

impl DocumentHandle {
    async fn subscribe(&self, since_version: i64) -> Option<SubscribeReply> {
        let (reply, rx) = oneshot::channel();
        self.tx.send(DocMsg::Subscribe { since_version, reply }).await.ok()?;
        rx.await.ok()
    }

    async fn append_ops(&self, envelopes: Vec<OpEnvelope>, origin: String) -> Vec<AppendedOp> {
        let (reply, rx) = oneshot::channel();
        if self.tx.send(DocMsg::AppendOps { envelopes, origin, reply }).await.is_err() {
            return Vec::new();
        }
        rx.await.unwrap_or_default()
    }

    async fn put_envelope(&self, version: i64, envelope: Value) -> Result<i64, i64> {
        let (reply, rx) = oneshot::channel();
        if self.tx.send(DocMsg::PutEnvelope { version, envelope, reply }).await.is_err() {
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
/// the injected {@link HubStorage}.
struct DocumentActor {
    document_id: String,
    storage: Arc<dyn HubStorage>,
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
    async fn load(studio_id: String, document_id: String, storage: Arc<dyn HubStorage>) -> Result<Self, os_hub_storage::error::StorageError> {
        let record: DocumentRecord = storage.ensure_document(&studio_id, &document_id).await?;
        let ops = storage.load_ops(&document_id).await?;
        let mut dag = OpDag::new();
        let mut seen = HashSet::new();
        for (_, envelope) in &ops {
            let _ = dag.insert(envelope.clone());
            seen.insert(envelope.id.0.clone());
        }
        let (broadcast, _) = broadcast::channel(256);
        Ok(Self { document_id, storage, schema: record.schema, snapshot: record.snapshot, version: record.version, dag, ops, seen, presence: HashMap::new(), broadcast })
    }

    async fn run(mut self, mut rx: mpsc::Receiver<DocMsg>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                DocMsg::Subscribe { since_version, reply } => {
                    let backlog = self.ops.iter().filter(|(version, _)| *version > since_version).map(|(_, envelope)| envelope.clone()).collect();
                    let _ = reply.send(SubscribeReply { receiver: self.broadcast.subscribe(), version: self.version, envelope: self.snapshot.clone(), presence: self.presence.values().cloned().collect(), backlog });
                }
                DocMsg::AppendOps { envelopes, origin, reply } => {
                    let mut appended = Vec::new();
                    let mut fresh = Vec::new();
                    for envelope in envelopes {
                        let op_id = envelope.id.0.clone();
                        if self.seen.contains(&op_id) {
                            appended.push(AppendedOp { version: self.version, op_id, is_new: false });
                            continue;
                        }
                        let inserted = match self.storage.insert_op(&self.document_id, self.version + 1, &envelope).await {
                            Ok(inserted) => inserted,
                            Err(error) => {
                                tracing::error!(%error, op_id = %op_id, "failed to insert op; dropping from this batch");
                                continue;
                            }
                        };
                        if !inserted {
                            self.seen.insert(op_id.clone());
                            appended.push(AppendedOp { version: self.version, op_id, is_new: false });
                            continue;
                        }
                        self.version += 1;
                        let _ = self.dag.insert(envelope.clone());
                        self.seen.insert(op_id.clone());
                        self.ops.push((self.version, envelope.clone()));
                        appended.push(AppendedOp { version: self.version, op_id, is_new: true });
                        fresh.push(envelope);
                    }
                    if !fresh.is_empty() {
                        if let Err(error) = self.storage.save_document(&self.document_id, &self.schema, &self.snapshot, self.version).await {
                            tracing::error!(%error, document_id = %self.document_id, "failed to persist document snapshot after append");
                        }
                        let _ = self.broadcast.send(HubServerFrame::Ops { version: self.version, envelopes: fresh, origin });
                    }
                    let _ = reply.send(appended);
                }
                DocMsg::PutEnvelope { version, envelope, reply } => {
                    if version != self.version {
                        let _ = reply.send(Err(self.version));
                        continue;
                    }
                    self.version += 1;
                    self.apply_envelope(&envelope);
                    if let Err(error) = self.storage.save_document(&self.document_id, &self.schema, &self.snapshot, self.version).await {
                        tracing::error!(%error, document_id = %self.document_id, "failed to persist document snapshot after put");
                    }
                    let _ = self.broadcast.send(HubServerFrame::SnapshotReplaced { version: self.version, envelope: self.snapshot.clone() });
                    let _ = reply.send(Ok(self.version));
                }
                DocMsg::GetDocument { reply } => {
                    let _ = reply.send((self.snapshot.clone(), self.version));
                }
                DocMsg::GetEnvelope { reply } => {
                    let _ = reply.send((self.envelope_view(), self.version));
                }
                DocMsg::OpsSince { since, reply } => {
                    let rows = self.ops.iter().filter(|(version, _)| *version > since).map(|(version, envelope)| (*version, envelope.clone())).collect();
                    let _ = reply.send(rows);
                }
                DocMsg::PresenceUpdate { peer } => {
                    self.presence.insert(peer.actor.clone(), peer);
                    let _ = self.broadcast.send(HubServerFrame::Presence { peers: self.presence.values().cloned().collect() });
                }
                DocMsg::PresenceLeave { actor } => {
                    if self.presence.remove(&actor).is_some() {
                        let _ = self.broadcast.send(HubServerFrame::Presence { peers: self.presence.values().cloned().collect() });
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

/// @emoji 🌱 Spawns the actor's async load inside its own task so `HubState::actor` stays a
/// synchronous `DashMap` lookup — the mailbox channel exists immediately, `DocumentActor::load`'s
/// storage IO happens before the actor's `run` loop starts draining it.
fn spawn_document_actor(studio_id: String, document_id: String, storage: Arc<dyn HubStorage>) -> DocumentHandle {
    let (tx, rx) = mpsc::channel(256);
    tokio::spawn(async move {
        match DocumentActor::load(studio_id, document_id, storage).await {
            Ok(actor) => actor.run(rx).await,
            // rx is dropped here, closing the mailbox; every DocumentHandle method already
            // tolerates a closed channel (`.ok()?` / `.is_err()` / `unwrap_or_default()`).
            Err(error) => tracing::error!(%error, "failed to load document actor"),
        }
    });
    DocumentHandle { tx }
}
//#endregion 🔖DocumentActor

//#region 🔖State
#[derive(Clone)]
struct HubState {
    storage: Arc<dyn HubStorage>,
    /// @emoji 🔑 Keyed by `(studio_id, document_id)` — the same document id in two different studios
    /// gets two independent actors (own mailbox, own presence roster, own broadcast fan-out).
    actors: Arc<DashMap<(String, String), DocumentHandle>>,
    admin_token: Option<String>,
}

impl HubState {
    /// @emoji 🗂️ Returns the (studio, document) actor, spawning it lazily on first access (open-on-demand).
    fn actor(&self, studio_id: &str, document_id: &str) -> DocumentHandle {
        let key = (studio_id.to_string(), document_id.to_string());
        if let Some(existing) = self.actors.get(&key) {
            return existing.clone();
        }
        self.actors.entry(key.clone()).or_insert_with(|| spawn_document_actor(key.0.clone(), key.1.clone(), self.storage.clone())).clone()
    }
}

fn bearer(headers: &HeaderMap) -> Option<String> {
    headers.get(axum::http::header::AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).map(|value| value.to_string())
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

#[derive(Deserialize)]
struct CreateAuthSessionRequest {
    email: String,
}

#[derive(Serialize)]
struct CreateAuthSessionResponse {
    token: String,
    user_id: String,
}

async fn list_nodes(Path(studio_id): Path<String>, Query(query): Query<NodesQuery>, State(state): State<HubState>) -> Result<Json<Vec<NodeRecord>>, StatusCode> {
    state.storage.list_nodes(&studio_id, query.parent.as_deref()).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_node(Path(studio_id): Path<String>, State(state): State<HubState>, Json(body): Json<CreateNodeRequest>) -> Result<Json<NodeRecord>, StatusCode> {
    state.storage.create_node(&studio_id, body.parent_id.as_deref(), &body.name, &body.kind).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_document(Path((studio_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<DocumentResponse>, StatusCode> {
    if !authorized(&state, &studio_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let (snapshot, version) = state.actor(&studio_id, &document_id).get_document().await.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(DocumentResponse { snapshot, version }))
}

async fn get_envelope(Path((studio_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<EnvelopeResponse>, StatusCode> {
    if !authorized(&state, &studio_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let (envelope, version) = state.actor(&studio_id, &document_id).get_envelope().await.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(EnvelopeResponse { envelope, version }))
}

async fn put_envelope(Path((studio_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, Json(body): Json<PutEnvelopeRequest>) -> Result<Json<PutEnvelopeResponse>, StatusCode> {
    if !authorized(&state, &studio_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    match state.actor(&studio_id, &document_id).put_envelope(body.version, body.envelope).await {
        Ok(version) => Ok(Json(PutEnvelopeResponse { version })),
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

async fn append_op(Path((studio_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, Json(body): Json<AppendOpRequest>) -> Result<Json<AppendOpResponse>, StatusCode> {
    if !authorized(&state, &studio_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let origin = body.envelope.actor.0.clone();
    let appended = state.actor(&studio_id, &document_id).append_ops(vec![body.envelope], origin).await;
    let version = appended.last().map(|op| op.version).unwrap_or(0);
    Ok(Json(AppendOpResponse { version }))
}

async fn get_ops_since(Path((studio_id, document_id)): Path<(String, String)>, Query(query): Query<SinceQuery>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<Vec<OpSinceRow>>, StatusCode> {
    if !authorized(&state, &studio_id, &document_id, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let rows = state.actor(&studio_id, &document_id).ops_since(query.since.unwrap_or(0)).await.into_iter().map(|(version, envelope)| OpSinceRow { version, envelope }).collect();
    Ok(Json(rows))
}

async fn create_share(Path((_studio_id, document_id)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<Json<ShareResponse>, StatusCode> {
    match state.admin_token.as_deref() {
        Some(expected) => {
            if bearer(&headers).as_deref() != Some(expected) {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        None => return Err(StatusCode::FORBIDDEN),
    }
    let token = state.storage.create_share_token(&document_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ShareResponse { token }))
}

/// @emoji 🧪 Dev-mode session mint: trades a bare email for a bearer session token, upserting the
/// user if it doesn't exist yet. No password/SSO check — real SSO/OAuth is explicitly future scope;
/// this exists only so `AuthSessionRecord`-backed routes have a caller until that lands.
async fn create_auth_session(State(state): State<HubState>, Json(body): Json<CreateAuthSessionRequest>) -> Result<Json<CreateAuthSessionResponse>, StatusCode> {
    let user = match state.storage.get_user_by_email(&body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => state.storage.create_user(&body.email, &body.email, None, None, None).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let session = state.storage.create_auth_session(&user.id, 60 * 60 * 24 * 30, None).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CreateAuthSessionResponse { token: session.id, user_id: user.id }))
}

/// @emoji 🔎 What a bearer token resolved to: an authenticated studio member, an anonymous
/// share-token viewer, or nothing.
enum AuthOutcome {
    Session { user_id: String, role: StudioRole },
    ShareToken,
    Denied,
}

/// @emoji 🔐 Tries the bearer as an `AuthSessionRecord` (session id → user → studio role) first;
/// falls back to the existing anonymous share-token scheme when session resolution fails. Tokenless
/// documents stay open (dev default) until any share token is issued for them.
async fn resolve_auth(state: &HubState, studio_id: &str, document_id: &str, token: Option<&str>) -> AuthOutcome {
    if let Some(session_id) = token {
        if let Ok(Some(session)) = state.storage.get_auth_session(session_id).await {
            if session.expires_at > now_ms() {
                if let Ok(Some(role)) = state.storage.get_role(studio_id, &session.user_id).await {
                    return AuthOutcome::Session { user_id: session.user_id, role };
                }
            }
        }
    }
    match state.storage.authorized_by_token(document_id, token).await {
        Ok(true) => AuthOutcome::ShareToken,
        _ => AuthOutcome::Denied,
    }
}

async fn authorized(state: &HubState, studio_id: &str, document_id: &str, token: Option<&str>) -> bool {
    !matches!(resolve_auth(state, studio_id, document_id, token).await, AuthOutcome::Denied)
}

//#region Blobs
/// @emoji 📦 Studio-scoped blobs have no owning document, so this borrows `resolve_auth`'s
/// session→role branch as-is (studio role lookup never touches `document_id`) by passing the
/// blob hash in the document-id slot; the share-token branch then degrades to `Denied` unless a
/// document happens to share the blob's hash as its id, which content hashes never do in
/// practice. A session with any studio role is required — a document's share token intentionally
/// does not widen into read access over the whole studio's content-addressed blob store.
async fn authorized_for_blob(state: &HubState, studio_id: &str, hash: &str, token: Option<&str>) -> bool {
    !matches!(resolve_auth(state, studio_id, hash, token).await, AuthOutcome::Denied)
}

async fn put_blob(Path((studio_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>, body: Bytes) -> Result<Json<BlobRecord>, StatusCode> {
    if !authorized_for_blob(&state, &studio_id, &hash, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let media_type = headers.get(axum::http::header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap_or("application/octet-stream");
    let record = state.storage.put_blob(&body, media_type).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // The path hash is client-supplied (content-addressed URL); a mismatch against the
    // storage-computed hash means the client sent the wrong bytes for that address — a bad
    // request, distinct from `put_envelope`'s CONFLICT which signals a version CAS race.
    if record.hash != hash {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(Json(record))
}

/// @emoji 📭 `HubStorage::get_blob` returns only bytes (media type isn't retrievable on read —
/// see `HubStorage::put_blob`'s doc comment), so the response always serves as generic binary;
/// a typed content-type on GET needs a storage-trait change out of this ticket's bin.rs-only scope.
async fn get_blob(Path((studio_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
    if !authorized_for_blob(&state, &studio_id, &hash, bearer(&headers).as_deref()).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    match state.storage.get_blob(&hash).await {
        Ok(Some(bytes)) => Ok(([(axum::http::header::CONTENT_TYPE, "application/octet-stream")], bytes)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn head_blob(Path((studio_id, hash)): Path<(String, String)>, headers: HeaderMap, State(state): State<HubState>) -> StatusCode {
    if !authorized_for_blob(&state, &studio_id, &hash, bearer(&headers).as_deref()).await {
        return StatusCode::UNAUTHORIZED;
    }
    match state.storage.has_blob(&hash).await {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
//#endregion Blobs
//#endregion 🔖Rest

//#region 🔖WebSocket
async fn document_ws(ws: WebSocketUpgrade, Path((studio_id, document_id)): Path<(String, String)>, State(state): State<HubState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, studio_id, document_id, state))
}

fn encode(frame: &HubServerFrame) -> Message {
    Message::Text(serde_json::to_string(frame).unwrap_or_default().into())
}

async fn handle_ws(socket: WebSocket, studio_id: String, document_id: String, state: HubState) {
    let (mut sender, mut receiver) = socket.split();

    let hello = match receiver.next().await {
        Some(Ok(Message::Text(text))) => serde_json::from_str::<HubClientFrame>(&text).ok(),
        _ => None,
    };
    let (actor, token, since_version) = match hello {
        Some(HubClientFrame::Hello { actor, token, since_version }) => (actor, token, since_version),
        _ => {
            let _ = sender.send(encode(&HubServerFrame::Error { message: "expected hello frame".into() })).await;
            return;
        }
    };
    let auth = resolve_auth(&state, &studio_id, &document_id, token.as_deref()).await;
    let (user_id, role) = match &auth {
        AuthOutcome::Session { user_id, role } => (Some(user_id.clone()), Some(role.as_str().to_string())),
        AuthOutcome::ShareToken => (None, None),
        AuthOutcome::Denied => {
            let _ = sender.send(encode(&HubServerFrame::Error { message: "unauthorized".into() })).await;
            return;
        }
    };

    let handle = state.actor(&studio_id, &document_id);
    let sub = match handle.subscribe(since_version).await {
        Some(sub) => sub,
        None => return,
    };
    let mut broadcast_rx = sub.receiver;
    let welcome = HubServerFrame::Welcome { version: sub.version, envelope: if since_version == 0 { Some(sub.envelope) } else { None }, presence: sub.presence, backlog: sub.backlog };
    if sender.send(encode(&welcome)).await.is_err() {
        return;
    }

    // Register this connection in the presence roster on connect; richer Presence frames update it later.
    handle.presence_update(PresencePeer { actor: actor.clone(), label: None, selection_json: None, connected_at_ms: now_ms(), user_id, role, cursor: None, viewport: None, drag_ghost_json: None }).await;

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<HubClientFrame>(&text) {
                            Ok(HubClientFrame::Ops { envelopes }) => {
                                let appended = handle.append_ops(envelopes, actor.clone()).await;
                                for op in appended {
                                    if op.is_new && sender.send(encode(&HubServerFrame::Ack { op_id: op.op_id, version: op.version })).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Ok(HubClientFrame::PutEnvelope { version, envelope }) => {
                                if let Err(current) = handle.put_envelope(version, envelope).await {
                                    if sender.send(encode(&HubServerFrame::Conflict { message: format!("stale version; current {current}") })).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Ok(HubClientFrame::Presence { peer }) => {
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
    handle.presence_leave(actor).await;
}
//#endregion 🔖WebSocket

//#region 🔖Main
fn router(state: HubState) -> Router {
    Router::new()
        .route("/auth/sessions", post(create_auth_session))
        .route("/studios/{studio_id}/nodes", get(list_nodes).post(create_node))
        .route("/studios/{studio_id}/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/studios/{studio_id}/documents/{id}", get(get_document))
        .route("/studios/{studio_id}/documents/{id}/envelope", get(get_envelope).put(put_envelope))
        .route("/studios/{studio_id}/documents/{id}/ops", post(append_op).get(get_ops_since))
        .route("/studios/{studio_id}/documents/{id}/share", post(create_share))
        .route("/studios/{studio_id}/documents/{id}/ws", get(document_ws))
        .with_state(state)
}

/// @emoji 🧬 Resolves and connects the storage backend selected by `OS_HUB_STORAGE_BACKEND`
/// (`sqlite` default; `postgres` when `OS_HUB_DATABASE_URL` is set).
async fn connect_storage() -> Result<Arc<dyn HubStorage>, HubError> {
    let backend = std::env::var("OS_HUB_STORAGE_BACKEND").unwrap_or_else(|_| "sqlite".into());
    match backend.as_str() {
        "sqlite" | "" => {
            let db_path = std::env::var("OS_HUB_DB").unwrap_or_else(|_| "./.semio/hub.db".into());
            if db_path != ":memory:" {
                if let Some(parent) = std::path::Path::new(&db_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
            }
            let storage = SqliteStorage::connect(&db_path).await?;
            storage.seed().await?;
            Ok(Arc::new(storage))
        }
        "postgres" => {
            let database_url = std::env::var("OS_HUB_DATABASE_URL")
                .map_err(|_| HubError::UnknownStorageBackend("postgres requires OS_HUB_DATABASE_URL".into()))?;
            let storage = os_hub_storage_postgres::PostgresStorage::connect(&database_url)
                .await
                .map_err(|error| HubError::Storage(error))?;
            storage.seed().await?;
            Ok(Arc::new(storage))
        }
        other => Err(HubError::UnknownStorageBackend(other.to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), HubError> {
    tracing_subscriber::fmt::init();
    let port: u16 = std::env::var("OS_HUB_PORT").ok().and_then(|value| value.parse().ok()).unwrap_or(6070);
    let storage = connect_storage().await?;
    let admin_token = std::env::var("OS_HUB_ADMIN_TOKEN").ok().filter(|value| !value.is_empty());
    let state = HubState { storage, actors: Arc::new(DashMap::new()), admin_token };
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("os-hub listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router(state)).await?;
    Ok(())
}
//#endregion 🔖Main

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    use os_hub_storage_sqlite::SqliteStorage;
    use semio_framework_core::{ActorId, DocumentDiff, DocumentId, DocumentVersion, InverseOperation, OperationId, PayloadHash, SchemaId, SchemaVersion, UndoPolicy};
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message as WsMessage;
    use uuid::Uuid;

    /// @emoji 🏛️ The seeded studio id every test routes against (see `SqliteStorage::seed`).
    const STUDIO: &str = "default";

    fn temp_db_path() -> String {
        std::env::temp_dir().join(format!("os-hub-test-{}.db", Uuid::now_v7())).to_string_lossy().into_owned()
    }

    async fn memory_state() -> HubState {
        let storage = SqliteStorage::connect(":memory:").await.expect("connect");
        storage.seed().await.expect("seed");
        HubState { storage: Arc::new(storage), actors: Arc::new(DashMap::new()), admin_token: None }
    }

    async fn file_state(path: &str) -> HubState {
        let storage = SqliteStorage::connect(path).await.expect("connect");
        storage.seed().await.expect("seed");
        HubState { storage: Arc::new(storage), actors: Arc::new(DashMap::new()), admin_token: None }
    }

    fn sample_envelope(id: &str) -> OpEnvelope {
        OpEnvelope {
            id: OperationId(id.into()),
            actor: ActorId("actor-1".into()),
            document: DocumentId("default".into()),
            schema_version: SchemaVersion("test.v1".into()),
            deps: Vec::new(),
            payload_hash: PayloadHash("hash".into()),
            diff: DocumentDiff { schema_id: SchemaId("diff.v1".into()), payload: serde_json::json!({ "value": id }) },
            inverse: InverseOperation {
                target_operation: OperationId(id.into()),
                inverse_diff: DocumentDiff { schema_id: SchemaId("diff.v1".into()), payload: serde_json::json!({}) },
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
                Some(Ok(WsMessage::Text(text))) => return serde_json::from_str::<HubServerFrame>(text.as_str()).expect("server frame"),
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
        let addr = spawn_server(memory_state().await).await;
        let url = format!("ws://{addr}/studios/{STUDIO}/documents/default/ws");

        let (mut a, _) = connect_async(&url).await.unwrap();
        a.send(client_text(&HubClientFrame::Hello { actor: "A".into(), token: None, since_version: 0 })).await.unwrap();
        assert!(matches!(next_server_frame(&mut a).await, HubServerFrame::Welcome { .. }));

        let (mut b, _) = connect_async(&url).await.unwrap();
        b.send(client_text(&HubClientFrame::Hello { actor: "B".into(), token: None, since_version: 0 })).await.unwrap();
        assert!(matches!(next_server_frame(&mut b).await, HubServerFrame::Welcome { .. }));

        a.send(client_text(&HubClientFrame::Ops { envelopes: vec![sample_envelope("op-1")] })).await.unwrap();

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
            let state = file_state(&path).await;
            let handle = state.actor(STUDIO, "default");
            for id in ["op-1", "op-2", "op-3"] {
                handle.append_ops(vec![sample_envelope(id)], "actor-1".into()).await;
            }
        }
        // Rebuild fresh state + actors against the same db file.
        let reopened = file_state(&path).await;
        let ops = reopened.actor(STUDIO, "default").ops_since(0).await;
        assert_eq!(ops.len(), 3);
        assert_eq!(ops.iter().map(|(_, e)| e.id.0.clone()).collect::<Vec<_>>(), vec!["op-1", "op-2", "op-3"]);
        let _ = std::fs::remove_file(&path);
    }

    // 🔬 Op-id dedupe: the same envelope appended twice yields one row and one new append.
    #[tokio::test]
    async fn op_id_dedupe() {
        let state = memory_state().await;
        let handle = state.actor(STUDIO, "default");
        let first = handle.append_ops(vec![sample_envelope("dup")], "actor-1".into()).await;
        let second = handle.append_ops(vec![sample_envelope("dup")], "actor-1".into()).await;
        assert!(first[0].is_new);
        assert!(!second[0].is_new);
        assert_eq!(state.storage.load_ops("default").await.unwrap().len(), 1);
        assert_eq!(handle.ops_since(0).await.len(), 1);
    }

    // 🔬 Snapshot CAS: a stale-version envelope replace is rejected without mutating state.
    #[tokio::test]
    async fn snapshot_cas_conflict() {
        let state = memory_state().await;
        let handle = state.actor(STUDIO, "default");
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
        let state = memory_state().await;
        let handle = state.actor(STUDIO, "default");
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
        let state = memory_state().await;
        let response = append_op(Path((STUDIO.to_string(), "default".to_string())), HeaderMap::new(), State(state.clone()), Json(AppendOpRequest { envelope: sample_envelope("op-1") })).await.expect("append");
        assert_eq!(response.0.version, 1);
    }

    // 🔬 GET /ops?since= filters by assigned version.
    #[tokio::test]
    async fn rest_ops_since_filters() {
        let state = memory_state().await;
        let handle = state.actor(STUDIO, "default");
        handle.append_ops(vec![sample_envelope("op-1")], "actor-1".into()).await;
        handle.append_ops(vec![sample_envelope("op-2")], "actor-1".into()).await;
        let all = get_ops_since(Path((STUDIO.to_string(), "default".to_string())), Query(SinceQuery { since: None }), HeaderMap::new(), State(state.clone())).await.unwrap();
        assert_eq!(all.0.len(), 2);
        let newer = get_ops_since(Path((STUDIO.to_string(), "default".to_string())), Query(SinceQuery { since: Some(1) }), HeaderMap::new(), State(state.clone())).await.unwrap();
        assert_eq!(newer.0.len(), 1);
        assert_eq!(newer.0[0].envelope.id.0, "op-2");
    }

    // 🔬 VFS nodes are durable and creatable.
    #[tokio::test]
    async fn nodes_create_and_list() {
        let state = memory_state().await;
        let created = create_node(Path(STUDIO.to_string()), State(state.clone()), Json(CreateNodeRequest { parent_id: None, name: "Projects".into(), kind: "folder".into() })).await.expect("create");
        let child = create_node(Path(STUDIO.to_string()), State(state.clone()), Json(CreateNodeRequest { parent_id: Some(created.0.id.clone()), name: "sketch".into(), kind: "document".into() })).await.expect("create child");
        let children = list_nodes(Path(STUDIO.to_string()), Query(NodesQuery { parent: Some(created.0.id.clone()) }), State(state.clone())).await.expect("list");
        assert_eq!(children.0.len(), 1);
        assert_eq!(children.0[0].id, child.0.id);
    }

    // 🔬 Auth-lite: issuing a share token closes an otherwise-open document.
    #[tokio::test]
    async fn share_token_gates_access() {
        let storage = SqliteStorage::connect(":memory:").await.expect("connect");
        storage.seed().await.expect("seed");
        let admin = HubState { storage: Arc::new(storage), actors: Arc::new(DashMap::new()), admin_token: Some("admin-secret".into()) };
        // Open before any token is issued.
        assert!(admin.storage.authorized_by_token("guarded", None).await.unwrap());
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::AUTHORIZATION, "Bearer admin-secret".parse().unwrap());
        let share = create_share(Path((STUDIO.to_string(), "guarded".to_string())), headers, State(admin.clone())).await.expect("share");
        // Now closed to tokenless access, open with the minted token.
        assert!(!admin.storage.authorized_by_token("guarded", None).await.unwrap());
        assert!(admin.storage.authorized_by_token("guarded", Some(&share.0.token)).await.unwrap());
        // Wrong admin bearer is rejected.
        let mut bad = HeaderMap::new();
        bad.insert(axum::http::header::AUTHORIZATION, "Bearer nope".parse().unwrap());
        assert!(create_share(Path((STUDIO.to_string(), "guarded".to_string())), bad, State(admin)).await.is_err());
    }

    // 🔬 Studio-scoped actor keys: the same document id in two different studios gets independent
    // actors — HP-5's whole point (was a single flat DashMap<document_id, _> before this ticket).
    #[tokio::test]
    async fn studio_scoped_actors_are_isolated() {
        let state = memory_state().await;
        let handle_a = state.actor("studio-a", "shared-doc");
        let handle_b = state.actor("studio-b", "shared-doc");
        handle_a.append_ops(vec![sample_envelope("only-in-a")], "actor-1".into()).await;
        assert_eq!(handle_a.ops_since(0).await.len(), 1);
        assert_eq!(handle_b.ops_since(0).await.len(), 0, "studio-b's actor must not see studio-a's ops");
    }

    // 🔬 Auth sessions: POST /auth/sessions mints a session that resolves the caller's studio role
    // and grants access even to a document a share token has otherwise closed.
    #[tokio::test]
    async fn auth_session_grants_role_and_bypasses_share_gate() {
        let state = memory_state().await;
        let studio = "studio-x";
        let document = "closed-doc";
        state.storage.ensure_document(studio, document).await.expect("ensure document");
        state.storage.create_share_token(document).await.expect("close with share token");
        assert!(!state.storage.authorized_by_token(document, None).await.unwrap());

        let minted = create_auth_session(State(state.clone()), Json(CreateAuthSessionRequest { email: "dev@example.com".into() })).await.expect("mint session");
        state.storage.upsert_membership(studio, &minted.0.user_id, StudioRole::Member).await.expect("grant membership");

        assert!(!authorized(&state, studio, document, None).await, "tokenless request still denied");
        assert!(authorized(&state, studio, document, Some(&minted.0.token)).await, "session token authorized despite no share token");

        match resolve_auth(&state, studio, document, Some(&minted.0.token)).await {
            AuthOutcome::Session { user_id, role } => {
                assert_eq!(user_id, minted.0.user_id);
                assert_eq!(role, StudioRole::Member);
            }
            _ => panic!("expected a resolved session"),
        }
    }

    // 🔬 Blob round-trip: PUT then GET returns identical bytes and HEAD reports found; a hash
    // that was never PUT is reported missing by both GET and HEAD.
    #[tokio::test]
    async fn blob_put_get_head_round_trip() {
        let state = memory_state().await;
        let bytes = Bytes::from_static(b"hello hub blob bytes");
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_TYPE, "text/plain".parse().unwrap());

        // Learn the content hash the storage backend assigns; put_blob is idempotent so this
        // doesn't change what the HTTP-level put below observes.
        let expected = state.storage.put_blob(&bytes, "text/plain").await.expect("seed hash");

        let put = put_blob(Path((STUDIO.to_string(), expected.hash.clone())), headers.clone(), State(state.clone()), bytes.clone()).await.expect("put blob");
        assert_eq!(put.0.hash, expected.hash);
        assert_eq!(put.0.media_type, "text/plain");
        assert_eq!(put.0.size, bytes.len() as i64);

        let response = get_blob(Path((STUDIO.to_string(), expected.hash.clone())), HeaderMap::new(), State(state.clone())).await.expect("get blob").into_response();
        let got = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("read body");
        assert_eq!(got.as_ref(), bytes.as_ref());

        assert_eq!(head_blob(Path((STUDIO.to_string(), expected.hash.clone())), HeaderMap::new(), State(state.clone())).await, StatusCode::OK);

        let missing = "not-a-real-hash".to_string();
        assert_eq!(head_blob(Path((STUDIO.to_string(), missing.clone())), HeaderMap::new(), State(state.clone())).await, StatusCode::NOT_FOUND);
        assert_eq!(get_blob(Path((STUDIO.to_string(), missing)), HeaderMap::new(), State(state)).await.err(), Some(StatusCode::NOT_FOUND));
    }

    // 🔬 A client-provided hash that doesn't match the computed content hash is a bad request,
    // and the wrong path hash never gets associated with those bytes in storage.
    #[tokio::test]
    async fn blob_put_rejects_hash_mismatch() {
        let state = memory_state().await;
        let bytes = Bytes::from_static(b"mismatched content");
        let result = put_blob(Path((STUDIO.to_string(), "not-the-real-hash".to_string())), HeaderMap::new(), State(state.clone()), bytes.clone()).await;
        assert_eq!(result.err(), Some(StatusCode::BAD_REQUEST));
        assert!(!state.storage.has_blob("not-the-real-hash").await.unwrap(), "wrong path hash must not be a stored key");
    }
}
//#endregion 🔖Tests
