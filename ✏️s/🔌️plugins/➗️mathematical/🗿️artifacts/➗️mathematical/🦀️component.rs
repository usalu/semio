//! 🧮️ Mathematical artifact — the document entities this plugin's app edits: a graph playground
//! (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one projection.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};
use store::DocumentDsl;

//#region 🔖️Constants
/// 🗂️ The store envelope schema AND the plugin's registered document codec key — see
/// `crate::artifacts::mathematical::engine::register`.
pub const MATH_DOCUMENT_SCHEMA: &str = "semio.mathematical/v1";
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MathNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// 🔌️ JSON-facing edge — plain `source`/`target` id strings, unchanged for the JS frontend's
/// `nodeGraphEdit`/`setDocument` payloads. The DSL-facing shape is `MathEdgeDsl` (see
/// `crate::artifacts::mathematical::dsl`), which folds these into one `dsl::Wire` literal per the
/// unified syntax law for graph edges.
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
/// JSON-facing only now — see `MathGraphDsl` in `crate::artifacts::mathematical::dsl` for the DSL-facing
/// twin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathGraph {
    pub directed: bool,
    pub nodes: Vec<MathNode>,
    pub edges: Vec<MathEdge>,
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
            algorithm: "topo".into(),
            algorithm_seed: None,
        }
    }
}

/// 📍️ A single geometry point — the DSL engine's `DslField` binding has no impl for raw Rust
/// tuples (only named types deriving `DslRecord`/`DslScalar`), so `MathGeometry::points` uses this
/// named record instead of a bare `(f64, f64)`; `From`/`Into` conversions keep the rest of the
/// crate's tuple-based call sites (JSON args, `math::geometry::Point`) unchanged.
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

/// 📐️ Geometry playground state: a point cloud for convex-hull/centroid demonstration.
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

/// 📄️ JSON-facing document projection — DSL text round-trips through `MathProjectionDsl` (see
/// `crate::artifacts::mathematical::dsl`), a manual `store::DocumentDsl` impl instead of the direct
/// derive, since `MathGraph`'s edges need the `dsl::Wire` shape that a plain-`String` `MathEdge` can't
/// itself carry alongside `Serialize`/`Deserialize`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathProjection {
    pub graph: MathGraph,
    pub geometry: MathGeometry,
}

impl DocumentDsl for MathProjection {
    const EXTENSION: &'static str = "mathematical";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let dsl_projection = <crate::artifacts::mathematical::dsl::MathProjectionDsl as DocumentDsl>::parse_dsl(text)?;
        crate::artifacts::mathematical::dsl::math_projection_from_dsl(dsl_projection).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <crate::artifacts::mathematical::dsl::MathProjectionDsl as DocumentDsl>::print_dsl(&crate::artifacts::mathematical::dsl::math_projection_to_dsl(self))
    }
}
//#endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::mathematical::create_mathematical_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.mathematical".into(),
        name: "Mathematical".into(),
        source_format: MATH_DOCUMENT_SCHEMA.into(),
        component_kind: "mathematical".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        schema: "computation.mathematical".into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "computation.mathematical");
        assert_eq!(MATH_DOCUMENT_SCHEMA, "semio.mathematical/v1");
    }

    #[test]
    fn default_graph_has_nodes_and_edges() {
        let graph = MathGraph::default();
        assert!(!graph.nodes.is_empty());
        assert!(!graph.edges.is_empty());
    }

    #[test]
    fn default_geometry_has_points() {
        assert!(!MathGeometry::default().points.is_empty());
    }
}
//#endregion 🧪️Tests
