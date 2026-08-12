//! 🧮️ Mathematical artifact — the document entities this plugin's app edits: a graph playground
//! (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one snapshot.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🗂️ The store envelope schema AND the plugin's registered document codec key — see
/// `crate::artifacts::mathematical::declaration`.
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

//#region 🔖️Declaration
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring note's
/// `pilot_languages()` convention. Sole caller is `declaration()` below (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "mathematical.document",
                    extension: Some("mathematical"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::mathematical::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::mathematical::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::mathematical::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::mathematical::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("mathematical.document"),
                },
                dsl::LanguageSpec {
                    id: "mathematical.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::mathematical::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::mathematical::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("mathematical.op"),
                },
                dsl::LanguageSpec {
                    id: "mathematical.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::mathematical::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::mathematical::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("mathematical.diff"),
                },
                dsl::LanguageSpec {
                    id: "mathematical.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::mathematical::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::mathematical::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("mathematical.pack"),
                },
                dsl::LanguageSpec {
                    id: "mathematical.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("mathematical.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from a
/// plugin `.setup()` callback (see `🗒️note`'s exemplar conversion, same shape).
/// `crate::apps::mathematical::config::schema::register_app_schema()` is the one exception, still called
/// from this file's own `.setup()`: it registers the `MathematicalPlayApp` CONFIG/PRESENCE schema, an
/// app-scope concern `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in the §6 artifact-scoped set.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.mathematical")
        .schema(crate::artifacts::mathematical::schema::mathematical_artifact_schema_descriptor())
        .inferences([crate::artifacts::mathematical::standards::v1::subsets::any::schema::inferences::mathematical_artifact_inference_descriptor()])
        .composers(crate::artifacts::mathematical::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::mathematical::MathematicalPlayApp>()
        .build()
}
//#endregion 🔖️Declaration

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
    use crate::artifacts::mathematical::standards::v1::subsets::any::io::io_registry as v1;

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
