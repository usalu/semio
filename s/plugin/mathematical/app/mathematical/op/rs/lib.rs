//! ⚡ Mathematical app — operation enum + laws (constitutional: op).

use mathematical::{math_graph_from_dsl, math_graph_to_dsl, MathGeometry, MathGraph, MathGraphDsl, MathProjection};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖Operation
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

/// 📤 Coarse-grained operations: each replaces one top-level projection slice; `backwards` snapshots the pre-state.
/// JSON-facing only — DSL op-line text round-trips through `MathOperationDsl` (see `🔖OpText`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum MathOperation {
    SetGraph { graph: MathGraph },
    SetGeometry { geometry: MathGeometry },
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
//#endregion 🔖Operation

//#region 🔖OpText
// 🧭 The DSL-only mirror (`MathOperationDsl`) and the manual `OpText`/`OpBinary` impls below live
// here (not in the `protocol` constitutional crate) because of Rust's orphan rule: `MathOperation` is
// defined in this crate, so foreign traits (`protocol::OpText`/`protocol::OpBinary`) can only be
// implemented for it here or inside `protocol` itself. The `protocol` crate stays a thin wrapper.

/// ⚡ DSL-only mirror of `MathOperation` — `SetGraph`/`SetGeometry` auto-kebab to
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
    },
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

/// ⚡ Binary mirror of the `OpText` impl above — `MathOperationDsl` already derives `OpBinary`
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
//#endregion 🔖OpText

//#region 🧪Tests
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
}
//#endregion 🧪Tests
