//! 🌐 Application-neutral 3D world canvas: mesh loading, orbit camera, picking, and marquee selection.

use kernel_3d_scene::{
    aabb_intersects_frustum, axis_rotate_angle, frustum_planes, gumball_extent, gumball_eye,
    gumball_project_ray_onto_axis, interpolate_mesh_uv, quat_from_basis, ray_aabb_slab,
    ray_pick_instance, ray_pick_mesh_detail, ray_plane_point, ray_segment_distance, rotate_vector,
    screen_select_components, screen_select_instances, transform_aabb, vec3_from_f64, Camera3d,
    Instance3d, LineDraw3d, LineVertex3d, Mat4, Mesh3d, OrbitController, SceneDraw3d, ScenePass3d,
    TexturedDraw3d, TexturedInstance3d, Vec3,
};
use semio_framework_core::{mesh_from_glb, mesh_from_kind, CommandDescriptor, MeshData, UiComponentSceneNode};
use base64::Engine;
use serde::de::Error as DeError;
use serde::Deserialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use ui_wgpu::{
    draw_text, mesh_content_version, GpuContext, HitKind, HitTarget, PointerModifiers, Rect, Rgba,
    WidgetContext,
};

//#region SceneRecords
fn deserialize_optional_string_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(serde_json::Value::Array(items)) => Ok(Some(
            items
                .into_iter()
                .filter_map(|item| match item {
                    serde_json::Value::String(value) => Some(value),
                    serde_json::Value::Number(value) => value.as_u64().map(|id| id.to_string()),
                    _ => None,
                })
                .collect(),
        )),
        Some(other) => Err(D::Error::custom(format!(
            "expected array for component ids, got {other}"
        ))),
    }
}

