//! 📜 EN 1990 basis of structural design — textual document grammar surface + laws (constitutional: dsl).
//!
//! 📄 No handcrafted `.en1990` DSL fixture exists for this app (unlike e.g. `note`'s `semio.note`) —
//! the original monolith's own DSL law test exercised only `Document::default()`, so that is the
//! representative document here too.

use en1990::Document;

/// 📖 Parses `.en1990` DSL text into a `Document`.
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
}
