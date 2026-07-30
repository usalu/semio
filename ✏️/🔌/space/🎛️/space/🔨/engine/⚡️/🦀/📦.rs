//! ⚙️ S Studio app — headless compute (constitutional: engine).
//!
//! 🕳️ Deviation from the usual "engine" content: every function here computes over
//! `semio_framework_os::{OsProjection, OsOperation}` (a foreign type owned by os-core, not a local
//! document this plugin defines) — building `OsOperation` values from arguments is still pure compute,
//! not an `apply_X_operation` match on a LOCALLY-owned enum, so it carries no circular-dependency risk
//! against a local `op` crate (this app has none — see `space_op`'s own doc comment).

use space::{SpacePanelState, StudioRuntimeState, S_PLAY_CATALOGUE_TAB_ID};
use semio_framework_os::{
    create_default_os_parameter, create_os_document_id, create_os_id, list_os_workflow_vfs_children,
    media_port_spec_id, negotiate_media_contract, os_app_primary_output_kind, os_app_registration,
    parameter_id_from_port_id, patch_os_parameter, MediaContract, OsAppInstance, OsDocumentRef,
    OsMediaPort, OsOperation, OsParameter, OsParameterFieldBinding, OsParameterType, OsProjection,
    OsWorkflow, OsWorkflowVfsNodeRecord, WorkflowPosition,
};
use semio_framework_plugin::ViewState;
use serde_json::{json, Value};
use std::collections::HashMap;
use infinite_board_port_directed_dag::{
    dag_fixture_to_wire_literal, DagCamera, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, IoPortSpec,
};

//#region 🔖PanelState
pub fn parse_panel_state(view_state: &ViewState) -> SpacePanelState {
    view_state
        .panel_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_else(|| SpacePanelState {
            active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
            workflows: Vec::new(),
            spawned_apps: Vec::new(),
            active_spawned_id: None,
        })
}

/// @emoji 🗂️ Serializes a panel state for a typed `HostEffect::SetPanel` effect.
pub fn panel_json(panel: &SpacePanelState) -> String {
    serde_json::to_string(panel).unwrap_or_else(|_| "{}".into())
}
//#endregion 🔖PanelState

//#region 🔖Parameters
pub fn parameter_entity_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

pub fn parameter_name(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { name, .. }
        | OsParameter::Categorical { name, .. }
        | OsParameter::Toggle { name, .. }
        | OsParameter::Text { name, .. } => name,
    }
}

pub trait OsParameterId {
    fn id(&self) -> &str;
}

impl OsParameterId for OsParameter {
    fn id(&self) -> &str {
        parameter_entity_id(self)
    }
}

/// @emoji ➕ Builds an `AddParameter` operation with a fresh default parameter of the requested type.
pub fn add_parameter_operation(parameter_type: &OsParameterType, name: &str) -> OsOperation {
    OsOperation::AddParameter {
        parameter: create_default_os_parameter(parameter_type, name, None),
    }
}

/// @emoji 🩹 Builds a `PatchParameter` operation by folding `patch` (a `{field: value}` object) into the
/// current parameter — the store-free operation-builder used in place of os-core's `OsStore::patch_parameter`.
pub fn patch_parameter_operation(projection: &OsProjection, parameter_id: &str, patch: &Value) -> Option<OsOperation> {
    let current = projection
        .parameters
        .iter()
        .find(|parameter| parameter_entity_id(parameter) == parameter_id)?;
    Some(OsOperation::PatchParameter {
        parameter_id: parameter_id.into(),
        parameter: patch_os_parameter(current, patch),
    })
}
//#endregion 🔖Parameters

//#region 🔖Selection
pub fn primary_selected_instance_id(runtime: &StudioRuntimeState, projection: &OsProjection) -> Option<String> {
    runtime.selected_app_instance_ids.first().cloned().or_else(|| {
        runtime.selected_media_node_ids.first().and_then(|node_id| {
            projection
                .workflow
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| node.instance_id.clone())
        })
    })
}

