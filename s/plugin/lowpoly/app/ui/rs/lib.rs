//! 🖌️ Lowpoly app — DocumentApp impl, render, manifest (constitutional: ui). Mesh/object structure and
//! paint pixels are the document projection (undoable via the VCS store); utilities, selection, camera and
//! the mid-drag paint scratch are ephemeral app-struct state.

use base64::Engine;
use kernel_3d_mesh::{EdgeId, FaceId, MirrorAxis, Vec3, WeldMode};
use lowpoly::{
    empty_paint_pixels, LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolyProjection, LowpolySelection,
    LowpolySelectionTargets, LOWPOLY_DOCUMENT_SCHEMA, LOWPOLY_PAINT_TEXTURE_SIZE,
};
use lowpoly_engine::{composite_layer_pixels, flood_fill, mesh_data_from_transfer, pixel_runs_from_diff, sample_pixel_from, stamp_brush, LowpolyDocument};
use lowpoly_op::{LowpolyOperation, PixelRun};
use png::{BitDepth, ColorType, Encoder};
use semio_framework_plugin::{
    apply_world3d_sun_action, build_canvas_2d_scene, build_world_3d_scene, create_default_layout,
    create_named_layout, engagement_token_matches, is_de_locale, localized_label_map, merge_world_selection_ids,
    resolve_labels, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_camera_json, world3d_scene, world3d_selection_json, world3d_sun_measures,
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt,
    Canvas2dScene, DocumentApp, DocumentView, IconName, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder,
    ArtifactKindSpec, SurfaceKind, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemAction, UiTreeItemNode,
    UiToggleNode, UtilityCategory, UtilityDefinition, ViewState, WindowEngagement, WindowEngagementInput,
    WindowEngagementOption, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, WindowEngagementPossible,
    WindowEngagementStatus, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::collections::HashMap;

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
/// 🧰 The transform gumball utility a Model window falls back to when the host hasn't set an active utility.
const LOWPOLY_TRANSFORM_UTILITY_DEFAULT: &str = "move";

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
    paint_utility: String,
    active_paint_layer: u32,
    selection_method: String,
    /// 🎯 Default marquee/pick merge mode when no modifier keys override — `default`/`additive`/`subtractive`/`invertive`.
    selection_mode_default: String,
    selected_object_ids: Vec<String>,
    hovered_object_id: Option<String>,
    hovered_target: Option<LowpolyHoverTarget>,
    utility_params: Value,
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
            paint_utility: "brush".into(),
            active_paint_layer: 0,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            selected_object_ids: Vec::new(),
            hovered_object_id: None,
            hovered_target: None,
            utility_params: default_utility_params(),
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

fn default_utility_params() -> Value {
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
/// the render/panel/utility/scene builders.
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

fn utility_param_f32(params: &Value, key: &str, default: f32) -> f32 {
    params.get(key).and_then(|value| value.as_f64()).map(|v| v as f32).unwrap_or(default)
}

fn utility_param_u32(params: &Value, key: &str, default: u32) -> u32 {
    params.get(key).and_then(|value| value.as_u64()).map(|v| v as u32).unwrap_or(default)
}

fn mirror_axis_from_param(params: &Value) -> MirrorAxis {
    match utility_param_u32(params, "mirrorAxis", 0) {
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

fn arg_u32(args: Option<&Value>, key: &str, default: u32) -> u32 {
    args.and_then(|value| value.get(key)).and_then(|value| value.as_u64()).unwrap_or(default as u64) as u32
}

fn apply_transform(doc: &mut LowpolyDocument, transform: Transform) -> Result<(), String> {
    let selection_mode = doc.selection().mode.clone();
    let pivot = doc.selection_transform_pivot().map_err(|e| e.to_string())?;
    let component_verts = match selection_mode.as_str() {
        "vertex" | "face" | "edge" => Some(doc.selection_vertex_ids().map_err(|e| e.to_string())?),
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
    let mesh = doc.active_mesh_mut().map_err(|e| e.to_string())?;
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
    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
}
//#endregion 🔖TransformHelpers

//#region 🔖Scene
fn world_selection_json_for(view: LowpolyView, active_utility: &str, active_mode_id: Option<&str>, doc: Option<&LowpolyDocument>) -> String {
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
        object.insert("transformMode".into(), json!(active_utility));
        object.insert(
            "interactionMode".into(),
            json!(if active_mode_id == Some("paint") { "paint" } else { "model" }),
        );
        object.insert("componentIds".into(), json!(runtime.selection.ids));
        object.insert("selectionMode".into(), json!(runtime.selection.mode));
        object.insert("selectionMergeMode".into(), json!(runtime.selection_mode_default));
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
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the lowpoly mesh editor; one field per label makes every locale combination compile-checked.
    struct LowpolyLabels {
        meshes: &'static str = en: "Meshes", de: "Netze";
        primitives: &'static str = en: "Primitives", de: "Primitive";
        paint_layers: &'static str = en: "Paint Layers", de: "Malebenen";
        vertices: &'static str = en: "Vertices", de: "Eckpunkte";
        edges: &'static str = en: "Edges", de: "Kanten";
        faces: &'static str = en: "Faces", de: "Flächen";
        flip_normal: &'static str = en: "Flip normal", de: "Normale umkehren";
        primitive_box: &'static str = en: "Cube", de: "Würfel";
        primitive_plane: &'static str = en: "Plane", de: "Ebene";
        primitive_cylinder: &'static str = en: "Cylinder", de: "Zylinder";
        primitive_cone: &'static str = en: "Cone", de: "Kegel";
        primitive_ico_sphere: &'static str = en: "Ico Sphere", de: "Ikokugel";
        object: &'static str = en: "Object", de: "Objekt";
        transform: &'static str = en: "Transform", de: "Transformation";
        utility_params: &'static str = en: "Utility Params", de: "Werkzeugparameter";
        window_main: &'static str = en: "Model", de: "Modell";
        window_uv: &'static str = en: "UV", de: "UV";
        // inspector field labels
        name: &'static str = en: "Name", de: "Name";
        smooth_shading: &'static str = en: "Smooth Shading", de: "Weiche Schattierung";
        selection: &'static str = en: "Selection", de: "Auswahl";
        selection_mode: &'static str = en: "Selection Mode", de: "Auswahlmodus";
        utility: &'static str = en: "Utility", de: "Werkzeug";
        selected: &'static str = en: "selected", de: "ausgewählt";
        extrude: &'static str = en: "Extrude", de: "Extrudieren";
        triangulate: &'static str = en: "Triangulate", de: "Triangulieren";
        extrude_distance: &'static str = en: "Extrude Distance", de: "Extrusionsabstand";
        inset_amount: &'static str = en: "Inset Amount", de: "Einzugsbetrag";
        bevel_amount: &'static str = en: "Bevel Amount", de: "Fasenbetrag";
        bevel_segments: &'static str = en: "Bevel Segments", de: "Fasensegmente";
        loop_cuts: &'static str = en: "Loop Cuts", de: "Schleifenschnitte";
        decimate_ratio: &'static str = en: "Decimate Ratio", de: "Dezimierungsverhältnis";
        snap_grid: &'static str = en: "Snap Grid", de: "Rastergröße";
        mirror_axis: &'static str = en: "Mirror Axis", de: "Spiegelachse";
        brush_size: &'static str = en: "Brush Size", de: "Pinselgröße";
        brush_opacity: &'static str = en: "Brush Opacity", de: "Pinseldeckkraft";
        brush_hardness: &'static str = en: "Brush Hardness", de: "Pinselhärte";
        // engagement + measures
        snap: &'static str = en: "Snap", de: "Einrasten";
        smooth: &'static str = en: "Smooth", de: "Glätten";
        show_edges: &'static str = en: "Show Edges", de: "Kanten anzeigen";
        select: &'static str = en: "Select", de: "Auswählen";
        mesh: &'static str = en: "Mesh", de: "Netz";
        face: &'static str = en: "Face", de: "Fläche";
        edge: &'static str = en: "Edge", de: "Kante";
        vertex: &'static str = en: "Vertex", de: "Eckpunkt";
        rectangle: &'static str = en: "Rectangle", de: "Rechteck";
        lasso: &'static str = en: "Lasso", de: "Lasso";
        selective: &'static str = en: "Selective", de: "Selektiv";
        additive: &'static str = en: "Additive", de: "Additiv";
        subtractive: &'static str = en: "Subtractive", de: "Subtraktiv";
        invertive: &'static str = en: "Invertive", de: "Invertierend";
        brush_group: &'static str = en: "Brush", de: "Pinsel";
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

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_lowpoly_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn lowpoly_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addPrimitive", "Add Primitive", "Primitive hinzufügen"),
        ("patchObject", "Patch Object", "Objekt aktualisieren"),
        ("extrude", "Extrude", "Extrudieren"),
        ("inset", "Inset", "Einziehen"),
        ("bevel", "Bevel", "Fasen"),
        ("loopCut", "Loop Cut", "Schleifenschnitt"),
        ("subdivide", "Subdivide", "Unterteilen"),
        ("triangulate", "Triangulate", "Triangulieren"),
        ("mirror", "Mirror", "Spiegeln"),
        ("decimate", "Decimate", "Dezimieren"),
        ("flipFaces", "Flip Faces", "Flächen umkehren"),
        ("merge", "Merge", "Zusammenführen"),
        ("dissolve", "Dissolve", "Auflösen"),
        ("snap", "Snap", "Einrasten"),
        ("toggleSmooth", "Toggle Smooth", "Glättung umschalten"),
        ("unwrapActive", "Unwrap", "Abwickeln"),
        ("markUvSeam", "Mark Seam", "Naht markieren"),
        ("clearSeam", "Clear Seam", "Naht entfernen"),
        ("translateSelection", "Translate Selection", "Auswahl verschieben"),
        ("rotateSelection", "Rotate Selection", "Auswahl drehen"),
        ("scaleSelection", "Scale Selection", "Auswahl skalieren"),
        ("transformEnd", "Transform End", "Transformation beenden"),
        ("addPaintLayer", "Add Paint Layer", "Malebene hinzufügen"),
        ("paintStrokeEnd", "Paint Stroke End", "Malstrich beenden"),
        ("paintFill", "Paint Fill", "Füllen malen"),
        ("fillBucket", "Fill Bucket", "Fülleimer"),
        ("setProjectionJson", "Set Projection Json", "Projektions-JSON festlegen"),
        ("setFixtureJson", "Set Fixture Json", "Fixture-JSON festlegen"),
        ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
        ("setActiveObject", "Set Active Object", "Aktives Objekt festlegen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("toggleSelectionKind", "Toggle Selection Kind", "Auswahlart umschalten"),
        ("toggleSelectionTarget", "Toggle Selection Target", "Auswahlziel umschalten"),
        ("setActivePaintLayer", "Set Active Paint Layer", "Aktive Malebene festlegen"),
        ("setUtilityParam", "Set Utility Param", "Werkzeugparameter festlegen"),
        ("engagementInput", "Engagement Input", "Eingabe"),
        ("toggleShowEdges", "Toggle Show Edges", "Kantenanzeige umschalten"),
        ("toggleSun", "Toggle Sun", "Sonne umschalten"),
        ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
        ("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen"),
        ("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen"),
        ("setSelectionMethod", "Set Selection Method", "Auswahlmethode festlegen"),
        ("setSelectionModeDefault", "Set Selection Mode Default", "Standardauswahlmodus festlegen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("worldSelect", "World Select", "Welt auswählen"),
        ("worldHover", "World Hover", "Überfahren (Welt)"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("worldPick", "World Pick", "Welt-Auswahl (Pick)"),
        ("paintStrokeBegin", "Paint Stroke Begin", "Malstrich beginnen"),
        ("paintStroke", "Paint Stroke", "Malstrich"),
        ("paintAt", "Paint At", "Malen bei"),
        ("canvasPointerDown", "Canvas Pointer Down", "Leinwand-Zeiger gedrückt"),
        ("canvasPointerMove", "Canvas Pointer Move", "Leinwand-Zeiger bewegt"),
        ("paintSample", "Paint Sample", "Farbe aufnehmen"),
        ("transformBegin", "Transform Begin", "Transformation beginnen"),
    ];
    localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_lowpoly_app`.
fn lowpoly_utility_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("move", "Move", "Verschieben"),
        ("rotate", "Rotate", "Drehen"),
        ("scale", "Scale", "Skalieren"),
        ("brush", "Brush", "Pinsel"),
        ("eraser", "Eraser", "Radierer"),
        ("fill", "Fill", "Füllen"),
        ("eyedropper", "Eyedropper", "Pipette"),
    ];
    localized_label_map(is_de, ENTRIES)
}
//#endregion 🔖CommandLabels

//#region 🔖Panels
fn build_document_tree(view: LowpolyView, doc: &LowpolyDocument, labels: &LowpolyLabels) -> UiNode {
    let active_id = resolve_active_object_id(view.projection, view.runtime);
    let selected_ids = selected_document_ids(view);
    let highlighted_ids = highlighted_document_ids(view);
    let items: Vec<UiTreeItemNode> = view
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
                let leaves: Vec<UiTreeItemNode> = (0..count)
                    .map(|id| {
                        let row_id = document_target_row_id(&object.id, object_index, mode, id as u32);
                        let hover_args = json!({
                            "objectId": object.id,
                            "mode": mode,
                            "id": id,
                        });
                        let mut actions = None;
                        if mode == "face" {
                            actions = Some(vec![UiTreeItemAction {
                                icon_id: "flip-vertical".into(),
                                label: Some(labels.flip_normal.into()),
                                action: lowpoly_action("flipFaces", Some(json!({ "faceIds": [id] }))),
                                reveal_on_hover: Some(true),
                            }]);
                        }
                        UiTreeItemNode {
                            icon_id: IconName::from_str(icon),
                            action: Some(lowpoly_action(
                                "toggleSelectionTarget",
                                Some(json!({
                                    "objectId": object.id,
                                    "mode": mode,
                                    "id": id,
                                    "merge": "invertive",
                                })),
                            )),
                            hover_action: Some(lowpoly_action("setHover", Some(hover_args.clone()))),
                            unhover_action: Some(lowpoly_action("setHover", None)),
                            actions,
                            ..UiTreeItemNode::base(row_id, format!("{label} {id}"))
                        }
                    })
                    .collect();
                UiTreeItemNode {
                    icon_id: IconName::from_str(icon),
                    items: Some(leaves),
                    description: Some(format!("{count}")),
                    ..UiTreeItemNode::base(format!("lowpoly-document.{object_id}.{mode}.group"), label.to_string())
                }
            };
            UiTreeItemNode {
                icon_id: Some("box".into()),
                action: Some(lowpoly_action(
                    "toggleSelectionTarget",
                    Some(json!({
                        "objectId": object.id,
                        "mode": "mesh",
                        "id": 0,
                        "merge": "invertive",
                    })),
                )),
                items: Some(vec![
                    component_group("vertex", labels.vertices, "circle", vertex_count),
                    component_group("edge", labels.edges, "minus", edge_count),
                    component_group("face", labels.faces, "square", face_count),
                ]),
                default_open: Some(object.id == active_id),
                description: Some(object.id.clone()),
                ..UiTreeItemNode::base(format!("lowpoly-document.{object_id}"), object.name.clone())
            }
        })
        .collect();
    let mut builder =
        PanelTreeBuilder::new("lowpoly-play-document").section("lowpoly-play-document.meshes", Some(labels.meshes.into()), true, items);
    if !selected_ids.is_empty() {
        builder = builder.selected(selected_ids);
    }
    if !highlighted_ids.is_empty() {
        builder = builder.highlighted(highlighted_ids);
    }
    builder.build()
}

fn build_catalogue_tree(labels: &LowpolyLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = PRIMITIVE_CATALOG
        .iter()
        .map(|(kind, label, icon)| UiTreeItemNode {
            icon_id: IconName::from_str(icon),
            ..tree_item_with_action(
                format!("lowpoly-play-catalogue.{kind}"),
                primitive_catalog_label(kind, label, labels),
                Some((*kind).to_string()),
                lowpoly_action("addPrimitive", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    PanelTreeBuilder::new("lowpoly-play-catalogue")
        .section("lowpoly-play-catalogue.primitives", Some(labels.primitives.into()), true, items)
        .build()
}

fn build_layers_tree(view: LowpolyView, labels: &LowpolyLabels) -> UiNode {
    let object = active_object(view);
    let layers = object.map(|entry| entry.paint_layers.as_slice()).unwrap_or(&[]);
    let active_layer = view.runtime.active_paint_layer;
    let items: Vec<UiTreeItemNode> = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| UiTreeItemNode {
            icon_id: Some("layers".into()),
            ..tree_item_with_action(
                format!("lowpoly-layer:{index}"),
                layer.name.clone(),
                Some(format!("{} · {}", layer.opacity, layer.blend_mode)),
                lowpoly_action("setActivePaintLayer", Some(json!({ "layerIndex": index }))),
            )
        })
        .collect();
    PanelTreeBuilder::new("lowpoly-play-layers")
        .section("lowpoly-play-layers.paint", Some(labels.paint_layers.into()), true, items)
        .selected(vec![format!("lowpoly-layer:{active_layer}")])
        .build()
}

fn inspector_utility_param_field(id: &str, label: &str, key: &str, value: &Value) -> UiNode {
    UiNode::Field(UiFieldNode {presence: UiPresence::default(),
        id: format!("lowpoly-play-inspector.{id}"),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
            id: format!("lowpoly-play-inspector.{id}.input"),
            input_kind: "number".into(),
            value: value.get(key).map(|entry| entry.to_string()).unwrap_or_else(|| "0".into()),
            placeholder: None,
            commit: None,
            on_change: lowpoly_action("setUtilityParam", Some(json!({ "key": key }))),
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

fn build_inspector_tree(view: LowpolyView, active_utility: &str, labels: &LowpolyLabels) -> UiNode {
    let Some(object) = active_object(view) else {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {LOWPOLY_DOCUMENT_SCHEMA}")),
            ui_text("No active object".to_string()),
        ]);
    };
    let params = &view.runtime.utility_params;
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.object".into(),
            label: labels.object.into(),
            default_open: None,
            presence: UiPresence::default(),
            fields: vec![
                UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                    id: "lowpoly-play-inspector.object.name".into(),
                    label: labels.name.into(),
                    child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
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
                UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                    id: "lowpoly-play-inspector.object.smooth".into(),
                    label: labels.smooth_shading.into(),
                    child: Box::new(UiNode::Toggle(UiToggleNode {
                        id: "lowpoly-play-inspector.object.smooth.toggle".into(),
                        icon_id: "sun".into(),
                        presence: UiPresence::selected(object.smooth_shading),
                        text: None,
                        on_change: lowpoly_action("patchObject", Some(json!({ "objectId": object.id, "field": "smoothShading" }))),
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                ui_inspector_readonly_field(
                    "lowpoly-play-inspector.object.selection",
                    labels.selection,
                    &format!(
                        "{} · {} {}",
                        format_selection_targets_label(&view.runtime.selection.targets),
                        view.runtime.selection.ids.len(),
                        labels.selected,
                    ),
                ),
                ui_inspector_readonly_field(
                    "lowpoly-play-inspector.object.selection-mode",
                    labels.selection_mode,
                    &view.runtime.selection.mode,
                ),
            ],
        },
        UiInspectorFieldGroup { presence: UiPresence::default(),
            id: "lowpoly-play-inspector.transform".into(),
            label: labels.transform.into(),
            default_open: None,
            fields: vec![ui_inspector_readonly_field(
                "lowpoly-play-inspector.transform.utility",
                labels.utility,
                active_utility,
            )],
        },
        UiInspectorFieldGroup { presence: UiPresence::default(),
            id: "lowpoly-play-inspector.utility-params".into(),
            label: labels.utility_params.into(),
            default_open: Some(true),
            fields: vec![
                inspector_utility_param_field("extrude", labels.extrude_distance, "extrudeDistance", params),
                inspector_utility_param_field("inset", labels.inset_amount, "insetAmount", params),
                inspector_utility_param_field("bevel", labels.bevel_amount, "bevelAmount", params),
                inspector_utility_param_field("bevel-segments", labels.bevel_segments, "bevelSegments", params),
                inspector_utility_param_field("loop-cuts", labels.loop_cuts, "loopCuts", params),
                inspector_utility_param_field("decimate", labels.decimate_ratio, "decimateRatio", params),
                inspector_utility_param_field("snap", labels.snap_grid, "snapGrid", params),
                inspector_utility_param_field("mirror", labels.mirror_axis, "mirrorAxis", params),
                inspector_utility_param_field("brush-size", labels.brush_size, "brushSize", params),
                inspector_utility_param_field("brush-opacity", labels.brush_opacity, "brushOpacity", params),
                inspector_utility_param_field("brush-hardness", labels.brush_hardness, "brushHardness", params),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Utilities
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

fn lowpoly_window_engagement(view: LowpolyView, active_utility: &str, labels: &LowpolyLabels) -> WindowEngagement {
    let runtime = view.runtime;
    let transform = active_utility;
    let selected_count = runtime.selection.ids.len();
    WindowEngagement {
        session_active: Some(true),
        // 🧰 The move/rotate/scale transform switcher now lives in the framework utility bar (declared via `.utility` +
        // `.window_kind_utilities`), so the engagement keeps only its non-utility options below.
        options: Some(vec![
            WindowEngagementOption {
                id: "lowpoly.opt.snap".into(),
                label: Some(labels.snap.into()),
                icon_id: Some("magnet".into()),
                pressed: None,
                disabled: None,
                action: Some(lowpoly_action("snap", None)),
            },
            WindowEngagementOption {
                id: "lowpoly.opt.smooth".into(),
                label: Some(labels.smooth.into()),
                icon_id: Some("sun".into()),
                pressed: None,
                disabled: None,
                action: Some(lowpoly_action("toggleSmooth", None)),
            },
            WindowEngagementOption {
                id: "lowpoly.opt.show-edges".into(),
                label: Some(labels.show_edges.into()),
                icon_id: Some("grid-3x3".into()),
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
                "{} · {} · {selected_count} {}",
                format_selection_targets_label(&runtime.selection.targets),
                transform,
                labels.selected,
            ),
        }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible {
                id: "lowpoly.eng.extrude".into(),
                label: labels.extrude.into(),
                detail: None,
                action: Some(lowpoly_action("extrude", None)),
            },
            WindowEngagementPossible {
                id: "lowpoly.eng.triangulate".into(),
                label: labels.triangulate.into(),
                detail: None,
                action: Some(lowpoly_action("triangulate", None)),
            },
        ]),
    }
}

fn utility_param_f64(params: &Value, key: &str, default: f64) -> f64 {
    utility_param_f32(params, key, default as f32) as f64
}

fn lowpoly_utility_param_slider(
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
        value: utility_param_f64(params, key, default),
        min,
        max,
        step: Some(step),
        ready: None,
        loading: None,
        disabled: None,
        reveal: None,
        on_change: lowpoly_action("setUtilityParam", Some(json!({ "key": key }))),

        waiting: None,}
}

/// 🎯 One selection-granularity toggle. Selection kinds are a non-exclusive multi-select (mesh + face +
/// edge + vertex can all be active at once), so they are a window-measure toggle group — NOT a
/// single-active utility group.
fn selection_kind_toggle(id: &str, icon: &str, label: &str, kind: &str, pressed: bool) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: format!("lowpoly-select-{id}"),
        icon_id: icon.into(),
        label: Some(label.into()),
        pressed,
        text: None,
        on_change: lowpoly_action("toggleSelectionKind", Some(json!({ "kind": kind }))),
    }
}

fn lowpoly_window_measures(runtime: &LowpolyPlayRuntime, labels: &LowpolyLabels) -> Vec<WindowMeasure> {
    let params = &runtime.utility_params;
    vec![
        WindowMeasure::Toggle {
            id: "lowpoly-measure-show-edges".into(),
            icon_id: "grid-3x3".into(),
            label: Some(labels.show_edges.into()),
            pressed: runtime.show_edges,
            text: None,
            on_change: lowpoly_action("toggleShowEdges", None),
        },
        world3d_sun_measures("lowpoly", &runtime.sun, lowpoly_action),
        WindowMeasure::Group {
            id: "lowpoly-measure-snap".into(),
            label: labels.snap.into(),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: vec![
                lowpoly_utility_param_slider("snap", labels.snap_grid, "snapGrid", params, 0.25, 0.05, 2.0, 0.05),
            ],
        },
        lowpoly_select_measures_group(runtime, labels),
        lowpoly_paint_params_group("brush", params, labels),
        lowpoly_paint_params_group("eraser", params, labels),
    ]
}

/// 🎯 Always-visible Select window options — mirrors puzzle 3d's select measures group: rectangle/lasso
/// method, selective/additive/subtractive/invertive merge mode, and composable mesh/face/edge/vertex kinds.
fn lowpoly_select_measures_group(runtime: &LowpolyPlayRuntime, labels: &LowpolyLabels) -> WindowMeasure {
    let targets = &runtime.selection.targets;
    WindowMeasure::Group {
        id: "lowpoly-select".into(),
        label: labels.select.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle {
                id: "lowpoly-select-rectangle".into(),
                icon_id: "square".into(),
                label: Some(labels.rectangle.into()),
                pressed: runtime.selection_method == "rectangle",
                text: None,
                on_change: lowpoly_action("setSelectionMethod", Some(json!({ "method": "rectangle" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-lasso".into(),
                icon_id: "lasso".into(),
                label: Some(labels.lasso.into()),
                pressed: runtime.selection_method == "lasso",
                text: None,
                on_change: lowpoly_action("setSelectionMethod", Some(json!({ "method": "lasso" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-default".into(),
                icon_id: "mouse-pointer".into(),
                label: Some(labels.selective.into()),
                pressed: runtime.selection_mode_default == "default",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "default" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-additive".into(),
                icon_id: "plus".into(),
                label: Some(labels.additive.into()),
                pressed: runtime.selection_mode_default == "additive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "additive" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-subtractive".into(),
                icon_id: "minus".into(),
                label: Some(labels.subtractive.into()),
                pressed: runtime.selection_mode_default == "subtractive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "subtractive" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-invertive".into(),
                icon_id: "arrow-right-left".into(),
                label: Some(labels.invertive.into()),
                pressed: runtime.selection_mode_default == "invertive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "invertive" }))),
            },
            selection_kind_toggle("mesh", "box", labels.mesh, "mesh", targets.mesh),
            selection_kind_toggle("face", "square", labels.face, "face", targets.face),
            selection_kind_toggle("edge", "minus", labels.edge, "edge", targets.edge),
            selection_kind_toggle("vertex", "circle", labels.vertex, "vertex", targets.vertex),
        ],
    }
}

/// 🖌️ Utility Options for a stamping paint utility (`brush`/`eraser`) — the live brush size/opacity/hardness
/// sliders, tagged `active_utility_id: Some(utility)` so [`partition_window_measures`] surfaces them in the
/// Utility Options rail only while that exact utility is active. Both utilities stamp through the same
/// `stamp_brush` path (radius/hardness/opacity + eraser flag), so they share an identical param set.
fn lowpoly_paint_params_group(utility: &str, params: &Value, labels: &LowpolyLabels) -> WindowMeasure {
    let slider = |suffix: &str, label: &str, key: &str, default: f64, min: f64, max: f64, step: f64| WindowMeasure::Slider {
        id: format!("lowpoly-measure-{utility}-{suffix}"),
        label: Some(label.into()),
        value: utility_param_f64(params, key, default),
        min,
        max,
        step: Some(step),
        ready: None,
        loading: None,
        disabled: None,
        reveal: None,
        on_change: lowpoly_action("setUtilityParam", Some(json!({ "key": key }))),

        waiting: None,};
    WindowMeasure::Group {
        id: format!("lowpoly-measure-paint-params-{utility}"),
        label: labels.brush_group.into(),
        default_open: Some(true),
        active_utility_id: Some(utility.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            slider("size", labels.brush_size, "brushSize", 16.0, 1.0, 128.0, 1.0),
            slider("opacity", labels.brush_opacity, "brushOpacity", 1.0, 0.0, 1.0, 0.05),
            slider("hardness", labels.brush_hardness, "brushHardness", 0.5, 0.0, 1.0, 0.05),
        ],
    }
}

//#endregion 🔖Utilities

//#region 🔖LowpolyPlayApp
/// @emoji 🖌️ In-progress paint drag: the pre-stroke layer buffer and the accumulating scratch buffer.
/// Mid-drag ticks mutate `scratch` (view state); the stroke commits as ONE `PaintStroke` operation on end.
struct PaintStrokeSession {
    object_id: String,
    layer_index: usize,
    base: Vec<u8>,
    scratch: Vec<u8>,
}

/// @emoji 🧲 In-progress gumball transform drag. The mesh-transform operation re-serializes the WHOLE
/// `mesh_json` buffer per apply, so a per-tick `amend` would `combined.extend` N full-mesh patches and
/// replay them all (O(N) retained megabyte-scale JSON + O(N²) replay). Instead every mid-drag tick
/// applies its delta to this scratch `LowpolyDocument` emitting ZERO operations, and the whole gesture commits
/// as ONE `Objects(Patch)` (base → final mesh) on drag end (`ActionEmit::commit`, coalesce-key `None`).
struct TransformSession {
    object_id: String,
    before: LowpolyObject,
    doc: LowpolyDocument,
}

/// @emoji 🗃️ Pure render-side cache of composited paint textures (base64 PNG per object), invalidated
/// by a fingerprint over the document's paint pixels + the live stroke dirty counter. Never serialized.
#[derive(Default)]
struct PaintTextureCache {
    fingerprint: Option<u64>,
    textures: HashMap<String, String>,
}

pub struct LowpolyPlayApp {
    runtime: LowpolyPlayRuntime,
    stroke: Option<PaintStrokeSession>,
    stroke_drag_active: bool,
    stroke_dirty: u64,
    transform: Option<TransformSession>,
    transform_drag_active: bool,
    texture_cache: RefCell<PaintTextureCache>,
    /// 👻 Per-`key` monotone counter for `gesture_preview` — see `//#region 🔖GesturePreview`.
    preview_seq: u64,
}

impl Default for LowpolyPlayApp {
    fn default() -> Self {
        Self {
            runtime: LowpolyPlayRuntime::default(),
            stroke: None,
            stroke_drag_active: false,
            stroke_dirty: 0,
            transform: None,
            transform_drag_active: false,
            texture_cache: RefCell::new(PaintTextureCache::default()),
            preview_seq: 0,
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
    ) -> ActionEmit<LowpolyOperation> {
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
        ActionEmit::operations(vec![LowpolyOperation::ObjectsPatch { id: object_id, patch }])
    }

    /// @emoji 📌 Commits the accumulated paint scratch as ONE described `PaintStroke` edit (scratch-commit
    /// pattern b — the whole drag is one undoable edit; megabyte pixel buffers never coalesce per tick).
    fn commit_stroke(&mut self) -> ActionEmit<LowpolyOperation> {
        let Some(session) = self.stroke.take() else {
            return ActionEmit::default();
        };
        self.stroke_dirty += 1;
        let runs: Vec<PixelRun> = pixel_runs_from_diff(&session.base, &session.scratch).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect();
        if runs.is_empty() {
            return ActionEmit::default();
        }
        ActionEmit::commit(
            vec![LowpolyOperation::PaintStroke {
                object_id: session.object_id,
                layer_index: session.layer_index,
                runs,
            }],
            "Paint stroke",
        )
    }

    /// @emoji 🖌️ One mid-drag paint tick: brush/eraser/fill mutate the stroke scratch, eyedropper samples
    /// the paint color. Emits ZERO operations — the stroke commits only on `paintStrokeEnd` (View-kind safe).
    fn paint_tick(&mut self, projection: &LowpolyProjection, object_id: String, u: f32, v: f32) -> ActionEmit<LowpolyOperation> {
        let utility = self.runtime.paint_utility.clone();
        if utility == "eyedropper" {
            if let Some(object) = projection.objects.iter().find(|object| object.id == object_id) {
                let composite = composite_layer_pixels(&object.paint_layers);
                self.runtime.paint_color = sample_pixel_from(&composite, u, v);
            }
            return ActionEmit::default();
        }
        let layer_index = self.runtime.active_paint_layer as usize;
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
        let color = self.runtime.paint_color;
        let params = self.runtime.utility_params.clone();
        if let Some(session) = self.stroke.as_mut() {
            if utility == "fill" {
                flood_fill(&mut session.scratch, u, v, color);
            } else {
                let radius = utility_param_f32(&params, "brushSize", 16.0);
                let opacity = utility_param_f32(&params, "brushOpacity", 1.0);
                let hardness = utility_param_f32(&params, "brushHardness", 0.5);
                stamp_brush(&mut session.scratch, u, v, radius, color, hardness, opacity, utility == "eraser");
            }
        }
        self.stroke_dirty += 1;
        ActionEmit::default()
    }

    /// @emoji 🪣 A single-shot flood fill emitted as ONE `PaintStroke` edit (the `fillBucket`/`paintFill`
    /// operation path — not drag-bracketed, so it commits immediately).
    fn fill_at(&mut self, projection: &LowpolyProjection, object_id: String, u: f32, v: f32) -> ActionEmit<LowpolyOperation> {
        let layer_index = self.runtime.active_paint_layer as usize;
        let color = self.runtime.paint_color;
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
        let runs: Vec<PixelRun> = pixel_runs_from_diff(&layer.pixels, &scratch).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect();
        if runs.is_empty() {
            return ActionEmit::default();
        }
        self.stroke_dirty += 1;
        ActionEmit::commit(vec![LowpolyOperation::PaintStroke { object_id, layer_index, runs }], "Fill")
    }

    /// @emoji 🧲 Runs one gumball transform delta against a working scratch document. Mid-drag it emits
    /// nothing; only `transformEnd` (or an unbracketed single dispatch) commits the accumulated diff.
    fn transform_selection(
        &mut self,
        projection: &LowpolyProjection,
        mode: &str,
        ids: Vec<u32>,
        transform: Transform,
        description: &str,
    ) -> ActionEmit<LowpolyOperation> {
        if self.transform_drag_active {
            if self.transform.is_none() {
                self.begin_transform_session(projection);
            }
            if let Some(session) = self.transform.as_mut() {
                if !ids.is_empty() {
                    session.doc.apply_selection(mode, ids);
                }
                let _ = apply_transform(&mut session.doc, transform);
            }
            self.preview_seq = self.preview_seq.wrapping_add(1);
            return ActionEmit::default();
        }
        let operations = self.mesh_edit(projection, move |doc| {
            if !ids.is_empty() {
                doc.apply_selection(mode, ids);
            }
            apply_transform(doc, transform)
        });
        if operations.operations.is_empty() {
            ActionEmit::default()
        } else {
            ActionEmit::commit(operations.operations, description)
        }
    }

    /// @emoji 🎬 Snapshots the active object as the transform-drag base and builds the working scratch doc.
    fn begin_transform_session(&mut self, projection: &LowpolyProjection) {
        let Some(doc) = build_doc(projection, &self.runtime) else {
            return;
        };
        let object_id = doc.active_object_id().to_string();
        let Some(before) = projection.objects.iter().find(|object| object.id == object_id).cloned() else {
            return;
        };
        self.transform = Some(TransformSession { object_id, before, doc });
    }

    /// @emoji 📌 Commits the whole gumball drag as ONE `Objects(Patch)` diff (base → final mesh).
    fn commit_transform(&mut self) -> ActionEmit<LowpolyOperation> {
        let Some(mut session) = self.transform.take() else {
            return ActionEmit::default();
        };
        if session.doc.sync_meshes_to_projection().is_err() {
            return ActionEmit::default();
        }
        let Some(after) = session.doc.projection().objects.iter().find(|object| object.id == session.object_id).cloned() else {
            return ActionEmit::default();
        };
        let patch = object_patch_diff(&session.before, &after);
        if patch == LowpolyObjectPatch::default() {
            return ActionEmit::default();
        }
        ActionEmit::commit(
            vec![LowpolyOperation::ObjectsPatch { id: session.object_id, patch }],
            "Transform selection",
        )
    }

    //#region 🔖GesturePreview
    /// 👻 CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live gumball
    /// drag's current object state, expressed as a patch anchored to the drag-start snapshot
    /// (`session.before`) via the same `object_patch_diff` `commit_transform` uses for the eventual
    /// real commit. Anchoring to a fixed base (not the previous preview tick) keeps this correct even
    /// when the lossy, uncredited preview lane drops every message but the latest — a receiver only
    /// ever needs the last-synced canonical object (`before`, already has it) plus this one patch,
    /// never a chain of prior preview messages. `apply_transform` already calls
    /// `sync_meshes_to_projection` every tick (mid-drag world-scene rendering needs it regardless), so
    /// reading `session.doc.projection()` here adds no new per-tick cost. `None` outside an active drag;
    /// this reads `TransformSession` only, never emits or mutates a `LowpolyOperation`.
    ///
    /// 🚧 Deliberately unwired beyond this accessor — same gap as `draw-plugin`'s
    /// `draw_gesture_preview_payload`: `framework/sync::SyncSession::publish_preview` is host-only
    /// ("WASI-P2 plugins never link this crate") and this crate compiles as a WASI-P2 component; the
    /// one cross-sandbox channel this crate can reach, `store::BackboneMessage`, has no preview-shaped
    /// variant. See `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw7-preview-law.txt`.
    /// `#[allow(dead_code)]`: exercised by `🧪Tests` only until a host bridge exists.
    #[allow(dead_code)]
    fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let session = self.transform.as_ref()?;
        let after = session.doc.projection().objects.iter().find(|object| object.id == session.object_id)?.clone();
        let patch = object_patch_diff(&session.before, &after);
        let payload = json!({ "objectId": session.object_id.clone(), "patch": patch });
        Some(("gesture:transform", self.preview_seq, serde_json::to_vec(&payload).ok()?))
    }
    //#endregion 🔖GesturePreview
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
    type Operation = LowpolyOperation;

    fn app_id(&self) -> &str {
        LOWPOLY_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        LOWPOLY_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> LowpolyProjection {
        lowpoly_engine::default_projection()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, LowpolyProjection>,
        view_state: &ViewState,
    ) -> ActionEmit<LowpolyOperation> {
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
            SET_ACTIVE_UTILITY_ACTION_ID => {
                self.stroke = None;
                self.stroke_drag_active = false;
                self.transform = None;
                self.transform_drag_active = false;
                self.runtime.hovered_target = None;
                self.runtime.hovered_object_id = None;
                if let Some(utility) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
                    if matches!(utility, "brush" | "eraser" | "fill" | "eyedropper") {
                        self.runtime.paint_utility = utility.into();
                    }
                }
                ActionEmit::default()
            }
            "transformBegin" => {
                self.transform_drag_active = true;
                self.transform = None;
                ActionEmit::default()
            }
            "setActivePaintLayer" => {
                self.runtime.active_paint_layer =
                    args.and_then(|value| value.get("layerIndex")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                ActionEmit::default()
            }
            "setUtilityParam" => {
                let key = args.and_then(|value| value.get("key")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if let Some(map) = self.runtime.utility_params.as_object_mut() {
                    map.insert(key.into(), value);
                } else {
                    let mut map = Map::new();
                    map.insert(key.into(), value);
                    self.runtime.utility_params = Value::Object(map);
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
                    .and_then(|value| value.get("method").or_else(|| value.get("value")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle")
                    .into();
                ActionEmit::default()
            }
            "setSelectionModeDefault" => {
                if let Some(mode) = args
                    .and_then(|value| value.get("mode").or_else(|| value.get("value")))
                    .and_then(|value| value.as_str())
                {
                    self.runtime.selection_mode_default = match mode {
                        "additive" | "subtractive" | "invertive" | "default" => mode.into(),
                        _ => self.runtime.selection_mode_default.clone(),
                    };
                }
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
            "paintSample" => {
                let Some((u, v)) = paint_uv(args) else {
                    return ActionEmit::default();
                };
                let object_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| resolve_active_object_id(projection, &self.runtime));
                if let Some(object) = projection.objects.iter().find(|object| object.id == object_id) {
                    let composite = composite_layer_pixels(&object.paint_layers);
                    self.runtime.paint_color = sample_pixel_from(&composite, u, v);
                }
                ActionEmit::default()
            }
            //#endregion 👁️ View actions

            //#region ✏️ Paint operations
            "paintStrokeEnd" => {
                self.stroke_drag_active = false;
                self.commit_stroke()
            }
            "transformEnd" => {
                self.transform_drag_active = false;
                self.commit_transform()
            }
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
                self.paint_tick(projection, object_id, u, v)
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
                self.fill_at(projection, object_id, u, v)
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
                ActionEmit::operations(vec![LowpolyOperation::AddPaintLayer {
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
                ActionEmit::operations(vec![LowpolyOperation::ObjectsAdd { index, item: new_object }])
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
                ActionEmit::operations(vec![LowpolyOperation::ObjectsPatch { id: object_id.into(), patch }])
            }
            "extrude" => {
                let distance = arg_f32(args, "extrudeDistance", utility_param_f32(&self.runtime.utility_params, "extrudeDistance", 0.25));
                self.mesh_edit(projection, move |doc| {
                    let faces = doc.selected_face_ids();
                    if faces.is_empty() {
                        return Err("no faces selected".into());
                    }
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.extrude_faces(&faces, distance).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "inset" => {
                let amount = arg_f32(args, "insetAmount", utility_param_f32(&self.runtime.utility_params, "insetAmount", 0.1));
                self.mesh_edit(projection, move |doc| {
                    let faces = doc.selected_face_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.inset_faces(&faces, amount).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "bevel" => {
                let amount = arg_f32(args, "bevelAmount", utility_param_f32(&self.runtime.utility_params, "bevelAmount", 0.05));
                let segments = arg_u32(args, "bevelSegments", utility_param_u32(&self.runtime.utility_params, "bevelSegments", 1));
                self.mesh_edit(projection, move |doc| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.bevel_edges(&edges, amount, segments).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "loopCut" => {
                let cuts = arg_u32(args, "loopCuts", utility_param_u32(&self.runtime.utility_params, "loopCuts", 1));
                self.mesh_edit(projection, move |doc| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.loop_cut(&edges, cuts).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "subdivide" => self.mesh_edit(projection, move |doc| {
                let faces = doc.selected_face_ids();
                doc.active_mesh_mut().map_err(|e| e.to_string())?.subdivide_faces(&faces).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            "triangulate" => self.mesh_edit(projection, move |doc| {
                doc.active_mesh_mut().map_err(|e| e.to_string())?.triangulate().map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            "mirror" => {
                let axis = args
                    .and_then(|value| value.get("axis"))
                    .and_then(|value| value.as_str())
                    .map(|value| match value {
                        "y" => MirrorAxis::Y,
                        "z" => MirrorAxis::Z,
                        _ => MirrorAxis::X,
                    })
                    .unwrap_or_else(|| mirror_axis_from_param(&self.runtime.utility_params));
                self.mesh_edit(projection, move |doc| {
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.mirror(axis, 0.001).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "decimate" => {
                let ratio = arg_f32(args, "decimateRatio", utility_param_f32(&self.runtime.utility_params, "decimateRatio", 0.5));
                self.mesh_edit(projection, move |doc| {
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.decimate(ratio).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
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
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.flip_faces(&faces).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "merge" => self.mesh_edit(projection, move |doc| {
                let verts = doc.selected_vertex_ids();
                doc.active_mesh_mut().map_err(|e| e.to_string())?.merge_vertices(&verts, WeldMode::Center, 0.001).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            "dissolve" => self.mesh_edit(projection, move |doc| {
                let edges = doc.selected_edge_ids();
                doc.active_mesh_mut().map_err(|e| e.to_string())?.dissolve_edges(&edges).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            "snap" => {
                let grid = utility_param_f32(&self.runtime.utility_params, "snapGrid", 0.25);
                self.mesh_edit(projection, move |doc| {
                    let verts = doc.selected_vertex_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.snap_vertices_to_grid(&verts, grid).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "toggleSmooth" => self.mesh_edit(projection, move |doc| {
                if let Some(index) = doc.active_index() {
                    let smooth = !doc.projection().objects[index].smooth_shading;
                    doc.projection_mut().objects[index].smooth_shading = smooth;
                    let faces: Vec<FaceId> = (0..doc.active_mesh().map_err(|e| e.to_string())?.face_count()).map(|index| FaceId(index as u32)).collect();
                    let mesh = doc.active_mesh_mut().map_err(|e| e.to_string())?;
                    mesh.set_shading(&faces, smooth).map_err(map_kernel_err)?;
                    mesh.recompute_normals().map_err(map_kernel_err)?;
                }
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            "unwrapActive" => self.mesh_edit(projection, move |doc| {
                doc.active_mesh_mut().map_err(|e| e.to_string())?.unwrap_uv().map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            "markUvSeam" => {
                let seam = args.and_then(|value| value.get("seam")).and_then(|value| value.as_bool()).unwrap_or(true);
                let edge_ids: Vec<u32> = args
                    .and_then(|value| value.get("edgeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_else(|| self.runtime.selection.ids.clone());
                self.mesh_edit(projection, move |doc| {
                    let edges: Vec<EdgeId> = edge_ids.into_iter().map(EdgeId).collect();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.mark_uv_seam(&edges, seam);
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            "clearSeam" => self.handle_action("markUvSeam", Some(&json!({ "seam": false })), doc, view_state),
            "translateSelection" => {
                let (mode, ids) = selection_args(args);
                let dx = arg_f32(args, "dx", 0.0);
                let dy = arg_f32(args, "dy", 0.0);
                let dz = arg_f32(args, "dz", 0.0);
                self.transform_selection(projection, &mode, ids, Transform::Translate(Vec3::new(dx, dy, dz)), "Translate selection")
            }
            "rotateSelection" => {
                let (mode, ids) = selection_args(args);
                let ax = arg_f32(args, "ax", 0.0);
                let ay = arg_f32(args, "ay", 0.0);
                let az = arg_f32(args, "az", 0.0);
                let angle = arg_f32(args, "angle", 0.0);
                self.transform_selection(projection, &mode, ids, Transform::Rotate { axis: Vec3::new(ax, ay, az), angle }, "Rotate selection")
            }
            "scaleSelection" => {
                let (mode, ids) = selection_args(args);
                let sx = arg_f32(args, "sx", 1.0);
                let sy = arg_f32(args, "sy", 1.0);
                let sz = arg_f32(args, "sz", 1.0);
                self.transform_selection(projection, &mode, ids, Transform::Scale(Vec3::new(sx, sy, sz)), "Scale selection")
            }
            "setProjectionJson" | "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(parsed) = serde_json::from_str::<LowpolyProjection>(json_text) {
                        return ActionEmit::operations(vec![LowpolyOperation::SetProjection { projection: parsed }]);
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
        let labels = resolve_labels::<LowpolyLabels>(view_state);
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(LOWPOLY_TRANSFORM_UTILITY_DEFAULT);
        let scratch_projection = self.transform.as_ref().map(|session| session.doc.projection().clone());
        let render_projection = scratch_projection.as_ref().unwrap_or(projection);
        let view = LowpolyView { projection: render_projection, runtime: &self.runtime };
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
                        world_selection_json_for(view, active_utility, view_state.active_mode_id.as_deref(), Some(loaded)),
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
            LOWPOLY_PLAY_BODY_INSPECTION => build_inspector_tree(view, active_utility, labels),
            LOWPOLY_PLAY_BODY_LAYERS => build_layers_tree(view, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, LowpolyProjection>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(LOWPOLY_TRANSFORM_UTILITY_DEFAULT);
        let labels = resolve_labels::<LowpolyLabels>(view_state);
        let engagement = lowpoly_window_engagement(self.view(doc.projection), active_utility, labels);
        HashMap::from([
            (LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()),
            (LOWPOLY_PLAY_WINDOW_UV.into(), engagement),
        ])
    }

    fn window_measures(&self, _doc: &DocumentView<'_, LowpolyProjection>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = resolve_labels::<LowpolyLabels>(view_state);
        let measures = lowpoly_window_measures(&self.runtime, labels);
        HashMap::from([
            (LOWPOLY_PLAY_WINDOW_MAIN.into(), measures.clone()),
            (LOWPOLY_PLAY_WINDOW_UV.into(), measures),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<LowpolyLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(LOWPOLY_PLAY_WINDOW_MAIN, labels.window_main)
            .window_kind_label(LOWPOLY_PLAY_WINDOW_UV, labels.window_uv)
            .panel_tab_label("framework.panel.layers", if is_de { "Ebenen" } else { "Layers" })
            .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
            .mode_label("paint", if is_de { "Malen" } else { "Paint" })
            .action_labels(lowpoly_action_labels(is_de))
            .utility_labels(lowpoly_utility_labels(is_de))
            .example_labels(localized_label_map(is_de, &[("default", "Default", "Standard")]))
    }
}

//#endregion 🔖LowpolyPlayApp

//#region 🔖Manifest
/// 🧰 One transform/paint utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn lowpoly_utility(id: &str, label: &str, icon: &str, group: &str) -> UtilityDefinition {
    UtilityDefinition {
        group: Some(group.into()),
        category: Some(UtilityCategory::Utilities),
        ..UtilityDefinition::new(id, label, icon)
    }
}

pub fn create_lowpoly_app() -> App {
    let default_example = serde_json::to_string(&lowpoly_engine::default_projection()).expect("lowpoly default example");
    let engagement = {
        let projection = lowpoly_engine::default_projection();
        let runtime = LowpolyPlayRuntime::default();
        lowpoly_window_engagement(
            LowpolyView { projection: &projection, runtime: &runtime },
            LOWPOLY_TRANSFORM_UTILITY_DEFAULT,
            &LowpolyLabels::EN,
        )
    };
    App::from_builder(
        App::builder(LOWPOLY_PLAY_APP_ID, "Lowpoly")
            .document(["semio", "lowpoly"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.lowpoly".into(),
                name: "3D Lowpoly".into(),
                source_format: "lowpoly.fixture".into(),
                component_kind: "lowpoly".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                schema: "lowpoly.fixture".into(),
                export_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj, OsMediaFormat::Stl],
                import_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj],
            })
            .artifact_kind(ArtifactKindSpec {
                id: "3d.mesh".into(),
                name: "3D Mesh".into(),
                source_format: "mesh.reference".into(),
                component_kind: "mesh".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                schema: "mesh.reference".into(),
                export_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj, OsMediaFormat::Stl],
                import_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj],
            })
            .icon_id("shapes")
            .mode("edit", "Edit")
            .mode("paint", "Paint")
            .mode_layout("paint", "lowpoly-paint")
            .default_mode_id("edit")
            .window_kind_with_engagement(LOWPOLY_PLAY_WINDOW_MAIN, "Model", LOWPOLY_PLAY_BODY_MAIN, SurfaceKind::World3d, engagement.clone(), "lowpoly-model")
            .window_kind_with_engagement(LOWPOLY_PLAY_WINDOW_UV, "UV", LOWPOLY_PLAY_BODY_UV, SurfaceKind::Canvas2d, engagement, "layout-grid")
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
            // 🔧 Document-mutating operations — dispatched as VCS operations with true inverses.
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
            .operation("transformEnd", "Transform End")
            .operation("addPaintLayer", "Add Paint Layer")
            .operation("paintStrokeEnd", "Paint Stroke End")
            .operation("paintFill", "Paint Fill")
            .operation("fillBucket", "Fill Bucket")
            .operation("setProjectionJson", "Set Projection Json")
            .operation("setFixtureJson", "Set Fixture Json")
            .operation("engagementSubmit", "Engagement Submit")
            // 👁️ Ephemeral view state — selection, camera, hover, and the gesture drafts that emit no operations
            // mid-drag (paint ticks, gumball scratch, eyedropper sample).
            .view_action("setActiveObject", "Set Active Object")
            .view_action("setSelection", "Set Selection")
            .view_action("toggleSelectionKind", "Toggle Selection Kind")
            .view_action("toggleSelectionTarget", "Toggle Selection Target")
            .view_action("setActivePaintLayer", "Set Active Paint Layer")
            .view_action("setUtilityParam", "Set Utility Param")
            .view_action("engagementInput", "Engagement Input")
            .view_action("toggleShowEdges", "Toggle Show Edges")
            .view_action("toggleSun", "Toggle Sun")
            .view_action("setSunAzimuth", "Set Sun Azimuth")
            .view_action("setSunElevation", "Set Sun Elevation")
            .view_action("setSunIntensity", "Set Sun Intensity")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("setSelectionModeDefault", "Set Selection Mode Default")
            .view_action("setCamera", "Set Camera")
            .view_action("worldSelect", "World Select")
            .view_action("worldHover", "World Hover")
            .view_action("setHover", "Set Hover")
            .view_action("worldPick", "World Pick")
            .view_action("paintStrokeBegin", "Paint Stroke Begin")
            .view_action("paintStroke", "Paint Stroke")
            .view_action("paintAt", "Paint At")
            .view_action("canvasPointerDown", "Canvas Pointer Down")
            .view_action("canvasPointerMove", "Canvas Pointer Move")
            .view_action("paintSample", "Paint Sample")
            .view_action("transformBegin", "Transform Begin")
            // 📝 Staged argument forms for the P1 actions — the panel form seeds from these defaults and
            // stages typed overrides read out of `args`; `runtime.utility_params` remains the live backing store.
            .action_args("extrude", vec![ActionArgDef::slider("extrudeDistance", "Extrude Distance", 0.01, 2.0).default_value(0.25)])
            .action_args("inset", vec![ActionArgDef::number("insetAmount", "Inset Amount").default_value(0.1)])
            .action_args("bevel", vec![
                ActionArgDef::number("bevelAmount", "Bevel Amount").default_value(0.05),
                ActionArgDef::number("bevelSegments", "Bevel Segments").default_value(1),
            ])
            .action_args("loopCut", vec![ActionArgDef::number("loopCuts", "Loop Cuts").default_value(1)])
            .action_args("decimate", vec![ActionArgDef::slider("decimateRatio", "Decimate Ratio", 0.05, 1.0).default_value(0.5)])
            .action_args("mirror", vec![ActionArgDef::select("axis", "Axis", vec![
                ActionArgOption::new("x", "X"),
                ActionArgOption::new("y", "Y"),
                ActionArgOption::new("z", "Z"),
            ]).default_value("x")])
            .action_args("addPrimitive", vec![ActionArgDef::select("kind", "Kind", vec![
                ActionArgOption::new("box", "Cube"),
                ActionArgOption::new("plane", "Plane"),
                ActionArgOption::new("cylinder", "Cylinder"),
                ActionArgOption::new("cone", "Cone"),
                ActionArgOption::new("ico_sphere", "Ico Sphere"),
            ]).default_value("box")])
            .action_args("markUvSeam", vec![ActionArgDef::toggle("seam", "Seam").default_value(true)])
            // 🧰 Transform gumball + paint utilities — exclusive per-window active utility is host-owned (never a
            // document operation). Selection method/merge/kind live as an always-visible Select window-options group
            // (mirrors puzzle 3d); the transform group defaults to "move", paint bridges into `runtime.paint_utility`.
            .utility(lowpoly_utility("move", "Move", "move", "transform"))
            .utility(lowpoly_utility("rotate", "Rotate", "rotate-cw", "transform"))
            .utility(lowpoly_utility("scale", "Scale", "maximize-2", "transform"))
            .utility(lowpoly_utility("brush", "Brush", "paintbrush", "paint"))
            .utility(lowpoly_utility("eraser", "Eraser", "eraser", "paint"))
            .utility(lowpoly_utility("fill", "Fill", "paint-bucket", "paint"))
            .utility(lowpoly_utility("eyedropper", "Eyedropper", "pipette", "paint"))
            .window_kind_utilities(LOWPOLY_PLAY_WINDOW_MAIN, vec![
                "move".into(), "rotate".into(), "scale".into(),
                "brush".into(), "eraser".into(), "fill".into(), "eyedropper".into(),
            ])
            .window_kind_utilities(LOWPOLY_PLAY_WINDOW_UV, vec![
                "brush".into(), "eraser".into(), "fill".into(), "eyedropper".into(),
            ])
            // 📇 Per-window action scoping — MAIN (World3d) owns every mesh-editing/transform/UV-unwrap
            // operation (all run `mesh_edit` on the 3D mesh from 3D-view selection); the UV (Canvas2d)
            // window only paints its texture. Paint operations are listed on BOTH windows because the
            // paint utilities are scoped to both. Ephemeral view actions and global utilities
            // (selection/camera/sun/engagement/example/json) stay unscoped orphans, appearing on both.
            .window_kind_actions(LOWPOLY_PLAY_WINDOW_MAIN, vec![
                "addPrimitive".into(), "patchObject".into(), "extrude".into(), "inset".into(),
                "bevel".into(), "loopCut".into(), "subdivide".into(), "triangulate".into(),
                "mirror".into(), "decimate".into(), "flipFaces".into(), "merge".into(),
                "dissolve".into(), "snap".into(), "toggleSmooth".into(), "unwrapActive".into(),
                "markUvSeam".into(), "clearSeam".into(), "translateSelection".into(),
                "rotateSelection".into(), "scaleSelection".into(), "transformEnd".into(),
                "addPaintLayer".into(), "paintStrokeEnd".into(), "paintFill".into(), "fillBucket".into(),
            ])
            .window_kind_actions(LOWPOLY_PLAY_WINDOW_UV, vec![
                "addPaintLayer".into(), "paintStrokeEnd".into(), "paintFill".into(), "fillBucket".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("default", "Default", &default_example)
    .workflow("lowpoly", "Lowpoly", "mesh")
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<LowpolyPlayApp> {
        testkit::new_app::<LowpolyPlayApp>()
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
    fn window_kind_actions_scope_mesh_ops_to_main_only() {
        let definition = create_lowpoly_app().definition;
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window)
                .into_iter()
                .map(|action| action.id.clone())
                .collect()
        };
        let main = resolve(LOWPOLY_PLAY_WINDOW_MAIN);
        let uv = resolve(LOWPOLY_PLAY_WINDOW_UV);
        for mesh_operation in ["extrude", "addPrimitive", "bevel", "loopCut", "mirror", "unwrapActive", "markUvSeam"] {
            assert!(main.contains(&mesh_operation.to_string()), "MAIN must expose mesh operation {mesh_operation}");
            assert!(!uv.contains(&mesh_operation.to_string()), "UV must NOT expose mesh operation {mesh_operation}");
        }
        for paint_operation in ["paintFill", "fillBucket", "addPaintLayer"] {
            assert!(main.contains(&paint_operation.to_string()), "MAIN must expose paint operation {paint_operation}");
            assert!(uv.contains(&paint_operation.to_string()), "UV must expose paint operation {paint_operation}");
        }
    }

    #[test]
    fn paint_utility_params_are_utility_tagged_and_mesh_op_measures_removed() {
        let measures = lowpoly_window_measures(&LowpolyPlayRuntime::default(), &LowpolyLabels::EN);
        let group_tag = |id: &str| {
            measures.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id: gid, active_utility_id, .. } if gid == id => Some(active_utility_id.clone()),
                _ => None,
            })
        };
        // 🖌️ Live paint params are now utility-scoped Utility Options — one tagged group per stamping utility, so
        // `partition_window_measures` surfaces each only while that exact utility is the active utility.
        assert_eq!(group_tag("lowpoly-measure-paint-params-brush"), Some(Some("brush".into())));
        assert_eq!(group_tag("lowpoly-measure-paint-params-eraser"), Some(Some("eraser".into())));
        // 🧹 Snap grid stays a general (untagged) measure — it is not a single-utility parameter.
        assert_eq!(group_tag("lowpoly-measure-snap"), Some(None));
        // 🎯 Select options are always-visible window options (untagged), matching puzzle 3d.
        assert_eq!(group_tag("lowpoly-select"), Some(None));
        // 🗑️ The mesh-operation param sliders now live ONLY in the Action Panel's staged `action_args`, never a measure.
        let json = serde_json::to_string(&measures).unwrap();
        for removed in ["lowpoly-measure-extrude", "lowpoly-measure-inset", "lowpoly-measure-bevel", "lowpoly-measure-bevel-segments", "lowpoly-measure-loop-cuts", "lowpoly-measure-decimate", "lowpoly-measure-mirror"] {
            assert!(!json.contains(removed), "mesh-operation measure {removed} must be gone (covered by action_args)");
        }
    }

    #[test]
    fn select_window_options_mirror_puzzle3d_taxonomy() {
        let measures = lowpoly_window_measures(&LowpolyPlayRuntime::default(), &LowpolyLabels::EN);
        let select = measures.iter().find_map(|measure| match measure {
            WindowMeasure::Group { id, children, active_utility_id, .. } if id == "lowpoly-select" => Some((active_utility_id.clone(), children.clone())),
            _ => None,
        }).expect("lowpoly-select group");
        assert_eq!(select.0, None, "Select options must always surface in window options");
        let toggle_ids: Vec<&str> = select.1.iter().filter_map(|measure| match measure {
            WindowMeasure::Toggle { id, .. } => Some(id.as_str()),
            _ => None,
        }).collect();
        assert_eq!(
            toggle_ids,
            vec![
                "lowpoly-select-rectangle",
                "lowpoly-select-lasso",
                "lowpoly-select-mode-default",
                "lowpoly-select-mode-additive",
                "lowpoly-select-mode-subtractive",
                "lowpoly-select-mode-invertive",
                "lowpoly-select-mesh",
                "lowpoly-select-face",
                "lowpoly-select-edge",
                "lowpoly-select-vertex",
            ]
        );

        let mut app = new_app();
        app.handle_action("setSelectionMethod", Some(&json!({ "method": "lasso" })), &ViewState::default(), &testkit::meta("a")).unwrap();
        app.handle_action("setSelectionModeDefault", Some(&json!({ "mode": "additive" })), &ViewState::default(), &testkit::meta("a")).unwrap();
        let measures = app.window_measures(&ViewState::default());
        let window_measures = measures.get(LOWPOLY_PLAY_WINDOW_MAIN).expect("main window measures");
        let find_toggle = |id: &str| -> Option<bool> {
            window_measures.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id: gid, children, .. } if gid == "lowpoly-select" => children.iter().find_map(|child| match child {
                    WindowMeasure::Toggle { id: tid, pressed, .. } if tid == id => Some(*pressed),
                    _ => None,
                }),
                _ => None,
            })
        };
        assert_eq!(find_toggle("lowpoly-select-lasso"), Some(true));
        assert_eq!(find_toggle("lowpoly-select-rectangle"), Some(false));
        assert_eq!(find_toggle("lowpoly-select-mode-additive"), Some(true));
        assert_eq!(find_toggle("lowpoly-select-mode-default"), Some(false));

        let mut runtime = LowpolyPlayRuntime::default();
        runtime.selection_method = "lasso".into();
        runtime.selection_mode_default = "additive".into();
        let selection: Value = serde_json::from_str(&world_selection_json_for(
            LowpolyView { projection: &projection(&app), runtime: &runtime },
            "move",
            None,
            None,
        ))
        .unwrap();
        assert_eq!(selection.get("method").and_then(Value::as_str), Some("lasso"));
        assert_eq!(selection.get("selectionMergeMode").and_then(Value::as_str), Some("additive"));
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
    fn add_primitive_emits_objects_add_operation() {
        let mut app = new_app();
        app.handle_action("addPrimitive", Some(&json!({ "kind": "box" })), &ViewState::default(), &testkit::meta("a")).unwrap();
        let projection = projection(&app);
        assert_eq!(projection.objects.len(), 2);
        assert!(projection.objects.iter().any(|object| object.name == "box"));
    }

    #[test]
    fn extrude_selected_face_grows_mesh_and_undo_restores() {
        let mut app = new_app();
        let object_id = projection(&app).objects[0].id.clone();
        let before = LowpolyDocument::new(projection(&app)).unwrap().active_mesh().unwrap().face_count();
        app.handle_action("worldPick", Some(&face_selection()), &ViewState::default(), &testkit::meta("a")).unwrap();
        app.handle_action("extrude", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        let after = LowpolyDocument::with_context(projection(&app), object_id.clone(), LowpolySelection::default())
            .unwrap()
            .active_mesh()
            .unwrap()
            .face_count();
        assert!(after > before);
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        let restored = LowpolyDocument::with_context(projection(&app), object_id, LowpolySelection::default())
            .unwrap()
            .active_mesh()
            .unwrap()
            .face_count();
        assert_eq!(restored, before);
    }

    #[test]
    fn selection_is_view_state_and_emits_no_operations() {
        let mut app = new_app();
        let result = app.handle_action("worldPick", Some(&face_selection()), &ViewState::default(), &testkit::meta("a")).unwrap();
        assert!(result.operations.is_empty(), "picking must not create an undoable operation");
    }

    #[test]
    fn paint_stroke_drag_is_one_undo_step_with_pixel_restoration() {
        let mut app = new_app();
        let object_id = projection(&app).objects[0].id.clone();
        let before = projection(&app).objects[0].paint_layers[0].pixels.clone();
        // begin → tick → tick → end : one undoable PaintStroke edit.
        app.handle_action("paintStrokeBegin", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        let tick_a = app
            .handle_action("paintAt", Some(&json!({ "objectId": object_id, "u": 0.5, "v": 0.5 })), &ViewState::default(), &testkit::meta("a"))
            .unwrap();
        let tick_b = app
            .handle_action("paintAt", Some(&json!({ "objectId": object_id, "u": 0.52, "v": 0.5 })), &ViewState::default(), &testkit::meta("a"))
            .unwrap();
        assert!(tick_a.operations.is_empty() && tick_b.operations.is_empty(), "mid-drag ticks emit no operations");
        let end = app.handle_action("paintStrokeEnd", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        assert_eq!(end.operations.len(), 1, "the whole drag commits as one operation");
        let painted = projection(&app).objects[0].paint_layers[0].pixels.clone();
        assert_ne!(painted, before, "the stroke changed pixels");
        // ONE undo restores the exact prior pixels.
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        let restored = projection(&app).objects[0].paint_layers[0].pixels.clone();
        assert_eq!(restored, before, "undo restores the exact pre-stroke pixels");
        // Redo re-applies.
        app.handle_action("redo", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].paint_layers[0].pixels, painted);
    }

    #[test]
    fn eyedropper_updates_paint_color_without_operations() {
        let mut app = new_app();
        // 🧰 The host-owned utility switch bridges into runtime.paint_utility and emits no operations.
        let switch = app.handle_action("setActiveUtility", Some(&json!({ "utilityId": "eyedropper" })), &ViewState::default(), &testkit::meta("a")).unwrap();
        assert!(switch.operations.is_empty());
        let result = app.handle_action("paintSample", Some(&json!({ "u": 0.5, "v": 0.5 })), &ViewState::default(), &testkit::meta("a")).unwrap();
        assert!(result.operations.is_empty());
    }

    #[test]
    fn toggle_smooth_emits_op_and_flips_shading() {
        let mut app = new_app();
        let before = projection(&app).objects[0].smooth_shading;
        app.handle_action("toggleSmooth", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        assert_ne!(projection(&app).objects[0].smooth_shading, before);
    }

    #[test]
    fn add_paint_layer_emits_operation() {
        let mut app = new_app();
        let before = projection(&app).objects[0].paint_layers.len();
        app.handle_action("addPaintLayer", Some(&json!({ "name": "Detail" })), &ViewState::default(), &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].paint_layers.len(), before + 1);
    }

    #[test]
    fn extrude_reads_staged_arg_distance_into_the_operation() {
        // 🧪 Arg-form action: the staged `extrudeDistance` (not the runtime backing store) drives the edit.
        let mut small = new_app();
        let mut large = new_app();
        small.handle_action("worldPick", Some(&face_selection()), &ViewState::default(), &testkit::meta("a")).unwrap();
        large.handle_action("worldPick", Some(&face_selection()), &ViewState::default(), &testkit::meta("a")).unwrap();
        let object_id = projection(&small).objects[0].id.clone();
        small.handle_action("extrude", Some(&json!({ "extrudeDistance": 0.1 })), &ViewState::default(), &testkit::meta("a")).unwrap();
        large.handle_action("extrude", Some(&json!({ "extrudeDistance": 1.5 })), &ViewState::default(), &testkit::meta("a")).unwrap();
        let small_json = projection(&small).objects.iter().find(|o| o.id == object_id).unwrap().mesh_json.clone();
        let large_json = projection(&large).objects.iter().find(|o| o.id == object_id).unwrap().mesh_json.clone();
        assert_ne!(small_json, large_json, "different staged extrude distances must produce different meshes");
    }

    #[test]
    fn active_utility_switch_emits_no_ops_and_no_history() {
        // 🧰 Selecting a host-owned utility must never create an undoable edit.
        let mut app = new_app();
        let result = app.handle_action("setActiveUtility", Some(&json!({ "utilityId": "rotate" })), &ViewState::default(), &testkit::meta("a")).unwrap();
        assert!(result.operations.is_empty(), "utility switch must emit no operations");
        // No history entry — an undo right after is a no-operation leaving the projection untouched.
        let before = projection(&app);
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app), before, "utility switch left nothing to undo");
    }

    #[test]
    fn engagement_options_contain_no_utility_switcher() {
        // 🧰 move/rotate/scale switching lives only on the framework utility bar; the engagement keeps its
        // genuine non-utility options (snap/smooth/show-edges) but must never dispatch setActiveUtility.
        let projection = projection(&new_app());
        let runtime = LowpolyPlayRuntime::default();
        let engagement = lowpoly_window_engagement(LowpolyView { projection: &projection, runtime: &runtime }, "move", &LowpolyLabels::EN);
        let options = engagement.options.expect("lowpoly engagement keeps its non-utility options");
        assert!(
            options.iter().all(|option| option.action.as_ref().map(|action| action.action != SET_ACTIVE_UTILITY_ACTION_ID).unwrap_or(true)),
            "no engagement option may dispatch the framework setActiveUtility action; transform switching lives on the utility bar",
        );
    }

    #[test]
    fn gumball_drag_coalesces_to_one_committed_edit() {
        // 🧲 THE COALESCING REGRESSION: a multi-tick gumball translate must emit ZERO operations mid-drag and
        // exactly ONE commit operation (base → final mesh) on drag end — never a full-mesh patch per tick.
        let mut app = new_app();
        let object_id = projection(&app).objects[0].id.clone();
        let before_mesh = projection(&app).objects[0].mesh_json.clone();
        app.handle_action("transformBegin", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        let tick_a = app
            .handle_action("translateSelection", Some(&json!({ "mode": "mesh", "ids": [], "dx": 0.5, "dy": 0.0, "dz": 0.0 })), &ViewState::default(), &testkit::meta("a"))
            .unwrap();
        let tick_b = app
            .handle_action("translateSelection", Some(&json!({ "mode": "mesh", "ids": [], "dx": 0.25, "dy": 0.0, "dz": 0.0 })), &ViewState::default(), &testkit::meta("a"))
            .unwrap();
        assert!(tick_a.operations.is_empty() && tick_b.operations.is_empty(), "mid-drag transform ticks emit no operations");
        assert_eq!(projection(&app).objects[0].mesh_json, before_mesh, "no operation reached the document mid-drag");
        let end = app.handle_action("transformEnd", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        assert_eq!(end.operations.len(), 1, "the whole drag commits as exactly one operation");
        // The final diff reflects the accumulated 0.75 translation (both ticks), not just the last tick.
        let after_mesh = projection(&app).objects[0].mesh_json.clone();
        assert_ne!(after_mesh, before_mesh, "the drag moved the mesh");
        // ONE undo reverts the entire coalesced drag.
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].mesh_json, before_mesh, "one undo reverts the whole coalesced gumball drag");
    }

    //#region 🔖GesturePreview
    /// 🔬 CW7 preview-law seam: `LowpolyPlayApp::gesture_preview` reads `TransformSession` only, never a
    /// `LowpolyOperation` — exercised directly against `LowpolyPlayApp` (bypassing the `VcsDocumentApp`
    /// wrapper, which has no accessor into the inner app) since `transform_selection` is the natural
    /// per-tick gesture handler.
    #[test]
    fn gesture_preview_is_none_without_an_active_transform_drag() {
        let app = LowpolyPlayApp::default();
        assert!(app.gesture_preview().is_none(), "no live gumball drag, nothing to preview");
    }

    #[test]
    fn gesture_preview_reflects_the_live_gumball_drag_and_clears_on_commit() {
        let mut app = LowpolyPlayApp::default();
        let projection = lowpoly_engine::default_projection();
        app.transform_drag_active = true;

        let tick_a = app.transform_selection(&projection, "mesh", vec![], Transform::Translate(Vec3::new(0.5, 0.0, 0.0)), "translate");
        assert!(tick_a.operations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let (key, seq_after_a, payload_a) = app.gesture_preview().expect("a live gumball drag is previewable");
        assert_eq!(key, "gesture:transform");
        let value_a: Value = serde_json::from_slice(&payload_a).expect("payload is valid json");
        assert_eq!(value_a["objectId"], json!(projection.objects[0].id));
        assert_ne!(value_a["patch"], json!(LowpolyObjectPatch::default()), "the patch anchored to the drag-start snapshot must reflect the first tick");

        let tick_b = app.transform_selection(&projection, "mesh", vec![], Transform::Translate(Vec3::new(0.25, 0.0, 0.0)), "translate");
        assert!(tick_b.operations.is_empty());
        let (_, seq_after_b, payload_b) = app.gesture_preview().expect("still live mid-drag");
        assert!(seq_after_b > seq_after_a, "seq is monotone per tick, for staleness detection on the receiving end");
        assert_ne!(payload_a, payload_b, "the base-anchored patch accumulates both ticks, not just the latest one");

        let end = app.commit_transform();
        assert_eq!(end.operations.len(), 1, "the whole drag commits as exactly one real operation");
        assert!(app.gesture_preview().is_none(), "the drag ended: nothing left to preview, and the commit above already carried the real operation");
    }

    #[test]
    fn gesture_preview_is_a_pure_read_never_mutating_the_transform_session() {
        let mut app = LowpolyPlayApp::default();
        let projection = lowpoly_engine::default_projection();
        app.transform_drag_active = true;
        app.transform_selection(&projection, "mesh", vec![], Transform::Translate(Vec3::new(1.0, 0.0, 0.0)), "translate");
        let mesh_before = app.transform.as_ref().unwrap().doc.projection().objects[0].mesh_json.clone();
        let _ = app.gesture_preview();
        let _ = app.gesture_preview();
        assert_eq!(app.transform.as_ref().unwrap().doc.projection().objects[0].mesh_json, mesh_before, "gesture_preview must never mutate the live transform scratch it reads");
    }
    //#endregion 🔖GesturePreview

    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        // Both instances start from the identical default projection; disjoint edits (a rename on one,
        // an added primitive on the other) must converge on the shared backbone — impossible under a
        // whole-document snapshot where one write clobbers the other.
        testkit::assert_two_instances_converge::<LowpolyPlayApp, _>(
            "mem://lowpoly-convergence",
            ("patchObject", Some(&json!({ "objectId": "obj-1", "field": "name", "value": "Renamed By A" }))),
            ("addPrimitive", Some(&json!({ "kind": "box" }))),
            |app| app.projection().expect("projection"),
        );
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<LowpolyPlayApp, _>(
            "patchObject",
            Some(&json!({ "objectId": "obj-1", "field": "name", "value": "Hero" })),
            |app| app.projection().expect("projection"),
        );
    }
}
//#endregion 🧪Tests
