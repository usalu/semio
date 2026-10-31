//! 📦️ Generation2d artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
#[cfg(test)]
use store::PackError;

/// 🔐️ Encodes a `Generation2dSnapshot` to its binary pack form.
pub fn encode(document: &Generation2dSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 🔬️ Batch decode exists only for constitutional equivalence tests; mounted UI code
/// has no production symbol that can reach the whole-document decoder.
#[cfg(test)]
pub fn decode(bytes: &[u8]) -> Result<Generation2dSnapshot, PackError> {
    <Generation2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession

#[path = "🏷️mounted-owner-state/🦀️.rs"]
mod mounted_owner_state;
pub use mounted_owner_state::Generation2dMountedTypedSnapshotOwner;
#[cfg(test)]
use mounted_owner_state::GENERATION2D_MOUNTED_PREFIX;

#[path = "🔤️owner-string-targets/🦀️.rs"]
mod owner_string_targets;

#[path = "🧮️owner-values-and-dictionaries/🦀️.rs"]
mod owner_values_and_dictionaries;

#[path = "🗃️owner-containers-and-widgets/🦀️.rs"]
mod owner_containers_and_widgets;

#[path = "🔌️owner-pack-protocol/🦀️.rs"]
mod owner_pack_protocol;
pub use owner_pack_protocol::{generation2d_mounted_pack_session, Generation2dMountedPackSession};

//#endregion 🔖️MountedCanonicalPackSession

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-mounted-laws/🦀️.rs"]
mod retained_mounted_laws;
