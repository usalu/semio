//! 📦️ EN 1999 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::En1999Snapshot;
use store::PackError;

/// 📦️ Encodes a `En1999Snapshot` to its binary pack form.
pub fn encode(document: &En1999Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `En1999Snapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1999Snapshot, PackError> {
    <En1999Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
pub fn decode_en1999_pack(bytes: &[u8]) -> Result<En1999Snapshot, String> {
    <En1999Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1999_pack(snapshot: &En1999Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
