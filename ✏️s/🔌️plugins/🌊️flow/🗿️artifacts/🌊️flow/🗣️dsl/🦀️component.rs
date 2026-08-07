//! 📜️ Flow artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::DocumentDsl for FlowFixture` is implemented directly in the flow kernel crate (`flow`;
//! see `🗿️artifacts/🌊️flow/🦀️component.rs` for why the entity itself lives there). This component only
//! adds the thin artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture
//! constant and its round-trip law.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::flow::FlowFixture;

/// 📄️ The canonical flow fixture, handcrafted in the `.flow` DSL — the same file the flow kernel's own
/// tests parse.
pub const FLOW_EXAMPLE_TEXT: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.flow` DSL text into a `FlowFixture`.
pub fn parse_dsl(text: &str) -> Result<FlowFixture, store::TextError> {
    <FlowFixture as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FlowFixture` back to `.flow` DSL text.
pub fn print_dsl(fixture: &FlowFixture) -> String {
    store::DocumentDsl::print_dsl(fixture)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_fixture_dsl_round_trips() {
        let fixture = parse_dsl(FLOW_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_round_trip(&fixture);
    }
}
//#endregion 🧪️Tests
