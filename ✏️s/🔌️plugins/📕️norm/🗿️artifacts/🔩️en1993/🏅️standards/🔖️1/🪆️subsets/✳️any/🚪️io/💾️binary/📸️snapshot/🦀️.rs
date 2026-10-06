//! 📦️ EN 1993 design of steel structures — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::En1993Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &En1993Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1993Snapshot, PackError> {
    <En1993Snapshot as store::ArtifactPack>::decode_pack(bytes)
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
    BridgeFatigue, ColdFormedMember, CraneRunway, DesignAction, FatigueBand, FatigueDetail, FireExposure, ForceAction, JointForceAction, LoadCase,
    MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial, SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg,
};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
/// 📦️ Decodes a [`En1993Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_pack(bytes: &[u8]) -> Result<En1993Snapshot, String> {
    <En1993Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes a [`En1993Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_pack(snapshot: &En1993Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
