//! 📜️ VDI 3805 app — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::vdi3805::Document;

/// 📜️ Bundled reference-catalogue example (`.semio` envelope + DSL body).
pub const REFERENCE_EXAMPLE_TEXT: &str = include_str!("../../📚️examples/♻️default/🗣️dsls/♻️default/🧬️component.norm.vdi3805.dsl.semio");

/// 📖️ Parses VDI 3805 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.vdi3805` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips_the_reference_fixture() {
        store::test_support::assert_dsl_round_trip(&crate::artifacts::vdi3805::reference_fixture());
    }

    #[test]
    fn bundled_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(REFERENCE_EXAMPLE_TEXT).expect("parse bundled example");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
