//! 📦️ EN 1990 basis of structural design — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::en1990::En1990Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub async fn encode(document: &En1990Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<En1990Snapshot, PackError> {
    <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn document_pack_round_trips_and_agrees_with_dsl() {
        let document = En1990Snapshot::default();
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
