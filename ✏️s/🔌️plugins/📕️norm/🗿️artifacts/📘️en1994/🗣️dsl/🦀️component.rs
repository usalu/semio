//! 📜️ EN 1994 design of composite steel and concrete structures — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::en1994::Document;

/// 🗄️ The composite-bridge-girder example fixture, handcrafted in `en1994`'s DSL (`store::DocumentDsl`):
/// an EN-annex EN 1994-2 composite bridge girder with a re-entrant deck under an R90 fire rating and a
/// shear-connector fatigue detail, distinct from `Document::default()`'s DE-annex/R60/trapezoidal-deck/
/// stud-welded values so the grammar's non-default branches (annex, fire rating, deck type, fatigue
/// detail) are exercised too.
pub const EN1994_COMPOSITE_BRIDGE_GIRDER_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/📘️en1994/📕️composite-bridge-girder.en1994");

/// 📖️ Parses `.en1994` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1994` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::AnnexChoice;

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&Document::default());
    }

    #[test]
    fn dsl_round_trip_agrees_with_print_parse_wrappers() {
        let document = Document::default();
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
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
