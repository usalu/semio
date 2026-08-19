//! 📜️ Flow artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::flow::FlowSnapshot;

/// 📄️ The canonical flow snapshot, handcrafted in the `.flow` DSL.
pub const FLOW_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.flow` DSL text into a `FlowSnapshot`.
pub async fn parse_dsl(text: &str) -> Result<FlowSnapshot, store::TextError> {
    <FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FlowSnapshot` back to `.flow` DSL text.
pub async fn print_dsl(snapshot: &FlowSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn example_fixture_dsl_round_trips() {
        let snapshot = parse_dsl(FLOW_EXAMPLE_TEXT).expect("parse default snapshot");
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }

    #[test]
    async fn default_snapshot_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&FlowSnapshot::default());
    }
}
//#endregion 🧪️Tests
