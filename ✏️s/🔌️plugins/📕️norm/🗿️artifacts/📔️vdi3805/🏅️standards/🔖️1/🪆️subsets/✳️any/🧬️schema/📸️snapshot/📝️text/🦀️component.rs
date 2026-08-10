//! 📜️ VDI 3805 app — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::vdi3805::Vdi3805Snapshot;

/// 📜️ Bundled reference-catalogue example (`.semio` envelope + DSL body).
pub const REFERENCE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses VDI 3805 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Vdi3805Snapshot, store::TextError> {
    <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.vdi3805` DSL text.
pub fn print_dsl(document: &Vdi3805Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips_the_reference_fixture() {
        store::os_store::test_support::assert_dsl_round_trip(&crate::artifacts::vdi3805::reference_fixture());
    }

    #[test]
    fn bundled_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(REFERENCE_EXAMPLE_TEXT).expect("parse bundled example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
