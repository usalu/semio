//! 🧮️ Mathematical artifact — the document entities this plugin's app edits: a graph playground
//! (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one snapshot.

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

//#region 🔖️Constants
/// 🗂️ The store envelope schema AND the plugin's registered document codec key — see
/// `crate::artifacts::mathematical::artifact`/`🚪️io/🦀️component.rs::io`.
pub const MATH_DOCUMENT_SCHEMA: &str = "semio.mathematical/v1";

/// 🎯️ This artifact's dialect coordinate (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §1) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a viewer
/// file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.mathematical.mathematical"` matches this file's own `definition()`'s
/// `"s.mathematical.schema.artifact"` capability row descriptor AND the subset schema's own
/// `#[artifact_schema(id = "s.mathematical.mathematical")]`
/// (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`) — never guessed. `standard`/`subset`
/// match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location, i.e. the canonical surface id is
/// `s.mathematical.mathematical@1/*#editor` / `s.mathematical.mathematical@1/*#viewer`.
pub const MATHEMATICAL_DIALECT: Dialect = Dialect { artifact_kind: "s.mathematical.mathematical", standard: StandardId("1"), subset: SubsetId::ANY };
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
        Self { points: vec![(40.0, 220.0), (260.0, 40.0), (360.0, 140.0), (300.0, 260.0), (140.0, 300.0), (180.0, 160.0)].into_iter().map(MathematicalPoint::from).collect() }
    }
}

pub use crate::artifacts::mathematical::snapshot::schema::MathematicalSnapshot;
//#endregion 🔖️Document

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`mathematical→C:text,table,value`): the
/// graph playground's node labels, its node table (id/x/y), and everything else (direction,
/// algorithm+seed, edges, the geometry point cloud) are no longer inline `MathematicalSnapshot`
/// fields — they compose stdio's `s.stdio.semio.text`/`table`/`value` subsets as three fixed CHILD
/// slots (`notation`/`results`/`computed` on `MathematicalSnapshot`). The three converters below are
/// real and bidirectional: `mathematical_notation_from_graph`'s runs and
/// `mathematical_results_from_graph`'s rows are positionally aligned with `graph.nodes` (both are
/// always regenerated together from the SAME node order), so `mathematical_graph_geometry_from_children`
/// zips them back losslessly; `mathematical_computed_from_state` is a genuinely derived/computed
/// structure (never independently authored), documented honestly rather than pretending it is
/// user-editable prose.

//#region 🔖️ChildTypes
pub type MathematicalNotationChild = store::ArtifactChild<SemioTextSnapshot>;
pub type MathematicalResultsChild = store::ArtifactChild<SemioTableSnapshot>;
pub type MathematicalComputedChild = store::ArtifactChild<SemioValueSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 REAL bidirectional converter: node labels (prose/notation) <-> `text` runs, one run per node
/// in `graph.nodes` order.
pub fn mathematical_notation_from_graph(graph: &MathematicalGraph) -> SemioTextSnapshot {
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: graph.nodes.iter().map(|node| SemioTextRun { language: String::new(), content: node.label.clone(), marks: Vec::new() }).collect() }
}

/// 🌉 REAL bidirectional converter: node `id`/`x`/`y` (tabulated results) <-> `table` rows, one row
/// per node in `graph.nodes` order — positionally aligned with `mathematical_notation_from_graph`'s
/// runs (see this region's own doc comment).
pub fn mathematical_results_from_graph(graph: &MathematicalGraph) -> SemioTableSnapshot {
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "x".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "y".into(), kind: SemioTableCellKind::Float }],
        rows: graph.nodes.iter().map(|node| SemioTableRow { cells: vec![SemioValue::Str { value: node.id.clone() }, SemioValue::Float { lexeme: format!("{}", node.x) }, SemioValue::Float { lexeme: format!("{}", node.y) }] }).collect(),
    }
}

