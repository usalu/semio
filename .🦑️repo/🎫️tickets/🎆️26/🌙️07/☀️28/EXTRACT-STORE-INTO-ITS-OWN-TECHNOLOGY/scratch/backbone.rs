//#region 🔖️Backbone
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioConflict {
    pub kind: String,
    pub uri: String,
    pub message: String,
}

/// @emoji 🎞️ Maps `protocol_command::Operation::reconcile`'s new `Vec<ReconcileReport>` result onto
/// this crate's own conflict type — see `reconcile_with_last`'s doc comment for why the mapping
/// happens at this edge rather than `protocol_command` knowing about `StudioConflict` directly.
/// `kind: report.id` verbatim (NOT prefixed with severity) — a technology's own `reconcile` override
/// (e.g. `framework/product/os/core`'s `OsOperation`) round-trips its own `StudioConflict.kind`
/// through `ReconcileReport.id` on the way in (see that crate's `reconcile` wrapper), and callers
/// pattern-match `StudioConflict.kind` against exact strings (e.g. `"media-graph/edge-orphaned"`) —
/// mangling it here would silently break every such exact-match call site. `severity` has no
/// `StudioConflict` field to land in, so it is dropped (a real, structural information loss inherent
/// to `ReconcileReport`'s frozen shape, not fixable at this edge). `ReconcileReport` also has no
/// URI-shaped field (it targets a schema-opaque `id`, not a studio member resource), so `uri` is
/// left empty for any report that didn't originate from a `StudioConflict` round-trip.
impl From<ReconcileReport> for StudioConflict {
    fn from(report: ReconcileReport) -> Self {
        StudioConflict {
            kind: report.id,
            uri: String::new(),
            message: report.message,
        }
    }
}

/// @emoji 📨️ Wire message exchanged over an attached backbone channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum BackboneMessage {
    Snapshot { envelope_json: String },
    Operations { envelopes: Vec<OperationEnvelope> },
    /// @emoji ✅️ Acknowledges inbound operations the store has ingested (store→actor). Lets a future actor
    /// implement at-least-once redelivery with id-based dedupe — safe across store crashes/reloads.
    Ack { op_ids: Vec<String> },
}

/// @emoji 🧵️ Non-blocking, IO-free in-memory queue contract between a `DocumentStore` and its
/// sync actor. `send`/`receive` MUST return immediately: implementations only enqueue/dequeue
/// `BackboneMessage`s — never HTTP, never filesystem, never a blocking wait. All IO (persistence,
/// hub sync, file watching, presence) lives behind this queue in `framework/sync`'s actor layer,
/// which owns the other end; the store's `pump()`/`flush_outbound()` run synchronously on the
/// caller's thread and must never be blocked by transport work.
///
/// URI schemes are resolved by the host actor (`framework/sync`): `temp://` (in-memory),
/// `file://` (single JSON blob), `folder://` (sqlite `.semio/document.db`), `remote://` (OS hub).
pub trait Backbone: Send + Sync {
    fn descriptor(&self) -> DocumentBackboneRef;
    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError>;
}

pub trait BackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

static HOST_BACKBONE_PORT: Mutex<Option<Arc<dyn BackbonePort>>> = Mutex::new(None);

/// @emoji 🔌️ Injects the browser or dev-server backbone port for wasm file/folder IO.
pub fn set_host_backbone_port(port: Arc<dyn BackbonePort>) {
    if let Ok(mut guard) = HOST_BACKBONE_PORT.lock() {
        *guard = Some(port);
    }
}

fn host_backbone_port() -> Option<Arc<dyn BackbonePort>> {
    HOST_BACKBONE_PORT.lock().ok().and_then(|guard| guard.clone())
}

#[derive(Default)]
pub struct MemoryBackbonePort {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryBackbonePort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BackbonePort for MemoryBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .get(uri)
            .cloned()
            .ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .insert(uri.to_string(), payload.to_string());
        Ok(())
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn local_storage_backbone_key(uri: &str) -> String {
    format!("semio:vcs:{uri}")
}

/// @emoji 💾️ Browser `localStorage` backbone port with in-memory fallback for native tests.
pub struct LocalStorageBackbonePort {
    fallback: MemoryBackbonePort,
}

impl LocalStorageBackbonePort {
    pub fn new() -> Self {
        Self {
            fallback: MemoryBackbonePort::new(),
        }
    }
}

impl Default for LocalStorageBackbonePort {
    fn default() -> Self {
        Self::new()
    }
}

impl BackbonePort for LocalStorageBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        if let Some(port) = host_backbone_port() {
            if let Ok(value) = port.read(uri) {
                return Ok(value);
            }
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(&local_storage_backbone_key(uri)) {
                        return Ok(value);
                    }
                }
            }
        }
        self.fallback.read(uri)
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.fallback.write(uri, payload)?;
        if let Some(port) = host_backbone_port() {
            let _ = port.write(uri, payload);
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(&local_storage_backbone_key(uri), payload);
                }
            }
        }
        Ok(())
    }
}

