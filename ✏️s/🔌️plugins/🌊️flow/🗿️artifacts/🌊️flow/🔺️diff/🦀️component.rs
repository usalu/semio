//! 🔺️ Flow artifact — the operation diff (constitutional: diff).
//!
//! `FlowDiff` and its `protocol::OperationDiff<FlowFixture>` impl are implemented directly in the flow
//! kernel crate (`flow_core`, `🔖️Operations` region) alongside the `FlowFixture` projection they patch —
//! see `🗿️artifacts/🌊️flow/🦀️component.rs` for why. Re-exported here so the artifact's diff slot names an
//! artifact-owned symbol.

//#region 🔖️Types

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


pub use flow_core::FlowDiff;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::{op::FlowOperation, FlowFixture};
    use protocol::{Operation, OperationDiff};

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation, and the diff carries only
    /// the touched slot — the `OperationDiff` contract undo/redo rides on.
    #[test]
    fn set_layout_diff_applies_onto_the_base_projection() {
        let base = FlowFixture::default();
        let operation = FlowOperation::SetLayout { entries: Vec::new() };
        let diff: FlowDiff = operation.diff(&base);
        assert!(diff.layout.is_some(), "SetLayout must produce a layout diff: {diff:?}");
        assert!(diff.fixture.is_none() && diff.widgets.is_none() && diff.synapses.is_none(), "SetLayout must touch only the layout slot: {diff:?}");
        assert_eq!(diff.apply(&base), base, "an empty layout diff is a no-operation on the projection");
    }
}
//#endregion 🧪️Tests
