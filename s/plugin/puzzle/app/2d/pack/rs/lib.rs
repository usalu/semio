//! 📦 Puzzle 2d app — binary document surface + laws (constitutional: pack).

use puzzle_2d::Puzzle2dProjection;
use store::PackError;

/// 📦 Encodes a `Puzzle2dProjection` to its binary pack form.
pub fn encode(document: &Puzzle2dProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes a `Puzzle2dProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle2dProjection, PackError> {
    <Puzzle2dProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = puzzle_2d_dsl::parse_dsl(puzzle_2d_dsl::PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete-forest example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪Tests
