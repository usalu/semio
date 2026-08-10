//! 📜️ VCS artifact — textual document grammar surface + laws (was: constitutional `dsl`).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::vcs::VcsSnapshot;

/// 📄️ The `demo` example checkpoint, handcrafted in the `.vcsdemo` DSL — a mid-review structural
/// change with a non-zero counter, freeform notes, an in-progress status, and a few tags.
pub const VCS_DEMO_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.vcsdemo` DSL text into a `VcsSnapshot`.
pub fn parse_dsl(text: &str) -> Result<VcsSnapshot, store::TextError> {
    <VcsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `VcsSnapshot` back to `.vcsdemo` DSL text.
pub fn print_dsl(projection: &VcsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_projection_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&crate::artifacts::vcs::engine::empty_vcs_snapshot());
    }

    #[test]
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(VCS_DEMO_DEFAULT_EXAMPLE_TEXT).expect("parse default .vcsdemo example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
