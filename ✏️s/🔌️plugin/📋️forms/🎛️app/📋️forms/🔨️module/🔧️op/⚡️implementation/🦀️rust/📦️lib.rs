//! ⚡️ Forms app — operation enum + laws (constitutional: op). The operation enum and its
//! `Operation`/`OperationDiff` impls live in the shared `playbook` kernel crate; this crate re-exports
//! them under forms' historical names, including the `apply_form_edit_operation` fn that matches on the
//! operation enum (kept out of `engine` to avoid a circular dependency — `op` already depends on
//! `engine`, so `engine` can never depend back on `op`).

pub use playbook::{apply_playbook_edit_operation as apply_form_edit_operation, PlaybookDiff as FormDiff, PlaybookOperation as FormOperation};

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use forms::FormStep;
    use forms_engine::empty_forms_projection;

    #[test]
    fn update_form_op_sets_title() {
        let spec = empty_forms_projection();
        let operation = FormOperation::UpdatePlaybook { title: Some("Renamed".into()) };
        let next = apply_form_edit_operation(&spec, &operation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn apply_form_edit_op_roundtrip() {
        let spec = empty_forms_projection();
        let step = FormStep {
            id: "step-test".into(),
            title: "Review".into(),
            description: None,
            blocks: Vec::new(),
        };
        let next = apply_form_edit_operation(&spec, &FormOperation::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }
}
//#endregion 🧪️Tests
