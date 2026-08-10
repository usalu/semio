//! 🗣️ Animate present artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::present::PresentSnapshot;

/// 📄️ The handcrafted `.present` DSL-text fixture — a multi-tile deck exercising every field
/// (including the optional `source-aspect`), embedded at compile time as the permanent proof that
/// the checked-in fixture still parses and round trips.
pub const PRESENT_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.present` DSL text into a `PresentSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PresentSnapshot, store::TextError> {
    <PresentSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PresentSnapshot` back to `.present` DSL text.
pub fn print_dsl(deck: &PresentSnapshot) -> String {
    store::DocumentDsl::print_dsl(deck)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::default_present_snapshot;
    use crate::artifacts::present::engine::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
    use store::os_store::test_support;

    #[test]
    fn dsl_round_trip_default_present_snapshot() {
        test_support::assert_dsl_round_trip(&default_present_snapshot());
        test_support::assert_dsl_pack_equivalence(&default_present_snapshot());
    }

    #[test]
    fn dsl_round_trip_present_deck_with_tiles() {
        let deck = default_present_snapshot();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let deck = PresentSnapshot { tiles, ..deck };
        test_support::assert_dsl_round_trip(&deck);
        test_support::assert_dsl_pack_equivalence(&deck);
    }

    #[test]
    fn present_dsl_round_trips_bundled_default_example() {
        let deck = parse_dsl(PRESENT_EXAMPLE_TEXT).expect("🎞️default.present must parse");
        test_support::assert_dsl_round_trip(&deck);
        test_support::assert_dsl_pack_equivalence(&deck);
    }
}
//#endregion 🧪️Tests