/// @emoji 🕸️ Injectable duplex transport across the wasm sandbox boundary (plugin ↔ host process).
pub trait BackboneChannelPort: Send + Sync {
    fn send(&self, uri: &str, message_json: &str) -> Result<(), VcsError>;
    fn poll(&self, uri: &str) -> Result<Vec<String>, VcsError>;
}

static HOST_BACKBONE_CHANNEL: Mutex<Option<Arc<dyn BackboneChannelPort>>> = Mutex::new(None);

/// @emoji 🔌️ Injects the plugin host's duplex backbone channel for wasm-sandboxed document stores.
pub fn set_host_backbone_channel(channel: Arc<dyn BackboneChannelPort>) {
    if let Ok(mut guard) = HOST_BACKBONE_CHANNEL.lock() {
        *guard = Some(channel);
    }
}

fn host_backbone_channel() -> Option<Arc<dyn BackboneChannelPort>> {
    HOST_BACKBONE_CHANNEL.lock().ok().and_then(|guard| guard.clone())
}

/// @emoji 🧵️ Backbone that forwards messages across the wasm sandbox boundary to the host process,
/// which resolves the real `file://`/`folder://`/`remote://` backbone on its own (native) side.
pub struct PortBackbone {
    uri: String,
}

impl PortBackbone {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.to_string() }
    }
}

impl Backbone for PortBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        let channel = host_backbone_channel()
            .ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let json = serde_json::to_string(&message).map_err(|e| VcsError::Serialize(e.to_string()))?;
        channel.send(&self.uri, &json)
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let channel = host_backbone_channel()
            .ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        channel
            .poll(&self.uri)?
            .into_iter()
            .map(|json| serde_json::from_str(&json).map_err(|e| VcsError::Deserialize(e.to_string())))
            .collect()
    }
}

/// @emoji 🔗️ Two crossed in-memory channel ends: whatever `a` sends, `b` receives, and vice versa.
pub struct MemoryBackbone {
    uri: String,
    inbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl MemoryBackbone {
    pub fn pair(uri_a: &str, uri_b: &str) -> (Self, Self) {
        let a_to_b = Arc::new(Mutex::new(VecDeque::new()));
        let b_to_a = Arc::new(Mutex::new(VecDeque::new()));
        (
            Self {
                uri: uri_a.to_string(),
                inbox: b_to_a.clone(),
                outbox: a_to_b.clone(),
            },
            Self {
                uri: uri_b.to_string(),
                inbox: a_to_b,
                outbox: b_to_a,
            },
        )
    }
}

impl Backbone for MemoryBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbox
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbox = self.inbox.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbox.drain(..).collect())
    }
}

/// @emoji 🔗️ The store-side end of a pair of crossed in-memory queues. Implements the non-blocking
/// {@link Backbone} contract; the matching {@link ChannelBackboneRemote} is held by an external sync
/// actor (built in `framework/sync`, a later workstream) that pushes inbound messages and drains the
/// store's outbound ones. This crate only provides the queue plumbing — never the actor itself.
pub struct ChannelBackbone {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

/// @emoji 🎛️ The actor-side end paired with a {@link ChannelBackbone}: `push` delivers a message to
/// the store's inbound queue, `drain` collects everything the store has sent outbound. Not a
/// `Backbone` — this is the handle an IO-owning actor endpoint holds across the store boundary.
pub struct ChannelBackboneRemote {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl ChannelBackbone {
    /// @emoji 🔗️ Creates a crossed pair sharing a URI: the store attaches the `ChannelBackbone`; the
    /// actor keeps the `ChannelBackboneRemote`.
    pub fn pair(uri: &str) -> (ChannelBackbone, ChannelBackboneRemote) {
        let inbound = Arc::new(Mutex::new(VecDeque::new()));
        let outbound = Arc::new(Mutex::new(VecDeque::new()));
        (
            ChannelBackbone {
                uri: uri.to_string(),
                inbound: inbound.clone(),
                outbound: outbound.clone(),
            },
            ChannelBackboneRemote {
                uri: uri.to_string(),
                inbound,
                outbound,
            },
        )
    }
}

impl Backbone for ChannelBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbound
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbound = self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbound.drain(..).collect())
    }
}

