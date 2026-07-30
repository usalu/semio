//! 📦 Forms app — binary document surface + laws (constitutional: pack).

use forms::FormSpec;
use store::PackError;

/// 📦 Encodes a `FormSpec` to its binary pack form.
pub fn encode(document: &FormSpec) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes a `FormSpec` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FormSpec, PackError> {
    <FormSpec as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_component_fixture_pack_agrees_with_dsl() {
        let spec = forms_dsl::parse_dsl(forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋building-component.forms parses");
        store::test_support::assert_dsl_pack_equivalence(&spec);
        let bytes = encode(&spec);
        assert_eq!(decode(&bytes).expect("decode"), spec);
    }
}
//#endregion 🧪Tests
