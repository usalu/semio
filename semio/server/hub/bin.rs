mod header { // 🧲Header
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Specs: Hub Protocol v1 (.repo/🎫/26/09/25/WORKING-SEMIO-HUB-COLLABORATION-END-TO-END/important.md): persons + tokens, sessions owning one authoritative native `semio` rs kit store each (single-writer actor), GraphQL operation replication with version + content hash, snapshots every 50 operations + replay, shares, websocket presence.
// Summary: semio-hub serves collaborative kit sessions over HTTP + websocket on top of the native `semio` crate. Postgres holds identities, sessions, the operations log and kit snapshots; kit content never leaves the rs store model.
} // 🧲Header

pub use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
pub use axum::extract::{DefaultBodyLimit, FromRequest, FromRequestParts, Query, Request, State};
pub use axum::http::{request::Parts, HeaderMap, StatusCode};
pub use axum::response::{IntoResponse, Response};
pub use axum::routing::{get, post};
pub use axum::{Json, Router};
pub use dashmap::DashMap;
pub use futures::{SinkExt, StreamExt};
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use serde_json::{json, Value};
pub use sqlx_postgres::{PgPool, PgPoolOptions, Postgres};
pub use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
pub use std::sync::Arc;
pub use std::time::{Duration, Instant};
pub use thiserror::Error;
pub use time::OffsetDateTime;
pub use tokio::sync::{broadcast, mpsc, oneshot};
pub use uuid::Uuid;

mod adapters { // 🔌Adapters
// Specs: Third-party capabilities beyond the web/db stack sit behind these interfaces: password KDF (argon2id), token minting + digest (OS entropy via uuid v4, blake3 via semio), GraphQL document inspection (async-graphql parser via semio).
// Summary: External library interfaces of the hub.

use super::*;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use semio::external_adapters::async_graphql::parser::parse_query;
use semio::external_adapters::async_graphql::parser::types::{ExecutableDocument, OperationType, Selection, SelectionSet};

/// 🔐 Password key-derivation interface.
pub trait PasswordKdf: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, HubError>;
    fn verify(&self, password: &str, encoded: &str) -> bool;
}

/// 🧂 Argon2id KDF ([RFC 9106](https://www.rfc-editor.org/rfc/rfc9106)); `fast` trades strength for test speed, `verify` honours the parameters encoded in each hash.
pub struct Argon2id(argon2::Argon2<'static>);

impl Argon2id {
    pub fn strong() -> Self {
        Self(argon2::Argon2::default())
    }

    pub fn fast() -> Self {
        Self(argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, argon2::Params::new(8, 1, 1, None).expect("argon2 params")))
    }
}

impl PasswordKdf for Argon2id {
    fn hash(&self, password: &str) -> Result<String, HubError> {
        let salt = SaltString::encode_b64(Uuid::new_v4().as_bytes()).map_err(|e| HubError::Internal(e.to_string()))?;
        self.0.hash_password(password.as_bytes(), &salt).map(|hash| hash.to_string()).map_err(|e| HubError::Internal(e.to_string()))
    }

    fn verify(&self, password: &str, encoded: &str) -> bool {
        PasswordHash::new(encoded).is_ok_and(|parsed| self.0.verify_password(password.as_bytes(), &parsed).is_ok())
    }
}

/// 🎟️ Opaque bearer token with 244 bits of OS entropy.
pub fn mint_token() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}

/// 🧮 Irreversible token digest (blake3) persisted instead of the token.
pub fn digest(token: &str) -> String {
    semio::external_adapters::blake3::hash(token.as_bytes()).to_hex().to_string()
}

/// 🔎 Shape of an accepted GraphQL document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentKind {
    Read,
    KitOperation,
}

const KIT_PATH: &[&str] = &["session", "store", "theKit", "unsavedChange", "kit"];
const FRAGMENT_DEPTH: usize = 16;

/// 🔎 Classifies a document: only queries (`Read`) or only mutations rooted at `session → store → theKit → unsavedChange → kit` (`KitOperation`).
pub fn inspect_document(query: &str) -> Result<DocumentKind, String> {
    let document = parse_query(query).map_err(|e| e.to_string())?;
    let operations: Vec<_> = document.operations.iter().map(|(_, operation)| &operation.node).collect();
    if operations.iter().all(|operation| operation.ty == OperationType::Query) {
        return Ok(DocumentKind::Read);
    }
    if operations.iter().all(|operation| operation.ty == OperationType::Mutation && kit_scoped(&document, &operation.selection_set.node, KIT_PATH, 0)) {
        return Ok(DocumentKind::KitOperation);
    }
    Err("operations must be kit-scoped mutations: session → store → theKit → unsavedChange → kit".into())
}

fn kit_scoped(document: &ExecutableDocument, set: &SelectionSet, path: &[&str], depth: usize) -> bool {
    let Some((head, rest)) = path.split_first() else { return true };
    depth < FRAGMENT_DEPTH
        && !set.items.is_empty()
        && set.items.iter().all(|item| match &item.node {
            Selection::Field(field) => field.node.name.node.as_str() == *head && kit_scoped(document, &field.node.selection_set.node, rest, depth),
            Selection::InlineFragment(fragment) => kit_scoped(document, &fragment.node.selection_set.node, path, depth + 1),
            Selection::FragmentSpread(spread) => document.fragments.get(&spread.node.fragment_name.node).is_some_and(|fragment| kit_scoped(document, &fragment.node.selection_set.node, path, depth + 1)),
        })
}

} // 🔌Adapters
pub use adapters::*;

mod error { // 🎼Error
// Specs: Every failure maps to one HTTP status and a `{error}` JSON body.
// Summary: Hub error type.

use super::*;

/// 🚨 Hub failure with its HTTP semantics.
#[derive(Debug, Error)]
pub enum HubError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Unprocessable(String),
    #[error("{0}")]
    Internal(String),
}

impl HubError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for HubError {
    fn into_response(self) -> Response {
        if let Self::Internal(message) = &self {
            tracing::error!("{message}");
        }
        (self.status(), Json(json!({ "error": self.to_string() }))).into_response()
    }
}

impl From<sqlx_core::Error> for HubError {
    fn from(error: sqlx_core::Error) -> Self {
        match error.as_database_error().and_then(|db| db.code()).as_deref() {
            Some("23505") => Self::Conflict("already exists".into()),
            _ => Self::Internal(error.to_string()),
        }
    }
}

} // 🎼Error
pub use error::*;

mod model { // 🗿Model
// Specs: Wire types of Hub Protocol v1 (camelCase JSON) and the resolved request callers.
// Summary: Roles, identities, sessions, operations, shares.

use super::*;

/// 🎖️ Session role; the ordering encodes privilege (`owner` > `editor` > `viewer`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Viewer,
    Editor,
    Owner,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Viewer => "viewer",
            Self::Editor => "editor",
            Self::Owner => "owner",
        }
    }

    pub fn parse(value: &str) -> Result<Self, HubError> {
        match value {
            "viewer" => Ok(Self::Viewer),
            "editor" => Ok(Self::Editor),
            "owner" => Ok(Self::Owner),
            other => Err(HubError::BadRequest(format!("unknown role `{other}`"))),
        }
    }
}

/// 🤝 Participant nature: people or AI agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantKind {
    Human,
    Agent,
}

impl ParticipantKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Agent => "agent",
        }
    }

    pub fn parse(value: &str) -> Self {
        if value == "agent" { Self::Agent } else { Self::Human }
    }
}

/// 👤 Registered person (public identity, mirrored in `semio.person`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub color: String,
}

/// 🪪 Authenticated person plus the participant kind its token grants.
#[derive(Debug, Clone)]
pub struct Principal {
    pub person: Person,
    pub kind: ParticipantKind,
}

/// 🔗 Read-only session access granted by a share token.
#[derive(Debug, Clone)]
pub struct ShareGrant {
    pub session_id: Uuid,
    pub role: Role,
}

/// 🎫 Resolved bearer of a request.
#[derive(Debug, Clone)]
pub enum Caller {
    Person(Principal),
    Share(ShareGrant),
}

impl Caller {
    pub fn principal(&self) -> Result<&Principal, HubError> {
        match self {
            Self::Person(principal) => Ok(principal),
            Self::Share(_) => Err(HubError::Unauthorized("a person token is required".into())),
        }
    }
}

/// 🔑 `POST /auth/register` + `POST /auth/login` result.
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub person: Person,
}

/// 🤖 `POST /auth/tokens` result.
#[derive(Debug, Clone, Serialize)]
pub struct AgentToken {
    pub token: String,
    pub label: String,
    pub kind: ParticipantKind,
}

