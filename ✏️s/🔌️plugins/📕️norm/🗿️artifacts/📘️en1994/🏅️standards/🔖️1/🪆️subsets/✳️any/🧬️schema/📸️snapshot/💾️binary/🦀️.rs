//! 📦️ EN 1994 design of composite steel and concrete structures — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::en1994::En1994Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &En1994Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<En1994Snapshot, PackError> {
    <En1994Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn document_pack_round_trips_and_agrees_with_dsl() {
        let document = En1994Snapshot::default();
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
