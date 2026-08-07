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


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::mathematical::{MathEdge, MathGeometry, MathGraph, MathNode, MathProjection};
use serde::{Deserialize, Serialize};
use store::DocumentDsl;

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `MathEdge` — folds `source`/`target` into one unified `dsl::Wire` literal
/// (`source->target`) instead of two separate string fields, per the unified syntax law for graph
/// edges/connections. Converts at the `store::DocumentDsl`/`protocol::OpText` boundary only
/// (`math_edge_to_dsl`/`math_edge_from_dsl`); `MathEdge` itself (JSON shape, `algorithm_overlay`,
/// `workflow_json`, the `nodeGraphEdit` action) is completely untouched.
///
/// No `Serialize`/`Deserialize` derive: `dsl::Wire` (the framework DSL kernel's wire-literal field type)
/// does not implement either, and it is out of this plugin's scope to add them there. `MathGraphDsl`
/// below — the only place this type is ever nested inside something serde-derived (`app_commands!`
/// unconditionally derives `Serialize`/`Deserialize` on the generated `MathCommand` enum, even though
/// its actual wire codec is `dsl::DslOps`, never `serde_json`; see `crate::apps::mathematical`'s
/// `🔖️Commands` doc comment) — hand-implements those traits by round-tripping through the fully
/// serde-able `MathGraph`/`MathEdge` JSON shape instead of deriving them field-by-field.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct MathEdgeDsl {
    id: String,
    wire: dsl::Wire,
}

pub fn math_edge_to_dsl(edge: &MathEdge, directed: bool) -> MathEdgeDsl {
    let from = dsl::WireNode { id: edge.source.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.target.clone(), kind: None, port: None };
    MathEdgeDsl { id: edge.id.clone(), wire: dsl::Wire(dsl::WireValue { from, edge: Some((directed, to)), edge_label: dsl::WireEdgeLabel::default(), properties: dsl::DslValue::Object(Vec::new()) }) }
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

/// 🔌️ Hand-rolled `Serialize`/`Deserialize` for `MathGraphDsl` — see the type's own doc comment for why
/// this can't be `#[derive(...)]`d. Round-trips through the fully serde-able `MathGraph` JSON shape via
/// the same `math_graph_to_dsl`/`math_graph_from_dsl` conversions the DSL/pack codecs already use, so the
/// JSON shape a caller would observe (were this ever actually put on a real wire) is `MathGraph`'s own
/// camelCase shape, not a `MathGraphDsl`-internal one.
impl Serialize for MathGraphDsl {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        math_graph_from_dsl(self.clone()).map_err(serde::ser::Error::custom)?.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MathGraphDsl {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(math_graph_to_dsl(&MathGraph::deserialize(deserializer)?))
    }
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

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for MathProjectionDsl {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for MathProjectionDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec


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
        let mut graph = MathGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..MathGraph::default() };
        graph.nodes.clear();
        graph.edges.clear();
        let projection = MathProjection { graph, geometry: MathGeometry { points: Vec::new() } };
        store::test_support::assert_dsl_round_trip(&projection);
    }
}
//#endregion 🧪️Tests
