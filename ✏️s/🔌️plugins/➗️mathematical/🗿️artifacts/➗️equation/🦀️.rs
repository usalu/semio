//! 🧮️ Equation artifact — the document entities this plugin's app edits: a graph playground
//! (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one snapshot.

use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use std::sync::Arc;

//#region 🔖️Constants
/// 🗂️ The store envelope schema AND the plugin's registered document codec key — see
/// `crate::artifacts::equation::artifact`/`🚪️io/🦀️.rs::io`.
pub const MATH_DOCUMENT_SCHEMA: &str = "semio.equation/v1";

/// 🎯️ This artifact's dialect coordinate (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §1) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a viewer
/// file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.mathematical.equation"` matches this file's own `definition()`'s
/// `"s.mathematical.schema.artifact"` capability row descriptor AND the subset schema's own
/// `#[artifact_schema(id = "s.mathematical.equation")]`
/// (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`) — never guessed. `standard`/`subset`
/// match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location, i.e. the canonical surface id is
/// `s.mathematical.equation@1/*#editor` / `s.mathematical.equation@1/*#viewer`.
pub const EQUATION_DIALECT: Dialect = Dialect { artifact_kind: "s.mathematical.equation", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct EquationNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// 🔌️ JSON-facing edge — plain `source`/`target` id strings for the JS frontend's node-graph payloads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EquationEdge {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct EquationCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for EquationCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🕸️ Graph playground state: quadrant toggle, retained layout, and the active algorithm overlay.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EquationGraph {
    pub directed: bool,
    pub nodes: Vec<EquationNode>,
    pub edges: Vec<EquationEdge>,
    pub algorithm: String,
    #[value(default)]
    pub algorithm_seed: Option<String>,
}

impl Default for EquationGraph {
    fn default() -> Self {
        Self {
            directed: true,
            nodes: vec![
                EquationNode { id: "a".into(), label: "A".into(), x: 40.0, y: 60.0 },
                EquationNode { id: "b".into(), label: "B".into(), x: 240.0, y: 20.0 },
                EquationNode { id: "c".into(), label: "C".into(), x: 240.0, y: 180.0 },
                EquationNode { id: "d".into(), label: "D".into(), x: 440.0, y: 100.0 },
            ],
            edges: vec![
                EquationEdge { id: "e1".into(), source: "a".into(), target: "b".into() },
                EquationEdge { id: "e2".into(), source: "a".into(), target: "c".into() },
                EquationEdge { id: "e3".into(), source: "b".into(), target: "d".into() },
                EquationEdge { id: "e4".into(), source: "c".into(), target: "d".into() },
            ],
            algorithm: "topo".into(),
            algorithm_seed: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
pub struct EquationPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for EquationPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<EquationPoint> for (f64, f64) {
    fn from(point: EquationPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📐️ Geometry playground state: a point cloud for convex-hull/centroid demonstration.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct EquationGeometry {
    pub points: Vec<EquationPoint>,
}

impl Default for EquationGeometry {
    fn default() -> Self {
        Self { points: vec![(40.0, 220.0), (260.0, 40.0), (360.0, 140.0), (300.0, 260.0), (140.0, 300.0), (180.0, 160.0)].into_iter().map(EquationPoint::from).collect() }
    }
}

pub use crate::artifacts::equation::snapshot::schema::{EquationExprSnapshot, EquationFixture, EquationSnapshot};
//#endregion 🔖️Document

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`equation→C:text,table,value`): the
/// graph playground's node labels, its node table (id/x/y), and everything else (direction,
/// algorithm+seed, edges, the geometry point cloud) are no longer inline `EquationSnapshot`
/// fields — they compose stdio's `s.stdio.semio.text`/`table`/`value` subsets as three fixed CHILD
/// slots (`notation`/`results`/`computed` on `EquationSnapshot`). The three converters below are
/// real and bidirectional: `equation_notation_from_graph`'s runs and
/// `equation_results_from_graph`'s rows are positionally aligned with `graph.nodes` (both are
/// always regenerated together from the SAME node order), so `equation_graph_geometry_from_children`
/// zips them back losslessly; `equation_computed_from_state` is a genuinely derived/computed
/// structure (never independently authored), documented honestly rather than pretending it is
/// user-editable prose.

//#region 🔖️ChildTypes
pub type EquationNotationChild = store::ArtifactChild<SemioTextSnapshot>;
pub type EquationResultsChild = store::ArtifactChild<SemioTableSnapshot>;
pub type EquationComputedChild = store::ArtifactChild<SemioValueSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 REAL bidirectional converter: node labels (prose/notation) <-> `text` runs, one run per node
/// in `graph.nodes` order.
pub fn equation_notation_from_graph(graph: &EquationGraph) -> SemioTextSnapshot {
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: graph.nodes.iter().map(|node| SemioTextRun { language: String::new(), content: node.label.clone(), marks: Vec::new() }).collect() }
}

/// 🌉 REAL bidirectional converter: node `id`/`x`/`y` (tabulated results) <-> `table` rows, one row
/// per node in `graph.nodes` order — positionally aligned with `equation_notation_from_graph`'s
/// runs (see this region's own doc comment).
pub fn equation_results_from_graph(graph: &EquationGraph) -> SemioTableSnapshot {
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "x".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "y".into(), kind: SemioTableCellKind::Float }],
        rows: graph.nodes.iter().map(|node| SemioTableRow { cells: vec![SemioValue::Str { value: node.id.clone() }, SemioValue::Float { lexeme: format!("{}", node.x) }, SemioValue::Float { lexeme: format!("{}", node.y) }] }).collect(),
    }
}

