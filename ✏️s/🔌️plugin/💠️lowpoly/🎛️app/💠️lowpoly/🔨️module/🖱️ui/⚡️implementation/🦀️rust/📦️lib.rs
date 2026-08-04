//! 🖌️ Lowpoly app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! migration — `LowpolyPlayApp` sheds its `RefCell<LowpolyPlayRuntime>` app-struct state; every former
//! runtime field (selection, active object, paint utility/layer, selection method/mode, hover, world
//! camera, sun, show-edges) now lives in `lowpoly_engine::LowpolyConfig`, written via
//! `lowpoly_op::LowpolyConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every action
//! dispatches through the single typed `lowpoly_protocol::LowpolyCommand` channel via
//! `DocumentApp::handle`. Mesh/object structure and paint pixels remain the document projection
//! (undoable via the VCS store); the mid-drag paint stroke scratch, gumball transform-drag scratch,
//! paint texture cache and preview sequence counter remain genuine `RefCell` app-struct state (the
//! "scratch + commit" pattern the `DocumentApp` trait itself sanctions).

use base64::Engine;
use kernel_3d_mesh::{EdgeId, FaceId, MirrorAxis, Vec3, WeldMode};
use lowpoly::{empty_paint_pixels, LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolyProjection, LowpolySelection, LowpolySelectionTargets, LOWPOLY_DOCUMENT_SCHEMA, LOWPOLY_PAINT_TEXTURE_SIZE};
use lowpoly_engine::{composite_layer_pixels, flood_fill, mesh_data_from_transfer, pixel_runs_from_diff, sample_pixel_from, stamp_brush, LowpolyConfig, LowpolyDocument};
use lowpoly_op::{LowpolyConfigOperation, LowpolyOperation, PixelRun};
use lowpoly_protocol::LowpolyCommand;
use png::{BitDepth, ColorType, Encoder};
use semio_framework_plugin::{
        apply_world3d_sun_action, build_canvas_2d_scene, build_world_3d_scene, create_default_layout, create_named_layout, engagement_token_matches, merge_world_selection_ids, tree_item_with_action, ui_inspector_groups_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, world3d_camera_json, world3d_scene, world3d_selection_json, world3d_sun_measures, ActionArgDef, ActionArgOption, ActionDescriptor, App, AppIo, AppLabels, ArtifactKindSpec, Canvas2dScene,
    ConfigView, DocumentApp, DocumentView, Emit, IconName, Label, LabelText, Locale, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder, SelectionSet,
    SurfaceKind, Terminology, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiToggleNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UtilityCategory, UtilityDefinition, WindowEngagement, WindowEngagementInput,
    WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use store::DocumentPack;

//#region 🔖️Constants
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
/// 🧰️ The transform gumball utility a Model window falls back to when the host hasn't set an active utility.
const LOWPOLY_TRANSFORM_UTILITY_DEFAULT: &str = "move";

const PRIMITIVE_CATALOG: &[(&str, &str, &str)] = &[("box", "Cube", "box"), ("plane", "Plane", "square"), ("cylinder", "Cylinder", "cylinder"), ("cone", "Cone", "triangle"), ("ico_sphere", "Ico Sphere", "globe")];
//#endregion 🔖️Constants

//#region 🔖️Locale

//#endregion 🔖️Locale

//#region 🔖️Config
fn lowpoly_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(LOWPOLY_PLAY_CONTROLLER_ID).action(action, args)
}

fn utility_param_f32(params: &Value, key: &str, default: f32) -> f32 {
    params.get(key).and_then(|value| value.as_f64()).map(|v| v as f32).unwrap_or(default)
}

fn utility_param_u32(params: &Value, key: &str, default: u32) -> u32 {
    params.get(key).and_then(|value| value.as_u64()).map(|v| v as u32).unwrap_or(default)
}

/// 🧮️ Parses `config.utility_params_json` back into a `Value` — the flattened `LowpolyConfig` field
/// carries it as canonical JSON text since a raw `Value` field has no direct DSL binding.
fn utility_params_value(config: &LowpolyConfig) -> Value {
    serde_json::from_str(&config.utility_params_json).unwrap_or_default()
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
    [(sx * cy * cz + cx * sy * sz) as f64, (cx * sy * cz - sx * cy * sz) as f64, (cx * cy * sz + sx * sy * cz) as f64, (cx * cy * cz - sx * sy * sz) as f64]
}

fn resolve_active_object_id(projection: &LowpolyProjection, config: &LowpolyConfig) -> String {
    if projection.objects.iter().any(|object| object.id == config.active_object_id) {
        config.active_object_id.clone()
    } else {
        projection.objects.first().map(|object| object.id.clone()).unwrap_or_default()
    }
}

/// 🧮️ Rebuilds a `LowpolySelection` from `LowpolyConfig`'s flattened selection fields — the boundary
/// where the config's scalar fields become the compute session's structured selection value.
fn selection_from_config(config: &LowpolyConfig) -> LowpolySelection {
    LowpolySelection { targets: selection_targets_from_config(config), keys: config.selection_keys.clone(), mode: config.selection_mode.clone(), ids: config.selection_ids.clone() }
}

fn selection_targets_from_config(config: &LowpolyConfig) -> LowpolySelectionTargets {
    LowpolySelectionTargets { mesh: config.selection_targets_mesh, vertex: config.selection_targets_vertex, edge: config.selection_targets_edge, face: config.selection_targets_face }
}

fn build_doc(projection: &LowpolyProjection, config: &LowpolyConfig) -> Option<LowpolyDocument> {
    let active = resolve_active_object_id(projection, config);
    LowpolyDocument::with_context(projection.clone(), active, selection_from_config(config)).ok()
}

fn active_object<'a>(view: LowpolyView<'a>) -> Option<&'a LowpolyObject> {
    let id = resolve_active_object_id(view.projection, view.config);
    view.projection.objects.iter().find(|object| object.id == id)
}

fn lowpoly_world_camera_json(config: &LowpolyConfig) -> String {
    world3d_camera_json(config.world_camera_position, config.world_camera_target, config.world_camera_fov)
}

fn lowpoly_sun_config(config: &LowpolyConfig) -> WorldSunConfig {
    WorldSunConfig { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }
}

/// 🌞️ Reuses the framework's `WorldSunConfig`-shaped sun toggle/slider logic (`apply_world3d_sun_action`),
/// threading it through `LowpolyConfig`'s flattened sun fields and returning the resulting `SetSun` config op.
fn apply_sun_command(config: &LowpolyConfig, action_id: &str, value: Option<f64>) -> LowpolyConfigOperation {
    let mut sun = lowpoly_sun_config(config);
    let args = value.map(|value| json!({ "value": value }));
    apply_world3d_sun_action(&mut sun, action_id, args.as_ref());
    LowpolyConfigOperation::SetSun { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color }
}

/// @emoji 🧭️ A borrowed read view — the document projection plus the config — threaded into the
/// render/panel/utility/scene builders.
#[derive(Clone, Copy)]
struct LowpolyView<'a> {
    projection: &'a LowpolyProjection,
    config: &'a LowpolyConfig,
}
//#endregion 🔖️Config

//#region 🔖️MeshTransfer
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

/// @emoji 🧮️ The changed-field patch turning `before` into `after`, for a mesh edit's `Objects(Patch)`.
fn object_patch_diff(before: &LowpolyObject, after: &LowpolyObject) -> LowpolyObjectPatch {
    LowpolyObjectPatch {
        name: (before.name != after.name).then(|| after.name.clone()),
        smooth_shading: (before.smooth_shading != after.smooth_shading).then_some(after.smooth_shading),
        transform: (before.transform != after.transform).then(|| after.transform.clone()),
        mesh_json: (before.mesh_json != after.mesh_json).then(|| after.mesh_json.clone()),
    }
}
//#endregion 🔖️MeshTransfer

//#region 🔖️SelectionHelpers
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

/// 🎯️ B1: the pure, typed-command counterpart of the pre-B1 `sync_selection_keys` — computes the
/// document-target selection keys for `mode`/`ids` without mutating anything.
fn selection_keys_for(projection: &LowpolyProjection, config: &LowpolyConfig, mode: &str, ids: &[u32]) -> Vec<String> {
    let active = resolve_active_object_id(projection, config);
    let object_index = object_index_for(projection, &active);
    ids.iter().map(|id| selection_key(&active, object_index, mode, *id)).collect()
}

/// 🎯️ B1: the pure, typed-command counterpart of the pre-B1 `apply_component_selection` — computes the
/// new selection mode/ids/keys/targets after selecting `incoming` at `mode` granularity, for the caller
/// to translate into `LowpolyConfigOperation`s (never mutates `config` directly).
fn apply_component_selection(config: &LowpolyConfig, projection: &LowpolyProjection, mode: &str, incoming: &[u32], merge: &str) -> (String, Vec<u32>, Vec<String>, LowpolySelectionTargets) {
    let normalized = LowpolyDocument::normalize_selection_mode(mode);
    let mut targets = selection_targets_from_config(config);
    enable_selection_target_kind(&mut targets, &normalized);
    let ids = merge_selection_ids(&config.selection_ids, incoming, merge);
    let keys = selection_keys_for(projection, config, &normalized, &ids);
    (normalized, ids, keys, targets)
}

fn selected_document_ids(view: LowpolyView<'_>) -> Vec<String> {
    let config = view.config;
    let active = resolve_active_object_id(view.projection, config);
    let object_index = object_index_for(view.projection, &active);
    config.selection_ids.iter().map(|id| document_target_row_id(&active, object_index, &config.selection_mode, *id)).collect()
}

fn highlighted_document_ids(view: LowpolyView<'_>) -> Vec<String> {
    let config = view.config;
    match (&config.hovered_target_object_id, &config.hovered_target_mode, config.hovered_target_id) {
        (Some(object_id), Some(mode), Some(id)) => {
            vec![document_target_row_id(object_id, object_index_for(view.projection, object_id), mode, id)]
        }
        _ => Vec::new(),
    }
}

fn gumball_target_world(doc: &LowpolyDocument, view: LowpolyView<'_>) -> Option<[f64; 3]> {
    let pivot = doc.selection_transform_pivot().ok()?;
    let active = resolve_active_object_id(view.projection, view.config);
    let object = view.projection.objects.iter().find(|entry| entry.id == active)?;
    let position = &object.transform.position;
    Some([position[0] as f64 + pivot.x() as f64, position[1] as f64 + pivot.y() as f64, position[2] as f64 + pivot.z() as f64])
}

fn gumball_active(view: LowpolyView<'_>) -> bool {
    let config = view.config;
    let active = resolve_active_object_id(view.projection, config);
    !config.selection_ids.is_empty() || (config.selection_targets_mesh && config.selected_object_ids.iter().any(|id| id == &active))
}
//#endregion 🔖️SelectionHelpers

//#region 🔖️TransformHelpers
enum Transform {
    Translate(Vec3),
    Rotate { axis: Vec3, angle: f32 },
    Scale(Vec3),
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

/// 🎯️ Extracts UV (0..1) from a paint command's fields — either direct `u`/`v` (world 3d picks) or
/// canvas `x`/`y` positions mapped through the paint-texture extent (UV canvas).
fn paint_uv_from_command(u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32>) -> Option<(f32, f32)> {
    if let (Some(u), Some(v)) = (u, v) {
        return Some((u, v));
    }
    let x = x?;
    let y = y?;
    let size = LOWPOLY_PAINT_TEXTURE_SIZE as f64;
    let u = ((x as f64 / size) + 0.5).clamp(0.0, 1.0);
    let v = (1.0 - ((y as f64 / size) + 0.5).clamp(0.0, 1.0)).clamp(0.0, 1.0);
    Some((u as f32, v as f32))
}
//#endregion 🔖️TransformHelpers

//#region 🔖️Scene
fn world_selection_json_for(view: LowpolyView<'_>, active_utility: &str, doc: Option<&LowpolyDocument>) -> String {
    let config = view.config;
    let active = resolve_active_object_id(view.projection, config);
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&config.selection_method, &config.selected_object_ids, config.hovered_object_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!(config.selection_mode));
        object.insert("targets".into(), json!(selection_targets_from_config(config)));
        object.insert("transformMode".into(), json!(active_utility));
        object.insert("interactionMode".into(), json!(if is_paint_utility(active_utility) { "paint" } else { "model" }));
        object.insert("componentIds".into(), json!(config.selection_ids));
        object.insert("selectionMode".into(), json!(config.selection_mode));
        object.insert("selectionMergeMode".into(), json!(config.selection_mode_default));
        object.insert("activeObjectId".into(), json!(active));
        object.insert("gumballActive".into(), json!(gumball_active(view)));
        object.insert("showEdges".into(), json!(config.show_edges));
        if let Some(object_id) = config.hovered_target_object_id.clone() {
            object.insert("hoveredComponent".into(), json!({ "objectId": object_id, "mode": config.hovered_target_mode, "id": config.hovered_target_id }));
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
    let items: Vec<Value> = serde_json::from_str(&doc.tessellate_all_json().unwrap_or_else(|_| "[]".into())).unwrap_or_default();
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

fn world_instances_json(view: LowpolyView<'_>) -> String {
    let config = view.config;
    let instances: Vec<Value> = view
        .projection
        .objects
        .iter()
        .enumerate()
        .map(|(object_index, object)| {
            let selected = config.selected_object_ids.iter().any(|id| id == &object.id) || (config.selection_mode == "mesh" && config.selection_ids.iter().any(|id| *id as usize == object_index));
            let hovered = if config.hovered_target_object_id.is_some() {
                config.hovered_target_mode.as_deref() == Some("mesh") && config.hovered_target_object_id.as_deref() == Some(object.id.as_str())
            } else {
                config.hovered_object_id.as_deref() == Some(object.id.as_str())
            };
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

fn uv_canvas_layers_json(doc: &LowpolyDocument, view: LowpolyView<'_>, texture_cache: &HashMap<String, String>) -> String {
    let object_id = resolve_active_object_id(view.projection, view.config);
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
            let edge_uvs: Vec<f32> = transfer.get("edgeUvs").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
            let edge_is_seam: Vec<u8> = transfer.get("edgeIsSeam").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
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
//#endregion 🔖️Scene

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the lowpoly mesh editor; one field per label makes every locale combination compile-checked.
    struct LowpolyLabels {
        meshes: native_en "Meshes", native_de "Netze", reuse_en "Meshes", reuse_de "Netze";
        primitives: native_en "Primitives", native_de "Primitive", reuse_en "Primitives", reuse_de "Primitive";
        paint_layers: native_en "Paint Layers", native_de "Malebenen", reuse_en "Paint Layers", reuse_de "Malebenen";
        vertices: native_en "Vertices", native_de "Eckpunkte", reuse_en "Vertices", reuse_de "Eckpunkte";
        edges: native_en "Edges", native_de "Kanten", reuse_en "Edges", reuse_de "Kanten";
        faces: native_en "Faces", native_de "Flächen", reuse_en "Faces", reuse_de "Flächen";
        flip_normal: native_en "Flip normal", native_de "Normale umkehren", reuse_en "Flip normal", reuse_de "Normale umkehren";
        primitive_box: native_en "Cube", native_de "Würfel", reuse_en "Cube", reuse_de "Würfel";
        primitive_plane: native_en "Plane", native_de "Ebene", reuse_en "Plane", reuse_de "Ebene";
        primitive_cylinder: native_en "Cylinder", native_de "Zylinder", reuse_en "Cylinder", reuse_de "Zylinder";
        primitive_cone: native_en "Cone", native_de "Kegel", reuse_en "Cone", reuse_de "Kegel";
        primitive_ico_sphere: native_en "Ico Sphere", native_de "Ikokugel", reuse_en "Ico Sphere", reuse_de "Ikokugel";
        object: native_en "Object", native_de "Objekt", reuse_en "Building component", reuse_de "Baukomponente";
        transform: native_en "Transform", native_de "Transformation", reuse_en "Transform", reuse_de "Transformation";
        utility_params: native_en "Utility Params", native_de "Werkzeugparameter", reuse_en "Utility Params", reuse_de "Werkzeugparameter";
        window_main: native_en "Model", native_de "Modell", reuse_en "Model", reuse_de "Modell";
        window_uv: native_en "UV", native_de "UV", reuse_en "UV", reuse_de "UV";
        // inspector field labels
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        smooth_shading: native_en "Smooth Shading", native_de "Weiche Schattierung", reuse_en "Smooth Shading", reuse_de "Weiche Schattierung";
        selection: native_en "Selection", native_de "Auswahl", reuse_en "Selection", reuse_de "Auswahl";
        selection_mode: native_en "Selection Mode", native_de "Auswahlmodus", reuse_en "Selection Mode", reuse_de "Auswahlmodus";
        utility: native_en "Utility", native_de "Werkzeug", reuse_en "Utility", reuse_de "Werkzeug";
        selected: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        extrude: native_en "Extrude", native_de "Extrudieren", reuse_en "Extrude", reuse_de "Extrudieren";
        triangulate: native_en "Triangulate", native_de "Triangulieren", reuse_en "Triangulate", reuse_de "Triangulieren";
        extrude_distance: native_en "Extrude Distance", native_de "Extrusionsabstand", reuse_en "Extrude Distance", reuse_de "Extrusionsabstand";
        inset_amount: native_en "Inset Amount", native_de "Einzugsbetrag", reuse_en "Inset Amount", reuse_de "Einzugsbetrag";
        bevel_amount: native_en "Bevel Amount", native_de "Fasenbetrag", reuse_en "Bevel Amount", reuse_de "Fasenbetrag";
        bevel_segments: native_en "Bevel Segments", native_de "Fasensegmente", reuse_en "Bevel Segments", reuse_de "Fasensegmente";
        loop_cuts: native_en "Loop Cuts", native_de "Schleifenschnitte", reuse_en "Loop Cuts", reuse_de "Schleifenschnitte";
        decimate_ratio: native_en "Decimate Ratio", native_de "Dezimierungsverhältnis", reuse_en "Decimate Ratio", reuse_de "Dezimierungsverhältnis";
        snap_grid: native_en "Snap Grid", native_de "Rastergröße", reuse_en "Snap Grid", reuse_de "Rastergröße";
        mirror_axis: native_en "Mirror Axis", native_de "Spiegelachse", reuse_en "Mirror Axis", reuse_de "Spiegelachse";
        brush_size: native_en "Brush Size", native_de "Pinselgröße", reuse_en "Brush Size", reuse_de "Pinselgröße";
        brush_opacity: native_en "Brush Opacity", native_de "Pinseldeckkraft", reuse_en "Brush Opacity", reuse_de "Pinseldeckkraft";
        brush_hardness: native_en "Brush Hardness", native_de "Pinselhärte", reuse_en "Brush Hardness", reuse_de "Pinselhärte";
        // engagement + measures
        snap: native_en "Snap", native_de "Einrasten", reuse_en "Snap", reuse_de "Einrasten";
        smooth: native_en "Smooth", native_de "Glätten", reuse_en "Smooth", reuse_de "Glätten";
        show_edges: native_en "Show Edges", native_de "Kanten anzeigen", reuse_en "Show Edges", reuse_de "Kanten anzeigen";
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        mesh: native_en "Mesh", native_de "Netz", reuse_en "Mesh", reuse_de "Netz";
        face: native_en "Face", native_de "Fläche", reuse_en "Face", reuse_de "Fläche";
        edge: native_en "Edge", native_de "Kante", reuse_en "Edge", reuse_de "Kante";
        vertex: native_en "Vertex", native_de "Eckpunkt", reuse_en "Vertex", reuse_de "Eckpunkt";
        rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        lasso: native_en "Lasso", native_de "Lasso", reuse_en "Lasso", reuse_de "Lasso";
        selective: native_en "Selective", native_de "Selektiv", reuse_en "Selective", reuse_de "Selektiv";
        additive: native_en "Additive", native_de "Additiv", reuse_en "Additive", reuse_de "Additiv";
        subtractive: native_en "Subtractive", native_de "Subtraktiv", reuse_en "Subtractive", reuse_de "Subtraktiv";
        invertive: native_en "Invertive", native_de "Invertierend", reuse_en "Invertive", reuse_de "Invertierend";
        brush_group: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
    }
}

/// 🗣️ Resolves a primitive catalogue entry's display label from its stable kind; unknown kinds fall back to the catalog's native English text.
fn primitive_catalog_label(kind: &str, fallback_label: &'static str, labels: &LowpolyLabels) -> Label {
    match kind {
        "box" => labels.primitive_box.into(),
        "plane" => labels.primitive_plane.into(),
        "cylinder" => labels.primitive_cylinder.into(),
        "cone" => labels.primitive_cone.into(),
        "ico_sphere" => labels.primitive_ico_sphere.into(),
        _ => Label::data(fallback_label),
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn build_document_tree(view: LowpolyView<'_>, doc: &LowpolyDocument, labels: &LowpolyLabels) -> UiNode {
    let active_id = resolve_active_object_id(view.projection, view.config);
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
            let component_group = |mode: &str, label: LabelText, icon: &str, count: usize| {
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
                                placement: Some(UiTreeActionPlacement::Menu),
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
                            menu: None,
                            ..UiTreeItemNode::base(row_id, Label::data(format!("{} {id}", label.as_str())))
                        }
                    })
                    .collect();
                UiTreeItemNode { icon_id: IconName::from_str(icon), items: Some(leaves), description: Some(format!("{count}")), menu: None, ..UiTreeItemNode::base(format!("lowpoly-document.{object_id}.{mode}.group"), label) }
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
                items: Some(vec![component_group("vertex", labels.vertices, "circle", vertex_count), component_group("edge", labels.edges, "minus", edge_count), component_group("face", labels.faces, "square", face_count)]),
                default_open: Some(object.id == active_id),
                description: Some(object.id.clone()),
                menu: None,
                ..UiTreeItemNode::base(format!("lowpoly-document.{object_id}"), Label::data(object.name.clone()))
            }
        })
        .collect();
    let mut builder = PanelTreeBuilder::new("lowpoly-play-document").section("lowpoly-play-document.meshes", Some(labels.meshes.into()), true, items);
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
            ..tree_item_with_action(format!("lowpoly-play-catalogue.{kind}"), primitive_catalog_label(kind, label, labels), Some((*kind).to_string()), lowpoly_action("addPrimitive", Some(json!({ "kind": kind }))))
        })
        .collect();
    PanelTreeBuilder::new("lowpoly-play-catalogue").section("lowpoly-play-catalogue.primitives", Some(labels.primitives.into()), true, items).build()
}

fn build_layers_tree(view: LowpolyView<'_>, labels: &LowpolyLabels) -> UiNode {
    let object = active_object(view);
    let layers = object.map(|entry| entry.paint_layers.as_slice()).unwrap_or(&[]);
    let active_layer = view.config.active_paint_layer;
    let items: Vec<UiTreeItemNode> = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| UiTreeItemNode {
            icon_id: Some("layers".into()),
            ..tree_item_with_action(format!("lowpoly-layer:{index}"), Label::data(layer.name.clone()), Some(format!("{} · {}", layer.opacity, layer.blend_mode)), lowpoly_action("setActivePaintLayer", Some(json!({ "layerIndex": index }))))
        })
        .collect();
    PanelTreeBuilder::new("lowpoly-play-layers").section("lowpoly-play-layers.paint", Some(labels.paint_layers.into()), true, items).selected(vec![format!("lowpoly-layer:{active_layer}")]).build()
}

fn inspector_utility_param_field(id: &str, label: LabelText, key: &str, value: &Value) -> UiNode {
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: format!("lowpoly-play-inspector.{id}"),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            presence: UiPresence::default(),
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
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

fn build_inspector_tree(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels) -> UiNode {
    let Some(object) = active_object(view) else {
        return ui_stack_vertical(vec![ui_text(Label::data(format!("Schema: {LOWPOLY_DOCUMENT_SCHEMA}"))), ui_text(Label::data("No active object"))]);
    };
    let config = view.config;
    let params = utility_params_value(config);
    let targets = selection_targets_from_config(config);
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.object".into(),
            label: labels.object.into(),
            default_open: None,
            presence: UiPresence::default(),
            fields: vec![
                UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
                    id: "lowpoly-play-inspector.object.name".into(),
                    label: labels.name.into(),
                    child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                        presence: UiPresence::default(),
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
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }),
                UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
                    id: "lowpoly-play-inspector.object.smooth".into(),
                    label: labels.smooth_shading.into(),
                    child: Box::new(UiNode::Toggle(UiToggleNode {
                        id: "lowpoly-play-inspector.object.smooth.toggle".into(),
                        icon_id: "sun".into(),
                        presence: UiPresence::selected(object.smooth_shading),
                        text: None,
                        on_change: lowpoly_action("patchObject", Some(json!({ "objectId": object.id, "field": "smoothShading" }))),
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }),
                ui_inspector_readonly_field("lowpoly-play-inspector.object.selection", labels.selection, &format!("{} · {} {}", format_selection_targets_label(&targets), config.selection_ids.len(), labels.selected.as_str(),)),
                ui_inspector_readonly_field("lowpoly-play-inspector.object.selection-mode", labels.selection_mode, &config.selection_mode),
            ],
        },
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "lowpoly-play-inspector.transform".into(),
            label: labels.transform.into(),
            default_open: None,
            fields: vec![ui_inspector_readonly_field("lowpoly-play-inspector.transform.utility", labels.utility, active_utility)],
        },
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "lowpoly-play-inspector.utility-params".into(),
            label: labels.utility_params.into(),
            default_open: Some(true),
            fields: vec![
                inspector_utility_param_field("extrude", labels.extrude_distance, "extrudeDistance", &params),
                inspector_utility_param_field("inset", labels.inset_amount, "insetAmount", &params),
                inspector_utility_param_field("bevel", labels.bevel_amount, "bevelAmount", &params),
                inspector_utility_param_field("bevel-segments", labels.bevel_segments, "bevelSegments", &params),
                inspector_utility_param_field("loop-cuts", labels.loop_cuts, "loopCuts", &params),
                inspector_utility_param_field("decimate", labels.decimate_ratio, "decimateRatio", &params),
                inspector_utility_param_field("snap", labels.snap_grid, "snapGrid", &params),
                inspector_utility_param_field("mirror", labels.mirror_axis, "mirrorAxis", &params),
                inspector_utility_param_field("brush-size", labels.brush_size, "brushSize", &params),
                inspector_utility_param_field("brush-opacity", labels.brush_opacity, "brushOpacity", &params),
                inspector_utility_param_field("brush-hardness", labels.brush_hardness, "brushHardness", &params),
            ],
        },
    ])
}
//#endregion 🔖️Panels

//#region 🔖️Utilities
fn is_paint_utility(utility_id: &str) -> bool {
    matches!(utility_id, "brush" | "eraser" | "fill" | "eyedropper")
}

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

fn lowpoly_window_engagement(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels) -> WindowEngagement {
    let config = view.config;
    let transform = active_utility;
    let selected_count = config.selection_ids.len();
    WindowEngagement {
        session_active: Some(true),
        // 🧰️ The move/rotate/scale transform switcher now lives in the framework utility bar (declared via `.utility` +
        // `.window_kind_utilities`), so the engagement keeps only its non-utility options below.
        options: Some(vec![
            WindowEngagementOption { id: "lowpoly.opt.snap".into(), label: Some(labels.snap.into()), icon_id: Some("magnet".into()), pressed: None, disabled: None, action: Some(lowpoly_action("snap", None)) },
            WindowEngagementOption { id: "lowpoly.opt.smooth".into(), label: Some(labels.smooth.into()), icon_id: Some("sun".into()), pressed: None, disabled: None, action: Some(lowpoly_action("toggleSmooth", None)) },
            WindowEngagementOption {
                id: "lowpoly.opt.show-edges".into(),
                label: Some(labels.show_edges.into()),
                icon_id: Some("grid-3x3".into()),
                pressed: Some(config.show_edges),
                disabled: None,
                action: Some(lowpoly_action("toggleShowEdges", None)),
            },
        ]),
        input: Some(WindowEngagementInput {
            id: Some("lowpoly-engagement".into()),
            value: Some(config.engagement_input.clone()),
            placeholder: Some("extrude, inset, mirror, decimate".into()),
            disabled: None,
            on_change: Some(lowpoly_action("engagementInput", None)),
            on_submit: Some(lowpoly_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "lowpoly-status".into(), text: format!("{} · {} · {selected_count} {}", format_selection_targets_label(&selection_targets_from_config(config)), transform, labels.selected.as_str(),) }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "lowpoly.eng.extrude".into(), label: labels.extrude.into(), detail: None, action: Some(lowpoly_action("extrude", None)) },
            WindowEngagementPossible { id: "lowpoly.eng.triangulate".into(), label: labels.triangulate.into(), detail: None, action: Some(lowpoly_action("triangulate", None)) },
        ]),
    }
}

fn utility_param_f64(params: &Value, key: &str, default: f64) -> f64 {
    utility_param_f32(params, key, default as f32) as f64
}

fn lowpoly_utility_param_slider(id: &str, label: LabelText, key: &str, params: &Value, default: f64, min: f64, max: f64, step: f64) -> WindowMeasure {
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

        waiting: None,
    }
}

/// 🎯️ One selection-granularity toggle. Selection kinds are a non-exclusive multi-select (mesh + face +
/// edge + vertex can all be active at once), so they are a window-measure toggle group — NOT a
/// single-active utility group.
fn selection_kind_toggle(id: &str, icon: &str, label: LabelText, kind: &str, pressed: bool) -> WindowMeasure {
    WindowMeasure::Toggle { id: format!("lowpoly-select-{id}"), icon_id: icon.into(), label: Some(label.into()), pressed, text: None, on_change: lowpoly_action("toggleSelectionKind", Some(json!({ "kind": kind }))) }
}

fn lowpoly_window_measures(config: &LowpolyConfig, labels: &LowpolyLabels) -> Vec<WindowMeasure> {
    let params = utility_params_value(config);
    vec![
        WindowMeasure::Toggle { id: "lowpoly-measure-show-edges".into(), icon_id: "grid-3x3".into(), label: Some(labels.show_edges.into()), pressed: config.show_edges, text: None, on_change: lowpoly_action("toggleShowEdges", None) },
        world3d_sun_measures("lowpoly", &lowpoly_sun_config(config), lowpoly_action),
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
            children: vec![lowpoly_utility_param_slider("snap", labels.snap_grid, "snapGrid", &params, 0.25, 0.05, 2.0, 0.05)],
        },
        lowpoly_select_measures_group(config, labels),
        lowpoly_paint_params_group("brush", &params, labels),
        lowpoly_paint_params_group("eraser", &params, labels),
    ]
}

/// 🎯️ Always-visible Select window options — mirrors puzzle 3d's select measures group: rectangle/lasso
/// method, selective/additive/subtractive/invertive merge mode, and composable mesh/face/edge/vertex kinds.
fn lowpoly_select_measures_group(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    let targets = selection_targets_from_config(config);
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
                pressed: config.selection_method == "rectangle",
                text: None,
                on_change: lowpoly_action("setSelectionMethod", Some(json!({ "method": "rectangle" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-lasso".into(),
                icon_id: "lasso".into(),
                label: Some(labels.lasso.into()),
                pressed: config.selection_method == "lasso",
                text: None,
                on_change: lowpoly_action("setSelectionMethod", Some(json!({ "method": "lasso" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-default".into(),
                icon_id: "mouse-pointer".into(),
                label: Some(labels.selective.into()),
                pressed: config.selection_mode_default == "default",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "default" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-additive".into(),
                icon_id: "plus".into(),
                label: Some(labels.additive.into()),
                pressed: config.selection_mode_default == "additive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "additive" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-subtractive".into(),
                icon_id: "minus".into(),
                label: Some(labels.subtractive.into()),
                pressed: config.selection_mode_default == "subtractive",
                text: None,
                on_change: lowpoly_action("setSelectionModeDefault", Some(json!({ "mode": "subtractive" }))),
            },
            WindowMeasure::Toggle {
                id: "lowpoly-select-mode-invertive".into(),
                icon_id: "arrow-right-left".into(),
                label: Some(labels.invertive.into()),
                pressed: config.selection_mode_default == "invertive",
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
    let slider = |suffix: &str, label: LabelText, key: &str, default: f64, min: f64, max: f64, step: f64| WindowMeasure::Slider {
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

        waiting: None,
    };
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

//#endregion 🔖️Utilities

//#region 🔖️LowpolyPlayApp
/// @emoji 🖌️ In-progress paint drag: the pre-stroke layer buffer and the accumulating scratch buffer.
/// Mid-drag ticks mutate `scratch` (view state); the stroke commits as ONE `PaintStroke` operation on end.
struct PaintStrokeSession {
    object_id: String,
    layer_index: usize,
    base: Vec<u8>,
    scratch: Vec<u8>,
}

/// @emoji 🧲️ In-progress gumball transform drag. The mesh-transform operation re-serializes the WHOLE
/// `mesh_json` buffer per apply, so a per-tick `amend` would `combined.extend` N full-mesh patches and
/// replay them all (O(N) retained megabyte-scale JSON + O(N²) replay). Instead every mid-drag tick
/// applies its delta to this scratch `LowpolyDocument` emitting ZERO operations, and the whole gesture commits
/// as ONE `Objects(Patch)` (base → final mesh) on drag end (`Emit::commit`, coalesce-key `None`).
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

/// @emoji 🖌️ B1: sheds `RefCell<LowpolyPlayRuntime>` entirely — every former runtime field now lives in
/// `lowpoly_engine::LowpolyConfig`, written through `LowpolyConfigOperation`s emitted from `handle`. The
/// remaining seven `RefCell` fields are genuine mid-gesture scratch buffers (paint stroke scratch,
/// transform-drag working document, texture cache, preview sequence counter) — the "scratch + commit"
/// pattern the `DocumentApp` trait itself sanctions for `&self`-only `handle`.
pub struct LowpolyPlayApp {
    stroke: RefCell<Option<PaintStrokeSession>>,
    stroke_drag_active: RefCell<bool>,
    stroke_dirty: RefCell<u64>,
    transform: RefCell<Option<TransformSession>>,
    transform_drag_active: RefCell<bool>,
    texture_cache: RefCell<PaintTextureCache>,
    /// 👻️ Per-`key` monotone counter for `gesture_preview` — see `//#region 🔖️GesturePreview`.
    preview_seq: RefCell<u64>,
}

impl Default for LowpolyPlayApp {
    fn default() -> Self {
        Self {
            stroke: RefCell::new(None),
            stroke_drag_active: RefCell::new(false),
            stroke_dirty: RefCell::new(0),
            transform: RefCell::new(None),
            transform_drag_active: RefCell::new(false),
            texture_cache: RefCell::new(PaintTextureCache::default()),
            preview_seq: RefCell::new(0),
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
    /// @emoji 🖼️ The layers to composite for `object`, overlaying the live stroke scratch when the drag
    /// targets that object so the in-progress stroke previews before it commits.
    fn composite_layers_for(&self, object: &LowpolyObject) -> Vec<u8> {
        if let Some(session) = &*self.stroke.borrow() {
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
        fnv1a_u64(hash, &self.stroke_dirty.borrow().to_le_bytes())
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

    /// @emoji 🔧️ Runs a kernel mesh edit against a compute session built from the projection + config,
    /// then emits the resulting `Objects(Patch)` capturing only the changed object fields.
    fn mesh_edit(&self, projection: &LowpolyProjection, config: &LowpolyConfig, edit: impl FnOnce(&mut LowpolyDocument) -> Result<(), String>) -> Emit<LowpolyOperation, LowpolyConfigOperation> {
        let Some(mut doc) = build_doc(projection, config) else {
            return Emit::default();
        };
        let object_id = doc.active_object_id().to_string();
        let Some(before) = projection.objects.iter().find(|object| object.id == object_id).cloned() else {
            return Emit::default();
        };
        if edit(&mut doc).is_err() {
            return Emit::default();
        }
        if doc.sync_meshes_to_projection().is_err() {
            return Emit::default();
        }
        let Some(after) = doc.projection().objects.iter().find(|object| object.id == object_id).cloned() else {
            return Emit::default();
        };
        let patch = object_patch_diff(&before, &after);
        if patch == LowpolyObjectPatch::default() {
            return Emit::default();
        }
        Emit::operations(vec![LowpolyOperation::ObjectsPatch { id: object_id, patch }])
    }

    /// @emoji 📌️ Commits the accumulated paint scratch as ONE described `PaintStroke` edit (scratch-commit
    /// pattern b — the whole drag is one undoable edit; megabyte pixel buffers never coalesce per tick).
    fn commit_stroke(&self) -> Emit<LowpolyOperation, LowpolyConfigOperation> {
        let Some(session) = self.stroke.borrow_mut().take() else {
            return Emit::default();
        };
        *self.stroke_dirty.borrow_mut() += 1;
        let runs: Vec<PixelRun> = pixel_runs_from_diff(&session.base, &session.scratch).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect();
        if runs.is_empty() {
            return Emit::default();
        }
        Emit::commit(vec![LowpolyOperation::PaintStroke { object_id: session.object_id, layer_index: session.layer_index, runs }], "Paint stroke")
    }

    /// @emoji 🖌️ One mid-drag paint tick: brush/eraser/fill mutate the stroke scratch, eyedropper samples
    /// the paint color (as a `SetPaintColor` config op). Emits ZERO document operations — the stroke
    /// commits only on `paintStrokeEnd` (View-kind safe).
    fn paint_tick(&self, projection: &LowpolyProjection, config: &LowpolyConfig, object_id: String, u: f32, v: f32) -> Emit<LowpolyOperation, LowpolyConfigOperation> {
        let utility = config.paint_utility.clone();
        if utility == "eyedropper" {
            let Some(object) = projection.objects.iter().find(|object| object.id == object_id) else {
                return Emit::default();
            };
            let composite = composite_layer_pixels(&object.paint_layers);
            let color = sample_pixel_from(&composite, u, v);
            return Emit::config(vec![LowpolyConfigOperation::SetPaintColor { r: color[0], g: color[1], b: color[2], a: color[3] }]);
        }
        let layer_index = config.active_paint_layer as usize;
        let need_new = match &*self.stroke.borrow() {
            Some(session) => session.object_id != object_id || session.layer_index != layer_index,
            None => true,
        };
        if need_new {
            let base = projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)).map(|layer| layer.pixels.clone()).unwrap_or_else(empty_paint_pixels);
            *self.stroke.borrow_mut() = Some(PaintStrokeSession { object_id: object_id.clone(), layer_index, scratch: base.clone(), base });
        }
        let color = [config.paint_color_r, config.paint_color_g, config.paint_color_b, config.paint_color_a];
        let params = utility_params_value(config);
        if let Some(session) = self.stroke.borrow_mut().as_mut() {
            if utility == "fill" {
                flood_fill(&mut session.scratch, u, v, color);
            } else {
                let radius = utility_param_f32(&params, "brushSize", 16.0);
                let opacity = utility_param_f32(&params, "brushOpacity", 1.0);
                let hardness = utility_param_f32(&params, "brushHardness", 0.5);
                stamp_brush(&mut session.scratch, u, v, radius, color, hardness, opacity, utility == "eraser");
            }
        }
        *self.stroke_dirty.borrow_mut() += 1;
        Emit::default()
    }

    /// @emoji 🪣️ A single-shot flood fill emitted as ONE `PaintStroke` edit (the `fillBucket`/`paintFill`
    /// operation path — not drag-bracketed, so it commits immediately).
    fn fill_at(&self, projection: &LowpolyProjection, config: &LowpolyConfig, object_id: String, u: f32, v: f32) -> Emit<LowpolyOperation, LowpolyConfigOperation> {
        let layer_index = config.active_paint_layer as usize;
        let color = [config.paint_color_r, config.paint_color_g, config.paint_color_b, config.paint_color_a];
        let Some(layer) = projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)) else {
            return Emit::default();
        };
        let mut scratch = layer.pixels.clone();
        flood_fill(&mut scratch, u, v, color);
        let runs: Vec<PixelRun> = pixel_runs_from_diff(&layer.pixels, &scratch).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect();
        if runs.is_empty() {
            return Emit::default();
        }
        *self.stroke_dirty.borrow_mut() += 1;
        Emit::commit(vec![LowpolyOperation::PaintStroke { object_id, layer_index, runs }], "Fill")
    }

    /// @emoji 🧲️ Runs one gumball transform delta against a working scratch document. Mid-drag it emits
    /// nothing; only `transformEnd` (or an unbracketed single dispatch) commits the accumulated diff.
    fn transform_selection(&self, projection: &LowpolyProjection, config: &LowpolyConfig, mode: &str, ids: Vec<u32>, transform: Transform, description: &str) -> Emit<LowpolyOperation, LowpolyConfigOperation> {
        if *self.transform_drag_active.borrow() {
            if self.transform.borrow().is_none() {
                self.begin_transform_session(projection, config);
            }
            if let Some(session) = self.transform.borrow_mut().as_mut() {
                if !ids.is_empty() {
                    session.doc.apply_selection(mode, ids);
                }
                let _ = apply_transform(&mut session.doc, transform);
            }
            let next_seq = self.preview_seq.borrow().wrapping_add(1);
            *self.preview_seq.borrow_mut() = next_seq;
            return Emit::default();
        }
        let emitted = self.mesh_edit(projection, config, move |doc| {
            if !ids.is_empty() {
                doc.apply_selection(mode, ids);
            }
            apply_transform(doc, transform)
        });
        if emitted.document_operations.is_empty() {
            Emit::default()
        } else {
            Emit::commit(emitted.document_operations, description)
        }
    }

    /// @emoji 🎬️ Snapshots the active object as the transform-drag base and builds the working scratch doc.
    fn begin_transform_session(&self, projection: &LowpolyProjection, config: &LowpolyConfig) {
        let Some(doc) = build_doc(projection, config) else {
            return;
        };
        let object_id = doc.active_object_id().to_string();
        let Some(before) = projection.objects.iter().find(|object| object.id == object_id).cloned() else {
            return;
        };
        *self.transform.borrow_mut() = Some(TransformSession { object_id, before, doc });
    }

    /// @emoji 📌️ Commits the whole gumball drag as ONE `Objects(Patch)` diff (base → final mesh).
    fn commit_transform(&self) -> Emit<LowpolyOperation, LowpolyConfigOperation> {
        let Some(mut session) = self.transform.borrow_mut().take() else {
            return Emit::default();
        };
        if session.doc.sync_meshes_to_projection().is_err() {
            return Emit::default();
        }
        let Some(after) = session.doc.projection().objects.iter().find(|object| object.id == session.object_id).cloned() else {
            return Emit::default();
        };
        let patch = object_patch_diff(&session.before, &after);
        if patch == LowpolyObjectPatch::default() {
            return Emit::default();
        }
        Emit::commit(vec![LowpolyOperation::ObjectsPatch { id: session.object_id, patch }], "Transform selection")
    }

    //#region 🔖️GesturePreview
    /// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live gumball
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
    /// 🚧️ Deliberately unwired beyond this accessor — same gap as `draw-plugin`'s
    /// `draw_gesture_preview_payload`: `framework/sync::SyncSession::publish_preview` is host-only
    /// ("WASI-P2 plugins never link this crate") and this crate compiles as a WASI-P2 component; the
    /// one cross-sandbox channel this crate can reach, `store::BackboneMessage`, has no preview-shaped
    /// variant. See `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw7-preview-law.txt`.
    /// `#[allow(dead_code)]`: exercised by `🧪️Tests` only until a host bridge exists.
    #[allow(dead_code)]
    fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let transform = self.transform.borrow();
        let session = transform.as_ref()?;
        let after = session.doc.projection().objects.iter().find(|object| object.id == session.object_id)?.clone();
        let patch = object_patch_diff(&session.before, &after);
        let payload = json!({ "objectId": session.object_id.clone(), "patch": patch });
        Some(("gesture:transform", *self.preview_seq.borrow(), serde_json::to_vec(&payload).ok()?))
    }
    //#endregion 🔖️GesturePreview
}

impl DocumentApp for LowpolyPlayApp {
    type Projection = LowpolyProjection;
    type Operation = LowpolyOperation;
    type Config = LowpolyConfig;
    type ConfigOperation = LowpolyConfigOperation;
    type Command = LowpolyCommand;

    fn app_id(&self) -> &str {
        LOWPOLY_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        LOWPOLY_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> LowpolyProjection {
        lowpoly_engine::default_projection()
    }

    fn io(&self) -> Option<AppIo> {
        Some(lowpoly_engine::lowpoly_io())
    }

    fn whole_document_operation(&self, projection: LowpolyProjection) -> Option<LowpolyOperation> {
        Some(LowpolyOperation::SetProjection { projection })
    }

    /// 🎞️ `mesh:out` (see `lowpoly_engine::lowpoly_mesh_from_document`/`mesh_document_from_mesh`) plus
    /// the inherited `document:out` default (the pack of `doc.projection`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, LowpolyProjection>) -> Result<Media, MediaError> {
        match port {
            "mesh:out" => {
                let document_json = serde_json::to_value(doc.projection).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let mesh = lowpoly_engine::lowpoly_mesh_from_document(&document_json).map_err(|error| MediaError::Payload(port.into(), error))?;
                let mesh_document = lowpoly_engine::mesh_document_from_mesh(&mesh).map_err(|error| MediaError::Payload(port.into(), error))?;
                let json = serde_json::to_string(&mesh_document).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } })
            }
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `mesh:in` round-trips a `mesh.document` payload into a `SetProjection` op; `document:in`
    /// replicates the trait's default whole-pack import inline (overriding `import_media` shadows the
    /// default for every port on this app, not just the new one).
    fn import_media(&self, port: &str, media: &Media, _doc: &DocumentView<'_, LowpolyProjection>) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, MediaError> {
        match port {
            "mesh:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.into(), "mesh:in importer only accepts a Structured payload".into()));
                };
                let mesh_document: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let mesh = lowpoly_engine::mesh_from_mesh_document(&mesh_document).map_err(|error| MediaError::Payload(port.into(), error))?;
                let projection_json = lowpoly_engine::lowpoly_document_from_mesh(&mesh).map_err(|error| MediaError::Payload(port.into(), error))?;
                let projection: LowpolyProjection = serde_json::from_value(projection_json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                Ok(Emit::operations(vec![LowpolyOperation::SetProjection { projection }]))
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.into(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let projection = <LowpolyProjection as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                match self.whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `LowpolyCommand` variant back to the action id it was declared under in
    /// `create_lowpoly_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &LowpolyCommand) -> &str {
        match command {
            LowpolyCommand::AddPrimitive { .. } => "addPrimitive",
            LowpolyCommand::PatchObject { .. } => "patchObject",
            LowpolyCommand::Extrude { .. } => "extrude",
            LowpolyCommand::Inset { .. } => "inset",
            LowpolyCommand::Bevel { .. } => "bevel",
            LowpolyCommand::LoopCut { .. } => "loopCut",
            LowpolyCommand::Subdivide => "subdivide",
            LowpolyCommand::Triangulate => "triangulate",
            LowpolyCommand::Mirror { .. } => "mirror",
            LowpolyCommand::Decimate { .. } => "decimate",
            LowpolyCommand::FlipFaces { .. } => "flipFaces",
            LowpolyCommand::Merge => "merge",
            LowpolyCommand::Dissolve => "dissolve",
            LowpolyCommand::Snap => "snap",
            LowpolyCommand::ToggleSmooth => "toggleSmooth",
            LowpolyCommand::UnwrapActive => "unwrapActive",
            LowpolyCommand::MarkUvSeam { .. } => "markUvSeam",
            LowpolyCommand::ClearSeam => "clearSeam",
            LowpolyCommand::TranslateSelection { .. } => "translateSelection",
            LowpolyCommand::RotateSelection { .. } => "rotateSelection",
            LowpolyCommand::ScaleSelection { .. } => "scaleSelection",
            LowpolyCommand::AddPaintLayer { .. } => "addPaintLayer",
            LowpolyCommand::PaintStrokeEnd => "paintStrokeEnd",
            LowpolyCommand::PaintFill { .. } => "paintFill",
            LowpolyCommand::FillBucket { .. } => "fillBucket",
            LowpolyCommand::TransformEnd => "transformEnd",
            LowpolyCommand::SetProjectionJson { .. } => "setProjectionJson",
            LowpolyCommand::SetFixtureJson { .. } => "setFixtureJson",
            LowpolyCommand::EngagementSubmit { .. } => "engagementSubmit",
            LowpolyCommand::SetActiveObject { .. } => "setActiveObject",
            LowpolyCommand::SetSelection { .. } => "setSelection",
            LowpolyCommand::ToggleSelectionKind { .. } => "toggleSelectionKind",
            LowpolyCommand::ToggleSelectionTarget { .. } => "toggleSelectionTarget",
            LowpolyCommand::SetActivePaintLayer { .. } => "setActivePaintLayer",
            LowpolyCommand::SetUtilityParam { .. } => "setUtilityParam",
            LowpolyCommand::EngagementInput { .. } => "engagementInput",
            LowpolyCommand::ToggleShowEdges => "toggleShowEdges",
            LowpolyCommand::ToggleSun => "toggleSun",
            LowpolyCommand::SetSunAzimuth { .. } => "setSunAzimuth",
            LowpolyCommand::SetSunElevation { .. } => "setSunElevation",
            LowpolyCommand::SetSunIntensity { .. } => "setSunIntensity",
            LowpolyCommand::SetSelectionMethod { .. } => "setSelectionMethod",
            LowpolyCommand::SetSelectionModeDefault { .. } => "setSelectionModeDefault",
            LowpolyCommand::SetCamera { .. } => "setCamera",
            LowpolyCommand::WorldSelect { .. } => "worldSelect",
            LowpolyCommand::WorldHover { .. } => "worldHover",
            LowpolyCommand::SetHover { .. } => "setHover",
            LowpolyCommand::WorldPick { .. } => "worldPick",
            LowpolyCommand::PaintStrokeBegin => "paintStrokeBegin",
            LowpolyCommand::PaintSample { .. } => "paintSample",
            LowpolyCommand::PaintStroke { .. } => "paintStroke",
            LowpolyCommand::PaintAt { .. } => "paintAt",
            LowpolyCommand::CanvasPointerDown { .. } => "canvasPointerDown",
            LowpolyCommand::CanvasPointerMove { .. } => "canvasPointerMove",
            LowpolyCommand::TransformBegin => "transformBegin",
            LowpolyCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
        }
    }

    fn handle(&self, command: &LowpolyCommand, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>) -> Result<Emit<LowpolyOperation, LowpolyConfigOperation>, Fault> {
        let projection = doc.projection;
        let config = cfg.projection;
        match command {
            //#region 👁️ View (config-only) commands
            LowpolyCommand::SetActiveObject { object_id } => {
                if projection.objects.iter().any(|object| &object.id == object_id) {
                    Emit::config(vec![LowpolyConfigOperation::SetActiveObject { object_id: object_id.clone() }])
                } else {
                    Emit::default()
                }
            }
            LowpolyCommand::SetSelection { mode, ids } => {
                let normalized = LowpolyDocument::normalize_selection_mode(mode);
                let keys = selection_keys_for(projection, config, &normalized, ids);
                Emit::config(vec![LowpolyConfigOperation::SetSelection { mode: normalized, ids: ids.clone() }, LowpolyConfigOperation::SetSelectionKeys { keys }])
            }
            LowpolyCommand::ToggleSelectionKind { kind } => {
                let mut targets = selection_targets_from_config(config);
                let enabled = match kind.as_str() {
                    "vertex" => {
                        targets.vertex = !targets.vertex;
                        targets.vertex
                    }
                    "edge" => {
                        targets.edge = !targets.edge;
                        targets.edge
                    }
                    "face" => {
                        targets.face = !targets.face;
                        targets.face
                    }
                    _ => {
                        targets.mesh = !targets.mesh;
                        targets.mesh
                    }
                };
                let mut config_operations = vec![LowpolyConfigOperation::SetSelectionTargets { mesh: targets.mesh, vertex: targets.vertex, edge: targets.edge, face: targets.face }];
                if enabled {
                    config_operations.push(LowpolyConfigOperation::SetSelection { mode: LowpolyDocument::normalize_selection_mode(kind), ids: config.selection_ids.clone() });
                    config_operations.push(LowpolyConfigOperation::SetHoveredTarget { object_id: None, mode: None, id: None });
                    config_operations.push(LowpolyConfigOperation::SetHoveredObject { object_id: None });
                }
                Emit::config(config_operations)
            }
            LowpolyCommand::ToggleSelectionTarget { object_id, mode, id, merge } => {
                if !projection.objects.iter().any(|object| &object.id == object_id) {
                    return Emit::default();
                }
                let (new_mode, ids, keys, targets) = apply_component_selection(config, projection, mode, &[*id], merge);
                Emit::config(vec![
                    LowpolyConfigOperation::SetActiveObject { object_id: object_id.clone() },
                    LowpolyConfigOperation::SetSelectionTargets { mesh: targets.mesh, vertex: targets.vertex, edge: targets.edge, face: targets.face },
                    LowpolyConfigOperation::SetSelection { mode: new_mode, ids },
                    LowpolyConfigOperation::SetSelectionKeys { keys },
                ])
            }
            LowpolyCommand::SetActivePaintLayer { layer_index } => Ok(Emit::config(vec![LowpolyConfigOperation::SetActivePaintLayer { value: *layer_index }]),
            LowpolyCommand::SetUtilityParam { key, value_json } => {
                let mut params = utility_params_value(config);
                let value: Value = serde_json::from_str(value_json).unwrap_or(Value::Null);
                if let Some(map) = params.as_object_mut() {
                    map.insert(key.clone(), value);
                } else {
                    let mut map = Map::new();
                    map.insert(key.clone(), value);
                    params = Value::Object(map);
                }
                Emit::config(vec![LowpolyConfigOperation::SetUtilityParams { json: params.to_string() }])
            }
            LowpolyCommand::EngagementInput { value } => Ok(Emit::config(vec![LowpolyConfigOperation::SetEngagementInput { value: value.clone() }]),
            LowpolyCommand::ToggleShowEdges => Ok(Emit::config(vec![LowpolyConfigOperation::SetShowEdges { value: !config.show_edges }]),
            LowpolyCommand::ToggleSun => Ok(Emit::config(vec![apply_sun_command(config, "toggleSun", None)]),
            LowpolyCommand::SetSunAzimuth { value } => Ok(Emit::config(vec![apply_sun_command(config, "setSunAzimuth", Some(*value))]),
            LowpolyCommand::SetSunElevation { value } => Ok(Emit::config(vec![apply_sun_command(config, "setSunElevation", Some(*value))]),
            LowpolyCommand::SetSunIntensity { value } => Ok(Emit::config(vec![apply_sun_command(config, "setSunIntensity", Some(*value))]),
            LowpolyCommand::SetSelectionMethod { value } => Ok(Emit::config(vec![LowpolyConfigOperation::SetSelectionMethod { value: value.clone() }]),
            LowpolyCommand::SetSelectionModeDefault { value } => {
                let next = match value.as_str() {
                    "additive" | "subtractive" | "invertive" | "default" => value.clone(),
                    _ => config.selection_mode_default.clone(),
                };
                Emit::config(vec![LowpolyConfigOperation::SetSelectionModeDefault { value: next }])
            }
            LowpolyCommand::SetCamera { position, target, fov } => Ok(Emit::config(vec![LowpolyConfigOperation::SetWorldCamera { position: *position, target: *target, fov: *fov }]),
            LowpolyCommand::WorldSelect { ids, merge } => {
                let current = SelectionSet::from_ids(config.selected_object_ids.clone());
                let merged = merge_world_selection_ids(&current, ids, merge).to_vec();
                let mut config_operations = vec![LowpolyConfigOperation::SetSelectedObjectIds { ids: merged.clone() }];
                if let Some(first) = merged.first() {
                    config_operations.push(LowpolyConfigOperation::SetActiveObject { object_id: first.clone() });
                }
                Emit::config(config_operations)
            }
            LowpolyCommand::WorldHover { object_id } => {
                let target = object_id.as_ref().map(|id| (id.clone(), "mesh".to_string(), 0u32));
                Emit::config(vec![
                    LowpolyConfigOperation::SetHoveredObject { object_id: object_id.clone() },
                    LowpolyConfigOperation::SetHoveredTarget { object_id: target.as_ref().map(|(id, _, _)| id.clone()), mode: target.as_ref().map(|(_, mode, _)| mode.clone()), id: target.as_ref().map(|(_, _, id)| *id) },
                ])
            }
            LowpolyCommand::SetHover { object_id, mode, id } => {
                Emit::config(vec![LowpolyConfigOperation::SetHoveredObject { object_id: object_id.clone() }, LowpolyConfigOperation::SetHoveredTarget { object_id: object_id.clone(), mode: mode.clone(), id: *id }])
            }
            LowpolyCommand::WorldPick { granularity, merge, id } => match id {
                None => {
                    if merge == "replace" {
                        let keys = selection_keys_for(projection, config, &config.selection_mode, &[]);
                        Emit::config(vec![LowpolyConfigOperation::SetSelection { mode: config.selection_mode.clone(), ids: Vec::new() }, LowpolyConfigOperation::SetSelectionKeys { keys }])
                    } else {
                        Emit::default()
                    }
                }
                Some(id) => {
                    let (mode, ids, keys, targets) = apply_component_selection(config, projection, granularity, &[*id], merge);
                    Emit::config(vec![
                        LowpolyConfigOperation::SetSelectionTargets { mesh: targets.mesh, vertex: targets.vertex, edge: targets.edge, face: targets.face },
                        LowpolyConfigOperation::SetSelection { mode, ids },
                        LowpolyConfigOperation::SetSelectionKeys { keys },
                    ])
                }
            },
            LowpolyCommand::PaintStrokeBegin => {
                *self.stroke_drag_active.borrow_mut() = true;
                *self.stroke.borrow_mut() = None;
                Emit::default()
            }
            LowpolyCommand::PaintSample { object_id, u, v, x, y } => {
                let Some((uu, vv)) = paint_uv_from_command(*u, *v, *x, *y) else { return Emit::default() };
                let object_id = object_id.clone().unwrap_or_else(|| resolve_active_object_id(projection, config));
                let Some(object) = projection.objects.iter().find(|object| object.id == object_id) else { return Emit::default() };
                let composite = composite_layer_pixels(&object.paint_layers);
                let color = sample_pixel_from(&composite, uu, vv);
                Emit::config(vec![LowpolyConfigOperation::SetPaintColor { r: color[0], g: color[1], b: color[2], a: color[3] }])
            }
            LowpolyCommand::TransformBegin => {
                *self.transform_drag_active.borrow_mut() = true;
                *self.transform.borrow_mut() = None;
                Emit::default()
            }
            LowpolyCommand::SetActiveUtility { utility_id } => {
                *self.stroke.borrow_mut() = None;
                *self.stroke_drag_active.borrow_mut() = false;
                *self.transform.borrow_mut() = None;
                *self.transform_drag_active.borrow_mut() = false;
                let mut config_operations =
                    vec![LowpolyConfigOperation::SetActiveUtility { utility_id: utility_id.clone() }, LowpolyConfigOperation::SetHoveredTarget { object_id: None, mode: None, id: None }, LowpolyConfigOperation::SetHoveredObject { object_id: None }];
                if is_paint_utility(utility_id) {
                    config_operations.push(LowpolyConfigOperation::SetPaintUtility { value: utility_id.clone() });
                }
                Emit::config(config_operations)
            }
            //#endregion 👁️ View (config-only) commands

            //#region ✏️ Paint operations
            LowpolyCommand::PaintStrokeEnd => {
                *self.stroke_drag_active.borrow_mut() = false;
                self.commit_stroke()
            }
            LowpolyCommand::TransformEnd => {
                *self.transform_drag_active.borrow_mut() = false;
                self.commit_transform()
            }
            LowpolyCommand::PaintStroke { object_id, u, v, x, y } | LowpolyCommand::PaintAt { object_id, u, v, x, y } | LowpolyCommand::CanvasPointerDown { object_id, u, v, x, y } => {
                let Some((uu, vv)) = paint_uv_from_command(*u, *v, *x, *y) else { return Emit::default() };
                let object_id = object_id.clone().unwrap_or_else(|| resolve_active_object_id(projection, config));
                self.paint_tick(projection, config, object_id, uu, vv)
            }
            LowpolyCommand::CanvasPointerMove { object_id, u, v, x, y } => {
                if !*self.stroke_drag_active.borrow() {
                    return Emit::default();
                }
                let Some((uu, vv)) = paint_uv_from_command(*u, *v, *x, *y) else { return Emit::default() };
                let object_id = object_id.clone().unwrap_or_else(|| resolve_active_object_id(projection, config));
                self.paint_tick(projection, config, object_id, uu, vv)
            }
            LowpolyCommand::PaintFill { object_id, u, v, x, y } | LowpolyCommand::FillBucket { object_id, u, v, x, y } => {
                let Some((uu, vv)) = paint_uv_from_command(*u, *v, *x, *y) else { return Emit::default() };
                let object_id = object_id.clone().unwrap_or_else(|| resolve_active_object_id(projection, config));
                self.fill_at(projection, config, object_id, uu, vv)
            }
            LowpolyCommand::AddPaintLayer { object_id, name } => {
                let object_id = object_id.clone().unwrap_or_else(|| resolve_active_object_id(projection, config));
                let name = name.as_deref().unwrap_or("Layer");
                let index = projection.objects.iter().find(|object| object.id == object_id).map(|object| object.paint_layers.len()).unwrap_or(0);
                Emit::operations(vec![LowpolyOperation::AddPaintLayer { object_id, index, layer: LowpolyPaintLayer::new(name) }])
            }
            //#endregion ✏️ Paint operations

            //#region ✏️ Object + mesh operations
            LowpolyCommand::AddPrimitive { kind } => {
                let kind = primitive_kind(kind.as_deref().unwrap_or("box")).to_string();
                let Some(mut build) = build_doc(projection, config) else { return Emit::default() };
                let Ok(new_id) = build.add_primitive(&kind) else { return Emit::default() };
                if build.sync_meshes_to_projection().is_err() {
                    return Emit::default();
                }
                let Some(new_object) = build.projection().objects.iter().find(|object| object.id == new_id).cloned() else {
                    return Emit::default();
                };
                let index = projection.objects.len();
                Emit {
                    document_operations: vec![LowpolyOperation::ObjectsAdd { index, item: new_object }],
                    config_operations: vec![
                        LowpolyConfigOperation::SetActiveObject { object_id: new_id },
                        LowpolyConfigOperation::SetSelectionTargets { mesh: true, vertex: false, edge: false, face: false },
                        LowpolyConfigOperation::SetSelection { mode: "mesh".into(), ids: Vec::new() },
                        LowpolyConfigOperation::SetSelectionKeys { keys: Vec::new() },
                    ],
                    ..Default::default()
                }
            }
            LowpolyCommand::PatchObject { object_id, field, value_json } => {
                let value = value_json.as_deref().and_then(|json| serde_json::from_str::<Value>(json).ok());
                let Some(object) = projection.objects.iter().find(|object| &object.id == object_id) else { return Emit::default() };
                let patch = match field.as_str() {
                    "name" => LowpolyObjectPatch { name: value.as_ref().and_then(|entry| entry.as_str()).map(str::to_string), ..Default::default() },
                    "smoothShading" => LowpolyObjectPatch { smooth_shading: Some(value.as_ref().and_then(|entry| entry.as_bool()).unwrap_or(!object.smooth_shading)), ..Default::default() },
                    _ => LowpolyObjectPatch::default(),
                };
                if patch == LowpolyObjectPatch::default() {
                    return Emit::default();
                }
                Emit::operations(vec![LowpolyOperation::ObjectsPatch { id: object_id.clone(), patch }])
            }
            LowpolyCommand::Extrude { extrude_distance } => {
                let params = utility_params_value(config);
                let distance = extrude_distance.unwrap_or_else(|| utility_param_f32(&params, "extrudeDistance", 0.25));
                self.mesh_edit(projection, config, move |doc| {
                    let faces = doc.selected_face_ids();
                    if faces.is_empty() {
                        return Err("no faces selected".into());
                    }
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.extrude_faces(&faces, distance).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::Inset { inset_amount } => {
                let params = utility_params_value(config);
                let amount = inset_amount.unwrap_or_else(|| utility_param_f32(&params, "insetAmount", 0.1));
                self.mesh_edit(projection, config, move |doc| {
                    let faces = doc.selected_face_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.inset_faces(&faces, amount).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::Bevel { bevel_amount, bevel_segments } => {
                let params = utility_params_value(config);
                let amount = bevel_amount.unwrap_or_else(|| utility_param_f32(&params, "bevelAmount", 0.05));
                let segments = bevel_segments.unwrap_or_else(|| utility_param_u32(&params, "bevelSegments", 1));
                self.mesh_edit(projection, config, move |doc| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.bevel_edges(&edges, amount, segments).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::LoopCut { loop_cuts } => {
                let params = utility_params_value(config);
                let cuts = loop_cuts.unwrap_or_else(|| utility_param_u32(&params, "loopCuts", 1));
                self.mesh_edit(projection, config, move |doc| {
                    let edges = doc.selected_edge_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.loop_cut(&edges, cuts).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::Subdivide => self.mesh_edit(projection, config, move |doc| {
                let faces = doc.selected_face_ids();
                doc.active_mesh_mut().map_err(|e| e.to_string())?.subdivide_faces(&faces).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            LowpolyCommand::Triangulate => self.mesh_edit(projection, config, move |doc| {
                doc.active_mesh_mut().map_err(|e| e.to_string())?.triangulate().map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            LowpolyCommand::Mirror { axis } => {
                let params = utility_params_value(config);
                let axis = axis
                    .as_deref()
                    .map(|value| match value {
                        "y" => MirrorAxis::Y,
                        "z" => MirrorAxis::Z,
                        _ => MirrorAxis::X,
                    })
                    .unwrap_or_else(|| mirror_axis_from_param(&params));
                self.mesh_edit(projection, config, move |doc| {
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.mirror(axis, 0.001).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::Decimate { decimate_ratio } => {
                let params = utility_params_value(config);
                let ratio = decimate_ratio.unwrap_or_else(|| utility_param_f32(&params, "decimateRatio", 0.5));
                self.mesh_edit(projection, config, move |doc| {
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.decimate(ratio).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::FlipFaces { face_ids } => {
                let face_ids = face_ids.clone();
                self.mesh_edit(projection, config, move |doc| {
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
            LowpolyCommand::Merge => self.mesh_edit(projection, config, move |doc| {
                let verts = doc.selected_vertex_ids();
                doc.active_mesh_mut().map_err(|e| e.to_string())?.merge_vertices(&verts, WeldMode::Center, 0.001).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            LowpolyCommand::Dissolve => self.mesh_edit(projection, config, move |doc| {
                let edges = doc.selected_edge_ids();
                doc.active_mesh_mut().map_err(|e| e.to_string())?.dissolve_edges(&edges).map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            LowpolyCommand::Snap => {
                let params = utility_params_value(config);
                let grid = utility_param_f32(&params, "snapGrid", 0.25);
                self.mesh_edit(projection, config, move |doc| {
                    let verts = doc.selected_vertex_ids();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.snap_vertices_to_grid(&verts, grid).map_err(map_kernel_err)?;
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::ToggleSmooth => self.mesh_edit(projection, config, move |doc| {
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
            LowpolyCommand::UnwrapActive => self.mesh_edit(projection, config, move |doc| {
                doc.active_mesh_mut().map_err(|e| e.to_string())?.unwrap_uv().map_err(map_kernel_err)?;
                doc.sync_meshes_to_projection().map_err(|e| e.to_string())
            }),
            LowpolyCommand::MarkUvSeam { seam, edge_ids } => {
                let seam = seam.unwrap_or(true);
                let edge_ids = edge_ids.clone().unwrap_or_else(|| config.selection_ids.clone());
                self.mesh_edit(projection, config, move |doc| {
                    let edges: Vec<EdgeId> = edge_ids.into_iter().map(EdgeId).collect();
                    doc.active_mesh_mut().map_err(|e| e.to_string())?.mark_uv_seam(&edges, seam);
                    doc.sync_meshes_to_projection().map_err(|e| e.to_string())
                })
            }
            LowpolyCommand::ClearSeam => self.handle(&LowpolyCommand::MarkUvSeam { seam: Some(false), edge_ids: None }, doc, cfg),
            LowpolyCommand::TranslateSelection { mode, ids, dx, dy, dz } => {
                let mode = mode.clone().unwrap_or_else(|| "mesh".into());
                let ids = ids.clone().unwrap_or_default();
                self.transform_selection(projection, config, &mode, ids, Transform::Translate(Vec3::new(*dx, *dy, *dz)), "Translate selection")
            }
            LowpolyCommand::RotateSelection { mode, ids, ax, ay, az, angle } => {
                let mode = mode.clone().unwrap_or_else(|| "mesh".into());
                let ids = ids.clone().unwrap_or_default();
                self.transform_selection(projection, config, &mode, ids, Transform::Rotate { axis: Vec3::new(*ax, *ay, *az), angle: *angle }, "Rotate selection")
            }
            LowpolyCommand::ScaleSelection { mode, ids, sx, sy, sz } => {
                let mode = mode.clone().unwrap_or_else(|| "mesh".into());
                let ids = ids.clone().unwrap_or_default();
                self.transform_selection(projection, config, &mode, ids, Transform::Scale(Vec3::new(*sx, *sy, *sz)), "Scale selection")
            }
            LowpolyCommand::SetProjectionJson { json } | LowpolyCommand::SetFixtureJson { json } => match serde_json::from_str::<LowpolyProjection>(json) {
                Ok(parsed) => Ok(Emit::operations(vec![LowpolyOperation::SetProjection { projection: parsed }]),
                Err(_) => Ok(Emit::default(),
            },
            LowpolyCommand::EngagementSubmit { value } => {
                const ENGAGEMENT_COMMANDS: &[&str] = &["extrude", "inset", "bevel", "loopCut", "subdivide", "triangulate", "mirror", "decimate", "flipFaces", "merge", "dissolve", "snap"];
                let Some(typed) = value.as_deref().map(str::trim).filter(|value| !value.is_empty()) else {
                    return Emit::default();
                };
                let Some(&resolved) = ENGAGEMENT_COMMANDS.iter().find(|candidate| engagement_token_matches(typed, candidate)) else {
                    return Emit::default();
                };
                let resolved_command = match resolved {
                    "extrude" => LowpolyCommand::Extrude { extrude_distance: None },
                    "inset" => LowpolyCommand::Inset { inset_amount: None },
                    "bevel" => LowpolyCommand::Bevel { bevel_amount: None, bevel_segments: None },
                    "loopCut" => LowpolyCommand::LoopCut { loop_cuts: None },
                    "subdivide" => LowpolyCommand::Subdivide,
                    "triangulate" => LowpolyCommand::Triangulate,
                    "mirror" => LowpolyCommand::Mirror { axis: None },
                    "decimate" => LowpolyCommand::Decimate { decimate_ratio: None },
                    "flipFaces" => LowpolyCommand::FlipFaces { face_ids: Vec::new() },
                    "merge" => LowpolyCommand::Merge,
                    "dissolve" => LowpolyCommand::Dissolve,
                    "snap" => LowpolyCommand::Snap,
                    _ => return Emit::default(),
                };
                self.handle(&resolved_command, doc, cfg)
            } //#endregion ✏️ Object + mesh operations
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>) -> UiNode {
        let projection = doc.projection;
        let config = cfg.projection;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>(&config.locale);
        let active_utility = config.active_utility_id.as_str();
        let scratch_projection = self.transform.borrow().as_ref().map(|session| session.doc.projection().clone());
        let render_projection = scratch_projection.as_ref().unwrap_or(projection);
        let view = LowpolyView { projection: render_projection, config };
        if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
            self.refresh_texture_cache(projection);
        }
        let texture_cache = self.texture_cache.borrow().textures.clone();
        let loaded = matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV | LOWPOLY_PLAY_BODY_DOCUMENT).then(|| build_doc(projection, config)).flatten();
        match body_key {
            LOWPOLY_PLAY_BODY_MAIN => match &loaded {
                Some(loaded) => build_world_3d_scene(
                    LOWPOLY_PLAY_SURFACE_MAIN,
                    LOWPOLY_PLAY_APP_ID,
                    world3d_scene(lowpoly_world_camera_json(config), world_meshes_json(loaded, &texture_cache), world_instances_json(view), world_selection_json_for(view, active_utility, Some(loaded)), &lowpoly_sun_config(config)),
                ),
                None => ui_text(Label::data("Failed to load lowpoly document")),
            },
            LOWPOLY_PLAY_BODY_UV => match &loaded {
                Some(loaded) => build_canvas_2d_scene(LOWPOLY_PLAY_SURFACE_UV, LOWPOLY_PLAY_APP_ID, Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: uv_canvas_layers_json(loaded, view, &texture_cache) }),
                None => ui_text(Label::data("Failed to load UV canvas")),
            },
            LOWPOLY_PLAY_BODY_DOCUMENT => match &loaded {
                Some(loaded) => build_document_tree(view, loaded, labels),
                None => ui_text(Label::data("Failed to load lowpoly document")),
            },
            LOWPOLY_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            LOWPOLY_PLAY_BODY_INSPECTION => build_inspector_tree(view, active_utility, labels),
            LOWPOLY_PLAY_BODY_LAYERS => build_layers_tree(view, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        let active_utility = config.active_utility_id.as_str();
        let labels = semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>(&config.locale);
        let engagement = lowpoly_window_engagement(LowpolyView { projection: doc.projection, config }, active_utility, labels);
        HashMap::from([(LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()), (LOWPOLY_PLAY_WINDOW_UV.into(), engagement)])
    }

    fn window_measures(&self, _doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>(&config.locale);
        let measures = lowpoly_window_measures(config, labels);
        HashMap::from([(LOWPOLY_PLAY_WINDOW_MAIN.into(), measures.clone()), (LOWPOLY_PLAY_WINDOW_UV.into(), measures)])
    }
}

//#endregion 🔖️LowpolyPlayApp

//#region 🔖️Manifest
/// 🧰️ One transform/paint utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn lowpoly_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(id, label, icon) }
}

pub fn create_lowpoly_app() -> App {
    let default_example = serde_json::to_string(&lowpoly_engine::default_projection()).expect("lowpoly default example");
    let engagement = {
        let projection = lowpoly_engine::default_projection();
        let config = LowpolyConfig::default();
        lowpoly_window_engagement(LowpolyView { projection: &projection, config: &config }, LOWPOLY_TRANSFORM_UTILITY_DEFAULT, &LowpolyLabels::NATIVE_EN)
    };
    App::from_builder(
        App::builder(LOWPOLY_PLAY_APP_ID, LocalizedLabel::native("Lowpoly", "Lowpoly"))
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
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .mode("paint", LocalizedLabel::native("Paint", "Malen"), "paintbrush")
            .mode_layout("paint", "lowpoly-paint")
            .default_mode_id("edit")
            .window_kind_with_engagement(LOWPOLY_PLAY_WINDOW_MAIN, LocalizedLabel::native("Model", "Modell"), LOWPOLY_PLAY_BODY_MAIN, SurfaceKind::World3d, engagement.clone(), "lowpoly-model")
            .window_kind_with_engagement(LOWPOLY_PLAY_WINDOW_UV, LocalizedLabel::native("UV", "UV"), LOWPOLY_PLAY_BODY_UV, SurfaceKind::Canvas2d, engagement, "layout-grid")
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
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, LOWPOLY_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, LOWPOLY_PLAY_BODY_CATALOGUE)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, LOWPOLY_PLAY_BODY_INSPECTION)
            .panel_tab("framework.panel.layers", LocalizedLabel::native("Layers", "Ebenen"), PanelGroup::Workbench, LOWPOLY_PLAY_BODY_LAYERS)
            // 🔧️ Document-mutating operations — dispatched as VCS operations with true inverses.
            .operation("addPrimitive", LocalizedLabel::native("Add Primitive", "Primitive hinzufügen"))
            .operation("patchObject", LocalizedLabel::native("Patch Object", "Objekt aktualisieren"))
            .operation("extrude", LocalizedLabel::native("Extrude", "Extrudieren"))
            .operation("inset", LocalizedLabel::native("Inset", "Einziehen"))
            .operation("bevel", LocalizedLabel::native("Bevel", "Fasen"))
            .operation("loopCut", LocalizedLabel::native("Loop Cut", "Schleifenschnitt"))
            .operation("subdivide", LocalizedLabel::native("Subdivide", "Unterteilen"))
            .operation("triangulate", LocalizedLabel::native("Triangulate", "Triangulieren"))
            .operation("mirror", LocalizedLabel::native("Mirror", "Spiegeln"))
            .operation("decimate", LocalizedLabel::native("Decimate", "Dezimieren"))
            .operation("flipFaces", LocalizedLabel::native("Flip Faces", "Flächen umkehren"))
            .operation("merge", LocalizedLabel::native("Merge", "Zusammenführen"))
            .operation("dissolve", LocalizedLabel::native("Dissolve", "Auflösen"))
            .operation("snap", LocalizedLabel::native("Snap", "Einrasten"))
            .operation("toggleSmooth", LocalizedLabel::native("Toggle Smooth", "Glättung umschalten"))
            .operation("unwrapActive", LocalizedLabel::native("Unwrap", "Abwickeln"))
            .operation("markUvSeam", LocalizedLabel::native("Mark Seam", "Naht markieren"))
            .operation("clearSeam", LocalizedLabel::native("Clear Seam", "Naht entfernen"))
            .operation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .operation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .operation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .operation("transformEnd", LocalizedLabel::native("Transform End", "Transformation beenden"))
            .operation("addPaintLayer", LocalizedLabel::native("Add Paint Layer", "Malebene hinzufügen"))
            .operation("paintStrokeEnd", LocalizedLabel::native("Paint Stroke End", "Malstrich beenden"))
            .operation("paintFill", LocalizedLabel::native("Paint Fill", "Füllen malen"))
            .operation("fillBucket", LocalizedLabel::native("Fill Bucket", "Fülleimer"))
            .operation("setProjectionJson", LocalizedLabel::native("Set Projection Json", "Projektions-JSON festlegen"))
            .operation("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"))
            .operation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            // 👁️ Ephemeral view state — selection, camera, hover, and the gesture drafts that emit no operations
            // mid-drag (paint ticks, gumball scratch, eyedropper sample).
            .view_action("setActiveObject", LocalizedLabel::native("Set Active Object", "Aktives Objekt festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("toggleSelectionKind", LocalizedLabel::native("Toggle Selection Kind", "Auswahlart umschalten"))
            .view_action("toggleSelectionTarget", LocalizedLabel::native("Toggle Selection Target", "Auswahlziel umschalten"))
            .view_action("setActivePaintLayer", LocalizedLabel::native("Set Active Paint Layer", "Aktive Malebene festlegen"))
            .view_action("setUtilityParam", LocalizedLabel::native("Set Utility Param", "Werkzeugparameter festlegen"))
            .view_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"))
            .view_action("toggleShowEdges", LocalizedLabel::native("Toggle Show Edges", "Kantenanzeige umschalten"))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"))
            .view_action("setSelectionModeDefault", LocalizedLabel::native("Set Selection Mode Default", "Standardauswahlmodus festlegen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen"))
            .view_action("worldHover", LocalizedLabel::native("World Hover", "Überfahren (Welt)"))
            .view_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"))
            .view_action("worldPick", LocalizedLabel::native("World Pick", "Welt-Auswahl (Pick)"))
            .view_action("paintStrokeBegin", LocalizedLabel::native("Paint Stroke Begin", "Malstrich beginnen"))
            .view_action("paintStroke", LocalizedLabel::native("Paint Stroke", "Malstrich"))
            .view_action("paintAt", LocalizedLabel::native("Paint At", "Malen bei"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
            .view_action("paintSample", LocalizedLabel::native("Paint Sample", "Farbe aufnehmen"))
            .view_action("transformBegin", LocalizedLabel::native("Transform Begin", "Transformation beginnen"))
            // 📝️ Staged argument forms for the P1 actions — the panel form seeds from these defaults and
            // stages typed overrides read out of `args`; `config.utility_params_json` remains the live backing store.
            .action_args("extrude", vec![ActionArgDef::slider("extrudeDistance", LocalizedLabel::native("Extrude Distance", "Extrusionsabstand"), 0.01, 2.0).default_value(0.25)])
            .action_args("inset", vec![ActionArgDef::number("insetAmount", LocalizedLabel::native("Inset Amount", "Einzugsbetrag")).default_value(0.1)])
            .action_args("bevel", vec![
                ActionArgDef::number("bevelAmount", LocalizedLabel::native("Bevel Amount", "Fasenbetrag")).default_value(0.05),
                ActionArgDef::number("bevelSegments", LocalizedLabel::native("Bevel Segments", "Fasensegmente")).default_value(1),
            ])
            .action_args("loopCut", vec![ActionArgDef::number("loopCuts", LocalizedLabel::native("Loop Cuts", "Schleifenschnitte")).default_value(1)])
            .action_args("decimate", vec![ActionArgDef::slider("decimateRatio", LocalizedLabel::native("Decimate Ratio", "Dezimierungsverhältnis"), 0.05, 1.0).default_value(0.5)])
            .action_args("mirror", vec![ActionArgDef::select("axis", LocalizedLabel::native("Axis", "Achse"), vec![
                ActionArgOption::new("x", LocalizedLabel::native("X", "X")),
                ActionArgOption::new("y", LocalizedLabel::native("Y", "Y")),
                ActionArgOption::new("z", LocalizedLabel::native("Z", "Z")),
            ]).default_value("x")])
            .action_args("addPrimitive", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("box", LocalizedLabel::native("Cube", "Würfel")),
                ActionArgOption::new("plane", LocalizedLabel::native("Plane", "Ebene")),
                ActionArgOption::new("cylinder", LocalizedLabel::native("Cylinder", "Zylinder")),
                ActionArgOption::new("cone", LocalizedLabel::native("Cone", "Kegel")),
                ActionArgOption::new("ico_sphere", LocalizedLabel::native("Ico Sphere", "Ikokugel")),
            ]).default_value("box")])
            .action_args("markUvSeam", vec![ActionArgDef::toggle("seam", LocalizedLabel::native("Seam", "Naht")).default_value(true)])
            // 🧰️ Transform gumball + paint utilities — exclusive per-window active utility is host-owned (never a
            // document operation). Selection method/merge/kind live as an always-visible Select window-options group
            // (mirrors puzzle 3d); the transform group defaults to "move", paint bridges into `config.paint_utility`.
            .utility(lowpoly_utility("move", LocalizedLabel::native("Move", "Verschieben"), "move", "transform"))
            .utility(lowpoly_utility("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw", "transform"))
            .utility(lowpoly_utility("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2", "transform"))
            .utility(lowpoly_utility("brush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "paint"))
            .utility(lowpoly_utility("eraser", LocalizedLabel::native("Eraser", "Radierer"), "eraser", "paint"))
            .utility(lowpoly_utility("fill", LocalizedLabel::native("Fill", "Füllen"), "paint-bucket", "paint"))
            .utility(lowpoly_utility("eyedropper", LocalizedLabel::native("Eyedropper", "Pipette"), "pipette", "paint"))
            .window_kind_utilities(LOWPOLY_PLAY_WINDOW_MAIN, vec![
                "move".into(), "rotate".into(), "scale".into(),
                "brush".into(), "eraser".into(), "fill".into(), "eyedropper".into(),
            ])
            .window_kind_utilities(LOWPOLY_PLAY_WINDOW_UV, vec![
                "brush".into(), "eraser".into(), "fill".into(), "eyedropper".into(),
            ])
            // 📇️ Per-window action scoping — MAIN (World3d) owns every mesh-editing/transform/UV-unwrap
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
            .keybinding("mod+shift+z", "redo")
            .config(LowpolyPlayApp::default().config_spec())
            .io(lowpoly_engine::lowpoly_io()),
    )
    .example("default", LocalizedLabel::native("Default", "Standard"), &default_example, "file")
    .workflow("lowpoly", "Lowpoly", "mesh")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, ViewState};

    fn new_app() -> VcsDocumentApp<LowpolyPlayApp> {
        testkit::new_app::<LowpolyPlayApp>()
    }

    fn projection(app: &VcsDocumentApp<LowpolyPlayApp>) -> LowpolyProjection {
        app.projection().expect("projection")
    }

    fn face_selection() -> LowpolyCommand {
        LowpolyCommand::WorldPick { granularity: "face".into(), merge: "replace".into(), id: Some(0) }
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
            semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
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
        let measures = lowpoly_window_measures(&LowpolyConfig::default(), &LowpolyLabels::NATIVE_EN);
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
        // 🧹️ Snap grid stays a general (untagged) measure — it is not a single-utility parameter.
        assert_eq!(group_tag("lowpoly-measure-snap"), Some(None));
        // 🎯️ Select options are always-visible window options (untagged), matching puzzle 3d.
        assert_eq!(group_tag("lowpoly-select"), Some(None));
        // 🗑️ The mesh-operation param sliders now live ONLY in the Action Panel's staged `action_args`, never a measure.
        let json = serde_json::to_string(&measures).unwrap();
        for removed in ["lowpoly-measure-extrude", "lowpoly-measure-inset", "lowpoly-measure-bevel", "lowpoly-measure-bevel-segments", "lowpoly-measure-loop-cuts", "lowpoly-measure-decimate", "lowpoly-measure-mirror"] {
            assert!(!json.contains(removed), "mesh-operation measure {removed} must be gone (covered by action_args)");
        }
    }

    #[test]
    fn select_window_options_mirror_puzzle3d_taxonomy() {
        let measures = lowpoly_window_measures(&LowpolyConfig::default(), &LowpolyLabels::NATIVE_EN);
        let select = measures
            .iter()
            .find_map(|measure| match measure {
                WindowMeasure::Group { id, children, active_utility_id, .. } if id == "lowpoly-select" => Some((active_utility_id.clone(), children.clone())),
                _ => None,
            })
            .expect("lowpoly-select group");
        assert_eq!(select.0, None, "Select options must always surface in window options");
        let toggle_ids: Vec<&str> = select
            .1
            .iter()
            .filter_map(|measure| match measure {
                WindowMeasure::Toggle { id, .. } => Some(id.as_str()),
                _ => None,
            })
            .collect();
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
        app.dispatch_typed(LowpolyCommand::SetSelectionMethod { value: "lasso".into() }, &testkit::meta("a")).unwrap();
        app.dispatch_typed(LowpolyCommand::SetSelectionModeDefault { value: "additive".into() }, &testkit::meta("a")).unwrap();
        let measures = app.window_measures();
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

        let config = LowpolyConfig { selection_method: "lasso".into(), selection_mode_default: "additive".into(), ..LowpolyConfig::default() };
        let selection: Value = serde_json::from_str(&world_selection_json_for(LowpolyView { projection: &projection(&app), config: &config }, "move", None)).unwrap();
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
        app.dispatch_typed(LowpolyCommand::AddPrimitive { kind: Some("box".into()) }, &testkit::meta("a")).unwrap();
        let projection = projection(&app);
        assert_eq!(projection.objects.len(), 2);
        assert!(projection.objects.iter().any(|object| object.name == "box"));
    }

    #[test]
    fn extrude_selected_face_grows_mesh_and_undo_restores() {
        let mut app = new_app();
        let object_id = projection(&app).objects[0].id.clone();
        let before = LowpolyDocument::new(projection(&app)).unwrap().active_mesh().unwrap().face_count();
        app.dispatch_typed(face_selection(), &testkit::meta("a")).unwrap();
        app.dispatch_typed(LowpolyCommand::Extrude { extrude_distance: None }, &testkit::meta("a")).unwrap();
        let after = LowpolyDocument::with_context(projection(&app), object_id.clone(), LowpolySelection::default()).unwrap().active_mesh().unwrap().face_count();
        assert!(after > before);
        app.handle_action("undo", None, &testkit::meta("a")).unwrap();
        let restored = LowpolyDocument::with_context(projection(&app), object_id, LowpolySelection::default()).unwrap().active_mesh().unwrap().face_count();
        assert_eq!(restored, before);
    }

    #[test]
    fn selection_is_view_state_and_emits_no_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(face_selection(), &testkit::meta("a")).unwrap();
        assert!(result.operations.is_empty(), "picking must not create an undoable operation");
    }

    #[test]
    fn paint_stroke_drag_is_one_undo_step_with_pixel_restoration() {
        let mut app = new_app();
        let object_id = projection(&app).objects[0].id.clone();
        let before = projection(&app).objects[0].paint_layers[0].pixels.clone();
        // begin → tick → tick → end : one undoable PaintStroke edit.
        app.dispatch_typed(LowpolyCommand::PaintStrokeBegin, &testkit::meta("a")).unwrap();
        let tick_a = app.dispatch_typed(LowpolyCommand::PaintAt { object_id: Some(object_id.clone()), u: Some(0.5), v: Some(0.5), x: None, y: None }, &testkit::meta("a")).unwrap();
        let tick_b = app.dispatch_typed(LowpolyCommand::PaintAt { object_id: Some(object_id.clone()), u: Some(0.52), v: Some(0.5), x: None, y: None }, &testkit::meta("a")).unwrap();
        assert!(tick_a.operations.is_empty() && tick_b.operations.is_empty(), "mid-drag ticks emit no operations");
        let end = app.dispatch_typed(LowpolyCommand::PaintStrokeEnd, &testkit::meta("a")).unwrap();
        assert_eq!(end.operations.len(), 1, "the whole drag commits as one operation");
        let painted = projection(&app).objects[0].paint_layers[0].pixels.clone();
        assert_ne!(painted, before, "the stroke changed pixels");
        // ONE undo restores the exact prior pixels.
        app.handle_action("undo", None, &testkit::meta("a")).unwrap();
        let restored = projection(&app).objects[0].paint_layers[0].pixels.clone();
        assert_eq!(restored, before, "undo restores the exact pre-stroke pixels");
        // Redo re-applies.
        app.handle_action("redo", None, &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].paint_layers[0].pixels, painted);
    }

    #[test]
    fn eyedropper_updates_paint_color_without_operations() {
        let mut app = new_app();
        // 🧰️ The host-owned utility switch bridges into config.paint_utility and emits no operations.
        let switch = app.dispatch_typed(LowpolyCommand::SetActiveUtility { utility_id: "eyedropper".into() }, &testkit::meta("a")).unwrap();
        assert!(switch.operations.is_empty());
        let result = app.dispatch_typed(LowpolyCommand::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }, &testkit::meta("a")).unwrap();
        assert!(result.operations.is_empty());
    }

    #[test]
    fn toggle_smooth_emits_op_and_flips_shading() {
        let mut app = new_app();
        let before = projection(&app).objects[0].smooth_shading;
        app.dispatch_typed(LowpolyCommand::ToggleSmooth, &testkit::meta("a")).unwrap();
        assert_ne!(projection(&app).objects[0].smooth_shading, before);
    }

    #[test]
    fn add_paint_layer_emits_operation() {
        let mut app = new_app();
        let before = projection(&app).objects[0].paint_layers.len();
        app.dispatch_typed(LowpolyCommand::AddPaintLayer { object_id: None, name: Some("Detail".into()) }, &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].paint_layers.len(), before + 1);
    }

    #[test]
    fn extrude_reads_staged_arg_distance_into_the_operation() {
        // 🧪️ Arg-form action: the staged `extrudeDistance` (not the config backing store) drives the edit.
        let mut small = new_app();
        let mut large = new_app();
        small.dispatch_typed(face_selection(), &testkit::meta("a")).unwrap();
        large.dispatch_typed(face_selection(), &testkit::meta("a")).unwrap();
        let object_id = projection(&small).objects[0].id.clone();
        small.dispatch_typed(LowpolyCommand::Extrude { extrude_distance: Some(0.1) }, &testkit::meta("a")).unwrap();
        large.dispatch_typed(LowpolyCommand::Extrude { extrude_distance: Some(1.5) }, &testkit::meta("a")).unwrap();
        let small_json = projection(&small).objects.iter().find(|o| o.id == object_id).unwrap().mesh_json.clone();
        let large_json = projection(&large).objects.iter().find(|o| o.id == object_id).unwrap().mesh_json.clone();
        assert_ne!(small_json, large_json, "different staged extrude distances must produce different meshes");
    }

    #[test]
    fn active_utility_switch_emits_no_ops_and_no_history() {
        // 🧰️ Selecting a host-owned utility must never create an undoable edit.
        let mut app = new_app();
        let result = app.dispatch_typed(LowpolyCommand::SetActiveUtility { utility_id: "rotate".into() }, &testkit::meta("a")).unwrap();
        assert!(result.operations.is_empty(), "utility switch must emit no operations");
        // No history entry — an undo right after is a no-operation leaving the projection untouched.
        let before = projection(&app);
        app.handle_action("undo", None, &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app), before, "utility switch left nothing to undo");
    }

    #[test]
    fn engagement_options_contain_no_utility_switcher() {
        // 🧰️ move/rotate/scale switching lives only on the framework utility bar; the engagement keeps its
        // genuine non-utility options (snap/smooth/show-edges) but must never dispatch setActiveUtility.
        let projection = projection(&new_app());
        let config = LowpolyConfig::default();
        let engagement = lowpoly_window_engagement(LowpolyView { projection: &projection, config: &config }, "move", &LowpolyLabels::NATIVE_EN);
        let options = engagement.options.expect("lowpoly engagement keeps its non-utility options");
        assert!(
            options.iter().all(|option| option.action.as_ref().map(|action| action.action != SET_ACTIVE_UTILITY_ACTION_ID).unwrap_or(true)),
            "no engagement option may dispatch the framework setActiveUtility action; transform switching lives on the utility bar",
        );
    }

    #[test]
    fn gumball_drag_coalesces_to_one_committed_edit() {
        // 🧲️ THE COALESCING REGRESSION: a multi-tick gumball translate must emit ZERO operations mid-drag and
        // exactly ONE commit operation (base → final mesh) on drag end — never a full-mesh patch per tick.
        let mut app = new_app();
        let before_mesh = projection(&app).objects[0].mesh_json.clone();
        app.dispatch_typed(LowpolyCommand::TransformBegin, &testkit::meta("a")).unwrap();
        let tick_a = app.dispatch_typed(LowpolyCommand::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.5, dy: 0.0, dz: 0.0 }, &testkit::meta("a")).unwrap();
        let tick_b = app.dispatch_typed(LowpolyCommand::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.25, dy: 0.0, dz: 0.0 }, &testkit::meta("a")).unwrap();
        assert!(tick_a.operations.is_empty() && tick_b.operations.is_empty(), "mid-drag transform ticks emit no operations");
        assert_eq!(projection(&app).objects[0].mesh_json, before_mesh, "no operation reached the document mid-drag");
        let end = app.dispatch_typed(LowpolyCommand::TransformEnd, &testkit::meta("a")).unwrap();
        assert_eq!(end.operations.len(), 1, "the whole drag commits as exactly one operation");
        // The final diff reflects the accumulated 0.75 translation (both ticks), not just the last tick.
        let after_mesh = projection(&app).objects[0].mesh_json.clone();
        assert_ne!(after_mesh, before_mesh, "the drag moved the mesh");
        // ONE undo reverts the entire coalesced drag.
        app.handle_action("undo", None, &testkit::meta("a")).unwrap();
        assert_eq!(projection(&app).objects[0].mesh_json, before_mesh, "one undo reverts the whole coalesced gumball drag");
    }

    //#region 🔖️GesturePreview
    /// 🔬️ CW7 preview-law seam: `LowpolyPlayApp::gesture_preview` reads `TransformSession` only, never a
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
        let app = LowpolyPlayApp::default();
        let projection = lowpoly_engine::default_projection();
        let config = LowpolyConfig::default();
        *app.transform_drag_active.borrow_mut() = true;

        let tick_a = app.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(0.5, 0.0, 0.0)), "translate");
        assert!(tick_a.document_operations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let (key, seq_after_a, payload_a) = app.gesture_preview().expect("a live gumball drag is previewable");
        assert_eq!(key, "gesture:transform");
        let value_a: Value = serde_json::from_slice(&payload_a).expect("payload is valid json");
        assert_eq!(value_a["objectId"], json!(projection.objects[0].id));
        assert_ne!(value_a["patch"], json!(LowpolyObjectPatch::default()), "the patch anchored to the drag-start snapshot must reflect the first tick");

        let tick_b = app.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(0.25, 0.0, 0.0)), "translate");
        assert!(tick_b.document_operations.is_empty());
        let (_, seq_after_b, payload_b) = app.gesture_preview().expect("still live mid-drag");
        assert!(seq_after_b > seq_after_a, "seq is monotone per tick, for staleness detection on the receiving end");
        assert_ne!(payload_a, payload_b, "the base-anchored patch accumulates both ticks, not just the latest one");

        let end = app.commit_transform();
        assert_eq!(end.document_operations.len(), 1, "the whole drag commits as exactly one real operation");
        assert!(app.gesture_preview().is_none(), "the drag ended: nothing left to preview, and the commit above already carried the real operation");
    }

    #[test]
    fn gesture_preview_is_a_pure_read_never_mutating_the_transform_session() {
        let app = LowpolyPlayApp::default();
        let projection = lowpoly_engine::default_projection();
        let config = LowpolyConfig::default();
        *app.transform_drag_active.borrow_mut() = true;
        app.transform_selection(&projection, &config, "mesh", vec![], Transform::Translate(Vec3::new(1.0, 0.0, 0.0)), "translate");
        let mesh_before = app.transform.borrow().as_ref().unwrap().doc.projection().objects[0].mesh_json.clone();
        let _ = app.gesture_preview();
        let _ = app.gesture_preview();
        assert_eq!(app.transform.borrow().as_ref().unwrap().doc.projection().objects[0].mesh_json, mesh_before, "gesture_preview must never mutate the live transform scratch it reads");
    }
    //#endregion 🔖️GesturePreview

    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        // Both instances start from the identical default projection; disjoint edits (a rename on one,
        // an added primitive on the other) must converge on the shared backbone — impossible under a
        // whole-document snapshot where one write clobbers the other.
        testkit::assert_two_instances_converge::<LowpolyPlayApp, _>(
            "mem://lowpoly-convergence",
            LowpolyCommand::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Renamed By A").unwrap()) },
            LowpolyCommand::AddPrimitive { kind: Some("box".into()) },
            |app| app.projection().expect("projection"),
        );
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<LowpolyPlayApp, _>(LowpolyCommand::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Hero").unwrap()) }, |app| app.projection().expect("projection"));
    }

    //#region 🔖️MediaPorts
    #[test]
    fn export_media_mesh_out_produces_mesh_document_payload() {
        let mut app = new_app();
        let media = app.export_media("mesh:out").expect("export mesh:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh });
        match media.payload {
            MediaPayload::Structured { schema, .. } => assert_eq!(schema, "mesh.document"),
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn import_media_mesh_in_round_trips_into_set_projection() {
        let mesh = semio_framework_plugin::mesh_from_kind("box");
        let mesh_document = lowpoly_engine::mesh_document_from_mesh(&mesh).expect("mesh document");
        let json = serde_json::to_string(&mesh_document).expect("mesh document json");
        let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } };
        let app = LowpolyPlayApp::default();
        let projection = lowpoly_engine::default_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.import_media("mesh:in", &media, &doc).expect("import mesh:in");
        assert_eq!(emit.document_operations.len(), 1);
        match &emit.document_operations[0] {
            LowpolyOperation::SetProjection { projection } => assert_eq!(projection.objects.len(), 1),
            other => panic!("expected SetProjection, got {other:?}"),
        }
    }
    //#endregion 🔖️MediaPorts
}
//#endregion 🧪️Tests
