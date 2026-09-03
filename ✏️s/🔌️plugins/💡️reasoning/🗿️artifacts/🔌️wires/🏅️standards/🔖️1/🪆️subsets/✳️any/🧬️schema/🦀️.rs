//! 🧬️ Wires artifact schema — every field of the artifact with its state class.

use dsl::DslValue;
use dsl::os_pack::json::Value;
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full wires artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresArtifact {
    #[state(artifact)]
    pub wires_fixture: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: crate::artifacts::wires::WiresContentChild,
    #[state(artifact)]
    pub camera: DslValue,
    #[state(artifact)]
    pub meta: DslValue,
    #[state(artifact)]
    pub drag_node_id: Option<String>,
    #[state(artifact)]
    pub drag_last_x: f64,
    #[state(artifact)]
    pub drag_last_y: f64,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WiresArtifact {
    fn default() -> Self {
        Self {
            wires_fixture: crate::artifacts::wires::empty_wires_fixture(),
            content: crate::artifacts::wires::wires_content_child_with_owner(Vec::new(), Vec::new()),
            camera: crate::artifacts::wires::empty_camera(),
            meta: DslValue::Null,
            drag_node_id: None,
            drag_last_x: 0.0,
            drag_last_y: 0.0,
            locale: "en-US".into(),
        }
    }
}

impl WiresArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> crate::artifacts::wires::WiresSnapshot {
        crate::artifacts::wires::WiresSnapshot { wires_fixture: self.wires_fixture.clone(), content: self.content.clone(), camera: self.camera.clone(), meta: self.meta.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: crate::artifacts::wires::WiresSnapshot) -> Self {
        Self { wires_fixture: snapshot.wires_fixture, content: snapshot.content, camera: snapshot.camera, meta: snapshot.meta, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: crate::artifacts::wires::WiresSnapshot) {
        self.wires_fixture = snapshot.wires_fixture;
        self.content = snapshot.content;
        self.camera = snapshot.camera;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.reasoning.wires` — twenty handcrafted schema leaves.
pub async fn wires_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.reasoning.wires",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
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
    use crate::artifacts::wires::schema::diff::WiresDiff;
    use crate::artifacts::wires::schema::mutations::WiresMutation;
    use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
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
        async fn empty() -> Self {
            Self { snapshot: crate::artifacts::wires::empty_wires_snapshot(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
pub async fn array_mut<'a>(fixture: &'a mut DslValue, key: &str) -> &'a mut Vec<DslValue> {
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

pub async fn entity_id<'a>(entity: &'a DslValue, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

/// 🔢️ `identityId`/`sourceIdentityId`/`targetIdentityId` (and similar numeric-id fields) read as a
/// whole `u64` regardless of whether the source JSON number is an integer or a float literal. Fixtures
/// round-tripped through the `.wires` DSL text arrive as exact JSON integers (`Number(1)`, see
/// `IdentityDsl`/`RelationshipDsl`'s plain `u64` fields), so this fallback stays for documents built or
/// patched outside that DSL path (e.g. hand-constructed `Value` fixtures), where nothing enforces the
/// integer representation.
pub async fn dsl_id(value: Option<&DslValue>) -> Option<u64> {
    value.and_then(|value| value.as_f64().map(|float| float as u64))
}

pub async fn dsl_to_json(value: &DslValue) -> Value {
    dsl::os_pack::json::from_dsl_value(value)
}

pub async fn fixture_json_string(fixture: &DslValue) -> String {
    dsl::os_pack::json::to_json_string(fixture)
}

pub async fn fixture_camera(fixture: &DslValue) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
}

pub async fn fixture_nodes(fixture: &DslValue) -> &[DslValue] {
    fixture.get("nodes").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub async fn fixture_edges(fixture: &DslValue) -> &[DslValue] {
    fixture.get("edges").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub async fn wires_identities(wires: &DslValue) -> &[DslValue] {
    wires.get("identities").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub async fn wires_relationships(wires: &DslValue) -> &[DslValue] {
    wires.get("relationships").and_then(|value| value.as_array()).unwrap_or(&[])
}

/// 📐️ A JSON node's position, defaulting missing coordinates to the origin.
pub async fn node_position(node: &DslValue) -> (f64, f64) {
    (node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0), node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0))
}

/// 🕸️ Re-lays out the board with the neutral `infinite_board_port_directed` force-graph solver — the
/// same shared mechanism `puzzle/2d`'s `forceLayout`/`reorganize` uses, depended on directly rather
/// than through puzzle's app program (mindmap's board schema is on its allowlist).
pub async fn force_layout_board(board: &mut DslValue) {
    let Ok(layout_json) = infinite_board_port_directed::apply_force_graph_layout_to_fixture_v1_json(&fixture_json_string(board), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = dsl::os_pack::json::from_json_str::<DslValue>(&layout_json) {
        *board = parsed;
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ExampleFixture
/// 📄️ The `metabolism` example, parsed once from `crate::artifacts::wires::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT`
/// — falls back to the empty document if the fixture ever fails to parse.
pub async fn metabolism_wires_example_snapshot() -> protocol::MutationApplyResult<crate::artifacts::wires::WiresSnapshot> {
    match <crate::artifacts::wires::WiresSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::wires::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT) {
        Ok(snapshot) if fixture_nodes(&crate::artifacts::wires::wires_working_board(&snapshot)).len() >= 7 => Ok(snapshot),
        _ => handcrafted_metabolism_snapshot(),
    }
}

/// 🧪️ Hand-built metabolism demo when the bundled `.dsl.semio` asset is still a stub envelope.
async fn handcrafted_metabolism_snapshot() -> protocol::MutationApplyResult<crate::artifacts::wires::WiresSnapshot> {
    let mut snapshot = crate::artifacts::wires::empty_wires_snapshot();
    for i in 1..=7 {
        let node_id = format!("node-{i}");
        let label = if i == 1 { "Metabolism".to_string() } else { format!("Topic {i}") };
        let node = DslValue::object([
            ("id".into(), DslValue::String(node_id.clone())),
            ("nodeKind".into(), DslValue::String("identity".into())),
            ("shape".into(), DslValue::String("circle".into())),
            ("x".into(), DslValue::float((i as f64) * 40.0)),
            ("y".into(), DslValue::float((i as f64) * 30.0)),
            ("radius".into(), DslValue::float(24.0)),
            ("text".into(), DslValue::String(label.clone())),
            ("handles".into(), DslValue::Array(vec![])),
        ]);
        snapshot = store::apply_mutation(&snapshot, &crate::artifacts::wires::mutations::create_node(node))?.0;
        array_mut(&mut snapshot.wires_fixture, "identities").push(DslValue::object([
            ("identityId".into(), DslValue::uint(i as u64)),
            ("identityKind".into(), DslValue::String("topic".into())),
            ("label".into(), DslValue::String(label)),
            ("nodeId".into(), DslValue::String(node_id)),
        ]));
    }
    for i in 1..=9 {
        let edge_id = format!("edge-{i}");
        let source = format!("node-{}", ((i - 1) % 7) + 1);
        let target = format!("node-{}", (i % 7) + 1);
        let kind = if i == 8 { "is" } else { "owns" };
        let edge = DslValue::object([("id".into(), DslValue::String(edge_id.clone())), ("source".into(), DslValue::String(source)), ("target".into(), DslValue::String(target))]);
        let relationship = DslValue::object([
            ("relationshipId".into(), DslValue::uint(i as u64)),
            ("kind".into(), DslValue::String(kind.into())),
            ("sourceIdentityId".into(), DslValue::uint((((i - 1) % 7) + 1) as u64)),
            ("targetIdentityId".into(), DslValue::uint(((i % 7) + 1) as u64)),
            ("edgeId".into(), DslValue::String(edge_id)),
        ]);
        snapshot = store::apply_mutation(&snapshot, &crate::artifacts::wires::mutations::connect_nodes(edge, relationship))?.0;
    }
    let board = crate::artifacts::wires::wires_working_board(&snapshot);
    if let DslValue::Object(entries) = &mut snapshot.wires_fixture {
        if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == "board") {
            *slot = board;
        }
    }
    Ok(snapshot)
}
//#endregion 🔖️ExampleFixture
