//! 🧠️ Wires artifact — the document entity this plugin's one app (🔌️wires) edits.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`reasoning/dag→C:graph`): the old inline
//! `board_fixture` field (a `DslValue` blob duplicating a neutral node/edge graph model) is replaced
//! by a composed `s.stdio.semio.graph` CHILD slot (`🔖️ContentBridge` below) — this plugin no longer
//! defines its own persisted node/edge graph model, it composes stdio's neutral `graph` subset
//! instead. `camera`/`meta` (pan/zoom view state, kind-catalog/allowed-identity config) are NOT part
//! of the neutral graph subset — they stay as their own small persisted `DslValue` fields on
//! `WiresSnapshot`, exactly as they always were, just no longer nested inside the now-gone
//! `board_fixture` blob. `wires_fixture`'s own shape (identities/relationships semantic layer, incl.
//! its pre-existing internal `board` mirror) is UNCHANGED by this migration — it's a separate,
//! narrower duplication concern this pass doesn't touch (see `📓️wave4-reports/reasoning-report.md`).
//! `⚙️engine`/`🖱️commands`/`🔧️op` still address board nodes/edges generically by id
//! (`array_mut`/`entity_id`/JSON-patch-style ops) via [`wires_working_board`], the single accessor
//! every call site that used to read `snapshot.board_fixture` now goes through.

extern crate infinite_canvas as infinite_board_port_directed;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
mod art_wires_demo_tests;
extern crate semio_framework_schema as framework_schema;

use dsl::DslValue;
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

//#region 🔖️Constants
/// 🪪️ This artifact's coordinate (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract
/// §2.1) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so `👁️viewer` can
/// read it without ever importing through the sibling editor module. `artifact_kind` matches
/// `#[artifact_schema(id = "s.reasoning.wires")]` on `WiresArtifact`
/// (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`); `standard`/`subset` match this
/// file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.reasoning.wires@1/*#editor` / `s.reasoning.wires@1/*#viewer`.
pub const WIRES_DIALECT: Dialect = Dialect { artifact_kind: "s.reasoning.wires", standard: StandardId("1"), subset: SubsetId::ANY };
pub use crate::schema::mutations::WiresMutation;

pub use crate::schema::diff::WiresDiff;

pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `puzzle.2d.fixture` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "reasoning.mindmap.fixture";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 📸️ Persisted wires snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::schema::snapshot::WiresSnapshot;
pub use crate::schema::WiresArtifact;
//#endregion 🔖️Types

//#region 🔖️EmptyFixtures
/// 📭️ Empty `reasoning.mindmap.fixture` board blob for tests and fresh documents.
pub fn empty_board_fixture() -> DslValue {
    DslValue::object([
        ("schema".into(), DslValue::String(MINDMAP_BOARD_SCHEMA.into())),
        ("camera".into(), DslValue::object([("x".into(), DslValue::float(0.0)), ("y".into(), DslValue::float(0.0)), ("zoom".into(), DslValue::float(1.0))])),
        ("nodes".into(), DslValue::Array(vec![])),
        ("edges".into(), DslValue::Array(vec![])),
        ("wires".into(), DslValue::Array(vec![])),
    ])
}

/// 📭️ Empty `reasoning.wires.fixture` blob for tests and fresh documents.
pub fn empty_wires_fixture() -> DslValue {
    DslValue::object([("schema".into(), DslValue::String(MINDMAP_WIRES_SCHEMA.into())), ("identities".into(), DslValue::Array(vec![])), ("relationships".into(), DslValue::Array(vec![])), ("board".into(), empty_board_fixture())])
}

/// 📭️ `{x:0, y:0, zoom:1}` — the default board camera, persisted as its own `WiresSnapshot.camera`
/// field (never part of the composed graph child — pan/zoom is app view state, not graph data).
pub fn empty_camera() -> DslValue {
    DslValue::object([("x".into(), DslValue::float(0.0)), ("y".into(), DslValue::float(0.0)), ("zoom".into(), DslValue::float(1.0))])
}

