//! 🔺️ Forms artifact — the operation diff (constitutional: diff).
//!
//! `FormDiff` and its `protocol::MutationDiff<FormSpec>` impl are implemented directly in the shared
//! `playbook` kernel crate (`PlaybookDiff`) alongside the `PlaybookSpec` projection it patches — see
//! `🗿️artifacts/📋️forms/🦀️component.rs` for why. Re-exported here so the artifact's diff slot names an
//! artifact-owned symbol.

//#region 🔖️Types

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


pub use crate::playbook::PlaybookDiff as FormDiff;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::{op::FormMutation, FormSpec, FormStep};
    use protocol::{Mutation, MutationDiff};

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation, and the diff carries only
    /// the touched slot — the `MutationDiff` contract undo/redo rides on.
    #[test]
    fn add_step_diff_applies_onto_the_base_projection() {
        let base = FormSpec { schema: crate::artifacts::forms::FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, steps: Vec::new() };
        let step = FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() };
        let operation = FormMutation::AddStep { step, index: None };
        let diff: FormDiff = operation.diff(&base);
        assert_eq!(diff.apply(&base).steps.len(), 1);
    }
}
//#endregion 🧪️Tests
