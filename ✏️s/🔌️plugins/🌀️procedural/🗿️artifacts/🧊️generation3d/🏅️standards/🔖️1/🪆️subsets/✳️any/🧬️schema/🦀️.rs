//! 🧬️ Generation3d artifact schema — every field of the artifact with its state class.

use crate::standards::v1::subsets::any::schema::snapshot::text::{
    GENERATION3D_EXAMPLE_BOX_FILLET_TEXT, GENERATION3D_EXAMPLE_BOX_SHELL_TEXT, GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT, GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT, GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT, GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT,
    GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT, GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT,
};
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot;
use crate::widget_id;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_fixture as apply_generation_values_to_fixture_json;

use ::semio_framework_schema::ArtifactSchema;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::{flow_host_with_session, FlowEvalSession, FlowHost};
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactDsl;

//#region 🔖️Generation3dArtifact
/// 🧬️ Generation3dArtifact facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation3d")]
pub struct Generation3dArtifact {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation3dArtifact

//#region 🔖️PreviewCamera
/// 📷️ 3D preview viewport camera (schema twin of the app config record).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dPreviewCamera {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub fov: f64,
}

impl Default for Generation3dPreviewCamera {
    fn default() -> Self {
        Self { position_x: 4.0, position_y: -4.0, position_z: 3.0, target_x: 0.0, target_y: 0.0, target_z: 0.0, fov: 45.0 }
    }
}
//#endregion 🔖️PreviewCamera

impl Default for Generation3dArtifact {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayState::default().into() }
    }
}

