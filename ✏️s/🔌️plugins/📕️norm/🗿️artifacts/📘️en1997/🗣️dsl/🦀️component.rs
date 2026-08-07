//! 📜️ EN 1997 app — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1997::Document;

/// 📄️ The `default` example document, handcrafted in the `.en1997` DSL — a shallow footing +
/// pile worked example (bearing, sliding, settlement, pile axial, ground investigation depth)
/// under the DE national annex, DA1-C1 design approach.
pub const EN1997_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.en1997` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1997` DSL text.
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
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(EN1997_DEFAULT_EXAMPLE_TEXT).expect("parse default .en1997 example");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
