//! 📜️ VDI 3805 app — textual document grammar surface + laws (constitutional: dsl).
//!
//! 📌️ No `include_str!` fixture exists for this app (the original monolith exercised the DSL
//! grammar purely against `crate::artifacts::vdi3805::reference_fixture()` — the same curated manufacturer-catalogue
//! builder that backs `Document::default()` and every other module's tests/fixtures in this app),
//! so these wrappers are the whole surface.

use crate::artifacts::vdi3805::Document;

/// 📖️ Parses `.vdi3805` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.vdi3805` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    #[test]
    fn document_dsl_round_trips_the_reference_fixture() {
        store::test_support::assert_dsl_round_trip(&crate::artifacts::vdi3805::reference_fixture());
    }
}
