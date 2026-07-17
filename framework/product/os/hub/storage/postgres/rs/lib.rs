mod header {
    // 🧲Header
    // HubStorage over PostgreSQL — direct `sqlx-postgres`/`sqlx-core` (not the `sqlx` facade),
    // matching the exact precedent in `compose/server/hub/rs/bin.rs`. The scale-out backend for
    // multi-node self-hosted deployments; schema bootstrap is `storage/postgres/schema.sql`.
}

use async_trait::async_trait;
use os_hub_storage::error::{StorageError, StorageResult};
use os_hub_storage::model::*;
use os_hub_storage::HubStorage;
use semio_framework_core::OpEnvelope;
use semio_framework_hash::hash_bytes;
pub use sqlx_core::row::Row;
pub use sqlx_postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;

const SCHEMA: &str = include_str!("../schema.sql");

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

/// @emoji 🐘 PostgreSQL-backed `HubStorage`, pooled via `PgPool`.
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    /// @emoji 🔌 Connects to `database_url` and bootstraps the schema (idempotent, no migration framework).
    pub async fn connect(database_url: &str) -> StorageResult<Self> {
        let pool = PgPoolOptions::new().max_connections(20).connect(database_url).await.map_err(backend)?;
        for statement in SCHEMA.split(';').map(str::trim).filter(|s| !s.is_empty() && !s.starts_with("--")) {
            sqlx_core::query::query(statement).execute(&pool).await.map_err(backend)?;
        }
        Ok(Self { pool })
    }

    /// @emoji 🌱 Seeds a placeholder `seed` system user, a default studio it owns, an owner-less
    /// default document, and a `Documents/default` node. The system user satisfies
    /// `hub_studio.owner_user_id`'s foreign key until a real bootstrap admin claims ownership
    /// through `/admin` (HP-6).
    pub async fn seed(&self) -> StorageResult<()> {
        let user_exists: (i64,) =
            sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'").fetch_one(&self.pool).await.map_err(backend)?;
        if user_exists.0 == 0 {
            sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, created_at) VALUES ('seed', 'seed@localhost', 'System', $1)")
                .bind(now_ms())
                .execute(&self.pool)
                .await
                .map_err(backend)?;
        }
        let exists: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_studio WHERE id = 'default'")
            .fetch_one(&self.pool)
            .await
            .map_err(backend)?;
        if exists.0 == 0 {
            sqlx_core::query::query("INSERT INTO hub_studio (id, name, owner_user_id, created_at) VALUES ('default', 'Studio', 'seed', $1)")
                .bind(now_ms())
                .execute(&self.pool)
                .await
                .map_err(backend)?;
        }
        self.ensure_document("default", "default").await?;
        let node_count: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_node").fetch_one(&self.pool).await.map_err(backend)?;
        if node_count.0 == 0 {
            let folder = self.create_node("default", None, "Documents", "folder").await?;
            self.create_node("default", Some(&folder.id), "default", "document").await?;
        }
        Ok(())
    }
}

#[async_trait]
impl HubStorage for PostgresStorage {
    //#region Documents
    async fn ensure_document(&self, studio_id: &str, id: &str) -> StorageResult<DocumentRecord> {
        let existing: Option<(String, String, serde_json::Value, i64)> =
            sqlx_core::query_as::query_as("SELECT studio_id, schema, snapshot, version FROM hub_document WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        if let Some((studio_id, schema, snapshot, version)) = existing {
            return Ok(DocumentRecord { id: id.to_string(), studio_id, schema, snapshot, version });
        }
        let snapshot = default_snapshot();
        let schema = snapshot.get("schema").and_then(|v| v.as_str()).unwrap_or("s.studio/v1").to_string();
        sqlx_core::query::query("INSERT INTO hub_document (id, studio_id, schema, snapshot, version) VALUES ($1, $2, $3, $4, 0)")
            .bind(id)
            .bind(studio_id)
            .bind(&schema)
            .bind(&snapshot)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(DocumentRecord { id: id.to_string(), studio_id: studio_id.to_string(), schema, snapshot, version: 0 })
    }

    async fn save_document(&self, id: &str, schema: &str, snapshot: &serde_json::Value, version: i64) -> StorageResult<()> {
        sqlx_core::query::query(
            "INSERT INTO hub_document (id, studio_id, schema, snapshot, version) VALUES ($1, 'default', $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET schema = $2, snapshot = $3, version = $4",
        )
        .bind(id)
        .bind(schema)
        .bind(snapshot)
        .bind(version)
        .execute(&self.pool)
        .await
        .map_err(backend)?;
        Ok(())
    }

