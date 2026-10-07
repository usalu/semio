//! 📦️ DIN EN 16798 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::Din16798Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Din16798Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Din16798Snapshot, PackError> {
    <Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes)
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
use crate::{VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
pub fn encode_din16798_pack(snapshot: &Din16798Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
pub fn decode_din16798_pack(bytes: &[u8]) -> Result<Din16798Snapshot, String> {
    <Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
}
pub use native_snapshot_codec::*;

crate::impl_norm_artifact_record!(@binary crate::Din16798Snapshot, extension="din16798", envelope_id="norm.din16798", sqlite=crate::standards::v1::subsets::any::io::sqlite::snapshot::sqlite_codec);
