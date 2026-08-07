//! ⚡️ Mathematical artifact — the operation enum + laws (constitutional: op).
//!
//! `protocol::OpText`/`protocol::OpBinary` for `MathOperation` are implemented directly here, next to the
//! type they're for (Rust's orphan rule only requires the foreign trait or the type to live in this
//! crate — since both now do, there is no reason to split them into a separate file the way the old
//! 7-crate layout's orphan-rule workaround did).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::mathematical::diff::MathDiff;
use crate::artifacts::mathematical::dsl::{math_graph_from_dsl, math_graph_to_dsl, MathGraphDsl};
use crate::artifacts::mathematical::{MathGeometry, MathGraph, MathProjection};
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Operation
/// 📤️ Coarse-grained operations: each replaces one top-level projection slice; `backwards` snapshots the
/// pre-state. JSON-facing only — DSL op-line text round-trips through `MathOperationDsl` (see
/// `🔖️OpText`).
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
//#endregion 🔖️Operation

//#region 🔖️OpText
/// ⚡️ DSL-only mirror of `MathOperation` — `SetGraph`/`SetGeometry` auto-kebab to
/// `set-graph`/`set-geometry` with no `#[dsl(key)]` override needed.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
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
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for MathOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for MathOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




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
/// via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OperationDiff;

    #[test]
    fn math_set_graph_op_round_trips() {
        store::test_support::assert_op_line_round_trip(&MathOperation::SetGraph { graph: MathGraph::default() });
    }

    #[test]
    fn math_set_geometry_op_round_trips() {
        store::test_support::assert_op_line_round_trip(&MathOperation::SetGeometry { geometry: MathGeometry::default() });
    }

    #[test]
    fn set_layout_backwards_restores_the_base_projection() {
        let base = MathProjection::default();
        let mut graph = base.graph.clone();
        graph.algorithm = "components".into();
        let operation = MathOperation::SetGraph { graph };
        let forward = operation.diff(&base).apply(&base);
        let restored = operation.backwards(&base).iter().fold(forward, |projection, inverse| inverse.diff(&projection).apply(&projection));
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
