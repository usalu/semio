# Scout Report: Hub Spaces Live Presence & Collaborative Studios

## 1. HubDirectory Trait, Models, and Errors

### Trait Signature
`🌎️hub/📇️directory/🦀️component.rs:140-184`

```rust
#[async_trait::async_trait]
pub trait HubDirectory: Send + Sync + 'static {
    // ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String>;
    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool>;
    
    // Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord>;
    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>>;
    
    // Spaces
    async fn create_space(&self, name: &str, owner_user_id: &str, kind: &str, visibility: &str) -> DirectoryResult<SpaceRecord>;
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>>;
    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>>;
    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>>;
    async fn upsert_membership(&self, space_id: &str, user_id: &str, role: SpaceRole) -> DirectoryResult<()>;
    async fn remove_membership(&self, space_id: &str, user_id: &str) -> DirectoryResult<()>;
    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>>;
    
    // AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord>;
    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>>;
    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()>;
    
    // SyncSessions
    async fn record_sync_session_open(&self, document_id: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord>;
    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()>;
    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>>;
}
```

### Model Structs
`🌎️hub/📇️directory/🦀️component.rs:43-127`

- **ShareTokenRecord** (lines 43-47): `token: String`, `document_id: String`, `created_at: i64`
- **UserRecord** (lines 50-58): `id`, `email`, `display_name`, `password_hash: Option<String>`, `sso_subject: Option<String>`, `sso_provider: Option<String>`, `created_at: i64`
- **SpaceRecord** (lines 65-72): `id`, `name`, `owner_user_id`, `created_at`, `kind` (string: "atelier"|"studio"|"archive"), `visibility` (string: "private"|"public")
- **SpaceRole** (lines 77-99): Enum with `Author` and `Spectator` variants; `as_str()` returns "author"/"spectator"; `parse()` method for string conversion
- **SpaceMembershipRecord** (lines 101-106): `space_id`, `user_id`, `role: SpaceRole`, `created_at`
- **AuthSessionRecord** (lines 109-115): `id`, `user_id`, `created_at`, `expires_at`, `sso_provider: Option<String>`
- **SyncSessionRecord** (lines 119-127): `id`, `document_id`, `user_id: Option<String>`, `space_role: Option<SpaceRole>`, `client_label`, `connected_at`, `disconnected_at: Option<i64>`

### DirectoryError Variants
`🌎️hub/📇️directory/🦀️component.rs:21-30`

```rust
pub enum DirectoryError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("backend error: {0}")]
    Backend(String),
}
```

### Module Wiring
- **glue.rs**: `🌎️hub/📦️packages/🦀️rust/📦️glue.rs:7-8` — Re-exports directory module via `#[path]`
- **Feature gates**: Lines 192-202 in component.rs:
  - `#[cfg(feature = "sqlite")]` → `🪶️sqlite/🦀️component.rs`
  - `#[cfg(feature = "postgres")]` → `🐘️postgres/🦀️component.rs`
  - `#[cfg(feature = "neo4j")]` → `🌐️neo4j/🦀️component.rs`

---

## 2. SQLite Backend Implementation

### CREATE TABLE Schema
`🌎️hub/📇️directory/🪶️sqlite/🦀️component.rs:23-71`

```sql
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
CREATE TABLE IF NOT EXISTS hub_space (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at INTEGER NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('atelier', 'studio', 'archive')),
    visibility TEXT NOT NULL CHECK (visibility IN ('private', 'public'))
);
CREATE TABLE IF NOT EXISTS hub_space_membership (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at INTEGER NOT NULL,
    PRIMARY KEY (space_id, user_id)
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
    space_role TEXT,
    client_label TEXT NOT NULL,
    connected_at INTEGER NOT NULL,
    disconnected_at INTEGER
);
CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_space_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
```

