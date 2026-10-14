//! 📦️ EN 1992 design of concrete structures — binary document surface + laws (constitutional: pack).

use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &En1992Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1992Snapshot, PackError> {
    <En1992Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod native_snapshot_codec {
use crate::binary::snapshot::*;
use crate::binary::snapshot::*;
use semio_s_artifact_norm_en1992_model::*;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1992_model::{Anchor, BarLayer, ConcreteGrade, ExposureClass, FireRating, FireSpec, LoadCaseActions, MemberKind, PrestressSpec, PrestressSteel, PunchingSpec, RcMember, ReinforcementGrade, Stirrups, SupportCondition, TightnessClass};
use framework_schema::ArtifactSchema;
use crate::text::snapshot::*;
/// 📦️ Decodes `.pack.semio` envelope.
pub fn decode_en1992_pack(bytes: &[u8]) -> Result<En1992Snapshot, String> {
    <En1992Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes `.pack.semio` envelope.
pub fn encode_en1992_pack(snapshot: &En1992Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
}
pub use crate::binary::snapshot::native_snapshot_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary En1992Snapshot, extension="en1992", envelope_id="norm.en1992", sqlite=crate::sqlite::snapshot::codec);
