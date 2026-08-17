mod header { // 🧲️Header
             // 2026 Ueli Saluz <ueli@semio_compose_rs-tech.de>
             // AGPL-3.0
             // Specs: Single-binary session-backend consolidating domain, command, event, state, error, store, actor, directory, API, WS, and admin modules.
             // Summary: Consolidated session-backend service for semio_compose_rs. `db`-backed (db::Database owns WAL/conflict/durability), single-writer actor per session, HTTP+WS (protocol_wire v2) API with axum, in-memory state with typed entity structs.
} // 🧲️Header

pub use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
pub use axum::extract::{Path, State};
pub use axum::http::StatusCode;
pub use axum::response::{IntoResponse, Response};
pub use axum::routing::{get, post, put};
pub use axum::{Json, Router};
pub use dashmap::DashMap;
pub use futures::{SinkExt, StreamExt};
pub use serde::{Deserialize, Serialize};
pub use std::collections::BTreeMap;
pub use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
pub use std::sync::Arc;
pub use std::time::Instant;
pub use thiserror::Error;
pub use tokio::sync::{broadcast, mpsc, oneshot};
pub use uuid::Uuid;

mod domain {
    // 🗿️Domain
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
    pub type ComposeVersion = i64;

    mod field_patch {
        // 📭️FieldPatch

        use super::*;
        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        #[serde(tag = "operation", content = "value")]
        pub enum FieldPatch<T> {
            #[default]
            NoChange,
            Set(T),
            Clear,
        }

