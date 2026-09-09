//! 🧮️ Equation artifact — the document entities this plugin's app edits: a graph playground
//! (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one snapshot.

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_equation_demo_tests;
extern crate semio_framework_schema as framework_schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<EquationMutation, EquationGraphWindowConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
// 🚚 Wave M3a (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS): `cas`/
// `polynomial` migrated verbatim from `🧮️math`'s crate root (files physically relocated under
// `🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`, the facet's
// Rust-only compute internals a named inference's `compute()` delegates into — mirrors stdio's
// `📐️step` io facet's `🪜️ladder`/`📐️part21`/`🧱️brep` precedent for deep Rust-only helper dirs under a
// facet). Mounted DIRECTLY at crate root, exactly as `🧮️math`'s own glue.rs mounted them — every
// `crate::cas::…`/`crate::polynomial::…` self-reference inside those two files (including references
// to non-`pub` inner modules, e.g. `crate::cas::canon`) is untouched, and privacy is structural in
// Rust: a re-export/alias layer (`pub use … as cas`) does NOT leak private inner items back out, so
// only a direct mount preserves them. `crate::algebra`'s two call sites became `math::algebra::MatG`/
// `math::algebra::VecG` against a since-removed `semio_framework_math as math` dependency — briefly a
// genuine pre-existing breakage (wave M3d had moved `algebra` out of `semio_framework_math` into
// `📸️remodel`, not knowing this wave had just created a second consumer here). Wave FIXALG (same
// ticket) relocated `VecG`/`MatG` out of `📸️remodel` into `semio_framework_number`'s own `algebra`
// module and repointed both sites at `number::MatG`/`number::VecG`, so `math` is no longer a
// dependency of this crate at all. `crate::number` became `number::` (wave MATHEND) against the
// `semio_framework_number` dependency below — `number` was relocated out of `🧮️math` into its own
// framework module, so it is a real top-level extern crate now, not a submodule of `math`.
#[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🦀️.rs"]
pub mod cas;
#[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs"]
pub mod polynomial;

#[cfg(test)]
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use std::sync::Arc;

//#region 🔖️Constants
/// 🗂️ The store envelope schema AND the plugin's registered document codec key — see
/// `crate::artifact`/`🚪️io/🦀️.rs::io`.
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

pub use crate::editor::equation::modes::edit::windows::graph::config::EquationCamera;
pub use crate::snapshot::schema::{EquationExprSnapshot, EquationFixture};
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
    let content_json = dsl::os_pack::json::to_json_string(&(graph.clone(), geometry.clone()));
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
    equation_scene_owner(snapshot).map_or_else(
        || EquationWorkingScene { graph: EquationGraph { directed: true, nodes: Vec::new(), edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }, geometry: EquationGeometry { points: Vec::new() } },
        |scene| (*scene).clone(),
    )
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
pub fn equation_snapshot_with_state(graph: &EquationGraph, geometry: &EquationGeometry) -> EquationSnapshot {
    let (notation, results, computed) = equation_children_from_state(graph, geometry);
    EquationSnapshot { notation, results, computed, equation: EquationExprSnapshot::default() }
}

