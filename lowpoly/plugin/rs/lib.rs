//! 🔺 Lowpoly plugin — mesh editing play app bundled as a hot-swappable WASM component.

use base64::Engine;
use kernel_3d_mesh::{EdgeId, FaceId, MirrorAxis, Vec3, WeldMode};
use lowpoly_core::{
    default_fixture, LowpolyDocument, LowpolyFixture, LowpolyObject, LowpolySelection,
    LowpolySelectionTargets, LOWPOLY_PAINT_TEXTURE_SIZE,
};
use png::{BitDepth, ColorType, Encoder};
use semio_framework_os::{register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult};
use semio_framework_plugin::{
    build_canvas_2d_scene, build_world_3d_scene, create_default_layout, create_named_layout,
    export_mesh_glb_bytes, export_mesh_obj, merge_world_selection_ids, ui_inspector_groups_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, world3d_default_camera, world3d_scene,
    world3d_selection_json, App, Canvas2dScene, CommandDescriptor, MeshData, PluginApp, PluginBundle,
    tool_button, tool_collection, tool_separator, tool_toggle, ToolNode, UiControlNode, UiFieldNode,
    UiInspectorFieldGroup, UiNode, UiToggleNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::LazyLock;

//#region 🔖Constants
const LOWPOLY_PLAY_APP_ID: &str = "lowpoly-play";
const LOWPOLY_PLAY_CONTROLLER_ID: &str = "lowpoly-play";
const LOWPOLY_PLAY_SURFACE_MAIN: &str = "lowpoly.play";
const LOWPOLY_PLAY_SURFACE_UV: &str = "lowpoly.play.uv";
const LOWPOLY_PLAY_BODY_MAIN: &str = "lowpoly.play.main";
const LOWPOLY_PLAY_BODY_UV: &str = "lowpoly.play.uv";
const LOWPOLY_PLAY_BODY_HIERARCHY: &str = "lowpoly.play.hierarchy";
const LOWPOLY_PLAY_BODY_CATALOGUE: &str = "lowpoly.play.catalogue";
const LOWPOLY_PLAY_BODY_INSPECTION: &str = "lowpoly.play.inspection";
const LOWPOLY_PLAY_BODY_LAYERS: &str = "lowpoly.play.layers";
const LOWPOLY_PLAY_WINDOW_MAIN: &str = "lowpoly-main";
const LOWPOLY_PLAY_WINDOW_UV: &str = "lowpoly-uv";
const LOWPOLY_FIXTURE_SCHEMA: &str = "lowpoly.fixture";

const PRIMITIVE_CATALOG: &[(&str, &str, &str)] = &[
    ("box", "Box", "box"),
    ("plane", "Plane", "square"),
    ("cylinder", "Cylinder", "cylinder"),
    ("cone", "Cone", "triangle"),
    ("ico_sphere", "Ico Sphere", "globe"),
];

const TOOL_PARAM_KEYS: &[&str] = &[
    "extrudeDistance",
    "insetAmount",
    "bevelAmount",
    "bevelSegments",
    "loopCuts",
    "decimateRatio",
    "snapGrid",
    "mirrorAxis",
    "brushSize",
    "brushOpacity",
    "brushHardness",
];
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyPlayRuntime {
    #[serde(default = "default_transform_tool")]
    transform_tool: String,
    #[serde(default = "default_paint_tool")]
    paint_tool: String,
    #[serde(default)]
    active_paint_layer: u32,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    selected_object_ids: Vec<String>,
    #[serde(default)]
    hovered_object_id: Option<String>,
    #[serde(default)]
    hovered_target: Option<LowpolyHoverTarget>,
    #[serde(default = "default_tool_params")]
    tool_params: Value,
    #[serde(default = "default_paint_color")]
    paint_color: [u8; 4],
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyHoverTarget {
    #[serde(default)]
    object_id: Option<String>,
    #[serde(default)]
    mode: Option<String>,
    #[serde(default)]
    id: Option<u32>,
}

fn default_transform_tool() -> String {
    "move".into()
}

fn default_paint_tool() -> String {
    "brush".into()
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_paint_color() -> [u8; 4] {
    [255, 64, 64, 255]
}

fn default_tool_params() -> Value {
    json!({
        "extrudeDistance": 0.25,
        "insetAmount": 0.1,
        "bevelAmount": 0.05,
        "bevelSegments": 1,
        "loopCuts": 1,
        "decimateRatio": 0.5,
        "snapGrid": 0.25,
        "mirrorAxis": 0,
        "brushSize": 16,
        "brushOpacity": 1,
        "brushHardness": 0.5,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyPlayEnvelope {
    fixture: LowpolyFixture,
    #[serde(default)]
    runtime: LowpolyPlayRuntime,
}

fn default_envelope() -> LowpolyPlayEnvelope {
    LowpolyPlayEnvelope {
        fixture: default_fixture(),
        runtime: LowpolyPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> LowpolyPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &LowpolyPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn lowpoly_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: LOWPOLY_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn tool_param_f32(params: &Value, key: &str, default: f32) -> f32 {
    params.get(key).and_then(|value| value.as_f64()).map(|v| v as f32).unwrap_or(default)
}

fn tool_param_u32(params: &Value, key: &str, default: u32) -> u32 {
    params.get(key).and_then(|value| value.as_u64()).map(|v| v as u32).unwrap_or(default)
}

fn mirror_axis_from_param(params: &Value) -> MirrorAxis {
    match tool_param_u32(params, "mirrorAxis", 0) {
        1 => MirrorAxis::Y,
        2 => MirrorAxis::Z,
        _ => MirrorAxis::X,
    }
}

fn primitive_kind(kind: &str) -> &str {
    match kind {
        "sphere" | "ico" => "ico_sphere",
        other => other,
    }
}

fn euler_degrees_to_quaternion(rotation: [f32; 3]) -> [f64; 4] {
    let to_rad = std::f32::consts::PI / 180.0;
    let (sx, cx) = (rotation[0] * to_rad * 0.5).sin_cos();
    let (sy, cy) = (rotation[1] * to_rad * 0.5).sin_cos();
    let (sz, cz) = (rotation[2] * to_rad * 0.5).sin_cos();
    [
        (sx * cy * cz + cx * sy * sz) as f64,
        (cx * sy * cz - sx * cy * sz) as f64,
        (cx * cy * sz + sx * sy * cz) as f64,
        (cx * cy * cz - sx * sy * sz) as f64,
    ]
}
//#endregion 🔖Envelope

//#region 🔖DocumentSession
#[derive(Clone, Debug)]
struct PaintSnapshot {
    object_id: String,
    layer_index: usize,
    pixels: Vec<u8>,
}

fn load_doc(envelope: &LowpolyPlayEnvelope, paint_pixels: &HashMap<String, Vec<Vec<u8>>>) -> Result<LowpolyDocument, String> {
    let mut doc = LowpolyDocument::new(envelope.fixture.clone())?;
    doc.replace_fixture(envelope.fixture.clone(), paint_pixels.clone())?;
    Ok(doc)
}

fn commit_doc(doc: LowpolyDocument, mut envelope: LowpolyPlayEnvelope) -> (LowpolyPlayEnvelope, HashMap<String, Vec<Vec<u8>>>) {
    envelope.fixture = doc.fixture().clone();
    (envelope, doc.paint_pixels().clone())
}

fn active_object<'a>(fixture: &'a LowpolyFixture) -> Option<&'a LowpolyObject> {
    fixture
        .objects
        .iter()
        .find(|object| object.id == fixture.active_object_id)
        .or_else(|| fixture.objects.first())
}

fn mesh_data_from_transfer(transfer: &Value, paint_texture: Option<String>) -> MeshData {
    let read_f32 = |key: &str| -> Vec<f32> {
        transfer
            .get(key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default()
    };
    let read_u32 = |key: &str| -> Vec<u32> {
        transfer
            .get(key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default()
    };
    let read_u8 = |key: &str| -> Vec<u8> {
        transfer
            .get(key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default()
    };
    MeshData {
        positions: read_f32("positions"),
        normals: read_f32("normals"),
        indices: read_u32("indices"),
        uvs: read_f32("uvs"),
        face_ids: read_u32("faceIds"),
        vertex_ids: read_u32("vertexIds"),
        edge_positions: read_f32("edgePositions"),
        edge_ids: read_u32("edgeIds"),
        edge_uvs: read_f32("edgeUvs"),
        edge_is_seam: read_u8("edgeIsSeam"),
        paint_texture_base64: paint_texture,
        ..MeshData::default()
    }
}

fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    {
        let mut encoder = Encoder::new(&mut bytes, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|error| error.to_string())?;
        writer.write_image_data(pixels).map_err(|error| error.to_string())?;
    }
    Ok(bytes)
}

fn map_kernel_err(error: kernel_3d_mesh::MeshKernelError) -> String {
    format!("{error:?}")
}
//#endregion 🔖DocumentSession

//#region 🔖SelectionHelpers
fn merge_selection_ids(existing: &[u32], incoming: &[u32], merge: &str) -> Vec<u32> {
    match merge {
        "add" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(*id);
                }
            }
            merged
        }
        "toggle" | "invertive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(*id);
                }
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

fn hierarchy_target_row_id(object_id: &str, _object_index: usize, mode: &str, id: u32) -> String {
    format!("lowpoly-hierarchy.{object_id}.{mode}.{id}")
}

fn selection_key(object_id: &str, object_index: usize, mode: &str, id: u32) -> String {
    format!("lowpoly:{object_id}:{object_index}:{mode}:{id}")
}

fn object_index_for(fixture: &LowpolyFixture, object_id: &str) -> usize {
    fixture
        .objects
        .iter()
        .position(|object| object.id == object_id)
        .unwrap_or(0)
}

fn enable_selection_target_kind(targets: &mut LowpolySelectionTargets, mode: &str) {
    match mode {
        "vertex" => targets.vertex = true,
        "edge" => targets.edge = true,
        "face" => targets.face = true,
        _ => targets.mesh = true,
    }
}

fn sync_selection_keys(fixture: &mut LowpolyFixture) {
    let object_index = object_index_for(fixture, &fixture.active_object_id);
    fixture.selection.keys = fixture
        .selection
        .ids
        .iter()
        .map(|id| selection_key(&fixture.active_object_id, object_index, &fixture.selection.mode, *id))
        .collect();
}

fn apply_component_selection(envelope: &mut LowpolyPlayEnvelope, mode: &str, incoming: &[u32], merge: &str) {
    let normalized = LowpolyDocument::normalize_selection_mode(mode);
    enable_selection_target_kind(&mut envelope.fixture.selection.targets, &normalized);
    envelope.fixture.selection.mode = normalized;
    envelope.fixture.selection.ids = merge_selection_ids(&envelope.fixture.selection.ids, incoming, merge);
    sync_selection_keys(&mut envelope.fixture);
}

fn selected_hierarchy_ids(fixture: &LowpolyFixture) -> Vec<String> {
    let object_index = object_index_for(fixture, &fixture.active_object_id);
    fixture
        .selection
        .ids
        .iter()
        .map(|id| hierarchy_target_row_id(&fixture.active_object_id, object_index, &fixture.selection.mode, *id))
        .collect()
}

fn highlighted_hierarchy_ids(runtime: &LowpolyPlayRuntime, fixture: &LowpolyFixture) -> Vec<String> {
    runtime
        .hovered_target
        .as_ref()
        .and_then(|target| {
            let object_id = target.object_id.as_deref()?;
            let mode = target.mode.as_deref()?;
            let id = target.id?;
            Some(hierarchy_target_row_id(
                object_id,
                object_index_for(fixture, object_id),
                mode,
                id,
            ))
        })
        .into_iter()
        .collect()
}

fn gumball_target_world(doc: &LowpolyDocument, fixture: &LowpolyFixture) -> Option<[f64; 3]> {
    let pivot = doc.selection_transform_pivot().ok()?;
    let object = fixture.objects.iter().find(|entry| entry.id == fixture.active_object_id)?;
    let position = &object.transform.position;
    Some([
        position[0] as f64 + pivot.x() as f64,
        position[1] as f64 + pivot.y() as f64,
        position[2] as f64 + pivot.z() as f64,
    ])
}

fn gumball_active(envelope: &LowpolyPlayEnvelope) -> bool {
    !envelope.fixture.selection.ids.is_empty()
        || (envelope.fixture.selection.targets.mesh
            && envelope
                .runtime
                .selected_object_ids
                .iter()
                .any(|id| id == &envelope.fixture.active_object_id))
}
//#endregion 🔖SelectionHelpers

//#region 🔖Scene
fn world_selection_json_for(
    envelope: &LowpolyPlayEnvelope,
    active_mode_id: Option<&str>,
    doc: Option<&LowpolyDocument>,
) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_object_ids,
        runtime.hovered_object_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!(envelope.fixture.selection.mode));
        object.insert("targets".into(), json!(envelope.fixture.selection.targets));
        object.insert("transformTool".into(), json!(runtime.transform_tool));
        object.insert(
            "interactionMode".into(),
            json!(if active_mode_id == Some("paint") { "paint" } else { "model" }),
        );
        object.insert("componentIds".into(), json!(envelope.fixture.selection.ids));
        object.insert("selectionMode".into(), json!(envelope.fixture.selection.mode));
        object.insert("activeObjectId".into(), json!(envelope.fixture.active_object_id));
        object.insert("gumballActive".into(), json!(gumball_active(envelope)));
        if let Some(target) = runtime.hovered_target.as_ref() {
            object.insert("hoveredComponent".into(), json!(target));
        }
        if let Some(loaded) = doc {
            if let Some(target) = gumball_target_world(loaded, &envelope.fixture) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

fn world_meshes_json(doc: &LowpolyDocument, texture_cache: &HashMap<String, String>) -> String {
    let items: Vec<Value> = serde_json::from_str(&doc.tessellate_all_json().unwrap_or_else(|_| "[]".into()))
        .unwrap_or_default();
    let meshes: Vec<Value> = items
        .iter()
        .filter_map(|item| {
            let id = item.get("id")?.as_str()?;
            let tessellation = item.get("tessellation")?;
            let texture = texture_cache.get(id).cloned();
            Some(json!({
                "id": id,
                "data": mesh_data_from_transfer(tessellation, texture),
            }))
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn world_instances_json(fixture: &LowpolyFixture, runtime: &LowpolyPlayRuntime) -> String {
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .map(|object| {
            let selected = runtime.selected_object_ids.iter().any(|id| id == &object.id)
                || object.id == fixture.active_object_id;
            let hovered = runtime.hovered_object_id.as_deref() == Some(object.id.as_str())
                || runtime
                    .hovered_target
                    .as_ref()
                    .and_then(|target| target.object_id.as_deref())
                    == Some(object.id.as_str());
            let rotation = euler_degrees_to_quaternion(object.transform.rotation);
            json!({
                "id": object.id,
                "meshId": object.id,
                "position": [
                    object.transform.position[0] as f64,
                    object.transform.position[1] as f64,
                    object.transform.position[2] as f64,
                ],
                "rotation": rotation,
                "scale": [
                    object.transform.scale[0] as f64,
                    object.transform.scale[1] as f64,
                    object.transform.scale[2] as f64,
                ],
                "label": object.name,
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn uv_canvas_layers_json(doc: &LowpolyDocument, envelope: &LowpolyPlayEnvelope, texture_cache: &HashMap<String, String>) -> String {
    let object_id = envelope.fixture.active_object_id.clone();
    let mut layers = Vec::new();
    if let Some(texture) = texture_cache.get(&object_id) {
        let size = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
        layers.push(json!({
            "id": "uv-paint-texture",
            "kind": "image",
            "name": "Paint",
            "x": -size * 0.5,
            "y": -size * 0.5,
            "width": size,
            "height": size,
            "dataUrl": format!("data:image/png;base64,{texture}"),
        }));
    }
    if let Ok(mesh) = doc.active_mesh() {
        if let Ok(transfer) = LowpolyDocument::tessellate_transfer_json(mesh) {
            let edge_uvs: Vec<f32> = transfer
                .get("edgeUvs")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .unwrap_or_default();
            let edge_is_seam: Vec<u8> = transfer
                .get("edgeIsSeam")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .unwrap_or_default();
            let mut points = Vec::new();
            for chunk in edge_uvs.chunks_exact(4) {
                let u0 = chunk[0] as f64;
                let v0 = (1.0 - chunk[1]) as f64;
                let u1 = chunk[2] as f64;
                let v1 = (1.0 - chunk[3]) as f64;
                let scale = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
                points.push([u0 * scale - scale * 0.5, v0 * scale - scale * 0.5]);
                points.push([u1 * scale - scale * 0.5, v1 * scale - scale * 0.5]);
            }
            layers.push(json!({
                "id": "uv-wireframe",
                "kind": "polyline",
                "name": "UV Wireframe",
                "points": points,
                "seams": edge_is_seam,
            }));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖Scene

//#region 🔖Panels
fn tree_item_with_command(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    command: CommandDescriptor,
) -> semio_framework_plugin::UiTreeItemNode {
    tree_item(
        id,
        label,
        icon_id,
        Some(command),
        None,
        None,
        None,
        None,
        None,
    )
}

fn tree_item(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    command: Option<CommandDescriptor>,
    hover_command: Option<CommandDescriptor>,
    unhover_command: Option<CommandDescriptor>,
    actions: Option<Vec<semio_framework_plugin::UiTreeItemAction>>,
    items: Option<Vec<semio_framework_plugin::UiTreeItemNode>>,
    description: Option<String>,
) -> semio_framework_plugin::UiTreeItemNode {
    semio_framework_plugin::UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        command,
        hover_command,
        unhover_command,
        actions,
        draggable: None,
        drag_data: None,
        items,
        control: None,
        is_hidden: None,
    }
}

fn build_hierarchy_tree(envelope: &LowpolyPlayEnvelope, doc: &LowpolyDocument) -> UiNode {
    let active_id = envelope.fixture.active_object_id.clone();
    let selected_ids = selected_hierarchy_ids(&envelope.fixture);
    let highlighted_ids = highlighted_hierarchy_ids(&envelope.runtime, &envelope.fixture);
    let items: Vec<semio_framework_plugin::UiTreeItemNode> = envelope
        .fixture
        .objects
        .iter()
        .enumerate()
        .map(|(object_index, object)| {
            let object_id = object.id.clone();
            let mesh = doc.object_index(&object.id).ok().and_then(|index| doc.mesh_at(index));
            let vertex_count = mesh.as_ref().map(|entry| entry.vertex_count()).unwrap_or(0);
            let edge_count = mesh.as_ref().map(|entry| entry.edge_count()).unwrap_or(0);
            let face_count = mesh.as_ref().map(|entry| entry.face_count()).unwrap_or(0);
            let component_group = |mode: &str, label: &str, icon: &str, count: usize| {
                let leaves: Vec<semio_framework_plugin::UiTreeItemNode> = (0..count)
                    .map(|id| {
                        let row_id = hierarchy_target_row_id(&object.id, object_index, mode, id as u32);
                        let hover_args = json!({
                            "objectId": object.id,
                            "mode": mode,
                            "id": id,
                        });
                        let mut actions = None;
                        if mode == "face" {
                            actions = Some(vec![semio_framework_plugin::UiTreeItemAction {
                                icon_id: "flip-horizontal".into(),
                                label: Some("Flip normal".into()),
                                command: lowpoly_cmd(
                                    "flipFaces",
                                    Some(json!({ "faceIds": [id] })),
                                ),
                                reveal_on_hover: Some(true),
                            }]);
                        }
                        tree_item(
                            row_id,
                            format!("{label} {id}"),
                            Some(icon),
                            Some(lowpoly_cmd(
                                "toggleSelectionTarget",
                                Some(json!({
                                    "objectId": object.id,
                                    "mode": mode,
                                    "id": id,
                                    "merge": "invertive",
                                })),
                            )),
                            Some(lowpoly_cmd("setHover", Some(hover_args.clone()))),
                            Some(lowpoly_cmd("setHover", None)),
                            actions,
                            None,
                            None,
                        )
                    })
                    .collect();
                tree_item(
                    format!("lowpoly-hierarchy.{object_id}.{mode}.group"),
                    label.to_string(),
                    Some(icon),
                    None,
                    None,
                    None,
                    None,
                    Some(leaves),
                    Some(format!("{count}")),
                )
            };
            tree_item(
                format!("lowpoly-hierarchy.{object_id}"),
                object.name.clone(),
                Some("box"),
                Some(lowpoly_cmd("setActiveObject", Some(json!({ "objectId": object.id })))),
                None,
                None,
                None,
                Some(vec![
                    component_group("vertex", "Vertices", "circle", vertex_count),
                    component_group("edge", "Edges", "minus", edge_count),
                    component_group("face", "Faces", "square", face_count),
                ]),
                Some(object.id.clone()),
            )
        })
        .collect();
    UiNode::Tree(semio_framework_plugin::UiTreeNode {
        sections: vec![semio_framework_plugin::UiTreeSectionNode {
            id: "lowpoly-play-hierarchy.meshes".into(),
            label: Some("Meshes".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: if selected_ids.is_empty() { None } else { Some(selected_ids) },
        highlighted_ids: if highlighted_ids.is_empty() {
            None
        } else {
            Some(highlighted_ids)
        },
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<semio_framework_plugin::UiTreeItemNode> = PRIMITIVE_CATALOG
        .iter()
        .map(|(kind, label, icon)| {
            tree_item_with_command(
                format!("lowpoly-play-catalogue.{kind}"),
                *label,
                Some(icon),
                lowpoly_cmd("addPrimitive", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    UiNode::Tree(semio_framework_plugin::UiTreeNode {
        sections: vec![semio_framework_plugin::UiTreeSectionNode {
            id: "lowpoly-play-catalogue.primitives".into(),
            label: Some("Primitives".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_layers_tree(envelope: &LowpolyPlayEnvelope) -> UiNode {
    let object = active_object(&envelope.fixture);
    let layers = object.map(|entry| entry.paint_layers.as_slice()).unwrap_or(&[]);
    let items: Vec<semio_framework_plugin::UiTreeItemNode> = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| {
            tree_item_with_command(
                format!("lowpoly-layer:{index}"),
                layer.name.clone(),
                Some("layers"),
                lowpoly_cmd("setActivePaintLayer", Some(json!({ "layerIndex": index }))),
            )
        })
        .collect();
    UiNode::Tree(semio_framework_plugin::UiTreeNode {
        sections: vec![semio_framework_plugin::UiTreeSectionNode {
            id: "lowpoly-play-layers.paint".into(),
            label: Some("Paint Layers".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn inspector_tool_param_field(id: &str, label: &str, key: &str, value: &Value) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("lowpoly-play-inspector.{id}"),
        label: label.into(),
        child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("lowpoly-play-inspector.{id}.input"),
            input_kind: "number".into(),
            value: value
                .get(key)
                .map(|entry| entry.to_string())
                .unwrap_or_else(|| "0".into()),
            placeholder: None,
            commit: None,
            on_change: lowpoly_cmd("setToolParam", Some(json!({ "key": key }))),
        }),
    })
}

fn build_inspector_tree(envelope: &LowpolyPlayEnvelope) -> UiNode {
    let Some(object) = active_object(&envelope.fixture) else {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {LOWPOLY_FIXTURE_SCHEMA}")),
            ui_text("No active object".to_string()),
        ]);
    };
    let params = &envelope.runtime.tool_params;
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.object".into(),
            label: "Object".into(),
            default_open: None,
            fields: vec![
                UiNode::Field(UiFieldNode {
                    id: "lowpoly-play-inspector.object.name".into(),
                    label: "Name".into(),
                    child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                        id: "lowpoly-play-inspector.object.name.input".into(),
                        input_kind: "text".into(),
                        value: object.name.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: lowpoly_cmd(
                            "patchObject",
                            Some(json!({ "objectId": object.id, "field": "name" })),
                        ),
                    }),
                }),
                UiNode::Field(UiFieldNode {
                    id: "lowpoly-play-inspector.object.smooth".into(),
                    label: "Smooth Shading".into(),
                    child: UiControlNode::Toggle(UiToggleNode {
                        id: "lowpoly-play-inspector.object.smooth.toggle".into(),
                        icon_id: "sun".into(),
                        pressed: object.smooth_shading,
                        text: None,
                        on_change: lowpoly_cmd(
                            "patchObject",
                            Some(json!({ "objectId": object.id, "field": "smoothShading" })),
                        ),
                    }),
                }),
                ui_inspector_readonly_field(
                    "lowpoly-play-inspector.object.selection",
                    "Selection Mode",
                    &envelope.fixture.selection.mode,
                ),
            ],
        },
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.transform".into(),
            label: "Transform".into(),
            default_open: None,
            fields: vec![ui_inspector_readonly_field(
                "lowpoly-play-inspector.transform.tool",
                "Tool",
                &envelope.runtime.transform_tool,
            )],
        },
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.tool-params".into(),
            label: "Tool Params".into(),
            default_open: Some(true),
            fields: vec![
                inspector_tool_param_field("extrude", "Extrude Distance", "extrudeDistance", params),
                inspector_tool_param_field("inset", "Inset Amount", "insetAmount", params),
                inspector_tool_param_field("bevel", "Bevel Amount", "bevelAmount", params),
                inspector_tool_param_field("bevel-segments", "Bevel Segments", "bevelSegments", params),
                inspector_tool_param_field("loop-cuts", "Loop Cuts", "loopCuts", params),
                inspector_tool_param_field("decimate", "Decimate Ratio", "decimateRatio", params),
                inspector_tool_param_field("snap", "Snap Grid", "snapGrid", params),
                inspector_tool_param_field("mirror", "Mirror Axis", "mirrorAxis", params),
                inspector_tool_param_field("brush-size", "Brush Size", "brushSize", params),
                inspector_tool_param_field("brush-opacity", "Brush Opacity", "brushOpacity", params),
                inspector_tool_param_field("brush-hardness", "Brush Hardness", "brushHardness", params),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Tools
fn edit_tools(envelope: &LowpolyPlayEnvelope) -> Vec<ToolNode> {
    let targets = &envelope.fixture.selection.targets;
    let transform = &envelope.runtime.transform_tool;
    vec![
        tool_collection(
            "lowpoly-tools-selection",
            "mouse-pointer",
            vec![
                tool_toggle(
                    "lowpoly-tools-selection-mesh",
                    "box",
                    targets.mesh,
                    lowpoly_cmd("toggleSelectionKind", Some(json!({ "kind": "mesh" }))),
                ),
                tool_toggle(
                    "lowpoly-tools-selection-vertex",
                    "circle",
                    targets.vertex,
                    lowpoly_cmd("toggleSelectionKind", Some(json!({ "kind": "vertex" }))),
                ),
                tool_toggle(
                    "lowpoly-tools-selection-edge",
                    "minus",
                    targets.edge,
                    lowpoly_cmd("toggleSelectionKind", Some(json!({ "kind": "edge" }))),
                ),
                tool_toggle(
                    "lowpoly-tools-selection-face",
                    "square",
                    targets.face,
                    lowpoly_cmd("toggleSelectionKind", Some(json!({ "kind": "face" }))),
                ),
            ],
        ),
        tool_separator("lowpoly-tools-sep-transform"),
        tool_collection(
            "lowpoly-tools-transform",
            "move",
            vec![
                tool_toggle(
                    "lowpoly-tools-transform-move",
                    "move",
                    transform == "move",
                    lowpoly_cmd("setTransformTool", Some(json!({ "tool": "move" }))),
                ),
                tool_toggle(
                    "lowpoly-tools-transform-rotate",
                    "rotate-cw",
                    transform == "rotate",
                    lowpoly_cmd("setTransformTool", Some(json!({ "tool": "rotate" }))),
                ),
                tool_toggle(
                    "lowpoly-tools-transform-scale",
                    "maximize",
                    transform == "scale",
                    lowpoly_cmd("setTransformTool", Some(json!({ "tool": "scale" }))),
                ),
            ],
        ),
        tool_separator("lowpoly-tools-sep-edit"),
        tool_button("lowpoly-tools-extrude", "arrow-up-from-line", lowpoly_cmd("extrude", None)),
        tool_button("lowpoly-tools-inset", "shrink", lowpoly_cmd("inset", None)),
        tool_button("lowpoly-tools-flip", "flip-horizontal", lowpoly_cmd("flipFaces", None)),
        tool_button("lowpoly-tools-bevel", "git-commit-horizontal", lowpoly_cmd("bevel", None)),
        tool_button("lowpoly-tools-loop-cut", "grip-lines", lowpoly_cmd("loopCut", None)),
        tool_button("lowpoly-tools-merge", "merge", lowpoly_cmd("merge", None)),
        tool_button("lowpoly-tools-dissolve", "eraser", lowpoly_cmd("dissolve", None)),
        tool_button("lowpoly-tools-subdivide", "grid-2x2", lowpoly_cmd("subdivide", None)),
        tool_button("lowpoly-tools-triangulate", "triangle", lowpoly_cmd("triangulate", None)),
        tool_button("lowpoly-tools-mirror", "flip-vertical", lowpoly_cmd("mirror", None)),
        tool_button("lowpoly-tools-decimate", "filter", lowpoly_cmd("decimate", None)),
    ]
}

fn paint_tools(envelope: &LowpolyPlayEnvelope) -> Vec<ToolNode> {
    let paint_tool = &envelope.runtime.paint_tool;
    vec![
        tool_collection(
            "lowpoly-paint-tools",
            "paintbrush",
            vec![
                tool_toggle(
                    "lowpoly-paint-brush",
                    "paintbrush",
                    paint_tool == "brush",
                    lowpoly_cmd("setPaintTool", Some(json!({ "tool": "brush" }))),
                ),
                tool_toggle(
                    "lowpoly-paint-eraser",
                    "eraser",
                    paint_tool == "eraser",
                    lowpoly_cmd("setPaintTool", Some(json!({ "tool": "eraser" }))),
                ),
                tool_toggle(
                    "lowpoly-paint-fill",
                    "paint-bucket",
                    paint_tool == "fill",
                    lowpoly_cmd("setPaintTool", Some(json!({ "tool": "fill" }))),
                ),
                tool_toggle(
                    "lowpoly-paint-eyedropper",
                    "pipette",
                    paint_tool == "eyedropper",
                    lowpoly_cmd("setPaintTool", Some(json!({ "tool": "eyedropper" }))),
                ),
            ],
        ),
        tool_separator("lowpoly-paint-sep-uv"),
        tool_button("lowpoly-paint-unwrap", "unfold", lowpoly_cmd("unwrapActive", None)),
        tool_button("lowpoly-paint-mark-seam", "scissors", lowpoly_cmd("markUvSeam", Some(json!({ "seam": true })))),
        tool_button("lowpoly-paint-clear-seam", "eraser", lowpoly_cmd("clearSeam", None)),
        tool_separator("lowpoly-paint-sep-history"),
        tool_button("lowpoly-paint-undo", "undo", lowpoly_cmd("paintUndo", None)),
        tool_button("lowpoly-paint-redo", "redo", lowpoly_cmd("paintRedo", None)),
    ]
}
//#endregion 🔖Tools

//#region 🔖LowpolyPlayApp
struct LowpolyPlayApp {
    paint_pixels: HashMap<String, Vec<Vec<u8>>>,
    paint_undo: Vec<PaintSnapshot>,
    paint_redo: Vec<PaintSnapshot>,
    paint_texture_cache: RefCell<HashMap<String, String>>,
}

impl Default for LowpolyPlayApp {
    fn default() -> Self {
        Self {
            paint_pixels: HashMap::new(),
            paint_undo: Vec::new(),
            paint_redo: Vec::new(),
            paint_texture_cache: RefCell::new(HashMap::new()),
        }
    }
}

impl LowpolyPlayApp {
    fn refresh_paint_texture_cache(&self, doc: &LowpolyDocument) {
        let mut cache = self.paint_texture_cache.borrow_mut();
        for object in &doc.fixture().objects {
            if let Ok(pixels) = doc.composite_layers(&object.id) {
                if let Ok(png_bytes) = encode_rgba_png(
                    &pixels,
                    LOWPOLY_PAINT_TEXTURE_SIZE as u32,
                    LOWPOLY_PAINT_TEXTURE_SIZE as u32,
                ) {
                    cache.insert(
                        object.id.clone(),
                        base64::engine::general_purpose::STANDARD.encode(png_bytes),
                    );
                }
            }
        }
    }

    fn push_paint_undo(&mut self, object_id: &str, layer_index: usize, pixels: &[u8]) {
        self.paint_undo.push(PaintSnapshot {
            object_id: object_id.into(),
            layer_index,
            pixels: pixels.to_vec(),
        });
        self.paint_redo.clear();
    }

    fn restore_paint_snapshot(&mut self, snapshot: PaintSnapshot) -> Result<(), String> {
        let mut doc = LowpolyDocument::new(default_fixture())?;
        doc.paint_pixels_mut().insert(snapshot.object_id.clone(), vec![snapshot.pixels.clone()]);
        self.paint_pixels = doc.paint_pixels().clone();
        Ok(())
    }

    fn mutate_mesh(
        &mut self,
        envelope: LowpolyPlayEnvelope,
        edit: impl FnOnce(&mut LowpolyDocument, &LowpolyPlayRuntime) -> Result<(), String>,
    ) -> Vec<String> {
        let runtime = envelope.runtime.clone();
        let mut doc = match load_doc(&envelope, &self.paint_pixels) {
            Ok(doc) => doc,
            Err(_) => return Vec::new(),
        };
        if edit(&mut doc, &runtime).is_err() {
            return Vec::new();
        }
        let (next_envelope, pixels) = commit_doc(doc, envelope);
        self.paint_pixels = pixels;
        vec![set_document_op(&next_envelope)]
    }

    fn apply_translate(&mut self, envelope: LowpolyPlayEnvelope, args: Option<&Value>) -> Vec<String> {
        let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
        let ids: Vec<u32> = args
            .and_then(|value| value.get("ids"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default();
        let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
        let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
        let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
        self.mutate_mesh(envelope, move |doc, _| {
            if !ids.is_empty() {
                doc.apply_selection(mode, ids);
            }
            let selection_mode = doc.fixture().selection.mode.clone();
            let delta = Vec3::new(dx, dy, dz);
            let component_verts = match selection_mode.as_str() {
                "vertex" | "face" | "edge" => Some(doc.selection_vertex_ids()?),
                _ => None,
            };
            let mesh = doc.active_mesh_mut()?;
            match selection_mode.as_str() {
                "vertex" | "face" | "edge" => {
                    let verts = component_verts.as_ref().ok_or_else(|| "no vertices".to_string())?;
                    if verts.is_empty() {
                        return Err("no component vertices in selection".into());
                    }
                    mesh.move_vertices(verts, delta).map_err(map_kernel_err)?;
                }
                "mesh" | "object" => mesh.translate(delta).map_err(map_kernel_err)?,
                _ => return Err("unsupported selection mode for translate".into()),
            }
            doc.sync_meshes_to_fixture()
        })
    }

    fn apply_rotate(&mut self, envelope: LowpolyPlayEnvelope, args: Option<&Value>) -> Vec<String> {
        let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
        let ids: Vec<u32> = args
            .and_then(|value| value.get("ids"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default();
        let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
        let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
        let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
        let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
        self.mutate_mesh(envelope, move |doc, _| {
            if !ids.is_empty() {
                doc.apply_selection(mode, ids);
            }
            let selection_mode = doc.fixture().selection.mode.clone();
            let pivot = doc.selection_transform_pivot()?;
            let component_verts = match selection_mode.as_str() {
                "vertex" | "face" | "edge" => Some(doc.selection_vertex_ids()?),
                _ => None,
            };
            let mesh = doc.active_mesh_mut()?;
            let axis = Vec3::new(ax, ay, az);
            match selection_mode.as_str() {
                "vertex" | "face" | "edge" => {
                    let verts = component_verts.as_ref().ok_or_else(|| "no vertices".to_string())?;
                    if verts.is_empty() {
                        return Err("no component vertices in selection".into());
                    }
                    mesh.rotate_vertices(verts, axis, angle, pivot).map_err(map_kernel_err)?;
                }
                "mesh" | "object" => mesh.rotate(axis, angle).map_err(map_kernel_err)?,
                _ => return Err("unsupported selection mode for rotate".into()),
            }
            doc.sync_meshes_to_fixture()
        })
    }

    fn apply_scale(&mut self, envelope: LowpolyPlayEnvelope, args: Option<&Value>) -> Vec<String> {
        let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
        let ids: Vec<u32> = args
            .and_then(|value| value.get("ids"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default();
        let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0) as f32;
        let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0) as f32;
        let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0) as f32;
        self.mutate_mesh(envelope, move |doc, _| {
            if !ids.is_empty() {
                doc.apply_selection(mode, ids);
            }
            let selection_mode = doc.fixture().selection.mode.clone();
            let pivot = doc.selection_transform_pivot()?;
            let component_verts = match selection_mode.as_str() {
                "vertex" | "face" | "edge" => Some(doc.selection_vertex_ids()?),
                _ => None,
            };
            let mesh = doc.active_mesh_mut()?;
            let scale = Vec3::new(sx, sy, sz);
            match selection_mode.as_str() {
                "vertex" | "face" | "edge" => {
                    let verts = component_verts.as_ref().ok_or_else(|| "no vertices".to_string())?;
                    if verts.is_empty() {
                        return Err("no component vertices in selection".into());
                    }
                    mesh.scale_vertices(verts, scale, pivot).map_err(map_kernel_err)?;
                }
                "mesh" | "object" => mesh.scale(scale).map_err(map_kernel_err)?,
                _ => return Err("unsupported selection mode for scale".into()),
            }
            doc.sync_meshes_to_fixture()
        })
    }

    fn paint_at_uv(&mut self, envelope: LowpolyPlayEnvelope, u: f32, v: f32) -> Vec<String> {
        let object_id = envelope.fixture.active_object_id.clone();
        let layer_index = envelope.runtime.active_paint_layer as usize;
        let params = envelope.runtime.tool_params.clone();
        let color = envelope.runtime.paint_color;
        let paint_tool = envelope.runtime.paint_tool.clone();
        let mut doc = match load_doc(&envelope, &self.paint_pixels) {
            Ok(doc) => doc,
            Err(_) => return Vec::new(),
        };
        if let Ok(before) = doc.layer_pixels(&object_id, layer_index).map(|pixels| pixels.to_vec()) {
            self.push_paint_undo(&object_id, layer_index, &before);
        }
        let radius = tool_param_f32(&params, "brushSize", 16.0);
        let opacity = tool_param_f32(&params, "brushOpacity", 1.0);
        let hardness = tool_param_f32(&params, "brushHardness", 0.5);
        let result = match paint_tool.as_str() {
            "eraser" => doc.paint_stroke(&object_id, layer_index, u, v, radius, color, hardness, opacity, true),
            "fill" => doc.fill_bucket(&object_id, layer_index, u, v, color),
            "eyedropper" => {
                if let Ok(sample) = doc.sample_pixel(&object_id, u, v) {
                    let mut next = envelope;
                    next.runtime.paint_color = sample;
                    return vec![set_document_op(&next)];
                }
                Ok(())
            }
            _ => doc.paint_stroke(&object_id, layer_index, u, v, radius, color, hardness, opacity, false),
        };
        if result.is_err() {
            return Vec::new();
        }
        self.paint_pixels = doc.paint_pixels().clone();
        self.refresh_paint_texture_cache(&doc);
        let (next_envelope, pixels) = commit_doc(doc, envelope);
        self.paint_pixels = pixels;
        vec![set_document_op(&next_envelope)]
    }
}

impl PluginApp for LowpolyPlayApp {
    fn app_id(&self) -> &str {
        LOWPOLY_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("lowpoly envelope json")
    }

    fn tools(&self, document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        let envelope = parse_envelope(document_json);
        match _view_state.active_mode_id.as_deref() {
            Some("paint") => paint_tools(&envelope),
            _ => edit_tools(&envelope),
        }
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
                    if let Ok(parsed) = serde_json::from_value::<LowpolyPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(fixture) = serde_json::from_str::<LowpolyFixture>(json_text) {
                        envelope.fixture = fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveObject" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if envelope.fixture.objects.iter().any(|object| object.id == object_id) {
                    envelope.fixture.active_object_id = object_id.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelection" => {
                let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
                let ids: Vec<u32> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.fixture.selection.mode = LowpolyDocument::normalize_selection_mode(mode);
                envelope.fixture.selection.ids = ids;
                sync_selection_keys(&mut envelope.fixture);
                return vec![set_document_op(&envelope)];
            }
            "toggleSelectionKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("mesh");
                match kind {
                    "vertex" => envelope.fixture.selection.targets.vertex = !envelope.fixture.selection.targets.vertex,
                    "edge" => envelope.fixture.selection.targets.edge = !envelope.fixture.selection.targets.edge,
                    "face" => envelope.fixture.selection.targets.face = !envelope.fixture.selection.targets.face,
                    _ => envelope.fixture.selection.targets.mesh = !envelope.fixture.selection.targets.mesh,
                }
                return vec![set_document_op(&envelope)];
            }
            "setTransformTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.runtime.transform_tool = tool.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setPaintTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.runtime.paint_tool = tool.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setActivePaintLayer" => {
                let layer_index = args
                    .and_then(|value| value.get("layerIndex"))
                    .and_then(|value| value.as_u64())
                    .unwrap_or(0) as u32;
                envelope.runtime.active_paint_layer = layer_index;
                return vec![set_document_op(&envelope)];
            }
            "addPaintLayer" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| envelope.fixture.active_object_id.clone());
                let name = args
                    .and_then(|value| value.get("name"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("Layer");
                return self.mutate_mesh(envelope, move |doc, _| {
                    let idx = doc.object_index(&object_id)?;
                    doc.fixture_mut().objects[idx]
                        .paint_layers
                        .push(lowpoly_core::LowpolyPaintLayer::new(name));
                    let layer_index = doc.fixture().objects[idx].paint_layers.len();
                    doc.ensure_object_paint_buffers(&object_id, layer_index);
                    Ok(())
                });
            }
            "setToolParam" => {
                let key = args.and_then(|value| value.get("key")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if let Some(map) = envelope.runtime.tool_params.as_object_mut() {
                    map.insert(key.into(), value);
                    return vec![set_document_op(&envelope)];
                }
                let mut map = Map::new();
                map.insert(key.into(), value);
                envelope.runtime.tool_params = Value::Object(map);
                return vec![set_document_op(&envelope)];
            }
            "patchObject" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned();
                for object in &mut envelope.fixture.objects {
                    if object.id != object_id {
                        continue;
                    }
                    match field {
                        "name" => {
                            if let Some(name) = value.as_ref().and_then(|entry| entry.as_str()) {
                                object.name = name.into();
                            }
                        }
                        "smoothShading" => {
                            object.smooth_shading =
                                value.as_ref().and_then(|entry| entry.as_bool()).unwrap_or(!object.smooth_shading);
                        }
                        _ => {}
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "addPrimitive" => {
                let kind = primitive_kind(args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("box"));
                return self.mutate_mesh(envelope, move |doc, _| doc.add_primitive(kind).map(|_| ()));
            }
            "extrude" => {
                let distance = tool_param_f32(&envelope.runtime.tool_params, "extrudeDistance", 0.25);
                return self.mutate_mesh(envelope, move |doc, _| {
                    let faces = doc.selected_face_ids();
                    if faces.is_empty() {
                        return Err("no faces selected".into());
                    }
                    doc.active_mesh_mut()?
                        .extrude_faces(&faces, distance)
                        .map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "inset" => {
                let amount = tool_param_f32(&envelope.runtime.tool_params, "insetAmount", 0.1);
                return self.mutate_mesh(envelope, move |doc, _| {
                    let faces = doc.selected_face_ids();
                    doc.active_mesh_mut()?.inset_faces(&faces, amount).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "bevel" => {
                let amount = tool_param_f32(&envelope.runtime.tool_params, "bevelAmount", 0.05);
                let segments = tool_param_u32(&envelope.runtime.tool_params, "bevelSegments", 1);
                return self.mutate_mesh(envelope, move |doc, _| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut()?
                        .bevel_edges(&edges, amount, segments)
                        .map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "loopCut" => {
                let cuts = tool_param_u32(&envelope.runtime.tool_params, "loopCuts", 1);
                return self.mutate_mesh(envelope, move |doc, _| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut()?.loop_cut(&edges, cuts).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "subdivide" => {
                return self.mutate_mesh(envelope, move |doc, _| {
                    let faces = doc.selected_face_ids();
                    doc.active_mesh_mut()?.subdivide_faces(&faces).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "triangulate" => {
                return self.mutate_mesh(envelope, move |doc, _| {
                    doc.active_mesh_mut()?.triangulate().map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "mirror" => {
                let axis = mirror_axis_from_param(&envelope.runtime.tool_params);
                return self.mutate_mesh(envelope, move |doc, _| {
                    doc.active_mesh_mut()?.mirror(axis, 0.001).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "decimate" => {
                let ratio = tool_param_f32(&envelope.runtime.tool_params, "decimateRatio", 0.5);
                return self.mutate_mesh(envelope, move |doc, _| {
                    doc.active_mesh_mut()?.decimate(ratio).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "flipFaces" => {
                let face_ids: Vec<u32> = args
                    .and_then(|value| value.get("faceIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                return self.mutate_mesh(envelope, move |doc, _| {
                    let faces = if !face_ids.is_empty() {
                        face_ids.into_iter().map(FaceId).collect()
                    } else if !doc.selected_face_ids().is_empty() {
                        doc.selected_face_ids()
                    } else {
                        doc.fixture().selection.ids.iter().map(|id| FaceId(*id)).collect()
                    };
                    doc.active_mesh_mut()?.flip_faces(&faces).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "merge" => {
                return self.mutate_mesh(envelope, move |doc, _| {
                    let verts = doc.selected_vertex_ids();
                    doc.active_mesh_mut()?
                        .merge_vertices(&verts, WeldMode::Center, 0.001)
                        .map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "dissolve" => {
                return self.mutate_mesh(envelope, move |doc, _| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut()?.dissolve_edges(&edges).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "snap" => {
                let grid = tool_param_f32(&envelope.runtime.tool_params, "snapGrid", 0.25);
                return self.mutate_mesh(envelope, move |doc, _| {
                    let verts = doc.selected_vertex_ids();
                    doc.active_mesh_mut()?
                        .snap_vertices_to_grid(&verts, grid)
                        .map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "toggleSmooth" => {
                return self.mutate_mesh(envelope, move |doc, _| {
                    if let Some(index) = doc.active_index() {
                        let smooth = !doc.fixture().objects[index].smooth_shading;
                        doc.fixture_mut().objects[index].smooth_shading = smooth;
                        let faces: Vec<FaceId> = (0..doc.active_mesh()?.face_count())
                            .map(|index| FaceId(index as u32))
                            .collect();
                        let mesh = doc.active_mesh_mut()?;
                        mesh.set_shading(&faces, smooth).map_err(map_kernel_err)?;
                        mesh.recompute_normals().map_err(map_kernel_err)?;
                    }
                    doc.sync_meshes_to_fixture()
                });
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_object_ids =
                    merge_world_selection_ids(&envelope.runtime.selected_object_ids, &ids, merge);
                if let Some(first) = envelope.runtime.selected_object_ids.first() {
                    envelope.fixture.active_object_id = first.clone();
                }
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_object_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                envelope.runtime.hovered_target = envelope.runtime.hovered_object_id.as_ref().map(|object_id| {
                    LowpolyHoverTarget {
                        object_id: Some(object_id.clone()),
                        mode: Some("mesh".into()),
                        id: Some(0),
                    }
                });
                return vec![set_document_op(&envelope)];
            }
            "setHover" => {
                if args.is_none() || args.and_then(|value| value.get("objectId")).is_none() {
                    envelope.runtime.hovered_target = None;
                    envelope.runtime.hovered_object_id = None;
                } else {
                    let object_id = args
                        .and_then(|value| value.get("objectId"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                    let mode = args
                        .and_then(|value| value.get("mode"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).map(|value| value as u32);
                    envelope.runtime.hovered_object_id = object_id.clone();
                    envelope.runtime.hovered_target = Some(LowpolyHoverTarget {
                        object_id,
                        mode,
                        id,
                    });
                }
                return vec![set_document_op(&envelope)];
            }
            "toggleSelectionTarget" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                let merge = args
                    .and_then(|value| value.get("merge"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("invertive");
                if envelope.fixture.objects.iter().any(|object| object.id == object_id) {
                    envelope.fixture.active_object_id = object_id.into();
                    apply_component_selection(&mut envelope, mode, &[id], merge);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelectionMethod" => {
                let method = args
                    .and_then(|value| value.get("method"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                return vec![set_document_op(&envelope)];
            }
            "worldPick" => {
                let granularity = args
                    .and_then(|value| value.get("granularity"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("mesh");
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                let merge = args
                    .and_then(|value| value.get("merge"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("replace");
                apply_component_selection(&mut envelope, granularity, &[id], merge);
                return vec![set_document_op(&envelope)];
            }
            "translateSelection" => return self.apply_translate(envelope, args),
            "rotateSelection" => return self.apply_rotate(envelope, args),
            "scaleSelection" => return self.apply_scale(envelope, args),
            "paintStroke" => {
                let u = args.and_then(|value| value.get("u")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
                let v = args.and_then(|value| value.get("v")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
                return self.paint_at_uv(envelope, u, v);
            }
            "paintFill" | "fillBucket" => {
                let u = args.and_then(|value| value.get("u")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
                let v = args.and_then(|value| value.get("v")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
                let mut next = envelope;
                next.runtime.paint_tool = "fill".into();
                return self.paint_at_uv(next, u, v);
            }
            "paintSample" => {
                let u = args.and_then(|value| value.get("u")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
                let v = args.and_then(|value| value.get("v")).and_then(|value| value.as_f64()).unwrap_or(0.0) as f32;
                let mut next = envelope;
                next.runtime.paint_tool = "eyedropper".into();
                return self.paint_at_uv(next, u, v);
            }
            "paintAt" | "canvasPointerDown" => {
                let u = args.and_then(|value| value.get("u")).and_then(|value| value.as_f64());
                let v = args.and_then(|value| value.get("v")).and_then(|value| value.as_f64());
                if let (Some(u), Some(v)) = (u, v) {
                    return self.paint_at_uv(envelope, u as f32, v as f32);
                }
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let size = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
                let u = ((x / size) + 0.5).clamp(0.0, 1.0) as f32;
                let v = (1.0 - ((y / size) + 0.5).clamp(0.0, 1.0)) as f32;
                return self.paint_at_uv(envelope, u, v);
            }
            "unwrapActive" => {
                return self.mutate_mesh(envelope, move |doc, _| {
                    doc.active_mesh_mut()?.unwrap_uv().map_err(map_kernel_err)?;
                    doc.sync_meshes_to_fixture()
                });
            }
            "markUvSeam" => {
                let seam = args.and_then(|value| value.get("seam")).and_then(|value| value.as_bool()).unwrap_or(true);
                let edge_ids: Vec<u32> = args
                    .and_then(|value| value.get("edgeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_else(|| envelope.fixture.selection.ids.clone());
                return self.mutate_mesh(envelope, move |doc, _| {
                    let edges: Vec<EdgeId> = edge_ids.into_iter().map(EdgeId).collect();
                    doc.active_mesh_mut()?.mark_uv_seam(&edges, seam);
                    doc.sync_meshes_to_fixture()
                });
            }
            "clearSeam" => {
                return self.handle_command("markUvSeam", Some(&json!({ "seam": false })), document_json, _view_state);
            }
            "paintUndo" => {
                if let Some(snapshot) = self.paint_undo.pop() {
                    if let Ok(current) = load_doc(&envelope, &self.paint_pixels)
                        .and_then(|doc| doc.layer_pixels(&snapshot.object_id, snapshot.layer_index).map(|pixels| pixels.to_vec()))
                    {
                        self.paint_redo.push(PaintSnapshot {
                            object_id: snapshot.object_id.clone(),
                            layer_index: snapshot.layer_index,
                            pixels: current,
                        });
                    }
                    let _ = self.restore_paint_snapshot(snapshot);
                    return vec![set_document_op(&envelope)];
                }
            }
            "paintRedo" => {
                if let Some(snapshot) = self.paint_redo.pop() {
                    if let Ok(current) = load_doc(&envelope, &self.paint_pixels)
                        .and_then(|doc| doc.layer_pixels(&snapshot.object_id, snapshot.layer_index).map(|pixels| pixels.to_vec()))
                    {
                        self.paint_undo.push(PaintSnapshot {
                            object_id: snapshot.object_id.clone(),
                            layer_index: snapshot.layer_index,
                            pixels: current,
                        });
                    }
                    let _ = self.restore_paint_snapshot(snapshot);
                    return vec![set_document_op(&envelope)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let doc = load_doc(&envelope, &self.paint_pixels).ok();
        let texture_cache = self.paint_texture_cache.borrow().clone();
        if doc.is_some() && texture_cache.is_empty() {
            if let Some(ref loaded) = doc {
                self.refresh_paint_texture_cache(loaded);
            }
        }
        let texture_cache = self.paint_texture_cache.borrow().clone();
        match body_key {
            LOWPOLY_PLAY_BODY_MAIN => {
                if let Some(ref loaded) = doc {
                    build_world_3d_scene(
                        LOWPOLY_PLAY_SURFACE_MAIN,
                        LOWPOLY_PLAY_APP_ID,
                        world3d_scene(
                            world3d_default_camera(),
                            world_meshes_json(loaded, &texture_cache),
                            world_instances_json(&envelope.fixture, &envelope.runtime),
                            world_selection_json_for(&envelope, _view_state.active_mode_id.as_deref(), Some(loaded)),
                        ),
                    )
                } else {
                    ui_text("Failed to load lowpoly document")
                }
            }
            LOWPOLY_PLAY_BODY_UV => {
                if let Some(ref loaded) = doc {
                    build_canvas_2d_scene(
                        LOWPOLY_PLAY_SURFACE_UV,
                        LOWPOLY_PLAY_APP_ID,
                        Canvas2dScene {
                            camera_x: 0.0,
                            camera_y: 0.0,
                            zoom: 1.0,
                            layers_json: uv_canvas_layers_json(loaded, &envelope, &texture_cache),
                        },
                    )
                } else {
                    ui_text("Failed to load UV canvas")
                }
            }
            LOWPOLY_PLAY_BODY_HIERARCHY => {
                if let Some(ref loaded) = doc {
                    build_hierarchy_tree(&envelope, loaded)
                } else {
                    ui_text("Failed to load lowpoly hierarchy")
                }
            }
            LOWPOLY_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            LOWPOLY_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope),
            LOWPOLY_PLAY_BODY_LAYERS => build_layers_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖LowpolyPlayApp

//#region 🔖Manifest
fn create_lowpoly_app() -> App {
    let default_example = serde_json::to_string(&default_envelope()).expect("lowpoly default example");
    App::from_builder(
        App::builder(LOWPOLY_PLAY_APP_ID, "Lowpoly")
            .icon_id("box")
            .mode("edit", "Edit")
            .mode("paint", "Paint")
            .default_mode_id("edit")
            .window_kind(LOWPOLY_PLAY_WINDOW_MAIN, "Model", LOWPOLY_PLAY_BODY_MAIN)
            .window_kind(LOWPOLY_PLAY_WINDOW_UV, "UV", LOWPOLY_PLAY_BODY_UV)
            .default_layout(create_default_layout(
                &[LOWPOLY_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Model".into()]),
            ))
            .named_layout(create_named_layout(
                "lowpoly-paint",
                "Paint",
                create_default_layout(
                    &[LOWPOLY_PLAY_WINDOW_MAIN.into(), LOWPOLY_PLAY_WINDOW_UV.into()],
                    "row",
                    Some(&[60.0, 40.0]),
                    Some(&["Model".into(), "UV".into()]),
                ),
                "builtin",
                Some("paintbrush".into()),
                None,
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                LOWPOLY_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                LOWPOLY_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                LOWPOLY_PLAY_BODY_INSPECTION,
            )
            .panel_tab("framework.panel.layers", "Layers", "workbench", LOWPOLY_PLAY_BODY_LAYERS)
            .mode_tools("edit", edit_tools(&default_envelope()))
            .mode_tools("paint", paint_tools(&default_envelope())),
    )
    .example("default", "Default", &default_example)
    .program("lowpoly", "Lowpoly", "mesh")
}

fn bundle() -> PluginBundle {
    register_lowpoly_exports();
    PluginBundle::new("lowpoly", "Lowpoly", "0.1.0").register_app(create_lowpoly_app(), || Box::new(LowpolyPlayApp::default()))
}

fn register_lowpoly_exports() {
    register_os_media_export_handler("3d.lowpoly", OsMediaExportFormat::Obj, |doc| {
        let envelope: LowpolyPlayEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let loaded = LowpolyDocument::new(envelope.fixture).map_err(|err| err.to_string())?;
        let mesh = loaded
            .active_mesh()
            .ok()
            .and_then(|mesh| LowpolyDocument::tessellate_transfer_json(mesh).ok())
            .map(|transfer| mesh_data_from_transfer(&transfer, None))
            .unwrap_or_default();
        let (data, mime_type) = export_mesh_obj(&mesh, "lowpoly");
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: "lowpoly.obj".into(),
        })
    });
    register_os_media_export_handler("3d.lowpoly", OsMediaExportFormat::Glb, |doc| {
        let envelope: LowpolyPlayEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let loaded = LowpolyDocument::new(envelope.fixture).map_err(|err| err.to_string())?;
        let mesh = loaded
            .active_mesh()
            .ok()
            .and_then(|mesh| LowpolyDocument::tessellate_transfer_json(mesh).ok())
            .map(|transfer| mesh_data_from_transfer(&transfer, None))
            .unwrap_or_default();
        let (bytes, mime_type) = export_mesh_glb_bytes(&mesh);
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type,
            file_name: "lowpoly.glb".into(),
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
    fn default_fixture_has_rock_object() {
        let app = LowpolyPlayApp::default();
        let envelope: LowpolyPlayEnvelope = serde_json::from_str(&app.initial_document_json()).unwrap();
        assert_eq!(envelope.fixture.objects.len(), 1);
        assert_eq!(envelope.fixture.objects[0].name, "Rock");
        let doc = LowpolyDocument::new(envelope.fixture).unwrap();
        assert!(doc.active_mesh().unwrap().face_count() > 0);
    }

    #[test]
    fn renders_world_scene() {
        let app = LowpolyPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(LOWPOLY_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn renders_uv_canvas() {
        let app = LowpolyPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(LOWPOLY_PLAY_BODY_UV, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn catalogue_lists_primitives() {
        let app = LowpolyPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(LOWPOLY_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("lowpoly-play-catalogue.box"));
        assert!(json.contains("Plane"));
        assert!(json.contains("Cone"));
        assert!(json.contains("Ico Sphere"));
    }

    #[test]
    fn add_primitive_creates_object() {
        let mut app = LowpolyPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command("addPrimitive", Some(&json!({ "kind": "box" })), &document, &ViewState::default());
        let envelope: LowpolyPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.fixture.objects.len(), 2);
        assert!(envelope.fixture.objects.iter().any(|object| object.name == "box"));
    }

    #[test]
    fn extrude_selected_face() {
        let mut app = LowpolyPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.fixture.selection = LowpolySelection {
            targets: LowpolySelectionTargets {
                mesh: false,
                vertex: false,
                edge: false,
                face: true,
            },
            keys: vec!["lowpoly:obj-1:0:face:0".into()],
            mode: "face".into(),
            ids: vec![0],
        };
        let before_faces = LowpolyDocument::new(envelope.fixture.clone())
            .unwrap()
            .active_mesh()
            .unwrap()
            .face_count();
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command("extrude", None, &document, &ViewState::default());
        let next: LowpolyPlayEnvelope = apply_ops(&envelope, &ops);
        let after_faces = LowpolyDocument::new(next.fixture).unwrap().active_mesh().unwrap().face_count();
        assert!(after_faces > before_faces);
    }

    #[test]
    fn set_active_object_switches_selection() {
        let mut app = LowpolyPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command("addPrimitive", Some(&json!({ "kind": "plane" })), &document, &ViewState::default());
        let mut envelope: LowpolyPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        let second_id = envelope.fixture.active_object_id.clone();
        let rock_id = envelope
            .fixture
            .objects
            .iter()
            .find(|object| object.name == "Rock")
            .map(|object| object.id.clone())
            .expect("rock object");
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command(
            "setActiveObject",
            Some(&json!({ "objectId": rock_id })),
            &document,
            &ViewState::default(),
        );
        envelope = apply_ops(&envelope, &ops);
        assert_eq!(envelope.fixture.active_object_id, rock_id);
        assert_ne!(envelope.fixture.active_object_id, second_id);
    }

    #[test]
    fn edit_tools_include_extrude() {
        let app = LowpolyPlayApp::default();
        let tools = app.tools(&app.initial_document_json(), &ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("lowpoly-tools-extrude"));
    }

    #[test]
    fn paint_tools_include_brush() {
        let app = LowpolyPlayApp::default();
        let mut view = ViewState::default();
        view.active_mode_id = Some("paint".into());
        let tools = app.tools(&app.initial_document_json(), &view);
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("lowpoly-paint-brush"));
        assert!(json.contains("lowpoly-paint-undo"));
    }

    #[test]
    fn world_pick_updates_selection() {
        let mut app = LowpolyPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "worldPick",
            Some(&json!({ "granularity": "face", "id": 0, "merge": "replace" })),
            &document,
            &ViewState::default(),
        );
        let envelope: LowpolyPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.fixture.selection.mode, "face");
        assert_eq!(envelope.fixture.selection.ids, vec![0]);
    }

    #[test]
    fn paint_stroke_updates_pixels() {
        let mut app = LowpolyPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "paintAt",
            Some(&json!({ "u": 0.5, "v": 0.5 })),
            &document,
            &ViewState::default(),
        );
        assert!(!ops.is_empty());
        let object_id = parse_envelope(&document).fixture.active_object_id;
        let pixels = app.paint_pixels.get(&object_id).expect("paint pixels stored");
        assert!(!pixels.is_empty());
        assert!(pixels[0].iter().any(|value| *value > 0));
    }

    #[test]
    fn selection_json_carries_paint_interaction_mode() {
        let app = LowpolyPlayApp::default();
        let envelope = parse_envelope(&app.initial_document_json());
        let selection = world_selection_json_for(&envelope, Some("paint"), None);
        assert!(selection.contains("\"interactionMode\":\"paint\""));
    }

    #[test]
    fn merge_selection_ids_supports_add_toggle_and_invertive() {
        assert_eq!(merge_selection_ids(&[1], &[2], "add"), vec![1, 2]);
        assert_eq!(merge_selection_ids(&[1, 2], &[2, 3], "toggle"), vec![1, 3]);
        assert_eq!(merge_selection_ids(&[1, 2], &[2], "invertive"), vec![1]);
        assert_eq!(merge_selection_ids(&[1], &[2], "replace"), vec![2]);
    }

    #[test]
    fn toggle_selection_target_sets_active_object_and_keys() {
        let mut app = LowpolyPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.fixture.selection.ids.clear();
        envelope.fixture.selection.keys.clear();
        let object_id = envelope.fixture.active_object_id.clone();
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command(
            "toggleSelectionTarget",
            Some(&json!({ "objectId": object_id, "mode": "face", "id": 0, "merge": "invertive" })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&envelope, &ops);
        assert_eq!(next.fixture.selection.mode, "face");
        assert_eq!(next.fixture.selection.ids, vec![0]);
        assert!(next.fixture.selection.targets.face);
        assert_eq!(next.fixture.selection.keys.len(), 1);
        assert!(next.fixture.selection.keys[0].contains(":face:0"));
    }

    #[test]
    fn set_hover_round_trips_through_runtime() {
        let mut app = LowpolyPlayApp::default();
        let envelope = parse_envelope(&app.initial_document_json());
        let object_id = envelope.fixture.active_object_id.clone();
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command(
            "setHover",
            Some(&json!({ "objectId": object_id, "mode": "vertex", "id": 3 })),
            &document,
            &ViewState::default(),
        );
        let hovered = apply_ops(&envelope, &ops);
        let target = hovered.runtime.hovered_target.as_ref().expect("hover target");
        assert_eq!(target.object_id.as_deref(), Some(object_id.as_str()));
        assert_eq!(target.mode.as_deref(), Some("vertex"));
        assert_eq!(target.id, Some(3));
        let highlighted = highlighted_hierarchy_ids(&hovered.runtime, &hovered.fixture);
        assert_eq!(highlighted.len(), 1);
        assert!(highlighted[0].contains(".vertex.3"));
        let clear_ops = app.handle_command("setHover", None, &serde_json::to_string(&hovered).unwrap(), &ViewState::default());
        let cleared = apply_ops(&hovered, &clear_ops);
        assert!(cleared.runtime.hovered_target.is_none());
    }

    #[test]
    fn gumball_target_emitted_for_face_selection() {
        let app = LowpolyPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.fixture.selection = LowpolySelection {
            targets: LowpolySelectionTargets {
                mesh: false,
                vertex: false,
                edge: false,
                face: true,
            },
            keys: vec![],
            mode: "face".into(),
            ids: vec![0],
        };
        let doc = LowpolyDocument::new(envelope.fixture.clone()).unwrap();
        let selection = world_selection_json_for(&envelope, Some("model"), Some(&doc));
        assert!(selection.contains("\"gumballTarget\""));
        assert!(selection.contains("\"gumballActive\":true"));
    }

    #[test]
    fn hierarchy_tree_nests_vertices_edges_and_faces() {
        let app = LowpolyPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.fixture.selection.mode = "vertex".into();
        envelope.fixture.selection.ids = vec![0, 1];
        sync_selection_keys(&mut envelope.fixture);
        let document = serde_json::to_string(&envelope).unwrap();
        let node = app.render(LOWPOLY_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Vertices"));
        assert!(json.contains("Edges"));
        assert!(json.contains("Faces"));
        assert!(json.contains("\"iconId\":\"circle\""));
        assert!(json.contains("\"iconId\":\"minus\""));
        assert!(json.contains("\"iconId\":\"square\""));
        assert!(json.contains("flipFaces"));
        let selected = selected_hierarchy_ids(&envelope.fixture);
        assert_eq!(selected.len(), 2);
        assert!(json.contains(&selected[0]));
    }

    fn apply_ops(envelope: &LowpolyPlayEnvelope, ops: &[String]) -> LowpolyPlayEnvelope {
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
