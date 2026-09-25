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

/// 🧬 Shape of an accepted GraphQL document.
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

/// 🎭 Participant nature: people or AI agents.
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

/// 🪢 Read-only session access granted by a share token.
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

/// 🗝️ `POST /auth/register` + `POST /auth/login` result.
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub person: Person,
}

/// 🦾 `POST /auth/tokens` result.
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

/// 📤 Operation submitted by a client (`POST …/operations` or websocket `operation`).
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

/// 🎉 `POST /shares/{token}/join` result.
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

/// 🐘 Postgres connection pool.
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

    /// #️⃣ Content hash of the materialized `theKit.kit`.
    pub async fn hash(&self) -> Result<String, HubError> {
        let data = self.data(HASH, json!({})).await?;
        data.pointer(&format!("{KIT}/hash")).and_then(Value::as_str).map(str::to_string).ok_or_else(|| HubError::Internal("store returned no kit hash".into()))
    }

    /// 🗃️ Content hash plus canonical projection of the materialized `theKit.kit`.
    pub async fn state(&self) -> Result<(String, Value), HubError> {
        let data = self.data(STATE, json!({})).await?;
        let kit = data.pointer(KIT).ok_or_else(|| HubError::Internal("store returned no kit".into()))?;
        let hash = kit.get("hash").and_then(Value::as_str).ok_or_else(|| HubError::Internal("store returned no kit hash".into()))?;
        let projection = kit.get("projection").and_then(Value::as_str).ok_or_else(|| HubError::Internal("store returned no kit projection".into()))?;
        Ok((hash.to_string(), serde_json::from_str(projection).map_err(|e| HubError::Internal(e.to_string()))?))
    }
}

/// 🫙 Projection of an empty kit.
pub fn empty_kit(name: &str) -> Value {
    json!({ "id": Uuid::now_v7(), "name": name })
}

/// 🩺 GraphQL errors plus the messages of every rs `Response` with `ok: false` inside `data`.
pub fn failures(response: &Value) -> Vec<String> {
    let mut out = messages(response.get("errors"));
    collect_not_ok(response.get("data").unwrap_or(&Value::Null), &mut out);
    out
}

fn messages(errors: Option<&Value>) -> Vec<String> {
    let listed: Vec<&Value> = match errors {
        Some(Value::Array(errors)) => errors.iter().collect(),
        Some(error @ Value::Object(_)) => vec![error],
        _ => Vec::new(),
    };
    listed.into_iter().filter_map(|error| error.get("message").and_then(Value::as_str).map(str::to_string)).collect()
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
        tracing::info!("session {} v{version} {hash} ← operation {} ({} {})", self.session_id, request.operation_id, author.kind.as_str(), author.person_id);
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
type MembershipRow = (Uuid, String, Uuid, String, i64, String, OffsetDateTime, String);
type OperationRow = (i64, String, String, Uuid, String, String, Value, String, OffsetDateTime);

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
    /// 🌱 Creates the hub and starts the agent presence sweeper.
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

    /// 📝 Registers a person with a password and returns a login token.
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

    /// 🛃 Resolves a bearer token (person login/agent token or share token).
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

    /// 🔒 Revokes a person token.
    pub async fn logout(&self, token: &str) -> Result<(), HubError> {
        let removed = query("DELETE FROM semio.token WHERE hash = $1").bind(digest(token)).execute(&self.pool).await?.rows_affected();
        if removed == 0 { Err(HubError::Unauthorized("invalid or expired token".into())) } else { Ok(()) }
    }

    /// 🛠️ Issues a long-lived agent token for MCP clients.
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
        let rows: Vec<MembershipRow> = query_as(
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
        tracing::info!("session {session_id} created by {} with hash {hash}", principal.person.id);
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

    /// 🧺 Current kit projection with version and hash.
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

    /// 👪 Members of a session.
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

    /// 🖋️ Applies a kit-scoped operation on the authoritative store, logs it and broadcasts it to every socket of the session (idempotent on `operationId`).
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

    /// 🔭 Executes a read-only GraphQL document against the session store.
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

    /// 🗂️ Logged operations after `after`.
    pub async fn history(&self, caller: &Caller, session_id: Uuid, after: i64) -> Result<Vec<OperationRecord>, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        let rows: Vec<OperationRow> = query_as("SELECT version, operation_id, client_id, person_id, participant_kind, query, variables, hash, created_at FROM semio.operation WHERE session_id = $1 AND version > $2 ORDER BY version")
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

    /// 🖇️ Creates a share token (`editor` or `viewer`).
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

    /// 📑 Share tokens of a session.
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

    /// 🚶 Joins the session of a share token (never downgrades an existing role).
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

    /// 🏡 Room of a session (created on demand).
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

    /// 🛰️ Joins or refreshes the socketless `kind: "agent"` participant of the caller for `client` (expires after [`AGENT_IDLE`] without activity).
    pub async fn agent_presence(&self, caller: &Caller, session_id: Uuid, client: &str) -> Result<Participant, HubError> {
        self.authorize(caller, session_id, Role::Viewer).await?;
        let principal = caller.principal()?;
        let client_id = format!("{client}:{}", principal.person.id);
        let existing = self.room(session_id).touch(|participant| participant.socketless && participant.client_id == client_id);
        Ok(existing.unwrap_or_else(|| self.join(session_id, principal, ParticipantKind::Agent, client, &client_id, true)))
    }

    /// 🎯 Merges a presence patch (`focus`, `selection`, `cursor`) into a participant and broadcasts `presence.updated`.
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
// Specs: HTTP surface of Hub Protocol v1. Handlers are thin: extract the caller (Bearer), parse JSON (`{error}` on failure) and delegate to [`Hub`]. CORS is permissive for development (any origin and method, requested headers mirrored because `Authorization` cannot be wildcarded: https://developer.mozilla.org/docs/Web/HTTP/Headers/Access-Control-Allow-Headers) and every request is traced at info level.
// Summary: axum router, extractors and handlers.

use super::*;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

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
        .route("/sessions/{session_id}/ws", get(crate::ws::session_socket))
        .route("/mcp", post(crate::mcp::handler))
        .fallback(|| async { HubError::NotFound("route not found".into()) })
        .layer(DefaultBodyLimit::max(64 * 1024 * 1024))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(AllowHeaders::mirror_request()).expose_headers(Any).max_age(Duration::from_secs(3600)))
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO)).on_response(DefaultOnResponse::new().level(Level::INFO)))
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
pub async fn session_socket(State(hub): State<Hub>, Param(session_id): Param<Uuid>, Query(params): Query<WsQuery>, upgrade: Result<WebSocketUpgrade, WebSocketUpgradeRejection>) -> Result<Response, HubError> {
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
    let _ = outbound.send(Message::Close(None)).await;
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

mod mcp { // 🧠Mcp
// Specs: `POST /mcp` is the collaborative semio MCP server (Model Context Protocol, Streamable HTTP transport answering with plain JSON: https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#streamable-http). Messages are JSON-RPC 2.0 (single or batch): `initialize` negotiates the protocol version and issues `Mcp-Session-Id`, notifications and client responses are acknowledged with 202, `GET /mcp` is 405 (no server stream). Every request needs `Authorization: Bearer <person or agent token>` (401 + `WWW-Authenticate: Bearer` otherwise). Tools read the session kit projection or execute exactly one kit-scoped hub operation each (persisted, versioned, broadcast to every collaborator) with uuid v7 ids minted here for created entities, so replicas replay deterministically. While working on a session the caller is a socketless `kind: "agent"`, `client: "mcp"` participant named `"<token label> (<person name>)"` whose focus/selection follow its edits.
// Summary: MCP JSON-RPC endpoint with semio tools, resources and prompts.

use super::*;
use semio::external_adapters::async_graphql::parser::parse_query;
use semio::external_adapters::async_graphql::parser::types::{Selection, SelectionSet};
use sqlx_core::query_scalar::query_scalar;

//#region 🔌Protocol

pub const PROTOCOL_VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];
const CLIENT: &str = "mcp";
const RESPONSE: &str = "{ ok errors { message } }";
const DEFAULT_LIMIT: usize = 200;

/// 🧯 JSON-RPC error.
struct RpcError(i64, String);

impl RpcError {
    fn invalid(message: impl Into<String>) -> Self {
        Self(-32602, message.into())
    }
}

/// 🤖 Authenticated MCP caller: the person, acting as `"<label> (<name>)"`.
#[derive(Clone)]
pub struct Agent {
    caller: Caller,
}

/// 🚪 `POST /mcp`.
pub async fn handler(State(hub): State<Hub>, headers: HeaderMap, body: String) -> Response {
    let unauthorized = |message: String| (StatusCode::UNAUTHORIZED, [("www-authenticate", "Bearer realm=\"semio-hub\", error=\"invalid_token\"")], Json(json!({ "error": message }))).into_response();
    let Some(token) = bearer(&headers) else { return unauthorized("missing bearer token".into()) };
    let principal = match hub.authenticate(&token).await {
        Ok(Caller::Person(principal)) => principal,
        Ok(Caller::Share(_)) => return unauthorized("the semio MCP needs a person or agent token (POST /auth/tokens)".into()),
        Err(error) => return unauthorized(error.to_string()),
    };
    if let Some(version) = headers.get("mcp-protocol-version").and_then(|value| value.to_str().ok()).filter(|version| !PROTOCOL_VERSIONS.contains(version)) {
        return HubError::BadRequest(format!("unsupported MCP-Protocol-Version `{version}`; supported: {}", PROTOCOL_VERSIONS.join(", "))).into_response();
    }
    let message: Value = match serde_json::from_str(&body) {
        Ok(message) => message,
        Err(error) => return (StatusCode::BAD_REQUEST, Json(failure(Value::Null, RpcError(-32700, format!("parse error: {error}"))))).into_response(),
    };
    let label: Option<String> = query_scalar("SELECT label FROM semio.token WHERE hash = $1").bind(digest(&token)).fetch_optional(&hub.pool).await.ok().flatten().flatten();
    let mut person = principal.person.clone();
    person.name = format!("{} ({})", label.unwrap_or_else(|| CLIENT.into()), person.name);
    let agent = Agent { caller: Caller::Person(Principal { person, kind: ParticipantKind::Agent }) };
    let is_initialize = |message: &Value| message.get("method").and_then(Value::as_str) == Some("initialize");
    let initializing = message.as_array().map_or_else(|| is_initialize(&message), |batch| batch.iter().any(is_initialize));
    let mut replies = Vec::new();
    match &message {
        Value::Array(batch) if batch.is_empty() => replies.push(failure(Value::Null, RpcError(-32600, "empty batch".into()))),
        Value::Array(batch) => {
            for item in batch {
                replies.extend(reply(&hub, &agent, item).await);
            }
        }
        single => replies.extend(reply(&hub, &agent, single).await),
    }
    let body = match (&message, replies.len()) {
        (_, 0) => return StatusCode::ACCEPTED.into_response(),
        (Value::Array(_), _) => Value::Array(replies),
        (_, _) => replies.into_iter().next().unwrap_or(Value::Null),
    };
    let mut response = Json(body).into_response();
    if initializing {
        if let Ok(id) = Uuid::now_v7().to_string().parse() {
            response.headers_mut().insert("mcp-session-id", id);
        }
    }
    response
}

fn success(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn failure(id: Value, RpcError(code, message): RpcError) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

async fn reply(hub: &Hub, agent: &Agent, message: &Value) -> Option<Value> {
    let Some(method) = message.get("method").and_then(Value::as_str) else {
        return message.get("id").filter(|_| message.get("result").is_none() && message.get("error").is_none()).map(|id| failure(id.clone(), RpcError(-32600, "invalid request".into())));
    };
    let id = message.get("id")?.clone();
    let params = message.get("params").cloned().unwrap_or_else(|| json!({}));
    Some(match dispatch(hub, agent, method, &params).await {
        Ok(result) => success(id, result),
        Err(error) => failure(id, error),
    })
}

async fn dispatch(hub: &Hub, agent: &Agent, method: &str, params: &Value) -> Result<Value, RpcError> {
    match method {
        "initialize" => Ok(initialize(params)),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools() })),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).ok_or_else(|| RpcError::invalid("tools/call needs `name`"))?;
            if !tools().iter().any(|tool| tool["name"] == name) {
                return Err(RpcError::invalid(format!("unknown tool `{name}`")));
            }
            let arguments = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
            Ok(match call(hub, agent, name, &arguments).await {
                Ok(value) => json!({ "content": [{ "type": "text", "text": serde_json::to_string_pretty(&value).unwrap_or_default() }], "structuredContent": if value.is_object() { value } else { json!({ "result": value }) }, "isError": false }),
                Err(error) => json!({ "content": [{ "type": "text", "text": error.to_string() }], "isError": true }),
            })
        }
        "resources/list" => Ok(json!({ "resources": resources(hub, agent).await })),
        "resources/templates/list" => Ok(json!({ "resourceTemplates": [{ "uriTemplate": "semio://sessions/{sessionId}/kit", "name": "session-kit", "title": "Session kit", "description": "Full kit projection (JSON) of a hub session: typologies with their types (connectors, ports, representations) and designs (pieces, connections).", "mimeType": "application/json" }] })),
        "resources/read" => read_resource(hub, agent, params.get("uri").and_then(Value::as_str).ok_or_else(|| RpcError::invalid("resources/read needs `uri`"))?).await,
        "prompts/list" => Ok(json!({ "prompts": [prompt_definition()] })),
        "prompts/get" => prompt(params),
        other => Err(RpcError(-32601, format!("method `{other}` not found"))),
    }
}

