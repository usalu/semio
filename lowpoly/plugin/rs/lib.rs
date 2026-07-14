//! 🔺 Lowpoly plugin — mesh + paint editor as a typed {@link DocumentApp}. Mesh/object structure and
//! paint pixels are the document projection (undoable via the VCS store); tools, selection, camera and
//! the mid-drag paint scratch are ephemeral app-struct state.

use base64::Engine;
use kernel_3d_mesh::{EdgeId, FaceId, MirrorAxis, Vec3, WeldMode};
use lowpoly_core::{
    composite_layer_pixels, default_projection, empty_paint_pixels, flood_fill, pixel_runs_from_diff,
    projection_from_mesh_json, sample_pixel_from, stamp_brush, LowpolyDocument, LowpolyObject,
    LowpolyObjectPatch, LowpolyOp, LowpolyPaintLayer, LowpolyProjection, LowpolySelection,
    LowpolySelectionTargets, LOWPOLY_DOCUMENT_SCHEMA, LOWPOLY_PAINT_TEXTURE_SIZE,
};
use png::{BitDepth, ColorType, Encoder};
use semio_framework_plugin::{
    apply_world3d_sun_action, build_canvas_2d_scene, build_world_3d_scene, create_default_layout,
    create_named_layout, engagement_token_matches, merge_world_selection_ids, mesh_from_kind,
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text,
    world3d_camera_json, world3d_scene, world3d_selection_json, world3d_sun_measures, ActionArgDef,
    ActionArgOption, ActionDescriptor, ActionEmit, App, AppLabelsOverlay, Canvas2dScene, DocumentApp,
    DocumentView, MeshData, PanelGroup, SurfaceKind, ToolCategory, ToolDefinition, UiFieldNode,
    UiInspectorFieldGroup, UiNode, UiToggleNode, ViewState, WindowEngagement, WindowEngagementInput,
    WindowEngagementOption, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, MeasureSelectItem, WindowEngagementPossible,
    WindowEngagementStatus, SET_ACTIVE_TOOL_ACTION_ID,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use vcs::CollectionOp;

//#region 🔖Constants
const LOWPOLY_PLAY_APP_ID: &str = "lowpoly-play";
const LOWPOLY_PLAY_CONTROLLER_ID: &str = "lowpoly-play";
const LOWPOLY_PLAY_SURFACE_MAIN: &str = "lowpoly.play";
const LOWPOLY_PLAY_SURFACE_UV: &str = "lowpoly.play.uv";
const LOWPOLY_PLAY_BODY_MAIN: &str = "lowpoly.play.main";
const LOWPOLY_PLAY_BODY_UV: &str = "lowpoly.play.uv";
const LOWPOLY_PLAY_BODY_DOCUMENT: &str = "lowpoly.play.document";
const LOWPOLY_PLAY_BODY_CATALOGUE: &str = "lowpoly.play.catalogue";
const LOWPOLY_PLAY_BODY_INSPECTION: &str = "lowpoly.play.inspection";
const LOWPOLY_PLAY_BODY_LAYERS: &str = "lowpoly.play.layers";
const LOWPOLY_PLAY_WINDOW_MAIN: &str = "lowpoly-main";
const LOWPOLY_PLAY_WINDOW_UV: &str = "lowpoly-uv";
/// 🧰 The transform gumball tool a Model window falls back to when the host hasn't set an active tool.
const LOWPOLY_TRANSFORM_TOOL_DEFAULT: &str = "move";

const PRIMITIVE_CATALOG: &[(&str, &str, &str)] = &[
    ("box", "Cube", "box"),
    ("plane", "Plane", "square"),
    ("cylinder", "Cylinder", "cylinder"),
    ("cone", "Cone", "triangle"),
    ("ico_sphere", "Ico Sphere", "globe"),
];
//#endregion 🔖Constants

//#region 🔖Runtime
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LowpolyWorldCamera {
    #[serde(default = "default_world_cam_pos")]
    position: [f64; 3],
    #[serde(default)]
    target: [f64; 3],
    #[serde(default = "default_world_cam_fov")]
    fov: f64,
}

impl Default for LowpolyWorldCamera {
    fn default() -> Self {
        Self {
            position: default_world_cam_pos(),
            target: [0.0, 0.0, 0.0],
            fov: default_world_cam_fov(),
        }
    }
}

fn default_world_cam_pos() -> [f64; 3] {
    [18.0, -18.0, 12.0]
}

fn default_world_cam_fov() -> f64 {
    45.0
}

fn lowpoly_world_camera_json(runtime: &LowpolyPlayRuntime) -> String {
    world3d_camera_json(
        runtime.world_camera.position,
        runtime.world_camera.target,
        runtime.world_camera.fov,
    )
}

/// @emoji 🪟 Ephemeral, per-viewer editing state — never serialized into the document. Selection and
/// active object live here so picking/hovering never pollutes the undo history.
#[derive(Clone, Debug, PartialEq)]
struct LowpolyPlayRuntime {
    active_object_id: String,
    selection: LowpolySelection,
    paint_tool: String,
    active_paint_layer: u32,
    selection_method: String,
    selected_object_ids: Vec<String>,
    hovered_object_id: Option<String>,
    hovered_target: Option<LowpolyHoverTarget>,
    tool_params: Value,
    paint_color: [u8; 4],
    world_camera: LowpolyWorldCamera,
    engagement_input: String,
    show_edges: bool,
    sun: WorldSunConfig,
}

impl Default for LowpolyPlayRuntime {
    fn default() -> Self {
        Self {
            active_object_id: String::new(),
            selection: LowpolySelection::default(),
            paint_tool: "brush".into(),
            active_paint_layer: 0,
            selection_method: "rectangle".into(),
            selected_object_ids: Vec::new(),
            hovered_object_id: None,
            hovered_target: None,
            tool_params: default_tool_params(),
            paint_color: [255, 64, 64, 255],
            world_camera: LowpolyWorldCamera::default(),
            engagement_input: String::new(),
            show_edges: true,
            sun: WorldSunConfig::default(),
        }
    }
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

/// @emoji 🧭 A borrowed read view — the document projection plus the ephemeral runtime — threaded into
/// the render/panel/tool/scene builders.
#[derive(Clone, Copy)]
struct LowpolyView<'a> {
    projection: &'a LowpolyProjection,
    runtime: &'a LowpolyPlayRuntime,
}

fn lowpoly_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: LOWPOLY_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
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

fn resolve_active_object_id(projection: &LowpolyProjection, runtime: &LowpolyPlayRuntime) -> String {
    if projection.objects.iter().any(|object| object.id == runtime.active_object_id) {
        runtime.active_object_id.clone()
    } else {
        projection.objects.first().map(|object| object.id.clone()).unwrap_or_default()
    }
}

fn build_doc(projection: &LowpolyProjection, runtime: &LowpolyPlayRuntime) -> Option<LowpolyDocument> {
    let active = resolve_active_object_id(projection, runtime);
    LowpolyDocument::with_context(projection.clone(), active, runtime.selection.clone()).ok()
}

fn active_object<'a>(view: LowpolyView<'a>) -> Option<&'a LowpolyObject> {
    let id = resolve_active_object_id(view.projection, view.runtime);
    view.projection.objects.iter().find(|object| object.id == id)
}
//#endregion 🔖Runtime

