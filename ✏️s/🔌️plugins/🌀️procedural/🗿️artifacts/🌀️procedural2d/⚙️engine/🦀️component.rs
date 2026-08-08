//! ⚙️ Procedural2d artifact — headless compute (constitutional: engine).

use crate::apps::procedural2d::config::Procedural2dConfig;
use crate::artifacts::procedural2d::dsl::PROCEDURAL2D_EXAMPLE_TEXT;
use crate::artifacts::procedural2d::Procedural2dDocument;
use flow::dag::DagFixture;
use flow::forms_bridge::apply_generation_values_to_fixture;
use flow::{flow_host_with_session, flow_neuron_kind_infos_json, FlowEvalSession, FlowFixture, FlowHost};
use flow::render_scene_json;
use flow::playbook::{selected_generation, GenerationPlayState};
use serde_json::{json, Value};
use store::DocumentDsl;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_procedural2d_app`'s
/// `.artifact_kind(...)` document schema/media type verbatim, plus two workflow ports: `params:in`
/// (generic Data×Value parametric input) and `drawing:out` (TwoD×Vector, tagged with draw's already-
/// registered `2d.drawing` kind id).
pub fn procedural2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "procedural.2d",
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "2d.procedural".into(), name: "2D Procedural".into(), dimension: "2d".into(), component_kind: "procedural2d".into() },
    )
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::One,
        },
        semio_framework_plugin::MediaPortSpec {
            id: "drawing:out".into(),
            label: "Drawing".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
            kind_id: Some("2d.drawing".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
pub fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    host
}

pub fn host_from_fixture_with_session(fixture: &FlowFixture, session: &FlowEvalSession) -> FlowHost {
    flow_host_with_session(fixture, session)
}

/// 🔀️ Runs a host mutation seeded from the projection fixture and diffs the result into operations.
/// Diffs against the host-normalized baseline (not the raw projection) so `FlowHost`'s own
/// dedupe/dag-rebuild normalization does not leak spurious collection operations — only the actual
/// mutation becomes an operation, which keeps concurrent disjoint edits mergeable on the backbone.
pub fn host_operations(fixture: &FlowFixture, mutate: impl FnOnce(&mut FlowHost)) -> Vec<crate::artifacts::procedural2d::op::Procedural2dMutation> {
    let mut host = host_from_fixture(fixture);
    let baseline = host.fixture.clone();
    mutate(&mut host);
    crate::artifacts::procedural2d::op::procedural2d_fixture_operations(&baseline, &host.fixture)
}

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

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

pub fn collect_drawing_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
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

pub fn affine_transform_array(value: &Value) -> [f64; 6] {
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

pub fn path_segments_from_node(node: &Value) -> Vec<Value> {
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

pub fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<Value> {
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
                "needsKernel": false,
            })
        })
        .collect()
}

pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    host.evaluate().unwrap_or_default()
}

pub fn generation_preview_layers(eval_json: &str) -> String {
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

/// 👁️ Recomputes the ephemeral generation preview for the currently selected generation and
/// stores it on the config (never on the persisted document).
pub fn refresh_generation_preview(config: &mut Procedural2dConfig, fixture: &FlowFixture, generation: &GenerationPlayState) {
    let Some(selected) = selected_generation(generation) else {
        config.generation_preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(fixture, &selected.values);
    config.generation_preview_text = Some(preview);
}

/// 📄️ The `procedural2d-play` "default" document — parsed from the bundled `.procedural2d` example
/// fixture, falling back to the empty document if the fixture ever fails to parse.
pub fn default_projection() -> Procedural2dDocument {
    Procedural2dDocument::parse_dsl(PROCEDURAL2D_EXAMPLE_TEXT).unwrap_or_default()
}

pub fn empty_procedural2d_projection() -> Procedural2dDocument {
    Procedural2dDocument::default()
}

pub fn procedural2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Procedural 2D", 1024, 768)
}

pub fn procedural2d_document_from_dwg(_drawing: &semio_framework::DwgDrawing) -> Result<Value, String> {
    serde_json::to_value(default_projection()).map_err(|err| err.to_string())
}

/// 🔌️ Registers this artifact's plugin-level exports — pack<->dsl document codec, mesh/svg export
/// bridges. Called once from the plugin-root `📦️glue.rs`'s `semio_plugin!` `setup:`.
pub fn register() {
    register_pilot_languages();
    semio_framework_os::register_2d_export_handlers("2d.procedural", "procedural2d", procedural2d_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.procedural", procedural2d_document_from_dwg);
    // 📦️ Registers `Procedural2dDocument`'s pack<->dsl codec so `framework/sync`'s `FolderEndpoint`
    // can print/parse `.procedural2d` packs without depending on this crate's concrete types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::procedural2d::Procedural2dPlayApp>(crate::artifacts::procedural2d::PROCEDURAL_2D_SCHEMA);
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_projection_parses_the_bundled_example() {
        assert!(!default_projection().fixture.widgets.is_empty());
    }

    #[test]
    fn document_from_dwg_returns_valid_default_projection() {
        let drawing = semio_framework::DwgDrawing::default();
        let document = procedural2d_document_from_dwg(&drawing).expect("dwg import document");
        let projection: Procedural2dDocument = serde_json::from_value(document).expect("parseable projection");
        assert_eq!(projection.fixture.schema, "flow.fixture");
    }

    #[test]
    fn procedural2d_io_declares_the_params_and_drawing_ports() {
        let io = procedural2d_io();
        assert_eq!(io.document_schema, "procedural.2d");
        let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert!(!params.required);
        let drawing = io.ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
        assert_eq!(drawing.kind_id.as_deref(), Some("2d.drawing"));
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural.procedural2d.document",
        extension: Some("procedural2d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::procedural2d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::procedural2d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::procedural2d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural2d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural.procedural2d.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural.procedural2d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::procedural2d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::procedural2d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural.procedural2d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural.procedural2d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::procedural2d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::procedural2d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("procedural.procedural2d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural2d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::procedural2d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural2d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural2d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural2d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural2d.spr"),
    });
}


//#region 🔖️ArtifactEngine
pub struct Procedural2dEngine {
    projection: crate::artifacts::procedural2d::Procedural2dDocument,
}

impl Procedural2dEngine {
    pub fn new(projection: crate::artifacts::procedural2d::Procedural2dDocument) -> Self {
        Self { projection }
    }
}

impl protocol::ArtifactEngine for Procedural2dEngine {
    type Projection = crate::artifacts::procedural2d::Procedural2dDocument;
    type Mutation = crate::artifacts::procedural2d::mutations::Procedural2dMutation;
    type Diff = crate::artifacts::procedural2d::diff::Procedural2dDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::procedural2d::mutations::apply_procedural2d_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
