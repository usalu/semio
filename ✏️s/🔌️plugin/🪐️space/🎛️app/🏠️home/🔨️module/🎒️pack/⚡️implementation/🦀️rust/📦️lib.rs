//! 📦️ S Home launcher app — binary document surface + laws (constitutional: pack).

use home::SHomeDocument;
use store::PackError;

/// 📦️ Encodes an `SHomeDocument` to its binary pack form.
pub fn encode(document: &SHomeDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes an `SHomeDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<SHomeDocument, PackError> {
    <SHomeDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_pack_round_trips_default_and_populated_documents() {
        store::test_support::assert_dsl_pack_equivalence(&SHomeDocument { schema: "s.home".into(), catalog_generation: 0 });
        store::test_support::assert_dsl_pack_equivalence(&SHomeDocument { schema: "s.home".into(), catalog_generation: 42 });
    }

    #[test]
    fn pack_round_trips_populated_document() {
        let document = SHomeDocument { schema: "s.home".into(), catalog_generation: 42 };
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