/// 🤝 Negotiates the protocol version (the client's when supported, the latest otherwise).
fn initialize(params: &Value) -> Value {
    let requested = params.get("protocolVersion").and_then(Value::as_str).unwrap_or_default();
    let version = PROTOCOL_VERSIONS.iter().find(|version| **version == requested).unwrap_or(&PROTOCOL_VERSIONS[0]);
    json!({
        "protocolVersion": version,
        "capabilities": { "tools": { "listChanged": false }, "resources": { "listChanged": false, "subscribe": false }, "prompts": { "listChanged": false } },
        "serverInfo": { "name": "semio-hub", "title": "semio", "version": env!("CARGO_PKG_VERSION") },
        "instructions": "semio hub: collaborative kit-of-parts design. Read the `semio://guide` resource first. Every write tool is one operation on a shared session and is visible live to all collaborators. Workflow: list_sessions → read_kit → list_types (connector ids/names) → read_design → create_design / add_piece / connect_pieces / update_piece / remove_pieces / remove_connections.",
    })
}

//#endregion 🔌Protocol

//#region 🧰Tools

fn schema(properties: Value, required: &[&str]) -> Value {
    json!({ "type": "object", "properties": properties, "required": required, "additionalProperties": false })
}

fn session_property() -> Value {
    json!({ "type": "string", "format": "uuid", "description": "Hub session id (from list_sessions or create_session)." })
}

fn position_property(description: &str) -> Value {
    let vector = |what: &str| json!({ "type": "object", "description": what, "properties": { "x": { "type": "number" }, "y": { "type": "number" }, "z": { "type": "number" } }, "required": ["x", "y", "z"], "additionalProperties": false });
    json!({
        "type": "object",
        "description": description,
        "properties": {
            "plane": { "type": "object", "description": "3D placement frame. xAxis and yAxis must be non-zero and not parallel (usually orthonormal).", "properties": { "origin": vector("Origin point."), "xAxis": vector("Local x direction."), "yAxis": vector("Local y direction.") }, "required": ["origin", "xAxis", "yAxis"], "additionalProperties": false },
            "center": { "type": "object", "description": "2D position of the piece in the design diagram.", "properties": { "u": { "type": "number" }, "v": { "type": "number" } }, "required": ["u", "v"], "additionalProperties": false }
        },
        "required": ["plane", "center"],
        "additionalProperties": false
    })
}

fn joint_properties() -> Value {
    json!({
        "gap": { "type": "number", "description": "Distance between the two connectors along the connector direction (default 0)." },
        "shift": { "type": "number", "description": "Lateral offset (default 0)." },
        "rise": { "type": "number", "description": "Vertical offset (default 0)." },
        "rotation": { "type": "number", "description": "Rotation around the connector direction in degrees (default 0)." },
        "turn": { "type": "number", "description": "Turn in degrees (default 0)." },
        "tilt": { "type": "number", "description": "Tilt in degrees (default 0)." },
        "u": { "type": "number", "description": "Diagram offset of the child relative to the parent (default 0)." },
        "v": { "type": "number", "description": "Diagram offset of the child relative to the parent (default 0)." }
    })
}

fn tool(name: &str, title: &str, description: &str, input: Value, read_only: bool, destructive: bool) -> Value {
    json!({ "name": name, "title": title, "description": description, "inputSchema": input, "annotations": { "title": title, "readOnlyHint": read_only, "destructiveHint": destructive, "idempotentHint": read_only, "openWorldHint": false } })
}

/// 🧰 Tool catalogue (names, LLM-facing descriptions, JSON Schemas).
pub fn tools() -> Vec<Value> {
    let session = session_property();
    let design = json!({ "type": "string", "description": "Design id (from read_kit or list_designs)." });
    let mut connect = joint_properties();
    for (key, value) in [
        ("sessionId", session.clone()),
        ("designId", design.clone()),
        ("parentPieceId", json!({ "type": "string", "description": "Piece that already exists in the design and acts as parent." })),
        ("parentConnector", json!({ "type": "string", "description": "Connector id or name on the parent piece's type (see list_types)." })),
        ("childPieceId", json!({ "type": "string", "description": "Second existing piece of the design." })),
        ("childConnector", json!({ "type": "string", "description": "Connector id or name on the child piece's type." })),
    ] {
        connect[key] = value;
    }
    let mut parent = joint_properties();
    parent["pieceId"] = json!({ "type": "string", "description": "Existing parent piece id." });
    parent["connector"] = json!({ "type": "string", "description": "Connector id or name on the parent piece's type." });
    parent["childConnector"] = json!({ "type": "string", "description": "Connector id or name on the new piece's type." });
    vec![
        tool("list_sessions", "List sessions", "List the hub sessions (shared kits) you are a member of, with your role, version (number of operations) and content hash. Start here to pick a sessionId.", schema(json!({}), &[]), true, false),
        tool("create_session", "Create session", "Create a new session that you own. Starts from an empty kit named `name`, or from a copy of the current kit of `fromSessionId` (fork). Returns the session summary with its id.", schema(json!({ "name": { "type": "string", "minLength": 1, "description": "Session and kit name." }, "fromSessionId": { "type": "string", "format": "uuid", "description": "Optional session whose kit is copied." } }), &["name"]), false, false),
        tool("read_kit", "Read kit", "Compact overview of a session kit: typologies, families, types (with connector counts) and designs (with piece and connection counts). Use it to discover ids before reading details.", schema(json!({ "sessionId": session }), &["sessionId"]), true, false),
        tool("list_designs", "List designs", "List the designs of a session kit with typology, description and piece/connection counts.", schema(json!({ "sessionId": session }), &["sessionId"]), true, false),
        tool("read_design", "Read design", "Read one design: its pieces (id, name, type or nested design, pose = plane + diagram center for fixed pieces, null for pieces placed through their parent connection) and its connections (parent/child piece + connector ids and joint parameters). Large designs are paged with offset/limit.", schema(json!({ "sessionId": session, "designId": design, "offset": { "type": "integer", "minimum": 0, "description": "Index of the first piece and connection (default 0)." }, "limit": { "type": "integer", "minimum": 1, "maximum": 2000, "description": "Maximum pieces and connections returned (default 200)." } }), &["sessionId", "designId"]), true, false),
        tool("list_types", "List types", "List the types (reusable parts) of a session kit with their connectors (id, name, port) and representation counts. Optionally filter by a case-insensitive name `query` or a `typologyId`.", schema(json!({ "sessionId": session, "query": { "type": "string", "description": "Case-insensitive substring of the type name." }, "typologyId": { "type": "string", "description": "Only types of this typology." } }), &["sessionId"]), true, false),
        tool("create_design", "Create design", "Create an empty design in the session kit (one operation). Returns the new designId.", schema(json!({ "sessionId": session, "name": { "type": "string", "minLength": 1 }, "description": { "type": "string" }, "icon": { "type": "string", "description": "Emoji or icon url." }, "unit": { "type": "string", "description": "Length unit, e.g. \"m\" or \"mm\"." } }), &["sessionId", "name"]), false, false),
        tool("add_piece", "Add piece", "Add a piece (an instance of a type, or of another design) to a design (one operation). Without `parent` the piece is fixed at `position` (default: world origin). With `parent` the piece is attached to an existing piece through a new connection between `parent.connector` (on the parent) and `parent.childConnector` (on the new piece); it follows its parent unless `position` is also given. Returns pieceId (and connectionId).", schema(json!({ "sessionId": session, "designId": design, "typeId": { "type": "string", "description": "Blueprint: a type id (list_types) or a design id for nesting." }, "name": { "type": "string" }, "description": { "type": "string" }, "position": position_property("Absolute placement of a fixed piece."), "parent": { "type": "object", "description": "Attach the new piece to an existing piece.", "properties": parent, "required": ["pieceId", "connector", "childConnector"], "additionalProperties": false } }), &["sessionId", "designId", "typeId"]), false, false),
        tool("connect_pieces", "Connect pieces", "Connect two existing pieces of a design through a connector of each (one operation), e.g. to close a loop. Returns the connectionId.", schema(connect, &["sessionId", "designId", "parentPieceId", "parentConnector", "childPieceId", "childConnector"]), false, false),
        tool("update_piece", "Update piece", "Rename a piece, change its description and/or place it at an absolute position (a placed piece becomes fixed). All given changes form one operation.", schema(json!({ "sessionId": session, "designId": design, "pieceId": { "type": "string" }, "name": { "type": "string" }, "description": { "type": "string" }, "position": position_property("New absolute placement.") }), &["sessionId", "designId", "pieceId"]), false, false),
        tool("remove_pieces", "Remove pieces", "Remove pieces from a design together with every connection touching them (one operation).", schema(json!({ "sessionId": session, "designId": design, "pieceIds": { "type": "array", "items": { "type": "string" }, "minItems": 1 } }), &["sessionId", "designId", "pieceIds"]), false, true),
        tool("remove_connections", "Remove connections", "Remove connections from a design; the pieces stay (one operation).", schema(json!({ "sessionId": session, "designId": design, "connectionIds": { "type": "array", "items": { "type": "string" }, "minItems": 1 } }), &["sessionId", "designId", "connectionIds"]), false, true),
        tool("list_participants", "List participants", "List who is currently in the session (people and agents) with their focus and selection.", schema(json!({ "sessionId": session }), &["sessionId"]), true, false),
        tool("get_history", "Get history", "List the latest operations of a session (who changed what, as GraphQL documents) after an optional version.", schema(json!({ "sessionId": session, "after": { "type": "integer", "minimum": 0, "description": "Only operations with a higher version." }, "limit": { "type": "integer", "minimum": 1, "maximum": 500, "description": "Maximum number of latest operations (default 20)." } }), &["sessionId"]), true, false),
        tool("run_query", "Run GraphQL query", "Run a read-only GraphQL query against the session's kit store (semio GraphQL schema). Example: `{ session { stores { edges { node { wip { theKit { kit { name hasDesigns { edges { node { id name } } } } } } } } } } }`.", schema(json!({ "sessionId": session, "query": { "type": "string" }, "variables": { "type": "object" } }), &["sessionId", "query"]), true, false),
        tool("run_operation", "Run GraphQL operation", "Apply any kit-scoped GraphQL mutation as one hub operation (broadcast to all collaborators). The document must have the shape `mutation($storeId: ID!, $changeId: ID!, …) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { … } } } } } }`; the hub binds $storeId and $changeId. Every field that creates an entity (createDesign, createType, addFixedPiece, addChildPieceWithParentConnection, connectPieces, …) MUST pass an explicit `id` (uuid v7) so collaborators replay it identically.", schema(json!({ "sessionId": session, "query": { "type": "string" }, "variables": { "type": "object" } }), &["sessionId", "query"]), false, true),
    ]
}

fn text<'a>(args: &'a Value, key: &str) -> Result<&'a str, HubError> {
    args.get(key).and_then(Value::as_str).filter(|value| !value.trim().is_empty()).ok_or_else(|| HubError::BadRequest(format!("`{key}` is required")))
}

fn optional(args: &Value, key: &str) -> Option<String> {
    args.get(key).and_then(Value::as_str).map(str::to_string)
}

fn session_of(args: &Value) -> Result<Uuid, HubError> {
    text(args, "sessionId")?.parse().map_err(|_| HubError::BadRequest("`sessionId` must be a uuid".into()))
}

fn ids(args: &Value, key: &str) -> Result<Vec<String>, HubError> {
    let values: Vec<String> = args.get(key).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
    if values.is_empty() { Err(HubError::BadRequest(format!("`{key}` needs at least one id"))) } else { Ok(values) }
}

fn joint(args: &Value) -> Value {
    let joint: serde_json::Map<String, Value> = ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"].iter().filter_map(|key| args.get(*key).filter(|value| value.is_number()).map(|value| (key.to_string(), value.clone()))).collect();
    Value::Object(joint)
}

/// 🧑‍🚀 Joins (or refreshes) the caller's agent participant in the session.
async fn presence(hub: &Hub, agent: &Agent, session_id: Uuid) -> Result<Participant, HubError> {
    hub.agent_presence(&agent.caller, session_id, CLIENT).await
}

/// ✍️ Executes one kit-scoped operation (`declarations` extend the bound `$storeId`/`$changeId`, `body` is the selection inside `kit`).
async fn operate(hub: &Hub, agent: &Agent, session_id: Uuid, participant: &Participant, declarations: &str, body: &str, variables: Value) -> Result<OperationResult, HubError> {
    let query = format!("mutation($storeId: ID!, $changeId: ID!{declarations}) {{ session {{ store(id: $storeId) {{ theKit {{ unsavedChange(id: $changeId) {{ kit {{ {body} }} }} }} }} }} }}");
    hub.execute_operation(&agent.caller, session_id, OperationRequest { operation_id: Uuid::now_v7().to_string(), client_id: participant.client_id.clone(), base_version: None, query, variables }, Some(participant.id)).await
}

fn applied(result: &OperationResult, extra: Value) -> Value {
    let mut out = json!({ "version": result.version, "hash": result.hash });
    if let (Some(out), Value::Object(extra)) = (out.as_object_mut(), extra) {
        out.extend(extra);
    }
    out
}

fn focus(hub: &Hub, session_id: Uuid, participant: &Participant, design_id: &str, piece_ids: Vec<String>, connection_ids: Vec<String>) {
    let _ = hub.update_presence(session_id, participant.id, &json!({ "focus": { "app": "design", "designId": design_id }, "selection": { "designId": design_id, "pieceIds": piece_ids, "connectionIds": connection_ids } }));
}