fn json_id_to_string(value: &serde_json::Value) -> Option<String> {
    value
        .as_str()
        .map(str::to_string)
        .or_else(|| value.as_u64().map(|id| id.to_string()))
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldCameraRecord {
    position: Option<[f64; 3]>,
    target: Option<[f64; 3]>,
    up: Option<[f64; 3]>,
    fov: Option<f64>,
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldMeshRecord {
    id: String,
    data: Option<MeshData>,
    url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldInstanceRecord {
    id: String,
    mesh_id: Option<String>,
    position: Option<[f64; 3]>,
    rotation: Option<[f64; 4]>,
    scale: Option<[f64; 3]>,
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    color: Option<String>,
    selected: Option<bool>,
    hovered: Option<bool>,
    label: Option<String>,
    smooth_shading: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldSelectionRecord {
    method: Option<String>,
    mode: Option<String>,
    ids: Option<Vec<String>>,
    hovered_id: Option<String>,
    granularity: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string_vec")]
    component_ids: Option<Vec<String>>,
    transform_tool: Option<String>,
    interaction_mode: Option<String>,
    gumball_target: Option<[f64; 3]>,
    selection_mode: Option<String>,
    hovered_component: Option<serde_json::Value>,
    show_edges: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldVortexRecord {
    full_id: String,
    position: Option<[f64; 3]>,
    radius: Option<f64>,
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldAttractionRecord {
    from: Option<[f64; 3]>,
    to: Option<[f64; 3]>,
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldTargetVolumeRecord {
    origin: Option<[f64; 3]>,
    orientation: Option<[f64; 4]>,
    scale: Option<[f64; 3]>,
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldReferenceRecord {
    id: String,
    url: Option<String>,
    origin: Option<[f64; 3]>,
    width_world: Option<f64>,
    hidden: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldBrushPreviewRecord {
    mesh_url: Option<String>,
    origin: Option<[f64; 3]>,
    orientation: Option<[f64; 4]>,
    scale: Option<serde_json::Value>,
    target_vortex_full_id: Option<String>,
    object_kind_id: Option<String>,
    source_vortex_index: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldInteractionRecord {
    active_tool: Option<String>,
    brush_candidate_index: Option<usize>,
    hovered_vortex_full_id: Option<String>,
}
//#endregion SceneRecords

//#region World3dState
pub struct World3dState {
    pub surface_id: String,
    pub controller_id: String,
    pub bounds: Rect,
    pub orbit: OrbitController,
    pub meshes: HashMap<String, Mesh3d>,
    pub mesh_versions: HashMap<String, u64>,
    pub draws: Vec<SceneDraw3d>,
    pub selection_method: String,
    pub local_hover_id: Option<String>,
    pub pending_glb_urls: HashSet<String>,
    pub marquee_points: Vec<[f32; 2]>,
    pub marquee_active: bool,
    scene_camera_json: Option<String>,
    scene_meshes_json: Option<String>,
    scene_instances_json: Option<String>,
    scene_selection_json: Option<String>,
    scene_vortices_json: Option<String>,
    scene_attractions_json: Option<String>,
    scene_target_volumes_json: Option<String>,
    scene_references_json: Option<String>,
    scene_brush_preview_json: Option<String>,
    scene_interaction_json: Option<String>,
    vortices: Vec<WorldVortexRecord>,
    attractions: Vec<WorldAttractionRecord>,
    target_volumes: Vec<WorldTargetVolumeRecord>,
    references: Vec<WorldReferenceRecord>,
    brush_preview: Option<WorldBrushPreviewRecord>,
    active_tool: String,
    hovered_vortex_id: Option<String>,
    drag_object_id: Option<String>,
    drag_object_z: f32,
    drag_last_position: Option<[f32; 3]>,
    selected_ids: Vec<String>,
    transform_tool: String,
    gumball_handle: Option<GumballHandle>,
    gumball_pivot: Vec3,
    gumball_drag_anchor: f32,
    gumball_drag_start_vec: Vec3,
    gumball_preview_translate: Vec3,
    gumball_preview_angle: f32,
    gumball_preview_scale: Vec3,
    pending_image_urls: HashSet<String>,
    reference_aspect: HashMap<String, f32>,
    reference_pixels: HashMap<String, (u32, u32, Vec<u8>)>,
    granularity: String,
    component_ids: Vec<String>,
    interaction_mode: String,
    gumball_target: Option<[f32; 3]>,
    marquee_preview_ids: Vec<String>,
    paint_stroke_active: bool,
    hovered_component_id: Option<String>,
    hovered_component_object_id: Option<String>,
    hovered_component_mode: Option<String>,
    show_edges: bool,
    press_object_id: Option<String>,
    mesh_paint_textures: HashMap<String, (u32, u32, Vec<u8>)>,
}

impl World3dState {
    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            surface_id,
            controller_id,
            bounds: Rect::default(),
            orbit: OrbitController::default(),
            meshes: HashMap::new(),
            mesh_versions: HashMap::new(),
            draws: Vec::new(),
            selection_method: "rectangle".into(),
            local_hover_id: None,
            pending_glb_urls: HashSet::new(),
            marquee_points: Vec::new(),
            marquee_active: false,
            scene_camera_json: None,
            scene_meshes_json: None,
            scene_instances_json: None,
            scene_selection_json: None,
            scene_vortices_json: None,
            scene_attractions_json: None,
            scene_target_volumes_json: None,
            scene_references_json: None,
            scene_brush_preview_json: None,
            scene_interaction_json: None,
            vortices: Vec::new(),
            attractions: Vec::new(),
            target_volumes: Vec::new(),
            references: Vec::new(),
            brush_preview: None,
            active_tool: "select".into(),
            hovered_vortex_id: None,
            drag_object_id: None,
            drag_object_z: 0.0,
            drag_last_position: None,
            selected_ids: Vec::new(),
            transform_tool: "translate".into(),
            gumball_handle: None,
            gumball_pivot: Vec3::ZERO,
            gumball_drag_anchor: 0.0,
            gumball_drag_start_vec: Vec3::ZERO,
            gumball_preview_translate: Vec3::ZERO,
            gumball_preview_angle: 0.0,
            gumball_preview_scale: Vec3::new(1.0, 1.0, 1.0),
            pending_image_urls: HashSet::new(),
            reference_aspect: HashMap::new(),
            reference_pixels: HashMap::new(),
            granularity: "object".into(),
            component_ids: Vec::new(),
            interaction_mode: "model".into(),
            gumball_target: None,
            marquee_preview_ids: Vec::new(),
            paint_stroke_active: false,
            hovered_component_id: None,
            hovered_component_object_id: None,
            hovered_component_mode: None,
            show_edges: false,
            press_object_id: None,
            mesh_paint_textures: HashMap::new(),
        }
    }
}
//#endregion World3dState

//#region Gumball
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GumballHandle {
    MoveX,
    MoveY,
    MoveZ,
    MoveXY,
    MoveYZ,
    MoveXZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

impl GumballHandle {
    fn axis_dir(self) -> Option<Vec3> {
        match self {
            Self::MoveX | Self::RotateX | Self::ScaleX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveY | Self::RotateY | Self::ScaleY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::MoveZ | Self::RotateZ | Self::ScaleZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    fn plane_normal(self) -> Option<Vec3> {
        match self {
            Self::MoveXY => Some(Vec3::new(0.0, 0.0, 1.0)),
            Self::MoveYZ => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveXZ => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::RotateY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    fn is_translate(self) -> bool {
        matches!(
            self,
            Self::MoveX | Self::MoveY | Self::MoveZ | Self::MoveXY | Self::MoveYZ | Self::MoveXZ
        )
    }

    fn is_rotate(self) -> bool {
        matches!(self, Self::RotateX | Self::RotateY | Self::RotateZ)
    }

    fn is_scale(self) -> bool {
        matches!(self, Self::ScaleX | Self::ScaleY | Self::ScaleZ)
    }
}

//#region MeshHelpers
fn mesh_from_data(data: &MeshData) -> Mesh3d {
    let mut mesh = Mesh3d::from_buffers(
        data.positions.clone(),
        data.normals.clone(),
        data.indices.clone(),
    );
    mesh.face_ids = data.face_ids.clone();
    mesh.vertex_ids = data.vertex_ids.clone();
    mesh.edge_positions = data.edge_positions.clone();
    mesh.edge_ids = data.edge_ids.clone();
    mesh.uvs = data.uvs.clone();
    if let Some(texture) = data.paint_texture_base64.as_deref() {
        bake_paint_colors(&mut mesh, texture);
    }
    mesh
}

fn bake_paint_colors(mesh: &mut Mesh3d, texture_base64: &str) {
    let payload = texture_base64
        .strip_prefix("data:image/png;base64,")
        .unwrap_or(texture_base64);
    let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(payload) else {
        return;
    };
    let Ok(image) = image::load_from_memory(&bytes) else {
        return;
    };
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    if width == 0 || height == 0 || mesh.uvs.len() < mesh.positions.len() / 3 * 2 {
        return;
    }
    let mut colors = Vec::with_capacity(mesh.positions.len());
    for uv in mesh.uvs.chunks_exact(2) {
        let u = uv[0].clamp(0.0, 1.0);
        let v = uv[1].clamp(0.0, 1.0);
        let x = ((u * width as f32) as u32).min(width - 1);
        let y = ((v * height as f32) as u32).min(height - 1);
        let index = ((y * width + x) * 4) as usize;
        let pixel = &rgba.as_raw()[index..index + 4];
        colors.extend_from_slice(&[
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
            pixel[3] as f32 / 255.0,
        ]);
    }
    mesh.colors = colors;
}

fn selection_mode_label(state: &World3dState) -> &'static str {
    match state.granularity.as_str() {
        "vertex" | "edge" | "face" => "component",
        _ => "mesh",
    }
}

fn component_mode_active(state: &World3dState) -> bool {
    matches!(
        state.granularity.as_str(),
        "vertex" | "edge" | "face" | "component"
    )
}

const CLICK_DRAG_THRESHOLD_PX: f32 = 4.0;

fn pointer_drag_distance(state: &World3dState, x: f32, y: f32) -> f32 {
    let Some(start) = state.marquee_points.first() else {
        return 0.0;
    };
    let dx = x - start[0];
    let dy = y - start[1];
    (dx * dx + dy * dy).sqrt()
}

fn is_click_gesture(state: &World3dState, x: f32, y: f32) -> bool {
    pointer_drag_distance(state, x, y) <= CLICK_DRAG_THRESHOLD_PX
}

fn push_line_segment(
    lines: &mut Vec<LineVertex3d>,
    from: Vec3,
    to: Vec3,
    color: [f32; 4],
) {
    lines.push(LineVertex3d {
        position: from.to_array(),
        color,
    });
    lines.push(LineVertex3d {
        position: to.to_array(),
        color,
    });
}

fn push_vertex_marker(lines: &mut Vec<LineVertex3d>, center: Vec3, color: [f32; 4], size: f32) {
    push_line_segment(lines, center, center.add(Vec3::new(size, 0.0, 0.0)), color);
    push_line_segment(lines, center, center.add(Vec3::new(-size, 0.0, 0.0)), color);
    push_line_segment(lines, center, center.add(Vec3::new(0.0, size, 0.0)), color);
    push_line_segment(lines, center, center.add(Vec3::new(0.0, -size, 0.0)), color);
    push_line_segment(lines, center, center.add(Vec3::new(0.0, 0.0, size)), color);
    push_line_segment(lines, center, center.add(Vec3::new(0.0, 0.0, -size)), color);
}

fn mesh_vertex(mesh: &Mesh3d, index: u32) -> Vec3 {
    let i = index as usize * 3;
    Vec3::new(mesh.positions[i], mesh.positions[i + 1], mesh.positions[i + 2])
}

fn append_component_overlays(
    state: &World3dState,
    lines: &mut Vec<LineVertex3d>,
    selected: &[String],
    preview: &[String],
    hovered: &Option<String>,
) {
    if state.interaction_mode == "paint"
        || component_mode_active(state)
        || state.show_edges
        || (state.granularity == "mesh" && !state.component_ids.is_empty())
    {
        let wire_color = [0.55, 0.65, 0.8, 0.75];
        for draw in &state.draws {
            let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            if mesh.edge_positions.is_empty() {
                continue;
            }
            for instance in &draw.instances {
                for chunk in mesh.edge_positions.chunks_exact(6) {
                    let a = instance.model.transform_point(Vec3::new(
                        chunk[0], chunk[1], chunk[2],
                    ));
                    let b = instance.model.transform_point(Vec3::new(
                        chunk[3], chunk[4], chunk[5],
                    ));
                    lines.push(LineVertex3d {
                        position: a.to_array(),
                        color: wire_color,
                    });
                    lines.push(LineVertex3d {
                        position: b.to_array(),
                        color: wire_color,
                    });
                }
            }
        }
    }
    let highlight = [0.35, 0.75, 1.0, 1.0];
    let preview_color = [1.0, 0.85, 0.35, 1.0];
    let ids: HashSet<String> = selected
        .iter()
        .chain(preview.iter())
        .chain(hovered.iter())
        .cloned()
        .collect();
    if ids.is_empty() {
        return;
    }
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            match state.granularity.as_str() {
                "vertex" => {
                    for (vertex_index, chunk) in mesh.positions.chunks_exact(3).enumerate() {
                        let id = mesh
                            .vertex_ids
                            .get(vertex_index)
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| vertex_index.to_string());
                        if !ids.contains(&id) {
                            continue;
                        }
                        let color = if preview.contains(&id) {
                            preview_color
                        } else {
                            highlight
                        };
                        let center = instance.model.transform_point(Vec3::new(
                            chunk[0], chunk[1], chunk[2],
                        ));
                        push_vertex_marker(lines, center, color, 0.06);
                    }
                }
                "edge" => {
                    for (edge_index, chunk) in mesh.edge_positions.chunks_exact(6).enumerate() {
                        let id = mesh
                            .edge_ids
                            .get(edge_index)
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| edge_index.to_string());
                        if !ids.contains(&id) {
                            continue;
                        }
                        let color = if preview.contains(&id) {
                            preview_color
                        } else {
                            highlight
                        };
                        let a = instance.model.transform_point(Vec3::new(
                            chunk[0], chunk[1], chunk[2],
                        ));
                        let b = instance.model.transform_point(Vec3::new(
                            chunk[3], chunk[4], chunk[5],
                        ));
                        push_line_segment(lines, a, b, color);
                    }
                }
                "face" => {
                    for (tri_index, tri) in mesh.indices.chunks_exact(3).enumerate() {
                        let id = mesh
                            .face_ids
                            .get(tri_index)
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| tri_index.to_string());
                        if !ids.contains(&id) {
                            continue;
                        }
                        let color = if preview.contains(&id) {
                            preview_color
                        } else {
                            highlight
                        };
                        let verts = [
                            instance.model.transform_point(mesh_vertex(mesh, tri[0])),
                            instance.model.transform_point(mesh_vertex(mesh, tri[1])),
                            instance.model.transform_point(mesh_vertex(mesh, tri[2])),
                        ];
                        push_line_segment(lines, verts[0], verts[1], color);
                        push_line_segment(lines, verts[1], verts[2], color);
                        push_line_segment(lines, verts[2], verts[0], color);
                    }
                }
                _ => {}
            }
        }
    }
}

fn selection_centroid(state: &World3dState) -> Option<Vec3> {
    if let Some(target) = state.gumball_target {
        return Some(Vec3::new(target[0], target[1], target[2]));
    }
    if state.selected_ids.is_empty() {
        return None;
    }
    let mut sum = Vec3::ZERO;
    let mut count = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            if state.selected_ids.iter().any(|id| id == &instance.id) {
                let t = instance.model.cols[3];
                sum = sum.add(Vec3::new(t[0], t[1], t[2]));
                count += 1;
            }
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum.scale(1.0 / count as f32))
    }
}

fn pick_gumball_handle_at(
    state: &World3dState,
    x: f32,
    y: f32,
    inner: Rect,
) -> Option<GumballHandle> {
    let pivot = selection_centroid(state)?;
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let pick_radius = extent * 0.08;
    let eye = gumball_eye(&camera, pivot);
    let mut best: Option<(f32, GumballHandle)> = None;
    let axes = [
        (GumballHandle::MoveX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]),
        (GumballHandle::MoveY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]),
        (GumballHandle::MoveZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0]),
    ];
    for (handle, axis, _) in axes {
        let end = pivot.add(axis.scale(extent));
        if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
            if dist <= pick_radius && best.as_ref().map_or(true, |(best_dist, _)| dist < *best_dist) {
                best = Some((dist, handle));
            }
        }
    }
    let planes = [
        (GumballHandle::MoveXY, Vec3::new(0.0, 0.0, 1.0), extent * 0.35),
        (GumballHandle::MoveYZ, Vec3::new(1.0, 0.0, 0.0), extent * 0.35),
        (GumballHandle::MoveXZ, Vec3::new(0.0, 1.0, 0.0), extent * 0.35),
    ];
    for (handle, normal, half) in planes {
        if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
            let offset = hit.sub(pivot);
            let u = if normal.z.abs() > 0.9 {
                offset.x.abs()
            } else if normal.x.abs() > 0.9 {
                offset.y.abs()
            } else {
                offset.x.abs()
            };
            let v = if normal.z.abs() > 0.9 {
                offset.y.abs()
            } else if normal.x.abs() > 0.9 {
                offset.z.abs()
            } else {
                offset.z.abs()
            };
            if u <= half && v <= half {
                let dist = origin.sub(hit).length();
                if best.as_ref().map_or(true, |(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    if matches!(state.transform_tool.as_str(), "rotate" | "rotateSelection") {
        for handle in [GumballHandle::RotateX, GumballHandle::RotateY, GumballHandle::RotateZ] {
            let Some(normal) = handle.plane_normal() else {
                continue;
            };
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let radial = hit.sub(pivot);
                let dist_ring = (radial.length() - extent * 0.85).abs();
                if dist_ring <= pick_radius * 2.0
                    && best.as_ref().map_or(true, |(best_dist, _)| dist_ring < *best_dist)
                {
                    best = Some((dist_ring, handle));
                }
            }
        }
    }
    if matches!(state.transform_tool.as_str(), "scale" | "scaleSelection") {
        for handle in [GumballHandle::ScaleX, GumballHandle::ScaleY, GumballHandle::ScaleZ] {
            let Some(axis) = handle.axis_dir() else {
                continue;
            };
            let end = pivot.add(axis.scale(extent * 1.1));
            if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
                if dist <= pick_radius && best.as_ref().map_or(true, |(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    let _ = eye;
    best.map(|(_, handle)| handle)
}

fn append_gumball_geometry(
    lines: &mut Vec<LineVertex3d>,
    translucent: &mut Vec<SceneDraw3d>,
    state: &World3dState,
    camera: &Camera3d,
    meshes: &HashMap<String, Mesh3d>,
    mesh_versions: &HashMap<String, u64>,
) {
    let Some(pivot) = selection_centroid(state) else {
        return;
    };
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let axis_colors = [
        (Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]),
        (Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]),
        (Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0]),
    ];
    for (axis, color) in axis_colors {
        let end = pivot.add(axis.scale(extent));
        lines.push(LineVertex3d {
            position: pivot.to_array(),
            color,
        });
        lines.push(LineVertex3d {
            position: end.to_array(),
            color,
        });
    }
    let ring_segments = 48usize;
    for (normal, color) in [
        (Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 0.85]),
        (Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 0.85]),
        (Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 0.85]),
    ] {
        let tangent_a = if normal.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let tangent_b = normal.cross(tangent_a).normalize();
        let tangent_a = tangent_b.cross(normal).normalize();
        let radius = extent * 0.85;
        for step in 0..ring_segments {
            let a0 = step as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let a1 = (step + 1) as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let p0 = pivot
                .add(tangent_a.scale(a0.cos() * radius))
                .add(tangent_b.scale(a0.sin() * radius));
            let p1 = pivot
                .add(tangent_a.scale(a1.cos() * radius))
                .add(tangent_b.scale(a1.sin() * radius));
            lines.push(LineVertex3d {
                position: p0.to_array(),
                color,
            });
            lines.push(LineVertex3d {
                position: p1.to_array(),
                color,
            });
        }
    }
    if meshes.contains_key("gumball-plane") {
        let mesh_version = *mesh_versions.get("gumball-plane").unwrap_or(&0);
        let half = extent * 0.35;
        let plane_specs = [
            (Vec3::new(0.0, 0.0, 1.0), [half, half, 1.0]),
            (Vec3::new(1.0, 0.0, 0.0), [1.0, half, half]),
            (Vec3::new(0.0, 1.0, 0.0), [half, 1.0, half]),
        ];
        for (normal, scale) in plane_specs {
            let tangent = if normal.z.abs() > 0.9 {
                Vec3::new(1.0, 0.0, 0.0)
            } else if normal.x.abs() > 0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            let bitangent = normal.cross(tangent).normalize();
            let tangent = bitangent.cross(normal).normalize();
            let rotation = quat_from_basis(tangent, bitangent, normal);
            translucent.push(SceneDraw3d {
                mesh_key: "gumball-plane".into(),
                mesh_version,
                instances: vec![Instance3d {
                    id: "gumball-plane".into(),
                    model: Instance3d::model_from_trs(pivot.to_array(), rotation, scale),
                    color: [0.75, 0.8, 0.9, 0.22],
                    selected: false,
                    hovered: false,
                }],
            });
        }
    }
}

fn apply_gumball_preview(state: &mut World3dState) {
    if state.gumball_handle.is_none() {
        return;
    }
    let pivot = state.gumball_pivot;
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            if !state.selected_ids.iter().any(|id| id == &instance.id) {
                continue;
            }
            let translation = instance.model.cols[3];
            let base = Vec3::new(translation[0], translation[1], translation[2]);
            if state.gumball_preview_translate.length() > 1e-4 {
                let next = base.add(state.gumball_preview_translate);
                instance.model.cols[3] = [next.x, next.y, next.z, 1.0];
            } else if state.gumball_preview_angle.abs() > 1e-6 {
                if let Some(handle) = state.gumball_handle {
                    if let Some(axis) = handle.axis_dir() {
                        let offset = base.sub(pivot);
                        let rotated = rotate_vector(offset, axis, state.gumball_preview_angle);
                        let next = pivot.add(rotated);
                        instance.model.cols[3] = [next.x, next.y, next.z, 1.0];
                    }
                }
            } else if (state.gumball_preview_scale.x - 1.0).abs() > 1e-6
                || (state.gumball_preview_scale.y - 1.0).abs() > 1e-6
                || (state.gumball_preview_scale.z - 1.0).abs() > 1e-6
            {
                if let Some(handle) = state.gumball_handle {
                    if let Some(axis) = handle.axis_dir() {
                        let offset = base.sub(pivot);
                        let scale = match handle {
                            GumballHandle::ScaleX => state.gumball_preview_scale.x,
                            GumballHandle::ScaleY => state.gumball_preview_scale.y,
                            GumballHandle::ScaleZ => state.gumball_preview_scale.z,
                            _ => 1.0,
                        };
                        let scaled = pivot.add(axis.scale(offset.dot(axis) * (scale - 1.0)).add(offset));
                        instance.model.cols[3] = [scaled.x, scaled.y, scaled.z, 1.0];
                    }
                }
            }
        }
    }
}

