//! 🧬️ Wires artifact schema — every field of the artifact with its state class.

use dsl::DslValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Artifact
/// 🧬️ Full wires artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
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
    #[state(presence)]
    pub selected_ids: Vec<String>,
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
            content: crate::artifacts::wires::wires_content_child_handle_and_cache(Vec::new(), Vec::new()),
            camera: crate::artifacts::wires::empty_camera(),
            meta: DslValue::Null,
            selected_ids: Vec::new(),
            drag_node_id: None,
            drag_last_x: 0.0,
            drag_last_y: 0.0,
            locale: "en-US".into(),
        }
    }
}

impl WiresArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::wires::WiresSnapshot {
        crate::artifacts::wires::WiresSnapshot {
            wires_fixture: self.wires_fixture.clone(),
            content: self.content.clone(),
            camera: self.camera.clone(),
            meta: self.meta.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::wires::WiresSnapshot) -> Self {
        Self {
            wires_fixture: snapshot.wires_fixture,
            content: snapshot.content,
            camera: snapshot.camera,
            meta: snapshot.meta,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::wires::WiresSnapshot) {
        self.wires_fixture = snapshot.wires_fixture;
        self.content = snapshot.content;
        self.camera = snapshot.camera;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.reasoning.wires` — twenty handcrafted schema leaves.
pub fn wires_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.reasoning.wires",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::wires::schema::diff::WiresDiff;
    use crate::artifacts::wires::schema::mutations::WiresMutation;
    use crate::artifacts::wires::schema::snapshot::WiresSnapshot;

    #[derive(Clone, Debug)]
    pub struct WiresBuilderConstruction {
        snapshot: WiresSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for WiresBuilderConstruction {
        type Snapshot = WiresSnapshot;
        type Mutation = WiresMutation;
        type Diff = WiresDiff;
        fn empty() -> Self { Self { snapshot: crate::artifacts::wires::empty_wires_snapshot(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::wires::WiresSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct WiresParts {
        pub snapshot: Option<WiresSnapshot>,
    }

    pub struct WiresAnalyzerAnalysis;

    impl ArtifactAnalysis for WiresAnalyzerAnalysis {
        type Parts = WiresParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.wires", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = WiresParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <WiresSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <WiresSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec WiresBuilderFacets {
        construction: derived_construction::WiresBuilderConstruction,
        analysis: derived_analysis::WiresAnalyzerAnalysis,
        composition: super::super::io::derived_composition::WiresComposerComposition,
    }
    builder: WiresBuilder,
    analyzer: WiresAnalyzer,
    composer: WiresComposer,
);
//#endregion 🧬️DerivedArtifactFacets

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
    dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null)
}

pub fn fixture_json_string(fixture: &DslValue) -> String {
    serde_json::to_string(&dsl_to_json(fixture)).unwrap_or_else(|_| "{}".into())
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

/// 🕸️ Re-lays out the board with the neutral `infinite_board_port_directed` force-graph solver — the
/// same shared mechanism `puzzle/2d`'s `forceLayout`/`reorganize` uses, depended on directly rather
/// than through puzzle's app program (mindmap's board schema is on its allowlist).
pub fn force_layout_board(board: &mut DslValue) {
    let Ok(layout_json) = infinite_board_port_directed::apply_force_graph_layout_to_fixture_v1_json(&fixture_json_string(board), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = serde_json::from_str::<Value>(&layout_json) {
        *board = dsl::to_dsl_value(&parsed).unwrap_or(DslValue::Null);
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ExampleFixture
/// 📄️ The `metabolism` example, parsed once from `crate::artifacts::wires::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT`
/// — falls back to the empty document if the fixture ever fails to parse.
pub fn metabolism_wires_example_snapshot() -> crate::artifacts::wires::WiresSnapshot {
    match <crate::artifacts::wires::WiresSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::wires::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT) {
        Ok(snapshot) if fixture_nodes(&crate::artifacts::wires::wires_working_board(&snapshot)).len() >= 7 => snapshot,
        _ => handcrafted_metabolism_snapshot(),
    }
}

/// 🧪️ Hand-built metabolism demo when the bundled `.dsl.semio` asset is still a stub envelope.
fn handcrafted_metabolism_snapshot() -> crate::artifacts::wires::WiresSnapshot {
    use serde_json::json;
    let mut snapshot = crate::artifacts::wires::empty_wires_snapshot();
    for i in 1..=7 {
        let node_id = format!("node-{i}");
        let label = if i == 1 { "Metabolism".to_string() } else { format!("Topic {i}") };
        let node = dsl::to_dsl_value(&json!({
            "id": node_id,
            "nodeKind": "identity",
            "shape": "circle",
            "x": (i as f64) * 40.0,
            "y": (i as f64) * 30.0,
            "radius": 24.0,
            "text": label,
            "handles": []
        }))
        .expect("node serializes");
        snapshot = store::apply_mutation(&snapshot, &crate::artifacts::wires::mutations::create_node(node));
        array_mut(&mut snapshot.wires_fixture, "identities").push(
            dsl::to_dsl_value(&json!({
                "identityId": i,
                "identityKind": "topic",
                "label": label,
                "nodeId": node_id,
            }))
            .expect("identity serializes"),
        );
    }
    for i in 1..=9 {
        let edge_id = format!("edge-{i}");
        let source = format!("node-{}", ((i - 1) % 7) + 1);
        let target = format!("node-{}", (i % 7) + 1);
        let kind = if i == 8 { "is" } else { "owns" };
        let edge = dsl::to_dsl_value(&json!({ "id": edge_id, "source": source, "target": target })).expect("edge serializes");
        let relationship = dsl::to_dsl_value(&json!({
            "relationshipId": i,
            "kind": kind,
            "sourceIdentityId": ((i - 1) % 7) + 1,
            "targetIdentityId": (i % 7) + 1,
            "edgeId": edge_id,
        }))
        .expect("relationship serializes");
        snapshot = store::apply_mutation(&snapshot, &crate::artifacts::wires::mutations::connect_nodes(edge, relationship));
    }
    let board = crate::artifacts::wires::wires_working_board(&snapshot);
    if let DslValue::Object(entries) = &mut snapshot.wires_fixture {
        if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == "board") {
            *slot = board;
        }
    }
    snapshot
}
//#endregion 🔖️ExampleFixture