async fn call(hub: &Hub, agent: &Agent, name: &str, args: &Value) -> Result<Value, HubError> {
    if !args.is_object() {
        return Err(HubError::BadRequest("arguments must be an object".into()));
    }
    match name {
        "list_sessions" => Ok(json!({ "sessions": hub.list_sessions(&agent.caller).await? })),
        "create_session" => {
            let kit = match optional(args, "fromSessionId") {
                Some(source) => Some(hub.session_kit(&agent.caller, source.parse().map_err(|_| HubError::BadRequest("`fromSessionId` must be a uuid".into()))?).await?.kit),
                None => None,
            };
            Ok(json!({ "session": hub.create_session(&agent.caller, text(args, "name")?, kit).await? }))
        }
        "list_participants" => {
            let session_id = session_of(args)?;
            presence(hub, agent, session_id).await?;
            Ok(json!({ "participants": hub.participants(&agent.caller, session_id).await? }))
        }
        "get_history" => {
            let session_id = session_of(args)?;
            presence(hub, agent, session_id).await?;
            let records = hub.history(&agent.caller, session_id, args.get("after").and_then(Value::as_i64).unwrap_or(0)).await?;
            let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(20) as usize;
            Ok(json!({ "total": records.len(), "operations": records[records.len().saturating_sub(limit)..] }))
        }
        "run_query" => {
            let session_id = session_of(args)?;
            presence(hub, agent, session_id).await?;
            hub.execute_query(&agent.caller, session_id, text(args, "query")?, args.get("variables").cloned()).await
        }
        "run_operation" => {
            let session_id = session_of(args)?;
            let query = text(args, "query")?;
            let missing = creations_without_ids(query);
            if !missing.is_empty() {
                return Err(HubError::BadRequest(format!("pass an explicit `id` (uuid v7) to {} so every collaborator replays the same ids", missing.join(", "))));
            }
            let participant = presence(hub, agent, session_id).await?;
            let variables = args.get("variables").cloned().unwrap_or_else(|| json!({}));
            let result = hub.execute_operation(&agent.caller, session_id, OperationRequest { operation_id: Uuid::now_v7().to_string(), client_id: participant.client_id.clone(), base_version: None, query: query.to_string(), variables }, Some(participant.id)).await?;
            Ok(json!({ "version": result.version, "hash": result.hash, "data": result.data }))
        }
        "read_kit" | "list_designs" | "read_design" | "list_types" => {
            let session_id = session_of(args)?;
            presence(hub, agent, session_id).await?;
            let state = hub.session_kit(&agent.caller, session_id).await?;
            match name {
                "read_kit" => Ok(kit_summary(&state)),
                "list_designs" => Ok(json!({ "designs": designs(&state.kit).iter().map(|(typology, design)| design_summary(typology, design)).collect::<Vec<_>>() })),
                "read_design" => read_design(&state.kit, text(args, "designId")?, args.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize, args.get("limit").and_then(Value::as_u64).unwrap_or(DEFAULT_LIMIT as u64) as usize),
                _ => Ok(list_types(&state.kit, optional(args, "query"), optional(args, "typologyId"))),
            }
        }
        "create_design" => {
            let session_id = session_of(args)?;
            let participant = presence(hub, agent, session_id).await?;
            let id = Uuid::now_v7().to_string();
            let result = operate(
                hub,
                agent,
                session_id,
                &participant,
                ", $id: ID!, $name: String!, $description: String, $icon: String, $unit: String",
                &format!("createDesign(id: $id, name: $name, description: $description, icon: $icon, unit: $unit) {RESPONSE}"),
                json!({ "id": id, "name": text(args, "name")?, "description": optional(args, "description"), "icon": optional(args, "icon"), "unit": optional(args, "unit") }),
            )
            .await?;
            focus(hub, session_id, &participant, &id, vec![], vec![]);
            Ok(applied(&result, json!({ "designId": id })))
        }
        "add_piece" => {
            let session_id = session_of(args)?;
            let design_id = text(args, "designId")?;
            let participant = presence(hub, agent, session_id).await?;
            let piece_id = Uuid::now_v7().to_string();
            let mut variables = json!({ "designId": design_id, "id": piece_id, "blueprintId": text(args, "typeId")?, "name": optional(args, "name"), "description": optional(args, "description") });
            let position = args.get("position").filter(|position| position.is_object()).cloned();
            let (declarations, field, connection_id) = match args.get("parent").filter(|parent| parent.is_object()) {
                Some(parent) => {
                    let connection_id = Uuid::now_v7().to_string();
                    for (key, value) in [("connectionId", json!(connection_id)), ("parentPieceId", json!(text(parent, "pieceId")?)), ("parentConnector", json!(text(parent, "connector")?)), ("childConnector", json!(text(parent, "childConnector")?)), ("joint", joint(parent)), ("position", position.clone().unwrap_or(Value::Null))] {
                        variables[key] = value;
                    }
                    (
                        ", $designId: ID!, $id: ID!, $connectionId: ID!, $blueprintId: ID!, $parentPieceId: ID!, $parentConnector: String!, $childConnector: String!, $name: String, $description: String, $position: PositionInput, $joint: ConnectionJointInput",
                        format!("addChildPieceWithParentConnection(id: $id, connectionId: $connectionId, blueprintId: $blueprintId, parentPieceId: $parentPieceId, parentConnector: $parentConnector, childConnector: $childConnector, name: $name, description: $description, position: $position, joint: $joint) {RESPONSE}"),
                        Some(connection_id),
                    )
                }
                None => {
                    variables["position"] = position.unwrap_or_else(|| json!({ "plane": { "origin": { "x": 0, "y": 0, "z": 0 }, "xAxis": { "x": 1, "y": 0, "z": 0 }, "yAxis": { "x": 0, "y": 1, "z": 0 } }, "center": { "u": 0, "v": 0 } }));
                    (", $designId: ID!, $id: ID!, $blueprintId: ID!, $position: PositionInput!, $name: String, $description: String", format!("addFixedPiece(id: $id, blueprintId: $blueprintId, position: $position, name: $name, description: $description) {RESPONSE}"), None)
                }
            };
            let result = operate(hub, agent, session_id, &participant, declarations, &format!("design(id: $designId) {{ {field} }}"), variables).await?;
            focus(hub, session_id, &participant, design_id, vec![piece_id.clone()], connection_id.iter().cloned().collect());
            Ok(applied(&result, json!({ "pieceId": piece_id, "connectionId": connection_id })))
        }
        "connect_pieces" => {
            let session_id = session_of(args)?;
            let design_id = text(args, "designId")?;
            let participant = presence(hub, agent, session_id).await?;
            let connection_id = Uuid::now_v7().to_string();
            let variables = json!({ "designId": design_id, "id": connection_id, "parentPieceId": text(args, "parentPieceId")?, "parentConnector": text(args, "parentConnector")?, "childPieceId": text(args, "childPieceId")?, "childConnector": text(args, "childConnector")?, "joint": joint(args) });
            let result = operate(
                hub,
                agent,
                session_id,
                &participant,
                ", $designId: ID!, $id: ID!, $parentPieceId: ID!, $parentConnector: String!, $childPieceId: ID!, $childConnector: String!, $joint: ConnectionJointInput",
                &format!("design(id: $designId) {{ connectPieces(id: $id, parentPieceId: $parentPieceId, parentConnector: $parentConnector, childPieceId: $childPieceId, childConnector: $childConnector, joint: $joint) {RESPONSE} }}"),
                variables,
            )
            .await?;
            focus(hub, session_id, &participant, design_id, vec![], vec![connection_id.clone()]);
            Ok(applied(&result, json!({ "connectionId": connection_id })))
        }
        "update_piece" => {
            let session_id = session_of(args)?;
            let design_id = text(args, "designId")?;
            let piece_id = text(args, "pieceId")?;
            let (mut declarations, mut fields, mut variables) = (String::from(", $designId: ID!, $pieceId: ID!"), Vec::new(), json!({ "designId": design_id, "pieceId": piece_id }));
            for (key, declaration, field) in [("name", ", $name: String!", "rename(newName: $name)"), ("description", ", $description: String!", "changeDescription(newDescription: $description)"), ("position", ", $position: PositionInput!", "move(position: $position)")] {
                if let Some(value) = args.get(key).filter(|value| !value.is_null()) {
                    declarations.push_str(declaration);
                    fields.push(format!("{key}: {field} {RESPONSE}"));
                    variables[key] = value.clone();
                }
            }
            if fields.is_empty() {
                return Err(HubError::BadRequest("give at least one of `name`, `description`, `position`".into()));
            }
            let participant = presence(hub, agent, session_id).await?;
            let result = operate(hub, agent, session_id, &participant, &declarations, &format!("design(id: $designId) {{ piece(id: $pieceId) {{ {} }} }}", fields.join(" ")), variables).await?;
            focus(hub, session_id, &participant, design_id, vec![piece_id.to_string()], vec![]);
            Ok(applied(&result, json!({ "pieceId": piece_id })))
        }
        "remove_pieces" | "remove_connections" => {
            let session_id = session_of(args)?;
            let design_id = text(args, "designId")?;
            let (key, field) = if name == "remove_pieces" { ("pieceIds", "deletePieces") } else { ("connectionIds", "deleteConnections") };
            let removed = ids(args, key)?;
            let participant = presence(hub, agent, session_id).await?;
            let result = operate(hub, agent, session_id, &participant, ", $designId: ID!, $ids: [ID!]!", &format!("design(id: $designId) {{ {field}(ids: $ids) {RESPONSE} }}"), json!({ "designId": design_id, "ids": removed })).await?;
            focus(hub, session_id, &participant, design_id, vec![], vec![]);
            let mut out = applied(&result, json!({}));
            out[key] = json!(removed);
            Ok(out)
        }
        other => Err(HubError::NotFound(format!("unknown tool `{other}`"))),
    }
}

const CREATING_FIELDS: [&str; 10] = ["createTag", "createConcept", "createQuality", "createType", "createDesign", "createFolder", "addFixedPiece", "addChildPieceWithParentConnection", "addHangingChildPieceWithParentConnection", "connectPieces"];

/// 🆔 Entity-creating fields of a document that lack an explicit `id` (and `connectionId` for child pieces).
pub fn creations_without_ids(query: &str) -> Vec<String> {
    fn walk(set: &SelectionSet, out: &mut Vec<String>) {
        for item in &set.items {
            if let Selection::Field(field) = &item.node {
                let name = field.node.name.node.as_str();
                let has = |argument: &str| field.node.arguments.iter().any(|(key, _)| key.node.as_str() == argument);
                if CREATING_FIELDS.contains(&name) && (!has("id") || (name.starts_with("addChild") || name.starts_with("addHanging")) && !has("connectionId")) {
                    out.push(format!("`{name}`"));
                }
                walk(&field.node.selection_set.node, out);
            }
        }
    }
    let mut out = Vec::new();
    if let Ok(document) = parse_query(query) {
        for (_, operation) in document.operations.iter() {
            walk(&operation.node.selection_set.node, &mut out);
        }
        for fragment in document.fragments.values() {
            walk(&fragment.node.selection_set.node, &mut out);
        }
    }
    out
}

//#endregion 🧰Tools

//#region 📚Projection

