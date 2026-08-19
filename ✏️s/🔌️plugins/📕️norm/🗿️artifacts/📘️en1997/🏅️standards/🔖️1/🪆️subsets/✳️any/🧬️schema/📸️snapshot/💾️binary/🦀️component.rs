//! 📦️ EN 1997 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::en1997::En1997Snapshot;
use store::PackError;

/// 📦️ Encodes a `En1997Snapshot` to its binary pack form.
pub async fn encode(document: &En1997Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `En1997Snapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<En1997Snapshot, PackError> {
    <En1997Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn pack_round_trips_and_agrees_with_dsl() {
        let document = En1997Snapshot::default();
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
