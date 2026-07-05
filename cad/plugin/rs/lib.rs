//! 📏 CAD plugin — spatial model play app bundled as a hot-swappable WASM component.

use cad_document::{empty_cad_projection, CadNode, CadOp, CadScene, CAD_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, export_mesh_glb_bytes, export_mesh_obj,
    merge_world_selection_ids, mesh_from_kind, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene,
    world3d_selection_json, App, CommandDescriptor, MeshData, PluginApp, PluginBundle, UiControlNode, UiFieldNode,
    UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_os::{register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
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
const CAD_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
const CAD_FALLBACK_MESH_KIND: &str = "box";

static CAD_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

const TYPOLOGY_MESH_URLS: &[(&str, &str)] = &[
    ("building.building.slab", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.column", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.beam", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("building.building.wall", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
    ("spatial.shape.box", "/mesh/hexagonal-cut-concrete-forest-left.glb"),
];

const TYPOLOGY_CATALOG: &[(&str, &str, &str)] = &[
    ("building.building.slab", "Slab", "square"),
    ("building.building.column", "Column", "columns"),
    ("building.building.beam", "Beam", "minus"),
    ("building.building.wall", "Wall", "panel-top"),
    ("spatial.shape.box", "Box", "box"),
];

const FOREST_LEFT_MODEL_JSON: &str =
    include_str!("../../asset/play/hexagonal-cut-concrete-forest-left.model.json");
//#endregion 🔖Constants

//#region 🔖BrepMeshes
use kernel_3d_brepkit::BrepkitKernel;
use kernel_3d_engine::{block_on, BrepKernel, GeometryHandle, MeshTransfer};
use semio_framework_core::mesh_from_indexed;
use std::sync::{Mutex, OnceLock};

static CAD_BREP_KERNEL: OnceLock<Mutex<BrepkitKernel>> = OnceLock::new();

fn cad_brep_kernel() -> &'static Mutex<BrepkitKernel> {
    CAD_BREP_KERNEL.get_or_init(|| Mutex::new(BrepkitKernel::new()))
}

fn typology_brep_mesh(typology: &str) -> MeshData {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return mesh_from_kind(typology_mesh_kind(typology));
    };
    let handle = block_on(async {
        match typology {
            "building.building.column" => kernel.cylinder_prim(0.25, 3.0).await,
            "building.building.beam" => kernel.box_prim(6.0, 0.3, 0.4).await,
            "building.building.slab" => kernel.box_prim(6.0, 4.0, 0.3).await,
            "building.building.wall" => kernel.box_prim(0.2, 4.0, 3.0).await,
            "spatial.shape.box" => kernel.box_prim(1.0, 1.0, 1.0).await,
            _ => kernel.box_prim(1.0, 1.0, 1.0).await,
        }
    });
    let Ok(handle) = handle else {
        return mesh_from_kind(typology_mesh_kind(typology));
    };
    let mesh: MeshTransfer = match block_on(kernel.tessellate(&handle, 0.1)) {
        Ok(mesh) => mesh,
        Err(_) => {
            let _ = block_on(kernel.dispose(&handle));
            return mesh_from_kind(typology_mesh_kind(typology));
        }
    };
    let _ = block_on(kernel.dispose(&handle));
    mesh_from_indexed(&mesh.position, &mesh.normal, &mesh.index)
}

fn object_origin_from_vertices(object_id: &str, vertices: &[Value]) -> [f64; 3] {
    let bim_token = object_id.strip_prefix("object-").unwrap_or(object_id);
    let mut count = 0usize;
    let mut sum = [0.0f64; 3];
    for vertex in vertices {
        let vertex_id = vertex.get("id").and_then(|value| value.as_str()).unwrap_or("");
        if !vertex_id.starts_with(bim_token) || !vertex_id.contains("-vertex-") {
            continue;
        }
        let Some(position) = vertex.get("position").and_then(|value| value.as_array()) else {
            continue;
        };
        if position.len() < 3 {
            continue;
        }
        sum[0] += position[0].as_f64().unwrap_or(0.0);
        sum[1] += position[1].as_f64().unwrap_or(0.0);
        sum[2] += position[2].as_f64().unwrap_or(0.0);
        count += 1;
    }
    if count == 0 {
        return [0.0, 0.0, 0.0];
    }
    let n = count as f64;
    [sum[0] / n, sum[1] / n, sum[2] / n]
}

fn cad_document_from_modelspace(json: &str, id: &str) -> Option<CadPlayDocument> {
    let root: Value = serde_json::from_str(json).ok()?;
    let objects = root.pointer("/models/0/model/objects")?.as_array()?;
    let vertices = root
        .pointer("/models/0/model/geometry/vertices")
        .and_then(|value| value.as_array())
        .map(|entries| entries.as_slice())
        .unwrap_or(&[]);
    let cad_objects: Vec<CadObject> = objects
        .iter()
        .filter_map(|entry| {
            let object_id = entry.get("id")?.as_str()?;
            let typology = entry.get("typology")?.as_str()?;
            let label = object_id
                .split('-')
                .last()
                .map(str::to_string)
                .unwrap_or_else(|| object_id.to_string());
            let mesh_url = TYPOLOGY_MESH_URLS
                .iter()
                .find(|(kind, _)| *kind == typology)
                .map(|(_, url)| url.to_string());
            Some(CadObject {
                id: object_id.into(),
                label,
                typology: typology.into(),
                visible: true,
                origin: object_origin_from_vertices(object_id, vertices),
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                mesh_url,
            })
        })
        .collect();
    if cad_objects.is_empty() {
        return None;
    }
    Some(CadPlayDocument {
        schema: "cad.document".into(),
        id: id.into(),
        camera: CadCamera {
            position: [12.0, -12.0, 8.0],
            target: [5.4, 2.34, 1.5],
            zoom: 1.0,
            fov: 50.0,
        },
        objects: cad_objects,
        nodes: vec![CadNode {
            id: "node-root".into(),
            label: "Concrete Forest Left".into(),
            kind: "group".into(),
        }],
        active_tool: Some("selectDirect".into()),
    })
}
//#endregion 🔖BrepMeshes

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

fn default_selection_method() -> String {
    "rectangle".into()
}

fn typology_mesh_kind(typology: &str) -> &'static str {
    match typology {
        "building.building.column" => "cylinder",
        "spatial.shape.box" => "box",
        _ => "box",
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadObject {
    id: String,
    label: String,
    typology: String,
    #[serde(default)]
    visible: bool,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
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
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_object_id: Option<String>,
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
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            mesh_url: Some("/mesh/hexagonal-cut-concrete-forest-left.glb".into()),
        }],
        nodes: scene.nodes,
        active_tool: Some("selectDirect".into()),
    }
}

