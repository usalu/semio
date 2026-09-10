//! 🧬️ Generation2d artifact schema — every field of the artifact with its state class.

use crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshot;
use semio_framework_artifact_infinite_dag::DagFixture;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_fixture;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::render_scene_json;

use ::semio_framework_schema::ArtifactSchema;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::{flow_host_with_session, flow_neuron_kind_info_map, FlowEvalSession, FlowHost};
#[cfg(feature = "component-app-assembly")]
use semio_framework_ui::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactDsl;
//#region 🔖️Generation2dArtifact
/// 🧬️ Generation2dArtifact facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, Default)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation2d")]
pub struct Generation2dArtifact {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation2dArtifact


impl Generation2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Generation2dSnapshot {
        Generation2dSnapshot { fixture: self.fixture.clone(), generation: self.generation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Generation2dSnapshot) -> Self {
        Self { fixture: snapshot.fixture, generation: snapshot.generation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Generation2dSnapshot) {
        self.fixture = snapshot.fixture;
        std::mem::replace(&mut self.generation, snapshot.generation).retire_cold();
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.generation2d` — twenty handcrafted schema leaves.
pub fn generation2d_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.procedural.generation2d",
        artifact: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🕸️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{Generation2dDiff, Generation2dMutation, Generation2dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Generation2dBuilderConstruction {
        snapshot: Generation2dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Generation2dBuilderConstruction {
        type Snapshot = Generation2dSnapshot;
        type Mutation = Generation2dMutation;
        type Diff = Generation2dDiff;
        fn empty() -> Self {
            Self { snapshot: Generation2dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Generation2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Generation2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Generation2dDiff as protocol::MutationDiff<Generation2dSnapshot>>::apply(&diff, &self.snapshot)?;
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
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::Generation2dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Generation2dParts {
        pub snapshot: Option<Generation2dSnapshot>,
    }

    pub struct Generation2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Generation2dAnalyzerAnalysis {
        type Parts = Generation2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation2d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Generation2dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Generation2dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Generation2dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Generation2dBuilderFacets {
        construction: Generation2dBuilderConstruction,
        analysis: Generation2dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Generation2dComposerComposition,
    }
    builder: Generation2dBuilder,
    analyzer: Generation2dAnalyzer,
    composer: Generation2dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧬️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// pure helpers over document types (`FlowFixture`/`DagFixture`/eval `Value`), not app-referencing.
/// 🏠️ Runs `body` against a catalogue-seeded host built from `fixture`, then retires that host.
///
/// A `FlowHost` owns a cloned `FlowFixture`, whose `layout: OrderedMap<WidgetLayout>` rejects a bare
/// drop (`ordered-map root must be explicitly retired before drop`,
/// `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), so a host is CLOSED through
/// [`FlowHost::retire_cold`], never dropped.
#[cfg(feature = "component-app-assembly")]
pub fn with_host<R>(fixture: &FlowFixture, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    FlowHost::with_fixture(fixture, |host| {
        host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
        body(host)
    })
}

/// 🏠️ [`with_host`]'s session-backed twin: the host shares `session`'s neural cache and converged
/// evaluation baseline, and is retired the same way. `body` receives the session back alongside the
/// host because every real caller needs it mutably (`sync`/`tick`).
#[cfg(feature = "component-app-assembly")]
pub fn with_host_session<R>(fixture: &FlowFixture, session: &mut FlowEvalSession, body: impl FnOnce(&mut FlowHost, &mut FlowEvalSession) -> R) -> R {
    let mut host = flow_host_with_session(fixture, session);
    let result = body(&mut host, session);
    host.retire_cold();
    result
}

/// 🔀️ Runs a host mutation seeded from the projection fixture and diffs the result into operations.
/// Diffs against the host-normalized baseline (not the raw projection) so `FlowHost`'s own
/// dedupe/dag-rebuild normalization does not leak spurious collection operations — only the actual
/// mutation becomes an operation, which keeps concurrent disjoint edits mergeable on the backbone.
#[cfg(feature = "component-app-assembly")]
pub fn host_operations(fixture: &FlowFixture, mutate: impl FnOnce(&mut FlowHost)) -> Vec<crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation> {
    with_host(fixture, |host| {
        let baseline = host.fixture.clone();
        mutate(host);
        let operations = crate::standards::v1::subsets::any::schema::mutations::text::generation2d_fixture_operations(&baseline, &host.fixture);
        baseline.retire_cold();
        operations
    })
}

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

#[cfg(feature = "component-app-assembly")]
pub fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
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

#[cfg(feature = "component-app-assembly")]
pub fn collect_drawing_handles_from_eval(value: &dsl::json::Value, handles: &mut Vec<String>) {
    match value {
        dsl::json::Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if handle.starts_with("drawing-") {
                    handles.push(handle.into());
                }
            }
            for (_, entry) in map.iter() {
                collect_drawing_handles_from_eval(entry, handles);
            }
        }
        dsl::json::Value::Array(items) => {
            for item in items {
                collect_drawing_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

#[cfg(feature = "component-app-assembly")]
pub fn affine_transform_array(value: &dsl::json::Value) -> [f64; 6] {
    if let Some(matrix) = value.as_array() {
        let mut out = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (index, entry) in matrix.iter().take(6).enumerate() {
            out[index] = entry.as_f64().unwrap_or(if index == 0 || index == 3 { 1.0 } else { 0.0 });
        }
        return out;
    }
    if let Some(matrix) = value.get("0").and_then(|entry| entry.as_array()) {
        let wrapped = dsl::json::Value::Array(matrix.clone());
        return affine_transform_array(&wrapped);
    }
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

#[cfg(feature = "component-app-assembly")]
pub fn path_segments_from_node(node: &dsl::json::Value) -> Vec<dsl::json::Value> {
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

#[cfg(feature = "component-app-assembly")]
pub fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<dsl::json::Value> {
    let scene_json = render_scene_json(handle);
    let Ok(scene) = dsl::json::parse(&scene_json) else {
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
            let transform: Vec<dsl::json::Value> = affine_transform_array(node.get("transform").unwrap_or(&dsl::json::Value::Null)).into_iter().map(dsl::json::Value::from).collect();
            let mut object = dsl::json::Object::new();
            object.insert("id", dsl::json::Value::from(format!("{prefix}-{handle}-{index}")));
            object.insert("transform", dsl::json::Value::from(transform));
            object.insert("segments", dsl::json::Value::from(path_segments_from_node(node_body)));
            object.insert("fill", node.get("fill").cloned().unwrap_or(dsl::json::Value::Null));
            object.insert("stroke", node.get("stroke").cloned().unwrap_or(dsl::json::Value::Null));
            object.insert("opacity", dsl::json::Value::from(node.get("opacity").and_then(|entry| entry.as_f64()).unwrap_or(1.0)));
            object.insert("blendMode", dsl::json::Value::from("normal"));
            object.insert("visible", dsl::json::Value::from(true));
            object.insert("needsKernel", dsl::json::Value::from(false));
            dsl::json::Value::Object(object)
        })
        .collect()
}

#[cfg(feature = "component-app-assembly")]
pub fn generation_preview_host(fixture: &FlowFixture, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> FlowHost {
    let fixture_json = dsl::json::to_json_string(fixture);
    let object: dsl::json::Object = values.iter().map(|(key, value)| (key.clone(), dsl::json::from_dsl_value(value))).collect();
    let patched = apply_generation_values_to_fixture(&fixture_json, &object);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    FlowHost::from_fixture(patched_fixture)
}

#[cfg(feature = "component-app-assembly")]
pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> String {
    let mut host = generation_preview_host(fixture, values);
    let evaluated = host.evaluate().unwrap_or_default();
    host.retire_cold();
    evaluated
}

#[cfg(feature = "component-app-assembly")]
pub fn generation_preview_layers(eval_json: &str) -> String {
    let prefix = "generation2d-generate-preview";
    let mut layers = Vec::new();
    if let Ok(outputs) = dsl::json::parse(eval_json) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    dsl::json::to_string(&dsl::json::Value::from(layers))
}

/// 📄️ The `procedural2d-play` "default" document — parsed from the bundled `.generation2d` example
/// fixture, falling back to the empty document if the fixture ever fails to parse.
pub fn default_snapshot() -> Generation2dSnapshot {
    Generation2dSnapshot::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::GENERATION2D_EXAMPLE_TEXT).unwrap_or_default()
}

pub fn empty_generation2d_snapshot() -> Generation2dSnapshot {
    Generation2dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::FlowFixture;
//#endregion 🔁️Re-exports