/// 🏷️ Compact person reference.
#[derive(Debug, Clone, Serialize)]
pub struct PersonRef {
    pub id: Uuid,
    pub name: String,
}

/// 🏘️ Session as seen by one caller.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummary {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    pub owner: PersonRef,
    pub version: i64,
    pub hash: String,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub participant_count: usize,
}

/// 📦 Kit projection at a version.
#[derive(Debug, Clone, Serialize)]
pub struct KitState {
    pub version: i64,
    pub hash: String,
    pub kit: Value,
}

/// ✍️ Operation submitted by a client (`POST …/operations` or websocket `operation`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRequest {
    pub operation_id: String,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub base_version: Option<i64>,
    pub query: String,
    #[serde(default)]
    pub variables: Value,
}

/// ✅ Outcome of an accepted operation.
#[derive(Debug, Clone, Serialize)]
pub struct OperationResult {
    pub version: i64,
    pub hash: String,
    pub data: Value,
}

/// 📜 Logged operation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRecord {
    pub version: i64,
    pub operation_id: String,
    pub client_id: String,
    pub person_id: Uuid,
    pub participant_kind: ParticipantKind,
    pub query: String,
    pub variables: Value,
    pub hash: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

/// 🧑‍🤝‍🧑 Session member.
#[derive(Debug, Clone, Serialize)]
pub struct Member {
    pub person: Person,
    pub role: Role,
}

/// 🔗 Share link.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub token: String,
    pub role: Role,
    pub label: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

/// 🚪 `POST /shares/{token}/join` result.
#[derive(Debug, Clone, Serialize)]
pub struct Joined {
    pub session: SessionSummary,
    pub role: Role,
}

} // 🗿Model
pub use model::*;

mod schema { // 🎞️Schema
// Specs: `postgres/schema.sql` is the single source of truth and is executed verbatim at startup (idempotent DDL).
// Summary: Database connection + schema bootstrap.

use super::*;

pub const SCHEMA_SQL: &str = include_str!("postgres/schema.sql");

/// 🔌 Postgres connection pool.
pub async fn connect(database_url: &str) -> Result<PgPool, HubError> {
    Ok(PgPoolOptions::new().max_connections(16).connect(database_url).await?)
}

/// 🧱 Applies [`SCHEMA_SQL`].
pub async fn migrate(pool: &PgPool) -> Result<(), HubError> {
    sqlx_core::raw_sql::raw_sql(SCHEMA_SQL).execute(pool).await?;
    Ok(())
}

} // 🎞️Schema
pub use schema::*;

mod store { // 🧮Store
// Specs: One native semio rs kit store (`semio::worker::ParentStore`) driven exclusively through its GraphQL control plane, i.e. the same contract the WASM `KitStoreHandle` replicas use: `installProjection` to hydrate, `startNewChange` + kit-scoped mutations to change, `theKit.kit { hash projection }` to observe. `storeId`/`changeId` variables are always bound by the executing store.
// Summary: Authoritative kit store wrapper.

use super::*;
use semio::external_adapters::async_graphql::{Request, Variables};
use semio::gql::{build_schema_for, AppSchema};
use semio::worker::ParentStore;

const INSTALL: &str = "mutation($storeId: ID!, $json: String!) { session { store(id: $storeId) { installProjection(json: $json) { ok errors { message } } } } }";
const START_CHANGE: &str = "mutation($storeId: ID!) { session { store(id: $storeId) { theKit { startNewChange { ok errors { message } result { ... on IdResult { value } } } } } } }";
const HASH: &str = "query { session { stores { edges { node { wip { theKit { kit { hash } } } } } } } }";
const STATE: &str = "query { session { stores { edges { node { wip { theKit { kit { hash projection } } } } } } } }";
const KIT: &str = "/session/stores/edges/0/node/wip/theKit/kit";

/// 🏪 Authoritative kit store of one hub session.
pub struct KitStore {
    id: String,
    rt: Arc<ParentStore>,
    schema: AppSchema,
}

impl KitStore {
    /// 📥 Fresh store hydrated from a kit projection via `installProjection` (exactly how replicas load `GET …/kit`).
    pub async fn open(id: &str, kit: &Value) -> Result<Self, HubError> {
        let rt = ParentStore::spawn().await;
        let store = Self { id: id.to_string(), schema: build_schema_for(rt.clone()), rt };
        store.data(INSTALL, json!({ "json": kit.to_string() })).await?;
        Ok(store)
    }

    /// 🌐 Executes a GraphQL document with `storeId` bound to this store and returns the raw GraphQL response.
    pub async fn execute(&self, query: &str, variables: &Value) -> Value {
        let mut bound = variables.as_object().cloned().unwrap_or_default();
        bound.insert("storeId".into(), Value::String(self.id.clone()));
        let request = Request::new(query).variables(Variables::from_json(Value::Object(bound))).data(self.rt.clone()).data(self.rt.bus.clone());
        serde_json::to_value(self.schema.execute(request).await).unwrap_or_else(|e| json!({ "errors": [{ "message": e.to_string() }] }))
    }

    async fn data(&self, query: &str, variables: Value) -> Result<Value, HubError> {
        let response = self.execute(query, &variables).await;
        let failures = failures(&response);
        if failures.is_empty() { Ok(response.get("data").cloned().unwrap_or(Value::Null)) } else { Err(HubError::Unprocessable(failures.join("; "))) }
    }

    /// 🧾 Applies one kit-scoped operation inside a fresh change bound to `changeId`; returns the response `data`.
    pub async fn apply(&self, query: &str, variables: &Value) -> Result<Value, HubError> {
        let started = self.data(START_CHANGE, json!({})).await?;
        let change = started.pointer("/session/store/theKit/startNewChange/result/value").cloned().ok_or_else(|| HubError::Internal("startNewChange returned no change id".into()))?;
        let mut bound = variables.as_object().cloned().unwrap_or_default();
        bound.insert("changeId".into(), change);
        self.data(query, Value::Object(bound)).await
    }

    /// 🪪 Content hash of the materialized `theKit.kit`.
    pub async fn hash(&self) -> Result<String, HubError> {
        let data = self.data(HASH, json!({})).await?;
        data.pointer(&format!("{KIT}/hash")).and_then(Value::as_str).map(str::to_string).ok_or_else(|| HubError::Internal("store returned no kit hash".into()))
    }

    /// 📦 Content hash plus canonical projection of the materialized `theKit.kit`.
    pub async fn state(&self) -> Result<(String, Value), HubError> {
        let data = self.data(STATE, json!({})).await?;
        let kit = data.pointer(KIT).ok_or_else(|| HubError::Internal("store returned no kit".into()))?;
        let hash = kit.get("hash").and_then(Value::as_str).ok_or_else(|| HubError::Internal("store returned no kit hash".into()))?;
        let projection = kit.get("projection").and_then(Value::as_str).ok_or_else(|| HubError::Internal("store returned no kit projection".into()))?;
        Ok((hash.to_string(), serde_json::from_str(projection).map_err(|e| HubError::Internal(e.to_string()))?))
    }
}

/// 🆕 Projection of an empty kit.
pub fn empty_kit(name: &str) -> Value {
    json!({ "id": Uuid::now_v7(), "name": name })
}

/// 🧯 GraphQL errors plus the messages of every rs `Response` with `ok: false` inside `data`.
pub fn failures(response: &Value) -> Vec<String> {
    let mut out = messages(response.get("errors"));
    collect_not_ok(response.get("data").unwrap_or(&Value::Null), &mut out);
    out
}

fn messages(errors: Option<&Value>) -> Vec<String> {
    errors.and_then(Value::as_array).into_iter().flatten().filter_map(|error| error.get("message").and_then(Value::as_str).map(str::to_string)).collect()
}

fn collect_not_ok(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if map.get("ok") == Some(&Value::Bool(false)) {
                let found = messages(map.get("errors"));
                if found.is_empty() { out.push("operation failed".into()) } else { out.extend(found) }
            }
            map.values().for_each(|child| collect_not_ok(child, out));
        }
        Value::Array(items) => items.iter().for_each(|child| collect_not_ok(child, out)),
        _ => {}
    }
}

} // 🧮Store
pub use store::*;