/// 🌉 REAL bidirectional converter: graph direction/algorithm/seed, edges, and the geometry point
/// cloud <-> one structured `value` Map — "scalar/structured computed values" per the migration
/// brief. Honestly a derived/computed structure, not independently-authored prose or a table.
pub fn mathematical_computed_from_state(graph: &MathematicalGraph, geometry: &MathematicalGeometry) -> SemioValueSnapshot {
    let edges = SemioValue::List {
        items: graph
            .edges
            .iter()
            .map(|edge| SemioValue::Map {
                entries: vec![
                    SemioValueEntry { key: "id".into(), value: SemioValue::Str { value: edge.id.clone() } },
                    SemioValueEntry { key: "source".into(), value: SemioValue::Str { value: edge.source.clone() } },
                    SemioValueEntry { key: "target".into(), value: SemioValue::Str { value: edge.target.clone() } },
                ],
            })
            .collect(),
    };
    let points = SemioValue::List {
        items: geometry
            .points
            .iter()
            .map(|point| SemioValue::Map {
                entries: vec![SemioValueEntry { key: "x".into(), value: SemioValue::Float { lexeme: format!("{}", point.x) } }, SemioValueEntry { key: "y".into(), value: SemioValue::Float { lexeme: format!("{}", point.y) } }],
            })
            .collect(),
    };
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map {
            entries: vec![
                SemioValueEntry { key: "directed".into(), value: SemioValue::Bool { value: graph.directed } },
                SemioValueEntry { key: "algorithm".into(), value: SemioValue::Str { value: graph.algorithm.clone() } },
                SemioValueEntry {
                    key: "algorithmSeed".into(),
                    value: match &graph.algorithm_seed {
                        Some(seed) => SemioValue::Str { value: seed.clone() },
                        None => SemioValue::Null,
                    },
                },
                SemioValueEntry { key: "edges".into(), value: edges },
                SemioValueEntry { key: "points".into(), value: points },
            ],
        },
        nodes: Vec::new(),
    }
}

