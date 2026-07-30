//! 📦 Puzzle 3d app — binary document surface + laws (constitutional: pack).

use puzzle_3d::Puzzle3dProjection;
use store::PackError;

/// 📦 Encodes a `Puzzle3dProjection` to its binary pack form.
pub fn encode(document: &Puzzle3dProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes a `Puzzle3dProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle3dProjection, PackError> {
    <Puzzle3dProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = puzzle_3d_dsl::parse_dsl(puzzle_3d_dsl::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete-forest example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪Tests
