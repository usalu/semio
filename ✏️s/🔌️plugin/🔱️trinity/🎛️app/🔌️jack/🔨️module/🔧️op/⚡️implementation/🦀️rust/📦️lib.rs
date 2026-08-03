//! ⚡️ Trinity Jack app — operation type + laws (constitutional: op).
//!
//! 📌️ Jack has no bespoke operation enum: `trinity_ram::TrinityGraphOperation` is shared directly by
//! both the `jack` and `rewrite` apps (it already carries its own `Operation`/`OpText`/`OpBinary`
//! impls), so `Operation` here is a re-export, not a wrapper.

use protocol::Operation as OperationTrait;
use serde::{Deserialize, Serialize};
use trinity_jack_engine::JackConfig;

pub use trinity_ram::TrinityGraphOperation as Operation;

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: jack's `JackConfig` operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `TrinityJackRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — same "whole-config-snapshot inverse" shape as `shooting_op::ShootingConfigOperation`
/// (see its doc comment for the full rationale).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum JackConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: JackConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: trinity_ram::Camera,
    },
    #[dsl(key = "active-fixture")]
    SetActiveFixture { value: String },
    #[dsl(key = "query")]
    SetQuery { value: String },
    #[dsl(key = "result")]
    SetResult { value: String },
    #[dsl(key = "editor-input")]
    SetEditorEngagementInput { value: String },
    #[dsl(key = "graph-input")]
    SetGraphEngagementInput { value: String },
    #[dsl(key = "results-input")]
    SetResultsEngagementInput { value: String },
    #[dsl(key = "reorganize-epoch")]
    SetReorganizeEpoch { value: u64 },
    #[dsl(key = "editor-selection")]
    SetEditorSelection {
        #[dsl(block)]
        selection: Option<trinity_jack_engine::JackEditorSelection>,
    },
    #[dsl(key = "lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "revision")]
    SetRevision { value: u64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl OperationTrait<JackConfig> for JackConfigOperation {
    type Diff = JackConfig;

    fn diff(&self, base: &JackConfig) -> JackConfig {
        let mut next = base.clone();
        match self {
            JackConfigOperation::Snapshot { config } => return config.clone(),
            JackConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            JackConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            JackConfigOperation::SetActiveFixture { value } => next.active_fixture_id = value.clone(),
            JackConfigOperation::SetQuery { value } => next.jack_query = value.clone(),
            JackConfigOperation::SetResult { value } => next.jack_result_json = value.clone(),
            JackConfigOperation::SetEditorEngagementInput { value } => next.editor_engagement_input = value.clone(),
            JackConfigOperation::SetGraphEngagementInput { value } => next.graph_engagement_input = value.clone(),
            JackConfigOperation::SetResultsEngagementInput { value } => next.results_engagement_input = value.clone(),
            JackConfigOperation::SetReorganizeEpoch { value } => next.reorganize_epoch = *value,
            JackConfigOperation::SetEditorSelection { selection } => next.editor_selection = selection.clone(),
            JackConfigOperation::SetLodMode { window_id, value } => {
                next.lod_mode_by_window.insert(window_id.clone(), value.clone());
            }
            JackConfigOperation::SetRevision { value } => next.revision = *value,
            JackConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &JackConfig) -> Vec<Self> {
        vec![JackConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::Rename { id: "node-1".into(), name: "Renamed".into() });
    }

    #[test]
    fn jack_config_operation_backwards_restores_prior_snapshot() {
        let base = JackConfig::default();
        let operation = JackConfigOperation::SetSelection { node_ids: vec!["n1".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_node_ids, vec!["n1".to_string()]);
        let backwards = operation.backwards(&base);
        let restored = backwards[0].diff(&next);
        assert_eq!(restored, base);
    }

    #[test]
    fn jack_config_operation_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&JackConfigOperation::SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() });
        store::test_support::assert_op_line_round_trip(&JackConfigOperation::SetSelection { node_ids: vec!["a".into(), "b".into()] });
    }
}
//#endregion 🧪️Tests
