//! 🧬️ Procedural3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::procedural3d::snapshot::schema::Procedural3dSnapshot;
use flow::CameraJson;
use flow::FlowFixture;
use flow::playbook::GenerationPlayState;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use crate::artifacts::procedural3d::dsl::{
    PROCEDURAL3D_EXAMPLE_BOX_FILLET_TEXT, PROCEDURAL3D_EXAMPLE_BOX_SHELL_TEXT, PROCEDURAL3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT, PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT, PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT, PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT,
    PROCEDURAL3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT, PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT};
use flow::dag::DagFixture;
use flow::forms_bridge::apply_generation_values_to_fixture;
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, Widget};
use flow::playbook::selected_generation;
use serde_json::Value;
use store::ArtifactDsl;
use crate::artifacts::procedural3d::widget_id;

//#region 🔖️Procedural3dArtifact
/// 🧬️ Procedural3dArtifact facet type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural3d")]

pub struct Procedural3dArtifact {
    #[state(artifact)] pub fixture: FlowFixture,
    #[state(artifact)] pub generation: GenerationPlayState,
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(config)] pub lod_mode: String,
    #[state(config)] pub show_mode: String,
    #[state(config)] pub selection_method: String,
    #[state(artifact)] pub hovered_node_id: Option<String>,
    #[state(config)] pub graph_camera: CameraJson,
    #[state(config)] pub preview_camera: Procedural3dPreviewCamera,
    #[state(config)] pub sun_json: String,
    #[state(presence)] pub selected_generation_id: Option<String>,
    #[state(artifact)] pub generation_preview_text: Option<String>,
    #[state(presence)] pub active_utility_id: String,
    #[state(config)] pub locale: String}
//#endregion 🔖️Procedural3dArtifact

//#region 🔖️PreviewCamera
/// 📷️ 3D preview viewport camera (schema twin of the app config record).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dPreviewCamera {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub fov: f64}

impl Default for Procedural3dPreviewCamera {
    fn default() -> Self {
        Self {
            position_x: 4.0,
            position_y: -4.0,
            position_z: 3.0,
            target_x: 0.0,
            target_y: 0.0,
            target_z: 0.0,
            fov: 45.0}
    }
}
//#endregion 🔖️PreviewCamera

impl Default for Procedural3dArtifact {
    fn default() -> Self {
        Self {
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default(),
            selected_node_ids: Vec::new(),
            lod_mode: String::new(),
            show_mode: "shaded".into(),
            selection_method: "rectangle".into(),
            hovered_node_id: None,
            graph_camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Procedural3dPreviewCamera::default(),
            sun_json: serde_json::to_string(&semio_framework_plugin::WorldSunConfig::default()).unwrap_or_default(),
            selected_generation_id: None,
            generation_preview_text: None,
            active_utility_id: "move".into(),
            locale: "en-US".into()}
    }
}

