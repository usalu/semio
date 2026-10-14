//! 📦️ ISO 16757 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Iso16757Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Iso16757Snapshot, PackError> {
    <Iso16757Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod native_snapshot_codec {
use crate::binary::snapshot::*;
use crate::binary::snapshot::*;
use semio_s_artifact_norm_iso16757_model::*;
use semio_s_artifact_norm_iso16757_model::{part_1, part_2, part_4, part_5, CatalogueValue};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;
use crate::text::snapshot::*;
/// 📦️ Decodes a [`Iso16757Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_iso16757_pack(bytes: &[u8]) -> Result<Iso16757Snapshot, String> {
    <Iso16757Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
/// 📦️ Encodes a [`Iso16757Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_iso16757_pack(snapshot: &Iso16757Snapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
}
pub use crate::binary::snapshot::native_snapshot_codec::*;

semio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary Iso16757Snapshot, extension="iso16757", envelope_id="norm.iso16757", sqlite=crate::sqlite::snapshot::codec);
