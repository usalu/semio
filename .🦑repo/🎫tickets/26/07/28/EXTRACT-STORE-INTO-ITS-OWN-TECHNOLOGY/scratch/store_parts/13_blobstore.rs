//#region 🔖BlobStore
/// @emoji 📦 A content-addressed blob's identity + metadata. Never carries the bytes themselves —
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
/// `DocumentKind::ContentAddressedBlob` (`framework/core/rs` 🔖MergeStrategy region). `put` is idempotent —
/// it dedupes by the Blake3 hash of the bytes ({@link semio_framework_hash::hash_bytes}), so writing
/// the same content twice never rewrites storage. Implementors decide the backing medium (sqlite here,
/// a hub HTTP route in a later ticket, an IndexedDB cache in the browser).
pub trait BlobStore: Send + Sync {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError>;
    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError>;
    fn has(&self, hash: &str) -> Result<bool, VcsError>;
    fn delete(&self, hash: &str) -> Result<(), VcsError>;
}
