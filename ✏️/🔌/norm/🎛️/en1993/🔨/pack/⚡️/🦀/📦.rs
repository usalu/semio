//! 📦 EN 1993 design of steel structures — binary document surface + laws (constitutional: pack).

use en1993::Document;
use store::PackError;

/// 📦 Encodes a `Document` to its binary pack form.
pub fn encode(document: &Document) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Document, PackError> {
    <Document as store::DocumentPack>::decode_pack(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_pack_round_trips_and_agrees_with_dsl() {
        let document = Document::default();
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
