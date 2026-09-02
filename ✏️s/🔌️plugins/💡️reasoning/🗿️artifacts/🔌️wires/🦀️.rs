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
pub use crate::artifacts::wires::schema::mutations::WiresMutation;

pub use crate::artifacts::wires::schema::diff::WiresDiff;

pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `puzzle.2d.fixture` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "reasoning.mindmap.fixture";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 📸️ Persisted wires snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
pub use crate::artifacts::wires::schema::WiresArtifact;
//#endregion 🔖️Types

//#region 🔖️EmptyFixtures
/// 📭️ Empty `reasoning.mindmap.fixture` board blob for tests and fresh documents.
pub async fn empty_board_fixture() -> DslValue {
    DslValue::object([
        ("schema".into(), DslValue::String(MINDMAP_BOARD_SCHEMA.into())),
        ("camera".into(), DslValue::object([("x".into(), DslValue::float(0.0)), ("y".into(), DslValue::float(0.0)), ("zoom".into(), DslValue::float(1.0))])),
        ("nodes".into(), DslValue::Array(vec![])),
        ("edges".into(), DslValue::Array(vec![])),
        ("wires".into(), DslValue::Array(vec![])),
    ])
}

/// 📭️ Empty `reasoning.wires.fixture` blob for tests and fresh documents.
pub async fn empty_wires_fixture() -> DslValue {
    DslValue::object([("schema".into(), DslValue::String(MINDMAP_WIRES_SCHEMA.into())), ("identities".into(), DslValue::Array(vec![])), ("relationships".into(), DslValue::Array(vec![])), ("board".into(), empty_board_fixture())])
}

/// 📭️ `{x:0, y:0, zoom:1}` — the default board camera, persisted as its own `WiresSnapshot.camera`
/// field (never part of the composed graph child — pan/zoom is app view state, not graph data).
pub async fn empty_camera() -> DslValue {
    DslValue::object([("x".into(), DslValue::float(0.0)), ("y".into(), DslValue::float(0.0)), ("zoom".into(), DslValue::float(1.0))])
}