fn reset_gumball_preview(state: &mut World3dState) {
    state.gumball_preview_translate = Vec3::ZERO;
    state.gumball_preview_angle = 0.0;
    state.gumball_preview_scale = Vec3::new(1.0, 1.0, 1.0);
}

fn gumball_commit_command(state: &World3dState) -> Option<CommandDescriptor> {
    let handle = state.gumball_handle?;
    let ids = state.selected_ids.clone();
    if ids.is_empty() {
        return None;
    }
    if handle.is_translate() && state.gumball_preview_translate.length() > 1e-4 {
        let delta = state.gumball_preview_translate;
        return Some(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "translateSelection".into(),
            args: Some(json!({
                "surfaceId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "dx": delta.x as f64,
                "dy": delta.y as f64,
                "dz": delta.z as f64,
            })),
        });
    }
    if handle.is_rotate() && state.gumball_preview_angle.abs() > 1e-6 {
        let axis = handle.axis_dir()?;
        return Some(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "rotateSelection".into(),
            args: Some(json!({
                "surfaceId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "ax": axis.x as f64,
                "ay": axis.y as f64,
                "az": axis.z as f64,
                "angle": state.gumball_preview_angle as f64,
            })),
        });
    }
    if handle.is_scale() {
        let scale = match handle {
            GumballHandle::ScaleX => state.gumball_preview_scale.x,
            GumballHandle::ScaleY => state.gumball_preview_scale.y,
            GumballHandle::ScaleZ => state.gumball_preview_scale.z,
            _ => 1.0,
        };
        if (scale - 1.0).abs() > 1e-6 {
            let mut args = json!({
                "surfaceId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "sx": 1.0,
                "sy": 1.0,
                "sz": 1.0,
            });
            if handle == GumballHandle::ScaleX {
                args["sx"] = json!(scale);
            } else if handle == GumballHandle::ScaleY {
                args["sy"] = json!(scale);
            } else {
                args["sz"] = json!(scale);
            }
            return Some(CommandDescriptor {
                controller_id: state.controller_id.clone(),
                command: "scaleSelection".into(),
                args: Some(args),
            });
        }
    }
    None
}

fn orbit_camera_command(state: &World3dState) -> CommandDescriptor {
    let camera = state.orbit.to_camera();
    CommandDescriptor {
        controller_id: state.controller_id.clone(),
        command: "setCamera".into(),
        args: Some(json!({
            "surfaceId": state.surface_id,
            "camera": {
                "position": [
                    camera.position.x as f64,
                    camera.position.y as f64,
                    camera.position.z as f64,
                ],
                "target": [
                    camera.target.x as f64,
                    camera.target.y as f64,
                    camera.target.z as f64,
                ],
                "fov": state.orbit.fov_y.to_degrees() as f64,
            }
        })),
    }
}

fn store_mesh(state: &mut World3dState, id: String, mesh: Mesh3d) {
    let version = mesh_content_version(&mesh.positions, &mesh.normals, &mesh.indices);
    state.mesh_versions.insert(id.clone(), version);
    state.meshes.insert(id, mesh);
}

pub fn sync_world3d_state(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    state.bounds = bounds;
    let Some(world) = &scene.world_3d else {
        state.draws.clear();
        state.scene_camera_json = None;
        state.scene_meshes_json = None;
        state.scene_instances_json = None;
        state.scene_selection_json = None;
        state.scene_vortices_json = None;
        state.scene_attractions_json = None;
        state.scene_target_volumes_json = None;
        state.scene_references_json = None;
        state.scene_brush_preview_json = None;
        state.scene_interaction_json = None;
        return;
    };
    let unchanged = state.scene_camera_json.as_deref() == Some(world.camera_json.as_str())
        && state.scene_meshes_json.as_deref() == Some(world.meshes_json.as_str())
        && state.scene_instances_json.as_deref() == Some(world.instances_json.as_str())
        && state.scene_selection_json.as_deref() == Some(world.selection_json.as_str())
        && state.scene_vortices_json.as_deref() == world.vortices_json.as_deref()
        && state.scene_attractions_json.as_deref() == world.attractions_json.as_deref()
        && state.scene_target_volumes_json.as_deref() == world.target_volumes_json.as_deref()
        && state.scene_references_json.as_deref() == world.references_json.as_deref()
        && state.scene_brush_preview_json.as_deref() == world.brush_preview_json.as_deref()
        && state.scene_interaction_json.as_deref() == world.interaction_json.as_deref();
    if unchanged {
        return;
    }
    let camera_changed = state.scene_camera_json.as_deref() != Some(world.camera_json.as_str());
    state.scene_camera_json = Some(world.camera_json.clone());
    state.scene_meshes_json = Some(world.meshes_json.clone());
    state.scene_instances_json = Some(world.instances_json.clone());
    state.scene_selection_json = Some(world.selection_json.clone());
    state.scene_vortices_json = world.vortices_json.clone();
    state.scene_attractions_json = world.attractions_json.clone();
    state.scene_target_volumes_json = world.target_volumes_json.clone();
    state.scene_references_json = world.references_json.clone();
    state.scene_brush_preview_json = world.brush_preview_json.clone();
    state.scene_interaction_json = world.interaction_json.clone();
    state.vortices = world
        .vortices_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_default();
    state.attractions = world
        .attractions_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_default();
    state.target_volumes = world
        .target_volumes_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_default();
    state.references = world
        .references_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_default();
    state.brush_preview = world
        .brush_preview_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok());
    let interaction: WorldInteractionRecord = world
        .interaction_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_default();
    state.active_tool = interaction
        .active_tool
        .unwrap_or_else(|| "select".into());
    state.hovered_vortex_id = interaction.hovered_vortex_full_id;
    for reference in &state.references {
        if reference.hidden.unwrap_or(false) {
            continue;
        }
        if let Some(url) = reference.url.as_deref() {
            if !state.reference_aspect.contains_key(url) {
                state.pending_image_urls.insert(url.to_string());
            }
        }
    }
    let camera: WorldCameraRecord = serde_json::from_str(&world.camera_json).unwrap_or_default();
    if camera_changed {
        if let (Some(position), Some(target)) = (camera.position, camera.target) {
            state.orbit = OrbitController::from_camera(&Camera3d {
                position: vec3_from_f64(position),
                target: vec3_from_f64(target),
                up: camera
                    .up
                    .map(vec3_from_f64)
                    .unwrap_or(Vec3::new(0.0, 0.0, 1.0)),
                fov_y: camera.fov.unwrap_or(45.0) as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            });
        } else if camera.x.is_some() || camera.y.is_some() || camera.z.is_some() {
            state.orbit = OrbitController::from_camera(&Camera3d {
                position: Vec3::new(
                    camera.x.unwrap_or(4.0) as f32,
                    camera.y.unwrap_or(-4.0) as f32,
                    camera.z.unwrap_or(3.0) as f32,
                ),
                target: Vec3::ZERO,
                up: Vec3::new(0.0, 0.0, 1.0),
                fov_y: camera.fov.unwrap_or(45.0) as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            });
        }
    }
    let meshes: Vec<WorldMeshRecord> =
        serde_json::from_str(&world.meshes_json).unwrap_or_default();
    for mesh in meshes {
        if let Some(data) = mesh.data {
            store_mesh(state, mesh.id.clone(), mesh_from_data(&data));
            if let Some(texture) = data.paint_texture_base64.as_deref() {
                if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(
                    texture.strip_prefix("data:image/png;base64,").unwrap_or(texture),
                ) {
                    if let Ok(image) = image::load_from_memory(&bytes) {
                        let rgba = image.to_rgba8();
                        let (width, height) = rgba.dimensions();
                        state.mesh_paint_textures.insert(
                            mesh.id.clone(),
                            (width, height, rgba.into_raw()),
                        );
                    }
                }
            }
        } else if let Some(url) = mesh.url {
            state.pending_glb_urls.insert(url);
        }
    }
    let instances: Vec<WorldInstanceRecord> =
        serde_json::from_str(&world.instances_json).unwrap_or_default();
    let selection: WorldSelectionRecord =
        serde_json::from_str(&world.selection_json).unwrap_or_default();
    state.selection_method = selection
        .method
        .unwrap_or_else(|| "rectangle".into());
    state.local_hover_id = selection.hovered_id;
    state.selected_ids = selection.ids.clone().unwrap_or_default();
    state.component_ids = selection
        .component_ids
        .clone()
        .unwrap_or_default();
    state.granularity = selection
        .granularity
        .or(selection.selection_mode)
        .unwrap_or_else(|| "object".into());
    if state.granularity == "object" {
        state.granularity = "mesh".into();
    }
    state.interaction_mode = selection
        .interaction_mode
        .unwrap_or_else(|| "model".into());
    state.gumball_target = selection.gumball_target.map(|target| {
        [
            target[0] as f32,
            target[1] as f32,
            target[2] as f32,
        ]
    });
    state.hovered_component_id = selection
        .hovered_component
        .as_ref()
        .and_then(|value| value.get("id"))
        .and_then(json_id_to_string);
    state.hovered_component_object_id = selection
        .hovered_component
        .as_ref()
        .and_then(|value| value.get("objectId"))
        .and_then(|value| value.as_str())
        .map(str::to_string);
    state.hovered_component_mode = selection
        .hovered_component
        .as_ref()
        .and_then(|value| value.get("mode"))
        .and_then(|value| value.as_str())
        .map(str::to_string);
    state.show_edges = selection.show_edges.unwrap_or(false);
    state.transform_tool = selection
        .transform_tool
        .unwrap_or_else(|| "translate".into());
    let mut grouped: HashMap<String, Vec<Instance3d>> = HashMap::new();
    for (index, instance) in instances.into_iter().enumerate() {
        let mesh_id = instance
            .mesh_id
            .unwrap_or_else(|| "box".into());
        if !state.meshes.contains_key(&mesh_id) {
            let primitive = mesh_from_kind(&mesh_id);
            store_mesh(
                state,
                mesh_id.clone(),
                Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices),
            );
        }
        let position = instance.position.unwrap_or([
            instance.x.unwrap_or(index as f64),
            instance.y.unwrap_or(0.0),
            instance.z.unwrap_or(0.0),
        ]);
        let scale = instance
            .scale
            .map(|value| [value[0] as f32, value[1] as f32, value[2] as f32])
            .unwrap_or([1.0, 1.0, 1.0]);
        let rotation = instance.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let mut color = parse_color(instance.color.as_deref().unwrap_or("#94a3b8"));
        if let Some(mesh) = state.meshes.get(&mesh_id) {
            if mesh.has_vertex_colors() {
                let mut avg = [0.0f32; 3];
                let count = mesh.colors.len() / 4;
                for chunk in mesh.colors.chunks_exact(4) {
                    avg[0] += chunk[0];
                    avg[1] += chunk[1];
                    avg[2] += chunk[2];
                }
                if count > 0 {
                    let count = count as f32;
                    color = [avg[0] / count, avg[1] / count, avg[2] / count, 1.0];
                }
            }
        }
        let selected = instance.selected.unwrap_or(false);
        let hovered = instance.hovered.unwrap_or(false)
            || state.local_hover_id.as_deref() == Some(instance.id.as_str());
        grouped.entry(mesh_id).or_default().push(Instance3d {
            id: instance.id,
            model: Instance3d::model_from_trs(
                [position[0] as f32, position[1] as f32, position[2] as f32],
                [
                    rotation[0] as f32,
                    rotation[1] as f32,
                    rotation[2] as f32,
                    rotation[3] as f32,
                ],
                scale,
            ),
            color,
            selected,
            hovered,
        });
    }
    state.draws = grouped
        .into_iter()
        .map(|(mesh_key, instances)| SceneDraw3d {
            mesh_key: mesh_key.clone(),
            mesh_version: *state.mesh_versions.get(&mesh_key).unwrap_or(&0),
            instances,
        })
        .collect();
}

