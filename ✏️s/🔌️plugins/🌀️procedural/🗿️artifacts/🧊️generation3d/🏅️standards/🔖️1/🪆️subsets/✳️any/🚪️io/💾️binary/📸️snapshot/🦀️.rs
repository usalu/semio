//! 📦️ Generation3d artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[cfg(test)]
use store::PackError;

/// 🔐️ Encodes a `Generation3dSnapshot` to its binary pack form.
pub fn encode(document: &Generation3dSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 🔬️ Batch decode exists only for constitutional equivalence tests; mounted UI code
/// has no production symbol that can reach the whole-document decoder.
#[cfg(test)]
pub fn decode(bytes: &[u8]) -> Result<Generation3dSnapshot, PackError> {
    <Generation3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

use store::mounted_pack_rt as mounted;
/// 🌀️ The producer counterpart of [`Generation3dMountedPackSession`]: this artifact's own `P3D3`
/// discriminator followed by the canonical `.spk` stream — the SAME bytes [`encode`] produces,
/// with the outer `\x89SEM` semio container unwrapped, since the mounted cursors decode a pack
/// file and not a semio envelope.
///
/// The mounted ingress route refuses any stream that does not lead with `P3D3` (`admit_byte`'s
/// prefix gate), and a whole-document `ArtifactPack` encode carries the repo-wide container magic
/// and no artifact discriminator — exactly like every committed `🎒️.pack.semio` asset — so it is
/// not admissible there and never was. generation2d builds the same shape inside its own
/// `ArtifactPack` impl (`P2D2` + `pack_rt::encode_document`), which is the cross-artifact
/// inconsistency this keeps out of the pack contract (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn encode_mounted(document: &Generation3dSnapshot) -> Vec<u8> {
    let container = encode(document);
    let (_envelope, canonical) = store::semio_format::unwrap_binary(&container).expect("this artifact's own pack encode is a well-formed semio binary container");
    let mut bytes = Vec::with_capacity(GENERATION3D_MOUNTED_PREFIX.len() + canonical.len());
    bytes.extend_from_slice(&GENERATION3D_MOUNTED_PREFIX);
    bytes.extend_from_slice(&canonical);
    bytes
}

#[path = "🧱️mounted-snapshot-owner-types-and-stepping/🦀️.rs"]
mod mounted_snapshot_owner_types_and_stepping;
pub use mounted_snapshot_owner_types_and_stepping::*;

#[path = "🪢️mounted-snapshot-owner-assignment/🦀️.rs"]
mod mounted_snapshot_owner_assignment;

#[path = "🧳️mounted-pack-session/🦀️.rs"]
mod mounted_pack_session;
pub use mounted_pack_session::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-mounted-laws/🦀️.rs"]
mod retained_mounted_laws;
