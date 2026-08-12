//! 🧮️ Mathematical artifact — the document entities this plugin's app edits: a graph playground
//! (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one snapshot.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🗂️ The store envelope schema AND the plugin's registered document codec key — see
/// `crate::artifacts::mathematical::engine::register`.
pub const MATH_DOCUMENT_SCHEMA: &str = "semio.mathematical/v1";
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MathematicalNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// 🔌️ JSON-facing edge — plain `source`/`target` id strings for the JS frontend's node-graph payloads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathematicalEdge {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MathematicalCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for MathematicalCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🕸️ Graph playground state: quadrant toggle, retained layout, and the active algorithm overlay.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathematicalGraph {
    pub directed: bool,
    pub nodes: Vec<MathematicalNode>,
    pub edges: Vec<MathematicalEdge>,
    pub algorithm: String,
    #[serde(default)]
    pub algorithm_seed: Option<String>,
}

impl Default for MathematicalGraph {
    fn default() -> Self {
        Self {
            directed: true,
            nodes: vec![
                MathematicalNode { id: "a".into(), label: "A".into(), x: 40.0, y: 60.0 },
                MathematicalNode { id: "b".into(), label: "B".into(), x: 240.0, y: 20.0 },
                MathematicalNode { id: "c".into(), label: "C".into(), x: 240.0, y: 180.0 },
                MathematicalNode { id: "d".into(), label: "D".into(), x: 440.0, y: 100.0 },
            ],
            edges: vec![
                MathematicalEdge { id: "e1".into(), source: "a".into(), target: "b".into() },
                MathematicalEdge { id: "e2".into(), source: "a".into(), target: "c".into() },
                MathematicalEdge { id: "e3".into(), source: "b".into(), target: "d".into() },
                MathematicalEdge { id: "e4".into(), source: "c".into(), target: "d".into() },
            ],
            algorithm: "topo".into(),
            algorithm_seed: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct MathematicalPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for MathematicalPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<MathematicalPoint> for (f64, f64) {
    fn from(point: MathematicalPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📐️ Geometry playground state: a point cloud for convex-hull/centroid demonstration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MathematicalGeometry {
    pub points: Vec<MathematicalPoint>,
}

impl Default for MathematicalGeometry {
    fn default() -> Self {
        Self {
            points: vec![(40.0, 220.0), (260.0, 40.0), (360.0, 140.0), (300.0, 260.0), (140.0, 300.0), (180.0, 160.0)]
                .into_iter()
                .map(MathematicalPoint::from)
                .collect(),
        }
    }
}

pub use crate::artifacts::mathematical::snapshot::schema::MathematicalSnapshot;
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
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
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
        let graph = MathematicalGraph::default();
        assert!(!graph.nodes.is_empty());
        assert!(!graph.edges.is_empty());
    }

    #[test]
    fn default_geometry_has_points() {
        assert!(!MathematicalGeometry::default().points.is_empty());
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::mathematical::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("MathematicalComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
