//! 📦️ Puzzle 5d app — binary document surface + laws (constitutional: pack).

use puzzle_5d::Puzzle5dProjection;
use store::PackError;

/// 📦️ Encodes a `Puzzle5dProjection` to its binary pack form.
pub fn encode(document: &Puzzle5dProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle5dProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle5dProjection, PackError> {
    <Puzzle5dProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_representative_document() {
        let document = Puzzle5dProjection::default();
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
