//! 📦️ DIN EN 16798 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Din16798Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Din16798Snapshot, PackError> {
    <Din16798Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod native_snapshot_codec {
use crate::binary::snapshot::*;
use crate::binary::snapshot::*;
use semio_s_artifact_norm_din16798_model::*;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_din16798_model::{VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;
use crate::text::snapshot::*;
pub fn encode_din16798_pack(snapshot: &Din16798Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
pub fn decode_din16798_pack(bytes: &[u8]) -> Result<Din16798Snapshot, String> {
    <Din16798Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
}
pub use crate::binary::snapshot::native_snapshot_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary Din16798Snapshot, extension="din16798", envelope_id="norm.din16798", sqlite=crate::sqlite::snapshot::sqlite_codec);