impl Generation3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Generation3dSnapshot {
        Generation3dSnapshot { fixture: self.fixture.clone(), generation: self.generation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Generation3dSnapshot) -> Self {
        Self { fixture: snapshot.fixture, generation: snapshot.generation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Generation3dSnapshot) {
        self.fixture = snapshot.fixture;
        std::mem::replace(&mut self.generation, snapshot.generation).retire_cold();
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.generation3d` — twenty handcrafted schema leaves.
pub fn generation3d_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.procedural.generation3d",
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
    use crate::{Generation3dDiff, Generation3dMutation, Generation3dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Generation3dBuilderConstruction {
        snapshot: Generation3dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Generation3dBuilderConstruction {
        type Snapshot = Generation3dSnapshot;
        type Mutation = Generation3dMutation;
        type Diff = Generation3dDiff;
        fn empty() -> Self {
            Self { snapshot: Generation3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Generation3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
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
            let snapshot = <Generation3dDiff as protocol::MutationDiff<Generation3dSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::Generation3dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Generation3dParts {
        pub snapshot: Option<Generation3dSnapshot>,
    }

    pub struct Generation3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Generation3dAnalyzerAnalysis {
        type Parts = Generation3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Generation3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Generation3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Generation3dBuilderFacets {
        construction: Generation3dBuilderConstruction,
        analysis: Generation3dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Generation3dComposerComposition,
    }
    builder: Generation3dBuilder,
    analyzer: Generation3dAnalyzer,
    composer: Generation3dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧬️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// pure helpers over document types (`FlowFixture`/`DagFixture`/`FlowHost`), not app-referencing (the
/// Config-referencing preview/mesh-export helpers that used to sit alongside these stayed in
/// `crate::editor::generation3d` instead — see that file's own `PreviewPipeline`/`MeshBridge` regions).
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
pub fn default_snapshot() -> Generation3dSnapshot {
    Generation3dSnapshot::parse_dsl(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()
}

/// 📄️ The artifact's `Default` projection. NOT empty: `FlowFixture::default()`
/// (`🧰️framework/…/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs`) is the three-widget
/// `slider → add → preview` demo graph, so this is the DEFAULT document, not the empty one — the
/// name it carried until ticket 26/09/09/PROCEDURAL-3D-END-TO-END said otherwise and made every
/// "empty document" law read against a populated graph.
pub fn default_generation3d_snapshot() -> Generation3dSnapshot {
    Generation3dSnapshot::default()
}

/// 🕳️ The genuinely EMPTY projection: no widgets, no synapses, no positions, no generations — the
/// identity element every totality law is written against.
pub fn empty_generation3d_snapshot() -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.fixture.widgets.clear();
    snapshot.fixture.synapses.clear();
    snapshot
}

/// 🧾️ Whether `example_id` names a bundled procedural-3d example fixture.
pub fn is_generation3d_example_id(example_id: &str) -> bool {
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
pub fn example_snapshot(example_id: &str) -> Option<Generation3dSnapshot> {
    let dsl = match example_id {
        PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT),
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT),
        PROCEDURAL_EXAMPLE_BOX_FILLET => Some(GENERATION3D_EXAMPLE_BOX_FILLET_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE => Some(GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT),
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE => Some(GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE => Some(GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT),
        PROCEDURAL_EXAMPLE_BOX_SHELL => Some(GENERATION3D_EXAMPLE_BOX_SHELL_TEXT),
        _ => None,
    };
    dsl.and_then(|text| Generation3dSnapshot::parse_dsl(text).ok())
}

/// 🧾️ Serializes an example's bare projection for registration via `App::example`.
pub fn example_document_json(example_id: &str) -> String {
    let snapshot = example_snapshot(example_id).unwrap_or_default();
    let json = dsl::json::to_json_string(&snapshot);
    snapshot.retire_cold();
    json
}

/// 🌉️ Bridges a `FormGeneration.values` map (`semio_framework_artifact_playbook_playbook::PlaybookValues`, see `FormGeneration`
/// in `📖️playbook/🦀️.rs`) into the `pack::json::Object` that `forms_bridge::apply_generation_values_to_fixture`
/// actually takes.
#[cfg(feature = "component-app-assembly")]
fn generation_values_to_pack_object(values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> dsl::json::Object {
    match dsl::json::from_dsl_value(&dsl::DslValue::object(values.clone())) {
        dsl::json::Value::Object(object) => object,
        _ => dsl::json::Object::new(),
    }
}

/// 🎯️ The roster entry `selected_id` names — the ONE lookup every generate-mode surface resolves its
/// "current generation" through.
///
/// 🐛️ Why not `playbook::selected_generation(state)`: that reads `GenerationPlayState.selected_generation_id`,
/// which the ARTIFACT only ever writes from `CreateGeneration`/`DeleteGeneration` replay — `selectGeneration`
/// emits no artifact mutation at all, only `Generation3dConfigMutation::SetSelectedGeneration`. The
/// evaluation path already resolved the selection off the CONFIG (`flow_eval_tick::evaluate`,
/// `generation_command_result`) while the three generate-mode window bodies still resolved it off the
/// artifact, so clicking a generation row moved the evaluation and moved nothing a user could see
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #7). The config is the single authority now, and this
/// function is where it is read.
pub fn generation_by_id<'a>(generation: &'a GenerationPlayState, selected_id: Option<&str>) -> Option<&'a semio_framework_artifact_playbook_playbook::FormGeneration> {
    let selected_id = selected_id?;
    generation.generations.iter().find(|entry| entry.id == selected_id)
}

pub fn generation_fixture_for(fixture: &FlowFixture, generation: &GenerationPlayState, selected_id: Option<&str>) -> FlowFixture {
    let Some(selected) = generation_by_id(generation, selected_id) else {
        return fixture.clone();
    };
    let mut patched = fixture.clone();
    for widget in &mut patched.widgets {
        let Some(value) = selected.values.get(widget_id(widget)) else {
            continue;
        };
        match widget {
            Widget::InputSlider { value: current, .. } => {
                if let Some(number) = value.as_f64() {
                    *current = number;
                }
            }
            Widget::InputNote { text, .. } => {
                if let Some(next) = value.as_str() {
                    *text = next.to_string();
                }
            }
            Widget::InputImage { src, .. } => {
                if let Some(next) = value.as_str() {
                    *src = next.to_string();
                }
            }
            Widget::Variable { name, .. } => {
                if let Some(next) = value.as_str() {
                    *name = next.to_string();
                }
            }
            _ => {}
        }
    }
    patched
}

/// 🏠️ Runs `body` against a catalogue-seeded host built from `fixture`, then retires that host.
///
/// A `FlowHost` owns a cloned `FlowFixture`, whose `layout: OrderedMap<WidgetLayout>` rejects a bare
/// drop (`ordered-map root must be explicitly retired before drop`,
/// `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), so a host is CLOSED through
/// [`FlowHost::retire_cold`], never dropped. This scope is the ONLY way generation3d builds one —
/// there is no `host_from_fixture(…) -> FlowHost` to leak.
#[cfg(feature = "component-app-assembly")]
pub fn with_host<R>(fixture: &FlowFixture, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    FlowHost::with_fixture(fixture, |host| {
        host.set_neuron_kind_info_map(semio_framework_os_flow::flow_neuron_kind_info_map());
        body(host)
    })
}

/// 🏠️ [`with_host`]'s session-backed twin: the host shares `session`'s neural cache and converged
/// evaluation baseline, and is retired the same way. `body` receives the session back alongside the
/// host because every real caller needs it mutably (`sync`/`tick`), which a captured `&mut` could
/// not provide while the scope itself holds the session borrow.
#[cfg(feature = "component-app-assembly")]
pub fn with_host_session<R>(fixture: &FlowFixture, session: &mut FlowEvalSession, body: impl FnOnce(&mut FlowHost, &mut FlowEvalSession) -> R) -> R {
    let mut host = flow_host_with_session(fixture, session);
    let result = body(&mut host, session);
    host.retire_cold();
    result
}

/// 🔀️ Rebuilds the fixture the flow host would normalize `before` to, then diffs `target` against
/// that baseline.
#[cfg(feature = "component-app-assembly")]
pub fn commit_fixture(before: &FlowFixture, target: &FlowFixture) -> Vec<crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation> {
    with_host(before, |host| crate::standards::v1::subsets::any::schema::mutations::text::generation3d_fixture_operations(&host.fixture, target))
}

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

#[cfg(feature = "component-app-assembly")]
pub fn fixture_to_workflow(fixture: &semio_framework_artifact_infinite_dag::DagFixture) -> (Vec<semio_framework_ui::wgpu::NodeGraphNodeRecord>, Vec<semio_framework_ui::wgpu::NodeGraphEdgeRecord>) {
    let nodes: Vec<semio_framework_ui::wgpu::NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| semio_framework_ui::wgpu::NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| semio_framework_ui::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| semio_framework_ui::wgpu::NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<semio_framework_ui::wgpu::NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            semio_framework_ui::wgpu::NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}

/// 🔌️ The widget id behind a preview instance id. Instance ids are channel-qualified
/// (`{widgetId}@{channel}#{index}`) so both suffixes have to come off; a bare widget id, a port id
/// and an instance id therefore all resolve to the same widget.
pub fn widget_id_from_instance_id(instance_id: &str) -> &str {
    let base = instance_id.split('#').next().unwrap_or(instance_id);
    base.split('@').next().unwrap_or(base)
}

#[cfg(feature = "component-app-assembly")]
pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> String {
    let fixture_json = dsl::json::to_json_string(fixture);
    let patched = apply_generation_values_to_fixture_json(&fixture_json, &generation_values_to_pack_object(values));
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    host.set_neuron_kind_info_map(semio_framework_os_flow::flow_neuron_kind_info_map());
    let evaluated = host.evaluate().unwrap_or_default();
    host.retire_cold();
    evaluated
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️GumballTransforms
/// 🧭️ Maps a gumball drag operation to the flow-graph transform neuron kind that persists it.
pub fn gumball_xform_kind(operation: &str) -> &'static str {
    match operation {
        "rotate" => "brep.xform.rotate",
        "scale" => "brep.xform.scale",
        _ => "brep.xform.translate",
    }
}

/// 🪪️ Deterministic id for the transform neuron generated by dragging `source_id`'s gumball for `operation`.
pub fn gumball_widget_id(source_id: &str, operation: &str) -> String {
    format!("{source_id}__gumball_{operation}")
}

#[cfg(feature = "component-app-assembly")]
pub fn gumball_widget_json(host: &FlowHost, widget_id_str: &str) -> Option<dsl::DslValue> {
    host.fixture.widgets.iter().find(|widget| widget_id(widget) == widget_id_str).map(dsl::ToValue::to_value)
}

#[cfg(feature = "component-app-assembly")]
pub fn gumball_widget_offset(host: &FlowHost, widget_id_str: &str) -> [f64; 3] {
    let offset = gumball_widget_json(host, widget_id_str).and_then(|widget_json| widget_json.get("params").and_then(|params| params.get("offset")).cloned());
    [
        offset.as_ref().and_then(|value| value.get("x")).and_then(dsl::DslValue::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("y")).and_then(dsl::DslValue::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("z")).and_then(dsl::DslValue::as_f64).unwrap_or(0.0),
    ]
}

#[cfg(feature = "component-app-assembly")]
pub fn gumball_widget_number_param(host: &FlowHost, widget_id_str: &str, key: &str, default: f64) -> f64 {
    gumball_widget_json(host, widget_id_str).and_then(|widget_json| widget_json.get("params").and_then(|params| params.get(key)).and_then(|entry| entry.get("value")).and_then(dsl::DslValue::as_f64)).unwrap_or(default)
}

#[cfg(feature = "component-app-assembly")]
pub fn gumball_translate_params_json(offset: [f64; 3]) -> String {
    dsl::json::to_json_string(&dsl::DslValue::object([(
        "offset".to_string(),
        dsl::DslValue::object([
            ("$schema".to_string(), dsl::DslValue::String("vector".into())),
            ("x".to_string(), dsl::DslValue::float(offset[0])),
            ("y".to_string(), dsl::DslValue::float(offset[1])),
            ("z".to_string(), dsl::DslValue::float(offset[2])),
        ]),
    )]))
}

#[cfg(feature = "component-app-assembly")]
pub fn gumball_rotate_params_json(axis: [f64; 3], angle: f64) -> String {
    dsl::json::to_json_string(&dsl::DslValue::object([
        (
            "axis".to_string(),
            dsl::DslValue::object([
                ("$schema".to_string(), dsl::DslValue::String("vector".into())),
                ("x".to_string(), dsl::DslValue::float(axis[0])),
                ("y".to_string(), dsl::DslValue::float(axis[1])),
                ("z".to_string(), dsl::DslValue::float(axis[2])),
            ]),
        ),
        ("angle".to_string(), dsl::DslValue::object([("$schema".to_string(), dsl::DslValue::String("number".into())), ("value".to_string(), dsl::DslValue::float(angle))])),
    ]))
}

#[cfg(feature = "component-app-assembly")]
pub fn gumball_scale_params_json(factor: f64) -> String {
    dsl::json::to_json_string(&dsl::DslValue::object([
        ("factor".to_string(), dsl::DslValue::object([("$schema".to_string(), dsl::DslValue::String("number".into())), ("value".to_string(), dsl::DslValue::float(factor))])),
        (
            "center".to_string(),
            dsl::DslValue::object([("$schema".to_string(), dsl::DslValue::String("point".into())), ("x".to_string(), dsl::DslValue::float(0.0)), ("y".to_string(), dsl::DslValue::float(0.0)), ("z".to_string(), dsl::DslValue::float(0.0))]),
        ),
    ]))
}

/// 🔀️ Finds (or splices in) the transform neuron that persists `selected_id`'s gumball drag for
/// `operation` into the flow graph, rewiring downstream consumers so the transformed geometry is what
/// actually evaluates and exports.
#[cfg(feature = "component-app-assembly")]
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
    let descriptor = dsl::json::to_json_string(&dsl::DslValue::object([
        ("kind".to_string(), dsl::DslValue::String("neuron".into())),
        ("id".to_string(), dsl::DslValue::String(transform_id.clone())),
        ("neuronKind".to_string(), dsl::DslValue::String(gumball_xform_kind(operation).into())),
    ]));
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::FlowFixture;
pub use semio_framework_artifact_flow_flow::Widget;
pub use semio_framework_artifact_playbook_playbook::GenerationPlayState;
//#endregion 🔁️Re-exports