mod presence { // 👥Presence
// Specs: Presence lives in memory per session room: participants (people over websockets, agents over MCP) plus one broadcast channel carrying serialized server messages to every socket of the session. Socketless agent participants expire after [`AGENT_IDLE`].
// Summary: Participants, server messages and rooms.

use super::*;

pub const AGENT_IDLE: Duration = Duration::from_secs(120);
const COLORS: [&str; 12] = ["#e6194b", "#3cb44b", "#4363d8", "#f58231", "#911eb4", "#42d4f4", "#f032e6", "#9a6324", "#469990", "#800000", "#808000", "#000075"];

/// 🎨 Stable color of a person.
pub fn person_color(id: Uuid) -> String {
    COLORS[(id.as_u128() % COLORS.len() as u128) as usize].to_string()
}

/// 🧑‍💻 Live session participant.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub id: Uuid,
    pub person_id: Uuid,
    pub name: String,
    pub color: String,
    pub kind: ParticipantKind,
    pub client: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Value>,
    #[serde(with = "time::serde::rfc3339")]
    pub joined_at: OffsetDateTime,
    #[serde(skip)]
    pub client_id: String,
    #[serde(skip)]
    pub socketless: bool,
    #[serde(skip)]
    pub last_seen: Instant,
}

/// 📣 Server → client websocket message.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum ServerMessage {
    #[serde(rename = "welcome")]
    Welcome {
        #[serde(rename = "self")]
        me: Participant,
        participants: Vec<Participant>,
        version: i64,
        hash: String,
    },
    #[serde(rename = "presence.joined")]
    PresenceJoined { participant: Participant },
    #[serde(rename = "presence.left")]
    PresenceLeft { participant_id: Uuid },
    #[serde(rename = "presence.updated")]
    PresenceUpdated { participant: Participant },
    #[serde(rename = "operation")]
    Operation { version: i64, hash: String, operation_id: String, client_id: String, participant_id: Option<Uuid>, person_id: Uuid, query: String, variables: Value },
    #[serde(rename = "error")]
    Error {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        operation_id: Option<String>,
    },
    #[serde(rename = "pong")]
    Pong,
}

/// 🏠 Per-session participant roster plus broadcast channel.
pub struct Room {
    tx: broadcast::Sender<Arc<str>>,
    roster: std::sync::Mutex<Vec<Participant>>,
    closed: AtomicBool,
}

impl Default for Room {
    fn default() -> Self {
        Self { tx: broadcast::channel(1024).0, roster: std::sync::Mutex::new(Vec::new()), closed: AtomicBool::new(false) }
    }
}

impl Room {
    fn roster(&self) -> std::sync::MutexGuard<'_, Vec<Participant>> {
        self.roster.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<str>> {
        self.tx.subscribe()
    }

    pub fn send(&self, message: &ServerMessage) {
        if let Ok(text) = serde_json::to_string(message) {
            let _ = self.tx.send(text.into());
        }
    }

    pub fn participants(&self) -> Vec<Participant> {
        self.roster().clone()
    }

    pub fn len(&self) -> usize {
        self.roster().len()
    }

    pub fn is_empty(&self) -> bool {
        self.roster().is_empty()
    }

    pub fn add(&self, participant: Participant) {
        self.roster().push(participant.clone());
        self.send(&ServerMessage::PresenceJoined { participant });
    }

    /// 👆 Finds a participant and marks it active.
    pub fn touch(&self, matches: impl Fn(&Participant) -> bool) -> Option<Participant> {
        let mut roster = self.roster();
        let participant = roster.iter_mut().find(|participant| matches(participant))?;
        participant.last_seen = Instant::now();
        Some(participant.clone())
    }

    /// 🖱️ Merges `focus` / `selection` / `cursor` of `patch` (explicit `null` clears) and broadcasts `presence.updated`.
    pub fn update(&self, id: Uuid, patch: &Value) -> Option<Participant> {
        let updated = {
            let mut roster = self.roster();
            let participant = roster.iter_mut().find(|participant| participant.id == id)?;
            for (key, slot) in [("focus", &mut participant.focus), ("selection", &mut participant.selection), ("cursor", &mut participant.cursor)] {
                if let Some(value) = patch.get(key) {
                    *slot = if value.is_null() { None } else { Some(value.clone()) };
                }
            }
            participant.last_seen = Instant::now();
            participant.clone()
        };
        self.send(&ServerMessage::PresenceUpdated { participant: updated.clone() });
        Some(updated)
    }

    pub fn remove(&self, id: Uuid) -> bool {
        let removed = {
            let mut roster = self.roster();
            let before = roster.len();
            roster.retain(|participant| participant.id != id);
            before != roster.len()
        };
        if removed {
            self.send(&ServerMessage::PresenceLeft { participant_id: id });
        }
        removed
    }

    /// 🧹 Drops socketless participants idle longer than `idle`.
    pub fn sweep(&self, idle: Duration) {
        let stale: Vec<Uuid> = self.roster().iter().filter(|participant| participant.socketless && participant.last_seen.elapsed() > idle).map(|participant| participant.id).collect();
        stale.into_iter().for_each(|id| {
            self.remove(id);
        });
    }

    pub fn close(&self, message: &str) {
        self.closed.store(true, Ordering::SeqCst);
        self.send(&ServerMessage::Error { message: message.into(), operation_id: None });
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }
}

} // 👥Presence
pub use presence::*;

mod actor { // 🎹Actor
// Specs: One single-writer actor per loaded session owns the authoritative [`KitStore`]. Operations are applied in arrival order, logged with a monotonically increasing version and the resulting content hash, and broadcast to the session room. Every [`SNAPSHOT_INTERVAL`] versions the kit projection is snapshotted and the live store is re-based onto it, so a reload (latest snapshot + replay) always reproduces the live store. A failed operation restores the store from the log. Idle actors of empty rooms passivate.
// Summary: Session actor, commands and snapshot/replay rebuild.

use super::*;
use sqlx_core::query::query;
use sqlx_core::query_as::query_as;

pub const SNAPSHOT_INTERVAL: i64 = 50;
pub const ACTOR_IDLE: Duration = Duration::from_secs(600);

pub type Actors = Arc<DashMap<Uuid, (u64, mpsc::Sender<ActorCommand>)>>;

/// ✉️ Actor inbox message.
pub enum ActorCommand {
    Operation { author: Author, request: OperationRequest, reply: oneshot::Sender<Result<OperationResult, HubError>> },
    Query { query: String, variables: Value, reply: oneshot::Sender<Result<Value, HubError>> },
    Kit { reply: oneshot::Sender<Result<KitState, HubError>> },
}

/// ✒️ Who performs an operation.
#[derive(Debug, Clone)]
pub struct Author {
    pub person_id: Uuid,
    pub kind: ParticipantKind,
    pub participant_id: Option<Uuid>,
}

/// 🎹 Single writer of one session.
pub struct SessionActor {
    session_id: Uuid,
    pool: PgPool,
    room: Arc<Room>,
    store: KitStore,
    version: i64,
    hash: String,
}

/// 🔁 Materializes a session at `version`: latest snapshot at or below it plus the logged operations after it.
pub async fn rebuild(pool: &PgPool, session_id: Uuid, version: i64) -> Result<(KitStore, String), HubError> {
    let (base, kit): (i64, Value) = query_as("SELECT version, kit FROM semio.snapshot WHERE session_id = $1 AND version <= $2 ORDER BY version DESC LIMIT 1")
        .bind(session_id)
        .bind(version)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| HubError::NotFound(format!("no snapshot for session {session_id} at version {version}")))?;
    let operations: Vec<(i64, String, Value)> = query_as("SELECT version, query, variables FROM semio.operation WHERE session_id = $1 AND version > $2 AND version <= $3 ORDER BY version")
        .bind(session_id)
        .bind(base)
        .bind(version)
        .fetch_all(pool)
        .await?;
    let store = KitStore::open(&session_id.to_string(), &kit).await?;
    for (logged, text, variables) in operations {
        store.apply(&text, &variables).await.map_err(|e| HubError::Internal(format!("replaying version {logged} of session {session_id} failed: {e}")))?;
    }
    let hash = store.hash().await?;
    Ok((store, hash))
}

impl SessionActor {
    pub fn new(pool: PgPool, room: Arc<Room>, session_id: Uuid, store: KitStore, version: i64, hash: String) -> Self {
        Self { session_id, pool, room, store, version, hash }
    }

