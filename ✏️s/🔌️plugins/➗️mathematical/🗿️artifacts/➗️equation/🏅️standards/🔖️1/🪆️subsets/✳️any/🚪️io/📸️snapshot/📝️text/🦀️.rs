//! 📜️ Equation artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The DSL-mirror types and the `store::ArtifactDsl` impl for `EquationSnapshot` live here rather than
//! next to `EquationSnapshot` itself in `crate`: Rust's orphan rule only requires
//! the foreign trait (`store::ArtifactDsl`) or the type (`EquationSnapshot`) to live in this crate — since
//! both now do (the old 7-crate split's per-crate orphan-rule boundary no longer exists), the impl is free
//! to live wherever is clearest, which is next to its own DSL-mirror machinery.
//!
//! No external `.equation` fixture file has ever shipped for this app, so these laws stay proven
//! purely against inline-constructed fixtures (mirrors the original flattened `🔖️DslTests`).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot;
#[cfg(test)]
use crate::EquationGeometry;
use crate::{EquationEdge, EquationGraph, EquationNode, EquationSnapshot};
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
use store::ArtifactDsl;

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `EquationEdge` — folds `source`/`target` into one unified `dsl::Wire` literal
/// (`source->target`) instead of two separate string fields, per the unified syntax law for graph
/// edges/connections. Converts at the `store::ArtifactDsl`/`protocol::OpText` boundary only
/// (`math_edge_to_dsl`/`math_edge_from_dsl`); `EquationEdge` itself (JSON shape, `algorithm_overlay`,
/// `workflow_json`, the `nodeGraphEdit` action) is completely untouched.
///
/// `dsl::Wire` (the framework DSL kernel's wire-literal field type) has no value codec. The enclosing
/// `EquationGraphDsl` bridges through `EquationGraph` instead.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct EquationEdgeDsl {
    id: String,
    wire: dsl::Wire,
}

pub fn math_edge_to_dsl(edge: &EquationEdge, directed: bool) -> EquationEdgeDsl {
    let from = dsl::WireNode { id: edge.source.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.target.clone(), kind: None, port: None };
    EquationEdgeDsl { id: edge.id.clone(), wire: dsl::Wire(dsl::WireValue { from, edge: Some((directed, to)), edge_label: dsl::WireEdgeLabel::default(), properties: DslValue::Object(Vec::new()) }) }
}

pub fn math_edge_from_dsl(edge: EquationEdgeDsl) -> Result<EquationEdge, String> {
    let dsl::WireValue { from, edge: link, .. } = edge.wire.0;
    let (_directed, to) = link.ok_or_else(|| "graph edge wire literal must have a target".to_string())?;
    Ok(EquationEdge { id: edge.id, source: from.id, target: to.id })
}

/// 🕸️ DSL-only mirror of `EquationGraph` — `nodes`/`edges` print as SoA tables, `edges` wire-typed via
/// `EquationEdgeDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct EquationGraphDsl {
    directed: bool,
    #[dsl(table)]
    nodes: Vec<EquationNode>,
    #[dsl(table)]
    edges: Vec<EquationEdgeDsl>,
    algorithm: String,
    algorithm_seed: Option<String>,
}

impl EquationGraphDsl {
    /// 🧮 Exposes bounded graph metadata without cloning the payload.
    pub fn retained_metadata(&self) -> (bool, &str, Option<&str>) {
        (self.directed, &self.algorithm, self.algorithm_seed.as_deref())
    }

    /// 🧮 Counts node rows for retained preflight.
    pub fn retained_node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 🧮 Borrows one node row for a retained microstep.
    pub fn retained_node(&self, index: usize) -> Option<&EquationNode> {
        self.nodes.get(index)
    }

    /// 🧮 Counts edge rows for retained preflight.
    pub fn retained_edge_count(&self) -> usize {
        self.edges.len()
    }

    /// 🧮 Materializes one edge row for a retained microstep.
    pub fn retained_edge(&self, index: usize) -> Result<EquationEdge, String> {
        let edge = self.edges.get(index).ok_or_else(|| "graph edge cursor is out of range".to_string())?;
        let dsl::WireValue { from, edge: link, .. } = &edge.wire.0;
        let (_, to) = link.as_ref().ok_or_else(|| "graph edge wire literal must have a target".to_string())?;
        Ok(EquationEdge { id: edge.id.clone(), source: from.id.clone(), target: to.id.clone() })
    }
}

pub fn math_graph_to_dsl(graph: &EquationGraph) -> EquationGraphDsl {
    EquationGraphDsl { directed: graph.directed, nodes: graph.nodes.clone(), edges: graph.edges.iter().map(|edge| math_edge_to_dsl(edge, graph.directed)).collect(), algorithm: graph.algorithm.clone(), algorithm_seed: graph.algorithm_seed.clone() }
}

pub fn math_graph_from_dsl(graph: EquationGraphDsl) -> Result<EquationGraph, String> {
    Ok(EquationGraph { directed: graph.directed, nodes: graph.nodes, edges: graph.edges.into_iter().map(math_edge_from_dsl).collect::<Result<Vec<_>, _>>()?, algorithm: graph.algorithm, algorithm_seed: graph.algorithm_seed })
}

/// 🌱️ Hand-written `ToValue`/`FromValue` for `EquationGraphDsl`: `dsl::Wire`, nested inside
/// `EquationEdgeDsl`, implements neither. `ToValue::to_value` has no `Result` to
/// propagate a conversion failure through (unlike `Serialize::serialize`) — falls back to
/// `DslValue::Null`, which cannot happen for any `EquationGraphDsl` actually produced by
/// `math_graph_to_dsl` (the only constructor), matching this file's other defensive `unwrap_or`
/// fallbacks over composed fields.
impl ToValue for EquationGraphDsl {
    fn to_value(&self) -> DslValue {
        match math_graph_from_dsl(self.clone()) {
            Ok(graph) => graph.to_value(),
            Err(_) => DslValue::Null,
        }
    }
}
impl FromValue for EquationGraphDsl {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(math_graph_to_dsl(&EquationGraph::from_value(value)?))
    }
}

//#endregion 🔖️Dsl

//#region 🔖️HandcraftedArtifactDsl
impl ArtifactDsl for EquationSnapshot {
    const EXTENSION: &'static str = "equation";
    fn envelope_id() -> &'static str {
        "mathematical.equation"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        super::binary::parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = super::binary::print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🔖️DslText
/// 📖️ Parses `.equation` DSL text into a `EquationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<EquationSnapshot, store::TextError> {
    <EquationSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `EquationSnapshot` back to `.equation` DSL text.
pub fn print_dsl(projection: &EquationSnapshot) -> String {
    ArtifactDsl::print_dsl(projection)
}
//#endregion 🔖️DslText

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#endregion 🧪️Tests