fn items(value: &Value) -> Vec<&Value> {
    match value {
        Value::Array(items) => items.iter().collect(),
        Value::Object(block) => block.get("items").and_then(Value::as_array).map(|items| items.iter().collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn typologies(kit: &Value) -> Vec<&Value> {
    items(&kit["typologies"])
}

fn designs(kit: &Value) -> Vec<(&Value, &Value)> {
    typologies(kit).into_iter().flat_map(|typology| items(&typology["designs"]).into_iter().map(move |design| (typology, design))).chain(items(&kit["designs"]).into_iter().map(|design| (&Value::Null, design))).collect()
}

fn types(kit: &Value) -> Vec<(&Value, &Value)> {
    typologies(kit).into_iter().flat_map(|typology| items(&typology["types"]).into_iter().map(move |ty| (typology, ty))).chain(items(&kit["types"]).into_iter().map(|ty| (&Value::Null, ty))).collect()
}

fn design_summary(typology: &Value, design: &Value) -> Value {
    json!({ "id": design["id"], "name": design["name"], "description": design.get("description"), "typologyId": typology.get("id"), "pieceCount": items(&design["pieces"]).len(), "connectionCount": items(&design["connections"]).len() })
}

fn type_summary(typology: &Value, ty: &Value) -> Value {
    json!({
        "id": ty["id"],
        "name": ty["name"],
        "description": ty.get("description"),
        "typologyId": typology.get("id"),
        "connectors": items(&ty["connectors"]).iter().map(|connector| json!({ "id": connector["id"], "name": connector.get("name"), "portId": connector.pointer("/port/id") })).collect::<Vec<_>>(),
        "representationCount": items(&ty["representations"]).len(),
    })
}

fn kit_summary(state: &KitState) -> Value {
    let kit = &state.kit;
    json!({
        "id": kit["id"],
        "name": kit["name"],
        "description": kit.get("description"),
        "sessionVersion": state.version,
        "hash": state.hash,
        "typologies": typologies(kit).iter().map(|typology| json!({ "id": typology["id"], "name": typology["name"], "typeCount": items(&typology["types"]).len(), "designCount": items(&typology["designs"]).len() })).collect::<Vec<_>>(),
        "families": items(&kit["families"]).iter().map(|family| json!({ "id": family["id"], "name": family.get("name") })).collect::<Vec<_>>(),
        "types": types(kit).iter().map(|(typology, ty)| json!({ "id": ty["id"], "name": ty["name"], "typologyId": typology.get("id"), "connectorCount": items(&ty["connectors"]).len() })).collect::<Vec<_>>(),
        "designs": designs(kit).iter().map(|(typology, design)| design_summary(typology, design)).collect::<Vec<_>>(),
    })
}

fn list_types(kit: &Value, query: Option<String>, typology_id: Option<String>) -> Value {
    let query = query.map(|query| query.to_lowercase());
    let listed: Vec<Value> = types(kit)
        .into_iter()
        .filter(|(typology, ty)| typology_id.as_deref().is_none_or(|id| typology.get("id").and_then(Value::as_str) == Some(id)) && query.as_deref().is_none_or(|query| ty["name"].as_str().unwrap_or_default().to_lowercase().contains(query)))
        .map(|(typology, ty)| type_summary(typology, ty))
        .collect();
    json!({ "types": listed })
}

fn read_design(kit: &Value, design_id: &str, offset: usize, limit: usize) -> Result<Value, HubError> {
    let (typology, design) = designs(kit).into_iter().find(|(_, design)| design["id"] == design_id).ok_or_else(|| HubError::NotFound(format!("design `{design_id}` not found")))?;
    let pieces = items(&design["pieces"]);
    let connections = items(&design["connections"]);
    let page = |list: &Vec<&Value>| list.iter().skip(offset).take(limit.max(1)).map(|value| (*value).clone()).collect::<Vec<_>>();
    let pieces_page: Vec<Value> = page(&pieces)
        .into_iter()
        .map(|piece| json!({ "id": piece["id"], "name": piece.get("name"), "description": piece.get("description"), "typeId": piece.pointer("/type/id"), "designId": piece.pointer("/design/id"), "pose": piece.get("pose"), "scale": piece.get("scale") }))
        .collect();
    Ok(json!({
        "id": design["id"],
        "name": design["name"],
        "description": design.get("description"),
        "unit": design.get("unit"),
        "typologyId": typology.get("id"),
        "fixedPieceIds": pieces.iter().filter(|piece| piece.get("pose").is_some_and(|pose| !pose.is_null())).map(|piece| piece["id"].clone()).collect::<Vec<_>>(),
        "pieceCount": pieces.len(),
        "connectionCount": connections.len(),
        "offset": offset,
        "pieces": pieces_page,
        "connections": page(&connections),
    }))
}

//#endregion 📚Projection

//#region 📖Resources

pub const GUIDE: &str = r#"# 🏘️ semio primer for AI collaborators

semio designs architecture as **kits of parts**. A hub **session** is one shared kit that several people and agents edit live together.

## Model
- **Kit** – the library of a session. **Typologies** group its **types** and **designs**; **families** group variants of a type; **qualities** define measurable properties; **files** hold geometry.
- **Type** – a reusable part (a capsule, a core segment, a bridge…). It exposes **connectors**: named docking points (point + direction) that may belong to a **port**; ports define which other ports are compatible. **Representations** are its geometry files.
- **Design** – an assembly of **pieces** joined by **connections**. A design can itself be used as the blueprint of a piece (nesting).
- **Piece** – one instance of a type (or of a design). A *fixed* piece has its own **pose**: a 3D **plane** (origin, xAxis, yAxis) plus a 2D **center** (u, v) in the design diagram. A *linked* piece has no pose: it is placed by the connection to its parent.
- **Connection** – joins a connector of a **parent** piece with a connector of a **child** piece. Joint parameters: `gap` (distance along the connector direction), `shift` / `rise` (offsets), `rotation` / `turn` / `tilt` (degrees) and `u` / `v` (diagram offset of the child relative to the parent).
- Every design needs at least one fixed piece; all other pieces hang off fixed ones through a tree of connections. Extra connections close loops.

## Working in a session
1. `list_sessions` (or `create_session`) → pick a `sessionId`.
2. `read_kit` → typologies, types and designs with ids.
3. `list_types` → connector ids and names of the types you want to place.
4. `read_design` → pieces, poses and connections of a design.
5. Write: `create_design` → `add_piece` (a fixed root with a `position`) → `add_piece` with `parent { pieceId, connector, childConnector }` for every further piece → `connect_pieces` to close loops → `update_piece`, `remove_pieces`, `remove_connections`.

Every write tool is **one operation**: it is versioned, persisted and broadcast to everybody in the session, and you appear to them as an agent participant whose selection follows your edits. Tools mint the ids of new entities and return them – reuse the returned ids. Use `get_history` to see what others changed and re-read before editing concurrently touched designs. `run_query` / `run_operation` give raw access to the semio GraphQL control plane.
"#;

async fn resources(hub: &Hub, agent: &Agent) -> Vec<Value> {
    let mut out = vec![json!({ "uri": "semio://guide", "name": "guide", "title": "semio primer", "description": "How semio kits, types, designs, pieces and connections work and how to collaborate through this server.", "mimeType": "text/markdown" })];
    for session in hub.list_sessions(&agent.caller).await.unwrap_or_default() {
        out.push(json!({ "uri": format!("semio://sessions/{}/kit", session.id), "name": format!("{}-kit", session.id), "title": format!("{} kit", session.name), "description": format!("Kit projection of session \"{}\" (version {}).", session.name, session.version), "mimeType": "application/json" }));
    }
    out
}

async fn read_resource(hub: &Hub, agent: &Agent, uri: &str) -> Result<Value, RpcError> {
    if uri == "semio://guide" {
        return Ok(json!({ "contents": [{ "uri": uri, "mimeType": "text/markdown", "text": GUIDE }] }));
    }
    let session_id = uri.strip_prefix("semio://sessions/").and_then(|rest| rest.strip_suffix("/kit")).and_then(|id| id.parse::<Uuid>().ok()).ok_or_else(|| RpcError(-32002, format!("resource `{uri}` not found")))?;
    let state = hub.session_kit(&agent.caller, session_id).await.map_err(|error| RpcError(-32002, format!("resource `{uri}`: {error}")))?;
    Ok(json!({ "contents": [{ "uri": uri, "mimeType": "application/json", "text": state.kit.to_string() }] }))
}

//#endregion 📖Resources

//#region 💬Prompts

fn prompt_definition() -> Value {
    json!({ "name": "design_assistant", "title": "semio design assistant", "description": "Collaborate on a design of a shared semio session: understand the kit, then build or edit the design step by step with the semio tools.", "arguments": [{ "name": "sessionId", "description": "Hub session to work in.", "required": true }, { "name": "goal", "description": "What to design or change.", "required": false }] })
}

fn prompt(params: &Value) -> Result<Value, RpcError> {
    if params.get("name").and_then(Value::as_str) != Some("design_assistant") {
        return Err(RpcError::invalid("unknown prompt"));
    }
    let arguments = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
    let session = arguments.get("sessionId").and_then(Value::as_str).ok_or_else(|| RpcError::invalid("`sessionId` is required"))?;
    let goal = arguments.get("goal").and_then(Value::as_str).unwrap_or("Improve the design together with me.");
    let text = format!("{GUIDE}\n---\nYou are working in the semio hub session `{session}` together with other people who see every change live.\nGoal: {goal}\n\nFirst call `read_kit` and `list_types` for this session, then `read_design` for the design you touch. Explain your plan briefly, make changes in small steps (one tool call per change) and summarize what you changed with the returned ids.");
    Ok(json!({ "description": "semio design assistant", "messages": [{ "role": "user", "content": { "type": "text", "text": text } }] }))
}

//#endregion 💬Prompts

} // 🧠Mcp
pub use mcp::*;

#[cfg(test)]
mod tests { // 📐Tests
// Specs: Integration tests run against a real Postgres (`SEMIO_HUB_TEST_DATABASE_URL`, default local dev server): every test gets its own temporary database (dropped afterwards) and a hub served on an ephemeral port, driven with reqwest + tokio-tungstenite.
// Summary: Auth, roles, sessions, operations, replay, shares and websocket tests.

use super::*;
use tokio_tungstenite::tungstenite;

//#region 🧪Harness

const METABOLISM: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/stores/metabolism/wip/initialKit/kit.semio.json");
const PASSWORD: &str = "correct horse battery";
const CREATE_DESIGN: &str = "mutation($storeId: ID!, $changeId: ID!, $id: ID!, $name: String!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { createDesign(id: $id, name: $name) { ok errors { message } } } } } } } }";
const RENAME_KIT: &str = "mutation($storeId: ID!, $changeId: ID!, $name: String!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { rename(newName: $name) { ok errors { message } } } } } } } }";
const DELETE_DESIGN: &str = "mutation($storeId: ID!, $changeId: ID!, $id: ID!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { deleteDesign(id: $id) { ok errors { message } } } } } } } }";
const KIT_NAME: &str = "query { session { stores { edges { node { wip { theKit { kit { name } } } } } } } }";

type Socket = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

/// 🗄️ Temporary database, dropped (with its connections) when the test ends.
struct TestDb {
    admin_url: String,
    name: String,
    url: String,
}

impl TestDb {
    async fn create() -> Self {
        let admin_url = std::env::var("SEMIO_HUB_TEST_DATABASE_URL").unwrap_or_else(|_| "postgres://semio:semio@localhost:5432/semio".into());
        let name = format!("semio_hub_test_{}", Uuid::new_v4().simple());
        let admin = connect(&admin_url).await.expect("test database server");
        sqlx_core::raw_sql::raw_sql(&format!("CREATE DATABASE {name}")).execute(&admin).await.expect("create test database");
        admin.close().await;
        let (server, _) = admin_url.rsplit_once('/').expect("database url");
        Self { url: format!("{server}/{name}"), admin_url, name }
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let (admin_url, name) = (self.admin_url.clone(), self.name.clone());
        let _ = std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().expect("runtime").block_on(async move {
                if let Ok(admin) = connect(&admin_url).await {
                    let _ = sqlx_core::raw_sql::raw_sql(&format!("DROP DATABASE IF EXISTS {name} WITH (FORCE)")).execute(&admin).await;
                }
            })
        })
        .join();
    }
}

/// 🧪 Hub served on an ephemeral port plus an HTTP client.
struct TestHub {
    hub: Hub,
    base: String,
    http: reqwest::Client,
    server: tokio::task::JoinHandle<()>,
    _db: TestDb,
}

impl Drop for TestHub {
    fn drop(&mut self) {
        self.server.abort();
    }
}

async fn start() -> TestHub {
    let db = TestDb::create().await;
    let pool = connect(&db.url).await.expect("test pool");
    migrate(&pool).await.expect("schema");
    let hub = Hub::new(pool, Arc::new(Argon2id::fast()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("listener");
    let base = format!("http://{}", listener.local_addr().expect("address"));
    let app = router(hub.clone());
    let server = tokio::spawn(async move { axum::serve(listener, app).await.expect("serve") });
    TestHub { hub, base, http: reqwest::Client::new(), server, _db: db }
}

impl TestHub {
    async fn call(&self, method: reqwest::Method, path: &str, token: Option<&str>, body: Option<Value>) -> (u16, Value) {
        let mut request = self.http.request(method, format!("{}{path}", self.base));
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request.send().await.expect("request");
        let status = response.status().as_u16();
        let text = response.text().await.expect("body");
        (status, if text.is_empty() { Value::Null } else { serde_json::from_str(&text).unwrap_or(Value::String(text)) })
    }

    async fn get(&self, path: &str, token: Option<&str>) -> (u16, Value) {
        self.call(reqwest::Method::GET, path, token, None).await
    }

    async fn post(&self, path: &str, token: Option<&str>, body: Value) -> (u16, Value) {
        self.call(reqwest::Method::POST, path, token, Some(body)).await
    }

    async fn delete(&self, path: &str, token: Option<&str>) -> (u16, Value) {
        self.call(reqwest::Method::DELETE, path, token, None).await
    }

    async fn register(&self, name: &str) -> (String, Value) {
        let (status, body) = self.post("/auth/register", None, json!({ "name": name, "email": format!("{name}@Semio.Test"), "password": PASSWORD })).await;
        assert_eq!(status, 201, "register {name}: {body}");
        (body["token"].as_str().expect("token").to_string(), body["person"].clone())
    }

    async fn create_session(&self, token: &str, name: &str, kit: Option<Value>) -> Value {
        let (status, body) = self.post("/sessions", Some(token), json!({ "name": name, "kit": kit })).await;
        assert_eq!(status, 201, "create session: {body}");
        body
    }

    async fn operate(&self, token: &str, session: &str, operation_id: &str, (query, variables): (&str, Value)) -> (u16, Value) {
        self.post(&format!("/sessions/{session}/operations"), Some(token), json!({ "operationId": operation_id, "clientId": "test-client", "baseVersion": 0, "query": query, "variables": variables })).await
    }

    async fn share(&self, token: &str, session: &str, role: &str) -> String {
        let (status, body) = self.post(&format!("/sessions/{session}/shares"), Some(token), json!({ "role": role, "label": format!("{role} link") })).await;
        assert_eq!(status, 201, "share: {body}");
        body["token"].as_str().expect("share token").to_string()
    }

    async fn join(&self, token: &str, share: &str) -> Value {
        let (status, body) = self.post(&format!("/shares/{share}/join"), Some(token), json!({})).await;
        assert_eq!(status, 200, "join: {body}");
        body
    }

    async fn socket(&self, session: &str, token: &str, client_id: &str) -> Result<Socket, tungstenite::Error> {
        let url = format!("{}/sessions/{session}/ws?token={token}&clientId={client_id}&client=test", self.base.replacen("http", "ws", 1));
        tokio_tungstenite::connect_async(url).await.map(|(socket, _)| socket)
    }

    async fn caller(&self, token: &str) -> Caller {
        self.hub.authenticate(token).await.expect("caller")
    }
}

fn create_design(id: &str, name: &str) -> (&'static str, Value) {
    (CREATE_DESIGN, json!({ "id": id, "name": name }))
}

fn rename_kit(name: &str) -> (&'static str, Value) {
    (RENAME_KIT, json!({ "name": name }))
}

fn metabolism() -> Value {
    serde_json::from_str(&std::fs::read_to_string(METABOLISM).expect("metabolism fixture")).expect("fixture json")
}

fn design_ids(kit: &Value) -> Vec<String> {
    kit["typologies"]["items"].as_array().into_iter().flatten().flat_map(|typology| typology["designs"]["items"].as_array().into_iter().flatten()).filter_map(|design| design["id"].as_str().map(str::to_string)).collect()
}

async fn next(socket: &mut Socket) -> Value {
    loop {
        match tokio::time::timeout(Duration::from_secs(20), socket.next()).await.expect("websocket message in time") {
            Some(Ok(tungstenite::Message::Text(text))) => return serde_json::from_str(text.as_str()).expect("json message"),
            Some(Ok(tungstenite::Message::Close(_))) | None => return json!({ "type": "closed" }),
            Some(Ok(_)) => continue,
            Some(Err(error)) => panic!("websocket error: {error}"),
        }
    }
}

async fn until(socket: &mut Socket, matches: impl Fn(&Value) -> bool) -> Value {
    loop {
        let message = next(socket).await;
        if matches(&message) {
            return message;
        }
        assert_ne!(message["type"], "closed", "socket closed before the expected message");
    }
}

async fn say(socket: &mut Socket, message: Value) {
    socket.send(tungstenite::Message::Text(message.to_string().into())).await.expect("send");
}

fn handshake_status(result: Result<Socket, tungstenite::Error>) -> u16 {
    match result {
        Err(tungstenite::Error::Http(response)) => response.status().as_u16(),
        Err(error) => panic!("unexpected websocket error: {error}"),
        Ok(_) => 101,
    }
}

//#endregion 🧪Harness

mod adapter_tests { // 🔌Adapter Tests

use super::*;

#[test]
fn documents_are_classified() {
    assert_eq!(inspect_document(KIT_NAME), Ok(DocumentKind::Read));
    assert_eq!(inspect_document(CREATE_DESIGN), Ok(DocumentKind::KitOperation));
    assert_eq!(inspect_document("fragment K on OperationInput { rename(newName: \"x\") { ok } } mutation($storeId: ID!, $changeId: ID!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { ...K } } } } } }"), Ok(DocumentKind::KitOperation));
    assert!(inspect_document("mutation($storeId: ID!, $json: String!) { session { store(id: $storeId) { installProjection(json: $json) { ok } } } }").is_err());
    assert!(inspect_document("mutation { session { store(id: \"x\") { backbone { detach { ok } } } } }").is_err());
    assert!(inspect_document("mutation { session { store(id: \"x\") { theKit { startNewChange { ok } } } } }").is_err());
    assert!(inspect_document("subscription { __typename }").is_err());
    assert!(inspect_document("fragment A on X { ...B } fragment B on X { ...A } mutation { ...A }").is_err());
    assert!(inspect_document("not graphql").is_err());
}

#[test]
fn passwords_and_tokens_are_never_stored_in_clear() {
    let kdf = Argon2id::fast();
    let encoded = kdf.hash(PASSWORD).expect("hash");
    assert!(encoded.starts_with("$argon2id$") && !encoded.contains(PASSWORD));
    assert!(kdf.verify(PASSWORD, &encoded));
    assert!(!kdf.verify("wrong password", &encoded));
    assert!(Argon2id::strong().verify(PASSWORD, &encoded));
    let token = mint_token();
    assert_eq!(token.len(), 64);
    assert_ne!(token, mint_token());
    assert_eq!(digest(&token), digest(&token));
    assert_ne!(digest(&token), token);
}

#[test]
fn failures_are_collected_from_errors_and_responses() {
    assert!(failures(&json!({ "data": { "a": { "ok": true } } })).is_empty());
    assert_eq!(failures(&json!({ "errors": [{ "message": "boom" }], "data": null })), vec!["boom"]);
    assert_eq!(failures(&json!({ "data": { "a": { "b": { "ok": false, "errors": [{ "message": "not found" }] } } } })), vec!["not found"]);
    assert_eq!(failures(&json!({ "data": { "a": [{ "ok": false }] } })), vec!["operation failed"]);
}

#[test]
fn server_messages_follow_the_protocol() {
    let message = serde_json::to_value(ServerMessage::PresenceLeft { participant_id: Uuid::nil() }).expect("json");
    assert_eq!(message, json!({ "type": "presence.left", "participantId": Uuid::nil() }));
    assert_eq!(serde_json::to_value(ServerMessage::Pong).expect("json"), json!({ "type": "pong" }));
    assert_eq!(person_color(Uuid::nil()), person_color(Uuid::nil()));
}

} // 🔌Adapter Tests

mod api_tests { // 🛕Api Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn cors_preflight_health_and_json_errors() {
    let t = start().await;
    let preflight = t
        .http
        .request(reqwest::Method::OPTIONS, format!("{}/sessions", t.base))
        .header("origin", "http://localhost:5173")
        .header("access-control-request-method", "POST")
        .header("access-control-request-headers", "authorization,content-type")
        .send()
        .await
        .expect("preflight");
    assert!(preflight.status().is_success(), "{}", preflight.status());
    let headers = preflight.headers();
    assert_eq!(headers.get("access-control-allow-origin").and_then(|value| value.to_str().ok()), Some("*"));
    assert!(headers.get("access-control-allow-headers").and_then(|value| value.to_str().ok()).is_some_and(|value| value.to_lowercase().contains("authorization")));
    assert_eq!(t.http.get(format!("{}/health", t.base)).send().await.expect("health").text().await.expect("body"), "ok");
    let (status, body) = t.get("/nowhere", None).await;
    assert_eq!((status, body["error"].is_string()), (404, true));
    let (status, body) = t.call(reqwest::Method::POST, "/auth/register", None, None).await;
    assert_eq!((status, body["error"].is_string()), (400, true));
}

} // 🛕Api Tests

mod auth_tests { // 🔑Auth Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn register_login_me_logout_and_agent_tokens() {
    let t = start().await;
    let (token, person) = t.register("alice").await;
    assert_eq!(person["email"], "alice@semio.test");
    assert!(person["color"].as_str().is_some_and(|color| color.starts_with('#')));
    assert_eq!(t.post("/auth/register", None, json!({ "name": "alice", "email": "ALICE@semio.test", "password": PASSWORD })).await.0, 409);
    assert_eq!(t.post("/auth/register", None, json!({ "name": "bob", "email": "bob@semio.test", "password": "short" })).await.0, 400);
    let (status, body) = t.post("/auth/register", None, json!({ "name": "bob" })).await;
    assert_eq!(status, 400);
    assert!(body["error"].is_string(), "{body}");
    assert_eq!(t.post("/auth/login", None, json!({ "email": "alice@semio.test", "password": "wrong password" })).await.0, 401);
    assert_eq!(t.post("/auth/login", None, json!({ "email": "nobody@semio.test", "password": PASSWORD })).await.0, 401);
    let (status, login) = t.post("/auth/login", None, json!({ "email": "Alice@Semio.test", "password": PASSWORD })).await;
    assert_eq!(status, 200, "{login}");
    let second = login["token"].as_str().expect("token").to_string();
    assert_ne!(second, token);
    assert_eq!(login["person"], person);
    let (status, me) = t.get("/auth/me", Some(&second)).await;
    assert_eq!((status, &me["person"]), (200, &person));
    assert_eq!(t.get("/auth/me", None).await.0, 401);
    assert_eq!(t.get("/auth/me", Some("garbage")).await.0, 401);
    let (status, agent) = t.post("/auth/tokens", Some(&token), json!({ "label": "cli" })).await;
    assert_eq!(status, 201, "{agent}");
    assert_eq!((&agent["label"], &agent["kind"]), (&json!("cli"), &json!("agent")));
    let agent_token = agent["token"].as_str().expect("agent token");
    assert_eq!(t.get("/auth/me", Some(agent_token)).await.1["person"], person);
    assert!(matches!(t.caller(agent_token).await, Caller::Person(Principal { kind: ParticipantKind::Agent, .. })));
    assert!(matches!(t.caller(&token).await, Caller::Person(Principal { kind: ParticipantKind::Human, .. })));
    assert_eq!(t.post("/auth/tokens", None, json!({})).await.0, 401);
    assert_eq!(t.call(reqwest::Method::POST, "/auth/logout", Some(&second), None).await.0, 204);
    assert_eq!(t.get("/auth/me", Some(&second)).await.0, 401);
    assert_eq!(t.call(reqwest::Method::POST, "/auth/logout", Some(&second), None).await.0, 401);
    assert_eq!(t.get("/auth/me", Some(&token)).await.0, 200);
    let stored: Vec<(String,)> = sqlx_core::query_as::query_as("SELECT hash FROM semio.token").fetch_all(&t.hub.pool).await.expect("tokens");
    assert!(stored.iter().all(|(hash,)| hash != &token && hash != agent_token));
    assert!(stored.iter().any(|(hash,)| hash == &digest(&token)));
    let (encoded,): (String,) = sqlx_core::query_as::query_as("SELECT password_hash FROM semio.credential").fetch_one(&t.hub.pool).await.expect("credential");
    assert!(encoded.starts_with("$argon2id$"));
}

} // 🔑Auth Tests

mod role_tests { // 🛂Role Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn roles_are_enforced_on_every_endpoint() {
    let t = start().await;
    let (owner, _) = t.register("owner").await;
    let (stranger, _) = t.register("stranger").await;
    let (editor, _) = t.register("editor").await;
    let (viewer, _) = t.register("viewer").await;
    let session = t.create_session(&owner, "Roles", None).await;
    let id = session["id"].as_str().expect("id").to_string();
    let editor_share = t.share(&owner, &id, "editor").await;
    let viewer_share = t.share(&owner, &id, "viewer").await;
    t.join(&editor, &editor_share).await;
    t.join(&viewer, &viewer_share).await;
    let counter = AtomicU64::new(0);
    let operation = || {
        let n = counter.fetch_add(1, Ordering::SeqCst);
        json!({ "operationId": format!("op-{n}"), "clientId": "c", "query": CREATE_DESIGN, "variables": { "id": Uuid::now_v7(), "name": format!("D{n}") } })
    };
    let endpoints: Vec<(reqwest::Method, String, Option<Value>, Role)> = vec![
        (reqwest::Method::GET, format!("/sessions/{id}"), None, Role::Viewer),
        (reqwest::Method::GET, format!("/sessions/{id}/kit"), None, Role::Viewer),
        (reqwest::Method::GET, format!("/sessions/{id}/kit/at/0"), None, Role::Viewer),
        (reqwest::Method::GET, format!("/sessions/{id}/operations?after=0"), None, Role::Viewer),
        (reqwest::Method::POST, format!("/sessions/{id}/graphql"), Some(json!({ "query": KIT_NAME })), Role::Viewer),
        (reqwest::Method::GET, format!("/sessions/{id}/members"), None, Role::Viewer),
        (reqwest::Method::GET, format!("/sessions/{id}/presence"), None, Role::Viewer),
        (reqwest::Method::POST, format!("/sessions/{id}/operations"), None, Role::Editor),
        (reqwest::Method::GET, format!("/sessions/{id}/shares"), None, Role::Owner),
        (reqwest::Method::POST, format!("/sessions/{id}/shares"), Some(json!({ "role": "viewer" })), Role::Owner),
        (reqwest::Method::DELETE, format!("/sessions/{id}/shares/unknown"), None, Role::Owner),
    ];
    for (method, path, body, min) in &endpoints {
        let body = || body.clone().or_else(|| path.ends_with("/operations").then(operation));
        assert_eq!(t.call(method.clone(), path, None, body()).await.0, 401, "{method} {path} without token");
        assert_eq!(t.call(method.clone(), path, Some("garbage"), body()).await.0, 401, "{method} {path} with garbage token");
        assert_eq!(t.call(method.clone(), path, Some(&stranger), body()).await.0, 403, "{method} {path} as stranger");
        for (token, role) in [(&viewer, Role::Viewer), (&editor, Role::Editor), (&owner, Role::Owner)] {
            let (status, response) = t.call(method.clone(), path, Some(token), body()).await;
            if role >= *min {
                assert!(![401, 403].contains(&status), "{method} {path} as {role:?}: {status} {response}");
            } else {
                assert_eq!(status, 403, "{method} {path} as {role:?}: {response}");
            }
        }
        let (status, _) = t.call(method.clone(), path, Some(&viewer_share), body()).await;
        assert_eq!(status == 403, *min > Role::Viewer, "{method} {path} with share token: {status}");
    }
    let unknown = Uuid::now_v7();
    assert_eq!(t.get(&format!("/sessions/{unknown}"), Some(&owner)).await.0, 404);
    assert_eq!(t.get(&format!("/sessions/{unknown}/kit"), Some(&owner)).await.0, 404);
    assert_eq!(t.get("/sessions/not-a-uuid", Some(&owner)).await.0, 400);
    assert_eq!(t.get("/sessions", None).await.0, 401);
    assert_eq!(t.get("/sessions", Some(&viewer_share)).await.0, 401);
    assert_eq!(t.post("/sessions", Some(&viewer_share), json!({ "name": "x" })).await.0, 401);
    assert_eq!(handshake_status(t.socket(&id, "garbage", "c").await), 401);
    assert_eq!(handshake_status(t.socket(&id, &stranger, "c").await), 403);
    assert_eq!(handshake_status(t.socket(&unknown.to_string(), &owner, "c").await), 404);
    assert_eq!(handshake_status(t.socket(&id, &viewer, "c").await), 101);
    assert_eq!(t.delete(&format!("/sessions/{id}"), Some(&editor)).await.0, 403);
    assert_eq!(t.delete(&format!("/sessions/{id}"), Some(&viewer)).await.0, 403);
    assert_eq!(t.delete(&format!("/sessions/{id}"), Some(&owner)).await.0, 204);
    assert_eq!(t.get(&format!("/sessions/{id}"), Some(&owner)).await.0, 404);
    assert_eq!(t.get(&format!("/sessions/{id}/kit"), Some(&viewer_share)).await.0, 401);
}

} // 🛂Role Tests

