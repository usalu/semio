//! 📦 Imperative app — binary document surface + laws (constitutional: pack).

use imperative::ImperativeDocument;
use store::PackError;

/// 📦 Encodes an `ImperativeDocument` to its binary pack form.
pub fn encode(document: &ImperativeDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes an `ImperativeDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<ImperativeDocument, PackError> {
    <ImperativeDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = imperative_dsl::parse_dsl(imperative_dsl::IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜default.imperative");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_representative_document_with_nested_control_body() {
        use imperative::{Dictionary, Path, Step};
        use std::collections::BTreeMap;

        let mut document = imperative_dsl::parse_dsl(imperative_dsl::IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜default.imperative");
        let inner = Step { id: "step-inner".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let mut owner = Step { id: "step-if".into(), kind: "control.if".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
        document.path.steps = vec![owner];

        store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪Tests
