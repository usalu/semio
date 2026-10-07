//! 📦️ EN 1992 design of concrete structures — binary document surface + laws (constitutional: pack).

use crate::En1992Snapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &En1992Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1992Snapshot, PackError> {
    <En1992Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::document::AnnexChoice;
use crate::{
    Anchor, BarLayer, ConcreteGrade, ExposureClass, FireRating, FireSpec, LoadCaseActions, MemberKind, PrestressSpec, PrestressSteel, PunchingSpec, RcMember, ReinforcementGrade, Stirrups, SupportCondition,
    TightnessClass,
};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
/// 📦️ Decodes `.pack.semio` envelope.
pub fn decode_en1992_pack(bytes: &[u8]) -> Result<En1992Snapshot, String> {
    <En1992Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes `.pack.semio` envelope.
pub fn encode_en1992_pack(snapshot: &En1992Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;

crate::impl_norm_artifact_record!(@binary crate::En1992Snapshot, extension="en1992", envelope_id="norm.en1992", sqlite=crate::snapshot::sqlite::codec);