impl ChannelBackboneRemote {
    pub fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    /// @emoji 📥️ Delivers a message to the store's inbound queue (actor→store).
    pub fn push(&self, message: BackboneMessage) -> Result<(), VcsError> {
        self.inbound
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    /// @emoji 📤️ Collects everything the store has sent outbound (store→actor), draining the queue.
    pub fn drain(&self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut outbound = self.outbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(outbound.drain(..).collect())
    }
}

/// @emoji 🗄️ Pure multi-document sqlite persistence (`folder://`), the canonical local store. Rows
/// are keyed by document id: `document(id, schema, json, updated_at)` — a single folder holds every
/// open document's envelope. No `Backbone` impl: the `framework/sync` actor layer drives this from
/// its own thread; this crate only owns the sqlite schema.
#[cfg(not(target_arch = "wasm32"))]
pub struct FolderSqliteStorage {
    folder: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderSqliteStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        Self { folder }
    }

    fn db_path(&self) -> std::path::PathBuf {
        self.folder.join(".semio").join("documents.db")
    }

    fn connection(&self) -> Result<rusqlite::Connection, VcsError> {
        let semio_dir = self.folder.join(".semio");
        std::fs::create_dir_all(&semio_dir).map_err(|e| VcsError::Backbone(e.to_string()))?;
        let conn = rusqlite::Connection::open(self.db_path()).map_err(|e| VcsError::Backbone(e.to_string()))?;
        Self::ensure_schema(&conn)?;
        Ok(conn)
    }

    fn ensure_schema(conn: &rusqlite::Connection) -> Result<(), VcsError> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS document (\
                 id TEXT PRIMARY KEY,\
                 schema TEXT,\
                 json TEXT NOT NULL,\
                 pack BLOB,\
                 updated_at INTEGER NOT NULL\
             );\
             CREATE TABLE IF NOT EXISTS blobs (\
                 hash TEXT PRIMARY KEY,\
                 media_type TEXT NOT NULL,\
                 size INTEGER NOT NULL,\
                 bytes BLOB NOT NULL\
             );",
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖️ Reads the stored envelope JSON for `document_id`, or `None` if absent.
    pub fn read(&self, document_id: &str) -> Result<Option<String>, VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT json FROM document WHERE id = ?1", [document_id], |row| row.get(0))
            .optional()
            .map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji ✍️ Upserts `document_id`'s envelope JSON (with its schema id and an `updated_at` stamp).
    pub fn write(&self, document_id: &str, schema: &str, envelope_json: &str) -> Result<(), VcsError> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO document (id, schema, json, updated_at) VALUES (?1, ?2, ?3, ?4) \
             ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, json = excluded.json, updated_at = excluded.updated_at",
            rusqlite::params![document_id, schema, envelope_json, now_ms() as i64],
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(())
    }

    /// @emoji 📖️ Reads the stored pack bytes for `document_id`, or `None` if absent — no row, or a
    /// row written before the `pack` column existed (SQL `NULL`, surfaced as `None` the same way).
    pub fn read_pack(&self, document_id: &str) -> Result<Option<Vec<u8>>, VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT pack FROM document WHERE id = ?1", [document_id], |row| row.get::<_, Option<Vec<u8>>>(0))
            .optional()
            .map(|row| row.flatten())
            .map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji ✍️ Upserts `document_id`'s envelope JSON + pack bytes together (schema id, `updated_at`
    /// stamp) — the pack-aware sibling of `write`.
    pub fn write_pack(&self, document_id: &str, schema: &str, envelope_json: &str, pack: &[u8]) -> Result<(), VcsError> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO document (id, schema, json, pack, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) \
             ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, json = excluded.json, pack = excluded.pack, updated_at = excluded.updated_at",
            rusqlite::params![document_id, schema, envelope_json, pack, now_ms() as i64],
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(())
    }

    /// @emoji 📇️ Lists every stored document id (newest write first), for a folder-wide index.
    pub fn document_ids(&self) -> Result<Vec<String>, VcsError> {
        let conn = self.connection()?;
        let mut statement = conn
            .prepare("SELECT id FROM document ORDER BY updated_at DESC")
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        let ids = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| VcsError::Backbone(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(ids)
    }
}