impl Procedural3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Procedural3dSnapshot {
        Procedural3dSnapshot {
            fixture: self.fixture.clone(),
            generation: self.generation.clone()}
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Procedural3dSnapshot) -> Self {
        Self {
            fixture: snapshot.fixture,
            generation: snapshot.generation,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Procedural3dSnapshot) {
        self.fixture = snapshot.fixture;
        self.generation = snapshot.generation;
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.procedural3d` — twenty handcrafted schema leaves.
pub fn procedural3d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.procedural.procedural3d",
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
    use crate::artifacts::procedural3d::{Procedural3dDiff, Procedural3dMutation, Procedural3dSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Procedural3dBuilderConstruction {
        snapshot: Procedural3dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Procedural3dBuilderConstruction {
        type Snapshot = Procedural3dSnapshot;
        type Mutation = Procedural3dMutation;
        type Diff = Procedural3dDiff;
        fn empty() -> Self { Self { snapshot: Procedural3dSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Procedural3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Procedural3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }
        fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Procedural3dDiff as protocol::MutationDiff<Procedural3dSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
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
    use crate::artifacts::procedural3d::Procedural3dSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Procedural3dParts {
        pub snapshot: Option<Procedural3dSnapshot>,
    }

    pub struct Procedural3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Procedural3dAnalyzerAnalysis {
        type Parts = Procedural3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Procedural3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Procedural3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Procedural3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Procedural3dBuilderFacets {
        construction: derived_construction::Procedural3dBuilderConstruction,
        analysis: derived_analysis::Procedural3dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Procedural3dComposerComposition,
    }
    builder: Procedural3dBuilder,
    analyzer: Procedural3dAnalyzer,
    composer: Procedural3dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧬️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// pure helpers over document types (`FlowFixture`/`DagFixture`/`FlowHost`), not app-referencing (the
/// Config-referencing preview/mesh-export helpers that used to sit alongside these stayed in
/// `crate::editor::procedural3d` instead — see that file's own `PreviewPipeline`/`MeshBridge` regions).
pub const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
pub const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
pub const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";
pub const PROCEDURAL_EXAMPLE_BOX_FILLET: &str = "box-fillet-preview";
pub const PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE: &str = "sphere-box-fuse";
pub const PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE: &str = "face-sweep-extrude";
pub const PROCEDURAL_EXAMPLE_RECTANGLE_WIRE: &str = "rectangle-wire-preview";
pub const PROCEDURAL_EXAMPLE_BOX_SHELL: &str = "box-shell-preview";

/// 📄️ The `procedural3d-play` "default" document — parsed from the bundled "hexagonal mushroom
/// column" example fixture.
pub fn default_snapshot() -> Procedural3dSnapshot {
    Procedural3dSnapshot::parse_dsl(PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()
}

pub fn empty_procedural3d_snapshot() -> Procedural3dSnapshot {
    Procedural3dSnapshot::default()
}

/// 🧾️ Whether `example_id` names a bundled procedural-3d example fixture.
pub fn is_procedural3d_example_id(example_id: &str) -> bool {
    matches!(
        example_id,
        PROCEDURAL_EXAMPLE_HEX_COLUMN
            | "demo"
            | PROCEDURAL_EXAMPLE_RECT_EXTRUDE
            | PROCEDURAL_EXAMPLE_SPHERE_TORUS
            | PROCEDURAL_EXAMPLE_BOX_FILLET
            | PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE
            | PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE
            | PROCEDURAL_EXAMPLE_RECTANGLE_WIRE
            | PROCEDURAL_EXAMPLE_BOX_SHELL
    )
}

/// 🧾️ Builds the projection for a named bundled example; unknown ids return `None`.
pub fn example_snapshot(example_id: &str) -> Option<Procedural3dSnapshot> {
    let dsl = match example_id {
        PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT),
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT),
        PROCEDURAL_EXAMPLE_BOX_FILLET => Some(PROCEDURAL3D_EXAMPLE_BOX_FILLET_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE => Some(PROCEDURAL3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT),
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE => Some(PROCEDURAL3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE => Some(PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT),
        PROCEDURAL_EXAMPLE_BOX_SHELL => Some(PROCEDURAL3D_EXAMPLE_BOX_SHELL_TEXT),
        _ => None};
    dsl.and_then(|text| Procedural3dSnapshot::parse_dsl(text).ok())
}

/// 🧾️ Serializes an example's bare projection for registration via `App::example`.
pub fn example_document_json(example_id: &str) -> String {
    serde_json::to_string(&example_snapshot(example_id).unwrap_or_default()).unwrap_or_default()
}

pub fn generation_fixture_for(fixture: &FlowFixture, generation: &GenerationPlayState) -> FlowFixture {
    if let Some(selected) = selected_generation(generation) {
        let patched = apply_generation_values_to_fixture(&serde_json::to_string(fixture).unwrap_or_default(), &selected.values);
        FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone())
    } else {
        fixture.clone()
    }
}

pub fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
    host
}

pub fn host_from_fixture_with_session(fixture: &FlowFixture, session: &FlowEvalSession) -> FlowHost {
    flow_host_with_session(fixture, session)
}

/// 🔀️ Rebuilds the fixture the flow host would normalize `before` to, then diffs `target` against
/// that baseline.
pub fn commit_fixture(before: &FlowFixture, target: &FlowFixture) -> Vec<crate::artifacts::procedural3d::op::Procedural3dMutation> {
    let baseline = host_from_fixture(before).fixture;
    crate::artifacts::procedural3d::op::procedural3d_fixture_operations(&baseline, target)
}

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<ui_wgpu::wgpu::NodeGraphNodeRecord>, Vec<ui_wgpu::wgpu::NodeGraphEdgeRecord>) {
    let nodes: Vec<ui_wgpu::wgpu::NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| ui_wgpu::wgpu::NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| ui_wgpu::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| ui_wgpu::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<ui_wgpu::wgpu::NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            ui_wgpu::wgpu::NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}

pub fn widget_id_from_instance_id(instance_id: &str) -> &str {
    instance_id.split('#').next().unwrap_or(instance_id)
}

pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
    host.evaluate().unwrap_or_default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️GumballTransforms
/// 🧭️ Maps a gumball drag operation to the flow-graph transform neuron kind that persists it.
pub fn gumball_xform_kind(operation: &str) -> &'static str {
    match operation {
        "rotate" => "brep.xform.rotate",
        "scale" => "brep.xform.scale",
        _ => "brep.xform.translate"}
}

/// 🪪️ Deterministic id for the transform neuron generated by dragging `source_id`'s gumball for `operation`.
pub fn gumball_widget_id(source_id: &str, operation: &str) -> String {
    format!("{source_id}__gumball_{operation}")
}

pub fn gumball_widget_json(host: &FlowHost, widget_id_str: &str) -> Option<Value> {
    host.fixture.widgets.iter().find(|widget| widget_id(widget) == widget_id_str).and_then(|widget| serde_json::to_value(widget).ok())
}

pub fn gumball_widget_offset(host: &FlowHost, widget_id_str: &str) -> [f64; 3] {
    let offset = gumball_widget_json(host, widget_id_str).and_then(|widget_json| widget_json.get("params").and_then(|params| params.get("offset")).cloned());
    [
        offset.as_ref().and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("z")).and_then(Value::as_f64).unwrap_or(0.0),
    ]
}

pub fn gumball_widget_number_param(host: &FlowHost, widget_id_str: &str, key: &str, default: f64) -> f64 {
    gumball_widget_json(host, widget_id_str).and_then(|widget_json| widget_json.get("params").and_then(|params| params.get(key)).and_then(|entry| entry.get("value")).and_then(Value::as_f64)).unwrap_or(default)
}

pub fn gumball_translate_params_json(offset: [f64; 3]) -> String {
    serde_json::json!({ "offset": { "$schema": "vector", "x": offset[0], "y": offset[1], "z": offset[2] } }).to_string()
}

pub fn gumball_rotate_params_json(axis: [f64; 3], angle: f64) -> String {
    serde_json::json!({
        "axis": { "$schema": "vector", "x": axis[0], "y": axis[1], "z": axis[2] },
        "angle": { "$schema": "number", "value": angle }})
    .to_string()
}

pub fn gumball_scale_params_json(factor: f64) -> String {
    serde_json::json!({
        "factor": { "$schema": "number", "value": factor },
        "center": { "$schema": "point", "x": 0.0, "y": 0.0, "z": 0.0 }})
    .to_string()
}

/// 🔀️ Finds (or splices in) the transform neuron that persists `selected_id`'s gumball drag for
/// `operation` into the flow graph, rewiring downstream consumers so the transformed geometry is what
/// actually evaluates and exports.
pub fn ensure_gumball_node(host: &mut FlowHost, selected_id: &str, operation: &str) -> Result<String, String> {
    let own_suffix = format!("__gumball_{operation}");
    if selected_id.ends_with(&own_suffix) && host.fixture.widgets.iter().any(|widget| widget_id(widget) == selected_id) {
        return Ok(selected_id.to_string());
    }
    let transform_id = gumball_widget_id(selected_id, operation);
    if host.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id) {
        return Ok(transform_id);
    }
    let (source_x, source_y) = host.fixture.layout.get(selected_id).map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    let descriptor = serde_json::json!({ "kind": "neuron", "id": transform_id, "neuronKind": gumball_xform_kind(operation) }).to_string();
    host.add_widget(&descriptor, source_x + 220.0, source_y).map_err(|err| err.to_string())?;
    let outgoing_port = host.fixture.synapses.iter().find(|synapse| synapse.from == selected_id).map(|synapse| synapse.from_port.clone());
    if let Some(port) = outgoing_port {
        host.insert_between(selected_id, &port, &transform_id, "geometry", "geometry").map_err(|err| err.to_string())?;
    } else {
        host.connect(selected_id, &transform_id).map_err(|err| err.to_string())?;
    }
    if let Some(Widget::Neuron { preview, .. }) = host.fixture.widgets.iter_mut().find(|widget| widget_id(widget) == selected_id) {
        *preview = false;
    }
    Ok(transform_id)
}
//#endregion 🔖️GumballTransforms
