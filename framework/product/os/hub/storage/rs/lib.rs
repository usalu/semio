mod header {
    // 🧲Header
    // Backend-agnostic os-hub storage interface. HubStorage is the single seam every persistence
    // backend (sqlite/postgres/neo4j) implements; bin.rs and DocumentActor only ever see this trait
    // and the DTOs in `model`, never a driver type (sqlx/neo4rs/rusqlite) — satisfies the "external
    // libraries stay behind an interface" rule for a trait three backends must share.
}

//#region 🔖Error
pub mod error {
    /// @emoji 🧯 Opaque storage error — never wraps a backend driver's error type, so no `sqlx`/
    /// `neo4rs`/`rusqlite` type ever crosses this crate's public API.
    #[derive(Debug, thiserror::Error)]
    pub enum StorageError {
        #[error("not found: {0}")]
        NotFound(String),
        #[error("conflict: {0}")]
        Conflict(String),
        #[error("unauthorized")]
        Unauthorized,
        #[error("backend error: {0}")]
        Backend(String),
    }

    pub type StorageResult<T> = Result<T, StorageError>;
}
//#endregion 🔖Error

//#region 🔖Model
pub mod model {
    use serde::{Deserialize, Serialize};

    /// @emoji 🗂️ VFS tree entry, scoped to a studio.
    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodeRecord {
        pub id: String,
        pub studio_id: String,
        pub parent_id: Option<String>,
        pub name: String,
        pub kind: String,
    }

    /// @emoji 📄 A document's durable snapshot + structural version, scoped to a studio.
    pub struct DocumentRecord {
        pub id: String,
        pub studio_id: String,
        pub schema: String,
        pub snapshot: serde_json::Value,
        pub version: i64,
    }

    /// @emoji 🔗 An anonymous per-document bearer token (existing auth-lite scheme, kept as-is).
    pub struct ShareTokenRecord {
        pub token: String,
        pub document_id: String,
        pub created_at: i64,
    }

    /// @emoji 🙋 A platform user — local password login and/or one linked SSO identity.
    pub struct UserRecord {
        pub id: String,
        pub email: String,
        pub display_name: String,
        pub password_hash: Option<String>,
        pub sso_subject: Option<String>,
        pub sso_provider: Option<String>,
        pub created_at: i64,
    }

    /// @emoji 🏛️ A studio: the tenant/workspace unit that owns documents, nodes, and memberships.
    pub struct StudioRecord {
        pub id: String,
        pub name: String,
        pub owner_user_id: String,
        pub created_at: i64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum StudioRole {
        Owner,
        Member,
        Viewer,
    }

    impl StudioRole {
        pub fn as_str(&self) -> &'static str {
            match self {
                StudioRole::Owner => "owner",
                StudioRole::Member => "member",
                StudioRole::Viewer => "viewer",
            }
        }

        pub fn parse(value: &str) -> Option<Self> {
            match value {
                "owner" => Some(StudioRole::Owner),
                "member" => Some(StudioRole::Member),
                "viewer" => Some(StudioRole::Viewer),
                _ => None,
            }
        }
    }

    pub struct StudioMembershipRecord {
        pub studio_id: String,
        pub user_id: String,
        pub role: StudioRole,
        pub created_at: i64,
    }

    /// @emoji 🍪 A browser login session (distinct from {@link SyncSessionRecord}'s realtime connection).
    pub struct AuthSessionRecord {
        pub id: String,
        pub user_id: String,
        pub created_at: i64,
        pub expires_at: i64,
        pub sso_provider: Option<String>,
    }

    /// @emoji 🔴 A realtime document connection — the "session as live-features backend" record;
    /// written by `DocumentActor` on `Subscribe`/disconnect, not per-op.
    pub struct SyncSessionRecord {
        pub id: String,
        pub document_id: String,
        pub user_id: Option<String>,
        pub studio_role: Option<StudioRole>,
        pub client_label: String,
        pub connected_at: i64,
        pub disconnected_at: Option<i64>,
    }
}
//#endregion 🔖Model

//#region 🔖Trait
use error::StorageResult;
use model::*;

/// @emoji 🗄️ Backend-agnostic os-hub persistence. Implemented once per backend (sqlite/postgres/neo4j);
/// `HubState` holds an `Arc<dyn HubStorage>` so the storage backend is a deploy-time choice, not a
/// compile-time one.
#[async_trait::async_trait]
pub trait HubStorage: Send + Sync + 'static {
    //#region Documents
    async fn ensure_document(&self, studio_id: &str, id: &str) -> StorageResult<DocumentRecord>;
    async fn save_document(&self, id: &str, schema: &str, snapshot: &serde_json::Value, version: i64) -> StorageResult<()>;
    async fn insert_op(&self, document_id: &str, version: i64, envelope: &semio_framework_core::OpEnvelope) -> StorageResult<bool>;
    async fn load_ops(&self, document_id: &str) -> StorageResult<Vec<(i64, semio_framework_core::OpEnvelope)>>;
    //#endregion

    //#region Vfs
    async fn list_nodes(&self, studio_id: &str, parent: Option<&str>) -> StorageResult<Vec<NodeRecord>>;
    async fn create_node(&self, studio_id: &str, parent_id: Option<&str>, name: &str, kind: &str) -> StorageResult<NodeRecord>;
    //#endregion

    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> StorageResult<String>;
    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> StorageResult<bool>;
    //#endregion

    //#region Users
    async fn create_user(
        &self,
        email: &str,
        display_name: &str,
        password_hash: Option<&str>,
        sso_subject: Option<&str>,
        sso_provider: Option<&str>,
    ) -> StorageResult<UserRecord>;
    async fn get_user_by_email(&self, email: &str) -> StorageResult<Option<UserRecord>>;
    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> StorageResult<Option<UserRecord>>;
    async fn list_users(&self, limit: i64, offset: i64) -> StorageResult<Vec<UserRecord>>;
    //#endregion

    //#region Studios
    async fn create_studio(&self, name: &str, owner_user_id: &str) -> StorageResult<StudioRecord>;
    async fn list_studios_for_user(&self, user_id: &str) -> StorageResult<Vec<(StudioRecord, StudioRole)>>;
    async fn list_studios(&self, limit: i64, offset: i64) -> StorageResult<Vec<StudioRecord>>;
    async fn list_documents_for_studio(&self, studio_id: &str) -> StorageResult<Vec<DocumentRecord>>;
    async fn upsert_membership(&self, studio_id: &str, user_id: &str, role: StudioRole) -> StorageResult<()>;
    async fn remove_membership(&self, studio_id: &str, user_id: &str) -> StorageResult<()>;
    async fn get_role(&self, studio_id: &str, user_id: &str) -> StorageResult<Option<StudioRole>>;
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> StorageResult<AuthSessionRecord>;
    async fn get_auth_session(&self, id: &str) -> StorageResult<Option<AuthSessionRecord>>;
    async fn revoke_auth_session(&self, id: &str) -> StorageResult<()>;
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(
        &self,
        document_id: &str,
        user_id: Option<&str>,
        studio_role: Option<StudioRole>,
        client_label: &str,
    ) -> StorageResult<SyncSessionRecord>;
    async fn record_sync_session_close(&self, sync_session_id: &str) -> StorageResult<()>;
    async fn list_sync_sessions_for_document(&self, document_id: &str) -> StorageResult<Vec<SyncSessionRecord>>;
    //#endregion
}
//#endregion 🔖Trait
