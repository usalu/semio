//! 🔌 Plugin host, studio document VCS store, backbone, and catalog.

use crate::instance::{
    create_default_os_parameter, create_os_id, patch_os_parameter, OsAppInstance,
    OsInstanceState, OsParameter, OsParameterFieldBinding, OsParameterType, OsSourceDocument,
};
use crate::media_graph::{
    empty_media_graph, media_graph_node_for_instance, sync_media_graph_parameter_ports,
    MediaGraphPosition, OsMediaGraph, OsMediaGraphEdge, OS_MEDIA_GRAPH_SCHEMA,
    OS_STUDIO_SCHEMA,
};
use crate::registry::{
    os_app_primary_output_kind, os_app_registration, PluginRegistry,
};
use semio_framework_core::{AppDefinition, PluginManifest, UiNode, ViewState};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use vcs::{
    create_document_vcs_envelope, materialize_document_projection, DocumentBackboneRef, DocumentVcs,
    DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff, VcsError,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginHotSwapEvent {
    pub plugin_id: String,
    pub version: String,
    pub added_apps: Vec<String>,
    pub removed_apps: Vec<String>,
}

pub struct LoadedPlugin {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    pub artifact_uri: String,
}

pub struct PluginHost {
    registry: PluginRegistry,
    instances: HashMap<u32, OsInstanceState>,
    next_instance_id: u32,
    plugins: HashMap<String, LoadedPlugin>,
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            instances: HashMap::new(),
            next_instance_id: 1,
            plugins: HashMap::new(),
        }
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    pub fn load_plugin(&mut self, plugin: LoadedPlugin) -> PluginHotSwapEvent {
        let previous_apps: Vec<String> = self
            .plugins
            .get(&plugin.plugin_id)
            .map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect())
            .unwrap_or_default();
        let next_apps: Vec<String> = plugin.manifest.apps.iter().map(|app| app.id.clone()).collect();
        for app in &plugin.manifest.apps {
            self.registry.register_app(app.clone());
        }
        for program in &plugin.manifest.programs {
            self.registry.register_program(program.clone());
        }
        self.plugins.insert(plugin.plugin_id.clone(), plugin);
        PluginHotSwapEvent {
            plugin_id: self.plugins.keys().next().cloned().unwrap_or_default(),
            version: self
                .plugins
                .values()
                .last()
                .map(|plugin| plugin.manifest.version.clone())
                .unwrap_or_default(),
            added_apps: next_apps
                .iter()
                .filter(|app| !previous_apps.contains(app))
                .cloned()
                .collect(),
            removed_apps: previous_apps
                .iter()
                .filter(|app| !next_apps.contains(app))
                .cloned()
                .collect(),
        }
    }

    pub fn hot_swap_plugin(&mut self, plugin: LoadedPlugin) -> PluginHotSwapEvent {
        let event = self.load_plugin(plugin);
        for instance in self.instances.values_mut() {
            instance.generation += 1;
        }
        event
    }

    pub fn apps(&self) -> Vec<AppDefinition> {
        self.registry.apps()
    }

    pub fn create_instance(&mut self, app_id: &str, document_json: String) -> Option<u32> {
        let app = self.registry.find_app(app_id)?;
        let id = self.next_instance_id;
        self.next_instance_id += 1;
        self.instances.insert(
            id,
            OsInstanceState {
                id,
                app_id: app.id.clone(),
                controller_id: app.controller_id.clone(),
                document_json,
                view_state: ViewState::default(),
                generation: 0,
            },
        );
        Some(id)
    }

    pub fn instance(&self, instance_id: u32) -> Option<&OsInstanceState> {
        self.instances.get(&instance_id)
    }

    pub fn instance_mut(&mut self, instance_id: u32) -> Option<&mut OsInstanceState> {
        self.instances.get_mut(&instance_id)
    }

    pub fn apply_ops(&mut self, instance_id: u32, ops: &[String]) -> bool {
        let Some(instance) = self.instances.get_mut(&instance_id) else {
            return false;
        };
        for op in ops {
            if let Ok(next) = apply_document_op(&instance.document_json, op) {
                instance.document_json = next;
                instance.generation += 1;
            }
        }
        true
    }

    pub fn set_view_state(&mut self, instance_id: u32, view_state: ViewState) {
        if let Some(instance) = self.instances.get_mut(&instance_id) {
            instance.view_state = view_state;
            instance.generation += 1;
        }
    }

    pub fn render_body(&self, instance_id: u32, body_key: &str, ui: UiNode) -> UiNode {
        let _ = (instance_id, body_key);
        ui
    }
}

fn apply_document_op(document_json: &str, op_json: &str) -> Result<String, String> {
    let mut document: serde_json::Value =
        serde_json::from_str(document_json).map_err(|error| error.to_string())?;
    let op: serde_json::Value = serde_json::from_str(op_json).map_err(|error| error.to_string())?;
    match op.get("op").and_then(|value| value.as_str()) {
        Some("setDocument") => {
            if let Some(next) = op.get("document") {
                document = next.clone();
            }
        }
        Some("patch") => {
            if let Some(patch) = op.get("patch") {
                merge_json(&mut document, patch);
            }
        }
        _ => {}
    }
    serde_json::to_string(&document).map_err(|error| error.to_string())
}

