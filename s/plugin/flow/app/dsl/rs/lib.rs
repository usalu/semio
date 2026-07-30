//! 📜 Flow app — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::DocumentDsl for FlowFixture` is implemented directly in the flow kernel crate (`flow_core`,
//! see `s/plugin/flow/app/rs/lib.rs` for why the entity itself lives there); this crate only adds
//! the thin app-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture constant and
//! its round-trip law.

use flow::FlowFixture;

/// 📄 The canonical flow fixture, handcrafted in the `.flow` DSL — the same file the flow kernel's own
/// tests parse via `include_str!("../../../../../../framework/os/kernel/flow/example/default.flow")`.
pub const FLOW_EXAMPLE_TEXT: &str = include_str!("../../../../../../framework/os/kernel/flow/example/default.flow");

/// 📖 Parses `.flow` DSL text into a `FlowFixture`.
pub fn parse_dsl(text: &str) -> Result<FlowFixture, store::TextError> {
    <FlowFixture as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FlowFixture` back to `.flow` DSL text.
pub fn print_dsl(fixture: &FlowFixture) -> String {
    store::DocumentDsl::print_dsl(fixture)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_fixture_dsl_round_trips() {
        let fixture = parse_dsl(FLOW_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_round_trip(&fixture);
    }
}
//#endregion 🧪Tests
