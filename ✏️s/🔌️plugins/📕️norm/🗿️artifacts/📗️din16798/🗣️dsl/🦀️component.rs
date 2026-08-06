//! 📜️ DIN EN 16798 app — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::din16798::Document;

/// 📜️ Bundled default example document (`.semio` envelope + DSL body).
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../📚️examples/♻️default/🗣️dsls/♻️default/🧬️component.norm.din16798.dsl.semio");

/// 📖️ Parses DIN EN 16798 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.din16798` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&Document::default());
    }

    #[test]
    fn bundled_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(DEFAULT_EXAMPLE_TEXT).expect("parse bundled example");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
