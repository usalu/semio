//! ⚡️ Trinity Rewrite app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use rewrite::{RewriteRuleState, TrinityRewriteError, REWRITE_RULE_SCHEMA};
use serde::{Deserialize, Serialize};
use store::{create_document_envelope, DocumentCommand, DocumentEnvelope, DocumentStore};

//#region 🔖️Types
/// 🔁️ Whole-state snapshot diff: the rule document is one small unit, so history stores full pre/post states rather than field-level patches.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewriteRuleDiff {
    pub next: Option<RewriteRuleState>,
}

impl OperationDiff<RewriteRuleState> for RewriteRuleDiff {
    fn apply(&self, projection: &RewriteRuleState) -> RewriteRuleState {
        self.next.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.next.is_some() {
            self.next = other.next;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RewriteRuleOperation {
    SetState { state: RewriteRuleState },
}

impl Operation<RewriteRuleState> for RewriteRuleOperation {
    type Diff = RewriteRuleDiff;

    fn diff(&self, _projection: &RewriteRuleState) -> Self::Diff {
        match self {
            RewriteRuleOperation::SetState { state } => RewriteRuleDiff { next: Some(state.clone()) },
        }
    }

    fn backwards(&self, projection: &RewriteRuleState) -> Vec<Self> {
        vec![RewriteRuleOperation::SetState { state: projection.clone() }]
    }
}

pub type RewriteRuleEnvelope = DocumentEnvelope<RewriteRuleState, RewriteRuleOperation>;
pub type RewriteRuleStore = DocumentStore<RewriteRuleState, RewriteRuleOperation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewriteRuleState) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}

pub fn dispatch_rewrite_rule_state(store: &mut RewriteRuleStore, state: RewriteRuleState) -> Result<(), TrinityRewriteError> {
    let current = store.projection()?;
    if current == state {
        return Ok(());
    }
    store.dispatch(DocumentCommand::Apply { operations: vec![RewriteRuleOperation::SetState { state }], description: None }).map_err(TrinityRewriteError::from)
}
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: rewrite's `RewriteConfig` operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `RewritePlayRuntime` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns — same "whole-config-snapshot inverse" shape as
/// `shooting_op::ShootingConfigOperation`/`trinity_jack_op::JackConfigOperation`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum RewriteConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: rewrite_engine::RewriteConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "before-pane-camera")]
    SetBeforePaneCamera {
        #[dsl(block)]
        camera: trinity_ram::Camera,
    },
    #[dsl(key = "reorganize-epoch")]
    SetReorganizeEpoch { value: u64 },
    #[dsl(key = "active-hover-var")]
    SetActiveHoverVar { value: String },
    #[dsl(key = "hover-epoch")]
    SetHoverEpoch { value: u64 },
    #[dsl(key = "active-select-var")]
    SetActiveSelectVar { value: String },
    #[dsl(key = "select-epoch")]
    SetSelectEpoch { value: u64 },
    #[dsl(key = "lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<rewrite_engine::RewriteConfig> for RewriteConfigOperation {
    type Diff = rewrite_engine::RewriteConfig;

    fn diff(&self, base: &rewrite_engine::RewriteConfig) -> rewrite_engine::RewriteConfig {
        let mut next = base.clone();
        match self {
            RewriteConfigOperation::Snapshot { config } => return config.clone(),
            RewriteConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            RewriteConfigOperation::SetBeforePaneCamera { camera } => next.before_pane_camera = camera.clone(),
            RewriteConfigOperation::SetReorganizeEpoch { value } => next.reorganize_epoch = *value,
            RewriteConfigOperation::SetActiveHoverVar { value } => next.active_hover_var = value.clone(),
            RewriteConfigOperation::SetHoverEpoch { value } => next.hover_epoch = *value,
            RewriteConfigOperation::SetActiveSelectVar { value } => next.active_select_var = value.clone(),
            RewriteConfigOperation::SetSelectEpoch { value } => next.select_epoch = *value,
            RewriteConfigOperation::SetLodMode { window_id, value } => {
                next.lod_mode_by_window.insert(window_id.clone(), value.clone());
            }
            RewriteConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &rewrite_engine::RewriteConfig) -> Vec<Self> {
        vec![RewriteConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OpText;
    use rewrite::{LayoutPoint, RewriteRuleState};
    use std::collections::BTreeMap;
    use store::test_support::{assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};
    use trinity_ram::PropertyValue;

    fn sample_rule_state() -> RewriteRuleState {
        let mut parameter_bindings = BTreeMap::new();
        parameter_bindings.insert("label".to_string(), PropertyValue::String("nakagin-core".into()));
        parameter_bindings.insert("count".to_string(), PropertyValue::Number(3.0));
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint::from((10.5, -20.25)));
        RewriteRuleState {
            before_fixture_json: "{\"schema\":\"trinity.graph\",\"name\":\"x \\\"quoted\\\"\\nline\"}".to_string(),
            lhs_json: r#"{"pattern":{"leftVar":"a","leftKind":"Piece"}}"#.to_string(),
            rhs_json: r#"{"set":[{"var":"a","prop":"label","value":"$label"}]}"#.to_string(),
            parameter_bindings,
            rule_layout,
        }
    }

    #[test]
    fn op_text_round_trip_set_state() {
        assert_op_line_round_trip(&RewriteRuleOperation::SetState { state: sample_rule_state() });
    }

    #[test]
    fn document_text_round_trip_rewrite_rule_store() {
        let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", sample_rule_state()));
        let mut next = sample_rule_state();
        next.lhs_json = "{}".into();
        dispatch_rewrite_rule_state(&mut store, next).unwrap();
        assert_document_text_round_trip(&store);
        assert_document_pack_round_trip(&store);
    }

    #[test]
    fn op_text_parse_op_errors_on_unknown_keyword() {
        let err = RewriteRuleOperation::parse_op("bogus xyz").unwrap_err();
        assert!(err.message.contains("unknown operation line"));
    }

    #[test]
    fn rewrite_config_operation_backwards_restores_prior_snapshot() {
        let base = rewrite_engine::RewriteConfig::default();
        let operation = RewriteConfigOperation::SetSelection { node_ids: vec!["n1".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_node_ids, vec!["n1".to_string()]);
        let backwards = operation.backwards(&base);
        let restored = backwards[0].diff(&next);
        assert_eq!(restored, base);
    }

    #[test]
    fn rewrite_config_operation_text_round_trips() {
        assert_op_line_round_trip(&RewriteConfigOperation::SetLodMode { window_id: "trinity-rewrite-before".into(), value: "compact".into() });
        assert_op_line_round_trip(&RewriteConfigOperation::SetSelection { node_ids: vec!["a".into(), "b".into()] });
    }
}
//#endregion 🧪️Tests
