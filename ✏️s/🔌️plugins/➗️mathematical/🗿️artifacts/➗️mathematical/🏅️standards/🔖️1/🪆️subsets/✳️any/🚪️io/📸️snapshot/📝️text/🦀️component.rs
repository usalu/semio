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

use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
#[cfg(test)]
use crate::artifacts::mathematical::MathematicalGeometry;
use crate::artifacts::mathematical::{MathematicalEdge, MathematicalGraph, MathematicalNode, MathematicalSnapshot};
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️component.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
use store::ArtifactDsl;

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `MathematicalEdge` — folds `source`/`target` into one unified `dsl::Wire` literal
/// (`source->target`) instead of two separate string fields, per the unified syntax law for graph
/// edges/connections. Converts at the `store::ArtifactDsl`/`protocol::OpText` boundary only
/// (`math_edge_to_dsl`/`math_edge_from_dsl`); `MathematicalEdge` itself (JSON shape, `algorithm_overlay`,
/// `workflow_json`, the `nodeGraphEdit` action) is completely untouched.
///
/// `dsl::Wire` (the framework DSL kernel's wire-literal field type) has no value codec. The enclosing
/// `MathematicalGraphDsl` bridges through `MathematicalGraph` instead.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct MathematicalEdgeDsl {
    id: String,
    wire: dsl::Wire,
}

pub async fn math_edge_to_dsl(edge: &MathematicalEdge, directed: bool) -> MathematicalEdgeDsl {
    let from = dsl::WireNode { id: edge.source.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.target.clone(), kind: None, port: None };
    MathematicalEdgeDsl { id: edge.id.clone(), wire: dsl::Wire(dsl::WireValue { from, edge: Some((directed, to)), edge_label: dsl::WireEdgeLabel::default(), properties: dsl::DslValue::Object(Vec::new()) }) }
}

pub async fn math_edge_from_dsl(edge: MathematicalEdgeDsl) -> Result<MathematicalEdge, String> {
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

impl MathematicalGraphDsl {
    /// 🧮 Exposes bounded graph metadata without cloning the payload.
    pub fn retained_metadata(&self) -> (bool, &str, Option<&str>) {
        (self.directed, &self.algorithm, self.algorithm_seed.as_deref())
    }

    /// 🧮 Counts node rows for retained preflight.
    pub fn retained_node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 🧮 Borrows one node row for a retained microstep.
    pub fn retained_node(&self, index: usize) -> Option<&MathematicalNode> {
        self.nodes.get(index)
    }

    /// 🧮 Counts edge rows for retained preflight.
    pub fn retained_edge_count(&self) -> usize {
        self.edges.len()
    }

    /// 🧮 Materializes one edge row for a retained microstep.
    pub fn retained_edge(&self, index: usize) -> Result<MathematicalEdge, String> {
        let edge = self.edges.get(index).ok_or_else(|| "graph edge cursor is out of range".to_string())?;
        let dsl::WireValue { from, edge: link, .. } = &edge.wire.0;
        let (_, to) = link.as_ref().ok_or_else(|| "graph edge wire literal must have a target".to_string())?;
        Ok(MathematicalEdge { id: edge.id.clone(), source: from.id.clone(), target: to.id.clone() })
    }
}

pub async fn math_graph_to_dsl(graph: &MathematicalGraph) -> MathematicalGraphDsl {
    MathematicalGraphDsl {
        directed: graph.directed,
        nodes: graph.nodes.clone(),
        edges: graph.edges.iter().map(|edge| math_edge_to_dsl(edge, graph.directed)).collect(),
        algorithm: graph.algorithm.clone(),
        algorithm_seed: graph.algorithm_seed.clone(),
    }
}

pub async fn math_graph_from_dsl(graph: MathematicalGraphDsl) -> Result<MathematicalGraph, String> {
    Ok(MathematicalGraph { directed: graph.directed, nodes: graph.nodes, edges: graph.edges.into_iter().map(math_edge_from_dsl).collect::<Result<Vec<_>, _>>()?, algorithm: graph.algorithm, algorithm_seed: graph.algorithm_seed })
}

/// 🌱️ Hand-written `ToValue`/`FromValue` for `MathematicalGraphDsl`: `dsl::Wire`, nested inside
/// `MathematicalEdgeDsl`, implements neither. `ToValue::to_value` has no `Result` to
/// propagate a conversion failure through (unlike `Serialize::serialize`) — falls back to
/// `DslValue::Null`, which cannot happen for any `MathematicalGraphDsl` actually produced by
/// `math_graph_to_dsl` (the only constructor), matching this file's other defensive `unwrap_or`
/// fallbacks over composed fields.
impl ToValue for MathematicalGraphDsl {
    fn to_value(&self) -> DslValue {
        match math_graph_from_dsl(self.clone()) {
            Ok(graph) => graph.to_value(),
            Err(_) => DslValue::Null,
        }
    }
}
impl FromValue for MathematicalGraphDsl {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(math_graph_to_dsl(&MathematicalGraph::from_value(value)?))
    }
}

