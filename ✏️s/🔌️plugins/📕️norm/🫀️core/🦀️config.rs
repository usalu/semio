//! 🧮️ Norm plugin — the ONE view-state config artifact every one of the fifteen norm apps uses.
//!
//! 📌️ Deliberately NOT a per-app `🎛️apps/<app>/🦀️config.rs`: all fifteen compliance apps have the
//! identical config shape (one field — which `CheckReport::checks` row the inspection panel points at),
//! so unlike `shooting`'s per-app `ShootingConfig` this is ONE type reused by every app rather than
//! fifteen byte-identical copies. It lives in `🫀️core` (the cross-artifact/cross-app kernel) because
//! that is the shallowest taxonomy node common to every consumer — the same "put shared declarations at
//! the shallowest common ancestor" rule the migration template states for shared window options.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ The shared `DocumentApp::Config` for every norm family app — one field: which
/// `CheckReport::checks` row the inspection panel currently renders.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "normcfg")]
#[dsl(layout = "lines")]
pub struct NormConfig {
    /// 👁️ Which `CheckReport::checks` row the inspection panel renders — `None` (the default) means
    /// "the first check".
    pub selected_check_index: Option<u32>,
}

impl store::ConfigRecord for NormConfig {}

/// 🧮️ Whole-record diff for `NormConfigOperation` — `apply` ignores `base` entirely, since
/// `NormConfigOperation::Snapshot` already carries the full post-op config.
impl protocol::OperationDiff<NormConfig> for NormConfig {
    fn apply(&self, _base: &NormConfig) -> NormConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `NormConfig`'s operation enum — `Snapshot` is the generic whole-config inverse every other
/// variant's `backwards()` returns (the simplest correct inverse for a config this small);
/// `SetSelectedCheckIndex` is the one real per-field edit every norm family app dispatches.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum NormConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: NormConfig,
    },
    #[dsl(key = "selected-check")]
    SetSelectedCheckIndex { index: Option<u32> },
}

impl Operation<NormConfig> for NormConfigOperation {
    type Diff = NormConfig;

    fn diff(&self, base: &NormConfig) -> NormConfig {
        match self {
            NormConfigOperation::Snapshot { config } => config.clone(),
            NormConfigOperation::SetSelectedCheckIndex { index } => NormConfig { selected_check_index: *index, ..base.clone() },
        }
    }

    fn backwards(&self, base: &NormConfig) -> Vec<Self> {
        vec![NormConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_config_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&NormConfig::default());
        store::test_support::assert_dsl_round_trip(&NormConfig { selected_check_index: Some(3) });
    }

    #[test]
    fn norm_config_dsl_pack_equivalence() {
        store::test_support::assert_dsl_pack_equivalence(&NormConfig::default());
        store::test_support::assert_dsl_pack_equivalence(&NormConfig { selected_check_index: Some(7) });
    }

    #[test]
    fn norm_config_operation_snapshot_is_a_real_inverse() {
        let base = NormConfig { selected_check_index: Some(1) };
        let op = NormConfigOperation::SetSelectedCheckIndex { index: Some(5) };
        let next = op.diff(&base);
        assert_eq!(next.selected_check_index, Some(5));
        let backwards = op.backwards(&base);
        assert_eq!(backwards, vec![NormConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&next);
        assert_eq!(restored, base);
    }

    #[test]
    fn norm_config_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&NormConfigOperation::SetSelectedCheckIndex { index: Some(2) });
        store::test_support::assert_op_line_round_trip(&NormConfigOperation::SetSelectedCheckIndex { index: None });
        store::test_support::assert_op_line_round_trip(&NormConfigOperation::Snapshot { config: NormConfig { selected_check_index: Some(9) } });
    }

    /// 🧷️ Pins the config operations' exact pre-migration wire bytes (from the ticket's
    /// `🧪️wire-baseline-before.txt`) — `NormConfig` moved file but must not move format.
    #[test]
    fn config_operations_keep_their_pre_migration_bytes() {
        let hex = |op: &NormConfigOperation| protocol::OpBinary::encode_op(op).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&NormConfigOperation::Snapshot { config: NormConfig::default() }), "01000001000e0d00");
        assert_eq!(hex(&NormConfigOperation::Snapshot { config: NormConfig { selected_check_index: Some(9) } }), "01000001000e0d01000409");
        assert_eq!(hex(&NormConfigOperation::SetSelectedCheckIndex { index: Some(2) }), "01010001000402");
        assert_eq!(hex(&NormConfigOperation::SetSelectedCheckIndex { index: None }), "01010000");
    }
}
//#endregion 🧪️Tests