fn apply_runtime_draw_flags(state: &mut World3dState) {
    let granularity = state.granularity.clone();
    let component_ids: std::collections::HashSet<String> =
        state.component_ids.iter().cloned().collect();
    let local_hover_id = state.local_hover_id.clone();
    let hovered_component_object_id = state.hovered_component_object_id.clone();
    let selected_ids: std::collections::HashSet<String> =
        state.selected_ids.iter().cloned().collect();
    let mut object_index_map = std::collections::HashMap::new();
    let mut index = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            object_index_map.insert(instance.id.clone(), index);
            index += 1;
        }
    }
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            let mesh_selected = granularity == "mesh"
                && object_index_map.get(&instance.id).is_some_and(|object_index| {
                    component_ids.contains(&object_index.to_string())
                });
            let local_hovered = local_hover_id.as_deref() == Some(instance.id.as_str())
                || hovered_component_object_id.as_deref() == Some(instance.id.as_str());
            let local_selected = selected_ids.contains(&instance.id) || mesh_selected;
            instance.hovered = instance.hovered || local_hovered;
            instance.selected = instance.selected || local_selected;
        }
    }
}

pub fn render_world_3d(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, CommandDescriptor>,
    state: &mut World3dState,
    gpu: &mut GpuContext,
) {
    let theme = ctx.theme;
    sync_world3d_state(state, scene, bounds);
    apply_runtime_draw_flags(state);
    apply_gumball_preview(state);
    let inner = bounds;
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let planes = frustum_planes(view_proj);
    let mut culled_draws = Vec::new();
    let mut culled_count = 0u32;
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let mesh_version = *state.mesh_versions.get(&draw.mesh_key).unwrap_or(&0);
        gpu.ensure_mesh(
            &draw.mesh_key,
            mesh_version,
            &mesh.positions,
            &mesh.normals,
            &mesh.indices,
        );
        let instances: Vec<Instance3d> = draw
            .instances
            .iter()
            .filter(|instance| {
                let (min, max) = transform_aabb(instance.model, mesh.aabb_min, mesh.aabb_max);
                let visible = aabb_intersects_frustum(&planes, min, max);
                if !visible {
                    culled_count += 1;
                }
                visible
            })
            .cloned()
            .collect();
        if !instances.is_empty() {
            culled_draws.push(SceneDraw3d {
                mesh_key: draw.mesh_key.clone(),
                mesh_version,
                instances,
            });
        }
    }
    let mut line_vertices = Vec::new();
    append_component_overlays(state, &mut line_vertices, &state.component_ids, &state.marquee_preview_ids, &state.hovered_component_id);
    for attraction in &state.attractions {
        let Some(from) = attraction.from else { continue };
        let Some(to) = attraction.to else { continue };
        let color = parse_color(attraction.color.as_deref().unwrap_or("#60a5fa"));
        line_vertices.push(LineVertex3d {
            position: [from[0] as f32, from[1] as f32, from[2] as f32],
            color,
        });
        line_vertices.push(LineVertex3d {
            position: [to[0] as f32, to[1] as f32, to[2] as f32],
            color,
        });
    }
    for volume in &state.target_volumes {
        append_box_wireframe(
            &mut line_vertices,
            volume.origin.unwrap_or([0.0, 0.0, 0.0]),
            volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
            volume.scale.unwrap_or([1.0, 1.0, 1.0]),
            parse_color(volume.color.as_deref().unwrap_or("#f472b6")),
        );
    }
    let mut extra_draws = Vec::new();
    let vortex_instances: Vec<Instance3d> = state
        .vortices
        .iter()
        .map(|vortex| {
            let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
            let radius = vortex.radius.unwrap_or(0.36) as f32;
            let hovered = state.hovered_vortex_id.as_deref() == Some(vortex.full_id.as_str());
            Instance3d {
                id: vortex.full_id.clone(),
                model: Instance3d::model_from_trs(
                    [position[0] as f32, position[1] as f32, position[2] as f32],
                    [0.0, 0.0, 0.0, 1.0],
                    [radius, radius, radius],
                ),
                color: parse_color(vortex.color.as_deref().unwrap_or("#38bdf8")),
                selected: false,
                hovered,
            }
        })
        .collect();
    if !vortex_instances.is_empty() {
        if !state.meshes.contains_key("vortex-marker") {
            let primitive = mesh_from_kind("vortex-marker");
            store_mesh(
                state,
                "vortex-marker".into(),
                Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices),
            );
        }
        let mesh_version = *state.mesh_versions.get("vortex-marker").unwrap_or(&0);
        if let Some(mesh) = state.meshes.get("vortex-marker") {
            gpu.ensure_mesh(
                "vortex-marker",
                mesh_version,
                &mesh.positions,
                &mesh.normals,
                &mesh.indices,
            );
        }
        extra_draws.push(SceneDraw3d {
            mesh_key: "vortex-marker".into(),
            mesh_version,
            instances: vortex_instances,
        });
    }
    let mut translucent_draws = Vec::new();
    if let Some(preview) = state.brush_preview.clone() {
        if let Some(mesh_url) = preview.mesh_url.as_deref() {
            let mesh_id = mesh_id_from_url(mesh_url);
            if !state.meshes.contains_key(&mesh_id) {
                let primitive = mesh_from_kind("box");
                store_mesh(
                    state,
                    mesh_id.clone(),
                    Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices),
                );
            }
            let origin = preview.origin.unwrap_or([0.0, 0.0, 0.0]);
            let rotation = preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let scale = preview_scale(preview.scale.as_ref());
            let mesh_version = *state.mesh_versions.get(&mesh_id).unwrap_or(&0);
            if let Some(mesh) = state.meshes.get(&mesh_id) {
                gpu.ensure_mesh(
                    &mesh_id,
                    mesh_version,
                    &mesh.positions,
                    &mesh.normals,
                    &mesh.indices,
                );
            }
            translucent_draws.push(SceneDraw3d {
                mesh_key: mesh_id,
                mesh_version,
                instances: vec![Instance3d {
                    id: "brush-preview".into(),
                    model: Instance3d::model_from_trs(
                        [origin[0] as f32, origin[1] as f32, origin[2] as f32],
                        [
                            rotation[0] as f32,
                            rotation[1] as f32,
                            rotation[2] as f32,
                            rotation[3] as f32,
                        ],
                        scale,
                    ),
                    color: [0.35, 0.75, 1.0, 0.45],
                    selected: false,
                    hovered: false,
                }],
            });
        }
    }
    let mut textured_draws = Vec::new();
    if !state.meshes.contains_key("reference-plane") {
        let primitive = mesh_from_kind("plane");
        store_mesh(
            state,
            "reference-plane".into(),
            Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices),
        );
    }
    let mut textured_instances = Vec::new();
    for reference in &state.references {
        if reference.hidden.unwrap_or(false) {
            continue;
        }
        let Some(url) = reference.url.as_deref() else {
            continue;
        };
        let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let width = reference.width_world.unwrap_or(1.0) as f32;
        let aspect = state.reference_aspect.get(url).copied().unwrap_or(1.0);
        let height = width / aspect.max(0.01);
        textured_instances.push(TexturedInstance3d {
            texture_key: url.to_string(),
            model: Instance3d::model_from_trs(
                [origin[0] as f32, origin[1] as f32, origin[2] as f32],
                [0.0, 0.0, 0.0, 1.0],
                [width, height, 1.0],
            ),
            tint: [1.0, 1.0, 1.0, 0.85],
        });
        if let Some((pixel_w, pixel_h, pixels)) = state.reference_pixels.get(url) {
            gpu.ensure_world_plane_texture(url, pixels, *pixel_w, *pixel_h);
        }
    }
    if !textured_instances.is_empty() {
        textured_draws.push(TexturedDraw3d {
            instances: textured_instances,
        });
    }
    if !state.meshes.contains_key("gumball-plane") {
        let primitive = mesh_from_kind("plane");
        store_mesh(
            state,
            "gumball-plane".into(),
            Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices),
        );
    }
    if !state.selected_ids.is_empty() && state.active_tool == "select" {
        append_gumball_geometry(
            &mut line_vertices,
            &mut translucent_draws,
            state,
            &camera,
            &state.meshes,
            &state.mesh_versions,
        );
    }
    culled_draws.extend(extra_draws);
    ctx.draw.push_scene_pass(ScenePass3d {
        viewport: [inner.x, inner.y, inner.w, inner.h],
        view_proj: view_proj.to_cols_array(),
        light_dir: [0.4, 0.6, 0.8],
        draws: culled_draws,
        line_draws: if line_vertices.is_empty() {
            Vec::new()
        } else {
            vec![LineDraw3d { vertices: line_vertices }]
        },
        translucent_draws,
        textured_draws,
        ..Default::default()
    });
    if state.marquee_active && state.marquee_points.len() >= 2 {
        let accent = theme.accent;
        if state.selection_method == "lasso" {
            for window in state.marquee_points.windows(2) {
                ctx.draw.push_line(
                    window[0][0],
                    window[0][1],
                    window[1][0],
                    window[1][1],
                    accent,
                    1.5,
                );
            }
        } else {
            let start = state.marquee_points[0];
            let end = state.marquee_points[state.marquee_points.len() - 1];
            ctx.draw.push_line(start[0], start[1], end[0], start[1], accent, 1.5);
            ctx.draw.push_line(end[0], start[1], end[0], end[1], accent, 1.5);
            ctx.draw.push_line(end[0], end[1], start[0], end[1], accent, 1.5);
            ctx.draw.push_line(start[0], end[1], start[0], start[1], accent, 1.5);
        }
    }
    if scene.world_3d.is_none() {
        draw_text(
            ctx,
            "world-3d (empty)",
            inner.x + 12.0,
            inner.y + 20.0,
            theme.font_size_small,
            theme.text_muted,
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: None,
        control_id: Some(state.surface_id.clone()),
        kind: HitKind::World3d,
        drag_axis: None,
    drag_data: None,
    });
}