    /// 📂 Loads the session from its latest snapshot plus replay.
    pub async fn load(pool: PgPool, room: Arc<Room>, session_id: Uuid) -> Result<Self, HubError> {
        let (version, logged): (i64, String) = query_as("SELECT version, hash FROM semio.session WHERE id = $1").bind(session_id).fetch_optional(&pool).await?.ok_or_else(|| HubError::NotFound("session not found".into()))?;
        let (store, hash) = rebuild(&pool, session_id, version).await?;
        if hash != logged {
            tracing::warn!("session {session_id} rebuilt to hash {hash} but the log recorded {logged}");
        }
        Ok(Self::new(pool, room, session_id, store, version, hash))
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<ActorCommand>, actors: Actors, generation: u64) {
        loop {
            let command = match tokio::time::timeout(ACTOR_IDLE, rx.recv()).await {
                Ok(Some(command)) => command,
                Ok(None) => break,
                Err(_) => {
                    if self.room.is_empty() {
                        actors.remove_if(&self.session_id, |_, (current, _)| *current == generation);
                    }
                    continue;
                }
            };
            match command {
                ActorCommand::Operation { author, request, reply } => {
                    let _ = reply.send(self.operation(author, request).await);
                }
                ActorCommand::Query { query, variables, reply } => {
                    let _ = reply.send(Ok(self.store.execute(&query, &variables).await));
                }
                ActorCommand::Kit { reply } => {
                    let _ = reply.send(self.store.state().await.map(|(hash, kit)| KitState { version: self.version, hash, kit }));
                }
            }
        }
        tracing::debug!("session actor {} stopped", self.session_id);
    }

    async fn operation(&mut self, author: Author, request: OperationRequest) -> Result<OperationResult, HubError> {
        let logged: Option<(i64, String, Value)> = query_as("SELECT version, hash, data FROM semio.operation WHERE session_id = $1 AND operation_id = $2").bind(self.session_id).bind(&request.operation_id).fetch_optional(&self.pool).await?;
        if let Some((version, hash, data)) = logged {
            return Ok(OperationResult { version, hash, data });
        }
        if let Some(base) = request.base_version.filter(|base| *base > self.version) {
            return Err(HubError::Conflict(format!("baseVersion {base} is ahead of session version {}", self.version)));
        }
        let variables = if request.variables.is_null() { json!({}) } else { request.variables.clone() };
        let version = self.version + 1;
        let applied = match self.store.apply(&request.query, &variables).await {
            Ok(data) => self.store.hash().await.map(|hash| (data, hash)),
            Err(error) => Err(error),
        };
        let persisted = match applied {
            Ok((data, hash)) => self.persist(&author, &request, &variables, &data, version, &hash).await.map(|_| (data, hash)),
            Err(error) => Err(error),
        };
        let (data, hash) = match persisted {
            Ok(done) => done,
            Err(error) => {
                self.restore().await;
                return Err(error);
            }
        };
        self.version = version;
        self.hash = hash.clone();
        if version % SNAPSHOT_INTERVAL == 0 {
            if let Err(error) = self.snapshot().await {
                tracing::warn!("snapshot of session {} at version {version} failed: {error}", self.session_id);
            }
        }
        self.room.send(&ServerMessage::Operation {
            version,
            hash: hash.clone(),
            operation_id: request.operation_id,
            client_id: request.client_id,
            participant_id: author.participant_id,
            person_id: author.person_id,
            query: request.query,
            variables,
        });
        Ok(OperationResult { version, hash, data })
    }

    async fn persist(&self, author: &Author, request: &OperationRequest, variables: &Value, data: &Value, version: i64, hash: &str) -> Result<(), HubError> {
        let mut tx = self.pool.begin().await?;
        query("INSERT INTO semio.operation (session_id, version, operation_id, client_id, person_id, participant_kind, query, variables, data, hash) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
            .bind(self.session_id)
            .bind(version)
            .bind(&request.operation_id)
            .bind(&request.client_id)
            .bind(author.person_id)
            .bind(author.kind.as_str())
            .bind(&request.query)
            .bind(variables)
            .bind(data)
            .bind(hash)
            .execute(&mut *tx)
            .await?;
        query("UPDATE semio.session SET version = $2, hash = $3, updated_at = now() WHERE id = $1").bind(self.session_id).bind(version).bind(hash).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn snapshot(&mut self) -> Result<(), HubError> {
        let (hash, kit) = self.store.state().await?;
        query("INSERT INTO semio.snapshot (session_id, version, hash, kit) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING").bind(self.session_id).bind(self.version).bind(&hash).bind(&kit).execute(&self.pool).await?;
        let store = KitStore::open(&self.session_id.to_string(), &kit).await?;
        let rebased = store.hash().await?;
        if rebased != self.hash {
            tracing::warn!("session {} changed hash while re-basing onto snapshot {}: {} → {rebased}", self.session_id, self.version, self.hash);
            query("UPDATE semio.session SET hash = $2 WHERE id = $1").bind(self.session_id).bind(&rebased).execute(&self.pool).await?;
            self.hash = rebased;
        }
        self.store = store;
        Ok(())
    }

    async fn restore(&mut self) {
        match rebuild(&self.pool, self.session_id, self.version).await {
            Ok((store, hash)) => {
                self.store = store;
                self.hash = hash;
            }
            Err(error) => tracing::error!("restoring session {} failed: {error}", self.session_id),
        }
    }
}

} // 🎹Actor
pub use actor::*;

mod hub { // 🏛️Hub
// Specs: `Hub` is the process-wide service (axum state) and the internal API shared by HTTP, websocket and MCP: every method takes a resolved [`Caller`] and enforces roles (401 unauthenticated, 404 unknown session, 403 insufficient role; share tokens grant read-only access to their session).
// Summary: Auth, sessions, operations, history, shares and presence.

use super::*;
use sqlx_core::query::query;
use sqlx_core::query_as::query_as;
use sqlx_core::query_scalar::query_scalar;

const ISSUE_TOKEN: &str = "INSERT INTO semio.token (hash, person_id, kind, label, expires_at) VALUES ($1, $2, $3, $4, CASE WHEN $3 = 'login' THEN now() + interval '30 days' END)";
const SESSION_SELECT: &str = "SELECT s.id, s.name, s.owner_id, o.name, s.version, s.hash, s.updated_at FROM semio.session s JOIN semio.person o ON o.id = s.owner_id";

type SessionRow = (Uuid, String, Uuid, String, i64, String, OffsetDateTime);

/// 🏛️ Hub service state.
#[derive(Clone)]
pub struct Hub {
    pub pool: PgPool,
    kdf: Arc<dyn PasswordKdf>,
    actors: Actors,
    rooms: Arc<DashMap<Uuid, Arc<Room>>>,
    loading: Arc<tokio::sync::Mutex<()>>,
    generation: Arc<AtomicU64>,
}

impl Hub {
    /// 🏗️ Creates the hub and starts the agent presence sweeper.
    pub fn new(pool: PgPool, kdf: Arc<dyn PasswordKdf>) -> Self {
        let hub = Self { pool, kdf, actors: Arc::default(), rooms: Arc::default(), loading: Arc::default(), generation: Arc::default() };
        let rooms = Arc::downgrade(&hub.rooms);
        tokio::spawn(async move {
            let mut ticks = tokio::time::interval(Duration::from_secs(10));
            loop {
                ticks.tick().await;
                let Some(rooms) = rooms.upgrade() else { break };
                rooms.iter().for_each(|room| room.sweep(AGENT_IDLE));
            }
        });
        hub
    }

    //#region 🔑Auth

    /// 🆕 Registers a person with a password and returns a login token.
    pub async fn register(&self, name: &str, email: &str, password: &str) -> Result<AuthResponse, HubError> {
        let (name, email) = (name.trim(), email.trim().to_lowercase());
        if name.is_empty() {
            return Err(HubError::BadRequest("name is required".into()));
        }
        if !email.contains('@') {
            return Err(HubError::BadRequest("a valid email is required".into()));
        }
        if password.chars().count() < 8 {
            return Err(HubError::BadRequest("password must have at least 8 characters".into()));
        }
        let (kdf, secret) = (self.kdf.clone(), password.to_string());
        let password_hash = tokio::task::spawn_blocking(move || kdf.hash(&secret)).await.map_err(|e| HubError::Internal(e.to_string()))??;
        let id = Uuid::now_v7();
        let person = Person { id, name: name.into(), email, color: person_color(id) };
        let token = mint_token();
        let mut tx = self.pool.begin().await?;
        query("INSERT INTO semio.person (id, name, email, color) VALUES ($1, $2, $3, $4)")
            .bind(id)
            .bind(&person.name)
            .bind(&person.email)
            .bind(&person.color)
            .execute(&mut *tx)
            .await
            .map_err(|e| match HubError::from(e) {
                HubError::Conflict(_) => HubError::Conflict("email is already registered".into()),
                other => other,
            })?;
        query("INSERT INTO semio.credential (person_id, password_hash) VALUES ($1, $2)").bind(id).bind(password_hash).execute(&mut *tx).await?;
        query(ISSUE_TOKEN).bind(digest(&token)).bind(id).bind("login").bind(None::<String>).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(AuthResponse { token, person })
    }