mod session_tests { // 🏘️Session Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sessions_from_the_metabolism_kit_and_from_an_empty_kit() {
    let t = start().await;
    let (owner, person) = t.register("owner").await;
    let empty = t.create_session(&owner, "  Empty  ", None).await;
    assert_eq!((&empty["name"], &empty["role"], &empty["version"], &empty["participantCount"]), (&json!("Empty"), &json!("owner"), &json!(0), &json!(0)));
    assert_eq!(empty["owner"], json!({ "id": person["id"], "name": "owner" }));
    assert_eq!(empty["hash"].as_str().map(str::len), Some(64));
    let empty_id = empty["id"].as_str().expect("id");
    let (status, kit) = t.get(&format!("/sessions/{empty_id}/kit"), Some(&owner)).await;
    assert_eq!(status, 200, "{kit}");
    assert_eq!((&kit["version"], &kit["hash"], &kit["kit"]["name"]), (&json!(0), &empty["hash"], &json!("Empty")));
    let fixture = metabolism();
    let session = t.create_session(&owner, "Metabolism", Some(fixture.clone())).await;
    let id = session["id"].as_str().expect("id");
    let (status, served) = t.get(&format!("/sessions/{id}/kit"), Some(&owner)).await;
    assert_eq!(status, 200);
    assert_eq!((&served["kit"]["id"], &served["kit"]["name"], &served["hash"]), (&fixture["id"], &json!("Metabolism"), &session["hash"]));
    assert_ne!(served["hash"], empty["hash"]);
    assert_eq!(served["kit"]["typologies"]["items"].as_array().map(Vec::len), fixture["typologies"]["items"].as_array().map(Vec::len));
    assert!(!design_ids(&served["kit"]).is_empty());
    let replica = KitStore::open("replica", &served["kit"]).await.expect("replica");
    assert_eq!(replica.hash().await.expect("replica hash"), served["hash"].as_str().expect("hash"), "installProjection(kit) on a replica must reproduce the hub hash");
    let (status, listed) = t.get("/sessions", Some(&owner)).await;
    assert_eq!(status, 200);
    let names: Vec<&str> = listed.as_array().expect("list").iter().filter_map(|summary| summary["name"].as_str()).collect();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"Empty") && names.contains(&"Metabolism"));
    let (status, fetched) = t.get(&format!("/sessions/{id}"), Some(&owner)).await;
    assert_eq!((status, &fetched["id"], &fetched["role"]), (200, &session["id"], &json!("owner")));
    let (_, members) = t.get(&format!("/sessions/{id}/members"), Some(&owner)).await;
    assert_eq!(members, json!([{ "person": person, "role": "owner" }]));
    assert_eq!(t.post("/sessions", Some(&owner), json!({ "name": "Bad", "kit": 42 })).await.0, 400);
    assert_eq!(t.post("/sessions", Some(&owner), json!({ "name": " " })).await.0, 400);
    assert_eq!(t.post("/sessions", Some(&owner), json!({})).await.0, 400);
}

} // 🏘️Session Tests