pub fn world3d_hit_target(scene: &UiComponentSceneNode, bounds: Rect) -> HitTarget<CommandDescriptor> {
    HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(scene.surface_id.clone()),
        kind: HitKind::World3d,
        drag_axis: None,
        drag_data: None,
    }
}

pub fn handle_world3d_pointer_move(
    state: &mut World3dState,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
) -> Option<CommandDescriptor> {
    if !state.bounds.contains(x, y) {
        return None;
    }
    let inner = state.bounds;
    if down && button == 0 {
        if state.marquee_active {
            state.marquee_points.push([x, y]);
            update_marquee_preview(state, inner);
        } else if state.paint_stroke_active && state.interaction_mode == "paint" {
            if let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) {
                return Some(CommandDescriptor {
                    controller_id: state.controller_id.clone(),
                    command: "paintAt".into(),
                    args: Some(json!({
                        "surfaceId": state.surface_id,
                        "objectId": object_id,
                        "u": u,
                        "v": v,
                    })),
                });
            }
        } else if button == 2 || button == 1 {
            return None;
        }
    }
    if !down {
        return pick_hover_command(state, x, y, inner);
    }
    if button == 2 {
        return None;
    }
    None
}

pub fn handle_world3d_paint_commands(
    state: &mut World3dState,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
) -> Vec<CommandDescriptor> {
    if state.interaction_mode != "paint" || button != 0 {
        return Vec::new();
    }
    let inner = state.bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    if down && state.paint_stroke_active {
        let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) else {
            return Vec::new();
        };
        return vec![CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "paintAt".into(),
            args: Some(json!({
                "surfaceId": state.surface_id,
                "objectId": object_id,
                "u": u,
                "v": v,
            })),
        }];
    }
    Vec::new()
}

pub fn handle_world3d_pointer_button(
    state: &mut World3dState,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    modifiers: &PointerModifiers,
) -> Option<CommandDescriptor> {
    let inner = state.bounds;
    if !inner.contains(x, y) {
        return None;
    }
    let shift = modifiers.shift;
    let ctrl = modifiers.ctrl;
    if down {
        if button == 0 {
            if state.interaction_mode == "paint" {
                state.paint_stroke_active = true;
                return Some(CommandDescriptor {
                    controller_id: state.controller_id.clone(),
                    command: "paintStrokeBegin".into(),
                    args: Some(json!({ "surfaceId": state.surface_id })),
                });
            }
            if state.active_tool == "brush" {
                if let Some(full_id) = pick_vortex_at(state, x, y, inner) {
                    return Some(CommandDescriptor {
                        controller_id: state.controller_id.clone(),
                        command: "worldVortexSelect".into(),
                        args: Some(json!({ "surfaceId": state.surface_id, "fullId": full_id })),
                    });
                }
            } else if state.active_tool == "select" {
                if !state.selected_ids.is_empty() {
                    if let Some(handle) = pick_gumball_handle_at(state, x, y, inner) {
                        start_gumball_drag(state, handle, x, y, inner);
                        return None;
                    }
                }
                if state.gumball_handle.is_none() && !component_mode_active(state) {
                    state.press_object_id = pick_instance_at(state, x, y, inner);
                } else {
                    state.press_object_id = None;
                }
            }
            state.marquee_active = true;
            state.marquee_points = vec![[x, y]];
            return None;
        }
        if button == 2 {
            state.marquee_active = false;
            state.marquee_points.clear();
        }
        return None;
    }
    if button == 0 && state.marquee_active {
        state.marquee_active = false;
        if is_click_gesture(state, x, y) {
            state.marquee_points.clear();
            state.marquee_preview_ids.clear();
            state.press_object_id = None;
            state.drag_object_id = None;
            state.drag_last_position = None;
            if state.gumball_handle.is_none() {
                return pick_select_command(state, x, y, inner, shift, ctrl);
            }
        } else {
            return marquee_select_command(state, inner, shift, ctrl);
        }
    }
    if button == 0 && state.interaction_mode == "paint" && state.paint_stroke_active {
        state.paint_stroke_active = false;
        let mut commands = Vec::new();
        commands.push(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "paintStrokeEnd".into(),
            args: Some(json!({ "surfaceId": state.surface_id })),
        });
        return commands.first().cloned();
    }
    if button == 0 {
        if let Some(command) = gumball_commit_command(state) {
            state.gumball_handle = None;
            reset_gumball_preview(state);
            return Some(command);
        }
        state.gumball_handle = None;
        reset_gumball_preview(state);
        state.press_object_id = None;
        if let Some(object_id) = state.drag_object_id.take() {
            if let Some(position) = state.drag_last_position {
                if !is_click_gesture(state, x, y) {
                    return Some(CommandDescriptor {
                        controller_id: state.controller_id.clone(),
                        command: "worldRelocate".into(),
                        args: Some(json!({
                            "surfaceId": state.surface_id,
                            "objectId": object_id,
                            "position": position,
                        })),
                    });
                }
            }
        }
        if state.active_tool == "brush" {
            if let Some(preview) = state.brush_preview.clone() {
                if let (Some(target), Some(kind), Some(index)) = (
                    preview.target_vortex_full_id,
                    preview.object_kind_id,
                    preview.source_vortex_index,
                ) {
                    let origin = preview.origin.unwrap_or([0.0, 0.0, 0.0]);
                    return Some(CommandDescriptor {
                        controller_id: state.controller_id.clone(),
                        command: "addBrushObject".into(),
                        args: Some(json!({
                            "targetVortexFullId": target,
                            "objectKindId": kind,
                            "sourceVortexIndex": index,
                            "origin": origin,
                            "orientation": preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                            "scale": preview.scale,
                        })),
                    });
                }
            }
        }
        return pick_select_command(state, x, y, inner, shift, ctrl);
    }
    if button == 1 || button == 2 {
        return Some(orbit_camera_command(state));
    }
    None
}

pub fn handle_world3d_pointer_drag(
    state: &mut World3dState,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    button: i16,
    modifiers: &PointerModifiers,
) {
    let inner = state.bounds;
    if button == 0 {
        if state.gumball_handle.is_some() && inner.contains(x, y) {
            gumball_drag_update(state, x, y, inner);
            return;
        }
        if state.drag_object_id.is_none()
            && state.gumball_handle.is_none()
            && !component_mode_active(state)
            && pointer_drag_distance(state, x, y) > CLICK_DRAG_THRESHOLD_PX
        {
            if let Some(object_id) = state
                .press_object_id
                .take()
                .or_else(|| pick_instance_at(state, x, y, inner))
            {
                state.drag_object_id = Some(object_id.clone());
                state.marquee_active = false;
                state.marquee_points.clear();
                state.marquee_preview_ids.clear();
                state.drag_last_position = object_world_position(state, &object_id);
                state.drag_object_z = state.drag_last_position.map(|p| p[2]).unwrap_or(0.0);
            }
        }
        if state.drag_object_id.is_some() && inner.contains(x, y) {
            if let Some(position) = ground_plane_pick(state, x, y, inner, state.drag_object_z) {
                state.drag_last_position = Some(position);
                if let Some(object_id) = state.drag_object_id.clone() {
                    update_dragged_instance_position(state, &object_id, position);
                }
            }
            return;
        }
    }
    if button == 1 || (button == 2 && modifiers.shift) {
        state.orbit.pan(-dx, -dy);
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        state.orbit.orbit(dx, dy);
    }
}

pub fn handle_world3d_wheel(state: &mut World3dState, delta: f32) {
    state.orbit.zoom(delta);
}

