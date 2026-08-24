//! 📜️ Playground artifact — textual document grammar surface + laws.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;

/// 📄️ The `demo` example checkpoint.
pub const PLAYGROUND_DEMO_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses playground DSL text into a `PlaygroundSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PlaygroundSnapshot, store::TextError> {
    <PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PlaygroundSnapshot` back to DSL text.
pub fn print_dsl(snapshot: &PlaygroundSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground_snapshot_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&crate::artifacts::playground::standards::v1::subsets::any::schema::empty_playground_snapshot());
    }

    #[test]
    fn default_example_dsl_round_trips() {
        let document = parse_dsl(PLAYGROUND_DEMO_DEFAULT_EXAMPLE_TEXT).expect("parse default playground example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
