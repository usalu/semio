//#region 🔖️BlobStore
/// @emoji 📦️ A content-addressed blob's identity + metadata. Never carries the bytes themselves —
/// callers that just put/read a blob already hold those; this is what gets embedded in a document
/// (e.g. a `MergeStrategyKind::ContentAddressedBlob` field) to reference it durably.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobRef {
    pub hash: String,
    pub size: u64,
    pub media_type: String,
}

/// @emoji 🗄️ Content-addressed blob persistence backing `MergeStrategyKind::ContentAddressedBlob` /
/// `DocumentKind::ContentAddressedBlob` (`framework/core/rs` 🔖️MergeStrategy region). `put` is idempotent —
/// it dedupes by the Blake3 hash of the bytes ({@link semio_framework_hash::hash_bytes}), so writing
/// the same content twice never rewrites storage. Implementors decide the backing medium (sqlite here,
/// a semio_hub HTTP route in a later ticket, an IndexedDB cache in the browser).
pub trait BlobStore: Send + Sync {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError>;
    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError>;
    fn has(&self, hash: &str) -> Result<bool, VcsError>;
    fn delete(&self, hash: &str) -> Result<(), VcsError>;
}

/// @emoji 🗄️ `FolderSqliteStorage`'s `blobs(hash, media_type, size, bytes)` table (bootstrapped
/// alongside `document` in `ensure_schema`) — one whole-blob `BLOB` column is plenty for v1; this
/// crate's other tables don't chunk large payloads either, and the `BlobStore` trait itself stays
/// whole-blob regardless of how a given backend chooses to store the bytes internally.
#[cfg(not(target_arch = "wasm32"))]
impl BlobStore for FolderSqliteStorage {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError> {
        let hash = hash_bytes(bytes);
        let conn = self.connection()?;
        conn.execute(
            "INSERT OR IGNORE INTO blobs (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![hash, media_type, bytes.len() as i64, bytes],
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT bytes FROM blobs WHERE hash = ?1", [hash], |row| row.get(0))
            .optional()
            .map_err(|e| VcsError::Backbone(e.to_string()))
    }

    fn has(&self, hash: &str) -> Result<bool, VcsError> {
        let conn = self.connection()?;
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM blobs WHERE hash = ?1", [hash], |row| row.get(0))
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(count > 0)
    }

    fn delete(&self, hash: &str) -> Result<(), VcsError> {
        let conn = self.connection()?;
        conn.execute("DELETE FROM blobs WHERE hash = ?1", [hash])
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(())
    }
}
//#endregion 🔖️BlobStore
