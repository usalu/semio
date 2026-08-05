//! 📜️ Mathematical artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The DSL-mirror types and the `store::DocumentDsl` impl for `MathProjection` live here rather than
//! next to `MathProjection` itself in `crate::artifacts::mathematical`: Rust's orphan rule only requires
//! the foreign trait (`store::DocumentDsl`) or the type (`MathProjection`) to live in this crate — since
//! both now do (the old 7-crate split's per-crate orphan-rule boundary no longer exists), the impl is free
//! to live wherever is clearest, which is next to its own DSL-mirror machinery.
//!
//! No external `.mathematical` fixture file has ever shipped for this app, so these laws stay proven
//! purely against inline-constructed fixtures (mirrors the original flattened `🔖️DslTests`).

use crate::artifacts::mathematical::{MathEdge, MathGeometry, MathGraph, MathNode, MathProjection};
use store::DocumentDsl;

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `MathEdge` — folds `source`/`target` into one unified `dsl::Wire` literal
/// (`source->target`) instead of two separate string fields, per the unified syntax law for graph
/// edges/connections. Converts at the `store::DocumentDsl`/`protocol::OpText` boundary only
/// (`math_edge_to_dsl`/`math_edge_from_dsl`); `MathEdge` itself (JSON shape, `algorithm_overlay`,
/// `workflow_json`, the `nodeGraphEdit` action) is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct MathEdgeDsl {
    id: String,
    wire: dsl::Wire,
}

pub fn math_edge_to_dsl(edge: &MathEdge, directed: bool) -> MathEdgeDsl {
    let from = dsl::WireNode { id: edge.source.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.target.clone(), kind: None, port: None };
    MathEdgeDsl { id: edge.id.clone(), wire: dsl::Wire(dsl::WireValue { from, edge: Some((directed, to)), properties: dsl::DslValue::Object(Vec::new()) }) }
}

pub fn math_edge_from_dsl(edge: MathEdgeDsl) -> Result<MathEdge, String> {
    let dsl::WireValue { from, edge: link, .. } = edge.wire.0;
    let (_directed, to) = link.ok_or_else(|| "graph edge wire literal must have a target".to_string())?;
    Ok(MathEdge { id: edge.id, source: from.id, target: to.id })
}

/// 🕸️ DSL-only mirror of `MathGraph` — `nodes`/`edges` print as SoA tables, `edges` wire-typed via
/// `MathEdgeDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct MathGraphDsl {
    directed: bool,
    #[dsl(table)]
    nodes: Vec<MathNode>,
    #[dsl(table)]
    edges: Vec<MathEdgeDsl>,
    algorithm: String,
    algorithm_seed: Option<String>,
}

pub fn math_graph_to_dsl(graph: &MathGraph) -> MathGraphDsl {
    MathGraphDsl { directed: graph.directed, nodes: graph.nodes.clone(), edges: graph.edges.iter().map(|edge| math_edge_to_dsl(edge, graph.directed)).collect(), algorithm: graph.algorithm.clone(), algorithm_seed: graph.algorithm_seed.clone() }
}

pub fn math_graph_from_dsl(graph: MathGraphDsl) -> Result<MathGraph, String> {
    Ok(MathGraph { directed: graph.directed, nodes: graph.nodes, edges: graph.edges.into_iter().map(math_edge_from_dsl).collect::<Result<Vec<_>, _>>()?, algorithm: graph.algorithm, algorithm_seed: graph.algorithm_seed })
}

/// 📄️ DSL-only mirror of `MathProjection` — the actual `#[derive(dsl::DslDocument)]` root.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "mathematical", layout = "lines")]
pub struct MathProjectionDsl {
    #[dsl(block)]
    graph: MathGraphDsl,
    #[dsl(block)]
    geometry: MathGeometry,
}

pub fn math_projection_to_dsl(projection: &MathProjection) -> MathProjectionDsl {
    MathProjectionDsl { graph: math_graph_to_dsl(&projection.graph), geometry: projection.geometry.clone() }
}

pub fn math_projection_from_dsl(projection: MathProjectionDsl) -> Result<MathProjection, String> {
    Ok(MathProjection { graph: math_graph_from_dsl(projection.graph)?, geometry: projection.geometry })
}
//#endregion 🔖️Dsl

//#region 🔖️DslText
/// 📖️ Parses `.mathematical` DSL text into a `MathProjection`.
pub fn parse_dsl(text: &str) -> Result<MathProjection, store::TextError> {
    <MathProjection as DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `MathProjection` back to `.mathematical` DSL text.
pub fn print_dsl(projection: &MathProjection) -> String {
    DocumentDsl::print_dsl(projection)
}
//#endregion 🔖️DslText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_projection_dsl_round_trips_default() {
        store::test_support::assert_dsl_round_trip(&MathProjection::default());
    }

    #[test]
    fn math_projection_dsl_round_trips_with_seed_and_empty_collections() {
        let mut graph = MathGraph::default();
        graph.algorithm = "bfs".into();
        graph.algorithm_seed = Some("a".into());
        graph.nodes.clear();
        graph.edges.clear();
        let projection = MathProjection { graph, geometry: MathGeometry { points: Vec::new() } };
        store::test_support::assert_dsl_round_trip(&projection);
    }
}
//#endregion 🧪️Tests
