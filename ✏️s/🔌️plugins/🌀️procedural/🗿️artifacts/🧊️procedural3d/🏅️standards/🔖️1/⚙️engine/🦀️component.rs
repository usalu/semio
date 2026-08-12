//! ⚙️ Procedural3d artifact — headless compute (constitutional: engine).

use crate::apps::procedural3d::config::Procedural3dConfig;
use crate::artifacts::procedural3d::dsl::{
    PROCEDURAL3D_EXAMPLE_BOX_FILLET_TEXT, PROCEDURAL3D_EXAMPLE_BOX_SHELL_TEXT, PROCEDURAL3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT, PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT, PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT, PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT,
    PROCEDURAL3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT, PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT};
use crate::artifacts::procedural3d::{widget_id, Procedural3dSnapshot};
use flow::dag::DagFixture;
use flow::forms_bridge::apply_generation_values_to_fixture;
use flow::{flow_host_with_session, FlowEvalSession, FlowFixture, FlowHost, Widget};
use flow::tessellate_geometry; // crate-root re-export via flow alias
use flow::playbook::{selected_generation, GenerationPlayState};
use serde_json::{json, Value};
use store::ArtifactDsl;

//#region 🔖️Constants
pub const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
pub const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
pub const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";
pub const PROCEDURAL_EXAMPLE_BOX_FILLET: &str = "box-fillet-preview";
pub const PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE: &str = "sphere-box-fuse";
pub const PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE: &str = "face-sweep-extrude";
pub const PROCEDURAL_EXAMPLE_RECTANGLE_WIRE: &str = "rectangle-wire-preview";
pub const PROCEDURAL_EXAMPLE_BOX_SHELL: &str = "box-shell-preview";
//#endregion 🔖️Constants

//#region 🔖️ExtensionContributions
use semio_framework::TopicContribution;
use std::sync::Mutex;

/// 🧩️ One host-aggregated plugin contribution entry (`contributionsJson` wire shape).
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProgramContributionEntry {
    plugin_id: String,
    #[serde(default)]
    topic_contribution: Option<TopicContribution>,
}

const FLOW_EXTENSION_TOPIC: &str = "flow.extension";

/// 🗂️ `flow.extension` topic payload shape, decoded from the open `TopicContribution`.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlowExtensionTopicPayload {
    manifest_json: String,
}

/// 🗂️ Reads the open `TopicContribution` (`"flow.extension"` topic) shape per entry.
fn flow_extension_manifest_json(entry: &ProgramContributionEntry) -> Option<String> {
    let topic_contribution = entry.topic_contribution.as_ref()?;
    if topic_contribution.topic != FLOW_EXTENSION_TOPIC {
        return None;
    }
    topic_contribution.decode::<FlowExtensionTopicPayload>().ok().map(|payload| payload.manifest_json)
}

