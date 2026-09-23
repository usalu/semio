//! 🧬️ Wires artifact schema — every field of the artifact with its state class.

use dsl::os_pack::json::Value;
use dsl::DslValue;
use framework_schema::ArtifactSchema;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Artifact
/// 🧬️ Shared wires artifact content.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresArtifact {
    #[state(artifact)]
    pub wires_fixture: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: crate::WiresContentChild,
    #[state(artifact)]
    pub meta: DslValue,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WiresArtifact {
    fn default() -> Self {
        Self { wires_fixture: crate::empty_wires_fixture(), content: crate::wires_content_child_with_owner(Vec::new(), Vec::new()), meta: DslValue::Null }
    }
}

impl WiresArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::WiresSnapshot {
        crate::WiresSnapshot { wires_fixture: self.wires_fixture.clone(), content: self.content.clone(), meta: self.meta.clone() }
    }

    /// 🧬️ Builds the shared artifact from its document snapshot.
    pub fn from_snapshot(snapshot: crate::WiresSnapshot) -> Self {
        Self { wires_fixture: snapshot.wires_fixture, content: snapshot.content, meta: snapshot.meta }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::WiresSnapshot) {
        self.wires_fixture = snapshot.wires_fixture;
        self.content = snapshot.content;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.reasoning.wires` — twenty handcrafted schema leaves.
pub fn wires_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.reasoning.wires",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️Construction
/// 🏗️ Hand-rolled `ArtifactBuilder` — the generic `semio_framework_plugin::app::SnapshotBuilder<S, M>`
/// (ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`, `📓️w4-sequence-report.md`'s
/// zero-boilerplate replacement) does NOT fit here: its `ArtifactBuilder` impl requires `S: Default`,
/// and `WiresSnapshot` deliberately has none — `content` is a composed `ArtifactChild` that needs a
/// freshly-minted, content-addressed handle (`empty_wires_snapshot()`'s
/// `wires_content_child_with_owner`), not a blanket zero value. This is the same class of
/// "the generic doesn't fit, keep the hand-rolled type" finding as `📓️w4-sequence-report.md`
/// `## recipeGaps` #1 (there for `ArtifactInferrer`, here for `ArtifactBuilder`). No current caller
/// exercises `ArtifactBuilder` for this subset (confirmed: zero references outside this module,
/// `derive_artifact_facets!`'s deleted generated wrapper, and the deleted `io_registry`) — kept as
/// real, correctly-typed SDK equipment matching the fan-out's established `Construction` convention,
/// not dead API (mirrors how `SurfaceDeclaration.mutation_roster` is kept unread, per debt tracked in
/// `📓️w1-c-report.md` openQuestion 3).
pub mod derived_construction {
    use crate::schema::diff::WiresDiff;
    use crate::schema::mutations::WiresMutation;
    use crate::schema::snapshot::WiresSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug)]
    pub struct WiresBuilderConstruction {
        snapshot: WiresSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for WiresBuilderConstruction {
        type Snapshot = WiresSnapshot;
        type Mutation = WiresMutation;
        type Diff = WiresDiff;
        fn empty() -> Self {
            Self { snapshot: crate::empty_wires_snapshot(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️Construction

//#region 🔖️DocumentHelpers
/// 🧬️ Pure helpers over `DslValue`-shaped documents — dissolved from the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every fn here is generic over document shape
/// (never `WiresSnapshot`, never an app type) so it has no home more specific than the artifact schema.
/// Reads that DO take `&WiresSnapshot` (`find_board_node`/`find_board_edge`/`find_relationship`) live
/// in `💡️inferences/` instead — see that file's `🔖️LookupHelpers` region.
pub fn array_mut<'a>(fixture: &'a mut DslValue, key: &str) -> &'a mut Vec<DslValue> {
    if !matches!(fixture, DslValue::Object(_)) {
        *fixture = DslValue::Object(vec![]);
    }
    let DslValue::Object(entries) = fixture else {
        unreachable!("fixture coerced to object above");
    };
    if let Some(idx) = entries.iter().position(|(entry_key, _)| entry_key == key) {
        let value = &mut entries[idx].1;
        if !matches!(value, DslValue::Array(_)) {
            *value = DslValue::Array(vec![]);
        }
        match value {
            DslValue::Array(items) => items,
            _ => unreachable!("array coerced above"),
        }
    } else {
        entries.push((key.to_string(), DslValue::Array(vec![])));
        match &mut entries.last_mut().expect("just pushed").1 {
            DslValue::Array(items) => items,
            _ => unreachable!("just pushed array"),
        }
    }
}

pub fn entity_id<'a>(entity: &'a DslValue, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

/// 🔢️ `identityId`/`sourceIdentityId`/`targetIdentityId` (and similar numeric-id fields) read as a
/// whole `u64` regardless of whether the source JSON number is an integer or a float literal. Fixtures
/// round-tripped through the `.wires` DSL text arrive as exact JSON integers (`Number(1)`, see
/// `IdentityDsl`/`RelationshipDsl`'s plain `u64` fields), so this fallback stays for documents built or
/// patched outside that DSL path (e.g. hand-constructed `Value` fixtures), where nothing enforces the
/// integer representation.
pub fn dsl_id(value: Option<&DslValue>) -> Option<u64> {
    value.and_then(|value| value.as_f64().map(|float| float as u64))
}

pub fn dsl_to_json(value: &DslValue) -> Value {
    dsl::os_pack::json::from_dsl_value(value)
}

pub fn fixture_json_string(fixture: &DslValue) -> String {
    dsl::os_pack::json::to_json_string(fixture)
}

pub fn fixture_camera(fixture: &DslValue) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
}

pub fn fixture_nodes(fixture: &DslValue) -> &[DslValue] {
    fixture.get("nodes").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn fixture_edges(fixture: &DslValue) -> &[DslValue] {
    fixture.get("edges").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_identities(wires: &DslValue) -> &[DslValue] {
    wires.get("identities").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_relationships(wires: &DslValue) -> &[DslValue] {
    wires.get("relationships").and_then(|value| value.as_array()).unwrap_or(&[])
}

/// 📐️ A JSON node's position, defaulting missing coordinates to the origin.
pub fn node_position(node: &DslValue) -> (f64, f64) {
    (node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0), node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0))
}

/// ⭕️ Radius a board node without an explicit one is drawn with — the value `addNode` mints.
pub const WIRES_DEFAULT_NODE_RADIUS: f64 = 24.0;

/// 📐️ A board node's drawn box `(x, y, width, height)`: its position is the box's top-left corner, its
/// extent the explicit `width`/`height` or else the circle's diameter.
pub fn node_box(node: &DslValue) -> (f64, f64, f64, f64) {
    let (x, y) = node_position(node);
    let diameter = 2.0 * node.get("radius").and_then(|value| value.as_f64()).unwrap_or(WIRES_DEFAULT_NODE_RADIUS);
    (x, y, node.get("width").and_then(|value| value.as_f64()).unwrap_or(diameter), node.get("height").and_then(|value| value.as_f64()).unwrap_or(diameter))
}

/// 🧾️ Keys the framework `Canvas2dScene` layer record gives its own meaning (`CanvasLayerRecord`). A board
/// field of the same name must not ride into a layer: a node's `text` STRING read as the record's
/// `text: {content, size}` routes every node to the scene-node painter, which then draws nothing
/// (measured live 2026-09-23 on the 05:13 activation: seven circle layers, zero arcs).
const CANVAS_LAYER_RESERVED_KEYS: &[&str] = &[
    "kind", "role", "utility", "name", "color", "selected", "width", "height", "x0", "y0", "x1", "y1", "dataUrl", "points", "seams", "base", "transform", "segments", "fill", "stroke", "opacity", "blendMode", "fillRule", "visible", "text", "image",
];

/// 🖼️ Projects the board into the framework `Canvas2dScene` layer contract (`kind`, box `x/y/width/
/// height`, `name`; lines as `x0/y0/x1/y1`) that the editor and viewer canvases both render.
///
/// 🐛️ Both canvases used to hand the host the RAW board records — circles carrying only a `radius`
/// and edges carrying only `source`/`target` ids. The host draws a box from `x/y/width/height` and a
/// line from `x0/y0/x1/y1`, so every node and edge of the reasoning-wires pane was skipped and the
/// canvas showed nothing but its grid (measured live 2026-09-23: seven parsed node layers, zero
/// shapes drawn). Every other board field rides along untouched, so hit ids and positions still read
/// exactly as the document stores them; the node's `text` becomes the record's `name`.
pub fn wires_canvas_layers(board: &DslValue, wires: &DslValue) -> Vec<Value> {
    let nodes = fixture_nodes(board);
    let centre = |id: &str| nodes.iter().find(|node| entity_id(node, "id") == Some(id)).map(|node| {
        let (x, y, width, height) = node_box(node);
        (x + width * 0.5, y + height * 0.5)
    });
    let relationship_kind = |edge_id: &str| wires_relationships(wires).iter().find(|relationship| entity_id(relationship, "edgeId") == Some(edge_id)).and_then(|relationship| relationship.get("kind")).cloned();
    let line = |id: &str, source: (f64, f64), target: (f64, f64), name: Option<DslValue>| {
        let mut entries = vec![
            ("id".to_string(), DslValue::String(id.to_string())),
            ("kind".to_string(), DslValue::String("line".into())),
            ("x0".to_string(), DslValue::float(source.0)),
            ("y0".to_string(), DslValue::float(source.1)),
            ("x1".to_string(), DslValue::float(target.0)),
            ("y1".to_string(), DslValue::float(target.1)),
        ];
        entries.extend(name.map(|name| ("name".to_string(), name)));
        dsl_to_json(&DslValue::Object(entries))
    };
    let mut layers: Vec<Value> = nodes
        .iter()
        .map(|node| {
            let (_, _, width, height) = node_box(node);
            let mut entries: Vec<(String, DslValue)> = match node {
                DslValue::Object(entries) => entries.iter().filter(|(key, _)| !CANVAS_LAYER_RESERVED_KEYS.contains(&key.as_str())).cloned().collect(),
                _ => Vec::new(),
            };
            let kind = if node.get("shape").and_then(|value| value.as_str()) == Some("circle") { "circle" } else { "rect" };
            entries.push(("kind".to_string(), DslValue::String(kind.into())));
            entries.push(("width".to_string(), DslValue::float(width)));
            entries.push(("height".to_string(), DslValue::float(height)));
            entries.extend(node.get("text").cloned().map(|text| ("name".to_string(), text)));
            dsl_to_json(&DslValue::Object(entries))
        })
        .collect();
    for edge in fixture_edges(board) {
        let (Some(id), Some(source), Some(target)) = (entity_id(edge, "id"), entity_id(edge, "source").and_then(|id| centre(id)), entity_id(edge, "target").and_then(|id| centre(id))) else { continue };
        layers.push(line(id, source, target, relationship_kind(id)));
    }
    let identity_node = |identity: Option<u64>| wires_identities(wires).iter().find(|row| dsl_id(row.get("identityId")) == identity && identity.is_some()).and_then(|row| entity_id(row, "nodeId")).and_then(|id| centre(id));
    for relationship in wires_relationships(wires) {
        let Some(edge_id) = entity_id(relationship, "edgeId").filter(|id| !id.is_empty()) else { continue };
        if fixture_edges(board).iter().any(|edge| entity_id(edge, "id") == Some(edge_id)) {
            continue;
        }
        if let (Some(source), Some(target)) = (identity_node(dsl_id(relationship.get("sourceIdentityId"))), identity_node(dsl_id(relationship.get("targetIdentityId")))) {
            layers.push(line(edge_id, source, target, relationship.get("kind").cloned()));
        }
    }
    layers
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ExampleFixture
/// 📄️ The `metabolism` example, parsed from `crate::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT`.
/// The committed asset IS the example — the only content `setActiveExample`, the `.example` manifest
/// registration and every metabolism test ever see. It used to be a stub envelope (an empty board
/// plus one "Demo" node) that a hand-built in-code graph silently stood in for whenever the parse
/// yielded fewer than seven nodes, so the play pane, which loads the asset itself, rendered an empty
/// canvas while every unit test saw the seven-node graph. The fallback is gone and the asset carries
/// the real graph (regenerated with this crate's own `ArtifactDsl::print_dsl`).
pub fn metabolism_wires_example_snapshot() -> protocol::MutationApplyResult<crate::WiresSnapshot> {
    <crate::WiresSnapshot as store::ArtifactDsl>::parse_dsl(crate::document_dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT)
        .map_err(|error| protocol::MutationApplyError::new("example.unparsable", format!("the committed metabolism example must parse: {error:?}")))
}

//#endregion 🔖️ExampleFixture