pub fn selected_instance_ids(runtime: &StudioRuntimeState, projection: &OsProjection) -> Vec<String> {
    if !runtime.selected_app_instance_ids.is_empty() {
        return runtime.selected_app_instance_ids.clone();
    }
    runtime
        .selected_media_node_ids
        .iter()
        .filter_map(|node_id| {
            projection
                .workflow
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| node.instance_id.clone())
        })
        .collect()
}
//#endregion 🔖Selection

//#region 🔖AppInstances
/// @emoji ✨ Builds the `SpawnAppInstance` operation (minting a deterministic instance id + app-document id
/// + workflow node id, all embedded in the operation, so replay never re-mints) plus the new instance
/// id for the caller to focus.
/// The store-free operation-builder the plugin uses in place of os-core's `OsStore::spawn_app_instance`
/// (a `DocumentApp` owns no store — its wrapper does).
pub fn spawn_app_instance_operation(
    plugin_id: &str,
    app_id: &str,
    label: Option<&str>,
    position: WorkflowPosition,
) -> Option<(OsOperation, String)> {
    let registration = os_app_registration(plugin_id, app_id)?;
    let instance_id = create_os_id("app");
    let node_id = create_os_id("node");
    let instance = OsAppInstance {
        id: instance_id.clone(),
        plugin_id: plugin_id.into(),
        app_id: app_id.into(),
        label: label.map(str::to_string).unwrap_or_else(|| registration.label.clone()),
        yields: os_app_primary_output_kind(&registration),
        document: OsDocumentRef {
            document_id: create_os_document_id(),
            schema: registration.source_format.clone(),
        },
    };
    Some((OsOperation::SpawnAppInstance { instance, position, node_id }, instance_id))
}
//#endregion 🔖AppInstances