mod operation_tests { // 📜Operation Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn operations_are_sequenced_idempotent_versioned_and_hashed() {
    let t = start().await;
    let (owner, person) = t.register("owner").await;
    let session = t.create_session(&owner, "Ops", None).await;
    let id = session["id"].as_str().expect("id");
    let h0 = session["hash"].clone();
    let design = Uuid::now_v7().to_string();
    let (status, first) = t.operate(&owner, id, "op-1", create_design(&design, "First")).await;
    assert_eq!(status, 200, "{first}");
    assert_eq!(first["version"], 1);
    assert_ne!(first["hash"], h0);
    assert_eq!(first["data"].pointer("/session/store/theKit/unsavedChange/kit/createDesign/ok"), Some(&json!(true)), "{first}");
    let (status, again) = t.operate(&owner, id, "op-1", create_design(&Uuid::now_v7().to_string(), "Ignored")).await;
    assert_eq!((status, &again), (200, &first), "duplicate operationId returns the original result");
    let (status, second) = t.operate(&owner, id, "op-2", rename_kit("Renamed")).await;
    assert_eq!((status, &second["version"]), (200, &json!(2)), "{second}");
    let (status, conflict) = t.post(&format!("/sessions/{id}/operations"), Some(&owner), json!({ "operationId": "op-x", "clientId": "c", "baseVersion": 99, "query": RENAME_KIT, "variables": { "name": "x" } })).await;
    assert_eq!(status, 409, "{conflict}");
    for query in [KIT_NAME, "mutation($storeId: ID!, $json: String!) { session { store(id: $storeId) { installProjection(json: $json) { ok } } } }", "mutation { session { store(id: \"x\") { backbone { detach { ok } } } } }", "garbage"] {
        let (status, body) = t.post(&format!("/sessions/{id}/operations"), Some(&owner), json!({ "operationId": "op-bad", "clientId": "c", "query": query, "variables": { "json": "{}" } })).await;
        assert_eq!(status, 400, "{query}: {body}");
    }
    assert_eq!(t.post(&format!("/sessions/{id}/operations"), Some(&owner), json!({ "operationId": "", "clientId": "c", "query": RENAME_KIT, "variables": { "name": "x" } })).await.0, 400);
    assert_eq!(t.post(&format!("/sessions/{id}/operations"), Some(&owner), json!({ "operationId": "op-v", "clientId": "c", "query": RENAME_KIT, "variables": [1] })).await.0, 400);
    let (status, rejected) = t.operate(&owner, id, "op-fail", (DELETE_DESIGN, json!({ "id": "missing-design" }))).await;
    assert_eq!(status, 422, "{rejected}");
    assert!(rejected["error"].is_string());
    let (status, unbound) = t.operate(&owner, id, "op-unbound", (CREATE_DESIGN, json!({ "name": "no id" }))).await;
    assert_eq!(status, 422, "missing variable: {unbound}");
    let (_, fetched) = t.get(&format!("/sessions/{id}"), Some(&owner)).await;
    assert_eq!((&fetched["version"], &fetched["hash"]), (&json!(2), &second["hash"]), "rejected operations are not recorded");
    let (status, third) = t.operate(&owner, id, "op-3", create_design(&Uuid::now_v7().to_string(), "Third")).await;
    assert_eq!((status, &third["version"]), (200, &json!(3)), "{third}");
    let (_, history) = t.get(&format!("/sessions/{id}/operations?after=0"), Some(&owner)).await;
    let history = history.as_array().expect("history").clone();
    assert_eq!(history.iter().map(|record| record["version"].clone()).collect::<Vec<_>>(), vec![json!(1), json!(2), json!(3)]);
    assert_eq!(history.iter().map(|record| record["operationId"].clone()).collect::<Vec<_>>(), vec![json!("op-1"), json!("op-2"), json!("op-3")]);
    assert_eq!((&history[0]["personId"], &history[0]["participantKind"], &history[0]["clientId"]), (&person["id"], &json!("human"), &json!("test-client")));
    assert_eq!((&history[0]["query"], &history[0]["variables"], &history[0]["hash"]), (&json!(CREATE_DESIGN), &json!({ "id": design, "name": "First" }), &first["hash"]));
    assert!(history[0]["createdAt"].is_string());
    assert_eq!(t.get(&format!("/sessions/{id}/operations?after=2"), Some(&owner)).await.1.as_array().map(Vec::len), Some(1));
    let (_, kit) = t.get(&format!("/sessions/{id}/kit"), Some(&owner)).await;
    assert_eq!((&kit["version"], &kit["hash"], &kit["kit"]["name"]), (&json!(3), &third["hash"], &json!("Renamed")));
    assert!(design_ids(&kit["kit"]).contains(&design), "{}", kit["kit"]);
    let (status, read) = t.post(&format!("/sessions/{id}/graphql"), Some(&owner), json!({ "query": KIT_NAME })).await;
    assert_eq!(status, 200);
    assert_eq!(read.pointer("/data/session/stores/edges/0/node/wip/theKit/kit/name"), Some(&json!("Renamed")), "{read}");
    assert_eq!(t.post(&format!("/sessions/{id}/graphql"), Some(&owner), json!({ "query": RENAME_KIT, "variables": { "name": "x" } })).await.0, 400);
    let (_, initial) = t.get(&format!("/sessions/{id}/kit/at/0"), Some(&owner)).await;
    let replica = KitStore::open("replica", &initial["kit"]).await.expect("replica");
    for record in &history {
        replica.apply(record["query"].as_str().expect("query"), &record["variables"]).await.expect("replay");
        assert_eq!(replica.hash().await.expect("hash"), record["hash"].as_str().expect("hash"), "replica replay diverged at version {}", record["version"]);
    }
}

} // 📜Operation Tests

mod replay_tests { // 🔁Replay Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reload_from_snapshot_and_replay_reproduces_the_hash() {
    let t = start().await;
    let (owner, _) = t.register("owner").await;
    let session = t.create_session(&owner, "Replay", None).await;
    let id = session["id"].as_str().expect("id");
    let session_id: Uuid = id.parse().expect("uuid");
    let mut hashes = vec![session["hash"].clone()];
    for n in 1..=SNAPSHOT_INTERVAL + 5 {
        let operation = if n % 2 == 0 { rename_kit(&format!("Kit {n}")) } else { create_design(&Uuid::now_v7().to_string(), &format!("Design {n}")) };
        let (status, result) = t.operate(&owner, id, &format!("op-{n}"), operation).await;
        assert_eq!((status, &result["version"]), (200, &json!(n)), "{result}");
        hashes.push(result["hash"].clone());
    }
    let snapshots: Vec<(i64,)> = sqlx_core::query_as::query_as("SELECT version FROM semio.snapshot WHERE session_id = $1 ORDER BY version").bind(session_id).fetch_all(&t.hub.pool).await.expect("snapshots");
    assert_eq!(snapshots, vec![(0,), (SNAPSHOT_INTERVAL,)]);
    let live = t.get(&format!("/sessions/{id}/kit"), Some(&owner)).await.1;
    assert_eq!(live["hash"], hashes[hashes.len() - 1]);
    t.hub.evict(session_id);
    let reloaded = t.get(&format!("/sessions/{id}/kit"), Some(&owner)).await.1;
    assert_eq!((&reloaded["version"], &reloaded["hash"]), (&live["version"], &live["hash"]), "reload = snapshot + replay");
    assert_eq!(reloaded["kit"], live["kit"]);
    let restarted = Hub::new(t.hub.pool.clone(), Arc::new(Argon2id::fast()));
    let caller = restarted.authenticate(&owner).await.expect("caller");
    assert_eq!(json!(restarted.session_kit(&caller, session_id).await.expect("kit").hash), live["hash"]);
    for version in [0, 1, 2, SNAPSHOT_INTERVAL - 1, SNAPSHOT_INTERVAL, SNAPSHOT_INTERVAL + 3] {
        let (status, state) = t.get(&format!("/sessions/{id}/kit/at/{version}"), Some(&owner)).await;
        assert_eq!((status, &state["version"], &state["hash"]), (200, &json!(version), &hashes[version as usize]), "kit at {version}");
    }
    assert_eq!(t.get(&format!("/sessions/{id}/kit/at/{}", SNAPSHOT_INTERVAL + 6), Some(&owner)).await.0, 404);
    assert_eq!(t.get(&format!("/sessions/{id}/kit/at/-1"), Some(&owner)).await.0, 404);
    let (status, next) = t.operate(&owner, id, "op-after-reload", rename_kit("After reload")).await;
    assert_eq!((status, &next["version"]), (200, &json!(SNAPSHOT_INTERVAL + 6)), "{next}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn metabolism_session_reloads_identically() {
    let t = start().await;
    let (owner, _) = t.register("owner").await;
    let session = t.create_session(&owner, "Metabolism", Some(metabolism())).await;
    let id = session["id"].as_str().expect("id");
    let design = Uuid::now_v7().to_string();
    assert_eq!(t.operate(&owner, id, "op-1", create_design(&design, "Capsule Cluster")).await.0, 200);
    let (status, last) = t.operate(&owner, id, "op-2", rename_kit("Metabolism Remix")).await;
    assert_eq!(status, 200, "{last}");
    t.hub.evict(id.parse().expect("uuid"));
    let (_, reloaded) = t.get(&format!("/sessions/{id}/kit"), Some(&owner)).await;
    assert_eq!((&reloaded["version"], &reloaded["hash"], &reloaded["kit"]["name"]), (&json!(2), &last["hash"], &json!("Metabolism Remix")));
    assert!(design_ids(&reloaded["kit"]).contains(&design));
}

} // 🔁Replay Tests

mod share_tests { // 🔗Share Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn shares_grant_membership_without_downgrades() {
    let t = start().await;
    let (owner, owner_person) = t.register("owner").await;
    let (bob, bob_person) = t.register("bob").await;
    let (carol, _) = t.register("carol").await;
    let session = t.create_session(&owner, "Shared", None).await;
    let id = session["id"].as_str().expect("id");
    let (status, share) = t.post(&format!("/sessions/{id}/shares"), Some(&owner), json!({ "role": "editor", "label": "team" })).await;
    assert_eq!(status, 201, "{share}");
    assert_eq!((&share["role"], &share["label"]), (&json!("editor"), &json!("team")));
    let editor_share = share["token"].as_str().expect("token").to_string();
    let viewer_share = t.share(&owner, id, "viewer").await;
    let (_, shares) = t.get(&format!("/sessions/{id}/shares"), Some(&owner)).await;
    assert_eq!(shares.as_array().map(Vec::len), Some(2));
    assert_eq!(t.post(&format!("/sessions/{id}/shares"), Some(&owner), json!({ "role": "owner" })).await.0, 400);
    assert_eq!(t.post(&format!("/sessions/{id}/shares"), Some(&owner), json!({ "role": "admin" })).await.0, 400);
    let joined = t.join(&bob, &editor_share).await;
    assert_eq!((&joined["role"], &joined["session"]["id"], &joined["session"]["role"]), (&json!("editor"), &session["id"], &json!("editor")));
    assert_eq!(t.join(&bob, &viewer_share).await["role"], "editor", "joining never downgrades");
    assert_eq!(t.join(&carol, &viewer_share).await["role"], "viewer");
    assert_eq!(t.join(&carol, &editor_share).await["role"], "editor", "joining upgrades");
    assert_eq!(t.join(&owner, &viewer_share).await["role"], "owner");
    let (_, members) = t.get(&format!("/sessions/{id}/members"), Some(&bob)).await;
    assert_eq!(members.as_array().map(Vec::len), Some(3));
    assert_eq!(members[0], json!({ "person": owner_person, "role": "owner" }));
    assert_eq!(members[1], json!({ "person": bob_person, "role": "editor" }));
    let (_, listed) = t.get("/sessions", Some(&bob)).await;
    assert_eq!((&listed[0]["id"], &listed[0]["role"]), (&session["id"], &json!("editor")));
    assert_eq!(t.delete(&format!("/sessions/{id}/shares/{editor_share}"), Some(&owner)).await.0, 204);
    assert_eq!(t.delete(&format!("/sessions/{id}/shares/{editor_share}"), Some(&owner)).await.0, 404);
    assert_eq!(t.post(&format!("/shares/{editor_share}/join"), Some(&bob), json!({})).await.0, 404);
    assert_eq!(t.post(&format!("/shares/{viewer_share}/join"), None, json!({})).await.0, 401);
    assert_eq!(t.get(&format!("/sessions/{id}/kit"), Some(&editor_share)).await.0, 401);
    let (status, kit) = t.get(&format!("/sessions/{id}/kit"), Some(&viewer_share)).await;
    assert_eq!((status, &kit["hash"]), (200, &session["hash"]));
}

} // 🔗Share Tests

