//! ⚡️ DAG app — operation type facade (constitutional: op).
//!
//! `DagOperation`, its `protocol::Operation`/`OperationDiff` impls, and the `apply`/`diff`/`backwards`
//! logic all live in the shared DAG kernel crate (`infinite_board_port_directed_dag`,
//! `framework/kernel/infinite/board/port/directed/dag/rs`, `🔖️DocumentVcs` region) alongside the
//! `DagDocument` projection they mutate — see `s/plugin/dag/app/rs/lib.rs` for why. Re-exported here so
//! sibling constitutional crates depend on the app-owned name instead of reaching into the kernel path
//! directly.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub use infinite_board_port_directed_dag::DagOperation;
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// 🧮️ `dag_engine::DagConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `DagPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns: since a config-only "View" dispatch is a plain `Apply` (not an `AmendLast` — see
/// `DagPlayApp::handle`), each tick is its own distinct, real config edit, and "undo this tick" is
/// exactly "restore the whole-config snapshot from just before it" — the simplest correct inverse,
/// needing no per-field reverse-patch bookkeeping. `Operation::Diff` is the WHOLE `DagConfig` (not a
/// granular patch type): `diff()` returns "the full config after this op", and
/// `OperationDiff<DagConfig>::apply` for `DagConfig` itself (in `dag_engine`) just returns that
/// snapshot verbatim, ignoring `base` — the same "whole-record diff" shape `DagOperation::SetDocument`
/// already uses for a full-document replace. Mirrors `shooting_op::ShootingConfigOperation` exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum DagConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: dag_engine::DagConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<dag_engine::DagConfig> for DagConfigOperation {
    type Diff = dag_engine::DagConfig;

    fn diff(&self, base: &dag_engine::DagConfig) -> dag_engine::DagConfig {
        let mut next = base.clone();
        match self {
            DagConfigOperation::Snapshot { config } => return config.clone(),
            DagConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            DagConfigOperation::SetCamera { x, y, zoom } => {
                next.camera_x = *x;
                next.camera_y = *y;
                next.camera_zoom = *zoom;
            }
            DagConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &dag_engine::DagConfig) -> Vec<Self> {
        vec![DagConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use dag_engine::DagConfig;

    #[test]
    fn dag_config_operation_text_binary_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::Snapshot { config: DagConfig { selected_node_ids: vec!["n1".into()], camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, locale: "de-DE".into() } });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetSelection { node_ids: vec!["n1".into(), "n2".into()] });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetSelection { node_ids: Vec::new() });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetCamera { x: 12.5, y: -3.0, zoom: 2.25 });
        store::test_support::assert_op_line_round_trip(&DagConfigOperation::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn dag_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = DagConfig { selected_node_ids: vec!["n1".into()], camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0, locale: "en-US".into() };
        let operation = DagConfigOperation::SetSelection { node_ids: vec!["n2".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_node_ids, vec!["n2".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![DagConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
