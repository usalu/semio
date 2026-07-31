//! 📦️ Trinity Jack app — binary document surface + laws (constitutional: pack).

use trinity_ram::GraphFixture;

/// 📦️ Encodes a `GraphFixture` to its binary pack form.
pub fn encode(document: &GraphFixture) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `GraphFixture` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<GraphFixture, store::PackError> {
    <GraphFixture as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nakagin_example_pack_round_trips_and_agrees_with_dsl() {
        let document = trinity_jack_dsl::parse_dsl(trinity_jack_dsl::NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
