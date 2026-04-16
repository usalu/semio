mod header { // 🧲Header
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Specs: Single-binary session-backend consolidating domain, command, event, state, error, schema, persistence, actor, directory, API, and WS modules.
// Summary: Consolidated session-backend service for semio. PostgreSQL-backed, single-writer actor per session, HTTP+WS API with axum, in-memory state with typed entity structs, property-clock conflict resolution.
} // 🧲Header



pub use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
pub use axum::extract::{Path, State};
pub use axum::http::StatusCode;
pub use axum::response::{IntoResponse, Response};
pub use axum::routing::{get, post, put};
pub use axum::{Json, Router};
pub use dashmap::DashMap;
pub use futures::{SinkExt, StreamExt};
pub use serde::{Deserialize, Serialize};
pub use sqlx_core::row::Row;
pub use sqlx_postgres::{PgPool, PgPoolOptions};
pub use std::collections::BTreeMap;
pub use std::sync::Arc;
pub use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
pub use std::time::Instant;
pub use time::OffsetDateTime;
#[cfg(test)]
pub use testcontainers::runners::AsyncRunner;
#[cfg(test)]
pub use testcontainers_modules::postgres::Postgres;
pub use thiserror::Error;
pub use tokio::sync::{broadcast, mpsc, oneshot};
pub use uuid::Uuid;


mod domain { // 🗿Domain
// Specs: Newtype IDs wrap Uuid for session-scoped identity. FieldPatch distinguishes no-change/set/clear. PropertyKey enumerates all mutable properties. ConflictPolicy defines per-property merge behaviour.
// Summary: Session domain newtypes, FieldPatch, PropertyKey, ConflictPolicy, EntityKind, Lifecycle, SessionStatus.




use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PersonId(pub Uuid);

pub type DomainVersion = i64;
pub type SemioVersion = i64;

mod field_patch { // 📭FieldPatch


use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "value")]
pub enum FieldPatch<T> {
    NoChange,
    Set(T),
    Clear,
}

impl<T> Default for FieldPatch<T> {
    fn default() -> Self {
        Self::NoChange
    }
}

impl<T> FieldPatch<T> {
    pub fn is_change(&self) -> bool {
        !matches!(self, Self::NoChange)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "value")]
pub enum RequiredFieldPatch<T> {
    NoChange,
    Set(T),
}

impl<T> Default for RequiredFieldPatch<T> {
    fn default() -> Self {
        Self::NoChange
    }
}

impl<T> RequiredFieldPatch<T> {
    pub fn is_change(&self) -> bool {
        !matches!(self, Self::NoChange)
    }
}

} // 📭FieldPatch
pub use field_patch::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Kit, Author, Location, Folder, File, Tag, Concept, Port, Quality,
    Benchmark, Type, Representation, Connector, Prop, Attribute, Design, Layer,
    Piece, Group, Connection, Stat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lifecycle {
    Active,
    Tombstoned { at: DomainVersion, by: CommandId },
}

impl Lifecycle {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictPolicy {
    RejectIfChanged,
    LastWriterWins,
    ReferenceMustExistAndBeActive,
    SemioLastWriterWins,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropertyKey {
    KitName, KitVersion, KitDescription, KitIcon, KitImage, KitPreview,
    KitRemote, KitHomepage, KitLicense,
    TypeName, TypeParent, TypeDescription, TypeIcon, TypeImage, TypeFolder,
    TypeUnit, TypeStock, TypeIsAbstract, TypeVirtual, TypeLocation,
    DesignName, DesignParent, DesignDescription, DesignIcon, DesignImage,
    DesignFolder, DesignUnit, DesignIsAbstract, DesignCanScale, DesignCanMirror,
    DesignActiveLayer, DesignLocation,
    PieceName, PieceType, PieceDesign, PiecePlane, PieceCenter, PieceScale,
    PieceMirrorPlane, PieceIsHidden, PieceIsLocked, PieceColor, PieceDescription,
    ConnectionConnected, ConnectionConnecting, ConnectionGap, ConnectionShift,
    ConnectionRise, ConnectionRotation, ConnectionTurn, ConnectionTilt,
    ConnectionU, ConnectionV, ConnectionDescription,
    AuthorName, AuthorEmail, FolderName, FolderParent, FolderDescription,
    FileName, FileRemote, FileFolder, FileBlob,
    TagName, TagDescription, TagIcon,
    ConceptName, ConceptDescription, ConceptIcon,
    PortName, PortDescription, PortIcon,
    QualityKey, QualityName, QualityDescription,
    LayerPath, LayerIsHidden, LayerIsLocked, LayerColor, LayerDescription,
    GroupName, GroupColor, GroupDescription,
    EntityLifecycle,
}

pub fn conflict_policy(key: PropertyKey) -> ConflictPolicy {
    match key {
        PropertyKey::KitName => ConflictPolicy::RejectIfChanged,
        PropertyKey::PieceType | PropertyKey::PieceDesign => {
            ConflictPolicy::ReferenceMustExistAndBeActive
        }
        PropertyKey::TypeParent | PropertyKey::DesignParent
        | PropertyKey::FolderParent | PropertyKey::DesignActiveLayer
        | PropertyKey::TypeLocation | PropertyKey::DesignLocation
        | PropertyKey::FileFolder => ConflictPolicy::ReferenceMustExistAndBeActive,
        _ => ConflictPolicy::LastWriterWins,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Passivated,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
    Owner,
    Viewer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShareTokenId(pub Uuid);

} // 🗿Domain
pub use domain::*;

mod lookback { // 🎏Lookback
// Specs: Named lookback points define retention boundaries for kit history. Each token maps to seconds.
// Summary: Configurable lookback points for historical kit snapshot retention and auto-compaction.


use super::*;
pub const LOOKBACK_POINTS: &[(&str, i64)] = &[
    ("1min", 60),
    ("5min", 300),
    ("10min", 600),
    ("30min", 1800),
    ("1h", 3600),
    ("5h", 18000),
    ("1d", 86400),
    ("3d", 259200),
    ("7d", 604800),
    ("1mo", 2592000),
    ("6mo", 15552000),
    ("1y", 31536000),
];

pub fn lookback_seconds(token: &str) -> Option<i64> {
    LOOKBACK_POINTS.iter().find(|(t, _)| *t == token).map(|(_, s)| *s)
}

pub fn lookback_tokens() -> Vec<&'static str> {
    LOOKBACK_POINTS.iter().map(|(t, _)| *t).collect()
}

} // 🎏Lookback
pub use lookback::*;

mod command { // 🪆Command
// Specs: CommandEnvelope carries per-command metadata. DomainCommand enumerates all CRUD variants. SemioCommand handles presence mutations. CommandResult reports outcome.
// Summary: Explicit command types for domain and semio mutations.


use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_id: CommandId,
    pub client_id: ClientId,
    pub request_id: RequestId,
    pub actor_person_id: PersonId,
    pub base_domain_version: DomainVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
pub enum DomainCommand {
    PatchKit(PatchKit),
    CreateType(CreateEntity), PatchType(PatchEntity), DeleteType(DeleteEntity),
    CreateDesign(CreateEntity), PatchDesign(PatchEntity), DeleteDesign(DeleteEntity),
    CreatePiece(CreatePiece), PatchPiece(PatchEntity), DeletePiece(DeleteEntity),
    CreateConnection(CreateConnection), PatchConnection(PatchEntity), DeleteConnection(DeleteEntity),
    CreateLayer(CreateEntity), PatchLayer(PatchEntity), DeleteLayer(DeleteEntity),
    CreateGroup(CreateEntity), PatchGroup(PatchEntity), DeleteGroup(DeleteEntity),
    CreateAuthor(CreateEntity), PatchAuthor(PatchEntity), DeleteAuthor(DeleteEntity),
    CreateTag(CreateEntity), PatchTag(PatchEntity), DeleteTag(DeleteEntity),
    CreateConcept(CreateEntity), PatchConcept(PatchEntity), DeleteConcept(DeleteEntity),
    CreatePort(CreateEntity), PatchPort(PatchEntity), DeletePort(DeleteEntity),
    CreateQuality(CreateEntity), PatchQuality(PatchEntity), DeleteQuality(DeleteEntity),
    CreateFolder(CreateEntity), PatchFolder(PatchEntity), DeleteFolder(DeleteEntity),
    CreateFile(CreateEntity), PatchFile(PatchEntity), DeleteFile(DeleteEntity),
    Batch(DomainBatch),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBatch {
    pub commands: Vec<DomainCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchKit { pub fields: serde_json::Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntity { pub entity_id: Uuid, pub fields: serde_json::Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEntity { pub entity_id: Uuid, pub fields: serde_json::Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEntity { pub entity_id: Uuid }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePiece { pub piece_id: Uuid, pub design_id: Uuid, pub fields: serde_json::Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnection { pub connection_id: Uuid, pub design_id: Uuid, pub fields: serde_json::Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemioEnvelope {
    pub client_id: ClientId,
    pub person_id: PersonId,
    pub frontend_id: String,
    pub base_semio_version: SemioVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
pub enum SemioCommand {
    UpsertCursor(UpsertCursor),
    UpsertLook(UpsertLook),
    SetSelection(SetSelection),
    ClearPresence(ClearPresence),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertCursor { pub u: f64, pub v: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertLook { pub position: [f64; 3], pub forward: [f64; 3], pub up: [f64; 3] }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSelection { pub piece_ids: Vec<Uuid>, pub design_ids: Vec<Uuid> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearPresence;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum CommandResult {
    Accepted { domain_version: DomainVersion },
    Rejected { conflicts: Vec<ConflictDetail> },
    IdempotentDuplicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetail {
    pub property: PropertyKey,
    pub entity_kind: EntityKind,
    pub entity_id: Uuid,
    pub reason: String,
}

} // 🪆Command
pub use command::*;

mod event { // 🏗️Event
// Specs: SessionEvent enumerates all broadcastable events. EntityChange describes domain mutations. SemioUpdate describes semio state changes.
// Summary: Broadcast event types for domain and semio state changes.


use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum SessionEvent {
    DomainCommandAccepted { command_id: CommandId, domain_version: DomainVersion, changes: Vec<EntityChange> },
    DomainCommandRejected { command_id: CommandId, conflicts: Vec<ConflictDetail> },
    SemioUpdated { semio_version: SemioVersion, person_id: PersonId, frontend_id: String, update: SemioUpdate },
    SessionClosed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum EntityChange {
    Created { entity_kind: EntityKind, entity_id: Uuid, snapshot: serde_json::Value },
    Updated { entity_kind: EntityKind, entity_id: Uuid, changed_fields: serde_json::Value },
    Deleted { entity_kind: EntityKind, entity_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SemioUpdate {
    CursorMoved { u: f64, v: f64 },
    LookChanged { position: [f64; 3], forward: [f64; 3], up: [f64; 3] },
    SelectionChanged { piece_ids: Vec<Uuid>, design_ids: Vec<Uuid> },
    PresenceCleared,
}

} // 🏗️Event
pub use event::*;

mod state { // 🖋️State
// Specs: SessionState holds full typed in-memory state for one session. Entity states mirror canonical DB rows.
// Summary: In-memory session state loaded from and persisted to PostgreSQL.


use super::*;
#[derive(Debug, Clone)]
pub struct SessionState {
    pub session_id: SessionId,
    pub domain_version: DomainVersion,
    pub semio_version: SemioVersion,
    pub status: SessionStatus,
    pub kit: KitState,
    pub authors: BTreeMap<Uuid, AuthorState>,
    pub locations: BTreeMap<Uuid, LocationState>,
    pub folders: BTreeMap<Uuid, FolderState>,
    pub files: BTreeMap<Uuid, FileState>,
    pub tags: BTreeMap<Uuid, TagState>,
    pub concepts: BTreeMap<Uuid, ConceptState>,
    pub ports: BTreeMap<Uuid, PortState>,
    pub qualities: BTreeMap<Uuid, QualityState>,
    pub types: BTreeMap<Uuid, TypeState>,
    pub designs: BTreeMap<Uuid, DesignState>,
    pub semio_people: BTreeMap<(Uuid, String), SemioPersonState>,
}

#[derive(Debug, Clone)]
pub struct KitState {
    pub kit_id: Uuid, pub name: String, pub version: Option<String>,
    pub description: Option<String>, pub icon: Option<String>, pub image: Option<String>,
    pub preview: Option<String>, pub remote: Option<String>, pub homepage: Option<String>,
    pub license: Option<String>, pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct AuthorState { pub author_id: Uuid, pub name: String, pub email: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct LocationState { pub location_id: Uuid, pub name: String, pub description: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct FolderState { pub folder_id: Uuid, pub name: String, pub parent_folder_id: Option<Uuid>, pub description: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct FileState { pub file_id: Uuid, pub name: String, pub remote: Option<String>, pub folder_id: Option<Uuid>, pub size: Option<i64>, pub hash: Option<String>, pub blob: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct TagState { pub tag_id: Uuid, pub name: String, pub description: Option<String>, pub icon: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct ConceptState { pub concept_id: Uuid, pub name: String, pub description: Option<String>, pub icon: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct PortState { pub port_id: Uuid, pub name: String, pub description: Option<String>, pub icon: Option<String>, pub max_children: Option<i32>, pub compatible_port_ids: Vec<Uuid>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct QualityState { pub quality_id: Uuid, pub key: String, pub name: String, pub description: Option<String>, pub icon: Option<String>, pub unit: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct TypeState {
    pub type_id: Uuid, pub name: String, pub parent_type_id: Option<Uuid>,
    pub description: Option<String>, pub icon: Option<String>, pub image: Option<String>,
    pub folder: Option<String>, pub unit: Option<String>, pub stock: Option<i32>,
    pub is_abstract: Option<bool>, pub virtual_type: Option<bool>, pub location_id: Option<Uuid>,
    pub connectors: BTreeMap<Uuid, ConnectorState>, pub representations: BTreeMap<Uuid, RepresentationState>,
    pub props: BTreeMap<Uuid, PropState>, pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct ConnectorState {
    pub connector_id: Uuid, pub name: Option<String>, pub t: f64,
    pub point: [f64; 3], pub direction: [f64; 3],
    pub description: Option<String>, pub port_id: Option<Uuid>,
    pub mandatory: Option<bool>, pub max_children: Option<i32>, pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct RepresentationState { pub representation_id: Uuid, pub file_id: Uuid, pub name: Option<String>, pub description: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct PropState { pub prop_id: Uuid, pub quality_id: Uuid, pub value: String, pub unit: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct DesignState {
    pub design_id: Uuid, pub name: String, pub parent_design_id: Option<Uuid>,
    pub description: Option<String>, pub icon: Option<String>, pub image: Option<String>,
    pub folder: Option<String>, pub unit: Option<String>,
    pub is_abstract: Option<bool>, pub can_scale: Option<bool>, pub can_mirror: Option<bool>,
    pub active_layer_id: Option<Uuid>, pub location_id: Option<Uuid>,
    pub pieces: BTreeMap<Uuid, PieceState>, pub connections: BTreeMap<Uuid, ConnectionState>,
    pub layers: BTreeMap<Uuid, LayerState>, pub groups: BTreeMap<Uuid, GroupState>,
    pub stats: BTreeMap<Uuid, StatState>, pub props: BTreeMap<Uuid, PropState>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct PieceState {
    pub piece_id: Uuid, pub name: Option<String>, pub type_id: Option<Uuid>,
    pub design_ref_id: Option<Uuid>, pub plane: Option<PlaneState>,
    pub center: Option<[f64; 2]>, pub scale: Option<f64>,
    pub mirror_plane: Option<PlaneState>, pub is_hidden: Option<bool>,
    pub is_locked: Option<bool>, pub color: Option<String>,
    pub description: Option<String>, pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct PlaneState { pub origin: [f64; 3], pub x_axis: [f64; 3], pub y_axis: [f64; 3] }

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub connection_id: Uuid,
    pub connected_piece_id: Uuid, pub connected_design_piece_id: Option<Uuid>, pub connected_connector_id: Option<Uuid>,
    pub connecting_piece_id: Uuid, pub connecting_design_piece_id: Option<Uuid>, pub connecting_connector_id: Option<Uuid>,
    pub gap: f64, pub shift: f64, pub rise: f64,
    pub rotation: f64, pub turn: f64, pub tilt: f64,
    pub u: Option<f64>, pub v: Option<f64>, pub description: Option<String>,
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone)]
pub struct LayerState { pub layer_id: Uuid, pub path: String, pub is_hidden: Option<bool>, pub is_locked: Option<bool>, pub color: Option<String>, pub description: Option<String>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct GroupState { pub group_id: Uuid, pub name: Option<String>, pub color: Option<String>, pub description: Option<String>, pub piece_ids: Vec<Uuid>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct StatState { pub stat_id: Uuid, pub quality_id: Uuid, pub unit: Option<String>, pub min: Option<f64>, pub min_excluded: Option<bool>, pub max: Option<f64>, pub max_excluded: Option<bool>, pub lifecycle: Lifecycle }

#[derive(Debug, Clone)]
pub struct SemioPersonState {
    pub person_id: Uuid, pub frontend_id: String,
    pub display_name: Option<String>, pub color: Option<String>,
    pub is_present: bool, pub cursor: Option<[f64; 2]>,
    pub look: Option<LookState>,
    pub selected_piece_ids: Vec<Uuid>, pub selected_design_ids: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct LookState { pub position: [f64; 3], pub forward: [f64; 3], pub up: [f64; 3] }

} // 🖋️State
pub use state::*;

mod error { // 🎼Error
// Specs: SessionError covers all service error cases. ErrorBody serializes error details for HTTP responses.
// Summary: Error types and HTTP response mapping for the session backend.


use super::*;
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session not found: {0}")]
    SessionNotFound(String),
    #[error("entity not found: {kind} {id}")]
    EntityNotFound { kind: String, id: String },
    #[error("conflict on property {property}: {reason}")]
    Conflict { property: String, reason: String },
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Database(#[from] sqlx_core::Error),
    #[error("actor mailbox closed")]
    ActorGone,
    #[error("idempotent duplicate: command {0} already processed")]
    IdempotentDuplicate(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
pub struct ErrorBody { error: String, detail: String }

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let (status, error, detail) = match &self {
            SessionError::SessionNotFound(_) => (StatusCode::NOT_FOUND, "session_not_found", self.to_string()),
            SessionError::EntityNotFound { .. } => (StatusCode::NOT_FOUND, "entity_not_found", self.to_string()),
            SessionError::Conflict { .. } => (StatusCode::CONFLICT, "conflict", self.to_string()),
            SessionError::Validation(_) => (StatusCode::BAD_REQUEST, "validation", self.to_string()),
            SessionError::IdempotentDuplicate(_) => (StatusCode::OK, "idempotent_duplicate", self.to_string()),
            SessionError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "unauthorized", self.to_string()),
            SessionError::Forbidden(_) => (StatusCode::FORBIDDEN, "forbidden", self.to_string()),
            SessionError::ActorGone => (StatusCode::SERVICE_UNAVAILABLE, "actor_gone", self.to_string()),
            SessionError::Database(_) | SessionError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal", self.to_string()),
        };
        let body = ErrorBody { error: error.to_string(), detail };
        (status, axum::Json(body)).into_response()
    }
}

} // 🎼Error
pub use error::*;

mod schema { // 🎞️Schema
// Specs: Migrations create all schemas, enums, and tables on startup. Schema names: runtime, core, history, semio.
// Summary: SQL schema creation and migration for the session backend.


use super::*;
pub async fn run_migrations(pool: &PgPool) {
    create_schemas(pool).await;
    create_enums(pool).await;
    create_runtime_tables(pool).await;
    create_core_tables(pool).await;
    create_semio_tables(pool).await;
    create_history_tables(pool).await;
    tracing::info!("database migrations complete");
}

async fn create_schemas(pool: &PgPool) {
    for schema in &["runtime", "core", "history", "semio"] {
        sqlx_core::query::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema))
            .execute(pool).await.expect("failed to create schema");
    }
}

async fn create_enums(pool: &PgPool) {
    let stmts = [
        "DO $$ BEGIN CREATE TYPE lifecycle_status AS ENUM ('active', 'tombstoned'); EXCEPTION WHEN duplicate_object THEN NULL; END $$",
        "DO $$ BEGIN CREATE TYPE session_status AS ENUM ('active', 'passivated', 'closed'); EXCEPTION WHEN duplicate_object THEN NULL; END $$",
        "DO $$ BEGIN CREATE TYPE command_status AS ENUM ('pending', 'accepted', 'rejected'); EXCEPTION WHEN duplicate_object THEN NULL; END $$",
    ];
    for s in &stmts {
        sqlx_core::query::query(s).execute(pool).await.expect("failed to create enum");
    }
}

async fn create_runtime_tables(pool: &PgPool) {
    sqlx_core::query::query(
        "CREATE TABLE IF NOT EXISTS runtime.session (
            session_id UUID PRIMARY KEY, root_kit_id UUID NOT NULL,
            owner_token UUID NOT NULL,
            domain_version BIGINT NOT NULL DEFAULT 0, semio_version BIGINT NOT NULL DEFAULT 0,
            status session_status NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
        )"
    ).execute(pool).await.expect("runtime.session");

    sqlx_core::query::query(
        "CREATE TABLE IF NOT EXISTS runtime.share_token (
            token UUID PRIMARY KEY,
            session_id UUID NOT NULL REFERENCES runtime.session(session_id),
            access_mode TEXT NOT NULL DEFAULT 'viewer',
            entity_kind TEXT,
            entity_id UUID,
            label TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            expires_at TIMESTAMPTZ
        )"
    ).execute(pool).await.expect("runtime.share_token");

    sqlx_core::query::query(
        "CREATE TABLE IF NOT EXISTS runtime.session_command (
            command_id UUID PRIMARY KEY, session_id UUID NOT NULL REFERENCES runtime.session(session_id),
            client_id UUID NOT NULL, request_id UUID NOT NULL, base_domain_version BIGINT NOT NULL,
            accepted_domain_version BIGINT, command_kind TEXT NOT NULL, actor_person_id UUID NOT NULL,
            received_at TIMESTAMPTZ NOT NULL DEFAULT now(), applied_at TIMESTAMPTZ,
            status command_status NOT NULL DEFAULT 'pending',
            UNIQUE (session_id, client_id, request_id)
        )"
    ).execute(pool).await.expect("runtime.session_command");

    sqlx_core::query::query(
        "CREATE TABLE IF NOT EXISTS runtime.property_clock (
            session_id UUID NOT NULL, entity_kind TEXT NOT NULL, entity_id UUID NOT NULL,
            property_key TEXT NOT NULL, last_changed_domain_version BIGINT NOT NULL,
            last_command_id UUID NOT NULL,
            PRIMARY KEY (session_id, entity_kind, entity_id, property_key)
        )"
    ).execute(pool).await.expect("runtime.property_clock");
}

async fn create_core_tables(pool: &PgPool) {
    create_core_kit(pool).await; create_core_author(pool).await; create_core_location(pool).await;
    create_core_folder(pool).await; create_core_file(pool).await; create_core_tag(pool).await;
    create_core_concept(pool).await; create_core_port(pool).await; create_core_quality(pool).await;
    create_core_type(pool).await; create_core_connector(pool).await; create_core_representation(pool).await;
    create_core_prop(pool).await; create_core_attribute(pool).await; create_core_design(pool).await;
    create_core_layer(pool).await; create_core_piece(pool).await; create_core_group(pool).await;
    create_core_connection(pool).await; create_core_stat(pool).await;
}

async fn exec(pool: &PgPool, sql: &str, name: &str) {
    sqlx_core::query::query(sql).execute(pool).await.expect(name);
}

async fn create_core_kit(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.kit (
        session_id UUID NOT NULL, kit_id UUID NOT NULL, name TEXT NOT NULL, version TEXT,
        description TEXT, icon TEXT, image TEXT, preview TEXT, remote TEXT, homepage TEXT, license TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, kit_id)
    )", "core.kit").await;
}

async fn create_core_author(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.author (
        session_id UUID NOT NULL, author_id UUID NOT NULL, name TEXT NOT NULL, email TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, author_id)
    )", "core.author").await;
}

async fn create_core_location(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.location (
        session_id UUID NOT NULL, location_id UUID NOT NULL, name TEXT NOT NULL, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, location_id)
    )", "core.location").await;
}

async fn create_core_folder(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.folder (
        session_id UUID NOT NULL, folder_id UUID NOT NULL, name TEXT NOT NULL, parent_folder_id UUID,
        description TEXT, lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, folder_id)
    )", "core.folder").await;
}

async fn create_core_file(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.file (
        session_id UUID NOT NULL, file_id UUID NOT NULL, name TEXT NOT NULL, remote TEXT, folder_id UUID,
        size_bytes BIGINT, hash TEXT, blob_ref TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, file_id)
    )", "core.file").await;
}

async fn create_core_tag(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.tag (
        session_id UUID NOT NULL, tag_id UUID NOT NULL, name TEXT NOT NULL, description TEXT, icon TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, tag_id)
    )", "core.tag").await;
}

async fn create_core_concept(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.concept (
        session_id UUID NOT NULL, concept_id UUID NOT NULL, name TEXT NOT NULL, description TEXT, icon TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, concept_id)
    )", "core.concept").await;
}

async fn create_core_port(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.port (
        session_id UUID NOT NULL, port_id UUID NOT NULL, name TEXT NOT NULL, description TEXT, icon TEXT, max_children INTEGER,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, port_id)
    )", "core.port").await;
    exec(pool, "CREATE TABLE IF NOT EXISTS core.port_compatibility (
        session_id UUID NOT NULL, port_id UUID NOT NULL, compatible_port_id UUID NOT NULL,
        PRIMARY KEY (session_id, port_id, compatible_port_id)
    )", "core.port_compatibility").await;
}

async fn create_core_quality(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.quality (
        session_id UUID NOT NULL, quality_id UUID NOT NULL, key TEXT NOT NULL, name TEXT NOT NULL,
        description TEXT, icon TEXT, unit TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, quality_id)
    )", "core.quality").await;
}

async fn create_core_type(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.type_entity (
        session_id UUID NOT NULL, type_id UUID NOT NULL, name TEXT NOT NULL, parent_type_id UUID,
        description TEXT, icon TEXT, image TEXT, folder TEXT, unit TEXT, stock INTEGER,
        is_abstract BOOLEAN, virtual_type BOOLEAN, location_id UUID,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, type_id)
    )", "core.type_entity").await;
}

async fn create_core_connector(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.connector (
        session_id UUID NOT NULL, connector_id UUID NOT NULL, type_id UUID NOT NULL, name TEXT,
        t DOUBLE PRECISION NOT NULL DEFAULT 0,
        point_x DOUBLE PRECISION NOT NULL DEFAULT 0, point_y DOUBLE PRECISION NOT NULL DEFAULT 0,
        point_z DOUBLE PRECISION NOT NULL DEFAULT 0,
        direction_x DOUBLE PRECISION NOT NULL DEFAULT 0, direction_y DOUBLE PRECISION NOT NULL DEFAULT 0,
        direction_z DOUBLE PRECISION NOT NULL DEFAULT 1,
        description TEXT, port_id UUID, mandatory BOOLEAN, max_children INTEGER,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, connector_id)
    )", "core.connector").await;
}

async fn create_core_representation(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.representation (
        session_id UUID NOT NULL, representation_id UUID NOT NULL, type_id UUID NOT NULL, file_id UUID NOT NULL,
        name TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, representation_id)
    )", "core.representation").await;
}

async fn create_core_prop(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.prop (
        session_id UUID NOT NULL, prop_id UUID NOT NULL, quality_id UUID NOT NULL, value TEXT NOT NULL,
        unit TEXT, owner_kind TEXT NOT NULL, owner_id UUID NOT NULL,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, prop_id)
    )", "core.prop").await;
}

async fn create_core_attribute(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.attribute (
        session_id UUID NOT NULL, attribute_id UUID NOT NULL, key TEXT NOT NULL, value TEXT,
        definition TEXT, owner_kind TEXT NOT NULL, owner_id UUID NOT NULL,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, attribute_id)
    )", "core.attribute").await;
}

async fn create_core_design(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.design (
        session_id UUID NOT NULL, design_id UUID NOT NULL, name TEXT NOT NULL, parent_design_id UUID,
        description TEXT, icon TEXT, image TEXT, folder TEXT, unit TEXT,
        is_abstract BOOLEAN, can_scale BOOLEAN, can_mirror BOOLEAN,
        active_layer_id UUID, location_id UUID,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, design_id)
    )", "core.design").await;
}

async fn create_core_layer(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.layer (
        session_id UUID NOT NULL, layer_id UUID NOT NULL, design_id UUID NOT NULL, path TEXT NOT NULL,
        is_hidden BOOLEAN, is_locked BOOLEAN, color TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, layer_id)
    )", "core.layer").await;
}

async fn create_core_piece(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.piece (
        session_id UUID NOT NULL, piece_id UUID NOT NULL, design_id UUID NOT NULL, name TEXT,
        type_id UUID, design_ref_id UUID,
        plane_origin_x DOUBLE PRECISION, plane_origin_y DOUBLE PRECISION, plane_origin_z DOUBLE PRECISION,
        plane_x_axis_x DOUBLE PRECISION, plane_x_axis_y DOUBLE PRECISION, plane_x_axis_z DOUBLE PRECISION,
        plane_y_axis_x DOUBLE PRECISION, plane_y_axis_y DOUBLE PRECISION, plane_y_axis_z DOUBLE PRECISION,
        center_u DOUBLE PRECISION, center_v DOUBLE PRECISION, scale DOUBLE PRECISION,
        mirror_origin_x DOUBLE PRECISION, mirror_origin_y DOUBLE PRECISION, mirror_origin_z DOUBLE PRECISION,
        mirror_x_axis_x DOUBLE PRECISION, mirror_x_axis_y DOUBLE PRECISION, mirror_x_axis_z DOUBLE PRECISION,
        mirror_y_axis_x DOUBLE PRECISION, mirror_y_axis_y DOUBLE PRECISION, mirror_y_axis_z DOUBLE PRECISION,
        is_hidden BOOLEAN, is_locked BOOLEAN, color TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, piece_id)
    )", "core.piece").await;
}

async fn create_core_group(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.group_entity (
        session_id UUID NOT NULL, group_id UUID NOT NULL, design_id UUID NOT NULL, name TEXT,
        color TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, group_id)
    )", "core.group_entity").await;
    exec(pool, "CREATE TABLE IF NOT EXISTS core.group_piece (
        session_id UUID NOT NULL, group_id UUID NOT NULL, piece_id UUID NOT NULL,
        ordinal INTEGER NOT NULL DEFAULT 0, PRIMARY KEY (session_id, group_id, piece_id)
    )", "core.group_piece").await;
}

async fn create_core_connection(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.connection (
        session_id UUID NOT NULL, connection_id UUID NOT NULL, design_id UUID NOT NULL,
        connected_piece_id UUID NOT NULL, connected_design_piece_id UUID, connected_connector_id UUID,
        connecting_piece_id UUID NOT NULL, connecting_design_piece_id UUID, connecting_connector_id UUID,
        gap DOUBLE PRECISION NOT NULL DEFAULT 0, shift_val DOUBLE PRECISION NOT NULL DEFAULT 0,
        rise DOUBLE PRECISION NOT NULL DEFAULT 0, rotation DOUBLE PRECISION NOT NULL DEFAULT 0,
        turn DOUBLE PRECISION NOT NULL DEFAULT 0, tilt DOUBLE PRECISION NOT NULL DEFAULT 0,
        u DOUBLE PRECISION, v DOUBLE PRECISION, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, connection_id)
    )", "core.connection").await;
}

async fn create_core_stat(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.stat (
        session_id UUID NOT NULL, stat_id UUID NOT NULL, design_id UUID NOT NULL, quality_id UUID NOT NULL,
        unit TEXT, min DOUBLE PRECISION, min_excluded BOOLEAN, max DOUBLE PRECISION, max_excluded BOOLEAN,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active', PRIMARY KEY (session_id, stat_id)
    )", "core.stat").await;
}

async fn create_semio_tables(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS semio.person (
        session_id UUID NOT NULL, person_id UUID NOT NULL, frontend_id TEXT NOT NULL,
        display_name TEXT, color TEXT, is_present BOOLEAN NOT NULL DEFAULT true,
        last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, person_id, frontend_id)
    )", "semio.person").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.cursor (
        session_id UUID NOT NULL, person_id UUID NOT NULL, frontend_id TEXT NOT NULL,
        u DOUBLE PRECISION NOT NULL, v DOUBLE PRECISION NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, person_id, frontend_id)
    )", "semio.cursor").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.look (
        session_id UUID NOT NULL, person_id UUID NOT NULL, frontend_id TEXT NOT NULL,
        position_x DOUBLE PRECISION NOT NULL, position_y DOUBLE PRECISION NOT NULL, position_z DOUBLE PRECISION NOT NULL,
        forward_x DOUBLE PRECISION NOT NULL, forward_y DOUBLE PRECISION NOT NULL, forward_z DOUBLE PRECISION NOT NULL,
        up_x DOUBLE PRECISION NOT NULL, up_y DOUBLE PRECISION NOT NULL, up_z DOUBLE PRECISION NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, person_id, frontend_id)
    )", "semio.look").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.selection_piece (
        session_id UUID NOT NULL, person_id UUID NOT NULL, frontend_id TEXT NOT NULL, piece_id UUID NOT NULL,
        PRIMARY KEY (session_id, person_id, frontend_id, piece_id)
    )", "semio.selection_piece").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.selection_design (
        session_id UUID NOT NULL, person_id UUID NOT NULL, frontend_id TEXT NOT NULL, design_id UUID NOT NULL,
        PRIMARY KEY (session_id, person_id, frontend_id, design_id)
    )", "semio.selection_design").await;
}

async fn create_history_tables(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS history.domain_commit (
        session_id UUID NOT NULL,
        domain_version BIGINT NOT NULL,
        command_id UUID NOT NULL,
        committed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, domain_version)
    )", "history.domain_commit").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS history.kit_snapshot (
        session_id UUID NOT NULL,
        domain_version BIGINT NOT NULL,
        kit_json JSONB NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, domain_version)
    )", "history.kit_snapshot").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS history.entity_change_log (
        session_id UUID NOT NULL,
        domain_version BIGINT NOT NULL,
        changes_json JSONB NOT NULL,
        PRIMARY KEY (session_id, domain_version)
    )", "history.entity_change_log").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS history.compaction_config (
        session_id UUID NOT NULL,
        lookback_tokens JSONB NOT NULL DEFAULT '[]'::jsonb,
        last_compacted_at TIMESTAMPTZ,
        PRIMARY KEY (session_id)
    )", "history.compaction_config").await;
}

} // 🎞️Schema
pub use schema::*;