### Connection & Schema Management
- **SqliteDirectory struct** (lines 85-87): `conn: Arc<Mutex<Connection>>`
- **connect()** (lines 92-96): Opens at path (`:memory:` for tests), executes full SCHEMA batch
- **seed()** (lines 106-117): Idempotent; creates 'seed' system user and 'default' studio/private space; grants owner SpaceRole::Author (or Spectator for archive)
- **Locking pattern** (lines 98-100): `Mutex<Connection>` with poison-check
- **Async bridge**: Trait methods are `async fn` but bodies use synchronous `rusqlite` calls; queries are short, mutex never held across `.await`

### Tests
`🌎️hub/📇️directory/🪶️sqlite/🦀️component.rs:348-415`

- `user_space_membership_round_trip` (lines 354-366): Creates users, spaces, memberships; tests role lookup
- `space_kind_membership_laws_are_enforced` (lines 371-389): Archive rejects author; atelier rejects second author; studio allows many
- `sync_session_lifecycle` (lines 393-402): Open/close lifecycle; listable
- `share_token_gating` (lines 406-413): Tokenless open; once issued, only valid token allows access

### Postgres Backend
`🌎️hub/📇️directory/🐘️postgres/🦀️component.rs:26-80`

- **Schema**: Lines 26-80, identical to SQLite (BIGINT for timestamps, no indices declared inline)
- **Pool-based** (line 94): `PgPoolOptions::new().max_connections(20).connect(database_url)` 
- **connect()** (lines 99-105): Uses `sqlx_core::query::query` directly; idempotent SCHEMA split on `;`
- **seed()** (lines 110-121): Same logic as SQLite
- **Tests** (lines 379-436): Use `testcontainers_modules::postgres::Postgres` async container; three tests mirroring SQLite suite

### Neo4j Backend
`🌎️hub/📇️directory/🌐️neo4j/🦀️component.rs:1-60`

- **Constraints** (lines 23-29): Six `CREATE CONSTRAINT IF NOT EXISTS` for `:User.id`, `:Space.id`, `:ShareToken.token`, `:AuthSession.id`, `:SyncSession.id`
- **Neo4jDirectory struct** (lines 32-34): `graph: Graph`
- **connect()** (lines 38-44): `Graph::new(uri, user, password)`; runs all constraints
- **seed()** (lines 47-60): Creates Space & checks User nodes via Cypher queries
- **Tests** (lines 411-418): Empty module with comment noting Neo4j has no in-memory test mode; integration tests require testcontainers/live instance

---

## 3. Hub Binary: Routes, State, Main, and Tests

### Region Names & Line Ranges
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`

- `⚠️ Errors` (35-51)
- `🔖️State` (58-168)
- `🔖️Auth` (170-223)
- `🔖️Rest` (225-295)
- `Blobs` (296-338)
- `🔖️WebSocket` (339-652)
- `🔖️Extensions` (653-715)
- `🔖️Main` (717-843)
- `🔖️Tests` (845-1043+)

### HubState Struct Fields
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:75-107`

```rust
#[derive(Clone)]
struct HubState {
    db: Arc<db::Database>,
    directory: Arc<dyn HubDirectory>,
    admin_token: Option<String>,
    fanout: Arc<DashMap<String, broadcast::Sender<ServerFrame>>>,
    presence: Arc<DashMap<(String, String), Vec<u8>>>,
    schema_hashes: Arc<DashMap<String, [u8; 32]>>,
    extensions_root: std::path::PathBuf,
    merge_policy: protocol::MergePolicy,
}
```

### Router Routes
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:718-727`

```rust
fn router(state: HubState) -> Router {
    Router::new()
        .route("/auth/sessions", post(create_auth_session))
        .route("/extensions", get(list_extensions))
        .route("/extensions/{extension_id}/{*rest}", get(get_extension_asset))
        .route("/spaces/{space_id}/blobs/{hash}", get(get_blob).head(head_blob).put(put_blob))
        .route("/spaces/{space_id}/documents/{id}", get(get_document_status))
        .route("/spaces/{space_id}/documents/{id}/share", post(create_share))
        .route("/spaces/{space_id}/documents/{id}/ws", get(document_ws))
        .with_state(state)
}
```

### Main Function
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:816-842`