//#region 🔖MediaContractConnect
/// @emoji 🤝 Resolves the source/target `OsMediaPort`s for a proposed connect from the live projection
/// and negotiates their wire contract — shared by both connect entry points (`"connectMediaPorts"` and
/// the `nodeGraphEdit`/`"connect"` fixture edit) so neither can push an `OsOperation::ConnectWorkflowPorts` for an
/// incompatible or unresolved pair of ports.
pub fn negotiate_media_connect(projection: &OsProjection, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<MediaContract, String> {
    let source_port: &OsMediaPort = projection
        .workflow
        .nodes
        .iter()
        .find(|node| node.id == source_node_id)
        .and_then(|node| node.outputs.iter().find(|port| port.id == source_port_id))
        .ok_or_else(|| format!("unknown source port {source_node_id}:{source_port_id}"))?;
    let target_port: &OsMediaPort = projection
        .workflow
        .nodes
        .iter()
        .find(|node| node.id == target_node_id)
        .and_then(|node| node.inputs.iter().find(|port| port.id == target_port_id))
        .ok_or_else(|| format!("unknown target port {target_node_id}:{target_port_id}"))?;
    negotiate_media_contract(source_port, target_port)
}
//#endregion 🔖MediaContractConnect

//#region 🔖MediaVfs
pub fn flatten_media_vfs_rows(
    parent_id: &str,
    instances: &[OsAppInstance],
    graph: &OsWorkflow,
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    rows: &mut Vec<Value>,
) {
    let children = list_os_workflow_vfs_children(parent_id, instances, graph, bindings, parameters);
    for child in &children {
        rows.push(vfs_node_to_row(child));
        if child.has_children {
            flatten_media_vfs_rows(&child.id, instances, graph, bindings, parameters, rows);
        }
    }
}

pub fn vfs_node_to_row(node: &OsWorkflowVfsNodeRecord) -> Value {
    json!({
        "id": node.id,
        "fileNodeKindId": node.file_node_kind_id,
        "name": node.name,
        "path": node.path,
        "parentId": node.parent_id,
        "hasChildren": node.has_children,
        "navigateUri": node.navigate_uri,
        "descriptorValues": node.descriptor_values
    })
}

pub fn media_port_label(
    port_id: &str,
    parameter_by_id: &HashMap<String, &OsParameter>,
) -> String {
    parameter_id_from_port_id(port_id)
        .and_then(|id| parameter_by_id.get(&id).map(|row| parameter_name(row).to_string()))
        .or_else(|| media_port_spec_id(port_id))
        .unwrap_or_else(|| port_id.to_string())
}
//#endregion 🔖MediaVfs

//#region 🔖CompiledDag
pub fn workflow_to_dag_fixture(projection: &OsProjection) -> DagFixture {
    let instance_by_id: HashMap<_, _> = projection
        .app_instances
        .iter()
        .map(|row| (row.id.clone(), row))
        .collect();
    let parameter_by_id: HashMap<_, _> = projection
        .parameters
        .iter()
        .map(|row| match row {
            OsParameter::Numeric { id, .. }
            | OsParameter::Categorical { id, .. }
            | OsParameter::Toggle { id, .. }
            | OsParameter::Text { id, .. } => (id.clone(), row),
        })
        .collect();
    let nodes = projection
        .workflow
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            let registration = instance
                .and_then(|row| os_app_registration(&row.plugin_id, &row.app_id));
            let icon = format!(
                "emoji:{}",
                registration
                    .map(|row| row.component_kind.clone())
                    .unwrap_or_else(|| "s".into())
            );
            DagNodeSpec {
                id: node.id.clone(),
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
                icon: icon.clone(),
                x: node.x + node.width / 2.0,
                y: node.y + node.height / 2.0,
                width: node.width,
                height: node.height,
                operator_kind: instance.map(|row| row.plugin_id.clone()),
                kind: DagNodeKind::AppInstance {
                    instance_id: node.instance_id.clone(),
                    plugin_id: instance
                        .map(|row| row.plugin_id.clone())
                        .unwrap_or_default(),
                    app_id: instance
                        .map(|row| row.app_id.clone())
                        .unwrap_or_default(),
                    icon,
                    inputs: node
                        .inputs
                        .iter()
                        .map(|port| {
                            let mut spec = IoPortSpec::simple(
                                &port.id,
                                media_port_label(&port.id, &parameter_by_id),
                            );
                            spec.artifact_kind = Some(port.artifact_kind.clone());
                            spec
                        })
                        .collect(),
                    outputs: node
                        .outputs
                        .iter()
                        .map(|port| {
                            let mut spec = IoPortSpec::simple(
                                &port.id,
                                media_port_label(&port.id, &parameter_by_id),
                            );
                            spec.artifact_kind = Some(port.artifact_kind.clone());
                            spec
                        })
                        .collect(),
                },
                ..Default::default()
            }
        })
        .collect();
    let edges = projection
        .workflow
        .edges
        .iter()
        .map(|edge| DagFixtureEdge {
            id: edge.id.clone(),
            source: format!("{}@{}", edge.source_node_id, edge.source_port_id),
            target: format!("{}@{}", edge.target_node_id, edge.target_port_id),
            ..Default::default()
        })
        .collect();
    DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        },
        nodes,
        edges,
    }
}

pub fn compiled_dag_wire_literal(projection: &OsProjection) -> String {
    let fixture = workflow_to_dag_fixture(projection);
    dag_fixture_to_wire_literal(&fixture)
}
//#endregion 🔖CompiledDag

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use space_shared::demo_space_projection;

    #[test]
    fn patch_parameter_op_updates_numeric_value() {
        let projection = demo_space_projection();
        let operation = patch_parameter_operation(&projection, "param-brush-size", &json!({ "value": 48.0 })).expect("operation");
        match operation {
            OsOperation::PatchParameter { parameter, .. } => match parameter {
                OsParameter::Numeric { value, .. } => assert_eq!(value, 48.0),
                _ => panic!("expected numeric"),
            },
            _ => panic!("expected patch parameter operation"),
        }
    }

    #[test]
    fn compiled_dag_wire_literal_mentions_app_instances() {
        let wire = compiled_dag_wire_literal(&demo_space_projection());
        assert!(wire.contains("appInstance") || wire.contains("draw"));
    }
}
//#endregion 🧪Tests