/// 📥️ Rebuilds composed child handles and their exact local owner from a complete carrier fixture.
pub fn equation_snapshot_from_fixture(fixture: EquationFixture) -> EquationSnapshot {
    let mut snapshot = equation_snapshot_with_state(&fixture.graph, &fixture.geometry);
    snapshot.equation = fixture.equation;
    snapshot
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::equation::create_equation_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
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
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "equation.document",
                    extension: Some("equation"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("equation.document"),
                },
                dsl::LanguageSpec {
                    id: "equation.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("equation.op"),
                },
                dsl::LanguageSpec {
                    id: "equation.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(io::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(io::diff::text::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("equation.pack"),
                },
                dsl::LanguageSpec {
                    id: "equation.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
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
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
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
pub fn artifact<A: EquationApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.mathematical.equation").expect("canonical mathematical.equation kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait EquationApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::equation::EquationPlayApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::equation::EquationViewer>>>
{
}

impl<A> EquationApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::equation::EquationPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::equation::EquationViewer>>>
{
}

//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        // 🚚 Wave M3a: first real `impl InferredField<P>` in this codebase — see its own
                        // doc header for why (every other named inference documents using the plain
                        // whole-snapshot pattern instead).
                        #[path = "."]
                        pub mod roots {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌱roots/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        // 🚚 Wave M3a (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS):
                        // `🌿️cas-internals/`'s and `📈️polynomial-internals/`'s Rust-only compute code lives
                        // PHYSICALLY right here, under this facet — but is MOUNTED at crate root as `pub mod cas`/
                        // `pub mod polynomial` (see the top of this file), not nested under this module. Every
                        // `crate::cas::…`/`crate::polynomial::…` self-reference inside those two files is
                        // untouched from the original `🧮️math` crate, and privacy is structural in Rust: a
                        // `mod canon { … }` (non-`pub`) nested here would need `pub use component::*` to leak
                        // it back out, which does NOT re-export private items — `crate::cas::canon` would 404.
                        // Direct crate-root mounting is the only way to preserve every private inner `mod`
                        // unedited, so these two crates deliberately do NOT also appear as a submodule of
                        // `inferences` — see `🌿️cas-internals/🦀️.rs`'s doc header for the full story.
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod graph {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_graph_directed {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph-directed/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph-directed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph-directed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧭️change-graph-directed/🧪️tests/🧪️keeps-an-already-directed-graph-directed/🦀️.rs"]
                            mod tests_keeps_an_already_directed_graph_directed;
                        }
                        #[path = "."]
                        pub mod update_graph_algorithm {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/🧪️restates-the-unset-algorithm-and-its-absent-seed/🦀️.rs"]
                            mod tests_restates_the_unset_algorithm_and_its_absent_seed;
                        }
                        #[path = "."]
                        pub mod replace_graph {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/🧪️replays-the-identical-empty-graph/🦀️.rs"]
                            mod tests_replays_the_identical_empty_graph;
                        }
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➕️create-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➕️create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➕️create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➕️create-node/🧪️tests/🧪️rejects-a-duplicate-node-id/🦀️.rs"]
                            mod tests_rejects_a_duplicate_node_id;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/❌️delete-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/❌️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/❌️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/❌️delete-node/🧪️tests/🧪️rejects-deleting-a-node-that-is-not-in-the-graph/🦀️.rs"]
                            mod tests_rejects_deleting_a_node_that_is_not_in_the_graph;
                        }
                        #[path = "."]
                        pub mod delete_nodes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🧪️tests/🧪️rejects-a-bulk-delete-where-every-id-is-absent/🦀️.rs"]
                            mod tests_rejects_a_bulk_delete_where_every_id_is_absent;
                        }
                        #[path = "."]
                        pub mod change_node_label {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🏷️change-node-label/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🏷️change-node-label/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🏷️change-node-label/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🏷️change-node-label/🧪️tests/🧪️rejects-relabelling-a-node-that-is-not-in-the-graph/🦀️.rs"]
                            mod tests_rejects_relabelling_a_node_that_is_not_in_the_graph;
                        }
                        #[path = "."]
                        pub mod move_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🕹️move-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🕹️move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🕹️move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🕹️move-node/🧪️tests/🧪️rejects-moving-a-node-that-is-not-in-the-graph/🦀️.rs"]
                            mod tests_rejects_moving_a_node_that_is_not_in_the_graph;
                        }
                        #[path = "."]
                        pub mod connect_nodes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/🧪️tests/🧪️rejects-an-edge-between-two-absent-endpoints/🦀️.rs"]
                            mod tests_rejects_an_edge_between_two_absent_endpoints;
                        }
                        #[path = "."]
                        pub mod disconnect_nodes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/🧪️rejects-severing-an-edge-that-is-not-in-the-graph/🦀️.rs"]
                            mod tests_rejects_severing_an_edge_that_is_not_in_the_graph;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod geometry {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod replace_points {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🔄️replace-points/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🔄️replace-points/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🔄️replace-points/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🔄️replace-points/🧪️tests/🧪️replays-the-identical-empty-point-cloud/🦀️.rs"]
                            mod tests_replays_the_identical_empty_point_cloud;
                        }
                        #[path = "."]
                        pub mod insert_point {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/🧪️seeds-the-empty-cloud-with-its-first-point/🦀️.rs"]
                            mod tests_seeds_the_empty_cloud_with_its_first_point;
                        }
                        #[path = "."]
                        pub mod remove_point {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➖️remove-point/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➖️remove-point/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➖️remove-point/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/🧪️rejects-removing-a-point-from-an-empty-cloud/🦀️.rs"]
                            mod tests_rejects_removing_a_point_from_an_empty_cloud;
                        }
                        #[path = "."]
                        pub mod move_point {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🎯️move-point/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🎯️move-point/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🎯️move-point/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/🧪️rejects-moving-a-point-that-is-not-in-the-cloud/🦀️.rs"]
                            mod tests_rejects_moving_a_point_that_is_not_in_the_cloud;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod equation {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_coefficient {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/➗️equation/🧬️schema/🧬️mutations/🎚️change-coefficient/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/➗️equation/🧬️schema/🧬️mutations/🎚️change-coefficient/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/➗️equation/🧬️schema/🧬️mutations/🎚️change-coefficient/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/➗️equation/🧬️schema/🧬️mutations/🎚️change-coefficient/🧪️tests/🧪️raises-the-leading-coefficient-to-three-halves/🦀️.rs"]
                            mod tests_raises_the_leading_coefficient_to_three_halves;
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::io::mutations::text::*;
    pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
}
pub mod pack {
    pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::io::diff::text::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
    }
}
pub use crate::standards::v1::subsets::any::schema::diff::EquationDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::EquationMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod equation {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-algorithm/🦀️.rs"]
            pub mod set_algorithm;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️set-artifact/🦀️.rs"]
            pub mod set_artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️set-directed/🦀️.rs"]
            pub mod set_directed;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-points/🦀️.rs"]
            pub mod set_points;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️geometry/🦀️.rs"]
                    pub mod geometry;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod equation {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📐️geometry/🦀️.rs"]
                    pub mod geometry;
                }
            }
        }
    }
}