/// 📭️ Fresh wires snapshot with empty fixtures.
pub fn empty_wires_snapshot() -> WiresSnapshot {
    WiresSnapshot { wires_fixture: empty_wires_fixture(), content: wires_content_child_with_owner(Vec::new(), Vec::new()), camera: empty_camera(), meta: DslValue::Null }
}
//#endregion 🔖️EmptyFixtures

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.graph` document — the wires board's
/// nodes/edges now live in this composed child rather than inline on `WiresSnapshot`.
pub type WiresContentChild = store::ArtifactChild<SemioGraphSnapshot>;

use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🏷️ `wires.node` is the honest string boundary carrying the FULL raw board node `DslValue` (every
/// key a board node can dynamically carry — `nodeKind`/`shape`/`radius`/`width`/`height`/`text`/
/// `root`/`🐙️handles`/... — this app's board nodes are an untyped `DslValue` object, not a fixed Rust
/// struct, so no fixed field list could ever be exhaustive) as JSON. `id`/`label`(=`text`)/
/// `kind`(=`nodeKind`)/`position`(=`x`,`y`) are ALSO projected onto the composed `SemioGraphNode`'s
/// own native fields for genuine graph-shape tooling that only understands the neutral subset — but
/// the JSON blob is the round-trip SOURCE OF TRUTH on decode (matches `dag`'s own `dag.node`
/// precedent, `📓️wave4-reports/dag-report.md`).
const WIRES_NODE_JSON_PROPERTY: &str = "wires.node";

fn semio_node_from_board_node(node: &DslValue) -> SemioGraphNode {
    let (x, y) = crate::schema::node_position(node);
    SemioGraphNode {
        id: SemioGraphNodeId::new(crate::schema::entity_id(node, "id").unwrap_or("").to_string()),
        kind: node.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("").to_string(),
        label: node.get("text").and_then(|value| value.as_str()).unwrap_or("").to_string(),
        position: SemioPoint2 { x, y },
        ports: Vec::new(),
        properties: vec![SemioValueEntry { key: WIRES_NODE_JSON_PROPERTY.into(), value: SemioValue::Str { value: crate::schema::fixture_json_string(node) } }],
    }
}

/// 🌉 Inverse of [`semio_node_from_board_node`] — falls back to a minimal node built from the
/// graph-native `id`/`label`/`position` fields only if the property is missing (content authored
/// outside this plugin, e.g. by a hand-written `graph` doc) — never panics.
fn board_node_from_semio_node(node: &SemioGraphNode) -> DslValue {
    for property in &node.properties {
        if property.key == WIRES_NODE_JSON_PROPERTY {
            if let SemioValue::Str { value } = &property.value {
                if let Ok(restored) = dsl::os_pack::json::from_json_str::<DslValue>(value) {
                    return restored;
                }
            }
        }
    }
    DslValue::object([
        ("id".into(), DslValue::String(node.id.value.clone())),
        ("nodeKind".into(), DslValue::String(node.kind.clone())),
        ("shape".into(), DslValue::String("circle".into())),
        ("x".into(), DslValue::float(node.position.x)),
        ("y".into(), DslValue::float(node.position.y)),
        ("text".into(), DslValue::String(node.label.clone())),
        ("handles".into(), DslValue::Array(vec![])),
    ])
}

/// 🏷️ `SemioGraphEdge` has no `properties` slot (unlike `SemioGraphNode`) — its `label` field (which
/// this app's own board edges never populate on their own behalf) is repurposed to carry the FULL raw
/// board edge `DslValue` as JSON, the round-trip source of truth on decode. `source`/`target` are also
/// projected onto their native fields, and `kind` from `edgeKind` when present, for genuine
/// graph-shape tooling.
fn semio_edge_from_board_edge(edge: &DslValue) -> SemioGraphEdge {
    SemioGraphEdge {
        id: SemioGraphEdgeId::new(crate::schema::entity_id(edge, "id").unwrap_or("").to_string()),
        source: SemioGraphNodeId::new(edge.get("source").and_then(|value| value.as_str()).unwrap_or("").to_string()),
        target: SemioGraphNodeId::new(edge.get("target").and_then(|value| value.as_str()).unwrap_or("").to_string()),
        kind: edge.get("edgeKind").and_then(|value| value.as_str()).unwrap_or("").to_string(),
        label: crate::schema::fixture_json_string(edge),
    }
}

/// 🌉 Inverse of [`semio_edge_from_board_edge`] — falls back to a bare node-id edge if `label` isn't
/// valid JSON (content authored outside this plugin) — never panics.
fn board_edge_from_semio_edge(edge: &SemioGraphEdge) -> DslValue {
    if let Ok(restored) = dsl::os_pack::json::from_json_str::<DslValue>(&edge.label) {
        return restored;
    }
    DslValue::object([("id".into(), DslValue::String(edge.id.value.clone())), ("source".into(), DslValue::String(edge.source.value.clone())), ("target".into(), DslValue::String(edge.target.value.clone()))])
}

/// 🌉 REAL bidirectional converter between the app's live board node/edge `DslValue` editing state and
/// the composed child's own `SemioGraphSnapshot` node/edge graph (the "ModelBridge"/"DocumentBridge"
/// pattern from `📓️wave3-reports/cad-report.md` and `📓️wave4-reports/flow-report.md`/`dag-report.md`).
pub fn wires_content_snapshot_from_scene(nodes: &[DslValue], edges: &[DslValue]) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: nodes.iter().map(semio_node_from_board_node).collect(), edges: edges.iter().map(semio_edge_from_board_edge).collect() }
}

/// 🌉 Inverse of [`wires_content_snapshot_from_scene`].
pub fn scene_from_wires_content_snapshot(content: &SemioGraphSnapshot) -> (Vec<DslValue>, Vec<DslValue>) {
    (content.nodes.iter().map(board_node_from_semio_node).collect(), content.edges.iter().map(board_edge_from_semio_edge).collect())
}

/// 🕸️ Deterministic content-addressed CHILD handle for the wires board content — same
/// `(child_id, target)` for identical `(nodes, edges)`, a different pair once the content actually
/// changes; mirrors `dag`'s `dag_content_child_handle`/writer's `document_child_handle`.
pub fn wires_content_child_handle(nodes: &[DslValue], edges: &[DslValue]) -> WiresContentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = wires_content_snapshot_from_scene(nodes, edges);
    let content_json = dsl::os_pack::json::to_json_string(&snapshot);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("wires-content-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "graph".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "wires-content".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral representation of one composed child's live nodes and edges. The value is attached
/// to the exact `ArtifactChild`; it is never persisted, never global, and is retired with that owner.
#[derive(Clone, Debug, Default)]
pub struct WiresWorkingScene {
    pub nodes: Vec<DslValue>,
    pub edges: Vec<DslValue>,
}

/// 📝 Transfers a decoded or test-provided scene into one exact child owner.
pub fn materialize_wires_content(handle: &mut WiresContentChild, nodes: Vec<DslValue>, edges: Vec<DslValue>) {
    handle.set_local_owner(std::sync::Arc::new(WiresWorkingScene { nodes, edges }));
}

/// 🔎 Retains this exact child's typed working owner. A wire-only handle fails soft until the host
/// materializes its child document.
pub fn wires_working_scene_for_handle(handle: &WiresContentChild) -> WiresWorkingScene {
    handle.local_owner::<WiresWorkingScene>().map(|scene| scene.as_ref().clone()).unwrap_or_default()
}

/// 🔎 Reads the current document's live nodes/edges off its `content` child handle.
pub fn wires_working_scene(snapshot: &WiresSnapshot) -> WiresWorkingScene {
    wires_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints one content-addressed child and transfers its immutable working scene into that exact
/// local owner. No matching identity in another snapshot can observe the payload.
pub fn wires_content_child_with_owner(nodes: Vec<DslValue>, edges: Vec<DslValue>) -> WiresContentChild {
    let handle = wires_content_child_handle(&nodes, &edges);
    handle.with_local_owner(std::sync::Arc::new(WiresWorkingScene { nodes, edges }))
}

/// 🔎 Reconstructs the FULL legacy board-shaped `DslValue`
/// (`schema`/`camera`/`nodes`/`edges`/`meta`?/`wires`) from the working scene plus the snapshot's own
/// `camera`/`meta` fields — the single accessor every render/panel/command call site that used to read
/// `snapshot.board_fixture` directly now goes through. `meta` is omitted entirely when absent
/// (`DslValue::Null`), matching the old `BoardFixtureDsl.meta`'s `skip_serializing_if` behavior.
pub fn wires_working_board(snapshot: &WiresSnapshot) -> DslValue {
    let scene = wires_working_scene(snapshot);
    let mut entries: Vec<(String, DslValue)> =
        vec![("schema".into(), DslValue::String(MINDMAP_BOARD_SCHEMA.into())), ("camera".into(), snapshot.camera.clone()), ("nodes".into(), DslValue::Array(scene.nodes)), ("edges".into(), DslValue::Array(scene.edges))];
    if !matches!(snapshot.meta, DslValue::Null) {
        entries.push(("meta".into(), snapshot.meta.clone()));
    }
    entries.push(("wires".into(), DslValue::Array(vec![])));
    DslValue::Object(entries)
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::wires::create_wires_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "graph.wires".into(),
        name: "Wires Graph".into(),
        source_format: MINDMAP_WIRES_SCHEMA.into(),
        component_kind: "wires".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
        schema: MINDMAP_WIRES_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into(), "stdio.png".into(), "stdio.svg".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into(), "stdio.png".into(), "stdio.svg".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.reasoning.wires")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.reasoning.wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.reasoning.wires.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.reasoning.wires@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.reasoning.wires@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.csv")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.csv@rfc4180/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.csv@rfc4180/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"reasoning.wires.fixture:wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "reasoning.wires.fixture")?)?
                .claim(ArtifactIdentityClaim::codec_extension("reasoning.wires.fixture", "wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Mindmap Wires")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Mindmap Wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.reasoning.wires.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Mindmap-Wires")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Mindmap-Wires")?)?,
        )
}

/// 🗿️ New declaration-tree registration channel (ticket
/// `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1/§2) — the ONLY channel: the old
/// `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...)
/// .document_codec(...)` chain) is deleted outright, not kept alongside this, per the ticket's own
/// "Rejected approaches" ruling against dual registration. `localization: &[]` is a documented
/// shortfall: the real en/de localized names (`"Mindmap Wires"`/`"Mindmap-Wires"`) still live on
/// `definition()`'s `ArtifactCapability` rows above (kept, per debt D1) — wiring them into this
/// field is real follow-up work, not required for this pass (mirrors `📓️w4-sequence-report.md`
/// `## openQuestions` #2).
pub fn artifact<A: WiresApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.reasoning.wires").expect("canonical reasoning.wires kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait WiresApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::wires::ReasoningWiresPlayApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::wires::WiresViewer>>>
{
}