    async fn insert_op(&self, document_id: &str, version: i64, envelope: &OpEnvelope) -> StorageResult<bool> {
        let payload = serde_json::to_value(envelope).unwrap_or(serde_json::Value::Null);
        let result = sqlx_core::query::query(
            "INSERT INTO hub_document_op (id, document_id, version, actor, envelope, created_at) VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO NOTHING",
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
        let rows: Vec<(i64, serde_json::Value)> =
            sqlx_core::query_as::query_as("SELECT version, envelope FROM hub_document_op WHERE document_id = $1 ORDER BY version ASC")
                .bind(document_id)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows.into_iter().filter_map(|(version, envelope)| serde_json::from_value(envelope).ok().map(|e| (version, e))).collect())
    }
    //#endregion

    //#region Vfs
    async fn list_nodes(&self, studio_id: &str, parent: Option<&str>) -> StorageResult<Vec<NodeRecord>> {
        let rows: Vec<(String, Option<String>, String, String)> = match parent {
            Some(parent) => sqlx_core::query_as::query_as(
                "SELECT id, parent_id, name, kind FROM hub_node WHERE studio_id = $1 AND parent_id = $2 ORDER BY name",
            )
            .bind(studio_id)
            .bind(parent)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
            None => sqlx_core::query_as::query_as(
                "SELECT id, parent_id, name, kind FROM hub_node WHERE studio_id = $1 AND parent_id IS NULL ORDER BY name",
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
        sqlx_core::query::query("INSERT INTO hub_node (id, studio_id, parent_id, name, kind) VALUES ($1, $2, $3, $4, $5)")
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
        sqlx_core::query::query("INSERT INTO hub_share_token (token, document_id, created_at) VALUES ($1, $2, $3)")
            .bind(&token)
            .bind(document_id)
            .bind(now_ms())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> StorageResult<bool> {
        let has_tokens: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_share_token WHERE document_id = $1")
            .bind(document_id)
            .fetch_one(&self.pool)
            .await
            .map_err(backend)?;
        if has_tokens.0 == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let valid: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_share_token WHERE document_id = $1 AND token = $2")
                    .bind(document_id)
                    .bind(token)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(backend)?;
                Ok(valid.0 > 0)
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
        sqlx_core::query::query(
            "INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
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
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> = sqlx_core::query_as::query_as(
            "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> StorageResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> = sqlx_core::query_as::query_as(
            "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = $1 AND sso_subject = $2",
        )
        .bind(provider)
        .bind(subject)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn list_users(&self, limit: i64, offset: i64) -> StorageResult<Vec<UserRecord>> {
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> = sqlx_core::query_as::query_as(
            "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows.into_iter().map(user_from_row).collect())
    }
    //#endregion

    //#region Studios
    async fn create_studio(&self, name: &str, owner_user_id: &str) -> StorageResult<StudioRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        sqlx_core::query::query("INSERT INTO hub_studio (id, name, owner_user_id, created_at) VALUES ($1, $2, $3, $4)")
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
        let rows: Vec<(String, String, String, i64, String)> = sqlx_core::query_as::query_as(
            "SELECT s.id, s.name, s.owner_user_id, s.created_at, m.role FROM hub_studio s
             JOIN hub_studio_membership m ON m.studio_id = s.id WHERE m.user_id = $1 ORDER BY s.created_at",
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
        let rows: Vec<(String, String, String, i64)> =
            sqlx_core::query_as::query_as("SELECT id, name, owner_user_id, created_at FROM hub_studio ORDER BY created_at LIMIT $1 OFFSET $2")
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows.into_iter().map(|(id, name, owner_user_id, created_at)| StudioRecord { id, name, owner_user_id, created_at }).collect())
    }

    async fn list_documents_for_studio(&self, studio_id: &str) -> StorageResult<Vec<DocumentRecord>> {
        let rows: Vec<(String, String, String, serde_json::Value, i64)> =
            sqlx_core::query_as::query_as("SELECT id, studio_id, schema, snapshot, version FROM hub_document WHERE studio_id = $1")
                .bind(studio_id)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows
            .into_iter()
            .map(|(id, studio_id, schema, snapshot, version)| DocumentRecord { id, studio_id, schema, snapshot, version })
            .collect())
    }

    async fn upsert_membership(&self, studio_id: &str, user_id: &str, role: StudioRole) -> StorageResult<()> {
        sqlx_core::query::query(
            "INSERT INTO hub_studio_membership (studio_id, user_id, role, created_at) VALUES ($1, $2, $3, $4)
             ON CONFLICT (studio_id, user_id) DO UPDATE SET role = $3",
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
        sqlx_core::query::query("DELETE FROM hub_studio_membership WHERE studio_id = $1 AND user_id = $2")
            .bind(studio_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn get_role(&self, studio_id: &str, user_id: &str) -> StorageResult<Option<StudioRole>> {
        let row: Option<(String,)> = sqlx_core::query_as::query_as("SELECT role FROM hub_studio_membership WHERE studio_id = $1 AND user_id = $2")
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
        sqlx_core::query::query("INSERT INTO hub_auth_session (id, user_id, created_at, expires_at, sso_provider) VALUES ($1, $2, $3, $4, $5)")
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
        let row: Option<(String, String, i64, i64, Option<String>)> =
            sqlx_core::query_as::query_as("SELECT id, user_id, created_at, expires_at, sso_provider FROM hub_auth_session WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        Ok(row.map(|(id, user_id, created_at, expires_at, sso_provider)| AuthSessionRecord { id, user_id, created_at, expires_at, sso_provider }))
    }

    async fn revoke_auth_session(&self, id: &str) -> StorageResult<()> {
        sqlx_core::query::query("DELETE FROM hub_auth_session WHERE id = $1").bind(id).execute(&self.pool).await.map_err(backend)?;
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
        sqlx_core::query::query(
            "INSERT INTO hub_sync_session (id, document_id, user_id, studio_role, client_label, connected_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
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
        sqlx_core::query::query("UPDATE hub_sync_session SET disconnected_at = $2 WHERE id = $1")
            .bind(sync_session_id)
            .bind(now_ms())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> StorageResult<Vec<SyncSessionRecord>> {
        let rows: Vec<(String, Option<String>, Option<String>, String, i64, Option<i64>)> = sqlx_core::query_as::query_as(
            "SELECT id, user_id, studio_role, client_label, connected_at, disconnected_at FROM hub_sync_session
             WHERE document_id = $1 ORDER BY connected_at DESC",
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

    //#region Blobs
    async fn put_blob(&self, bytes: &[u8], media_type: &str) -> StorageResult<BlobRecord> {
        let hash = hash_bytes(bytes);
        sqlx_core::query::query("INSERT INTO hub_blob (hash, media_type, size, bytes) VALUES ($1, $2, $3, $4) ON CONFLICT (hash) DO NOTHING")
            .bind(&hash)
            .bind(media_type)
            .bind(bytes.len() as i64)
            .bind(bytes)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(BlobRecord { hash, media_type: media_type.to_string(), size: bytes.len() as i64 })
    }

    async fn get_blob(&self, hash: &str) -> StorageResult<Option<Vec<u8>>> {
        let row: Option<(Vec<u8>,)> =
            sqlx_core::query_as::query_as("SELECT bytes FROM hub_blob WHERE hash = $1").bind(hash).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(|(bytes,)| bytes))
    }

    async fn has_blob(&self, hash: &str) -> StorageResult<bool> {
        let row: (i64,) =
            sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_blob WHERE hash = $1").bind(hash).fetch_one(&self.pool).await.map_err(backend)?;
        Ok(row.0 > 0)
    }
    //#endregion
}

fn user_from_row(row: (String, String, String, Option<String>, Option<String>, Option<String>, i64)) -> UserRecord {
    let (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) = row;
    UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }
}

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;

    async fn test_storage() -> (PostgresStorage, testcontainers::ContainerAsync<Postgres>) {
        let container = Postgres::default().start().await.expect("start postgres container");
        let port = container.get_host_port_ipv4(5432).await.expect("port");
        let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
        let storage = PostgresStorage::connect(&url).await.expect("connect");
        (storage, container)
    }

    // 🔬 Schema bootstraps and a document round-trips through ensure/save/load against a real Postgres.
    #[tokio::test]
    async fn document_round_trip() {
        let (storage, _container) = test_storage().await;
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
        let (storage, _container) = test_storage().await;
        let user = storage.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let studio = storage.create_studio("Studio A", &user.id).await.expect("create studio");
        assert_eq!(storage.get_role(&studio.id, &user.id).await.unwrap(), Some(StudioRole::Owner));
    }

    // 🔬 Blobs dedupe by content hash against a real Postgres.
    #[tokio::test]
    async fn blob_put_get_dedupes_idempotently() {
        let (storage, _container) = test_storage().await;
        let bytes = b"hello hub blob";
        assert!(!storage.has_blob("not-a-real-hash").await.unwrap());
        let first = storage.put_blob(bytes, "text/plain").await.expect("first put");
        let second = storage.put_blob(bytes, "text/plain").await.expect("second put");
        assert_eq!(first.hash, second.hash, "identical bytes dedupe to the same hash");
        assert!(storage.has_blob(&first.hash).await.unwrap());
        let fetched = storage.get_blob(&first.hash).await.unwrap().expect("blob present");
        assert_eq!(fetched, bytes);
    }
}
//#endregion 🔖Tests