mod persistence { // 🔮Persistence
// Specs: Pool creates a connection pool from DATABASE_URL. Session CRUD creates, loads, and updates session metadata.
// Summary: PostgreSQL persistence: pool creation, session CRUD, snapshot loading.


use super::*;
pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new().max_connections(20).connect(database_url).await.expect("failed to connect to PostgreSQL")
}

fn session_kit_id(kit_json: &serde_json::Value) -> Result<Uuid, SessionError> {
    let id = kit_json
        .get("id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| SessionError::Validation("kit snapshot must include string id".into()))?;
    Uuid::parse_str(id)
        .map_err(|err| SessionError::Validation(format!("invalid kit id '{id}': {err}")))
}

fn session_kit_name<'a>(kit_json: &'a serde_json::Value) -> Result<&'a str, SessionError> {
    kit_json
        .get("name")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| SessionError::Validation("kit snapshot must include string name".into()))
}

fn session_kit_string(kit_json: &serde_json::Value, field: &str) -> Option<String> {
    kit_json
        .get(field)
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn initial_session_kit(
    fallback_kit_id: Uuid,
    fallback_kit_name: &str,
    initial_kit: Option<&serde_json::Value>,
) -> Result<(Uuid, String, serde_json::Value), SessionError> {
    match initial_kit {
        Some(kit_json) => Ok((
            session_kit_id(kit_json)?,
            session_kit_name(kit_json)?.to_string(),
            kit_json.clone(),
        )),
        None => Ok((
            fallback_kit_id,
            fallback_kit_name.to_string(),
            serde_json::json!({
                "id": fallback_kit_id,
                "name": fallback_kit_name,
                "types": [],
                "designs": [],
                "authors": [],
                "tags": [],
                "concepts": [],
                "ports": [],
                "qualities": [],
                "folders": [],
                "files": [],
                "createdAt": chrono_now_iso(),
                "updatedAt": chrono_now_iso(),
            }),
        )),
    }
}

pub async fn create_session(
    pool: &PgPool,
    session_id: Uuid,
    fallback_kit_id: Uuid,
    fallback_kit_name: &str,
    initial_kit: Option<&serde_json::Value>,
) -> Result<Uuid, SessionError> {
    let (kit_id, kit_name, initial_kit) =
        initial_session_kit(fallback_kit_id, fallback_kit_name, initial_kit)?;
    let owner_token = Uuid::now_v7();
    let mut tx = pool.begin().await?;
    sqlx_core::query::query("INSERT INTO runtime.session (session_id, root_kit_id, owner_token) VALUES ($1, $2, $3)")
        .bind(session_id).bind(kit_id).bind(owner_token).execute(&mut *tx).await?;
    sqlx_core::query::query(
        "INSERT INTO core.kit (session_id, kit_id, name, version, description, icon, image, preview, remote, homepage, license)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
    )
        .bind(session_id)
        .bind(kit_id)
        .bind(&kit_name)
        .bind(session_kit_string(&initial_kit, "version"))
        .bind(session_kit_string(&initial_kit, "description"))
        .bind(session_kit_string(&initial_kit, "icon"))
        .bind(session_kit_string(&initial_kit, "image"))
        .bind(session_kit_string(&initial_kit, "preview"))
        .bind(session_kit_string(&initial_kit, "remote"))
        .bind(session_kit_string(&initial_kit, "homepage"))
        .bind(session_kit_string(&initial_kit, "license"))
        .execute(&mut *tx).await?;
    sqlx_core::query::query(
        "INSERT INTO history.kit_snapshot (session_id, domain_version, kit_json) VALUES ($1, 0, $2)"
    ).bind(session_id).bind(&initial_kit).execute(&mut *tx).await?;
    // Store initial compaction config with default lookback tokens
    let default_tokens: Vec<&str> = lookback_tokens();
    let tokens_json = serde_json::to_value(&default_tokens).unwrap_or(serde_json::json!([]));
    sqlx_core::query::query(
        "INSERT INTO history.compaction_config (session_id, lookback_tokens) VALUES ($1, $2)"
    ).bind(session_id).bind(&tokens_json).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(owner_token)
}

pub async fn load_session_meta(pool: &PgPool, session_id: Uuid) -> Result<(DomainVersion, SemioVersion), SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (i64, i64)>(
        "SELECT domain_version, semio_version FROM runtime.session WHERE session_id = $1"
    ).bind(session_id).fetch_optional(pool).await?
     .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
    Ok(row)
}

pub async fn bump_domain_version(pool: &PgPool, session_id: Uuid, new_version: DomainVersion) -> Result<(), SessionError> {
    sqlx_core::query::query("UPDATE runtime.session SET domain_version = $2, updated_at = now() WHERE session_id = $1")
        .bind(session_id).bind(new_version).execute(pool).await?;
    Ok(())
}

pub async fn bump_semio_version(pool: &PgPool, session_id: Uuid, new_version: SemioVersion) -> Result<(), SessionError> {
    sqlx_core::query::query("UPDATE runtime.session SET semio_version = $2, updated_at = now() WHERE session_id = $1")
        .bind(session_id).bind(new_version).execute(pool).await?;
    Ok(())
}

pub async fn replace_session_snapshot(
    pool: &PgPool,
    session_id: Uuid,
    kit_json: &serde_json::Value,
) -> Result<(DomainVersion, SemioVersion), SessionError> {
    let kit_id = session_kit_id(kit_json)?;
    let kit_name = session_kit_name(kit_json)?.to_string();
    let (domain_version, semio_version) = load_session_meta(pool, session_id).await?;
    let mut tx = pool.begin().await?;
    sqlx_core::query::query(
        "UPDATE runtime.session SET root_kit_id = $2, updated_at = now() WHERE session_id = $1"
    )
        .bind(session_id)
        .bind(kit_id)
        .execute(&mut *tx)
        .await?;
    sqlx_core::query::query("DELETE FROM core.kit WHERE session_id = $1")
        .bind(session_id)
        .execute(&mut *tx)
        .await?;
    sqlx_core::query::query(
        "INSERT INTO core.kit (session_id, kit_id, name, version, description, icon, image, preview, remote, homepage, license)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
    )
        .bind(session_id)
        .bind(kit_id)
        .bind(&kit_name)
        .bind(session_kit_string(kit_json, "version"))
        .bind(session_kit_string(kit_json, "description"))
        .bind(session_kit_string(kit_json, "icon"))
        .bind(session_kit_string(kit_json, "image"))
        .bind(session_kit_string(kit_json, "preview"))
        .bind(session_kit_string(kit_json, "remote"))
        .bind(session_kit_string(kit_json, "homepage"))
        .bind(session_kit_string(kit_json, "license"))
        .execute(&mut *tx)
        .await?;
    sqlx_core::query::query(
        "INSERT INTO history.kit_snapshot (session_id, domain_version, kit_json) VALUES ($1, $2, $3)
         ON CONFLICT (session_id, domain_version) DO UPDATE SET kit_json = $3, created_at = now()"
    )
        .bind(session_id)
        .bind(domain_version)
        .bind(kit_json)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok((domain_version, semio_version))
}

pub async fn load_session_state(pool: &PgPool, session_id: Uuid) -> Result<SessionState, SessionError> {
    let (domain_version, semio_version) = load_session_meta(pool, session_id).await?;
    let kit = load_kit(pool, session_id).await?;
    let authors = load_authors(pool, session_id).await?;
    let tags = load_tags(pool, session_id).await?;
    let concepts = load_concepts(pool, session_id).await?;
    let ports = load_ports(pool, session_id).await?;
    let qualities = load_qualities(pool, session_id).await?;
    let folders = load_folders(pool, session_id).await?;
    let files = load_files(pool, session_id).await?;
    let types = load_types(pool, session_id).await?;
    let designs = load_designs(pool, session_id).await?;
    Ok(SessionState {
        session_id: SessionId(session_id), domain_version, semio_version,
        status: SessionStatus::Active, kit, authors, locations: BTreeMap::new(),
        folders, files, tags, concepts, ports, qualities, types, designs,
        semio_people: BTreeMap::new(),
    })
}

async fn load_kit(pool: &PgPool, sid: Uuid) -> Result<KitState, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>,
        Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)>(
        "SELECT kit_id, name, version, description, icon, image, preview, remote, homepage, license
         FROM core.kit WHERE session_id = $1 AND lifecycle = 'active' LIMIT 1"
    ).bind(sid).fetch_one(pool).await?;
    Ok(KitState {
        kit_id: row.0, name: row.1, version: row.2, description: row.3, icon: row.4,
        image: row.5, preview: row.6, remote: row.7, homepage: row.8, license: row.9,
        lifecycle: Lifecycle::Active,
    })
}

async fn load_authors(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, AuthorState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>)>(
        "SELECT author_id, name, email FROM core.author WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, AuthorState { author_id: r.0, name: r.1, email: r.2, lifecycle: Lifecycle::Active })).collect())
}

async fn load_tags(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, TagState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT tag_id, name, description, icon FROM core.tag WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, TagState { tag_id: r.0, name: r.1, description: r.2, icon: r.3, lifecycle: Lifecycle::Active })).collect())
}

async fn load_concepts(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, ConceptState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT concept_id, name, description, icon FROM core.concept WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, ConceptState { concept_id: r.0, name: r.1, description: r.2, icon: r.3, lifecycle: Lifecycle::Active })).collect())
}

async fn load_ports(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, PortState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>, Option<i32>)>(
        "SELECT port_id, name, description, icon, max_children FROM core.port WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, PortState { port_id: r.0, name: r.1, description: r.2, icon: r.3, max_children: r.4, compatible_port_ids: vec![], lifecycle: Lifecycle::Active })).collect())
}

async fn load_qualities(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, QualityState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, String, Option<String>, Option<String>, Option<String>)>(
        "SELECT quality_id, key, name, description, icon, unit FROM core.quality WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, QualityState { quality_id: r.0, key: r.1, name: r.2, description: r.3, icon: r.4, unit: r.5, lifecycle: Lifecycle::Active })).collect())
}

async fn load_folders(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, FolderState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<Uuid>, Option<String>)>(
        "SELECT folder_id, name, parent_folder_id, description FROM core.folder WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, FolderState { folder_id: r.0, name: r.1, parent_folder_id: r.2, description: r.3, lifecycle: Lifecycle::Active })).collect())
}

async fn load_files(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, FileState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<Uuid>, Option<i64>, Option<String>, Option<String>)>(
        "SELECT file_id, name, remote, folder_id, size_bytes, hash, blob_ref FROM core.file WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, FileState { file_id: r.0, name: r.1, remote: r.2, folder_id: r.3, size: r.4, hash: r.5, blob: r.6, lifecycle: Lifecycle::Active })).collect())
}

async fn load_types(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, TypeState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<Uuid>, Option<String>,
        Option<String>, Option<String>, Option<String>, Option<String>,
        Option<i32>, Option<bool>, Option<bool>, Option<Uuid>)>(
        "SELECT type_id, name, parent_type_id, description, icon, image, folder, unit, stock, is_abstract, virtual_type, location_id
         FROM core.type_entity WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| (r.0, TypeState {
        type_id: r.0, name: r.1, parent_type_id: r.2, description: r.3, icon: r.4, image: r.5,
        folder: r.6, unit: r.7, stock: r.8, is_abstract: r.9, virtual_type: r.10, location_id: r.11,
        connectors: BTreeMap::new(), representations: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_designs(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, DesignState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<Uuid>, Option<String>,
        Option<String>, Option<String>, Option<String>, Option<String>,
        Option<bool>, Option<bool>, Option<bool>, Option<Uuid>, Option<Uuid>)>(
        "SELECT design_id, name, parent_design_id, description, icon, image, folder, unit, is_abstract, can_scale, can_mirror, active_layer_id, location_id
         FROM core.design WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    let mut designs: BTreeMap<Uuid, DesignState> = rows.into_iter().map(|r| (r.0, DesignState {
        design_id: r.0, name: r.1, parent_design_id: r.2, description: r.3, icon: r.4, image: r.5,
        folder: r.6, unit: r.7, is_abstract: r.8, can_scale: r.9, can_mirror: r.10,
        active_layer_id: r.11, location_id: r.12,
        pieces: BTreeMap::new(), connections: BTreeMap::new(), layers: BTreeMap::new(),
        groups: BTreeMap::new(), stats: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
    })).collect();
    load_pieces_into_designs(pool, sid, &mut designs).await?;
    load_connections_into_designs(pool, sid, &mut designs).await?;
    load_layers_into_designs(pool, sid, &mut designs).await?;
    Ok(designs)
}

async fn load_pieces_into_designs(pool: &PgPool, sid: Uuid, designs: &mut BTreeMap<Uuid, DesignState>) -> Result<(), SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, Uuid, Option<String>, Option<Uuid>, Option<Uuid>,
        Option<f64>, Option<f64>, Option<bool>, Option<bool>, Option<String>, Option<String>)>(
        "SELECT piece_id, design_id, name, type_id, design_ref_id, center_u, center_v, is_hidden, is_locked, color, description
         FROM core.piece WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    for r in rows {
        if let Some(design) = designs.get_mut(&r.1) {
            design.pieces.insert(r.0, PieceState {
                piece_id: r.0, name: r.2, type_id: r.3, design_ref_id: r.4, plane: None,
                center: match (r.5, r.6) { (Some(u), Some(v)) => Some([u, v]), _ => None },
                scale: None, mirror_plane: None, is_hidden: r.7, is_locked: r.8,
                color: r.9, description: r.10, lifecycle: Lifecycle::Active,
            });
        }
    }
    Ok(())
}

async fn load_connections_into_designs(pool: &PgPool, sid: Uuid, designs: &mut BTreeMap<Uuid, DesignState>) -> Result<(), SessionError> {
    let rows = sqlx_core::query::query(
        "SELECT connection_id, design_id, connected_piece_id, connected_design_piece_id, connected_connector_id,
                connecting_piece_id, connecting_design_piece_id, connecting_connector_id,
                gap, shift_val, rise, rotation, turn, tilt, u, v, description
         FROM core.connection WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    for r in rows {
        let connection_id: Uuid = r.get(0);
        let design_id: Uuid = r.get(1);
        if let Some(design) = designs.get_mut(&design_id) {
            design.connections.insert(connection_id, ConnectionState {
                connection_id,
                connected_piece_id: r.get(2), connected_design_piece_id: r.get(3), connected_connector_id: r.get(4),
                connecting_piece_id: r.get(5), connecting_design_piece_id: r.get(6), connecting_connector_id: r.get(7),
                gap: r.get(8), shift: r.get(9), rise: r.get(10),
                rotation: r.get(11), turn: r.get(12), tilt: r.get(13),
                u: r.get(14), v: r.get(15), description: r.get(16), lifecycle: Lifecycle::Active,
            });
        }
    }
    Ok(())
}

async fn load_layers_into_designs(pool: &PgPool, sid: Uuid, designs: &mut BTreeMap<Uuid, DesignState>) -> Result<(), SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, Uuid, String, Option<bool>, Option<bool>, Option<String>, Option<String>)>(
        "SELECT layer_id, design_id, path, is_hidden, is_locked, color, description
         FROM core.layer WHERE session_id = $1 AND lifecycle = 'active'"
    ).bind(sid).fetch_all(pool).await?;
    for r in rows {
        if let Some(design) = designs.get_mut(&r.1) {
            design.layers.insert(r.0, LayerState { layer_id: r.0, path: r.2, is_hidden: r.3, is_locked: r.4, color: r.5, description: r.6, lifecycle: Lifecycle::Active });
        }
    }
    Ok(())
}

pub async fn check_property_clock(pool: &PgPool, session_id: Uuid, entity_kind: &str, entity_id: Uuid, property_key: &str, base_version: DomainVersion) -> Result<bool, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (i64,)>(
        "SELECT last_changed_domain_version FROM runtime.property_clock WHERE session_id = $1 AND entity_kind = $2 AND entity_id = $3 AND property_key = $4"
    ).bind(session_id).bind(entity_kind).bind(entity_id).bind(property_key).fetch_optional(pool).await?;
    match row { Some((v,)) => Ok(v <= base_version), None => Ok(true) }
}

pub async fn upsert_property_clock(pool: &PgPool, session_id: Uuid, entity_kind: &str, entity_id: Uuid, property_key: &str, domain_version: DomainVersion, command_id: Uuid) -> Result<(), SessionError> {
    sqlx_core::query::query(
        "INSERT INTO runtime.property_clock (session_id, entity_kind, entity_id, property_key, last_changed_domain_version, last_command_id)
         VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (session_id, entity_kind, entity_id, property_key)
         DO UPDATE SET last_changed_domain_version = $5, last_command_id = $6"
    ).bind(session_id).bind(entity_kind).bind(entity_id).bind(property_key).bind(domain_version).bind(command_id).execute(pool).await?;
    Ok(())
}

pub async fn record_command(pool: &PgPool, session_id: Uuid, command_id: Uuid, client_id: Uuid, request_id: Uuid, base_version: DomainVersion, command_kind: &str, actor_person_id: Uuid) -> Result<bool, SessionError> {
    let result = sqlx_core::query::query(
        "INSERT INTO runtime.session_command (command_id, session_id, client_id, request_id, base_domain_version, command_kind, actor_person_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (session_id, client_id, request_id) DO NOTHING"
    ).bind(command_id).bind(session_id).bind(client_id).bind(request_id).bind(base_version).bind(command_kind).bind(actor_person_id).execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn mark_command_accepted(pool: &PgPool, command_id: Uuid, accepted_version: DomainVersion) -> Result<(), SessionError> {
    sqlx_core::query::query("UPDATE runtime.session_command SET status = 'accepted', accepted_domain_version = $2, applied_at = now() WHERE command_id = $1")
        .bind(command_id).bind(accepted_version).execute(pool).await?;
    Ok(())
}

mod auth { // 🔑Auth
// Specs: Auth persistence: load owner token, create/resolve/list/delete share tokens, resolve access mode from bearer token.
// Summary: Auth token persistence for session ownership and share tokens.


use super::*;
pub async fn load_owner_token(pool: &PgPool, session_id: Uuid) -> Result<Uuid, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (Uuid,)>(
        "SELECT owner_token FROM runtime.session WHERE session_id = $1"
    ).bind(session_id).fetch_optional(pool).await?
     .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
    Ok(row.0)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareTokenRow {
    pub token: Uuid,
    pub session_id: Uuid,
    pub access_mode: String,
    pub entity_kind: Option<String>,
    pub entity_id: Option<Uuid>,
    pub label: Option<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}

pub async fn create_share_token(pool: &PgPool, session_id: Uuid, access_mode: AccessMode, entity_kind: Option<&str>, entity_id: Option<Uuid>, label: Option<&str>, expires_at: Option<&str>) -> Result<Uuid, SessionError> {
    let token = Uuid::now_v7();
    let mode_str = match access_mode { AccessMode::Owner => "owner", AccessMode::Viewer => "viewer" };
    sqlx_core::query::query(
        "INSERT INTO runtime.share_token (token, session_id, access_mode, entity_kind, entity_id, label, expires_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7::timestamptz)"
    ).bind(token).bind(session_id).bind(mode_str).bind(entity_kind).bind(entity_id).bind(label).bind(expires_at).execute(pool).await?;
    Ok(token)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedShareToken {
    pub session_id: Uuid,
    pub access_mode: AccessMode,
    pub entity_kind: Option<String>,
    pub entity_id: Option<Uuid>,
    pub label: Option<String>,
}

pub async fn resolve_share_token(pool: &PgPool, token: Uuid) -> Result<ResolvedShareToken, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<Uuid>, Option<String>)>(
        "SELECT session_id, access_mode, entity_kind, entity_id, label FROM runtime.share_token
         WHERE token = $1 AND (expires_at IS NULL OR expires_at > now())"
    ).bind(token).fetch_optional(pool).await?
     .ok_or_else(|| SessionError::EntityNotFound { kind: "share_token".into(), id: token.to_string() })?;
    let access_mode = match row.1.as_str() { "owner" => AccessMode::Owner, _ => AccessMode::Viewer };
    Ok(ResolvedShareToken { session_id: row.0, access_mode, entity_kind: row.2, entity_id: row.3, label: row.4 })
}

pub async fn list_share_tokens(pool: &PgPool, session_id: Uuid) -> Result<Vec<ShareTokenRow>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, Uuid, String, Option<String>, Option<Uuid>, Option<String>)>(
        "SELECT token, session_id, access_mode, entity_kind, entity_id, label FROM runtime.share_token
         WHERE session_id = $1 AND (expires_at IS NULL OR expires_at > now())
         ORDER BY created_at DESC"
    ).bind(session_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| ShareTokenRow {
        token: r.0, session_id: r.1, access_mode: r.2, entity_kind: r.3, entity_id: r.4, label: r.5,
        created_at: String::new(), expires_at: None,
    }).collect())
}

