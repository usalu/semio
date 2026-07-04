//! 📏 CAD plugin — spatial model play app bundled as a hot-swappable WASM component.

use cad_document::{empty_cad_projection, CadNode, CadOp, CadScene, CAD_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, ui_inspector_groups_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, CommandDescriptor, PluginApp,
    PluginBundle, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, World3dScene, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use vcs::{Operation, OperationDiff};

//#region 🔖Constants
const CAD_PLAY_APP_ID: &str = "cad-play";
const CAD_PLAY_CONTROLLER_ID: &str = "cad-play";
const CAD_PLAY_SURFACE_COMPOSITE: &str = "cad.play.composite";
const CAD_PLAY_BODY_COMPOSITE: &str = "cad.play.composite";
const CAD_PLAY_BODY_HIERARCHY: &str = "cad.play.hierarchy";
const CAD_PLAY_BODY_CATALOGUE: &str = "cad.play.catalogue";
const CAD_PLAY_BODY_PROPERTIES: &str = "cad.play.properties";
const CAD_PLAY_WINDOW_COMPOSITE: &str = "cad-composite";

static CAD_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

const TYPOLOGY_CATALOG: &[(&str, &str, &str)] = &[
    ("building.building.slab", "Slab", "square"),
    ("building.building.column", "Column", "columns"),
    ("building.building.beam", "Beam", "minus"),
    ("building.building.wall", "Wall", "panel-top"),
    ("spatial.shape.box", "Box", "box"),
];
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CadCamera {
    #[serde(default = "default_camera_position")]
    position: [f64; 3],
    #[serde(default = "default_camera_target")]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
    #[serde(default = "default_fov")]
    fov: f64,
}

fn default_camera_position() -> [f64; 3] {
    [12.0, -12.0, 8.0]
}

fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadObject {
    id: String,
    label: String,
    typology: String,
    #[serde(default)]
    visible: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayDocument {
    schema: String,
    id: String,
    #[serde(default)]
    camera: CadCamera,
    #[serde(default)]
    objects: Vec<CadObject>,
    #[serde(default)]
    nodes: Vec<CadNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_tool: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayRuntime {
    #[serde(default)]
    selected_object_ids: Vec<String>,
    #[serde(default)]
    selected_node_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayEnvelope {
    document: CadPlayDocument,
    #[serde(default)]
    runtime: CadPlayRuntime,
}

fn default_document() -> CadPlayDocument {
    let mut scene = empty_cad_projection();
    scene.nodes = vec![
        CadNode {
            id: "node-root".into(),
            label: "Model".into(),
            kind: "group".into(),
        },
        CadNode {
            id: "node-box".into(),
            label: "Box".into(),
            kind: "solid".into(),
        },
    ];
    CadPlayDocument {
        schema: "cad.document".into(),
        id: "cad".into(),
        camera: CadCamera {
            position: default_camera_position(),
            target: default_camera_target(),
            zoom: 1.0,
            fov: default_fov(),
        },
        objects: vec![CadObject {
            id: "object-box-1".into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
        }],
        nodes: scene.nodes,
        active_tool: Some("selectDirect".into()),
    }
}

fn default_envelope() -> CadPlayEnvelope {
    CadPlayEnvelope {
        document: default_document(),
        runtime: CadPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> CadPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn next_cad_id(prefix: &str) -> String {
    let next = CAD_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

fn set_document_op(envelope: &CadPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn cad_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: CAD_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn camera_json(camera: &CadCamera) -> String {
    json!({
        "x": camera.position[0],
        "y": camera.position[1],
        "z": camera.position[2],
        "fov": camera.fov,
    })
    .to_string()
}

fn world_instances_json(document: &CadPlayDocument, runtime: &CadPlayRuntime) -> String {
    let instances: Vec<Value> = document
        .objects
        .iter()
        .enumerate()
        .filter(|(_, object)| object.visible)
        .map(|(index, object)| {
            let selected = runtime.selected_object_ids.contains(&object.id);
            json!({
                "id": object.id,
                "x": index as f64 * 1.5,
                "y": 0.0,
                "z": 0.0,
                "scale": 1.0,
                "label": object.label,
                "color": if selected { "#3b82f6" } else { "#64748b" },
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn apply_cad_node_op(document: &CadPlayDocument, op: &CadOp) -> CadPlayDocument {
    let scene = CadScene {
        schema: CAD_DOCUMENT_SCHEMA.into(),
        id: document.id.clone(),
        nodes: document.nodes.clone(),
    };
    let diff = op.diff(&scene);
    let next_scene = diff.apply(&scene);
    let mut next = document.clone();
    next.nodes = next_scene.nodes;
    next
}
//#endregion 🔖Document

//#region 🔖Panels
fn tree_item_with_command(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    command: CommandDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        command: Some(command),
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_hierarchy_tree(envelope: &CadPlayEnvelope) -> UiNode {
    let object_items: Vec<UiTreeItemNode> = envelope
        .document
        .objects
        .iter()
        .map(|object| {
            tree_item_with_command(
                format!("cad-object:{}", object.id),
                object.label.clone(),
                Some("box"),
                cad_cmd("setSelection", Some(json!({ "objectIds": [object.id] }))),
            )
        })
        .collect();
    let node_items: Vec<UiTreeItemNode> = envelope
        .document
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_command(
                format!("cad-node:{}", node.id),
                node.label.clone(),
                Some("git-branch"),
                cad_cmd("setNodeSelection", Some(json!({ "nodeIds": [node.id] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "cad-play-hierarchy.objects".into(),
                label: Some("Objects".into()),
                default_open: Some(true),
                items: object_items,
            },
            UiTreeSectionNode {
                id: "cad-play-hierarchy.nodes".into(),
                label: Some("Nodes".into()),
                default_open: Some(true),
                items: node_items,
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<UiTreeItemNode> = TYPOLOGY_CATALOG
        .iter()
        .map(|(typology, label, icon)| {
            tree_item_with_command(
                format!("cad-play-catalogue.{typology}"),
                *label,
                Some(icon),
                cad_cmd("addObject", Some(json!({ "typology": typology }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "cad-play-catalogue.typologies".into(),
            label: Some("Typologies".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_properties_panel(envelope: &CadPlayEnvelope) -> UiNode {
    if let Some(object_id) = envelope.runtime.selected_object_ids.first() {
        if let Some(object) = envelope.document.objects.iter().find(|entry| &entry.id == object_id) {
            return ui_inspector_groups_to_tree(&[object_inspector_group(object)]);
        }
    }
    if let Some(node_id) = envelope.runtime.selected_node_ids.first() {
        if let Some(node) = envelope.document.nodes.iter().find(|entry| &entry.id == node_id) {
            return ui_inspector_groups_to_tree(&[node_inspector_group(node)]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {}", envelope.document.schema)),
        ui_text(format!(
            "Tool: {}",
            envelope
                .document
                .active_tool
                .clone()
                .unwrap_or_else(|| "selectDirect".into())
        )),
        ui_text(format!("Objects: {}", envelope.document.objects.len())),
    ])
}

fn object_inspector_group(object: &CadObject) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.object".into(),
        label: "Object".into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.label".into(),
                label: "Label".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "cad-play-inspector.object.label.input".into(),
                    input_kind: "text".into(),
                    value: object.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_cmd(
                        "patchObject",
                        Some(json!({ "objectId": object.id, "field": "label" })),
                    ),
                }),
            }),
            ui_inspector_readonly_field("cad-play-inspector.object.typology", "Typology", &object.typology),
        ],
    }
}

fn node_inspector_group(node: &CadNode) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.node".into(),
        label: "Node".into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.node.label".into(),
                label: "Label".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "cad-play-inspector.node.label.input".into(),
                    input_kind: "text".into(),
                    value: node.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_cmd(
                        "renameNode",
                        Some(json!({ "nodeId": node.id })),
                    ),
                }),
            }),
            ui_inspector_readonly_field("cad-play-inspector.node.kind", "Kind", &node.kind),
        ],
    }
}
//#endregion 🔖Panels

//#region 🔖CadApp
struct CadApp;

impl PluginApp for CadApp {
    fn app_id(&self) -> &str {
        CAD_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("cad envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.document.active_tool = Some(tool.into());
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelection" => {
                let object_ids: Vec<String> = args
                    .and_then(|value| value.get("objectIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_object_ids = object_ids;
                envelope.runtime.selected_node_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "setNodeSelection" => {
                let node_ids: Vec<String> = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_node_ids = node_ids;
                envelope.runtime.selected_object_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addObject" => {
                let typology = args.and_then(|value| value.get("typology")).and_then(|value| value.as_str()).unwrap_or("spatial.shape.box");
                let label = TYPOLOGY_CATALOG
                    .iter()
                    .find(|(id, _, _)| *id == typology)
                    .map(|(_, name, _)| *name)
                    .unwrap_or("Object");
                let id = next_cad_id("object");
                envelope.document.objects.push(CadObject {
                    id: id.clone(),
                    label: format!("{label} {}", envelope.document.objects.len() + 1),
                    typology: typology.into(),
                    visible: true,
                });
                envelope.runtime.selected_object_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "patchObject" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned();
                for object in &mut envelope.document.objects {
                    if object.id != object_id {
                        continue;
                    }
                    if field == "label" {
                        if let Some(label) = value.as_ref().and_then(|entry| entry.as_str()) {
                            object.label = label.into();
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("solid");
                let id = next_cad_id("node");
                let label = format!("Node {}", envelope.document.nodes.len() + 1);
                envelope.document = apply_cad_node_op(
                    &envelope.document,
                    &CadOp::AddNode {
                        node: CadNode {
                            id: id.clone(),
                            label,
                            kind: kind.into(),
                        },
                    },
                );
                envelope.runtime.selected_node_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "renameNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).unwrap_or("");
                let label = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if !node_id.is_empty() && !label.is_empty() {
                    envelope.document = apply_cad_node_op(
                        &envelope.document,
                        &CadOp::RenameNode {
                            node_id: node_id.into(),
                            label: label.into(),
                        },
                    );
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            CAD_PLAY_BODY_COMPOSITE => build_world_3d_scene(
                CAD_PLAY_SURFACE_COMPOSITE,
                CAD_PLAY_APP_ID,
                World3dScene {
                    camera_json: camera_json(&envelope.document.camera),
                    meshes_json: "[]".into(),
                    instances_json: world_instances_json(&envelope.document, &envelope.runtime),
                    selection_json: "[]".into(),
                },
            ),
            CAD_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope),
            CAD_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            CAD_PLAY_BODY_PROPERTIES => build_properties_panel(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖CadApp

//#region 🔖Manifest
fn create_cad_app() -> App {
    App::from_builder(
        App::builder(CAD_PLAY_APP_ID, "CAD")
            .icon_id("box")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(CAD_PLAY_WINDOW_COMPOSITE, "Model", CAD_PLAY_BODY_COMPOSITE)
            .default_layout(create_default_layout(
                &[CAD_PLAY_WINDOW_COMPOSITE.into()],
                "row",
                Some(&[100.0]),
                Some(&["Model".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                CAD_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                CAD_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                CAD_PLAY_BODY_PROPERTIES,
            ),
    )
    .example("default", "Default", &serde_json::to_string(&default_envelope()).unwrap())
    .program("cad", "CAD", "model")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("cad", "CAD", "0.1.0").register_app(create_cad_app(), || Box::new(CadApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_world_scene() {
        let app = CadApp;
        let document = app.initial_document_json();
        let node = app.render(CAD_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn hierarchy_lists_objects_and_nodes() {
        let app = CadApp;
        let document = app.initial_document_json();
        let node = app.render(CAD_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-object:"));
        assert!(json.contains("cad-node:"));
    }

    #[test]
    fn add_object_command_appends_object() {
        let mut app = CadApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "addObject",
            Some(&json!({ "typology": "building.building.column" })),
            &document,
            &ViewState::default(),
        );
        let envelope: CadPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope
            .document
            .objects
            .iter()
            .any(|object| object.typology == "building.building.column"));
    }

    #[test]
    fn cad_document_schema_matches_domain() {
        let scene = empty_cad_projection();
        assert_eq!(scene.schema, CAD_DOCUMENT_SCHEMA);
    }

    fn apply_ops(envelope: &CadPlayEnvelope, ops: &[String]) -> CadPlayEnvelope {
        let mut next = envelope.clone();
        for op_json in ops {
            if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                if let Some(document) = op.get("document") {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        next = parsed;
                    }
                }
            }
        }
        next
    }
}
//#endregion 🧪Tests
