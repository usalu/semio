//! 📜 DIN 4108 app — textual document grammar surface + laws (constitutional: dsl).
//!
//! 📌 No `include_str!` fixture exists for this app (the original monolith exercised the DSL
//! grammar purely against `Document::default()`), so these wrappers are the whole surface.

use din4108::Document;

/// 📖 Parses `.din4108` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.din4108` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&Document::default());
    }
}
//#endregion 🧪Tests