fn merge_string_ids(existing: &[String], incoming: &[String], merge: &str) -> Vec<String> {
    match merge {
        "add" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(id.clone());
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
                    merged.push(id.clone());
                }
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

/// Applies hover/selection command payloads to renderer-local world state before the plugin round-trip.
pub fn apply_world_command_preview(state: &mut World3dState, command: &CommandDescriptor) {
    let Some(args) = command.args.as_ref() else {
        if command.command == "setHover" {
            state.hovered_component_id = None;
            state.hovered_component_object_id = None;
            state.hovered_component_mode = None;
            state.local_hover_id = None;
        }
        return;
    };
    match command.command.as_str() {
        "setHover" => {
            if args.get("objectId").is_none() {
                state.hovered_component_id = None;
                state.hovered_component_object_id = None;
                state.hovered_component_mode = None;
                state.local_hover_id = None;
            } else {
                let object_id = args
                    .get("objectId")
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let mode = args
                    .get("mode")
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let id = args.get("id").and_then(json_id_to_string);
                state.hovered_component_object_id = object_id.clone();
                state.hovered_component_mode = mode;
                state.hovered_component_id = id;
                state.local_hover_id = object_id;
            }
        }
        "worldPick" => {
            let merge = args
                .get("merge")
                .and_then(|value| value.as_str())
                .unwrap_or("replace");
            if let Some(granularity) = args.get("granularity").and_then(|value| value.as_str()) {
                state.granularity = granularity.to_string();
            }
            if args.get("id").map_or(true, |value| value.is_null()) {
                if merge == "replace" {
                    state.component_ids.clear();
                }
            } else if let Some(id) = args.get("id").and_then(json_id_to_string) {
                state.component_ids = merge_string_ids(&state.component_ids, &[id], merge);
            }
        }
        "worldSelect" => {
            let merge = args
                .get("merge")
                .and_then(|value| value.as_str())
                .unwrap_or("replace");
            let ids: Vec<String> = args
                .get("ids")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .unwrap_or_default();
            state.selected_ids = merge_string_ids(&state.selected_ids, &ids, merge);
        }
        "worldHover" => {
            state.local_hover_id = args
                .get("id")
                .and_then(|value| value.as_str())
                .map(str::to_string);
        }
        _ => {}
    }
}

pub fn ingest_glb_mesh(state: &mut World3dState, url: &str, mesh: MeshData, mesh_id: String) {
    state.pending_glb_urls.remove(url);
    store_mesh(state, mesh_id, mesh_from_data(&mesh));
}

#[cfg(target_arch = "wasm32")]
fn debug_log_world(message: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn debug_log_world(message: &str) {
    eprintln!("{message}");
}

fn pick_hover_command(state: &mut World3dState, x: f32, y: f32, inner: Rect) -> Option<CommandDescriptor> {
    let debug_hit = pick_instance_at(state, x, y, inner);
    debug_log_world(&format!(
        "[DEBUG] pick_hover_command x={x} y={y} inner={:?} granularity={} component_mode_active={} debug_pick_instance_at={:?} local_hover_id={:?} draws_len={} meshes_len={}",
        inner,
        state.granularity,
        component_mode_active(state),
        debug_hit,
        state.local_hover_id,
        state.draws.len(),
        state.meshes.len(),
    ));
    if state.active_tool == "brush" {
        let hit = pick_vortex_at(state, x, y, inner);
        if state.hovered_vortex_id == hit {
            return None;
        }
        state.hovered_vortex_id = hit.clone();
        return Some(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "worldVortexHover".into(),
            args: Some(json!({ "surfaceId": state.surface_id, "fullId": hit })),
        });
    }
    if component_mode_active(state) {
        if let Some((mode, id, object_id)) = pick_component_at(state, x, y, inner) {
            if state.hovered_component_id.as_deref() == Some(id.as_str())
                && state.hovered_component_object_id.as_deref() == Some(object_id.as_str())
                && state.hovered_component_mode.as_deref() == Some(mode.as_str())
            {
                return None;
            }
            state.hovered_component_id = Some(id.clone());
            state.hovered_component_object_id = Some(object_id.clone());
            state.hovered_component_mode = Some(mode.clone());
            state.local_hover_id = Some(object_id.clone());
            let id_num = id.parse::<u64>().unwrap_or(0);
            return Some(CommandDescriptor {
                controller_id: state.controller_id.clone(),
                command: "setHover".into(),
                args: Some(json!({
                    "objectId": object_id,
                    "mode": mode,
                    "id": id_num,
                })),
            });
        }
        if state.hovered_component_id.is_none()
            && state.hovered_component_object_id.is_none()
            && state.hovered_component_mode.is_none()
        {
            return None;
        }
        state.hovered_component_id = None;
        state.hovered_component_object_id = None;
        state.hovered_component_mode = None;
        state.local_hover_id = None;
        return Some(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "setHover".into(),
            args: None,
        });
    }
    let hit = pick_instance_at(state, x, y, inner);
    if state.local_hover_id == hit {
        return None;
    }
    state.local_hover_id = hit.clone();
    Some(CommandDescriptor {
        controller_id: state.controller_id.clone(),
        command: "worldHover".into(),
        args: Some(json!({ "surfaceId": state.surface_id, "id": hit })),
    })
}

fn pick_select_command(
    state: &World3dState,
    x: f32,
    y: f32,
    inner: Rect,
    shift: bool,
    ctrl: bool,
) -> Option<CommandDescriptor> {
    let merge = if shift { "add" } else if ctrl { "toggle" } else { "replace" };
    if state.interaction_mode == "paint" {
        return None;
    }
    if component_mode_active(state) {
        let Some((granularity, id, _object_id)) = pick_component_at(state, x, y, inner) else {
            return Some(CommandDescriptor {
                controller_id: state.controller_id.clone(),
                command: "worldPick".into(),
                args: Some(json!({
                    "surfaceId": state.surface_id,
                    "granularity": state.granularity,
                    "id": null,
                    "merge": merge,
                })),
            });
        };
        return Some(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "worldPick".into(),
            args: Some(json!({
                "surfaceId": state.surface_id,
                "granularity": granularity,
                "id": id.parse::<u64>().ok(),
                "merge": merge,
            })),
        });
    }
    if state.granularity == "mesh" {
        let hit = pick_instance_at(state, x, y, inner);
        let id = hit
            .as_deref()
            .and_then(|object_id| instance_object_index(state, object_id));
        return Some(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "worldPick".into(),
            args: Some(json!({
                "surfaceId": state.surface_id,
                "granularity": "mesh",
                "id": id,
                "merge": merge,
            })),
        });
    }
    let hit = pick_instance_at(state, x, y, inner);
    Some(CommandDescriptor {
        controller_id: state.controller_id.clone(),
        command: "worldSelect".into(),
        args: Some(json!({
            "surfaceId": state.surface_id,
            "ids": hit.map(|id| vec![id]).unwrap_or_default(),
            "merge": merge,
        })),
    })
}

fn instance_object_index(state: &World3dState, object_id: &str) -> Option<u32> {
    let mut index = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            if instance.id == object_id {
                return Some(index);
            }
            index += 1;
        }
    }
    None
}

fn merge_u32_ids(existing: &[String], incoming: &[String], merge: &str) -> Vec<u32> {
    let parse = |ids: &[String]| -> Vec<u32> {
        ids.iter().filter_map(|id| id.parse().ok()).collect()
    };
    let existing_ids = parse(existing);
    let incoming_ids = parse(incoming);
    match merge {
        "add" => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if !merged.contains(&id) {
                    merged.push(id);
                }
            }
            merged
        }
        "toggle" | "invertive" => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if let Some(index) = merged.iter().position(|entry| *entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id);
                }
            }
            merged
        }
        _ => incoming_ids,
    }
}

fn marquee_select_command(
    state: &mut World3dState,
    inner: Rect,
    shift: bool,
    ctrl: bool,
) -> Option<CommandDescriptor> {
    if state.marquee_points.len() < 2 {
        return None;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let rectangle = state.selection_method != "lasso";
    let polygon: Vec<[f32; 2]> = if rectangle {
        let start = state.marquee_points[0];
        let end = state.marquee_points[state.marquee_points.len() - 1];
        vec![start, [end[0], start[1]], end, [start[0], end[1]]]
    } else {
        state.marquee_points.clone()
    };
    let ids = if component_mode_active(state) {
        screen_select_components(
            &state.meshes,
            &state.draws,
            view_proj,
            inner.w,
            inner.h,
            &polygon,
            rectangle,
            state.granularity.as_str(),
        )
    } else {
        screen_select_instances(
            &state.meshes,
            &state.draws,
            view_proj,
            inner.w,
            inner.h,
            &polygon,
            rectangle,
        )
    };
    state.marquee_points.clear();
    state.marquee_preview_ids.clear();
    let merge = if shift { "add" } else if ctrl { "toggle" } else { "replace" };
    if component_mode_active(state) {
        let merged = merge_u32_ids(&state.component_ids, &ids, merge);
        return Some(CommandDescriptor {
            controller_id: state.controller_id.clone(),
            command: "setSelection".into(),
            args: Some(json!({
                "mode": state.granularity,
                "ids": merged,
            })),
        });
    }
    Some(CommandDescriptor {
        controller_id: state.controller_id.clone(),
        command: "worldSelect".into(),
        args: Some(json!({
            "surfaceId": state.surface_id,
            "ids": ids,
            "merge": merge,
        })),
    })
}

fn gumball_drag_update(state: &mut World3dState, x: f32, y: f32, inner: Rect) {
    let Some(handle) = state.gumball_handle else {
        return;
    };
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let pivot = state.gumball_pivot;
    let eye = gumball_eye(&camera, pivot);
    reset_gumball_preview(state);
    if handle.is_translate() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let delta = current - state.gumball_drag_anchor;
                state.gumball_preview_translate = axis.normalize().scale(delta);
            }
        } else if let Some(normal) = handle.plane_normal() {
            if let Some(current) = ray_plane_point(origin, dir, pivot, normal) {
                state.gumball_preview_translate = current.sub(state.gumball_drag_start_vec);
            }
        }
    } else if handle.is_rotate() {
        if let Some(normal) = handle.plane_normal() {
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let current = hit.sub(pivot);
                if current.length() > 1e-4 && state.gumball_drag_start_vec.length() > 1e-4 {
                    state.gumball_preview_angle =
                        axis_rotate_angle(state.gumball_drag_start_vec, current, normal);
                }
            }
        }
    } else if handle.is_scale() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let factor = if state.gumball_drag_anchor.abs() > 1e-4 {
                    current / state.gumball_drag_anchor
                } else {
                    1.0
                };
                let factor = factor.clamp(0.05, 20.0);
                match handle {
                    GumballHandle::ScaleX => state.gumball_preview_scale.x = factor,
                    GumballHandle::ScaleY => state.gumball_preview_scale.y = factor,
                    GumballHandle::ScaleZ => state.gumball_preview_scale.z = factor,
                    _ => {}
                }
            }
        }
    }
    apply_gumball_preview(state);
}

fn start_gumball_drag(state: &mut World3dState, handle: GumballHandle, x: f32, y: f32, inner: Rect) {
    let Some(pivot) = selection_centroid(state) else {
        return;
    };
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let eye = gumball_eye(&camera, pivot);
    state.gumball_handle = Some(handle);
    state.gumball_pivot = pivot;
    reset_gumball_preview(state);
    if let Some(axis) = handle.axis_dir() {
        state.gumball_drag_anchor =
            gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye).unwrap_or(0.0);
    } else if let Some(normal) = handle.plane_normal() {
        state.gumball_drag_start_vec =
            ray_plane_point(origin, dir, pivot, normal).unwrap_or(pivot);
        if handle.is_rotate() {
            state.gumball_drag_start_vec = state.gumball_drag_start_vec.sub(pivot);
        }
        state.gumball_drag_anchor = 0.0;
    }
}