    /// 🔓 Exchanges email + password for a login token.
    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse, HubError> {
        let row: Option<(Uuid, String, String, String, String)> = query_as("SELECT p.id, p.name, p.email, p.color, c.password_hash FROM semio.person p JOIN semio.credential c ON c.person_id = p.id WHERE p.email = $1")
            .bind(email.trim().to_lowercase())
            .fetch_optional(&self.pool)
            .await?;
        let invalid = || HubError::Unauthorized("invalid email or password".into());
        let (id, name, email, color, encoded) = row.ok_or_else(invalid)?;
        let (kdf, secret) = (self.kdf.clone(), password.to_string());
        if !tokio::task::spawn_blocking(move || kdf.verify(&secret, &encoded)).await.map_err(|e| HubError::Internal(e.to_string()))? {
            return Err(invalid());
        }
        let token = mint_token();
        query(ISSUE_TOKEN).bind(digest(&token)).bind(id).bind("login").bind(None::<String>).execute(&self.pool).await?;
        Ok(AuthResponse { token, person: Person { id, name, email, color } })
    }

    /// 🎫 Resolves a bearer token (person login/agent token or share token).
    pub async fn authenticate(&self, token: &str) -> Result<Caller, HubError> {
        let person: Option<(Uuid, String, String, String, String)> = query_as("SELECT p.id, p.name, p.email, p.color, t.kind FROM semio.token t JOIN semio.person p ON p.id = t.person_id WHERE t.hash = $1 AND (t.expires_at IS NULL OR t.expires_at > now())")
            .bind(digest(token))
            .fetch_optional(&self.pool)
            .await?;
        if let Some((id, name, email, color, kind)) = person {
            return Ok(Caller::Person(Principal { person: Person { id, name, email, color }, kind: ParticipantKind::parse(&kind) }));
        }
        let share: Option<(Uuid, String)> = query_as("SELECT session_id, role FROM semio.share WHERE token = $1").bind(token).fetch_optional(&self.pool).await?;
        match share {
            Some((session_id, role)) => Ok(Caller::Share(ShareGrant { session_id, role: Role::parse(&role)? })),
            None => Err(HubError::Unauthorized("invalid or expired token".into())),
        }
    }

    /// 🚪 Revokes a person token.
    pub async fn logout(&self, token: &str) -> Result<(), HubError> {
        let removed = query("DELETE FROM semio.token WHERE hash = $1").bind(digest(token)).execute(&self.pool).await?.rows_affected();
        if removed == 0 { Err(HubError::Unauthorized("invalid or expired token".into())) } else { Ok(()) }
    }

    /// 🤖 Issues a long-lived agent token for MCP clients.
    pub async fn create_agent_token(&self, caller: &Caller, label: Option<&str>) -> Result<AgentToken, HubError> {
        let principal = caller.principal()?;
        let label = label.map(str::trim).filter(|label| !label.is_empty()).unwrap_or("agent").to_string();
        let token = mint_token();
        query(ISSUE_TOKEN).bind(digest(&token)).bind(principal.person.id).bind("agent").bind(&label).execute(&self.pool).await?;
        Ok(AgentToken { token, label, kind: ParticipantKind::Agent })
    }

    /// 🛂 Role of `caller` in a session, failing below `min`.
    pub async fn authorize(&self, caller: &Caller, session_id: Uuid, min: Role) -> Result<Role, HubError> {
        let role = match caller {
            Caller::Person(principal) => {
                let row: Option<(Option<String>,)> = query_as("SELECT m.role FROM semio.session s LEFT JOIN semio.member m ON m.session_id = s.id AND m.person_id = $2 WHERE s.id = $1")
                    .bind(session_id)
                    .bind(principal.person.id)
                    .fetch_optional(&self.pool)
                    .await?;
                match row {
                    None => return Err(HubError::NotFound("session not found".into())),
                    Some((None,)) => return Err(HubError::Forbidden("not a member of this session".into())),
                    Some((Some(role),)) => Role::parse(&role)?,
                }
            }
            Caller::Share(grant) if grant.session_id == session_id => Role::Viewer,
            Caller::Share(_) => return Err(HubError::Forbidden("share token is not valid for this session".into())),
        };
        if role < min {
            return Err(HubError::Forbidden(format!("requires the {} role", min.as_str())));
        }
        Ok(role)
    }

    //#endregion 🔑Auth

    //#region 🏘️Sessions

    fn summary(&self, (id, name, owner_id, owner_name, version, hash, updated_at): SessionRow, role: Role) -> SessionSummary {
        let participant_count = self.rooms.get(&id).map(|room| room.len()).unwrap_or(0);
        SessionSummary { id, name, role, owner: PersonRef { id: owner_id, name: owner_name }, version, hash, updated_at, participant_count }
    }

    async fn session_row(&self, session_id: Uuid) -> Result<SessionRow, HubError> {
        query_as(&format!("{SESSION_SELECT} WHERE s.id = $1")).bind(session_id).fetch_optional(&self.pool).await?.ok_or_else(|| HubError::NotFound("session not found".into()))
    }

    /// 📋 Sessions the caller is a member of (most recently updated first).
    pub async fn list_sessions(&self, caller: &Caller) -> Result<Vec<SessionSummary>, HubError> {
        let principal = caller.principal()?;
        let rows: Vec<(Uuid, String, Uuid, String, i64, String, OffsetDateTime, String)> = query_as(
            "SELECT s.id, s.name, s.owner_id, o.name, s.version, s.hash, s.updated_at, m.role FROM semio.member m JOIN semio.session s ON s.id = m.session_id JOIN semio.person o ON o.id = s.owner_id WHERE m.person_id = $1 ORDER BY s.updated_at DESC",
        )
        .bind(principal.person.id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(|(id, name, owner_id, owner_name, version, hash, updated_at, role)| Ok(self.summary((id, name, owner_id, owner_name, version, hash, updated_at), Role::parse(&role)?))).collect()
    }

    /// 🆕 Creates a session owned by the caller from a kit projection (or an empty kit named `name`); the stored version 0 is the store's own export of that kit.
    pub async fn create_session(&self, caller: &Caller, name: &str, kit: Option<Value>) -> Result<SessionSummary, HubError> {
        let principal = caller.principal()?;
        let name = name.trim();
        if name.is_empty() {
            return Err(HubError::BadRequest("name is required".into()));
        }
        let kit = match kit {
            None | Some(Value::Null) => empty_kit(name),
            Some(kit @ Value::Object(_)) => kit,
            Some(_) => return Err(HubError::BadRequest("kit must be a kit projection object".into())),
        };
        let session_id = Uuid::now_v7();
        let invalid = |e: HubError| HubError::BadRequest(format!("invalid kit: {e}"));
        let (_, projection) = KitStore::open(&session_id.to_string(), &kit).await.map_err(invalid)?.state().await?;
        let store = KitStore::open(&session_id.to_string(), &projection).await.map_err(invalid)?;
        let hash = store.hash().await?;
        let mut tx = self.pool.begin().await?;
        query("INSERT INTO semio.session (id, name, owner_id, hash) VALUES ($1, $2, $3, $4)").bind(session_id).bind(name).bind(principal.person.id).bind(&hash).execute(&mut *tx).await?;
        query("INSERT INTO semio.member (session_id, person_id, role) VALUES ($1, $2, 'owner')").bind(session_id).bind(principal.person.id).execute(&mut *tx).await?;
        query("INSERT INTO semio.snapshot (session_id, version, hash, kit) VALUES ($1, 0, $2, $3)").bind(session_id).bind(&hash).bind(&projection).execute(&mut *tx).await?;
        tx.commit().await?;
        self.spawn_actor(session_id, SessionActor::new(self.pool.clone(), self.room(session_id), session_id, store, 0, hash));
        Ok(self.summary(self.session_row(session_id).await?, Role::Owner))
    }

    /// 🔍 One session.
    pub async fn session(&self, caller: &Caller, session_id: Uuid) -> Result<SessionSummary, HubError> {
        let role = self.authorize(caller, session_id, Role::Viewer).await?;
        Ok(self.summary(self.session_row(session_id).await?, role))
    }

    /// 🗑️ Deletes a session with its history and disconnects its sockets.
    pub async fn delete_session(&self, caller: &Caller, session_id: Uuid) -> Result<(), HubError> {
        self.authorize(caller, session_id, Role::Owner).await?;
        query("DELETE FROM semio.session WHERE id = $1").bind(session_id).execute(&self.pool).await?;
        self.actors.remove(&session_id);
        if let Some((_, room)) = self.rooms.remove(&session_id) {
            room.close("session deleted");
        }
        Ok(())
    }

    /// 📦 Current kit projection with version and hash.
    pub async fn session_kit(&self, caller: &Caller, session_id: Uuid) -> Result<KitState, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        self.call(session_id, |reply| ActorCommand::Kit { reply }).await
    }