- **Env vars**: `OS_HUB_PORT` (default 8787), `OS_HUB_DATA` (default `./.semio/hub/`), `OS_HUB_ADMIN_TOKEN`, `OS_HUB_EXTENSIONS_DIR`
- **connect_db()** (lines 737-771): Reads `OS_HUB_STORAGE_BACKEND` (default "fs"); supports "fs" (→ `{data_dir}/db`), "sqlite", "postgres", "neo4j"
- **connect_directory()** (lines 783-814): Reads `OS_HUB_DIRECTORY_BACKEND` (default "sqlite"); supports "sqlite" (→ `{data_dir}/directory.db`), "postgres", "neo4j"
- **main()**: 
  - Line 819: Tracing init
  - Line 820: Parse port, data_dir
  - Line 821: `mkdir -p {data_dir}`
  - Line 822-823: Call connect_db, connect_directory
  - Line 824-825: Read admin_token, extensions_root
  - Line 827-836: Construct HubState
  - Line 837-840: Bind listener, serve router

### Test Helper Functions
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:845-934`

```rust
// Trait
async fn test_state() -> HubState
    // Creates temp db dir, `:memory:` directory, initializes state

fn sample_envelope(id: &str, document: &WireArtifactId) -> MutationEnvelope
    // Constructs test mutation with pathmap schema

async fn spawn_server(state: HubState) -> SocketAddr
    // Binds TcpListener on 127.0.0.1:0, spawns router task, returns addr

async fn next_server_frame<S>(ws: &mut S) -> ServerFrame
    where S: StreamExt<Item = Result<WsMessage, ...>> + Unpin
    // 5s timeout loop reading binary frames from WebSocket

fn client_binary(frame: &ClientFrame, lane: Lane) -> WsMessage
    // Encodes frame with protocol::encode_client_frame

fn hello(actor: &str) -> ClientFrame
    // Constructs Hello with wire_version=1, protocol_version=1, schema="test.v1", zero pack_schema_hash
```

### Example Test
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:939-969`

```rust
#[tokio::test]
async fn ws_duplex_fan_out() {
    let addr = spawn_server(test_state().await).await;
    let url = format!("ws://{addr}/spaces/{STUDIO}/documents/default/ws");
    
    let (mut a, _) = connect_async(&url).await.unwrap();
    a.send(client_binary(&hello("A"), Lane::Command)).await.unwrap();
    assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Welcome { .. }));
    
    let (mut b, _) = connect_async(&url).await.unwrap();
    b.send(client_binary(&hello("B"), Lane::Command)).await.unwrap();
    assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Welcome { .. }));
    
    let document = WireArtifactId(format!("{STUDIO}:default"));
    a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![sample_envelope("op-1", &document)] }, Lane::Command)).await.unwrap();
    
    assert!(matches!(next_server_frame(&mut a).await, ServerFrame::Ack { batch_id: 1, .. }));
    
    loop {
        match next_server_frame(&mut b).await {
            ServerFrame::Commands { envelopes, origin, .. } => {
                assert_eq!(envelopes.len(), 1);
                assert_eq!(envelopes[0].mutation_id.0, "op-1");
                assert_eq!(origin, ActorId("A".to_string()));
                break;
            }
            ServerFrame::Presence { .. } => continue,
            other => panic!("unexpected frame on B: {other:?}"),
        }
    }
}
```

---

## 4. DB Crate Public API Surface

### Primary Entry Point
`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs:30-33`

```rust
pub use crate::db_engine::{
    CatalogEntry, CatalogView, CommandReceipt, Consistency, Database, DbCapabilities, DbConfig,
    DbHealth, DbStorage, ArtifactHandle, ArtifactSpec, DurabilityClass, Frontier, HistoryEntry,
    HistoryView, LiveQuery, LiveQuerySpec, PreviewHandle, Profile, Query, QueryStream,
    SecurityAuthzHook, SnapshotFuture, SnapshotKind, SnapshotReceipt, SubmitFuture,
};
```

