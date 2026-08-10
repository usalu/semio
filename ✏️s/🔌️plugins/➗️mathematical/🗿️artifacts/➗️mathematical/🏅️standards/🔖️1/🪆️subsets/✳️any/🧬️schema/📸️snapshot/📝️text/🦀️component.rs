//! 📜️ Mathematical artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The DSL-mirror types and the `store::ArtifactDsl` impl for `MathematicalSnapshot` live here rather than
//! next to `MathematicalSnapshot` itself in `crate::artifacts::mathematical`: Rust's orphan rule only requires
//! the foreign trait (`store::ArtifactDsl`) or the type (`MathematicalSnapshot`) to live in this crate — since
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


use crate::artifacts::mathematical::{MathematicalEdge, MathematicalGeometry, MathematicalGraph, MathematicalNode, MathematicalSnapshot};
use serde::{Deserialize, Serialize};
use store::ArtifactDsl;

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `MathematicalEdge` — folds `source`/`target` into one unified `dsl::Wire` literal
/// (`source->target`) instead of two separate string fields, per the unified syntax law for graph
/// edges/connections. Converts at the `store::ArtifactDsl`/`protocol::OpText` boundary only
/// (`math_edge_to_dsl`/`math_edge_from_dsl`); `MathematicalEdge` itself (JSON shape, `algorithm_overlay`,
/// `workflow_json`, the `nodeGraphEdit` action) is completely untouched.
///
/// No `Serialize`/`Deserialize` derive: `dsl::Wire` (the framework DSL kernel's wire-literal field type)
/// does not implement either, and it is out of this plugin's scope to add them there. `MathematicalGraphDsl`
/// below — the only place this type is ever nested inside something serde-derived (`app_commands!`
/// unconditionally derives `Serialize`/`Deserialize` on the generated `MathematicalCommand` enum, even though
/// its actual wire codec is `dsl::DslOps`, never `serde_json`; see `crate::apps::mathematical`'s
/// `🔖️Commands` doc comment) — hand-implements those traits by round-tripping through the fully
/// serde-able `MathematicalGraph`/`MathematicalEdge` JSON shape instead of deriving them field-by-field.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct MathematicalEdgeDsl {
    id: String,
    wire: dsl::Wire,
}

pub fn math_edge_to_dsl(edge: &MathematicalEdge, directed: bool) -> MathematicalEdgeDsl {
    let from = dsl::WireNode { id: edge.source.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.target.clone(), kind: None, port: None };
    MathematicalEdgeDsl { id: edge.id.clone(), wire: dsl::Wire(dsl::WireValue { from, edge: Some((directed, to)), edge_label: dsl::WireEdgeLabel::default(), properties: dsl::DslValue::Object(Vec::new()) }) }
}

pub fn math_edge_from_dsl(edge: MathematicalEdgeDsl) -> Result<MathematicalEdge, String> {
    let dsl::WireValue { from, edge: link, .. } = edge.wire.0;
    let (_directed, to) = link.ok_or_else(|| "graph edge wire literal must have a target".to_string())?;
    Ok(MathematicalEdge { id: edge.id, source: from.id, target: to.id })
}

/// 🕸️ DSL-only mirror of `MathematicalGraph` — `nodes`/`edges` print as SoA tables, `edges` wire-typed via
/// `MathematicalEdgeDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct MathematicalGraphDsl {
    directed: bool,
    #[dsl(table)]
    nodes: Vec<MathematicalNode>,
    #[dsl(table)]
    edges: Vec<MathematicalEdgeDsl>,
    algorithm: String,
    algorithm_seed: Option<String>,
}

pub fn math_graph_to_dsl(graph: &MathematicalGraph) -> MathematicalGraphDsl {
    MathematicalGraphDsl { directed: graph.directed, nodes: graph.nodes.clone(), edges: graph.edges.iter().map(|edge| math_edge_to_dsl(edge, graph.directed)).collect(), algorithm: graph.algorithm.clone(), algorithm_seed: graph.algorithm_seed.clone() }
}

