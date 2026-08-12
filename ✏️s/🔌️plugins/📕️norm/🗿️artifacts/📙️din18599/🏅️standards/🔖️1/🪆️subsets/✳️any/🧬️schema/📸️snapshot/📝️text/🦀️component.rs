//! 📜️ DIN V 18599 app — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::din18599::Din18599Snapshot;

/// 📜️ Bundled default example document (`.semio` envelope + DSL body).
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses DIN V 18599 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Din18599Snapshot, store::TextError> {
    <Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.din18599` DSL text.
pub fn print_dsl(document: &Din18599Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&Din18599Snapshot::default());
    }

    #[test]
    fn bundled_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(DEFAULT_EXAMPLE_TEXT).expect("parse bundled example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
