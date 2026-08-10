//! 📜️ EN 1990 basis of structural design — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1990::En1990Snapshot;

/// 🏢️ The high-consequence-office example fixture, handcrafted in `en1990`'s DSL
/// (`store::DocumentDsl`): a CC3 (high-consequence) office building basis-of-design check with
/// three variable-action entries under the EN annex and the seismic accidental action disabled —
/// distinct from `En1990Snapshot::default()`'s CC2/DE-annex/seismic-enabled values so the grammar's
/// non-default branches (consequence class, annex, `q_k` table cardinality) are exercised too.
pub const EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/📕️high-consequence-office/🖼️assets/🗣️high-consequence-office.dsl.semio");

/// 📖️ Parses `.en1990` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1990Snapshot, store::TextError> {
    <En1990Snapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1990` DSL text.
pub fn print_dsl(document: &En1990Snapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::AnnexChoice;

    #[test]
    fn document_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&En1990Snapshot::default());
    }

    #[test]
    fn dsl_round_trip_agrees_with_print_parse_wrappers() {
        let document = En1990Snapshot::default();
        let printed = print_dsl(&document);
        assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
    }

    #[test]
    fn high_consequence_office_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT).expect("parse high consequence office example");
        assert_eq!(document.consequence_class, 3);
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.seismic_a_ed_kn, 0.0);
        assert_eq!(document.q_k.len(), 3);
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