/// 🌉 Inverse of the three converters above — real reconstruction, not a stub. `notation`/`results`
/// are expected to have the same length/order (always true for any triple this plugin itself
/// minted); a short/missing row or run degrades honestly (empty id/label, `0.0` coordinate) rather
/// than panicking, since an externally-composed mismatch is possible in principle.
pub fn mathematical_graph_geometry_from_children(notation: &SemioTextSnapshot, results: &SemioTableSnapshot, computed: &SemioValueSnapshot) -> (MathematicalGraph, MathematicalGeometry) {
    fn cell_str(row: &SemioTableRow, index: usize) -> String {
        match row.cells.get(index) {
            Some(SemioValue::Str { value }) => value.clone(),
            _ => String::new(),
        }
    }
    fn cell_f64(row: &SemioTableRow, index: usize) -> f64 {
        match row.cells.get(index) {
            Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    let nodes: Vec<MathematicalNode> =
        results.rows.iter().enumerate().map(|(i, row)| MathematicalNode { id: cell_str(row, 0), label: notation.runs.get(i).map(|run| run.content.clone()).unwrap_or_default(), x: cell_f64(row, 1), y: cell_f64(row, 2) }).collect();

    fn map_entries(value: &SemioValue) -> &[SemioValueEntry] {
        match value {
            SemioValue::Map { entries } => entries.as_slice(),
            _ => &[],
        }
    }
    fn find_entry<'v>(entries: &'v [SemioValueEntry], key: &str) -> Option<&'v SemioValue> {
        entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
    }
    fn value_f64(value: Option<&SemioValue>) -> f64 {
        match value {
            Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    let root_entries = map_entries(&computed.root);
    let directed = matches!(find_entry(root_entries, "directed"), Some(SemioValue::Bool { value: true }));
    let algorithm = match find_entry(root_entries, "algorithm") {
        Some(SemioValue::Str { value }) => value.clone(),
        _ => String::new(),
    };
    let algorithm_seed = match find_entry(root_entries, "algorithmSeed") {
        Some(SemioValue::Str { value }) => Some(value.clone()),
        _ => None,
    };
    let edges: Vec<MathematicalEdge> = match find_entry(root_entries, "edges") {
        Some(SemioValue::List { items }) => items
            .iter()
            .map(|item| {
                let entries = map_entries(item);
                MathematicalEdge {
                    id: match find_entry(entries, "id") {
                        Some(SemioValue::Str { value }) => value.clone(),
                        _ => String::new(),
                    },
                    source: match find_entry(entries, "source") {
                        Some(SemioValue::Str { value }) => value.clone(),
                        _ => String::new(),
                    },
                    target: match find_entry(entries, "target") {
                        Some(SemioValue::Str { value }) => value.clone(),
                        _ => String::new(),
                    },
                }
            })
            .collect(),
        _ => Vec::new(),
    };
    let points: Vec<MathematicalPoint> = match find_entry(root_entries, "points") {
        Some(SemioValue::List { items }) => items
            .iter()
            .map(|item| {
                let entries = map_entries(item);
                MathematicalPoint { x: value_f64(find_entry(entries, "x")), y: value_f64(find_entry(entries, "y")) }
            })
            .collect(),
        _ => Vec::new(),
    };
    (MathematicalGraph { directed, nodes, edges, algorithm, algorithm_seed }, MathematicalGeometry { points })
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side cache of the live `(graph, geometry)` state behind a triple of
/// composed-child handles — NEVER persisted (matches the `EngineRep` contract: wholly derived,
/// droppable at any instant, rebuilt from base). No `LinkResolver`/child-dispatch seam exists in
/// `ArtifactApp::handle` yet (checked directly against `🔌️plugin/🦀️component.rs`, same standing gap
/// every prior wave's report documents), so this is the only way a persisted content-addressed
/// handle round-trips to the real graph/geometry within one process — mirrors writer's
/// `WRITER_SCRATCH`/lowpoly's `mesh_workspace`, scaled to three co-derived children that always
/// share ONE scene id (a triple is always minted together from the same `(graph, geometry)` pair,
/// so `notation.child_id == results.child_id == computed.child_id`, and one cache entry serves all
/// three reads).
///
/// ⚠️ Same documented staleness gap as every prior exemplar: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, so a handle can in principle go uncached (fresh process, or an
/// undo past this session's history). `mathematical_graph`/`mathematical_geometry` fail soft
/// (empty graph/geometry) rather than panicking.
pub struct MathematicalWorkingScene {
    pub graph: MathematicalGraph,
    pub geometry: MathematicalGeometry,
}

thread_local! {
    static MATH_SCRATCH: RefCell<HashMap<String, MathematicalWorkingScene>> = RefCell::new(HashMap::new());
}

fn mathematical_scene_id(graph: &MathematicalGraph, geometry: &MathematicalGeometry) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(&(graph, geometry)).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("mathematical-scene-{:016x}", hasher.finish())
}

/// 🏗️ Mints all three composed-child handles for a `(graph, geometry)` pair AND seeds the scratch
/// cache in one call — the standard way every mutation-diff/fixture builder in this plugin creates
/// `notation`/`results`/`computed` field values; never construct these handles without also
/// caching, or `mathematical_graph`/`mathematical_geometry` will read back empty.
pub fn mathematical_children_from_state(graph: &MathematicalGraph, geometry: &MathematicalGeometry) -> (MathematicalNotationChild, MathematicalResultsChild, MathematicalComputedChild) {
    let scene_id = mathematical_scene_id(graph, geometry);
    MATH_SCRATCH.with(|cache| {
        cache.borrow_mut().insert(scene_id.clone(), MathematicalWorkingScene { graph: graph.clone(), geometry: geometry.clone() });
    });
    let dialect_for = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let target_for = |subset: &str| store::os_io::ArtifactRef { artifact_id: format!("mathematical-{subset}"), dialect: dialect_for(subset) };
    (store::ArtifactChild::new(scene_id.clone(), target_for("text")), store::ArtifactChild::new(scene_id.clone(), target_for("table")), store::ArtifactChild::new(scene_id, target_for("value")))
}

/// 🔎 Reads the cached working scene behind a snapshot's composed children — an empty graph/
/// geometry (never a panic) on a cache miss, per this region's own doc comment.
pub fn mathematical_scene(snapshot: &MathematicalSnapshot) -> MathematicalWorkingScene {
    MATH_SCRATCH
        .with(|cache| cache.borrow().get(&snapshot.results.child_id).map(|scene| MathematicalWorkingScene { graph: scene.graph.clone(), geometry: scene.geometry.clone() }))
        .unwrap_or_else(|| MathematicalWorkingScene { graph: MathematicalGraph { directed: true, nodes: Vec::new(), edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }, geometry: MathematicalGeometry { points: Vec::new() } })
}

/// 🔎 The live graph behind a snapshot's composed children — the single read call site every
/// render/inference/export/command path in this plugin now uses instead of the old `.graph` field.
pub fn mathematical_graph(snapshot: &MathematicalSnapshot) -> MathematicalGraph {
    mathematical_scene(snapshot).graph
}

/// 🔎 The live geometry behind a snapshot's composed children — twin of [`mathematical_graph`].
pub fn mathematical_geometry(snapshot: &MathematicalSnapshot) -> MathematicalGeometry {
    mathematical_scene(snapshot).geometry
}

/// 🏗️ Builds a full `MathematicalSnapshot` from a literal `(graph, geometry)` pair — the standard
/// fixture/import constructor replacing the old 2-field struct literal now that `notation`/
/// `results`/`computed` are composed child handles, not plain fields.
pub fn mathematical_snapshot_with_state(graph: MathematicalGraph, geometry: MathematicalGeometry) -> MathematicalSnapshot {
    let (notation, results, computed) = mathematical_children_from_state(&graph, &geometry);
    MathematicalSnapshot { notation, results, computed, equation: crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationSnapshot::default() }
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::mathematical::create_mathematical_app`'s `🔖️Manifest` region.
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
/// 📌️ ⚠️ DEAD (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): was the sole grammar/
/// protocol source for the OLD `declaration()` (`.languages(pilot_languages())`), deleted below in
/// the atomic cutover to `artifact()`/`.declare_artifact(...)`. Kept only as a documented historical
/// marker of why `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair { text: None,
/// binary: None }` in `🚪️io/🦀️component.rs::io()` is a deliberate scope-narrowing, not an
/// oversight — the five roles below are exactly the five `dsl::LanguageSpec`s that field still
/// needs wiring one day. Handcrafted facet grammars (text) and protocols (binary) for in-process
/// execution — built once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't
/// `const fn`, mirroring note's `pilot_languages()` convention.
#[allow(dead_code)]
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
                    grammar: Some(crate::artifacts::mathematical::io::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::mathematical::io::diff::text::COMPONENT_GRAMMAR_PATH),
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
/// `crate::editor::mathematical::config::schema::register_app_schema()` is the one exception, still called
/// from this file's own `.setup()`: it registers the `MathematicalPlayApp` CONFIG/PRESENCE schema, an
/// app-scope concern `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in the §6 artifact-scoped set.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.mathematical.standard.v1", "standard", "1", &[], None),
        ("s.mathematical.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.mathematical.schema.artifact", "schema", "s.mathematical.mathematical", &[("schema", "s.mathematical.mathematical")], None),
        ("s.mathematical.inference.artifact", "inference", "s.mathematical.mathematical.inference", &[("schema", "s.mathematical.mathematical.inference")], None),
        ("s.mathematical.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.mathematical.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.mathematical.grammar.document", "grammar", "mathematical.document", &[("grammar", "mathematical.document")], None),
        ("s.mathematical.grammar.op", "grammar", "mathematical.op", &[("grammar", "mathematical.op")], None),
        ("s.mathematical.grammar.diff", "grammar", "mathematical.diff", &[("grammar", "mathematical.diff")], None),
        ("s.mathematical.grammar.pack", "grammar", "mathematical.pack", &[("grammar", "mathematical.pack")], None),
        ("s.mathematical.grammar.spr", "grammar", "mathematical.spr", &[("grammar", "mathematical.spr")], None),
        ("s.mathematical.codec.document.v1", "codec", "semio.mathematical/v1:mathematical", &[("codec", "semio.mathematical/v1"), ("extension", "mathematical")], None),
        ("s.mathematical.localization.en", "localization", "Mathematical", &[], Some(("en", "Mathematical"))),
        ("s.mathematical.localization.de", "localization", "Mathematik", &[], Some(("de", "Mathematik"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.mathematical")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): the whole
/// `s.mathematical.mathematical` artifact through the declaration tree — one standard (`1`), one
/// subset (`any`). Replaces the OLD `declaration()`/`ArtifactDeclaration::builder(...)` channel
/// outright (atomic cutover; both channels never coexist — the OLD channel's `.composers(...)`
/// registered a native composer entry (`MathematicalComposerComposition`, writing the
/// non-canonical, under-qualified `Dialect{artifact_kind:"s.mathematical",...}` coordinate) with
/// NO matching `composer` capability row in `definition()` above — `definition()` only ever
/// declared composer capabilities for the two EXPORT directions (`s.stdio.md`/`s.stdio.json`), never
/// for this artifact's own native composer — which is exactly what shipped this plugin's WASM
/// manifest as `assembly-failed` (`try_build()`'s `runtime_capability_requirements`/capability-row
/// mismatch faulted every `try_build()` call). `declare_artifact`/`artifact()` do not run that same
/// composer-capability preflight at all — the new tree's io hops are typed `Serializer`/
/// `Deserializer` entries validated by `io_register`, not `ComposerEntry` capability rows — so this
/// cutover fixes the manifest as a side effect of deleting the broken channel, not a separate fix.
/// `localization: &[]` is a documented shortfall, not an oversight — the real en/de localized
/// descriptors still live on `definition()`'s `ArtifactCapability` rows above (kept per debt D1,
/// deleted repo-wide only in W6); wiring them into this field too is real follow-up work, not
/// required for the tree to register or for any law to hold (mirrors `🎬️sequence`'s and the stdio
/// pilot's own documented deviation).
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration {
        kind: ArtifactKindId::parse("s.mathematical.mathematical").expect("canonical mathematical.mathematical kind"),
        localization: &[],
        standards: vec![crate::artifacts::mathematical::standards::v1::standard()],
    }
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
