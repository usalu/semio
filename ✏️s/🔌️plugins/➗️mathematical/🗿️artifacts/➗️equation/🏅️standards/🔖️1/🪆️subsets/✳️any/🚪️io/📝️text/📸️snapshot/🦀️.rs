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

#[cfg(test)]
use crate::EquationGeometry;
use crate::{EquationEdge, EquationGraph, EquationNode, EquationSnapshot};
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
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
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct EquationEdgeDsl {
    id: String,
    wire: semio_framework_dsl_record::Wire,
}

pub fn math_edge_to_dsl(edge: &EquationEdge, directed: bool) -> EquationEdgeDsl {
    let from = semio_framework_dsl_record::WireNode { id: edge.source.clone(), kind: None, port: None };
    let to = semio_framework_dsl_record::WireNode { id: edge.target.clone(), kind: None, port: None };
    EquationEdgeDsl { id: edge.id.clone(), wire: semio_framework_dsl_record::Wire(semio_framework_dsl_record::WireValue { from, edge: Some((directed, to)), edge_label: semio_framework_dsl_record::WireEdgeLabel::default(), properties: semio_framework_value::DslValue::Object(Vec::new()) }) }
}

pub fn math_edge_from_dsl(edge: EquationEdgeDsl) -> Result<EquationEdge, String> {
    let semio_framework_dsl_record::WireValue { from, edge: link, .. } = edge.wire.0;
    let (_directed, to) = link.ok_or_else(|| "graph edge wire literal must have a target".to_string())?;
    Ok(EquationEdge { id: edge.id, source: from.id, target: to.id })
}

/// 🕸️ DSL-only mirror of `EquationGraph` — `nodes`/`edges` print as SoA tables, `edges` wire-typed via
/// `EquationEdgeDsl`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
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
        let semio_framework_dsl_record::WireValue { from, edge: link, .. } = &edge.wire.0;
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
            Err(_) => semio_framework_value::DslValue::Null,
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
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Equation logical text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)));}rest},
            Err(_) => text,
        };
        crate::standards::v1::subsets::any::io::binary::snapshot::parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = crate::standards::v1::subsets::any::io::binary::snapshot::print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🔖️DslText
/// 📖️ Parses `.equation` DSL text into a `EquationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<EquationSnapshot, semio_framework_diagnostic::TextError> {
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

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{EquationComputedChild, EquationGeometry, EquationGraph, EquationNotationChild, EquationResultsChild};
use framework_schema::ArtifactSchema;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `equation_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn equation_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <EquationSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <EquationSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <EquationSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <EquationSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <EquationSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <EquationSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = semio_framework_pack_json::object([
        ("parsed".to_string(), semio_framework_pack_json::from_dsl_value(&parsed.to_value())),
        ("reparsed".to_string(), semio_framework_pack_json::from_dsl_value(&reparsed.to_value())),
        ("packDecoded".to_string(), semio_framework_pack_json::from_dsl_value(&unpacked.to_value())),
        ("canonicalText".to_string(), semio_framework_pack_json::Value::String(canonical)),
        ("canonicalTextAgain".to_string(), semio_framework_pack_json::Value::String(canonical_again)),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use snapshot_codec::*;