/// 📭️ Fresh wires snapshot with empty fixtures.
pub async fn empty_wires_snapshot() -> WiresSnapshot {
    WiresSnapshot { wires_fixture: empty_wires_fixture(), content: wires_content_child_with_owner(Vec::new(), Vec::new()), camera: empty_camera(), meta: DslValue::Null }
}
//#endregion 🔖️EmptyFixtures

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.graph` document — the wires board's
/// nodes/edges now live in this composed child rather than inline on `WiresSnapshot`.
pub type WiresContentChild = store::ArtifactChild<SemioGraphSnapshot>;

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId as SemioGraphEdgeId, GraphNodeId as SemioGraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

/// 🏷️ `wires.node` is the honest string boundary carrying the FULL raw board node `DslValue` (every
/// key a board node can dynamically carry — `nodeKind`/`shape`/`radius`/`width`/`height`/`text`/
/// `root`/`handles`/... — this app's board nodes are an untyped `DslValue` object, not a fixed Rust
/// struct, so no fixed field list could ever be exhaustive) as JSON. `id`/`label`(=`text`)/
/// `kind`(=`nodeKind`)/`position`(=`x`,`y`) are ALSO projected onto the composed `SemioGraphNode`'s
/// own native fields for genuine graph-shape tooling that only understands the neutral subset — but
/// the JSON blob is the round-trip SOURCE OF TRUTH on decode (matches `dag`'s own `dag.node`
/// precedent, `📓️wave4-reports/dag-report.md`).
const WIRES_NODE_JSON_PROPERTY: &str = "wires.node";

async fn semio_node_from_board_node(node: &DslValue) -> SemioGraphNode {
    let (x, y) = crate::artifacts::wires::schema::node_position(node);
    SemioGraphNode {
        id: SemioGraphNodeId::new(crate::artifacts::wires::schema::entity_id(node, "id").unwrap_or("").to_string()),
        kind: node.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("").to_string(),
        label: node.get("text").and_then(|value| value.as_str()).unwrap_or("").to_string(),
        position: SemioPoint2 { x, y },
        ports: Vec::new(),
        properties: vec![SemioValueEntry { key: WIRES_NODE_JSON_PROPERTY.into(), value: SemioValue::Str { value: crate::artifacts::wires::schema::fixture_json_string(node) } }],
    }
}

/// 🌉 Inverse of [`semio_node_from_board_node`] — falls back to a minimal node built from the
/// graph-native `id`/`label`/`position` fields only if the property is missing (content authored
/// outside this plugin, e.g. by a hand-written `graph` doc) — never panics.
async fn board_node_from_semio_node(node: &SemioGraphNode) -> DslValue {
    for property in &node.properties {
        if property.key == WIRES_NODE_JSON_PROPERTY {
            if let SemioValue::Str { value } = &property.value {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(value) {
                    if let Ok(restored) = dsl::to_dsl_value(&parsed) {
                        return restored;
                    }
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
async fn semio_edge_from_board_edge(edge: &DslValue) -> SemioGraphEdge {
    SemioGraphEdge {
        id: SemioGraphEdgeId::new(crate::artifacts::wires::schema::entity_id(edge, "id").unwrap_or("").to_string()),
        source: SemioGraphNodeId::new(edge.get("source").and_then(|value| value.as_str()).unwrap_or("").to_string()),
        target: SemioGraphNodeId::new(edge.get("target").and_then(|value| value.as_str()).unwrap_or("").to_string()),
        kind: edge.get("edgeKind").and_then(|value| value.as_str()).unwrap_or("").to_string(),
        label: crate::artifacts::wires::schema::fixture_json_string(edge),
    }
}

/// 🌉 Inverse of [`semio_edge_from_board_edge`] — falls back to a bare node-id edge if `label` isn't
/// valid JSON (content authored outside this plugin) — never panics.
async fn board_edge_from_semio_edge(edge: &SemioGraphEdge) -> DslValue {
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&edge.label) {
        if let Ok(restored) = dsl::to_dsl_value(&parsed) {
            return restored;
        }
    }
    DslValue::object([("id".into(), DslValue::String(edge.id.value.clone())), ("source".into(), DslValue::String(edge.source.value.clone())), ("target".into(), DslValue::String(edge.target.value.clone()))])
}

/// 🌉 REAL bidirectional converter between the app's live board node/edge `DslValue` editing state and
/// the composed child's own `SemioGraphSnapshot` node/edge graph (the "ModelBridge"/"DocumentBridge"
/// pattern from `📓️wave3-reports/cad-report.md` and `📓️wave4-reports/flow-report.md`/`dag-report.md`).
pub async fn wires_content_snapshot_from_scene(nodes: &[DslValue], edges: &[DslValue]) -> SemioGraphSnapshot {
    SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: nodes.iter().map(semio_node_from_board_node).collect(), edges: edges.iter().map(semio_edge_from_board_edge).collect() }
}

/// 🌉 Inverse of [`wires_content_snapshot_from_scene`].
pub async fn scene_from_wires_content_snapshot(content: &SemioGraphSnapshot) -> (Vec<DslValue>, Vec<DslValue>) {
    (content.nodes.iter().map(board_node_from_semio_node).collect(), content.edges.iter().map(board_edge_from_semio_edge).collect())
}

/// 🕸️ Deterministic content-addressed CHILD handle for the wires board content — same
/// `(child_id, target)` for identical `(nodes, edges)`, a different pair once the content actually
/// changes; mirrors `dag`'s `dag_content_child_handle`/writer's `document_child_handle`.
pub async fn wires_content_child_handle(nodes: &[DslValue], edges: &[DslValue]) -> WiresContentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = wires_content_snapshot_from_scene(nodes, edges);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
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
pub async fn wires_working_scene_for_handle(handle: &WiresContentChild) -> WiresWorkingScene {
    handle.local_owner::<WiresWorkingScene>().map(|scene| scene.as_ref().clone()).unwrap_or_default()
}

/// 🔎 Reads the current document's live nodes/edges off its `content` child handle.
pub async fn wires_working_scene(snapshot: &WiresSnapshot) -> WiresWorkingScene {
    wires_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints one content-addressed child and transfers its immutable working scene into that exact
/// local owner. No matching identity in another snapshot can observe the payload.
pub async fn wires_content_child_with_owner(nodes: Vec<DslValue>, edges: Vec<DslValue>) -> WiresContentChild {
    let handle = wires_content_child_handle(&nodes, &edges);
    handle.with_local_owner(std::sync::Arc::new(WiresWorkingScene { nodes, edges }))
}

/// 🔎 Reconstructs the FULL legacy board-shaped `DslValue`
/// (`schema`/`camera`/`nodes`/`edges`/`meta`?/`wires`) from the working scene plus the snapshot's own
/// `camera`/`meta` fields — the single accessor every render/panel/command call site that used to read
/// `snapshot.board_fixture` directly now goes through. `meta` is omitted entirely when absent
/// (`DslValue::Null`), matching the old `BoardFixtureDsl.meta`'s `skip_serializing_if` behavior.
pub async fn wires_working_board(snapshot: &WiresSnapshot) -> DslValue {
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
pub async fn artifact_kind() -> ArtifactKindSpec {
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
        export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md", "stdio.png", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    trait WiresChildOwnerOracle {
        fn expected() -> serde_json::Value;
    }

    struct SerdeJsonWiresChildOwnerOracle;

    impl WiresChildOwnerOracle for SerdeJsonWiresChildOwnerOracle {
        fn expected() -> serde_json::Value {
            serde_json::from_str(include_str!("🧪️fixtures/🎯️child-owner-isolation.json")).expect("language-neutral Wires child-owner fixture")
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_uses_the_wires_fixture_schema() {
        assert_eq!(artifact_kind().schema, MINDMAP_WIRES_SCHEMA);
        assert_eq!(artifact_kind().id, "graph.wires");
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_has_empty_fixtures() {
        let snapshot = empty_wires_snapshot();
        assert_eq!(snapshot.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(0));
        assert_eq!(wires_working_board(&snapshot).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(0));
    }

    /// 🧪️ Round-trip law: every board node/edge field survives `wires_content_snapshot_from_scene`
    /// → `scene_from_wires_content_snapshot`, including fields the neutral `SemioGraphNode`/
    /// `SemioGraphEdge` shape has no native slot for (`radius`/`root`/`edgeKind`/...).
    #[semio_framework_async_macros::async_test]
    async fn node_edge_content_round_trips_through_the_composed_child_snapshot() {
        let node = dsl::to_dsl_value(&serde_json::json!({
            "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 3.0, "y": 4.0,
            "radius": 24.0, "text": "Alpha", "root": true, "handles": []
        }))
        .unwrap();
        let edge = dsl::to_dsl_value(&serde_json::json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let content = wires_content_snapshot_from_scene(std::slice::from_ref(&node), std::slice::from_ref(&edge));
        assert_eq!(content.nodes.len(), 1);
        assert_eq!(content.nodes[0].id.value, "node-1");
        assert_eq!(content.nodes[0].label, "Alpha");
        assert_eq!(content.edges[0].source.value, "node-1");
        let (nodes, edges) = scene_from_wires_content_snapshot(&content);
        assert_eq!(nodes, vec![node]);
        assert_eq!(edges, vec![edge]);
    }

    #[semio_framework_async_macros::async_test]
    async fn content_child_handle_is_content_addressed_and_deterministic() {
        let node = dsl::to_dsl_value(&serde_json::json!({ "id": "a", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "text": "A", "handles": [] })).unwrap();
        let handle_a = wires_content_child_handle(std::slice::from_ref(&node), &[]);
        let handle_b = wires_content_child_handle(std::slice::from_ref(&node), &[]);
        assert_eq!(handle_a.child_id, handle_b.child_id, "same content must mint the same handle");
        let handle_c = wires_content_child_handle(&[], &[]);
        assert_ne!(handle_a.child_id, handle_c.child_id, "different content must mint a different handle");
    }

    #[semio_framework_async_macros::async_test]
    async fn wires_working_scene_is_owned_by_the_exact_snapshot_child() {
        let owned = wires_content_child_with_owner(Vec::new(), Vec::new());
        let wire = serde_json::to_vec(&owned).expect("Wires child wire identity");
        let reconstructed: WiresContentChild = serde_json::from_slice(&wire).expect("Wires child wire roundtrip");
        let observed = serde_json::json!({
            "ownedHasScene": owned.local_owner::<WiresWorkingScene>().is_some(),
            "wireIdentityMatches": owned == reconstructed,
            "wireHasScene": reconstructed.local_owner::<WiresWorkingScene>().is_some(),
        });

        assert_eq!(observed, SerdeJsonWiresChildOwnerOracle::expected());
    }
}
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.wires")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.reasoning.wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.reasoning.wires.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.reasoning.wires.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.wires@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.wires@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.composer.csv")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.csv@rfc4180/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.csv@rfc4180/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"reasoning.wires.fixture:wires")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "reasoning.wires.fixture")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Mindmap Wires")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Mindmap Wires")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.wires.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Mindmap-Wires")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Mindmap-Wires")?)?,
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
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.reasoning.wires").expect("canonical reasoning.wires kind"), localization: &[], standards: vec![crate::artifacts::wires::standards::v1::standard()] }
}
//#endregion 🔖️Declaration
