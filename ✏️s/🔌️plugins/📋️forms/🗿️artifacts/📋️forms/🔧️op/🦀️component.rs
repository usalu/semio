//! 🔧️ Forms artifact — the operation enum + laws (constitutional: op).
//!
//! `FormOperation`, its `protocol::Operation<FormSpec>` impl and the private `apply_playbook_edit_operation`
//! fn all live in the shared `playbook` kernel crate alongside the `PlaybookSpec` projection they mutate —
//! see `🗿️artifacts/📋️forms/🦀️component.rs` for why. Re-exported here so every taxonomy node names an
//! artifact-owned symbol instead of reaching into the kernel path.

//#region 🔖️Types

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


pub use playbook::{apply_playbook_edit_operation as apply_form_edit_operation, PlaybookOperation as FormOperation};
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::engine::empty_forms_projection;

    #[test]
    fn update_form_op_sets_title() {
        let spec = empty_forms_projection();
        let operation = FormOperation::UpdatePlaybook { title: Some("Renamed".into()) };
        let next = apply_form_edit_operation(&spec, &operation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn apply_form_edit_op_roundtrip() {
        use crate::artifacts::forms::FormStep;

        let spec = empty_forms_projection();
        let step = FormStep { id: "step-test".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        let next = apply_form_edit_operation(&spec, &FormOperation::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }
}
//#endregion 🧪️Tests
