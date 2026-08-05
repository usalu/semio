//! 📦️ DIN 4108 app — binary document surface + laws (constitutional: pack).

use crate::artifacts::din4108::Document;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Document) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Document, PackError> {
    <Document as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_pack_equivalence() {
        store::test_support::assert_dsl_pack_equivalence(&Document::default());
    }

    #[test]
    fn pack_round_trips() {
        let document = Document::default();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
