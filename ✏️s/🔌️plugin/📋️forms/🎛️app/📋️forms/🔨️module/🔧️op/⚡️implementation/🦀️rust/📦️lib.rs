//! ⚡️ Forms app — operation enum + laws (constitutional: op). The operation enum and its
//! `Operation`/`OperationDiff` impls live in the shared `playbook` kernel crate; this crate re-exports
//! them under forms' historical names, including the `apply_form_edit_operation` fn that matches on the
//! operation enum (kept out of `engine` to avoid a circular dependency — `op` already depends on
//! `engine`, so `engine` can never depend back on `op`).

use protocol::Operation;
use serde::{Deserialize, Serialize};

pub use playbook::{apply_playbook_edit_operation as apply_form_edit_operation, PlaybookDiff as FormDiff, PlaybookOperation as FormOperation};

//#region 🔖️ConfigOperations
/// @emoji 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS Config recipe: `forms_engine::FormsConfig`'s operation
/// enum — mirrors `shooting_op::ShootingConfigOperation`'s shape exactly: one variant per settled
/// interaction (was a `FormsPlayRuntime` field write pre-B1), plus a generic `Snapshot` every variant's
/// `backwards()` returns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FormsConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: forms_engine::FormsConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "step-index")]
    SetStepIndex { index: u32 },
    #[dsl(key = "try-values")]
    SetTryValues { json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

impl Operation<forms_engine::FormsConfig> for FormsConfigOperation {
    type Diff = forms_engine::FormsConfig;

    fn diff(&self, base: &forms_engine::FormsConfig) -> forms_engine::FormsConfig {
        let mut next = base.clone();
        match self {
            FormsConfigOperation::Snapshot { config } => return config.clone(),
            FormsConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            FormsConfigOperation::SetStepIndex { index } => next.current_step_index = *index,
            FormsConfigOperation::SetTryValues { json } => next.try_values_json = json.clone(),
            FormsConfigOperation::SetLocale { value } => next.locale = value.clone(),
            FormsConfigOperation::SetContributions { json } => next.contributions_json = json.clone(),
        }
        next
    }

    fn backwards(&self, base: &forms_engine::FormsConfig) -> Vec<Self> {
        vec![FormsConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

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

    //#region 🧪️ConfigOperations
    fn config_round_trip(base: &forms_engine::FormsConfig, operation: &FormsConfigOperation) -> forms_engine::FormsConfig {
        let forward = operation.diff(base);
        let backwards = operation.backwards(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_operations_apply_and_restore_every_field() {
        let base = forms_engine::FormsConfig::default();
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetSelection { ids: vec!["q1".into()] }).selected_ids, vec!["q1".to_string()]);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetStepIndex { index: 2 }).current_step_index, 2);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetTryValues { json: r#"{"a":1}"#.into() }).try_values_json, r#"{"a":1}"#);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetContributions { json: "[]".into() }).contributions_json, "[]");
    }

    #[test]
    fn config_snapshot_op_text_round_trips() {
        let config = forms_engine::FormsConfig {
            selected_ids: vec!["q1".into(), "q2".into()],
            current_step_index: 1,
            try_values_json: r#"{"name":"Ada"}"#.into(),
            locale: "de-DE".into(),
            contributions_json: "[]".into(),
        };
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetSelection { ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetStepIndex { index: 3 });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🧪️ConfigOperations
}
//#endregion 🧪️Tests