    /// ⏪ Kit projection as of `version` (snapshot + replay).
    pub async fn kit_at(&self, caller: &Caller, session_id: Uuid, version: i64) -> Result<KitState, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        let current: i64 = query_scalar("SELECT version FROM semio.session WHERE id = $1").bind(session_id).fetch_one(&self.pool).await?;
        if !(0..=current).contains(&version) {
            return Err(HubError::NotFound(format!("version {version} does not exist (current version {current})")));
        }
        let (store, _) = rebuild(&self.pool, session_id, version).await?;
        let (hash, kit) = store.state().await?;
        Ok(KitState { version, hash, kit })
    }

    /// 🧑‍🤝‍🧑 Members of a session.
    pub async fn members(&self, caller: &Caller, session_id: Uuid) -> Result<Vec<Member>, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        let rows: Vec<(Uuid, String, String, String, String)> = query_as("SELECT p.id, p.name, p.email, p.color, m.role FROM semio.member m JOIN semio.person p ON p.id = m.person_id WHERE m.session_id = $1 ORDER BY m.joined_at")
            .bind(session_id)
            .fetch_all(&self.pool)
            .await?;
        rows.into_iter().map(|(id, name, email, color, role)| Ok(Member { person: Person { id, name, email, color }, role: Role::parse(&role)? })).collect()
    }

    /// 📍 Current version and hash of a session.
    pub async fn head(&self, session_id: Uuid) -> Result<(i64, String), HubError> {
        query_as("SELECT version, hash FROM semio.session WHERE id = $1").bind(session_id).fetch_optional(&self.pool).await?.ok_or_else(|| HubError::NotFound("session not found".into()))
    }

    //#endregion 🏘️Sessions

    //#region 📜Operations

    /// ✍️ Applies a kit-scoped operation on the authoritative store, logs it and broadcasts it to every socket of the session (idempotent on `operationId`).
    pub async fn execute_operation(&self, caller: &Caller, session_id: Uuid, request: OperationRequest, participant_id: Option<Uuid>) -> Result<OperationResult, HubError> {
        self.authorize(caller, session_id, Role::Editor).await?;
        let principal = caller.principal()?;
        if request.operation_id.trim().is_empty() || request.client_id.trim().is_empty() {
            return Err(HubError::BadRequest("operationId and clientId are required".into()));
        }
        if !(request.variables.is_null() || request.variables.is_object()) {
            return Err(HubError::BadRequest("variables must be an object".into()));
        }
        match inspect_document(&request.query).map_err(HubError::BadRequest)? {
            DocumentKind::KitOperation => {}
            DocumentKind::Read => return Err(HubError::BadRequest("queries are read-only; use POST /sessions/{id}/graphql".into())),
        }
        let participant = self.rooms.get(&session_id).and_then(|room| {
            room.touch(|participant| match participant_id {
                Some(id) => participant.id == id,
                None => participant.person_id == principal.person.id && participant.client_id == request.client_id,
            })
        });
        let author = Author { person_id: principal.person.id, kind: participant.as_ref().map_or(principal.kind, |participant| participant.kind), participant_id: participant.map(|participant| participant.id) };
        self.call(session_id, |reply| ActorCommand::Operation { author, request, reply }).await
    }

    /// 🔎 Executes a read-only GraphQL document against the session store.
    pub async fn execute_query(&self, caller: &Caller, session_id: Uuid, query: &str, variables: Option<Value>) -> Result<Value, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        if inspect_document(query).map_err(|_| HubError::BadRequest("mutations are not allowed here; use POST /sessions/{id}/operations".into()))? != DocumentKind::Read {
            return Err(HubError::BadRequest("mutations are not allowed here; use POST /sessions/{id}/operations".into()));
        }
        let variables = variables.unwrap_or(Value::Null);
        if !(variables.is_null() || variables.is_object()) {
            return Err(HubError::BadRequest("variables must be an object".into()));
        }
        let query = query.to_string();
        self.call(session_id, |reply| ActorCommand::Query { query, variables, reply }).await
    }

    /// 📜 Logged operations after `after`.
    pub async fn history(&self, caller: &Caller, session_id: Uuid, after: i64) -> Result<Vec<OperationRecord>, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        let rows: Vec<(i64, String, String, Uuid, String, String, Value, String, OffsetDateTime)> =
            query_as("SELECT version, operation_id, client_id, person_id, participant_kind, query, variables, hash, created_at FROM semio.operation WHERE session_id = $1 AND version > $2 ORDER BY version")
                .bind(session_id)
                .bind(after)
                .fetch_all(&self.pool)
                .await?;
        Ok(rows
            .into_iter()
            .map(|(version, operation_id, client_id, person_id, kind, query, variables, hash, created_at)| OperationRecord { version, operation_id, client_id, person_id, participant_kind: ParticipantKind::parse(&kind), query, variables, hash, created_at })
            .collect())
    }

    //#endregion 📜Operations

    //#region 🔗Shares

    /// 🔗 Creates a share token (`editor` or `viewer`).
    pub async fn create_share(&self, caller: &Caller, session_id: Uuid, role: Role, label: Option<String>) -> Result<Share, HubError> {
        self.authorize(caller, session_id, Role::Owner).await?;
        let principal = caller.principal()?;
        if role == Role::Owner {
            return Err(HubError::BadRequest("shares grant editor or viewer".into()));
        }
        let token = mint_token();
        let created_at: OffsetDateTime = query_scalar("INSERT INTO semio.share (token, session_id, role, label, created_by) VALUES ($1, $2, $3, $4, $5) RETURNING created_at")
            .bind(&token)
            .bind(session_id)
            .bind(role.as_str())
            .bind(&label)
            .bind(principal.person.id)
            .fetch_one(&self.pool)
            .await?;
        Ok(Share { token, role, label, created_at })
    }

    /// 📋 Share tokens of a session.
    pub async fn shares(&self, caller: &Caller, session_id: Uuid) -> Result<Vec<Share>, HubError> {
        self.authorize(caller, session_id, Role::Owner).await?;
        let rows: Vec<(String, String, Option<String>, OffsetDateTime)> = query_as("SELECT token, role, label, created_at FROM semio.share WHERE session_id = $1 ORDER BY created_at").bind(session_id).fetch_all(&self.pool).await?;
        rows.into_iter().map(|(token, role, label, created_at)| Ok(Share { token, role: Role::parse(&role)?, label, created_at })).collect()
    }

    /// ✂️ Revokes a share token.
    pub async fn delete_share(&self, caller: &Caller, session_id: Uuid, token: &str) -> Result<(), HubError> {
        self.authorize(caller, session_id, Role::Owner).await?;
        let removed = query("DELETE FROM semio.share WHERE session_id = $1 AND token = $2").bind(session_id).bind(token).execute(&self.pool).await?.rows_affected();
        if removed == 0 { Err(HubError::NotFound("share not found".into())) } else { Ok(()) }
    }

    /// 🚪 Joins the session of a share token (never downgrades an existing role).
    pub async fn join_share(&self, caller: &Caller, token: &str) -> Result<Joined, HubError> {
        let principal = caller.principal()?;
        let (session_id, granted): (Uuid, String) = query_as("SELECT session_id, role FROM semio.share WHERE token = $1").bind(token).fetch_optional(&self.pool).await?.ok_or_else(|| HubError::NotFound("share not found".into()))?;
        let current: Option<String> = query_scalar("SELECT role FROM semio.member WHERE session_id = $1 AND person_id = $2").bind(session_id).bind(principal.person.id).fetch_optional(&self.pool).await?;
        let role = current.as_deref().map(Role::parse).transpose()?.into_iter().chain([Role::parse(&granted)?]).max().unwrap_or(Role::Viewer);
        query("INSERT INTO semio.member (session_id, person_id, role) VALUES ($1, $2, $3) ON CONFLICT (session_id, person_id) DO UPDATE SET role = EXCLUDED.role")
            .bind(session_id)
            .bind(principal.person.id)
            .bind(role.as_str())
            .execute(&self.pool)
            .await?;
        Ok(Joined { session: self.summary(self.session_row(session_id).await?, role), role })
    }

    //#endregion 🔗Shares

    //#region 👥Presence

    /// 🏠 Room of a session (created on demand).
    pub fn room(&self, session_id: Uuid) -> Arc<Room> {
        self.rooms.entry(session_id).or_default().clone()
    }

    /// 👥 Participants of a session.
    pub async fn participants(&self, caller: &Caller, session_id: Uuid) -> Result<Vec<Participant>, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        Ok(self.rooms.get(&session_id).map(|room| room.participants()).unwrap_or_default())
    }

    /// ➕ Adds a participant for `principal` and broadcasts `presence.joined`.
    pub fn join(&self, session_id: Uuid, principal: &Principal, kind: ParticipantKind, client: &str, client_id: &str, socketless: bool) -> Participant {
        let participant = Participant {
            id: Uuid::now_v7(),
            person_id: principal.person.id,
            name: principal.person.name.clone(),
            color: principal.person.color.clone(),
            kind,
            client: client.into(),
            focus: None,
            selection: None,
            cursor: None,
            joined_at: OffsetDateTime::now_utc(),
            client_id: client_id.into(),
            socketless,
            last_seen: Instant::now(),
        };
        self.room(session_id).add(participant.clone());
        participant
    }

    /// 🤖 Joins or refreshes the socketless `kind: "agent"` participant of the caller for `client` (expires after [`AGENT_IDLE`] without activity).
    pub async fn agent_presence(&self, caller: &Caller, session_id: Uuid, client: &str) -> Result<Participant, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        let principal = caller.principal()?;
        let client_id = format!("{client}:{}", principal.person.id);
        let existing = self.room(session_id).touch(|participant| participant.socketless && participant.client_id == client_id);
        Ok(existing.unwrap_or_else(|| self.join(session_id, principal, ParticipantKind::Agent, client, &client_id, true)))
    }

    /// 🖱️ Merges a presence patch (`focus`, `selection`, `cursor`) into a participant and broadcasts `presence.updated`.
    pub fn update_presence(&self, session_id: Uuid, participant_id: Uuid, patch: &Value) -> Result<Participant, HubError> {
        self.rooms.get(&session_id).and_then(|room| room.update(participant_id, patch)).ok_or_else(|| HubError::NotFound("participant not found".into()))
    }

    /// ➖ Removes a participant and broadcasts `presence.left`.
    pub fn leave_presence(&self, session_id: Uuid, participant_id: Uuid) {
        if let Some(room) = self.rooms.get(&session_id) {
            room.remove(participant_id);
        }
    }

    //#endregion 👥Presence

    //#region 🎹Actors

    fn spawn_actor(&self, session_id: Uuid, actor: SessionActor) -> mpsc::Sender<ActorCommand> {
        let (tx, rx) = mpsc::channel(256);
        let generation = self.generation.fetch_add(1, Ordering::SeqCst);
        self.actors.insert(session_id, (generation, tx.clone()));
        tokio::spawn(actor.run(rx, self.actors.clone(), generation));
        tx
    }

    async fn actor(&self, session_id: Uuid) -> Result<mpsc::Sender<ActorCommand>, HubError> {
        if let Some(entry) = self.actors.get(&session_id) {
            return Ok(entry.1.clone());
        }
        let _loading = self.loading.lock().await;
        if let Some(entry) = self.actors.get(&session_id) {
            return Ok(entry.1.clone());
        }
        let actor = SessionActor::load(self.pool.clone(), self.room(session_id), session_id).await?;
        Ok(self.spawn_actor(session_id, actor))
    }

    async fn call<T>(&self, session_id: Uuid, command: impl FnOnce(oneshot::Sender<Result<T, HubError>>) -> ActorCommand) -> Result<T, HubError> {
        let stopped = || HubError::Internal("session actor stopped".into());
        let (reply, response) = oneshot::channel();
        self.actor(session_id).await?.send(command(reply)).await.map_err(|_| stopped())?;
        response.await.map_err(|_| stopped())?
    }

    /// 💤 Drops the loaded actor of a session (next access reloads it from snapshot + replay).
    pub fn evict(&self, session_id: Uuid) {
        self.actors.remove(&session_id);
    }

    //#endregion 🎹Actors
}

} // 🏛️Hub
pub use hub::*;

