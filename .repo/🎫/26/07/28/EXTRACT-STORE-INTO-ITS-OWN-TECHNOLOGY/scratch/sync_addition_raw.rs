
//#region 🔖FolderStorage
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

    fn connection(&self) -> Result<rusqlite::Connection, vcs::VcsError> {
        let semio_dir = self.folder.join(".semio");
        std::fs::create_dir_all(&semio_dir).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        let conn = rusqlite::Connection::open(self.db_path()).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Self::ensure_schema(&conn)?;
        Ok(conn)
    }

    fn ensure_schema(conn: &rusqlite::Connection) -> Result<(), vcs::VcsError> {
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
        .map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖 Reads the stored envelope JSON for `document_id`, or `None` if absent.
    pub fn read(&self, document_id: &str) -> Result<Option<String>, vcs::VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT json FROM document WHERE id = ?1", [document_id], |row| row.get(0))
            .optional()
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji ✍️ Upserts `document_id`'s envelope JSON (with its schema id and an `updated_at` stamp).
    pub fn write(&self, document_id: &str, schema: &str, envelope_json: &str) -> Result<(), vcs::VcsError> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO document (id, schema, json, updated_at) VALUES (?1, ?2, ?3, ?4) \
             ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, json = excluded.json, updated_at = excluded.updated_at",
            rusqlite::params![document_id, schema, envelope_json, now_ms() as i64],
        )
        .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(())
    }

    /// @emoji 📖 Reads the stored pack bytes for `document_id`, or `None` if absent — no row, or a
    /// row written before the `pack` column existed (SQL `NULL`, surfaced as `None` the same way).
    pub fn read_pack(&self, document_id: &str) -> Result<Option<Vec<u8>>, vcs::VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT pack FROM document WHERE id = ?1", [document_id], |row| row.get::<_, Option<Vec<u8>>>(0))
            .optional()
            .map(|row| row.flatten())
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji ✍️ Upserts `document_id`'s envelope JSON + pack bytes together (schema id, `updated_at`
    /// stamp) — the pack-aware sibling of `write`.
    pub fn write_pack(&self, document_id: &str, schema: &str, envelope_json: &str, pack: &[u8]) -> Result<(), vcs::VcsError> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO document (id, schema, json, pack, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) \
             ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, json = excluded.json, pack = excluded.pack, updated_at = excluded.updated_at",
            rusqlite::params![document_id, schema, envelope_json, pack, now_ms() as i64],
        )
        .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(())
    }

    /// @emoji 📇 Lists every stored document id (newest write first), for a folder-wide index.
    pub fn document_ids(&self) -> Result<Vec<String>, vcs::VcsError> {
        let conn = self.connection()?;
        let mut statement = conn
            .prepare("SELECT id FROM document ORDER BY updated_at DESC")
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        let ids = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(ids)
    }
}

