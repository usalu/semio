//! 🎬 Media graph, VFS projection types, and media export registry.

use crate::instance::{
    is_parameter_port_id, media_port_spec_id, parameter_id_from_port_id,
    parameter_port_id, OsAppInstance, OsParameter, OsParameterFieldBinding,
};
use crate::registry::{os_app_primary_output_kind, os_app_registration, OsAppRegistration};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

pub const OS_STUDIO_SCHEMA: &str = "s.studio";
pub const OS_MEDIA_GRAPH_SCHEMA: &str = "s.media-graph";
pub const OS_MEDIA_GRAPH_VFS_ROOT_ID: &str = "os-media-graph-root";
pub const OS_MEDIA_FLOW_MODULE_ID: &str = "os-media";

//#region 🔖MediaGraph
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaPort {
    pub id: String,
    pub resource_kind: String,
    pub direction: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphNode {
    pub id: String,
    pub instance_id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub inputs: Vec<OsMediaPort>,
    pub outputs: Vec<OsMediaPort>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraph {
    pub schema: String,
    pub nodes: Vec<OsMediaGraphNode>,
    pub edges: Vec<OsMediaGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaGraphPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediaGraphValidation {
    pub ok: bool,
    pub errors: Vec<String>,
}

pub fn empty_media_graph() -> OsMediaGraph {
    OsMediaGraph {
        schema: OS_MEDIA_GRAPH_SCHEMA.into(),
        nodes: Vec::new(),
        edges: Vec::new(),
    }
}

pub fn media_graph_node_for_instance(
    instance: &OsAppInstance,
    registration: &OsAppRegistration,
    position: &MediaGraphPosition,
    node_id: &str,
) -> OsMediaGraphNode {
    let (inputs, outputs) =
        crate::registry::media_graph_node_ports_for_registration(&instance.id, registration);
    let port_count = inputs.len().max(outputs.len()).max(1);
    OsMediaGraphNode {
        id: node_id.into(),
        instance_id: instance.id.clone(),
        x: position.x,
        y: position.y,
        width: 220.0,
        height: 56.0 + port_count as f64 * 18.0,
        inputs,
        outputs,
    }
}

fn sync_media_node_parameter_ports(
    node: &OsMediaGraphNode,
    bindings: &[OsParameterFieldBinding],
) -> OsMediaGraphNode {
    let instance_bindings: Vec<_> = bindings
        .iter()
        .filter(|binding| binding.instance_id == node.instance_id)
        .collect();
    let base_inputs: Vec<_> = node
        .inputs
        .iter()
        .filter(|port| !is_parameter_port_id(&port.id))
        .cloned()
        .collect();
    let parameter_inputs: Vec<_> = instance_bindings
        .iter()
        .map(|binding| OsMediaPort {
            id: parameter_port_id(&node.instance_id, &binding.parameter_id),
            resource_kind: "parameter.value".into(),
            direction: "in".into(),
        })
        .collect();
    let inputs: Vec<_> = base_inputs.into_iter().chain(parameter_inputs).collect();
    let port_count = inputs.len().max(node.outputs.len()).max(1);
    OsMediaGraphNode {
        inputs,
        height: 56.0 + port_count as f64 * 18.0,
        ..node.clone()
    }
}

pub fn sync_media_graph_parameter_ports(
    graph: &OsMediaGraph,
    bindings: &[OsParameterFieldBinding],
) -> OsMediaGraph {
    OsMediaGraph {
        schema: OS_MEDIA_GRAPH_SCHEMA.into(),
        nodes: graph
            .nodes
            .iter()
            .map(|node| sync_media_node_parameter_ports(node, bindings))
            .collect(),
        edges: graph.edges.clone(),
    }
}

/// @emoji ✅ Validates media graph connectivity and cycle freedom.
pub fn validate_media_graph(graph: &OsMediaGraph) -> MediaGraphValidation {
    let mut errors = Vec::new();
    let node_ids: HashSet<_> = graph.nodes.iter().map(|node| node.id.clone()).collect();
    for edge in &graph.edges {
        if !node_ids.contains(&edge.source_node_id) {
            errors.push(format!("missing source node {}", edge.source_node_id));
        }
        if !node_ids.contains(&edge.target_node_id) {
            errors.push(format!("missing target node {}", edge.target_node_id));
        }
    }
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        adjacency
            .entry(edge.source_node_id.clone())
            .or_default()
            .push(edge.target_node_id.clone());
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    fn dfs(
        node_id: &str,
        adjacency: &HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        errors: &mut Vec<String>,
    ) {
        if visited.contains(node_id) {
            return;
        }
        if visiting.contains(node_id) {
            errors.push(format!("cycle detected at {node_id}"));
            return;
        }
        visiting.insert(node_id.to_string());
        for next in adjacency.get(node_id).into_iter().flatten() {
            dfs(next, adjacency, visiting, visited, errors);
        }
        visiting.remove(node_id);
        visited.insert(node_id.to_string());
    }
    for node in &graph.nodes {
        dfs(&node.id, &adjacency, &mut visiting, &mut visited, &mut errors);
    }
    MediaGraphValidation {
        ok: errors.is_empty(),
        errors,
    }
}

pub fn os_media_neuron_kind_for_node(node_id: &str) -> String {
    format!("os.media.node.{node_id}")
}

pub fn os_media_graph_to_flow_fixture(
    graph: &OsMediaGraph,
    instances: &[OsAppInstance],
) -> Value {
    let instance_by_id: HashMap<_, _> = instances.iter().map(|instance| (instance.id.clone(), instance)).collect();
    let widgets: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            json!({
                "kind": "neuron",
                "id": node.id,
                "neuronKind": os_media_neuron_kind_for_node(&node.id),
                "inputPorts": node.inputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                "outputPorts": node.outputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                "params": {
                    "instanceId": node.instance_id,
                    "programId": instance.map(|entry| &entry.program_id).unwrap_or(&String::new()),
                    "appId": instance.map(|entry| &entry.app_id).unwrap_or(&String::new()),
                },
                "preview": true,
            })
        })
        .collect();
    let layout: HashMap<_, _> = graph
        .nodes
        .iter()
        .map(|node| {
            (
                node.id.clone(),
                json!({ "x": node.x + node.width / 2.0, "y": node.y + node.height / 2.0 }),
            )
        })
        .collect();
    let synapses: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "from": edge.source_node_id,
                "to": edge.target_node_id,
                "fromPort": edge.source_port_id,
                "toPort": edge.target_port_id,
            })
        })
        .collect();
    json!({
        "schema": "flow.fixture",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "widgets": widgets,
        "synapses": synapses,
        "layout": layout,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaNodeGraphPayload {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
    pub find_items_json: String,
}

/** @emoji 🕸️ Serializes an OS media graph into generic node-graph scene payloads. */
pub fn os_media_graph_to_node_graph_payload(
    graph: &OsMediaGraph,
    instances: &[OsAppInstance],
) -> OsMediaNodeGraphPayload {
    let instance_by_id: HashMap<_, _> = instances.iter().map(|instance| (instance.id.clone(), instance)).collect();
    let nodes: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            let label = instance
                .map(|entry| format!("{} / {}", entry.program_id, entry.app_id))
                .unwrap_or_else(|| node.instance_id.clone());
            json!({
                "id": node.id,
                "instanceId": node.instance_id,
                "label": label,
                "x": node.x,
                "y": node.y,
                "width": node.width,
                "height": node.height,
                "inputs": node.inputs.iter().map(|port| json!({
                    "id": port.id,
                    "resourceKind": port.resource_kind,
                    "direction": port.direction,
                    "label": port.id,
                })).collect::<Vec<_>>(),
                "outputs": node.outputs.iter().map(|port| json!({
                    "id": port.id,
                    "resourceKind": port.resource_kind,
                    "direction": port.direction,
                    "label": port.id,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let edges: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "sourceNodeId": edge.source_node_id,
                "sourcePortId": edge.source_port_id,
                "targetNodeId": edge.target_node_id,
                "targetPortId": edge.target_port_id,
            })
        })
        .collect();
    let find_items: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            json!({
                "id": node.instance_id,
                "label": instance
                    .map(|entry| format!("{} / {}", entry.program_id, entry.app_id))
                    .unwrap_or_else(|| node.instance_id.clone()),
                "category": "Media graph",
            })
        })
        .collect();
    OsMediaNodeGraphPayload {
        nodes_json: serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        edges_json: serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
        viewport_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
        find_items_json: serde_json::to_string(&find_items).unwrap_or_else(|_| "[]".into()),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaFlowChannelSpec {
    pub name: String,
    pub code: String,
    pub abbreviation: String,
    pub full_name: String,
    pub operators: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaFlowOperatorInfo {
    pub id: String,
    pub module: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<OsMediaFlowChannelSpec>,
    pub outputs: Vec<OsMediaFlowChannelSpec>,
}

fn os_media_flow_channel_spec(port_id: &str, resource_kind: &str, label: &str) -> OsMediaFlowChannelSpec {
    let code = port_id
        .chars()
        .next()
        .map(|ch| ch.to_uppercase().collect::<String>())
        .unwrap_or_else(|| "P".into());
    let abbreviation = if label.chars().count() <= 3 {
        label.into()
    } else {
        label.chars().take(3).collect()
    };
    OsMediaFlowChannelSpec {
        name: port_id.into(),
        code,
        abbreviation,
        full_name: label.into(),
        operators: vec![resource_kind.into()],
    }
}

fn parameter_label(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { name, .. }
        | OsParameter::Categorical { name, .. }
        | OsParameter::Toggle { name, .. }
        | OsParameter::Text { name, .. } => name,
    }
}

fn parameter_entity_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

/// @emoji 🧩 Registers per-node neuron metadata for the OS media graph flow extension.
pub fn build_os_media_flow_operator_infos(
    graph: &OsMediaGraph,
    instances: &[OsAppInstance],
    parameters: &[OsParameter],
) -> Vec<OsMediaFlowOperatorInfo> {
    let instance_by_id: HashMap<_, _> = instances.iter().map(|row| (row.id.clone(), row)).collect();
    let parameter_by_id: HashMap<_, _> = parameters
        .iter()
        .map(|row| (parameter_entity_id(row).to_string(), row))
        .collect();
    graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            let registration = instance
                .and_then(|row| os_app_registration(&row.program_id, &row.app_id));
            let neuron_kind = os_media_neuron_kind_for_node(&node.id);
            OsMediaFlowOperatorInfo {
                id: neuron_kind,
                module: OS_MEDIA_FLOW_MODULE_ID.into(),
                name: instance
                    .map(|row| row.label.clone())
                    .unwrap_or_else(|| node.instance_id.clone()),
                abbreviation: instance
                    .map(|row| {
                        if row.app_id.chars().count() <= 3 {
                            row.app_id.clone()
                        } else {
                            row.app_id.chars().take(3).collect()
                        }
                    })
                    .unwrap_or_else(|| "app".into()),
                icon: format!(
                    "emoji:{}",
                    registration
                        .map(|row| row.component_kind.clone())
                        .unwrap_or_else(|| "s".into())
                ),
                summary: instance
                    .map(|row| format!("{}/{}", row.program_id, row.app_id))
                    .unwrap_or_else(|| "App instance".into()),
                inputs: node
                    .inputs
                    .iter()
                    .map(|port| {
                        let parameter_id = parameter_id_from_port_id(&port.id);
                        let label = parameter_id
                            .as_ref()
                            .and_then(|id| parameter_by_id.get(id))
                            .map(|parameter| parameter_label(*parameter).to_string())
                            .or_else(|| media_port_spec_id(&port.id))
                            .unwrap_or_else(|| port.id.clone());
                        os_media_flow_channel_spec(&port.id, &port.resource_kind, &label)
                    })
                    .collect(),
                outputs: node
                    .outputs
                    .iter()
                    .map(|port| {
                        let label = media_port_spec_id(&port.id).unwrap_or_else(|| port.id.clone());
                        os_media_flow_channel_spec(&port.id, &port.resource_kind, &label)
                    })
                    .collect(),
            }
        })
        .collect()
}
//#endregion 🔖MediaGraph

//#region 🔖ProgramRegistry
#[derive(Clone, Debug, Default)]
pub struct ProgramRegistry {
    instances: HashMap<String, OsAppInstance>,
}

impl ProgramRegistry {
    pub fn materialize_instance(&mut self, instance: OsAppInstance) {
        self.instances.insert(instance.id.clone(), instance);
    }

    pub fn get_instance(&self, instance_id: &str) -> Option<&OsAppInstance> {
        self.instances.get(instance_id)
    }
}
//#endregion 🔖ProgramRegistry

//#region 🔖MediaExport
#[derive(Clone, Debug, PartialEq)]
pub enum OsMediaExportFormat {
    Svg,
    Png,
    Obj,
    Glb,
}

impl OsMediaExportFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::Png => "png",
            Self::Obj => "obj",
            Self::Glb => "glb",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "svg" => Some(Self::Svg),
            "png" => Some(Self::Png),
            "obj" => Some(Self::Obj),
            "glb" => Some(Self::Glb),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OsMediaExportResult {
    pub data: String,
    pub mime_type: String,
    pub file_name: String,
}

type OsMediaExportHandler = Box<dyn Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync>;

fn export_handlers() -> &'static Mutex<HashMap<String, OsMediaExportHandler>> {
    static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaExportHandler>>> = OnceLock::new();
    HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn os_media_export_key(resource_kind: &str, format: &OsMediaExportFormat) -> String {
    format!("{}:{}", resource_kind, format.as_str())
}

/// @emoji 💾 Registers an export handler for a media resource kind and format.
pub fn register_os_media_export_handler(
    resource_kind: &str,
    format: OsMediaExportFormat,
    handler: impl Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync + 'static,
) {
    export_handlers()
        .lock()
        .expect("lock")
        .insert(os_media_export_key(resource_kind, &format), Box::new(handler));
}

pub fn required_os_media_export_formats(dimension: &str) -> Vec<OsMediaExportFormat> {
    match dimension {
        "2d" => vec![OsMediaExportFormat::Svg, OsMediaExportFormat::Png],
        "3d" | "5d" => vec![OsMediaExportFormat::Glb, OsMediaExportFormat::Obj],
        _ => Vec::new(),
    }
}

/// @emoji ✅ Ensures every known resource kind has required export handlers.
pub fn assert_os_media_export_coverage() -> Result<(), String> {
    let handlers = export_handlers().lock().expect("lock");
    let mut missing = Vec::new();
    for descriptor in crate::registry::list_os_resource_descriptors() {
        for format in required_os_media_export_formats(&descriptor.dimension) {
            if !handlers.contains_key(&os_media_export_key(&descriptor.kind, &format)) {
                missing.push(format!("{}:{}", descriptor.kind, format.as_str()));
            }
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("missing os media export handlers: {}", missing.join(", ")))
    }
}

pub fn export_os_app_instance_media(
    instance: &OsAppInstance,
    source_document: &Value,
    format: OsMediaExportFormat,
) -> Result<OsMediaExportResult, String> {
    let handlers = export_handlers().lock().expect("lock");
    let handler = handlers
        .get(&os_media_export_key(&instance.yields, &format))
        .ok_or_else(|| format!("no export handler for {}:{}", instance.yields, format.as_str()))?;
    handler(source_document)
}

pub fn os_media_export_extension_for_format(format: &OsMediaExportFormat) -> &'static str {
    format.as_str()
}
//#endregion 🔖MediaExport