pub async fn delete_share_token(pool: &PgPool, token: Uuid) -> Result<bool, SessionError> {
    let result = sqlx_core::query::query("DELETE FROM runtime.share_token WHERE token = $1")
        .bind(token).execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

/// Resolve access mode from an optional bearer token for a given session.
/// Returns (AccessMode, Option<SessionId>) - the session id from share token if resolved.
pub async fn resolve_access(pool: &PgPool, session_id: Uuid, bearer: Option<&str>) -> Result<AccessMode, SessionError> {
    match bearer {
        Some(token_str) => {
            let token = Uuid::parse_str(token_str)
                .map_err(|_| SessionError::Unauthorized("invalid token format".into()))?;
            // Check if it's the session owner token
            let owner_token = load_owner_token(pool, session_id).await?;
            if token == owner_token {
                return Ok(AccessMode::Owner);
            }
            // Check if it's a share token
            let resolved = resolve_share_token(pool, token).await
                .map_err(|_| SessionError::Unauthorized("invalid or expired token".into()))?;
            if resolved.session_id != session_id {
                return Err(SessionError::Unauthorized("token does not match session".into()));
            }
            Ok(resolved.access_mode)
        }
        None => Ok(AccessMode::Viewer),
    }
}

} // 🔑Auth
pub use auth::*;

mod history { // 🔩History
// Specs: History persistence stores domain commits with timestamps, full kit snapshots at baselines, and
// incremental entity change logs. Supports lookback-based kit reconstruction and auto-compaction.
// Summary: History storage: domain commits, kit snapshots, entity change logs, lookback reconstruction, compaction.


use super::*;
pub async fn record_domain_commit(pool: &PgPool, session_id: Uuid, domain_version: DomainVersion, command_id: Uuid) -> Result<(), SessionError> {
    sqlx_core::query::query(
        "INSERT INTO history.domain_commit (session_id, domain_version, command_id) VALUES ($1, $2, $3)
         ON CONFLICT (session_id, domain_version) DO NOTHING"
    ).bind(session_id).bind(domain_version).bind(command_id).execute(pool).await?;
    Ok(())
}

pub async fn store_kit_snapshot(pool: &PgPool, session_id: Uuid, domain_version: DomainVersion, kit_json: &serde_json::Value) -> Result<(), SessionError> {
    sqlx_core::query::query(
        "INSERT INTO history.kit_snapshot (session_id, domain_version, kit_json) VALUES ($1, $2, $3)
         ON CONFLICT (session_id, domain_version) DO UPDATE SET kit_json = $3, created_at = now()"
    ).bind(session_id).bind(domain_version).bind(kit_json).execute(pool).await?;
    Ok(())
}

pub async fn store_entity_change_log(pool: &PgPool, session_id: Uuid, domain_version: DomainVersion, changes: &[EntityChange]) -> Result<(), SessionError> {
    let changes_json = serde_json::to_value(changes).unwrap_or(serde_json::json!([]));
    sqlx_core::query::query(
        "INSERT INTO history.entity_change_log (session_id, domain_version, changes_json) VALUES ($1, $2, $3)
         ON CONFLICT (session_id, domain_version) DO NOTHING"
    ).bind(session_id).bind(domain_version).bind(&changes_json).execute(pool).await?;
    Ok(())
}

pub async fn get_latest_snapshot_before(pool: &PgPool, session_id: Uuid, target_version: DomainVersion) -> Result<Option<(DomainVersion, serde_json::Value)>, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (i64, serde_json::Value)>(
        "SELECT domain_version, kit_json FROM history.kit_snapshot
         WHERE session_id = $1 AND domain_version <= $2
         ORDER BY domain_version DESC LIMIT 1"
    ).bind(session_id).bind(target_version).fetch_optional(pool).await?;
    Ok(row)
}

pub async fn get_change_logs_in_range(pool: &PgPool, session_id: Uuid, from_version_exclusive: DomainVersion, to_version_inclusive: DomainVersion) -> Result<Vec<(DomainVersion, serde_json::Value)>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (i64, serde_json::Value)>(
        "SELECT domain_version, changes_json FROM history.entity_change_log
         WHERE session_id = $1 AND domain_version > $2 AND domain_version <= $3
         ORDER BY domain_version ASC"
    ).bind(session_id).bind(from_version_exclusive).bind(to_version_inclusive).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_version_at_time(pool: &PgPool, session_id: Uuid, seconds_ago: i64) -> Result<Option<DomainVersion>, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (Option<i64>,)>(
        "SELECT MAX(domain_version) FROM history.domain_commit
         WHERE session_id = $1 AND committed_at <= now() - make_interval(secs => $2::double precision)"
    ).bind(session_id).bind(seconds_ago as f64).fetch_optional(pool).await?;
    Ok(row.and_then(|(v,)| v))
}

pub fn serialize_session_kit(state: &SessionState) -> serde_json::Value {
    let types: Vec<serde_json::Value> = state.types.values().filter(|t| t.lifecycle.is_active()).map(|t| {
        serde_json::json!({
            "id": t.type_id, "name": t.name,
            "description": t.description, "icon": t.icon, "image": t.image,
            "folder": t.folder, "unit": t.unit, "stock": t.stock,
            "isAbstract": t.is_abstract, "virtual": t.virtual_type,
            "parent": t.parent_type_id.map(|id| serde_json::json!({ "id": id })),
            "location": t.location_id.map(|id| serde_json::json!({ "id": id })),
            "connectors": t.connectors.values().filter(|c| c.lifecycle.is_active()).map(|c| serde_json::json!({
                "id": c.connector_id,
                "name": c.name,
                "t": c.t,
                "point": { "x": c.point[0], "y": c.point[1], "z": c.point[2] },
                "direction": { "x": c.direction[0], "y": c.direction[1], "z": c.direction[2] },
                "description": c.description,
                "port": c.port_id.map(|id| serde_json::json!({ "id": id })),
                "mandatory": c.mandatory,
                "maxChildren": c.max_children,
            })).collect::<Vec<_>>(),
            "representations": t.representations.values().filter(|m| m.lifecycle.is_active()).map(|m| serde_json::json!({
                "id": m.representation_id,
                "file": { "id": m.file_id },
                "name": m.name,
                "description": m.description,
            })).collect::<Vec<_>>(),
        })
    }).collect();
    let designs: Vec<serde_json::Value> = state.designs.values().filter(|d| d.lifecycle.is_active()).map(|d| {
        let pieces: Vec<serde_json::Value> = d.pieces.values().filter(|p| p.lifecycle.is_active()).map(|p| {
            serde_json::json!({
                "id": p.piece_id, "name": p.name, "type": p.type_id,
                "center": p.center.map(|c| serde_json::json!({"u": c[0], "v": c[1]})),
                "isHidden": p.is_hidden, "isLocked": p.is_locked,
                "color": p.color, "description": p.description,
                "design": { "id": d.design_id },
            })
        }).collect();
        let connections: Vec<serde_json::Value> = d.connections.values().filter(|c| c.lifecycle.is_active()).map(|c| {
            serde_json::json!({
                "id": c.connection_id,
                "connected": {
                    "piece": { "id": c.connected_piece_id },
                    "designPiece": c.connected_design_piece_id.map(|id| serde_json::json!({ "id": id })),
                    "connector": c.connected_connector_id.map(|id| serde_json::json!({ "id": id })),
                },
                "connecting": {
                    "piece": { "id": c.connecting_piece_id },
                    "designPiece": c.connecting_design_piece_id.map(|id| serde_json::json!({ "id": id })),
                    "connector": c.connecting_connector_id.map(|id| serde_json::json!({ "id": id })),
                },
                "gap": c.gap, "shift": c.shift, "rise": c.rise,
                "rotation": c.rotation, "turn": c.turn, "tilt": c.tilt,
                "u": c.u, "v": c.v, "description": c.description,
            })
        }).collect();
        serde_json::json!({
            "id": d.design_id, "name": d.name,
            "description": d.description, "icon": d.icon, "image": d.image,
            "folder": d.folder, "unit": d.unit, "isAbstract": d.is_abstract,
            "canScale": d.can_scale, "canMirror": d.can_mirror,
            "parent": d.parent_design_id.map(|id| serde_json::json!({ "id": id })),
            "activeLayer": d.active_layer_id.map(|id| serde_json::json!({ "id": id })),
            "location": d.location_id.map(|id| serde_json::json!({ "id": id })),
            "pieces": pieces, "connections": connections,
        })
    }).collect();
    let authors: Vec<serde_json::Value> = state.authors.values().filter(|a| a.lifecycle.is_active()).map(|a| {
        serde_json::json!({"id": a.author_id, "name": a.name, "email": a.email})
    }).collect();
    let tags: Vec<serde_json::Value> = state.tags.values().filter(|t| t.lifecycle.is_active()).map(|t| {
        serde_json::json!({"id": t.tag_id, "name": t.name, "description": t.description, "icon": t.icon})
    }).collect();
    let concepts: Vec<serde_json::Value> = state.concepts.values().filter(|c| c.lifecycle.is_active()).map(|c| {
        serde_json::json!({"id": c.concept_id, "name": c.name, "description": c.description, "icon": c.icon})
    }).collect();
    let ports: Vec<serde_json::Value> = state.ports.values().filter(|p| p.lifecycle.is_active()).map(|p| {
        serde_json::json!({
            "id": p.port_id,
            "name": p.name,
            "description": p.description,
            "icon": p.icon,
            "maxChildren": p.max_children,
            "compatiblePorts": p.compatible_port_ids.iter().map(|id| serde_json::json!({ "id": id })).collect::<Vec<_>>(),
        })
    }).collect();
    let qualities: Vec<serde_json::Value> = state.qualities.values().filter(|q| q.lifecycle.is_active()).map(|q| {
        serde_json::json!({
            "id": q.quality_id,
            "key": q.key,
            "name": q.name,
            "description": q.description,
            "icon": q.icon,
            "unit": q.unit,
        })
    }).collect();
    let folders: Vec<serde_json::Value> = state.folders.values().filter(|f| f.lifecycle.is_active()).map(|f| {
        serde_json::json!({
            "id": f.folder_id,
            "name": f.name,
            "parent": f.parent_folder_id.map(|id| serde_json::json!({ "id": id })),
            "description": f.description,
        })
    }).collect();
    let files: Vec<serde_json::Value> = state.files.values().filter(|f| f.lifecycle.is_active()).map(|f| {
        serde_json::json!({
            "id": f.file_id,
            "name": f.name,
            "remote": f.remote,
            "folder": f.folder_id.map(|id| serde_json::json!({ "id": id })),
            "size": f.size,
            "hash": f.hash,
            "blob": f.blob,
        })
    }).collect();
    serde_json::json!({
        "id": state.kit.kit_id, "name": state.kit.name,
        "version": state.kit.version, "description": state.kit.description,
        "icon": state.kit.icon, "image": state.kit.image,
        "preview": state.kit.preview, "remote": state.kit.remote,
        "homepage": state.kit.homepage, "license": state.kit.license,
        "types": types, "designs": designs, "authors": authors, "tags": tags,
        "concepts": concepts, "ports": ports, "qualities": qualities, "folders": folders, "files": files,
        "createdAt": chrono_now_iso(), "updatedAt": chrono_now_iso(),
    })
}

pub fn chrono_now_iso() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let nanos = now.subsec_nanos();
    format!("{}.{:09}Z", secs, nanos)
}