/// @emoji 🗃️ Textual persistence for one folder of documents: `<id>.<ext>` holds the DSL text (initial
/// projection), `<id>.<ext>.ops` holds the append-only op log (see `store::print_document_text`/
/// `store::parse_document_text`). No `Backbone` impl: like `FolderSqliteStorage` above, this actor
/// layer drives it from its own thread; this crate only owns the file format. Additive alongside the
/// sqlite storage today — a technology adopts it by implementing `DocumentDsl`/`OpText` and having
/// its sync endpoint construct one of these instead; nothing currently reads or writes through it
/// automatically.
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

    /// @emoji 📖 Reads both files for `document_id`, or `None` if the DSL file does not exist yet.
    pub fn read(&self, document_id: &str, extension: &str) -> Result<Option<DocumentTextFiles>, vcs::VcsError> {
        let dsl = match std::fs::read_to_string(self.dsl_path(document_id, extension)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let ops = match std::fs::read_to_string(self.ops_path(document_id, extension)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(DocumentTextFiles { dsl, ops }))
    }

    /// @emoji ✍️ Overwrites both files wholesale (the structural-command cold path — undo/redo/
    /// checkpoint/alternative — mirrors `FileJsonStorage::write`'s whole-envelope semantics).
    pub fn write(&self, document_id: &str, extension: &str, files: &DocumentTextFiles) -> Result<(), vcs::VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, extension), &files.dsl).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, extension), &files.ops).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖 Pack-first read: reads the pack bytes + op log for `document_id`, or `None` if the
    /// `.pack` file itself doesn't exist (unlike `read`, the DSL mirror's existence alone doesn't
    /// count — pack is authoritative per the disk-layout LAW, the DSL file is import-only).
    pub fn read_pack(&self, document_id: &str, extension: &str) -> Result<Option<DocumentPackFiles>, vcs::VcsError> {
        let pack = match std::fs::read(self.pack_path(document_id, extension)) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let ops = match std::fs::read_to_string(self.ops_path(document_id, extension)) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        Ok(Some(DocumentPackFiles { pack, ops }))
    }

    /// @emoji ✍️ Overwrites all three files: the authoritative `.pack`, the shared `.ops` log, and the
    /// always-written DSL mirror `dsl_mirror` (`print_dsl` on the initial projection) — the pack-aware
    /// sibling of `write`.
    pub fn write_pack(&self, document_id: &str, extension: &str, files: &DocumentPackFiles, dsl_mirror: &str) -> Result<(), vcs::VcsError> {
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.pack_path(document_id, extension), &files.pack).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.ops_path(document_id, extension), &files.ops).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        std::fs::write(self.dsl_path(document_id, extension), dsl_mirror).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji ➕ Appends already-printed op-log lines (one {@link print_edit_lines} block) to the `.ops`
    /// file without rewriting it — the hot-path append unit, O(new edit) instead of O(whole history).
    pub fn append_ops(&self, document_id: &str, extension: &str, lines: &str) -> Result<(), vcs::VcsError> {
        use std::io::Write;
        std::fs::create_dir_all(&self.folder).map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.ops_path(document_id, extension))
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        file.write_all(lines.as_bytes()).map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📇 Lists every stored document id (by DSL file stem) for a given extension.
    pub fn document_ids(&self, extension: &str) -> Result<Vec<String>, vcs::VcsError> {
        let suffix = format!(".{extension}");
        let entries = match std::fs::read_dir(&self.folder) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(vcs::VcsError::Backbone(err.to_string())),
        };
        let mut ids = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
            if let Some(name) = entry.file_name().to_str() {
                if let Some(id) = name.strip_suffix(&suffix) {
                    ids.push(id.to_string());
                }
            }
        }
        Ok(ids)
    }
}
//#endregion 🔖FolderStorage

//#region 🔖BlobStoreImpl

/// @emoji 🗄️ `FolderSqliteStorage`'s `blobs(hash, media_type, size, bytes)` table (bootstrapped
/// alongside `document` in `ensure_schema`) — one whole-blob `BLOB` column is plenty for v1; this
/// crate's other tables don't chunk large payloads either, and the `store::BlobStore` trait itself stays
/// whole-blob regardless of how a given backend chooses to store the bytes internally.
#[cfg(not(target_arch = "wasm32"))]
impl store::BlobStore for FolderSqliteStorage {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, vcs::VcsError> {
        let hash = semio_framework_hash::hash_bytes(bytes);
        let conn = self.connection()?;
        conn.execute(
            "INSERT OR IGNORE INTO blobs (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![hash, media_type, bytes.len() as i64, bytes],
        )
        .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, vcs::VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT bytes FROM blobs WHERE hash = ?1", [hash], |row| row.get(0))
            .optional()
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))
    }

    fn has(&self, hash: &str) -> Result<bool, vcs::VcsError> {
        let conn = self.connection()?;
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM blobs WHERE hash = ?1", [hash], |row| row.get(0))
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(count > 0)
    }

    fn delete(&self, hash: &str) -> Result<(), vcs::VcsError> {
        let conn = self.connection()?;
        conn.execute("DELETE FROM blobs WHERE hash = ?1", [hash])
            .map_err(|e| vcs::VcsError::Backbone(e.to_string()))?;
        Ok(())
    }
}
//#endregion 🔖BlobStoreImpl