fn pick_component_at(
    state: &World3dState,
    x: f32,
    y: f32,
    inner: Rect,
) -> Option<(String, String, String)> {
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let granularity = state.granularity.as_str();
    match granularity {
        "vertex" => {
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    for (vertex_index, chunk) in mesh.positions.chunks_exact(3).enumerate() {
                        let world = instance.model.transform_point(Vec3::new(
                            chunk[0], chunk[1], chunk[2],
                        ));
                        let Some(screen) =
                            kernel_3d_scene::project_point(camera.view_proj(aspect), world, inner.w, inner.h)
                        else {
                            continue;
                        };
                        let dx = screen[0] - local_x;
                        let dy = screen[1] - local_y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= 12.0
                            && best.as_ref().map_or(true, |(best_dist, _, _)| dist < *best_dist)
                        {
                            let id = mesh
                                .vertex_ids
                                .get(vertex_index)
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| vertex_index.to_string());
                            best = Some((dist, id, instance.id.clone()));
                        }
                    }
                }
            }
            debug_log_world(&format!("[DEBUG] pick_component_at vertex best={:?}", best));
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "edge" => {
            let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    for (edge_index, chunk) in mesh.edge_positions.chunks_exact(6).enumerate() {
                        let a = instance.model.transform_point(Vec3::new(
                            chunk[0], chunk[1], chunk[2],
                        ));
                        let b = instance.model.transform_point(Vec3::new(
                            chunk[3], chunk[4], chunk[5],
                        ));
                        if let Some(dist) = ray_segment_distance(origin, dir, a, b) {
                            if dist <= 0.08
                                && best.as_ref().map_or(true, |(best_dist, _, _)| dist < *best_dist)
                            {
                                let id = mesh
                                    .edge_ids
                                    .get(edge_index)
                                    .map(|value| value.to_string())
                                    .unwrap_or_else(|| edge_index.to_string());
                                best = Some((dist, id, instance.id.clone()));
                            }
                        }
                    }
                }
            }
            debug_log_world(&format!("[DEBUG] pick_component_at edge best={:?}", best));
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "face" => {
            let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    if let Some(hit) = ray_pick_mesh_detail(origin, dir, mesh, instance) {
                        if best.as_ref().map_or(true, |(best_dist, _, _)| hit.distance < *best_dist)
                        {
                            let id = mesh
                                .face_ids
                                .get(hit.triangle_index)
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| hit.triangle_index.to_string());
                            best = Some((hit.distance, id, instance.id.clone()));
                        }
                    }
                }
            }
            debug_log_world(&format!("[DEBUG] pick_component_at face best={:?}", best));
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        _ => {}
    }
    None
}

fn pick_paint_hit(state: &World3dState, x: f32, y: f32, inner: Rect) -> Option<(String, f32, f32)> {
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let mut best: Option<(f32, String, f32, f32)> = None;
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(hit) = ray_pick_mesh_detail(origin, dir, mesh, instance) {
                if let Some((u, v)) =
                    interpolate_mesh_uv(mesh, hit.triangle_index, hit.bary_u, hit.bary_v)
                {
                    if best.as_ref().map_or(true, |(best_dist, _, _, _)| hit.distance < *best_dist)
                    {
                        best = Some((hit.distance, instance.id.clone(), u, v));
                    }
                }
            }
        }
    }
    best.map(|(_, object_id, u, v)| (object_id, u, v))
}

fn update_marquee_preview(state: &mut World3dState, inner: Rect) {
    if state.marquee_points.len() < 2 {
        state.marquee_preview_ids.clear();
        return;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let rectangle = state.selection_method != "lasso";
    let polygon: Vec<[f32; 2]> = if rectangle {
        let start = state.marquee_points[0];
        let end = state.marquee_points[state.marquee_points.len() - 1];
        vec![start, [end[0], start[1]], end, [start[0], end[1]]]
    } else {
        state.marquee_points.clone()
    };
    state.marquee_preview_ids = if component_mode_active(state) {
        screen_select_components(
            &state.meshes,
            &state.draws,
            view_proj,
            inner.w,
            inner.h,
            &polygon,
            rectangle,
            state.granularity.as_str(),
        )
    } else {
        screen_select_instances(
            &state.meshes,
            &state.draws,
            view_proj,
            inner.w,
            inner.h,
            &polygon,
            rectangle,
        )
    };
}

fn pick_instance_at(state: &World3dState, x: f32, y: f32, inner: Rect) -> Option<String> {
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    debug_log_world(&format!(
        "[DEBUG] pick_instance_at local=({local_x},{local_y}) origin={:?} dir={:?} draws={}",
        origin, dir, state.draws.len()
    ));
    let mut best: Option<(f32, String)> = None;
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            debug_log_world(&format!(
                "[DEBUG] pick_instance_at MISSING mesh_key={} available_keys={:?}",
                draw.mesh_key,
                state.meshes.keys().collect::<Vec<_>>()
            ));
            continue;
        };
        debug_log_world(&format!(
            "[DEBUG] pick_instance_at mesh_key={} aabb_min={:?} aabb_max={:?} instances={} tris={}",
            draw.mesh_key, mesh.aabb_min, mesh.aabb_max, draw.instances.len(), mesh.indices.len() / 3
        ));
        for instance in &draw.instances {
            let (world_min, world_max) = transform_aabb(instance.model, mesh.aabb_min, mesh.aabb_max);
            let aabb_hit = ray_aabb_slab(origin, dir, world_min, world_max);
            debug_log_world(&format!(
                "[DEBUG] pick_instance_at instance.id={} world_min={:?} world_max={:?} aabb_hit={:?}",
                instance.id, world_min, world_max, aabb_hit
            ));
            if let Some(distance) = ray_pick_instance(origin, dir, mesh, instance) {
                if best.as_ref().map_or(true, |(best_distance, _)| distance < *best_distance) {
                    best = Some((distance, instance.id.clone()));
                }
            } else {
                for (ti, tri) in mesh.indices.chunks_exact(3).enumerate() {
                    let get = |idx: u32| {
                        let i = idx as usize * 3;
                        instance.model.transform_point(Vec3::new(
                            mesh.positions[i],
                            mesh.positions[i + 1],
                            mesh.positions[i + 2],
                        ))
                    };
                    let a = get(tri[0]);
                    let b = get(tri[1]);
                    let c = get(tri[2]);
                    let edge1 = Vec3::new(b.x - a.x, b.y - a.y, b.z - a.z);
                    let edge2 = Vec3::new(c.x - a.x, c.y - a.y, c.z - a.z);
                    let h = dir.cross(edge2);
                    let det = edge1.dot(h);
                    if det.abs() < 1e-8 {
                        continue;
                    }
                    let f = 1.0 / det;
                    let s = Vec3::new(origin.x - a.x, origin.y - a.y, origin.z - a.z);
                    let u = f * s.dot(h);
                    let q = s.cross(edge1);
                    let v = f * dir.dot(q);
                    let t = f * edge2.dot(q);
                    debug_log_world(&format!(
                        "[DEBUG] tri#{ti} u={u} v={v} t={t} accept={}",
                        (0.0..=1.0).contains(&u) && v >= 0.0 && u + v <= 1.0 && t > 1e-4
                    ));
                }
            }
        }
    }
    best.map(|(_, id)| id)
}

fn pick_vortex_at(state: &World3dState, x: f32, y: f32, inner: Rect) -> Option<String> {
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let mut best: Option<(f32, String)> = None;
    for vortex in &state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
        let min = [center.x - radius, center.y - radius, center.z - radius];
        let max = [center.x + radius, center.y + radius, center.z + radius];
        if let Some(distance) = ray_aabb_slab(origin, dir, min, max) {
            if best.as_ref().map_or(true, |(best_distance, _)| distance < *best_distance) {
                best = Some((distance, vortex.full_id.clone()));
            }
        }
    }
    best.map(|(_, id)| id)
}

fn object_world_position(state: &World3dState, object_id: &str) -> Option<[f32; 3]> {
    for draw in &state.draws {
        for instance in &draw.instances {
            if instance.id == object_id {
                let translation = instance.model.cols[3];
                return Some([translation[0], translation[1], translation[2]]);
            }
        }
    }
    None
}

fn update_dragged_instance_position(state: &mut World3dState, object_id: &str, position: [f32; 3]) {
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            if instance.id == object_id {
                instance.model.cols[3] = [position[0], position[1], position[2], 1.0];
            }
        }
    }
}

fn ground_plane_pick(state: &World3dState, x: f32, y: f32, inner: Rect, plane_z: f32) -> Option<[f32; 3]> {
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    if dir.z.abs() < 1e-5 {
        return None;
    }
    let t = (plane_z - origin.z) / dir.z;
    if t < 0.0 {
        return None;
    }
    let hit = origin.add(dir.scale(t));
    Some([hit.x, hit.y, hit.z])
}

fn preview_scale(scale: Option<&serde_json::Value>) -> [f32; 3] {
    match scale {
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [
            values[0].as_f64().unwrap_or(1.0) as f32,
            values[1].as_f64().unwrap_or(1.0) as f32,
            values[2].as_f64().unwrap_or(1.0) as f32,
        ],
        _ => [1.0, 1.0, 1.0],
    }
}

fn append_box_wireframe(
    lines: &mut Vec<LineVertex3d>,
    origin: [f64; 3],
    orientation: [f64; 4],
    scale: [f64; 3],
    color: [f32; 4],
) {
    let corners = [
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
    ];
    let model = Instance3d::model_from_trs(
        [origin[0] as f32, origin[1] as f32, origin[2] as f32],
        [
            orientation[0] as f32,
            orientation[1] as f32,
            orientation[2] as f32,
            orientation[3] as f32,
        ],
        [scale[0] as f32, scale[1] as f32, scale[2] as f32],
    );
    let world_corners: Vec<[f32; 3]> = corners
        .iter()
        .map(|corner| {
            model
                .transform_point(Vec3::new(corner[0], corner[1], corner[2]))
                .to_array()
        })
        .collect();
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    for (a, b) in edges {
        lines.push(LineVertex3d {
            position: world_corners[a],
            color,
        });
        lines.push(LineVertex3d {
            position: world_corners[b],
            color,
        });
    }
}

fn parse_color(value: &str) -> [f32; 4] {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix('#') {
        let expanded = match hex.len() {
            3 => hex.chars().map(|ch| format!("{ch}{ch}")).collect::<String>(),
            _ => hex.to_string(),
        };
        if expanded.len() >= 6 {
            let r = u8::from_str_radix(&expanded[0..2], 16).unwrap_or(148) as f32 / 255.0;
            let g = u8::from_str_radix(&expanded[2..4], 16).unwrap_or(163) as f32 / 255.0;
            let b = u8::from_str_radix(&expanded[4..6], 16).unwrap_or(184) as f32 / 255.0;
            return [r, g, b, 1.0];
        }
    }
    [0.58, 0.64, 0.72, 1.0]
}

#[derive(Clone, Debug)]
pub struct PendingGlbFetch {
    pub surface_id: String,
    pub url: String,
}

pub fn collect_pending_glb_fetches(states: &HashMap<String, World3dState>) -> Vec<PendingGlbFetch> {
    let mut pending = Vec::new();
    for (surface_id, state) in states {
        for url in &state.pending_glb_urls {
            let mesh_id = mesh_id_from_url(url);
            if state.meshes.contains_key(&mesh_id) {
                continue;
            }
            pending.push(PendingGlbFetch {
                surface_id: surface_id.clone(),
                url: url.clone(),
            });
        }
    }
    pending
}

