//! 📦️ Block 3D app — binary document surface + laws (constitutional: pack).

use block_3d::Block3dDefinition;
use store::PackError;

/// 📦️ Encodes a `Block3dDefinition` to its binary pack form.
pub fn encode(document: &Block3dDefinition) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Block3dDefinition` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Block3dDefinition, PackError> {
    <Block3dDefinition as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_representative_document() {
        let document = Block3dDefinition::default();
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
