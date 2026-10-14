//! 📦️ EN 1991 actions on structures — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &En1991Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1991Snapshot, PackError> {
    <En1991Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod native_snapshot_codec {
use crate::binary::snapshot::*;
use crate::binary::snapshot::*;
use semio_s_artifact_norm_en1991_model::*;
use semio_s_artifact_norm_en1991_model::{AccidentalCase, FloorArea, RoofArea, SelfWeightElement, WindFace};
use framework_schema::ArtifactSchema;
use crate::text::snapshot::*;
/// 📦️ Decode `.pack.semio` envelope.
pub fn decode_en1991_pack(bytes: &[u8]) -> Result<En1991Snapshot, String> {
    <En1991Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encode `.pack.semio` envelope.
pub fn encode_en1991_pack(snapshot: &En1991Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
}
pub use crate::binary::snapshot::native_snapshot_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary En1991Snapshot, extension="en1991", envelope_id="norm.en1991", sqlite=crate::sqlite::snapshot::sqlite_codec);