//#region 🔖MeshTransfer
fn mesh_data_from_transfer(transfer: &Value, paint_texture: Option<String>) -> MeshData {
    let read_f32 = |key: &str| -> Vec<f32> {
        transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
    };
    let read_u32 = |key: &str| -> Vec<u32> {
        transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
    };
    let read_u8 = |key: &str| -> Vec<u8> {
        transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
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

/// @emoji 🧮 The changed-field patch turning `before` into `after`, for a mesh edit's `Objects(Patch)`.
fn object_patch_diff(before: &LowpolyObject, after: &LowpolyObject) -> LowpolyObjectPatch {
    LowpolyObjectPatch {
        name: (before.name != after.name).then(|| after.name.clone()),
        smooth_shading: (before.smooth_shading != after.smooth_shading).then_some(after.smooth_shading),
        transform: (before.transform != after.transform).then(|| after.transform.clone()),
        mesh_json: (before.mesh_json != after.mesh_json).then(|| after.mesh_json.clone()),
    }
}
//#endregion 🔖MeshTransfer

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
        "remove" | "subtractive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                merged.retain(|entry| entry != id);
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

fn document_target_row_id(object_id: &str, _object_index: usize, mode: &str, id: u32) -> String {
    format!("lowpoly-document.{object_id}.{mode}.{id}")
}

fn selection_key(object_id: &str, object_index: usize, mode: &str, id: u32) -> String {
    format!("lowpoly:{object_id}:{object_index}:{mode}:{id}")
}

fn object_index_for(projection: &LowpolyProjection, object_id: &str) -> usize {
    projection.objects.iter().position(|object| object.id == object_id).unwrap_or(0)
}

fn enable_selection_target_kind(targets: &mut LowpolySelectionTargets, mode: &str) {
    match mode {
        "vertex" => targets.vertex = true,
        "edge" => targets.edge = true,
        "face" => targets.face = true,
        _ => targets.mesh = true,
    }
}

fn sync_selection_keys(runtime: &mut LowpolyPlayRuntime, projection: &LowpolyProjection) {
    let active = resolve_active_object_id(projection, runtime);
    let object_index = object_index_for(projection, &active);
    let mode = runtime.selection.mode.clone();
    runtime.selection.keys = runtime
        .selection
        .ids
        .iter()
        .map(|id| selection_key(&active, object_index, &mode, *id))
        .collect();
}

fn apply_component_selection(
    runtime: &mut LowpolyPlayRuntime,
    projection: &LowpolyProjection,
    mode: &str,
    incoming: &[u32],
    merge: &str,
) {
    let normalized = LowpolyDocument::normalize_selection_mode(mode);
    enable_selection_target_kind(&mut runtime.selection.targets, &normalized);
    runtime.selection.mode = normalized;
    runtime.selection.ids = merge_selection_ids(&runtime.selection.ids, incoming, merge);
    sync_selection_keys(runtime, projection);
}

fn selected_document_ids(view: LowpolyView) -> Vec<String> {
    let active = resolve_active_object_id(view.projection, view.runtime);
    let object_index = object_index_for(view.projection, &active);
    view.runtime
        .selection
        .ids
        .iter()
        .map(|id| document_target_row_id(&active, object_index, &view.runtime.selection.mode, *id))
        .collect()
}

fn highlighted_document_ids(view: LowpolyView) -> Vec<String> {
    view.runtime
        .hovered_target
        .as_ref()
        .and_then(|target| {
            let object_id = target.object_id.as_deref()?;
            let mode = target.mode.as_deref()?;
            let id = target.id?;
            Some(document_target_row_id(
                object_id,
                object_index_for(view.projection, object_id),
                mode,
                id,
            ))
        })
        .into_iter()
        .collect()
}

fn gumball_target_world(doc: &LowpolyDocument, view: LowpolyView) -> Option<[f64; 3]> {
    let pivot = doc.selection_transform_pivot().ok()?;
    let active = resolve_active_object_id(view.projection, view.runtime);
    let object = view.projection.objects.iter().find(|entry| entry.id == active)?;
    let position = &object.transform.position;
    Some([
        position[0] as f64 + pivot.x() as f64,
        position[1] as f64 + pivot.y() as f64,
        position[2] as f64 + pivot.z() as f64,
    ])
}

fn gumball_active(view: LowpolyView) -> bool {
    let active = resolve_active_object_id(view.projection, view.runtime);
    !view.runtime.selection.ids.is_empty()
        || (view.runtime.selection.targets.mesh
            && view.runtime.selected_object_ids.iter().any(|id| id == &active))
}
//#endregion 🔖SelectionHelpers

//#region 🔖Scene
fn world_selection_json_for(view: LowpolyView, active_tool: &str, active_mode_id: Option<&str>, doc: Option<&LowpolyDocument>) -> String {
    let runtime = view.runtime;
    let active = resolve_active_object_id(view.projection, runtime);
    let mut value: Value = serde_json::from_str(&world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_object_ids,
        runtime.hovered_object_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!(runtime.selection.mode));
        object.insert("targets".into(), json!(runtime.selection.targets));
        object.insert("transformTool".into(), json!(active_tool));
        object.insert(
            "interactionMode".into(),
            json!(if active_mode_id == Some("paint") { "paint" } else { "model" }),
        );
        object.insert("componentIds".into(), json!(runtime.selection.ids));
        object.insert("selectionMode".into(), json!(runtime.selection.mode));
        object.insert("activeObjectId".into(), json!(active));
        object.insert("gumballActive".into(), json!(gumball_active(view)));
        object.insert("showEdges".into(), json!(runtime.show_edges));
        if let Some(target) = runtime.hovered_target.as_ref() {
            object.insert("hoveredComponent".into(), json!(target));
        }
        if let Some(loaded) = doc {
            if let Some(target) = gumball_target_world(loaded, view) {
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

fn world_instances_json(view: LowpolyView) -> String {
    let runtime = view.runtime;
    let instances: Vec<Value> = view
        .projection
        .objects
        .iter()
        .enumerate()
        .map(|(object_index, object)| {
            let selected = runtime.selected_object_ids.iter().any(|id| id == &object.id)
                || (runtime.selection.mode == "mesh"
                    && runtime.selection.ids.iter().any(|id| *id as usize == object_index));
            let hovered = runtime
                .hovered_target
                .as_ref()
                .map(|target| {
                    target.mode.as_deref() == Some("mesh") && target.object_id.as_deref() == Some(object.id.as_str())
                })
                .unwrap_or_else(|| runtime.hovered_object_id.as_deref() == Some(object.id.as_str()));
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
                "smoothShading": object.smooth_shading,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn uv_canvas_layers_json(doc: &LowpolyDocument, view: LowpolyView, texture_cache: &HashMap<String, String>) -> String {
    let object_id = resolve_active_object_id(view.projection, view.runtime);
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

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the lowpoly mesh editor; one field per label makes every locale combination compile-checked.
struct LowpolyLabels {
    meshes: &'static str,
    primitives: &'static str,
    paint_layers: &'static str,
    vertices: &'static str,
    edges: &'static str,
    faces: &'static str,
    flip_normal: &'static str,
    primitive_box: &'static str,
    primitive_plane: &'static str,
    primitive_cylinder: &'static str,
    primitive_cone: &'static str,
    primitive_ico_sphere: &'static str,
    object: &'static str,
    transform: &'static str,
    tool_params: &'static str,
    selection: &'static str,
    edit: &'static str,
    history: &'static str,
    paint: &'static str,
    uv: &'static str,
    window_main: &'static str,
    window_uv: &'static str,
}

const LOWPOLY_LABELS_NATIVE_EN: LowpolyLabels = LowpolyLabels {
    meshes: "Meshes",
    primitives: "Primitives",
    paint_layers: "Paint Layers",
    vertices: "Vertices",
    edges: "Edges",
    faces: "Faces",
    flip_normal: "Flip normal",
    primitive_box: "Cube",
    primitive_plane: "Plane",
    primitive_cylinder: "Cylinder",
    primitive_cone: "Cone",
    primitive_ico_sphere: "Ico Sphere",
    object: "Object",
    transform: "Transform",
    tool_params: "Tool Params",
    selection: "Selection",
    edit: "Edit",
    history: "History",
    paint: "Paint",
    uv: "UV",
    window_main: "Model",
    window_uv: "UV",
};

const LOWPOLY_LABELS_NATIVE_DE: LowpolyLabels = LowpolyLabels {
    meshes: "Netze",
    primitives: "Primitive",
    paint_layers: "Malebenen",
    vertices: "Eckpunkte",
    edges: "Kanten",
    faces: "Flächen",
    flip_normal: "Normale umkehren",
    primitive_box: "Würfel",
    primitive_plane: "Ebene",
    primitive_cylinder: "Zylinder",
    primitive_cone: "Kegel",
    primitive_ico_sphere: "Ikokugel",
    object: "Objekt",
    transform: "Transformation",
    tool_params: "Werkzeugparameter",
    selection: "Auswahl",
    edit: "Bearbeiten",
    history: "Verlauf",
    paint: "Malen",
    uv: "UV",
    window_main: "Modell",
    window_uv: "UV",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unsupported locales fall back to native English.
fn lowpoly_labels(view_state: &ViewState) -> &'static LowpolyLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &LOWPOLY_LABELS_NATIVE_DE
    } else {
        &LOWPOLY_LABELS_NATIVE_EN
    }
}

/// 🗣️ Resolves a primitive catalogue entry's display label from its stable kind; unknown kinds fall back to the catalog's native English text.
fn primitive_catalog_label(kind: &str, fallback_label: &'static str, labels: &LowpolyLabels) -> &'static str {
    match kind {
        "box" => labels.primitive_box,
        "plane" => labels.primitive_plane,
        "cylinder" => labels.primitive_cylinder,
        "cone" => labels.primitive_cone,
        "ico_sphere" => labels.primitive_ico_sphere,
        _ => fallback_label,
    }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn tree_item(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    action: Option<ActionDescriptor>,
    hover_action: Option<ActionDescriptor>,
    unhover_action: Option<ActionDescriptor>,
    actions: Option<Vec<semio_framework_plugin::UiTreeItemAction>>,
    items: Option<Vec<semio_framework_plugin::UiTreeItemNode>>,
    default_open: Option<bool>,
    description: Option<String>,
) -> semio_framework_plugin::UiTreeItemNode {
    semio_framework_plugin::UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open,
        action,
        hover_action,
        unhover_action,
        actions,
        draggable: None,
        drag_data: None,
        items,
        control: None,
        is_hidden: None,
    }
}

fn build_document_tree(view: LowpolyView, doc: &LowpolyDocument, labels: &LowpolyLabels) -> UiNode {
    let active_id = resolve_active_object_id(view.projection, view.runtime);
    let selected_ids = selected_document_ids(view);
    let highlighted_ids = highlighted_document_ids(view);
    let items: Vec<semio_framework_plugin::UiTreeItemNode> = view
        .projection
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
                        let row_id = document_target_row_id(&object.id, object_index, mode, id as u32);
                        let hover_args = json!({
                            "objectId": object.id,
                            "mode": mode,
                            "id": id,
                        });
                        let mut actions = None;
                        if mode == "face" {
                            actions = Some(vec![semio_framework_plugin::UiTreeItemAction {
                                icon_id: "flip-vertical".into(),
                                label: Some(labels.flip_normal.into()),
                                action: lowpoly_action("flipFaces", Some(json!({ "faceIds": [id] }))),
                                reveal_on_hover: Some(true),
                            }]);
                        }
                        tree_item(
                            row_id,
                            format!("{label} {id}"),
                            Some(icon),
                            Some(lowpoly_action(
                                "toggleSelectionTarget",
                                Some(json!({
                                    "objectId": object.id,
                                    "mode": mode,
                                    "id": id,
                                    "merge": "invertive",
                                })),
                            )),
                            Some(lowpoly_action("setHover", Some(hover_args.clone()))),
                            Some(lowpoly_action("setHover", None)),
                            actions,
                            None,
                            None,
                            None,
                        )
                    })
                    .collect();
                tree_item(
                    format!("lowpoly-document.{object_id}.{mode}.group"),
                    label.to_string(),
                    Some(icon),
                    None,
                    None,
                    None,
                    None,
                    Some(leaves),
                    None,
                    Some(format!("{count}")),
                )
            };
            tree_item(
                format!("lowpoly-document.{object_id}"),
                object.name.clone(),
                Some("box"),
                Some(lowpoly_action(
                    "toggleSelectionTarget",
                    Some(json!({
                        "objectId": object.id,
                        "mode": "mesh",
                        "id": 0,
                        "merge": "invertive",
                    })),
                )),
                None,
                None,
                None,
                Some(vec![
                    component_group("vertex", labels.vertices, "circle", vertex_count),
                    component_group("edge", labels.edges, "minus", edge_count),
                    component_group("face", labels.faces, "square", face_count),
                ]),
                Some(object.id == active_id),
                Some(object.id.clone()),
            )
        })
        .collect();
    UiNode::Tree(semio_framework_plugin::UiTreeNode {
        sections: vec![semio_framework_plugin::UiTreeSectionNode {
            id: "lowpoly-play-document.meshes".into(),
            label: Some(labels.meshes.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: if selected_ids.is_empty() { None } else { Some(selected_ids) },
        highlighted_ids: if highlighted_ids.is_empty() { None } else { Some(highlighted_ids) },
        selection_change: None,
        drop_action: None,
    })
}

fn build_catalogue_tree(labels: &LowpolyLabels) -> UiNode {
    let items: Vec<semio_framework_plugin::UiTreeItemNode> = PRIMITIVE_CATALOG
        .iter()
        .map(|(kind, label, icon)| {
            tree_item(
                format!("lowpoly-play-catalogue.{kind}"),
                primitive_catalog_label(kind, label, labels),
                Some(icon),
                Some(lowpoly_action("addPrimitive", Some(json!({ "kind": kind })))),
                None,
                None,
                None,
                None,
                None,
                Some((*kind).to_string()),
            )
        })
        .collect();
    UiNode::Tree(semio_framework_plugin::UiTreeNode {
        sections: vec![semio_framework_plugin::UiTreeSectionNode {
            id: "lowpoly-play-catalogue.primitives".into(),
            label: Some(labels.primitives.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_layers_tree(view: LowpolyView, labels: &LowpolyLabels) -> UiNode {
    let object = active_object(view);
    let layers = object.map(|entry| entry.paint_layers.as_slice()).unwrap_or(&[]);
    let active_layer = view.runtime.active_paint_layer;
    let items: Vec<semio_framework_plugin::UiTreeItemNode> = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| {
            tree_item(
                format!("lowpoly-layer:{index}"),
                layer.name.clone(),
                Some("layers"),
                Some(lowpoly_action("setActivePaintLayer", Some(json!({ "layerIndex": index })))),
                None,
                None,
                None,
                None,
                None,
                Some(format!("{} · {}", layer.opacity, layer.blend_mode)),
            )
        })
        .collect();
    UiNode::Tree(semio_framework_plugin::UiTreeNode {
        sections: vec![semio_framework_plugin::UiTreeSectionNode {
            id: "lowpoly-play-layers.paint".into(),
            label: Some(labels.paint_layers.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: Some(vec![format!("lowpoly-layer:{active_layer}")]),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn inspector_tool_param_field(id: &str, label: &str, key: &str, value: &Value) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("lowpoly-play-inspector.{id}"),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("lowpoly-play-inspector.{id}.input"),
            input_kind: "number".into(),
            value: value.get(key).map(|entry| entry.to_string()).unwrap_or_else(|| "0".into()),
            placeholder: None,
            commit: None,
            on_change: lowpoly_action("setToolParam", Some(json!({ "key": key }))),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn build_inspector_tree(view: LowpolyView, active_tool: &str, labels: &LowpolyLabels) -> UiNode {
    let Some(object) = active_object(view) else {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {LOWPOLY_DOCUMENT_SCHEMA}")),
            ui_text("No active object".to_string()),
        ]);
    };
    let params = &view.runtime.tool_params;
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.object".into(),
            label: labels.object.into(),
            default_open: None,
            fields: vec![
                UiNode::Field(UiFieldNode {
                    id: "lowpoly-play-inspector.object.name".into(),
                    label: "Name".into(),
                    child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                        id: "lowpoly-play-inspector.object.name.input".into(),
                        input_kind: "text".into(),
                        value: object.name.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: lowpoly_action("patchObject", Some(json!({ "objectId": object.id, "field": "name" }))),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                UiNode::Field(UiFieldNode {
                    id: "lowpoly-play-inspector.object.smooth".into(),
                    label: "Smooth Shading".into(),
                    child: Box::new(UiNode::Toggle(UiToggleNode {
                        id: "lowpoly-play-inspector.object.smooth.toggle".into(),
                        icon_id: "sun".into(),
                        pressed: object.smooth_shading,
                        text: None,
                        on_change: lowpoly_action("patchObject", Some(json!({ "objectId": object.id, "field": "smoothShading" }))),
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                ui_inspector_readonly_field(
                    "lowpoly-play-inspector.object.selection",
                    "Selection",
                    &format!(
                        "{} · {} selected",
                        format_selection_targets_label(&view.runtime.selection.targets),
                        view.runtime.selection.ids.len()
                    ),
                ),
                ui_inspector_readonly_field(
                    "lowpoly-play-inspector.object.selection-mode",
                    "Selection Mode",
                    &view.runtime.selection.mode,
                ),
            ],
        },
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.transform".into(),
            label: labels.transform.into(),
            default_open: None,
            fields: vec![ui_inspector_readonly_field(
                "lowpoly-play-inspector.transform.tool",
                "Tool",
                active_tool,
            )],
        },
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.tool-params".into(),
            label: labels.tool_params.into(),
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
fn format_selection_targets_label(targets: &LowpolySelectionTargets) -> String {
    let mut parts = Vec::new();
    if targets.mesh {
        parts.push("mesh");
    }
    if targets.vertex {
        parts.push("vertex");
    }
    if targets.edge {
        parts.push("edge");
    }
    if targets.face {
        parts.push("face");
    }
    if parts.is_empty() {
        "none".into()
    } else {
        parts.join("+")
    }
}

fn set_active_tool_action(tool_id: &str) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: LOWPOLY_PLAY_CONTROLLER_ID.into(),
        action: SET_ACTIVE_TOOL_ACTION_ID.into(),
        args: Some(json!({ "toolId": tool_id, "windowKindId": LOWPOLY_PLAY_WINDOW_MAIN })),
    }
}

fn lowpoly_window_engagement(view: LowpolyView, active_tool: &str) -> WindowEngagement {
    let runtime = view.runtime;
    let transform = active_tool;
    let selected_count = runtime.selection.ids.len();
    WindowEngagement {
        session_active: Some(true),
        options: Some(vec![
            WindowEngagementOption {
                id: "lowpoly.opt.move".into(),
                label: Some("Move".into()),
                icon_id: Some("move".into()),
                pressed: Some(transform == "move"),
                disabled: None,
                action: Some(set_active_tool_action("move")),
            },
            WindowEngagementOption {
                id: "lowpoly.opt.rotate".into(),
                label: Some("Rotate".into()),
                icon_id: Some("rotate-cw".into()),
                pressed: Some(transform == "rotate"),
                disabled: None,
                action: Some(set_active_tool_action("rotate")),
            },
            WindowEngagementOption {
                id: "lowpoly.opt.scale".into(),
                label: Some("Scale".into()),
                icon_id: Some("maximize-2".into()),
                pressed: Some(transform == "scale"),
                disabled: None,
                action: Some(set_active_tool_action("scale")),
            },
            WindowEngagementOption {
                id: "lowpoly.opt.snap".into(),
                label: Some("Snap".into()),
                icon_id: Some("magnet".into()),
                pressed: None,
                disabled: None,
                action: Some(lowpoly_action("snap", None)),
            },
            WindowEngagementOption {
                id: "lowpoly.opt.smooth".into(),
                label: Some("Smooth".into()),
                icon_id: Some("sun".into()),
                pressed: None,
                disabled: None,
                action: Some(lowpoly_action("toggleSmooth", None)),
            },
            WindowEngagementOption {
                id: "lowpoly.opt.show-edges".into(),
                label: Some("Show Edges".into()),
                icon_id: Some("git-commit-horizontal".into()),
                pressed: Some(runtime.show_edges),
                disabled: None,
                action: Some(lowpoly_action("toggleShowEdges", None)),
            },
        ]),
        input: Some(WindowEngagementInput {
            id: Some("lowpoly-engagement".into()),
            value: Some(runtime.engagement_input.clone()),
            placeholder: Some("extrude, inset, mirror, decimate".into()),
            disabled: None,
            on_change: Some(lowpoly_action("engagementInput", None)),
            on_submit: Some(lowpoly_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "lowpoly-status".into(),
            text: format!(
                "{} · {} · {selected_count} selected",
                format_selection_targets_label(&runtime.selection.targets),
                transform,
            ),
        }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible {
                id: "lowpoly.eng.extrude".into(),
                label: "Extrude".into(),
                detail: None,
                action: Some(lowpoly_action("extrude", None)),
            },
            WindowEngagementPossible {
                id: "lowpoly.eng.triangulate".into(),
                label: "Triangulate".into(),
                detail: None,
                action: Some(lowpoly_action("triangulate", None)),
            },
        ]),
    }
}

fn tool_param_f64(params: &Value, key: &str, default: f64) -> f64 {
    tool_param_f32(params, key, default as f32) as f64
}

fn lowpoly_tool_param_slider(
    id: &str,
    label: &str,
    key: &str,
    params: &Value,
    default: f64,
    min: f64,
    max: f64,
    step: f64,
) -> WindowMeasure {
    WindowMeasure::Slider {
        id: format!("lowpoly-measure-{id}"),
        label: Some(label.into()),
        value: tool_param_f64(params, key, default),
        min,
        max,
        step: Some(step),
        on_change: lowpoly_action("setToolParam", Some(json!({ "key": key }))),
    }
}

/// 🎯 One selection-granularity toggle. Selection kinds are a non-exclusive multi-select (mesh + face +
/// edge + vertex can all be active at once), so they are a window-measure toggle group — NOT a
/// single-active tool group.
fn selection_kind_toggle(id: &str, icon: &str, label: &str, kind: &str, pressed: bool) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: format!("lowpoly-measure-selection-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: lowpoly_action("toggleSelectionKind", Some(json!({ "kind": kind }))),
    }
}

fn lowpoly_window_measures(runtime: &LowpolyPlayRuntime) -> Vec<WindowMeasure> {
    let params = &runtime.tool_params;
    let targets = &runtime.selection.targets;
    vec![
        WindowMeasure::Toggle {
            id: "lowpoly-measure-show-edges".into(),
            icon_id: "git-commit-horizontal".into(),
            label: Some("Show Edges".into()),
            pressed: runtime.show_edges,
            text: None,
            on_change: lowpoly_action("toggleShowEdges", None),
        },
        WindowMeasure::Group {
            id: "lowpoly-measure-selection-kind".into(),
            label: "Selection Kind".into(),
            default_open: Some(true),
            children: vec![
                selection_kind_toggle("mesh", "box", "Mesh", "mesh", targets.mesh),
                selection_kind_toggle("face", "square", "Face", "face", targets.face),
                selection_kind_toggle("edge", "minus", "Edge", "edge", targets.edge),
                selection_kind_toggle("vertex", "circle", "Vertex", "vertex", targets.vertex),
            ],
        },
        world3d_sun_measures("lowpoly", &runtime.sun, lowpoly_action),
        WindowMeasure::Select {
            id: "lowpoly-measure-selection-method".into(),
            label: Some("Selection Method".into()),
            value: runtime.selection_method.clone(),
            items: vec![
                MeasureSelectItem { id: "rectangle".into(), value: "rectangle".into(), label: "Rectangle".into() },
                MeasureSelectItem { id: "lasso".into(), value: "lasso".into(), label: "Lasso".into() },
            ],
            on_change: lowpoly_action("setSelectionMethod", None),
        },
        WindowMeasure::Group {
            id: "lowpoly-measure-tool-params".into(),
            label: "Tool Params".into(),
            default_open: Some(true),
            children: vec![
                lowpoly_tool_param_slider("extrude", "Extrude Distance", "extrudeDistance", params, 0.25, 0.01, 2.0, 0.01),
                lowpoly_tool_param_slider("inset", "Inset Amount", "insetAmount", params, 0.1, 0.01, 1.0, 0.01),
                lowpoly_tool_param_slider("bevel", "Bevel Amount", "bevelAmount", params, 0.05, 0.01, 0.5, 0.01),
                lowpoly_tool_param_slider("bevel-segments", "Bevel Segments", "bevelSegments", params, 1.0, 1.0, 8.0, 1.0),
                lowpoly_tool_param_slider("loop-cuts", "Loop Cuts", "loopCuts", params, 1.0, 1.0, 16.0, 1.0),
                lowpoly_tool_param_slider("decimate", "Decimate Ratio", "decimateRatio", params, 0.5, 0.05, 1.0, 0.05),
                lowpoly_tool_param_slider("snap", "Snap Grid", "snapGrid", params, 0.25, 0.05, 2.0, 0.05),
                lowpoly_tool_param_slider("mirror", "Mirror Axis", "mirrorAxis", params, 0.0, 0.0, 2.0, 1.0),
                lowpoly_tool_param_slider("brush-size", "Brush Size", "brushSize", params, 16.0, 1.0, 128.0, 1.0),
                lowpoly_tool_param_slider("brush-opacity", "Brush Opacity", "brushOpacity", params, 1.0, 0.0, 1.0, 0.05),
                lowpoly_tool_param_slider("brush-hardness", "Brush Hardness", "brushHardness", params, 0.5, 0.0, 1.0, 0.05),
            ],
        },
    ]
}

fn edit_tools(view: LowpolyView, labels: &LowpolyLabels) -> Vec<ToolNode> {
    let targets = &view.runtime.selection.targets;
    let transform = &view.runtime.transform_tool;
    vec![
        tool_collection(
            "lowpoly-tools-selection",
            "mouse-pointer",
            labels.selection,
            vec![
                tool_toggle("lowpoly-tools-selection-mesh", "box", "Mesh", targets.mesh, lowpoly_action("toggleSelectionKind", Some(json!({ "kind": "mesh" })))),
                tool_toggle("lowpoly-tools-selection-face", "square", "Face", targets.face, lowpoly_action("toggleSelectionKind", Some(json!({ "kind": "face" })))),
                tool_toggle("lowpoly-tools-selection-edge", "minus", "Edge", targets.edge, lowpoly_action("toggleSelectionKind", Some(json!({ "kind": "edge" })))),
                tool_toggle("lowpoly-tools-selection-vertex", "circle", "Vertex", targets.vertex, lowpoly_action("toggleSelectionKind", Some(json!({ "kind": "vertex" })))),
            ],
        )
        .with_category(ToolCategory::Selection),
        tool_collection(
            "lowpoly-tools-transform",
            "move",
            labels.transform,
            vec![
                tool_toggle("lowpoly-tools-transform-move", "move", "Move", transform == "move", lowpoly_action("setTransformTool", Some(json!({ "tool": "move" })))),
                tool_toggle("lowpoly-tools-transform-rotate", "rotate-cw", "Rotate", transform == "rotate", lowpoly_action("setTransformTool", Some(json!({ "tool": "rotate" })))),
                tool_toggle("lowpoly-tools-transform-scale", "maximize-2", "Scale", transform == "scale", lowpoly_action("setTransformTool", Some(json!({ "tool": "scale" })))),
            ],
        )
        .with_category(ToolCategory::Tools),
        tool_collection(
            "lowpoly-tools-edit",
            "pen-tool",
            labels.edit,
            vec![
                tool_button("lowpoly-tools-extrude", "box", "Extrude", lowpoly_action("extrude", None)),
                tool_button("lowpoly-tools-inset", "square", "Inset", lowpoly_action("inset", None)),
                tool_button("lowpoly-tools-flip", "flip-vertical", "Flip Normals", lowpoly_action("flipFaces", None)),
                tool_button("lowpoly-tools-bevel", "git-branch", "Bevel", lowpoly_action("bevel", None)),
                tool_button("lowpoly-tools-loop-cut", "git-commit", "Loop Cut", lowpoly_action("loopCut", None)),
                tool_button("lowpoly-tools-merge", "git-merge", "Merge", lowpoly_action("merge", None)),
                tool_button("lowpoly-tools-dissolve", "eraser", "Dissolve", lowpoly_action("dissolve", None)),
                tool_button("lowpoly-tools-subdivide", "grid-3x3", "Subdivide", lowpoly_action("subdivide", None)),
                tool_button("lowpoly-tools-triangulate", "triangle", "Triangulate", lowpoly_action("triangulate", None)),
                tool_button("lowpoly-tools-mirror", "flip-horizontal", "Mirror", lowpoly_action("mirror", None)),
                tool_button("lowpoly-tools-decimate", "minimize-2", "Decimate", lowpoly_action("decimate", None)),
            ],
        )
        .with_category(ToolCategory::Actions),
        tool_collection(
            "lowpoly-tools-history",
            "undo",
            labels.history,
            vec![
                tool_button("lowpoly-tools-undo", "undo", "Undo", lowpoly_action("undo", None)),
                tool_button("lowpoly-tools-redo", "redo", "Redo", lowpoly_action("redo", None)),
            ],
        )
        .with_category(ToolCategory::History),
    ]
}

fn paint_tools(runtime: &LowpolyPlayRuntime, labels: &LowpolyLabels) -> Vec<ToolNode> {
    let paint_tool = &runtime.paint_tool;
    vec![
        tool_collection(
            "lowpoly-paint-tools",
            "paintbrush",
            labels.paint,
            vec![
                tool_toggle("lowpoly-paint-brush", "paintbrush", "Brush", paint_tool == "brush", lowpoly_action("setPaintTool", Some(json!({ "tool": "brush" })))),
                tool_toggle("lowpoly-paint-eraser", "eraser", "Eraser", paint_tool == "eraser", lowpoly_action("setPaintTool", Some(json!({ "tool": "eraser" })))),
                tool_toggle("lowpoly-paint-fill", "paint-bucket", "Fill", paint_tool == "fill", lowpoly_action("setPaintTool", Some(json!({ "tool": "fill" })))),
                tool_toggle("lowpoly-paint-eyedropper", "pipette", "Eyedropper", paint_tool == "eyedropper", lowpoly_action("setPaintTool", Some(json!({ "tool": "eyedropper" })))),
            ],
        )
        .with_category(ToolCategory::Tools),
        tool_collection(
            "lowpoly-paint-uv",
            "grid-3x3",
            labels.uv,
            vec![
                tool_button("lowpoly-paint-unwrap", "grid-3x3", "Unwrap", lowpoly_action("unwrapActive", None)),
                tool_button("lowpoly-paint-mark-seam", "scissors", "Mark Seam", lowpoly_action("markUvSeam", Some(json!({ "seam": true })))),
                tool_button("lowpoly-paint-clear-seam", "unlink", "Clear Seam", lowpoly_action("clearSeam", None)),
            ],
        )
        .with_category(ToolCategory::Actions),
        tool_collection(
            "lowpoly-paint-history",
            "undo",
            labels.history,
            vec![
                tool_button("lowpoly-paint-undo", "undo", "Undo", lowpoly_action("undo", None)),
                tool_button("lowpoly-paint-redo", "redo", "Redo", lowpoly_action("redo", None)),
            ],
        )
        .with_category(ToolCategory::History),
    ]
}
//#endregion 🔖Tools

//#region 🔖LowpolyPlayApp
/// @emoji 🖌️ In-progress paint drag: the pre-stroke layer buffer and the accumulating scratch buffer.
/// Mid-drag ticks mutate `scratch` (view state); the stroke commits as ONE `PaintStroke` op on end.
struct PaintStrokeSession {
    object_id: String,
    layer_index: usize,
    base: Vec<u8>,
    scratch: Vec<u8>,
}

/// @emoji 🗃️ Pure render-side cache of composited paint textures (base64 PNG per object), invalidated
/// by a fingerprint over the document's paint pixels + the live stroke dirty counter. Never serialized.
#[derive(Default)]
struct PaintTextureCache {
    fingerprint: Option<u64>,
    textures: HashMap<String, String>,
}

struct LowpolyPlayApp {
    runtime: LowpolyPlayRuntime,
    stroke: Option<PaintStrokeSession>,
    stroke_drag_active: bool,
    stroke_dirty: u64,
    texture_cache: RefCell<PaintTextureCache>,
}

impl Default for LowpolyPlayApp {
    fn default() -> Self {
        Self {
            runtime: LowpolyPlayRuntime::default(),
            stroke: None,
            stroke_drag_active: false,
            stroke_dirty: 0,
            texture_cache: RefCell::new(PaintTextureCache::default()),
        }
    }
}

fn fnv1a_u64(mut hash: u64, bytes: &[u8]) -> u64 {
    let mut chunks = bytes.chunks_exact(8);
    for chunk in &mut chunks {
        let word = u64::from_le_bytes(chunk.try_into().unwrap());
        hash ^= word;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for &byte in chunks.remainder() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

impl LowpolyPlayApp {
    fn view<'a>(&'a self, projection: &'a LowpolyProjection) -> LowpolyView<'a> {
        LowpolyView { projection, runtime: &self.runtime }
    }

    /// @emoji 🖼️ The layers to composite for `object`, overlaying the live stroke scratch when the drag
    /// targets that object so the in-progress stroke previews before it commits.
    fn composite_layers_for(&self, object: &LowpolyObject) -> Vec<u8> {
        if let Some(session) = &self.stroke {
            if session.object_id == object.id {
                let mut layers = object.paint_layers.clone();
                if let Some(layer) = layers.get_mut(session.layer_index) {
                    layer.pixels = session.scratch.clone();
                }
                return composite_layer_pixels(&layers);
            }
        }
        composite_layer_pixels(&object.paint_layers)
    }

    fn paint_fingerprint(&self, projection: &LowpolyProjection) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for object in &projection.objects {
            hash = fnv1a_u64(hash, object.id.as_bytes());
            for layer in &object.paint_layers {
                hash = fnv1a_u64(hash, &[layer.visible as u8]);
                hash = fnv1a_u64(hash, &layer.opacity.to_le_bytes());
                hash = fnv1a_u64(hash, &layer.pixels);
            }
        }
        fnv1a_u64(hash, &self.stroke_dirty.to_le_bytes())
    }

    fn refresh_texture_cache(&self, projection: &LowpolyProjection) {
        let fingerprint = self.paint_fingerprint(projection);
        if self.texture_cache.borrow().fingerprint == Some(fingerprint) {
            return;
        }
        let mut textures = HashMap::new();
        for object in &projection.objects {
            let composite = self.composite_layers_for(object);
            if let Ok(png_bytes) = encode_rgba_png(&composite, LOWPOLY_PAINT_TEXTURE_SIZE as u32, LOWPOLY_PAINT_TEXTURE_SIZE as u32) {
                textures.insert(object.id.clone(), base64::engine::general_purpose::STANDARD.encode(png_bytes));
            }
        }
        *self.texture_cache.borrow_mut() = PaintTextureCache { fingerprint: Some(fingerprint), textures };
    }

    /// @emoji 🔧 Runs a kernel mesh edit against a compute session built from the projection + runtime,
    /// then emits the resulting `Objects(Patch)` capturing only the changed object fields.
    fn mesh_edit(
        &self,
        projection: &LowpolyProjection,
        edit: impl FnOnce(&mut LowpolyDocument) -> Result<(), String>,
    ) -> ActionEmit<LowpolyOp> {
        let Some(mut doc) = build_doc(projection, &self.runtime) else {
            return ActionEmit::default();
        };
        let object_id = doc.active_object_id().to_string();
        let Some(before) = projection.objects.iter().find(|object| object.id == object_id).cloned() else {
            return ActionEmit::default();
        };
        if edit(&mut doc).is_err() {
            return ActionEmit::default();
        }
        if doc.sync_meshes_to_projection().is_err() {
            return ActionEmit::default();
        }
        let Some(after) = doc.projection().objects.iter().find(|object| object.id == object_id).cloned() else {
            return ActionEmit::default();
        };
        let patch = object_patch_diff(&before, &after);
        if patch == LowpolyObjectPatch::default() {
            return ActionEmit::default();
        }
        ActionEmit::ops(vec![LowpolyOp::Objects(CollectionOp::Patch { id: object_id, patch })])
    }

    fn commit_stroke(&mut self) -> ActionEmit<LowpolyOp> {
        let Some(session) = self.stroke.take() else {
            return ActionEmit::default();
        };
        self.stroke_dirty += 1;
        let runs = pixel_runs_from_diff(&session.base, &session.scratch);
        if runs.is_empty() {
            return ActionEmit::default();
        }
        ActionEmit::ops(vec![LowpolyOp::PaintStroke {
            object_id: session.object_id,
            layer_index: session.layer_index,
            runs,
        }])
    }

    fn paint_at(&mut self, projection: &LowpolyProjection, object_id: String, u: f32, v: f32) -> ActionEmit<LowpolyOp> {
        let layer_index = self.runtime.active_paint_layer as usize;
        let tool = self.runtime.paint_tool.clone();
        let params = self.runtime.tool_params.clone();
        let color = self.runtime.paint_color;
        match tool.as_str() {
            "eyedropper" => {
                if let Some(object) = projection.objects.iter().find(|object| object.id == object_id) {
                    let composite = composite_layer_pixels(&object.paint_layers);
                    self.runtime.paint_color = sample_pixel_from(&composite, u, v);
                }
                ActionEmit::default()
            }
            "fill" => {
                let Some(layer) = projection
                    .objects
                    .iter()
                    .find(|object| object.id == object_id)
                    .and_then(|object| object.paint_layers.get(layer_index))
                else {
                    return ActionEmit::default();
                };
                let mut scratch = layer.pixels.clone();
                flood_fill(&mut scratch, u, v, color);
                let runs = pixel_runs_from_diff(&layer.pixels, &scratch);
                if runs.is_empty() {
                    return ActionEmit::default();
                }
                self.stroke_dirty += 1;
                ActionEmit::ops(vec![LowpolyOp::PaintStroke { object_id, layer_index, runs }])
            }
            _ => {
                let radius = tool_param_f32(&params, "brushSize", 16.0);
                let opacity = tool_param_f32(&params, "brushOpacity", 1.0);
                let hardness = tool_param_f32(&params, "brushHardness", 0.5);
                let eraser = tool == "eraser";
                let need_new = match &self.stroke {
                    Some(session) => session.object_id != object_id || session.layer_index != layer_index,
                    None => true,
                };
                if need_new {
                    let base = projection
                        .objects
                        .iter()
                        .find(|object| object.id == object_id)
                        .and_then(|object| object.paint_layers.get(layer_index))
                        .map(|layer| layer.pixels.clone())
                        .unwrap_or_else(empty_paint_pixels);
                    self.stroke = Some(PaintStrokeSession {
                        object_id: object_id.clone(),
                        layer_index,
                        scratch: base.clone(),
                        base,
                    });
                }
                if let Some(session) = self.stroke.as_mut() {
                    stamp_brush(&mut session.scratch, u, v, radius, color, hardness, opacity, eraser);
                }
                self.stroke_dirty += 1;
                if self.stroke_drag_active {
                    ActionEmit::default()
                } else {
                    self.commit_stroke()
                }
            }
        }
    }
}

/// @emoji 🎯 Extracts UV (0..1) from a paint action's args — either direct `u`/`v` (world 3d picks) or
/// canvas `x`/`y` positions mapped through the paint-texture extent (UV canvas).
fn paint_uv(args: Option<&Value>) -> Option<(f32, f32)> {
    let u = args.and_then(|value| value.get("u")).and_then(|value| value.as_f64());
    let v = args.and_then(|value| value.get("v")).and_then(|value| value.as_f64());
    if let (Some(u), Some(v)) = (u, v) {
        return Some((u as f32, v as f32));
    }
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64())?;
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64())?;
    let size = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
    let u = ((x / size) + 0.5).clamp(0.0, 1.0);
    let v = (1.0 - ((y / size) + 0.5).clamp(0.0, 1.0)).clamp(0.0, 1.0);
    Some((u as f32, v as f32))
}

impl DocumentApp for LowpolyPlayApp {
    type Projection = LowpolyProjection;
    type Op = LowpolyOp;

    fn app_id(&self) -> &str {
        LOWPOLY_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        LOWPOLY_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> LowpolyProjection {
        default_projection()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, LowpolyProjection>,
        view_state: &ViewState,
    ) -> ActionEmit<LowpolyOp> {
        let projection = doc.projection;
        match action {
            //#region 👁️ View actions
            "setActiveObject" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if projection.objects.iter().any(|object| object.id == object_id) {
                    self.runtime.active_object_id = object_id.into();
                }
                ActionEmit::default()
            }
            "setSelection" => {
                let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
                let ids: Vec<u32> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                self.runtime.selection.mode = LowpolyDocument::normalize_selection_mode(mode);
                self.runtime.selection.ids = ids;
                sync_selection_keys(&mut self.runtime, projection);
                ActionEmit::default()
            }
            "toggleSelectionKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("mesh");
                let enabled = match kind {
                    "vertex" => {
                        self.runtime.selection.targets.vertex = !self.runtime.selection.targets.vertex;
                        self.runtime.selection.targets.vertex
                    }
                    "edge" => {
                        self.runtime.selection.targets.edge = !self.runtime.selection.targets.edge;
                        self.runtime.selection.targets.edge
                    }
                    "face" => {
                        self.runtime.selection.targets.face = !self.runtime.selection.targets.face;
                        self.runtime.selection.targets.face
                    }
                    _ => {
                        self.runtime.selection.targets.mesh = !self.runtime.selection.targets.mesh;
                        self.runtime.selection.targets.mesh
                    }
                };
                if enabled {
                    self.runtime.selection.mode = LowpolyDocument::normalize_selection_mode(kind);
                    self.runtime.hovered_target = None;
                    self.runtime.hovered_object_id = None;
                }
                ActionEmit::default()
            }
            "setTransformTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    self.runtime.transform_tool = tool.into();
                }
                ActionEmit::default()
            }
            "setPaintTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    self.runtime.paint_tool = tool.into();
                }
                ActionEmit::default()
            }
            "setActivePaintLayer" => {
                self.runtime.active_paint_layer =
                    args.and_then(|value| value.get("layerIndex")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                ActionEmit::default()
            }
            "setToolParam" => {
                let key = args.and_then(|value| value.get("key")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if let Some(map) = self.runtime.tool_params.as_object_mut() {
                    map.insert(key.into(), value);
                } else {
                    let mut map = Map::new();
                    map.insert(key.into(), value);
                    self.runtime.tool_params = Value::Object(map);
                }
                ActionEmit::default()
            }
            "engagementInput" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    self.runtime.engagement_input = value.into();
                }
                ActionEmit::default()
            }
            "toggleShowEdges" => {
                self.runtime.show_edges = !self.runtime.show_edges;
                ActionEmit::default()
            }
            "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                apply_world3d_sun_action(&mut self.runtime.sun, action, args);
                ActionEmit::default()
            }
            "setSelectionMethod" => {
                self.runtime.selection_method = args
                    .and_then(|value| value.get("method"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle")
                    .into();
                ActionEmit::default()
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        self.runtime.world_camera = parsed;
                    }
                }
                ActionEmit::default()
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                self.runtime.selected_object_ids = merge_world_selection_ids(&self.runtime.selected_object_ids, &ids, merge);
                if let Some(first) = self.runtime.selected_object_ids.first() {
                    self.runtime.active_object_id = first.clone();
                }
                ActionEmit::default()
            }
            "worldHover" => {
                self.runtime.hovered_object_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                self.runtime.hovered_target = self.runtime.hovered_object_id.as_ref().map(|object_id| LowpolyHoverTarget {
                    object_id: Some(object_id.clone()),
                    mode: Some("mesh".into()),
                    id: Some(0),
                });
                ActionEmit::default()
            }
            "setHover" => {
                if args.is_none() || args.and_then(|value| value.get("objectId")).is_none() {
                    self.runtime.hovered_target = None;
                    self.runtime.hovered_object_id = None;
                } else {
                    let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
                    let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).map(str::to_string);
                    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).map(|value| value as u32);
                    self.runtime.hovered_object_id = object_id.clone();
                    self.runtime.hovered_target = Some(LowpolyHoverTarget { object_id, mode, id });
                }
                ActionEmit::default()
            }
            "toggleSelectionTarget" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh");
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("invertive");
                if projection.objects.iter().any(|object| object.id == object_id) {
                    self.runtime.active_object_id = object_id.into();
                    apply_component_selection(&mut self.runtime, projection, mode, &[id], merge);
                }
                ActionEmit::default()
            }
            "worldPick" => {
                let granularity = args.and_then(|value| value.get("granularity")).and_then(|value| value.as_str()).unwrap_or("mesh");
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                if args.and_then(|value| value.get("id")).map_or(true, |value| value.is_null()) {
                    if merge == "replace" {
                        self.runtime.selection.ids.clear();
                        sync_selection_keys(&mut self.runtime, projection);
                    }
                    return ActionEmit::default();
                }
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                apply_component_selection(&mut self.runtime, projection, granularity, &[id], merge);
                ActionEmit::default()
            }
            "paintStrokeBegin" => {
                self.stroke_drag_active = true;
                self.stroke = None;
                ActionEmit::default()
            }
            "paintStrokeEnd" => {
                self.stroke_drag_active = false;
                self.commit_stroke()
            }
            //#endregion 👁️ View actions

            //#region ✏️ Paint operations
            "paintStroke" | "paintAt" | "canvasPointerDown" | "canvasPointerMove" => {
                if action == "canvasPointerMove" && !self.stroke_drag_active {
                    return ActionEmit::default();
                }
                let Some((u, v)) = paint_uv(args) else {
                    return ActionEmit::default();
                };
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| resolve_active_object_id(projection, &self.runtime));
                self.paint_at(projection, object_id, u, v)
            }
            "paintFill" | "fillBucket" => {
                let Some((u, v)) = paint_uv(args) else {
                    return ActionEmit::default();
                };
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| resolve_active_object_id(projection, &self.runtime));
                self.runtime.paint_tool = "fill".into();
                self.paint_at(projection, object_id, u, v)
            }
            "paintSample" => {
                let Some((u, v)) = paint_uv(args) else {
                    return ActionEmit::default();
                };
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| resolve_active_object_id(projection, &self.runtime));
                self.runtime.paint_tool = "eyedropper".into();
                self.paint_at(projection, object_id, u, v)
            }
            "addPaintLayer" => {
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| resolve_active_object_id(projection, &self.runtime));
                let name = args.and_then(|value| value.get("name")).and_then(|value| value.as_str()).unwrap_or("Layer");
                let index = projection
                    .objects
                    .iter()
                    .find(|object| object.id == object_id)
                    .map(|object| object.paint_layers.len())
                    .unwrap_or(0);
                ActionEmit::ops(vec![LowpolyOp::AddPaintLayer {
                    object_id,
                    index,
                    layer: LowpolyPaintLayer::new(name),
                }])
            }
            //#endregion ✏️ Paint operations

            //#region ✏️ Object + mesh operations
            "addPrimitive" => {
                let kind = primitive_kind(args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("box")).to_string();
                let Some(mut build) = build_doc(projection, &self.runtime) else {
                    return ActionEmit::default();
                };
                let Ok(new_id) = build.add_primitive(&kind) else {
                    return ActionEmit::default();
                };
                if build.sync_meshes_to_projection().is_err() {
                    return ActionEmit::default();
                }
                let Some(new_object) = build.projection().objects.iter().find(|object| object.id == new_id).cloned() else {
                    return ActionEmit::default();
                };
                let index = projection.objects.len();
                self.runtime.active_object_id = new_id;
                self.runtime.selection = LowpolySelection::default();
                ActionEmit::ops(vec![LowpolyOp::Objects(CollectionOp::Add { index, item: new_object })])
            }
            "patchObject" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned();
                let Some(object) = projection.objects.iter().find(|object| object.id == object_id) else {
                    return ActionEmit::default();
                };
                let patch = match field {
                    "name" => LowpolyObjectPatch {
                        name: value.as_ref().and_then(|entry| entry.as_str()).map(str::to_string),
                        ..Default::default()
                    },
                    "smoothShading" => LowpolyObjectPatch {
                        smooth_shading: Some(value.as_ref().and_then(|entry| entry.as_bool()).unwrap_or(!object.smooth_shading)),
                        ..Default::default()
                    },
                    _ => LowpolyObjectPatch::default(),
                };
                if patch == LowpolyObjectPatch::default() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(vec![LowpolyOp::Objects(CollectionOp::Patch { id: object_id.into(), patch })])
            }
            "extrude" => {
                let distance = tool_param_f32(&self.runtime.tool_params, "extrudeDistance", 0.25);
                self.mesh_edit(projection, move |doc| {
                    let faces = doc.selected_face_ids();
                    if faces.is_empty() {
                        return Err("no faces selected".into());
                    }
                    doc.active_mesh_mut()?.extrude_faces(&faces, distance).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "inset" => {
                let amount = tool_param_f32(&self.runtime.tool_params, "insetAmount", 0.1);
                self.mesh_edit(projection, move |doc| {
                    let faces = doc.selected_face_ids();
                    doc.active_mesh_mut()?.inset_faces(&faces, amount).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "bevel" => {
                let amount = tool_param_f32(&self.runtime.tool_params, "bevelAmount", 0.05);
                let segments = tool_param_u32(&self.runtime.tool_params, "bevelSegments", 1);
                self.mesh_edit(projection, move |doc| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut()?.bevel_edges(&edges, amount, segments).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "loopCut" => {
                let cuts = tool_param_u32(&self.runtime.tool_params, "loopCuts", 1);
                self.mesh_edit(projection, move |doc| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut()?.loop_cut(&edges, cuts).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "subdivide" => self.mesh_edit(projection, move |doc| {
                let faces = doc.selected_face_ids();
                doc.active_mesh_mut()?.subdivide_faces(&faces).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection()
            }),
            "triangulate" => self.mesh_edit(projection, move |doc| {
                doc.active_mesh_mut()?.triangulate().map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection()
            }),
            "mirror" => {
                let axis = mirror_axis_from_param(&self.runtime.tool_params);
                self.mesh_edit(projection, move |doc| {
                    doc.active_mesh_mut()?.mirror(axis, 0.001).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "decimate" => {
                let ratio = tool_param_f32(&self.runtime.tool_params, "decimateRatio", 0.5);
                self.mesh_edit(projection, move |doc| {
                    doc.active_mesh_mut()?.decimate(ratio).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "flipFaces" => {
                let face_ids: Vec<u32> = args
                    .and_then(|value| value.get("faceIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                self.mesh_edit(projection, move |doc| {
                    let faces: Vec<FaceId> = if !face_ids.is_empty() {
                        face_ids.into_iter().map(FaceId).collect()
                    } else if !doc.selected_face_ids().is_empty() {
                        doc.selected_face_ids()
                    } else {
                        doc.selection().ids.iter().map(|id| FaceId(*id)).collect()
                    };
                    doc.active_mesh_mut()?.flip_faces(&faces).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "merge" => self.mesh_edit(projection, move |doc| {
                let verts = doc.selected_vertex_ids();
                doc.active_mesh_mut()?.merge_vertices(&verts, WeldMode::Center, 0.001).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection()
            }),
            "dissolve" => self.mesh_edit(projection, move |doc| {
                let edges = doc.selected_edge_ids();
                doc.active_mesh_mut()?.dissolve_edges(&edges).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection()
            }),
            "snap" => {
                let grid = tool_param_f32(&self.runtime.tool_params, "snapGrid", 0.25);
                self.mesh_edit(projection, move |doc| {
                    let verts = doc.selected_vertex_ids();
                    doc.active_mesh_mut()?.snap_vertices_to_grid(&verts, grid).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection()
                })
            }
            "toggleSmooth" => self.mesh_edit(projection, move |doc| {
                if let Some(index) = doc.active_index() {
                    let smooth = !doc.projection().objects[index].smooth_shading;
                    doc.projection_mut().objects[index].smooth_shading = smooth;
                    let faces: Vec<FaceId> = (0..doc.active_mesh()?.face_count()).map(|index| FaceId(index as u32)).collect();
                    let mesh = doc.active_mesh_mut()?;
                    mesh.set_shading(&faces, smooth).map_err(map_kernel_err)?;
                    mesh.recompute_normals().map_err(map_kernel_err)?;
                }
                doc.sync_meshes_to_projection()
            }),
            "unwrapActive" => self.mesh_edit(projection, move |doc| {
                doc.active_mesh_mut()?.unwrap_uv().map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection()
            }),
            "markUvSeam" => {
                let seam = args.and_then(|value| value.get("seam")).and_then(|value| value.as_bool()).unwrap_or(true);
                let edge_ids: Vec<u32> = args
                    .and_then(|value| value.get("edgeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_else(|| self.runtime.selection.ids.clone());
                self.mesh_edit(projection, move |doc| {
                    let edges: Vec<EdgeId> = edge_ids.into_iter().map(EdgeId).collect();
                    doc.active_mesh_mut()?.mark_uv_seam(&edges, seam);
                    doc.sync_meshes_to_projection()
                })
            }
            "clearSeam" => self.handle_action("markUvSeam", Some(&json!({ "seam": false })), doc, view_state),
            "translateSelection" => {
                let (mode, ids) = selection_args(args);
                let dx = arg_f32(args, "dx", 0.0);
                let dy = arg_f32(args, "dy", 0.0);
                let dz = arg_f32(args, "dz", 0.0);
                self.mesh_edit(projection, move |doc| {
                    if !ids.is_empty() {
                        doc.apply_selection(&mode, ids);
                    }
                    apply_transform(doc, Transform::Translate(Vec3::new(dx, dy, dz)))
                })
            }
            "rotateSelection" => {
                let (mode, ids) = selection_args(args);
                let ax = arg_f32(args, "ax", 0.0);
                let ay = arg_f32(args, "ay", 0.0);
                let az = arg_f32(args, "az", 0.0);
                let angle = arg_f32(args, "angle", 0.0);
                self.mesh_edit(projection, move |doc| {
                    if !ids.is_empty() {
                        doc.apply_selection(&mode, ids);
                    }
                    apply_transform(doc, Transform::Rotate { axis: Vec3::new(ax, ay, az), angle })
                })
            }
            "scaleSelection" => {
                let (mode, ids) = selection_args(args);
                let sx = arg_f32(args, "sx", 1.0);
                let sy = arg_f32(args, "sy", 1.0);
                let sz = arg_f32(args, "sz", 1.0);
                self.mesh_edit(projection, move |doc| {
                    if !ids.is_empty() {
                        doc.apply_selection(&mode, ids);
                    }
                    apply_transform(doc, Transform::Scale(Vec3::new(sx, sy, sz)))
                })
            }
            "setProjectionJson" | "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(parsed) = serde_json::from_str::<LowpolyProjection>(json_text) {
                        return ActionEmit::ops(vec![LowpolyOp::SetProjection { projection: parsed }]);
                    }
                }
                ActionEmit::default()
            }
            "engagementSubmit" => {
                const ENGAGEMENT_COMMANDS: &[&str] =
                    &["extrude", "inset", "bevel", "loopCut", "subdivide", "triangulate", "mirror", "decimate", "flipFaces", "merge", "dissolve", "snap"];
                let typed = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).filter(|value| !value.is_empty());
                if let Some(typed) = typed {
                    if let Some(&resolved) = ENGAGEMENT_COMMANDS.iter().find(|candidate| engagement_token_matches(typed, candidate)) {
                        return self.handle_action(resolved, None, doc, view_state);
                    }
                }
                ActionEmit::default()
            }
            //#endregion ✏️ Object + mesh operations
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, LowpolyProjection>, view_state: &ViewState) -> UiNode {
        let projection = doc.projection;
        let labels = lowpoly_labels(view_state);
        let view = self.view(projection);
        if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
            self.refresh_texture_cache(projection);
        }
        let texture_cache = self.texture_cache.borrow().textures.clone();
        let loaded = matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV | LOWPOLY_PLAY_BODY_DOCUMENT)
            .then(|| build_doc(projection, &self.runtime))
            .flatten();
        match body_key {
            LOWPOLY_PLAY_BODY_MAIN => match &loaded {
                Some(loaded) => build_world_3d_scene(
                    LOWPOLY_PLAY_SURFACE_MAIN,
                    LOWPOLY_PLAY_APP_ID,
                    world3d_scene(
                        lowpoly_world_camera_json(&self.runtime),
                        world_meshes_json(loaded, &texture_cache),
                        world_instances_json(view),
                        world_selection_json_for(view, view_state.active_mode_id.as_deref(), Some(loaded)),
                        &self.runtime.sun,
                    ),
                ),
                None => ui_text("Failed to load lowpoly document"),
            },
            LOWPOLY_PLAY_BODY_UV => match &loaded {
                Some(loaded) => build_canvas_2d_scene(
                    LOWPOLY_PLAY_SURFACE_UV,
                    LOWPOLY_PLAY_APP_ID,
                    Canvas2dScene {
                        camera_x: 0.0,
                        camera_y: 0.0,
                        zoom: 1.0,
                        layers_json: uv_canvas_layers_json(loaded, view, &texture_cache),
                    },
                ),
                None => ui_text("Failed to load UV canvas"),
            },
            LOWPOLY_PLAY_BODY_DOCUMENT => match &loaded {
                Some(loaded) => build_document_tree(view, loaded, labels),
                None => ui_text("Failed to load lowpoly document"),
            },
            LOWPOLY_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            LOWPOLY_PLAY_BODY_INSPECTION => build_inspector_tree(view, labels),
            LOWPOLY_PLAY_BODY_LAYERS => build_layers_tree(view, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn tools(&self, doc: &DocumentView<'_, LowpolyProjection>, view_state: &ViewState) -> Vec<ToolNode> {
        let view = self.view(doc.projection);
        let labels = lowpoly_labels(view_state);
        match view_state.active_mode_id.as_deref() {
            Some("paint") => paint_tools(&self.runtime, labels),
            _ => edit_tools(view, labels),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, LowpolyProjection>, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let engagement = lowpoly_window_engagement(self.view(doc.projection));
        HashMap::from([
            (LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()),
            (LOWPOLY_PLAY_WINDOW_UV.into(), engagement),
        ])
    }

    fn window_measures(&self, _doc: &DocumentView<'_, LowpolyProjection>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let measures = lowpoly_window_measures(&self.runtime);
        HashMap::from([
            (LOWPOLY_PLAY_WINDOW_MAIN.into(), measures.clone()),
            (LOWPOLY_PLAY_WINDOW_UV.into(), measures),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = lowpoly_labels(view_state);
        AppLabelsOverlay {
            app_label: None,
            window_kind_labels: HashMap::from([
                (LOWPOLY_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string()),
                (LOWPOLY_PLAY_WINDOW_UV.to_string(), labels.window_uv.to_string()),
            ]),
            panel_tab_labels: HashMap::new(),
            mode_labels: HashMap::new(),
        }
    }
}

//#region 🔖TransformHelpers
enum Transform {
    Translate(Vec3),
    Rotate { axis: Vec3, angle: f32 },
    Scale(Vec3),
}

fn selection_args(args: Option<&Value>) -> (String, Vec<u32>) {
    let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("mesh").to_string();
    let ids: Vec<u32> = args
        .and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    (mode, ids)
}

fn arg_f32(args: Option<&Value>, key: &str, default: f32) -> f32 {
    args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(default as f64) as f32
}

fn apply_transform(doc: &mut LowpolyDocument, transform: Transform) -> Result<(), String> {
    let selection_mode = doc.selection().mode.clone();
    let pivot = doc.selection_transform_pivot()?;
    let component_verts = match selection_mode.as_str() {
        "vertex" | "face" | "edge" => Some(doc.selection_vertex_ids()?),
        _ => None,
    };
    let component = matches!(selection_mode.as_str(), "vertex" | "face" | "edge");
    let verts = if component {
        let verts = component_verts.ok_or_else(|| "no vertices".to_string())?;
        if verts.is_empty() {
            return Err("no component vertices in selection".into());
        }
        Some(verts)
    } else {
        None
    };
    let mesh = doc.active_mesh_mut()?;
    match transform {
        Transform::Translate(delta) => match &verts {
            Some(verts) => mesh.move_vertices(verts, delta).map_err(map_kernel_err)?,
            None => mesh.translate(delta).map_err(map_kernel_err)?,
        },
        Transform::Rotate { axis, angle } => match &verts {
            Some(verts) => mesh.rotate_vertices(verts, axis, angle, pivot).map_err(map_kernel_err)?,
            None => mesh.rotate(axis, angle).map_err(map_kernel_err)?,
        },
        Transform::Scale(scale) => match &verts {
            Some(verts) => mesh.scale_vertices(verts, scale, pivot).map_err(map_kernel_err)?,
            None => mesh.scale(scale).map_err(map_kernel_err)?,
        },
    }
    doc.sync_meshes_to_projection()
}
//#endregion 🔖TransformHelpers
//#endregion 🔖LowpolyPlayApp

//#region 🔖Manifest
fn default_view_tools() -> Vec<ToolNode> {
    let projection = default_projection();
    let runtime = LowpolyPlayRuntime::default();
    edit_tools(LowpolyView { projection: &projection, runtime: &runtime }, lowpoly_labels(&ViewState::default()))
}

fn default_paint_tools() -> Vec<ToolNode> {
    paint_tools(&LowpolyPlayRuntime::default(), lowpoly_labels(&ViewState::default()))
}

fn create_lowpoly_app() -> App {
    let default_example = serde_json::to_string(&default_projection()).expect("lowpoly default example");
    let engagement = {
        let projection = default_projection();
        let runtime = LowpolyPlayRuntime::default();
        lowpoly_window_engagement(LowpolyView { projection: &projection, runtime: &runtime })
    };
    App::from_builder(
        App::builder(LOWPOLY_PLAY_APP_ID, "Lowpoly")
            .document(["semio", "lowpoly"])
            .icon_id("box")
            .mode("edit", "Edit")
            .mode("paint", "Paint")
            .mode_layout("paint", "lowpoly-paint")
            .default_mode_id("edit")
            .window_kind_with_engagement(LOWPOLY_PLAY_WINDOW_MAIN, "Model", LOWPOLY_PLAY_BODY_MAIN, SurfaceKind::World3d, engagement.clone())
            .window_kind_with_engagement(LOWPOLY_PLAY_WINDOW_UV, "UV", LOWPOLY_PLAY_BODY_UV, SurfaceKind::Canvas2d, engagement)
            .default_layout(create_default_layout(&[LOWPOLY_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Model".into()])))
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
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, LOWPOLY_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, LOWPOLY_PLAY_BODY_CATALOGUE)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, LOWPOLY_PLAY_BODY_INSPECTION)
            .panel_tab("framework.panel.layers", "Layers", PanelGroup::Workbench, LOWPOLY_PLAY_BODY_LAYERS)
            .mode_tools("edit", default_view_tools())
            .mode_tools("paint", default_paint_tools())
            // 🔧 Document-mutating operations — dispatched as VCS ops with true inverses.
            .operation("addPrimitive", "Add Primitive")
            .operation("patchObject", "Patch Object")
            .operation("extrude", "Extrude")
            .operation("inset", "Inset")
            .operation("bevel", "Bevel")
            .operation("loopCut", "Loop Cut")
            .operation("subdivide", "Subdivide")
            .operation("triangulate", "Triangulate")
            .operation("mirror", "Mirror")
            .operation("decimate", "Decimate")
            .operation("flipFaces", "Flip Faces")
            .operation("merge", "Merge")
            .operation("dissolve", "Dissolve")
            .operation("snap", "Snap")
            .operation("toggleSmooth", "Toggle Smooth")
            .operation("unwrapActive", "Unwrap")
            .operation("markUvSeam", "Mark Seam")
            .operation("clearSeam", "Clear Seam")
            .operation("translateSelection", "Translate Selection")
            .operation("rotateSelection", "Rotate Selection")
            .operation("scaleSelection", "Scale Selection")
            .operation("addPaintLayer", "Add Paint Layer")
            .operation("paintStroke", "Paint Stroke")
            .operation("paintAt", "Paint At")
            .operation("canvasPointerDown", "Canvas Pointer Down")
            .operation("canvasPointerMove", "Canvas Pointer Move")
            .operation("paintFill", "Paint Fill")
            .operation("fillBucket", "Fill Bucket")
            .operation("paintSample", "Paint Sample")
            .operation("setProjectionJson", "Set Projection Json")
            .operation("setFixtureJson", "Set Fixture Json")
            .operation("engagementSubmit", "Engagement Submit")
            // 👁️ Ephemeral view state — selection, tools, camera, hover, paint drag.
            .view_action("setActiveObject", "Set Active Object")
            .view_action("setSelection", "Set Selection")
            .view_action("toggleSelectionKind", "Toggle Selection Kind")
            .view_action("toggleSelectionTarget", "Toggle Selection Target")
            .view_action("setTransformTool", "Set Transform Tool")
            .view_action("setPaintTool", "Set Paint Tool")
            .view_action("setActivePaintLayer", "Set Active Paint Layer")
            .view_action("setToolParam", "Set Tool Param")
            .view_action("engagementInput", "Engagement Input")
            .view_action("toggleShowEdges", "Toggle Show Edges")
            .view_action("toggleSun", "Toggle Sun")
            .view_action("setSunAzimuth", "Set Sun Azimuth")
            .view_action("setSunElevation", "Set Sun Elevation")
            .view_action("setSunIntensity", "Set Sun Intensity")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("setCamera", "Set Camera")
            .view_action("worldSelect", "World Select")
            .view_action("worldHover", "World Hover")
            .view_action("setHover", "Set Hover")
            .view_action("worldPick", "World Pick")
            .view_action("paintStrokeBegin", "Paint Stroke Begin")
            .view_action("paintStrokeEnd", "Paint Stroke End")
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("default", "Default", &default_example)
    .program("lowpoly", "Lowpoly", "mesh")
}

fn lowpoly_mesh_from_document(doc: &serde_json::Value) -> Result<MeshData, String> {
    let projection: LowpolyProjection = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    let loaded = LowpolyDocument::new(projection)?;
    Ok(loaded
        .active_mesh()
        .ok()
        .and_then(|mesh| LowpolyDocument::tessellate_transfer_json(mesh).ok())
        .map(|transfer| mesh_data_from_transfer(&transfer, None))
        .unwrap_or_default())
}

/// 🔺 Rebuilds a fresh single-object lowpoly projection from a DWG-imported mesh.
fn lowpoly_document_from_mesh(mesh: &MeshData) -> Result<serde_json::Value, String> {
    let halfedge = kernel_3d_mesh::HalfedgeMesh::from_indexed_triangles(&mesh.positions, &mesh.indices).map_err(|err| format!("{err:?}"))?;
    let mesh_json = halfedge.to_json().map_err(|err| format!("{err:?}"))?;
    let projection = projection_from_mesh_json(&mesh_json, "obj-1", "Imported Mesh");
    serde_json::to_value(projection).map_err(|err| err.to_string())
}

/// 🧊 Minimal document wrapper for `3d.mesh` resources — no dedicated schema exists yet.
fn mesh_document_from_mesh(mesh: &MeshData) -> Result<serde_json::Value, String> {
    let mesh_value = serde_json::to_value(mesh).map_err(|err| err.to_string())?;
    Ok(json!({ "schema": "mesh.document", "mesh": mesh_value }))
}

fn mesh_from_mesh_document(doc: &serde_json::Value) -> Result<MeshData, String> {
    doc.get("mesh")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .filter(|mesh: &MeshData| !mesh.positions.is_empty() && !mesh.indices.is_empty())
        .map(Ok)
        .unwrap_or_else(|| Ok(mesh_from_kind("box")))
}

/// 🔌 One call per `MeshExporter`/`MeshImporter` format so the OS media-graph VFS auto-populates from
/// `required_os_media_export_formats`/`required_os_media_import_formats`.
fn register_lowpoly_exports() {
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", lowpoly_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", lowpoly_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", lowpoly_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.lowpoly", "lowpoly", lowpoly_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.lowpoly", lowpoly_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", lowpoly_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", lowpoly_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.lowpoly", lowpoly_document_from_mesh);
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", mesh_from_mesh_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", mesh_from_mesh_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", mesh_from_mesh_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.mesh", "mesh", mesh_from_mesh_document);
    semio_framework_os::register_mesh_importer("3d.mesh", mesh_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.mesh", mesh_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.mesh", mesh_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.mesh", mesh_document_from_mesh);
}

semio_framework_plugin::semio_plugin! {
    id: "lowpoly", label: "Lowpoly", version: "0.1.0",
    setup: register_lowpoly_exports,
    apps: [ create_lowpoly_app => LowpolyPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
    use vcs::{Backbone, BackboneMessage, MemoryBackbone};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<LowpolyPlayApp> {
        VcsDocumentApp::new(LowpolyPlayApp::default())
    }

    fn projection(app: &VcsDocumentApp<LowpolyPlayApp>) -> LowpolyProjection {
        app.projection().expect("projection")
    }

    fn face_selection() -> Value {
        json!({ "granularity": "face", "id": 0, "merge": "replace" })
    }

    #[test]
    fn renders_world_scene() {
        let mut app = new_app();
        let node = app.render(LOWPOLY_PLAY_BODY_MAIN, None, &ViewState::default()).unwrap();
        assert!(serde_json::to_string(&node).unwrap().contains("world-3d"));
    }

    #[test]
    fn renders_uv_canvas() {
        let mut app = new_app();
        let node = app.render(LOWPOLY_PLAY_BODY_UV, None, &ViewState::default()).unwrap();
        assert!(serde_json::to_string(&node).unwrap().contains("canvas-2d"));
    }

    #[test]
    fn catalogue_lists_primitives() {
        let mut app = new_app();
        let node = app.render(LOWPOLY_PLAY_BODY_CATALOGUE, None, &ViewState::default()).unwrap();
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("lowpoly-play-catalogue.box"));
        assert!(json.contains("Cube"));
        assert!(json.contains("Ico Sphere"));
    }

    #[test]
    fn add_primitive_emits_objects_add_op() {
        let mut app = new_app();
        app.handle_action("addPrimitive", Some(&json!({ "kind": "box" })), &ViewState::default(), &meta("a")).unwrap();
        let projection = projection(&app);
        assert_eq!(projection.objects.len(), 2);
        assert!(projection.objects.iter().any(|object| object.name == "box"));
    }

    #[test]
    fn extrude_selected_face_grows_mesh_and_undo_restores() {
        let mut app = new_app();
        let object_id = projection(&app).objects[0].id.clone();
        let before = LowpolyDocument::new(projection(&app)).unwrap().active_mesh().unwrap().face_count();
        app.handle_action("worldPick", Some(&face_selection()), &ViewState::default(), &meta("a")).unwrap();
        app.handle_action("extrude", None, &ViewState::default(), &meta("a")).unwrap();
        let after = LowpolyDocument::with_context(projection(&app), object_id.clone(), LowpolySelection::default())
            .unwrap()
            .active_mesh()
            .unwrap()
            .face_count();
        assert!(after > before);
        app.handle_action("undo", None, &ViewState::default(), &meta("a")).unwrap();
        let restored = LowpolyDocument::with_context(projection(&app), object_id, LowpolySelection::default())
            .unwrap()
            .active_mesh()
            .unwrap()
            .face_count();
        assert_eq!(restored, before);
    }

    #[test]
    fn selection_is_view_state_and_emits_no_ops() {
        let mut app = new_app();
        let result = app.handle_action("worldPick", Some(&face_selection()), &ViewState::default(), &meta("a")).unwrap();
        assert!(result.operations.is_empty(), "picking must not create an undoable op");
    }

    #[test]
    fn paint_stroke_drag_is_one_undo_step_with_pixel_restoration() {
        let mut app = new_app();
        let object_id = projection(&app).objects[0].id.clone();
        let before = projection(&app).objects[0].paint_layers[0].pixels.clone();
        // begin → tick → tick → end : one undoable PaintStroke edit.
        app.handle_action("paintStrokeBegin", None, &ViewState::default(), &meta("a")).unwrap();
        let tick_a = app
            .handle_action("paintAt", Some(&json!({ "objectId": object_id, "u": 0.5, "v": 0.5 })), &ViewState::default(), &meta("a"))
            .unwrap();
        let tick_b = app
            .handle_action("paintAt", Some(&json!({ "objectId": object_id, "u": 0.52, "v": 0.5 })), &ViewState::default(), &meta("a"))
            .unwrap();
        assert!(tick_a.operations.is_empty() && tick_b.operations.is_empty(), "mid-drag ticks emit no ops");
        let end = app.handle_action("paintStrokeEnd", None, &ViewState::default(), &meta("a")).unwrap();
        assert_eq!(end.operations.len(), 1, "the whole drag commits as one op");
        let painted = projection(&app).objects[0].paint_layers[0].pixels.clone();
        assert_ne!(painted, before, "the stroke changed pixels");
        // ONE undo restores the exact prior pixels.
        app.handle_action("undo", None, &ViewState::default(), &meta("a")).unwrap();
        let restored = projection(&app).objects[0].paint_layers[0].pixels.clone();
        assert_eq!(restored, before, "undo restores the exact pre-stroke pixels");
        // Redo re-applies.
        app.handle_action("redo", None, &ViewState::default(), &meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].paint_layers[0].pixels, painted);
    }

    #[test]
    fn eyedropper_updates_paint_color_without_ops() {
        let mut app = new_app();
        app.handle_action("setPaintTool", Some(&json!({ "tool": "eyedropper" })), &ViewState::default(), &meta("a")).unwrap();
        let result = app.handle_action("paintSample", Some(&json!({ "u": 0.5, "v": 0.5 })), &ViewState::default(), &meta("a")).unwrap();
        assert!(result.operations.is_empty());
    }

    #[test]
    fn toggle_smooth_emits_op_and_flips_shading() {
        let mut app = new_app();
        let before = projection(&app).objects[0].smooth_shading;
        app.handle_action("toggleSmooth", None, &ViewState::default(), &meta("a")).unwrap();
        assert_ne!(projection(&app).objects[0].smooth_shading, before);
    }

    #[test]
    fn add_paint_layer_emits_op() {
        let mut app = new_app();
        let before = projection(&app).objects[0].paint_layers.len();
        app.handle_action("addPaintLayer", Some(&json!({ "name": "Detail" })), &ViewState::default(), &meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].paint_layers.len(), before + 1);
    }

    #[test]
    fn paint_tools_include_history_undo() {
        let mut app = new_app();
        let mut view = ViewState::default();
        view.active_mode_id = Some("paint".into());
        let tools = app.tools(&view);
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("lowpoly-paint-brush"));
        assert!(json.contains("lowpoly-paint-history"));
        assert!(json.contains("\"action\":\"undo\""));
    }

    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        // Both instances start from the identical default projection; disjoint edits (a rename on one,
        // an added primitive on the other) must converge on the shared backbone — impossible under a
        // whole-document snapshot where one write clobbers the other.
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://lowpoly-convergence", "mem://lowpoly-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a
            .handle_action("patchObject", Some(&json!({ "objectId": "obj-1", "field": "name", "value": "Renamed By A" })), &ViewState::default(), &meta("actor-a"))
            .expect("a renames object");
        instance_b
            .handle_action("addPrimitive", Some(&json!({ "kind": "box" })), &ViewState::default(), &meta("actor-b"))
            .expect("b adds primitive");

        // A neutral history action pumps inbound ops before doing anything else.
        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        assert_eq!(projection_a.objects.len(), 2, "A converges on B's added primitive");
        assert_eq!(projection_b.objects.len(), 2, "B converges on A's rename base");
        assert!(projection_a.objects.iter().any(|object| object.name == "Renamed By A"), "A keeps its rename");
        assert!(projection_b.objects.iter().any(|object| object.name == "Renamed By A"), "B converges on A's rename");
        assert!(projection_a.objects.iter().any(|object| object.name == "box"), "A converges on B's primitive");
        assert!(projection_b.objects.iter().any(|object| object.name == "box"), "B keeps its primitive");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        let mut sender = new_app();
        let (near, mut far) = MemoryBackbone::pair("mem://lowpoly-doc", "mem://lowpoly-doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        sender
            .handle_action("patchObject", Some(&json!({ "objectId": "obj-1", "field": "name", "value": "Hero" })), &ViewState::default(), &meta("local"))
            .expect("rename");

        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Ops { envelopes: ops } = message {
                envelopes.extend(ops);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied op to flow onto the channel");
        let operations_json = serde_json::to_string(&envelopes).expect("serialize envelopes");

        let mut receiver = new_app();
        receiver.ingest_operations(&operations_json).expect("ingest once");
        receiver.ingest_operations(&operations_json).expect("ingest twice");
        assert!(receiver.projection().expect("projection").objects.iter().any(|object| object.name == "Hero"));
    }
}
//#endregion 🧪Tests