impl<A> WiresApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::wires::ReasoningWiresPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::wires::WiresViewer>>>
{
}

//#endregion 🔖️Declaration

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
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/🚫️rejects-a-node-id-be1d7d/🦀️.rs"]
                                    mod tests_rejects_a_node_id_the_board_already_holds;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/🚫️rejects-deleting-41bc08/🦀️.rs"]
                                    mod tests_rejects_deleting_a_node_the_board_never_held;
                                }
                                #[path = "."]
                                pub mod move_node {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭move-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭move-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭move-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭move-node/🧪️tests/📓️reports-a-no-op-when-ba77ae/🦀️.rs"]
                                    mod tests_reports_a_no_op_when_a_y_less_node_is_moved_to_y_zero;
                                }
                                #[path = "."]
                                pub mod resize_node {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐resize-node/🧪️tests/📖️reports-a-no-op-ce97eb/🦀️.rs"]
                                    mod tests_reports_a_no_op_when_the_radius_already_matches;
                                }
                                #[path = "."]
                                pub mod change_node_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-kind/🧪️tests/📖️reports-a-no-op-da417d/🦀️.rs"]
                                    mod tests_reports_a_no_op_when_the_kind_already_reads_topic;
                                }
                                #[path = "."]
                                pub mod change_node_shape {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷change-node-shape/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷change-node-shape/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷change-node-shape/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷change-node-shape/🧪️tests/📖️reports-a-no-op-adc55e/🦀️.rs"]
                                    mod tests_reports_a_no_op_when_the_shape_already_reads_circle;
                                }
                                #[path = "."]
                                pub mod edit_node_text {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/🧪️tests/🔤️reports-a-no-op-e94c5f/🦀️.rs"]
                                    mod tests_reports_a_no_op_when_the_label_is_retyped_verbatim;
                                }
                                #[path = "."]
                                pub mod set_node_root {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩set-node-root/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩set-node-root/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩set-node-root/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩set-node-root/🧪️tests/📓️reports-a-no-op-22ecc5/🦀️.rs"]
                                    mod tests_reports_a_no_op_when_an_unflagged_node_is_set_to_not_root;
                                }
                                #[path = "."]
                                pub mod connect_nodes {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️connect-nodes/🧪️tests/🚫️rejects-an-edge-6bdb01/🦀️.rs"]
                                    mod tests_rejects_an_edge_whose_source_node_is_absent;
                                }
                                #[path = "."]
                                pub mod disconnect_nodes {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/🚫️rejects-cutting-54b5a8/🦀️.rs"]
                                    mod tests_rejects_cutting_an_edge_the_board_never_carried;
                                }
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
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
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
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
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
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
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
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::io::diff::text::*;
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::diff::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::diff::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::mutations::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

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
    pub mod wires {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔵️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️add-relationship/🦀️.rs"]
            pub mod add_relationship;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👇️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
            pub mod canvas_pointer_move;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👆️canvas-pointer-up/🦀️.rs"]
            pub mod canvas_pointer_up;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚛️force-layout/🦀️.rs"]
            pub mod force_layout;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️reorganize/🦀️.rs"]
            pub mod reorganize;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🦀️.rs"]
                    pub mod canvas;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wires {
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️canvas/🦀️.rs"]
                    pub mod canvas;
                }
            }
        }
    }
}
