//! 📜️ EN 1990 basis of structural design — textual document grammar surface + laws (constitutional: dsl).

use en1990::Document;

/// 🏢️ The high-consequence-office example fixture, handcrafted in `en1990`'s DSL
/// (`store::DocumentDsl`): a CC3 (high-consequence) office building basis-of-design check with
/// three variable-action entries under the EN annex and the seismic accidental action disabled —
/// distinct from `Document::default()`'s CC2/DE-annex/seismic-enabled values so the grammar's
/// non-default branches (consequence class, annex, `q_k` table cardinality) are exercised too.
pub const EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugins/📕️norm/📚️examples/📘️en1990/📕️high-consequence-office.en1990");

/// 📖️ Parses `.en1990` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1990` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use norm_core::AnnexChoice;

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
    fn high_consequence_office_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT).expect("parse high consequence office example");
        assert_eq!(document.consequence_class, 3);
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.seismic_a_ed_kn, 0.0);
        assert_eq!(document.q_k.len(), 3);
        store::test_support::assert_dsl_round_trip(&document);
    }
}