//#region 🔖MediaGraphVfs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsDescriptorKind {
    pub id: String,
    pub name: String,
    pub presentation: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsFileNodeDescriptor {
    pub id: String,
    pub descriptor_kind_id: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsFileNodeKind {
    pub id: String,
    pub name: String,
    pub descriptors: Vec<OsMediaGraphVfsFileNodeDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsSchema {
    pub descriptor_kinds: HashMap<String, OsMediaGraphVfsDescriptorKind>,
    pub file_node_kinds: HashMap<String, OsMediaGraphVfsFileNodeKind>,
    pub descriptor_column_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsMediaGraphVfsNodeRecord {
    pub id: String,
    pub file_node_kind_id: String,
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub has_children: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigate_uri: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub descriptor_values: HashMap<String, String>,
}

pub fn os_media_graph_vfs_schema() -> OsMediaGraphVfsSchema {
    let mut descriptor_kinds = HashMap::new();
    descriptor_kinds.insert(
        "text".into(),
        OsMediaGraphVfsDescriptorKind {
            id: "text".into(),
            name: "Text".into(),
            presentation: "text".into(),
        },
    );
    let binding = OsMediaGraphVfsFileNodeDescriptor {
        id: "binding".into(),
        descriptor_kind_id: "text".into(),
        label: "Binding".into(),
    };
    let mut file_node_kinds = HashMap::new();
    for kind in ["root", "instance", "folder", "source", "input"] {
        file_node_kinds.insert(
            kind.into(),
            OsMediaGraphVfsFileNodeKind {
                id: kind.into(),
                name: kind.into(),
                descriptors: vec![binding.clone()],
            },
        );
    }
    file_node_kinds.insert(
        "export".into(),
        OsMediaGraphVfsFileNodeKind {
            id: "export".into(),
            name: "Export".into(),
            descriptors: vec![
                binding.clone(),
                OsMediaGraphVfsFileNodeDescriptor {
                    id: "format".into(),
                    descriptor_kind_id: "text".into(),
                    label: "Format".into(),
                },
            ],
        },
    );
    OsMediaGraphVfsSchema {
        descriptor_kinds,
        file_node_kinds,
        descriptor_column_ids: vec!["binding".into(), "format".into()],
    }
}

pub fn os_media_graph_vfs_instance_id(node_id: &str) -> Option<String> {
    let captures = regex_lite(node_id, r"^inst:([^:]+)(?::|$)");
    captures
}

pub fn os_media_graph_vfs_instance_folder_id(instance_id: &str) -> String {
    format!("inst:{instance_id}")
}

pub fn os_media_graph_vfs_source_id(instance_id: &str) -> String {
    format!("inst:{instance_id}:source")
}

pub fn os_media_graph_vfs_inputs_folder_id(instance_id: &str) -> String {
    format!("inst:{instance_id}:inputs")
}

pub fn os_media_graph_vfs_outputs_folder_id(instance_id: &str) -> String {
    format!("inst:{instance_id}:outputs")
}

pub fn os_media_graph_vfs_input_port_id(instance_id: &str, port_spec_id: &str) -> String {
    format!("inst:{instance_id}:input:{port_spec_id}")
}

pub fn os_media_graph_vfs_export_id(
    instance_id: &str,
    port_spec_id: &str,
    format: &OsMediaExportFormat,
) -> String {
    format!("inst:{instance_id}:export:{port_spec_id}:{}", format.as_str())
}

fn regex_lite(input: &str, pattern: &str) -> Option<String> {
    if pattern == r"^inst:([^:]+)(?::|$)" {
        let rest = input.strip_prefix("inst:")?;
        let instance_id = rest.split(':').next()?;
        if instance_id.is_empty() {
            None
        } else {
            Some(instance_id.to_string())
        }
    } else {
        None
    }
}

/// @emoji 📁 Lists VFS children for one media graph folder node.
pub fn list_os_media_graph_vfs_children(
    parent_id: &str,
    instances: &[OsAppInstance],
    graph: &OsMediaGraph,
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
) -> Vec<OsMediaGraphVfsNodeRecord> {
    if parent_id == OS_MEDIA_GRAPH_VFS_ROOT_ID {
        return instances
            .iter()
            .map(|instance| {
                let registration = os_app_registration(&instance.program_id, &instance.app_id);
                OsMediaGraphVfsNodeRecord {
                    id: os_media_graph_vfs_instance_folder_id(&instance.id),
                    file_node_kind_id: "instance".into(),
                    name: format!(
                        "{} ({}.{}))",
                        instance.label, instance.program_id, instance.app_id
                    ),
                    path: format!("/{}", instance.label),
                    parent_id: Some(OS_MEDIA_GRAPH_VFS_ROOT_ID.into()),
                    has_children: true,
                    icon: registration
                        .as_ref()
                        .map(|entry| entry.component_kind.clone()),
                    navigate_uri: None,
                    descriptor_values: HashMap::from([("binding".into(), instance.yields.clone())]),
                }
            })
            .collect();
    }
    let Some(instance_id) = os_media_graph_vfs_instance_id(parent_id) else {
        return Vec::new();
    };
    let Some(instance) = instances.iter().find(|entry| entry.id == instance_id) else {
        return Vec::new();
    };
    let registration = os_app_registration(&instance.program_id, &instance.app_id);
    if parent_id == os_media_graph_vfs_instance_folder_id(&instance_id) {
        return vec![
            OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_source_id(&instance_id),
                file_node_kind_id: "source".into(),
                name: "source.json".into(),
                path: format!("/{}/source.json", instance.label),
                parent_id: Some(parent_id.into()),
                has_children: false,
                icon: Some("json".into()),
                navigate_uri: Some(format!("os://instance/{}", instance.id)),
                descriptor_values: HashMap::from([(
                    "binding".into(),
                    registration
                        .as_ref()
                        .map(|entry| entry.source_format.clone())
                        .unwrap_or_else(|| instance.yields.clone()),
                )]),
            },
            OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_inputs_folder_id(&instance_id),
                file_node_kind_id: "folder".into(),
                name: "inputs".into(),
                path: format!("/{}/inputs", instance.label),
                parent_id: Some(parent_id.into()),
                has_children: registration.as_ref().map(|entry| !entry.inputs.is_empty()).unwrap_or(false)
                    || bindings.iter().any(|binding| binding.instance_id == instance_id),
                icon: Some("folder-input".into()),
                navigate_uri: None,
                descriptor_values: HashMap::new(),
            },
            OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_outputs_folder_id(&instance_id),
                file_node_kind_id: "folder".into(),
                name: "outputs".into(),
                path: format!("/{}/outputs", instance.label),
                parent_id: Some(parent_id.into()),
                has_children: registration.as_ref().map(|entry| !entry.outputs.is_empty()).unwrap_or(false),
                icon: Some("folder-output".into()),
                navigate_uri: None,
                descriptor_values: HashMap::new(),
            },
        ];
    }
    if parent_id == os_media_graph_vfs_inputs_folder_id(&instance_id) {
        let mut rows = Vec::new();
        if let Some(registration) = registration.as_ref() {
            for spec in &registration.inputs {
                rows.push(OsMediaGraphVfsNodeRecord {
                    id: os_media_graph_vfs_input_port_id(&instance_id, &spec.id),
                    file_node_kind_id: "input".into(),
                    name: spec.id.clone(),
                    path: format!("/{}/inputs/{}", instance.label, spec.id),
                    parent_id: Some(parent_id.into()),
                    has_children: false,
                    icon: Some("plug".into()),
                    navigate_uri: None,
                    descriptor_values: HashMap::from([("binding".into(), spec.resource_kind.clone())]),
                });
            }
        }
        for binding in bindings
            .iter()
            .filter(|entry| entry.instance_id == instance_id)
        {
            let parameter = parameters
                .iter()
                .find(|entry| match entry {
                    crate::instance::OsParameter::Numeric { id, .. }
                    | crate::instance::OsParameter::Categorical { id, .. }
                    | crate::instance::OsParameter::Toggle { id, .. }
                    | crate::instance::OsParameter::Text { id, .. } => id == &binding.parameter_id,
                });
            rows.push(OsMediaGraphVfsNodeRecord {
                id: os_media_graph_vfs_input_port_id(&instance_id, &format!("param.{}", binding.parameter_id)),
                file_node_kind_id: "input".into(),
                name: parameter
                    .map(|entry| match entry {
                        crate::instance::OsParameter::Numeric { name, .. }
                        | crate::instance::OsParameter::Categorical { name, .. }
                        | crate::instance::OsParameter::Toggle { name, .. }
                        | crate::instance::OsParameter::Text { name, .. } => name.clone(),
                    })
                    .unwrap_or_else(|| binding.field_path.clone()),
                path: format!("/{}/inputs/param.{}", instance.label, binding.parameter_id),
                parent_id: Some(parent_id.into()),
                has_children: false,
                icon: Some("sliders-horizontal".into()),
                navigate_uri: None,
                descriptor_values: HashMap::from([(
                    "binding".into(),
                    parameter
                        .map(|entry| match entry {
                            crate::instance::OsParameter::Numeric { name, .. }
                            | crate::instance::OsParameter::Categorical { name, .. }
                            | crate::instance::OsParameter::Toggle { name, .. }
                            | crate::instance::OsParameter::Text { name, .. } => name.clone(),
                        })
                        .unwrap_or_else(|| binding.parameter_id.clone()),
                )]),
            });
        }
        return rows;
    }
    if parent_id == os_media_graph_vfs_outputs_folder_id(&instance_id) {
        let descriptor = crate::registry::os_resource_descriptor(&instance.yields);
        let formats = required_os_media_export_formats(&descriptor.dimension);
        let mut rows = Vec::new();
        if let Some(registration) = registration.as_ref() {
            for spec in &registration.outputs {
                for format in &formats {
                    let ext = os_media_export_extension_for_format(format);
                    rows.push(OsMediaGraphVfsNodeRecord {
                        id: os_media_graph_vfs_export_id(&instance_id, &spec.id, format),
                        file_node_kind_id: "export".into(),
                        name: format!("{}.{}", spec.id, ext),
                        path: format!("/{}/outputs/{}.{}", instance.label, spec.id, ext),
                        parent_id: Some(parent_id.into()),
                        has_children: false,
                        icon: Some(ext.into()),
                        navigate_uri: Some(format!(
                            "os://export/{}/{}/{}",
                            instance.id,
                            spec.id,
                            format.as_str()
                        )),
                        descriptor_values: HashMap::from([
                            ("binding".into(), spec.resource_kind.clone()),
                            ("format".into(), format.as_str().into()),
                        ]),
                    });
                }
            }
        }
        return rows;
    }
    let _ = graph;
    let _ = parameter_id_from_port_id;
    let _ = media_port_spec_id;
    let _ = os_app_primary_output_kind;
    Vec::new()
}
//#endregion 🔖MediaGraphVfs

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::OsSourceDocument;
    use crate::registry::{merge_os_program_definition, os_baseline_resource, OsPlatformAppInput, OsPlatformInput};

    #[test]
    fn validates_empty_media_graph() {
        assert!(validate_media_graph(&empty_media_graph()).ok);
    }

    #[test]
    fn export_coverage_accepts_registered_handlers() {
        for descriptor in crate::registry::list_os_resource_descriptors() {
            for format in required_os_media_export_formats(&descriptor.dimension) {
                register_os_media_export_handler(&descriptor.kind, format, |_| {
                    Ok(OsMediaExportResult {
                        data: "export".into(),
                        mime_type: "application/octet-stream".into(),
                        file_name: "export.bin".into(),
                    })
                });
            }
        }
        assert!(assert_os_media_export_coverage().is_ok());
    }

    #[test]
    fn flow_fixture_projects_neuron_preview() {
        let mut resources = HashMap::new();
        resources.insert(
            "draw".into(),
            os_baseline_resource("2d.drawing", "draw.document", "draw"),
        );
        let platform = OsPlatformInput {
            id: "draw".into(),
            name: "Draw".into(),
            api_version: "1".into(),
            apps: vec![OsPlatformAppInput {
                id: "draw".into(),
                label: "Draw".into(),
                hierarchy: vec!["semio".into(), "draw".into()],
                controller_id: "draw-play".into(),
                modes: vec![],
                default_mode_id: None,
            }],
        };
        merge_os_program_definition("draw", &platform, &resources).expect("merge");
        let registration = os_app_registration("draw", "draw").expect("registration");
        let instance = OsAppInstance {
            id: "app-1".into(),
            program_id: "draw".into(),
            app_id: "draw".into(),
            label: "Draw".into(),
            yields: os_app_primary_output_kind(&registration),
            source_document: OsSourceDocument {
                format: "draw.document".into(),
                vcs_json: None,
                inline: Some("{}".into()),
                payload_ref: None,
            },
        };
        let mut graph = empty_media_graph();
        graph.nodes.push(media_graph_node_for_instance(
            &instance,
            &registration,
            &MediaGraphPosition { x: 0.0, y: 0.0 },
            "node-1",
        ));
        let fixture = os_media_graph_to_flow_fixture(&graph, &[instance.clone()]);
        assert_eq!(fixture["schema"], "flow.fixture");
        assert_eq!(fixture["widgets"][0]["preview"], true);
        let operators = build_os_media_flow_operator_infos(&graph, &[instance], &[]);
        assert_eq!(operators.len(), 1);
        assert_eq!(operators[0].id, "os.media.node.node-1");
        assert_eq!(operators[0].module, OS_MEDIA_FLOW_MODULE_ID);
        assert_eq!(operators[0].name, "Draw");
    }
}
//#endregion 🧪Tests