fn merge_json(target: &mut serde_json::Value, patch: &serde_json::Value) {
    match (target, patch) {
        (serde_json::Value::Object(target_map), serde_json::Value::Object(patch_map)) => {
            for (key, value) in patch_map {
                if value.is_null() {
                    target_map.remove(key);
                } else {
                    let entry = target_map
                        .entry(key.clone())
                        .or_insert(serde_json::Value::Null);
                    merge_json(entry, value);
                }
            }
        }
        (target_slot, patch_value) => {
            *target_slot = patch_value.clone();
        }
    }
}

//#region 🔖OsDocument
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsBackboneRef {
    pub kind: String,
    pub uri: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsConflict {
    pub kind: String,
    pub uri: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_revision: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsProjection {
    pub programs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    pub app_instances: Vec<OsAppInstance>,
    pub media_graph: OsMediaGraph,
    #[serde(default)]
    pub parameters: Vec<OsParameter>,
    #[serde(default)]
    pub parameter_bindings: Vec<OsParameterFieldBinding>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum OsOp {
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    ApplyAppOperation {
        instance_id: String,
        next_source: OsSourceDocument,
    },
    SpawnAppInstance {
        instance: OsAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: OsMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchAppSource {
        instance_id: String,
        inline: String,
    },
    PatchAppInstance {
        instance_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    },
    AddParameter {
        parameter: OsParameter,
    },
    RemoveParameter {
        parameter_id: String,
    },
    PatchParameter {
        parameter_id: String,
        parameter: OsParameter,
    },
    BindParameterField {
        binding: OsParameterFieldBinding,
    },
    UnbindParameterField {
        instance_id: String,
        field_path: String,
    },
    SyncParameterPorts,
}

pub type OsVcs = DocumentVcs<OsProjection, OsOp>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsDocument {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub vcs: OsVcs,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applied_edit_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<OsBackboneRef>,
}

pub type OsEnvelope = DocumentVcsEnvelope<OsProjection, OsOp>;

pub fn default_os_projection() -> OsProjection {
    OsProjection {
        programs: Vec::new(),
        active_program_id: None,
        active_alternative_id: None,
        app_instances: Vec::new(),
        media_graph: empty_media_graph(),
        parameters: Vec::new(),
        parameter_bindings: Vec::new(),
    }
}

pub fn create_empty_os_document(id: &str, name: &str) -> OsDocument {
    OsDocument {
        schema: OS_STUDIO_SCHEMA.into(),
        id: id.into(),
        name: name.into(),
        vcs: create_document_vcs_envelope(
            OS_STUDIO_SCHEMA,
            id,
            default_os_projection(),
            None,
        )
        .vcs,
        applied_edit_ids: Vec::new(),
        backbone: Some(OsBackboneRef {
            kind: "dev".into(),
            uri: "dev://studio.json".into(),
        }),
    }
}

pub fn apply_os_operation(projection: &OsProjection, operation: &OsOp) -> OsProjection {
    let mut next = projection.clone();
    match operation {
        OsOp::SetActiveProgram { program_id } => {
            next.active_program_id = program_id.clone();
        }
        OsOp::SetActiveAlternative { alternative_id } => {
            next.active_alternative_id = alternative_id.clone();
        }
        OsOp::ApplyAppOperation {
            instance_id,
            next_source,
        } => {
            for instance in &mut next.app_instances {
                if instance.id == *instance_id {
                    instance.source_document = next_source.clone();
                }
            }
        }
        OsOp::SpawnAppInstance { instance, position } => {
            if !next.programs.contains(&instance.program_id) {
                next.programs.push(instance.program_id.clone());
            }
            if let Some(registration) = os_app_registration(&instance.program_id, &instance.app_id) {
                let node = sync_media_node_parameter_ports(
                    &media_graph_node_for_instance(
                        instance,
                        &registration,
                        position,
                        &create_os_id("node"),
                    ),
                    &next.parameter_bindings,
                );
                next.media_graph.nodes.push(node);
            }
            next.app_instances.push(instance.clone());
        }
        OsOp::RemoveAppInstance { instance_id } => {
            let node_id = next
                .media_graph
                .nodes
                .iter()
                .find(|node| node.instance_id == *instance_id)
                .map(|node| node.id.clone());
            next.app_instances.retain(|instance| instance.id != *instance_id);
            next.parameter_bindings
                .retain(|binding| binding.instance_id != *instance_id);
            next.media_graph
                .nodes
                .retain(|node| node.instance_id != *instance_id);
            if let Some(node_id) = node_id {
                next.media_graph.edges.retain(|edge| {
                    edge.source_node_id != node_id && edge.target_node_id != node_id
                });
            }
        }
        OsOp::ConnectMediaPorts { edge } => next.media_graph.edges.push(edge.clone()),
        OsOp::DisconnectMediaEdge { edge_id } => next
            .media_graph
            .edges
            .retain(|edge| edge.id != *edge_id),
        OsOp::MoveMediaNode { node_id, x, y } => {
            for node in &mut next.media_graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
                }
            }
        }
        OsOp::PatchAppSource { instance_id, inline } => {
            for instance in &mut next.app_instances {
                if instance.id == *instance_id {
                    instance.source_document.inline = Some(inline.clone());
                }
            }
        }
        OsOp::PatchAppInstance { instance_id, label } => {
            if let Some(label) = label {
                for instance in &mut next.app_instances {
                    if instance.id == *instance_id {
                        instance.label = label.clone();
                    }
                }
            }
        }
        OsOp::AddParameter { parameter } => next.parameters.push(parameter.clone()),
        OsOp::RemoveParameter { parameter_id } => {
            next.parameters.retain(|parameter| parameter_entity_id(parameter) != *parameter_id);
            next.parameter_bindings
                .retain(|binding| binding.parameter_id != *parameter_id);
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
        OsOp::PatchParameter {
            parameter_id,
            parameter,
        } => {
            for entry in &mut next.parameters {
                if parameter_entity_id(entry) == *parameter_id {
                    *entry = parameter.clone();
                }
            }
        }
        OsOp::BindParameterField { binding } => {
            next.parameter_bindings.retain(|entry| {
                !(entry.instance_id == binding.instance_id && entry.field_path == binding.field_path)
            });
            next.parameter_bindings.push(binding.clone());
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
        OsOp::UnbindParameterField {
            instance_id,
            field_path,
        } => {
            next.parameter_bindings.retain(|binding| {
                !(binding.instance_id == *instance_id && binding.field_path == *field_path)
            });
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
        OsOp::SyncParameterPorts => {
            next.media_graph =
                sync_media_graph_parameter_ports(&next.media_graph, &next.parameter_bindings);
        }
    }
    next
}

fn sync_media_node_parameter_ports(
    node: &crate::media_graph::OsMediaGraphNode,
    bindings: &[OsParameterFieldBinding],
) -> crate::media_graph::OsMediaGraphNode {
    sync_media_graph_parameter_ports(
        &OsMediaGraph {
            schema: OS_MEDIA_GRAPH_SCHEMA.into(),
            nodes: vec![node.clone()],
            edges: Vec::new(),
        },
        bindings,
    )
    .nodes
    .into_iter()
    .next()
    .unwrap_or_else(|| node.clone())
}

fn parameter_entity_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum OsDiff {
    #[default]
    Empty,
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    ApplyAppOperation {
        instance_id: String,
        next_source: OsSourceDocument,
    },
    SpawnAppInstance {
        instance: OsAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: OsMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
    PatchAppSource {
        instance_id: String,
        inline: String,
    },
    PatchAppInstance {
        instance_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    },
    AddParameter {
        parameter: OsParameter,
    },
    RemoveParameter {
        parameter_id: String,
    },
    PatchParameter {
        parameter_id: String,
        parameter: OsParameter,
    },
    BindParameterField {
        binding: OsParameterFieldBinding,
    },
    UnbindParameterField {
        instance_id: String,
        field_path: String,
    },
    SyncParameterPorts,
}

impl OperationDiff<OsProjection> for OsDiff {
    fn apply(&self, projection: &OsProjection) -> OsProjection {
        let op = match self {
            OsDiff::Empty => return projection.clone(),
            OsDiff::SetActiveProgram { program_id } => OsOp::SetActiveProgram {
                program_id: program_id.clone(),
            },
            OsDiff::SetActiveAlternative { alternative_id } => OsOp::SetActiveAlternative {
                alternative_id: alternative_id.clone(),
            },
            OsDiff::ApplyAppOperation {
                instance_id,
                next_source,
            } => OsOp::ApplyAppOperation {
                instance_id: instance_id.clone(),
                next_source: next_source.clone(),
            },
            OsDiff::SpawnAppInstance { instance, position } => OsOp::SpawnAppInstance {
                instance: instance.clone(),
                position: position.clone(),
            },
            OsDiff::RemoveAppInstance { instance_id } => OsOp::RemoveAppInstance {
                instance_id: instance_id.clone(),
            },
            OsDiff::ConnectMediaPorts { edge } => OsOp::ConnectMediaPorts { edge: edge.clone() },
            OsDiff::DisconnectMediaEdge { edge_id } => OsOp::DisconnectMediaEdge {
                edge_id: edge_id.clone(),
            },
            OsDiff::MoveMediaNode { node_id, x, y } => OsOp::MoveMediaNode {
                node_id: node_id.clone(),
                x: *x,
                y: *y,
            },
            OsDiff::PatchAppSource { instance_id, inline } => OsOp::PatchAppSource {
                instance_id: instance_id.clone(),
                inline: inline.clone(),
            },
            OsDiff::PatchAppInstance { instance_id, label } => OsOp::PatchAppInstance {
                instance_id: instance_id.clone(),
                label: label.clone(),
            },
            OsDiff::AddParameter { parameter } => OsOp::AddParameter {
                parameter: parameter.clone(),
            },
            OsDiff::RemoveParameter { parameter_id } => OsOp::RemoveParameter {
                parameter_id: parameter_id.clone(),
            },
            OsDiff::PatchParameter {
                parameter_id,
                parameter,
            } => OsOp::PatchParameter {
                parameter_id: parameter_id.clone(),
                parameter: parameter.clone(),
            },
            OsDiff::BindParameterField { binding } => OsOp::BindParameterField {
                binding: binding.clone(),
            },
            OsDiff::UnbindParameterField {
                instance_id,
                field_path,
            } => OsOp::UnbindParameterField {
                instance_id: instance_id.clone(),
                field_path: field_path.clone(),
            },
            OsDiff::SyncParameterPorts => OsOp::SyncParameterPorts,
        };
        apply_os_operation(projection, &op)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, OsDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<OsProjection> for OsOp {
    type Diff = OsDiff;

    fn diff(&self, _projection: &OsProjection) -> OsDiff {
        match self {
            OsOp::SetActiveProgram { program_id } => OsDiff::SetActiveProgram {
                program_id: program_id.clone(),
            },
            OsOp::SetActiveAlternative { alternative_id } => OsDiff::SetActiveAlternative {
                alternative_id: alternative_id.clone(),
            },
            OsOp::ApplyAppOperation {
                instance_id,
                next_source,
            } => OsDiff::ApplyAppOperation {
                instance_id: instance_id.clone(),
                next_source: next_source.clone(),
            },
            OsOp::SpawnAppInstance { instance, position } => OsDiff::SpawnAppInstance {
                instance: instance.clone(),
                position: position.clone(),
            },
            OsOp::RemoveAppInstance { instance_id } => OsDiff::RemoveAppInstance {
                instance_id: instance_id.clone(),
            },
            OsOp::ConnectMediaPorts { edge } => OsDiff::ConnectMediaPorts { edge: edge.clone() },
            OsOp::DisconnectMediaEdge { edge_id } => OsDiff::DisconnectMediaEdge {
                edge_id: edge_id.clone(),
            },
            OsOp::MoveMediaNode { node_id, x, y } => OsDiff::MoveMediaNode {
                node_id: node_id.clone(),
                x: *x,
                y: *y,
            },
            OsOp::PatchAppSource { instance_id, inline } => OsDiff::PatchAppSource {
                instance_id: instance_id.clone(),
                inline: inline.clone(),
            },
            OsOp::PatchAppInstance { instance_id, label } => OsDiff::PatchAppInstance {
                instance_id: instance_id.clone(),
                label: label.clone(),
            },
            OsOp::AddParameter { parameter } => OsDiff::AddParameter {
                parameter: parameter.clone(),
            },
            OsOp::RemoveParameter { parameter_id } => OsDiff::RemoveParameter {
                parameter_id: parameter_id.clone(),
            },
            OsOp::PatchParameter {
                parameter_id,
                parameter,
            } => OsDiff::PatchParameter {
                parameter_id: parameter_id.clone(),
                parameter: parameter.clone(),
            },
            OsOp::BindParameterField { binding } => OsDiff::BindParameterField {
                binding: binding.clone(),
            },
            OsOp::UnbindParameterField {
                instance_id,
                field_path,
            } => OsDiff::UnbindParameterField {
                instance_id: instance_id.clone(),
                field_path: field_path.clone(),
            },
            OsOp::SyncParameterPorts => OsDiff::SyncParameterPorts,
        }
    }

    fn backwards(&self, projection: &OsProjection) -> Vec<Self> {
        match self {
            OsOp::SetActiveProgram { .. } => vec![OsOp::SetActiveProgram {
                program_id: projection.active_program_id.clone(),
            }],
            OsOp::SetActiveAlternative { .. } => vec![OsOp::SetActiveAlternative {
                alternative_id: projection.active_alternative_id.clone(),
            }],
            OsOp::ApplyAppOperation { instance_id, .. } => projection
                .app_instances
                .iter()
                .find(|instance| instance.id == *instance_id)
                .map(|instance| {
                    vec![OsOp::ApplyAppOperation {
                        instance_id: instance_id.clone(),
                        next_source: instance.source_document.clone(),
                    }]
                })
                .unwrap_or_default(),
            OsOp::SpawnAppInstance { instance, .. } => vec![OsOp::RemoveAppInstance {
                instance_id: instance.id.clone(),
            }],
            OsOp::RemoveAppInstance { instance_id } => projection
                .app_instances
                .iter()
                .find(|instance| instance.id == *instance_id)
                .map(|instance| {
                    let node = projection
                        .media_graph
                        .nodes
                        .iter()
                        .find(|entry| entry.instance_id == *instance_id);
                    vec![OsOp::SpawnAppInstance {
                        instance: instance.clone(),
                        position: MediaGraphPosition {
                            x: node.map(|entry| entry.x).unwrap_or(0.0),
                            y: node.map(|entry| entry.y).unwrap_or(0.0),
                        },
                    }]
                })
                .unwrap_or_default(),
            OsOp::ConnectMediaPorts { edge } => vec![OsOp::DisconnectMediaEdge {
                edge_id: edge.id.clone(),
            }],
            OsOp::DisconnectMediaEdge { edge_id } => projection
                .media_graph
                .edges
                .iter()
                .find(|edge| edge.id == *edge_id)
                .map(|edge| vec![OsOp::ConnectMediaPorts { edge: edge.clone() }])
                .unwrap_or_default(),
            OsOp::MoveMediaNode { node_id, .. } => projection
                .media_graph
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| {
                    vec![OsOp::MoveMediaNode {
                        node_id: node_id.clone(),
                        x: node.x,
                        y: node.y,
                    }]
                })
                .unwrap_or_default(),
            OsOp::PatchAppSource { instance_id, .. } => projection
                .app_instances
                .iter()
                .find(|instance| instance.id == *instance_id)
                .map(|instance| {
                    vec![OsOp::PatchAppSource {
                        instance_id: instance_id.clone(),
                        inline: instance.source_document.inline.clone().unwrap_or_default(),
                    }]
                })
                .unwrap_or_default(),
            OsOp::PatchAppInstance { instance_id, .. } => projection
                .app_instances
                .iter()
                .find(|instance| instance.id == *instance_id)
                .map(|instance| {
                    vec![OsOp::PatchAppInstance {
                        instance_id: instance_id.clone(),
                        label: Some(instance.label.clone()),
                    }]
                })
                .unwrap_or_default(),
            OsOp::AddParameter { parameter } => vec![OsOp::RemoveParameter {
                parameter_id: parameter_entity_id(parameter).into(),
            }],
            OsOp::RemoveParameter { parameter_id } => projection
                .parameters
                .iter()
                .find(|parameter| parameter_entity_id(parameter) == *parameter_id)
                .map(|parameter| vec![OsOp::AddParameter {
                    parameter: parameter.clone(),
                }])
                .unwrap_or_default(),
            OsOp::PatchParameter {
                parameter_id,
                parameter,
            } => projection
                .parameters
                .iter()
                .find(|entry| parameter_entity_id(entry) == *parameter_id)
                .map(|current| {
                    vec![OsOp::PatchParameter {
                        parameter_id: parameter_id.clone(),
                        parameter: current.clone(),
                    }]
                })
                .unwrap_or_else(|| {
                    vec![OsOp::PatchParameter {
                        parameter_id: parameter_id.clone(),
                        parameter: parameter.clone(),
                    }]
                }),
            OsOp::BindParameterField { binding } => vec![OsOp::UnbindParameterField {
                instance_id: binding.instance_id.clone(),
                field_path: binding.field_path.clone(),
            }],
            OsOp::UnbindParameterField {
                instance_id,
                field_path,
            } => projection
                .parameter_bindings
                .iter()
                .find(|binding| binding.instance_id == *instance_id && binding.field_path == *field_path)
                .map(|binding| {
                    vec![OsOp::BindParameterField {
                        binding: binding.clone(),
                    }]
                })
                .unwrap_or_default(),
            OsOp::SyncParameterPorts => Vec::new(),
        }
    }
}

pub fn materialize_os_projection(
    document: &OsDocument,
    applied_edit_ids: &[String],
) -> Result<OsProjection, VcsError> {
    let envelope = OsEnvelope {
        schema: document.schema.clone(),
        id: document.id.clone(),
        vcs: document.vcs.clone(),
        backbone: document
            .backbone
            .as_ref()
            .map(|entry| DocumentBackboneRef {
                kind: entry.kind.clone(),
                uri: entry.uri.clone(),
            }),
        active_alternative_id: document.vcs.initial_projection.active_alternative_id.clone(),
    };
    materialize_document_projection(&envelope, applied_edit_ids)
}

pub fn os_document_to_json(document: &OsDocument) -> Result<String, VcsError> {
    serde_json::to_string_pretty(document).map_err(|error| VcsError::Serialize(error.to_string()))
}

pub fn os_document_from_json(json: &str) -> Result<OsDocument, VcsError> {
    let document: OsDocument =
        serde_json::from_str(json).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    if document.schema != OS_STUDIO_SCHEMA {
        return Err(VcsError::Deserialize(format!(
            "expected schema {OS_STUDIO_SCHEMA}"
        )));
    }
    Ok(document)
}
//#endregion 🔖OsDocument

//#region 🔖OsStore
pub struct OsStore {
    inner: DocumentVcsStore<OsProjection, OsOp>,
    name: String,
}

impl OsStore {
    pub fn new(document: OsDocument) -> Self {
        let applied_edit_ids = document.applied_edit_ids.clone();
        let envelope = OsEnvelope {
            schema: document.schema,
            id: document.id,
            vcs: document.vcs,
            backbone: document.backbone.map(|entry| DocumentBackboneRef {
                kind: entry.kind,
                uri: entry.uri,
            }),
            active_alternative_id: None,
        };
        let mut inner = DocumentVcsStore::new(envelope);
        if !applied_edit_ids.is_empty() {
            let snapshot = inner.envelope().clone();
            inner.set_envelope(snapshot, applied_edit_ids);
        }
        Self {
            inner,
            name: document.name,
        }
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn projection(&self) -> Result<OsProjection, VcsError> {
        self.inner.projection()
    }

    pub fn document(&self) -> OsDocument {
        let envelope = self.inner.envelope();
        OsDocument {
            schema: envelope.schema.clone(),
            id: envelope.id.clone(),
            name: self.name.clone(),
            vcs: envelope.vcs.clone(),
            applied_edit_ids: self.inner.applied_edit_ids().to_vec(),
            backbone: envelope.backbone.as_ref().map(|entry| OsBackboneRef {
                kind: entry.kind.clone(),
                uri: entry.uri.clone(),
            }),
        }
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        self.inner.dispatch_json(command_json)
    }

    pub fn dispatch_apply(&mut self, operations: Vec<OsOp>) -> Result<(), VcsError> {
        self.inner.dispatch(DocumentVcsCommand::Apply {
            operations,
            description: None,
        })
    }

    pub fn set_studio_name(&mut self, name: &str) {
        self.name = name.into();
        let _ = self.inner.generation();
    }

    pub fn spawn_app_instance(
        &mut self,
        program_id: &str,
        app_id: &str,
        label: Option<&str>,
        position: MediaGraphPosition,
    ) -> Result<String, VcsError> {
        let registration = os_app_registration(program_id, app_id)
            .ok_or_else(|| VcsError::Deserialize(format!("unknown app {program_id}/{app_id}")))?;
        let instance_id = create_os_id("app");
        let instance = OsAppInstance {
            id: instance_id.clone(),
            program_id: program_id.into(),
            app_id: app_id.into(),
            label: label
                .map(str::to_string)
                .unwrap_or_else(|| registration.label.clone()),
            yields: os_app_primary_output_kind(&registration),
            source_document: OsSourceDocument {
                format: registration.source_format.clone(),
                vcs_json: None,
                inline: Some("{}".into()),
                payload_ref: None,
            },
        };
        self.dispatch_apply(vec![OsOp::SpawnAppInstance {
            instance,
            position,
        }])?;
        Ok(instance_id)
    }

    pub fn add_parameter(
        &mut self,
        parameter_type: &OsParameterType,
        name: &str,
    ) -> Result<String, VcsError> {
        let parameter = create_default_os_parameter(parameter_type, name, None);
        let parameter_id_value = parameter_entity_id(&parameter).to_string();
        self.dispatch_apply(vec![OsOp::AddParameter { parameter }])?;
        Ok(parameter_id_value)
    }

    pub fn patch_parameter(&mut self, target_parameter_id: &str, patch: &serde_json::Value) -> Result<(), VcsError> {
        let projection = self.projection()?;
        let current = projection
            .parameters
            .iter()
            .find(|parameter| parameter_entity_id(parameter) == target_parameter_id)
            .cloned()
            .ok_or_else(|| VcsError::Deserialize(format!("unknown parameter {target_parameter_id}")))?;
        let next = patch_os_parameter(&current, patch);
        self.dispatch_apply(vec![OsOp::PatchParameter {
            parameter_id: target_parameter_id.into(),
            parameter: next,
        }])
    }

    pub fn sync_backbone(&self) -> Result<(), VcsError> {
        self.inner.sync_backbone()
    }

    pub fn load_backbone(&mut self) -> Result<(), VcsError> {
        self.inner.load_backbone()
    }
}
//#endregion 🔖OsStore

//#region 🔖Backbone
pub trait OsBackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

impl<T: vcs::BackbonePort> OsBackbonePort for T {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        vcs::BackbonePort::read(self, uri)
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        vcs::BackbonePort::write(self, uri, payload)
    }
}

/// @emoji 💾 Dev JSON backbone (`dev://`) over an OS backbone port.
pub struct DevJsonBackbone {
    uri: Option<String>,
    port: Arc<dyn OsBackbonePort>,
}

impl DevJsonBackbone {
    pub fn new(port: Arc<dyn OsBackbonePort>) -> Self {
        Self { uri: None, port }
    }

    pub fn attach(&mut self, uri: &str) {
        self.uri = Some(uri.into());
    }

    pub fn status(&self) -> HashMap<&'static str, Option<String>> {
        let mut status = HashMap::new();
        status.insert("attachedUri", self.uri.clone());
        status.insert("kind", Some("dev".into()));
        status
    }

    pub fn load_attached(&self) -> Result<Option<OsDocument>, VcsError> {
        let Some(uri) = &self.uri else {
            return Ok(None);
        };
        let json = self.port.read(uri)?;
        if json.is_empty() {
            return Ok(None);
        }
        Ok(Some(os_document_from_json(&json)?))
    }

    pub fn sync(&self, document: &OsDocument) -> Result<String, VcsError> {
        let mut synced = document.clone();
        if let Some(uri) = &self.uri {
            synced.backbone = Some(OsBackboneRef {
                kind: "dev".into(),
                uri: uri.clone(),
            });
            let json = os_document_to_json(&synced)?;
            self.port.write(uri, &json)?;
            return Ok(json);
        }
        os_document_to_json(&synced)
    }
}

/// @emoji 💾 Local-first backbone stub (`local://`) mirroring dev JSON port shape.
pub struct LocalJsonBackbone {
    uri: Option<String>,
    port: Arc<dyn OsBackbonePort>,
}

impl LocalJsonBackbone {
    pub fn new(port: Arc<dyn OsBackbonePort>) -> Self {
        Self { uri: None, port }
    }

    pub fn attach(&mut self, uri: &str) -> Result<(), VcsError> {
        if !uri.starts_with("local://") {
            return Err(VcsError::Backbone(format!("expected local:// uri, got {uri}")));
        }
        self.uri = Some(uri.into());
        Ok(())
    }

    pub fn load_attached(&self) -> Result<Option<OsDocument>, VcsError> {
        let Some(uri) = &self.uri else {
            return Ok(None);
        };
        let json = self.port.read(uri)?;
        if json.is_empty() {
            return Ok(None);
        }
        Ok(Some(os_document_from_json(&json)?))
    }

    pub fn sync(&self, document: &OsDocument) -> Result<String, VcsError> {
        let mut synced = document.clone();
        if let Some(uri) = &self.uri {
            synced.backbone = Some(OsBackboneRef {
                kind: "local".into(),
                uri: uri.clone(),
            });
            let json = os_document_to_json(&synced)?;
            self.port.write(uri, &json)?;
            return Ok(json);
        }
        os_document_to_json(&synced)
    }
}

/// @emoji 🌐 Remote OS backbone (`remote://`) stub with conflict surfacing.
pub struct RemoteOsBackbone {
    uri: Option<String>,
    cached_document: Mutex<Option<OsDocument>>,
    last_conflict: Mutex<Option<OsConflict>>,
}

impl RemoteOsBackbone {
    pub fn new() -> Self {
        Self {
            uri: None,
            cached_document: Mutex::new(None),
            last_conflict: Mutex::new(None),
        }
    }

    pub fn attach(&mut self, uri: &str) -> Result<(), VcsError> {
        if !uri.starts_with("remote://") {
            return Err(VcsError::Backbone(format!("expected remote:// uri, got {uri}")));
        }
        self.uri = Some(uri.into());
        Ok(())
    }

    pub fn status(&self) -> (Option<String>, Option<OsConflict>) {
        (
            self.uri.clone(),
            self.last_conflict.lock().ok().and_then(|guard| guard.clone()),
        )
    }

    pub fn load_attached(&self) -> Option<OsDocument> {
        self.cached_document.lock().ok().and_then(|guard| guard.clone())
    }

    pub fn sync(&self, document: &OsDocument) -> Result<String, VcsError> {
        let uri = self
            .uri
            .clone()
            .unwrap_or_else(|| "remote://unknown".into());
        let conflict = OsConflict {
            kind: "os-conflict".into(),
            uri: uri.clone(),
            message: "remote backbone sync is not implemented".into(),
            remote_revision: None,
        };
        if let Ok(mut guard) = self.last_conflict.lock() {
            *guard = Some(conflict);
        }
        if let Ok(mut guard) = self.cached_document.lock() {
            *guard = Some(OsDocument {
                backbone: Some(OsBackboneRef {
                    kind: "remote".into(),
                    uri,
                }),
                ..document.clone()
            });
        }
        Err(VcsError::RemoteSyncNotImplemented)
    }

    pub fn clear_conflict(&self) {
        if let Ok(mut guard) = self.last_conflict.lock() {
            *guard = None;
        }
    }
}

impl Default for RemoteOsBackbone {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖Backbone

//#region 🔖StudioCatalog
pub const OS_HOME_VFS_ROOT_ID: &str = "os-home-root";
pub const OS_STUDIO_BACKBONE_URI_PREFIX: &str = "dev://studio/";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsStudioCatalogEntry {
    pub id: String,
    pub name: String,
    pub backbone_uri: String,
    pub app_count: usize,
    pub node_count: usize,
    pub updated_at: String,
}

static STUDIO_CATALOG_URIS: LazyLock<Mutex<HashMap<usize, HashSet<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn port_key(port: &Arc<dyn OsBackbonePort>) -> usize {
    Arc::as_ptr(port) as *const () as usize
}

fn track_os_studio_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
    STUDIO_CATALOG_URIS
        .lock()
        .expect("lock")
        .entry(port_key(port))
        .or_default()
        .insert(uri.into());
}

fn untrack_os_studio_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
    if let Some(uris) = STUDIO_CATALOG_URIS.lock().expect("lock").get_mut(&port_key(port)) {
        uris.remove(uri);
    }
}

fn os_studio_backbone_uri(studio_id: &str) -> String {
    format!("{OS_STUDIO_BACKBONE_URI_PREFIX}{studio_id}")
}

fn os_studio_id_from_backbone_uri(uri: &str) -> Option<String> {
    uri.strip_prefix(OS_STUDIO_BACKBONE_URI_PREFIX)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
}

fn os_studio_catalog_entry_from_document(
    backbone_uri: &str,
    document: &OsDocument,
) -> Result<OsStudioCatalogEntry, VcsError> {
    let studio_id = os_studio_id_from_backbone_uri(backbone_uri)
        .unwrap_or_else(|| document.id.clone());
    let projection = materialize_os_projection(document, &[])?;
    let updated_at = document
        .vcs
        .changes
        .last()
        .map(|change| change.saved_at.clone())
        .unwrap_or_else(|| "0".into());
    Ok(OsStudioCatalogEntry {
        id: studio_id,
        name: document.name.clone(),
        backbone_uri: backbone_uri.into(),
        app_count: projection.app_instances.len(),
        node_count: projection.media_graph.nodes.len(),
        updated_at,
    })
}

/// @emoji 📚 Lists persisted studio documents from the dev backbone namespace.
pub fn list_os_studio_catalog_entries(
    port: Arc<dyn OsBackbonePort>,
) -> Result<Vec<OsStudioCatalogEntry>, VcsError> {
    let mut entries = Vec::new();
    let uris: Vec<String> = STUDIO_CATALOG_URIS
        .lock()
        .expect("lock")
        .get(&port_key(&port))
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    for uri in uris {
        let json = port.read(&uri)?;
        if json.is_empty() {
            continue;
        }
        let document = os_document_from_json(&json)?;
        entries.push(os_studio_catalog_entry_from_document(&uri, &document)?);
    }
    entries.sort_by(|left, right| {
        right
            .updated_at
            .cmp(&left.updated_at)
            .then_with(|| left.name.cmp(&right.name))
    });
    Ok(entries)
}

/// @emoji 🆕 Creates an empty studio document on the dev backbone.
pub fn create_os_studio(
    name: &str,
    port: Arc<dyn OsBackbonePort>,
) -> Result<OsStudioCatalogEntry, VcsError> {
    let id = create_os_id("studio");
    let document = create_empty_os_document(&id, name.trim());
    let backbone_uri = os_studio_backbone_uri(&id);
    let mut backbone = DevJsonBackbone::new(port.clone());
    backbone.attach(&backbone_uri);
    backbone.sync(&document)?;
    track_os_studio_backbone_uri(&port, &backbone_uri);
    os_studio_catalog_entry_from_document(&backbone_uri, &document)
}

/// @emoji 🗑️ Deletes a studio document from the dev backbone.
pub fn delete_os_studio(studio_id: &str, port: Arc<dyn OsBackbonePort>) -> Result<(), VcsError> {
    let uri = os_studio_backbone_uri(studio_id);
    untrack_os_studio_backbone_uri(&port, &uri);
    port.write(&uri, "")
}

/// @emoji 📥 Imports a studio document JSON onto the dev backbone.
pub fn import_os_studio_from_json(
    json: &str,
    port: Arc<dyn OsBackbonePort>,
) -> Result<OsStudioCatalogEntry, VcsError> {
    let mut document = os_document_from_json(json)?;
    let studio_id = if document.id.is_empty() {
        create_os_id("studio")
    } else {
        document.id.clone()
    };
    let backbone_uri = os_studio_backbone_uri(&studio_id);
    document.id = studio_id;
    let mut backbone = DevJsonBackbone::new(port.clone());
    backbone.attach(&backbone_uri);
    backbone.sync(&document)?;
    track_os_studio_backbone_uri(&port, &backbone_uri);
    os_studio_catalog_entry_from_document(&backbone_uri, &document)
}

/// @emoji 📂 Loads a studio document from the dev backbone.
pub fn load_os_studio_document(
    studio_id: &str,
    port: Arc<dyn OsBackbonePort>,
) -> Result<OsDocument, VcsError> {
    let backbone_uri = os_studio_backbone_uri(studio_id);
    let mut backbone = DevJsonBackbone::new(port);
    backbone.attach(&backbone_uri);
    backbone
        .load_attached()?
        .ok_or_else(|| VcsError::Backbone(format!("unknown os studio: {studio_id}")))
}

/// @emoji 🌱 Seeds the demo studio when the catalog is empty.
pub fn seed_os_studio_catalog_if_empty(
    seed_document: OsDocument,
    port: Arc<dyn OsBackbonePort>,
) -> Result<Option<OsStudioCatalogEntry>, VcsError> {
    if !list_os_studio_catalog_entries(port.clone())?.is_empty() {
        return Ok(None);
    }
    let studio_id = if seed_document.id.is_empty() {
        "default".into()
    } else {
        seed_document.id.clone()
    };
    let backbone_uri = os_studio_backbone_uri(&studio_id);
    let mut seeded = seed_document;
    seeded.id = studio_id;
    let mut backbone = DevJsonBackbone::new(port.clone());
    backbone.attach(&backbone_uri);
    backbone.sync(&seeded)?;
    track_os_studio_backbone_uri(&port, &backbone_uri);
    Ok(Some(os_studio_catalog_entry_from_document(
        &backbone_uri,
        &seeded,
    )?))
}
//#endregion 🔖StudioCatalog

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{merge_os_program_definition, os_baseline_resource, OsPlatformAppInput, OsPlatformInput};
    use crate::media_graph::{empty_media_graph, validate_media_graph};
    use semio_framework_core::{ModeDefinition, PluginManifest, WindowKindDefinition};
    use std::sync::Arc;
    use vcs::MemoryBackbonePort;

    #[test]
    fn loads_plugin_apps_into_registry() {
        let mut host = PluginHost::new();
        let manifest = PluginManifest {
            plugin_id: "draw".into(),
            label: "Draw".into(),
            version: "0.1.0".into(),
            apps: vec![AppDefinition {
                id: "draw-play".into(),
                label: "Draw".into(),
                hierarchy: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: vec![ModeDefinition {
                    id: "edit".into(),
                    label: "Edit".into(),
                    tools: Vec::new(),
                }],
                default_mode_id: Some("edit".into()),
                window_kinds: vec![WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    icon_id: None,
                    measures: Vec::new(),
                    engagement: None,
                }],
                panel_tabs: vec![],
                keybindings: vec![],
                named_layouts: Vec::new(),
                default_layout: None,
            }],
            programs: vec![],
            examples: vec![],
        };
        host.load_plugin(LoadedPlugin {
            plugin_id: "draw".into(),
            manifest,
            artifact_uri: "plugin://draw".into(),
        });
        assert_eq!(host.apps().len(), 1);
    }

    fn seed_draw_program() {
        let mut resources = HashMap::new();
        resources.insert(
            "draw".into(),
            os_baseline_resource("2d.drawing", "draw.document", "draw"),
        );
        merge_os_program_definition(
            "draw",
            &OsPlatformInput {
                id: "draw".into(),
                name: "Draw".into(),
                api_version: "1".into(),
                apps: vec![OsPlatformAppInput {
                    id: "draw".into(),
                    label: "Draw".into(),
                    hierarchy: vec!["semio".into(), "draw".into()],
                    controller_id: "draw-play".into(),
                    modes: vec![ModeDefinition {
                        id: "edit".into(),
                        label: "Edit".into(),
                        tools: Vec::new(),
                    }],
                    default_mode_id: None,
                }],
            },
            &resources,
        )
        .expect("merge");
    }

    #[test]
    fn spawns_and_removes_app_instances() {
        seed_draw_program();
        let mut store = OsStore::new(create_empty_os_document("studio", "Studio"));
        store
            .spawn_app_instance(
                "draw",
                "draw",
                None,
                MediaGraphPosition { x: 40.0, y: 40.0 },
            )
            .expect("spawn");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 1);
        store
            .dispatch_json(r#"{"kind":"undo"}"#)
            .expect("undo");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 0);
    }

    #[test]
    fn adds_and_patches_studio_parameters() {
        let mut store = OsStore::new(create_empty_os_document("studio", "Studio"));
        let parameter_id = store
            .add_parameter(&OsParameterType::Numeric, "Zoom")
            .expect("add");
        store
            .patch_parameter(&parameter_id, &serde_json::json!({ "value": 12.0, "max": 10.0 }))
            .expect("patch");
        match &store.projection().expect("projection").parameters[0] {
            OsParameter::Numeric { value, .. } => assert_eq!(*value, 10.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn creates_and_lists_studio_catalog_entries() {
        let port = Arc::new(MemoryBackbonePort::new());
        let entry = create_os_studio("Catalog Studio", port.clone()).expect("create");
        let listed = list_os_studio_catalog_entries(port.clone()).expect("list");
        assert!(listed.iter().any(|row| row.id == entry.id));
        delete_os_studio(&entry.id, port.clone()).expect("delete");
        assert!(!list_os_studio_catalog_entries(port)
            .expect("list")
            .iter()
            .any(|row| row.id == entry.id));
    }

    #[test]
    fn validates_media_graph_cycles() {
        assert!(validate_media_graph(&empty_media_graph()).ok);
    }
}
