//! 📦️ Procedural3d artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::procedural3d::Procedural3dDocument;
use store::PackError;

/// 📦️ Encodes a `Procedural3dDocument` to its binary pack form.
pub fn encode(document: &Procedural3dDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Procedural3dDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Procedural3dDocument, PackError> {
    <Procedural3dDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::dsl;
    use store::test_support;

    #[test]
    fn dsl_pack_equivalence_empty_projection() {
        test_support::assert_dsl_pack_equivalence(&Procedural3dDocument::default());
    }

    #[test]
    fn pack_round_trips_the_hex_column_example() {
        let projection = dsl::parse_dsl(dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse fixture");
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }
}
//#endregion 🧪️Tests