/// @emoji 🗃️ Textual persistence for one folder of documents: `<id>.<ext>` holds the DSL text (initial
/// projection), `<id>.<ext>.ops` holds the append-only op log (see {@link print_document_text}/
/// {@link parse_document_text}). No `Backbone` impl: like {@link FileJsonStorage}/
/// {@link FolderSqliteStorage}, the `framework/sync` actor layer drives this from its own thread; this
/// crate only owns the file format. Additive alongside the JSON/sqlite storages today — a technology
/// adopts it by implementing `DocumentDsl`/`OpText` and having its sync endpoint construct one of
/// these instead; nothing currently reads or writes through it automatically.
#[cfg(not(target_arch = "wasm32"))]
pub struct FolderTextStorage {
    folder: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderTextStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        Self { folder }
    }

    fn dsl_path(&self, document_id: &str, extension: &str) -> std::path::PathBuf {
        self.folder.join(format!("{document_id}.{extension}"))
    }

    fn ops_path(&self, document_id: &str, extension: &str) -> std::path::PathBuf {
        self.folder.join(format!("{document_id}.{extension}.ops"))
    }

    /// @emoji 🏷️ Path of the authoritative binary pack file — `dsl_path` with a `.pack` suffix.
    pub fn pack_path(&self, document_id: &str, extension: &str) -> std::path::PathBuf {
        self.folder.join(format!("{document_id}.{extension}.pack"))
    }

    /// @emoji 📖️ Reads both files for `document_id`, or `None` if the DSL file does not exist yet.
    pub fn read(&self, document_id: &str, extension: &str) -> Result<Option<DocumentTextFiles>, VcsError> {
        let dsl = match std::fs::read_to_string(self.dsl_path(document_id, extension)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(VcsError::Backbone(err.to_string())),
        };
        let ops = match std::fs::read_to_string(self.ops_path(document_id, extension)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(VcsError::Backbone(err.to_string())),
        };
        Ok(Some(DocumentTextFiles { dsl, ops }))
    }

    /// @emoji ✍️ Overwrites both files wholesale (the structural-command cold path — undo/redo/
    /// checkpoint/alternative — mirrors `FileJsonStorage::write`'s whole-envelope semantics).
    pub fn write(&self, document_id: &str, extension: &str, files: &DocumentTextFiles) -> Result<(), VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, extension), &files.dsl).map_err(|e| VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, extension), &files.ops).map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖️ Pack-first read: reads the pack bytes + op log for `document_id`, or `None` if the
    /// `.pack` file itself doesn't exist (unlike `read`, the DSL mirror's existence alone doesn't
    /// count — pack is authoritative per the disk-layout LAW, the DSL file is import-only).
    pub fn read_pack(&self, document_id: &str, extension: &str) -> Result<Option<DocumentPackFiles>, VcsError> {
        let pack = match std::fs::read(self.pack_path(document_id, extension)) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(VcsError::Backbone(err.to_string())),
        };
        let ops = match std::fs::read_to_string(self.ops_path(document_id, extension)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(VcsError::Backbone(err.to_string())),
        };
        Ok(Some(DocumentPackFiles { pack, ops }))
    }

    /// @emoji ✍️ Overwrites all three files: the authoritative `.pack`, the shared `.ops` log, and the
    /// always-written DSL mirror `dsl_mirror` (`print_dsl` on the initial projection) — the pack-aware
    /// sibling of `write`.
    pub fn write_pack(&self, document_id: &str, extension: &str, files: &DocumentPackFiles, dsl_mirror: &str) -> Result<(), VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.pack_path(document_id, extension), &files.pack).map_err(|e| VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, extension), &files.ops).map_err(|e| VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, extension), dsl_mirror).map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji ➕️ Appends already-printed op-log lines (one {@link print_edit_lines} block) to the `.ops`
    /// file without rewriting it — the hot-path append unit, O(new edit) instead of O(whole history).
    pub fn append_ops(&self, document_id: &str, extension: &str, lines: &str) -> Result<(), VcsError> {
        use std::io::Write;
        std::fs::create_dir_all(&self.folder).map_err(|e| VcsError::Backbone(e.to_string()))?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.ops_path(document_id, extension))
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        file.write_all(lines.as_bytes()).map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📇️ Lists every stored document id (by DSL file stem) for a given extension.
    pub fn document_ids(&self, extension: &str) -> Result<Vec<String>, VcsError> {
        let suffix = format!(".{extension}");
        let entries = match std::fs::read_dir(&self.folder) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(VcsError::Backbone(err.to_string())),
        };
        let mut ids = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| VcsError::Backbone(e.to_string()))?;
            if let Some(name) = entry.file_name().to_str() {
                if let Some(id) = name.strip_suffix(&suffix) {
                    ids.push(id.to_string());
                }
            }
        }
        Ok(ids)
    }
}

/// @emoji 🔌️ Resolves a backbone URI to a concrete channel implementation. Only available inside the
/// wasm sandbox, where every scheme forwards to the host process over the injected
/// {@link BackboneChannelPort} (a pure in-memory queue). Native IO-performing backbones moved out of
/// this crate entirely — the `framework/sync` actor layer owns them.
#[cfg(target_arch = "wasm32")]
pub fn resolve_backbone(uri: &str) -> Result<Box<dyn Backbone>, VcsError> {
    Ok(Box::new(PortBackbone::new(uri)))
}
//#endregion 🔖️Backbone