pub fn apply_change_log_to_kit(kit: &mut serde_json::Value, changes: &serde_json::Value) {
    let changes_arr = match changes.as_array() {
        Some(a) => a,
        None => return,
    };
    for change in changes_arr {
        let op = change.get("op").and_then(|v| v.as_str()).unwrap_or("");
        let entity_kind = change.get("entity_kind").and_then(|v| v.as_str()).unwrap_or("");
        let entity_id = change.get("entity_id").and_then(|v| v.as_str()).unwrap_or("");
        match op {
            "Created" => {
                let snapshot = change.get("snapshot").cloned().unwrap_or(serde_json::json!({}));
                let mut entity = snapshot.clone();
                if !entity.get("id").is_some() {
                    entity["id"] = serde_json::Value::String(entity_id.to_string());
                }
                match entity_kind {
                    "type" => push_to_array(kit, "types", entity),
                    "design" => push_to_array(kit, "designs", entity),
                    "author" => push_to_array(kit, "authors", entity),
                    "tag" => push_to_array(kit, "tags", entity),
                    "piece" => {
                        if let Some(design_id) = change.get("snapshot").and_then(|s| s.get("design_id")).and_then(|v| v.as_str()) {
                            push_to_design_array(kit, design_id, "pieces", entity);
                        }
                    }
                    "connection" => {
                        if let Some(design_id) = change.get("snapshot").and_then(|s| s.get("design_id")).and_then(|v| v.as_str()) {
                            push_to_design_array(kit, design_id, "connections", entity);
                        }
                    }
                    _ => {}
                }
            }
            "Updated" => {
                let changed_fields = change.get("changed_fields").cloned().unwrap_or(serde_json::json!({}));
                match entity_kind {
                    "kit" => {
                        if let Some(obj) = kit.as_object_mut() {
                            if let Some(fields) = changed_fields.as_object() {
                                for (k, v) in fields {
                                    obj.insert(k.clone(), v.clone());
                                }
                            }
                        }
                    }
                    "type" | "design" | "author" | "tag" => {
                        let collection_key = match entity_kind { "type" => "types", "design" => "designs", "author" => "authors", "tag" => "tags", _ => "" };
                        update_in_array(kit, collection_key, entity_id, &changed_fields);
                    }
                    "piece" => {
                        if let Some(design_id) = changed_fields.get("design_id").and_then(|v| v.as_str()) {
                            update_in_design_array(kit, design_id, "pieces", entity_id, &changed_fields);
                        }
                    }
                    "connection" => {
                        if let Some(design_id) = changed_fields.get("design_id").and_then(|v| v.as_str()) {
                            update_in_design_array(kit, design_id, "connections", entity_id, &changed_fields);
                        }
                    }
                    _ => {}
                }
            }
            "Deleted" => {
                match entity_kind {
                    "type" => remove_from_array(kit, "types", entity_id),
                    "design" => remove_from_array(kit, "designs", entity_id),
                    "author" => remove_from_array(kit, "authors", entity_id),
                    "tag" => remove_from_array(kit, "tags", entity_id),
                    "piece" => remove_from_design_arrays(kit, "pieces", entity_id),
                    "connection" => remove_from_design_arrays(kit, "connections", entity_id),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub fn push_to_array(kit: &mut serde_json::Value, key: &str, item: serde_json::Value) {
    if let Some(arr) = kit.get_mut(key).and_then(|v| v.as_array_mut()) {
        arr.push(item);
    } else {
        kit[key] = serde_json::json!([item]);
    }
}

pub fn push_to_design_array(kit: &mut serde_json::Value, design_id: &str, key: &str, item: serde_json::Value) {
    if let Some(designs) = kit.get_mut("designs").and_then(|v| v.as_array_mut()) {
        for d in designs.iter_mut() {
            if d.get("id").and_then(|g| g.as_str()) == Some(design_id) {
                if let Some(arr) = d.get_mut(key).and_then(|v| v.as_array_mut()) {
                    arr.push(item.clone());
                } else {
                    d[key] = serde_json::json!([item]);
                }
                break;
            }
        }
    }
}

pub fn update_in_array(kit: &mut serde_json::Value, key: &str, entity_id: &str, fields: &serde_json::Value) {
    if let Some(arr) = kit.get_mut(key).and_then(|v| v.as_array_mut()) {
        for item in arr.iter_mut() {
            if item.get("id").and_then(|g| g.as_str()) == Some(entity_id) {
                if let Some(obj) = item.as_object_mut() {
                    if let Some(f) = fields.as_object() {
                        for (k, v) in f {
                            obj.insert(k.clone(), v.clone());
                        }
                    }
                }
                break;
            }
        }
    }
}

pub fn update_in_design_array(kit: &mut serde_json::Value, design_id: &str, key: &str, entity_id: &str, fields: &serde_json::Value) {
    if let Some(designs) = kit.get_mut("designs").and_then(|v| v.as_array_mut()) {
        for design in designs.iter_mut() {
            if design.get("id").and_then(|g| g.as_str()) != Some(design_id) {
                continue;
            }
            if let Some(items) = design.get_mut(key).and_then(|v| v.as_array_mut()) {
                for item in items.iter_mut() {
                    if item.get("id").and_then(|g| g.as_str()) == Some(entity_id) {
                        if let Some(obj) = item.as_object_mut() {
                            if let Some(f) = fields.as_object() {
                                for (k, v) in f {
                                    obj.insert(k.clone(), v.clone());
                                }
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
}

pub fn remove_from_array(kit: &mut serde_json::Value, key: &str, entity_id: &str) {
    if let Some(arr) = kit.get_mut(key).and_then(|v| v.as_array_mut()) {
        arr.retain(|item| item.get("id").and_then(|g| g.as_str()) != Some(entity_id));
    }
}

pub fn remove_from_design_arrays(kit: &mut serde_json::Value, key: &str, entity_id: &str) {
    if let Some(designs) = kit.get_mut("designs").and_then(|v| v.as_array_mut()) {
        for design in designs.iter_mut() {
            if let Some(items) = design.get_mut(key).and_then(|v| v.as_array_mut()) {
                items.retain(|item| item.get("id").and_then(|g| g.as_str()) != Some(entity_id));
            }
        }
    }
}

pub async fn reconstruct_kit_at_version(pool: &PgPool, session_id: Uuid, target_version: DomainVersion) -> Result<serde_json::Value, SessionError> {
    let (snap_version, mut kit) = get_latest_snapshot_before(pool, session_id, target_version).await?
        .ok_or_else(|| SessionError::Internal("no baseline snapshot found".to_string()))?;
    if snap_version < target_version {
        let logs = get_change_logs_in_range(pool, session_id, snap_version, target_version).await?;
        for (_version, changes) in &logs {
            apply_change_log_to_kit(&mut kit, changes);
        }
    }
    Ok(kit)
}

pub async fn get_kit_at_lookback(pool: &PgPool, session_id: Uuid, lookback_token: &str) -> Result<serde_json::Value, SessionError> {
    let seconds = lookback_seconds(lookback_token)
        .ok_or_else(|| SessionError::Validation(format!("unknown lookback token: {}", lookback_token)))?;
    let target_version = get_version_at_time(pool, session_id, seconds).await?
        .ok_or_else(|| SessionError::Internal("no version found at lookback time".to_string()))?;
    reconstruct_kit_at_version(pool, session_id, target_version).await
}

pub async fn compact_history(pool: &PgPool, session_id: Uuid, current_state: &SessionState) -> Result<CompactionResult, SessionError> {
    let mut snapshots_created = 0u32;
    let mut logs_deleted = 0u64;
    let current_version = current_state.domain_version;
    // Create snapshot at current version (always keep latest)
    let current_kit = serialize_session_kit(current_state);
    store_kit_snapshot(pool, session_id, current_version, &current_kit).await?;
    snapshots_created += 1;
    // For each lookback boundary, create a snapshot at the boundary version
    for &(token, seconds) in LOOKBACK_POINTS {
        let boundary_version = get_version_at_time(pool, session_id, seconds).await?;
        if let Some(bv) = boundary_version {
            if bv > 0 {
                let existing = get_latest_snapshot_before(pool, session_id, bv).await?;
                match existing {
                    Some((sv, _)) if sv == bv => {} // already have snapshot at exact version
                    _ => {
                        match reconstruct_kit_at_version(pool, session_id, bv).await {
                            Ok(kit) => {
                                store_kit_snapshot(pool, session_id, bv, &kit).await?;
                                snapshots_created += 1;
                            }
                            Err(_) => {
                                tracing::warn!("compaction: could not reconstruct kit at version {} for lookback {}", bv, token);
                            }
                        }
                    }
                }
            }
        }
    }
    // Delete change logs that are fully covered by snapshots
    // Keep logs newer than the oldest lookback boundary
    let oldest_seconds = LOOKBACK_POINTS.last().map(|(_, s)| *s).unwrap_or(31536000);
    let oldest_version = get_version_at_time(pool, session_id, oldest_seconds).await?;
    if let Some(ov) = oldest_version {
        if ov > 0 {
            let result = sqlx_core::query::query(
                "DELETE FROM history.entity_change_log
                 WHERE session_id = $1 AND domain_version < $2
                 AND domain_version IN (
                     SELECT ecl.domain_version FROM history.entity_change_log ecl
                     WHERE ecl.session_id = $1 AND ecl.domain_version < $2
                     AND EXISTS (SELECT 1 FROM history.kit_snapshot ks
                                 WHERE ks.session_id = $1 AND ks.domain_version >= ecl.domain_version)
                 )"
            ).bind(session_id).bind(ov).execute(pool).await?;
            logs_deleted = result.rows_affected();
        }
    }
    Ok(CompactionResult { snapshots_created, logs_deleted })
}

#[derive(Debug, Clone, Serialize)]
pub struct CompactionResult {
    pub snapshots_created: u32,
    pub logs_deleted: u64,
}

} // 🔩History
pub use history::*;

} // 🔮Persistence
pub use persistence::*;

mod actor { // 🎹Actor
// Specs: ActorMessage is the inbox message kind. SessionActor processes commands one at a time in arrival order.
// Summary: Session actor: single-writer task processing commands sequentially.


use super::*;
pub enum ActorMessage {
    DomainCommand { envelope: CommandEnvelope, command: DomainCommand, reply: oneshot::Sender<Result<CommandResult, SessionError>> },
    SemioCommand { envelope: SemioEnvelope, command: SemioCommand, reply: oneshot::Sender<Result<(), SessionError>> },
    GetSnapshot { reply: oneshot::Sender<SessionSnapshot> },
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionSnapshot {
    pub session_id: Uuid,
    pub domain_version: DomainVersion,
    pub semio_version: SemioVersion,
    pub kit: serde_json::Value,
}

pub struct SessionActor {
    state: SessionState,
    pool: PgPool,
    event_tx: broadcast::Sender<SessionEvent>,
}

impl SessionActor {
    pub fn new(state: SessionState, pool: PgPool, event_tx: broadcast::Sender<SessionEvent>) -> Self {
        Self { state, pool, event_tx }
    }

    pub async fn run(&mut self, mut rx: mpsc::Receiver<ActorMessage>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                ActorMessage::DomainCommand { envelope, command, reply } => {
                    let result = self.handle_domain_command(envelope, command).await;
                    let _ = reply.send(result);
                }
                ActorMessage::SemioCommand { envelope, command, reply } => {
                    let result = self.handle_semio_command(envelope, command).await;
                    let _ = reply.send(result);
                }
                ActorMessage::GetSnapshot { reply } => {
                    let _ = reply.send(self.build_snapshot());
                }
            }
        }
    }

    async fn handle_domain_command(&mut self, envelope: CommandEnvelope, command: DomainCommand) -> Result<CommandResult, SessionError> {
        let session_id = self.state.session_id.0;
        let cmd_id = envelope.command_id.0;
        let is_new = record_command(&self.pool, session_id, cmd_id, envelope.client_id.0, envelope.request_id.0,
            envelope.base_domain_version, &format!("{:?}", command), envelope.actor_person_id.0).await?;
        if !is_new { return Ok(CommandResult::IdempotentDuplicate); }
        let new_version = self.state.domain_version + 1;
        let changes = self.apply_domain_command(&command, new_version, cmd_id).await?;
        bump_domain_version(&self.pool, session_id, new_version).await?;
        mark_command_accepted(&self.pool, cmd_id, new_version).await?;
        // Record history: domain commit + entity change log
        record_domain_commit(&self.pool, session_id, new_version, cmd_id).await?;
        store_entity_change_log(&self.pool, session_id, new_version, &changes).await?;
        // Auto-compact every 50 versions
        if new_version % 50 == 0 {
            if let Err(e) = compact_history(&self.pool, session_id, &self.state).await {
                tracing::warn!("compaction failed at version {}: {}", new_version, e);
            }
        }
        self.state.domain_version = new_version;
        let event = SessionEvent::DomainCommandAccepted { command_id: envelope.command_id, domain_version: new_version, changes };
        let _ = self.event_tx.send(event);
        Ok(CommandResult::Accepted { domain_version: new_version })
    }

    async fn apply_domain_command(&mut self, command: &DomainCommand, version: DomainVersion, cmd_id: Uuid) -> Result<Vec<EntityChange>, SessionError> {
        let sid = self.state.session_id.0;
        let mut changes = Vec::new();
        match command {
            DomainCommand::PatchKit(patch) => {
                if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                    self.state.kit.name = name.to_string();
                    sqlx_core::query::query("UPDATE core.kit SET name = $3 WHERE session_id = $1 AND kit_id = $2")
                        .bind(sid).bind(self.state.kit.kit_id).bind(name).execute(&self.pool).await?;
                    upsert_property_clock(&self.pool, sid, "kit", self.state.kit.kit_id, "kit_name", version, cmd_id).await?;
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Kit, entity_id: self.state.kit.kit_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::CreateType(create) => {
                let name = create.fields.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled");
                sqlx_core::query::query("INSERT INTO core.type_entity (session_id, type_id, name) VALUES ($1, $2, $3)")
                    .bind(sid).bind(create.entity_id).bind(name).execute(&self.pool).await?;
                self.state.types.insert(create.entity_id, TypeState {
                    type_id: create.entity_id, name: name.to_string(), parent_type_id: None, description: None,
                    icon: None, image: None, folder: None, unit: None, stock: None, is_abstract: None,
                    virtual_type: None, location_id: None,
                    connectors: BTreeMap::new(), representations: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
                });
                changes.push(EntityChange::Created { entity_kind: EntityKind::Type, entity_id: create.entity_id, snapshot: create.fields.clone() });
            }
            DomainCommand::DeleteType(del) => {
                sqlx_core::query::query("UPDATE core.type_entity SET lifecycle = 'tombstoned' WHERE session_id = $1 AND type_id = $2")
                    .bind(sid).bind(del.entity_id).execute(&self.pool).await?;
                if let Some(t) = self.state.types.get_mut(&del.entity_id) {
                    t.lifecycle = Lifecycle::Tombstoned { at: version, by: CommandId(cmd_id) };
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Type, entity_id: del.entity_id });
            }
            DomainCommand::PatchType(patch) => {
                if let Some(type_state) = self.state.types.get_mut(&patch.entity_id) {
                    if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                        type_state.name = name.to_string();
                        sqlx_core::query::query("UPDATE core.type_entity SET name = $3 WHERE session_id = $1 AND type_id = $2")
                            .bind(sid).bind(patch.entity_id).bind(name).execute(&self.pool).await?;
                    }
                    if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                        type_state.description = Some(description.to_string());
                        sqlx_core::query::query("UPDATE core.type_entity SET description = $3 WHERE session_id = $1 AND type_id = $2")
                            .bind(sid).bind(patch.entity_id).bind(description).execute(&self.pool).await?;
                    }
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Type, entity_id: patch.entity_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::CreateDesign(create) => {
                let name = create.fields.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled");
                sqlx_core::query::query("INSERT INTO core.design (session_id, design_id, name) VALUES ($1, $2, $3)")
                    .bind(sid).bind(create.entity_id).bind(name).execute(&self.pool).await?;
                self.state.designs.insert(create.entity_id, DesignState {
                    design_id: create.entity_id, name: name.to_string(), parent_design_id: None, description: None,
                    icon: None, image: None, folder: None, unit: None, is_abstract: None, can_scale: None, can_mirror: None,
                    active_layer_id: None, location_id: None,
                    pieces: BTreeMap::new(), connections: BTreeMap::new(), layers: BTreeMap::new(),
                    groups: BTreeMap::new(), stats: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
                });
                changes.push(EntityChange::Created { entity_kind: EntityKind::Design, entity_id: create.entity_id, snapshot: create.fields.clone() });
            }
            DomainCommand::DeleteDesign(del) => {
                sqlx_core::query::query("UPDATE core.design SET lifecycle = 'tombstoned' WHERE session_id = $1 AND design_id = $2")
                    .bind(sid).bind(del.entity_id).execute(&self.pool).await?;
                if let Some(d) = self.state.designs.get_mut(&del.entity_id) {
                    d.lifecycle = Lifecycle::Tombstoned { at: version, by: CommandId(cmd_id) };
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Design, entity_id: del.entity_id });
            }
            DomainCommand::PatchDesign(patch) => {
                if let Some(design_state) = self.state.designs.get_mut(&patch.entity_id) {
                    if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                        design_state.name = name.to_string();
                        sqlx_core::query::query("UPDATE core.design SET name = $3 WHERE session_id = $1 AND design_id = $2")
                            .bind(sid).bind(patch.entity_id).bind(name).execute(&self.pool).await?;
                    }
                    if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                        design_state.description = Some(description.to_string());
                        sqlx_core::query::query("UPDATE core.design SET description = $3 WHERE session_id = $1 AND design_id = $2")
                            .bind(sid).bind(patch.entity_id).bind(description).execute(&self.pool).await?;
                    }
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Design, entity_id: patch.entity_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::CreatePiece(create) => {
                let name = create.fields.get("name").and_then(|v| v.as_str());
                sqlx_core::query::query("INSERT INTO core.piece (session_id, piece_id, design_id, name) VALUES ($1, $2, $3, $4)")
                    .bind(sid).bind(create.piece_id).bind(create.design_id).bind(name).execute(&self.pool).await?;
                if let Some(design) = self.state.designs.get_mut(&create.design_id) {
                    design.pieces.insert(create.piece_id, PieceState {
                        piece_id: create.piece_id, name: name.map(|s| s.to_string()),
                        type_id: None, design_ref_id: None, plane: None, center: None, scale: None,
                        mirror_plane: None, is_hidden: None, is_locked: None, color: None, description: None, lifecycle: Lifecycle::Active,
                    });
                }
                changes.push(EntityChange::Created { entity_kind: EntityKind::Piece, entity_id: create.piece_id, snapshot: create.fields.clone() });
            }
            DomainCommand::PatchPiece(patch) => {
                let center_u = patch.fields.get("center").and_then(|center| center.get("u")).and_then(|v| v.as_f64());
                let center_v = patch.fields.get("center").and_then(|center| center.get("v")).and_then(|v| v.as_f64());
                if center_u.is_some() || center_v.is_some() {
                    sqlx_core::query::query("UPDATE core.piece SET center_u = COALESCE($3, center_u), center_v = COALESCE($4, center_v) WHERE session_id = $1 AND piece_id = $2")
                        .bind(sid).bind(patch.entity_id).bind(center_u).bind(center_v).execute(&self.pool).await?;
                }
                if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                    sqlx_core::query::query("UPDATE core.piece SET name = $3 WHERE session_id = $1 AND piece_id = $2")
                        .bind(sid).bind(patch.entity_id).bind(name).execute(&self.pool).await?;
                }
                for design in self.state.designs.values_mut() {
                    if let Some(piece) = design.pieces.get_mut(&patch.entity_id) {
                        if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                            piece.name = Some(name.to_string());
                        }
                        if let (Some(u), Some(v)) = (center_u, center_v) {
                            piece.center = Some([u, v]);
                        }
                        if let Some(is_hidden) = patch.fields.get("isHidden").and_then(|v| v.as_bool()) {
                            piece.is_hidden = Some(is_hidden);
                        }
                        if let Some(is_locked) = patch.fields.get("isLocked").and_then(|v| v.as_bool()) {
                            piece.is_locked = Some(is_locked);
                        }
                        if let Some(color) = patch.fields.get("color").and_then(|v| v.as_str()) {
                            piece.color = Some(color.to_string());
                        }
                        if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                            piece.description = Some(description.to_string());
                        }
                        break;
                    }
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Piece, entity_id: patch.entity_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::DeletePiece(del) => {
                sqlx_core::query::query("UPDATE core.piece SET lifecycle = 'tombstoned' WHERE session_id = $1 AND piece_id = $2")
                    .bind(sid).bind(del.entity_id).execute(&self.pool).await?;
                for design in self.state.designs.values_mut() {
                    if let Some(piece) = design.pieces.get_mut(&del.entity_id) {
                        piece.lifecycle = Lifecycle::Tombstoned { at: version, by: CommandId(cmd_id) };
                        break;
                    }
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Piece, entity_id: del.entity_id });
            }
            DomainCommand::CreateConnection(create) => {
                let connected_piece = create.fields.get("connected_piece_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                let connecting_piece = create.fields.get("connecting_piece_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                sqlx_core::query::query("INSERT INTO core.connection (session_id, connection_id, design_id, connected_piece_id, connecting_piece_id) VALUES ($1, $2, $3, $4, $5)")
                    .bind(sid).bind(create.connection_id).bind(create.design_id).bind(connected_piece).bind(connecting_piece).execute(&self.pool).await?;
                if let Some(design) = self.state.designs.get_mut(&create.design_id) {
                    design.connections.insert(create.connection_id, ConnectionState {
                        connection_id: create.connection_id,
                        connected_piece_id: connected_piece, connected_design_piece_id: None, connected_connector_id: None,
                        connecting_piece_id: connecting_piece, connecting_design_piece_id: None, connecting_connector_id: None,
                        gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0,
                        u: None, v: None, description: None, lifecycle: Lifecycle::Active,
                    });
                }
                changes.push(EntityChange::Created { entity_kind: EntityKind::Connection, entity_id: create.connection_id, snapshot: create.fields.clone() });
            }
            DomainCommand::PatchConnection(patch) => {
                if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                    sqlx_core::query::query("UPDATE core.connection SET description = $3 WHERE session_id = $1 AND connection_id = $2")
                        .bind(sid).bind(patch.entity_id).bind(description).execute(&self.pool).await?;
                }
                if let Some(u) = patch.fields.get("u").and_then(|v| v.as_f64()) {
                    sqlx_core::query::query("UPDATE core.connection SET u = $3 WHERE session_id = $1 AND connection_id = $2")
                        .bind(sid).bind(patch.entity_id).bind(u).execute(&self.pool).await?;
                }
                if let Some(v) = patch.fields.get("v").and_then(|v| v.as_f64()) {
                    sqlx_core::query::query("UPDATE core.connection SET v = $3 WHERE session_id = $1 AND connection_id = $2")
                        .bind(sid).bind(patch.entity_id).bind(v).execute(&self.pool).await?;
                }
                for design in self.state.designs.values_mut() {
                    if let Some(connection) = design.connections.get_mut(&patch.entity_id) {
                        if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                            connection.description = Some(description.to_string());
                        }
                        if let Some(u) = patch.fields.get("u").and_then(|v| v.as_f64()) {
                            connection.u = Some(u);
                        }
                        if let Some(v) = patch.fields.get("v").and_then(|v| v.as_f64()) {
                            connection.v = Some(v);
                        }
                        break;
                    }
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Connection, entity_id: patch.entity_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::DeleteConnection(del) => {
                sqlx_core::query::query("UPDATE core.connection SET lifecycle = 'tombstoned' WHERE session_id = $1 AND connection_id = $2")
                    .bind(sid).bind(del.entity_id).execute(&self.pool).await?;
                for design in self.state.designs.values_mut() {
                    if let Some(connection) = design.connections.get_mut(&del.entity_id) {
                        connection.lifecycle = Lifecycle::Tombstoned { at: version, by: CommandId(cmd_id) };
                        break;
                    }
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Connection, entity_id: del.entity_id });
            }
            DomainCommand::Batch(batch) => {
                for sub in &batch.commands {
                    let sub_changes = Box::pin(self.apply_domain_command(sub, version, cmd_id)).await?;
                    changes.extend(sub_changes);
                }
            }
            _ => { tracing::warn!("unhandled command variant: {:?}", std::mem::discriminant(command)); }
        }
        Ok(changes)
    }

    async fn handle_semio_command(&mut self, envelope: SemioEnvelope, command: SemioCommand) -> Result<(), SessionError> {
        let sid = self.state.session_id.0;
        let pid = envelope.person_id.0;
        let fid = &envelope.frontend_id;
        let new_version = self.state.semio_version + 1;
        let update = match &command {
            SemioCommand::UpsertCursor(c) => {
                sqlx_core::query::query("INSERT INTO semio.cursor (session_id, person_id, frontend_id, u, v) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (session_id, person_id, frontend_id) DO UPDATE SET u = $4, v = $5, updated_at = now()")
                    .bind(sid).bind(pid).bind(fid).bind(c.u).bind(c.v).execute(&self.pool).await?;
                SemioUpdate::CursorMoved { u: c.u, v: c.v }
            }
            SemioCommand::UpsertLook(l) => {
                sqlx_core::query::query("INSERT INTO semio.look (session_id, person_id, frontend_id, position_x, position_y, position_z, forward_x, forward_y, forward_z, up_x, up_y, up_z) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) ON CONFLICT (session_id, person_id, frontend_id) DO UPDATE SET position_x=$4, position_y=$5, position_z=$6, forward_x=$7, forward_y=$8, forward_z=$9, up_x=$10, up_y=$11, up_z=$12, updated_at=now()")
                    .bind(sid).bind(pid).bind(fid).bind(l.position[0]).bind(l.position[1]).bind(l.position[2]).bind(l.forward[0]).bind(l.forward[1]).bind(l.forward[2]).bind(l.up[0]).bind(l.up[1]).bind(l.up[2]).execute(&self.pool).await?;
                SemioUpdate::LookChanged { position: l.position, forward: l.forward, up: l.up }
            }
            SemioCommand::SetSelection(s) => {
                sqlx_core::query::query("DELETE FROM semio.selection_piece WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                for piece_id in &s.piece_ids {
                    sqlx_core::query::query("INSERT INTO semio.selection_piece (session_id,person_id,frontend_id,piece_id) VALUES ($1,$2,$3,$4)")
                        .bind(sid).bind(pid).bind(fid).bind(piece_id).execute(&self.pool).await?;
                }
                SemioUpdate::SelectionChanged { piece_ids: s.piece_ids.clone(), design_ids: s.design_ids.clone() }
            }
            SemioCommand::ClearPresence(_) => {
                sqlx_core::query::query("DELETE FROM semio.cursor WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                sqlx_core::query::query("DELETE FROM semio.look WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                sqlx_core::query::query("DELETE FROM semio.selection_piece WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                SemioUpdate::PresenceCleared
            }
        };
        bump_semio_version(&self.pool, sid, new_version).await?;
        self.state.semio_version = new_version;
        let _ = self.event_tx.send(SessionEvent::SemioUpdated {
            semio_version: new_version, person_id: envelope.person_id,
            frontend_id: envelope.frontend_id.clone(), update,
        });
        Ok(())
    }

    pub fn build_snapshot(&self) -> SessionSnapshot {
        let kit_json = serialize_session_kit(&self.state);
        SessionSnapshot { session_id: self.state.session_id.0, domain_version: self.state.domain_version, semio_version: self.state.semio_version, kit: kit_json }
    }
}

} // 🎹Actor
pub use actor::*;

mod directory { // 🎯Directory
// Specs: SessionHandle holds the sender to an active session actor. SessionDirectory provides get-or-create semantics.
// Summary: Session directory: process-global registry mapping SessionId to actor handles.


use super::*;
#[derive(Clone)]
pub struct SessionHandle {
    pub command_tx: mpsc::Sender<ActorMessage>,
    pub event_tx: broadcast::Sender<SessionEvent>,
    pub active_connections: Arc<AtomicUsize>,
    pub activated_at: Arc<Instant>,
}

//#region 🔖ActiveSessionInfo

#[derive(Debug, Clone, Serialize)]
pub struct ActiveSessionInfo {
    pub session_id: Uuid,
    pub active_connections: usize,
    pub activated_at_secs_ago: u64,
}

//#endregion 🔖ActiveSessionInfo

#[derive(Clone)]
pub struct SessionDirectory {
    sessions: Arc<DashMap<Uuid, SessionHandle>>,
    pool: PgPool,
}

impl SessionDirectory {
    pub fn new(pool: PgPool) -> Self {
        Self { sessions: Arc::new(DashMap::new()), pool }
    }

    pub async fn get_or_activate(&self, session_id: SessionId) -> Option<SessionHandle> {
        // Fast path: session already active
        if let Some(handle) = self.sessions.get(&session_id.0) {
            return Some(handle.clone());
        }
        // Slow path: load from DB and activate
        let state = load_session_state(&self.pool, session_id.0).await.ok()?;
        // Use entry API to avoid TOCTOU race: only insert if still absent
        let handle = {
            let entry = self.sessions.entry(session_id.0);
            match entry {
                dashmap::mapref::entry::Entry::Occupied(o) => o.get().clone(),
                dashmap::mapref::entry::Entry::Vacant(v) => {
                    let (command_tx, command_rx) = mpsc::channel(256);
                    let (event_tx, _) = broadcast::channel(256);
                    let handle = SessionHandle {
                        command_tx,
                        event_tx: event_tx.clone(),
                        active_connections: Arc::new(AtomicUsize::new(0)),
                        activated_at: Arc::new(Instant::now()),
                    };
                    v.insert(handle.clone());
                    let pool = self.pool.clone();
                    let sessions = self.sessions.clone();
                    let sid = session_id.0;
                    tokio::spawn(async move {
                        let mut actor = SessionActor::new(state, pool, event_tx);
                        actor.run(command_rx).await;
                        sessions.remove(&sid);
                        tracing::info!("session actor {} passivated", sid);
                    });
                    handle
                }
            }
        };
        Some(handle)
    }

    pub fn remove(&self, session_id: &Uuid) { self.sessions.remove(session_id); }

    pub fn deactivate(&self, session_id: SessionId) { self.sessions.remove(&session_id.0); }

    //#region 🔖Admin Introspection

    /// Snapshot of all currently-active session actors with WS connection counts.
    pub fn list_active(&self) -> Vec<ActiveSessionInfo> {
        self.sessions.iter()
            .map(|entry| {
                let h = entry.value();
                ActiveSessionInfo {
                    session_id: *entry.key(),
                    active_connections: h.active_connections.load(AtomicOrdering::Relaxed),
                    activated_at_secs_ago: h.activated_at.elapsed().as_secs(),
                }
            })
            .collect()
    }

    /// Number of currently-active session actors.
    pub fn active_session_count(&self) -> usize { self.sessions.len() }

    /// Total WS connections across all active sessions.
    pub fn total_active_connections(&self) -> usize {
        self.sessions.iter()
            .map(|e| e.value().active_connections.load(AtomicOrdering::Relaxed))
            .sum()
    }

    //#endregion 🔖Admin Introspection
}

} // 🎯Directory
pub use directory::*;

mod api { // 🛕Api
// Specs: AppState holds shared resources. Router defines all HTTP endpoints. Auth enforced via Bearer token: owner token for mutations, viewer/no token for reads. Share tokens provide scoped read-only access.
// Summary: HTTP API routes for session management, command submission, auth enforcement, and sharable links.


use super::*;
use axum::http::HeaderMap;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub directory: SessionDirectory,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let directory = SessionDirectory::new(pool.clone());
        Self { pool, directory }
    }
}

fn extract_bearer(headers: &HeaderMap) -> Option<String> {
    headers.get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

pub fn router(state: AppState) -> Router<()> {
    Router::new()
        .route("/health", get(health))
        .route("/sessions", post(handler_create_session))
        .route("/sessions/{session_id}/snapshot", get(handler_get_snapshot).put(handler_put_snapshot))
        .route("/sessions/{session_id}/commands/domain", post(handler_post_domain_command))
        .route("/sessions/{session_id}/commands/semio", post(handler_post_semio_command))
        .route("/sessions/{session_id}/kit/at/{lookback}", get(handler_get_kit_at_lookback))
        .route("/sessions/{session_id}/kit/at-version/{version}", get(handler_get_kit_at_version))
        .route("/sessions/{session_id}/history/compact", post(handler_compact_history))
        .route("/sessions/{session_id}/history/lookback-tokens", get(handler_get_lookback_tokens))
        .route("/sessions/{session_id}/shares", post(handler_create_share))
        .route("/sessions/{session_id}/shares", get(handler_list_shares))
        .route("/sessions/{session_id}/shares/{token}", axum::routing::delete(handler_delete_share))
        .route("/shares/{token}", get(handler_resolve_share))
        .route("/sessions/{session_id}/ws", get(ws_handler))
        .with_state(state)
}

async fn health() -> &'static str { "ok" }

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    kit_name: String,
    kit: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct CreateSessionResponse { session_id: Uuid, kit_id: Uuid, owner_token: Uuid }

async fn handler_create_session(
    State(state): State<AppState>, Json(req): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, SessionError> {
    let session_id = Uuid::now_v7();
    let kit_id = Uuid::now_v7();
    let owner_token = create_session(&state.pool, session_id, kit_id, &req.kit_name, req.kit.as_ref()).await?;
    let response_kit_id = req.kit.as_ref().map(session_kit_id).transpose()?.unwrap_or(kit_id);
    Ok(Json(CreateSessionResponse { session_id, kit_id: response_kit_id, owner_token }))
}

async fn handler_get_snapshot(
    State(state): State<AppState>, Path(session_id): Path<Uuid>,
) -> Result<Json<SessionSnapshot>, SessionError> {
    let (domain_version, semio_version) = load_session_meta(&state.pool, session_id).await?;
    let kit = reconstruct_kit_at_version(&state.pool, session_id, domain_version).await?;
    Ok(Json(SessionSnapshot { session_id, domain_version, semio_version, kit }))
}

#[derive(Deserialize)]
pub struct ReplaceSnapshotRequest {
    kit: serde_json::Value,
}

async fn handler_put_snapshot(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    headers: HeaderMap,
    Json(req): Json<ReplaceSnapshotRequest>,
) -> Result<Json<SessionSnapshot>, SessionError> {
    let bearer = extract_bearer(&headers);
    let access = resolve_access(&state.pool, session_id, bearer.as_deref()).await?;
    if access != AccessMode::Owner {
        return Err(SessionError::Forbidden("write access requires owner token".into()));
    }
    let (domain_version, semio_version) = replace_session_snapshot(&state.pool, session_id, &req.kit).await?;
    state.directory.deactivate(SessionId(session_id));
    Ok(Json(SessionSnapshot { session_id, domain_version, semio_version, kit: req.kit }))
}

#[derive(Deserialize)]
pub struct DomainCommandRequest {
    #[serde(flatten)] envelope: CommandEnvelope,
    #[serde(flatten)] command: DomainCommand,
}

async fn handler_post_domain_command(
    State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap, Json(req): Json<DomainCommandRequest>,
) -> Result<Json<CommandResult>, SessionError> {
    let bearer = extract_bearer(&headers);
    let access = resolve_access(&state.pool, session_id, bearer.as_deref()).await?;
    if access != AccessMode::Owner {
        return Err(SessionError::Forbidden("write access requires owner token".into()));
    }
    let handle = state.directory.get_or_activate(SessionId(session_id)).await
        .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
    let (tx, rx) = oneshot::channel();
    handle.command_tx.send(ActorMessage::DomainCommand { envelope: req.envelope, command: req.command, reply: tx })
        .await.map_err(|_| SessionError::ActorGone)?;
    let result = rx.await.map_err(|_| SessionError::ActorGone)??;
    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct SemioCommandRequest {
    #[serde(flatten)] envelope: SemioEnvelope,
    #[serde(flatten)] command: SemioCommand,
}

async fn handler_post_semio_command(
    State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap, Json(req): Json<SemioCommandRequest>,
) -> Result<Json<serde_json::Value>, SessionError> {
    let bearer = extract_bearer(&headers);
    let access = resolve_access(&state.pool, session_id, bearer.as_deref()).await?;
    if access != AccessMode::Owner {
        return Err(SessionError::Forbidden("write access requires owner token".into()));
    }
    let handle = state.directory.get_or_activate(SessionId(session_id)).await
        .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
    let (tx, rx) = oneshot::channel();
    handle.command_tx.send(ActorMessage::SemioCommand { envelope: req.envelope, command: req.command, reply: tx })
        .await.map_err(|_| SessionError::ActorGone)?;
    rx.await.map_err(|_| SessionError::ActorGone)??;
    Ok(Json(serde_json::json!({"status": "ok"})))
}

async fn handler_get_kit_at_lookback(
    State(state): State<AppState>, Path((session_id, lookback)): Path<(Uuid, String)>,
) -> Result<Json<serde_json::Value>, SessionError> {
    let kit = get_kit_at_lookback(&state.pool, session_id, &lookback).await?;
    Ok(Json(kit))
}

async fn handler_get_kit_at_version(
    State(state): State<AppState>, Path((session_id, version)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, SessionError> {
    let kit = reconstruct_kit_at_version(&state.pool, session_id, version).await?;
    Ok(Json(kit))
}

async fn handler_compact_history(
    State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap,
) -> Result<Json<CompactionResult>, SessionError> {
    let bearer = extract_bearer(&headers);
    let access = resolve_access(&state.pool, session_id, bearer.as_deref()).await?;
    if access != AccessMode::Owner {
        return Err(SessionError::Forbidden("write access requires owner token".into()));
    }
    let handle = state.directory.get_or_activate(SessionId(session_id)).await
        .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
    let (tx, rx) = oneshot::channel();
    handle.command_tx.send(ActorMessage::GetSnapshot { reply: tx }).await.map_err(|_| SessionError::ActorGone)?;
    let _snapshot = rx.await.map_err(|_| SessionError::ActorGone)?;
    // Load full state for compaction
    let session_state = load_session_state(&state.pool, session_id).await?;
    let result = compact_history(&state.pool, session_id, &session_state).await?;
    Ok(Json(result))
}

async fn handler_get_lookback_tokens() -> Json<Vec<&'static str>> {
    Json(lookback_tokens())
}

#[derive(Deserialize)]
pub struct CreateShareRequest {
    pub access_mode: Option<String>,
    pub entity_kind: Option<String>,
    pub entity_id: Option<Uuid>,
    pub label: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Serialize)]
pub struct CreateShareResponse {
    pub token: Uuid,
    pub session_id: Uuid,
    pub access_mode: String,
    pub entity_kind: Option<String>,
    pub entity_id: Option<Uuid>,
    pub label: Option<String>,
}

async fn handler_create_share(
    State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap, Json(req): Json<CreateShareRequest>,
) -> Result<Json<CreateShareResponse>, SessionError> {
    let bearer = extract_bearer(&headers);
    let access = resolve_access(&state.pool, session_id, bearer.as_deref()).await?;
    if access != AccessMode::Owner {
        return Err(SessionError::Forbidden("creating shares requires owner token".into()));
    }
    let mode = match req.access_mode.as_deref() {
        Some("owner") => AccessMode::Owner,
        _ => AccessMode::Viewer,
    };
    let token = create_share_token(
        &state.pool, session_id, mode,
        req.entity_kind.as_deref(), req.entity_id,
        req.label.as_deref(), req.expires_at.as_deref(),
    ).await?;
    let mode_str = match mode { AccessMode::Owner => "owner", AccessMode::Viewer => "viewer" };
    Ok(Json(CreateShareResponse {
        token, session_id, access_mode: mode_str.to_string(),
        entity_kind: req.entity_kind, entity_id: req.entity_id, label: req.label,
    }))
}

async fn handler_list_shares(
    State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap,
) -> Result<Json<Vec<ShareTokenRow>>, SessionError> {
    let bearer = extract_bearer(&headers);
    let access = resolve_access(&state.pool, session_id, bearer.as_deref()).await?;
    if access != AccessMode::Owner {
        return Err(SessionError::Forbidden("listing shares requires owner token".into()));
    }
    let tokens = list_share_tokens(&state.pool, session_id).await?;
    Ok(Json(tokens))
}

async fn handler_delete_share(
    State(state): State<AppState>, Path((session_id, token)): Path<(Uuid, Uuid)>, headers: HeaderMap,
) -> Result<Json<serde_json::Value>, SessionError> {
    let bearer = extract_bearer(&headers);
    let access = resolve_access(&state.pool, session_id, bearer.as_deref()).await?;
    if access != AccessMode::Owner {
        return Err(SessionError::Forbidden("deleting shares requires owner token".into()));
    }
    let deleted = delete_share_token(&state.pool, token).await?;
    Ok(Json(serde_json::json!({"deleted": deleted})))
}

async fn handler_resolve_share(
    State(state): State<AppState>, Path(token): Path<Uuid>,
) -> Result<Json<ResolvedShareToken>, SessionError> {
    let resolved = resolve_share_token(&state.pool, token).await?;
    Ok(Json(resolved))
}

} // 🛕Api
pub use api::*;

mod ws { // 🤖Ws
// Specs: WebSocket handler upgrades HTTP to WS and streams session events.
// Summary: WebSocket handler for real-time session event streaming.


use super::*;
pub async fn ws_handler(
    ws: WebSocketUpgrade, State(state): State<AppState>, Path(session_id): Path<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, session_id))
}

async fn handle_socket(socket: WebSocket, state: AppState, session_id: Uuid) {
    let handle = match state.directory.get_or_activate(SessionId(session_id)).await {
        Some(h) => h,
        None => { tracing::warn!("ws: session {} not found", session_id); return; }
    };
    //#region 🔖Connection Accounting
    // Increment active connection counter for admin visibility. Decrement on scope exit via _guard.
    handle.active_connections.fetch_add(1, AtomicOrdering::Relaxed);
    let conn_counter = handle.active_connections.clone();
    struct Decrement(Arc<AtomicUsize>);
    impl Drop for Decrement {
        fn drop(&mut self) { self.0.fetch_sub(1, AtomicOrdering::Relaxed); }
    }
    let _guard = Decrement(conn_counter);
    //#endregion 🔖Connection Accounting
    let mut event_rx = handle.event_tx.subscribe();
    let (mut ws_tx, mut ws_rx) = socket.split();
    // Send connection acknowledgment so clients know the subscription is active
    let _ = ws_tx.send(Message::Text(r#"{"kind":"connected"}"#.into())).await;
    let send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let json = match serde_json::to_string(&event) { Ok(j) => j, Err(e) => { tracing::error!("ws serialize error: {}", e); continue; } };
            if ws_tx.send(Message::Text(json.into())).await.is_err() { break; }
        }
    });
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            match msg { Message::Close(_) => break, Message::Text(text) => { tracing::debug!("ws received text: {}", text); }, _ => {} }
        }
    });
    tokio::select! { _ = send_task => {}, _ = recv_task => {} }
    tracing::debug!("ws connection closed for session {}", session_id);
}

} // 🤖Ws
pub use ws::*;

mod admin { // 🛡️Admin
// Specs: Server-admin HTTP surface protected by a shared bearer token (SEMIO_ADMIN_TOKEN). Exposes read-only introspection
// (sessions, kits, persons, share tokens, connections, config) and targeted write operations (close/passivate a session,
// revoke a share token, update compaction config). When SEMIO_ADMIN_TOKEN is unset the /admin/* endpoints return 503 so
// an unconfigured deployment never silently exposes itself. A single embedded HTML dashboard aggregates all views for
// human operators; it calls the same JSON endpoints over fetch() with the bearer token supplied at sign-in.
// Summary: Server-admin dashboard, introspection endpoints, and configuration API for semio-hub.


use super::*;
use axum::http::HeaderMap;

//#region 🔖AdminConfig

/// Process-global admin configuration. Populated from environment at startup.
#[derive(Clone)]
pub struct AdminConfig {
    pub admin_token: Option<String>,
    pub started_at: Arc<Instant>,
}

impl AdminConfig {
    pub fn from_env() -> Self {
        let admin_token = std::env::var("SEMIO_ADMIN_TOKEN").ok().filter(|s| !s.is_empty());
        Self { admin_token, started_at: Arc::new(Instant::now()) }
    }
}

//#endregion 🔖AdminConfig

//#region 🔖AdminAuth

/// Validates Bearer token against configured admin token. Returns error if token is unset or wrong.
pub fn require_admin(headers: &HeaderMap, config: &AdminConfig) -> Result<(), SessionError> {
    let expected = match &config.admin_token {
        Some(t) => t,
        None => return Err(SessionError::Forbidden("admin endpoints disabled: SEMIO_ADMIN_TOKEN is not set".into())),
    };
    let provided = headers.get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");
    if provided.is_empty() {
        return Err(SessionError::Unauthorized("admin token required".into()));
    }
    if provided != expected.as_str() {
        return Err(SessionError::Unauthorized("admin token invalid".into()));
    }
    Ok(())
}

//#endregion 🔖AdminAuth

//#region 🔖AdminRows

#[derive(Debug, Serialize)]
pub struct AdminSessionRow {
    pub session_id: Uuid,
    pub root_kit_id: Uuid,
    pub status: String,
    pub domain_version: i64,
    pub semio_version: i64,
    pub created_at: String,
    pub updated_at: String,
    pub active_connections: usize,
    pub is_activated: bool,
}

#[derive(Debug, Serialize)]
pub struct AdminKitRow {
    pub session_id: Uuid,
    pub kit_id: Uuid,
    pub name: String,
    pub version: Option<String>,
    pub remote: Option<String>,
    pub lifecycle: String,
}

#[derive(Debug, Serialize)]
pub struct AdminPersonRow {
    pub session_id: Uuid,
    pub person_id: Uuid,
    pub frontend_id: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub is_present: bool,
    pub last_seen_at: String,
}

#[derive(Debug, Serialize)]
pub struct AdminShareTokenRow {
    pub token: Uuid,
    pub session_id: Uuid,
    pub access_mode: String,
    pub entity_kind: Option<String>,
    pub entity_id: Option<Uuid>,
    pub label: Option<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminOverview {
    pub uptime_secs: u64,
    pub total_sessions: i64,
    pub active_sessions: i64,
    pub passivated_sessions: i64,
    pub closed_sessions: i64,
    pub total_kits: i64,
    pub total_persons: i64,
    pub total_share_tokens: i64,
    pub active_actors: usize,
    pub active_connections: usize,
}

#[derive(Debug, Serialize)]
pub struct AdminSessionDetail {
    pub row: AdminSessionRow,
    pub kit: Option<AdminKitRow>,
    pub persons: Vec<AdminPersonRow>,
    pub share_tokens: Vec<AdminShareTokenRow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminCompactionConfig {
    pub session_id: Uuid,
    pub lookback_tokens: Vec<String>,
    pub last_compacted_at: Option<String>,
}

//#endregion 🔖AdminRows

//#region 🔖AdminQueries

pub async fn load_admin_overview(pool: &PgPool, directory: &SessionDirectory, started_at: Instant) -> Result<AdminOverview, SessionError> {
    let (total, active, passivated, closed): (i64, i64, i64, i64) = sqlx_core::query_as::query_as(
        "SELECT
            COUNT(*)::bigint,
            COUNT(*) FILTER (WHERE status = 'active')::bigint,
            COUNT(*) FILTER (WHERE status = 'passivated')::bigint,
            COUNT(*) FILTER (WHERE status = 'closed')::bigint
         FROM runtime.session"
    ).fetch_one(pool).await?;
    let (total_kits,): (i64,) = sqlx_core::query_as::query_as(
        "SELECT COUNT(*)::bigint FROM core.kit WHERE lifecycle = 'active'"
    ).fetch_one(pool).await?;
    let (total_persons,): (i64,) = sqlx_core::query_as::query_as(
        "SELECT COUNT(*)::bigint FROM semio.person"
    ).fetch_one(pool).await?;
    let (total_share_tokens,): (i64,) = sqlx_core::query_as::query_as(
        "SELECT COUNT(*)::bigint FROM runtime.share_token"
    ).fetch_one(pool).await?;
    Ok(AdminOverview {
        uptime_secs: started_at.elapsed().as_secs(),
        total_sessions: total,
        active_sessions: active,
        passivated_sessions: passivated,
        closed_sessions: closed,
        total_kits,
        total_persons,
        total_share_tokens,
        active_actors: directory.active_session_count(),
        active_connections: directory.total_active_connections(),
    })
}

pub async fn load_admin_sessions(pool: &PgPool, directory: &SessionDirectory) -> Result<Vec<AdminSessionRow>, SessionError> {
    let rows: Vec<(Uuid, Uuid, String, i64, i64, time::OffsetDateTime, time::OffsetDateTime)> = sqlx_core::query_as::query_as(
        "SELECT session_id, root_kit_id, status::text, domain_version, semio_version, created_at, updated_at
         FROM runtime.session ORDER BY created_at DESC"
    ).fetch_all(pool).await?;
    let active = directory.list_active();
    Ok(rows.into_iter().map(|r| {
        let is_activated = active.iter().any(|a| a.session_id == r.0);
        let active_connections = active.iter().find(|a| a.session_id == r.0).map(|a| a.active_connections).unwrap_or(0);
        AdminSessionRow {
            session_id: r.0, root_kit_id: r.1, status: r.2,
            domain_version: r.3, semio_version: r.4,
            created_at: r.5.to_string(), updated_at: r.6.to_string(),
            active_connections, is_activated,
        }
    }).collect())
}

pub async fn load_admin_session_detail(pool: &PgPool, directory: &SessionDirectory, session_id: Uuid) -> Result<AdminSessionDetail, SessionError> {
    let row: Option<(Uuid, Uuid, String, i64, i64, time::OffsetDateTime, time::OffsetDateTime)> = sqlx_core::query_as::query_as(
        "SELECT session_id, root_kit_id, status::text, domain_version, semio_version, created_at, updated_at
         FROM runtime.session WHERE session_id = $1"
    ).bind(session_id).fetch_optional(pool).await?;
    let row = row.ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
    let active = directory.list_active();
    let is_activated = active.iter().any(|a| a.session_id == session_id);
    let active_connections = active.iter().find(|a| a.session_id == session_id).map(|a| a.active_connections).unwrap_or(0);
    let session_row = AdminSessionRow {
        session_id: row.0, root_kit_id: row.1, status: row.2,
        domain_version: row.3, semio_version: row.4,
        created_at: row.5.to_string(), updated_at: row.6.to_string(),
        active_connections, is_activated,
    };
    let kit: Option<(Uuid, String, Option<String>, Option<String>, String)> = sqlx_core::query_as::query_as(
        "SELECT kit_id, name, version, remote, lifecycle::text FROM core.kit WHERE session_id = $1 LIMIT 1"
    ).bind(session_id).fetch_optional(pool).await?;
    let kit = kit.map(|k| AdminKitRow { session_id, kit_id: k.0, name: k.1, version: k.2, remote: k.3, lifecycle: k.4 });
    let person_rows: Vec<(Uuid, String, Option<String>, Option<String>, bool, time::OffsetDateTime)> = sqlx_core::query_as::query_as(
        "SELECT person_id, frontend_id, display_name, color, is_present, last_seen_at
         FROM semio.person WHERE session_id = $1 ORDER BY last_seen_at DESC"
    ).bind(session_id).fetch_all(pool).await?;
    let persons = person_rows.into_iter().map(|p| AdminPersonRow {
        session_id, person_id: p.0, frontend_id: p.1, display_name: p.2, color: p.3, is_present: p.4, last_seen_at: p.5.to_string(),
    }).collect();
    let token_rows: Vec<(Uuid, String, Option<String>, Option<Uuid>, Option<String>, time::OffsetDateTime, Option<time::OffsetDateTime>)> = sqlx_core::query_as::query_as(
        "SELECT token, access_mode, entity_kind, entity_id, label, created_at, expires_at
         FROM runtime.share_token WHERE session_id = $1 ORDER BY created_at DESC"
    ).bind(session_id).fetch_all(pool).await?;
    let share_tokens = token_rows.into_iter().map(|t| AdminShareTokenRow {
        token: t.0, session_id, access_mode: t.1, entity_kind: t.2, entity_id: t.3, label: t.4,
        created_at: t.5.to_string(), expires_at: t.6.map(|d| d.to_string()),
    }).collect();
    Ok(AdminSessionDetail { row: session_row, kit, persons, share_tokens })
}

pub async fn load_admin_kits(pool: &PgPool) -> Result<Vec<AdminKitRow>, SessionError> {
    let rows: Vec<(Uuid, Uuid, String, Option<String>, Option<String>, String)> = sqlx_core::query_as::query_as(
        "SELECT session_id, kit_id, name, version, remote, lifecycle::text FROM core.kit ORDER BY name"
    ).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| AdminKitRow {
        session_id: r.0, kit_id: r.1, name: r.2, version: r.3, remote: r.4, lifecycle: r.5,
    }).collect())
}

pub async fn load_admin_persons(pool: &PgPool) -> Result<Vec<AdminPersonRow>, SessionError> {
    let rows: Vec<(Uuid, Uuid, String, Option<String>, Option<String>, bool, time::OffsetDateTime)> = sqlx_core::query_as::query_as(
        "SELECT session_id, person_id, frontend_id, display_name, color, is_present, last_seen_at
         FROM semio.person ORDER BY last_seen_at DESC"
    ).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| AdminPersonRow {
        session_id: r.0, person_id: r.1, frontend_id: r.2, display_name: r.3, color: r.4, is_present: r.5, last_seen_at: r.6.to_string(),
    }).collect())
}

pub async fn load_admin_share_tokens(pool: &PgPool) -> Result<Vec<AdminShareTokenRow>, SessionError> {
    let rows: Vec<(Uuid, Uuid, String, Option<String>, Option<Uuid>, Option<String>, time::OffsetDateTime, Option<time::OffsetDateTime>)> = sqlx_core::query_as::query_as(
        "SELECT token, session_id, access_mode, entity_kind, entity_id, label, created_at, expires_at
         FROM runtime.share_token ORDER BY created_at DESC"
    ).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|r| AdminShareTokenRow {
        token: r.0, session_id: r.1, access_mode: r.2, entity_kind: r.3, entity_id: r.4, label: r.5,
        created_at: r.6.to_string(), expires_at: r.7.map(|d| d.to_string()),
    }).collect())
}

