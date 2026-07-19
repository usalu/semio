mod header {
    // 🧲Header
    // HubStorage over SQLite (rusqlite) — the zero-touch default for local dev and single-user
    // self-hosting; no external database service required.
    //
    // Uses synchronous `rusqlite` behind `Arc<Mutex<Connection>>`, not an async SQLite driver:
    // a Cargo workspace may link only one native `sqlite3` (`links = "sqlite3"`), and `rusqlite`
    // is already the sqlite binding used elsewhere in this workspace (compose's unrelated
    // `compose/client/lib/rs`) — adding `sqlx-sqlite`'s `libsqlite3-sys` alongside it is a hard
    // `cargo` resolution conflict, not a style choice. Trait methods stay `async fn` (satisfying
    // the shared `HubStorage` interface) but their bodies are synchronous rusqlite calls, exactly
    // as the pre-HP-1 `bin.rs` reasoned: queries are short, the mutex guard is never held across
    // an `.await`, so nothing here blocks the executor for longer than a real query takes.
}

use async_trait::async_trait;
use os_hub_storage::error::{StorageError, StorageResult};
use os_hub_storage::model::*;
use os_hub_storage::HubStorage;
use rusqlite::{Connection, OptionalExtension};
use semio_framework_core::OpEnvelope;
use semio_framework_hash::hash_bytes;
use std::sync::{Arc, Mutex};
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
CREATE TABLE IF NOT EXISTS hub_blob (
    hash TEXT PRIMARY KEY,
    media_type TEXT NOT NULL,
    size INTEGER NOT NULL,
    bytes BLOB NOT NULL
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

/// @emoji 🗄️ SQLite-backed `HubStorage`. One `rusqlite::Connection` behind a `Mutex` — see `header`
/// for why this isn't an async SQLite driver.
pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStorage {
    /// @emoji 🔌 Opens (creating if absent) the SQLite database at `path` and bootstraps the schema.
    /// `path` may be `:memory:` for tests.
    pub async fn connect(path: &str) -> StorageResult<Self> {
        let conn = Connection::open(path).map_err(backend)?;
        conn.execute_batch(SCHEMA).map_err(backend)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    fn lock(&self) -> StorageResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|_| StorageError::Backend("sqlite connection lock poisoned".into()))
    }

    /// @emoji 🌱 Seeds a placeholder `seed` system user, a default studio it owns, an owner-less
    /// default document, and a `Documents/default` node. The system user satisfies
    /// `hub_studio.owner_user_id`'s foreign key until a real bootstrap admin claims ownership
    /// through `/admin` (HP-6).
    pub async fn seed(&self) -> StorageResult<()> {
        let user_exists: i64 =
            self.lock()?.query_row("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'", [], |row| row.get(0)).map_err(backend)?;
        if user_exists == 0 {
            self.lock()?
                .execute(
                    "INSERT INTO hub_user (id, email, display_name, created_at) VALUES ('seed', 'seed@localhost', 'System', ?1)",
                    rusqlite::params![now_ms()],
                )
                .map_err(backend)?;
        }
        let studio_exists: i64 = self
            .lock()?
            .query_row("SELECT COUNT(*) FROM hub_studio WHERE id = 'default'", [], |row| row.get(0))
            .map_err(backend)?;
        if studio_exists == 0 {
            self.lock()?
                .execute(
                    "INSERT INTO hub_studio (id, name, owner_user_id, created_at) VALUES ('default', 'Studio', 'seed', ?1)",
                    rusqlite::params![now_ms()],
                )
                .map_err(backend)?;
        }
        self.ensure_document("default", "default").await?;
        let node_count: i64 = self.lock()?.query_row("SELECT COUNT(*) FROM node", [], |row| row.get(0)).map_err(backend)?;
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
        let conn = self.lock()?;
        let existing = conn
            .query_row("SELECT studio_id, schema, snapshot, version FROM document WHERE id = ?1", [id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, i64>(3)?))
            })
            .optional()
            .map_err(backend)?;
        if let Some((studio_id, schema, snapshot, version)) = existing {
            let snapshot = serde_json::from_str(&snapshot).unwrap_or_else(|_| default_snapshot());
            return Ok(DocumentRecord { id: id.to_string(), studio_id, schema, snapshot, version });
        }
        let snapshot = default_snapshot();
        let schema = snapshot.get("schema").and_then(|v| v.as_str()).unwrap_or("s.studio/v1").to_string();
        conn.execute(
            "INSERT INTO document (id, studio_id, schema, snapshot, version) VALUES (?1, ?2, ?3, ?4, 0)",
            rusqlite::params![id, studio_id, schema, snapshot.to_string()],
        )
        .map_err(backend)?;
        Ok(DocumentRecord { id: id.to_string(), studio_id: studio_id.to_string(), schema, snapshot, version: 0 })
    }

    async fn save_document(&self, id: &str, schema: &str, snapshot: &serde_json::Value, version: i64) -> StorageResult<()> {
        self.lock()?
            .execute(
                "INSERT INTO document (id, studio_id, schema, snapshot, version) VALUES (?1, 'default', ?2, ?3, ?4)
                 ON CONFLICT(id) DO UPDATE SET schema = ?2, snapshot = ?3, version = ?4",
                rusqlite::params![id, schema, snapshot.to_string(), version],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn insert_op(&self, document_id: &str, version: i64, envelope: &OpEnvelope) -> StorageResult<bool> {
        let payload = serde_json::to_string(envelope).unwrap_or_default();
        let changed = self
            .lock()?
            .execute(
                "INSERT OR IGNORE INTO document_op (id, document_id, version, actor, envelope, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![envelope.id.0, document_id, version, envelope.actor.0, payload, now_ms()],
            )
            .map_err(backend)?;
        Ok(changed > 0)
    }

    async fn load_ops(&self, document_id: &str) -> StorageResult<Vec<(i64, OpEnvelope)>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT version, envelope FROM document_op WHERE document_id = ?1 ORDER BY version ASC")
            .map_err(backend)?;
        let rows = stmt
            .query_map([document_id], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)))
            .map_err(backend)?;
        Ok(rows
            .filter_map(|row| row.ok())
            .filter_map(|(version, envelope)| serde_json::from_str(&envelope).ok().map(|e| (version, e)))
            .collect())
    }
    //#endregion

    //#region Vfs
    async fn list_nodes(&self, studio_id: &str, parent: Option<&str>) -> StorageResult<Vec<NodeRecord>> {
        let conn = self.lock()?;
        let row_mapper = |row: &rusqlite::Row| -> rusqlite::Result<NodeRecord> {
            Ok(NodeRecord { id: row.get(0)?, studio_id: studio_id.to_string(), parent_id: row.get(1)?, name: row.get(2)?, kind: row.get(3)? })
        };
        let rows = match parent {
            Some(parent) => {
                let mut stmt = conn
                    .prepare("SELECT id, parent_id, name, kind FROM node WHERE studio_id = ?1 AND parent_id = ?2 ORDER BY name")
                    .map_err(backend)?;
                let mapped = stmt.query_map(rusqlite::params![studio_id, parent], row_mapper).map_err(backend)?;
                let collected: Vec<_> = mapped.filter_map(|row| row.ok()).collect();
                collected
            }
            None => {
                let mut stmt = conn
                    .prepare("SELECT id, parent_id, name, kind FROM node WHERE studio_id = ?1 AND parent_id IS NULL ORDER BY name")
                    .map_err(backend)?;
                let mapped = stmt.query_map([studio_id], row_mapper).map_err(backend)?;
                let collected: Vec<_> = mapped.filter_map(|row| row.ok()).collect();
                collected
            }
        };
        Ok(rows)
    }

    async fn create_node(&self, studio_id: &str, parent_id: Option<&str>, name: &str, kind: &str) -> StorageResult<NodeRecord> {
        let id = Uuid::now_v7().to_string();
        self.lock()?
            .execute(
                "INSERT INTO node (id, studio_id, parent_id, name, kind) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, studio_id, parent_id, name, kind],
            )
            .map_err(backend)?;
        Ok(NodeRecord { id, studio_id: studio_id.to_string(), parent_id: parent_id.map(str::to_string), name: name.to_string(), kind: kind.to_string() })
    }
    //#endregion

    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> StorageResult<String> {
        let token = Uuid::now_v7().to_string();
        self.lock()?
            .execute(
                "INSERT INTO share_token (token, document_id, created_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![token, document_id, now_ms()],
            )
            .map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> StorageResult<bool> {
        let conn = self.lock()?;
        let has_tokens: i64 = conn
            .query_row("SELECT COUNT(*) FROM share_token WHERE document_id = ?1", [document_id], |row| row.get(0))
            .map_err(backend)?;
        if has_tokens == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let valid: i64 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM share_token WHERE document_id = ?1 AND token = ?2",
                        rusqlite::params![document_id, token],
                        |row| row.get(0),
                    )
                    .map_err(backend)?;
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
        self.lock()?
            .execute(
                "INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![id, email, display_name, password_hash, sso_subject, sso_provider, created_at],
            )
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
        self.lock()?
            .query_row(
                "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = ?1",
                [email],
                user_row,
            )
            .optional()
            .map_err(backend)
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> StorageResult<Option<UserRecord>> {
        self.lock()?
            .query_row(
                "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = ?1 AND sso_subject = ?2",
                rusqlite::params![provider, subject],
                user_row,
            )
            .optional()
            .map_err(backend)
    }

    async fn list_users(&self, limit: i64, offset: i64) -> StorageResult<Vec<UserRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT ?1 OFFSET ?2")
            .map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], user_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
    //#endregion

    //#region Studios
    async fn create_studio(&self, name: &str, owner_user_id: &str) -> StorageResult<StudioRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        self.lock()?
            .execute(
                "INSERT INTO hub_studio (id, name, owner_user_id, created_at) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, name, owner_user_id, created_at],
            )
            .map_err(backend)?;
        self.upsert_membership(&id, owner_user_id, StudioRole::Owner).await?;
        Ok(StudioRecord { id, name: name.to_string(), owner_user_id: owner_user_id.to_string(), created_at })
    }

    async fn list_studios_for_user(&self, user_id: &str) -> StorageResult<Vec<(StudioRecord, StudioRole)>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT s.id, s.name, s.owner_user_id, s.created_at, m.role FROM hub_studio s
                 JOIN hub_studio_membership m ON m.studio_id = s.id WHERE m.user_id = ?1 ORDER BY s.created_at",
            )
            .map_err(backend)?;
        let rows = stmt
            .query_map([user_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(backend)?;
        Ok(rows
            .filter_map(|row| row.ok())
            .filter_map(|(id, name, owner_user_id, created_at, role)| {
                StudioRole::parse(&role).map(|role| (StudioRecord { id, name, owner_user_id, created_at }, role))
            })
            .collect())
    }

    async fn list_studios(&self, limit: i64, offset: i64) -> StorageResult<Vec<StudioRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, name, owner_user_id, created_at FROM hub_studio ORDER BY created_at LIMIT ?1 OFFSET ?2")
            .map_err(backend)?;
        let rows = stmt
            .query_map(rusqlite::params![limit, offset], |row| {
                Ok(StudioRecord {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    owner_user_id: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_documents_for_studio(&self, studio_id: &str) -> StorageResult<Vec<DocumentRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, studio_id, schema, snapshot, version FROM document WHERE studio_id = ?1")
            .map_err(backend)?;
        let rows = stmt
            .query_map([studio_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                ))
            })
            .map_err(backend)?;
        Ok(rows
            .filter_map(|row| row.ok())
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
        self.lock()?
            .execute(
                "INSERT INTO hub_studio_membership (studio_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(studio_id, user_id) DO UPDATE SET role = ?3",
                rusqlite::params![studio_id, user_id, role.as_str(), now_ms()],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn remove_membership(&self, studio_id: &str, user_id: &str) -> StorageResult<()> {
        self.lock()?
            .execute(
                "DELETE FROM hub_studio_membership WHERE studio_id = ?1 AND user_id = ?2",
                rusqlite::params![studio_id, user_id],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn get_role(&self, studio_id: &str, user_id: &str) -> StorageResult<Option<StudioRole>> {
        let role: Option<String> = self
            .lock()?
            .query_row(
                "SELECT role FROM hub_studio_membership WHERE studio_id = ?1 AND user_id = ?2",
                rusqlite::params![studio_id, user_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(backend)?;
        Ok(role.and_then(|r| StudioRole::parse(&r)))
    }
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> StorageResult<AuthSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        self.lock()?
            .execute(
                "INSERT INTO hub_auth_session (id, user_id, created_at, expires_at, sso_provider) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, user_id, created_at, expires_at, sso_provider],
            )
            .map_err(backend)?;
        Ok(AuthSessionRecord { id, user_id: user_id.to_string(), created_at, expires_at, sso_provider: sso_provider.map(str::to_string) })
    }

    async fn get_auth_session(&self, id: &str) -> StorageResult<Option<AuthSessionRecord>> {
        self.lock()?
            .query_row(
                "SELECT id, user_id, created_at, expires_at, sso_provider FROM hub_auth_session WHERE id = ?1",
                [id],
                |row| {
                    Ok(AuthSessionRecord {
                        id: row.get(0)?,
                        user_id: row.get(1)?,
                        created_at: row.get(2)?,
                        expires_at: row.get(3)?,
                        sso_provider: row.get(4)?,
                    })
                },
            )
            .optional()
            .map_err(backend)
    }

    async fn revoke_auth_session(&self, id: &str) -> StorageResult<()> {
        self.lock()?.execute("DELETE FROM hub_auth_session WHERE id = ?1", [id]).map_err(backend)?;
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
        self.lock()?
            .execute(
                "INSERT INTO hub_sync_session (id, document_id, user_id, studio_role, client_label, connected_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![id, document_id, user_id, role_str, client_label, connected_at],
            )
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
        self.lock()?
            .execute(
                "UPDATE hub_sync_session SET disconnected_at = ?2 WHERE id = ?1",
                rusqlite::params![sync_session_id, now_ms()],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> StorageResult<Vec<SyncSessionRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, user_id, studio_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE document_id = ?1 ORDER BY connected_at DESC",
            )
            .map_err(backend)?;
        let rows = stmt
            .query_map([document_id], |row| {
                let studio_role: Option<String> = row.get(2)?;
                Ok(SyncSessionRecord {
                    id: row.get(0)?,
                    document_id: document_id.to_string(),
                    user_id: row.get(1)?,
                    studio_role: studio_role.and_then(|r| StudioRole::parse(&r)),
                    client_label: row.get(3)?,
                    connected_at: row.get(4)?,
                    disconnected_at: row.get(5)?,
                })
            })
            .map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
    //#endregion

    //#region Blobs
    async fn put_blob(&self, bytes: &[u8], media_type: &str) -> StorageResult<BlobRecord> {
        let hash = hash_bytes(bytes);
        self.lock()?
            .execute(
                "INSERT OR IGNORE INTO hub_blob (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![hash, media_type, bytes.len() as i64, bytes],
            )
            .map_err(backend)?;
        Ok(BlobRecord { hash, media_type: media_type.to_string(), size: bytes.len() as i64 })
    }

    async fn get_blob(&self, hash: &str) -> StorageResult<Option<Vec<u8>>> {
        self.lock()?
            .query_row("SELECT bytes FROM hub_blob WHERE hash = ?1", [hash], |row| row.get(0))
            .optional()
            .map_err(backend)
    }

    async fn has_blob(&self, hash: &str) -> StorageResult<bool> {
        let count: i64 = self
            .lock()?
            .query_row("SELECT COUNT(*) FROM hub_blob WHERE hash = ?1", [hash], |row| row.get(0))
            .map_err(backend)?;
        Ok(count > 0)
    }
    //#endregion
}

fn user_row(row: &rusqlite::Row) -> rusqlite::Result<UserRecord> {
    Ok(UserRecord {
        id: row.get(0)?,
        email: row.get(1)?,
        display_name: row.get(2)?,
        password_hash: row.get(3)?,
        sso_subject: row.get(4)?,
        sso_provider: row.get(5)?,
        created_at: row.get(6)?,
    })
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

    // 🔬 Blobs dedupe by content hash: an identical re-put is idempotent, distinct bytes hash apart.
    #[tokio::test]
    async fn blob_put_get_dedupes_idempotently() {
        let storage = SqliteStorage::connect(":memory:").await.expect("connect");
        let bytes = b"hello hub blob";
        assert!(!storage.has_blob("not-a-real-hash").await.unwrap());
        let first = storage.put_blob(bytes, "text/plain").await.expect("first put");
        let second = storage.put_blob(bytes, "text/plain").await.expect("second put");
        assert_eq!(first.hash, second.hash, "identical bytes dedupe to the same hash");
        assert_eq!(first.size, bytes.len() as i64);
        assert!(storage.has_blob(&first.hash).await.unwrap());
        let fetched = storage.get_blob(&first.hash).await.unwrap().expect("blob present");
        assert_eq!(fetched, bytes);
        let other = storage.put_blob(b"different bytes", "text/plain").await.expect("put other");
        assert_ne!(other.hash, first.hash);
    }
}
//#endregion 🔖Tests
