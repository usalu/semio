//! 📦 Lowpoly app — binary document surface + laws (constitutional: pack).

use lowpoly::LowpolyProjection;
use store::PackError;

/// 📦 Encodes a `LowpolyProjection` to its binary pack form.
pub fn encode(projection: &LowpolyProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖 Decodes a `LowpolyProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<LowpolyProjection, PackError> {
    <LowpolyProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let projection = lowpoly_dsl::parse_dsl(lowpoly_dsl::LOWPOLY_EXAMPLE_TEXT).expect("default projection DSL parses");
        store::test_support::assert_dsl_pack_equivalence(&projection);
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }

    #[test]
    fn pack_round_trips_a_projection_with_a_painted_layer() {
        let mut projection = lowpoly_dsl::parse_dsl(lowpoly_dsl::LOWPOLY_EXAMPLE_TEXT).expect("default projection DSL parses");
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }
}
//#endregion 🧪Tests