pub async fn admin_close_session(pool: &PgPool, session_id: Uuid) -> Result<bool, SessionError> {
    let res = sqlx_core::query::query("UPDATE runtime.session SET status = 'closed', updated_at = now() WHERE session_id = $1")
        .bind(session_id).execute(pool).await?;
    Ok(res.rows_affected() > 0)
}

pub async fn admin_load_compaction_config(pool: &PgPool, session_id: Uuid) -> Result<AdminCompactionConfig, SessionError> {
    let row: Option<(serde_json::Value, Option<time::OffsetDateTime>)> = sqlx_core::query_as::query_as(
        "SELECT lookback_tokens, last_compacted_at FROM history.compaction_config WHERE session_id = $1"
    ).bind(session_id).fetch_optional(pool).await?;
    let (tokens, last) = match row {
        Some((json, ts)) => {
            let tokens: Vec<String> = serde_json::from_value(json).unwrap_or_default();
            (tokens, ts.map(|d| d.to_string()))
        }
        None => (lookback_tokens().iter().map(|s| s.to_string()).collect(), None),
    };
    Ok(AdminCompactionConfig { session_id, lookback_tokens: tokens, last_compacted_at: last })
}

pub async fn admin_update_compaction_config(pool: &PgPool, session_id: Uuid, tokens: Vec<String>) -> Result<AdminCompactionConfig, SessionError> {
    let json = serde_json::to_value(&tokens).unwrap_or(serde_json::json!([]));
    sqlx_core::query::query(
        "INSERT INTO history.compaction_config (session_id, lookback_tokens) VALUES ($1, $2)
         ON CONFLICT (session_id) DO UPDATE SET lookback_tokens = EXCLUDED.lookback_tokens"
    ).bind(session_id).bind(&json).execute(pool).await?;
    admin_load_compaction_config(pool, session_id).await
}

//#endregion 🔖AdminQueries

//#region 🔖AdminHandlers

#[derive(Clone)]
pub struct AdminState {
    pub pool: PgPool,
    pub directory: SessionDirectory,
    pub config: AdminConfig,
}

pub fn router(state: AdminState) -> Router<()> {
    Router::new()
        .route("/admin", get(handler_dashboard))
        .route("/admin/", get(handler_dashboard))
        .route("/admin/overview", get(handler_overview))
        .route("/admin/sessions", get(handler_list_sessions))
        .route("/admin/sessions/{id}", get(handler_session_detail))
        .route("/admin/sessions/{id}/passivate", post(handler_passivate_session))
        .route("/admin/sessions/{id}/close", post(handler_close_session))
        .route("/admin/kits", get(handler_list_kits))
        .route("/admin/persons", get(handler_list_persons))
        .route("/admin/share-tokens", get(handler_list_share_tokens))
        .route("/admin/share-tokens/{token}", axum::routing::delete(handler_revoke_share_token))
        .route("/admin/connections", get(handler_list_connections))
        .route("/admin/config/{session_id}", get(handler_get_config))
        .route("/admin/config/{session_id}", axum::routing::patch(handler_patch_config))
        .with_state(state)
}

async fn handler_dashboard() -> Response {
    (StatusCode::OK, [("content-type", "text/html; charset=utf-8")], DASHBOARD_HTML).into_response()
}

