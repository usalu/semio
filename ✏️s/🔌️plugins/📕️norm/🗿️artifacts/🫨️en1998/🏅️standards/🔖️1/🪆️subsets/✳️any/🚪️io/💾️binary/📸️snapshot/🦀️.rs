//! 📦️ EN 1998 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::En1998Snapshot;
use store::PackError;

/// 📦️ Encodes a `En1998Snapshot` to its binary pack form.
pub fn encode(document: &En1998Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `En1998Snapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1998Snapshot, PackError> {
    <En1998Snapshot as store::ArtifactPack>::decode_pack(bytes)
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
use crate::{
    DeGroundCombo, DeSeismicZone, En1998Assessment, En1998Bridge, En1998Building, En1998Foundation, En1998RetainingWall, En1998Site, En1998Silo,
    En1998Storey, En1998VariableAction, En1998System, En1998Tank, En1998Tower, En1998Member,
};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::any::io::text::snapshot::*;
pub fn encode_en1998_pack(snapshot: &En1998Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
pub fn decode_en1998_pack(bytes: &[u8]) -> Result<En1998Snapshot, String> {
    <En1998Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
}
pub use native_snapshot_codec::*;