/// 📌️ `MathematicalGraphDsl`/`MathematicalEdgeDsl` above are the DSL-only shape the `SetArtifact`
/// app command's own payload uses (`🎮️commands/📄️set-artifact/🦀️component.rs`) — that command still
/// carries a WHOLE graph as one gesture (routed onto the granular `ReplaceGraph`/`ReplacePoints`
/// mutations, never a banned whole-snapshot replace), so it kept its own `#[derive(dsl::DslRecord)]`
/// wire shape. The former `MathematicalSnapshotDsl` mirror — the snapshot's OWN codec — is gone:
/// `MathematicalSnapshot` no longer derives (indirectly or otherwise) `dsl::DslRecord` now that
/// `notation`/`results`/`computed` are composed `ArtifactChild<S>` slots (no `DslField` impl for
/// those reachable from this crate); its `ArtifactDsl` is hand-rolled directly below
/// (`🔖️HandcraftedArtifactDsl` region, this file) — same upgrade `📐️cad`/`✒️writer` made.
//#endregion 🔖️Dsl

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ P6 handcrafted `ArtifactDsl`, real hex/bracket text primitives — moved here verbatim from
/// `🧬️schema/📸️snapshot/🦀️component.rs` (design.md §1 CORRECTION: the native codec is one
/// bidirectional thing per representation, unsplit, and sits directly under `🚪️io/<facet>/
/// <representation>/`). Same upgrade `📐️cad`/`✒️writer` made once their snapshot gained a real
/// `ArtifactChild<S>` slot (the old `dsl::DslRecord`-derive-driven path cannot express a
/// composed child slot, which has no `dsl::DslField` impl reachable from this crate).
//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`'s/`✒️writer`'s own `enc_child`/
/// `dec_child`) — a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened
/// via `to_uri()`), never the child's own content.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
async fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
async fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
async fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
/// 🧮️ `equation` has no handcrafted grammar of its own yet (future wave) — round-tripped as
/// hex-encoded first-party JSON (`pack::json::to_json_string`/`from_json_str`, over `EquationSnapshot`'s
/// own `ToValue`/`FromValue`), the same "real codec, minimal grammar" trade `child` handles above
/// already make for their own opaque payload half (the `ArtifactRef` URI).
async fn enc_equation(e: &EquationSnapshot) -> String {
    enc_str(&pack::json::to_json_string(e))
}
async fn dec_equation(s: &str) -> Result<EquationSnapshot, String> {
    pack::json::from_json_str(&dec_str(s)?).map_err(|e| e.to_string())
}

async fn print_mathematical_snapshot_body(s: &MathematicalSnapshot) -> String {
    format!("notation={}\nresults={}\ncomputed={}\nequation={}", enc_child(&s.notation), enc_child(&s.results), enc_child(&s.computed), enc_equation(&s.equation))
}
async fn parse_mathematical_snapshot_body(body: &str) -> Result<MathematicalSnapshot, String> {
    let mut notation = None;
    let mut results = None;
    let mut computed = None;
    let mut equation = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("notation=") {
            notation = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("results=") {
            results = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("computed=") {
            computed = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("equation=") {
            equation = Some(dec_equation(rest)?);
        } else {
            return Err(format!("mathematical snapshot: unknown line {line:?}"));
        }
    }
    Ok(MathematicalSnapshot {
        notation: notation.ok_or_else(|| "mathematical snapshot: missing notation line".to_string())?,
        results: results.ok_or_else(|| "mathematical snapshot: missing results line".to_string())?,
        computed: computed.ok_or_else(|| "mathematical snapshot: missing computed line".to_string())?,
        equation: equation.ok_or_else(|| "mathematical snapshot: missing equation line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

impl store::ArtifactDsl for MathematicalSnapshot {
    const EXTENSION: &'static str = "mathematical";
    async fn envelope_id() -> &'static str {
        "mathematical.mathematical"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_mathematical_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_mathematical_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🔖️DslText
/// 📖️ Parses `.mathematical` DSL text into a `MathematicalSnapshot`.
pub async fn parse_dsl(text: &str) -> Result<MathematicalSnapshot, store::TextError> {
    <MathematicalSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `MathematicalSnapshot` back to `.mathematical` DSL text.
pub async fn print_dsl(projection: &MathematicalSnapshot) -> String {
    ArtifactDsl::print_dsl(projection)
}
//#endregion 🔖️DslText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn math_projection_dsl_round_trips_default() {
        store::os_store::test_support::assert_dsl_round_trip(&MathematicalSnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn example_primary_text_round_trips() {
        let text = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
        let parsed = crate::artifacts::mathematical::dsl::parse_dsl(text).expect("parse example");
        store::os_store::test_support::assert_dsl_round_trip(&parsed);
    }

    #[semio_framework_async_macros::async_test]
    async fn math_projection_dsl_round_trips_with_seed_and_empty_collections() {
        let mut graph = MathematicalGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..MathematicalGraph::default() };
        graph.nodes.clear();
        graph.edges.clear();
        let projection = crate::artifacts::mathematical::mathematical_snapshot_with_state(graph, MathematicalGeometry { points: Vec::new() });
        store::os_store::test_support::assert_dsl_round_trip(&projection);
    }
}
//#endregion 🧪️Tests
//#endregion 🧪️Tests
