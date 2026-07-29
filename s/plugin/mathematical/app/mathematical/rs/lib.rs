//! 🧮 Mathematical app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};
use store::DocumentDsl;

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MathNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// 🔌 JSON-facing edge — plain `source`/`target` id strings, unchanged for the JS frontend's
/// `nodeGraphEdit`/`setDocument` payloads. The DSL-facing shape is `MathEdgeDsl` (see `🔖Dsl`),
/// which folds these into one `dsl::Wire` literal per the unified syntax law for graph edges.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathEdge {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MathCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for MathCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🕸️ Graph playground state: quadrant toggle, retained layout, and the active algorithm overlay.
/// JSON-facing only now — see `MathGraphDsl` in `🔖Dsl` for the DSL-facing twin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathGraph {
    pub directed: bool,
    pub nodes: Vec<MathNode>,
    pub edges: Vec<MathEdge>,
    pub camera: MathCamera,
    pub algorithm: String,
    #[serde(default)]
    pub algorithm_seed: Option<String>,
}

impl Default for MathGraph {
    fn default() -> Self {
        Self {
            directed: true,
            nodes: vec![
                MathNode { id: "a".into(), label: "A".into(), x: 40.0, y: 60.0 },
                MathNode { id: "b".into(), label: "B".into(), x: 240.0, y: 20.0 },
                MathNode { id: "c".into(), label: "C".into(), x: 240.0, y: 180.0 },
                MathNode { id: "d".into(), label: "D".into(), x: 440.0, y: 100.0 },
            ],
            edges: vec![
                MathEdge { id: "e1".into(), source: "a".into(), target: "b".into() },
                MathEdge { id: "e2".into(), source: "a".into(), target: "c".into() },
                MathEdge { id: "e3".into(), source: "b".into(), target: "d".into() },
                MathEdge { id: "e4".into(), source: "c".into(), target: "d".into() },
            ],
            camera: MathCamera::default(),
            algorithm: "topo".into(),
            algorithm_seed: None,
        }
    }
}

/// 📍 A single geometry point — the DSL engine's `DslField` binding has no impl for raw Rust
/// tuples (only named types deriving `DslRecord`/`DslScalar`), so `MathGeometry::points` uses this
/// named record instead of a bare `(f64, f64)`; `From`/`Into` conversions keep the rest of the
/// crate's tuple-based call sites (JSON args, `mathematical_geometry::Point`) unchanged.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct MathPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for MathPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<MathPoint> for (f64, f64) {
    fn from(point: MathPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📐 Geometry playground state: a point cloud for convex-hull/centroid demonstration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MathGeometry {
    pub points: Vec<MathPoint>,
}

impl Default for MathGeometry {
    fn default() -> Self {
        Self { points: vec![(40.0, 220.0), (260.0, 40.0), (360.0, 140.0), (300.0, 260.0), (140.0, 300.0), (180.0, 160.0)].into_iter().map(MathPoint::from).collect() }
    }
}

/// 📄 JSON-facing document projection — DSL text round-trips through `MathProjectionDsl` (see
/// `🔖Dsl`), a manual `store::DocumentDsl` impl instead of the direct derive, since `MathGraph`'s
/// edges need the `dsl::Wire` shape that a plain-`String` `MathEdge` can't itself carry alongside
/// `Serialize`/`Deserialize`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathProjection {
    pub graph: MathGraph,
    pub geometry: MathGeometry,
}
//#endregion 🔖Document

//#region 🔖Dsl
// The crate never having shipped an external `.mathematical` fixture file, this stays proven
// purely against inline-constructed fixtures (see the `mathematical` app's `dsl`/`pack` crates).
//
// 🧭 These DSL-mirror types and the `DocumentDsl`/`DocumentPack` impls below live here (not in the
// `dsl`/`pack` constitutional crates) because of Rust's orphan rule: `MathProjection` is defined in
// this crate, so a foreign trait (`store::DocumentDsl`/`store::DocumentPack`) can only be implemented
// for it here or inside `store` itself. The `dsl`/`pack` crates stay thin wrappers over these impls.

/// 🔌 DSL-only mirror of `MathEdge` — folds `source`/`target` into one unified `dsl::Wire` literal
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
    #[dsl(block)]
    camera: MathCamera,
    algorithm: String,
    algorithm_seed: Option<String>,
}

pub fn math_graph_to_dsl(graph: &MathGraph) -> MathGraphDsl {
    MathGraphDsl {
        directed: graph.directed,
        nodes: graph.nodes.clone(),
        edges: graph.edges.iter().map(|edge| math_edge_to_dsl(edge, graph.directed)).collect(),
        camera: graph.camera.clone(),
        algorithm: graph.algorithm.clone(),
        algorithm_seed: graph.algorithm_seed.clone(),
    }
}

pub fn math_graph_from_dsl(graph: MathGraphDsl) -> Result<MathGraph, String> {
    Ok(MathGraph {
        directed: graph.directed,
        nodes: graph.nodes,
        edges: graph.edges.into_iter().map(math_edge_from_dsl).collect::<Result<Vec<_>, _>>()?,
        camera: graph.camera,
        algorithm: graph.algorithm,
        algorithm_seed: graph.algorithm_seed,
    })
}

/// 📄 DSL-only mirror of `MathProjection` — the actual `#[derive(dsl::DslDocument)]` root.
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

impl DocumentDsl for MathProjection {
    const EXTENSION: &'static str = "mathematical";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let dsl_projection = <MathProjectionDsl as DocumentDsl>::parse_dsl(text)?;
        math_projection_from_dsl(dsl_projection).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <MathProjectionDsl as DocumentDsl>::print_dsl(&math_projection_to_dsl(self))
    }
}

/// 📦 Manual `store::DocumentPack` mirror of the manual `DocumentDsl` impl above — `MathProjectionDsl`
/// (which derives `dsl::DslDocument`) gets `DocumentPack` for free from `dsl_derive`; `MathProjection`
/// itself doesn't derive it (its `edges` need the `MathEdgeDsl`/`dsl::Wire` folding), so this delegates
/// through the same `math_projection_to_dsl`/`math_projection_from_dsl` conversions as `parse_dsl`/`print_dsl`.
impl store::DocumentPack for MathProjection {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        math_projection_to_dsl(self).encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let dsl_projection = MathProjectionDsl::decode_pack_with(bytes, options)?;
        math_projection_from_dsl(dsl_projection).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}
//#endregion 🔖Dsl
