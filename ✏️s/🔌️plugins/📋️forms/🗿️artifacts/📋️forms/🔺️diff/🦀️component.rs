//! 🔺️ Forms artifact — the operation diff (constitutional: diff).
//!
//! `FormDiff` and its `protocol::OperationDiff<FormSpec>` impl are implemented directly in the shared
//! `playbook` kernel crate (`PlaybookDiff`) alongside the `PlaybookSpec` projection it patches — see
//! `🗿️artifacts/📋️forms/🦀️component.rs` for why. Re-exported here so the artifact's diff slot names an
//! artifact-owned symbol.

//#region 🔖️Types
pub use playbook::PlaybookDiff as FormDiff;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::{op::FormOperation, FormSpec, FormStep};
    use protocol::{Operation, OperationDiff};

    /// ⚖️ LAW: `op.diff(base)` applied to `base` equals applying the operation, and the diff carries only
    /// the touched slot — the `OperationDiff` contract undo/redo rides on.
    #[test]
    fn add_step_diff_applies_onto_the_base_projection() {
        let base = FormSpec { schema: crate::artifacts::forms::FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, steps: Vec::new() };
        let step = FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() };
        let operation = FormOperation::AddStep { step, index: None };
        let diff: FormDiff = operation.diff(&base);
        assert_eq!(diff.apply(&base).steps.len(), 1);
    }
}
//#endregion 🧪️Tests