mod websocket_tests { // 🤖Websocket Tests

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_clients_share_presence_and_operations() {
    let t = start().await;
    let (alice, alice_person) = t.register("alice").await;
    let (bob, bob_person) = t.register("bob").await;
    let (carol, _) = t.register("carol").await;
    let session = t.create_session(&alice, "Live", None).await;
    let id = session["id"].as_str().expect("id").to_string();
    t.join(&bob, &t.share(&alice, &id, "editor").await).await;
    t.join(&carol, &t.share(&alice, &id, "viewer").await).await;
    let mut a = t.socket(&id, &alice, "client-a").await.expect("socket a");
    let welcome_a = until(&mut a, |message| message["type"] == "welcome").await;
    assert_eq!((&welcome_a["self"]["personId"], &welcome_a["self"]["kind"], &welcome_a["self"]["client"]), (&alice_person["id"], &json!("human"), &json!("test")));
    assert_eq!((&welcome_a["version"], &welcome_a["hash"]), (&json!(0), &session["hash"]));
    assert_eq!(welcome_a["participants"].as_array().map(Vec::len), Some(1));
    let a_id = welcome_a["self"]["id"].clone();
    let mut b = t.socket(&id, &bob, "client-b").await.expect("socket b");
    let welcome_b = until(&mut b, |message| message["type"] == "welcome").await;
    assert_eq!(welcome_b["participants"].as_array().map(Vec::len), Some(2));
    let b_id = welcome_b["self"]["id"].clone();
    let joined = until(&mut a, |message| message["type"] == "presence.joined" && message["participant"]["id"] == b_id).await;
    assert_eq!((&joined["participant"]["name"], &joined["participant"]["color"]), (&json!("bob"), &bob_person["color"]));
    assert_eq!(t.get(&format!("/sessions/{id}/presence"), Some(&carol)).await.1.as_array().map(Vec::len), Some(2));
    assert_eq!(t.get(&format!("/sessions/{id}"), Some(&alice)).await.1["participantCount"], 2);
    say(&mut b, json!({ "type": "presence", "cursor": { "x": 1.5, "y": 2.0, "space": "design" } })).await;
    let updated = until(&mut a, |message| message["type"] == "presence.updated").await;
    assert_eq!((&updated["participant"]["id"], &updated["participant"]["cursor"]), (&b_id, &json!({ "x": 1.5, "y": 2.0, "space": "design" })));
    say(&mut b, json!({ "type": "presence", "focus": { "app": "sketchpad", "designId": "d" } })).await;
    let merged = until(&mut a, |message| message["type"] == "presence.updated" && message["participant"]["focus"].is_object()).await;
    assert_eq!(merged["participant"]["cursor"]["x"], 1.5, "partial presence updates merge");
    let design = Uuid::now_v7().to_string();
    say(&mut a, json!({ "type": "operation", "operationId": "ws-1", "baseVersion": 0, "query": CREATE_DESIGN, "variables": { "id": design, "name": "Live Design" } })).await;
    for socket in [&mut a, &mut b] {
        let operation = until(socket, |message| message["type"] == "operation").await;
        assert_eq!((&operation["version"], &operation["operationId"], &operation["clientId"]), (&json!(1), &json!("ws-1"), &json!("client-a")));
        assert_eq!((&operation["participantId"], &operation["personId"], &operation["query"]), (&a_id, &alice_person["id"], &json!(CREATE_DESIGN)));
        assert_eq!(operation["variables"], json!({ "id": design, "name": "Live Design" }));
        assert_eq!(operation["hash"].as_str().map(str::len), Some(64));
    }
    let (status, posted) = t.post(&format!("/sessions/{id}/operations"), Some(&bob), json!({ "operationId": "http-2", "clientId": "client-b", "baseVersion": 1, "query": RENAME_KIT, "variables": { "name": "Live Kit" } })).await;
    assert_eq!(status, 200, "{posted}");
    for socket in [&mut a, &mut b] {
        let operation = until(socket, |message| message["type"] == "operation").await;
        assert_eq!((&operation["version"], &operation["participantId"], &operation["hash"]), (&json!(2), &b_id, &posted["hash"]));
    }
    say(&mut b, json!({ "type": "operation", "operationId": "ws-bad", "query": KIT_NAME })).await;
    let error = until(&mut b, |message| message["type"] == "error").await;
    assert_eq!(error["operationId"], "ws-bad");
    let mut c = t.socket(&id, &carol, "client-c").await.expect("socket c");
    until(&mut c, |message| message["type"] == "welcome").await;
    say(&mut c, json!({ "type": "operation", "operationId": "ws-viewer", "query": RENAME_KIT, "variables": { "name": "nope" } })).await;
    assert!(until(&mut c, |message| message["type"] == "error").await["message"].as_str().is_some_and(|message| message.contains("editor")));
    say(&mut b, json!({ "type": "ping" })).await;
    until(&mut b, |message| message["type"] == "pong").await;
    let agent = t.post("/auth/tokens", Some(&alice), json!({ "label": "mcp" })).await.1["token"].as_str().expect("agent token").to_string();
    let agent_caller = t.caller(&agent).await;
    let participant = t.hub.agent_presence(&agent_caller, id.parse().expect("uuid"), "mcp").await.expect("agent presence");
    let joined = until(&mut a, |message| message["type"] == "presence.joined" && message["participant"]["kind"] == "agent").await;
    assert_eq!((&joined["participant"]["id"], &joined["participant"]["client"]), (&json!(participant.id), &json!("mcp")));
    assert_eq!(t.hub.agent_presence(&agent_caller, id.parse().expect("uuid"), "mcp").await.expect("agent presence").id, participant.id);
    b.close(None).await.expect("close b");
    let left = until(&mut a, |message| message["type"] == "presence.left").await;
    assert_eq!(left["participantId"], b_id);
    let present: Vec<Value> = t.get(&format!("/sessions/{id}/presence"), Some(&alice)).await.1.as_array().expect("presence").clone();
    assert!(present.iter().all(|participant| participant["id"] != b_id));
    assert_eq!(present.len(), 3);
    assert_eq!(t.delete(&format!("/sessions/{id}"), Some(&alice)).await.0, 204);
    assert_eq!(until(&mut a, |message| message["type"] == "error").await["message"], "session deleted");
    assert_eq!(until(&mut a, |message| message["type"] == "closed").await["type"], "closed");
}

} // 🤖Websocket Tests

mod mcp_tests { // 🧠Mcp Tests

use super::*;

const MCP_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/stores/metabolism/wip/initialKit");

/// 🏗️ Metabolism kit projection with its type and design files inlined into the typologies.
fn metabolism_full() -> Value {
    let read = |path: String| -> Value { serde_json::from_str(&std::fs::read_to_string(path).expect("fixture")).expect("fixture json") };
    let mut kit = read(format!("{MCP_DIR}/kit.semio.json"));
    let index = read(format!("{MCP_DIR}/index.semio.json"));
    let files: std::collections::HashMap<String, Value> = ["types", "designs"].iter().flat_map(|key| index[*key].as_array().cloned().unwrap_or_default()).map(|entry| (entry["id"].as_str().expect("id").to_string(), read(format!("{MCP_DIR}/{}", entry["file"].as_str().expect("file"))))).collect();
    for typology in kit["typologies"]["items"].as_array_mut().expect("typologies") {
        for key in ["types", "designs"] {
            for item in typology[key]["items"].as_array_mut().into_iter().flatten() {
                if let Some(full) = files.get(item["id"].as_str().expect("id")) {
                    *item = full.clone();
                }
            }
        }
    }
    kit
}

impl TestHub {
    async fn mcp(&self, token: Option<&str>, headers: &[(&str, &str)], body: Value) -> (u16, reqwest::header::HeaderMap, Value) {
        let mut request = self.http.post(format!("{}/mcp", self.base)).header("content-type", "application/json").header("accept", "application/json, text/event-stream").body(body.to_string());
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        for (key, value) in headers {
            request = request.header(*key, *value);
        }
        let response = request.send().await.expect("mcp request");
        let (status, headers) = (response.status().as_u16(), response.headers().clone());
        let text = response.text().await.expect("mcp body");
        (status, headers, if text.is_empty() { Value::Null } else { serde_json::from_str(&text).expect("mcp json") })
    }

    async fn rpc(&self, token: &str, method: &str, params: Value) -> Value {
        let (status, _, body) = self.mcp(Some(token), &[("mcp-protocol-version", "2025-06-18")], json!({ "jsonrpc": "2.0", "id": 7, "method": method, "params": params })).await;
        assert_eq!((status, &body["jsonrpc"], &body["id"]), (200, &json!("2.0"), &json!(7)), "{method}: {body}");
        body
    }

    async fn tool(&self, token: &str, name: &str, arguments: Value) -> Value {
        let body = self.rpc(token, "tools/call", json!({ "name": name, "arguments": arguments })).await;
        assert_eq!(body["result"]["isError"], false, "{name}: {body}");
        let text: Value = serde_json::from_str(body["result"]["content"][0]["text"].as_str().expect("text content")).expect("json text content");
        assert_eq!(text, body["result"]["structuredContent"], "{name}: text and structured content agree");
        body["result"]["structuredContent"].clone()
    }