### Database Methods (from bin.rs imports & usage)
- `db::Database::open_at(root: &Path, profile: Profile) -> Result<Database, DbError>` — zero-touch FS storage
- `database.document(id: &ArtifactId) -> Result<ArtifactHandle, DbError>` — get or NotFound
- `database.create_document(spec: ArtifactSpec) -> Result<ArtifactHandle, DbError>` — mint new document
- `database.catalog() -> Result<CatalogView, DbError>` — list documents
- `database.hello(doc_id: &ArtifactId, ...) -> Result<ServerFrame, DbError>` — frontier bootstrap
- `database.storage() -> &dyn DbStorage` — access storage trait
- (Inferred) `database.health() -> ...` — liveness check

### ArtifactHandle Methods
- `handle.submit(batch: CommandReceipt, ...) -> Result<SubmitFuture, DbError>` — enqueue mutation batch
- `handle.frontier() -> Frontier` — current document state edge
- `handle.document_id() -> &ArtifactId` — identity

### DB Sync Module
`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:309`

```rust
pub fn handle_frontier_advertise(
    storage: &dyn db_storage::WalStorage,
    document: ArtifactId,
    advertised: &protocol::RuntimeFrontierSummary,
    origin: protocol::ActorId
) -> Result<Option<protocol::ServerFrame>, DbError>
```
Relay missing commands when remote replica lags; returns None if caught up.

### DB Security Module
Imported: `db::security::*` (bin.rs line 23 pattern); likely contains authz hooks, field redaction, replay guards.

### Storage & Data Layout
- **Root**: `OS_HUB_DATA/db` (default `./.semio/hub/db`)
- **On-disk structure** (FS backend): WAL segments, snapshots, index runs (see `db_wal`, `db_snapshot`, `db_index` modules)
- **Persistence model**: Write-ahead log + versioned snapshots; `db::Frontier` tracks the live edge

---

## 5. Protocol Items

### ClientFrame Variants
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs:49-57`

```rust
pub enum ClientFrame {
    Hello { wire_version, protocol_version, schema, pack_schema_hash, actor, token, resume_token, frontier },
    Commands { batch_id, envelopes: Vec<MutationEnvelope> },
    FrontierAdvertise { frontier },
    PreviewPublish { key, seq, payload },
    Presence { peer: Vec<u8> },
    CreditGrant { n },
    Bye,
}
```

### ServerFrame Variants
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs:95-105`

```rust
pub enum ServerFrame {
    Welcome { session_id, resume_token, server_frontier, bootstrap: Bootstrap },
    SnapshotChunk { seq, bytes },
    SnapshotDone { seq_count },
    Commands { envelopes, origin: ActorId, frontier },
    Ack { batch_id, stages: Vec<AckStage>, frontier },
    Preview { actor, key, seq, payload },
    Presence { peers: Vec<Vec<u8>> },
    CreditGrant { n },
    Error { code, message },
}
```

### Lane
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs:26-43`

```rust
pub enum Lane {
    Command = 0,
    Preview = 1,
}
```

### Codec Functions
- `encode_server_frame(frame: &ServerFrame, lane: Lane) -> Vec<u8>` — line 351
- `decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), ProtocolError>` — line 322
- Both use hand-rolled binary: `lane: u8 | tag: u8 | fields...` (no body-length prefix)

### PresencePeer
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs:718-746`

```rust
pub struct PresencePeer {
    pub actor: String,
    pub label: Option<String>,
    pub presence_pack: Option<Vec<u8>>,  // Flag bit 1
    pub connected_at_ms: i64,
    pub user_id: Option<String>,           // Flag bit 2
    pub role: Option<String>,              // Flag bit 3
    pub cursor: Option<PresencePoint>,     // Flag bit 4
    pub viewport: Option<PresenceViewport>,// Flag bit 5
    pub drag_ghost_json: Option<String>,   // Flag bit 6
    pub interaction: Option<PresenceInteraction>, // Flag bit 7
}
```

**Binary codec** (lines 758-847):
- `encode_presence_peer()`: `actor str | presence_bitmask u8 | connected_at_ms varint | [conditional fields...]`
- `decode_presence_peer()`: Inverse; reads bitmask, unpacks 8 optional fields per bit
- **All 8 bits used** (bits 0-7); bit layout in encode_presence_peer lines 762-786

