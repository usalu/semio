//! 📜️ DIN EN 16798 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::din16798::Din16798Snapshot;

/// 📜️ Bundled default example document (`.semio` envelope + DSL body).
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses DIN EN 16798 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Din16798Snapshot, store::TextError> {
    <Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.din16798` DSL text.
pub fn print_dsl(document: &Din16798Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn document_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&Din16798Snapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    fn bundled_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(DEFAULT_EXAMPLE_TEXT).expect("parse bundled example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