        impl<T> FieldPatch<T> {
            pub fn is_change(&self) -> bool {
                !matches!(self, Self::NoChange)
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        #[serde(tag = "operation", content = "value")]
        pub enum RequiredFieldPatch<T> {
            #[default]
            NoChange,
            Set(T),
        }

        impl<T> RequiredFieldPatch<T> {
            pub fn is_change(&self) -> bool {
                !matches!(self, Self::NoChange)
            }
        }
    } // 📭️FieldPatch
    pub use field_patch::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum EntityKind {
        Kit,
        Author,
        Location,
        Folder,
        File,
        Tag,
        Concept,
        Port,
        Quality,
        Benchmark,
        Type,
        Representation,
        Connector,
        Prop,
        Attribute,
        Design,
        Layer,
        Piece,
        Group,
        Connection,
        Stat,
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
        ComposeLastWriterWins,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PropertyKey {
        KitName,
        KitVersion,
        KitDescription,
        KitIcon,
        KitImage,
        KitPreview,
        KitRemote,
        KitHomepage,
        KitLicense,
        TypeName,
        TypeParent,
        TypeDescription,
        TypeIcon,
        TypeImage,
        TypeFolder,
        TypeUnit,
        TypeStock,
        TypeIsAbstract,
        TypeVirtual,
        TypeLocation,
        DesignName,
        DesignParent,
        DesignDescription,
        DesignIcon,
        DesignImage,
        DesignFolder,
        DesignUnit,
        DesignIsAbstract,
        DesignCanScale,
        DesignCanMirror,
        DesignActiveLayer,
        DesignLocation,
        PieceName,
        PieceType,
        PieceDesign,
        PiecePlane,
        PieceCenter,
        PieceScale,
        PieceMirrorPlane,
        PieceIsHidden,
        PieceIsLocked,
        PieceColor,
        PieceDescription,
        ConnectionConnected,
        ConnectionConnecting,
        ConnectionGap,
        ConnectionShift,
        ConnectionRise,
        ConnectionRotation,
        ConnectionTurn,
        ConnectionTilt,
        ConnectionU,
        ConnectionV,
        ConnectionDescription,
        AuthorName,
        AuthorEmail,
        FolderName,
        FolderParent,
        FolderDescription,
        FileName,
        FileRemote,
        FileFolder,
        FileBlob,
        TagName,
        TagDescription,
        TagIcon,
        ConceptName,
        ConceptDescription,
        ConceptIcon,
        PortName,
        PortDescription,
        PortIcon,
        QualityKey,
        QualityName,
        QualityDescription,
        LayerPath,
        LayerIsHidden,
        LayerIsLocked,
        LayerColor,
        LayerDescription,
        GroupName,
        GroupColor,
        GroupDescription,
        EntityLifecycle,
    }

    pub fn conflict_policy(key: PropertyKey) -> ConflictPolicy {
        match key {
            PropertyKey::KitName => ConflictPolicy::RejectIfChanged,
            PropertyKey::PieceType | PropertyKey::PieceDesign => ConflictPolicy::ReferenceMustExistAndBeActive,
            PropertyKey::TypeParent | PropertyKey::DesignParent | PropertyKey::FolderParent | PropertyKey::DesignActiveLayer | PropertyKey::TypeLocation | PropertyKey::DesignLocation | PropertyKey::FileFolder => {
                ConflictPolicy::ReferenceMustExistAndBeActive
            }
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
} // 🗿️Domain
pub use domain::*;

mod lookback {
    // 🎏️Lookback
    // Specs: Named lookback points define retention boundaries for kit history. Each token maps to seconds.
    // Summary: Configurable lookback points for historical kit snapshot retention and auto-compaction.

    pub const LOOKBACK_POINTS: &[(&str, i64)] = &[("1min", 60), ("5min", 300), ("10min", 600), ("30min", 1800), ("1h", 3600), ("5h", 18000), ("1d", 86400), ("3d", 259200), ("7d", 604800), ("1mo", 2592000), ("6mo", 15552000), ("1y", 31536000)];

    pub fn lookback_seconds(token: &str) -> Option<i64> {
        LOOKBACK_POINTS.iter().find(|(t, _)| *t == token).map(|(_, s)| *s)
    }

    pub fn lookback_tokens() -> Vec<&'static str> {
        LOOKBACK_POINTS.iter().map(|(t, _)| *t).collect()
    }
} // 🎏️Lookback
pub use lookback::*;

mod command {
    // 🪆️Command
    // Specs: CommandEnvelope carries per-command metadata. DomainCommand enumerates all CRUD variants. ComposeCommand handles presence mutations. CommandResult reports outcome.
    // Summary: Explicit command types for domain and semio_compose_rs mutations.

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
        CreateType(CreateEntity),
        PatchType(PatchEntity),
        DeleteType(DeleteEntity),
        CreateDesign(CreateEntity),
        PatchDesign(PatchEntity),
        DeleteDesign(DeleteEntity),
        CreatePiece(CreatePiece),
        PatchPiece(PatchEntity),
        DeletePiece(DeleteEntity),
        CreateConnection(CreateConnection),
        PatchConnection(PatchEntity),
        DeleteConnection(DeleteEntity),
        CreateLayer(CreateEntity),
        PatchLayer(PatchEntity),
        DeleteLayer(DeleteEntity),
        CreateGroup(CreateEntity),
        PatchGroup(PatchEntity),
        DeleteGroup(DeleteEntity),
        CreateAuthor(CreateEntity),
        PatchAuthor(PatchEntity),
        DeleteAuthor(DeleteEntity),
        CreateTag(CreateEntity),
        PatchTag(PatchEntity),
        DeleteTag(DeleteEntity),
        CreateConcept(CreateEntity),
        PatchConcept(PatchEntity),
        DeleteConcept(DeleteEntity),
        CreatePort(CreateEntity),
        PatchPort(PatchEntity),
        DeletePort(DeleteEntity),
        CreateQuality(CreateEntity),
        PatchQuality(PatchEntity),
        DeleteQuality(DeleteEntity),
        CreateFolder(CreateEntity),
        PatchFolder(PatchEntity),
        DeleteFolder(DeleteEntity),
        CreateFile(CreateEntity),
        PatchFile(PatchEntity),
        DeleteFile(DeleteEntity),
        Batch(DomainBatch),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DomainBatch {
        pub commands: Vec<DomainCommand>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PatchKit {
        pub fields: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateEntity {
        pub entity_id: Uuid,
        pub fields: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PatchEntity {
        pub entity_id: Uuid,
        pub fields: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteEntity {
        pub entity_id: Uuid,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreatePiece {
        pub piece_id: Uuid,
        pub design_id: Uuid,
        pub fields: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateConnection {
        pub connection_id: Uuid,
        pub design_id: Uuid,
        pub fields: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ComposeEnvelope {
        pub client_id: ClientId,
        pub person_id: PersonId,
        pub frontend_id: String,
        pub base_compose_version: ComposeVersion,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "kind", content = "payload")]
    pub enum ComposeCommand {
        UpsertCursor(UpsertCursor),
        UpsertLook(UpsertLook),
        SetSelection(SetSelection),
        ClearPresence(ClearPresence),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpsertCursor {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpsertLook {
        pub position: [f64; 3],
        pub forward: [f64; 3],
        pub up: [f64; 3],
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SetSelection {
        pub piece_ids: Vec<Uuid>,
        pub design_ids: Vec<Uuid>,
    }

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
} // 🪆️Command
pub use command::*;

mod event {
    // 🏗️Event
    // Specs: SessionEvent enumerates all broadcastable events. EntityChange describes domain mutations. ComposeUpdate describes semio_compose_rs state changes.
    // Summary: Broadcast event types for domain and semio_compose_rs state changes.

    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "event")]
    pub enum SessionEvent {
        DomainCommandAccepted { command_id: CommandId, domain_version: DomainVersion, changes: Vec<EntityChange> },
        DomainCommandRejected { command_id: CommandId, conflicts: Vec<ConflictDetail> },
        ComposeUpdated { compose_version: ComposeVersion, person_id: PersonId, frontend_id: String, update: ComposeUpdate },
        SessionClosed,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "operation")]
    pub enum EntityChange {
        Created { entity_kind: EntityKind, entity_id: Uuid, snapshot: serde_json::Value },
        Updated { entity_kind: EntityKind, entity_id: Uuid, changed_fields: serde_json::Value },
        Deleted { entity_kind: EntityKind, entity_id: Uuid },
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "kind")]
    pub enum ComposeUpdate {
        CursorMoved { u: f64, v: f64 },
        LookChanged { position: [f64; 3], forward: [f64; 3], up: [f64; 3] },
        SelectionChanged { piece_ids: Vec<Uuid>, design_ids: Vec<Uuid> },
        PresenceCleared,
    }
} // 🏗️Event
pub use event::*;

mod state {
    // 🖋️State
    // Specs: SessionState holds full typed in-memory state for one session. Entity states mirror canonical DB rows.
    // Summary: In-memory session state loaded from and persisted to PostgreSQL.

    use super::*;
    #[derive(Debug, Clone)]
    pub struct SessionState {
        pub session_id: SessionId,
        pub domain_version: DomainVersion,
        pub compose_version: ComposeVersion,
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
        pub compose_people: BTreeMap<(Uuid, String), ComposePersonState>,
    }

    #[derive(Debug, Clone)]
    pub struct KitState {
        pub kit_id: Uuid,
        pub name: String,
        pub version: Option<String>,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub preview: Option<String>,
        pub remote: Option<String>,
        pub homepage: Option<String>,
        pub license: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct AuthorState {
        pub author_id: Uuid,
        pub name: String,
        pub email: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct LocationState {
        pub location_id: Uuid,
        pub name: String,
        pub description: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct FolderState {
        pub folder_id: Uuid,
        pub name: String,
        pub parent_folder_id: Option<Uuid>,
        pub description: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct FileState {
        pub file_id: Uuid,
        pub name: String,
        pub remote: Option<String>,
        pub folder_id: Option<Uuid>,
        pub size: Option<i64>,
        pub hash: Option<String>,
        pub blob: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct TagState {
        pub tag_id: Uuid,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct ConceptState {
        pub concept_id: Uuid,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct PortState {
        pub port_id: Uuid,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub max_children: Option<i32>,
        pub compatible_port_ids: Vec<Uuid>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct QualityState {
        pub quality_id: Uuid,
        pub key: String,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub unit: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct TypeState {
        pub type_id: Uuid,
        pub name: String,
        pub parent_type_id: Option<Uuid>,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub folder: Option<String>,
        pub unit: Option<String>,
        pub stock: Option<i32>,
        pub is_abstract: Option<bool>,
        pub virtual_type: Option<bool>,
        pub location_id: Option<Uuid>,
        pub connectors: BTreeMap<Uuid, ConnectorState>,
        pub representations: BTreeMap<Uuid, RepresentationState>,
        pub props: BTreeMap<Uuid, PropState>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct ConnectorState {
        pub connector_id: Uuid,
        pub name: Option<String>,
        pub t: f64,
        pub point: [f64; 3],
        pub direction: [f64; 3],
        pub description: Option<String>,
        pub port_id: Option<Uuid>,
        pub mandatory: Option<bool>,
        pub max_children: Option<i32>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct RepresentationState {
        pub representation_id: Uuid,
        pub file_id: Uuid,
        pub name: Option<String>,
        pub description: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct PropState {
        pub prop_id: Uuid,
        pub quality_id: Uuid,
        pub value: String,
        pub unit: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct DesignState {
        pub design_id: Uuid,
        pub name: String,
        pub parent_design_id: Option<Uuid>,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub folder: Option<String>,
        pub unit: Option<String>,
        pub is_abstract: Option<bool>,
        pub can_scale: Option<bool>,
        pub can_mirror: Option<bool>,
        pub active_layer_id: Option<Uuid>,
        pub location_id: Option<Uuid>,
        pub pieces: BTreeMap<Uuid, PieceState>,
        pub connections: BTreeMap<Uuid, ConnectionState>,
        pub layers: BTreeMap<Uuid, LayerState>,
        pub groups: BTreeMap<Uuid, GroupState>,
        pub stats: BTreeMap<Uuid, StatState>,
        pub props: BTreeMap<Uuid, PropState>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct PieceState {
        pub piece_id: Uuid,
        pub name: Option<String>,
        pub type_id: Option<Uuid>,
        pub design_ref_id: Option<Uuid>,
        pub plane: Option<PlaneState>,
        pub center: Option<[f64; 2]>,
        pub scale: Option<f64>,
        pub mirror_plane: Option<PlaneState>,
        pub is_hidden: Option<bool>,
        pub is_locked: Option<bool>,
        pub color: Option<String>,
        pub description: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct PlaneState {
        pub origin: [f64; 3],
        pub x_axis: [f64; 3],
        pub y_axis: [f64; 3],
    }

    #[derive(Debug, Clone)]
    pub struct ConnectionState {
        pub connection_id: Uuid,
        pub parent_piece_id: Uuid,
        pub parent_design_piece_id: Option<Uuid>,
        pub parent_connector_id: Option<Uuid>,
        pub child_piece_id: Uuid,
        pub child_design_piece_id: Option<Uuid>,
        pub child_connector_id: Option<Uuid>,
        pub gap: f64,
        pub shift: f64,
        pub rise: f64,
        pub rotation: f64,
        pub turn: f64,
        pub tilt: f64,
        pub u: Option<f64>,
        pub v: Option<f64>,
        pub description: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct LayerState {
        pub layer_id: Uuid,
        pub path: String,
        pub is_hidden: Option<bool>,
        pub is_locked: Option<bool>,
        pub color: Option<String>,
        pub description: Option<String>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct GroupState {
        pub group_id: Uuid,
        pub name: Option<String>,
        pub color: Option<String>,
        pub description: Option<String>,
        pub piece_ids: Vec<Uuid>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct StatState {
        pub stat_id: Uuid,
        pub quality_id: Uuid,
        pub unit: Option<String>,
        pub min: Option<f64>,
        pub min_excluded: Option<bool>,
        pub max: Option<f64>,
        pub max_excluded: Option<bool>,
        pub lifecycle: Lifecycle,
    }

    #[derive(Debug, Clone)]
    pub struct ComposePersonState {
        pub person_id: Uuid,
        pub frontend_id: String,
        pub display_name: Option<String>,
        pub color: Option<String>,
        pub is_present: bool,
        pub cursor: Option<[f64; 2]>,
        pub look: Option<LookState>,
        pub selected_piece_ids: Vec<Uuid>,
        pub selected_design_ids: Vec<Uuid>,
    }

    #[derive(Debug, Clone)]
    pub struct LookState {
        pub position: [f64; 3],
        pub forward: [f64; 3],
        pub up: [f64; 3],
    }
} // 🖋️State
pub use state::*;

mod error {
    // 🎼️Error
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
        // 🗄️ CW6b: the bespoke Postgres error variant is retired — `db::Database`/`ArtifactHandle`
        // (WAL, conflict detection, durability) now own document persistence end to end.
        #[error("database error: {0}")]
        Database(#[from] db::DbError),
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
    pub struct ErrorBody {
        error: String,
        detail: String,
    }

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
            (status, Json(body)).into_response()
        }
    }
} // 🎼️Error
pub use error::*;

mod store {
    // 🗄️Store
    // Specs: `db::Database` (WAL/conflict/durability/frontier authority, generic path-value document
    // documents per `db_artifact`'s schema-erased convention) replaces the bespoke Postgres schema +
    // persistence layer. `ComposeDirectoryStore` holds identity/tenancy-shaped bookkeeping db does not
    // own (session ownership, share tokens, compaction config) — the semio_compose_rs-semio_hub analog of the plan's
    // `HubDirectory` split, file-backed (zero-touch, no external service) instead of sqlite/postgres/
    // neo4j since semio_compose_rs-semio_hub's own storage-swappability surface was never itemized in the campaign
    // contract (only `db`'s and os-semio_hub-directory's were) — a swappable `ComposeHubDirectory` trait is
    // future work, flagged in this ticket's report. `HistoryStore` holds semio_compose_rs's own bespoke kit
    // history/lookback/compaction feature: `db_query`'s `Query::Get`/`GetMany` are point lookups only
    // (no historical/range read yet — see `db_engine::Consistency::Historical`'s doc), so time-travel
    // reconstruction keeps its own lightweight, file-backed snapshot + change-log store, now JSON files
    // instead of Postgres rows.
    // Summary: db-backed document storage bridge, file-backed directory/history stores, and the pure
    // kit-JSON (de)serialization + change-log replay helpers.

    use super::*;

    //#region 🔖️Database
    /// 🗄️ Opens (or creates) the `db::Database` backing every semio_compose_rs session document, rooted at
    /// `<data_dir>/db`. Zero-touch: `FsStorage`, the family's default profile is overridable via
    /// `COMPOSE_HUB_DB_PROFILE` (`dev` default, `prod`, or `test`).
    pub fn open_database(data_dir: &std::path::Path) -> Result<db::Database, SessionError> {
        let profile = match std::env::var("COMPOSE_HUB_DB_PROFILE").ok().as_deref() {
            Some("prod") => db::Profile::Prod,
            Some("test") => db::Profile::Test,
            _ => db::Profile::Dev,
        };
        let root = data_dir.join("db");
        std::fs::create_dir_all(&root).map_err(|e| SessionError::Internal(format!("create db dir {}: {e}", root.display())))?;
        Ok(db::Database::open_at(&root, profile)?)
    }

    /// 🗄️#⃣ The document id a semio_compose_rs session's `db::Database` document lives under.
    pub fn document_id(session_id: Uuid) -> protocol::ArtifactId {
        protocol::ArtifactId(session_id.to_string())
    }

    /// 🗄️🌉️ `db::Frontier` -> `protocol::RuntimeFrontierSummary` (the `protocol_wire` frame shape) —
    /// `head_edit_id` has no direct db counterpart (a frontier summary carries no per-op label), so a
    /// deterministic `<document>@<head_seq>` stand-in is used; only `head_edit_ordinal`/`chain_hash`
    /// participate in `protocol::runtime_frontier_delta`'s actual comparison.
    pub fn frontier_to_wire(frontier: &db::Frontier) -> protocol::RuntimeFrontierSummary {
        protocol::RuntimeFrontierSummary { document_id: frontier.document.clone(), head_edit_ordinal: frontier.head_seq, head_edit_id: format!("{}@{}", frontier.document.0, frontier.head_seq), last_commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash }
    }

    /// 🗄️⏰️ A `HybridLogicalTimestamp` for one submit: `actor` seeded from the first 8 bytes of the
    /// submitting person's uuid (stable per-person tiebreak), `physical_ms` real wall-clock.
    pub fn now_hlc(actor_person_id: Uuid) -> protocol::HybridLogicalTimestamp {
        let bytes = actor_person_id.as_bytes();
        let actor_seed = u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
        let physical_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        protocol::HybridLogicalTimestamp::new(actor_seed, physical_ms)
    }

    fn now_ms() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64
    }
    //#endregion 🔖️Database

    //#region 🔖️Atomic JSON File
    /// 🗄️✍️ Reads `path` as JSON, defaulting to `T::default()` if the file does not exist yet.
    fn read_json_or_default<T: serde::de::DeserializeOwned + Default>(path: &std::path::Path) -> Result<T, SessionError> {
        match std::fs::read(path) {
            Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| SessionError::Internal(format!("corrupt store file {}: {e}", path.display()))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(T::default()),
            Err(e) => Err(SessionError::Internal(format!("read {}: {e}", path.display()))),
        }
    }

    /// 🗄️✍️ Writes `value` to `path` atomically (temp file + rename) so a crash mid-write never
    /// corrupts the previous contents — the same `write_atomic` convention `pack`/`FsStorage` use.
    fn write_json_atomic<T: Serialize>(path: &std::path::Path, value: &T) -> Result<(), SessionError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| SessionError::Internal(format!("create dir {}: {e}", parent.display())))?;
        }
        let bytes = serde_json::to_vec_pretty(value).map_err(|e| SessionError::Internal(format!("serialize {}: {e}", path.display())))?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, &bytes).map_err(|e| SessionError::Internal(format!("write {}: {e}", tmp.display())))?;
        std::fs::rename(&tmp, path).map_err(|e| SessionError::Internal(format!("rename {} -> {}: {e}", tmp.display(), path.display())))?;
        Ok(())
    }
    //#endregion 🔖️Atomic JSON File

    //#region 🔖️Directory
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SessionRecord {
        pub root_kit_id: Uuid,
        pub owner_token: Uuid,
        pub status: SessionStatus,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ShareTokenRecord {
        pub session_id: Uuid,
        pub access_mode: AccessMode,
        pub entity_kind: Option<String>,
        pub entity_id: Option<Uuid>,
        pub label: Option<String>,
        pub created_at: String,
        pub expires_at: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    struct DirectoryState {
        sessions: BTreeMap<Uuid, SessionRecord>,
        share_tokens: BTreeMap<Uuid, ShareTokenRecord>,
        compaction_configs: BTreeMap<Uuid, Vec<String>>,
    }

    /// 🗄️🧭️ File-backed identity/tenancy bookkeeping semio_compose_rs-semio_hub owns directly (not `db`'s concern,
    /// mirroring the plan's `os-semio_hub`/`HubDirectory` split): session ownership + status, share tokens,
    /// per-session compaction (lookback) configuration.
    pub struct ComposeDirectoryStore {
        path: std::path::PathBuf,
        state: std::sync::Mutex<DirectoryState>,
    }

    impl ComposeDirectoryStore {
        pub fn open(data_dir: &std::path::Path) -> Result<Self, SessionError> {
            let path = data_dir.join("directory.json");
            let state = read_json_or_default(&path)?;
            Ok(Self { path, state: std::sync::Mutex::new(state) })
        }

        fn with_state<R>(&self, f: impl FnOnce(&mut DirectoryState) -> R) -> Result<R, SessionError> {
            let mut guard = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let result = f(&mut guard);
            write_json_atomic(&self.path, &*guard)?;
            Ok(result)
        }

        pub fn create_session(&self, session_id: Uuid, root_kit_id: Uuid) -> Result<Uuid, SessionError> {
            let owner_token = Uuid::now_v7();
            let now = chrono_now_iso();
            self.with_state(|s| {
                s.sessions.insert(session_id, SessionRecord { root_kit_id, owner_token, status: SessionStatus::Active, created_at: now.clone(), updated_at: now });
            })?;
            Ok(owner_token)
        }

        pub fn session_record(&self, session_id: Uuid) -> Result<SessionRecord, SessionError> {
            let guard = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.sessions.get(&session_id).cloned().ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))
        }

        pub fn touch_session(&self, session_id: Uuid) -> Result<(), SessionError> {
            self.with_state(|s| {
                if let Some(rec) = s.sessions.get_mut(&session_id) {
                    rec.updated_at = chrono_now_iso();
                }
            })
        }

        pub fn set_session_root_kit(&self, session_id: Uuid, root_kit_id: Uuid) -> Result<(), SessionError> {
            self.with_state(|s| {
                if let Some(rec) = s.sessions.get_mut(&session_id) {
                    rec.root_kit_id = root_kit_id;
                    rec.updated_at = chrono_now_iso();
                }
            })
        }

        pub fn close_session(&self, session_id: Uuid) -> Result<bool, SessionError> {
            self.with_state(|s| match s.sessions.get_mut(&session_id) {
                Some(rec) => {
                    rec.status = SessionStatus::Closed;
                    rec.updated_at = chrono_now_iso();
                    true
                }
                None => false,
            })
        }

        pub fn load_owner_token(&self, session_id: Uuid) -> Result<Uuid, SessionError> {
            Ok(self.session_record(session_id)?.owner_token)
        }

        #[allow(clippy::too_many_arguments)]
        pub fn create_share_token(&self, session_id: Uuid, access_mode: AccessMode, entity_kind: Option<&str>, entity_id: Option<Uuid>, label: Option<&str>, expires_at: Option<&str>) -> Result<Uuid, SessionError> {
            // Referential check mirroring the old `runtime.share_token`'s FK on `session_id`.
            self.session_record(session_id)?;
            let token = Uuid::now_v7();
            let record = ShareTokenRecord { session_id, access_mode, entity_kind: entity_kind.map(str::to_string), entity_id, label: label.map(str::to_string), created_at: chrono_now_iso(), expires_at: expires_at.map(str::to_string) };
            self.with_state(|s| {
                s.share_tokens.insert(token, record);
            })?;
            Ok(token)
        }

        pub fn resolve_share_token(&self, token: Uuid) -> Result<ResolvedShareToken, SessionError> {
            let guard = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let record = guard.share_tokens.get(&token).ok_or_else(|| SessionError::EntityNotFound { kind: "share_token".into(), id: token.to_string() })?;
            if let Some(expires_at) = &record.expires_at {
                if expires_at.as_str() < chrono_now_iso().as_str() {
                    return Err(SessionError::EntityNotFound { kind: "share_token".into(), id: token.to_string() });
                }
            }
            Ok(ResolvedShareToken { session_id: record.session_id, access_mode: record.access_mode, entity_kind: record.entity_kind.clone(), entity_id: record.entity_id, label: record.label.clone() })
        }

        pub fn list_share_tokens(&self, session_id: Uuid) -> Result<Vec<ShareTokenRow>, SessionError> {
            let guard = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let mut rows: Vec<(String, ShareTokenRow)> = guard
                .share_tokens
                .iter()
                .filter(|(_, r)| r.session_id == session_id)
                .map(|(token, r)| {
                    let mode = match r.access_mode {
                        AccessMode::Owner => "owner",
                        AccessMode::Viewer => "viewer",
                    };
                    (r.created_at.clone(), ShareTokenRow { token: *token, session_id: r.session_id, access_mode: mode.to_string(), entity_kind: r.entity_kind.clone(), entity_id: r.entity_id, label: r.label.clone(), created_at: r.created_at.clone(), expires_at: r.expires_at.clone() })
                })
                .collect();
            rows.sort_by(|a, b| b.0.cmp(&a.0));
            Ok(rows.into_iter().map(|(_, row)| row).collect())
        }

        pub fn delete_share_token(&self, token: Uuid) -> Result<bool, SessionError> {
            self.with_state(|s| s.share_tokens.remove(&token).is_some())
        }

        pub fn resolve_access(&self, session_id: Uuid, bearer: Option<&str>) -> Result<AccessMode, SessionError> {
            match bearer {
                Some(token_str) => {
                    let token = Uuid::parse_str(token_str).map_err(|_| SessionError::Unauthorized("invalid token format".into()))?;
                    let owner_token = self.load_owner_token(session_id)?;
                    if token == owner_token {
                        return Ok(AccessMode::Owner);
                    }
                    let resolved = self.resolve_share_token(token).map_err(|_| SessionError::Unauthorized("invalid or expired token".into()))?;
                    if resolved.session_id != session_id {
                        return Err(SessionError::Unauthorized("token does not match session".into()));
                    }
                    Ok(resolved.access_mode)
                }
                None => Ok(AccessMode::Viewer),
            }
        }

        pub fn compaction_config(&self, session_id: Uuid) -> Vec<String> {
            let guard = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.compaction_configs.get(&session_id).cloned().unwrap_or_else(|| lookback_tokens().iter().map(|s| s.to_string()).collect())
        }

        pub fn set_compaction_config(&self, session_id: Uuid, tokens: Vec<String>) -> Result<(), SessionError> {
            self.with_state(|s| {
                s.compaction_configs.insert(session_id, tokens);
            })
        }

        pub fn all_sessions(&self) -> Vec<(Uuid, SessionRecord)> {
            let guard = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.sessions.iter().map(|(id, rec)| (*id, rec.clone())).collect()
        }

        pub fn all_share_tokens(&self) -> Vec<(Uuid, ShareTokenRecord)> {
            let guard = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.share_tokens.iter().map(|(id, rec)| (*id, rec.clone())).collect()
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResolvedShareToken {
        pub session_id: Uuid,
        pub access_mode: AccessMode,
        pub entity_kind: Option<String>,
        pub entity_id: Option<Uuid>,
        pub label: Option<String>,
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
    //#endregion 🔖️Directory

    //#region 🔖️Kit Identity
    pub fn session_kit_id(kit_json: &serde_json::Value) -> Result<Uuid, SessionError> {
        let id = kit_json.get("id").and_then(|value| value.as_str()).ok_or_else(|| SessionError::Validation("kit snapshot must include string id".into()))?;
        Uuid::parse_str(id).map_err(|err| SessionError::Validation(format!("invalid kit id '{id}': {err}")))
    }

    pub fn session_kit_name(kit_json: &serde_json::Value) -> Result<&str, SessionError> {
        kit_json.get("name").and_then(|value| value.as_str()).filter(|value| !value.trim().is_empty()).ok_or_else(|| SessionError::Validation("kit snapshot must include string name".into()))
    }

    pub fn session_kit_string(kit_json: &serde_json::Value, field: &str) -> Option<String> {
        kit_json.get(field).and_then(|value| value.as_str()).map(|value| value.to_string())
    }

    pub fn chrono_now_iso() -> String {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
        let secs = now.as_secs();
        let nanos = now.subsec_nanos();
        format!("{}.{:09}Z", secs, nanos)
    }
    //#endregion 🔖️Kit Identity

    //#region 🔖️Kit Serialization
    /// 🗄️📤️ Serializes a `SessionState` into the same kit JSON shape the semio_compose_rs GraphQL schema
    /// expects — unchanged from the pre-CW6b implementation (pure function, no storage dependency).
    pub fn serialize_session_kit(state: &SessionState) -> serde_json::Value {
        let types: Vec<serde_json::Value> = state
            .types
            .values()
            .filter(|t| t.lifecycle.is_active())
            .map(|t| {
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
            })
            .collect();
        let designs: Vec<serde_json::Value> = state
            .designs
            .values()
            .filter(|d| d.lifecycle.is_active())
            .map(|d| {
                let pieces: Vec<serde_json::Value> = d
                    .pieces
                    .values()
                    .filter(|p| p.lifecycle.is_active())
                    .map(|p| {
                        serde_json::json!({
                            "id": p.piece_id, "name": p.name, "type": p.type_id,
                            "center": p.center.map(|c| serde_json::json!({"u": c[0], "v": c[1]})),
                            "isHidden": p.is_hidden, "isLocked": p.is_locked,
                            "color": p.color, "description": p.description,
                            "design": { "id": d.design_id },
                        })
                    })
                    .collect();
                let connections: Vec<serde_json::Value> = d
                    .connections
                    .values()
                    .filter(|c| c.lifecycle.is_active())
                    .map(|c| {
                        serde_json::json!({
                            "id": c.connection_id,
                            "parent": {
                                "piece": { "id": c.parent_piece_id },
                                "designPiece": c.parent_design_piece_id.map(|id| serde_json::json!({ "id": id })),
                                "connector": c.parent_connector_id.map(|id| serde_json::json!({ "id": id })),
                            },
                            "child": {
                                "piece": { "id": c.child_piece_id },
                                "designPiece": c.child_design_piece_id.map(|id| serde_json::json!({ "id": id })),
                                "connector": c.child_connector_id.map(|id| serde_json::json!({ "id": id })),
                            },
                            "gap": c.gap, "shift": c.shift, "rise": c.rise,
                            "rotation": c.rotation, "turn": c.turn, "tilt": c.tilt,
                            "u": c.u, "v": c.v, "description": c.description,
                        })
                    })
                    .collect();
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
            })
            .collect();
        let authors: Vec<serde_json::Value> = state.authors.values().filter(|a| a.lifecycle.is_active()).map(|a| serde_json::json!({"id": a.author_id, "name": a.name, "email": a.email})).collect();
        let tags: Vec<serde_json::Value> = state.tags.values().filter(|t| t.lifecycle.is_active()).map(|t| serde_json::json!({"id": t.tag_id, "name": t.name, "description": t.description, "icon": t.icon})).collect();
        let concepts: Vec<serde_json::Value> = state.concepts.values().filter(|c| c.lifecycle.is_active()).map(|c| serde_json::json!({"id": c.concept_id, "name": c.name, "description": c.description, "icon": c.icon})).collect();
        let ports: Vec<serde_json::Value> = state
            .ports
            .values()
            .filter(|p| p.lifecycle.is_active())
            .map(|p| {
                serde_json::json!({
                    "id": p.port_id,
                    "name": p.name,
                    "description": p.description,
                    "icon": p.icon,
                    "maxChildren": p.max_children,
                    "compatiblePorts": p.compatible_port_ids.iter().map(|id| serde_json::json!({ "id": id })).collect::<Vec<_>>(),
                })
            })
            .collect();
        let qualities: Vec<serde_json::Value> = state
            .qualities
            .values()
            .filter(|q| q.lifecycle.is_active())
            .map(|q| {
                serde_json::json!({
                    "id": q.quality_id,
                    "key": q.key,
                    "name": q.name,
                    "description": q.description,
                    "icon": q.icon,
                    "unit": q.unit,
                })
            })
            .collect();
        let folders: Vec<serde_json::Value> = state
            .folders
            .values()
            .filter(|f| f.lifecycle.is_active())
            .map(|f| {
                serde_json::json!({
                    "id": f.folder_id,
                    "name": f.name,
                    "parent": f.parent_folder_id.map(|id| serde_json::json!({ "id": id })),
                    "description": f.description,
                })
            })
            .collect();
        let files: Vec<serde_json::Value> = state
            .files
            .values()
            .filter(|f| f.lifecycle.is_active())
            .map(|f| {
                serde_json::json!({
                    "id": f.file_id,
                    "name": f.name,
                    "remote": f.remote,
                    "folder": f.folder_id.map(|id| serde_json::json!({ "id": id })),
                    "size": f.size,
                    "hash": f.hash,
                    "blob": f.blob,
                })
            })
            .collect();
        serde_json::json!({
            "id": state.kit.kit_id, "name": state.kit.name,
            "version": state.kit.version, "description": state.kit.description,
            "icon": state.🖼️kit.icon, "image": state.kit.image,
            "preview": state.kit.preview, "remote": state.kit.remote,
            "homepage": state.kit.homepage, "license": state.kit.license,
            "types": types, "designs": designs, "authors": authors, "tags": tags,
            "concepts": concepts, "ports": ports, "qualities": qualities, "folders": folders, "files": files,
            "createdAt": chrono_now_iso(), "updatedAt": chrono_now_iso(),
        })
    }

    pub fn apply_change_log_to_kit(kit: &mut serde_json::Value, changes: &serde_json::Value) {
        let changes_arr = match changes.as_array() {
            Some(a) => a,
            None => return,
        };
        for change in changes_arr {
            let operation = change.get("operation").and_then(|v| v.as_str()).unwrap_or("");
            let entity_kind = change.get("entity_kind").and_then(|v| v.as_str()).unwrap_or("");
            let entity_id = change.get("entity_id").and_then(|v| v.as_str()).unwrap_or("");
            match operation {
                "Created" => {
                    let snapshot = change.get("snapshot").cloned().unwrap_or(serde_json::json!({}));
                    let mut entity = snapshot.clone();
                    if entity.get("id").is_none() {
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
                            let collection_key = match entity_kind {
                                "type" => "types",
                                "design" => "designs",
                                "author" => "authors",
                                "tag" => "tags",
                                _ => "",
                            };
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
                "Deleted" => match entity_kind {
                    "type" => remove_from_array(kit, "types", entity_id),
                    "design" => remove_from_array(kit, "designs", entity_id),
                    "author" => remove_from_array(kit, "authors", entity_id),
                    "tag" => remove_from_array(kit, "tags", entity_id),
                    "piece" => remove_from_design_arrays(kit, "pieces", entity_id),
                    "connection" => remove_from_design_arrays(kit, "connections", entity_id),
                    _ => {}
                },
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
                        arr.push(item);
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
    //#endregion 🔖️Kit Serialization

    //#region 🔖️History
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    struct SessionHistory {
        domain_commits: Vec<(i64, Uuid, i64)>,
        kit_snapshots: Vec<(i64, serde_json::Value)>,
        entity_change_logs: Vec<(i64, Vec<EntityChange>)>,
        last_compacted_at: Option<String>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct CompactionResult {
        pub snapshots_created: u32,
        pub logs_deleted: u64,
    }

    /// 🗄️🔩️ Compose's own bespoke kit-history/lookback/compaction feature — see this module's doc for
    /// why it stays a lightweight file-backed store rather than reaching into `db`'s WAL directly.
    pub struct HistoryStore {
        root: std::path::PathBuf,
    }

    impl HistoryStore {
        pub fn open(data_dir: &std::path::Path) -> Self {
            Self { root: data_dir.join("history") }
        }

        fn path(&self, session_id: Uuid) -> std::path::PathBuf {
            self.root.join(format!("{session_id}.json"))
        }

        fn load(&self, session_id: Uuid) -> Result<SessionHistory, SessionError> {
            read_json_or_default(&self.path(session_id))
        }

        fn save(&self, session_id: Uuid, history: &SessionHistory) -> Result<(), SessionError> {
            write_json_atomic(&self.path(session_id), history)
        }

        pub fn record_domain_commit(&self, session_id: Uuid, domain_version: DomainVersion, command_id: Uuid) -> Result<(), SessionError> {
            let mut history = self.load(session_id)?;
            if !history.domain_commits.iter().any(|(v, _, _)| *v == domain_version) {
                history.domain_commits.push((domain_version, command_id, now_ms()));
            }
            self.save(session_id, &history)
        }

        pub fn store_kit_snapshot(&self, session_id: Uuid, domain_version: DomainVersion, kit_json: &serde_json::Value) -> Result<(), SessionError> {
            let mut history = self.load(session_id)?;
            match history.kit_snapshots.iter_mut().find(|(v, _)| *v == domain_version) {
                Some((_, existing)) => *existing = kit_json.clone(),
                None => history.kit_snapshots.push((domain_version, kit_json.clone())),
            }
            self.save(session_id, &history)
        }

        pub fn store_entity_change_log(&self, session_id: Uuid, domain_version: DomainVersion, changes: &[EntityChange]) -> Result<(), SessionError> {
            let mut history = self.load(session_id)?;
            if !history.entity_change_logs.iter().any(|(v, _)| *v == domain_version) {
                history.entity_change_logs.push((domain_version, changes.to_vec()));
            }
            self.save(session_id, &history)
        }

        pub fn get_latest_snapshot_before(&self, session_id: Uuid, target_version: DomainVersion) -> Result<Option<(DomainVersion, serde_json::Value)>, SessionError> {
            let history = self.load(session_id)?;
            Ok(history.kit_snapshots.iter().filter(|(v, _)| *v <= target_version).max_by_key(|(v, _)| *v).cloned())
        }

        pub fn get_change_logs_in_range(&self, session_id: Uuid, from_version_exclusive: DomainVersion, to_version_inclusive: DomainVersion) -> Result<Vec<(DomainVersion, serde_json::Value)>, SessionError> {
            let history = self.load(session_id)?;
            let mut logs: Vec<(DomainVersion, serde_json::Value)> = history.entity_change_logs.iter().filter(|(v, _)| *v > from_version_exclusive && *v <= to_version_inclusive).map(|(v, changes)| (*v, serde_json::to_value(changes).unwrap_or(serde_json::json!([])))).collect();
            logs.sort_by_key(|(v, _)| *v);
            Ok(logs)
        }

        pub fn get_version_at_time(&self, session_id: Uuid, seconds_ago: i64) -> Result<Option<DomainVersion>, SessionError> {
            let history = self.load(session_id)?;
            let cutoff_ms = now_ms() - seconds_ago * 1000;
            Ok(history.domain_commits.iter().filter(|(_, _, at_ms)| *at_ms <= cutoff_ms).map(|(v, _, _)| *v).max())
        }

        pub fn reconstruct_kit_at_version(&self, session_id: Uuid, target_version: DomainVersion) -> Result<serde_json::Value, SessionError> {
            let (snap_version, mut kit) = self.get_latest_snapshot_before(session_id, target_version)?.ok_or_else(|| SessionError::Internal("no baseline snapshot found".to_string()))?;
            if snap_version < target_version {
                let logs = self.get_change_logs_in_range(session_id, snap_version, target_version)?;
                for (_version, changes) in &logs {
                    apply_change_log_to_kit(&mut kit, changes);
                }
            }
            Ok(kit)
        }

        pub fn get_kit_at_lookback(&self, session_id: Uuid, lookback_token: &str) -> Result<serde_json::Value, SessionError> {
            let seconds = lookback_seconds(lookback_token).ok_or_else(|| SessionError::Validation(format!("unknown lookback token: {}", lookback_token)))?;
            let target_version = self.get_version_at_time(session_id, seconds)?.ok_or_else(|| SessionError::Internal("no version found at lookback time".to_string()))?;
            self.reconstruct_kit_at_version(session_id, target_version)
        }

        pub fn last_compacted_at(&self, session_id: Uuid) -> Option<String> {
            self.load(session_id).ok().and_then(|h| h.last_compacted_at)
        }

        pub fn compact(&self, session_id: Uuid, current_state: &SessionState) -> Result<CompactionResult, SessionError> {
            let mut snapshots_created = 0u32;
            let mut logs_deleted = 0u64;
            let current_version = current_state.domain_version;
            let current_kit = serialize_session_kit(current_state);
            self.store_kit_snapshot(session_id, current_version, &current_kit)?;
            snapshots_created += 1;
            for &(token, seconds) in LOOKBACK_POINTS {
                let boundary_version = self.get_version_at_time(session_id, seconds)?;
                if let Some(bv) = boundary_version {
                    if bv > 0 {
                        let existing = self.get_latest_snapshot_before(session_id, bv)?;
                        match existing {
                            Some((sv, _)) if sv == bv => {}
                            _ => match self.reconstruct_kit_at_version(session_id, bv) {
                                Ok(kit) => {
                                    self.store_kit_snapshot(session_id, bv, &kit)?;
                                    snapshots_created += 1;
                                }
                                Err(_) => {
                                    tracing::warn!("compaction: could not reconstruct kit at version {} for lookback {}", bv, token);
                                }
                            },
                        }
                    }
                }
            }
            let oldest_seconds = LOOKBACK_POINTS.last().map_or(31536000, |(_, s)| *s);
            let oldest_version = self.get_version_at_time(session_id, oldest_seconds)?;
            if let Some(ov) = oldest_version {
                if ov > 0 {
                    let mut history = self.load(session_id)?;
                    let before = history.entity_change_logs.len();
                    history.entity_change_logs.retain(|(v, _)| !(*v < ov && history.kit_snapshots.iter().any(|(sv, _)| *sv >= *v)));
                    logs_deleted = (before - history.entity_change_logs.len()) as u64;
                    history.last_compacted_at = Some(chrono_now_iso());
                    self.save(session_id, &history)?;
                }
            }
            Ok(CompactionResult { snapshots_created, logs_deleted })
        }
    }
    //#endregion 🔖️History
} // 🗄️Store
pub use store::*;

mod actor {
    // 🎹️Actor
    // Specs: ActorMessage is the inbox message kind. SessionActor processes commands one at a time in
    // arrival order, submitting through `db::ArtifactHandle` (WAL/conflict/durability/frontier
    // authority) and keeping an in-memory `SessionState` replica for fast reads + kit-JSON history.
    // Summary: Session actor: single-writer task processing commands sequentially against `db`.

    use super::*;

    pub enum ActorMessage {
        DomainCommand { envelope: CommandEnvelope, command: DomainCommand, reply: oneshot::Sender<Result<CommandResult, SessionError>> },
        ComposeCommand { envelope: ComposeEnvelope, command: ComposeCommand, reply: oneshot::Sender<Result<(), SessionError>> },
        GetSnapshot { reply: oneshot::Sender<SessionSnapshot> },
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct SessionSnapshot {
        pub session_id: Uuid,
        pub domain_version: DomainVersion,
        pub compose_version: ComposeVersion,
        pub kit: serde_json::Value,
    }

    /// 🎞️ What the actor broadcasts on the `protocol_wire` lanes — the WS layer (see `ws` module)
    /// encodes these into real `ServerFrame::Commands`/`ServerFrame::Preview` binary frames. Kept
    /// separate from the legacy JSON `SessionEvent` broadcast (still emitted, still exercised by
    /// existing tests/admin introspection) rather than replacing it, since the two serve different
    /// consumers: `SessionEvent` is semio_compose_rs's own typed domain event, `WireEvent` is the wire-v2
    /// envelope/frontier pair `protocol_wire` clients (framework/sync-style actors) expect.
    #[derive(Clone)]
    pub enum WireEvent {
        Commands { envelope: Box<protocol::OperationEnvelope>, frontier: db::Frontier },
        Preview { actor: protocol::ActorId, key: String, seq: u64, payload: Vec<u8> },
    }

    pub struct SessionActor {
        session_id: Uuid,
        state: SessionState,
        handle: db::ArtifactHandle,
        history: Arc<HistoryStore>,
        directory: Arc<ComposeDirectoryStore>,
        event_tx: broadcast::Sender<SessionEvent>,
        wire_tx: broadcast::Sender<WireEvent>,
    }

    impl SessionActor {
        pub fn new(session_id: Uuid, state: SessionState, handle: db::ArtifactHandle, history: Arc<HistoryStore>, directory: Arc<ComposeDirectoryStore>, event_tx: broadcast::Sender<SessionEvent>, wire_tx: broadcast::Sender<WireEvent>) -> Self {
            Self { session_id, state, handle, history, directory, event_tx, wire_tx }
        }

        pub async fn run(&mut self, mut rx: mpsc::Receiver<ActorMessage>) {
            while let Some(msg) = rx.recv().await {
                match msg {
                    ActorMessage::DomainCommand { envelope, command, reply } => {
                        let result = self.handle_domain_command(envelope, command).await;
                        let _ = reply.send(result);
                    }
                    ActorMessage::ComposeCommand { envelope, command, reply } => {
                        self.handle_compose_command(&envelope, &command);
                        let _ = reply.send(Ok(()));
                    }
                    ActorMessage::GetSnapshot { reply } => {
                        let _ = reply.send(self.build_snapshot());
                    }
                }
            }
        }

        async fn handle_domain_command(&mut self, envelope: CommandEnvelope, command: DomainCommand) -> Result<CommandResult, SessionError> {
            // 🗄️ `db::ArtifactHandle::submit` dedupes durably by `operation_id` — keyed here by
            // `(client_id, request_id)` (this actor's own logical idempotency key, matching the old
            // `runtime.session_command` UNIQUE(session_id, client_id, request_id) constraint) rather
            // than the client-supplied `command_id`, so a retried request with a fresh `command_id`
            // still dedupes correctly. A resubmit resolves to the SAME cached `CommandReceipt` (same
            // frontier) db_artifact already returned for the first submit — comparing the frontier
            // before/after is this actor's way of telling a genuine commit from a replay, since this
            // actor is the sole writer to its document (single-writer-per-session, unchanged from the
            // pre-CW6b design).
            let before_seq = self.handle.frontier()?.head_seq;

            let mut trial_state = self.state.clone();
            let mut entries: Vec<(String, Option<serde_json::Value>)> = Vec::new();
            let mut changes: Vec<EntityChange> = Vec::new();
            apply_domain_command(&mut trial_state, &command, envelope.command_id.0, &mut entries, &mut changes);

            let payload: serde_json::Map<String, serde_json::Value> = entries.into_iter().map(|(path, value)| (path, value.unwrap_or(serde_json::Value::Null))).collect();
            let op_envelope = protocol::OperationEnvelope {
                operation_id: protocol::OperationId(format!("{}:{}", envelope.client_id.0, envelope.request_id.0)),
                document_id: document_id(self.session_id),
                actor: protocol::ActorId(envelope.actor_person_id.0.to_string()),
                dependencies: Vec::new(),
                diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: serde_json::to_vec(&serde_json::Value::Object(payload)).unwrap_or_default() },
                // 🎯️ No real inverse yet — semio_compose_rs-semio_hub has no undo/redo feature today (the original
                // Postgres implementation didn't have one either); a per-command-kind inverse is
                // future work, flagged in this ticket's report.
                inverse: protocol::InverseOperation {
                    schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                    payload: serde_json::to_vec(&serde_json::Value::Object(serde_json::Map::new())).unwrap_or_default(),
                },
                timestamp: now_hlc(envelope.actor_person_id.0),
            };
            let batch = db::document::CommandBatch::new(vec![op_envelope.clone()]).map_err(SessionError::Database)?;
            let receipt = self.handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync }).await??;

            if receipt.frontier.head_seq == before_seq {
                return Ok(CommandResult::IdempotentDuplicate);
            }

            let new_version = receipt.frontier.head_seq as DomainVersion;
            self.state = trial_state;
            self.state.domain_version = new_version;
            self.history.record_domain_commit(self.session_id, new_version, envelope.command_id.0)?;
            self.history.store_entity_change_log(self.session_id, new_version, &changes)?;
            let _ = self.directory.touch_session(self.session_id);
            if new_version % 50 == 0 {
                if let Err(e) = self.history.compact(self.session_id, &self.state) {
                    tracing::warn!("compaction failed at version {}: {}", new_version, e);
                }
            }

            let _ = self.event_tx.send(SessionEvent::DomainCommandAccepted { command_id: envelope.command_id, domain_version: new_version, changes });
            let _ = self.wire_tx.send(WireEvent::Commands { envelope: Box::new(op_envelope), frontier: receipt.frontier });
            Ok(CommandResult::Accepted { domain_version: new_version })
        }

        /// 🌫️ Compose presence (cursor/look/selection) is ephemeral by design — never submitted to
        /// `db` (matches `db_preview`'s "never durable" law and `protocol_wire`'s loss-tolerant
        /// Preview lane), broadcast-only. The pre-CW6b Postgres implementation persisted these to
        /// `semio_compose_rs.cursor`/`semio_compose_rs.look`/`semio_compose_rs.selection_*` tables but never actually wrote them
        /// into `SessionState.compose_people` (a latent dead field) — fixed here: presence now
        /// genuinely updates the in-memory replica admin/introspection reads.
        fn handle_compose_command(&mut self, envelope: &ComposeEnvelope, command: &ComposeCommand) {
            let pid = envelope.person_id.0;
            let fid = envelope.frontend_id.clone();
            let new_version = self.state.compose_version + 1;
            let key = (pid, fid.clone());
            let person = self.state.compose_people.entry(key).or_insert_with(|| ComposePersonState { person_id: pid, frontend_id: fid.clone(), display_name: None, color: None, is_present: true, cursor: None, look: None, selected_piece_ids: Vec::new(), selected_design_ids: Vec::new() });
            let update = match command {
                ComposeCommand::UpsertCursor(c) => {
                    person.cursor = Some([c.u, c.v]);
                    person.is_present = true;
                    ComposeUpdate::CursorMoved { u: c.u, v: c.v }
                }
                ComposeCommand::UpsertLook(l) => {
                    person.look = Some(LookState { position: l.position, forward: l.forward, up: l.up });
                    person.is_present = true;
                    ComposeUpdate::LookChanged { position: l.position, forward: l.forward, up: l.up }
                }
                ComposeCommand::SetSelection(s) => {
                    person.selected_piece_ids = s.piece_ids.clone();
                    person.selected_design_ids = s.design_ids.clone();
                    ComposeUpdate::SelectionChanged { piece_ids: s.piece_ids.clone(), design_ids: s.design_ids.clone() }
                }
                ComposeCommand::ClearPresence(_) => {
                    self.state.compose_people.remove(&(pid, fid.clone()));
                    ComposeUpdate::PresenceCleared
                }
            };
            self.state.compose_version = new_version;
            let _ = self.event_tx.send(SessionEvent::ComposeUpdated { compose_version: new_version, person_id: envelope.person_id, frontend_id: envelope.frontend_id.clone(), update: update.clone() });
            if let Ok(payload) = serde_json::to_vec(&update) {
                let _ = self.wire_tx.send(WireEvent::Preview { actor: protocol::ActorId(pid.to_string()), key: format!("semio_compose_rs:{}:{}", pid, envelope.frontend_id), seq: new_version as u64, payload });
            }
        }

        pub fn build_snapshot(&self) -> SessionSnapshot {
            let kit_json = serialize_session_kit(&self.state);
            SessionSnapshot { session_id: self.state.session_id.0, domain_version: self.state.domain_version, compose_version: self.state.compose_version, kit: kit_json }
        }
    }

    //#region 🔖️Command Apply
    fn find_design_id_for_piece(state: &SessionState, piece_id: Uuid) -> Option<Uuid> {
        state.designs.values().find(|d| d.pieces.contains_key(&piece_id)).map(|d| d.design_id)
    }

    fn find_design_id_for_connection(state: &SessionState, connection_id: Uuid) -> Option<Uuid> {
        state.designs.values().find(|d| d.connections.contains_key(&connection_id)).map(|d| d.design_id)
    }

    /// 🗄️ Mutates `state` and collects `(path, value)` diff entries (the `db_artifact` generic
    /// path-value convention — `None` is an explicit tombstone) plus typed `EntityChange`s for the
    /// kit-history change log. Pure/synchronous — no I/O — unlike the pre-CW6b version this replaces
    /// (which interleaved a Postgres `UPDATE`/`INSERT` per field inside this same match).
    fn apply_domain_command(state: &mut SessionState, command: &DomainCommand, cmd_id: Uuid, entries: &mut Vec<(String, Option<serde_json::Value>)>, changes: &mut Vec<EntityChange>) {
        match command {
            DomainCommand::PatchKit(patch) => {
                if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                    state.kit.name = name.to_string();
                    entries.push((format!("kit/{}/name", state.kit.kit_id), Some(serde_json::json!(name))));
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Kit, entity_id: state.kit.kit_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::CreateType(create) => {
                let name = create.fields.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled");
                entries.push((format!("types/{}", create.entity_id), Some(create.fields.clone())));
                state.types.insert(
                    create.entity_id,
                    TypeState {
                        type_id: create.entity_id,
                        name: name.to_string(),
                        parent_type_id: None,
                        description: None,
                        icon: None,
                        image: None,
                        folder: None,
                        unit: None,
                        stock: None,
                        is_abstract: None,
                        virtual_type: None,
                        location_id: None,
                        connectors: BTreeMap::new(),
                        representations: BTreeMap::new(),
                        props: BTreeMap::new(),
                        lifecycle: Lifecycle::Active,
                    },
                );
                changes.push(EntityChange::Created { entity_kind: EntityKind::Type, entity_id: create.entity_id, snapshot: create.fields.clone() });
            }
            DomainCommand::DeleteType(del) => {
                entries.push((format!("types/{}", del.entity_id), None));
                if let Some(t) = state.types.get_mut(&del.entity_id) {
                    t.lifecycle = Lifecycle::Tombstoned { at: state.domain_version, by: CommandId(cmd_id) };
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Type, entity_id: del.entity_id });
            }
            DomainCommand::PatchType(patch) => {
                if let Some(type_state) = state.types.get_mut(&patch.entity_id) {
                    if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                        type_state.name = name.to_string();
                        entries.push((format!("types/{}/name", patch.entity_id), Some(serde_json::json!(name))));
                    }
                    if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                        type_state.description = Some(description.to_string());
                        entries.push((format!("types/{}/description", patch.entity_id), Some(serde_json::json!(description))));
                    }
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Type, entity_id: patch.entity_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::CreateDesign(create) => {
                let name = create.fields.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled");
                entries.push((format!("designs/{}", create.entity_id), Some(create.fields.clone())));
                state.designs.insert(
                    create.entity_id,
                    DesignState {
                        design_id: create.entity_id,
                        name: name.to_string(),
                        parent_design_id: None,
                        description: None,
                        icon: None,
                        image: None,
                        folder: None,
                        unit: None,
                        is_abstract: None,
                        can_scale: None,
                        can_mirror: None,
                        active_layer_id: None,
                        location_id: None,
                        pieces: BTreeMap::new(),
                        connections: BTreeMap::new(),
                        layers: BTreeMap::new(),
                        groups: BTreeMap::new(),
                        stats: BTreeMap::new(),
                        props: BTreeMap::new(),
                        lifecycle: Lifecycle::Active,
                    },
                );
                changes.push(EntityChange::Created { entity_kind: EntityKind::Design, entity_id: create.entity_id, snapshot: create.fields.clone() });
            }
            DomainCommand::DeleteDesign(del) => {
                entries.push((format!("designs/{}", del.entity_id), None));
                if let Some(d) = state.designs.get_mut(&del.entity_id) {
                    d.lifecycle = Lifecycle::Tombstoned { at: state.domain_version, by: CommandId(cmd_id) };
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Design, entity_id: del.entity_id });
            }
            DomainCommand::PatchDesign(patch) => {
                if let Some(design_state) = state.designs.get_mut(&patch.entity_id) {
                    if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                        design_state.name = name.to_string();
                        entries.push((format!("designs/{}/name", patch.entity_id), Some(serde_json::json!(name))));
                    }
                    if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                        design_state.description = Some(description.to_string());
                        entries.push((format!("designs/{}/description", patch.entity_id), Some(serde_json::json!(description))));
                    }
                }
                changes.push(EntityChange::Updated { entity_kind: EntityKind::Design, entity_id: patch.entity_id, changed_fields: patch.fields.clone() });
            }
            DomainCommand::CreatePiece(create) => {
                let name = create.fields.get("name").and_then(|v| v.as_str());
                entries.push((format!("designs/{}/pieces/{}", create.design_id, create.piece_id), Some(create.fields.clone())));
                if let Some(design) = state.designs.get_mut(&create.design_id) {
                    design.pieces.insert(
                        create.piece_id,
                        PieceState { piece_id: create.piece_id, name: name.map(|s| s.to_string()), type_id: None, design_ref_id: None, plane: None, center: None, scale: None, mirror_plane: None, is_hidden: None, is_locked: None, color: None, description: None, lifecycle: Lifecycle::Active },
                    );
                }
                changes.push(EntityChange::Created { entity_kind: EntityKind::Piece, entity_id: create.piece_id, snapshot: create.fields.clone() });
            }
            DomainCommand::PatchPiece(patch) => {
                let center_u = patch.fields.get("center").and_then(|center| center.get("u")).and_then(|v| v.as_f64());
                let center_v = patch.fields.get("center").and_then(|center| center.get("v")).and_then(|v| v.as_f64());
                if let Some(design_id) = find_design_id_for_piece(state, patch.entity_id) {
                    if center_u.is_some() || center_v.is_some() {
                        entries.push((format!("designs/{}/pieces/{}/center", design_id, patch.entity_id), Some(serde_json::json!({"u": center_u, "v": center_v}))));
                    }
                    if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                        entries.push((format!("designs/{}/pieces/{}/name", design_id, patch.entity_id), Some(serde_json::json!(name))));
                    }
                }
                for design in state.designs.values_mut() {
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
                if let Some(design_id) = find_design_id_for_piece(state, del.entity_id) {
                    entries.push((format!("designs/{}/pieces/{}", design_id, del.entity_id), None));
                }
                for design in state.designs.values_mut() {
                    if let Some(piece) = design.pieces.get_mut(&del.entity_id) {
                        piece.lifecycle = Lifecycle::Tombstoned { at: state.domain_version, by: CommandId(cmd_id) };
                        break;
                    }
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Piece, entity_id: del.entity_id });
            }
            DomainCommand::CreateConnection(create) => {
                let connected_piece = create.fields.get("parent_piece_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                let connecting_piece = create.fields.get("child_piece_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                entries.push((format!("designs/{}/connections/{}", create.design_id, create.connection_id), Some(create.fields.clone())));
                if let Some(design) = state.designs.get_mut(&create.design_id) {
                    design.connections.insert(
                        create.connection_id,
                        ConnectionState {
                            connection_id: create.connection_id,
                            parent_piece_id: connected_piece,
                            parent_design_piece_id: None,
                            parent_connector_id: None,
                            child_piece_id: connecting_piece,
                            child_design_piece_id: None,
                            child_connector_id: None,
                            gap: 0.0,
                            shift: 0.0,
                            rise: 0.0,
                            rotation: 0.0,
                            turn: 0.0,
                            tilt: 0.0,
                            u: None,
                            v: None,
                            description: None,
                            lifecycle: Lifecycle::Active,
                        },
                    );
                }
                changes.push(EntityChange::Created { entity_kind: EntityKind::Connection, entity_id: create.connection_id, snapshot: create.fields.clone() });
            }
            DomainCommand::PatchConnection(patch) => {
                if let Some(design_id) = find_design_id_for_connection(state, patch.entity_id) {
                    if let Some(description) = patch.fields.get("description").and_then(|v| v.as_str()) {
                        entries.push((format!("designs/{}/connections/{}/description", design_id, patch.entity_id), Some(serde_json::json!(description))));
                    }
                    if let Some(u) = patch.fields.get("u").and_then(|v| v.as_f64()) {
                        entries.push((format!("designs/{}/connections/{}/u", design_id, patch.entity_id), Some(serde_json::json!(u))));
                    }
                    if let Some(v) = patch.fields.get("v").and_then(|v| v.as_f64()) {
                        entries.push((format!("designs/{}/connections/{}/v", design_id, patch.entity_id), Some(serde_json::json!(v))));
                    }
                }
                for design in state.designs.values_mut() {
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
                if let Some(design_id) = find_design_id_for_connection(state, del.entity_id) {
                    entries.push((format!("designs/{}/connections/{}", design_id, del.entity_id), None));
                }
                for design in state.designs.values_mut() {
                    if let Some(connection) = design.connections.get_mut(&del.entity_id) {
                        connection.lifecycle = Lifecycle::Tombstoned { at: state.domain_version, by: CommandId(cmd_id) };
                        break;
                    }
                }
                changes.push(EntityChange::Deleted { entity_kind: EntityKind::Connection, entity_id: del.entity_id });
            }
            DomainCommand::Batch(batch) => {
                for sub in &batch.commands {
                    apply_domain_command(state, sub, cmd_id, entries, changes);
                }
            }
            _ => {
                tracing::warn!("unhandled command variant: {:?}", std::mem::discriminant(command));
            }
        }
    }
    //#endregion 🔖️Command Apply
} // 🎹️Actor
pub use actor::*;

mod directory {
    // 🎯️Directory
    // Specs: SessionHandle holds the sender to an active session actor. SessionDirectory provides
    // get-or-create semantics, rebuilding a cold session's typed `SessionState` by reconstructing its
    // kit JSON from `HistoryStore` (snapshot + change-log replay) and reading `db`'s own frontier for
    // the authoritative `domain_version`.
    // Summary: Session directory: process-global registry mapping SessionId to actor handles.

    use super::*;
    #[derive(Clone)]
    pub struct SessionHandle {
        pub command_tx: mpsc::Sender<ActorMessage>,
        pub event_tx: broadcast::Sender<SessionEvent>,
        pub wire_tx: broadcast::Sender<WireEvent>,
        pub active_connections: Arc<AtomicUsize>,
        pub activated_at: Arc<Instant>,
    }

    //#region 🔖️ActiveSessionInfo

    #[derive(Debug, Clone, Serialize)]
    pub struct ActiveSessionInfo {
        pub session_id: Uuid,
        pub active_connections: usize,
        pub activated_at_secs_ago: u64,
    }

    //#endregion 🔖️ActiveSessionInfo

    //#region 🔖️StateRehydration
    /// 🗄️📥️ Rebuilds a typed `SessionState` from a reconstructed kit JSON (`HistoryStore::
    /// reconstruct_kit_at_version`/a freshly-seeded genesis kit) — the single place both session
    /// creation and cold-activation funnel through, so they can never drift from each other. Scope
    /// matches the pre-CW6b `load_session_state`'s own (it too never populated `locations`/
    /// `compose_people` — those stay empty here too, `compose_people` because presence is ephemeral
    /// by design, `locations` because no command ever produced one).
    pub fn session_state_from_kit_json(session_id: Uuid, domain_version: DomainVersion, compose_version: ComposeVersion, kit_json: &serde_json::Value) -> Result<SessionState, SessionError> {
        let kit_id = session_kit_id(kit_json)?;
        let kit = KitState {
            kit_id,
            name: session_kit_name(kit_json)?.to_string(),
            version: session_kit_string(kit_json, "version"),
            description: session_kit_string(kit_json, "description"),
            icon: session_kit_string(kit_json, "icon"),
            image: session_kit_string(kit_json, "image"),
            preview: session_kit_string(kit_json, "preview"),
            remote: session_kit_string(kit_json, "remote"),
            homepage: session_kit_string(kit_json, "homepage"),
            license: session_kit_string(kit_json, "license"),
            lifecycle: Lifecycle::Active,
        };
        fn uuid_at(v: &serde_json::Value, key: &str) -> Option<Uuid> {
            v.get(key).and_then(|x| x.as_str()).and_then(|s| Uuid::parse_str(s).ok())
        }
        fn str_at(v: &serde_json::Value, key: &str) -> Option<String> {
            v.get(key).and_then(|x| x.as_str()).map(str::to_string)
        }

        let mut authors = BTreeMap::new();
        for a in kit_json.get("authors").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(a, "id") {
                authors.insert(id, AuthorState { author_id: id, name: str_at(a, "name").unwrap_or_default(), email: str_at(a, "email"), lifecycle: Lifecycle::Active });
            }
        }
        let mut tags = BTreeMap::new();
        for t in kit_json.get("tags").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(t, "id") {
                tags.insert(id, TagState { tag_id: id, name: str_at(t, "name").unwrap_or_default(), description: str_at(t, "description"), icon: str_at(t, "icon"), lifecycle: Lifecycle::Active });
            }
        }
        let mut concepts = BTreeMap::new();
        for c in kit_json.get("concepts").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(c, "id") {
                concepts.insert(id, ConceptState { concept_id: id, name: str_at(c, "name").unwrap_or_default(), description: str_at(c, "description"), icon: str_at(c, "icon"), lifecycle: Lifecycle::Active });
            }
        }
        let mut ports = BTreeMap::new();
        for p in kit_json.get("ports").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(p, "id") {
                ports.insert(id, PortState { port_id: id, name: str_at(p, "name").unwrap_or_default(), description: str_at(p, "description"), icon: str_at(p, "icon"), max_children: p.get("maxChildren").and_then(|v| v.as_i64()).map(|n| n as i32), compatible_port_ids: Vec::new(), lifecycle: Lifecycle::Active });
            }
        }
        let mut qualities = BTreeMap::new();
        for q in kit_json.get("qualities").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(q, "id") {
                qualities.insert(id, QualityState { quality_id: id, key: str_at(q, "key").unwrap_or_default(), name: str_at(q, "name").unwrap_or_default(), description: str_at(q, "description"), icon: str_at(q, "icon"), unit: str_at(q, "unit"), lifecycle: Lifecycle::Active });
            }
        }
        let mut folders = BTreeMap::new();
        for f in kit_json.get("folders").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(f, "id") {
                folders.insert(id, FolderState { folder_id: id, name: str_at(f, "name").unwrap_or_default(), parent_folder_id: f.get("parent").and_then(|p| uuid_at(p, "id")), description: str_at(f, "description"), lifecycle: Lifecycle::Active });
            }
        }
        let mut files = BTreeMap::new();
        for f in kit_json.get("files").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(f, "id") {
                files.insert(id, FileState { file_id: id, name: str_at(f, "name").unwrap_or_default(), remote: str_at(f, "remote"), folder_id: f.get("folder").and_then(|p| uuid_at(p, "id")), size: f.get("size").and_then(|v| v.as_i64()), hash: str_at(f, "hash"), blob: str_at(f, "blob"), lifecycle: Lifecycle::Active });
            }
        }
        let mut types = BTreeMap::new();
        for t in kit_json.get("types").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(id) = uuid_at(t, "id") {
                types.insert(
                    id,
                    TypeState {
                        type_id: id,
                        name: str_at(t, "name").unwrap_or_else(|| "Untitled".to_string()),
                        parent_type_id: t.get("parent").and_then(|p| uuid_at(p, "id")),
                        description: str_at(t, "description"),
                        icon: str_at(t, "icon"),
                        image: str_at(t, "image"),
                        folder: str_at(t, "folder"),
                        unit: str_at(t, "unit"),
                        stock: t.get("stock").and_then(|v| v.as_i64()).map(|n| n as i32),
                        is_abstract: t.get("isAbstract").and_then(|v| v.as_bool()),
                        virtual_type: t.get("virtual").and_then(|v| v.as_bool()),
                        location_id: t.get("location").and_then(|p| uuid_at(p, "id")),
                        connectors: BTreeMap::new(),
                        representations: BTreeMap::new(),
                        props: BTreeMap::new(),
                        lifecycle: Lifecycle::Active,
                    },
                );
            }
        }
        let mut designs = BTreeMap::new();
        for d in kit_json.get("designs").and_then(|v| v.as_array()).into_iter().flatten() {
            let Some(id) = uuid_at(d, "id") else { continue };
            let mut pieces = BTreeMap::new();
            for p in d.get("pieces").and_then(|v| v.as_array()).into_iter().flatten() {
                let Some(pid) = uuid_at(p, "id") else { continue };
                let center = match (p.get("center").and_then(|c| c.get("u")).and_then(|v| v.as_f64()), p.get("center").and_then(|c| c.get("v")).and_then(|v| v.as_f64())) {
                    (Some(u), Some(v)) => Some([u, v]),
                    _ => None,
                };
                pieces.insert(
                    pid,
                    PieceState {
                        piece_id: pid,
                        name: str_at(p, "name"),
                        type_id: uuid_at(p, "type"),
                        design_ref_id: None,
                        plane: None,
                        center,
                        scale: None,
                        mirror_plane: None,
                        is_hidden: p.get("isHidden").and_then(|v| v.as_bool()),
                        is_locked: p.get("isLocked").and_then(|v| v.as_bool()),
                        color: str_at(p, "color"),
                        description: str_at(p, "description"),
                        lifecycle: Lifecycle::Active,
                    },
                );
            }
            let mut connections = BTreeMap::new();
            for c in d.get("connections").and_then(|v| v.as_array()).into_iter().flatten() {
                let Some(cid) = uuid_at(c, "id") else { continue };
                let parent_piece_id = c.get("parent").and_then(|p| p.get("piece")).and_then(|p| uuid_at(p, "id")).unwrap_or(Uuid::nil());
                let child_piece_id = c.get("child").and_then(|p| p.get("piece")).and_then(|p| uuid_at(p, "id")).unwrap_or(Uuid::nil());
                connections.insert(
                    cid,
                    ConnectionState {
                        connection_id: cid,
                        parent_piece_id,
                        parent_design_piece_id: c.get("parent").and_then(|p| p.get("designPiece")).and_then(|p| uuid_at(p, "id")),
                        parent_connector_id: c.get("parent").and_then(|p| p.get("connector")).and_then(|p| uuid_at(p, "id")),
                        child_piece_id,
                        child_design_piece_id: c.get("child").and_then(|p| p.get("designPiece")).and_then(|p| uuid_at(p, "id")),
                        child_connector_id: c.get("child").and_then(|p| p.get("connector")).and_then(|p| uuid_at(p, "id")),
                        gap: c.get("gap").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        shift: c.get("shift").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        rise: c.get("rise").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        rotation: c.get("rotation").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        turn: c.get("turn").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        tilt: c.get("tilt").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        u: c.get("u").and_then(|v| v.as_f64()),
                        v: c.get("v").and_then(|v| v.as_f64()),
                        description: str_at(c, "description"),
                        lifecycle: Lifecycle::Active,
                    },
                );
            }
            designs.insert(
                id,
                DesignState {
                    design_id: id,
                    name: str_at(d, "name").unwrap_or_else(|| "Untitled".to_string()),
                    parent_design_id: d.get("parent").and_then(|p| uuid_at(p, "id")),
                    description: str_at(d, "description"),
                    icon: str_at(d, "icon"),
                    image: str_at(d, "image"),
                    folder: str_at(d, "folder"),
                    unit: str_at(d, "unit"),
                    is_abstract: d.get("isAbstract").and_then(|v| v.as_bool()),
                    can_scale: d.get("canScale").and_then(|v| v.as_bool()),
                    can_mirror: d.get("canMirror").and_then(|v| v.as_bool()),
                    active_layer_id: d.get("activeLayer").and_then(|p| uuid_at(p, "id")),
                    location_id: d.get("location").and_then(|p| uuid_at(p, "id")),
                    pieces,
                    connections,
                    layers: BTreeMap::new(),
                    groups: BTreeMap::new(),
                    stats: BTreeMap::new(),
                    props: BTreeMap::new(),
                    lifecycle: Lifecycle::Active,
                },
            );
        }
        Ok(SessionState { session_id: SessionId(session_id), domain_version, compose_version, status: SessionStatus::Active, kit, authors, locations: BTreeMap::new(), folders, files, tags, concepts, ports, qualities, types, designs, compose_people: BTreeMap::new() })
    }

    /// 🗄️🌱️ The kit JSON a brand-new session starts from — `initial_kit` (if the caller supplied one)
    /// is used as-is; otherwise an empty kit named `fallback_kit_name`. Unchanged pure logic from the
    /// pre-CW6b implementation, still the single seeding path `create_session` and
    /// `session_state_from_kit_json` both funnel through (fixing a latent pre-CW6b gap: a caller-
    /// supplied `initial_kit`'s nested `types`/`designs` are now actually reflected in the session's
    /// typed state and `db` documents, not just the very first history snapshot row).
    pub fn initial_session_kit(fallback_kit_id: Uuid, fallback_kit_name: &str, initial_kit: Option<&serde_json::Value>) -> Result<(Uuid, String, serde_json::Value), SessionError> {
        match initial_kit {
            Some(kit_json) => Ok((session_kit_id(kit_json)?, session_kit_name(kit_json)?.to_string(), kit_json.clone())),
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
    //#endregion 🔖️StateRehydration

    #[derive(Clone)]
    pub struct SessionDirectory {
        sessions: Arc<DashMap<Uuid, SessionHandle>>,
        db: Arc<db::Database>,
        history: Arc<HistoryStore>,
        directory_store: Arc<ComposeDirectoryStore>,
    }

    impl SessionDirectory {
        pub fn new(db: Arc<db::Database>, history: Arc<HistoryStore>, directory_store: Arc<ComposeDirectoryStore>) -> Self {
            Self { sessions: Arc::new(DashMap::new()), db, history, directory_store }
        }

        /// 🗄️🌱️ Creates a brand-new session: a `db` document (empty at `head_seq == 0`, matching
        /// `domain_version == 0` — no genesis commit is submitted through `db` itself, so the first
        /// REAL domain command still lands at `domain_version == 1`, unchanged from the pre-CW6b
        /// numbering) + directory record + a `HistoryStore` snapshot at version 0 for the initial kit.
        pub async fn create_session(&self, fallback_kit_id: Uuid, fallback_kit_name: &str, initial_kit: Option<&serde_json::Value>) -> Result<(Uuid, Uuid, Uuid), SessionError> {
            let session_id = Uuid::now_v7();
            let (kit_id, _kit_name, kit_json) = initial_session_kit(fallback_kit_id, fallback_kit_name, initial_kit)?;
            self.db.create_document(db::ArtifactSpec::new(document_id(session_id)))?;
            let owner_token = self.directory_store.create_session(session_id, kit_id)?;

            // Validates the kit JSON shape up front (same `session_kit_id`/`session_kit_name`
            // requirements the pre-CW6b implementation enforced at creation time); the typed
            // `SessionState` itself is (re)built lazily by `get_or_activate` on first activation.
            session_state_from_kit_json(session_id, 0, 0, &kit_json)?;

            self.history.record_domain_commit(session_id, 0, session_id)?;
            self.history.store_kit_snapshot(session_id, 0, &kit_json)?;
            Ok((session_id, kit_id, owner_token))
        }

        pub async fn get_or_activate(&self, session_id: SessionId) -> Option<SessionHandle> {
            if let Some(handle) = self.sessions.get(&session_id.0) {
                return Some(handle.clone());
            }
            let db_handle = self.db.document(&document_id(session_id.0)).ok()?;
            let target_version = db_handle.frontier().ok()?.head_seq as DomainVersion;
            let kit_json = self.history.reconstruct_kit_at_version(session_id.0, target_version).ok()?;
            let state = session_state_from_kit_json(session_id.0, target_version, 0, &kit_json).ok()?;

            let entry = self.sessions.entry(session_id.0);
            let handle = match entry {
                dashmap::mapref::entry::Entry::Occupied(o) => o.get().clone(),
                dashmap::mapref::entry::Entry::Vacant(v) => {
                    let (command_tx, command_rx) = mpsc::channel(256);
                    let (event_tx, _) = broadcast::channel(256);
                    let (wire_tx, _) = broadcast::channel(256);
                    let handle = SessionHandle { command_tx, event_tx: event_tx.clone(), wire_tx: wire_tx.clone(), active_connections: Arc::new(AtomicUsize::new(0)), activated_at: Arc::new(Instant::now()) };
                    v.insert(handle.clone());
                    let sessions = self.sessions.clone();
                    let history = self.history.clone();
                    let directory_store = self.directory_store.clone();
                    let sid = session_id.0;
                    tokio::spawn(async move {
                        let mut actor = SessionActor::new(sid, state, db_handle, history, directory_store, event_tx, wire_tx);
                        actor.run(command_rx).await;
                        sessions.remove(&sid);
                        tracing::info!("session actor {} passivated", sid);
                    });
                    handle
                }
            };
            Some(handle)
        }

        pub fn remove(&self, session_id: &Uuid) {
            self.sessions.remove(session_id);
        }

        pub fn deactivate(&self, session_id: SessionId) {
            self.sessions.remove(&session_id.0);
        }

        pub fn db(&self) -> &Arc<db::Database> {
            &self.db
        }

        pub fn history(&self) -> &Arc<HistoryStore> {
            &self.history
        }

        pub fn directory_store(&self) -> &Arc<ComposeDirectoryStore> {
            &self.directory_store
        }

        //#region 🔖️Admin Introspection

        /// Snapshot of all currently-active session actors with WS connection counts.
        pub fn list_active(&self) -> Vec<ActiveSessionInfo> {
            self.sessions
                .iter()
                .map(|entry| {
                    let h = entry.value();
                    ActiveSessionInfo { session_id: *entry.key(), active_connections: h.active_connections.load(AtomicOrdering::Relaxed), activated_at_secs_ago: h.activated_at.elapsed().as_secs() }
                })
                .collect()
        }

        /// Number of currently-active session actors.
        pub fn active_session_count(&self) -> usize {
            self.sessions.len()
        }

        /// Total WS connections across all active sessions.
        pub fn total_active_connections(&self) -> usize {
            self.sessions.iter().map(|e| e.value().active_connections.load(AtomicOrdering::Relaxed)).sum()
        }

        //#endregion 🔖️Admin Introspection
    }
} // 🎯️Directory
pub use directory::*;

mod api {
    // 🛕️Api
    // Specs: AppState holds shared resources. Router defines all HTTP endpoints. Auth enforced via Bearer token: owner token for mutations, viewer/no token for reads. Share tokens provide scoped read-only access.
    // Summary: HTTP API routes for session management, command submission, auth enforcement, and sharable links.

    use super::*;
    use axum::http::HeaderMap;

    #[derive(Clone)]
    pub struct AppState {
        pub directory: SessionDirectory,
    }

    impl AppState {
        pub fn new(directory: SessionDirectory) -> Self {
            Self { directory }
        }
    }

    fn extract_bearer(headers: &HeaderMap) -> Option<String> {
        headers.get("authorization").and_then(|v| v.to_str().ok()).and_then(|v| v.strip_prefix("Bearer ")).map(|s| s.to_string())
    }

    /// 🗄️ `db::Database::document` translated into `SessionError::SessionNotFound` (a clean 404)
    /// rather than the generic `SessionError::Database` 500 a bare `?` would produce — every read/
    /// write handler that needs a live `db::ArtifactHandle` for an EXISTING session goes through this.
    fn open_session_handle(state: &AppState, session_id: Uuid) -> Result<db::ArtifactHandle, SessionError> {
        state.directory.db().document(&document_id(session_id)).map_err(|err| match err {
            db::DbError::NotFound(_) => SessionError::SessionNotFound(session_id.to_string()),
            other => SessionError::Database(other),
        })
    }

    pub fn router(state: AppState) -> Router<()> {
        Router::new()
            .route("/health", get(health))
            .route("/sessions", post(handler_create_session))
            .route("/sessions/{session_id}/snapshot", get(handler_get_snapshot).put(handler_put_snapshot))
            .route("/sessions/{session_id}/commands/domain", post(handler_post_domain_command))
            .route("/sessions/{session_id}/commands/semio_compose_rs", post(handler_post_compose_command))
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

    async fn health() -> &'static str {
        "ok"
    }

    #[derive(Deserialize)]
    pub struct CreateSessionRequest {
        kit_name: String,
        kit: Option<serde_json::Value>,
    }

    #[derive(Serialize)]
    pub struct CreateSessionResponse {
        session_id: Uuid,
        kit_id: Uuid,
        owner_token: Uuid,
    }

    async fn handler_create_session(State(state): State<AppState>, Json(req): Json<CreateSessionRequest>) -> Result<Json<CreateSessionResponse>, SessionError> {
        let fallback_kit_id = Uuid::now_v7();
        let (session_id, kit_id, owner_token) = state.directory.create_session(fallback_kit_id, &req.kit_name, req.kit.as_ref()).await?;
        Ok(Json(CreateSessionResponse { session_id, kit_id, owner_token }))
    }

    async fn handler_get_snapshot(State(state): State<AppState>, Path(session_id): Path<Uuid>) -> Result<Json<SessionSnapshot>, SessionError> {
        let handle = open_session_handle(&state, session_id)?;
        let domain_version = handle.frontier()?.head_seq as DomainVersion;
        let kit = state.directory.history().reconstruct_kit_at_version(session_id, domain_version)?;
        Ok(Json(SessionSnapshot { session_id, domain_version, compose_version: 0, kit }))
    }

    #[derive(Deserialize)]
    pub struct ReplaceSnapshotRequest {
        kit: serde_json::Value,
    }

    async fn handler_put_snapshot(State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap, Json(req): Json<ReplaceSnapshotRequest>) -> Result<Json<SessionSnapshot>, SessionError> {
        let bearer = extract_bearer(&headers);
        let access = state.directory.directory_store().resolve_access(session_id, bearer.as_deref())?;
        if access != AccessMode::Owner {
            return Err(SessionError::Forbidden("write access requires owner token".into()));
        }
        let kit_id = session_kit_id(&req.kit)?;
        let handle = open_session_handle(&state, session_id)?;
        let mut payload = serde_json::Map::new();
        payload.insert(format!("kit/{}", kit_id), req.kit.clone());
        let envelope = protocol::OperationEnvelope {
            operation_id: protocol::OperationId(format!("replace-snapshot:{}:{}", session_id, Uuid::now_v7())),
            document_id: document_id(session_id),
            actor: protocol::ActorId("system".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: serde_json::to_vec(&serde_json::Value::Object(payload)).unwrap_or_default() },
            inverse: protocol::InverseOperation {
                schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::Value::Object(serde_json::Map::new())).unwrap_or_default(),
            },
            timestamp: now_hlc(Uuid::nil()),
        };
        let batch = db::document::CommandBatch::new(vec![envelope])?;
        let receipt = handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync }).await??;
        let domain_version = receipt.frontier.head_seq as DomainVersion;
        state.directory.history().store_kit_snapshot(session_id, domain_version, &req.kit)?;
        state.directory.directory_store().set_session_root_kit(session_id, kit_id)?;
        state.directory.deactivate(SessionId(session_id));
        Ok(Json(SessionSnapshot { session_id, domain_version, compose_version: 0, kit: req.kit }))
    }

    #[derive(Deserialize)]
    pub struct DomainCommandRequest {
        #[serde(flatten)]
        envelope: CommandEnvelope,
        #[serde(flatten)]
        command: DomainCommand,
    }

    async fn handler_post_domain_command(State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap, Json(req): Json<DomainCommandRequest>) -> Result<Json<CommandResult>, SessionError> {
        let bearer = extract_bearer(&headers);
        let access = state.directory.directory_store().resolve_access(session_id, bearer.as_deref())?;
        if access != AccessMode::Owner {
            return Err(SessionError::Forbidden("write access requires owner token".into()));
        }
        let handle = state.directory.get_or_activate(SessionId(session_id)).await.ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
        let (tx, rx) = oneshot::channel();
        handle.command_tx.send(ActorMessage::DomainCommand { envelope: req.envelope, command: req.command, reply: tx }).await.map_err(|_| SessionError::ActorGone)?;
        let result = rx.await.map_err(|_| SessionError::ActorGone)??;
        Ok(Json(result))
    }

    #[derive(Deserialize)]
    pub struct ComposeCommandRequest {
        #[serde(flatten)]
        envelope: ComposeEnvelope,
        #[serde(flatten)]
        command: ComposeCommand,
    }

    async fn handler_post_compose_command(State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap, Json(req): Json<ComposeCommandRequest>) -> Result<Json<serde_json::Value>, SessionError> {
        let bearer = extract_bearer(&headers);
        let access = state.directory.directory_store().resolve_access(session_id, bearer.as_deref())?;
        if access != AccessMode::Owner {
            return Err(SessionError::Forbidden("write access requires owner token".into()));
        }
        let handle = state.directory.get_or_activate(SessionId(session_id)).await.ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
        let (tx, rx) = oneshot::channel();
        handle.command_tx.send(ActorMessage::ComposeCommand { envelope: req.envelope, command: req.command, reply: tx }).await.map_err(|_| SessionError::ActorGone)?;
        rx.await.map_err(|_| SessionError::ActorGone)??;
        Ok(Json(serde_json::json!({"status": "ok"})))
    }

    async fn handler_get_kit_at_lookback(State(state): State<AppState>, Path((session_id, lookback)): Path<(Uuid, String)>) -> Result<Json<serde_json::Value>, SessionError> {
        let kit = state.directory.history().get_kit_at_lookback(session_id, &lookback)?;
        Ok(Json(kit))
    }

    async fn handler_get_kit_at_version(State(state): State<AppState>, Path((session_id, version)): Path<(Uuid, i64)>) -> Result<Json<serde_json::Value>, SessionError> {
        let kit = state.directory.history().reconstruct_kit_at_version(session_id, version)?;
        Ok(Json(kit))
    }

    async fn handler_compact_history(State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap) -> Result<Json<CompactionResult>, SessionError> {
        let bearer = extract_bearer(&headers);
        let access = state.directory.directory_store().resolve_access(session_id, bearer.as_deref())?;
        if access != AccessMode::Owner {
            return Err(SessionError::Forbidden("write access requires owner token".into()));
        }
        let handle = open_session_handle(&state, session_id)?;
        let domain_version = handle.frontier()?.head_seq as DomainVersion;
        let kit = state.directory.history().reconstruct_kit_at_version(session_id, domain_version)?;
        let typed_state = session_state_from_kit_json(session_id, domain_version, 0, &kit)?;
        let result = state.directory.history().compact(session_id, &typed_state)?;
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

    async fn handler_create_share(State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap, Json(req): Json<CreateShareRequest>) -> Result<Json<CreateShareResponse>, SessionError> {
        let bearer = extract_bearer(&headers);
        let access = state.directory.directory_store().resolve_access(session_id, bearer.as_deref())?;
        if access != AccessMode::Owner {
            return Err(SessionError::Forbidden("creating shares requires owner token".into()));
        }
        let mode = match req.access_mode.as_deref() {
            Some("owner") => AccessMode::Owner,
            _ => AccessMode::Viewer,
        };
        let token = state.directory.directory_store().create_share_token(session_id, mode, req.entity_kind.as_deref(), req.entity_id, req.label.as_deref(), req.expires_at.as_deref())?;
        let mode_str = match mode {
            AccessMode::Owner => "owner",
            AccessMode::Viewer => "viewer",
        };
        Ok(Json(CreateShareResponse { token, session_id, access_mode: mode_str.to_string(), entity_kind: req.entity_kind, entity_id: req.entity_id, label: req.label }))
    }

    async fn handler_list_shares(State(state): State<AppState>, Path(session_id): Path<Uuid>, headers: HeaderMap) -> Result<Json<Vec<ShareTokenRow>>, SessionError> {
        let bearer = extract_bearer(&headers);
        let access = state.directory.directory_store().resolve_access(session_id, bearer.as_deref())?;
        if access != AccessMode::Owner {
            return Err(SessionError::Forbidden("listing shares requires owner token".into()));
        }
        let tokens = state.directory.directory_store().list_share_tokens(session_id)?;
        Ok(Json(tokens))
    }

    async fn handler_delete_share(State(state): State<AppState>, Path((session_id, token)): Path<(Uuid, Uuid)>, headers: HeaderMap) -> Result<Json<serde_json::Value>, SessionError> {
        let bearer = extract_bearer(&headers);
        let access = state.directory.directory_store().resolve_access(session_id, bearer.as_deref())?;
        if access != AccessMode::Owner {
            return Err(SessionError::Forbidden("deleting shares requires owner token".into()));
        }
        let deleted = state.directory.directory_store().delete_share_token(token)?;
        Ok(Json(serde_json::json!({"deleted": deleted})))
    }

    async fn handler_resolve_share(State(state): State<AppState>, Path(token): Path<Uuid>) -> Result<Json<ResolvedShareToken>, SessionError> {
        let resolved = state.directory.directory_store().resolve_share_token(token)?;
        Ok(Json(resolved))
    }
} // 🛕️Api
pub use api::*;

mod ws {
    // 🤖️Ws
    // Specs: WebSocket handler upgrades HTTP to WS and speaks `protocol_wire`'s binary lane-tagged
    // `ClientFrame`/`ServerFrame` frames — CW6b's "semio_compose_rs client sync moves to wire v2" (the pre-
    // CW6b handler sent ad hoc JSON text and never actually parsed anything a client sent back).
    // Summary: WebSocket handler: binary `protocol_wire` frames for real-time session command/preview
    // streaming.

    use super::*;

    pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>, Path(session_id): Path<Uuid>) -> impl IntoResponse {
        ws.on_upgrade(move |socket| handle_socket(socket, state, session_id))
    }

    async fn handle_socket(socket: WebSocket, state: AppState, session_id: Uuid) {
        let handle = match state.directory.get_or_activate(SessionId(session_id)).await {
            Some(h) => h,
            None => {
                tracing::warn!("ws: session {} not found", session_id);
                return;
            }
        };
        //#region 🔖️Connection Accounting
        handle.active_connections.fetch_add(1, AtomicOrdering::Relaxed);
        let conn_counter = handle.active_connections.clone();
        struct Decrement(Arc<AtomicUsize>);
        impl Drop for Decrement {
            fn drop(&mut self) {
                self.0.fetch_sub(1, AtomicOrdering::Relaxed);
            }
        }
        let _guard = Decrement(conn_counter);
        //#endregion 🔖️Connection Accounting

        let Ok(db_handle) = state.directory.db().document(&document_id(session_id)) else {
            tracing::warn!("ws: session {} has no db document", session_id);
            return;
        };
        let server_frontier = match db_handle.frontier() {
            Ok(f) => frontier_to_wire(&f),
            Err(e) => {
                tracing::warn!("ws: frontier lookup failed for {}: {}", session_id, e);
                return;
            }
        };

        let mut wire_rx = handle.wire_tx.subscribe();
        let (mut ws_tx, mut ws_rx) = socket.split();

        let welcome = protocol::ServerFrame::Welcome { session_id: session_id.to_string(), resume_token: Uuid::now_v7().to_string(), server_frontier, bootstrap: protocol::Bootstrap::Tail };
        if ws_tx.send(Message::Binary(protocol::encode_server_frame(&welcome, protocol::Lane::Command).into())).await.is_err() {
            return;
        }

        loop {
            tokio::select! {
                event = wire_rx.recv() => {
                    let Ok(event) = event else { break };
                    let (frame, lane) = match event {
                        WireEvent::Commands { envelope, frontier } => {
                            let origin = envelope.actor.clone();
                            (protocol::ServerFrame::Commands { envelopes: vec![*envelope], origin, frontier: frontier_to_wire(&frontier) }, protocol::Lane::Command)
                        }
                        WireEvent::Preview { actor, key, seq, payload } => (protocol::ServerFrame::Preview { actor, key, seq, payload }, protocol::Lane::Preview),
                    };
                    if ws_tx.send(Message::Binary(protocol::encode_server_frame(&frame, lane).into())).await.is_err() {
                        break;
                    }
                }
                incoming = ws_rx.next() => {
                    let Some(Ok(msg)) = incoming else { break };
                    match msg {
                        Message::Close(_) => break,
                        Message::Binary(bytes) => {
                            match protocol::decode_client_frame(&bytes) {
                                Ok((_lane, frame)) => {
                                    if handle_client_frame(&db_handle, &handle, session_id, frame, &mut ws_tx).await.is_err() {
                                        break;
                                    }
                                }
                                Err(e) => tracing::debug!("ws: malformed client frame from {}: {}", session_id, e),
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        tracing::debug!("ws connection closed for session {}", session_id);
    }

    /// 🎞️ Handles one decoded `ClientFrame`. `Commands` submits its envelopes straight through
    /// `db::ArtifactHandle::submit` (this is the wire-native path — raw `db_artifact` path-value
    /// diffs, not semio_compose_rs's typed `DomainCommand`s, so unlike the HTTP `/commands/domain` route it
    /// does not update `SessionActor`'s typed `SessionState`/`EntityChange` history; reconciling wire-
    /// native diffs back into semio_compose_rs's typed replica is future work for the client-side rewrite this
    /// wave scoped out — see this ticket's report). `Hello`/`FrontierAdvertise`/`CreditGrant` are
    /// acknowledged but otherwise inert (no credit-based flow control or resume-token validation
    /// implemented yet, matching `db_sync`'s own current scope).
    async fn handle_client_frame(db_handle: &db::ArtifactHandle, session: &SessionHandle, session_id: Uuid, frame: protocol::ClientFrame, ws_tx: &mut futures::stream::SplitSink<WebSocket, Message>) -> Result<(), ()> {
        match frame {
            protocol::ClientFrame::Hello { .. } | protocol::ClientFrame::FrontierAdvertise { .. } | protocol::ClientFrame::CreditGrant { .. } => Ok(()),
            protocol::ClientFrame::Commands { batch_id, envelopes } => {
                if envelopes.is_empty() {
                    return Ok(());
                }
                let stage = match db::document::CommandBatch::new(envelopes) {
                    Ok(batch) => match db_handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync }).await {
                        Ok(Ok(_receipt)) => protocol::AckStage::Applied { outcome: Box::new(protocol::ApplyOutcome::Accepted) },
                        Ok(Err(e)) | Err(e) => protocol::AckStage::Applied { outcome: Box::new(protocol::ApplyOutcome::Rejected { reason: e.to_string() }) },
                    },
                    Err(e) => protocol::AckStage::Applied { outcome: Box::new(protocol::ApplyOutcome::Rejected { reason: e.to_string() }) },
                };
                let frontier = db_handle.frontier().map_or(protocol::RuntimeFrontierSummary { document_id: document_id(session_id), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] }, |f| frontier_to_wire(&f));
                let ack = protocol::ServerFrame::Ack { batch_id, stages: vec![protocol::AckStage::Received, protocol::AckStage::Persisted, stage], frontier };
                ws_tx.send(Message::Binary(protocol::encode_server_frame(&ack, protocol::Lane::Command).into())).await.map_err(|_| ())
            }
            protocol::ClientFrame::PreviewPublish { key, seq, payload } => {
                let _ = session.wire_tx.send(WireEvent::Preview { actor: protocol::ActorId(session_id.to_string()), key, seq, payload });
                Ok(())
            }
            protocol::ClientFrame::Presence { peer_json } => {
                let frame = protocol::ServerFrame::Presence { peers_json: vec![peer_json] };
                ws_tx.send(Message::Binary(protocol::encode_server_frame(&frame, protocol::Lane::Preview).into())).await.map_err(|_| ())
            }
            protocol::ClientFrame::Bye => Err(()),
        }
    }
} // 🤖️Ws
pub use ws::*;

mod admin {
    // 🛡️Admin
    // Specs: Server-admin HTTP surface protected by a shared bearer token (COMPOSE_ADMIN_TOKEN). Exposes read-only introspection
    // (sessions, kits, persons, share tokens, connections, config) and targeted write operations (close/passivate a session,
    // revoke a share token, update compaction config). When COMPOSE_ADMIN_TOKEN is unset the /admin/* endpoints return 503 so
    // an unconfigured deployment never silently exposes itself. A single embedded HTML dashboard aggregates all views for
    // human operators; it calls the same JSON endpoints over fetch() with the bearer token supplied at sign-in.
    // Summary: Server-admin dashboard, introspection endpoints, and configuration API for semio_compose_rs-semio_hub.

    use super::*;
    use axum::http::HeaderMap;

    //#region 🔖️AdminConfig

    /// Process-global admin configuration. Populated from environment at startup.
    #[derive(Clone)]
    pub struct AdminConfig {
        pub admin_token: Option<String>,
        pub started_at: Arc<Instant>,
    }

    impl AdminConfig {
        pub fn from_env() -> Self {
            let admin_token = std::env::var("COMPOSE_ADMIN_TOKEN").ok().filter(|s| !s.is_empty());
            Self { admin_token, started_at: Arc::new(Instant::now()) }
        }
    }

    //#endregion 🔖️AdminConfig

    //#region 🔖️AdminAuth

    /// Validates Bearer token against configured admin token. Returns error if token is unset or wrong.
    pub fn require_admin(headers: &HeaderMap, config: &AdminConfig) -> Result<(), SessionError> {
        let expected = match &config.admin_token {
            Some(t) => t,
            None => return Err(SessionError::Forbidden("admin endpoints disabled: COMPOSE_ADMIN_TOKEN is not set".into())),
        };
        let provided = headers.get("authorization").and_then(|v| v.to_str().ok()).and_then(|v| v.strip_prefix("Bearer ")).unwrap_or("");
        if provided.is_empty() {
            return Err(SessionError::Unauthorized("admin token required".into()));
        }
        if provided != expected.as_str() {
            return Err(SessionError::Unauthorized("admin token invalid".into()));
        }
        Ok(())
    }

    //#endregion 🔖️AdminAuth

    //#region 🔖️AdminRows

    #[derive(Debug, Serialize)]
    pub struct AdminSessionRow {
        pub session_id: Uuid,
        pub root_kit_id: Uuid,
        pub status: String,
        pub domain_version: i64,
        pub compose_version: i64,
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

    //#endregion 🔖️AdminRows

    //#region 🔖️AdminQueries
    // 🗄️ Reads through `SessionDirectory` (`db` frontier for kit identity + `ComposeDirectoryStore`
    // for session/share-token bookkeeping) instead of `sqlx_core::query_as` row tuples against
    // Postgres — every function here keeps its pre-CW6b name/shape so `AdminHandlers` below barely
    // changes.

    fn admin_session_row(directory: &SessionDirectory, session_id: Uuid, record: &SessionRecord) -> AdminSessionRow {
        let active = directory.list_active();
        let is_activated = active.iter().any(|a| a.session_id == session_id);
        let active_connections = active.iter().find(|a| a.session_id == session_id).map_or(0, |a| a.active_connections);
        let domain_version = directory.db().document(&document_id(session_id)).ok().and_then(|h| h.frontier().ok()).map_or(0, |f| f.head_seq as i64);
        let status = match record.status {
            SessionStatus::Active => "active",
            SessionStatus::Passivated => "passivated",
            SessionStatus::Closed => "closed",
        };
        AdminSessionRow { session_id, root_kit_id: record.root_kit_id, status: status.to_string(), domain_version, compose_version: 0, created_at: record.created_at.clone(), updated_at: record.updated_at.clone(), active_connections, is_activated }
    }

    pub fn load_admin_overview(directory: &SessionDirectory, started_at: Instant) -> Result<AdminOverview, SessionError> {
        let sessions = directory.directory_store().all_sessions();
        let active_sessions = sessions.iter().filter(|(_, r)| r.status == SessionStatus::Active).count() as i64;
        let closed_sessions = sessions.iter().filter(|(_, r)| r.status == SessionStatus::Closed).count() as i64;
        let passivated_sessions = sessions.iter().filter(|(_, r)| r.status == SessionStatus::Passivated).count() as i64;
        Ok(AdminOverview {
            uptime_secs: started_at.elapsed().as_secs(),
            total_sessions: sessions.len() as i64,
            active_sessions,
            passivated_sessions,
            closed_sessions,
            total_kits: sessions.len() as i64,
            total_persons: 0,
            total_share_tokens: directory.directory_store().all_share_tokens().len() as i64,
            active_actors: directory.active_session_count(),
            active_connections: directory.total_active_connections(),
        })
    }

    pub fn load_admin_sessions(directory: &SessionDirectory) -> Result<Vec<AdminSessionRow>, SessionError> {
        Ok(directory.directory_store().all_sessions().into_iter().map(|(id, record)| admin_session_row(directory, id, &record)).collect())
    }

    pub fn load_admin_session_detail(directory: &SessionDirectory, session_id: Uuid) -> Result<AdminSessionDetail, SessionError> {
        let record = directory.directory_store().session_record(session_id)?;
        let row = admin_session_row(directory, session_id, &record);
        let kit_name = directory
            .db()
            .document(&document_id(session_id))
            .ok()
            .and_then(|h| h.frontier().ok())
            .and_then(|f| directory.history().reconstruct_kit_at_version(session_id, f.head_seq as DomainVersion).ok())
            .and_then(|kit| session_kit_name(&kit).ok().map(str::to_string))
            .unwrap_or_else(|| "Untitled".to_string());
        let kit = Some(AdminKitRow { session_id, kit_id: record.root_kit_id, name: kit_name, version: None, remote: None, lifecycle: "active".to_string() });
        let share_tokens = directory.directory_store().list_share_tokens(session_id)?.into_iter().map(|t| AdminShareTokenRow { token: t.token, session_id: t.session_id, access_mode: t.access_mode, entity_kind: t.entity_kind, entity_id: t.entity_id, label: t.label, created_at: t.created_at, expires_at: t.expires_at }).collect();
        Ok(AdminSessionDetail { row, kit, persons: Vec::new(), share_tokens })
    }

    pub fn load_admin_kits(directory: &SessionDirectory) -> Result<Vec<AdminKitRow>, SessionError> {
        let mut kits: Vec<AdminKitRow> = Vec::new();
        for (session_id, record) in directory.directory_store().all_sessions() {
            let name = directory
                .db()
                .document(&document_id(session_id))
                .ok()
                .and_then(|h| h.frontier().ok())
                .and_then(|f| directory.history().reconstruct_kit_at_version(session_id, f.head_seq as DomainVersion).ok())
                .and_then(|kit| session_kit_name(&kit).ok().map(str::to_string))
                .unwrap_or_else(|| "Untitled".to_string());
            kits.push(AdminKitRow { session_id, kit_id: record.root_kit_id, name, version: None, remote: None, lifecycle: "active".to_string() });
        }
        kits.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(kits)
    }

    /// 🌫️ Presence is ephemeral, in-actor-only (never persisted — see `actor`'s doc), so listing it
    /// admin-wide would require polling every active actor's mailbox; deferred (empty for now, still
    /// returns 200 so dashboard/API consumers don't break — flagged in this ticket's report).
    pub fn load_admin_persons() -> Vec<AdminPersonRow> {
        Vec::new()
    }

    pub fn load_admin_share_tokens(directory: &SessionDirectory) -> Result<Vec<AdminShareTokenRow>, SessionError> {
        let mut rows: Vec<(String, AdminShareTokenRow)> = directory
            .directory_store()
            .all_share_tokens()
            .into_iter()
            .map(|(token, r)| {
                let mode = match r.access_mode {
                    AccessMode::Owner => "owner",
                    AccessMode::Viewer => "viewer",
                };
                (r.created_at.clone(), AdminShareTokenRow { token, session_id: r.session_id, access_mode: mode.to_string(), entity_kind: r.entity_kind, entity_id: r.entity_id, label: r.label, created_at: r.created_at, expires_at: r.expires_at })
            })
            .collect();
        rows.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(rows.into_iter().map(|(_, row)| row).collect())
    }

    pub fn admin_close_session(directory: &SessionDirectory, session_id: Uuid) -> Result<bool, SessionError> {
        let closed = directory.directory_store().close_session(session_id)?;
        if closed {
            directory.remove(&session_id);
        }
        Ok(closed)
    }

    pub fn admin_load_compaction_config(directory: &SessionDirectory, session_id: Uuid) -> Result<AdminCompactionConfig, SessionError> {
        directory.directory_store().session_record(session_id)?; // 404 if unknown session
        let tokens = directory.directory_store().compaction_config(session_id);
        let last_compacted_at = directory.history().last_compacted_at(session_id);
        Ok(AdminCompactionConfig { session_id, lookback_tokens: tokens, last_compacted_at })
    }

    pub fn admin_update_compaction_config(directory: &SessionDirectory, session_id: Uuid, tokens: Vec<String>) -> Result<AdminCompactionConfig, SessionError> {
        directory.directory_store().set_compaction_config(session_id, tokens)?;
        admin_load_compaction_config(directory, session_id)
    }

    //#endregion 🔖️AdminQueries

    //#region 🔖️AdminHandlers

    #[derive(Clone)]
    pub struct AdminState {
        pub directory: SessionDirectory,
        pub config: AdminConfig,
    }

    pub fn admin_router(state: AdminState) -> Router<()> {
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
        let overview = load_admin_overview(&s.directory, *s.config.started_at)?;
        Ok(Json(overview))
    }

    async fn handler_list_sessions(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminSessionRow>>, SessionError> {
        require_admin(&headers, &s.config)?;
        Ok(Json(load_admin_sessions(&s.directory)?))
    }

    async fn handler_session_detail(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<AdminSessionDetail>, SessionError> {
        require_admin(&headers, &s.config)?;
        Ok(Json(load_admin_session_detail(&s.directory, session_id)?))
    }

    async fn handler_passivate_session(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<serde_json::Value>, SessionError> {
        require_admin(&headers, &s.config)?;
        s.directory.remove(&session_id);
        Ok(Json(serde_json::json!({"passivated": true, "session_id": session_id})))
    }

    async fn handler_close_session(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<serde_json::Value>, SessionError> {
        require_admin(&headers, &s.config)?;
        let ok = admin_close_session(&s.directory, session_id)?;
        if !ok {
            return Err(SessionError::SessionNotFound(session_id.to_string()));
        }
        Ok(Json(serde_json::json!({"closed": true, "session_id": session_id})))
    }

    async fn handler_list_kits(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminKitRow>>, SessionError> {
        require_admin(&headers, &s.config)?;
        Ok(Json(load_admin_kits(&s.directory)?))
    }

    async fn handler_list_persons(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminPersonRow>>, SessionError> {
        require_admin(&headers, &s.config)?;
        Ok(Json(load_admin_persons()))
    }

    async fn handler_list_share_tokens(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<AdminShareTokenRow>>, SessionError> {
        require_admin(&headers, &s.config)?;
        Ok(Json(load_admin_share_tokens(&s.directory)?))
    }

    async fn handler_revoke_share_token(State(s): State<AdminState>, headers: HeaderMap, Path(token): Path<Uuid>) -> Result<Json<serde_json::Value>, SessionError> {
        require_admin(&headers, &s.config)?;
        let deleted = s.directory.directory_store().delete_share_token(token)?;
        Ok(Json(serde_json::json!({"revoked": deleted, "token": token})))
    }

    async fn handler_list_connections(State(s): State<AdminState>, headers: HeaderMap) -> Result<Json<Vec<ActiveSessionInfo>>, SessionError> {
        require_admin(&headers, &s.config)?;
        Ok(Json(s.directory.list_active()))
    }

    async fn handler_get_config(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>) -> Result<Json<AdminCompactionConfig>, SessionError> {
        require_admin(&headers, &s.config)?;
        Ok(Json(admin_load_compaction_config(&s.directory, session_id)?))
    }

    #[derive(Deserialize)]
    pub struct PatchConfigBody {
        pub lookback_tokens: Vec<String>,
    }

    async fn handler_patch_config(State(s): State<AdminState>, headers: HeaderMap, Path(session_id): Path<Uuid>, Json(body): Json<PatchConfigBody>) -> Result<Json<AdminCompactionConfig>, SessionError> {
        require_admin(&headers, &s.config)?;
        let known: std::collections::HashSet<&'static str> = lookback_tokens().iter().copied().collect();
        for t in &body.lookback_tokens {
            if !known.contains(t.as_str()) {
                return Err(SessionError::Validation(format!("unknown lookback token: {}", t)));
            }
        }
        Ok(Json(admin_update_compaction_config(&s.directory, session_id, body.lookback_tokens)?))
    }

    //#endregion 🔖️AdminHandlers

    //#region 🔖️Dashboard HTML

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
    <p class="muted">Enter the admin bearer token configured via <code>COMPOSE_ADMIN_TOKEN</code>.</p>
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
const LS_KEY = 'semio_compose_rs.admin.token';
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
  return `<table><thead><tr><th>Session</th><th>Kit</th><th>Status</th><th>Domain v</th><th>Compose v</th><th>Conn</th><th>Actor</th><th>Updated</th><th></th></tr></thead><tbody>${rows.map(r => `
    <tr>
      <td><code>${esc(r.session_id)}</code></td>
      <td><code class="muted">${esc(r.root_kit_id)}</code></td>
      <td class="${r.status === 'active' ? 'ok' : (r.status === 'closed' ? 'danger' : 'muted')}">${esc(r.status)}</td>
      <td>${r.domain_version}</td>
      <td>${r.compose_version}</td>
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
      <div class="card"><div class="k">Compose v</div><div class="v">${d.row.compose_version}</div></div>
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
    //#endregion 🔖️Dashboard HTML
} // 🛡️Admin
pub use admin::*;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "semio_compose_hub=debug,tower_http=debug".into())).init();
    // 🗄️ Zero-touch data root: `db/` (FsStorage-backed `db::Database`), `directory.json` + `history/`
    // (semio_compose_rs-semio_hub's own file-backed session/share-token/kit-history bookkeeping) — replaces
    // `DATABASE_URL`/Postgres entirely.
    let data_dir = std::env::var("COMPOSE_HUB_DATA").map_or_else(|_| std::path::PathBuf::from("./.🧬semio/🌐hub/compose-rs"), std::path::PathBuf::from);
    let database = match open_database(&data_dir) {
        Ok(database) => database,
        Err(e) => {
            tracing::error!("failed to open db database at {}: {e}", data_dir.display());
            return std::process::ExitCode::FAILURE;
        }
    };
    let directory_store = match ComposeDirectoryStore::open(&data_dir) {
        Ok(store) => store,
        Err(e) => {
            tracing::error!("failed to open semio_compose_rs-semio_hub directory store: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let history_store = HistoryStore::open(&data_dir);
    let directory = SessionDirectory::new(Arc::new(database), Arc::new(history_store), Arc::new(directory_store));

    let admin_config = AdminConfig::from_env();
    if admin_config.admin_token.is_none() {
        tracing::warn!("COMPOSE_ADMIN_TOKEN is not set: /admin/* endpoints will return 403");
    } else {
        tracing::info!("admin dashboard mounted at /admin");
    }
    let app_state = AppState::new(directory.clone());
    let admin_state = AdminState { directory, config: admin_config };
    let app_router = router(app_state).merge(admin_router(admin_state));
    let default_host = if std::env::var("DEVCONTAINER").as_deref() == Ok("true") { "0.0.0.0" } else { "127.0.0.1" };
    let addr: std::net::SocketAddr = match std::env::var("LISTEN_ADDR").unwrap_or_else(|_| format!("{}:8080", default_host)).parse() {
        Ok(addr) => addr,
        Err(e) => {
            tracing::error!("invalid LISTEN_ADDR: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    tracing::info!("semio_compose_rs-semio_hub listening on {}", addr);
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("bind {addr}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    if let Err(e) = axum::serve(listener, app_router).await {
        tracing::error!("server: {e}");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}

// 🔖️Main End

#[cfg(test)]
// Specs: Tests cover domain types, commands, events, serde, error HTTP mapping, and integration with metabolism/nakagin data.
// Summary: Comprehensive tests for all domain types, serialization, error mapping, and integration with real asset data.
mod tests {
    // 📐️Tests

    use super::*;
    mod domain_tests {
        // 👓️Domain Tests

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
            let kinds = vec![EntityKind::Kit, EntityKind::Type, EntityKind::Design, EntityKind::Piece, EntityKind::Connection, EntityKind::Author];
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
            if let Lifecycle::Tombstoned { at, .. } = l {
                assert_eq!(at, 42);
            } else {
                panic!();
            }
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
    } // 👓️Domain Tests

    mod command_tests {
        // 📜️Command Tests

        use super::*;
        #[test]
        pub fn command_envelope_serde() {
            let env = CommandEnvelope { command_id: CommandId(Uuid::nil()), client_id: ClientId(Uuid::nil()), request_id: RequestId(Uuid::nil()), actor_person_id: PersonId(Uuid::nil()), base_domain_version: 0 };
            let json = serde_json::to_value(&env).unwrap();
            assert!(json.get("command_id").is_some());
        }

        #[test]
        pub fn create_type_command_serde() {
            let cmd = DomainCommand::CreateType(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": "TestType"}) });
            let json = serde_json::to_string(&cmd).unwrap();
            assert!(json.contains("CreateType"));
        }

        #[test]
        pub fn delete_type_command_serde() {
            let cmd = DomainCommand::DeleteType(DeleteEntity { entity_id: Uuid::now_v7() });
            let json = serde_json::to_string(&cmd).unwrap();
            assert!(json.contains("DeleteType"));
        }

        #[test]
        pub fn create_design_command_serde() {
            let cmd = DomainCommand::CreateDesign(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": "TestDesign"}) });
            let json = serde_json::to_string(&cmd).unwrap();
            assert!(json.contains("CreateDesign"));
        }

        #[test]
        pub fn create_piece_command_serde() {
            let cmd = DomainCommand::CreatePiece(CreatePiece { piece_id: Uuid::now_v7(), design_id: Uuid::now_v7(), fields: serde_json::json!({"name": "piece_a"}) });
            let json = serde_json::to_string(&cmd).unwrap();
            assert!(json.contains("CreatePiece"));
        }

        #[test]
        pub fn create_connection_command_serde() {
            let cmd = DomainCommand::CreateConnection(CreateConnection { connection_id: Uuid::now_v7(), design_id: Uuid::now_v7(), fields: serde_json::json!({"parent_piece_id": Uuid::nil().to_string()}) });
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
        pub fn compose_command_cursor_serde() {
            let cmd = ComposeCommand::UpsertCursor(UpsertCursor { u: 1.0, v: 2.0 });
            let json = serde_json::to_string(&cmd).unwrap();
            assert!(json.contains("UpsertCursor"));
        }

        #[test]
        pub fn compose_command_look_serde() {
            let cmd = ComposeCommand::UpsertLook(UpsertLook { position: [1.0, 2.0, 3.0], forward: [0.0, 0.0, 1.0], up: [0.0, 1.0, 0.0] });
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
    } // 📜️Command Tests

    mod error_tests {
        // 🌤️Error Tests

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

    mod event_tests {
        // 🔮️Event Tests

        use super::*;
        #[test]
        pub fn session_event_domain_accepted_serde() {
            let ev =
                SessionEvent::DomainCommandAccepted { command_id: CommandId(Uuid::nil()), domain_version: 1, changes: vec![EntityChange::Created { entity_kind: EntityKind::Type, entity_id: Uuid::nil(), snapshot: serde_json::json!({"name": "T"}) }] };
            let json = serde_json::to_string(&ev).unwrap();
            assert!(json.contains("DomainCommandAccepted"));
            let back: SessionEvent = serde_json::from_str(&json).unwrap();
            assert!(matches!(back, SessionEvent::DomainCommandAccepted { .. }));
        }

        #[test]
        pub fn session_event_compose_updated_serde() {
            let ev = SessionEvent::ComposeUpdated { compose_version: 3, person_id: PersonId(Uuid::nil()), frontend_id: "desktop".into(), update: ComposeUpdate::CursorMoved { u: 1.0, v: 2.0 } };
            let json = serde_json::to_string(&ev).unwrap();
            assert!(json.contains("ComposeUpdated"));
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
        pub fn compose_update_variants_serde() {
            let updates = vec![
                ComposeUpdate::CursorMoved { u: 0.5, v: 0.5 },
                ComposeUpdate::LookChanged { position: [1.0, 2.0, 3.0], forward: [0.0, 0.0, 1.0], up: [0.0, 1.0, 0.0] },
                ComposeUpdate::SelectionChanged { piece_ids: vec![Uuid::nil()], design_ids: vec![] },
                ComposeUpdate::PresenceCleared,
            ];
            for u in updates {
                let json = serde_json::to_string(&u).unwrap();
                let _back: ComposeUpdate = serde_json::from_str(&json).unwrap();
            }
        }
    } // 🔮️Event Tests

    mod state_tests {
        // 📝️State Tests

        use super::*;
        #[test]
        pub fn session_state_creation() {
            let sid = Uuid::now_v7();
            let kid = Uuid::now_v7();
            let state = SessionState {
                session_id: SessionId(sid),
                domain_version: 0,
                compose_version: 0,
                status: SessionStatus::Active,
                kit: KitState { kit_id: kid, name: "Test".into(), version: None, description: None, icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
                authors: BTreeMap::new(),
                locations: BTreeMap::new(),
                folders: BTreeMap::new(),
                files: BTreeMap::new(),
                tags: BTreeMap::new(),
                concepts: BTreeMap::new(),
                ports: BTreeMap::new(),
                qualities: BTreeMap::new(),
                types: BTreeMap::new(),
                designs: BTreeMap::new(),
                compose_people: BTreeMap::new(),
            };
            assert_eq!(state.session_id.0, sid);
            assert_eq!(state.kit.name, "Test");
        }

        #[test]
        pub fn type_state_with_connectors() {
            let tid = Uuid::now_v7();
            let cid = Uuid::now_v7();
            let mut ts = TypeState {
                type_id: tid,
                name: "Box".into(),
                parent_type_id: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location_id: None,
                connectors: BTreeMap::new(),
                representations: BTreeMap::new(),
                props: BTreeMap::new(),
                lifecycle: Lifecycle::Active,
            };
            ts.connectors.insert(
                cid,
                ConnectorState { connector_id: cid, name: Some("top".into()), t: 0.5, point: [0.0, 0.0, 1.0], direction: [0.0, 0.0, 1.0], description: None, port_id: None, mandatory: Some(true), max_children: None, lifecycle: Lifecycle::Active },
            );
            assert_eq!(ts.connectors.len(), 1);
        }

        #[test]
        pub fn design_state_with_pieces_and_connections() {
            let did = Uuid::now_v7();
            let p1 = Uuid::now_v7();
            let p2 = Uuid::now_v7();
            let conn_id = Uuid::now_v7();
            let mut ds = DesignState {
                design_id: did,
                name: "Tower".into(),
                parent_design_id: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                is_abstract: None,
                can_scale: None,
                can_mirror: None,
                active_layer_id: None,
                location_id: None,
                pieces: BTreeMap::new(),
                connections: BTreeMap::new(),
                layers: BTreeMap::new(),
                groups: BTreeMap::new(),
                stats: BTreeMap::new(),
                props: BTreeMap::new(),
                lifecycle: Lifecycle::Active,
            };
            ds.pieces.insert(
                p1,
                PieceState {
                    piece_id: p1,
                    name: Some("a".into()),
                    type_id: None,
                    design_ref_id: None,
                    plane: None,
                    center: Some([0.0, 0.0]),
                    scale: None,
                    mirror_plane: None,
                    is_hidden: None,
                    is_locked: None,
                    color: None,
                    description: None,
                    lifecycle: Lifecycle::Active,
                },
            );
            ds.pieces.insert(
                p2,
                PieceState {
                    piece_id: p2,
                    name: Some("b".into()),
                    type_id: None,
                    design_ref_id: None,
                    plane: None,
                    center: None,
                    scale: None,
                    mirror_plane: None,
                    is_hidden: None,
                    is_locked: None,
                    color: None,
                    description: None,
                    lifecycle: Lifecycle::Active,
                },
            );
            ds.connections.insert(
                conn_id,
                ConnectionState {
                    connection_id: conn_id,
                    parent_piece_id: p1,
                    parent_design_piece_id: None,
                    parent_connector_id: None,
                    child_piece_id: p2,
                    child_design_piece_id: None,
                    child_connector_id: None,
                    gap: 0.0,
                    shift: 0.0,
                    rise: 0.0,
                    rotation: 0.0,
                    turn: 0.0,
                    tilt: 0.0,
                    u: None,
                    v: None,
                    description: None,
                    lifecycle: Lifecycle::Active,
                },
            );
            assert_eq!(ds.pieces.len(), 2);
            assert_eq!(ds.connections.len(), 1);
        }
    } // 📝️State Tests

    mod metabolism_integration_tests {
        // 🔐️Metabolism Integration Tests

        use super::*;
        pub fn load_metabolism_kit_json() -> serde_json::Value {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().parent().unwrap().join("fixture/metabolism.shallow.kit.semio_compose_rs.json");
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
            // 🧾️ The fixture's `authors` is content-addressed (`{hash, items: [...]}`), not a bare
            // array — matches every other content-addressed collection in this fixture family.
            let authors = kit["authors"]["items"].as_array().expect("authors items array");
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
                session_id: SessionId(Uuid::now_v7()),
                domain_version: 0,
                compose_version: 0,
                status: SessionStatus::Active,
                kit: KitState { kit_id, name: "Metabolism".into(), version: Some("1.0".into()), description: None, icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
                authors: BTreeMap::new(),
                locations: BTreeMap::new(),
                folders: BTreeMap::new(),
                files: BTreeMap::new(),
                tags: BTreeMap::new(),
                concepts: BTreeMap::new(),
                ports: BTreeMap::new(),
                qualities: BTreeMap::new(),
                types: BTreeMap::new(),
                designs: BTreeMap::new(),
                compose_people: BTreeMap::new(),
            };
            for t in types_json {
                let id = Uuid::parse_str(t["id"].as_str().unwrap()).unwrap();
                let name = t["name"].as_str().unwrap().to_string();
                let parent_id = t.get("parent").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok());
                let desc = t.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                let icon = t.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string());
                state.types.insert(
                    id,
                    TypeState {
                        type_id: id,
                        name,
                        parent_type_id: parent_id,
                        description: desc,
                        icon,
                        image: None,
                        folder: None,
                        unit: None,
                        stock: None,
                        is_abstract: None,
                        virtual_type: None,
                        location_id: None,
                        connectors: BTreeMap::new(),
                        representations: BTreeMap::new(),
                        props: BTreeMap::new(),
                        lifecycle: Lifecycle::Active,
                    },
                );
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
    } // 🔐️Metabolism Integration Tests
    pub use metabolism_integration_tests::*;

    mod nakagin_integration_tests {
        // 🎵️Nakagin Integration Tests

        use super::*;
        pub fn load_nakagin_design_json() -> serde_json::Value {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().parent().unwrap().join("fixture/nakagin-capsule-tower.shallow.design.semio_compose_rs.json");
            let data = std::fs::read_to_string(&path).expect("nakagin design JSON");
            serde_json::from_str(&data).expect("parse nakagin design JSON")
        }

        pub fn load_nakagin_diff_json() -> serde_json::Value {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().parent().unwrap().join("fixture/nakagin-capsule-tower.with-diff.design.semio_compose_rs.json");
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
                design_id,
                name: design_json["name"].as_str().unwrap().to_string(),
                parent_design_id: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: design_json.get("unit").and_then(|v| v.as_str()).map(|s| s.to_string()),
                is_abstract: None,
                can_scale: None,
                can_mirror: None,
                active_layer_id: None,
                location_id: None,
                pieces: BTreeMap::new(),
                connections: BTreeMap::new(),
                layers: BTreeMap::new(),
                groups: BTreeMap::new(),
                stats: BTreeMap::new(),
                props: BTreeMap::new(),
                lifecycle: Lifecycle::Active,
            };
            for p in design_json["pieces"].as_array().unwrap() {
                let pid = Uuid::parse_str(p["id"].as_str().unwrap()).unwrap();
                let name = p.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
                let center = p.get("center").and_then(|c| {
                    let u = c.get("u")?.as_f64()?;
                    let v = c.get("v")?.as_f64()?;
                    Some([u, v])
                });
                ds.pieces.insert(
                    pid,
                    PieceState {
                        piece_id: pid,
                        name,
                        type_id: None,
                        design_ref_id: None,
                        plane: None,
                        center,
                        scale: None,
                        mirror_plane: None,
                        is_hidden: p.get("isHidden").and_then(|v| v.as_bool()),
                        is_locked: p.get("isLocked").and_then(|v| v.as_bool()),
                        color: None,
                        description: p.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        lifecycle: Lifecycle::Active,
                    },
                );
            }
            for c in design_json["connections"].as_array().unwrap() {
                let cid = Uuid::parse_str(c["id"].as_str().unwrap()).unwrap();
                let connected_piece = c.get("parent").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                let connecting_piece = c.get("child").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                ds.connections.insert(
                    cid,
                    ConnectionState {
                        connection_id: cid,
                        parent_piece_id: connected_piece,
                        parent_design_piece_id: None,
                        parent_connector_id: None,
                        child_piece_id: connecting_piece,
                        child_design_piece_id: None,
                        child_connector_id: None,
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
                    },
                );
            }
            assert_eq!(ds.pieces.len(), 180, "design state should have 180 pieces");
            assert_eq!(ds.connections.len(), 179, "design state should have 179 connections");
        }

        #[test]
        pub fn nakagin_diff_has_diff_status() {
            let diff_json = load_nakagin_diff_json();
            let pieces = diff_json["pieces"].as_array().unwrap();
            assert!(!pieces.is_empty());
            let has_diff = pieces.iter().any(|p| p.get("attributes").and_then(|a| a.as_array()).is_some_and(|attrs| attrs.iter().any(|attr| attr.get("key").and_then(|k| k.as_str()) == Some("semio_compose_rs.diffStatus"))));
            assert!(has_diff, "at least one piece should have semio_compose_rs.diffStatus attribute");
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
                let connected_piece = c.get("parent").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).unwrap_or("");
                let connecting_piece = c.get("child").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).unwrap_or("");
                let fields = serde_json::json!({
                    "parent_piece_id": connected_piece,
                    "child_piece_id": connecting_piece,
                    "gap": c.get("gap"),
                    "shift": c.get("shift"),
                });
                commands.push(DomainCommand::CreateConnection(CreateConnection { connection_id: cid, design_id, fields }));
            }
            assert_eq!(commands.len(), 179, "should create 179 CreateConnection commands");
        }
    } // 🎵️Nakagin Integration Tests
    pub use nakagin_integration_tests::*;

    mod multi_frontend_tests {
        // 🗽️Multi-Frontend Tests

        use super::*;
        #[test]
        pub fn multi_frontend_cursor_events() {
            let person_a = PersonId(Uuid::now_v7());
            let person_b = PersonId(Uuid::now_v7());
            let events: Vec<SessionEvent> = vec![
                SessionEvent::ComposeUpdated { compose_version: 1, person_id: person_a, frontend_id: "desktop".into(), update: ComposeUpdate::CursorMoved { u: 0.1, v: 0.2 } },
                SessionEvent::ComposeUpdated { compose_version: 2, person_id: person_b, frontend_id: "web".into(), update: ComposeUpdate::CursorMoved { u: 0.5, v: 0.5 } },
                SessionEvent::ComposeUpdated { compose_version: 3, person_id: person_a, frontend_id: "desktop".into(), update: ComposeUpdate::CursorMoved { u: 0.3, v: 0.4 } },
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
            let ev_a = SessionEvent::ComposeUpdated { compose_version: 1, person_id: pid_a, frontend_id: "desktop".into(), update: ComposeUpdate::SelectionChanged { piece_ids: vec![piece1], design_ids: vec![] } };
            let ev_b = SessionEvent::ComposeUpdated { compose_version: 2, person_id: pid_b, frontend_id: "web".into(), update: ComposeUpdate::SelectionChanged { piece_ids: vec![piece2], design_ids: vec![] } };
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
                SessionEvent::ComposeUpdated { compose_version: 1, person_id: pid, frontend_id: "desktop".into(), update: ComposeUpdate::LookChanged { position: [10.0, 20.0, 30.0], forward: [0.0, 0.0, -1.0], up: [0.0, 1.0, 0.0] } },
                SessionEvent::ComposeUpdated { compose_version: 2, person_id: pid, frontend_id: "vr".into(), update: ComposeUpdate::LookChanged { position: [5.0, 5.0, 5.0], forward: [1.0, 0.0, 0.0], up: [0.0, 0.0, 1.0] } },
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
            let event = SessionEvent::ComposeUpdated { compose_version: 1, person_id: pid, frontend_id: "desktop".into(), update: ComposeUpdate::CursorMoved { u: 0.5, v: 0.5 } };
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
                    let cmd = DomainCommand::CreateType(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": format!("A-{}", i)}) });
                    cmd_tx1.send(("frontend-1".into(), cmd)).await.unwrap();
                }
            });
            let t2 = tokio::spawn(async move {
                for i in 0..10 {
                    let cmd = DomainCommand::CreateType(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": format!("B-{}", i)}) });
                    cmd_tx2.send(("frontend-2".into(), cmd)).await.unwrap();
                }
            });
            let t3 = tokio::spawn(async move {
                for i in 0..10 {
                    let cmd = DomainCommand::CreateType(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": format!("C-{}", i)}) });
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
    } // 🗽️Multi-Frontend Tests

    mod full_metabolism_nakagin_session_test {
        // 🌦️Full Metabolism + Nakagin Session Test

        use super::*;
        #[test]
        pub fn full_session_with_metabolism_types_and_nakagin_design() {
            let kit_json = load_metabolism_kit_json();
            let design_json = load_nakagin_design_json();
            let session_id = Uuid::now_v7();
            let kit_id = Uuid::parse_str(kit_json["id"].as_str().unwrap()).unwrap();
            let mut state = SessionState {
                session_id: SessionId(session_id),
                domain_version: 0,
                compose_version: 0,
                status: SessionStatus::Active,
                kit: KitState {
                    kit_id,
                    name: kit_json["name"].as_str().unwrap().to_string(),
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
                authors: BTreeMap::new(),
                locations: BTreeMap::new(),
                folders: BTreeMap::new(),
                files: BTreeMap::new(),
                tags: BTreeMap::new(),
                concepts: BTreeMap::new(),
                ports: BTreeMap::new(),
                qualities: BTreeMap::new(),
                types: BTreeMap::new(),
                designs: BTreeMap::new(),
                compose_people: BTreeMap::new(),
            };
            // Add all 50 types
            for t in kit_json["types"].as_array().unwrap() {
                let id = Uuid::parse_str(t["id"].as_str().unwrap()).unwrap();
                state.types.insert(
                    id,
                    TypeState {
                        type_id: id,
                        name: t["name"].as_str().unwrap().to_string(),
                        parent_type_id: None,
                        description: t.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        icon: t.get("icon").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        image: None,
                        folder: None,
                        unit: None,
                        stock: None,
                        is_abstract: None,
                        virtual_type: None,
                        location_id: None,
                        connectors: BTreeMap::new(),
                        representations: BTreeMap::new(),
                        props: BTreeMap::new(),
                        lifecycle: Lifecycle::Active,
                    },
                );
                state.domain_version += 1;
            }
            assert_eq!(state.types.len(), 50);
            assert_eq!(state.domain_version, 50);
            // Add nakagin design with 180 pieces and 179 connections
            let design_id = Uuid::parse_str(design_json["id"].as_str().unwrap()).unwrap();
            let mut ds = DesignState {
                design_id,
                name: design_json["name"].as_str().unwrap().to_string(),
                parent_design_id: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                is_abstract: None,
                can_scale: None,
                can_mirror: None,
                active_layer_id: None,
                location_id: None,
                pieces: BTreeMap::new(),
                connections: BTreeMap::new(),
                layers: BTreeMap::new(),
                groups: BTreeMap::new(),
                stats: BTreeMap::new(),
                props: BTreeMap::new(),
                lifecycle: Lifecycle::Active,
            };
            for p in design_json["pieces"].as_array().unwrap() {
                let pid = Uuid::parse_str(p["id"].as_str().unwrap()).unwrap();
                ds.pieces.insert(
                    pid,
                    PieceState {
                        piece_id: pid,
                        name: p.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        type_id: None,
                        design_ref_id: None,
                        plane: None,
                        center: p.get("center").and_then(|c| Some([c.get("u")?.as_f64()?, c.get("v")?.as_f64()?])),
                        scale: None,
                        mirror_plane: None,
                        is_hidden: p.get("isHidden").and_then(|v| v.as_bool()),
                        is_locked: p.get("isLocked").and_then(|v| v.as_bool()),
                        color: None,
                        description: None,
                        lifecycle: Lifecycle::Active,
                    },
                );
                state.domain_version += 1;
            }
            for c in design_json["connections"].as_array().unwrap() {
                let cid = Uuid::parse_str(c["id"].as_str().unwrap()).unwrap();
                let connected_piece = c.get("parent").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                let connecting_piece = c.get("child").and_then(|v| v.get("piece")).and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or(Uuid::nil());
                ds.connections.insert(
                    cid,
                    ConnectionState {
                        connection_id: cid,
                        parent_piece_id: connected_piece,
                        parent_design_piece_id: None,
                        parent_connector_id: None,
                        child_piece_id: connecting_piece,
                        child_design_piece_id: None,
                        child_connector_id: None,
                        gap: c.get("gap").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        shift: c.get("shift").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        rise: c.get("rise").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        rotation: c.get("rotation").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        turn: c.get("turn").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        tilt: c.get("tilt").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        u: c.get("u").and_then(|v| v.as_f64()),
                        v: c.get("v").and_then(|v| v.as_f64()),
                        description: None,
                        lifecycle: Lifecycle::Active,
                    },
                );
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

    mod metabolism_diff_tests {
        // 📹️Metabolism Diff Tests

        use super::*;
        pub fn load_metabolism_diff_json() -> serde_json::Value {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().parent().unwrap().join("fixture/metabolism.kit.diff.semio_compose_rs.json");
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
                            commands.push(DomainCommand::PatchType(PatchEntity { entity_id: id, fields: u.clone() }));
                        }
                    }
                }
            }
            let batch = DomainCommand::Batch(DomainBatch { commands: commands.clone() });
            let json = serde_json::to_string(&batch).unwrap();
            let back: DomainCommand = serde_json::from_str(&json).unwrap();
            assert!(matches!(back, DomainCommand::Batch(_)));
        }
    } // 📹️Metabolism Diff Tests

    mod lookback_tests {
        // 🧫️Lookback Tests

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
    } // 🧫️Lookback Tests

    mod history_unit_tests {
        // 💊️History Unit Tests

        use super::*;
        #[test]
        pub fn serialize_session_kit_has_required_fields() {
            let sid = Uuid::now_v7();
            let kid = Uuid::now_v7();
            let state = SessionState {
                session_id: SessionId(sid),
                domain_version: 5,
                compose_version: 0,
                status: SessionStatus::Active,
                kit: KitState { kit_id: kid, name: "TestKit".into(), version: Some("1.0".into()), description: Some("A test".into()), icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
                authors: BTreeMap::new(),
                locations: BTreeMap::new(),
                folders: BTreeMap::new(),
                files: BTreeMap::new(),
                tags: BTreeMap::new(),
                concepts: BTreeMap::new(),
                ports: BTreeMap::new(),
                qualities: BTreeMap::new(),
                types: BTreeMap::new(),
                designs: BTreeMap::new(),
                compose_people: BTreeMap::new(),
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
                session_id: SessionId(Uuid::now_v7()),
                domain_version: 10,
                compose_version: 0,
                status: SessionStatus::Active,
                kit: KitState { kit_id: kid, name: "Metabolism".into(), version: None, description: None, icon: None, image: None, preview: None, remote: None, homepage: None, license: None, lifecycle: Lifecycle::Active },
                authors: BTreeMap::new(),
                locations: BTreeMap::new(),
                folders: BTreeMap::new(),
                files: BTreeMap::new(),
                tags: BTreeMap::new(),
                concepts: BTreeMap::new(),
                ports: BTreeMap::new(),
                qualities: BTreeMap::new(),
                types: BTreeMap::new(),
                designs: BTreeMap::new(),
                compose_people: BTreeMap::new(),
            };
            // Add 3 types
            for i in 0..3 {
                let tid = Uuid::now_v7();
                state.types.insert(
                    tid,
                    TypeState {
                        type_id: tid,
                        name: format!("Type{}", i),
                        parent_type_id: None,
                        description: None,
                        icon: None,
                        image: None,
                        folder: None,
                        unit: None,
                        stock: None,
                        is_abstract: None,
                        virtual_type: None,
                        location_id: None,
                        connectors: BTreeMap::new(),
                        representations: BTreeMap::new(),
                        props: BTreeMap::new(),
                        lifecycle: Lifecycle::Active,
                    },
                );
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
                "operation": "Created",
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
                "operation": "Updated",
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
                "operation": "Deleted",
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
                "operation": "Updated",
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
                {"operation": "Created", "entity_kind": "type", "entity_id": t1, "snapshot": {"id": t1, "name": "A"}},
                {"operation": "Created", "entity_kind": "type", "entity_id": t2, "snapshot": {"id": t2, "name": "B"}},
            ]);
            apply_change_log_to_kit(&mut kit, &changes1);
            assert_eq!(kit["types"].as_array().unwrap().len(), 2);
            // Second: delete one type, update the other
            let changes2 = serde_json::json!([
                {"operation": "Deleted", "entity_kind": "type", "entity_id": t1},
                {"operation": "Updated", "entity_kind": "type", "entity_id": t2, "changed_fields": {"name": "B_updated"}},
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
    } // 💊️History Unit Tests

    mod exhaustive {
        // 🌊️Exhaustive: full db-backed integration suite (no external services — `db::Database` is a
        // zero-touch `FsStorage` embedded in a tempdir, replacing the Postgres testcontainer suite
        // this module used to gate behind `docker_available()`).

        use super::*;

        fn test_data_dir(name: &str) -> std::path::PathBuf {
            let mut dir = std::env::temp_dir();
            dir.push(format!("semio_compose_rs-semio_hub-test-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
            dir
        }

        fn test_app_state(name: &str) -> AppState {
            let data_dir = test_data_dir(name);
            let database = open_database(&data_dir).expect("open test database");
            let directory_store = ComposeDirectoryStore::open(&data_dir).expect("open directory store");
            let history_store = HistoryStore::open(&data_dir);
            let directory = SessionDirectory::new(Arc::new(database), Arc::new(history_store), Arc::new(directory_store));
            AppState::new(directory)
        }

        async fn spawn_router(app: Router) -> String {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = listener.local_addr().unwrap();
            tokio::spawn(async move {
                axum::serve(listener, app).await.unwrap();
            });
            format!("http://{}", addr)
        }

        async fn recv_broadcast<T: Clone>(rx: &mut tokio::sync::broadcast::Receiver<T>, label: &str) -> T {
            match tokio::time::timeout(std::time::Duration::from_secs(5), rx.recv()).await {
                Ok(Ok(value)) => value,
                Ok(Err(error)) => panic!("{label} broadcast recv error: {error:?}"),
                Err(_) => panic!("{label} not received before 5s deadline"),
            }
        }

        #[tokio::test]
        async fn session_lifecycle() {
            let state = test_app_state("session-lifecycle");
            let (session_id, kit_id, _owner_token) = state.directory.create_session(Uuid::now_v7(), "E2E Kit", None).await.unwrap();

            let handle = state.directory.db().document(&document_id(session_id)).unwrap();
            assert_eq!(handle.frontier().unwrap().head_seq, 0);

            let kit = state.directory.history().reconstruct_kit_at_version(session_id, 0).unwrap();
            assert_eq!(kit["name"].as_str().unwrap(), "E2E Kit");
            assert_eq!(session_kit_id(&kit).unwrap(), kit_id);
        }

        #[tokio::test]
        async fn domain_commands_and_history() {
            let state = test_app_state("domain-commands-and-history");
            let (session_id, _kit_id, _owner_token) = state.directory.create_session(Uuid::now_v7(), "History Kit", None).await.unwrap();

            let handle = state.directory.get_or_activate(SessionId(session_id)).await.unwrap();
            let mut wire_rx = handle.wire_tx.subscribe();
            let mut event_rx = handle.event_tx.subscribe();

            let type_id = Uuid::now_v7();
            let (reply_tx, reply_rx) = oneshot::channel();
            handle
                .command_tx
                .send(ActorMessage::DomainCommand {
                    envelope: CommandEnvelope { command_id: CommandId(Uuid::now_v7()), client_id: ClientId(Uuid::now_v7()), request_id: RequestId(Uuid::now_v7()), actor_person_id: PersonId(Uuid::now_v7()), base_domain_version: 0 },
                    command: DomainCommand::CreateType(CreateEntity { entity_id: type_id, fields: serde_json::json!({"name": "Tower"}) }),
                    reply: reply_tx,
                })
                .await
                .unwrap();
            let result = reply_rx.await.unwrap().unwrap();
            assert!(matches!(result, CommandResult::Accepted { domain_version: 1 }));

            // Kit at version 1 reflects the new type.
            let kit_v1 = state.directory.history().reconstruct_kit_at_version(session_id, 1).unwrap();
            let types = kit_v1["types"].as_array().unwrap();
            assert_eq!(types.len(), 1);
            assert_eq!(types[0]["name"].as_str().unwrap(), "Tower");

            // Both broadcast channels fired.
            let event = recv_broadcast(&mut event_rx, "domain command accepted event").await;
            assert!(matches!(event, SessionEvent::DomainCommandAccepted { .. }));
            let wire_event = recv_broadcast(&mut wire_rx, "commands wire event").await;
            assert!(matches!(wire_event, WireEvent::Commands { .. }));

            // db itself is the durable frontier authority.
            assert_eq!(state.directory.db().document(&document_id(session_id)).unwrap().frontier().unwrap().head_seq, 1);
        }

        #[tokio::test]
        async fn resubmitting_the_same_client_request_is_idempotent() {
            let state = test_app_state("idempotent-resubmit");
            let (session_id, _kit_id, _owner_token) = state.directory.create_session(Uuid::now_v7(), "Idempotency Kit", None).await.unwrap();
            let handle = state.directory.get_or_activate(SessionId(session_id)).await.unwrap();

            let client_id = ClientId(Uuid::now_v7());
            let request_id = RequestId(Uuid::now_v7());
            let command = DomainCommand::CreateType(CreateEntity { entity_id: Uuid::now_v7(), fields: serde_json::json!({"name": "Once"}) });

            let (tx1, rx1) = oneshot::channel();
            handle.command_tx.send(ActorMessage::DomainCommand { envelope: CommandEnvelope { command_id: CommandId(Uuid::now_v7()), client_id, request_id, actor_person_id: PersonId(Uuid::now_v7()), base_domain_version: 0 }, command: command.clone(), reply: tx1 }).await.unwrap();
            assert!(matches!(rx1.await.unwrap().unwrap(), CommandResult::Accepted { domain_version: 1 }));

            // Same (client_id, request_id) resubmitted with a fresh command_id — must dedupe.
            let (tx2, rx2) = oneshot::channel();
            handle.command_tx.send(ActorMessage::DomainCommand { envelope: CommandEnvelope { command_id: CommandId(Uuid::now_v7()), client_id, request_id, actor_person_id: PersonId(Uuid::now_v7()), base_domain_version: 0 }, command, reply: tx2 }).await.unwrap();
            assert!(matches!(rx2.await.unwrap().unwrap(), CommandResult::IdempotentDuplicate));

            assert_eq!(state.directory.db().document(&document_id(session_id)).unwrap().frontier().unwrap().head_seq, 1);
        }

        #[tokio::test]
        async fn http_api_round_trip() {
            let state = test_app_state("http-api");
            let app = router(state);
            let base = spawn_router(app).await;
            let client = reqwest::Client::new();

            let resp = client.get(format!("{}/health", base)).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            assert_eq!(resp.text().await.unwrap(), "ok");

            let resp = client.post(format!("{}/sessions", base)).json(&serde_json::json!({"kit_name": "API Test Kit"})).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let body: serde_json::Value = resp.json().await.unwrap();
            let session_id = body["session_id"].as_str().unwrap();
            let owner_token = body["owner_token"].as_str().unwrap().to_string();

            let resp = client.get(format!("{}/sessions/{}/snapshot", base, session_id)).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let snapshot: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(snapshot["domain_version"].as_i64().unwrap(), 0);

            let resp = client.get(format!("{}/sessions/{}/history/lookback-tokens", base, session_id)).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let tokens: Vec<String> = resp.json().await.unwrap();
            assert_eq!(tokens.len(), 12);
            assert_eq!(tokens[0], "1min");

            let type_id = Uuid::now_v7();
            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(&owner_token)
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                    "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 0,
                    "kind": "CreateType",
                    "payload": {"entity_id": type_id, "fields": {"name": "APIType"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 200);
            let result: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(result["status"].as_str().unwrap(), "Accepted");
            assert_eq!(result["domain_version"].as_i64().unwrap(), 1);

            let resp = client.get(format!("{}/sessions/{}/kit/at-version/1", base, session_id)).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let kit: serde_json::Value = resp.json().await.unwrap();
            let types = kit["types"].as_array().unwrap();
            assert_eq!(types.len(), 1);
            assert_eq!(types[0]["name"].as_str().unwrap(), "APIType");

            let resp = client.post(format!("{}/sessions/{}/history/compact", base, session_id)).bearer_auth(&owner_token).send().await.unwrap();
            assert_eq!(resp.status(), 200);
        }

        #[tokio::test]
        async fn metabolism_full_kit_history() {
            let state = test_app_state("metabolism-full-kit-history");
            let kit_json = load_metabolism_kit_json();
            let kit_id = Uuid::parse_str(kit_json["id"].as_str().unwrap()).unwrap();
            let (session_id, _kid, _owner_token) = state.directory.create_session(kit_id, "Metabolism", None).await.unwrap();
            let handle = state.directory.get_or_activate(SessionId(session_id)).await.unwrap();

            let types_json = kit_json["types"].as_array().unwrap();
            let mut commands: Vec<DomainCommand> = Vec::new();
            for t in types_json {
                let id = Uuid::parse_str(t["id"].as_str().unwrap()).unwrap();
                commands.push(DomainCommand::CreateType(CreateEntity { entity_id: id, fields: serde_json::json!({"name": t["name"]}) }));
            }
            let batch = DomainCommand::Batch(DomainBatch { commands });
            let (reply_tx, reply_rx) = oneshot::channel();
            handle
                .command_tx
                .send(ActorMessage::DomainCommand {
                    envelope: CommandEnvelope { command_id: CommandId(Uuid::now_v7()), client_id: ClientId(Uuid::now_v7()), request_id: RequestId(Uuid::now_v7()), actor_person_id: PersonId(Uuid::now_v7()), base_domain_version: 0 },
                    command: batch,
                    reply: reply_tx,
                })
                .await
                .unwrap();
            let result = reply_rx.await.unwrap().unwrap();
            assert!(matches!(result, CommandResult::Accepted { domain_version: 1 }));

            let kit_v1 = state.directory.history().reconstruct_kit_at_version(session_id, 1).unwrap();
            assert_eq!(kit_v1["types"].as_array().unwrap().len(), 50, "reconstructed kit at v1 should have 50 types");

            let kit_v0 = state.directory.history().reconstruct_kit_at_version(session_id, 0).unwrap();
            let empty_vec = vec![];
            let v0_types = kit_v0["types"].as_array().unwrap_or(&empty_vec);
            assert_eq!(v0_types.len(), 0, "baseline at v0 should have 0 types");
        }

        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn multi_frontend_websocket_wire_v2() {
            let state = test_app_state("multi-frontend-ws");
            let app = router(state);
            let base = spawn_router(app).await;
            let client = reqwest::Client::new();

            let resp = client.post(format!("{}/sessions", base)).json(&serde_json::json!({"kit_name": "WS Test Kit"})).send().await.unwrap();
            let body: serde_json::Value = resp.json().await.unwrap();
            let session_id = body["session_id"].as_str().unwrap().to_string();
            let owner_token = body["owner_token"].as_str().unwrap().to_string();

            let ws_url = base.replacen("http://", "ws://", 1) + &format!("/sessions/{}/ws", session_id);
            let (ws1, _) = tokio_tungstenite::connect_async(&ws_url).await.expect("ws1 connect");
            let (ws2, _) = tokio_tungstenite::connect_async(&ws_url).await.expect("ws2 connect");
            let (mut _ws1_write, mut ws1_read) = ws1.split();
            let (mut _ws2_write, mut ws2_read) = ws2.split();

            // First frame on each socket is a binary `ServerFrame::Welcome`.
            let welcome1 = tokio::time::timeout(std::time::Duration::from_secs(5), ws1_read.next()).await.expect("ws1 welcome").unwrap().unwrap();
            let welcome2 = tokio::time::timeout(std::time::Duration::from_secs(5), ws2_read.next()).await.expect("ws2 welcome").unwrap().unwrap();
            for msg in [welcome1, welcome2] {
                let bytes = msg.into_data();
                let (_, frame) = protocol::decode_server_frame(&bytes).expect("decode welcome");
                assert!(matches!(frame, protocol::ServerFrame::Welcome { .. }));
            }

            let type_id = Uuid::now_v7();
            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(&owner_token)
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(),
                    "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 0,
                    "kind": "CreateType",
                    "payload": {"entity_id": type_id, "fields": {"name": "WSType"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 200);

            let msg1 = tokio::time::timeout(std::time::Duration::from_secs(5), ws1_read.next()).await.expect("ws1 commands frame").unwrap().unwrap();
            let msg2 = tokio::time::timeout(std::time::Duration::from_secs(5), ws2_read.next()).await.expect("ws2 commands frame").unwrap().unwrap();
            for msg in [msg1, msg2] {
                let bytes = msg.into_data();
                let (lane, frame) = protocol::decode_server_frame(&bytes).expect("decode commands frame");
                assert_eq!(lane, protocol::Lane::Command);
                assert!(matches!(frame, protocol::ServerFrame::Commands { .. }));
            }
        }

        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn snapshot_and_piece_patch_roundtrip() {
            let state = test_app_state("snapshot-piece-patch-roundtrip");
            let app = router(state);
            let base = spawn_router(app).await;
            let client = reqwest::Client::new();

            let create_resp = client.post(format!("{}/sessions", base)).json(&serde_json::json!({"kit_name": "Roundtrip Kit"})).send().await.unwrap();
            let create_body: serde_json::Value = create_resp.json().await.unwrap();
            let session_id = create_body["session_id"].as_str().unwrap();
            let owner_token = create_body["owner_token"].as_str().unwrap().to_string();

            let design_id = Uuid::now_v7();
            let piece_id = Uuid::now_v7();

            let create_design = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(&owner_token)
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 0, "kind": "CreateDesign",
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
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 1, "kind": "CreatePiece",
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
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 2, "kind": "PatchPiece",
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
        async fn auth_forbidden_without_token() {
            let state = test_app_state("auth-forbidden");
            let app = router(state);
            let base = spawn_router(app).await;
            let client = reqwest::Client::new();

            let resp = client.post(format!("{}/sessions", base)).json(&serde_json::json!({"kit_name": "Auth Test Kit"})).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let body: serde_json::Value = resp.json().await.unwrap();
            let session_id = body["session_id"].as_str().unwrap();
            let owner_token = body["owner_token"].as_str().unwrap().to_string();
            assert!(!owner_token.is_empty());

            let resp = client.get(format!("{}/sessions/{}/snapshot", base, session_id)).send().await.unwrap();
            assert_eq!(resp.status(), 200);

            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 0, "kind": "CreateType",
                    "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "NoAuth"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 403);

            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(Uuid::now_v7().to_string())
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 0, "kind": "CreateType",
                    "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "WrongAuth"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 401);

            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(&owner_token)
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 0, "kind": "CreateType",
                    "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "AuthOk"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 200);

            let resp = client
                .post(format!("{}/sessions/{}/commands/semio_compose_rs", base, session_id))
                .json(&serde_json::json!({"client_id": Uuid::now_v7(), "person_id": Uuid::now_v7(), "frontend_id": "test", "base_compose_version": 0, "kind": "UpsertCursor", "payload": {"u": 0.5, "v": 0.5}}))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 403);

            let resp = client.post(format!("{}/sessions/{}/history/compact", base, session_id)).send().await.unwrap();
            assert_eq!(resp.status(), 403);
        }

        #[tokio::test]
        async fn share_token_flow() {
            let state = test_app_state("share-token-flow");
            let app = router(state);
            let base = spawn_router(app).await;
            let client = reqwest::Client::new();

            let resp = client.post(format!("{}/sessions", base)).json(&serde_json::json!({"kit_name": "Share Test Kit"})).send().await.unwrap();
            let body: serde_json::Value = resp.json().await.unwrap();
            let session_id = body["session_id"].as_str().unwrap().to_string();
            let owner_token = body["owner_token"].as_str().unwrap().to_string();

            let type_id = Uuid::now_v7();
            let design_id = Uuid::now_v7();
            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(&owner_token)
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 0, "kind": "CreateType",
                    "payload": {"entity_id": type_id, "fields": {"name": "SharedType"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 200);

            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(&owner_token)
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 1, "kind": "CreateDesign",
                    "payload": {"entity_id": design_id, "fields": {"name": "SharedDesign"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 200);

            let resp = client.post(format!("{}/sessions/{}/shares", base, session_id)).json(&serde_json::json!({"label": "kit share"})).send().await.unwrap();
            assert_eq!(resp.status(), 403);

            let resp = client.post(format!("{}/sessions/{}/shares", base, session_id)).bearer_auth(&owner_token).json(&serde_json::json!({"access_mode": "viewer", "label": "Kit Read-Only"})).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let share: serde_json::Value = resp.json().await.unwrap();
            let kit_share_token = share["token"].as_str().unwrap().to_string();
            assert_eq!(share["access_mode"].as_str().unwrap(), "viewer");
            assert_eq!(share["label"].as_str().unwrap(), "Kit Read-Only");

            let resp = client.post(format!("{}/sessions/{}/shares", base, session_id)).bearer_auth(&owner_token).json(&serde_json::json!({"access_mode": "viewer", "entity_kind": "type", "entity_id": type_id, "label": "Type Share"})).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let type_share: serde_json::Value = resp.json().await.unwrap();
            let type_share_token = type_share["token"].as_str().unwrap().to_string();

            let resp = client.post(format!("{}/sessions/{}/shares", base, session_id)).bearer_auth(&owner_token).json(&serde_json::json!({"access_mode": "viewer", "entity_kind": "design", "entity_id": design_id, "label": "Design Share"})).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let design_share: serde_json::Value = resp.json().await.unwrap();
            let _design_share_token = design_share["token"].as_str().unwrap().to_string();

            let resp = client.get(format!("{}/shares/{}", base, kit_share_token)).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let resolved: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(resolved["session_id"].as_str().unwrap(), session_id);
            assert_eq!(resolved["access_mode"].as_str().unwrap(), "viewer");
            assert!(resolved["entity_kind"].is_null());

            let resp = client.get(format!("{}/shares/{}", base, type_share_token)).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let resolved: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(resolved["entity_kind"].as_str().unwrap(), "type");
            assert_eq!(resolved["entity_id"].as_str().unwrap(), type_id.to_string());

            let resp = client.get(format!("{}/sessions/{}/snapshot", base, session_id)).bearer_auth(&kit_share_token).send().await.unwrap();
            assert_eq!(resp.status(), 200);

            let resp = client
                .post(format!("{}/sessions/{}/commands/domain", base, session_id))
                .bearer_auth(&kit_share_token)
                .json(&serde_json::json!({
                    "command_id": Uuid::now_v7(), "client_id": Uuid::now_v7(), "request_id": Uuid::now_v7(), "actor_person_id": Uuid::now_v7(),
                    "base_domain_version": 2, "kind": "CreateType",
                    "payload": {"entity_id": Uuid::now_v7(), "fields": {"name": "ShouldFail"}}
                }))
                .send()
                .await
                .unwrap();
            assert_eq!(resp.status(), 403);

            let resp = client.get(format!("{}/sessions/{}/shares", base, session_id)).bearer_auth(&owner_token).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let shares: Vec<serde_json::Value> = resp.json().await.unwrap();
            assert_eq!(shares.len(), 3, "should have 3 share tokens");

            let resp = client.delete(format!("{}/sessions/{}/shares/{}", base, session_id, kit_share_token)).bearer_auth(&owner_token).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let del_body: serde_json::Value = resp.json().await.unwrap();
            assert!(del_body["deleted"].as_bool().unwrap());

            let resp = client.get(format!("{}/shares/{}", base, kit_share_token)).send().await.unwrap();
            assert_eq!(resp.status(), 404);

            let resp = client.get(format!("{}/sessions/{}/shares", base, session_id)).bearer_auth(&owner_token).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let shares: Vec<serde_json::Value> = resp.json().await.unwrap();
            assert_eq!(shares.len(), 2, "should have 2 share tokens after deletion");
        }

        #[test]
        pub fn require_admin_rejects_without_token_set() {
            // Specs: When COMPOSE_ADMIN_TOKEN is unset, require_admin returns Forbidden so /admin/* never leaks in misconfigured deployments.
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
            let handle = SessionHandle { command_tx: mpsc::channel(1).0, event_tx: broadcast::channel(1).0, wire_tx: broadcast::channel(1).0, active_connections: Arc::new(AtomicUsize::new(0)), activated_at: Arc::new(Instant::now()) };
            handle.active_connections.fetch_add(3, AtomicOrdering::Relaxed);
            assert_eq!(handle.active_connections.load(AtomicOrdering::Relaxed), 3);
            handle.active_connections.fetch_sub(1, AtomicOrdering::Relaxed);
            assert_eq!(handle.active_connections.load(AtomicOrdering::Relaxed), 2);
        }

        #[tokio::test]
        async fn admin_endpoints_round_trip() {
            // Specs: Full round-trip against the embedded admin router: overview, session list, kit list, share-token list, session detail, compaction config read/write, and auth boundary.
            let state = test_app_state("admin-endpoints");
            let admin_config = AdminConfig { admin_token: Some("test-admin-token".into()), started_at: Arc::new(Instant::now()) };
            let admin_state = AdminState { directory: state.directory.clone(), config: admin_config };
            let app = router(state).merge(admin_router(admin_state));
            let base = spawn_router(app).await;
            let client = reqwest::Client::new();

            let resp_a = client.post(format!("{}/sessions", base)).json(&serde_json::json!({"kit_name": "Admin Kit A"})).send().await.unwrap();
            assert_eq!(resp_a.status(), 200);
            let body_a: serde_json::Value = resp_a.json().await.unwrap();
            let session_a = body_a["session_id"].as_str().unwrap().to_string();
            let owner_a = body_a["owner_token"].as_str().unwrap().to_string();

            let resp_b = client.post(format!("{}/sessions", base)).json(&serde_json::json!({"kit_name": "Admin Kit B"})).send().await.unwrap();
            assert_eq!(resp_b.status(), 200);
            let body_b: serde_json::Value = resp_b.json().await.unwrap();
            let session_b = body_b["session_id"].as_str().unwrap().to_string();

            let share_resp = client.post(format!("{}/sessions/{}/shares", base, session_a)).bearer_auth(&owner_a).json(&serde_json::json!({"access_mode": "viewer", "label": "demo"})).send().await.unwrap();
            assert_eq!(share_resp.status(), 200);
            let share_body: serde_json::Value = share_resp.json().await.unwrap();
            let share_token = share_body["token"].as_str().unwrap().to_string();

            let resp = client.get(format!("{}/admin/overview", base)).send().await.unwrap();
            assert_eq!(resp.status(), 401);

            let resp = client.get(format!("{}/admin/overview", base)).bearer_auth("wrong").send().await.unwrap();
            assert_eq!(resp.status(), 401);

            let resp = client.get(format!("{}/admin/overview", base)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let overview: serde_json::Value = resp.json().await.unwrap();
            assert!(overview["total_sessions"].as_i64().unwrap() >= 2);
            assert!(overview["total_kits"].as_i64().unwrap() >= 2);
            assert!(overview["total_share_tokens"].as_i64().unwrap() >= 1);

            let resp = client.get(format!("{}/admin/sessions", base)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let sessions: Vec<serde_json::Value> = resp.json().await.unwrap();
            assert!(sessions.iter().any(|s| s["session_id"].as_str() == Some(&session_a)));
            assert!(sessions.iter().any(|s| s["session_id"].as_str() == Some(&session_b)));

            let resp = client.get(format!("{}/admin/sessions/{}", base, session_a)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let detail: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(detail["row"]["session_id"].as_str().unwrap(), session_a);
            assert_eq!(detail["kit"]["name"].as_str().unwrap(), "Admin Kit A");
            assert_eq!(detail["share_tokens"].as_array().unwrap().len(), 1);

            let resp = client.get(format!("{}/admin/kits", base)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let kits: Vec<serde_json::Value> = resp.json().await.unwrap();
            let names: Vec<&str> = kits.iter().map(|k| k["name"].as_str().unwrap()).collect();
            assert!(names.contains(&"Admin Kit A"));
            assert!(names.contains(&"Admin Kit B"));

            let resp = client.get(format!("{}/admin/share-tokens", base)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let tokens: Vec<serde_json::Value> = resp.json().await.unwrap();
            assert!(tokens.iter().any(|t| t["token"].as_str() == Some(&share_token)));

            let resp = client.delete(format!("{}/admin/share-tokens/{}", base, share_token)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let revoke: serde_json::Value = resp.json().await.unwrap();
            assert!(revoke["revoked"].as_bool().unwrap());

            let resp = client.get(format!("{}/admin/persons", base)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let _persons: Vec<serde_json::Value> = resp.json().await.unwrap();

            let resp = client.get(format!("{}/admin/connections", base)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let _connections: Vec<serde_json::Value> = resp.json().await.unwrap();

            let resp = client.get(format!("{}/admin/config/{}", base, session_a)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let cfg: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(cfg["lookback_tokens"].as_array().unwrap().len(), 12);

            let resp = client.patch(format!("{}/admin/config/{}", base, session_a)).bearer_auth("test-admin-token").json(&serde_json::json!({"lookback_tokens": ["1min", "1h", "1d"]})).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let cfg: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(cfg["lookback_tokens"].as_array().unwrap().len(), 3);

            let resp = client.patch(format!("{}/admin/config/{}", base, session_a)).bearer_auth("test-admin-token").json(&serde_json::json!({"lookback_tokens": ["not-a-real-token"]})).send().await.unwrap();
            assert_eq!(resp.status(), 400);

            let resp = client.post(format!("{}/admin/sessions/{}/close", base, session_b)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let close: serde_json::Value = resp.json().await.unwrap();
            assert!(close["closed"].as_bool().unwrap());

            let resp = client.get(format!("{}/admin/sessions/{}", base, session_b)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let detail: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(detail["row"]["status"].as_str().unwrap(), "closed");

            let resp = client.post(format!("{}/admin/sessions/{}/passivate", base, session_a)).bearer_auth("test-admin-token").send().await.unwrap();
            assert_eq!(resp.status(), 200);

            let resp = client.get(format!("{}/admin", base)).send().await.unwrap();
            assert_eq!(resp.status(), 200);
            let ctype = resp.headers().get("content-type").unwrap().to_str().unwrap().to_string();
            assert!(ctype.starts_with("text/html"));
            let html = resp.text().await.unwrap();
            assert!(html.contains("semio_compose_rs"));
            assert!(html.contains("overview"));
        }
    } // 🌊️Exhaustive: full db-backed integration suite (no external services)
} // 📐️Tests
