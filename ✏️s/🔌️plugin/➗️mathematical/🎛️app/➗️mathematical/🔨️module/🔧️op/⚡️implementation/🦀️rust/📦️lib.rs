//! ⚡️ Mathematical app — operation enum + laws (constitutional: op).

use mathematical::{math_graph_from_dsl, math_graph_to_dsl, MathCamera, MathGeometry, MathGraph, MathGraphDsl, MathProjection};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Operation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathDiff {
    #[serde(default)]
    graph: Option<MathGraph>,
    #[serde(default)]
    geometry: Option<MathGeometry>,
}

impl OperationDiff<MathProjection> for MathDiff {
    fn apply(&self, projection: &MathProjection) -> MathProjection {
        let mut next = projection.clone();
        if let Some(graph) = &self.graph {
            next.graph = graph.clone();
        }
        if let Some(geometry) = &self.geometry {
            next.geometry = geometry.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.graph.is_some() {
            self.graph = other.graph;
        }
        if other.geometry.is_some() {
            self.geometry = other.geometry;
        }
    }
}

/// 📤️ Coarse-grained operations: each replaces one top-level projection slice; `backwards` snapshots the pre-state.
/// JSON-facing only — DSL op-line text round-trips through `MathOperationDsl` (see `🔖️OpText`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum MathOperation {
    SetGraph { graph: MathGraph },
    SetGeometry { geometry: MathGeometry }
}

impl Operation<MathProjection> for MathOperation {
    type Diff = MathDiff;

    fn diff(&self, _projection: &MathProjection) -> MathDiff {
        match self {
            MathOperation::SetGraph { graph } => MathDiff { graph: Some(graph.clone()), geometry: None },
            MathOperation::SetGeometry { geometry } => MathDiff { graph: None, geometry: Some(geometry.clone()) },
        }
    }

    fn backwards(&self, projection: &MathProjection) -> Vec<Self> {
        match self {
            MathOperation::SetGraph { .. } => vec![MathOperation::SetGraph { graph: projection.graph.clone() }],
            MathOperation::SetGeometry { .. } => vec![MathOperation::SetGeometry { geometry: projection.geometry.clone() }],
        }
    }
}
//#endregion 🔖️Operation

//#region 🔖️OpText
// 🧭️ The DSL-only mirror (`MathOperationDsl`) and the manual `OpText`/`OpBinary` impls below live
// here (not in the `protocol` constitutional crate) because of Rust's orphan rule: `MathOperation` is
// defined in this crate, so foreign traits (`protocol::OpText`/`protocol::OpBinary`) can only be
// implemented for it here or inside `protocol` itself. The `protocol` crate stays a thin wrapper.

/// ⚡️ DSL-only mirror of `MathOperation` — `SetGraph`/`SetGeometry` auto-kebab to
/// `set-graph`/`set-geometry` with no `#[dsl(key)]` override needed.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum MathOperationDsl {
    SetGraph {
        #[dsl(block)]
        graph: MathGraphDsl,
    },
    SetGeometry {
        #[dsl(block)]
        geometry: MathGeometry,
    }
}

fn math_operation_to_dsl(operation: &MathOperation) -> MathOperationDsl {
    match operation {
        MathOperation::SetGraph { graph } => MathOperationDsl::SetGraph { graph: math_graph_to_dsl(graph) },
        MathOperation::SetGeometry { geometry } => MathOperationDsl::SetGeometry { geometry: geometry.clone() },
    }
}

fn math_operation_from_dsl(operation: MathOperationDsl) -> Result<MathOperation, String> {
    Ok(match operation {
        MathOperationDsl::SetGraph { graph } => MathOperation::SetGraph { graph: math_graph_from_dsl(graph)? },
        MathOperationDsl::SetGeometry { geometry } => MathOperation::SetGeometry { geometry },
    })
}

impl protocol::OpText for MathOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let dsl_operation = <MathOperationDsl as protocol::OpText>::parse_op(line)?;
        math_operation_from_dsl(dsl_operation).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <MathOperationDsl as protocol::OpText>::print_op(&math_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `MathOperationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for MathOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        math_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let dsl_operation = MathOperationDsl::decode_op(bytes)?;
        math_operation_from_dsl(dsl_operation).map_err(|message| protocol::ProtocolError::Malformed { what: "math operation", offset: 0, detail: message })
    }
}
//#endregion 🔖️OpText

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `mathematical_engine::MathConfig`'s operation enum — one variant per settled
/// interaction (mirrors the pre-B1 `MathPlayRuntime` field writes), plus a generic `Snapshot` every
/// variant's `backwards()` returns — mirrors `shooting_op::ShootingConfigOperation`'s "undo this tick is
/// exactly restore the whole-config snapshot from just before it" pattern: `Operation::Diff` is the
/// WHOLE `MathConfig` (not a granular patch type), `diff()` returns "the full config after this op", and
/// `protocol::OperationDiff<MathConfig>::apply` for `MathConfig` itself (see `mathematical_engine`) just
/// returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum MathConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: mathematical_engine::MathConfig,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: MathCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<mathematical_engine::MathConfig> for MathConfigOperation {
    type Diff = mathematical_engine::MathConfig;

    fn diff(&self, base: &mathematical_engine::MathConfig) -> mathematical_engine::MathConfig {
        let mut next = base.clone();
        match self {
            MathConfigOperation::Snapshot { config } => return config.clone(),
            MathConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            MathConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &mathematical_engine::MathConfig) -> Vec<Self> {
        vec![MathConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_set_graph_op_round_trips() {
        store::test_support::assert_op_line_round_trip(&MathOperation::SetGraph { graph: MathGraph::default() });
    }

    #[test]
    fn math_set_geometry_op_round_trips() {
        store::test_support::assert_op_line_round_trip(&MathOperation::SetGeometry { geometry: MathGeometry::default() });
    }

    //#region MathConfigOperation
    #[test]
    fn config_operation_snapshot_diff_ignores_base() {
        let base = mathematical_engine::MathConfig::default();
        let mut snapshot = base.clone();
        snapshot.locale = "de-DE".into();
        let operation = MathConfigOperation::Snapshot { config: snapshot.clone() };
        assert_eq!(Operation::diff(&operation, &base), snapshot);
    }

    #[test]
    fn config_operation_set_camera_round_trips() {
        let base = mathematical_engine::MathConfig::default();
        let camera = MathCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let operation = MathConfigOperation::SetCamera { camera: camera.clone() };
        let next = Operation::diff(&operation, &base);
        assert_eq!(next.camera, camera);
        let backwards = Operation::backwards(&operation, &base);
        assert_eq!(backwards, vec![MathConfigOperation::Snapshot { config: base }]);
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn config_operation_set_locale_round_trips() {
        store::test_support::assert_op_line_round_trip(&MathConfigOperation::SetLocale { value: "de-DE".into() });
    }
    //#endregion MathConfigOperation
}
//#endregion 🧪️Tests
