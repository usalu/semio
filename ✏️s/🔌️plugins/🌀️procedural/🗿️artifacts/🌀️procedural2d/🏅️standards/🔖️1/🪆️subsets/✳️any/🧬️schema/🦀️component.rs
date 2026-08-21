//! 🧬️ Procedural2d artifact schema — every field of the artifact with its state class.

use crate::artifacts::procedural2d::snapshot::schema::Procedural2dSnapshot;
use flow::dag::DagFixture;
use flow::forms_bridge::apply_generation_values_to_fixture;
use flow::playbook::GenerationPlayState;
use flow::render_scene_json;
use flow::CameraJson;
use flow::FlowFixture;
use flow::{flow_host_with_session, flow_neuron_kind_infos_json, FlowEvalSession, FlowHost};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use store::ArtifactDsl;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Procedural2dArtifact
/// 🧬️ Procedural2dArtifact facet type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural2d")]

pub struct Procedural2dArtifact {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayState,
    #[state(presence)]
    pub selected_ids: Vec<String>,
    #[state(config)]
    pub graph_camera: CameraJson,
    #[state(config)]
    pub show_mode: String,
    #[state(presence)]
    pub selected_generation_id: Option<String>,
    #[state(artifact)]
    pub generation_preview_text: Option<String>,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Procedural2dArtifact

impl Default for Procedural2dArtifact {
    fn default() -> Self {
        Self {
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default(),
            selected_ids: Vec::new(),
            graph_camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            show_mode: "preview".into(),
            selected_generation_id: None,
            generation_preview_text: None,
            locale: "en-US".into(),
        }
    }
}

impl Procedural2dArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> Procedural2dSnapshot {
        Procedural2dSnapshot { fixture: self.fixture.clone(), generation: self.generation.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: Procedural2dSnapshot) -> Self {
        Self { fixture: snapshot.fixture, generation: snapshot.generation, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: Procedural2dSnapshot) {
        self.fixture = snapshot.fixture;
        self.generation = snapshot.generation;
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.procedural2d` — twenty handcrafted schema leaves.
pub async fn procedural2d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.procedural.procedural2d",
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
    use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dMutation, Procedural2dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Procedural2dBuilderConstruction {
        snapshot: Procedural2dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Procedural2dBuilderConstruction {
        type Snapshot = Procedural2dSnapshot;
        type Mutation = Procedural2dMutation;
        type Diff = Procedural2dDiff;
        async fn empty() -> Self {
            Self { snapshot: Procedural2dSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Procedural2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Procedural2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Procedural2dDiff as protocol::MutationDiff<Procedural2dSnapshot>>::apply(&diff, &self.snapshot)?;
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
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::procedural2d::Procedural2dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Procedural2dParts {
        pub snapshot: Option<Procedural2dSnapshot>,
    }

    pub struct Procedural2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Procedural2dAnalyzerAnalysis {
        type Parts = Procedural2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural2d", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Procedural2dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Procedural2dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Procedural2dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Procedural2dBuilderFacets {
        construction: Procedural2dBuilderConstruction,
        analysis: Procedural2dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Procedural2dComposerComposition,
    }
    builder: Procedural2dBuilder,
    analyzer: Procedural2dAnalyzer,
    composer: Procedural2dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧬️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// pure helpers over document types (`FlowFixture`/`DagFixture`/eval `Value`), not app-referencing.
pub async fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    host
}

pub async fn host_from_fixture_with_session(fixture: &FlowFixture, session: &FlowEvalSession) -> FlowHost {
    flow_host_with_session(fixture, session)
}

/// 🔀️ Runs a host mutation seeded from the projection fixture and diffs the result into operations.
/// Diffs against the host-normalized baseline (not the raw projection) so `FlowHost`'s own
/// dedupe/dag-rebuild normalization does not leak spurious collection operations — only the actual
/// mutation becomes an operation, which keeps concurrent disjoint edits mergeable on the backbone.
pub async fn host_operations(fixture: &FlowFixture, mutate: impl FnOnce(&mut FlowHost)) -> Vec<crate::artifacts::procedural2d::op::Procedural2dMutation> {
    let mut host = host_from_fixture(fixture);
    let baseline = host.fixture.clone();
    mutate(&mut host);
    crate::artifacts::procedural2d::op::procedural2d_fixture_operations(&baseline, &host.fixture)
}

pub async fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub async fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}

pub async fn collect_drawing_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if handle.starts_with("drawing-") {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_drawing_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_drawing_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

pub async fn affine_transform_array(value: &Value) -> [f64; 6] {
    if let Some(matrix) = value.as_array() {
        let mut out = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (index, entry) in matrix.iter().take(6).enumerate() {
            out[index] = entry.as_f64().unwrap_or(if index == 0 || index == 3 { 1.0 } else { 0.0 });
        }
        return out;
    }
    if let Some(matrix) = value.get("0").and_then(|entry| entry.as_array()) {
        let wrapped = Value::Array(matrix.clone());
        return affine_transform_array(&wrapped);
    }
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

pub async fn path_segments_from_node(node: &Value) -> Vec<Value> {
    if let Some(segments) = node.get("segments").and_then(|entry| entry.as_array()) {
        return segments.clone();
    }
    for key in ["path", "shape", "line", "polyline", "rect", "ellipse", "circle", "polygon"] {
        if let Some(inner) = node.get(key) {
            if let Some(segments) = inner.get("segments").and_then(|entry| entry.as_array()) {
                return segments.clone();
            }
        }
    }
    Vec::new()
}

pub async fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<Value> {
    let scene_json = render_scene_json(handle);
    let Ok(scene) = serde_json::from_str::<Value>(&scene_json) else {
        return Vec::new();
    };
    if scene.get("error").is_some() {
        return Vec::new();
    }
    let Some(nodes) = scene.get("nodes").and_then(|entry| entry.as_array()) else {
        return Vec::new();
    };
    nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let node_body = node.get("node").unwrap_or(node);
            json!({
                "id": format!("{prefix}-{handle}-{index}"),
                "transform": affine_transform_array(node.get("transform").unwrap_or(&Value::Null)),
                "segments": path_segments_from_node(node_body),
                "fill": node.get("fill").cloned().unwrap_or(Value::Null),
                "stroke": node.get("stroke").cloned().unwrap_or(Value::Null),
                "opacity": node.get("opacity").and_then(|entry| entry.as_f64()).unwrap_or(1.0),
                "blendMode": "normal",
                "visible": true,
                "needsKernel": false})
        })
        .collect()
}

pub async fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    host.evaluate().unwrap_or_default()
}

pub async fn generation_preview_layers(eval_json: &str) -> String {
    let prefix = "procedural2d-generate-preview";
    let mut layers = Vec::new();
    if let Ok(outputs) = serde_json::from_str::<Value>(eval_json) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

/// 📄️ The `procedural2d-play` "default" document — parsed from the bundled `.procedural2d` example
/// fixture, falling back to the empty document if the fixture ever fails to parse.
pub async fn default_snapshot() -> Procedural2dSnapshot {
    Procedural2dSnapshot::parse_dsl(crate::artifacts::procedural2d::dsl::PROCEDURAL2D_EXAMPLE_TEXT).unwrap_or_default()
}

pub async fn empty_procedural2d_snapshot() -> Procedural2dSnapshot {
    Procedural2dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_parses_the_bundled_example() {
        assert!(!default_snapshot().fixture.widgets.is_empty());
    }
}
//#endregion 🧪️Tests
