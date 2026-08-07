//! 📜️ EN 1993 design of steel structures — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1993::Document;

/// 🔩️ The high-strength-connection example fixture, handcrafted in `en1993`'s DSL
/// (`store::DocumentDsl`): an S460 high-strength steel member and bolted/welded connection
/// worked example (4×M24 grade 10.9 bolts, safe-life fatigue assessment, subgrade K2 toughness)
/// under the EN annex — distinct from `Document::default()`'s DE-annex/S355/2-bolt/damage-tolerant
/// values so the grammar's non-default branches (annex, bolt count, fatigue method, HSS section
/// class) are exercised too.
pub const EN1993_HIGH_STRENGTH_CONNECTION_EXAMPLE_TEXT: &str = include_str!("../📚️examples/📕️high-strength-connection/🗣️dsls/📕️high-strength-connection/🧬️component.norm.en1993.dsl.semio");

/// 📖️ Parses `.en1993` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1993` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn high_strength_connection_example_fixture_parses_and_round_trips() {
        use crate::document::AnnexChoice;
        let document = parse_dsl(EN1993_HIGH_STRENGTH_CONNECTION_EXAMPLE_TEXT).expect("parse high strength connection example");
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.bolt_n_bolts, 4);
        assert_eq!(document.fatigue_method, "safe_life");
        assert_eq!(document.hss_section_class, 3);
        store::test_support::assert_dsl_round_trip(&document);
    }
}