mod api { // 🛕Api
// Specs: HTTP surface of Hub Protocol v1. Handlers are thin: extract the caller (Bearer), parse JSON (`{error}` on failure) and delegate to [`Hub`]. CORS is permissive (any origin, Authorization header) and requests are traced.
// Summary: axum router, extractors and handlers.

use super::*;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// 🔑 Bearer token of a request.
pub fn bearer(headers: &HeaderMap) -> Option<String> {
    headers.get("authorization").and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).map(|token| token.trim().to_string()).filter(|token| !token.is_empty())
}

impl FromRequestParts<Hub> for Caller {
    type Rejection = HubError;

    async fn from_request_parts(parts: &mut Parts, hub: &Hub) -> Result<Self, Self::Rejection> {
        let token = bearer(&parts.headers).ok_or_else(|| HubError::Unauthorized("missing bearer token".into()))?;
        hub.authenticate(&token).await
    }
}

/// 📨 JSON body extractor rejecting with `{error}`.
pub struct Body<T>(pub T);

impl<T: DeserializeOwned, S: Send + Sync> FromRequest<S> for Body<T> {
    type Rejection = HubError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(request, state).await.map(|Json(value)| Body(value)).map_err(|e| HubError::BadRequest(e.body_text()))
    }
}

/// 🧭 Path extractor rejecting with `{error}`.
pub struct Param<T>(pub T);

impl<T: DeserializeOwned + Send, S: Send + Sync> FromRequestParts<S> for Param<T> {
    type Rejection = HubError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<T>::from_request_parts(parts, state).await.map(|axum::extract::Path(value)| Param(value)).map_err(|e| HubError::BadRequest(e.body_text()))
    }
}

type Reply<T> = Result<Json<T>, HubError>;
type Created<T> = Result<(StatusCode, Json<T>), HubError>;

