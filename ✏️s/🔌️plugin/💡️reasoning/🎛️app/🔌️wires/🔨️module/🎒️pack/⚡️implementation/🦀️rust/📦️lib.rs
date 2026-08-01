//! 📦️ Reasoning wires app — binary document surface + laws (constitutional: pack).

use reasoning_wires::MindmapWiresDocument;
use store::PackError;

/// 📦️ Encodes a `MindmapWiresDocument` to its binary pack form.
pub fn encode(document: &MindmapWiresDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `MindmapWiresDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<MindmapWiresDocument, PackError> {
    <MindmapWiresDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl_metabolism() {
        let document = reasoning_wires_dsl::parse_dsl(reasoning_wires_dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT).expect("parse metabolism example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_empty_document() {
        let document = reasoning_wires::empty_mindmap_wires_document();
        store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪️Tests