fn mesh_id_from_url(url: &str) -> String {
    let slug = url
        .trim_start_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(url)
        .trim_end_matches(".glb")
        .trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

pub fn apply_glb_bytes(state: &mut World3dState, url: &str, bytes: &[u8]) {
    let mesh_id = mesh_id_from_url(url);
    if state.meshes.contains_key(&mesh_id) {
        state.pending_glb_urls.remove(url);
        return;
    }
    if let Ok(mesh) = mesh_from_glb(bytes) {
        ingest_glb_mesh(state, url, mesh, mesh_id);
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_url_bytes(url: &str) -> Option<Vec<u8>> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    let window = web_sys::window()?;
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts).ok()?;
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await.ok()?;
    let response = response_value.dyn_into::<Response>().ok()?;
    let buffer = JsFuture::from(response.array_buffer().ok()?).await.ok()?;
    Some(js_sys::Uint8Array::new(&buffer).to_vec())
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_pending_glb_meshes(states: &mut HashMap<String, World3dState>) {
    let pending = collect_pending_glb_fetches(states);
    for item in pending {
        let Some(bytes) = fetch_url_bytes(&item.url).await else {
            continue;
        };
        if let Some(state) = states.get_mut(&item.surface_id) {
            apply_glb_bytes(state, &item.url, &bytes);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_url_bytes(url: &str) -> Option<Vec<u8>> {
    if let Some(path) = url.strip_prefix("file://") {
        return std::fs::read(path).ok();
    }
    if url.starts_with('/') {
        let workspace_relative = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../..")
            .join(url.trim_start_matches('/'));
        if workspace_relative.exists() {
            return std::fs::read(workspace_relative).ok();
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_pending_glb_meshes(states: &mut HashMap<String, World3dState>) {
    let pending = collect_pending_glb_fetches(states);
    for item in pending {
        let Some(bytes) = fetch_url_bytes(&item.url).await else {
            continue;
        };
        if let Some(state) = states.get_mut(&item.surface_id) {
            apply_glb_bytes(state, &item.url, &bytes);
        }
    }
}

pub fn apply_reference_image_bytes(state: &mut World3dState, url: &str, bytes: &[u8]) {
    let reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok();
    let Some(reader) = reader else {
        return;
    };
    if let Ok(image) = reader.decode() {
        let rgba = image.to_rgba8();
        let aspect = rgba.width() as f32 / rgba.height().max(1) as f32;
        state.reference_aspect.insert(url.to_string(), aspect);
        state.reference_pixels.insert(
            url.to_string(),
            (rgba.width(), rgba.height(), rgba.into_raw()),
        );
        state.pending_image_urls.remove(url);
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_pending_reference_images(states: &mut HashMap<String, World3dState>) {
    let mut pending = Vec::new();
    for (surface_id, state) in states.iter() {
        for url in &state.pending_image_urls {
            pending.push((surface_id.clone(), url.clone()));
        }
    }
    for (surface_id, url) in pending {
        let Some(bytes) = fetch_url_bytes(&url).await else {
            continue;
        };
        if let Some(state) = states.get_mut(&surface_id) {
            apply_reference_image_bytes(state, &url, &bytes);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_pending_reference_images(_states: &mut HashMap<String, World3dState>) {}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_core::{UiComponentSceneNode, World3dScene};

    fn topology_mesh() -> Mesh3d {
        let mut mesh = Mesh3d::from_buffers(
            vec![
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0,
            ],
            vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
            vec![0, 1, 2, 1, 3, 2],
        );
        mesh.face_ids = vec![10, 11];
        mesh.vertex_ids = vec![1, 2, 3, 4];
        mesh.edge_positions = vec![
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
        ];
        mesh.edge_ids = vec![5, 6];
        mesh
    }

    fn scene_with_selection(selection_json: &str) -> UiComponentSceneNode {
        UiComponentSceneNode {
            surface_id: "surface-1".into(),
            controller_id: "controller-1".into(),
            component_kind: "world-3d".into(),
            pane_id: None,
            binding_id: None,
            canvas_2d: None,
            world_3d: Some(World3dScene {
                camera_json: r#"{"position":[4.0,4.0,4.0],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0],"fov":45.0}"#.into(),
                meshes_json: r#"[{"id":"mesh-1","data":{"positions":[0,0,0,1,0,0,0,1,0],"normals":[0,0,1,0,0,1,0,0,1],"indices":[0,1,2],"faceIds":[10],"vertexIds":[1,2,3],"edgePositions":[0,0,0,1,0,0],"edgeIds":[5]}}]"#.into(),
                instances_json: r#"[{"id":"obj-1","meshId":"mesh-1","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into(),
                selection_json: selection_json.into(),
                vortices_json: None,
                attractions_json: None,
                target_volumes_json: None,
                references_json: None,
                brush_preview_json: None,
                interaction_json: None,
            }),
            node_graph: None,
            text_editor: None,
            table: None,
            raster: None,
            virtual_file_system: None,
            gis_map: None,
        }
    }

    #[test]
    fn sync_parses_numeric_component_ids_and_hovered_component() {
        let selection = r#"{
            "granularity":"vertex",
            "componentIds":[1,2],
            "hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":3},
            "showEdges":true
        }"#;
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        sync_world3d_state(
            &mut state,
            &scene_with_selection(selection),
            Rect {
                x: 0.0,
                y: 0.0,
                w: 400.0,
                h: 400.0,
            },
        );
        assert_eq!(state.granularity, "vertex");
        assert_eq!(state.component_ids, vec!["1".to_string(), "2".to_string()]);
        assert_eq!(state.hovered_component_id.as_deref(), Some("3"));
        assert_eq!(state.hovered_component_object_id.as_deref(), Some("obj-1"));
        assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));
        assert!(state.show_edges);
    }

    #[test]
    fn append_component_overlays_draws_vertex_and_face_highlights() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d {
            mesh_key: "mesh-1".into(),
            mesh_version: 0,
            instances: vec![Instance3d {
                id: "obj-1".into(),
                model: Mat4::identity(),
                color: [1.0, 1.0, 1.0, 1.0],
                selected: false,
                hovered: false,
            }],
        });
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines, &["1".into()], &[], &None);
        assert!(!lines.is_empty());

        state.granularity = "face".into();
        let mut face_lines = Vec::new();
        append_component_overlays(&state, &mut face_lines, &["10".into()], &[], &None);
        assert!(!face_lines.is_empty());
    }

    #[test]
    fn merge_u32_ids_supports_add_and_toggle() {
        assert_eq!(
            merge_u32_ids(&["1".into()], &["2".into()], "add"),
            vec![1, 2]
        );
        assert_eq!(
            merge_u32_ids(&["1".into(), "2".into()], &["2".into(), "3".into()], "toggle"),
            vec![1, 3]
        );
    }

    #[test]
    fn pick_select_emits_numeric_world_pick_id() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d {
            mesh_key: "mesh-1".into(),
            mesh_version: 0,
            instances: vec![Instance3d {
                id: "obj-1".into(),
                model: Mat4::identity(),
                color: [1.0, 1.0, 1.0, 1.0],
                selected: false,
                hovered: false,
            }],
        });
        let inner = Rect {
            x: 0.0,
            y: 0.0,
            w: 400.0,
            h: 400.0,
        };
        let camera = state.orbit.to_camera();
        let screen = kernel_3d_scene::project_point(
            camera.view_proj(1.0),
            Vec3::ZERO,
            inner.w,
            inner.h,
        )
        .expect("vertex projects");
        let command = pick_select_command(&state, screen[0], screen[1], inner, false, false)
            .expect("pick command");
        assert_eq!(command.command, "worldPick");
        let args = command.args.expect("args");
        assert_eq!(args["id"], json!(1));
    }

    #[test]
    fn marquee_component_mode_emits_set_selection_with_numeric_ids() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.component_ids = vec!["1".into()];
        state.marquee_points = vec![[10.0, 10.0], [390.0, 390.0]];
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d {
            mesh_key: "mesh-1".into(),
            mesh_version: 0,
            instances: vec![Instance3d {
                id: "obj-1".into(),
                model: Mat4::identity(),
                color: [1.0, 1.0, 1.0, 1.0],
                selected: false,
                hovered: false,
            }],
        });
        let command = marquee_select_command(
            &mut state,
            Rect {
                x: 0.0,
                y: 0.0,
                w: 400.0,
                h: 400.0,
            },
            true,
            false,
        )
        .expect("marquee command");
        assert_eq!(command.command, "setSelection");
        let args = command.args.expect("args");
        assert_eq!(args["mode"], json!("vertex"));
        assert!(args["ids"].as_array().is_some());
    }

    #[test]
    fn click_release_routes_to_pick_select_instead_of_empty_marquee() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.active_tool = "select".into();
        state.granularity = "mesh".into();
        state.marquee_active = true;
        state.marquee_points = vec![[120.0, 140.0]];
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d {
            mesh_key: "mesh-1".into(),
            mesh_version: 0,
            instances: vec![Instance3d {
                id: "obj-1".into(),
                model: Mat4::identity(),
                color: [1.0, 1.0, 1.0, 1.0],
                selected: false,
                hovered: false,
            }],
        });
        let inner = Rect {
            x: 0.0,
            y: 0.0,
            w: 400.0,
            h: 400.0,
        };
        state.bounds = inner;
        let command = handle_world3d_pointer_button(
            &mut state,
            120.0,
            140.0,
            false,
            0,
            &PointerModifiers::default(),
        )
        .expect("click should pick");
        assert_eq!(command.command, "worldPick");
        assert!(!state.marquee_active);
    }

    #[test]
    fn apply_world_command_preview_updates_component_hover_and_selection() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        apply_world_command_preview(
            &mut state,
            &CommandDescriptor {
                controller_id: "controller-1".into(),
                command: "setHover".into(),
                args: Some(json!({
                    "objectId": "obj-1",
                    "mode": "vertex",
                    "id": 2,
                })),
            },
        );
        assert_eq!(state.hovered_component_id.as_deref(), Some("2"));
        assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));

        apply_world_command_preview(
            &mut state,
            &CommandDescriptor {
                controller_id: "controller-1".into(),
                command: "worldPick".into(),
                args: Some(json!({
                    "granularity": "vertex",
                    "id": 4,
                    "merge": "replace",
                })),
            },
        );
        assert_eq!(state.component_ids, vec!["4".to_string()]);
        assert_eq!(state.granularity, "vertex");
    }

    #[test]
    fn preview_survives_sync_when_scene_json_unchanged() {
        let selection = r#"{"granularity":"vertex","componentIds":[1],"hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":2}}"#;
        let scene = scene_with_selection(selection);
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        sync_world3d_state(
            &mut state,
            &scene,
            Rect {
                x: 0.0,
                y: 0.0,
                w: 400.0,
                h: 400.0,
            },
        );
        apply_world_command_preview(
            &mut state,
            &CommandDescriptor {
                controller_id: "controller-1".into(),
                command: "worldPick".into(),
                args: Some(json!({
                    "granularity": "vertex",
                    "id": 5,
                    "merge": "replace",
                })),
            },
        );
        sync_world3d_state(
            &mut state,
            &scene,
            Rect {
                x: 0.0,
                y: 0.0,
                w: 400.0,
                h: 400.0,
            },
        );
        assert_eq!(state.component_ids, vec!["5".to_string()]);
    }
}