pub fn math_graph_from_dsl(graph: MathematicalGraphDsl) -> Result<MathematicalGraph, String> {
    Ok(MathematicalGraph { directed: graph.directed, nodes: graph.nodes, edges: graph.edges.into_iter().map(math_edge_from_dsl).collect::<Result<Vec<_>, _>>()?, algorithm: graph.algorithm, algorithm_seed: graph.algorithm_seed })
}

/// 🔌️ Hand-rolled `Serialize`/`Deserialize` for `MathematicalGraphDsl` — see the type's own doc comment for why
/// this can't be `#[derive(...)]`d. Round-trips through the fully serde-able `MathematicalGraph` JSON shape via
/// the same `math_graph_to_dsl`/`math_graph_from_dsl` conversions the DSL/pack codecs already use, so the
/// JSON shape a caller would observe (were this ever actually put on a real wire) is `MathematicalGraph`'s own
/// camelCase shape, not a `MathematicalGraphDsl`-internal one.
impl Serialize for MathematicalGraphDsl {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        math_graph_from_dsl(self.clone()).map_err(serde::ser::Error::custom)?.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MathematicalGraphDsl {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(math_graph_to_dsl(&MathematicalGraph::deserialize(deserializer)?))
    }
}

/// 📄️ DSL-only mirror of `MathematicalSnapshot` — the actual `#[derive(dsl::DslRecord)]` root.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "mathematical.mathematical", layout = "lines")]
pub struct MathematicalSnapshotDsl {
    #[dsl(block)]
    graph: MathematicalGraphDsl,
    #[dsl(block)]
    geometry: MathematicalGeometry,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for MathematicalSnapshotDsl {
    const EXTENSION: &'static str = "mathematical";
    fn envelope_id() -> &'static str {
        "mathematical.mathematical"
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
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for MathematicalSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedArtifactCodecs




pub fn mathematical_snapshot_to_dsl(projection: &MathematicalSnapshot) -> MathematicalSnapshotDsl {
    MathematicalSnapshotDsl { graph: math_graph_to_dsl(&projection.graph), geometry: projection.geometry.clone() }
}

pub fn mathematical_snapshot_from_dsl(projection: MathematicalSnapshotDsl) -> Result<MathematicalSnapshot, String> {
    Ok(MathematicalSnapshot { graph: math_graph_from_dsl(projection.graph)?, geometry: projection.geometry })
}
//#endregion 🔖️Dsl

//#region 🔖️DslText
/// 📖️ Parses `.mathematical` DSL text into a `MathematicalSnapshot`.
pub fn parse_dsl(text: &str) -> Result<MathematicalSnapshot, store::TextError> {
    <MathematicalSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `MathematicalSnapshot` back to `.mathematical` DSL text.
pub fn print_dsl(projection: &MathematicalSnapshot) -> String {
    ArtifactDsl::print_dsl(projection)
}
//#endregion 🔖️DslText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_projection_dsl_round_trips_default() {
        store::os_store::test_support::assert_dsl_round_trip(&MathematicalSnapshot::default());
    }

    #[test]
    fn example_primary_text_round_trips() {
        let text = include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
        let parsed = crate::artifacts::mathematical::dsl::parse_dsl(text).expect("parse example");
        store::os_store::test_support::assert_dsl_round_trip(&parsed);
    }

    #[test]
    fn math_projection_dsl_round_trips_with_seed_and_empty_collections() {
        let mut graph = MathematicalGraph {
            algorithm: "bfs".into(),
            algorithm_seed: Some("a".into()),
            ..MathematicalGraph::default()
        };
        graph.nodes.clear();
        graph.edges.clear();
        let projection = MathematicalSnapshot {
            graph,
            geometry: MathematicalGeometry { points: Vec::new() },
        };
        store::os_store::test_support::assert_dsl_round_trip(&projection);
    }
}
//#endregion 🧪️Tests