fn forest_play_document() -> CadPlayDocument {
    cad_document_from_modelspace(FOREST_LEFT_MODEL_JSON, "hexagonal-cut-concrete-forest-left")
        .unwrap_or_else(|| CadPlayDocument {
        schema: "cad.document".into(),
        id: "hexagonal-cut-concrete-forest-left".into(),
        camera: CadCamera {
            position: [12.0, -12.0, 8.0],
            target: [5.4, 2.34, 1.5],
            zoom: 1.0,
            fov: 50.0,
        },
        objects: vec![
            CadObject {
                id: "object-slab".into(),
                label: "Slab".into(),
                typology: "building.building.slab".into(),
                visible: true,
                origin: [5.4, 2.34, 1.5],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                mesh_url: None,
            },
            CadObject {
                id: "object-column-10".into(),
                label: "Column 10".into(),
                typology: "building.building.column".into(),
                visible: true,
                origin: [4.05, 4.68, 3.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                mesh_url: None,
            },
            CadObject {
                id: "object-column-11".into(),
                label: "Column 11".into(),
                typology: "building.building.column".into(),
                visible: true,
                origin: [6.75, 4.68, 3.0],
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                mesh_url: None,
            },
            CadObject {
                id: "object-beam-2".into(),
                label: "Beam 2".into(),
                typology: "building.building.beam".into(),
                visible: true,
                origin: [5.4, 2.34, 3.0],
                orientation: Some([0.0, 0.7071, 0.0, 0.7071]),
                mesh_url: None,
            },
        ],
        nodes: vec![
            CadNode {
                id: "node-root".into(),
                label: "Concrete Forest Left".into(),
                kind: "group".into(),
            },
        ],
        active_tool: Some("selectDirect".into()),
        })
}

fn forest_play_envelope() -> CadPlayEnvelope {
    CadPlayEnvelope {
        document: forest_play_document(),
        runtime: CadPlayRuntime::default(),
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

fn resolve_object_mesh_url(object: &CadObject) -> Option<String> {
    if let Some(url) = object.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    TYPOLOGY_MESH_URLS
        .iter()
        .find(|(typology, _)| *typology == object.typology)
        .map(|(_, url)| url.to_string())
}

fn collect_mesh_urls(document: &CadPlayDocument) -> Vec<String> {
    let mut urls = HashSet::new();
    for object in &document.objects {
        if let Some(url) = resolve_object_mesh_url(object) {
            urls.insert(url);
        }
    }
    urls.into_iter().collect()
}

fn object_scale_json(_object: &CadObject) -> [f64; 3] {
    [1.0, 1.0, 1.0]
}

fn world_instances_json(document: &CadPlayDocument, runtime: &CadPlayRuntime) -> String {
    let instances: Vec<Value> = document
        .objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object)
                .map(|url| world3d_mesh_id_from_url(&url))
                .unwrap_or_else(|| typology_mesh_kind(&object.typology).to_string());
            let selected = runtime.selected_object_ids.contains(&object.id);
            let hovered = runtime.hovered_object_id.as_deref() == Some(object.id.as_str());
            json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": object_scale_json(object),
                "label": object.label,
                "color": if selected { "#3b82f6" } else { "#64748b" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(document: &CadPlayDocument) -> String {
    let urls = collect_mesh_urls(document);
    if !urls.is_empty() {
        return semio_framework_plugin::world3d_meshes_json_from_urls(&urls);
    }
    let mut kinds: Vec<String> = document
        .objects
        .iter()
        .map(|object| typology_mesh_kind(&object.typology).to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    if kinds.is_empty() {
        kinds.push(CAD_FALLBACK_MESH_KIND.into());
    }
    let meshes: Vec<Value> = kinds
        .iter()
        .map(|kind| {
            let typology = document
                .objects
                .iter()
                .find(|object| typology_mesh_kind(&object.typology) == kind.as_str())
                .map(|object| object.typology.as_str())
                .unwrap_or("spatial.shape.box");
            let data = typology_brep_mesh(typology);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn world_selection_json(runtime: &CadPlayRuntime) -> String {
    world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_object_ids,
        runtime.hovered_object_id.as_deref(),
    )
}

fn export_mesh_from_envelope(envelope: &CadPlayEnvelope) -> MeshData {
    let typology = envelope
        .document
        .objects
        .iter()
        .find(|object| envelope.runtime.selected_object_ids.contains(&object.id))
        .map(|object| object.typology.as_str())
        .unwrap_or("spatial.shape.box");
    typology_brep_mesh(typology)
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
        hover_command: None,
        unhover_command: None,
        actions: None,
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
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    CadPlayEnvelope {
                        document: CadPlayDocument {
                            schema: "cad.document".into(),
                            id: "cad".into(),
                            camera: CadCamera {
                                position: default_camera_position(),
                                target: default_camera_target(),
                                zoom: 1.0,
                                fov: default_fov(),
                            },
                            objects: Vec::new(),
                            nodes: Vec::new(),
                            active_tool: Some("selectDirect".into()),
                        },
                        runtime: CadPlayRuntime::default(),
                    }
                } else if example_id == "default" {
                    default_envelope()
                } else if example_id == CAD_EXAMPLE_FOREST_LEFT || example_id == "forest-left" {
                    forest_play_envelope()
                } else {
                    envelope
                };
                return vec![set_document_op(&envelope)];
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
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    mesh_url: TYPOLOGY_MESH_URLS
                        .iter()
                        .find(|(entry, _)| *entry == typology)
                        .map(|(_, url)| url.to_string()),
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
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_object_ids =
                    merge_world_selection_ids(&envelope.runtime.selected_object_ids, &ids, merge);
                envelope.runtime.selected_node_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_object_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setSelectionMethod" => {
                let method = args
                    .and_then(|value| value.get("method"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                return vec![set_document_op(&envelope)];
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
                world3d_scene(
                    camera_json(&envelope.document.camera),
                    world_meshes_json(&envelope.document),
                    world_instances_json(&envelope.document, &envelope.runtime),
                    world_selection_json(&envelope.runtime),
                ),
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
    .example(
        CAD_EXAMPLE_FOREST_LEFT,
        "Hexagonal Cut Concrete Forest Left",
        &serde_json::to_string(&forest_play_envelope()).unwrap(),
    )
    .program("cad", "CAD", "model")
}

fn bundle() -> PluginBundle {
    register_cad_exports();
    PluginBundle::new("cad", "CAD", "0.1.0").register_app(create_cad_app(), || Box::new(CadApp))
}

fn register_cad_exports() {
    register_os_media_export_handler("3d.cad", OsMediaExportFormat::Obj, |doc| {
        let envelope: CadPlayEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (data, mime_type) = export_mesh_obj(&mesh, "cad");
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: "cad.obj".into(),
        })
    });
    register_os_media_export_handler("3d.cad", OsMediaExportFormat::Glb, |doc| {
        let envelope: CadPlayEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (bytes, mime_type) = export_mesh_glb_bytes(&mesh);
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type,
            file_name: "cad.glb".into(),
        })
    });
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
    fn forest_example_uses_mesh_urls_and_origins() {
        let envelope = forest_play_envelope();
        let json = world_instances_json(&envelope.document, &envelope.runtime);
        assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        assert!(json.contains("4.05") || json.contains("8.10"));
        let meshes = world_meshes_json(&envelope.document);
        assert!(meshes.contains("hexagonal-cut-concrete-forest-left.glb"));
        assert!(envelope.document.objects.len() > 5);
    }

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
