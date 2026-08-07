//! 🎒️ Animate present artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::present::PresentDeck;
use store::PackError;

/// 📦️ Encodes a `PresentDeck` to its binary pack form.
pub fn encode(deck: &PresentDeck) -> Vec<u8> {
    store::DocumentPack::encode_pack(deck)
}

/// 📖️ Decodes a `PresentDeck` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<PresentDeck, PackError> {
    <PresentDeck as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::default_present_deck;
    use crate::artifacts::present::engine::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
    use store::test_support;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let deck = default_present_deck();
        test_support::assert_dsl_pack_equivalence(&deck);
        let bytes = encode(&deck);
        assert_eq!(decode(&bytes).expect("decode"), deck);
    }

    #[test]
    fn pack_round_trips_deck_with_tiles() {
        let deck = default_present_deck();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let deck = PresentDeck { tiles, ..deck };
        test_support::assert_dsl_pack_equivalence(&deck);
        let bytes = encode(&deck);
        assert_eq!(decode(&bytes).expect("decode"), deck);
    }
}
//#endregion 🧪️Tests