#[derive(Deserialize)]
struct RegisterBody {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginBody {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct TokenBody {
    label: Option<String>,
}

#[derive(Deserialize)]
struct SessionBody {
    name: String,
    kit: Option<Value>,
}

#[derive(Deserialize)]
struct GraphqlBody {
    query: String,
    variables: Option<Value>,
}

#[derive(Deserialize)]
struct ShareBody {
    role: Role,
    label: Option<String>,
}

#[derive(Deserialize)]
struct HistoryQuery {
    #[serde(default)]
    after: i64,
}

/// 🗺️ Hub Protocol v1 routes.
pub fn router(hub: Hub) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
        .route("/auth/logout", post(logout))
        .route("/auth/tokens", post(create_token))
        .route("/sessions", get(list_sessions).post(create_session))
        .route("/sessions/{session_id}", get(get_session).delete(delete_session))
        .route("/sessions/{session_id}/kit", get(get_kit))
        .route("/sessions/{session_id}/kit/at/{version}", get(get_kit_at))
        .route("/sessions/{session_id}/operations", get(list_operations).post(post_operation))
        .route("/sessions/{session_id}/graphql", post(post_graphql))
        .route("/sessions/{session_id}/members", get(list_members))
        .route("/sessions/{session_id}/shares", get(list_shares).post(create_share))
        .route("/sessions/{session_id}/shares/{token}", axum::routing::delete(delete_share))
        .route("/shares/{token}/join", post(join_share))
        .route("/sessions/{session_id}/presence", get(list_presence))
        .route("/sessions/{session_id}/ws", get(crate::ws::connect))
        .layer(DefaultBodyLimit::max(64 * 1024 * 1024))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(hub)
}

async fn register(State(hub): State<Hub>, Body(body): Body<RegisterBody>) -> Created<AuthResponse> {
    Ok((StatusCode::CREATED, Json(hub.register(&body.name, &body.email, &body.password).await?)))
}

async fn login(State(hub): State<Hub>, Body(body): Body<LoginBody>) -> Reply<AuthResponse> {
    Ok(Json(hub.login(&body.email, &body.password).await?))
}

async fn me(caller: Caller) -> Reply<Value> {
    Ok(Json(json!({ "person": caller.principal()?.person })))
}

async fn logout(State(hub): State<Hub>, headers: HeaderMap) -> Result<StatusCode, HubError> {
    hub.logout(&bearer(&headers).ok_or_else(|| HubError::Unauthorized("missing bearer token".into()))?).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_token(State(hub): State<Hub>, caller: Caller, Body(body): Body<TokenBody>) -> Created<AgentToken> {
    Ok((StatusCode::CREATED, Json(hub.create_agent_token(&caller, body.label.as_deref()).await?)))
}

async fn list_sessions(State(hub): State<Hub>, caller: Caller) -> Reply<Vec<SessionSummary>> {
    Ok(Json(hub.list_sessions(&caller).await?))
}

async fn create_session(State(hub): State<Hub>, caller: Caller, Body(body): Body<SessionBody>) -> Created<SessionSummary> {
    Ok((StatusCode::CREATED, Json(hub.create_session(&caller, &body.name, body.kit).await?)))
}

async fn get_session(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>) -> Reply<SessionSummary> {
    Ok(Json(hub.session(&caller, session_id).await?))
}

async fn delete_session(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>) -> Result<StatusCode, HubError> {
    hub.delete_session(&caller, session_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_kit(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>) -> Reply<KitState> {
    Ok(Json(hub.session_kit(&caller, session_id).await?))
}

async fn get_kit_at(State(hub): State<Hub>, caller: Caller, Param((session_id, version)): Param<(Uuid, i64)>) -> Reply<KitState> {
    Ok(Json(hub.kit_at(&caller, session_id, version).await?))
}

async fn post_operation(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>, Body(request): Body<OperationRequest>) -> Reply<OperationResult> {
    Ok(Json(hub.execute_operation(&caller, session_id, request, None).await?))
}

async fn list_operations(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>, Query(history): Query<HistoryQuery>) -> Reply<Vec<OperationRecord>> {
    Ok(Json(hub.history(&caller, session_id, history.after).await?))
}

async fn post_graphql(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>, Body(body): Body<GraphqlBody>) -> Reply<Value> {
    Ok(Json(hub.execute_query(&caller, session_id, &body.query, body.variables).await?))
}

async fn list_members(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>) -> Reply<Vec<Member>> {
    Ok(Json(hub.members(&caller, session_id).await?))
}

async fn create_share(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>, Body(body): Body<ShareBody>) -> Created<Share> {
    Ok((StatusCode::CREATED, Json(hub.create_share(&caller, session_id, body.role, body.label).await?)))
}

async fn list_shares(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>) -> Reply<Vec<Share>> {
    Ok(Json(hub.shares(&caller, session_id).await?))
}

async fn delete_share(State(hub): State<Hub>, caller: Caller, Param((session_id, token)): Param<(Uuid, String)>) -> Result<StatusCode, HubError> {
    hub.delete_share(&caller, session_id, &token).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn join_share(State(hub): State<Hub>, caller: Caller, Param(token): Param<String>) -> Reply<Joined> {
    Ok(Json(hub.join_share(&caller, &token).await?))
}

async fn list_presence(State(hub): State<Hub>, caller: Caller, Param(session_id): Param<Uuid>) -> Reply<Vec<Participant>> {
    Ok(Json(hub.participants(&caller, session_id).await?))
}

} // 🛕Api
pub use api::*;

mod ws { // 🤖Ws
// Specs: `GET /sessions/{id}/ws?token=&clientId=&client=` authenticates before upgrading (401/403/404 as JSON), joins the caller as a participant, sends `welcome`, then forwards the room broadcast (operations, presence) and handles `presence` / `operation` / `ping` messages. Closing the socket removes the participant (`presence.left`). Clients ignore `operation` messages with `version` ≤ `welcome.version`.
// Summary: Session websocket.

use super::*;
use axum::extract::ws::rejection::WebSocketUpgradeRejection;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsQuery {
    token: Option<String>,
    client_id: Option<String>,
    client: Option<String>,
}

/// 🔌 Authenticates and upgrades a session websocket.
pub async fn connect(State(hub): State<Hub>, Param(session_id): Param<Uuid>, Query(params): Query<WsQuery>, upgrade: Result<WebSocketUpgrade, WebSocketUpgradeRejection>) -> Result<Response, HubError> {
    let token = params.token.ok_or_else(|| HubError::Unauthorized("missing token".into()))?;
    let caller = hub.authenticate(&token).await?;
    hub.authorize(&caller, session_id, Role::Viewer).await?;
    let principal = caller.principal()?.clone();
    let upgrade = upgrade.map_err(|e| HubError::BadRequest(e.body_text()))?;
    let client = params.client.unwrap_or_else(|| "unknown".into());
    let client_id = params.client_id.unwrap_or_else(|| Uuid::now_v7().to_string());
    Ok(upgrade.on_upgrade(move |socket| serve(hub, session_id, principal, client, client_id, socket)))
}

async fn serve(hub: Hub, session_id: Uuid, principal: Principal, client: String, client_id: String, socket: WebSocket) {
    let room = hub.room(session_id);
    let mut events = room.subscribe();
    let me = hub.join(session_id, &principal, principal.kind, &client, &client_id, false);
    let (mut outbound, mut inbound) = socket.split();
    let welcome = match hub.head(session_id).await {
        Ok((version, hash)) => ServerMessage::Welcome { me: me.clone(), participants: room.participants(), version, hash },
        Err(error) => ServerMessage::Error { message: error.to_string(), operation_id: None },
    };
    let mut open = send(&mut outbound, &welcome).await;
    while open {
        tokio::select! {
            event = events.recv() => open = match event {
                Ok(text) => outbound.send(Message::Text(text.as_ref().into())).await.is_ok() && !room.is_closed(),
                Err(broadcast::error::RecvError::Lagged(missed)) => send(&mut outbound, &ServerMessage::Error { message: format!("missed {missed} events; resync from GET /sessions/{session_id}/kit"), operation_id: None }).await,
                Err(broadcast::error::RecvError::Closed) => false,
            },
            incoming = inbound.next() => open = match incoming {
                Some(Ok(Message::Text(text))) => match handle(&hub, session_id, &principal, &me, text.as_str()).await {
                    Some(reply) => send(&mut outbound, &reply).await,
                    None => true,
                },
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => false,
                Some(Ok(_)) => true,
            },
        }
    }
    hub.leave_presence(session_id, me.id);
}

async fn send(outbound: &mut futures::stream::SplitSink<WebSocket, Message>, message: &ServerMessage) -> bool {
    match serde_json::to_string(message) {
        Ok(text) => outbound.send(Message::Text(text.into())).await.is_ok(),
        Err(_) => true,
    }
}

async fn handle(hub: &Hub, session_id: Uuid, principal: &Principal, me: &Participant, text: &str) -> Option<ServerMessage> {
    let failure = |error: HubError, operation_id: Option<String>| Some(ServerMessage::Error { message: error.to_string(), operation_id });
    let message: Value = match serde_json::from_str(text) {
        Ok(message) => message,
        Err(error) => return failure(HubError::BadRequest(error.to_string()), None),
    };
    match message.get("type").and_then(Value::as_str) {
        Some("ping") => Some(ServerMessage::Pong),
        Some("presence") => hub.update_presence(session_id, me.id, &message).err().and_then(|error| failure(error, None)),
        Some("operation") => {
            let mut request: OperationRequest = match serde_json::from_value(message) {
                Ok(request) => request,
                Err(error) => return failure(HubError::BadRequest(error.to_string()), None),
            };
            request.client_id = me.client_id.clone();
            let operation_id = request.operation_id.clone();
            hub.execute_operation(&Caller::Person(principal.clone()), session_id, request, Some(me.id)).await.err().and_then(|error| failure(error, Some(operation_id)))
        }
        _ => failure(HubError::BadRequest("unknown message type".into()), None),
    }
}

} // 🤖Ws
pub use ws::*;

mod main { // 🚀Main
// Specs: Env `DATABASE_URL` (default `postgres://semio:semio@localhost:5432/semio`), `LISTEN_ADDR` (default `127.0.0.1:8080`, `0.0.0.0:8080` when `DEVCONTAINER=true`), `RUST_LOG`.
// Summary: Process entry point.

use super::*;

/// 🚀 Boots tracing, database schema and the HTTP server.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "semio_hub=info,tower_http=info".into())).init();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://semio:semio@localhost:5432/semio".into());
    let pool = connect(&database_url).await?;
    migrate(&pool).await?;
    let host = if std::env::var("DEVCONTAINER").as_deref() == Ok("true") { "0.0.0.0" } else { "127.0.0.1" };
    let addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| format!("{host}:8080"));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("semio-hub listening on http://{}", listener.local_addr()?);
    axum::serve(listener, router(Hub::new(pool, Arc::new(Argon2id::strong()))))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

} // 🚀Main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    main::run().await
}