### RuntimeFrontierSummary
- Aliased from `os_spr::causal::FrontierSummary` (glue.rs line 26)
- Fields (per db_sync bridge): `document_id: ArtifactId`, `head_edit_ordinal: u64`, `head_edit_id: String`, `last_commit_seq: u64`, `chain_hash: [u8; 32]`
- Codec: `encode_frontier()` / `decode_frontier()` in causal module

### MergePolicy
`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️component.rs:331-339`

```rust
#[derive(Clone, Copy, Debug)]
pub struct MergePolicy {
    pub max_runs_before_merge: usize,
}

impl Default for MergePolicy {
    fn default() -> Self {
        Self { max_runs_before_merge: 4 }
    }
}
```
Index LSM-lite trigger: merge two oldest runs when live run count exceeds threshold.

---

## 6. Axum Static-File Serving Pattern

### get_extension_asset Handler
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:709-714`

```rust
async fn get_extension_asset(
    Path((extension_id, rest)): Path<(String, String)>,
    State(state): State<HubState>
) -> Result<impl IntoResponse, StatusCode> {
    let path = extension_asset_path(&state.extensions_root, &extension_id, &rest)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let bytes = tokio::fs::read(&path).await
        .map_err(|error| if error.kind() == std::io::ErrorKind::NotFound { StatusCode::NOT_FOUND } else { StatusCode::INTERNAL_SERVER_ERROR })?;
    let content_type = extension_asset_content_type(&path);
    Ok(([(axum::http::header::CONTENT_TYPE, content_type)], bytes))
}
```

### Path Validation
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:669-682`

```rust
fn extension_asset_path(root: &std::path::Path, extension_id: &str, rest: &str) -> Option<std::path::PathBuf> {
    if extension_id.is_empty() || extension_id.contains('/') || extension_id.contains('\\') || extension_id.contains("..") {
        return None;
    }
    if rest.is_empty() || rest.contains("..") {
        return None;
    }
    let base = root.join(extension_id);
    let path = base.join(rest);
    if !path.starts_with(&base) {
        return None;
    }
    Some(path)
}
```

### Content-Type Mapping
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:660-667`

```rust
fn extension_asset_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("js") | Some("mjs") => "text/javascript",
        Some("wasm") => "application/wasm",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
```

### Key Pattern
- **No `tower-http`**: Plain axum handlers + `tokio::fs::read()`
- **No `include_dir`**: Dynamic directory walk at `/extensions` route (lines 684-707)
- **Return tuple**: `(HeaderMap, Vec<u8>)` as `IntoResponse`; no custom wrapper type needed

---

## 7. Dependencies & Workspace Pins

### Semio-Hub Cargo.toml
`🌎️hub/📦️packages/🦀️rust/Cargo.toml:36,41`

**Direct dependencies** (not from workspace):
- `uuid = { version = "1.20", features = ["v7", "serde"] }` — present
- `dashmap = "6"` — present
- `tokio = { workspace = true, features = ["full"] }` — present (includes `broadcast`, `Notify`, `Mutex`)

**From workspace pinned** (`Cargo.toml:119-134`):
- `serde_json = "1.0.149"` — present
- `tokio = { version = "1" }` — present (workspace pin; features added locally)

**Protocol & DB**:
- `protocol` (path `../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`, package `semio-framework-os-kernel`)
- `db` (path `../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust`, package `semio-framework-os-kernel-db`)

### Feature Coordination
`🌎️hub/📦️packages/🦀️rust/Cargo.toml:21-29`

```toml
[features]
default = ["sqlite"]
sqlite = ["dep:rusqlite", "db/sqlite"]
postgres = ["dep:sqlx_core", "dep:sqlx_postgres", "db/postgres"]
neo4j = ["dep:neo4rs", "db/neo4j"]
```

One feature name controls both directory backend AND storage backend (no split-brain).

### Optional Dependencies
- `rusqlite = { version = "0.38.0", features = ["bundled"], optional = true }`
- `sqlx_core` + `sqlx_postgres` (0.8 with uuid/json features)
- `neo4rs = { version = "0.8", optional = true }`

