//! 📜️ EN 1995 app — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::en1995::Document;

/// 🗄️ The glulam-footbridge example fixture, handcrafted in `en1995`'s DSL (`store::DocumentDsl`):
/// an EN-annex EN 1995-2 glulam pedestrian footbridge beam under service class 2 and long-duration
/// traffic loading, distinct from `Document::default()`'s DE-annex/SC1/medium-duration values so the
/// grammar's non-default branches (annex, service class, load duration) are exercised too.
pub const EN1995_GLULAM_FOOTBRIDGE_EXAMPLE_TEXT: &str = include_str!("../../📚️examples/📕️glulam-footbridge/🗣️dsls/📕️glulam-footbridge/🧬️component.norm.en1995.dsl.semio");

/// 📖️ Parses `.en1995` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1995` DSL text.
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
    fn glulam_footbridge_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1995_GLULAM_FOOTBRIDGE_EXAMPLE_TEXT).expect("parse glulam footbridge example");
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.service_class, "sc2");
        assert_eq!(document.load_duration, "long");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
