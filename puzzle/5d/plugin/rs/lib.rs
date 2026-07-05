//! 👯 Puzzle 5D plugin — 2D/3D coupled puzzle play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_canvas_2d_scene, build_world_3d_scene, create_default_layout, merge_world_selection_ids,
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text,
    world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene, world3d_selection_json,
    App, Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, UiControlNode, UiFieldNode, UiInspectorFieldGroup,
    UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
const PUZZLE5D_PLAY_SURFACE_2D: &str = "puzzle.5d.play.2d";
const PUZZLE5D_PLAY_SURFACE_3D: &str = "puzzle.5d.play.3d";
const PUZZLE5D_PLAY_BODY_2D: &str = "puzzle.5d.play.2d";
const PUZZLE5D_PLAY_BODY_3D: &str = "puzzle.5d.play.3d";
const PUZZLE5D_PLAY_BODY_HIERARCHY: &str = "puzzle.5d.play.hierarchy";
const PUZZLE5D_PLAY_BODY_KINDS: &str = "puzzle.5d.play.kinds";
const PUZZLE5D_PLAY_BODY_INSPECTOR: &str = "puzzle.5d.play.inspector";
const PUZZLE5D_PLAY_WINDOW_2D: &str = "puzzle5d-2d";
const PUZZLE5D_PLAY_WINDOW_3D: &str = "puzzle5d-3d";
const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
const PUZZLE5D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
const PUZZLE5D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";

const PUZZLE5D_FALLBACK_MESH_KIND: &str = "box";

const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../example/concrete-forest.5d.json");
const NAKAGIN_EXAMPLE_JSON: &str = include_str!("../../example/nakagin-capsule-tower.5d.json");
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dCamera2d {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "one_f64")]
    zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dCamera3d {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
}

fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart2d {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    shape: String,
    #[serde(default)]
    radius: f64,
    #[serde(default)]
    text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart3d {
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart {
    id: String,
    #[serde(rename = "partKind")]
    part_kind: String,
    #[serde(default, rename = "2d")]
    part_2d: Puzzle5dPart2d,
    #[serde(default, rename = "3d")]
    part_3d: Puzzle5dPart3d,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dDocument {
    schema: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    camera2d: Puzzle5dCamera2d,
    #[serde(default)]
    camera3d: Puzzle5dCamera3d,
    #[serde(default)]
    parts: Vec<Puzzle5dPart>,
    #[serde(default, rename = "kindCatalogs")]
    kind_catalogs: Option<Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dSelection {
    #[serde(default)]
    part_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dRuntime {
    #[serde(default)]
    selection: Puzzle5dSelection,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_part_id: Option<String>,
    #[serde(default = "default_active_tool")]
    active_tool: String,
    #[serde(default)]
    fill_count: u32,
}

fn default_active_tool() -> String {
    "select".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dEnvelope {
    document: Puzzle5dDocument,
    #[serde(default)]
    runtime: Puzzle5dRuntime,
}

static PUZZLE5D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

fn empty_document() -> Puzzle5dDocument {
    Puzzle5dDocument {
        schema: PUZZLE5D_SCHEMA.into(),
        domain: "architecture".into(),
        camera2d: Puzzle5dCamera2d::default(),
        camera3d: Puzzle5dCamera3d::default(),
        parts: Vec::new(),
        kind_catalogs: None,
    }
}

fn envelope_from_document_json(json_text: &str) -> Option<Puzzle5dEnvelope> {
    serde_json::from_str::<Puzzle5dDocument>(json_text)
        .ok()
        .map(|document| Puzzle5dEnvelope {
            document,
            runtime: Puzzle5dRuntime::default(),
        })
}

fn default_envelope() -> Puzzle5dEnvelope {
    envelope_from_document_json(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|| Puzzle5dEnvelope {
        document: empty_document(),
        runtime: Puzzle5dRuntime::default(),
    })
}

fn parse_envelope(document_json: &str) -> Puzzle5dEnvelope {
    if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(document_json) {
        return Puzzle5dEnvelope {
            document,
            runtime: Puzzle5dRuntime::default(),
        };
    }
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Puzzle5dEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn puzzle5d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn canvas_layers_json(document: &Puzzle5dDocument, selection: &Puzzle5dSelection) -> String {
    let layers: Vec<Value> = document
        .parts
        .iter()
        .map(|part| {
            let shape = if part.part_2d.shape.is_empty() {
                "circle"
            } else {
                part.part_2d.shape.as_str()
            };
            json!({
                "id": part.id,
                "kind": shape,
                "x": part.part_2d.x,
                "y": part.part_2d.y,
                "radius": part.part_2d.radius.max(8.0),
                "label": part.part_2d.text,
                "selected": selection.part_ids.contains(&part.id),
            })
        })
        .collect();
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

fn resolve_part_mesh_url(part: &Puzzle5dPart, kind_catalogs: Option<&Value>) -> Option<String> {
    if let Some(url) = part.part_3d.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    let catalogs = kind_catalogs?;
    let parts = catalogs.get("parts")?.as_array()?;
    for entry in parts {
        if entry.get("id").and_then(|v| v.as_str()) == Some(part.part_kind.as_str()) {
            return entry
                .get("meshUrl")
                .and_then(|v| v.as_str())
                .map(str::to_string);
        }
    }
    None
}

fn collect_mesh_urls(document: &Puzzle5dDocument) -> Vec<String> {
    let mut urls = HashSet::new();
    for part in &document.parts {
        if let Some(url) = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()) {
            urls.insert(url);
        }
    }
    if let Some(catalogs) = document.kind_catalogs.as_ref() {
        if let Some(parts) = catalogs.get("parts").and_then(|v| v.as_array()) {
            for entry in parts {
                if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                    urls.insert(url.to_string());
                }
            }
        }
    }
    urls.into_iter().collect()
}

fn part_scale_json(part: &Puzzle5dPart) -> [f64; 3] {
    match &part.part_3d.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [
            values[0].as_f64().unwrap_or(1.0),
            values[1].as_f64().unwrap_or(1.0),
            values[2].as_f64().unwrap_or(1.0),
        ],
        _ => [1.0, 1.0, 1.0],
    }
}

fn world_instances_json(document: &Puzzle5dDocument, runtime: &Puzzle5dRuntime) -> String {
    let instances: Vec<Value> = document
        .parts
        .iter()
        .map(|part| {
            let selected = runtime.selection.part_ids.contains(&part.id);
            let hovered = runtime.hovered_part_id.as_deref() == Some(part.id.as_str());
            let mesh_id = resolve_part_mesh_url(part, document.kind_catalogs.as_ref())
                .map(|url| world3d_mesh_id_from_url(&url))
                .unwrap_or_else(|| PUZZLE5D_FALLBACK_MESH_KIND.into());
            json!({
                "id": part.id,
                "meshId": mesh_id,
                "position": [
                    part.part_3d.origin.first().copied().unwrap_or(0.0),
                    part.part_3d.origin.get(1).copied().unwrap_or(0.0),
                    part.part_3d.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": part_scale_json(part),
                "label": part.part_kind,
                "color": if selected { "#8b5cf6" } else { "#94a3b8" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(document: &Puzzle5dDocument) -> String {
    world3d_meshes_json_from_kinds_and_urls(&[PUZZLE5D_FALLBACK_MESH_KIND.into()], &collect_mesh_urls(document))
}

fn world_selection_json(runtime: &Puzzle5dRuntime) -> String {
    world3d_selection_json(
        &runtime.selection_method,
        &runtime.selection.part_ids,
        runtime.hovered_part_id.as_deref(),
    )
}

fn camera3d_json(camera: &Puzzle5dCamera3d) -> String {
    json!({
        "x": camera.position.first().copied().unwrap_or(0.0),
        "y": camera.position.get(1).copied().unwrap_or(-5.0),
        "z": camera.position.get(2).copied().unwrap_or(3.0),
        "fov": 45.0,
    })
    .to_string()
}

fn resolve_part_kind_mesh_url(part_kind: &str, kind_catalogs: Option<&Value>) -> Option<String> {
    let catalogs = kind_catalogs?;
    let parts = catalogs.get("parts")?.as_array()?;
    for entry in parts {
        if entry.get("id").and_then(|v| v.as_str()) == Some(part_kind) {
            return entry
                .get("meshUrl")
                .and_then(|v| v.as_str())
                .map(str::to_string);
        }
    }
    None
}

fn next_part_id() -> String {
    let next = PUZZLE5D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("part-{next}")
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

fn build_hierarchy_tree(envelope: &Puzzle5dEnvelope) -> UiNode {
    let items: Vec<UiTreeItemNode> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            tree_item_with_command(
                format!("puzzle5d-part:{}", part.id),
                part.part_kind.clone(),
                Some("circle"),
                puzzle5d_cmd("setSelection", Some(json!({ "partIds": [part.id] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "puzzle5d-play-hierarchy.parts".into(),
            label: Some("Parts".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_kinds_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "puzzle5d-play-kinds.parts".into(),
            label: Some("Part Kinds".into()),
            default_open: Some(true),
            items: vec![
                kind_item("Hexagonal Cut Concrete Forest Left"),
                kind_item("Hexagonal Cut Concrete Forest Right"),
            ],
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn kind_item(kind: &str) -> UiTreeItemNode {
    tree_item_with_command(
        format!("puzzle5d-kind:{kind}"),
        kind,
        Some("box"),
        puzzle5d_cmd("addPartKind", Some(json!({ "partKind": kind }))),
    )
}

fn build_inspector_tree(envelope: &Puzzle5dEnvelope) -> UiNode {
    if let Some(part_id) = envelope.runtime.selection.part_ids.first() {
        if let Some(part) = envelope.document.parts.iter().find(|entry| &entry.id == part_id) {
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
                id: "puzzle5d-play-inspector.part".into(),
                label: "Part".into(),
                default_open: None,
                fields: vec![
                    ui_inspector_readonly_field("puzzle5d-play-inspector.part.id", "Id", &part.id),
                    ui_inspector_readonly_field("puzzle5d-play-inspector.part.kind", "Kind", &part.part_kind),
                    UiNode::Field(UiFieldNode {
                        id: "puzzle5d-play-inspector.part.label".into(),
                        label: "Label".into(),
                        child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                            id: "puzzle5d-play-inspector.part.label.input".into(),
                            input_kind: "text".into(),
                            value: part.part_2d.text.clone(),
                            placeholder: None,
                            commit: None,
                            on_change: puzzle5d_cmd(
                                "patchPart",
                                Some(json!({ "partId": part.id, "field": "text" })),
                            ),
                        }),
                    }),
                ],
            }]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {}", envelope.document.schema)),
        ui_text(format!("Parts: {}", envelope.document.parts.len())),
    ])
}
//#endregion 🔖Panels

//#region 🔖Puzzle5dPlayApp
struct Puzzle5dPlayApp;

impl PluginApp for Puzzle5dPlayApp {
    fn app_id(&self) -> &str {
        PUZZLE5D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("puzzle5d envelope json")
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
                    Puzzle5dEnvelope {
                        document: empty_document(),
                        runtime: Puzzle5dRuntime::default(),
                    }
                } else if example_id == PUZZLE5D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                    default_envelope()
                } else if example_id == PUZZLE5D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
                    envelope_from_document_json(NAKAGIN_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else {
                    envelope
                };
                return vec![set_document_op(&envelope)];
            }
            "setSelection" => {
                let part_ids: Vec<String> = args
                    .and_then(|value| value.get("partIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selection.part_ids = part_ids;
                return vec![set_document_op(&envelope)];
            }
            "setActiveTool" => {
                let tool = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()).unwrap_or("select");
                if tool == "select" || tool == "brush" || tool == "fill" {
                    envelope.runtime.active_tool = tool.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "addBrushPart" => {
                let part_kind = args
                    .and_then(|value| value.get("partKind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if part_kind.is_empty() {
                    return Vec::new();
                }
                let id = next_part_id();
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
                envelope.document.parts.push(Puzzle5dPart {
                    id: id.clone(),
                    part_kind: part_kind.into(),
                    part_2d: Puzzle5dPart2d {
                        x,
                        y,
                        shape: "circle".into(),
                        radius: 20.0,
                        text: part_kind.into(),
                    },
                    part_3d: Puzzle5dPart3d {
                        origin: [0.0, 0.0, 0.0],
                        mesh_url,
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                    },
                });
                envelope.runtime.selection.part_ids = vec![id];
                envelope.runtime.active_tool = "brush".into();
                return vec![set_document_op(&envelope)];
            }
            "setFillCount" => {
                let count = args
                    .and_then(|value| value.get("count"))
                    .and_then(|value| value.as_u64())
                    .unwrap_or(0) as u32;
                envelope.runtime.fill_count = count;
                envelope.runtime.active_tool = "fill".into();
                return vec![set_document_op(&envelope)];
            }
            "addPartKind" => {
                let part_kind = args
                    .and_then(|value| value.get("partKind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("Part");
                let id = next_part_id();
                let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
                envelope.document.parts.push(Puzzle5dPart {
                    id: id.clone(),
                    part_kind: part_kind.into(),
                    part_2d: Puzzle5dPart2d {
                        x: 120.0,
                        y: 120.0,
                        shape: "circle".into(),
                        radius: 20.0,
                        text: part_kind.into(),
                    },
                    part_3d: Puzzle5dPart3d {
                        origin: [0.0, 0.0, 0.0],
                        mesh_url,
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                    },
                });
                envelope.runtime.selection.part_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "patchPart" => {
                let part_id = args.and_then(|value| value.get("partId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned();
                for part in &mut envelope.document.parts {
                    if part.id != part_id {
                        continue;
                    }
                    if field == "text" {
                        if let Some(text) = value.as_ref().and_then(|entry| entry.as_str()) {
                            part.part_2d.text = text.into();
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "setCamera2d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera2d = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setCamera3d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera3d = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selection.part_ids =
                    merge_world_selection_ids(&envelope.runtime.selection.part_ids, &ids, merge);
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_part_id = args
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
            "worldPointerDown" | "canvasPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            PUZZLE5D_PLAY_BODY_2D => build_canvas_2d_scene(
                PUZZLE5D_PLAY_SURFACE_2D,
                PUZZLE5D_PLAY_APP_ID,
                Canvas2dScene {
                    camera_x: envelope.document.camera2d.x,
                    camera_y: envelope.document.camera2d.y,
                    zoom: envelope.document.camera2d.zoom,
                    layers_json: canvas_layers_json(&envelope.document, &envelope.runtime.selection),
                },
            ),
            PUZZLE5D_PLAY_BODY_3D => build_world_3d_scene(
                PUZZLE5D_PLAY_SURFACE_3D,
                PUZZLE5D_PLAY_APP_ID,
                world3d_scene(
                    camera3d_json(&envelope.document.camera3d),
                    world_meshes_json(&envelope.document),
                    world_instances_json(&envelope.document, &envelope.runtime),
                    world_selection_json(&envelope.runtime),
                ),
            ),
            PUZZLE5D_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope),
            PUZZLE5D_PLAY_BODY_KINDS => build_kinds_tree(),
            PUZZLE5D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Puzzle5dPlayApp

//#region 🔖Manifest
fn create_puzzle5d_app() -> App {
    App::from_builder(
        App::builder(PUZZLE5D_PLAY_APP_ID, "Puzzle 5D")
            .icon_id("puzzle")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(PUZZLE5D_PLAY_WINDOW_2D, "Puzzle 2D", PUZZLE5D_PLAY_BODY_2D)
            .window_kind(PUZZLE5D_PLAY_WINDOW_3D, "Puzzle 3D", PUZZLE5D_PLAY_BODY_3D)
            .default_layout(create_default_layout(
                &[PUZZLE5D_PLAY_WINDOW_2D.into(), PUZZLE5D_PLAY_WINDOW_3D.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Puzzle 2D".into(), "Puzzle 3D".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                PUZZLE5D_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                PUZZLE5D_PLAY_BODY_KINDS,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                PUZZLE5D_PLAY_BODY_INSPECTOR,
            ),
    )
    .example(
        PUZZLE5D_EXAMPLE_CONCRETE_FOREST,
        "Concrete Forest",
        CONCRETE_FOREST_EXAMPLE_JSON,
    )
    .example(
        PUZZLE5D_EXAMPLE_NAKAGIN,
        "Nakagin Capsule Tower",
        NAKAGIN_EXAMPLE_JSON,
    )
    .program("puzzle5d", "Puzzle 5D", "model")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("puzzle5d", "Puzzle 5D", "0.1.0").register_app(create_puzzle5d_app(), || Box::new(Puzzle5dPlayApp))
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
    fn renders_canvas_2d_scene() {
        let app = Puzzle5dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PUZZLE5D_PLAY_BODY_2D, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_world_3d_scene() {
        let app = Puzzle5dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PUZZLE5D_PLAY_BODY_3D, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn concrete_forest_uses_real_mesh_urls() {
        let app = Puzzle5dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PUZZLE5D_PLAY_BODY_3D, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("hexagonal-cut-concrete-forest-left"));
    }

    #[test]
    fn set_active_tool_updates_runtime() {
        let mut app = Puzzle5dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("setActiveTool", Some(&json!({ "tool": "brush" })), &document, &ViewState::default());
        let envelope: Puzzle5dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.active_tool, "brush");
    }

    #[test]
    fn add_brush_part_appends_part() {
        let mut app = Puzzle5dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "addBrushPart",
            Some(&json!({ "partKind": "Hexagonal Cut Concrete Forest Left", "x": 200.0, "y": 180.0 })),
            &document,
            &ViewState::default(),
        );
        let envelope: Puzzle5dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.document.parts.iter().any(|part| part.part_2d.x == 200.0));
    }

    #[test]
    fn concrete_forest_example_parses() {
        let envelope = default_envelope();
        assert_eq!(envelope.document.schema, PUZZLE5D_SCHEMA);
        assert!(!envelope.document.parts.is_empty());
    }

    #[test]
    fn add_part_kind_appends_part() {
        let mut app = Puzzle5dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "addPartKind",
            Some(&json!({ "partKind": "Test Part" })),
            &document,
            &ViewState::default(),
        );
        let envelope: Puzzle5dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.document.parts.iter().any(|part| part.part_kind == "Test Part"));
    }

    fn apply_ops(envelope: &Puzzle5dEnvelope, ops: &[String]) -> Puzzle5dEnvelope {
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
