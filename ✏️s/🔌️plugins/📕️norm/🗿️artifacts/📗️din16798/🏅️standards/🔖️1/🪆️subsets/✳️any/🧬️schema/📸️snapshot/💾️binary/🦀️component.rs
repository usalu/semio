//! 📦️ DIN EN 16798 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::din16798::Din16798Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub async fn encode(document: &Din16798Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<Din16798Snapshot, PackError> {
    <Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn document_dsl_pack_equivalence() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&Din16798Snapshot::default());
    }

    #[test]
    async fn pack_round_trips() {
        let document = Din16798Snapshot::default();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