async fn handler_overview(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<AdminOverview>, SessionError> {
    require_admin(&headers, &s.config)?;
    let overview = load_admin_overview(&s.pool, &s.directory, *s.config.started_at).await?;
    Ok(Json(overview))
}

async fn handler_list_sessions(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminSessionRow>>, SessionError> {
    require_admin(&headers, &s.config)?;
    Ok(Json(load_admin_sessions(&s.pool, &s.directory).await?))
}

async fn handler_session_detail(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<AdminSessionDetail>, SessionError> {
    require_admin(&headers, &s.config)?;
    Ok(Json(load_admin_session_detail(&s.pool, &s.directory, session_id).await?))
}

async fn handler_passivate_session(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<serde_json::Value>, SessionError> {
    require_admin(&headers, &s.config)?;
    s.directory.remove(&session_id);
    Ok(Json(serde_json::json!({"passivated": true, "session_id": session_id})))
}

async fn handler_close_session(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<serde_json::Value>, SessionError> {
    require_admin(&headers, &s.config)?;
    let ok = admin_close_session(&s.pool, session_id).await?;
    if !ok { return Err(SessionError::SessionNotFound(session_id.to_string())); }
    s.directory.remove(&session_id);
    Ok(Json(serde_json::json!({"closed": true, "session_id": session_id})))
}

async fn handler_list_kits(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminKitRow>>, SessionError> {
    require_admin(&headers, &s.config)?;
    Ok(Json(load_admin_kits(&s.pool).await?))
}

async fn handler_list_persons(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminPersonRow>>, SessionError> {
    require_admin(&headers, &s.config)?;
    Ok(Json(load_admin_persons(&s.pool).await?))
}

async fn handler_list_share_tokens(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminShareTokenRow>>, SessionError> {
    require_admin(&headers, &s.config)?;
    Ok(Json(load_admin_share_tokens(&s.pool).await?))
}

async fn handler_revoke_share_token(State(s): State<AdminState>, headers: HeaderMap, Path(token): Path<Uuid>) -> Result<Json<serde_json::Value>, SessionError> {
    require_admin(&headers, &s.config)?;
    let deleted = delete_share_token(&s.pool, token).await?;
    Ok(Json(serde_json::json!({"revoked": deleted, "token": token})))
}

async fn handler_list_connections(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<ActiveSessionInfo>>, SessionError> {
    require_admin(&headers, &s.config)?;
    Ok(Json(s.directory.list_active()))
}

async fn handler_get_config(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<AdminCompactionConfig>, SessionError> {
    require_admin(&headers, &s.config)?;
    Ok(Json(admin_load_compaction_config(&s.pool, session_id).await?))
}

#[derive(Deserialize)]
pub struct PatchConfigBody { pub lookback_tokens: Vec<String> }

async fn handler_patch_config(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>, Json(body): Json<PatchConfigBody>) -> Result<Json<AdminCompactionConfig>, SessionError> {
    require_admin(&headers, &s.config)?;
    let known: std::collections::HashSet<&'static str> = lookback_tokens().iter().copied().collect();
    for t in &body.lookback_tokens {
        if !known.contains(t.as_str()) {
            return Err(SessionError::Validation(format!("unknown lookback token: {}", t)));
        }
    }
    Ok(Json(admin_update_compaction_config(&s.pool, session_id, body.lookback_tokens).await?))
}

//#endregion 🔖AdminHandlers

//#region 🔖Dashboard HTML

pub const DASHBOARD_HTML: &str = r###"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>semio · server admin</title>
<style>
:root {
  --bg: #0e1014;
  --fg: #e6e8eb;
  --muted: #8b95a1;
  --panel: #171a21;
  --panel-2: #1f232c;
  --border: #2a2f3a;
  --accent: #ff7a1f;
  --accent-2: #4fd1c5;
  --danger: #ef476f;
  --ok: #06d6a0;
}
* { box-sizing: border-box; }
html,body { margin:0; padding:0; background: var(--bg); color: var(--fg); font-family: 'JetBrains Mono', ui-monospace, Menlo, Consolas, monospace; font-size: 13px; }
header { display:flex; justify-content:space-between; align-items:center; padding: 12px 20px; border-bottom: 1px solid var(--border); background: var(--panel); }
header h1 { margin:0; font-size: 15px; font-weight: 600; letter-spacing: 0.06em; text-transform: uppercase; }
header h1 span { color: var(--accent); }
.badge { padding: 2px 8px; border: 1px solid var(--border); border-radius: 2px; color: var(--muted); margin-left: 10px; font-size: 11px; }
.main { display: grid; grid-template-columns: 200px 1fr; min-height: calc(100vh - 54px); }
nav { border-right: 1px solid var(--border); background: var(--panel); padding: 12px 0; }
nav button { display:block; width:100%; text-align:left; padding: 10px 20px; background: transparent; color: var(--fg); border: none; cursor: pointer; font: inherit; border-left: 3px solid transparent; }
nav button:hover { background: var(--panel-2); }
nav button.active { border-left-color: var(--accent); background: var(--panel-2); color: var(--accent); }
section.view { padding: 20px; overflow: auto; }
section.view h2 { margin: 0 0 16px 0; font-size: 13px; font-weight: 600; letter-spacing: 0.08em; text-transform: uppercase; color: var(--muted); }
.cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(180px,1fr)); gap: 12px; margin-bottom: 24px; }
.card { border: 1px solid var(--border); padding: 14px; background: var(--panel); }
.card .k { color: var(--muted); font-size: 11px; letter-spacing: 0.08em; text-transform: uppercase; }
.card .v { font-size: 22px; margin-top: 8px; color: var(--fg); }
.card .v.accent { color: var(--accent); }
.card .v.ok { color: var(--ok); }
.card .v.danger { color: var(--danger); }
table { width: 100%; border-collapse: collapse; background: var(--panel); border: 1px solid var(--border); }
th, td { padding: 8px 10px; border-bottom: 1px solid var(--border); text-align: left; vertical-align: top; font-size: 12px; }
th { background: var(--panel-2); color: var(--muted); font-weight: 600; text-transform: uppercase; font-size: 11px; letter-spacing: 0.06em; }
tr:hover td { background: var(--panel-2); }
.muted { color: var(--muted); }
.ok { color: var(--ok); }
.danger { color: var(--danger); }
button.act { background: var(--panel-2); color: var(--fg); border: 1px solid var(--border); padding: 4px 10px; font-family: inherit; font-size: 11px; cursor: pointer; }
button.act:hover { border-color: var(--accent); color: var(--accent); }
button.act.danger { border-color: var(--danger); color: var(--danger); }
.controls { display:flex; gap: 8px; align-items: center; margin-bottom: 12px; }
input[type=text], input[type=password] { background: var(--panel-2); color: var(--fg); border: 1px solid var(--border); padding: 6px 10px; font: inherit; min-width: 260px; }
input[type=text]:focus, input[type=password]:focus { outline: none; border-color: var(--accent); }
pre.json { background: var(--panel-2); padding: 12px; border: 1px solid var(--border); overflow: auto; white-space: pre-wrap; max-height: 500px; }
.auth-wrap { display:flex; align-items:center; justify-content:center; min-height: 80vh; }
.auth-box { border: 1px solid var(--border); padding: 28px 32px; background: var(--panel); width: 380px; }
.auth-box h2 { margin-top: 0; }
.flex-row { display:flex; gap: 8px; align-items: center; flex-wrap: wrap; }
code { color: var(--accent-2); }
</style>
</head>
<body>
<div id="auth" class="auth-wrap">
  <div class="auth-box">
    <h2>semio · server admin</h2>
    <p class="muted">Enter the admin bearer token configured via <code>SEMIO_ADMIN_TOKEN</code>.</p>
    <form id="auth-form">
      <div class="flex-row" style="margin-top:12px"><input type="password" id="token-input" placeholder="admin token" autofocus></div>
      <div class="flex-row" style="margin-top:12px"><button type="submit" class="act">Sign in</button><span class="muted" id="auth-error"></span></div>
    </form>
  </div>
</div>
<div id="app" style="display:none">
  <header>
    <h1>semio · <span>server</span> · admin<span class="badge" id="uptime">uptime · --</span></h1>
    <div class="flex-row"><span class="muted" id="ts"></span><button class="act" onclick="signOut()">Sign out</button></div>
  </header>
  <div class="main">
    <nav>
      <button class="tab-btn active" data-tab="overview">Overview</button>
      <button class="tab-btn" data-tab="sessions">Sessions</button>
      <button class="tab-btn" data-tab="kits">Kits</button>
      <button class="tab-btn" data-tab="persons">Persons</button>
      <button class="tab-btn" data-tab="shares">Share tokens</button>
      <button class="tab-btn" data-tab="connections">Connections</button>
      <button class="tab-btn" data-tab="config">Config</button>
    </nav>
    <section class="view" id="view"></section>
  </div>
</div>
<script>
const LS_KEY = 'semio.admin.token';
let token = sessionStorage.getItem(LS_KEY) || '';
let current = 'overview';

async function api(path, opts={}) {
  const headers = Object.assign({'authorization': 'Bearer ' + token, 'content-type': 'application/json'}, opts.headers || {});
  const res = await fetch(path, Object.assign({}, opts, {headers}));
  if (!res.ok) { const t = await res.text(); throw new Error(res.status + ' ' + t); }
  const ct = res.headers.get('content-type') || '';
  return ct.includes('application/json') ? res.json() : res.text();
}

function show(id) { document.getElementById('auth').style.display = id === 'auth' ? 'flex' : 'none'; document.getElementById('app').style.display = id === 'app' ? 'block' : 'none'; }

function fmtSecs(s) { if (s < 60) return s + 's'; if (s < 3600) return Math.floor(s/60) + 'm'; if (s < 86400) return Math.floor(s/3600) + 'h'; return Math.floor(s/86400) + 'd'; }
function esc(s) { if (s == null) return ''; return String(s).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c])); }

async function signIn(e) {
  e.preventDefault();
  token = document.getElementById('token-input').value.trim();
  try { await api('/admin/overview'); sessionStorage.setItem(LS_KEY, token); show('app'); bindTabs(); load('overview'); setInterval(() => load(current, true), 5000); }
  catch (err) { document.getElementById('auth-error').textContent = 'Invalid token: ' + err.message; }
}

function signOut() { sessionStorage.removeItem(LS_KEY); token = ''; show('auth'); }

function bindTabs() {
  document.querySelectorAll('.tab-btn').forEach(b => b.addEventListener('click', () => {
    document.querySelectorAll('.tab-btn').forEach(x => x.classList.remove('active'));
    b.classList.add('active');
    load(b.dataset.tab);
  }));
}

async function load(tab, silent=false) {
  current = tab;
  const view = document.getElementById('view');
  try {
    if (tab === 'overview') await renderOverview(view);
    else if (tab === 'sessions') await renderSessions(view);
    else if (tab === 'kits') await renderKits(view);
    else if (tab === 'persons') await renderPersons(view);
    else if (tab === 'shares') await renderShares(view);
    else if (tab === 'connections') await renderConnections(view);
    else if (tab === 'config') await renderConfig(view);
    document.getElementById('ts').textContent = 'updated ' + new Date().toLocaleTimeString();
  } catch (err) {
    if (!silent) view.innerHTML = '<pre class="json danger">' + esc(err.message) + '</pre>';
  }
}

async function renderOverview(v) {
  const o = await api('/admin/overview');
  document.getElementById('uptime').textContent = 'uptime · ' + fmtSecs(o.uptime_secs);
  v.innerHTML = `<h2>Overview</h2>
    <div class="cards">
      <div class="card"><div class="k">Sessions total</div><div class="v">${o.total_sessions}</div></div>
      <div class="card"><div class="k">Active</div><div class="v ok">${o.active_sessions}</div></div>
      <div class="card"><div class="k">Passivated</div><div class="v">${o.passivated_sessions}</div></div>
      <div class="card"><div class="k">Closed</div><div class="v danger">${o.closed_sessions}</div></div>
      <div class="card"><div class="k">Actors in memory</div><div class="v accent">${o.active_actors}</div></div>
      <div class="card"><div class="k">WS connections</div><div class="v accent">${o.active_connections}</div></div>
      <div class="card"><div class="k">Kits</div><div class="v">${o.total_kits}</div></div>
      <div class="card"><div class="k">Persons</div><div class="v">${o.total_persons}</div></div>
      <div class="card"><div class="k">Share tokens</div><div class="v">${o.total_share_tokens}</div></div>
    </div>`;
}

async function renderSessions(v) {
  const rows = await api('/admin/sessions');
  v.innerHTML = `<h2>Sessions (${rows.length})</h2>` + tableSessions(rows);
}

function tableSessions(rows) {
  if (!rows.length) return '<p class="muted">No sessions.</p>';
  return `<table><thead><tr><th>Session</th><th>Kit</th><th>Status</th><th>Domain v</th><th>Semio v</th><th>Conn</th><th>Actor</th><th>Updated</th><th></th></tr></thead><tbody>${rows.map(r => `
    <tr>
      <td><code>${esc(r.session_id)}</code></td>
      <td><code class="muted">${esc(r.root_kit_id)}</code></td>
      <td class="${r.status === 'active' ? 'ok' : (r.status === 'closed' ? 'danger' : 'muted')}">${esc(r.status)}</td>
      <td>${r.domain_version}</td>
      <td>${r.semio_version}</td>
      <td>${r.active_connections}</td>
      <td>${r.is_activated ? '<span class="ok">yes</span>' : '<span class="muted">no</span>'}</td>
      <td class="muted">${esc(r.updated_at)}</td>
      <td class="flex-row">
        <button class="act" onclick="detail('${r.session_id}')">Detail</button>
        ${r.is_activated ? `<button class="act" onclick="passivate('${r.session_id}')">Passivate</button>` : ''}
        ${r.status !== 'closed' ? `<button class="act danger" onclick="closeSession('${r.session_id}')">Close</button>` : ''}
      </td>
    </tr>`).join('')}</tbody></table>`;
}

async function detail(id) {
  const d = await api('/admin/sessions/' + id);
  const v = document.getElementById('view');
  v.innerHTML = `<h2>Session · ${esc(id)}</h2>
    <button class="act" onclick="load('sessions')">&larr; Back</button>
    <div class="cards">
      <div class="card"><div class="k">Status</div><div class="v">${esc(d.row.status)}</div></div>
      <div class="card"><div class="k">Connections</div><div class="v">${d.row.active_connections}</div></div>
      <div class="card"><div class="k">Domain v</div><div class="v">${d.row.domain_version}</div></div>
      <div class="card"><div class="k">Semio v</div><div class="v">${d.row.semio_version}</div></div>
    </div>
    <h2>Kit</h2>${d.kit ? `<pre class="json">${esc(JSON.stringify(d.kit, null, 2))}</pre>` : '<p class="muted">No kit.</p>'}
    <h2>Persons (${d.persons.length})</h2>${d.persons.length ? tablePersons(d.persons) : '<p class="muted">None.</p>'}
    <h2>Share tokens (${d.share_tokens.length})</h2>${d.share_tokens.length ? tableShares(d.share_tokens) : '<p class="muted">None.</p>'}`;
}

async function passivate(id) { if (!confirm('Passivate actor for ' + id + '?')) return; await api('/admin/sessions/' + id + '/passivate', {method:'POST'}); load('sessions'); }
async function closeSession(id) { if (!confirm('Close session ' + id + '? This is permanent.')) return; await api('/admin/sessions/' + id + '/close', {method:'POST'}); load('sessions'); }

async function renderKits(v) {
  const rows = await api('/admin/kits');
  v.innerHTML = `<h2>Kits (${rows.length})</h2>` + (rows.length ? `<table><thead><tr><th>Name</th><th>Version</th><th>Session</th><th>Kit id</th><th>Remote</th><th>Lifecycle</th></tr></thead><tbody>${rows.map(r => `<tr><td>${esc(r.name)}</td><td>${esc(r.version || '')}</td><td><code>${esc(r.session_id)}</code></td><td><code class="muted">${esc(r.kit_id)}</code></td><td class="muted">${esc(r.remote || '')}</td><td>${esc(r.lifecycle)}</td></tr>`).join('')}</tbody></table>` : '<p class="muted">No kits.</p>');
}

async function renderPersons(v) { const rows = await api('/admin/persons'); v.innerHTML = `<h2>Persons (${rows.length})</h2>` + (rows.length ? tablePersons(rows) : '<p class="muted">No persons.</p>'); }
function tablePersons(rows) { return `<table><thead><tr><th>Display name</th><th>Person id</th><th>Frontend</th><th>Session</th><th>Present</th><th>Last seen</th></tr></thead><tbody>${rows.map(r => `<tr><td>${esc(r.display_name || '(anonymous)')}</td><td><code class="muted">${esc(r.person_id)}</code></td><td>${esc(r.frontend_id)}</td><td><code class="muted">${esc(r.session_id)}</code></td><td>${r.is_present ? '<span class="ok">yes</span>' : '<span class="muted">no</span>'}</td><td class="muted">${esc(r.last_seen_at)}</td></tr>`).join('')}</tbody></table>`; }

async function renderShares(v) { const rows = await api('/admin/share-tokens'); v.innerHTML = `<h2>Share tokens (${rows.length})</h2>` + (rows.length ? tableShares(rows) : '<p class="muted">None.</p>'); }
function tableShares(rows) { return `<table><thead><tr><th>Token</th><th>Session</th><th>Access</th><th>Entity</th><th>Label</th><th>Expires</th><th></th></tr></thead><tbody>${rows.map(r => `<tr><td><code>${esc(r.token)}</code></td><td><code class="muted">${esc(r.session_id)}</code></td><td>${esc(r.access_mode)}</td><td>${esc(r.entity_kind || '')} ${esc(r.entity_id || '')}</td><td>${esc(r.label || '')}</td><td class="muted">${esc(r.expires_at || '—')}</td><td><button class="act danger" onclick="revokeShare('${r.token}')">Revoke</button></td></tr>`).join('')}</tbody></table>`; }
async function revokeShare(token) { if (!confirm('Revoke share token ' + token + '?')) return; await api('/admin/share-tokens/' + token, {method:'DELETE'}); load(current); }

async function renderConnections(v) {
  const rows = await api('/admin/connections');
  v.innerHTML = `<h2>Active WebSocket connections (${rows.length} session(s))</h2>` + (rows.length ? `<table><thead><tr><th>Session</th><th>Connections</th><th>Activated</th></tr></thead><tbody>${rows.map(r => `<tr><td><code>${esc(r.session_id)}</code></td><td>${r.active_connections}</td><td class="muted">${fmtSecs(r.activated_at_secs_ago)} ago</td></tr>`).join('')}</tbody></table>` : '<p class="muted">No active actors.</p>');
}

async function renderConfig(v) {
  v.innerHTML = `<h2>Compaction config (per session)</h2>
    <div class="controls"><input type="text" id="cfg-sid" placeholder="session uuid"><button class="act" onclick="loadConfig()">Load</button></div>
    <div id="cfg"></div>`;
}

async function loadConfig() {
  const sid = document.getElementById('cfg-sid').value.trim();
  if (!sid) return;
  const cfg = await api('/admin/config/' + sid);
  document.getElementById('cfg').innerHTML = `<p>Last compacted: <span class="muted">${esc(cfg.last_compacted_at || '—')}</span></p>
    <label>Lookback tokens (comma-separated)</label><br><input type="text" id="cfg-tokens" value="${esc(cfg.lookback_tokens.join(','))}" style="width: 500px">
    <div style="margin-top:8px"><button class="act" onclick="saveConfig('${sid}')">Save</button></div>
    <pre class="json">${esc(JSON.stringify(cfg, null, 2))}</pre>`;
}

async function saveConfig(sid) {
  const tokens = document.getElementById('cfg-tokens').value.split(',').map(s => s.trim()).filter(Boolean);
  await api('/admin/config/' + sid, {method:'PATCH', body: JSON.stringify({lookback_tokens: tokens})});
  loadConfig();
}

document.getElementById('auth-form').addEventListener('submit', signIn);
if (token) { api('/admin/overview').then(() => { show('app'); bindTabs(); load('overview'); setInterval(() => load(current, true), 5000); }).catch(() => show('auth')); }
else show('auth');
</script>
</body>
</html>
"###;

//#endregion 🔖Dashboard HTML

} // 🛡️Admin
pub use admin::*;

// 🔖Main
// Specs: Main bootstraps tracing, database, admin config, and HTTP server with both session and admin routers merged.
// Summary: Entry point for the semio session-backend service.


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "semio_hub=debug,tower_http=debug".into()))
        .init();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://semio:semio@localhost:5432/semio_hub".to_string());
    let pool = create_pool(&database_url).await;
    run_migrations(&pool).await;
    let admin_config = AdminConfig::from_env();
    if admin_config.admin_token.is_none() {
        tracing::warn!("SEMIO_ADMIN_TOKEN is not set: /admin/* endpoints will return 403");
    } else {
        tracing::info!("admin dashboard mounted at /admin");
    }
    let app_state = AppState::new(pool.clone());
    let admin_state = AdminState { pool, directory: app_state.directory.clone(), config: admin_config };
    let app_router = api::router(app_state).merge(admin::router(admin_state));
    let default_host = if std::env::var("DEVCONTAINER").as_deref() == Ok("true") { "0.0.0.0" } else { "127.0.0.1" };
    let addr: std::net::SocketAddr = std::env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| format!("{}:8080", default_host))
        .parse().expect("invalid LISTEN_ADDR");
    tracing::info!("semio-hub listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app_router).await.unwrap();
}

// 🔖Main End

#[cfg(test)]
// Specs: Tests cover domain types, commands, events, serde, error HTTP mapping, and integration with metabolism/nakagin data.
// Summary: Comprehensive tests for all domain types, serialization, error mapping, and integration with real asset data.


mod tests { // 📐Tests

use super::*;
    mod domain_tests { // 👓Domain Tests


use super::*;
    #[test]
    pub fn session_id_newtype() {
        let u = Uuid::now_v7();
        let s = SessionId(u);
        assert_eq!(s.0, u);
    }

    #[test]
    pub fn command_id_newtype() {
        let u = Uuid::now_v7();
        let c = CommandId(u);
        assert_eq!(c.0, u);
    }

    #[test]
    pub fn client_id_newtype() {
        let u = Uuid::now_v7();
        let c = ClientId(u);
        assert_eq!(c.0, u);
    }

    #[test]
    pub fn request_id_newtype() {
        let u = Uuid::now_v7();
        let r = RequestId(u);
        assert_eq!(r.0, u);
    }

    #[test]
    pub fn person_id_newtype() {
        let u = Uuid::now_v7();
        let p = PersonId(u);
        assert_eq!(p.0, u);
    }

    #[test]
    pub fn field_patch_set() {
        let fp: FieldPatch<String> = FieldPatch::Set("hello".to_string());
        let json_str = serde_json::to_string(&fp).unwrap();
        let parsed: FieldPatch<String> = serde_json::from_str(&json_str).unwrap();
        assert!(matches!(parsed, FieldPatch::Set(_)));
    }

    #[test]
    pub fn field_patch_clear() {
        let fp: FieldPatch<String> = FieldPatch::Clear;
        let json_str = serde_json::to_string(&fp).unwrap();
        assert!(json_str.contains("null") || json_str.contains("Clear"));
    }

    #[test]
    pub fn entity_kind_serde_roundtrip() {
        let kinds = vec![
            EntityKind::Kit, EntityKind::Type, EntityKind::Design,
            EntityKind::Piece, EntityKind::Connection, EntityKind::Author,
        ];
        for kind in kinds {
            let json = serde_json::to_string(&kind).unwrap();
            let back: EntityKind = serde_json::from_str(&json).unwrap();
            assert_eq!(format!("{:?}", kind), format!("{:?}", back));
        }
    }

    #[test]
    pub fn lifecycle_active_default() {
        let l = Lifecycle::Active;
        assert!(matches!(l, Lifecycle::Active));
    }

    #[test]
    pub fn lifecycle_tombstoned() {
        let l = Lifecycle::Tombstoned { at: 42, by: CommandId(Uuid::nil()) };
        if let Lifecycle::Tombstoned { at, .. } = l { assert_eq!(at, 42); } else { panic!(); }
    }

    #[test]
    pub fn conflict_policy_last_writer_wins() {
        assert_eq!(conflict_policy(PropertyKey::KitName), ConflictPolicy::RejectIfChanged);
    }

    #[test]
    pub fn session_status_display() {
        let s = SessionStatus::Active;
        assert_eq!(format!("{:?}", s), "Active");
    }

    #[test]
    pub fn access_mode_serde_roundtrip() {
        let modes = vec![AccessMode::Owner, AccessMode::Viewer];
        for mode in modes {
            let json = serde_json::to_string(&mode).unwrap();
            let back: AccessMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, back);
        }
    }

    #[test]
    pub fn access_mode_viewer_default() {
        let mode: AccessMode = serde_json::from_str("\"viewer\"").unwrap();
        assert_eq!(mode, AccessMode::Viewer);
    }

    #[test]
    pub fn share_token_id_newtype() {
        let u = Uuid::now_v7();
        let s = ShareTokenId(u);
        assert_eq!(s.0, u);
    }

    } // 👓Domain Tests
    pub use domain_tests::*;

    mod command_tests { // 📜Command Tests


use super::*;
    #[test]
    pub fn command_envelope_serde() {
        let env = CommandEnvelope {
            command_id: CommandId(Uuid::nil()),
            client_id: ClientId(Uuid::nil()),
            request_id: RequestId(Uuid::nil()),
            actor_person_id: PersonId(Uuid::nil()),
            base_domain_version: 0,
        };
        let json = serde_json::to_value(&env).unwrap();
        assert!(json.get("command_id").is_some());
    }

    #[test]
    pub fn create_type_command_serde() {
        let cmd = DomainCommand::CreateType(CreateEntity {
            entity_id: Uuid::now_v7(),
            fields: serde_json::json!({"name": "TestType"}),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("CreateType"));
    }

    #[test]
    pub fn delete_type_command_serde() {
        let cmd = DomainCommand::DeleteType(DeleteEntity {
            entity_id: Uuid::now_v7(),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("DeleteType"));
    }

    #[test]
    pub fn create_design_command_serde() {
        let cmd = DomainCommand::CreateDesign(CreateEntity {
            entity_id: Uuid::now_v7(),
            fields: serde_json::json!({"name": "TestDesign"}),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("CreateDesign"));
    }

    #[test]
    pub fn create_piece_command_serde() {
        let cmd = DomainCommand::CreatePiece(CreatePiece {
            piece_id: Uuid::now_v7(),
            design_id: Uuid::now_v7(),
            fields: serde_json::json!({"name": "piece_a"}),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("CreatePiece"));
    }

    #[test]
    pub fn create_connection_command_serde() {
        let cmd = DomainCommand::CreateConnection(CreateConnection {
            connection_id: Uuid::now_v7(),
            design_id: Uuid::now_v7(),
            fields: serde_json::json!({"connected_piece_id": Uuid::nil().to_string()}),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("CreateConnection"));
    }

    #[test]
    pub fn batch_command_serde() {
        let cmd = DomainCommand::Batch(DomainBatch {
            commands: vec![
                DomainCommand::CreateType(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": "A"}) }),
                DomainCommand::CreateType(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": "B"}) }),
            ],
        });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("Batch"));
    }

    #[test]
    pub fn semio_command_cursor_serde() {
        let cmd = SemioCommand::UpsertCursor(UpsertCursor { u: 1.0, v: 2.0 });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("UpsertCursor"));
    }

    #[test]
    pub fn semio_command_look_serde() {
        let cmd = SemioCommand::UpsertLook(UpsertLook {
            position: [1.0, 2.0, 3.0], forward: [0.0, 0.0, 1.0], up: [0.0, 1.0, 0.0],
        });
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("UpsertLook"));
    }

    #[test]
    pub fn command_result_serde() {
        let r = CommandResult::Accepted { domain_version: 5 };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("Accepted"));
        assert!(json.contains("5"));
    }

    } // 📜Command Tests
    pub use command_tests::*;

    mod error_tests { // 🌤️Error Tests


use super::*;
    pub fn status_of(err: SessionError) -> StatusCode {
        err.into_response().status()
    }

    #[test]
    pub fn session_not_found_returns_404() {
        assert_eq!(status_of(SessionError::SessionNotFound("x".into())), StatusCode::NOT_FOUND);
    }

    #[test]
    pub fn entity_not_found_returns_404() {
        assert_eq!(status_of(SessionError::EntityNotFound { kind: "type".into(), id: "abc".into() }), StatusCode::NOT_FOUND);
    }

    #[test]
    pub fn conflict_returns_409() {
        assert_eq!(status_of(SessionError::Conflict { property: "name".into(), reason: "changed".into() }), StatusCode::CONFLICT);
    }

    #[test]
    pub fn validation_returns_400() {
        assert_eq!(status_of(SessionError::Validation("bad".into())), StatusCode::BAD_REQUEST);
    }

    #[test]
    pub fn actor_gone_returns_503() {
        assert_eq!(status_of(SessionError::ActorGone), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    pub fn idempotent_duplicate_returns_200() {
        assert_eq!(status_of(SessionError::IdempotentDuplicate("cmd".into())), StatusCode::OK);
    }

    #[test]
    pub fn internal_returns_500() {
        assert_eq!(status_of(SessionError::Internal("oops".into())), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    pub fn unauthorized_returns_401() {
        assert_eq!(status_of(SessionError::Unauthorized("bad token".into())), StatusCode::UNAUTHORIZED);
    }

    #[test]
    pub fn forbidden_returns_403() {
        assert_eq!(status_of(SessionError::Forbidden("no write access".into())), StatusCode::FORBIDDEN);
    }

    } // 🌤️Error Tests
    pub use error_tests::*;

    mod event_tests { // 🔮Event Tests


use super::*;
    #[test]
    pub fn session_event_domain_accepted_serde() {
        let ev = SessionEvent::DomainCommandAccepted {
            command_id: CommandId(Uuid::nil()),
            domain_version: 1,
            changes: vec![EntityChange::Created {
                entity_kind: EntityKind::Type,
                entity_id: Uuid::nil(),
                snapshot: serde_json::json!({"name": "T"}),
            }],
        };
        let json = serde_json::to_string(&ev).unwrap();
        assert!(json.contains("DomainCommandAccepted"));
        let back: SessionEvent = serde_json::from_str(&json).unwrap();
        assert!(matches!(back, SessionEvent::DomainCommandAccepted { .. }));
    }

    #[test]
    pub fn session_event_semio_updated_serde() {
        let ev = SessionEvent::SemioUpdated {
            semio_version: 3,
            person_id: PersonId(Uuid::nil()),
            frontend_id: "desktop".into(),
            update: SemioUpdate::CursorMoved { u: 1.0, v: 2.0 },
        };
        let json = serde_json::to_string(&ev).unwrap();
        assert!(json.contains("SemioUpdated"));
    }

    #[test]
    pub fn session_event_closed_serde() {
        let ev = SessionEvent::SessionClosed;
        let json = serde_json::to_string(&ev).unwrap();
        assert!(json.contains("SessionClosed"));
    }

    #[test]
    pub fn entity_change_variants_serde() {
        let created = EntityChange::Created { entity_kind: EntityKind::Piece, entity_id: Uuid::nil(), snapshot: serde_json::json!({}) };
        let updated = EntityChange::Updated { entity_kind: EntityKind::Kit, entity_id: Uuid::nil(), changed_fields: serde_json::json!({"name": "x"}) };
        let deleted = EntityChange::Deleted { entity_kind: EntityKind::Connection, entity_id: Uuid::nil() };
        for change in [created, updated, deleted] {
            let json = serde_json::to_string(&change).unwrap();
            let _back: EntityChange = serde_json::from_str(&json).unwrap();
        }
    }

    #[test]
    pub fn semio_update_variants_serde() {
        let updates = vec![
            SemioUpdate::CursorMoved { u: 0.5, v: 0.5 },
            SemioUpdate::LookChanged { position: [1.0, 2.0, 3.0], forward: [0.0, 0.0, 1.0], up: [0.0, 1.0, 0.0] },
            SemioUpdate::SelectionChanged { piece_ids: vec![Uuid::nil()], design_ids: vec![] },
            SemioUpdate::PresenceCleared,
        ];
        for u in updates {
            let json = serde_json::to_string(&u).unwrap();
            let _back: SemioUpdate = serde_json::from_str(&json).unwrap();
        }
    }

    } // 🔮Event Tests
    pub use event_tests::*;

    mod state_tests { // 📝State Tests


use super::*;
    #[test]
    pub fn session_state_creation() {
        let sid = Uuid::now_v7();
        let kid = Uuid::now_v7();
        let state = SessionState {
            session_id: SessionId(sid), domain_version: 0, semio_version: 0,
            status: SessionStatus::Active,
            kit: KitState { kit_id: kid, name: "Test".into(), version: None, description: None, icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
            authors: BTreeMap::new(), locations: BTreeMap::new(), folders: BTreeMap::new(), files: BTreeMap::new(),
            tags: BTreeMap::new(), concepts: BTreeMap::new(), ports: BTreeMap::new(), qualities: BTreeMap::new(),
            types: BTreeMap::new(), designs: BTreeMap::new(), semio_people: BTreeMap::new(),
        };
        assert_eq!(state.session_id.0, sid);
        assert_eq!(state.kit.name, "Test");
    }

    #[test]
    pub fn type_state_with_connectors() {
        let tid = Uuid::now_v7();
        let cid = Uuid::now_v7();
        let mut ts = TypeState {
            type_id: tid, name: "Box".into(), parent_type_id: None, description: None, icon: None, image: None,
            folder: None, unit: None, stock: None, is_abstract: None, virtual_type: None, location_id: None,
            connectors: BTreeMap::new(), representations: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
        };
        ts.connectors.insert(cid, ConnectorState {
            connector_id: cid, name: Some("top".into()), t: 0.5,
            point: [0.0, 0.0, 1.0], direction: [0.0, 0.0, 1.0],
            description: None, port_id: None, mandatory: Some(true), max_children: None, lifecycle: Lifecycle::Active,
        });
        assert_eq!(ts.connectors.len(), 1);
    }

    #[test]
    pub fn design_state_with_pieces_and_connections() {
        let did = Uuid::now_v7();
        let p1 = Uuid::now_v7();
        let p2 = Uuid::now_v7();
        let conn_id = Uuid::now_v7();
        let mut ds = DesignState {
            design_id: did, name: "Tower".into(), parent_design_id: None, description: None, icon: None, image: None,
            folder: None, unit: None, is_abstract: None, can_scale: None, can_mirror: None,
            active_layer_id: None, location_id: None,
            pieces: BTreeMap::new(), connections: BTreeMap::new(), layers: BTreeMap::new(),
            groups: BTreeMap::new(), stats: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
        };
        ds.pieces.insert(p1, PieceState {
            piece_id: p1, name: Some("a".into()), type_id: None, design_ref_id: None,
            plane: None, center: Some([0.0, 0.0]), scale: None, mirror_plane: None,
            is_hidden: None, is_locked: None, color: None, description: None, lifecycle: Lifecycle::Active,
        });
        ds.pieces.insert(p2, PieceState {
            piece_id: p2, name: Some("b".into()), type_id: None, design_ref_id: None,
            plane: None, center: None, scale: None, mirror_plane: None,
            is_hidden: None, is_locked: None, color: None, description: None, lifecycle: Lifecycle::Active,
        });
        ds.connections.insert(conn_id, ConnectionState {
            connection_id: conn_id, connected_piece_id: p1, connected_design_piece_id: None, connected_connector_id: None,
            connecting_piece_id: p2, connecting_design_piece_id: None, connecting_connector_id: None,
            gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0,
            u: None, v: None, description: None, lifecycle: Lifecycle::Active,
        });
        assert_eq!(ds.pieces.len(), 2);
        assert_eq!(ds.connections.len(), 1);
    }

    } // 📝State Tests
    pub use state_tests::*;

    mod metabolism_integration_tests { // 🔐Metabolism Integration Tests


use super::*;
    pub fn load_metabolism_kit_json() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("assets/semio/metabolism.kit.semio.json");
        let data = std::fs::read_to_string(&path).expect("metabolism kit JSON");
        serde_json::from_str(&data).expect("parse metabolism kit JSON")
    }

    #[test]
    pub fn metabolism_kit_parses_50_types() {
        let kit = load_metabolism_kit_json();
        let types = kit["types"].as_array().expect("types array");
        assert_eq!(types.len(), 50, "metabolism kit should have 50 types");
    }

    #[test]
    pub fn metabolism_kit_name() {
        let kit = load_metabolism_kit_json();
        assert_eq!(kit["name"].as_str().unwrap(), "Metabolism");
    }

    #[test]
    pub fn metabolism_kit_has_authors() {
        let kit = load_metabolism_kit_json();
        let authors = kit["authors"].as_array().expect("authors array");
        assert!(!authors.is_empty());
        assert_eq!(authors[0]["name"].as_str().unwrap(), "Ueli Saluz");
    }

    #[test]
    pub fn metabolism_kit_has_designs() {
        let kit = load_metabolism_kit_json();
        let designs = kit["designs"].as_array().expect("designs array");
        assert_eq!(designs.len(), 10, "metabolism kit has 10 designs");
    }

    #[test]
    pub fn metabolism_build_types_in_session_state() {
        let kit_json = load_metabolism_kit_json();
        let types_json = kit_json["types"].as_array().unwrap();
        let kit_id = Uuid::now_v7();
        let mut state = SessionState {
            session_id: SessionId(Uuid::now_v7()), domain_version: 0, semio_version: 0,
            status: SessionStatus::Active,
            kit: KitState { kit_id, name: "Metabolism".into(), version: Some("1.0".into()), description: None, icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
            authors: BTreeMap::new(), locations: BTreeMap::new(), folders: BTreeMap::new(), files: BTreeMap::new(),
            tags: BTreeMap::new(), concepts: BTreeMap::new(), ports: BTreeMap::new(), qualities: BTreeMap::new(),
            types: BTreeMap::new(), designs: BTreeMap::new(), semio_people: BTreeMap::new(),
        };
        for t in types_json {
            let id = Uuid::parse_str(t["id"].as_str().unwrap()).unwrap();
            let name = t["name"].as_str().unwrap().to_string();
            let parent_id = t.get("parent").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok());
            let desc = t.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
            let icon = t.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string());
            state.types.insert(id, TypeState {
                type_id: id, name, parent_type_id: parent_id, description: desc,
                icon, image: None, folder: None, unit: None, stock: None, is_abstract: None,
                virtual_type: None, location_id: None,
                connectors: BTreeMap::new(), representations: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
            });
        }
        assert_eq!(state.types.len(), 50, "state should contain all 50 metabolism types");
        let capsule_id = Uuid::parse_str("71749140-9db9-43f6-bd81-d89011667b80").unwrap();
        assert!(state.types.contains_key(&capsule_id), "should have Capsule type");
        assert_eq!(state.types[&capsule_id].name, "Capsule");
    }

    #[test]
    pub fn metabolism_create_type_commands_for_all_types() {
        let kit_json = load_metabolism_kit_json();
        let types_json = kit_json["types"].as_array().unwrap();
        let mut commands: Vec<DomainCommand> = Vec::new();
        for t in types_json {
            let id = Uuid::parse_str(t["id"].as_str().unwrap()).unwrap();
            let fields = serde_json::json!({"name": t["name"]});
            commands.push(DomainCommand::CreateType(CreateEntity { entity_id: id, fields }));
        }
        assert_eq!(commands.len(), 50);
        let batch = DomainCommand::Batch(DomainBatch { commands });
        let json = serde_json::to_string(&batch).unwrap();
        assert!(json.contains("Batch"));
        assert!(json.contains("Capsule"));
    }

    } // 🔐Metabolism Integration Tests
    pub use metabolism_integration_tests::*;

    mod nakagin_integration_tests { // 🎵Nakagin Integration Tests


use super::*;
    pub fn load_nakagin_design_json() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("assets/semio/nakagin-capsule-tower.shallow.design.semio.json");
        let data = std::fs::read_to_string(&path).expect("nakagin design JSON");
        serde_json::from_str(&data).expect("parse nakagin design JSON")
    }

    pub fn load_nakagin_diff_json() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("assets/semio/nakagin-capsule-tower.with-diff.design.semio.json");
        let data = std::fs::read_to_string(&path).expect("nakagin diff JSON");
        serde_json::from_str(&data).expect("parse nakagin diff JSON")
    }

    #[test]
    pub fn nakagin_design_parses_180_pieces() {
        let design = load_nakagin_design_json();
        let pieces = design["pieces"].as_array().unwrap();
        assert_eq!(pieces.len(), 180, "nakagin design should have 180 pieces");
    }

    #[test]
    pub fn nakagin_design_parses_179_connections() {
        let design = load_nakagin_design_json();
        let conns = design["connections"].as_array().unwrap();
        assert_eq!(conns.len(), 179, "nakagin design should have 179 connections");
    }

    #[test]
    pub fn nakagin_design_name() {
        let design = load_nakagin_design_json();
        assert_eq!(design["name"].as_str().unwrap(), "Nakagin Capsule Tower");
    }

    #[test]
    pub fn nakagin_build_design_state() {
        let design_json = load_nakagin_design_json();
        let design_id = Uuid::parse_str(design_json["id"].as_str().unwrap()).unwrap();
        let mut ds = DesignState {
            design_id, name: design_json["name"].as_str().unwrap().to_string(),
            parent_design_id: None, description: None, icon: None, image: None,
            folder: None, unit: design_json.get("unit").and_then(|v| v.as_str()).map(|s| s.to_string()),
            is_abstract: None, can_scale: None, can_mirror: None,
            active_layer_id: None, location_id: None,
            pieces: BTreeMap::new(), connections: BTreeMap::new(), layers: BTreeMap::new(),
            groups: BTreeMap::new(), stats: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
        };
        for p in design_json["pieces"].as_array().unwrap() {
            let pid = Uuid::parse_str(p["id"].as_str().unwrap()).unwrap();
            let name = p.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
            let center = p.get("center").and_then(|c| {
                let u = c.get("u")?.as_f64()?;
                let v = c.get("v")?.as_f64()?;
                Some([u, v])
            });
            ds.pieces.insert(pid, PieceState {
                piece_id: pid, name, type_id: None, design_ref_id: None,
                plane: None, center, scale: None, mirror_plane: None,
                is_hidden: p.get("isHidden").and_then(|v| v.as_bool()),
                is_locked: p.get("isLocked").and_then(|v| v.as_bool()),
                color: None, description: p.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                lifecycle: Lifecycle::Active,
            });
        }
        for c in design_json["connections"].as_array().unwrap() {
            let cid = Uuid::parse_str(c["id"].as_str().unwrap()).unwrap();
            let connected_piece = c.get("connected").and_then(|v| v.get("piece")).and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
            let connecting_piece = c.get("connecting").and_then(|v| v.get("piece")).and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
            ds.connections.insert(cid, ConnectionState {
                connection_id: cid,
                connected_piece_id: connected_piece, connected_design_piece_id: None, connected_connector_id: None,
                connecting_piece_id: connecting_piece, connecting_design_piece_id: None, connecting_connector_id: None,
                gap: c.get("gap").and_then(|v| v.as_f64()).unwrap_or(0.0),
                shift: c.get("shift").and_then(|v| v.as_f64()).unwrap_or(0.0),
                rise: c.get("rise").and_then(|v| v.as_f64()).unwrap_or(0.0),
                rotation: c.get("rotation").and_then(|v| v.as_f64()).unwrap_or(0.0),
                turn: c.get("turn").and_then(|v| v.as_f64()).unwrap_or(0.0),
                tilt: c.get("tilt").and_then(|v| v.as_f64()).unwrap_or(0.0),
                u: c.get("u").and_then(|v| v.as_f64()),
                v: c.get("v").and_then(|v| v.as_f64()),
                description: c.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                lifecycle: Lifecycle::Active,
            });
        }
        assert_eq!(ds.pieces.len(), 180, "design state should have 180 pieces");
        assert_eq!(ds.connections.len(), 179, "design state should have 179 connections");
    }

    #[test]
    pub fn nakagin_diff_has_diff_status() {
        let diff_json = load_nakagin_diff_json();
        let pieces = diff_json["pieces"].as_array().unwrap();
        assert!(!pieces.is_empty());
        let has_diff = pieces.iter().any(|p| {
            p.get("attributes").and_then(|a| a.as_array()).map_or(false, |attrs| {
                attrs.iter().any(|attr| attr.get("key").and_then(|k| k.as_str()) == Some("semio.diffStatus"))
            })
        });
        assert!(has_diff, "at least one piece should have semio.diffStatus attribute");
    }

    #[test]
    pub fn nakagin_create_piece_commands_for_all_pieces() {
        let design_json = load_nakagin_design_json();
        let design_id = Uuid::parse_str(design_json["id"].as_str().unwrap()).unwrap();
        let pieces_json = design_json["pieces"].as_array().unwrap();
        let mut commands: Vec<DomainCommand> = Vec::new();
        for p in pieces_json {
            let pid = Uuid::parse_str(p["id"].as_str().unwrap()).unwrap();
            let fields = serde_json::json!({"name": p.get("name")});
            commands.push(DomainCommand::CreatePiece(CreatePiece { piece_id: pid, design_id, fields }));
        }
        assert_eq!(commands.len(), 180, "should create 180 CreatePiece commands");
    }

    #[test]
    pub fn nakagin_create_connection_commands_for_all_connections() {
        let design_json = load_nakagin_design_json();
        let design_id = Uuid::parse_str(design_json["id"].as_str().unwrap()).unwrap();
        let conns_json = design_json["connections"].as_array().unwrap();
        let mut commands: Vec<DomainCommand> = Vec::new();
        for c in conns_json {
            let cid = Uuid::parse_str(c["id"].as_str().unwrap()).unwrap();
            let connected_piece = c.get("connected").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).unwrap_or("");
            let connecting_piece = c.get("connecting").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).unwrap_or("");
            let fields = serde_json::json!({
                "connected_piece_id": connected_piece,
                "connecting_piece_id": connecting_piece,
                "gap": c.get("gap"),
                "shift": c.get("shift"),
            });
            commands.push(DomainCommand::CreateConnection(CreateConnection { connection_id: cid, design_id, fields }));
        }
        assert_eq!(commands.len(), 179, "should create 179 CreateConnection commands");
    }

    } // 🎵Nakagin Integration Tests
    pub use nakagin_integration_tests::*;

    mod multi_frontend_tests { // 🗽Multi-Frontend Tests


use super::*;
    #[test]
    pub fn multi_frontend_cursor_events() {
        let person_a = PersonId(Uuid::now_v7());
        let person_b = PersonId(Uuid::now_v7());
        let events: Vec<SessionEvent> = vec![
            SessionEvent::SemioUpdated { semio_version: 1, person_id: person_a, frontend_id: "desktop".into(), update: SemioUpdate::CursorMoved { u: 0.1, v: 0.2 } },
            SessionEvent::SemioUpdated { semio_version: 2, person_id: person_b, frontend_id: "web".into(), update: SemioUpdate::CursorMoved { u: 0.5, v: 0.5 } },
            SessionEvent::SemioUpdated { semio_version: 3, person_id: person_a, frontend_id: "desktop".into(), update: SemioUpdate::CursorMoved { u: 0.3, v: 0.4 } },
        ];
        assert_eq!(events.len(), 3);
        for ev in &events {
            let json = serde_json::to_string(ev).unwrap();
            let _back: SessionEvent = serde_json::from_str(&json).unwrap();
        }
    }

    #[test]
    pub fn multi_frontend_selection_independence() {
        let pid_a = PersonId(Uuid::now_v7());
        let pid_b = PersonId(Uuid::now_v7());
        let piece1 = Uuid::now_v7();
        let piece2 = Uuid::now_v7();
        let ev_a = SessionEvent::SemioUpdated {
            semio_version: 1, person_id: pid_a, frontend_id: "desktop".into(),
            update: SemioUpdate::SelectionChanged { piece_ids: vec![piece1], design_ids: vec![] },
        };
        let ev_b = SessionEvent::SemioUpdated {
            semio_version: 2, person_id: pid_b, frontend_id: "web".into(),
            update: SemioUpdate::SelectionChanged { piece_ids: vec![piece2], design_ids: vec![] },
        };
        let json_a = serde_json::to_string(&ev_a).unwrap();
        let json_b = serde_json::to_string(&ev_b).unwrap();
        assert!(json_a.contains(&piece1.to_string()));
        assert!(json_b.contains(&piece2.to_string()));
        assert!(!json_a.contains(&piece2.to_string()));
        assert!(!json_b.contains(&piece1.to_string()));
    }

    #[test]
    pub fn multi_frontend_look_changes() {
        let pid = PersonId(Uuid::now_v7());
        let events = vec![
            SessionEvent::SemioUpdated {
                semio_version: 1, person_id: pid, frontend_id: "desktop".into(),
                update: SemioUpdate::LookChanged { position: [10.0, 20.0, 30.0], forward: [0.0, 0.0, -1.0], up: [0.0, 1.0, 0.0] },
            },
            SessionEvent::SemioUpdated {
                semio_version: 2, person_id: pid, frontend_id: "vr".into(),
                update: SemioUpdate::LookChanged { position: [5.0, 5.0, 5.0], forward: [1.0, 0.0, 0.0], up: [0.0, 0.0, 1.0] },
            },
        ];
        for ev in &events {
            let json = serde_json::to_string(ev).unwrap();
            assert!(json.contains("LookChanged"));
        }
    }

    #[tokio::test]
    async fn multi_frontend_broadcast_via_channel() {
        let (tx, _) = broadcast::channel::<SessionEvent>(64);
        let mut rx1 = tx.subscribe();
        let mut rx2 = tx.subscribe();
        let mut rx3 = tx.subscribe();
        let pid = PersonId(Uuid::now_v7());
        let event = SessionEvent::SemioUpdated {
            semio_version: 1, person_id: pid, frontend_id: "desktop".into(),
            update: SemioUpdate::CursorMoved { u: 0.5, v: 0.5 },
        };
        tx.send(event.clone()).unwrap();
        let e1 = rx1.recv().await.unwrap();
        let e2 = rx2.recv().await.unwrap();
        let e3 = rx3.recv().await.unwrap();
        let j1 = serde_json::to_string(&e1).unwrap();
        let j2 = serde_json::to_string(&e2).unwrap();
        let j3 = serde_json::to_string(&e3).unwrap();
        assert_eq!(j1, j2);
        assert_eq!(j2, j3);
    }

    #[tokio::test]
    async fn multi_frontend_concurrent_commands_via_actor_channel() {
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<(String, DomainCommand)>(64);
        let cmd_tx1 = cmd_tx.clone();
        let cmd_tx2 = cmd_tx.clone();
        let cmd_tx3 = cmd_tx.clone();
        let t1 = tokio::spawn(async move {
            for i in 0..10 {
                let cmd = DomainCommand::CreateType(CreateEntity {
                    entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": format!("A-{}", i)}),
                });
                cmd_tx1.send(("frontend-1".into(), cmd)).await.unwrap();
            }
        });
        let t2 = tokio::spawn(async move {
            for i in 0..10 {
                let cmd = DomainCommand::CreateType(CreateEntity {
                    entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": format!("B-{}", i)}),
                });
                cmd_tx2.send(("frontend-2".into(), cmd)).await.unwrap();
            }
        });
        let t3 = tokio::spawn(async move {
            for i in 0..10 {
                let cmd = DomainCommand::CreateType(CreateEntity {
                    entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": format!("C-{}", i)}),
                });
                cmd_tx3.send(("frontend-3".into(), cmd)).await.unwrap();
            }
        });
        drop(cmd_tx);
        t1.await.unwrap();
        t2.await.unwrap();
        t3.await.unwrap();
        let mut received = Vec::new();
        while let Some((frontend, cmd)) = cmd_rx.recv().await {
            received.push((frontend, cmd));
        }
        assert_eq!(received.len(), 30, "should receive all 30 commands from 3 frontends");
        let f1_count = received.iter().filter(|(f, _)| f == "frontend-1").count();
        let f2_count = received.iter().filter(|(f, _)| f == "frontend-2").count();
        let f3_count = received.iter().filter(|(f, _)| f == "frontend-3").count();
        assert_eq!(f1_count, 10);
        assert_eq!(f2_count, 10);
        assert_eq!(f3_count, 10);
    }

    } // 🗽Multi-Frontend Tests
    pub use multi_frontend_tests::*;

    mod full_metabolism_nakagin_session_test { // 🌦️Full Metabolism + Nakagin Session Test


use super::*;
    #[test]
    pub fn full_session_with_metabolism_types_and_nakagin_design() {
        let kit_json = load_metabolism_kit_json();
        let design_json = load_nakagin_design_json();
        let session_id = Uuid::now_v7();
        let kit_id = Uuid::parse_str(kit_json["id"].as_str().unwrap()).unwrap();
        let mut state = SessionState {
            session_id: SessionId(session_id), domain_version: 0, semio_version: 0,
            status: SessionStatus::Active,
            kit: KitState {
                kit_id, name: kit_json["name"].as_str().unwrap().to_string(),
                version: kit_json.get("version").and_then(|v| v.as_str()).map(|s| s.to_string()),
                description: kit_json.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                icon: kit_json.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string()),
                image: kit_json.get("image").and_then(|v| v.as_str()).map(|s| s.to_string()),
                preview: kit_json.get("preview").and_then(|v| v.as_str()).map(|s| s.to_string()),
                remote: kit_json.get("remote").and_then(|v| v.as_str()).map(|s| s.to_string()),
                homepage: kit_json.get("homepage").and_then(|v| v.as_str()).map(|s| s.to_string()),
                license: kit_json.get("license").and_then(|v| v.as_str()).map(|s| s.to_string()),
                lifecycle: Lifecycle::Active,
            },
            authors: BTreeMap::new(), locations: BTreeMap::new(), folders: BTreeMap::new(), files: BTreeMap::new(),
            tags: BTreeMap::new(), concepts: BTreeMap::new(), ports: BTreeMap::new(), qualities: BTreeMap::new(),
            types: BTreeMap::new(), designs: BTreeMap::new(), semio_people: BTreeMap::new(),
        };
        // Add all 50 types
        for t in kit_json["types"].as_array().unwrap() {
            let id = Uuid::parse_str(t["id"].as_str().unwrap()).unwrap();
            state.types.insert(id, TypeState {
                type_id: id, name: t["name"].as_str().unwrap().to_string(),
                parent_type_id: None, description: t.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                icon: t.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string()),
                image: None, folder: None, unit: None, stock: None, is_abstract: None,
                virtual_type: None, location_id: None,
                connectors: BTreeMap::new(), representations: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
            });
            state.domain_version += 1;
        }
        assert_eq!(state.types.len(), 50);
        assert_eq!(state.domain_version, 50);
        // Add nakagin design with 180 pieces and 179 connections
        let design_id = Uuid::parse_str(design_json["id"].as_str().unwrap()).unwrap();
        let mut ds = DesignState {
            design_id, name: design_json["name"].as_str().unwrap().to_string(),
            parent_design_id: None, description: None, icon: None, image: None,
            folder: None, unit: None, is_abstract: None, can_scale: None, can_mirror: None,
            active_layer_id: None, location_id: None,
            pieces: BTreeMap::new(), connections: BTreeMap::new(), layers: BTreeMap::new(),
            groups: BTreeMap::new(), stats: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
        };
        for p in design_json["pieces"].as_array().unwrap() {
            let pid = Uuid::parse_str(p["id"].as_str().unwrap()).unwrap();
            ds.pieces.insert(pid, PieceState {
                piece_id: pid, name: p.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
                type_id: None, design_ref_id: None, plane: None,
                center: p.get("center").and_then(|c| Some([c.get("u")?.as_f64()?, c.get("v")?.as_f64()?])),
                scale: None, mirror_plane: None,
                is_hidden: p.get("isHidden").and_then(|v| v.as_bool()),
                is_locked: p.get("isLocked").and_then(|v| v.as_bool()),
                color: None, description: None, lifecycle: Lifecycle::Active,
            });
            state.domain_version += 1;
        }
        for c in design_json["connections"].as_array().unwrap() {
            let cid = Uuid::parse_str(c["id"].as_str().unwrap()).unwrap();
            let connected_piece = c.get("connected").and_then(|v| v.get("piece")).and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
            let connecting_piece = c.get("connecting").and_then(|v| v.get("piece")).and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
            ds.connections.insert(cid, ConnectionState {
                connection_id: cid,
                connected_piece_id: connected_piece, connected_design_piece_id: None, connected_connector_id: None,
                connecting_piece_id: connecting_piece, connecting_design_piece_id: None, connecting_connector_id: None,
                gap: c.get("gap").and_then(|v| v.as_f64()).unwrap_or(0.0),
                shift: c.get("shift").and_then(|v| v.as_f64()).unwrap_or(0.0),
                rise: c.get("rise").and_then(|v| v.as_f64()).unwrap_or(0.0),
                rotation: c.get("rotation").and_then(|v| v.as_f64()).unwrap_or(0.0),
                turn: c.get("turn").and_then(|v| v.as_f64()).unwrap_or(0.0),
                tilt: c.get("tilt").and_then(|v| v.as_f64()).unwrap_or(0.0),
                u: c.get("u").and_then(|v| v.as_f64()),
                v: c.get("v").and_then(|v| v.as_f64()),
                description: None, lifecycle: Lifecycle::Active,
            });
            state.domain_version += 1;
        }
        state.designs.insert(design_id, ds);
        state.domain_version += 1;
        // Verify final state
        assert_eq!(state.types.len(), 50, "50 metabolism types");
        assert_eq!(state.designs.len(), 1, "1 nakagin design");
        let design = &state.designs[&design_id];
        assert_eq!(design.pieces.len(), 180, "180 nakagin pieces");
        assert_eq!(design.connections.len(), 179, "179 nakagin connections");
        assert_eq!(design.name, "Nakagin Capsule Tower");
        assert_eq!(state.kit.name, "Metabolism");
        // domain_version = 50 types + 180 pieces + 179 connections + 1 design = 410
        assert_eq!(state.domain_version, 410);
    }

    } // 🌦️Full Metabolism + Nakagin Session Test
    pub use full_metabolism_nakagin_session_test::*;

    mod metabolism_diff_tests { // 📹Metabolism Diff Tests


use super::*;
    pub fn load_metabolism_diff_json() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("assets/semio/metabolism.kit.diff.semio.json");
        let data = std::fs::read_to_string(&path).expect("metabolism diff JSON");
        serde_json::from_str(&data).expect("parse metabolism diff JSON")
    }

    #[test]
    pub fn metabolism_diff_has_removed_types() {
        let diff = load_metabolism_diff_json();
        let types = diff.get("types").expect("types key");
        let removed = types.get("removed");
        let updated = types.get("updated");
        assert!(removed.is_some() || updated.is_some(), "diff should have removed or updated types");
    }

    #[test]
    pub fn metabolism_diff_roundtrip_commands() {
        let diff = load_metabolism_diff_json();
        let mut commands: Vec<DomainCommand> = Vec::new();
        if let Some(removed) = diff.get("types").and_then(|t| t.get("removed")).and_then(|r| r.as_array()) {
            for r in removed {
                if let Some(id_str) = r.get("id").and_then(|g| g.as_str()) {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        commands.push(DomainCommand::DeleteType(DeleteEntity { entity_id: id }));
                    }
                }
            }
        }
        if let Some(updated) = diff.get("types").and_then(|t| t.get("updated")).and_then(|u| u.as_array()) {
            for u in updated {
                if let Some(id_str) = u.get("id").and_then(|g| g.as_str()) {
                    if let Ok(id) = Uuid::parse_str(id_str) {
                        commands.push(DomainCommand::PatchType(PatchEntity {
                            entity_id: id, fields: u.clone(),
                        }));
                    }
                }
            }
        }
        let batch = DomainCommand::Batch(DomainBatch { commands: commands.clone() });
        let json = serde_json::to_string(&batch).unwrap();
        let back: DomainCommand = serde_json::from_str(&json).unwrap();
        assert!(matches!(back, DomainCommand::Batch(_)));
    }

    } // 📹Metabolism Diff Tests
    pub use metabolism_diff_tests::*;

    mod lookback_tests { // 🧫Lookback Tests


use super::*;
    #[test]
    pub fn lookback_seconds_known_tokens() {
        assert_eq!(lookback_seconds("1min"), Some(60));
        assert_eq!(lookback_seconds("5min"), Some(300));
        assert_eq!(lookback_seconds("10min"), Some(600));
        assert_eq!(lookback_seconds("30min"), Some(1800));
        assert_eq!(lookback_seconds("1h"), Some(3600));
        assert_eq!(lookback_seconds("5h"), Some(18000));
        assert_eq!(lookback_seconds("1d"), Some(86400));
        assert_eq!(lookback_seconds("3d"), Some(259200));
        assert_eq!(lookback_seconds("7d"), Some(604800));
        assert_eq!(lookback_seconds("1mo"), Some(2592000));
        assert_eq!(lookback_seconds("6mo"), Some(15552000));
        assert_eq!(lookback_seconds("1y"), Some(31536000));
    }

    #[test]
    pub fn lookback_seconds_unknown_token() {
        assert_eq!(lookback_seconds("99x"), None);
        assert_eq!(lookback_seconds(""), None);
    }

    #[test]
    pub fn lookback_tokens_returns_all_12() {
        let tokens = lookback_tokens();
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0], "1min");
        assert_eq!(tokens[11], "1y");
    }

    #[test]
    pub fn lookback_points_ordered_ascending() {
        let mut prev = 0i64;
        for &(_, secs) in LOOKBACK_POINTS {
            assert!(secs > prev, "lookback points must be in ascending order");
            prev = secs;
        }
    }

    } // 🧫Lookback Tests
    pub use lookback_tests::*;

    mod history_unit_tests { // 💊History Unit Tests


use super::*;
    #[test]
    pub fn serialize_session_kit_has_required_fields() {
        let sid = Uuid::now_v7();
        let kid = Uuid::now_v7();
        let state = SessionState {
            session_id: SessionId(sid), domain_version: 5, semio_version: 0,
            status: SessionStatus::Active,
            kit: KitState { kit_id: kid, name: "TestKit".into(), version: Some("1.0".into()), description: Some("A test".into()), icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
            authors: BTreeMap::new(), locations: BTreeMap::new(), folders: BTreeMap::new(), files: BTreeMap::new(),
            tags: BTreeMap::new(), concepts: BTreeMap::new(), ports: BTreeMap::new(), qualities: BTreeMap::new(),
            types: BTreeMap::new(), designs: BTreeMap::new(), semio_people: BTreeMap::new(),
        };
        let kit_json = serialize_session_kit(&state);
        assert_eq!(kit_json["name"].as_str().unwrap(), "TestKit");
        assert_eq!(kit_json["version"].as_str().unwrap(), "1.0");
        assert_eq!(kit_json["description"].as_str().unwrap(), "A test");
        assert!(kit_json["types"].as_array().unwrap().is_empty());
        assert!(kit_json["designs"].as_array().unwrap().is_empty());
        assert!(kit_json["id"].as_str().is_some());
        assert!(kit_json["createdAt"].as_str().is_some());
    }

    #[test]
    pub fn session_kit_identity_helpers_require_id_and_name() {
        let id = Uuid::now_v7();
        let kit_json = serde_json::json!({
            "id": id,
            "name": "Remote Snapshot",
            "description": "transport-level baseline"
        });
        assert_eq!(session_kit_id(&kit_json).unwrap(), id);
        assert_eq!(session_kit_name(&kit_json).unwrap(), "Remote Snapshot");
        assert_eq!(session_kit_string(&kit_json, "description").as_deref(), Some("transport-level baseline"));

        let missing_id = serde_json::json!({"name": "Broken"});
        assert!(matches!(session_kit_id(&missing_id), Err(SessionError::Validation(_))));

        let missing_name = serde_json::json!({"id": id});
        assert!(matches!(session_kit_name(&missing_name), Err(SessionError::Validation(_))));
    }

    #[test]
    pub fn serialize_session_kit_with_types_and_designs() {
        let kit_json_src = load_metabolism_kit_json();
        let kid = Uuid::parse_str(kit_json_src["id"].as_str().unwrap()).unwrap();
        let mut state = SessionState {
            session_id: SessionId(Uuid::now_v7()), domain_version: 10, semio_version: 0,
            status: SessionStatus::Active,
            kit: KitState { kit_id: kid, name: "Metabolism".into(), version: None, description: None, icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
            authors: BTreeMap::new(), locations: BTreeMap::new(), folders: BTreeMap::new(), files: BTreeMap::new(),
            tags: BTreeMap::new(), concepts: BTreeMap::new(), ports: BTreeMap::new(), qualities: BTreeMap::new(),
            types: BTreeMap::new(), designs: BTreeMap::new(), semio_people: BTreeMap::new(),
        };
        // Add 3 types
        for i in 0..3 {
            let tid = Uuid::now_v7();
            state.types.insert(tid, TypeState {
                type_id: tid, name: format!("Type{}", i), parent_type_id: None, description: None,
                icon: None, image: None, folder: None, unit: None, stock: None, is_abstract: None,
                virtual_type: None, location_id: None,
                connectors: BTreeMap::new(), representations: BTreeMap::new(), props: BTreeMap::new(), lifecycle: Lifecycle::Active,
            });
        }
        let kit_json = serialize_session_kit(&state);
        assert_eq!(kit_json["types"].as_array().unwrap().len(), 3);
        assert_eq!(kit_json["name"].as_str().unwrap(), "Metabolism");
    }

    #[test]
    pub fn apply_change_log_create_type() {
        let mut kit = serde_json::json!({"id": "abc", "name": "Kit", "types": [], "designs": []});
        let type_id = Uuid::now_v7();
        let changes = serde_json::json!([{
            "op": "Created",
            "entity_kind": "type",
            "entity_id": type_id.to_string(),
            "snapshot": {"id": type_id.to_string(), "name": "NewType"}
        }]);
        apply_change_log_to_kit(&mut kit, &changes);
        let types = kit["types"].as_array().unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0]["name"].as_str().unwrap(), "NewType");
    }

    #[test]
    pub fn apply_change_log_update_kit_name() {
        let mut kit = serde_json::json!({"id": "abc", "name": "OldName", "types": []});
        let changes = serde_json::json!([{
            "op": "Updated",
            "entity_kind": "kit",
            "entity_id": "abc",
            "changed_fields": {"name": "NewName"}
        }]);
        apply_change_log_to_kit(&mut kit, &changes);
        assert_eq!(kit["name"].as_str().unwrap(), "NewName");
    }

    #[test]
    pub fn apply_change_log_delete_type() {
        let type_id = Uuid::now_v7().to_string();
        let mut kit = serde_json::json!({"id": "abc", "name": "Kit", "types": [
            {"id": type_id, "name": "ToDelete"},
            {"id": "other", "name": "Keep"}
        ]});
        let changes = serde_json::json!([{
            "op": "Deleted",
            "entity_kind": "type",
            "entity_id": type_id,
        }]);
        apply_change_log_to_kit(&mut kit, &changes);
        let types = kit["types"].as_array().unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0]["name"].as_str().unwrap(), "Keep");
    }

    #[test]
    pub fn apply_change_log_update_type_fields() {
        let type_id = Uuid::now_v7().to_string();
        let mut kit = serde_json::json!({"id": "abc", "name": "Kit", "types": [
            {"id": type_id, "name": "OldName", "description": null}
        ]});
        let changes = serde_json::json!([{
            "op": "Updated",
            "entity_kind": "type",
            "entity_id": type_id,
            "changed_fields": {"name": "NewTypeName", "description": "Updated desc"}
        }]);
        apply_change_log_to_kit(&mut kit, &changes);
        let types = kit["types"].as_array().unwrap();
        assert_eq!(types[0]["name"].as_str().unwrap(), "NewTypeName");
        assert_eq!(types[0]["description"].as_str().unwrap(), "Updated desc");
    }

    #[test]
    pub fn apply_multiple_change_logs_sequentially() {
        let mut kit = serde_json::json!({"id": "abc", "name": "Kit", "types": [], "designs": []});
        let t1 = Uuid::now_v7().to_string();
        let t2 = Uuid::now_v7().to_string();
        // First: create two types
        let changes1 = serde_json::json!([
            {"op": "Created", "entity_kind": "type", "entity_id": t1, "snapshot": {"id": t1, "name": "A"}},
            {"op": "Created", "entity_kind": "type", "entity_id": t2, "snapshot": {"id": t2, "name": "B"}},
        ]);
        apply_change_log_to_kit(&mut kit, &changes1);
        assert_eq!(kit["types"].as_array().unwrap().len(), 2);
        // Second: delete one type, update the other
        let changes2 = serde_json::json!([
            {"op": "Deleted", "entity_kind": "type", "entity_id": t1},
            {"op": "Updated", "entity_kind": "type", "entity_id": t2, "changed_fields": {"name": "B_updated"}},
        ]);
        apply_change_log_to_kit(&mut kit, &changes2);
        let types = kit["types"].as_array().unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0]["name"].as_str().unwrap(), "B_updated");
    }

    #[test]
    pub fn compaction_result_serde() {
        let r = CompactionResult { snapshots_created: 3, logs_deleted: 42 };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("snapshots_created"));
        assert!(json.contains("logs_deleted"));
    }

    } // 💊History Unit Tests
    pub use history_unit_tests::*;

    mod e2_e_testcontainer_tests { // 🌊E2E Testcontainer Tests

    /// Check if Docker/testcontainers are available at runtime.

use super::*;
    pub fn docker_available() -> bool {
        std::process::Command::new("docker").arg("info").output().map(|o| o.status.success()).unwrap_or(false)
    }

    #[tokio::test]
    async fn e2e_session_lifecycle_with_postgres() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        // Create session
        let session_id = Uuid::now_v7();
        let kit_id = Uuid::now_v7();
        create_session(&pool, session_id, kit_id, "E2E Kit").await.unwrap();

        // Verify session meta
        let (dv, sv) = load_session_meta(&pool, session_id).await.unwrap();
        assert_eq!(dv, 0);
        assert_eq!(sv, 0);

        // Verify initial baseline snapshot exists
        let snapshot = get_latest_snapshot_before(&pool, session_id, 0).await.unwrap();
        assert!(snapshot.is_some());
        let (snap_version, snap_kit) = snapshot.unwrap();
        assert_eq!(snap_version, 0);
        assert_eq!(snap_kit["name"].as_str().unwrap(), "E2E Kit");

        // Load session state
        let state = load_session_state(&pool, session_id).await.unwrap();
        assert_eq!(state.kit.name, "E2E Kit");
        assert_eq!(state.domain_version, 0);
    }

    #[tokio::test]
    async fn e2e_domain_commands_and_history() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let session_id = Uuid::now_v7();
        let kit_id = Uuid::now_v7();
        create_session(&pool, session_id, kit_id, "History Kit").await.unwrap();

        // Start actor
        let state = load_session_state(&pool, session_id).await.unwrap();
        let (cmd_tx, cmd_rx) = mpsc::channel(256);
        let (event_tx, mut event_rx) = broadcast::channel(256);
        let pool_clone = pool.clone();
        tokio::spawn(async move {
            let mut actor = SessionActor::new(state, pool_clone, event_tx);
            actor.run(cmd_rx).await;
        });

        // Send CreateType command
        let type_id = Uuid::now_v7();
        let cmd_id = Uuid::now_v7();
        let (reply_tx, reply_rx) = oneshot::channel();
        cmd_tx.send(ActorMessage::DomainCommand {
            envelope: CommandEnvelope {
                command_id: CommandId(cmd_id), client_id: ClientId(Uuid::now_v7()),
                request_id: RequestId(Uuid::now_v7()), actor_person_id: PersonId(Uuid::now_v7()),
                base_domain_version: 0,
            },
            command: DomainCommand::CreateType(CreateEntity {
                entity_id: type_id, fields: serde_json::json!({"name": "Tower"}),
            }),
            reply: reply_tx,
        }).await.unwrap();
        let result = reply_rx.await.unwrap().unwrap();
        assert!(matches!(result, CommandResult::Accepted { domain_version: 1 }));

        // Verify domain_commit was recorded
        let commit = sqlx_core::query_as::query_as::<_, (i64, Uuid)>(
            "SELECT domain_version, command_id FROM history.domain_commit WHERE session_id = $1 AND domain_version = 1"
        ).bind(session_id).fetch_optional(&pool).await.unwrap();
        assert!(commit.is_some());

        // Verify entity_change_log was recorded
        let log = sqlx_core::query_as::query_as::<_, (i64, serde_json::Value)>(
            "SELECT domain_version, changes_json FROM history.entity_change_log WHERE session_id = $1 AND domain_version = 1"
        ).bind(session_id).fetch_optional(&pool).await.unwrap();
        assert!(log.is_some());
        let (_, changes) = log.unwrap();
        let changes_arr = changes.as_array().unwrap();
        assert!(!changes_arr.is_empty());
        assert_eq!(changes_arr[0]["op"].as_str().unwrap(), "Created");

        // Reconstruct kit at version 1
        let kit_v1 = reconstruct_kit_at_version(&pool, session_id, 1).await.unwrap();
        let types = kit_v1["types"].as_array().unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0]["name"].as_str().unwrap(), "Tower");

        // Verify event was broadcast
        let event = event_rx.recv().await.unwrap();
        assert!(matches!(event, SessionEvent::DomainCommandAccepted { .. }));

        drop(cmd_tx);
    }

    #[tokio::test]
    async fn e2e_http_api_with_postgres() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let app_state = AppState::new(pool);
        let app = api::router(app_state);

        // Start server on random port
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

        let client = reqwest::Client::new();
        let base = format!("http://{}", addr);

        // Health check
        let resp = client.get(format!("{}/health", base)).send().await.unwrap();
        assert_eq!(resp.status(), 200);
        assert_eq!(resp.text().await.unwrap(), "ok");

        // Create session
        let resp = client.post(format!("{}/sessions", base))
            .json(&serde_json::json!({"kit_name": "API Test Kit"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await.unwrap();
        let session_id = body["session_id"].as_str().unwrap();
        let _kit_id = body["kit_id"].as_str().unwrap();
        let owner_token = body["owner_token"].as_str().unwrap().to_string();

        // Get snapshot (read-only, no auth required)
        let resp = client.get(format!("{}/sessions/{}/snapshot", base, session_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let snapshot: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(snapshot["domain_version"].as_i64().unwrap(), 0);

        // Get lookback tokens
        let resp = client.get(format!("{}/sessions/{}/history/lookback-tokens", base, session_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let tokens: Vec<String> = resp.json().await.unwrap();
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0], "1min");

        // Send domain command (requires owner token)
        let cmd_id = Uuid::now_v7();
        let type_id = Uuid::now_v7();
        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": cmd_id, "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 0,
                "kind": "CreateType",
                "payload": {"entity_id": type_id, "fields": {"name": "APIType"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let result: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(result["status"].as_str().unwrap(), "Accepted");
        assert_eq!(result["domain_version"].as_i64().unwrap(), 1);

        // Get kit at version 1
        let resp = client.get(format!("{}/sessions/{}/kit/at-version/1", base, session_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let kit: serde_json::Value = resp.json().await.unwrap();
        let types = kit["types"].as_array().unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0]["name"].as_str().unwrap(), "APIType");

        // Compact history (requires owner token)
        let resp = client.post(format!("{}/sessions/{}/history/compact", base, session_id))
            .bearer_auth(&owner_token)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
    }

    #[tokio::test]
    async fn e2e_metabolism_full_kit_history() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let session_id = Uuid::now_v7();
        let kit_json = load_metabolism_kit_json();
        let kit_id = Uuid::parse_str(kit_json["id"].as_str().unwrap()).unwrap();
        create_session(&pool, session_id, kit_id, "Metabolism").await.unwrap();

        let state = load_session_state(&pool, session_id).await.unwrap();
        let (cmd_tx, cmd_rx) = mpsc::channel(256);
        let (event_tx, _) = broadcast::channel(256);
        let pool_clone = pool.clone();
        tokio::spawn(async move {
            let mut actor = SessionActor::new(state, pool_clone, event_tx);
            actor.run(cmd_rx).await;
        });

        // Create all 50 types via batch
        let types_json = kit_json["types"].as_array().unwrap();
        let mut commands: Vec<DomainCommand> = Vec::new();
        for t in types_json {
            let id = Uuid::parse_str(t["id"].as_str().unwrap()).unwrap();
            commands.push(DomainCommand::CreateType(CreateEntity {
                entity_id: id, fields: serde_json::json!({"name": t["name"]}),
            }));
        }
        let batch = DomainCommand::Batch(DomainBatch { commands });
        let (reply_tx, reply_rx) = oneshot::channel();
        cmd_tx.send(ActorMessage::DomainCommand {
            envelope: CommandEnvelope {
                command_id: CommandId(Uuid::now_v7()), client_id: ClientId(Uuid::now_v7()),
                request_id: RequestId(Uuid::now_v7()), actor_person_id: PersonId(Uuid::now_v7()),
                base_domain_version: 0,
            },
            command: batch,
            reply: reply_tx,
        }).await.unwrap();
        let result = reply_rx.await.unwrap().unwrap();
        assert!(matches!(result, CommandResult::Accepted { domain_version: 1 }));

        // Verify kit at version 1 has all 50 types
        let kit_v1 = reconstruct_kit_at_version(&pool, session_id, 1).await.unwrap();
        let reconstructed_types = kit_v1["types"].as_array().unwrap();
        assert_eq!(reconstructed_types.len(), 50, "reconstructed kit at v1 should have 50 types");

        // Verify baseline at version 0 has 0 types
        let kit_v0 = reconstruct_kit_at_version(&pool, session_id, 0).await.unwrap();
        let empty_vec = vec![];
        let v0_types = kit_v0["types"].as_array().unwrap_or(&empty_vec);
        assert_eq!(v0_types.len(), 0, "baseline at v0 should have 0 types");

        drop(cmd_tx);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn e2e_multi_frontend_websocket() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let app_state = AppState::new(pool.clone());
        let app = api::router(app_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

        let client = reqwest::Client::new();
        let base = format!("http://{}", addr);

        // Create session
        let resp = client.post(format!("{}/sessions", base))
            .json(&serde_json::json!({"kit_name": "WS Test Kit"}))
            .send().await.unwrap();
        let body: serde_json::Value = resp.json().await.unwrap();
        let session_id = body["session_id"].as_str().unwrap().to_string();
        let owner_token = body["owner_token"].as_str().unwrap().to_string();

        // Connect two WebSocket frontends
        let ws_url = format!("ws://{}/sessions/{}/ws", addr, session_id);
        let (ws1, _) = tokio_tungstenite::connect_async(&ws_url).await.expect("ws1 connect");
        let (ws2, _) = tokio_tungstenite::connect_async(&ws_url).await.expect("ws2 connect");
        let (mut _ws1_write, mut ws1_read) = ws1.split();
        let (mut _ws2_write, mut ws2_read) = ws2.split();

        // Wait for connection acknowledgment from both WebSocket handlers
        let ack1 = tokio::time::timeout(std::time::Duration::from_secs(5), ws1_read.next()).await;
        assert!(ack1.is_ok(), "ws1 should receive connection ack");
        let ack2 = tokio::time::timeout(std::time::Duration::from_secs(5), ws2_read.next()).await;
        assert!(ack2.is_ok(), "ws2 should receive connection ack");

        // Send domain command via HTTP (with owner token)
        let cmd_id = Uuid::now_v7();
        let type_id = Uuid::now_v7();
        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": cmd_id, "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 0,
                "kind": "CreateType",
                "payload": {"entity_id": type_id, "fields": {"name": "WSType"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);

        // Both WebSocket frontends should receive the event
        let msg1 = tokio::time::timeout(std::time::Duration::from_secs(5), ws1_read.next()).await;
        let msg2 = tokio::time::timeout(std::time::Duration::from_secs(5), ws2_read.next()).await;
        assert!(msg1.is_ok(), "ws1 should receive event");
        assert!(msg2.is_ok(), "ws2 should receive event");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn e2e_snapshot_and_piece_patch_roundtrip() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let app_state = AppState::new(pool.clone());
        let app = api::router(app_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

        let client = reqwest::Client::new();
        let base = format!("http://{}", addr);

        let create_resp = client
            .post(format!("{}/sessions", base))
            .json(&serde_json::json!({"kit_name": "Roundtrip Kit"}))
            .send()
            .await
            .unwrap();
        let create_body: serde_json::Value = create_resp.json().await.unwrap();
        let session_id = create_body["session_id"].as_str().unwrap();
        let owner_token = create_body["owner_token"].as_str().unwrap().to_string();

        let design_id = Uuid::now_v7();
        let piece_id = Uuid::now_v7();

        let create_design = client
            .post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(),
                "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(),
                "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 0,
                "kind": "CreateDesign",
                "payload": {"entity_id": design_id, "fields": {"id": design_id, "name": "Remote Design"}}
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(create_design.status(), 200);

        let create_piece = client
            .post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(),
                "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(),
                "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 1,
                "kind": "CreatePiece",
                "payload": {"piece_id": piece_id, "design_id": design_id, "fields": {"id": piece_id, "design_id": design_id, "name": "Remote Piece"}}
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(create_piece.status(), 200);

        let patch_piece = client
            .post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(),
                "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(),
                "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 2,
                "kind": "PatchPiece",
                "payload": {"entity_id": piece_id, "fields": {"design_id": design_id, "center": {"u": 12.5, "v": -4.25}}}
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(patch_piece.status(), 200);

        let snapshot_resp = client.get(format!("{}/sessions/{}/snapshot", base, session_id)).send().await.unwrap();
        assert_eq!(snapshot_resp.status(), 200);
        let snapshot: serde_json::Value = snapshot_resp.json().await.unwrap();
        assert_eq!(snapshot["kit"]["designs"][0]["name"].as_str().unwrap(), "Remote Design");
        assert_eq!(snapshot["kit"]["designs"][0]["pieces"][0]["center"]["u"].as_f64().unwrap(), 12.5);
        assert_eq!(snapshot["kit"]["designs"][0]["pieces"][0]["center"]["v"].as_f64().unwrap(), -4.25);
    }

    #[tokio::test]
    async fn e2e_auth_forbidden_without_token() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let app_state = AppState::new(pool);
        let app = api::router(app_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

        let client = reqwest::Client::new();
        let base = format!("http://{}", addr);

        // Create session — returns owner_token
        let resp = client.post(format!("{}/sessions", base))
            .json(&serde_json::json!({"kit_name": "Auth Test Kit"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await.unwrap();
        let session_id = body["session_id"].as_str().unwrap();
        let owner_token = body["owner_token"].as_str().unwrap().to_string();
        assert!(!owner_token.is_empty());

        // Read endpoints should work without token
        let resp = client.get(format!("{}/sessions/{}/snapshot", base, session_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);

        // Domain command without token should be forbidden
        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 0,
                "kind": "CreateType",
                "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "NoAuth"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 403);

        // Domain command with wrong token should fail
        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(Uuid::now_v7().to_string())
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 0,
                "kind": "CreateType",
                "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "WrongAuth"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 401);

        // Domain command with correct token should succeed
        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 0,
                "kind": "CreateType",
                "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "AuthOk"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);

        // Semio command without token should be forbidden
        let resp = client.post(format!("{}/sessions/{}/commands/semio", base, session_id))
            .json(&serde_json::json!({
                "client_id": Uuid::now_v7(), "person_id": Uuid::now_v7(),
                "frontend_id": "test", "base_semio_version": 0,
                "kind": "UpsertCursor", "payload": {"u": 0.5, "v": 0.5}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 403);

        // Compact without token should be forbidden
        let resp = client.post(format!("{}/sessions/{}/history/compact", base, session_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 403);
    }

    #[tokio::test]
    async fn e2e_share_token_flow() {
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let app_state = AppState::new(pool);
        let app = api::router(app_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

        let client = reqwest::Client::new();
        let base = format!("http://{}", addr);

        // Create session
        let resp = client.post(format!("{}/sessions", base))
            .json(&serde_json::json!({"kit_name": "Share Test Kit"}))
            .send().await.unwrap();
        let body: serde_json::Value = resp.json().await.unwrap();
        let session_id = body["session_id"].as_str().unwrap().to_string();
        let owner_token = body["owner_token"].as_str().unwrap().to_string();

        // Create a type with owner token first
        let type_id = Uuid::now_v7();
        let design_id = Uuid::now_v7();
        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 0,
                "kind": "CreateType",
                "payload": {"entity_id": type_id, "fields": {"name": "SharedType"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);

        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 1,
                "kind": "CreateDesign",
                "payload": {"entity_id": design_id, "fields": {"name": "SharedDesign"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);

        // Create share without owner token should fail
        let resp = client.post(format!("{}/sessions/{}/shares", base, session_id))
            .json(&serde_json::json!({"label": "kit share"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 403);

        // Create viewer share for kit
        let resp = client.post(format!("{}/sessions/{}/shares", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({"access_mode": "viewer", "label": "Kit Read-Only"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let share: serde_json::Value = resp.json().await.unwrap();
        let kit_share_token = share["token"].as_str().unwrap().to_string();
        assert_eq!(share["access_mode"].as_str().unwrap(), "viewer");
        assert_eq!(share["label"].as_str().unwrap(), "Kit Read-Only");

        // Create viewer share for specific type
        let resp = client.post(format!("{}/sessions/{}/shares", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({"access_mode": "viewer", "entity_kind": "type", "entity_id": type_id, "label": "Type Share"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let type_share: serde_json::Value = resp.json().await.unwrap();
        let type_share_token = type_share["token"].as_str().unwrap().to_string();

        // Create viewer share for specific design
        let resp = client.post(format!("{}/sessions/{}/shares", base, session_id))
            .bearer_auth(&owner_token)
            .json(&serde_json::json!({"access_mode": "viewer", "entity_kind": "design", "entity_id": design_id, "label": "Design Share"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let design_share: serde_json::Value = resp.json().await.unwrap();
        let _design_share_token = design_share["token"].as_str().unwrap().to_string();

        // Resolve kit share token
        let resp = client.get(format!("{}/shares/{}", base, kit_share_token))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let resolved: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(resolved["session_id"].as_str().unwrap(), session_id);
        assert_eq!(resolved["access_mode"].as_str().unwrap(), "viewer");
        assert!(resolved["entity_kind"].is_null());

        // Resolve type share token
        let resp = client.get(format!("{}/shares/{}", base, type_share_token))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let resolved: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(resolved["entity_kind"].as_str().unwrap(), "type");
        assert_eq!(resolved["entity_id"].as_str().unwrap(), type_id.to_string());

        // Viewer share token should allow reading snapshot
        let resp = client.get(format!("{}/sessions/{}/snapshot", base, session_id))
            .bearer_auth(&kit_share_token)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);

        // Viewer share token should NOT allow mutations
        let resp = client.post(format!("{}/sessions/{}/commands/domain", base, session_id))
            .bearer_auth(&kit_share_token)
            .json(&serde_json::json!({
                "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                "base_domain_version": 2,
                "kind": "CreateType",
                "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "ShouldFail"}}
            }))
            .send().await.unwrap();
        assert_eq!(resp.status(), 403);

        // List shares (requires owner)
        let resp = client.get(format!("{}/sessions/{}/shares", base, session_id))
            .bearer_auth(&owner_token)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let shares: Vec<serde_json::Value> = resp.json().await.unwrap();
        assert_eq!(shares.len(), 3, "should have 3 share tokens");

        // Delete a share
        let resp = client.delete(format!("{}/sessions/{}/shares/{}", base, session_id, kit_share_token))
            .bearer_auth(&owner_token)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let del_body: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(del_body["deleted"].as_bool().unwrap(), true);

        // Deleted share token should no longer resolve
        let resp = client.get(format!("{}/shares/{}", base, kit_share_token))
            .send().await.unwrap();
        assert_eq!(resp.status(), 404);

        // List shares should now show 2
        let resp = client.get(format!("{}/sessions/{}/shares", base, session_id))
            .bearer_auth(&owner_token)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let shares: Vec<serde_json::Value> = resp.json().await.unwrap();
        assert_eq!(shares.len(), 2, "should have 2 share tokens after deletion");
    }

    #[test]
    pub fn require_admin_rejects_without_token_set() {
        // Specs: When SEMIO_ADMIN_TOKEN is unset, require_admin returns Forbidden so /admin/* never leaks in misconfigured deployments.
        let cfg = AdminConfig { admin_token: None, started_at: Arc::new(Instant::now()) };
        let headers = axum::http::HeaderMap::new();
        let err = require_admin(&headers, &cfg).unwrap_err();
        assert_eq!(err.into_response().status(), StatusCode::FORBIDDEN);
    }

    #[test]
    pub fn require_admin_rejects_missing_bearer() {
        let cfg = AdminConfig { admin_token: Some("s3cret".into()), started_at: Arc::new(Instant::now()) };
        let headers = axum::http::HeaderMap::new();
        let err = require_admin(&headers, &cfg).unwrap_err();
        assert_eq!(err.into_response().status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    pub fn require_admin_rejects_wrong_bearer() {
        let cfg = AdminConfig { admin_token: Some("s3cret".into()), started_at: Arc::new(Instant::now()) };
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", axum::http::HeaderValue::from_static("Bearer nope"));
        let err = require_admin(&headers, &cfg).unwrap_err();
        assert_eq!(err.into_response().status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    pub fn require_admin_accepts_correct_bearer() {
        let cfg = AdminConfig { admin_token: Some("s3cret".into()), started_at: Arc::new(Instant::now()) };
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", axum::http::HeaderValue::from_static("Bearer s3cret"));
        require_admin(&headers, &cfg).unwrap();
    }

    #[test]
    pub fn session_directory_tracks_active_connections() {
        // Specs: active_connections counter and activated_at are initialized on actor creation and observable through list_active.
        let handle = SessionHandle {
            command_tx: tokio::sync::mpsc::channel(1).0,
            event_tx: tokio::sync::broadcast::channel(1).0,
            active_connections: Arc::new(AtomicUsize::new(0)),
            activated_at: Arc::new(Instant::now()),
        };
        handle.active_connections.fetch_add(3, AtomicOrdering::Relaxed);
        assert_eq!(handle.active_connections.load(AtomicOrdering::Relaxed), 3);
        handle.active_connections.fetch_sub(1, AtomicOrdering::Relaxed);
        assert_eq!(handle.active_connections.load(AtomicOrdering::Relaxed), 2);
    }

    #[tokio::test]
    async fn e2e_admin_endpoints_with_postgres() {
        // Specs: Full round-trip against the embedded admin router: overview, session list, kit list, share-token list, session detail, compaction config read/write, and auth boundary.
        if !docker_available() {
            eprintln!("[SKIP] Docker not available, skipping admin E2E test");
            return;
        }

        let pg = Postgres::default().start().await.expect("start postgres container");
        let host_port = pg.get_host_port_ipv4(5432).await.expect("get postgres port");
        let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);

        let pool = create_pool(&db_url).await;
        run_migrations(&pool).await;

        let app_state = AppState::new(pool.clone());
        let admin_config = AdminConfig { admin_token: Some("test-admin-token".into()), started_at: Arc::new(Instant::now()) };
        let admin_state = AdminState { pool: pool.clone(), directory: app_state.directory.clone(), config: admin_config };
        let app = api::router(app_state).merge(admin::router(admin_state));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

        let client = reqwest::Client::new();
        let base = format!("http://{}", addr);

        // Seed: create two sessions via public HTTP API.
        let resp_a = client.post(format!("{}/sessions", base))
            .json(&serde_json::json!({"kit_name": "Admin Kit A"}))
            .send().await.unwrap();
        assert_eq!(resp_a.status(), 200);
        let body_a: serde_json::Value = resp_a.json().await.unwrap();
        let session_a = body_a["session_id"].as_str().unwrap().to_string();
        let owner_a = body_a["owner_token"].as_str().unwrap().to_string();

        let resp_b = client.post(format!("{}/sessions", base))
            .json(&serde_json::json!({"kit_name": "Admin Kit B"}))
            .send().await.unwrap();
        assert_eq!(resp_b.status(), 200);
        let body_b: serde_json::Value = resp_b.json().await.unwrap();
        let session_b = body_b["session_id"].as_str().unwrap().to_string();

        // Seed a share token for session A so admin share-token endpoints have data.
        let share_resp = client.post(format!("{}/sessions/{}/shares", base, session_a))
            .bearer_auth(&owner_a)
            .json(&serde_json::json!({"access_mode": "viewer", "label": "demo"}))
            .send().await.unwrap();
        assert_eq!(share_resp.status(), 200);
        let share_body: serde_json::Value = share_resp.json().await.unwrap();
        let share_token = share_body["token"].as_str().unwrap().to_string();

        // Admin without token -> 401
        let resp = client.get(format!("{}/admin/overview", base)).send().await.unwrap();
        assert_eq!(resp.status(), 401);

        // Admin with wrong token -> 401
        let resp = client.get(format!("{}/admin/overview", base))
            .bearer_auth("wrong").send().await.unwrap();
        assert_eq!(resp.status(), 401);

        // Overview
        let resp = client.get(format!("{}/admin/overview", base))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let overview: serde_json::Value = resp.json().await.unwrap();
        assert!(overview["total_sessions"].as_i64().unwrap() >= 2);
        assert!(overview["total_kits"].as_i64().unwrap() >= 2);
        assert!(overview["total_share_tokens"].as_i64().unwrap() >= 1);

        // List sessions
        let resp = client.get(format!("{}/admin/sessions", base))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let sessions: Vec<serde_json::Value> = resp.json().await.unwrap();
        assert!(sessions.iter().any(|s| s["session_id"].as_str() == Some(&session_a)));
        assert!(sessions.iter().any(|s| s["session_id"].as_str() == Some(&session_b)));

        // Session detail for A
        let resp = client.get(format!("{}/admin/sessions/{}", base, session_a))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let detail: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(detail["row"]["session_id"].as_str().unwrap(), session_a);
        assert_eq!(detail["kit"]["name"].as_str().unwrap(), "Admin Kit A");
        assert_eq!(detail["share_tokens"].as_array().unwrap().len(), 1);

        // List kits
        let resp = client.get(format!("{}/admin/kits", base))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let kits: Vec<serde_json::Value> = resp.json().await.unwrap();
        let names: Vec<&str> = kits.iter().map(|k| k["name"].as_str().unwrap()).collect();
        assert!(names.contains(&"Admin Kit A"));
        assert!(names.contains(&"Admin Kit B"));

        // List share tokens
        let resp = client.get(format!("{}/admin/share-tokens", base))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let tokens: Vec<serde_json::Value> = resp.json().await.unwrap();
        assert!(tokens.iter().any(|t| t["token"].as_str() == Some(&share_token)));

        // Revoke share token
        let resp = client.delete(format!("{}/admin/share-tokens/{}", base, share_token))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let revoke: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(revoke["revoked"].as_bool().unwrap(), true);

        // List persons (empty but should return 200)
        let resp = client.get(format!("{}/admin/persons", base))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let _persons: Vec<serde_json::Value> = resp.json().await.unwrap();

        // Connections endpoint
        let resp = client.get(format!("{}/admin/connections", base))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let _connections: Vec<serde_json::Value> = resp.json().await.unwrap();

        // Compaction config: GET returns defaults, PATCH updates, GET reflects update.
        let resp = client.get(format!("{}/admin/config/{}", base, session_a))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let cfg: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(cfg["lookback_tokens"].as_array().unwrap().len(), 12);

        let resp = client.patch(format!("{}/admin/config/{}", base, session_a))
            .bearer_auth("test-admin-token")
            .json(&serde_json::json!({"lookback_tokens": ["1min", "1h", "1d"]}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let cfg: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(cfg["lookback_tokens"].as_array().unwrap().len(), 3);

        // Invalid lookback token -> 400
        let resp = client.patch(format!("{}/admin/config/{}", base, session_a))
            .bearer_auth("test-admin-token")
            .json(&serde_json::json!({"lookback_tokens": ["not-a-real-token"]}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 400);

        // Close session B
        let resp = client.post(format!("{}/admin/sessions/{}/close", base, session_b))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let close: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(close["closed"].as_bool().unwrap(), true);

        // After close, detail should report status = closed.
        let resp = client.get(format!("{}/admin/sessions/{}", base, session_b))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let detail: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(detail["row"]["status"].as_str().unwrap(), "closed");

        // Passivate a session that was never activated -> still returns 200 (idempotent).
        let resp = client.post(format!("{}/admin/sessions/{}/passivate", base, session_a))
            .bearer_auth("test-admin-token").send().await.unwrap();
        assert_eq!(resp.status(), 200);

        // Embedded dashboard is served without auth (static HTML; endpoints behind auth).
        let resp = client.get(format!("{}/admin", base)).send().await.unwrap();
        assert_eq!(resp.status(), 200);
        let ctype = resp.headers().get("content-type").unwrap().to_str().unwrap().to_string();
        assert!(ctype.starts_with("text/html"));
        let html = resp.text().await.unwrap();
        assert!(html.contains("semio"));
        assert!(html.contains("overview"));
    }

    } // 🌊E2E Testcontainer Tests
    pub use e2_e_testcontainer_tests::*;
} // 📐Tests
