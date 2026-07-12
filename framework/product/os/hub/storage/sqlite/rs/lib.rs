mod header {
    // 🧲Header
    // HubStorage over SQLite (async, via sqlx-sqlite/sqlx-core direct — no `sqlx` facade, matching
    // the workspace's Postgres precedent). Zero-touch default: one file, no external service.
}

use async_trait::async_trait;
use os_hub_storage::error::{StorageError, StorageResult};
use os_hub_storage::model::*;
use os_hub_storage::HubStorage;
use semio_framework_core::OpEnvelope;
use sqlx_core::pool::PoolOptions;
use sqlx_core::query::query;
use sqlx_core::query_as::query_as;
use sqlx_core::row::Row;
use sqlx_sqlite::{Sqlite, SqlitePool};
use uuid::Uuid;

//#region 🔖Schema
const SCHEMA: &str = "\
CREATE TABLE IF NOT EXISTS node (
    id TEXT PRIMARY KEY,
    studio_id TEXT NOT NULL,
    parent_id TEXT REFERENCES node(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS document (
    id TEXT PRIMARY KEY,
    studio_id TEXT NOT NULL,
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
CREATE TABLE IF NOT EXISTS share_token (
    token TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    sso_subject TEXT,
    sso_provider TEXT,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_studio (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_studio_membership (
    studio_id TEXT NOT NULL REFERENCES hub_studio(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (studio_id, user_id)
);
CREATE TABLE IF NOT EXISTS hub_auth_session (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    sso_provider TEXT
);
CREATE TABLE IF NOT EXISTS hub_sync_session (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    user_id TEXT REFERENCES hub_user(id) ON DELETE SET NULL,
    studio_role TEXT,
    client_label TEXT NOT NULL,
    connected_at INTEGER NOT NULL,
    disconnected_at INTEGER
);
CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_studio_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_node_studio_parent ON node (studio_id, parent_id);
CREATE INDEX IF NOT EXISTS idx_op_document_version ON document_op (document_id, version);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
";
//#endregion 🔖Schema

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

fn backend<E: std::fmt::Display>(err: E) -> StorageError {
    StorageError::Backend(err.to_string())
}

fn default_snapshot() -> serde_json::Value {
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

/// @emoji 🗄️ SQLite-backed `HubStorage`. One pooled connection is enough — SQLite serializes writes
/// internally and this backend targets single-node/dev deployments.
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    /// @emoji 🔌 Opens (creating if absent) the SQLite database at `path` and bootstraps the schema.
    /// `path` may be `:memory:` for tests.
    pub async fn connect(path: &str) -> StorageResult<Self> {
        let url = if path == ":memory:" { "sqlite::memory:".to_string() } else { format!("sqlite://{path}?mode=rwc") };
        let pool: SqlitePool = PoolOptions::<Sqlite>::new().max_connections(1).connect(&url).await.map_err(backend)?;
        for statement in SCHEMA.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            query(statement).execute(&pool).await.map_err(backend)?;
        }
        Ok(Self { pool })
    }

    /// @emoji 🌱 Seeds a default studio, its owner-less default document, and a `Documents/default` node.
    pub async fn seed(&self) -> StorageResult<()> {
        let studio_exists: i64 = query_as::<_, (i64,)>("SELECT COUNT(*) FROM hub_studio WHERE id = 'default'")
            .fetch_one(&self.pool)
            .await
            .map_err(backend)?
            .0;
        if studio_exists == 0 {
            query("INSERT INTO hub_studio (id, name, owner_user_id, created_at) VALUES ('default', 'Studio', 'seed', ?1)")
                .bind(now_ms())
                .execute(&self.pool)
                .await
                .map_err(backend)?;
        }
        self.ensure_document("default", "default").await?;
        let node_count: i64 = query_as::<_, (i64,)>("SELECT COUNT(*) FROM node").fetch_one(&self.pool).await.map_err(backend)?.0;
        if node_count == 0 {
            let folder = self.create_node("default", None, "Documents", "folder").await?;
            self.create_node("default", Some(&folder.id), "default", "document").await?;
        }
        Ok(())
    }
}

#[async_trait]
impl HubStorage for SqliteStorage {
    //#region Documents
    async fn ensure_document(&self, studio_id: &str, id: &str) -> StorageResult<DocumentRecord> {
        let existing = query_as::<_, (String, String, String, i64)>("SELECT id, studio_id, schema, version FROM document WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(backend)?;
        if existing.is_some() {
            let row = query_as::<_, (String, String, String, String, i64)>("SELECT id, studio_id, schema, snapshot, version FROM document WHERE id = ?1")
                .bind(id)
                .fetch_one(&self.pool)
                .await
                .map_err(backend)?;
            let snapshot = serde_json::from_str(&row.3).unwrap_or_else(|_| default_snapshot());
            return Ok(DocumentRecord { id: row.0, studio_id: row.1, schema: row.2, snapshot, version: row.4 });
        }
        let snapshot = default_snapshot();
        let schema = snapshot.get("schema").and_then(|v| v.as_str()).unwrap_or("s.studio/v1").to_string();
        query("INSERT INTO document (id, studio_id, schema, snapshot, version) VALUES (?1, ?2, ?3, ?4, 0)")
            .bind(id)
            .bind(studio_id)
            .bind(&schema)
            .bind(snapshot.to_string())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(DocumentRecord { id: id.to_string(), studio_id: studio_id.to_string(), schema, snapshot, version: 0 })
    }

    async fn save_document(&self, id: &str, schema: &str, snapshot: &serde_json::Value, version: i64) -> StorageResult<()> {
        query(
            "INSERT INTO document (id, studio_id, schema, snapshot, version) VALUES (?1, 'default', ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET schema = ?2, snapshot = ?3, version = ?4",
        )
        .bind(id)
        .bind(schema)
        .bind(snapshot.to_string())
        .bind(version)
        .execute(&self.pool)
        .await
        .map_err(backend)?;
        Ok(())
    }

    async fn insert_op(&self, document_id: &str, version: i64, envelope: &OpEnvelope) -> StorageResult<bool> {
        let payload = serde_json::to_string(envelope).unwrap_or_default();
        let result = query(
            "INSERT OR IGNORE INTO document_op (id, document_id, version, actor, envelope, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )
        .bind(&envelope.id.0)
        .bind(document_id)
        .bind(version)
        .bind(&envelope.actor.0)
        .bind(payload)
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .map_err(backend)?;
        Ok(result.rows_affected() > 0)
    }

    async fn load_ops(&self, document_id: &str) -> StorageResult<Vec<(i64, OpEnvelope)>> {
        let rows = query_as::<_, (i64, String)>("SELECT version, envelope FROM document_op WHERE document_id = ?1 ORDER BY version ASC")
            .bind(document_id)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?;
        Ok(rows.into_iter().filter_map(|(version, envelope)| serde_json::from_str(&envelope).ok().map(|e| (version, e))).collect())
    }
    //#endregion

    //#region Vfs
    async fn list_nodes(&self, studio_id: &str, parent: Option<&str>) -> StorageResult<Vec<NodeRecord>> {
        let rows = match parent {
            Some(parent) => query_as::<_, (String, Option<String>, String, String)>(
                "SELECT id, parent_id, name, kind FROM node WHERE studio_id = ?1 AND parent_id = ?2 ORDER BY name",
            )
            .bind(studio_id)
            .bind(parent)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
            None => query_as::<_, (String, Option<String>, String, String)>(
                "SELECT id, parent_id, name, kind FROM node WHERE studio_id = ?1 AND parent_id IS NULL ORDER BY name",
            )
            .bind(studio_id)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
        };
        Ok(rows
            .into_iter()
            .map(|(id, parent_id, name, kind)| NodeRecord { id, studio_id: studio_id.to_string(), parent_id, name, kind })
            .collect())
    }

    async fn create_node(&self, studio_id: &str, parent_id: Option<&str>, name: &str, kind: &str) -> StorageResult<NodeRecord> {
        let id = Uuid::now_v7().to_string();
        query("INSERT INTO node (id, studio_id, parent_id, name, kind) VALUES (?1, ?2, ?3, ?4, ?5)")
            .bind(&id)
            .bind(studio_id)
            .bind(parent_id)
            .bind(name)
            .bind(kind)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(NodeRecord { id, studio_id: studio_id.to_string(), parent_id: parent_id.map(str::to_string), name: name.to_string(), kind: kind.to_string() })
    }
    //#endregion

    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> StorageResult<String> {
        let token = Uuid::now_v7().to_string();
        query("INSERT INTO share_token (token, document_id, created_at) VALUES (?1, ?2, ?3)")
            .bind(&token)
            .bind(document_id)
            .bind(now_ms())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> StorageResult<bool> {
        let has_tokens: i64 = query_as::<_, (i64,)>("SELECT COUNT(*) FROM share_token WHERE document_id = ?1")
            .bind(document_id)
            .fetch_one(&self.pool)
            .await
            .map_err(backend)?
            .0;
        if has_tokens == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let valid: i64 = query_as::<_, (i64,)>("SELECT COUNT(*) FROM share_token WHERE document_id = ?1 AND token = ?2")
                    .bind(document_id)
                    .bind(token)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(backend)?
                    .0;
                Ok(valid > 0)
            }
        }
    }
    //#endregion

    //#region Users
    async fn create_user(
        &self,
        email: &str,
        display_name: &str,
        password_hash: Option<&str>,
        sso_subject: Option<&str>,
        sso_provider: Option<&str>,
    ) -> StorageResult<UserRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        query("INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")
            .bind(&id)
            .bind(email)
            .bind(display_name)
            .bind(password_hash)
            .bind(sso_subject)
            .bind(sso_provider)
            .bind(created_at)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(UserRecord {
            id,
            email: email.to_string(),
            display_name: display_name.to_string(),
            password_hash: password_hash.map(str::to_string),
            sso_subject: sso_subject.map(str::to_string),
            sso_provider: sso_provider.map(str::to_string),
            created_at,
        })
    }

    async fn get_user_by_email(&self, email: &str) -> StorageResult<Option<UserRecord>> {
        let row = query_as::<_, (String, String, String, Option<String>, Option<String>, Option<String>, i64)>(
            "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = ?1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        Ok(row.map(|(id, email, display_name, password_hash, sso_subject, sso_provider, created_at)| UserRecord {
            id,
            email,
            display_name,
            password_hash,
            sso_subject,
            sso_provider,
            created_at,
        }))
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> StorageResult<Option<UserRecord>> {
        let row = query_as::<_, (String, String, String, Option<String>, Option<String>, Option<String>, i64)>(
            "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = ?1 AND sso_subject = ?2",
        )
        .bind(provider)
        .bind(subject)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        Ok(row.map(|(id, email, display_name, password_hash, sso_subject, sso_provider, created_at)| UserRecord {
            id,
            email,
            display_name,
            password_hash,
            sso_subject,
            sso_provider,
            created_at,
        }))
    }

    async fn list_users(&self, limit: i64, offset: i64) -> StorageResult<Vec<UserRecord>> {
        let rows = query_as::<_, (String, String, String, Option<String>, Option<String>, Option<String>, i64)>(
            "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT ?1 OFFSET ?2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows
            .into_iter()
            .map(|(id, email, display_name, password_hash, sso_subject, sso_provider, created_at)| UserRecord {
                id,
                email,
                display_name,
                password_hash,
                sso_subject,
                sso_provider,
                created_at,
            })
            .collect())
    }
    //#endregion

    //#region Studios
    async fn create_studio(&self, name: &str, owner_user_id: &str) -> StorageResult<StudioRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        query("INSERT INTO hub_studio (id, name, owner_user_id, created_at) VALUES (?1, ?2, ?3, ?4)")
            .bind(&id)
            .bind(name)
            .bind(owner_user_id)
            .bind(created_at)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        self.upsert_membership(&id, owner_user_id, StudioRole::Owner).await?;
        Ok(StudioRecord { id, name: name.to_string(), owner_user_id: owner_user_id.to_string(), created_at })
    }

    async fn list_studios_for_user(&self, user_id: &str) -> StorageResult<Vec<(StudioRecord, StudioRole)>> {
        let rows = query_as::<_, (String, String, String, i64, String)>(
            "SELECT s.id, s.name, s.owner_user_id, s.created_at, m.role FROM hub_studio s
             JOIN hub_studio_membership m ON m.studio_id = s.id WHERE m.user_id = ?1 ORDER BY s.created_at",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows
            .into_iter()
            .filter_map(|(id, name, owner_user_id, created_at, role)| {
                StudioRole::parse(&role).map(|role| (StudioRecord { id, name, owner_user_id, created_at }, role))
            })
            .collect())
    }

    async fn list_studios(&self, limit: i64, offset: i64) -> StorageResult<Vec<StudioRecord>> {
        let rows = query_as::<_, (String, String, String, i64)>("SELECT id, name, owner_user_id, created_at FROM hub_studio ORDER BY created_at LIMIT ?1 OFFSET ?2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?;
        Ok(rows.into_iter().map(|(id, name, owner_user_id, created_at)| StudioRecord { id, name, owner_user_id, created_at }).collect())
    }

    async fn list_documents_for_studio(&self, studio_id: &str) -> StorageResult<Vec<DocumentRecord>> {
        let rows = query_as::<_, (String, String, String, String, i64)>("SELECT id, studio_id, schema, snapshot, version FROM document WHERE studio_id = ?1")
            .bind(studio_id)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?;
        Ok(rows
            .into_iter()
            .map(|(id, studio_id, schema, snapshot, version)| DocumentRecord {
                id,
                studio_id,
                schema,
                snapshot: serde_json::from_str(&snapshot).unwrap_or_else(|_| default_snapshot()),
                version,
            })
            .collect())
    }

    async fn upsert_membership(&self, studio_id: &str, user_id: &str, role: StudioRole) -> StorageResult<()> {
        query(
            "INSERT INTO hub_studio_membership (studio_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(studio_id, user_id) DO UPDATE SET role = ?3",
        )
        .bind(studio_id)
        .bind(user_id)
        .bind(role.as_str())
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .map_err(backend)?;
        Ok(())
    }

    async fn remove_membership(&self, studio_id: &str, user_id: &str) -> StorageResult<()> {
        query("DELETE FROM hub_studio_membership WHERE studio_id = ?1 AND user_id = ?2")
            .bind(studio_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn get_role(&self, studio_id: &str, user_id: &str) -> StorageResult<Option<StudioRole>> {
        let row = query_as::<_, (String,)>("SELECT role FROM hub_studio_membership WHERE studio_id = ?1 AND user_id = ?2")
            .bind(studio_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(backend)?;
        Ok(row.and_then(|(role,)| StudioRole::parse(&role)))
    }
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> StorageResult<AuthSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        query("INSERT INTO hub_auth_session (id, user_id, created_at, expires_at, sso_provider) VALUES (?1, ?2, ?3, ?4, ?5)")
            .bind(&id)
            .bind(user_id)
            .bind(created_at)
            .bind(expires_at)
            .bind(sso_provider)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(AuthSessionRecord { id, user_id: user_id.to_string(), created_at, expires_at, sso_provider: sso_provider.map(str::to_string) })
    }

    async fn get_auth_session(&self, id: &str) -> StorageResult<Option<AuthSessionRecord>> {
        let row = query_as::<_, (String, String, i64, i64, Option<String>)>(
            "SELECT id, user_id, created_at, expires_at, sso_provider FROM hub_auth_session WHERE id = ?1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        Ok(row.map(|(id, user_id, created_at, expires_at, sso_provider)| AuthSessionRecord { id, user_id, created_at, expires_at, sso_provider }))
    }

    async fn revoke_auth_session(&self, id: &str) -> StorageResult<()> {
        query("DELETE FROM hub_auth_session WHERE id = ?1").bind(id).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(
        &self,
        document_id: &str,
        user_id: Option<&str>,
        studio_role: Option<StudioRole>,
        client_label: &str,
    ) -> StorageResult<SyncSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let connected_at = now_ms();
        let role_str = studio_role.map(|r| r.as_str());
        query("INSERT INTO hub_sync_session (id, document_id, user_id, studio_role, client_label, connected_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
            .bind(&id)
            .bind(document_id)
            .bind(user_id)
            .bind(role_str)
            .bind(client_label)
            .bind(connected_at)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(SyncSessionRecord {
            id,
            document_id: document_id.to_string(),
            user_id: user_id.map(str::to_string),
            studio_role,
            client_label: client_label.to_string(),
            connected_at,
            disconnected_at: None,
        })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> StorageResult<()> {
        query("UPDATE hub_sync_session SET disconnected_at = ?2 WHERE id = ?1")
            .bind(sync_session_id)
            .bind(now_ms())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> StorageResult<Vec<SyncSessionRecord>> {
        let rows = query_as::<_, (String, Option<String>, Option<String>, String, i64, Option<i64>)>(
            "SELECT id, user_id, studio_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE document_id = ?1 ORDER BY connected_at DESC",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows
            .into_iter()
            .map(|(id, user_id, studio_role, client_label, connected_at, disconnected_at)| SyncSessionRecord {
                id,
                document_id: document_id.to_string(),
                user_id,
                studio_role: studio_role.and_then(|r| StudioRole::parse(&r)),
                client_label,
                connected_at,
                disconnected_at,
            })
            .collect())
    }
    //#endregion
}

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🔬 Schema bootstraps and a document round-trips through ensure/save/load against `:memory:`.
    #[tokio::test]
    async fn document_round_trip() {
        let storage = SqliteStorage::connect(":memory:").await.expect("connect");
        storage.seed().await.expect("seed");
        let doc = storage.ensure_document("default", "default").await.expect("ensure");
        assert_eq!(doc.version, 0);
        storage.save_document("default", &doc.schema, &doc.snapshot, 1).await.expect("save");
        let reloaded = storage.ensure_document("default", "default").await.expect("reload");
        assert_eq!(reloaded.version, 1);
    }

    // 🔬 Users, studios, and role-based membership round-trip.
    #[tokio::test]
    async fn user_studio_membership_round_trip() {
        let storage = SqliteStorage::connect(":memory:").await.expect("connect");
        let user = storage.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let studio = storage.create_studio("Studio A", &user.id).await.expect("create studio");
        assert_eq!(storage.get_role(&studio.id, &user.id).await.unwrap(), Some(StudioRole::Owner));
        let member = storage.create_user("b@example.com", "Bob", None, None, None).await.expect("create user 2");
        storage.upsert_membership(&studio.id, &member.id, StudioRole::Viewer).await.expect("add member");
        assert_eq!(storage.get_role(&studio.id, &member.id).await.unwrap(), Some(StudioRole::Viewer));
        let studios = storage.list_studios_for_user(&member.id).await.unwrap();
        assert_eq!(studios.len(), 1);
        storage.remove_membership(&studio.id, &member.id).await.expect("remove");
        assert_eq!(storage.get_role(&studio.id, &member.id).await.unwrap(), None);
    }

    // 🔬 SyncSession open/close is durable and listable.
    #[tokio::test]
    async fn sync_session_lifecycle() {
        let storage = SqliteStorage::connect(":memory:").await.expect("connect");
        storage.seed().await.expect("seed");
        let session = storage.record_sync_session_open("default", None, None, "test-client").await.expect("open");
        assert!(session.disconnected_at.is_none());
        storage.record_sync_session_close(&session.id).await.expect("close");
        let sessions = storage.list_sync_sessions_for_document("default").await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert!(sessions[0].disconnected_at.is_some());
    }

    // 🔬 Share tokens: tokenless is open; once issued, only a valid token authorizes.
    #[tokio::test]
    async fn share_token_gating() {
        let storage = SqliteStorage::connect(":memory:").await.expect("connect");
        storage.seed().await.expect("seed");
        assert!(storage.authorized_by_token("default", None).await.unwrap());
        let token = storage.create_share_token("default").await.expect("mint token");
        assert!(!storage.authorized_by_token("default", None).await.unwrap());
        assert!(storage.authorized_by_token("default", Some(&token)).await.unwrap());
    }
}
//#endregion 🔖Tests
