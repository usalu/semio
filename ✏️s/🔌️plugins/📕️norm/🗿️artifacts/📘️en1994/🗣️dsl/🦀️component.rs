//! 📜️ EN 1994 design of composite steel and concrete structures — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1994::En1994Snapshot;

/// 🗄️ The composite-bridge-girder example fixture, handcrafted in `en1994`'s DSL (`store::DocumentDsl`):
/// an EN-annex EN 1994-2 composite bridge girder with a re-entrant deck under an R90 fire rating and a
/// shear-connector fatigue detail, distinct from `En1994Snapshot::default()`'s DE-annex/R60/trapezoidal-deck/
/// stud-welded values so the grammar's non-default branches (annex, fire rating, deck type, fatigue
/// detail) are exercised too.
pub const EN1994_COMPOSITE_BRIDGE_GIRDER_EXAMPLE_TEXT: &str = include_str!("../📚️examples/📕️composite-bridge-girder/🖼️assets/🗣️composite-bridge-girder.dsl.semio");

/// 📖️ Parses `.en1994` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1994Snapshot, store::TextError> {
    <En1994Snapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1994` DSL text.
pub fn print_dsl(document: &En1994Snapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::AnnexChoice;

    #[test]
    fn document_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&En1994Snapshot::default());
    }

    #[test]
    fn dsl_round_trip_agrees_with_print_parse_wrappers() {
        let document = En1994Snapshot::default();
        let printed = print_dsl(&document);
        assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
    }

    #[test]
    fn composite_bridge_girder_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1994_COMPOSITE_BRIDGE_GIRDER_EXAMPLE_TEXT).expect("parse composite bridge girder example");
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.fire_rating, "r90");
        assert_eq!(document.deck_type, "re-entrant");
        assert_eq!(document.fatigue_detail, "shear_connector");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