/// 🔌️ Installs or refreshes contributed `flow.extension` manifests when the host pushes a new catalogue.
pub fn sync_flow_extension_contributions(contributions_json: &str) {
    static LAST: Mutex<String> = Mutex::new(String::new());
    let mut last = LAST.lock().expect("flow contributions lock");
    if *last == contributions_json {
        return;
    }
    for info in flow::installed_flow_extensions() {
        flow::uninstall_flow_extension(&info.id);
    }
    if let Ok(entries) = serde_json::from_str::<Vec<ProgramContributionEntry>>(contributions_json) {
        for entry in entries {
            if let Some(manifest_json) = flow_extension_manifest_json(&entry) {
                flow::install_flow_extension_manifest(&entry.plugin_id, &manifest_json);
            }
        }
    }
    *last = contributions_json.to_string();
}
//#endregion 🔖️ExtensionContributions

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_procedural3d_app` declares via `.artifact_kind(...)`; `params:in`/`geometry:out` are the
/// workflow-specific ports beyond the implicit document in/out ports.
pub fn procedural3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "procedural.3d",
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "3d.procedural".into(), name: "3D Procedural".into(), dimension: "3d".into(), component_kind: "procedural3d".into() },
    )
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::One},
        semio_framework_plugin::MediaPortSpec {
            id: "geometry:out".into(),
            label: "Geometry".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
            kind_id: Some("3d.mesh".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many},
    ])
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
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

pub fn preview_tolerance(lod_mode: &str) -> f64 {
    match lod_mode {
        "coarse" => 0.15,
        "fine" => 0.02,
        _ => 0.05}
}

pub fn preview_camera_json(cfg: &Procedural3dConfig) -> String {
    ui_wgpu::wgpu::world3d_camera_json(cfg.preview_camera.position, cfg.preview_camera.target, cfg.preview_camera.fov)
}

/// 🧭️ World-3d selection payload with the host-owned gumball utility spliced in, so the transform
/// handles follow `cfg.active_utility_id` instead of any document-stored utility.
pub fn preview_selection_json(cfg: &Procedural3dConfig, active_utility: &str) -> String {
    let mut value: Value = serde_json::from_str(&semio_framework_plugin::world3d_selection_json(&cfg.selection_method, &cfg.selected_node_ids, cfg.hovered_node_id.as_deref())).unwrap_or_else(|_| json!({}));
    let show_mode = if cfg.show_mode.is_empty() { "shaded" } else { cfg.show_mode.as_str() };
    let (show_edges, selection_mode) = match show_mode {
        "wireframe" => (true, "mesh"),
        "points" => (false, "mesh"),
        "shaded+edges" => (true, "mesh"),
        _ => (false, "mesh")};
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode".into(), json!(active_utility));
        object.insert("gumballActive".into(), json!(!cfg.selected_node_ids.is_empty()));
        object.insert("showEdges".into(), json!(show_edges));
        object.insert("selectionMode".into(), json!(selection_mode));
        object.insert("granularity".into(), json!(selection_mode));
    }
    value.to_string()
}

fn merge_status_json(computing: Option<String>, preview_status: Option<String>) -> Option<String> {
    match (computing, preview_status) {
        (Some(c), Some(p)) => {
            let mut computing_val: Value = serde_json::from_str(&c).unwrap_or(json!({ "computing": true }));
            let preview_val: Value = serde_json::from_str(&p).unwrap_or(json!({}));
            if let (Some(c_obj), Some(p_obj)) = (computing_val.as_object_mut(), preview_val.as_object()) {
                for (k, v) in p_obj {
                    c_obj.insert(k.clone(), v.clone());
                }
            }
            Some(computing_val.to_string())
        }
        (Some(c), None) => Some(c),
        (None, Some(p)) => Some(p),
        (None, None) => None}
}

/// 👁️ Merges the session's live "still computing" flag with a fresh `preview_status_json` result.
pub fn preview_scene_status_json(session: &FlowEvalSession, preview_status: Option<String>) -> Option<String> {
    let computing = session.pending().then(|| r#"{"computing":true}"#.to_string());
    merge_status_json(computing, preview_status)
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

pub fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() {
        return false;
    }
    if handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
    {
        return true;
    }
    // Blake3 hex digests minted by `BrepKernel::mint` (no kind prefix).
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

pub fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_geometry_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

pub fn geometry_handles_for_widget(eval: &Value, widget_id: &str) -> Vec<String> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"));
    let Some(channels) = channels else {
        return Vec::new();
    };
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles
}

fn mesh_has_preview_geometry(data: &semio_framework_plugin::MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

fn apply_show_mode_mesh(mut data: semio_framework_plugin::MeshData, show_mode: &str) -> semio_framework_plugin::MeshData {
    let show_mode = match show_mode {
        "solid" | "shaded" | "shaded+edges" | "wireframe" | "points" => show_mode,
        _ => "shaded"};
    match show_mode {
        "wireframe" => {
            data.positions.clear();
            data.normals.clear();
            data.indices.clear();
            data.face_ids.clear();
            data
        }
        "points" => {
            data.indices.clear();
            data.normals.clear();
            data.edge_positions.clear();
            data
        }
        _ => data}
}

pub fn preview_status_json(eval_json: &str, fixture: &FlowFixture) -> Option<String> {
    let eval: Value = serde_json::from_str(eval_json).ok()?;
    if eval.get("error").and_then(Value::as_str).is_some() {
        return Some(json!({ "error": eval.get("error") }).to_string());
    }
    let mut errors = serde_json::Map::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let Some(entry) = eval.get(&id) else { continue };
        if let Some(error) = entry.get("error").and_then(Value::as_str) {
            errors.insert(id, Value::String(error.to_string()));
        }
    }
    if errors.is_empty() {
        None
    } else {
        Some(json!({ "widgetErrors": errors }).to_string())
    }
}

/// 🧵️ Pure per-render tessellation: bounded-cost, safe to call fresh on every render call instead of
/// behind an outer memoization layer.
fn mesh_data_for_preview_handle(handle: &str, tolerance: f64, session: Option<&FlowEvalSession>) -> Option<semio_framework_plugin::MeshData> {
    if let Some(session) = session {
        if let Some(json) = session.preview_mesh_json(handle) {
            if let Ok(value) = serde_json::from_str::<Value>(json) {
                if value.get("error").is_none() {
                    if let Ok(data) = serde_json::from_value::<semio_framework_plugin::MeshData>(value) {
                        if mesh_has_preview_geometry(&data) {
                            return Some(data);
                        }
                    }
                }
            }
        }
    }
    let data = tessellate_geometry(handle, tolerance).ok()?;
    mesh_has_preview_geometry(&data).then_some(data)
}

/// 🧊 Geometry handles on preview widgets that still need an extension tessellate.
pub fn pending_preview_tessellate_handles(eval_json: &str, fixture: &FlowFixture, session: &FlowEvalSession) -> Vec<String> {
    if eval_json.is_empty() {
        return Vec::new();
    }
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    let mut handles = Vec::new();
    for widget in &fixture.widgets {
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let id = widget_id(widget).to_string();
        for handle in geometry_handles_for_widget(&eval, &id) {
            let ready = session.preview_mesh_json(&handle).and_then(|json| {
                let value = serde_json::from_str::<Value>(json).ok()?;
                if value.get("error").is_some() {
                    return None;
                }
                let data = serde_json::from_value::<semio_framework_plugin::MeshData>(value).ok()?;
                mesh_has_preview_geometry(&data).then_some(())
            });
            if ready.is_none() {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 📨 Host effects that tessellate preview handles inside the owning brep extension kernel.
pub fn preview_tessellate_effects(session: &mut FlowEvalSession, eval_json: &str, fixture: &FlowFixture, cfg: &Procedural3dConfig) -> Vec<semio_framework_plugin::HostEffect> {
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let tolerance_bits = tolerance.to_bits();
    let mut live = std::collections::HashSet::new();
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        for handle in geometry_handles_for_widget(&eval, &id) {
            live.insert(handle);
        }
    }
    session.retain_preview_meshes(&live);
    let mut effects = Vec::new();
    for handle in pending_preview_tessellate_handles(eval_json, fixture, session) {
        let node_hash = flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone()) {
            effects.push(semio_framework_plugin::HostEffect::InvokeExtension {
                extension_id: "brep".into(),
                capability: "tessellate".into(),
                request_json: json!({ "handle": handle, "tolerance": tolerance, "nodeHash": node_hash }).to_string(),
                response_action: "flowTessellateResolve".into()});
        }
    }
    effects
}

pub fn preview_payload_from_eval(eval_json: &str, fixture: &FlowFixture, cfg: &Procedural3dConfig) -> (String, String) {
    preview_payload_from_eval_with_session(eval_json, fixture, cfg, None)
}

pub fn preview_payload_from_eval_with_session(eval_json: &str, fixture: &FlowFixture, cfg: &Procedural3dConfig, session: Option<&FlowEvalSession>) -> (String, String) {
    if eval_json.is_empty() {
        return ("[]".into(), "[]".into());
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(eval_json) {
        if parsed.get("error").and_then(Value::as_str).is_some() {
            return ("[]".into(), "[]".into());
        }
    }
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let handles = geometry_handles_for_widget(&eval, &id);
        if handles.is_empty() {
            continue;
        }
        let selected = cfg.selected_node_ids.iter().any(|entry| entry == &id);
        let hovered = cfg.hovered_node_id.as_deref() == Some(id.as_str());
        for (index, handle) in handles.iter().enumerate() {
            let mesh_id = if handles.len() == 1 { format!("eval-{id}") } else { format!("eval-{id}#{index}") };
            let instance_id = if handles.len() == 1 { id.clone() } else { format!("{id}#{index}") };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                if let Some(data) = mesh_data_for_preview_handle(handle, tolerance, session) {
                    let data = apply_show_mode_mesh(data, show_mode);
                    if mesh_has_preview_geometry(&data) {
                        meshes.push(json!({ "id": mesh_id, "data": data }));
                    }
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                instances.push(json!({
                    "id": instance_id,
                    "meshId": mesh_id,
                    "position": [0.0, 0.0, 0.0],
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0],
                    "label": id,
                    "selected": selected,
                    "hovered": hovered}));
            }
        }
    }
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
    host.evaluate().unwrap_or_default()
}

pub fn merge_preview_meshes(meshes: &[semio_framework_plugin::MeshData]) -> semio_framework_plugin::MeshData {
    let mut merged = semio_framework_plugin::MeshData::default();
    for mesh in meshes {
        let vertex_offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(&mesh.positions);
        merged.normals.extend(&mesh.normals);
        merged.colors.extend(&mesh.colors);
        merged.indices.extend(mesh.indices.iter().map(|index| index + vertex_offset));
        merged.edge_positions.extend(&mesh.edge_positions);
        if !mesh.edge_ids.is_empty() {
            let edge_base = merged.edge_ids.len() as u32;
            merged.edge_ids.extend(mesh.edge_ids.iter().map(|id| id + edge_base));
        }
    }
    merged
}

pub fn export_mesh_from_document(projection: &Procedural3dSnapshot) -> semio_framework_plugin::MeshData {
    let config = Procedural3dConfig::default();
    let mut host = host_from_fixture(&projection.fixture);
    let eval_json = host.evaluate().unwrap_or_default();
    let (meshes_json, _) = preview_payload_from_eval(&eval_json, &projection.fixture, &config);
    let meshes: Vec<semio_framework_plugin::MeshData> = serde_json::from_str::<Vec<Value>>(&meshes_json).unwrap_or_default().into_iter().filter_map(|entry| serde_json::from_value(entry.get("data").cloned().unwrap_or(Value::Null)).ok()).collect();
    merge_preview_meshes(&meshes)
}

pub fn procedural3d_mesh_from_document(doc: &Value) -> Result<semio_framework_plugin::MeshData, String> {
    let projection: Procedural3dSnapshot = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_document(&projection))
}

pub fn procedural3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(default_snapshot()).map_err(|err| err.to_string())
}

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
    json!({ "offset": { "$schema": "vector", "x": offset[0], "y": offset[1], "z": offset[2] } }).to_string()
}

pub fn gumball_rotate_params_json(axis: [f64; 3], angle: f64) -> String {
    json!({
        "axis": { "$schema": "vector", "x": axis[0], "y": axis[1], "z": axis[2] },
        "angle": { "$schema": "number", "value": angle }})
    .to_string()
}

pub fn gumball_scale_params_json(factor: f64) -> String {
    json!({
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
    let descriptor = json!({ "kind": "neuron", "id": transform_id, "neuronKind": gumball_xform_kind(operation) }).to_string();
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

/// 🔌️ Registers this artifact's plugin-level exports — mesh export/import bridges, DWG mesh bridge,
/// pack<->dsl document codec. Called once from the plugin-root `📦️glue.rs`'s `semio_plugin!` `setup:`.
/// 📎 Registers the procedural3d artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::procedural3d::schema::procedural3d_artifact_schema_descriptor());
}

pub fn register() {
    crate::artifacts::procedural3d::io_registry::register();

    register_artifact_schema();
    register_pilot_languages();
    semio_framework_os::register_mesh_dwg_import_handler("3d.procedural", procedural3d_document_from_mesh);
    // 📦️ Registers `Procedural3dSnapshot`'s pack<->dsl codec so `framework/sync`'s `FolderEndpoint`
    // can print/parse `.procedural3d` packs without depending on this crate's concrete types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::procedural3d::Procedural3dPlayApp>(crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA);
}
//#endregion 🔖️DocumentHelpers


/// 🔗 Registers in-process flow extension operators so eval + tessellate share one brep kernel.
/// Safe to call repeatedly; installers are registered once and the host registry is rebuilt.
pub fn ensure_linked_flow_extensions() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        flow::register_linked_flow_extension_installer("brep", semio_s_plugin_flow_extension_brep::register);
        flow::register_linked_flow_extension_installer("math", semio_s_plugin_flow_extension_math::register);
        flow::register_linked_flow_extension_installer("primitive", semio_s_plugin_flow_extension_primitive::register);
        flow::register_linked_flow_extension_installer("logic", semio_s_plugin_flow_extension_logic::register);
        flow::register_linked_flow_extension_installer("dictionary", semio_s_plugin_flow_extension_dictionary::register);
        flow::register_linked_flow_extension_installer("list", semio_s_plugin_flow_extension_list::register);
        flow::register_linked_flow_extension_installer("text", semio_s_plugin_flow_extension_text::register);
        flow::sync_host_flow_extension_contributions("[]");
    });
}

//#region 🧪️TestSupport
/// 🧵️ `tessellate_geometry` (flow core brep geometry session) (and the flow-eval neuron kernel cache it sits behind)
/// is a process-wide cache shared by every test in this ONE merged crate — before the crate
/// consolidation, the artifact/app constitutional crates each ran in their own `cargo test` process, so
/// a `TEST_SERIAL` local to one of them never had to coordinate with the other's. Now that every
/// taxonomy node's tests share one test binary, ANY test that evaluates a flow fixture and/or tessellates
/// BRep geometry (directly here, or indirectly via the app's preview-window `render()`) must acquire
/// THIS single crate-wide lock — see `crate::apps::procedural3d::modes::edit::windows::preview`'s test
/// for the app-side half of this.
#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::{Mutex, MutexGuard};

    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    pub fn lock() -> MutexGuard<'static, ()> {
        TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
//#endregion 🧪️TestSupport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_3d::scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3};
    use std::sync::MutexGuard;

    fn test_serial() -> MutexGuard<'static, ()> {
        test_support::lock()
    }

    fn preview_payload_from_evaluated_fixture(fixture: &FlowFixture, cfg: &Procedural3dConfig) -> (String, String) {
        ensure_linked_flow_extensions();
        let mut host = FlowHost::from_fixture(fixture.clone());
        host.set_neuron_kind_infos_json(&flow::flow_neuron_kind_infos_json());
        let eval_json = host.evaluate().unwrap_or_default();
        preview_payload_from_eval(&eval_json, fixture, cfg)
    }

    #[test]
    fn preview_payload_has_meshes_and_instances() {
        let _serial = test_serial();
        let projection = default_snapshot();
        let config = Procedural3dConfig::default();
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        assert_ne!(meshes_json, "[]", "meshes_json was empty");
        assert_ne!(instances_json, "[]", "instances_json was empty");
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
        let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
        assert!(!meshes.is_empty());
        assert!(!instances.is_empty());
        for mesh in &meshes {
            let id = mesh.get("id").and_then(|value| value.as_str()).unwrap_or("");
            assert!(id.starts_with("eval-"), "mesh id must be tessellated eval handle, got {id}");
            let data: semio_framework::MeshData = serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
            assert!(data.positions.len() >= 9, "mesh has too few positions");
            assert!(data.indices.len() >= 3, "mesh has too few indices");
            assert!(!data.edge_positions.is_empty(), "brep preview should include edge geometry");
        }
        let camera = Camera3d {
            position: Vec3::from_array([config.preview_camera.position[0] as f32, config.preview_camera.position[1] as f32, config.preview_camera.position[2] as f32]),
            target: Vec3::from_array([config.preview_camera.target[0] as f32, config.preview_camera.target[1] as f32, config.preview_camera.target[2] as f32]),
            up: Vec3::new(0.0, 0.0, 1.0),
            fov_y: config.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
            near: 0.1,
            far: 1000.0};
        let view_proj = camera.view_proj(0.6);
        let planes = frustum_planes(view_proj);
        let mut visible = 0usize;
        for instance in instances {
            let mesh_id = instance.get("meshId").or_else(|| instance.get("mesh_id")).and_then(|value| value.as_str()).unwrap_or("eval-missing");
            let mesh = meshes.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id)).expect("mesh record");
            let data: semio_framework::MeshData = serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
            let mesh3d = Mesh3d::from_buffers(data.positions, data.normals, data.indices);
            let position =
                instance.get("position").and_then(|value| value.as_array()).map_or([0.0, 0.0, 0.0], |items| [items[0].as_f64().unwrap_or(0.0) as f32, items[1].as_f64().unwrap_or(0.0) as f32, items[2].as_f64().unwrap_or(0.0) as f32]);
            assert_eq!(position, [0.0, 0.0, 0.0], "preview instances stay in world space");
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
            let (min, max) = transform_aabb(model, mesh3d.aabb_min, mesh3d.aabb_max);
            if aabb_intersects_frustum(&planes, min, max) {
                visible += 1;
            }
        }
        assert!(visible > 0, "no preview instances intersect camera frustum");
    }

    #[test]
    fn document_from_mesh_returns_valid_default_snapshot() {
        let _serial = test_serial();
        let mesh = semio_framework_plugin::MeshData::default();
        let document = procedural3d_document_from_mesh(&mesh).expect("dwg mesh import document");
        let projection: Procedural3dSnapshot = serde_json::from_value(document).expect("parseable projection");
        assert_eq!(projection.fixture.schema, "flow.fixture");
    }

    #[test]
    fn procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
        let _serial = test_serial();
        use semio_framework_plugin::{GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter};
        let document_json = serde_json::to_value(default_snapshot()).expect("projection json");
        let mesh = procedural3d_mesh_from_document(&document_json).expect("mesh from document");
        assert!(!mesh.positions.is_empty());

        let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
        let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
        let obj_document = procedural3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
        let _: Procedural3dSnapshot = serde_json::from_value(obj_document).expect("parseable obj projection");

        let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
        let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
        let glb_document = procedural3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
        let _: Procedural3dSnapshot = serde_json::from_value(glb_document).expect("parseable glb projection");

        let stl_bytes = StlExporter.export(&mesh).expect("stl export");
        let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
        let stl_document = procedural3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
        let _: Procedural3dSnapshot = serde_json::from_value(stl_document).expect("parseable stl projection");
    }

    #[test]
    fn rectangle_wire_preview_emits_edge_only_mesh() {
        let _serial = test_serial();
        let projection = Procedural3dSnapshot::parse_dsl(PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
        let config = Procedural3dConfig::default();
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty(), "rectangle wire preview should tessellate curve edges");
        let data: semio_framework::MeshData = serde_json::from_value(meshes[0].get("data").cloned().unwrap_or_default()).expect("mesh data");
        assert!(data.indices.is_empty(), "wire preview has no shaded triangles");
        assert!(data.edge_positions.len() >= 6, "curve preview should include edge polylines");
        assert!(!instances_json.is_empty());
    }

    #[test]
    fn all_bundled_examples_emit_preview_meshes() {
        let _serial = test_serial();
        let config = Procedural3dConfig::default();
        let cases = [
            ("hexagonal-mushroom-column", PROCEDURAL_EXAMPLE_HEX_COLUMN),
            ("rectangle-extrude-volume", PROCEDURAL_EXAMPLE_RECT_EXTRUDE),
            ("sphere-cut-with-torus", PROCEDURAL_EXAMPLE_SPHERE_TORUS),
            ("box-fillet-preview", PROCEDURAL_EXAMPLE_BOX_FILLET),
            ("sphere-box-fuse", PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE),
            ("face-sweep-extrude", PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE),
            ("rectangle-wire-preview", PROCEDURAL_EXAMPLE_RECTANGLE_WIRE),
            ("box-shell-preview", PROCEDURAL_EXAMPLE_BOX_SHELL),
        ];
        for (label, example_id) in cases {
            let projection = example_snapshot(example_id).unwrap_or_else(|| panic!("{label}: missing projection"));
            let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
            assert_ne!(meshes_json, "[]", "{label}: meshes empty; eval may have failed");
            assert_ne!(instances_json, "[]", "{label}: instances empty");
            let meshes: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_else(|err| panic!("{label}: meshes json: {err}"));
            assert!(!meshes.is_empty(), "{label}: no mesh entries");
        }
    }

    #[test]
    fn preview_tolerance_follows_lod_mode() {
        assert!((preview_tolerance("coarse") - 0.15).abs() < 1e-9);
        assert!((preview_tolerance("fine") - 0.02).abs() < 1e-9);
        assert!((preview_tolerance("") - 0.05).abs() < 1e-9);
    }

    #[test]
    fn wireframe_show_mode_strips_shaded_triangles() {
        let _serial = test_serial();
        let projection = default_snapshot();
        let config = Procedural3dConfig { show_mode: "wireframe".into(), ..Default::default() };
        let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty());
        let data: semio_framework::MeshData = serde_json::from_value(meshes[0].get("data").cloned().unwrap_or_default()).expect("mesh data");
        assert!(data.indices.is_empty());
        assert!(!data.edge_positions.is_empty());
    }

    #[test]
    fn procedural3d_io_declares_the_params_and_geometry_ports() {
        let io = procedural3d_io();
        assert_eq!(io.document_schema, "procedural.3d");
        assert_eq!(io.artifact.id, "3d.procedural");
        let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert_eq!(params.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(!params.required);
        let geometry = io.ports.iter().find(|port| port.id == "geometry:out").expect("geometry:out declared");
        assert_eq!(geometry.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(geometry.kind_id.as_deref(), Some("3d.mesh"));
        assert_eq!(geometry.multiplicity, semio_framework::PortMultiplicity::Many);
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural.procedural3d.document",
        extension: Some("procedural3d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::procedural3d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::procedural3d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural.procedural3d.document")});
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural.procedural3d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::procedural3d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::procedural3d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural.procedural3d.op")});
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural.procedural3d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::procedural3d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::procedural3d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("procedural.procedural3d.diff")});
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural3d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural3d.pack")});
    dsl::register_language(dsl::LanguageSpec {
        id: "procedural3d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("procedural3d.spr")});
}


//#region 🔖️ArtifactEngine
pub struct Procedural3dEngine {
    artifact: crate::artifacts::procedural3d::schema::Procedural3dArtifact,
    snapshot: crate::artifacts::procedural3d::Procedural3dSnapshot}

impl Procedural3dEngine {
    pub fn new(snapshot: crate::artifacts::procedural3d::Procedural3dSnapshot) -> Self {
        let artifact = crate::artifacts::procedural3d::schema::Procedural3dArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::Procedural3dComposer as Procedural3dAnyComposer;
    use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::Procedural3dBuilder as Procedural3dAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const PROCEDURAL3D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural3d", standard: StandardId("1"), subset: SubsetId("*") };
    const PROCEDURAL3D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::procedural3d::Procedural3dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PROCEDURAL3D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Procedural3dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Procedural3dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Procedural3dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PROCEDURAL3D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::procedural3d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Procedural3dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_las(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_ply(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::procedural3d::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<Procedural3dAnyComposer>(),
            ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_las },
            ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_ply },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_gltf },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[PROCEDURAL3D_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