    async fn tool_error(&self, token: &str, name: &str, arguments: Value) -> String {
        let body = self.rpc(token, "tools/call", json!({ "name": name, "arguments": arguments })).await;
        assert_eq!(body["result"]["isError"], true, "{name}: {body}");
        body["result"]["content"][0]["text"].as_str().expect("error text").to_string()
    }
}

fn position(x: f64, u: f64) -> Value {
    json!({ "plane": { "origin": { "x": x, "y": 0, "z": 0 }, "xAxis": { "x": 1, "y": 0, "z": 0 }, "yAxis": { "x": 0, "y": 1, "z": 0 } }, "center": { "u": u, "v": 0 } })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn protocol_handshake_auth_and_catalogues() {
    let t = start().await;
    let (owner, _) = t.register("owner").await;
    let agent = t.post("/auth/tokens", Some(&owner), json!({ "label": "claude" })).await.1["token"].as_str().expect("agent token").to_string();
    let session = t.create_session(&owner, "Mcp", None).await;
    let id = session["id"].as_str().expect("id").to_string();
    let viewer_share = t.share(&owner, &id, "viewer").await;
    let initialize = json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "test", "version": "1" } } });
    for token in [None, Some("garbage"), Some(viewer_share.as_str())] {
        let (status, headers, body) = t.mcp(token, &[], initialize.clone()).await;
        assert_eq!(status, 401, "{token:?}: {body}");
        assert!(headers.get("www-authenticate").and_then(|value| value.to_str().ok()).is_some_and(|value| value.starts_with("Bearer")), "{token:?}");
    }
    assert_eq!(t.http.get(format!("{}/mcp", t.base)).bearer_auth(&agent).send().await.expect("get").status().as_u16(), 405);
    let (status, headers, body) = t.mcp(Some(&agent), &[], initialize.clone()).await;
    assert_eq!(status, 200, "{body}");
    assert!(headers.get("mcp-session-id").is_some_and(|value| !value.is_empty()));
    assert_eq!((&body["id"], &body["result"]["protocolVersion"], &body["result"]["serverInfo"]["name"]), (&json!(1), &json!("2025-06-18"), &json!("semio-hub")));
    assert!(body["result"]["capabilities"]["tools"].is_object() && body["result"]["capabilities"]["resources"].is_object() && body["result"]["capabilities"]["prompts"].is_object());
    assert!(body["result"]["instructions"].as_str().is_some_and(|text| text.contains("semio://guide")));
    for (requested, negotiated) in [("2025-03-26", "2025-03-26"), ("2024-11-05", "2024-11-05"), ("1999-01-01", "2025-06-18")] {
        let mut message = initialize.clone();
        message["params"]["protocolVersion"] = json!(requested);
        assert_eq!(t.mcp(Some(&agent), &[], message).await.2["result"]["protocolVersion"], negotiated);
    }
    assert_eq!(t.mcp(Some(&agent), &[("mcp-protocol-version", "1999-01-01")], json!({ "jsonrpc": "2.0", "id": 2, "method": "ping" })).await.0, 400);
    let (status, _, body) = t.mcp(Some(&agent), &[], json!({ "jsonrpc": "2.0", "method": "notifications/initialized" })).await;
    assert_eq!((status, body), (202, Value::Null));
    assert_eq!(t.rpc(&agent, "ping", json!({})).await["result"], json!({}));
    let (status, _, batch) = t.mcp(Some(&agent), &[], json!([{ "jsonrpc": "2.0", "id": "a", "method": "ping" }, { "jsonrpc": "2.0", "method": "notifications/initialized" }])).await;
    assert_eq!((status, batch.as_array().map(Vec::len), &batch[0]["id"]), (200, Some(1), &json!("a")));
    let response = t.http.post(format!("{}/mcp", t.base)).bearer_auth(&agent).body("{not json").send().await.expect("parse request");
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(response.json::<Value>().await.expect("parse body")["error"]["code"], -32700);
    assert_eq!(t.rpc(&agent, "unknown/method", json!({})).await["error"]["code"], -32601);
    assert_eq!(t.rpc(&agent, "tools/call", json!({ "name": "no_such_tool", "arguments": {} })).await["error"]["code"], -32602);
    let tools = t.rpc(&agent, "tools/list", json!({})).await["result"]["tools"].as_array().expect("tools").clone();
    let names: Vec<&str> = tools.iter().filter_map(|tool| tool["name"].as_str()).collect();
    for name in ["list_sessions", "create_session", "read_kit", "list_designs", "read_design", "list_types", "create_design", "add_piece", "connect_pieces", "update_piece", "remove_pieces", "remove_connections", "list_participants", "get_history", "run_query", "run_operation"] {
        assert!(names.contains(&name), "missing tool {name}");
    }
    assert!(tools.iter().all(|tool| tool["inputSchema"]["type"] == "object" && tool["description"].as_str().is_some_and(|text| text.len() > 30)), "every tool has an object schema and a helpful description");
    let resources = t.rpc(&agent, "resources/list", json!({})).await["result"]["resources"].as_array().expect("resources").clone();
    assert!(resources.iter().any(|resource| resource["uri"] == "semio://guide"));
    assert!(resources.iter().any(|resource| resource["uri"] == format!("semio://sessions/{id}/kit")));
    let guide = t.rpc(&agent, "resources/read", json!({ "uri": "semio://guide" })).await;
    assert!(guide["result"]["contents"][0]["text"].as_str().is_some_and(|text| text.contains("Connection") && text.contains("connector")));
    let kit = t.rpc(&agent, "resources/read", json!({ "uri": format!("semio://sessions/{id}/kit") })).await;
    assert_eq!(serde_json::from_str::<Value>(kit["result"]["contents"][0]["text"].as_str().expect("kit text")).expect("kit json")["name"], "Mcp");
    assert_eq!(t.rpc(&agent, "resources/read", json!({ "uri": "semio://nothing" })).await["error"]["code"], -32002);
    assert_eq!(t.rpc(&agent, "resources/templates/list", json!({})).await["result"]["resourceTemplates"][0]["uriTemplate"], "semio://sessions/{sessionId}/kit");
    assert_eq!(t.rpc(&agent, "prompts/list", json!({})).await["result"]["prompts"][0]["name"], "design_assistant");
    let prompt = t.rpc(&agent, "prompts/get", json!({ "name": "design_assistant", "arguments": { "sessionId": id, "goal": "Stack two capsules" } })).await;
    assert!(prompt["result"]["messages"][0]["content"]["text"].as_str().is_some_and(|text| text.contains("Stack two capsules") && text.contains(&id)));
    let (other, _) = t.register("other").await;
    assert!(t.tool_error(&other, "read_kit", json!({ "sessionId": id })).await.contains("not a member"));
    assert!(t.tool_error(&agent, "read_kit", json!({})).await.contains("sessionId"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn tools_edit_a_live_metabolism_session_as_an_agent() {
    let t = start().await;
    let (alice, alice_person) = t.register("alice").await;
    let agent = t.post("/auth/tokens", Some(&alice), json!({ "label": "claude" })).await.1["token"].as_str().expect("agent token").to_string();
    let session = t.create_session(&alice, "Metabolism", Some(metabolism_full())).await;
    let id = session["id"].as_str().expect("id").to_string();
    let mut socket = t.socket(&id, &alice, "client-a").await.expect("socket");
    until(&mut socket, |message| message["type"] == "welcome").await;
    let listed = t.tool(&agent, "list_sessions", json!({})).await;
    assert!(listed["sessions"].as_array().expect("sessions").iter().any(|summary| summary["id"] == id.as_str()));
    let overview = t.tool(&agent, "read_kit", json!({ "sessionId": id })).await;
    let joined = until(&mut socket, |message| message["type"] == "presence.joined" && message["participant"]["kind"] == "agent").await;
    let agent_participant = joined["participant"].clone();
    assert_eq!((&agent_participant["client"], &agent_participant["name"], &agent_participant["personId"]), (&json!("mcp"), &json!("claude (alice)"), &alice_person["id"]));
    assert_eq!((&overview["name"], &overview["sessionVersion"]), (&json!("Metabolism"), &json!(0)));
    assert!(overview["designs"].as_array().expect("designs").iter().any(|design| design["name"] == "Nakagin Capsule Tower" && design["pieceCount"] == 180 && design["connectionCount"] == 179), "{}", overview["designs"]);
    let nakagin = overview["designs"].as_array().expect("designs").iter().find(|design| design["name"] == "Nakagin Capsule Tower").expect("nakagin")["id"].clone();
    let page = t.tool(&agent, "read_design", json!({ "sessionId": id, "designId": nakagin, "limit": 5 })).await;
    assert_eq!((page["pieces"].as_array().map(Vec::len), page["connections"].as_array().map(Vec::len), &page["pieceCount"]), (Some(5), Some(5), &json!(180)));
    assert_eq!(page["fixedPieceIds"].as_array().map(Vec::len), Some(1));
    assert_eq!(t.tool(&agent, "list_designs", json!({ "sessionId": id })).await["designs"].as_array().map(Vec::len), overview["designs"].as_array().map(Vec::len));
    let types = t.tool(&agent, "list_types", json!({ "sessionId": id, "query": "tambour" })).await["types"].as_array().expect("types").clone();
    let tambour = types.iter().find(|ty| ty["name"] == "First Storey Tambour").expect("tambour type")["id"].as_str().expect("id").to_string();
    let base = t.tool(&agent, "list_types", json!({ "sessionId": id, "query": "base" })).await["types"].as_array().expect("types").iter().find(|ty| ty["name"] == "Base").expect("base type").clone();
    assert!(base["connectors"].as_array().expect("connectors").iter().any(|connector| connector["name"] == "c0"));
    let base = base["id"].as_str().expect("id").to_string();
    let created = t.tool(&agent, "create_design", json!({ "sessionId": id, "name": "Agent Tower", "description": "Built over MCP", "unit": "m" })).await;
    let design = created["designId"].as_str().expect("design id").to_string();
    assert_eq!(created["version"], 1);
    let broadcast = until(&mut socket, |message| message["type"] == "operation").await;
    assert_eq!((&broadcast["version"], &broadcast["participantId"], &broadcast["personId"], &broadcast["hash"]), (&json!(1), &agent_participant["id"], &alice_person["id"], &created["hash"]));
    assert!(broadcast["query"].as_str().is_some_and(|query| query.contains("createDesign(id: $id")) && broadcast["variables"]["id"] == design.as_str());
    let root = t.tool(&agent, "add_piece", json!({ "sessionId": id, "designId": design, "typeId": base, "name": "root", "position": position(0.0, 0.0) })).await;
    let root_id = root["pieceId"].as_str().expect("piece id").to_string();
    let focused = until(&mut socket, |message| message["type"] == "presence.updated" && message["participant"]["selection"]["pieceIds"][0] == root_id.as_str()).await;
    assert_eq!(focused["participant"]["focus"], json!({ "app": "design", "designId": design }));
    let first = t.tool(&agent, "add_piece", json!({ "sessionId": id, "designId": design, "typeId": tambour, "name": "storey 1", "parent": { "pieceId": root_id, "connector": "c0", "childConnector": "b", "rotation": 90 } })).await;
    let first_id = first["pieceId"].as_str().expect("piece id").to_string();
    assert!(first["connectionId"].is_string());
    let second = t.tool(&agent, "add_piece", json!({ "sessionId": id, "designId": design, "typeId": tambour, "name": "storey 2", "parent": { "pieceId": first_id, "connector": "t", "childConnector": "b", "gap": 0.25 } })).await;
    let second_id = second["pieceId"].as_str().expect("piece id").to_string();
    let loose = t.tool(&agent, "add_piece", json!({ "sessionId": id, "designId": design, "typeId": tambour, "name": "loose", "position": position(10.0, 3.0) })).await;
    let loose_id = loose["pieceId"].as_str().expect("piece id").to_string();
    let loop_connection = t.tool(&agent, "connect_pieces", json!({ "sessionId": id, "designId": design, "parentPieceId": root_id, "parentConnector": "c1", "childPieceId": loose_id, "childConnector": "b", "rotation": 180 })).await;
    let loop_id = loop_connection["connectionId"].as_str().expect("connection id").to_string();
    let updated = t.tool(&agent, "update_piece", json!({ "sessionId": id, "designId": design, "pieceId": second_id, "name": "storey two", "description": "renamed by the agent", "position": position(0.0, 2.0) })).await;
    assert_eq!(updated["version"], 7);
    let rejected = t.tool_error(&agent, "add_piece", json!({ "sessionId": id, "designId": design, "typeId": tambour, "parent": { "pieceId": root_id, "connector": "no-such-connector", "childConnector": "b" } })).await;
    assert!(rejected.contains("no-such-connector"), "{rejected}");
    let rejected = t.tool_error(&agent, "update_piece", json!({ "sessionId": id, "designId": design, "pieceId": second_id })).await;
    assert!(rejected.contains("at least one"), "{rejected}");
    let read = t.tool(&agent, "read_design", json!({ "sessionId": id, "designId": design })).await;
    assert_eq!((&read["name"], &read["pieceCount"], &read["connectionCount"]), (&json!("Agent Tower"), &json!(4), &json!(3)));
    let piece = |piece_id: &str| read["pieces"].as_array().expect("pieces").iter().find(|piece| piece["id"] == piece_id).cloned().expect("piece");
    assert_eq!((&piece(&first_id)["pose"], &piece(&first_id)["typeId"]), (&Value::Null, &json!(tambour)));
    assert_eq!((&piece(&second_id)["name"], &piece(&second_id)["description"], &piece(&second_id)["pose"]["center"]["u"]), (&json!("storey two"), &json!("renamed by the agent"), &json!(2.0)));
    let connection = read["connections"].as_array().expect("connections").iter().find(|connection| connection["id"] == loop_id.as_str()).cloned().expect("loop connection");
    assert_eq!((&connection["parent"]["piece"]["id"], &connection["child"]["piece"]["id"], &connection["rotation"]), (&json!(root_id), &json!(loose_id), &json!(180.0)));
    assert!(connection["parent"]["connector"]["id"].as_str().is_some_and(|connector| connector != "c1"), "connector names resolve to connector ids");
    t.tool(&agent, "remove_connections", json!({ "sessionId": id, "designId": design, "connectionIds": [loop_id] })).await;
    let removed = t.tool(&agent, "remove_pieces", json!({ "sessionId": id, "designId": design, "pieceIds": [first_id] })).await;
    assert_eq!(removed["version"], 9);
    let after = t.tool(&agent, "read_design", json!({ "sessionId": id, "designId": design })).await;
    assert_eq!((&after["pieceCount"], &after["connectionCount"]), (&json!(3), &json!(0)), "removing a piece removes its connections: {after}");
    let participants = t.tool(&agent, "list_participants", json!({ "sessionId": id })).await["participants"].as_array().expect("participants").clone();
    assert!(participants.iter().any(|participant| participant["kind"] == "agent" && participant["client"] == "mcp") && participants.iter().any(|participant| participant["kind"] == "human"));
    let history = t.tool(&agent, "get_history", json!({ "sessionId": id, "limit": 3 })).await;
    assert_eq!((&history["total"], history["operations"].as_array().map(Vec::len)), (&json!(9), Some(3)));
    assert!(history["operations"].as_array().expect("operations").iter().all(|record| record["participantKind"] == "agent"));
    let queried = t.tool(&agent, "run_query", json!({ "sessionId": id, "query": "{ session { stores { edges { node { wip { theKit { kit { name } } } } } } } }" })).await;
    assert_eq!(queried.pointer("/data/session/stores/edges/0/node/wip/theKit/kit/name"), Some(&json!("Metabolism")));
    let unsafe_create = "mutation($storeId: ID!, $changeId: ID!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { createDesign(name: \"No id\") { ok } } } } } } }";
    assert!(t.tool_error(&agent, "run_operation", json!({ "sessionId": id, "query": unsafe_create })).await.contains("createDesign"));
    let renamed = t.tool(&agent, "run_operation", json!({ "sessionId": id, "query": "mutation($storeId: ID!, $changeId: ID!, $name: String!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { rename(newName: $name) { ok errors { message } } } } } } } }", "variables": { "name": "Metabolism by Agent" } })).await;
    assert_eq!(renamed["version"], 10);
    let (_, alice_view) = t.get(&format!("/sessions/{id}/kit"), Some(&alice)).await;
    assert_eq!((&alice_view["version"], &alice_view["hash"], &alice_view["kit"]["name"]), (&json!(10), &renamed["hash"], &json!("Metabolism by Agent")));
    let (_, initial) = t.get(&format!("/sessions/{id}/kit/at/0"), Some(&alice)).await;
    let replica = KitStore::open("replica", &initial["kit"]).await.expect("replica");
    let (_, records) = t.get(&format!("/sessions/{id}/operations?after=0"), Some(&alice)).await;
    for record in records.as_array().expect("records") {
        replica.apply(record["query"].as_str().expect("query"), &record["variables"]).await.expect("replay");
        assert_eq!(replica.hash().await.expect("hash"), record["hash"].as_str().expect("hash"), "replica diverged at version {}", record["version"]);
    }
    let viewer = t.register("viewer").await.0;
    t.join(&viewer, &t.share(&alice, &id, "viewer").await).await;
    assert!(t.tool_error(&viewer, "create_design", json!({ "sessionId": id, "name": "nope" })).await.contains("editor"));
    let forked = t.tool(&agent, "create_session", json!({ "name": "Fork", "fromSessionId": id })).await;
    assert_eq!(forked["session"]["hash"], renamed["hash"], "a fork starts from the same kit content");
}

} // 🧠Mcp Tests

} // 📐Tests