/// 🌉 REAL bidirectional converter: graph direction/algorithm/seed, edges, and the geometry point
/// cloud <-> one structured `value` Map — "scalar/structured computed values" per the migration
/// brief. Honestly a derived/computed structure, not independently-authored prose or a table.
pub fn equation_computed_from_state(graph: &EquationGraph, geometry: &EquationGeometry) -> SemioValueSnapshot {
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
pub fn equation_graph_geometry_from_children(notation: &SemioTextSnapshot, results: &SemioTableSnapshot, computed: &SemioValueSnapshot) -> (EquationGraph, EquationGeometry) {
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
    let nodes: Vec<EquationNode> =
        results.rows.iter().enumerate().map(|(i, row)| EquationNode { id: cell_str(row, 0), label: notation.runs.get(i).map(|run| run.content.clone()).unwrap_or_default(), x: cell_f64(row, 1), y: cell_f64(row, 2) }).collect();

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
    let edges: Vec<EquationEdge> = match find_entry(root_entries, "edges") {
        Some(SemioValue::List { items }) => items
            .iter()
            .map(|item| {
                let entries = map_entries(item);
                EquationEdge {
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
    let points: Vec<EquationPoint> = match find_entry(root_entries, "points") {
        Some(SemioValue::List { items }) => items
            .iter()
            .map(|item| {
                let entries = map_entries(item);
                EquationPoint { x: value_f64(find_entry(entries, "x")), y: value_f64(find_entry(entries, "y")) }
            })
            .collect(),
        _ => Vec::new(),
    };
    (EquationGraph { directed, nodes, edges, algorithm, algorithm_seed }, EquationGeometry { points })
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral artifact-instance owner of the live `(graph, geometry)` materialization behind
/// one composed-child triple. All three handles minted for a snapshot retain the same immutable
/// owner; other snapshots and hostile identity reuse cannot observe or replace it. Wire and DSL
/// codecs omit the owner, so unresolved decoded handles fail soft until materialized.
#[derive(Clone)]
pub struct EquationWorkingScene {
    pub graph: EquationGraph,
    pub geometry: EquationGeometry,
}

fn equation_scene_id(graph: &EquationGraph, geometry: &EquationGeometry) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = pack::json::to_json_string(&(graph.clone(), geometry.clone()));
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("equation-scene-{:016x}", hasher.finish())
}

/// 🏗️ Mints all three composed-child handles for a `(graph, geometry)` pair and attaches one
/// shared immutable artifact-instance owner.
pub fn equation_children_from_state(graph: &EquationGraph, geometry: &EquationGeometry) -> (EquationNotationChild, EquationResultsChild, EquationComputedChild) {
    let scene_id = equation_scene_id(graph, geometry);
    let owner = Arc::new(EquationWorkingScene { graph: graph.clone(), geometry: geometry.clone() });
    let dialect_for = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let target_for = |subset: &str| store::os_io::ArtifactRef { artifact_id: format!("equation-{subset}"), dialect: dialect_for(subset) };
    (
        store::ArtifactChild::new(scene_id.clone(), target_for("text")).with_local_owner(owner.clone()),
        store::ArtifactChild::new(scene_id.clone(), target_for("table")).with_local_owner(owner.clone()),
        store::ArtifactChild::new(scene_id, target_for("value")).with_local_owner(owner),
    )
}

/// 🔎 Reads the exact artifact-instance scene behind a snapshot's composed children.
pub fn equation_scene(snapshot: &EquationSnapshot) -> EquationWorkingScene {
    equation_scene_owner(snapshot)
        .map(|scene| (*scene).clone())
        .unwrap_or_else(|| EquationWorkingScene { graph: EquationGraph { directed: true, nodes: Vec::new(), edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }, geometry: EquationGeometry { points: Vec::new() } })
}

/// 🧵 Retains the exact immutable scene owner for a resumable app operation.
pub fn equation_scene_owner(snapshot: &EquationSnapshot) -> Option<Arc<EquationWorkingScene>> {
    snapshot.results.local_owner::<EquationWorkingScene>()
}

/// 📤️ Requires the materialized scene for an export or another content-reading boundary.
pub fn require_equation_scene(snapshot: &EquationSnapshot) -> Result<Arc<EquationWorkingScene>, store::ArtifactChildMaterializationError> {
    snapshot.results.require_local_owner::<EquationWorkingScene>()
}

/// 📤️ Projects every equation field for a foreign carrier when the composed scene is present.
pub fn equation_fixture(snapshot: &EquationSnapshot) -> Result<EquationFixture, store::ArtifactChildMaterializationError> {
    let scene = require_equation_scene(snapshot)?;
    Ok(EquationFixture { graph: scene.graph.clone(), geometry: scene.geometry.clone(), equation: snapshot.equation.clone() })
}

/// 🔎 The live graph behind a snapshot's composed children — the single read call site every
/// render/inference/export/command path in this plugin now uses instead of the old `.graph` field.
pub fn equation_graph(snapshot: &EquationSnapshot) -> EquationGraph {
    equation_scene(snapshot).graph
}

/// 🔎 The live geometry behind a snapshot's composed children — twin of [`equation_graph`].
pub fn equation_geometry(snapshot: &EquationSnapshot) -> EquationGeometry {
    equation_scene(snapshot).geometry
}

/// 🏗️ Builds a full `EquationSnapshot` from a literal `(graph, geometry)` pair — the standard
/// fixture/import constructor replacing the old 2-field struct literal now that `notation`/
/// `results`/`computed` are composed child handles, not plain fields.
pub fn equation_snapshot_with_state(graph: EquationGraph, geometry: EquationGeometry) -> EquationSnapshot {
    let (notation, results, computed) = equation_children_from_state(&graph, &geometry);
    EquationSnapshot { notation, results, computed, equation: crate::artifacts::equation::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot::default() }
}

/// 📥️ Rebuilds composed child handles and their exact local owner from a complete carrier fixture.
pub fn equation_snapshot_from_fixture(fixture: EquationFixture) -> EquationSnapshot {
    let mut snapshot = equation_snapshot_with_state(fixture.graph, fixture.geometry);
    snapshot.equation = fixture.equation;
    snapshot
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::equation::create_equation_app`'s `🔖️Manifest` region.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.equation".into(),
        name: "Equation".into(),
        source_format: MATH_DOCUMENT_SCHEMA.into(),
        component_kind: "equation".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        schema: "computation.equation".into(),
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
/// binary: None }` in `🚪️io/🦀️.rs::io()` is a deliberate scope-narrowing, not an
/// oversight — the five roles below are exactly the five `dsl::LanguageSpec`s that field still
/// needs wiring one day. Handcrafted facet grammars (text) and protocols (binary) for in-process
/// execution — built once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't
/// `const fn`, mirroring note's `pilot_languages()` convention.
#[allow(dead_code)]
async fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "equation.document",
                    extension: Some("equation"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::equation::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::equation::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::equation::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::equation::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("equation.document"),
                },
                dsl::LanguageSpec {
                    id: "equation.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::equation::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::equation::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::equation::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::equation::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("equation.op"),
                },
                dsl::LanguageSpec {
                    id: "equation.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::equation::io::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::equation::io::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("equation.diff"),
                },
                dsl::LanguageSpec {
                    id: "equation.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::equation::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::equation::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("equation.pack"),
                },
                dsl::LanguageSpec {
                    id: "equation.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::equation::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::equation::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("equation.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from a
/// plugin `.setup()` callback (see `🗒️note`'s exemplar conversion, same shape).
/// `crate::editor::equation::config::schema::register_app_schema()` is the one exception, still called
/// from this file's own `.setup()`: it registers the `EquationPlayApp` CONFIG/PRESENCE schema, an
/// app-scope concern `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in the §6 artifact-scoped set.
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.mathematical.equation.standard.v1", "standard", "1", &[], None),
        ("s.mathematical.equation.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.mathematical.equation.schema.artifact", "schema", "s.mathematical.equation", &[("schema", "s.mathematical.equation")], None),
        ("s.mathematical.equation.inference.artifact", "inference", "s.mathematical.equation.inference", &[("schema", "s.mathematical.equation.inference")], None),
        ("s.mathematical.equation.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.mathematical.equation.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.mathematical.equation.grammar.document", "grammar", "equation.document", &[("grammar", "equation.document")], None),
        ("s.mathematical.equation.grammar.op", "grammar", "equation.op", &[("grammar", "equation.op")], None),
        ("s.mathematical.equation.grammar.diff", "grammar", "equation.diff", &[("grammar", "equation.diff")], None),
        ("s.mathematical.equation.grammar.pack", "grammar", "equation.pack", &[("grammar", "equation.pack")], None),
        ("s.mathematical.equation.grammar.spr", "grammar", "equation.spr", &[("grammar", "equation.spr")], None),
        ("s.mathematical.equation.codec.document.v1", "codec", "semio.equation/v1:equation", &[("codec", "semio.equation/v1"), ("codec-extension", "17:semio.equation/v1:equation")], None),
        ("s.mathematical.equation.localization.en", "localization", "Equation", &[], Some(("en", "Equation"))),
        ("s.mathematical.equation.localization.de", "localization", "Gleichung", &[], Some(("de", "Gleichung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.mathematical.equation")?);
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
/// `s.mathematical.equation` artifact through the declaration tree — one standard (`1`), one
/// subset (`any`). Replaces the OLD `declaration()`/`ArtifactDeclaration::builder(...)` channel
/// outright (atomic cutover; both channels never coexist — the OLD channel's `.composers(...)`
/// registered a native composer entry (`EquationComposerComposition`, writing the
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
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.mathematical.equation").expect("canonical mathematical.equation kind"), localization: &[], standards: vec![crate::artifacts::equation::standards::v1::standard()] }
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn scene(directed: bool) -> EquationWorkingScene {
        let mut graph = EquationGraph::default();
        graph.directed = directed;
        EquationWorkingScene { graph, geometry: EquationGeometry::default() }
    }

    async fn owned_snapshot(directed: bool) -> EquationSnapshot {
        let scene = scene(directed);
        equation_snapshot_with_state(scene.graph, scene.geometry)
    }

    fn replace_scene_owner(snapshot: &mut EquationSnapshot, scene: Arc<EquationWorkingScene>) {
        snapshot.notation.set_local_owner(scene.clone());
        snapshot.results.set_local_owner(scene.clone());
        snapshot.computed.set_local_owner(scene);
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "computation.equation");
        assert_eq!(MATH_DOCUMENT_SCHEMA, "semio.equation/v1");
    }

    #[semio_framework_async_macros::async_test]
    async fn default_graph_has_nodes_and_edges() {
        let graph = EquationGraph::default();
        assert!(!graph.nodes.is_empty());
        assert!(!graph.edges.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn default_geometry_has_points() {
        assert!(!EquationGeometry::default().points.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn carrier_fixture_contains_child_state_and_rejects_a_wire_only_parent() {
        let snapshot = owned_snapshot(true).await;
        let fixture = equation_fixture(&snapshot).expect("owned scene projects");
        assert_eq!(fixture.graph, equation_scene_owner(&snapshot).unwrap().graph);
        assert_eq!(fixture.geometry, EquationGeometry::default());

        let wire = snapshot.to_value();
        let decoded = EquationSnapshot::from_value(wire).expect("parent wire decodes");
        assert_eq!(equation_fixture(&decoded), Err(store::ArtifactChildMaterializationError::Absent));
    }

    #[semio_framework_async_macros::async_test]
    async fn scene_owner_fixture_proves_identity_isolation_aba_wire_omission_and_bounded_close() {
        let fixture: pack::json::Value = pack::json::parse(include_str!("🧪️fixtures/👑️equation-scene-owner-law.json")).expect("language-neutral equation scene fixture");
        let cases = fixture["cases"].as_array().expect("fixture cases");
        assert_eq!(fixture["schemaVersion"], 1);
        assert_eq!(fixture["ownedSlots"], 3);
        assert_eq!(cases.len(), fixture["maximumCases"].as_u64().expect("bounded maximum") as usize);
        assert_eq!(cases.len(), 5);

        for case in cases {
            let law = case["law"].as_str().expect("law");
            let left_directed = case["leftDirected"].as_bool().expect("leftDirected");
            let right_directed = case["rightDirected"].as_bool().expect("rightDirected");
            match law {
                "tripleIdentity" => {
                    let snapshot = owned_snapshot(left_directed).await;
                    let notation = snapshot.notation.local_owner::<EquationWorkingScene>().expect("notation owner");
                    let results = snapshot.results.local_owner::<EquationWorkingScene>().expect("results owner");
                    let computed = snapshot.computed.local_owner::<EquationWorkingScene>().expect("computed owner");
                    assert!(Arc::ptr_eq(&notation, &results) && Arc::ptr_eq(&results, &computed));
                    assert_eq!(Arc::strong_count(&notation), 6);
                }
                "instanceIsolation" => {
                    let left = owned_snapshot(left_directed).await;
                    let mut right = left.clone();
                    replace_scene_owner(&mut right, Arc::new(scene(right_directed)));
                    assert_eq!(left.results.child_id, right.results.child_id, "hostile identity collision is deliberate");
                    assert_eq!(equation_graph(&left).directed, left_directed);
                    assert_eq!(equation_graph(&right).directed, right_directed);
                }
                "abaIsolation" => {
                    let stale_a = owned_snapshot(left_directed).await;
                    let mut reused_identity_b = stale_a.clone();
                    replace_scene_owner(&mut reused_identity_b, Arc::new(scene(right_directed)));
                    assert_eq!(stale_a.computed.child_id, reused_identity_b.computed.child_id);
                    assert_eq!(equation_graph(&reused_identity_b).directed, right_directed);
                    drop(reused_identity_b);
                    assert_eq!(equation_graph(&stale_a).directed, left_directed);
                }
                "wireOmission" => {
                    let left = owned_snapshot(left_directed).await;
                    let mut right = left.clone();
                    replace_scene_owner(&mut right, Arc::new(scene(right_directed)));
                    let left_wire = left.to_value();
                    let right_wire = right.to_value();
                    assert_eq!(left_wire, right_wire, "local owners never alter the durable wire");
                    let decoded = EquationSnapshot::from_value(left_wire).expect("first-party codec decodes snapshot");
                    assert!(decoded.results.local_owner::<EquationWorkingScene>().is_none());
                }
                "boundedClose" => {
                    let snapshot = owned_snapshot(left_directed).await;
                    let retained = snapshot.results.local_owner::<EquationWorkingScene>().expect("retained owner");
                    let weak = Arc::downgrade(&retained);
                    assert_eq!(Arc::strong_count(&retained), fixture["ownedSlots"].as_u64().expect("owned slots") as usize + 1);
                    drop(snapshot);
                    assert_eq!(Arc::strong_count(&retained), 1);
                    drop(retained);
                    assert!(weak.upgrade().is_none());
                }
                other => panic!("unexpected equation scene law {other}"),
            }
        }
    }
}
//#endregion 🧪️Tests
