//! 🌐 Application-neutral 3D world canvas: mesh loading, orbit camera, picking, and marquee selection.

use base64::Engine;
use framework_surface_terrain::TerrainSessionCore;
use kernel_3d_scene::{
    aabb_intersects_frustum, axis_rotate_angle, frustum_planes, grid_placement_anchor, gumball_extent, gumball_eye, gumball_project_ray_onto_axis, interpolate_mesh_uv, lod_from_camera_distance, lod_progressive_grid_layers,
    marquee_is_crossing_from_path, pick_closest_mesh_url, quat_from_basis, ray_aabb_slab, ray_pick_instance, ray_pick_mesh_detail, ray_plane_point, ray_segment_distance, rotate_vector, screen_select_components, screen_select_instances,
    transform_aabb, vec3_from_f64, Camera3d, Instance3d, LineDraw3d, LineVertex3d, Mat4, Mesh3d, OrbitController, SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d, Vec3,
};
use semio_framework_core::{mesh_from_glb, mesh_from_kind, MeshData};
use serde::de::Error as DeError;
use serde::Deserialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use ui_wgpu::{draw_text, draw_text_overlay, mesh_content_version, paint_selection_marquee, ActionDescriptor, GpuContext, HitKind, HitTarget, PointerModifiers, Rect, Rgba, UiComponentSceneNode, WidgetContext};

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
        Some(other) => Err(D::Error::custom(format!("expected array for component ids, got {other}"))),
    }
}

fn json_id_to_string(value: &serde_json::Value) -> Option<String> {
    value.as_str().map(str::to_string).or_else(|| value.as_u64().map(|id| id.to_string()))
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldMeshLodEntry {
    lod: f64,
    url: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldMeshRecord {
    id: String,
    data: Option<MeshData>,
    url: Option<String>,
    lods: Option<Vec<WorldMeshLodEntry>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldLodRecord {
    #[serde(default = "default_true")]
    automatic: bool,
    #[serde(default = "default_manual_lod")]
    manual: f64,
    #[serde(default = "default_distance_reference")]
    distance_reference: f64,
    #[serde(default)]
    depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    grid_factor: f64,
    #[serde(default = "default_true")]
    show_grid: bool,
    #[serde(default)]
    grid_datum: Option<[f64; 3]>,
}

impl Default for WorldLodRecord {
    fn default() -> Self {
        default_lod_record()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldChunkingRecord {
    chunk_size: f64,
    max_distance: f64,
}

fn default_manual_lod() -> f64 {
    100.0
}

fn default_distance_reference() -> f64 {
    100.0
}

fn default_grid_factor() -> f64 {
    10.0
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
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldSelectionTargets {
    #[serde(default)]
    vertex: bool,
    #[serde(default)]
    edge: bool,
    #[serde(default)]
    face: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldSelectionRecord {
    method: Option<String>,
    ids: Option<Vec<String>>,
    hovered_id: Option<String>,
    granularity: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string_vec")]
    component_ids: Option<Vec<String>>,
    transform_mode: Option<String>,
    interaction_mode: Option<String>,
    gumball_target: Option<[f64; 3]>,
    selection_mode: Option<String>,
    show_edges: Option<bool>,
    targets: Option<WorldSelectionTargets>,
    active_object_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldVortexRecord {
    full_id: String,
    position: Option<[f64; 3]>,
    direction: Option<[f64; 3]>,
    display_direction: Option<String>,
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
    color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldInteractionRecord {
    active_utility: Option<String>,
    hovered_vortex_full_id: Option<String>,
}

//#region Environment
/// ☀️ Directional sun light — `enabled` gates whether `azimuth`/`elevation` (degrees, horizontal
/// coordinate system) replace the renderer's default `light_dir`; `intensity`/`color` have no
/// representable channel in `ScenePass3d` (single direction vector, no color/intensity) yet.
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldEnvironmentSunRecord {
    enabled: Option<bool>,
    azimuth: Option<f64>,
    elevation: Option<f64>,
    #[allow(dead_code)] // 🔌 no per-light intensity channel in ScenePass3d yet; wiring gap, see report.
    intensity: Option<f64>,
    #[allow(dead_code)] // 🔌 no per-light color channel in ScenePass3d yet; wiring gap, see report.
    color: Option<String>,
}

/// 💡 Ambient light — parsed for scene-shape completeness; `ScenePass3d` has no ambient
/// color/intensity channel to apply it to (wiring gap, see report).
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct WorldEnvironmentAmbientRecord {
    intensity: Option<f64>,
    color: Option<String>,
}

/// 🌑 Shadow toggle — dead in the React reference too (no shadow-map consumer there either);
/// kept for scene-shape completeness only.
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct WorldEnvironmentShadowRecord {
    enabled: Option<bool>,
    opacity: Option<f64>,
    softness: Option<f64>,
}

/// 🎨 Neutral-instance material override — `color` becomes the base-color fallback for instances
/// without an explicit per-instance color (mirrors the React reference's "only applies when the
/// instance isn't selected/hovered" rule, since Rust's selection/hover highlighting is a separate
/// boolean layered on top rather than a color premix). `metalness`/`roughness`/`emissive*` have no
/// PBR channel on `Instance3d` yet (wiring gap, see report).
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldEnvironmentMaterialRecord {
    color: Option<String>,
    #[allow(dead_code)]
    metalness: Option<f64>,
    #[allow(dead_code)]
    roughness: Option<f64>,
    #[allow(dead_code)]
    emissive: Option<String>,
    #[allow(dead_code)]
    emissive_intensity: Option<f64>,
}

/// 🌍 `World3dScene.environmentJson` mirror — see `world-3d-host.tsx`'s `WorldEnvironmentRecord`.
/// Only `background` (canvas clear color), `sun` (light direction), and `material.color` (neutral
/// instance base-color fallback) are representable in this renderer today; the rest is parsed for
/// forward-compat and documented per-field above.
#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldEnvironmentRecord {
    background: Option<String>,
    ambient: Option<WorldEnvironmentAmbientRecord>,
    sun: Option<WorldEnvironmentSunRecord>,
    shadow: Option<WorldEnvironmentShadowRecord>,
    material: Option<WorldEnvironmentMaterialRecord>,
}
//#endregion Environment

//#region TerrainStyle
/// 🌐⛰️ `World3dScene.terrainJson` mirror — GIS-3D terrain style/source descriptor consumed by
/// `WorldTerrainLayer` in the React reference. `color_ramp`/`min_zoom`/`max_zoom` are parsed but
/// not branched on, mirroring the React reference (single hardcoded hypsometric ramp; zoom bounds
/// fixed inside `framework_surface_terrain::tiles`) — not a gap, a faithful match of upstream.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WorldTerrainStyle {
    tile_url_template: String,
    #[serde(default)]
    project_origin_lon: f64,
    #[serde(default)]
    project_origin_lat: f64,
    #[serde(default = "default_terrain_exaggeration")]
    exaggeration: f64,
    #[serde(default = "default_terrain_color_ramp")]
    #[allow(dead_code)]
    color_ramp: String,
    #[serde(default = "default_terrain_min_zoom")]
    #[allow(dead_code)]
    min_zoom: u32,
    #[serde(default = "default_terrain_max_zoom")]
    #[allow(dead_code)]
    max_zoom: u32,
}

fn default_terrain_exaggeration() -> f64 {
    1.0
}

fn default_terrain_color_ramp() -> String {
    "hypsometric".into()
}

fn default_terrain_min_zoom() -> u32 {
    6
}

fn default_terrain_max_zoom() -> u32 {
    14
}
//#endregion TerrainStyle
//#endregion SceneRecords

//#region World3dState
pub struct World3dState {
    pub surface_id: String,
    pub controller_id: String,
    pub bounds: Rect,
    pub pick_bounds: Rect,
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
    scene_engagement_preview_json: Option<String>,
    vortices: Vec<WorldVortexRecord>,
    attractions: Vec<WorldAttractionRecord>,
    target_volumes: Vec<WorldTargetVolumeRecord>,
    references: Vec<WorldReferenceRecord>,
    brush_preview: Option<WorldBrushPreviewRecord>,
    active_utility: String,
    hovered_vortex_id: Option<String>,
    drag_object_id: Option<String>,
    drag_object_z: f32,
    drag_last_position: Option<[f32; 3]>,
    selected_ids: Vec<String>,
    transform_mode: String,
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
    selection_targets: WorldSelectionTargets,
    active_object_id: Option<String>,
    press_object_id: Option<String>,
    mesh_paint_textures: HashMap<String, (u32, u32, Vec<u8>)>,
    lod: WorldLodRecord,
    chunking: Option<WorldChunkingRecord>,
    visible_chunks: HashSet<(i64, i64, i64)>,
    mesh_lod_catalog: HashMap<String, Vec<WorldMeshLodEntry>>,
    mesh_url_fallback: HashMap<String, String>,
    instance_positions: HashMap<String, [f64; 3]>,
    parsed_instances: Vec<WorldInstanceRecord>,
    mesh_pool: RefCountPool<String>,
    mesh_source_urls: HashMap<String, String>,
    resolved_lod_pick: Option<f64>,
    scene_lod_json: Option<String>,
    scene_chunking_json: Option<String>,
    environment: WorldEnvironmentRecord,
    scene_environment_json: Option<String>,
    terrain_style: Option<WorldTerrainStyle>,
    scene_terrain_json: Option<String>,
    terrain_applied_signature: Option<(String, f64, f64, f64)>,
    terrain_session: TerrainSessionCore,
    terrain_visible_tiles: HashSet<(u32, u32, u32)>,
    terrain_built_tiles: HashSet<(u32, u32, u32)>,
    pending_terrain_tile_urls: HashMap<String, (u32, u32, u32)>,
    right_press_point: Option<[f32; 2]>,
    gizmo_hovered_tip: Option<usize>,
}

impl World3dState {
    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            surface_id,
            controller_id,
            bounds: Rect::default(),
            pick_bounds: Rect::default(),
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
            scene_engagement_preview_json: None,
            vortices: Vec::new(),
            attractions: Vec::new(),
            target_volumes: Vec::new(),
            references: Vec::new(),
            brush_preview: None,
            active_utility: "select".into(),
            hovered_vortex_id: None,
            drag_object_id: None,
            drag_object_z: 0.0,
            drag_last_position: None,
            selected_ids: Vec::new(),
            transform_mode: "translate".into(),
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
            show_edges: true,
            selection_targets: WorldSelectionTargets::default(),
            active_object_id: None,
            press_object_id: None,
            mesh_paint_textures: HashMap::new(),
            lod: WorldLodRecord { automatic: true, manual: default_manual_lod(), distance_reference: default_distance_reference(), depth_variable: false, grid_factor: default_grid_factor(), show_grid: true, grid_datum: Some([0.0, 0.0, 0.0]) },
            chunking: None,
            visible_chunks: HashSet::new(),
            mesh_lod_catalog: HashMap::new(),
            mesh_url_fallback: HashMap::new(),
            instance_positions: HashMap::new(),
            parsed_instances: Vec::new(),
            mesh_pool: RefCountPool::new(),
            mesh_source_urls: HashMap::new(),
            resolved_lod_pick: None,
            scene_lod_json: None,
            scene_chunking_json: None,
            environment: WorldEnvironmentRecord::default(),
            scene_environment_json: None,
            terrain_style: None,
            scene_terrain_json: None,
            terrain_applied_signature: None,
            terrain_session: TerrainSessionCore::default(),
            terrain_visible_tiles: HashSet::new(),
            terrain_built_tiles: HashSet::new(),
            pending_terrain_tile_urls: HashMap::new(),
            right_press_point: None,
            gizmo_hovered_tip: None,
        }
    }
}
//#endregion World3dState

//#region Pool
struct RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    counts: HashMap<K, u32>,
}

impl<K> RefCountPool<K>
where
    K: Eq + std::hash::Hash + Clone,
{
    fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    fn acquire(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    fn release(&mut self, key: K) -> bool {
        match self.counts.get_mut(&key) {
            Some(count) if *count > 1 => {
                *count -= 1;
                false
            }
            Some(_) => {
                self.counts.remove(&key);
                true
            }
            None => false,
        }
    }

    fn contains(&self, key: &K) -> bool {
        self.counts.contains_key(key)
    }

    fn keys(&self) -> impl Iterator<Item = &K> {
        self.counts.keys()
    }
}
//#endregion Pool

//#region Chunking
fn chunk_key_indices(position: [f64; 3], chunk_size: f64) -> (i64, i64, i64) {
    ((position[0] / chunk_size).floor() as i64, (position[1] / chunk_size).floor() as i64, (position[2] / chunk_size).floor() as i64)
}

fn chunk_center(key: (i64, i64, i64), chunk_size: f64) -> Vec3 {
    let size = chunk_size as f32;
    Vec3::new((key.0 as f32 + 0.5) * size, (key.1 as f32 + 0.5) * size, (key.2 as f32 + 0.5) * size)
}

fn chunk_bounds_radius(chunk_size: f64) -> f64 {
    chunk_size * 0.866
}

fn chunk_distance_visible(cam_pos: Vec3, chunk_center: Vec3, chunk_size: f64, max_dist: f64, was_visible: bool) -> bool {
    let bounds_r = chunk_bounds_radius(chunk_size);
    let dist = cam_pos.sub(chunk_center).length() as f64;
    let enter_dist = max_dist + bounds_r;
    let exit_dist = enter_dist + chunk_size * 0.5;
    if dist <= enter_dist {
        return true;
    }
    if was_visible && dist <= exit_dist {
        return true;
    }
    false
}

fn update_visible_chunks(state: &mut World3dState, cam_pos: Vec3) {
    let Some(chunking) = state.chunking.clone() else {
        return;
    };
    let chunk_size = chunking.chunk_size;
    let max_distance = chunking.max_distance;
    let mut chunk_keys = HashSet::new();
    for position in state.instance_positions.values() {
        chunk_keys.insert(chunk_key_indices(*position, chunk_size));
    }
    let previous = state.visible_chunks.clone();
    let mut next_visible = HashSet::new();
    for key in chunk_keys.iter().chain(previous.iter()) {
        let center = chunk_center(*key, chunk_size);
        let was = previous.contains(key);
        if chunk_distance_visible(cam_pos, center, chunk_size, max_distance, was) {
            next_visible.insert(*key);
        }
    }
    state.visible_chunks = next_visible;
}

fn instance_chunk_visible(state: &World3dState, position: [f64; 3]) -> bool {
    let Some(chunking) = &state.chunking else {
        return true;
    };
    let key = chunk_key_indices(position, chunking.chunk_size);
    state.visible_chunks.contains(&key)
}
//#endregion Chunking

//#region LodGrid
const WORLD_LOD_EPSILON: f64 = 0.01;
const WORLD_GRID_SIZE: f32 = 12_000.0;

fn default_lod_record() -> WorldLodRecord {
    WorldLodRecord { automatic: true, manual: default_manual_lod(), distance_reference: default_distance_reference(), depth_variable: false, grid_factor: default_grid_factor(), show_grid: true, grid_datum: Some([0.0, 0.0, 0.0]) }
}

fn scene_lod(state: &World3dState) -> f64 {
    let camera = state.orbit.to_camera();
    let distance = camera.position.sub(camera.target).length() as f64;
    let auto_lod = lod_from_camera_distance(distance, state.lod.distance_reference);
    if state.lod.automatic || state.lod.depth_variable {
        auto_lod
    } else {
        state.lod.manual
    }
}

fn resolve_physical_mesh_id(state: &World3dState, logical_id: &str, desired_lod: f64) -> String {
    if let Some(lods) = state.mesh_lod_catalog.get(logical_id) {
        let entries: Vec<(f64, &str)> = lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect();
        let fallback = state.mesh_url_fallback.get(logical_id).map(String::as_str);
        if let Some(url) = pick_closest_mesh_url(&entries, desired_lod, fallback) {
            return mesh_id_from_url(url);
        }
    }
    if let Some(url) = state.mesh_url_fallback.get(logical_id) {
        return mesh_id_from_url(url);
    }
    logical_id.to_string()
}

fn append_lod_grid_lines(line_vertices: &mut Vec<LineVertex3d>, lod: f64, grid_factor: f64, anchor: Vec3, base_color: [f32; 4]) {
    for (step_world, opacity) in lod_progressive_grid_layers(lod, grid_factor) {
        let step = step_world as f32;
        let divs = ((WORLD_GRID_SIZE / step).round() as i32).clamp(2, 512);
        let half = WORLD_GRID_SIZE * 0.5;
        let step_size = WORLD_GRID_SIZE / divs as f32;
        let color = [base_color[0], base_color[1], base_color[2], base_color[3] * opacity];
        let z = anchor.z + 0.002;
        for i in 0..=divs {
            let offset = -half + i as f32 * step_size;
            line_vertices.push(LineVertex3d { position: [anchor.x - half, anchor.y + offset, z], color });
            line_vertices.push(LineVertex3d { position: [anchor.x + half, anchor.y + offset, z], color });
            line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y - half, z], color });
            line_vertices.push(LineVertex3d { position: [anchor.x + offset, anchor.y + half, z], color });
        }
    }
}

fn sync_mesh_pool(state: &mut World3dState, needed_mesh_keys: &HashSet<String>, gpu: &mut GpuContext) {
    const PINNED: &[&str] = &["vortex-marker", "cylinder", "cone", "reference-plane", "vertex-marker"];
    for key in needed_mesh_keys {
        if !state.mesh_pool.contains(key) {
            state.mesh_pool.acquire(key.clone());
        }
    }
    let stale: Vec<String> = state.mesh_pool.keys().filter(|key| !needed_mesh_keys.contains(*key) && !PINNED.contains(&key.as_str())).cloned().collect();
    for key in stale {
        if state.mesh_pool.release(key.clone()) {
            state.meshes.remove(&key);
            state.mesh_versions.remove(&key);
            state.mesh_paint_textures.remove(&key);
            state.mesh_source_urls.remove(&key);
            state.pending_glb_urls.remove(&key);
            gpu.evict_mesh(&key);
        }
    }
}

fn queue_lod_mesh_fetch(state: &mut World3dState, logical_id: &str, scene_lod: f64) {
    let entries: Vec<(f64, &str)> = state.mesh_lod_catalog.get(logical_id).map(|lods| lods.iter().map(|entry| (entry.lod, entry.url.as_str())).collect()).unwrap_or_default();
    let fallback = state.mesh_url_fallback.get(logical_id).map(String::as_str);
    if let Some(url) = pick_closest_mesh_url(&entries, scene_lod, fallback) {
        state.pending_glb_urls.insert(url.to_string());
    } else if let Some(url) = fallback {
        state.pending_glb_urls.insert(url.to_string());
    }
}

fn rebuild_instance_draws(state: &mut World3dState, scene_lod: f64) {
    state.instance_positions.clear();
    let instances = state.parsed_instances.clone();
    let mut grouped: HashMap<String, Vec<Instance3d>> = HashMap::new();
    for (index, instance) in instances.iter().enumerate() {
        let logical_mesh_id = instance.mesh_id.clone().unwrap_or_else(|| "box".into());
        let physical_mesh_id = resolve_physical_mesh_id(state, &logical_mesh_id, scene_lod);
        if !state.meshes.contains_key(&physical_mesh_id) {
            if state.mesh_url_fallback.contains_key(&logical_mesh_id) || state.mesh_lod_catalog.contains_key(&logical_mesh_id) {
                queue_lod_mesh_fetch(state, &logical_mesh_id, scene_lod);
            } else {
                let primitive = mesh_from_kind(&logical_mesh_id);
                store_mesh(state, physical_mesh_id.clone(), Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices));
            }
        }
        let position = instance.position.unwrap_or([instance.x.unwrap_or(index as f64), instance.y.unwrap_or(0.0), instance.z.unwrap_or(0.0)]);
        state.instance_positions.insert(instance.id.clone(), position);
        let scale = instance.scale.map(|value| [value[0] as f32, value[1] as f32, value[2] as f32]).unwrap_or([1.0, 1.0, 1.0]);
        let rotation = instance.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let neutral_default_color = state.environment.material.as_ref().and_then(|material| material.color.as_deref()).unwrap_or("#94a3b8");
        let mut color = parse_color(instance.color.as_deref().unwrap_or(neutral_default_color));
        if let Some(mesh) = state.meshes.get(&physical_mesh_id) {
            if mesh.has_vertex_colors() {
                let mut avg = [0.0f32; 3];
                let count = mesh.colors.len() / 4;
                for chunk in mesh.colors.as_chunks::<4>().0 {
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
        let hovered = if component_mode_active(state) { false } else { instance.hovered.unwrap_or(false) || state.local_hover_id.as_deref() == Some(instance.id.as_str()) };
        grouped.entry(physical_mesh_id).or_default().push(Instance3d {
            id: instance.id.clone(),
            model: Instance3d::model_from_trs([position[0] as f32, position[1] as f32, position[2] as f32], [rotation[0] as f32, rotation[1] as f32, rotation[2] as f32, rotation[3] as f32], scale),
            color,
            selected,
            hovered,
        });
    }
    state.draws = grouped.into_iter().map(|(mesh_key, instances)| SceneDraw3d { mesh_key: mesh_key.clone(), mesh_version: *state.mesh_versions.get(&mesh_key).unwrap_or(&0), instances }).collect();
}
//#endregion LodGrid

//#region Environment
/// ☀️🎨 Resolves the renderer's scene-pass light direction from `environment.sun` when the sun is
/// explicitly enabled (horizontal coordinate system, see https://en.wikipedia.org/wiki/Horizontal_coordinate_system,
/// matching `sunPositionFromAzimuthElevation` in `ui/js/react/index.tsx`), else keeps the default.
fn environment_light_dir(environment: &WorldEnvironmentRecord) -> [f32; 3] {
    const DEFAULT_LIGHT_DIR: [f32; 3] = [0.4, 0.6, 0.8];
    let Some(sun) = environment.sun.as_ref() else {
        return DEFAULT_LIGHT_DIR;
    };
    if sun.enabled != Some(true) {
        return DEFAULT_LIGHT_DIR;
    }
    let azimuth = sun.azimuth.unwrap_or(45.0).to_radians();
    let elevation = sun.elevation.unwrap_or(35.0).to_radians();
    let direction = Vec3::new((elevation.cos() * azimuth.cos()) as f32, (elevation.cos() * azimuth.sin()) as f32, elevation.sin() as f32);
    if direction.length() < 1e-6 {
        DEFAULT_LIGHT_DIR
    } else {
        direction.normalize().to_array()
    }
}

/// 🖼️ Resolves the canvas clear color from `environment.background`, falling back to the ambient
/// theme clear color when absent or `"transparent"` (mirrors `isTransparentWorldBackground`).
fn environment_clear_color(environment: &WorldEnvironmentRecord, theme_clear: Rgba) -> Rgba {
    let Some(background) = environment.background.as_deref() else {
        return theme_clear;
    };
    if background.eq_ignore_ascii_case("transparent") {
        return theme_clear;
    }
    let [r, g, b, a] = parse_color(background);
    Rgba::new(r, g, b, a)
}
//#endregion Environment

//#region Terrain
/// 🧮 Elevation-band count for terrain tile shading — `Instance3d`/`World3dVertex` carry no
/// per-vertex color channel (wiring gap, see report), so the continuous hypsometric ramp from the
/// React reference is approximated by bucketing each tile's triangles into flat-colored bands by
/// their (per-tile-normalized) average elevation, reusing the same per-color-bucket technique as
/// `append_component_face_translucent_overlays`.
const TERRAIN_COLOR_BANDS: usize = 10;

#[derive(Deserialize)]
struct TerrainVisibleTileRow {
    z: u32,
    x: u32,
    y: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TerrainTileMeshPayload {
    positions: Vec<f32>,
    normals: Vec<f32>,
    indices: Vec<u32>,
    uvs: Vec<f32>,
}

fn terrain_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
    template.replace("{z}", &z.to_string()).replace("{x}", &x.to_string()).replace("{y}", &y.to_string())
}

fn terrain_band_mesh_key(surface_id: &str, z: u32, x: u32, y: u32, band: usize) -> String {
    format!("terrain:{surface_id}:{z}:{x}:{y}:{band}")
}

/// 🎨 Vertical hypsometric ramp — same stops as `getHypsometricTexture` in `world-terrain-layer.tsx`
/// (green low ground -> tan -> grey -> white peaks), sampled at a band's center elevation ratio.
fn hypsometric_color(t: f32) -> [f32; 4] {
    let stops: [(f32, [f32; 3]); 4] =
        [(0.0, [0x4b as f32 / 255.0, 0x6b as f32 / 255.0, 0x3a as f32 / 255.0]), (0.5, [0xa6 as f32 / 255.0, 0x8a as f32 / 255.0, 0x5b as f32 / 255.0]), (0.85, [0x8f as f32 / 255.0, 0x8f as f32 / 255.0, 0x8f as f32 / 255.0]), (1.0, [1.0, 1.0, 1.0])];
    let t = t.clamp(0.0, 1.0);
    for window in stops.windows(2) {
        let (t0, c0) = window[0];
        let (t1, c1) = window[1];
        if t <= t1 {
            let f = if (t1 - t0).abs() < 1e-6 { 0.0 } else { (t - t0) / (t1 - t0) };
            return [c0[0] + (c1[0] - c0[0]) * f, c0[1] + (c1[1] - c0[1]) * f, c0[2] + (c1[2] - c0[2]) * f, 1.0];
        }
    }
    [1.0, 1.0, 1.0, 1.0]
}

/// 🪣 Buckets one elevation band's triangles out of a decoded terrain-tile mesh payload into a
/// standalone (re-indexed) `Mesh3d`, or `None` if the band is empty for this tile.
fn build_terrain_band_mesh(mesh: &TerrainTileMeshPayload, band: usize, band_count: usize) -> Option<Mesh3d> {
    let band_count_f = band_count as f32;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    for tri in mesh.indices.as_chunks::<3>().0 {
        let elevations: [f32; 3] = tri.map(|index| mesh.uvs.get(index as usize * 2 + 1).copied().unwrap_or(0.0));
        let average = (elevations[0] + elevations[1] + elevations[2]) / 3.0;
        let tri_band = ((average * band_count_f) as usize).min(band_count - 1);
        if tri_band != band {
            continue;
        }
        let base = (positions.len() / 3) as u32;
        for &vertex_index in tri {
            let i = vertex_index as usize * 3;
            let Some(position) = mesh.positions.get(i..i + 3) else { continue };
            let normal = mesh.normals.get(i..i + 3).unwrap_or(&[0.0, 0.0, 1.0]);
            positions.extend_from_slice(position);
            normals.extend_from_slice(normal);
        }
        indices.extend_from_slice(&[base, base + 1, base + 2]);
    }
    if positions.is_empty() {
        None
    } else {
        Some(Mesh3d::from_buffers(positions, normals, indices))
    }
}

/// 🔄 GPU-free half of `apply_terrain_style_if_changed`: applies `state.terrain_style` to the tile
/// session and purges cached tile meshes from CPU-side maps whenever the tile source, project
/// origin, or exaggeration changes (their old positions/heights are baked into cached geometry and
/// would otherwise render stale). Returns the mesh keys the caller must also evict from the GPU.
fn apply_terrain_style_if_changed_state(state: &mut World3dState) -> Vec<String> {
    let signature = state.terrain_style.as_ref().map(|style| (style.tile_url_template.clone(), style.project_origin_lon, style.project_origin_lat, style.exaggeration));
    if signature == state.terrain_applied_signature {
        return Vec::new();
    }
    let prefix = format!("terrain:{}:", state.surface_id);
    let stale_keys: Vec<String> = state.meshes.keys().filter(|key| key.starts_with(&prefix)).cloned().collect();
    for key in &stale_keys {
        state.meshes.remove(key);
        state.mesh_versions.remove(key);
    }
    let visible = std::mem::take(&mut state.terrain_visible_tiles);
    for (z, x, y) in visible {
        state.terrain_session.evict_terrain_tile(z, x, y);
    }
    state.terrain_built_tiles.clear();
    state.pending_terrain_tile_urls.clear();
    if let Some(style) = &state.terrain_style {
        state.terrain_session.set_project_origin(style.project_origin_lon, style.project_origin_lat);
        state.terrain_session.set_exaggeration(style.exaggeration);
    }
    state.terrain_applied_signature = signature;
    stale_keys
}

fn apply_terrain_style_if_changed(state: &mut World3dState, gpu: &mut GpuContext) {
    for key in apply_terrain_style_if_changed_state(state) {
        gpu.evict_mesh(&key);
    }
}

/// 🏔️ One terrain band's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`) plus the flat hypsometric color for that band.
struct TerrainBandDraw {
    mesh_key: String,
    mesh_version: u64,
    color: [f32; 4],
}

/// 🏔️ GPU-free half of `sync_terrain`: asks `TerrainSessionCore` which DEM tiles are visible for
/// the current camera, evicts (CPU-side) tiles that scrolled out of view, queues byte-fetches for
/// tiles not yet uploaded (see `fetch_pending_terrain_tiles`), and builds/caches banded meshes for
/// tiles whose elevation data is already available. Returns the bands to draw this frame plus the
/// mesh keys the caller must evict from the GPU.
fn sync_terrain_state(state: &mut World3dState, camera: &Camera3d) -> (Vec<TerrainBandDraw>, Vec<String>) {
    let Some(style) = state.terrain_style.clone() else {
        return (Vec::new(), Vec::new());
    };
    let camera_json = json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
    })
    .to_string();
    let visible_json = state.terrain_session.visible_terrain_tiles_json(&camera_json);
    let visible_rows: Vec<TerrainVisibleTileRow> = serde_json::from_str(&visible_json).unwrap_or_default();
    let visible_set: HashSet<(u32, u32, u32)> = visible_rows.iter().map(|row| (row.z, row.x, row.y)).collect();

    let stale: Vec<(u32, u32, u32)> = state.terrain_visible_tiles.iter().copied().filter(|key| !visible_set.contains(key)).collect();
    let mut evicted_mesh_keys = Vec::new();
    for (z, x, y) in stale {
        state.terrain_session.evict_terrain_tile(z, x, y);
        state.terrain_built_tiles.remove(&(z, x, y));
        for band in 0..TERRAIN_COLOR_BANDS {
            let mesh_key = terrain_band_mesh_key(&state.surface_id, z, x, y, band);
            state.meshes.remove(&mesh_key);
            state.mesh_versions.remove(&mesh_key);
            evicted_mesh_keys.push(mesh_key);
        }
    }
    state.pending_terrain_tile_urls.retain(|_, tile| visible_set.contains(tile));
    state.terrain_visible_tiles = visible_set.clone();

    let mut band_draws = Vec::new();
    for (z, x, y) in visible_set {
        if !state.terrain_built_tiles.contains(&(z, x, y)) {
            let mesh_json = state.terrain_session.terrain_tile_mesh_json(z, x, y);
            if mesh_json == "null" {
                state.pending_terrain_tile_urls.insert(terrain_tile_url(&style.tile_url_template, z, x, y), (z, x, y));
            } else if let Ok(mesh_payload) = serde_json::from_str::<TerrainTileMeshPayload>(&mesh_json) {
                for band in 0..TERRAIN_COLOR_BANDS {
                    if let Some(band_mesh) = build_terrain_band_mesh(&mesh_payload, band, TERRAIN_COLOR_BANDS) {
                        store_mesh(state, terrain_band_mesh_key(&state.surface_id, z, x, y, band), band_mesh);
                    }
                }
                state.terrain_built_tiles.insert((z, x, y));
            }
        }
        for band in 0..TERRAIN_COLOR_BANDS {
            let mesh_key = terrain_band_mesh_key(&state.surface_id, z, x, y, band);
            if !state.meshes.contains_key(&mesh_key) {
                continue;
            }
            let mesh_version = *state.mesh_versions.get(&mesh_key).unwrap_or(&0);
            let band_center = (band as f32 + 0.5) / TERRAIN_COLOR_BANDS as f32;
            band_draws.push(TerrainBandDraw { mesh_key, mesh_version, color: hypsometric_color(band_center) });
        }
    }
    (band_draws, evicted_mesh_keys)
}

/// 🏔️ Per-frame terrain sync entry point used by `render_world_3d` — see `sync_terrain_state` for
/// the (unit-testable) tile-visibility/meshing/fetch-queueing logic this wraps with GPU upload and
/// eviction calls.
fn sync_terrain(state: &mut World3dState, gpu: &mut GpuContext, camera: &Camera3d) -> Vec<SceneDraw3d> {
    let (band_draws, evicted_mesh_keys) = sync_terrain_state(state, camera);
    for key in evicted_mesh_keys {
        gpu.evict_mesh(&key);
    }
    band_draws
        .into_iter()
        .filter_map(|band| {
            let mesh = state.meshes.get(&band.mesh_key)?;
            gpu.ensure_mesh(&band.mesh_key, band.mesh_version, &mesh.positions, &mesh.normals, &mesh.indices);
            Some(SceneDraw3d { mesh_key: band.mesh_key.clone(), mesh_version: band.mesh_version, instances: vec![Instance3d { id: format!("terrain-{}", band.mesh_key), model: Mat4::identity(), color: band.color, selected: false, hovered: false }] })
        })
        .collect()
}
//#endregion Terrain

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
        matches!(self, Self::MoveX | Self::MoveY | Self::MoveZ | Self::MoveXY | Self::MoveYZ | Self::MoveXZ)
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
    let mut mesh = Mesh3d::from_buffers(data.positions.clone(), data.normals.clone(), data.indices.clone());
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
    let payload = texture_base64.strip_prefix("data:image/png;base64,").unwrap_or(texture_base64);
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
    for uv in mesh.uvs.as_chunks::<2>().0 {
        let u = uv[0].clamp(0.0, 1.0);
        let v = uv[1].clamp(0.0, 1.0);
        let x = ((u * width as f32) as u32).min(width - 1);
        let y = ((v * height as f32) as u32).min(height - 1);
        let index = ((y * width + x) * 4) as usize;
        let pixel = &rgba.as_raw()[index..index + 4];
        colors.extend_from_slice(&[pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0, pixel[3] as f32 / 255.0]);
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
    matches!(state.granularity.as_str(), "vertex" | "edge" | "face" | "component")
}

const CLICK_DRAG_THRESHOLD_PX: f32 = 4.0;
const PICK_VERTEX_SCREEN_PX: f32 = 14.0;
const PICK_EDGE_SCREEN_PX: f32 = 18.0;
const FACE_OVERLAY_OFFSET: f32 = 0.003;

fn world_pick_rect(state: &World3dState) -> Rect {
    if state.pick_bounds.w > 0.0 && state.pick_bounds.h > 0.0 {
        state.pick_bounds
    } else {
        state.bounds
    }
}

fn render_pick_viewport(state: &World3dState) -> Rect {
    state.bounds
}

fn pointer_in_pick_rect(state: &World3dState, x: f32, y: f32) -> Option<(f32, f32, Rect)> {
    let clip = world_pick_rect(state);
    if !clip.contains(x, y) {
        return None;
    }
    let viewport = render_pick_viewport(state);
    if !viewport.contains(x, y) {
        return None;
    }
    Some((x - viewport.x, y - viewport.y, viewport))
}

fn pick_targets_instance(state: &World3dState, instance_id: &str) -> bool {
    state.active_object_id.as_deref().is_none_or(|active_id| active_id == instance_id)
}

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

fn push_line_segment(lines: &mut Vec<LineVertex3d>, from: Vec3, to: Vec3, color: [f32; 4]) {
    lines.push(LineVertex3d { position: from.to_array(), color });
    lines.push(LineVertex3d { position: to.to_array(), color });
}

const VERTEX_MARKER_MESH: &str = "vertex-marker";
const VERTEX_BASE_SCALE: f32 = 0.05;
const VERTEX_HOVER_SCALE: f32 = 0.09;
const VERTEX_SELECT_SCALE: f32 = 0.09;

fn component_overlay_color(id: &str, selected: &HashSet<String>, preview: &HashSet<String>, hovered: &Option<String>) -> Option<([f32; 4], f32)> {
    if preview.contains(id) {
        return Some(([1.0, 0.85, 0.35, 1.0], VERTEX_HOVER_SCALE));
    }
    if hovered.as_deref() == Some(id) {
        return Some(([0.35, 0.75, 1.0, 0.9], VERTEX_HOVER_SCALE));
    }
    if selected.contains(id) {
        return Some(([0.35, 0.75, 1.0, 1.0], VERTEX_SELECT_SCALE));
    }
    None
}

fn mesh_face_id(mesh: &Mesh3d, tri_index: usize) -> String {
    mesh.face_ids.get(tri_index).map(|value| value.to_string()).unwrap_or_else(|| tri_index.to_string())
}

fn face_component_mode_active(state: &World3dState) -> bool {
    state.selection_targets.face || state.granularity == "face"
}

fn apply_hovered_component_from_selection(state: &mut World3dState, selection_json: &str) {
    let Some(selection_value) = serde_json::from_str::<serde_json::Value>(selection_json).ok() else {
        return;
    };
    match selection_value.get("hoveredComponent") {
        None => return,
        Some(value) if value.is_null() => {
            state.hovered_component_id = None;
            state.hovered_component_object_id = None;
            state.hovered_component_mode = None;
        }
        Some(value) => {
            state.hovered_component_id = value.get("id").and_then(json_id_to_string);
            state.hovered_component_object_id = value.get("objectId").and_then(|entry| entry.as_str()).map(str::to_string);
            state.hovered_component_mode = value.get("mode").and_then(|entry| entry.as_str()).map(str::to_string);
        }
    }
    if state.hovered_component_mode.as_deref() != Some(state.granularity.as_str()) {
        state.hovered_component_id = None;
        state.hovered_component_object_id = None;
        state.hovered_component_mode = None;
    }
}
fn mesh_vertex(mesh: &Mesh3d, index: u32) -> Vec3 {
    let i = index as usize * 3;
    Vec3::new(mesh.positions[i], mesh.positions[i + 1], mesh.positions[i + 2])
}

fn instance_hovered_component_id(state: &World3dState, instance_id: &str) -> Option<String> {
    if !pick_targets_instance(state, instance_id) {
        return None;
    }
    if state.hovered_component_mode.as_deref() != Some(state.granularity.as_str()) {
        return None;
    }
    if state.hovered_component_object_id.as_deref().is_some_and(|object_id| object_id != instance_id) {
        return None;
    }
    state.hovered_component_id.clone()
}

fn append_component_vertex_spheres(_state: &mut World3dState) -> Vec<Instance3d> {
    Vec::new()
}

fn append_component_overlays(state: &World3dState, lines: &mut Vec<LineVertex3d>) {
    let wire_color = [0.55, 0.65, 0.8, 0.75];
    if state.interaction_mode == "paint" || component_mode_active(state) || state.show_edges || state.selection_targets.edge || (state.granularity == "mesh" && !state.component_ids.is_empty()) {
        for draw in &state.draws {
            let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            if mesh.edge_positions.is_empty() {
                continue;
            }
            for instance in &draw.instances {
                for chunk in mesh.edge_positions.as_chunks::<6>().0 {
                    let a = instance.model.transform_point(Vec3::new(chunk[0], chunk[1], chunk[2]));
                    let b = instance.model.transform_point(Vec3::new(chunk[3], chunk[4], chunk[5]));
                    lines.push(LineVertex3d { position: a.to_array(), color: wire_color });
                    lines.push(LineVertex3d { position: b.to_array(), color: wire_color });
                }
            }
        }
    }
    let selected: HashSet<String> = state.component_ids.iter().cloned().collect();
    let preview: HashSet<String> = state.marquee_preview_ids.iter().cloned().collect();
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            let hovered = instance_hovered_component_id(state, &instance.id);
            if state.granularity.as_str() != "edge" {
                continue;
            }
            if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                continue;
            }
            for (edge_index, chunk) in mesh.edge_positions.as_chunks::<6>().0.iter().enumerate() {
                let id = mesh.edge_ids.get(edge_index).map(|value| value.to_string()).unwrap_or_else(|| edge_index.to_string());
                let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered) else {
                    continue;
                };
                let a = instance.model.transform_point(Vec3::new(chunk[0], chunk[1], chunk[2]));
                let b = instance.model.transform_point(Vec3::new(chunk[3], chunk[4], chunk[5]));
                push_line_segment(lines, a, b, color);
            }
        }
    }
    if face_component_mode_active(state) {
        for draw in &state.draws {
            let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            for instance in &draw.instances {
                let hovered = instance_hovered_component_id(state, &instance.id);
                if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                    continue;
                }
                for (tri_index, tri) in mesh.indices.as_chunks::<3>().0.iter().enumerate() {
                    let id = mesh_face_id(mesh, tri_index);
                    let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered) else {
                        continue;
                    };
                    let verts = [instance.model.transform_point(mesh_vertex(mesh, tri[0])), instance.model.transform_point(mesh_vertex(mesh, tri[1])), instance.model.transform_point(mesh_vertex(mesh, tri[2]))];
                    push_line_segment(lines, verts[0], verts[1], color);
                    push_line_segment(lines, verts[1], verts[2], color);
                    push_line_segment(lines, verts[2], verts[0], color);
                }
            }
        }
    }
    if state.selection_targets.vertex || state.granularity == "vertex" {
        let wire_color = [0.55, 0.65, 0.8, 0.9];
        for draw in &state.draws {
            let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            for instance in &draw.instances {
                let hovered = instance_hovered_component_id(state, &instance.id);
                for (vertex_index, chunk) in mesh.positions.as_chunks::<3>().0.iter().enumerate() {
                    let id = mesh.vertex_ids.get(vertex_index).map(|value| value.to_string()).unwrap_or_else(|| vertex_index.to_string());
                    let center = instance.model.transform_point(Vec3::new(chunk[0], chunk[1], chunk[2]));
                    let (color, scale) = component_overlay_color(&id, &selected, &preview, &hovered).unwrap_or((wire_color, VERTEX_BASE_SCALE));
                    if !state.selection_targets.vertex && component_overlay_color(&id, &selected, &preview, &hovered).is_none() {
                        continue;
                    }
                    let d = scale * 0.15;
                    push_line_segment(lines, center.sub(Vec3::new(d, 0.0, 0.0)), center.add(Vec3::new(d, 0.0, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, d, 0.0)), center.add(Vec3::new(0.0, d, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, 0.0, d)), center.add(Vec3::new(0.0, 0.0, d)), color);
                }
            }
        }
    }
}

/// 🪣 Per-color accumulation bucket for translucent face overlays: fill color, interleaved positions, normals, and triangle indices.
type FaceOverlayBucket = ([f32; 4], Vec<f32>, Vec<f32>, Vec<u32>);

fn append_component_face_translucent_overlays(state: &mut World3dState, gpu: &mut GpuContext, translucent: &mut Vec<SceneDraw3d>) {
    if !face_component_mode_active(state) {
        return;
    }
    let selected: HashSet<String> = state.component_ids.iter().cloned().collect();
    let preview: HashSet<String> = state.marquee_preview_ids.iter().cloned().collect();
    let mut buckets: Vec<FaceOverlayBucket> = Vec::new();
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            let hovered = instance_hovered_component_id(state, &instance.id);
            for (tri_index, tri) in mesh.indices.as_chunks::<3>().0.iter().enumerate() {
                let id = mesh_face_id(mesh, tri_index);
                // 🎚️ Opacity ordering (selected > hovered > marquee-preview) mirrors the React
                // reference's face-overlay stack (0.62 / 0.48 / 0.36); preview previously matched
                // hovered's 0.48 exactly, collapsing the intended ranking.
                let color = if preview.contains(&id) {
                    [1.0, 0.85, 0.35, 0.36]
                } else if hovered.as_deref() == Some(id.as_str()) {
                    [0.35, 0.75, 1.0, 0.48]
                } else if selected.contains(&id) {
                    [0.35, 0.75, 1.0, 0.62]
                } else {
                    continue;
                };
                let verts = [instance.model.transform_point(mesh_vertex(mesh, tri[0])), instance.model.transform_point(mesh_vertex(mesh, tri[1])), instance.model.transform_point(mesh_vertex(mesh, tri[2]))];
                let normal = verts[1].sub(verts[0]).cross(verts[2].sub(verts[0])).normalize();
                let offset = if hovered.as_deref() == Some(id.as_str()) { FACE_OVERLAY_OFFSET } else { FACE_OVERLAY_OFFSET * 0.5 };
                let verts = verts.map(|vert| vert.add(normal.scale(offset)));
                let bucket = buckets.iter_mut().find(|(bucket_color, _, _, _)| *bucket_color == color);
                let bucket = if let Some(bucket) = bucket {
                    bucket
                } else {
                    buckets.push((color, Vec::new(), Vec::new(), Vec::new()));
                    buckets.last_mut().expect("just pushed a bucket above, so buckets is non-empty")
                };
                let base = bucket.1.len() as u32 / 3;
                for vert in verts {
                    bucket.1.extend_from_slice(&vert.to_array());
                    bucket.2.extend_from_slice(&normal.to_array());
                }
                bucket.3.extend_from_slice(&[base, base + 1, base + 2]);
                bucket.3.extend_from_slice(&[base, base + 2, base + 1]);
            }
        }
    }
    for (index, (color, positions, normals, indices)) in buckets.into_iter().enumerate() {
        if positions.is_empty() {
            continue;
        }
        let mesh_key = format!("component-face-overlay:{}:{index}", state.surface_id);
        store_mesh(state, mesh_key.clone(), Mesh3d::from_buffers(positions, normals, indices));
        let mesh_version = *state.mesh_versions.get(&mesh_key).unwrap_or(&0);
        if let Some(mesh) = state.meshes.get(&mesh_key) {
            gpu.ensure_mesh(&mesh_key, mesh_version, &mesh.positions, &mesh.normals, &mesh.indices);
        }
        translucent.push(SceneDraw3d { mesh_key, mesh_version, instances: vec![Instance3d { id: format!("face-overlay-{index}"), model: Mat4::identity(), color, selected: false, hovered: false }] });
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

fn pick_gumball_handle_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<GumballHandle> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let pivot = selection_centroid(state)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let pick_radius = extent * 0.08;
    let eye = gumball_eye(&camera, pivot);
    let mut best: Option<(f32, GumballHandle)> = None;
    let axes = [(GumballHandle::MoveX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (GumballHandle::MoveY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (GumballHandle::MoveZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (handle, axis, _) in axes {
        let end = pivot.add(axis.scale(extent));
        if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
            if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                best = Some((dist, handle));
            }
        }
    }
    let planes = [(GumballHandle::MoveXY, Vec3::new(0.0, 0.0, 1.0), extent * 0.35), (GumballHandle::MoveYZ, Vec3::new(1.0, 0.0, 0.0), extent * 0.35), (GumballHandle::MoveXZ, Vec3::new(0.0, 1.0, 0.0), extent * 0.35)];
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
            let v = if normal.z.abs() > 0.9 { offset.y.abs() } else { offset.z.abs() };
            if u <= half && v <= half {
                let dist = origin.sub(hit).length();
                if best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    if matches!(state.transform_mode.as_str(), "rotate" | "rotateSelection") {
        for handle in [GumballHandle::RotateX, GumballHandle::RotateY, GumballHandle::RotateZ] {
            let Some(normal) = handle.plane_normal() else {
                continue;
            };
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let radial = hit.sub(pivot);
                let dist_ring = (radial.length() - extent * 0.85).abs();
                if dist_ring <= pick_radius * 2.0 && best.as_ref().is_none_or(|(best_dist, _)| dist_ring < *best_dist) {
                    best = Some((dist_ring, handle));
                }
            }
        }
    }
    if matches!(state.transform_mode.as_str(), "scale" | "scaleSelection") {
        for handle in [GumballHandle::ScaleX, GumballHandle::ScaleY, GumballHandle::ScaleZ] {
            let Some(axis) = handle.axis_dir() else {
                continue;
            };
            let end = pivot.add(axis.scale(extent * 1.1));
            if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
                if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    let _ = eye;
    best.map(|(_, handle)| handle)
}

fn append_gumball_geometry(lines: &mut Vec<LineVertex3d>, translucent: &mut Vec<SceneDraw3d>, state: &World3dState, camera: &Camera3d, meshes: &HashMap<String, Mesh3d>, mesh_versions: &HashMap<String, u64>) {
    let Some(pivot) = selection_centroid(state) else {
        return;
    };
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let axis_colors = [(Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (axis, color) in axis_colors {
        let end = pivot.add(axis.scale(extent));
        lines.push(LineVertex3d { position: pivot.to_array(), color });
        lines.push(LineVertex3d { position: end.to_array(), color });
    }
    let ring_segments = 48usize;
    for (normal, color) in [(Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 0.85]), (Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 0.85]), (Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 0.85])] {
        let tangent_a = if normal.x.abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let tangent_b = normal.cross(tangent_a).normalize();
        let tangent_a = tangent_b.cross(normal).normalize();
        let radius = extent * 0.85;
        for step in 0..ring_segments {
            let a0 = step as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let a1 = (step + 1) as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let p0 = pivot.add(tangent_a.scale(a0.cos() * radius)).add(tangent_b.scale(a0.sin() * radius));
            let p1 = pivot.add(tangent_a.scale(a1.cos() * radius)).add(tangent_b.scale(a1.sin() * radius));
            lines.push(LineVertex3d { position: p0.to_array(), color });
            lines.push(LineVertex3d { position: p1.to_array(), color });
        }
    }
    if meshes.contains_key("gumball-plane") {
        let mesh_version = *mesh_versions.get("gumball-plane").unwrap_or(&0);
        let half = extent * 0.35;
        let plane_specs = [(Vec3::new(0.0, 0.0, 1.0), [half, half, 1.0]), (Vec3::new(1.0, 0.0, 0.0), [1.0, half, half]), (Vec3::new(0.0, 1.0, 0.0), [half, 1.0, half])];
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
                instances: vec![Instance3d { id: "gumball-plane".into(), model: Instance3d::model_from_trs(pivot.to_array(), rotation, scale), color: [0.75, 0.8, 0.9, 0.22], selected: false, hovered: false }],
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
            } else if (state.gumball_preview_scale.x - 1.0).abs() > 1e-6 || (state.gumball_preview_scale.y - 1.0).abs() > 1e-6 || (state.gumball_preview_scale.z - 1.0).abs() > 1e-6 {
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

fn gumball_commit_action(state: &World3dState) -> Option<ActionDescriptor> {
    let handle = state.gumball_handle?;
    let ids = state.selected_ids.clone();
    if ids.is_empty() {
        return None;
    }
    if handle.is_translate() && state.gumball_preview_translate.length() > 1e-4 {
        let delta = state.gumball_preview_translate;
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "translateSelection".into(),
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
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "rotateSelection".into(),
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
            return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "scaleSelection".into(), args: Some(args) });
        }
    }
    None
}

fn orbit_camera_action(state: &World3dState) -> ActionDescriptor {
    let camera = state.orbit.to_camera();
    ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "setCamera".into(),
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
        state.scene_engagement_preview_json = None;
        state.scene_lod_json = None;
        state.scene_chunking_json = None;
        state.scene_environment_json = None;
        state.environment = WorldEnvironmentRecord::default();
        state.scene_terrain_json = None;
        state.terrain_style = None;
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
        && state.scene_interaction_json.as_deref() == world.interaction_json.as_deref()
        && state.scene_engagement_preview_json.as_deref() == world.engagement_preview_json.as_deref()
        && state.scene_lod_json.as_deref() == world.lod_json.as_deref()
        && state.scene_chunking_json.as_deref() == world.chunking_json.as_deref()
        && state.scene_environment_json.as_deref() == world.environment_json.as_deref()
        && state.scene_terrain_json.as_deref() == world.terrain_json.as_deref();
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
    state.scene_engagement_preview_json = world.engagement_preview_json.clone();
    state.scene_lod_json = world.lod_json.clone();
    state.scene_chunking_json = world.chunking_json.clone();
    state.scene_environment_json = world.environment_json.clone();
    state.scene_terrain_json = world.terrain_json.clone();
    state.lod = world.lod_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_else(default_lod_record);
    state.chunking = world.chunking_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    state.environment = world.environment_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.terrain_style = world.terrain_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    state.vortices = world.vortices_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.attractions = world.attractions_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.target_volumes = world.target_volumes_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.references = world.references_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.brush_preview = world.brush_preview_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    let interaction: WorldInteractionRecord = world.interaction_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    state.active_utility = interaction.active_utility.unwrap_or_else(|| "select".into());
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
                up: camera.up.map(vec3_from_f64).unwrap_or(Vec3::new(0.0, 0.0, 1.0)),
                fov_y: camera.fov.unwrap_or(45.0) as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            });
        } else if camera.x.is_some() || camera.y.is_some() || camera.z.is_some() {
            state.orbit = OrbitController::from_camera(&Camera3d {
                position: Vec3::new(camera.x.unwrap_or(4.0) as f32, camera.y.unwrap_or(-4.0) as f32, camera.z.unwrap_or(3.0) as f32),
                target: Vec3::ZERO,
                up: Vec3::new(0.0, 0.0, 1.0),
                fov_y: camera.fov.unwrap_or(45.0) as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            });
        }
    }
    let meshes: Vec<WorldMeshRecord> = serde_json::from_str(&world.meshes_json).unwrap_or_default();
    state.mesh_lod_catalog.clear();
    state.mesh_url_fallback.clear();
    for mesh in meshes {
        if let Some(lods) = mesh.lods.filter(|entries| !entries.is_empty()) {
            state.mesh_lod_catalog.insert(mesh.id.clone(), lods);
            if let Some(url) = mesh.url.clone() {
                state.mesh_url_fallback.insert(mesh.id.clone(), url);
            }
            queue_lod_mesh_fetch(state, &mesh.id, scene_lod(state));
        } else if let Some(data) = mesh.data {
            store_mesh(state, mesh.id.clone(), mesh_from_data(&data));
            if let Some(texture) = data.paint_texture_base64.as_deref() {
                if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(texture.strip_prefix("data:image/png;base64,").unwrap_or(texture)) {
                    if let Ok(image) = image::load_from_memory(&bytes) {
                        let rgba = image.to_rgba8();
                        let (width, height) = rgba.dimensions();
                        state.mesh_paint_textures.insert(mesh.id.clone(), (width, height, rgba.into_raw()));
                    }
                }
            }
        } else if let Some(url) = mesh.url {
            state.mesh_url_fallback.insert(mesh.id.clone(), url.clone());
            state.pending_glb_urls.insert(url);
        }
    }
    state.parsed_instances = serde_json::from_str(&world.instances_json).unwrap_or_default();
    let selection: WorldSelectionRecord = serde_json::from_str(&world.selection_json).unwrap_or_default();
    state.selection_method = selection.method.unwrap_or_else(|| "rectangle".into());
    state.local_hover_id = selection.hovered_id;
    state.selected_ids = selection.ids.clone().unwrap_or_default();
    state.component_ids = selection.component_ids.clone().unwrap_or_default();
    state.granularity = selection.granularity.or(selection.selection_mode).unwrap_or_else(|| "object".into());
    if state.granularity == "object" {
        state.granularity = "mesh".into();
    }
    state.interaction_mode = selection.interaction_mode.unwrap_or_else(|| "model".into());
    state.gumball_target = selection.gumball_target.map(|target| [target[0] as f32, target[1] as f32, target[2] as f32]);
    apply_hovered_component_from_selection(state, &world.selection_json);
    state.show_edges = selection.show_edges.unwrap_or(true);
    state.selection_targets = selection.targets.unwrap_or_default();
    state.active_object_id = selection.active_object_id;
    state.transform_mode = selection.transform_mode.unwrap_or_else(|| "translate".into());
    let current_lod = scene_lod(state);
    rebuild_instance_draws(state, current_lod);
    state.resolved_lod_pick = Some(current_lod);
}

fn apply_runtime_draw_flags(state: &mut World3dState) {
    let granularity = state.granularity.clone();
    let component_ids: std::collections::HashSet<String> = state.component_ids.iter().cloned().collect();
    let local_hover_id = state.local_hover_id.clone();
    let hovered_component_object_id = state.hovered_component_object_id.clone();
    let selected_ids: std::collections::HashSet<String> = state.selected_ids.iter().cloned().collect();
    let mut object_index_map = std::collections::HashMap::new();
    let mut index = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            object_index_map.insert(instance.id.clone(), index);
            index += 1;
        }
    }
    let component_mode = component_mode_active(state);
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            let mesh_selected = granularity == "mesh" && object_index_map.get(&instance.id).is_some_and(|object_index| component_ids.contains(&object_index.to_string()));
            let local_hovered = if component_mode { false } else { local_hover_id.as_deref() == Some(instance.id.as_str()) || hovered_component_object_id.as_deref() == Some(instance.id.as_str()) };
            let local_selected = selected_ids.contains(&instance.id) || mesh_selected;
            // 🎨 Selection/hover flags must follow the live selection snapshot — OR-ing with the
            // instancesJson bits left deselected meshes painted selected until a later hover rebuild.
            instance.hovered = local_hovered;
            instance.selected = local_selected;
        }
    }
}

pub fn render_world_3d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut WidgetContext<'_, ActionDescriptor>, state: &mut World3dState, gpu: &mut GpuContext) {
    let theme = ctx.theme;
    state.pick_bounds = ctx.pick_clip.unwrap_or(bounds);
    sync_world3d_state(state, scene, bounds);
    apply_terrain_style_if_changed(state, gpu);
    let current_lod = scene_lod(state);
    let lod_changed = state.resolved_lod_pick.is_none_or(|previous| (previous - current_lod).abs() > WORLD_LOD_EPSILON);
    if lod_changed {
        rebuild_instance_draws(state, current_lod);
        state.resolved_lod_pick = Some(current_lod);
    }
    apply_runtime_draw_flags(state);
    apply_gumball_preview(state);
    let inner = bounds;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], environment_clear_color(&state.environment, theme.canvas_clear));
    let camera = state.orbit.to_camera();
    let light_dir = environment_light_dir(&state.environment);
    let terrain_draws = sync_terrain(state, gpu, &camera);
    update_visible_chunks(state, camera.position);
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let planes = frustum_planes(view_proj);
    let mut culled_draws = Vec::new();
    let mut culled_count = 0u32;
    let mut needed_mesh_keys = HashSet::new();
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            if let Some(url) = state.mesh_source_urls.get(&draw.mesh_key) {
                state.pending_glb_urls.insert(url.clone());
            }
            continue;
        };
        let mesh_version = *state.mesh_versions.get(&draw.mesh_key).unwrap_or(&0);
        let instances: Vec<Instance3d> = draw
            .instances
            .iter()
            .filter(|instance| {
                let position = state.instance_positions.get(&instance.id).copied().unwrap_or([0.0, 0.0, 0.0]);
                if !instance_chunk_visible(state, position) {
                    return false;
                }
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
            needed_mesh_keys.insert(draw.mesh_key.clone());
            gpu.ensure_mesh(&draw.mesh_key, mesh_version, &mesh.positions, &mesh.normals, &mesh.indices);
            culled_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances });
        }
    }
    sync_mesh_pool(state, &needed_mesh_keys, gpu);
    let mut line_vertices = Vec::new();
    if state.lod.show_grid {
        let datum = state.lod.grid_datum.unwrap_or([0.0, 0.0, 0.0]);
        let anchor = grid_placement_anchor(camera.target, datum);
        append_lod_grid_lines(&mut line_vertices, current_lod, state.lod.grid_factor, anchor, [theme.text_element.r, theme.text_element.g, theme.text_element.b, theme.text_element.a]);
    }
    append_component_overlays(state, &mut line_vertices);
    for attraction in &state.attractions {
        let Some(from) = attraction.from else { continue };
        let Some(to) = attraction.to else { continue };
        let color = parse_color(attraction.color.as_deref().unwrap_or("#60a5fa"));
        line_vertices.push(LineVertex3d { position: [from[0] as f32, from[1] as f32, from[2] as f32], color });
        line_vertices.push(LineVertex3d { position: [to[0] as f32, to[1] as f32, to[2] as f32], color });
    }
    for volume in &state.target_volumes {
        append_box_wireframe(&mut line_vertices, volume.origin.unwrap_or([0.0, 0.0, 0.0]), volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), volume.scale.unwrap_or([1.0, 1.0, 1.0]), parse_color(volume.color.as_deref().unwrap_or("#f472b6")));
    }
    let mut extra_draws = Vec::new();
    append_vortex_arrow_draws(state, gpu, &mut extra_draws);
    let vertex_instances = append_component_vertex_spheres(state);
    if !vertex_instances.is_empty() {
        let mesh_version = *state.mesh_versions.get(VERTEX_MARKER_MESH).unwrap_or(&0);
        if let Some(mesh) = state.meshes.get(VERTEX_MARKER_MESH) {
            gpu.ensure_mesh(VERTEX_MARKER_MESH, mesh_version, &mesh.positions, &mesh.normals, &mesh.indices);
        }
        extra_draws.push(SceneDraw3d { mesh_key: VERTEX_MARKER_MESH.into(), mesh_version, instances: vertex_instances });
    }
    let mut translucent_draws = Vec::new();
    append_component_face_translucent_overlays(state, gpu, &mut translucent_draws);
    if let Some(preview) = state.brush_preview.clone() {
        // 👻 Mirrors `BrushPreviewGhost`: renders whenever `origin` is present, regardless of
        // `meshUrl` — a translucent unit box is the fallback ghost when there's no mesh URL (or
        // its GLB hasn't resolved into `state.meshes` yet), not "nothing at all".
        if let Some(origin) = preview.origin {
            let mesh_id = brush_preview_mesh_id(preview.mesh_url.as_deref());
            if !state.meshes.contains_key(&mesh_id) {
                let primitive = mesh_from_kind("box");
                store_mesh(state, mesh_id.clone(), Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices));
            }
            let rotation = preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let scale = preview_scale(preview.scale.as_ref());
            let mesh_version = *state.mesh_versions.get(&mesh_id).unwrap_or(&0);
            if let Some(mesh) = state.meshes.get(&mesh_id) {
                gpu.ensure_mesh(&mesh_id, mesh_version, &mesh.positions, &mesh.normals, &mesh.indices);
            }
            translucent_draws.push(SceneDraw3d {
                mesh_key: mesh_id,
                mesh_version,
                instances: vec![Instance3d {
                    id: "brush-preview".into(),
                    model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [rotation[0] as f32, rotation[1] as f32, rotation[2] as f32, rotation[3] as f32], scale),
                    color: {
                        let mut color = parse_color(preview.color.as_deref().unwrap_or("#59bfff"));
                        color[3] = 0.45;
                        color
                    },
                    selected: false,
                    hovered: false,
                }],
            });
        }
    }
    let mut textured_draws = Vec::new();
    if !state.meshes.contains_key("reference-plane") {
        let primitive = mesh_from_kind("plane");
        store_mesh(state, "reference-plane".into(), Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices));
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
        textured_instances.push(TexturedInstance3d { texture_key: url.to_string(), model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [width, height, 1.0]), tint: [1.0, 1.0, 1.0, 0.85] });
        if let Some((pixel_w, pixel_h, pixels)) = state.reference_pixels.get(url) {
            gpu.ensure_world_plane_texture(url, pixels, *pixel_w, *pixel_h);
        }
    }
    if !textured_instances.is_empty() {
        textured_draws.push(TexturedDraw3d { instances: textured_instances });
    }
    if !state.meshes.contains_key("gumball-plane") {
        let primitive = mesh_from_kind("plane");
        store_mesh(state, "gumball-plane".into(), Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices));
    }
    if !state.selected_ids.is_empty() && state.active_utility == "select" {
        append_gumball_geometry(&mut line_vertices, &mut translucent_draws, state, &camera, &state.meshes, &state.mesh_versions);
    }
    culled_draws.extend(extra_draws);
    culled_draws.extend(terrain_draws);
    ctx.draw.push_scene_pass(ScenePass3d {
        viewport: [inner.x, inner.y, inner.w, inner.h],
        view_proj: view_proj.to_cols_array(),
        light_dir,
        draws: culled_draws,
        line_draws: if line_vertices.is_empty() { Vec::new() } else { vec![LineDraw3d { vertices: line_vertices }] },
        translucent_draws,
        textured_draws,
        ..Default::default()
    });
    if state.marquee_active && state.marquee_points.len() >= 2 {
        let crossing = marquee_is_crossing_from_path(&state.marquee_points, state.selection_method == "lasso");
        paint_selection_marquee(ctx.draw, theme, crossing, state.selection_method == "lasso", &state.marquee_points, false);
    }
    paint_world_orbit_view_gizmo(ctx, &camera, inner, state.gizmo_hovered_tip);
    if scene.world_3d.is_none() {
        draw_text(ctx, "world-3d (empty)", inner.x + 12.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
    }
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(state.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None });
}

//#region 🧭WorldOrbitViewGizmo
/** 🧭 Permanent X/Y/Z paints — primary / secondary / tertiary (semio tokens), not muted chrome. */
fn spatial_axis_rgba(axis: u8, alpha: f32) -> Rgba {
    match axis {
        0 => Rgba::new(1.0, 0.204, 0.310, alpha),   // primary #ff344f
        1 => Rgba::new(0.204, 0.820, 0.749, alpha), // secondary #34d1bf
        _ => Rgba::new(0.980, 0.584, 0.0, alpha),   // tertiary #fa9500
    }
}

/** 🧭 Mirrors `resolveSceneGizmoViewportPlacement` — bottom-right corner inset matching pane `--spacing-single` chrome. */
pub fn world_orbit_view_gizmo_placement(viewport: Rect) -> (f32, f32) {
    let chrome_inset = 4.0_f32;
    let gizmo_half_extent = 28.0_f32;
    let preferred = chrome_inset + gizmo_half_extent;
    let max_fit = (viewport.w.min(viewport.h) / 3.0).floor().max(22.0);
    let margin = preferred.min(max_fit);
    (margin, margin)
}

/** 🧭 Screen-space tip used for orbit-view gizmo hover hit-testing and paint. */
struct WorldOrbitViewGizmoTip {
    screen_x: f32,
    screen_y: f32,
    depth: f32,
    pick_radius: f32,
    label: &'static str,
    color: Rgba,
    is_corner: bool,
    prominent: bool,
}

fn world_orbit_view_gizmo_tips(camera: &Camera3d, viewport: Rect) -> Vec<WorldOrbitViewGizmoTip> {
    let (margin_x, margin_y) = world_orbit_view_gizmo_placement(viewport);
    let origin_x = viewport.x + viewport.w - margin_x;
    let origin_y = viewport.y + viewport.h - margin_y;
    let axis_len = (viewport.w.min(viewport.h) * 0.04).clamp(14.0, 24.0);
    let forward = camera.position.sub(camera.target);
    let forward_len = forward.length();
    if forward_len < 1e-5 {
        return Vec::new();
    }
    let forward = forward.scale(1.0 / forward_len);
    let right = forward.cross(camera.up);
    let right_len = right.length();
    if right_len < 1e-5 {
        return Vec::new();
    }
    let right = right.scale(1.0 / right_len);
    let up = right.cross(forward).normalize();
    let neutral = Rgba::new(0.62, 0.62, 0.66, 0.9);
    let axes = [
        (Vec3::new(1.0, 0.0, 0.0), "X", spatial_axis_rgba(0, 1.0), false, true),
        (Vec3::new(-1.0, 0.0, 0.0), "", spatial_axis_rgba(0, 0.75), false, false),
        (Vec3::new(0.0, 1.0, 0.0), "Y", spatial_axis_rgba(1, 1.0), false, true),
        (Vec3::new(0.0, -1.0, 0.0), "", spatial_axis_rgba(1, 0.75), false, false),
        (Vec3::new(0.0, 0.0, 1.0), "Z", spatial_axis_rgba(2, 1.0), false, true),
        (Vec3::new(0.0, 0.0, -1.0), "", spatial_axis_rgba(2, 0.75), false, false),
    ];
    let corners = [
        (Vec3::new(0.72, 0.72, 0.72), "NE", true),
        (Vec3::new(-0.72, 0.72, 0.72), "NW", true),
        (Vec3::new(0.72, -0.72, 0.72), "SE", true),
        (Vec3::new(-0.72, -0.72, 0.72), "SW", true),
        (Vec3::new(0.72, 0.72, -0.72), "", false),
        (Vec3::new(-0.72, 0.72, -0.72), "", false),
        (Vec3::new(0.72, -0.72, -0.72), "", false),
        (Vec3::new(-0.72, -0.72, -0.72), "", false),
    ];
    let mut tips: Vec<WorldOrbitViewGizmoTip> = axes
        .into_iter()
        .map(|(axis, label, color, is_corner, prominent)| {
            let sx = axis.dot(right);
            let sy = -axis.dot(up);
            let depth = axis.dot(forward);
            let tip_x = origin_x + sx * axis_len;
            let tip_y = origin_y + sy * axis_len;
            let pick_radius = if prominent { 10.0 } else { 7.0 };
            WorldOrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, label, color, is_corner, prominent }
        })
        .chain(corners.into_iter().map(|(axis, label, prominent)| {
            let sx = axis.dot(right);
            let sy = -axis.dot(up);
            let depth = axis.dot(forward);
            let tip_x = origin_x + sx * axis_len;
            let tip_y = origin_y + sy * axis_len;
            let pick_radius = if prominent { 10.0 } else { 7.0 };
            WorldOrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, label, color: neutral, is_corner: true, prominent }
        }))
        .collect();
    tips.push(WorldOrbitViewGizmoTip {
        screen_x: origin_x,
        screen_y: origin_y,
        depth: 0.0,
        pick_radius: 9.0,
        label: "3D",
        color: neutral,
        is_corner: false,
        prominent: true,
    });
    tips
}

fn world_orbit_view_gizmo_hit_test(x: f32, y: f32, tips: &[WorldOrbitViewGizmoTip]) -> Option<usize> {
    tips.iter()
        .enumerate()
        .filter_map(|(index, tip)| {
            let distance = ((x - tip.screen_x).powi(2) + (y - tip.screen_y).powi(2)).sqrt();
            if distance <= tip.pick_radius + 3.0 {
                Some((index, distance))
            } else {
                None
            }
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
}

fn update_world_orbit_view_gizmo_hover(state: &mut World3dState, x: f32, y: f32, inner: Rect) {
    let (margin_x, margin_y) = world_orbit_view_gizmo_placement(inner);
    let origin_x = inner.x + inner.w - margin_x;
    let origin_y = inner.y + inner.h - margin_y;
    let zone_radius = 56.0;
    if ((x - origin_x).powi(2) + (y - origin_y).powi(2)).sqrt() > zone_radius {
        state.gizmo_hovered_tip = None;
        return;
    }
    let camera = state.orbit.to_camera();
    let tips = world_orbit_view_gizmo_tips(&camera, inner);
    state.gizmo_hovered_tip = world_orbit_view_gizmo_hit_test(x, y, &tips);
}

/** 🧭 Screen-space XYZ orientation gizmo in the lower-right of every world-3d window (wgpu parity with React `WorldOrbitViewGizmo`). */
fn paint_world_orbit_view_gizmo<E>(ctx: &mut WidgetContext<'_, E>, camera: &Camera3d, viewport: Rect, hovered_tip: Option<usize>) {
    let (margin_x, margin_y) = world_orbit_view_gizmo_placement(viewport);
    let origin_x = viewport.x + viewport.w - margin_x;
    let origin_y = viewport.y + viewport.h - margin_y;
    let tips = world_orbit_view_gizmo_tips(camera, viewport);
    let mut ordered: Vec<(f32, usize)> = tips.iter().enumerate().map(|(index, tip)| (tip.depth, index)).collect();
    ordered.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let has_hover = hovered_tip.is_some();
    for (_, index) in ordered {
        let tip = &tips[index];
        let hovered = hovered_tip == Some(index);
        let depth_fade = if tip.depth > 0.05 { 0.45 } else { 1.0 };
        let hover_fade = if has_hover && !hovered { 0.42 } else { 1.0 };
        let alpha = (tip.color.a * depth_fade * hover_fade).min(1.0);
        let stroke = Rgba::new(tip.color.r, tip.color.g, tip.color.b, if hovered { tip.color.a.min(1.0) } else { alpha });
        ctx.draw.push_line_overlay(origin_x, origin_y, tip.screen_x, tip.screen_y, stroke, if tip.is_corner { 1.5 } else { 2.0 });
        if tip.is_corner || tip.label.is_empty() {
            let r = if tip.prominent {
                if hovered { 3.6 } else { 3.0 }
            } else if hovered {
                2.4
            } else {
                2.0
            };
            ctx.draw.push_solid_overlay([tip.screen_x - r, tip.screen_y - r, tip.screen_x + r, tip.screen_y + r], stroke);
        }
        if !tip.label.is_empty() {
            draw_text_overlay(ctx, tip.label, tip.screen_x + 3.0, tip.screen_y - 4.0, ctx.theme.font_size_small, stroke);
        }
    }
}
//#endregion 🧭WorldOrbitViewGizmo

pub fn world3d_hit_target(scene: &UiComponentSceneNode, bounds: Rect) -> HitTarget<ActionDescriptor> {
    HitTarget { rect: bounds, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None }
}

pub fn handle_world3d_pointer_move(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        state.gizmo_hovered_tip = None;
        return None;
    }
    if down && button == 0 {
        if state.marquee_active {
            state.marquee_points.push([x, y]);
            update_marquee_preview(state, render_pick_viewport(state));
        } else if state.paint_stroke_active && state.interaction_mode == "paint" {
            if let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: state.controller_id.clone(),
                    action: "paintAt".into(),
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
        update_world_orbit_view_gizmo_hover(state, x, y, inner);
        return pick_hover_action(state, x, y, inner);
    }
    if button == 2 {
        return None;
    }
    None
}

pub fn handle_world3d_paint_actions(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16) -> Vec<ActionDescriptor> {
    if state.interaction_mode != "paint" || button != 0 {
        return Vec::new();
    }
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        return Vec::new();
    }
    if down && state.paint_stroke_active {
        let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) else {
            return Vec::new();
        };
        return vec![ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "paintAt".into(),
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

pub fn handle_world3d_pointer_button(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        return None;
    }
    let shift = modifiers.shift;
    let ctrl = modifiers.ctrl;
    if down {
        if button == 0 {
            if state.interaction_mode == "paint" {
                state.paint_stroke_active = true;
                return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "paintStrokeBegin".into(), args: Some(json!({ "surfaceId": state.surface_id })) });
            }
            if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
                if let Some(full_id) = pick_vortex_at(state, x, y, inner) {
                    let merge = if shift {
                        "add"
                    } else if ctrl {
                        "toggle"
                    } else {
                        "replace"
                    };
                    return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldVortexSelect".into(), args: Some(json!({ "surfaceId": state.surface_id, "fullId": full_id, "merge": merge })) });
                }
            } else if state.active_utility == "select" {
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
            state.right_press_point = Some([x, y]);
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
                return pick_select_action(state, x, y, inner, shift, ctrl);
            }
        } else {
            return marquee_select_action(state, render_pick_viewport(state), shift, ctrl);
        }
    }
    if button == 0 && state.interaction_mode == "paint" && state.paint_stroke_active {
        state.paint_stroke_active = false;
        let mut actions = Vec::new();
        actions.push(ActionDescriptor { controller_id: state.controller_id.clone(), action: "paintStrokeEnd".into(), args: Some(json!({ "surfaceId": state.surface_id })) });
        return actions.first().cloned();
    }
    if button == 0 {
        if let Some(action) = gumball_commit_action(state) {
            state.gumball_handle = None;
            reset_gumball_preview(state);
            return Some(action);
        }
        state.gumball_handle = None;
        reset_gumball_preview(state);
        state.press_object_id = None;
        if let Some(object_id) = state.drag_object_id.take() {
            if let Some(position) = state.drag_last_position {
                if !is_click_gesture(state, x, y) {
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "worldRelocate".into(),
                        args: Some(json!({
                            "surfaceId": state.surface_id,
                            "objectId": object_id,
                            "position": position,
                        })),
                    });
                }
            }
        }
        if state.active_utility == "brush" {
            if let Some(preview) = state.brush_preview.clone() {
                if let (Some(target), Some(kind), Some(index)) = (preview.target_vortex_full_id, preview.object_kind_id, preview.source_vortex_index) {
                    let origin = preview.origin.unwrap_or([0.0, 0.0, 0.0]);
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "addBrushObject".into(),
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
        return pick_select_action(state, x, y, inner, shift, ctrl);
    }
    if button == 2 {
        // 🖱️ A right-drag (orbit-via-right-button, see `handle_world3d_pointer_drag`) must not
        // also pop a context menu — only a right-*click* (no meaningful movement since press)
        // resolves+dispatches a context-menu target, mirroring the React reference's
        // `onContextMenu` (which only fires on a genuine click, not a drag-then-release).
        let is_click = state
            .right_press_point
            .map(|start| {
                let dx = x - start[0];
                let dy = y - start[1];
                (dx * dx + dy * dy).sqrt() <= CLICK_DRAG_THRESHOLD_PX
            })
            .unwrap_or(true);
        state.right_press_point = None;
        if is_click {
            if let Some((kind, id)) = resolve_world_context_menu_target(state) {
                return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "contextMenuAt".into(), args: Some(json!({ "surfaceId": state.surface_id, "kind": kind, "id": id })) });
            }
        }
        return Some(orbit_camera_action(state));
    }
    if button == 1 {
        return Some(orbit_camera_action(state));
    }
    None
}

/// 🖱️📋 Resolves which entity a right-click context menu targets, in the React reference's exact
/// priority order — `resolveWorldContextMenuTarget` in `world-3d-host.tsx`: a hovered vortex wins
/// first, then a hovered mesh component (vertex/edge/face — reported as kind `"object"`, the
/// component's owning instance, matching the React source's own naming), then a hovered reference
/// image plane; `None` (no context menu) if nothing is currently hovered.
fn resolve_world_context_menu_target(state: &World3dState) -> Option<(&'static str, String)> {
    if let Some(vortex_id) = state.hovered_vortex_id.clone() {
        return Some(("vortex", vortex_id));
    }
    if state.hovered_component_mode.is_some() {
        if let Some(object_id) = state.hovered_component_object_id.clone() {
            return Some(("object", object_id));
        }
    }
    if let Some(reference_id) = state.local_hover_id.as_deref().and_then(|hovered| hovered.strip_prefix("reference:")) {
        return Some(("reference", reference_id.to_string()));
    }
    None
}

pub fn handle_world3d_pointer_drag(state: &mut World3dState, x: f32, y: f32, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) {
    let inner = world_pick_rect(state);
    if button == 0 {
        if state.gumball_handle.is_some() && inner.contains(x, y) {
            gumball_drag_update(state, x, y, inner);
            return;
        }
        if state.drag_object_id.is_none() && state.gumball_handle.is_none() && !component_mode_active(state) && pointer_drag_distance(state, x, y) > CLICK_DRAG_THRESHOLD_PX {
            if let Some(object_id) = state.press_object_id.take().or_else(|| pick_instance_at(state, x, y, inner)) {
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

/// Applies hover/selection action payloads to renderer-local world state before the plugin round-trip.
pub fn apply_world_action_preview(state: &mut World3dState, action: &ActionDescriptor) {
    let Some(args) = action.args.as_ref() else {
        if action.action == "setHover" {
            state.hovered_component_id = None;
            state.hovered_component_object_id = None;
            state.hovered_component_mode = None;
            state.local_hover_id = None;
        }
        return;
    };
    match action.action.as_str() {
        "setHover" => {
            if args.get("objectId").is_none() {
                state.hovered_component_id = None;
                state.hovered_component_object_id = None;
                state.hovered_component_mode = None;
                state.local_hover_id = None;
            } else {
                let object_id = args.get("objectId").and_then(|value| value.as_str()).map(str::to_string);
                let mode = args.get("mode").and_then(|value| value.as_str()).map(str::to_string);
                let id = args.get("id").and_then(json_id_to_string);
                state.hovered_component_object_id = object_id.clone();
                state.hovered_component_mode = mode.clone();
                state.hovered_component_id = id;
                if mode.as_deref() == Some("mesh") {
                    state.local_hover_id = object_id;
                } else {
                    state.local_hover_id = None;
                }
            }
        }
        "worldPick" => {
            let merge = args.get("merge").and_then(|value| value.as_str()).unwrap_or("replace");
            if let Some(granularity) = args.get("granularity").and_then(|value| value.as_str()) {
                state.granularity = granularity.to_string();
            }
            if args.get("id").is_none_or(|value| value.is_null()) {
                if merge == "replace" {
                    state.component_ids.clear();
                }
            } else if let Some(id) = args.get("id").and_then(json_id_to_string) {
                state.component_ids = merge_string_ids(&state.component_ids, &[id], merge);
            }
        }
        "worldSelect" => {
            let merge = args.get("merge").and_then(|value| value.as_str()).unwrap_or("replace");
            let ids: Vec<String> = args.get("ids").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
            state.selected_ids = merge_string_ids(&state.selected_ids, &ids, merge);
        }
        "worldHover" => {
            state.local_hover_id = args.get("id").and_then(|value| value.as_str()).map(str::to_string);
        }
        "setSelection" => {
            if let Some(mode) = args.get("mode").and_then(|value| value.as_str()) {
                state.granularity = mode.to_string();
            }
            let ids: Vec<String> = args.get("ids").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
            state.component_ids = ids;
        }
        _ => {}
    }
}

pub fn ingest_glb_mesh(state: &mut World3dState, url: &str, mesh: MeshData, mesh_id: String) {
    state.pending_glb_urls.remove(url);
    store_mesh(state, mesh_id, mesh_from_data(&mesh));
}

fn pick_hover_action(state: &mut World3dState, x: f32, y: f32, inner: Rect) -> Option<ActionDescriptor> {
    if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
        let hit = pick_vortex_at(state, x, y, inner);
        if state.hovered_vortex_id == hit {
            return None;
        }
        state.hovered_vortex_id = hit.clone();
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldVortexHover".into(), args: Some(json!({ "surfaceId": state.surface_id, "fullId": hit })) });
    }
    if component_mode_active(state) {
        if let Some((mode, id, object_id)) = pick_component_at(state, x, y, inner) {
            if state.hovered_component_id.as_deref() == Some(id.as_str()) && state.hovered_component_object_id.as_deref() == Some(object_id.as_str()) && state.hovered_component_mode.as_deref() == Some(mode.as_str()) {
                return None;
            }
            state.hovered_component_id = Some(id.clone());
            state.hovered_component_object_id = Some(object_id.clone());
            state.hovered_component_mode = Some(mode.clone());
            let id_num = id.parse::<u64>().unwrap_or(0);
            return Some(ActionDescriptor {
                controller_id: state.controller_id.clone(),
                action: "setHover".into(),
                args: Some(json!({
                    "objectId": object_id,
                    "mode": mode,
                    "id": id_num,
                })),
            });
        }
        if state.hovered_component_id.is_none() && state.hovered_component_object_id.is_none() && state.hovered_component_mode.is_none() {
            return None;
        }
        state.hovered_component_id = None;
        state.hovered_component_object_id = None;
        state.hovered_component_mode = None;
        state.local_hover_id = None;
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "setHover".into(), args: None });
    }
    // 🖼️ Falls back to reference-image plane hit-testing when no mesh instance is under the
    // cursor — mirrors React's `hoveredId` covering both mesh instances and `"reference:"`-prefixed
    // reference planes (see `resolveWorldContextMenuTarget`'s reference tier).
    let hit = pick_instance_at(state, x, y, inner).or_else(|| pick_reference_at(state, x, y, inner).map(|url| format!("reference:{url}")));
    if state.local_hover_id == hit {
        return None;
    }
    state.local_hover_id = hit.clone();
    Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldHover".into(), args: Some(json!({ "surfaceId": state.surface_id, "id": hit })) })
}

fn pick_select_action(state: &World3dState, x: f32, y: f32, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    let merge = if shift {
        "add"
    } else if ctrl {
        "toggle"
    } else {
        "replace"
    };
    if state.interaction_mode == "paint" {
        return None;
    }
    if component_mode_active(state) {
        let Some((granularity, id, _object_id)) = pick_component_at(state, x, y, inner) else {
            return Some(ActionDescriptor {
                controller_id: state.controller_id.clone(),
                action: "worldPick".into(),
                args: Some(json!({
                    "surfaceId": state.surface_id,
                    "granularity": state.granularity,
                    "id": null,
                    "merge": merge,
                })),
            });
        };
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "worldPick".into(),
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
        let id = hit.as_deref().and_then(|object_id| instance_object_index(state, object_id));
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "worldPick".into(),
            args: Some(json!({
                "surfaceId": state.surface_id,
                "granularity": "mesh",
                "id": id,
                "merge": merge,
            })),
        });
    }
    let hit = pick_instance_at(state, x, y, inner);
    Some(ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "worldSelect".into(),
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
    let parse = |ids: &[String]| -> Vec<u32> { ids.iter().filter_map(|id| id.parse().ok()).collect() };
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

fn marquee_select_action(state: &mut World3dState, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    if state.marquee_points.len() < 2 {
        return None;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let (polygon, rectangle, crossing) = marquee_local_polygon(state, inner);
    let ids = if component_mode_active(state) {
        screen_select_components(&state.meshes, &state.draws, view_proj, inner.w, inner.h, &polygon, rectangle, state.granularity.as_str(), state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&state.meshes, &state.draws, view_proj, inner.w, inner.h, &polygon, rectangle, crossing)
    };
    state.marquee_points.clear();
    state.marquee_preview_ids.clear();
    let merge = if shift {
        "add"
    } else if ctrl {
        "toggle"
    } else {
        "replace"
    };
    if component_mode_active(state) {
        let merged = merge_u32_ids(&state.component_ids, &ids, merge);
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "setSelection".into(),
            args: Some(json!({
                "mode": state.granularity,
                "ids": merged,
            })),
        });
    }
    Some(ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "worldSelect".into(),
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
                    state.gumball_preview_angle = axis_rotate_angle(state.gumball_drag_start_vec, current, normal);
                }
            }
        }
    } else if handle.is_scale() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let factor = if state.gumball_drag_anchor.abs() > 1e-4 { current / state.gumball_drag_anchor } else { 1.0 };
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
        state.gumball_drag_anchor = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye).unwrap_or(0.0);
    } else if let Some(normal) = handle.plane_normal() {
        state.gumball_drag_start_vec = ray_plane_point(origin, dir, pivot, normal).unwrap_or(pivot);
        if handle.is_rotate() {
            state.gumball_drag_start_vec = state.gumball_drag_start_vec.sub(pivot);
        }
        state.gumball_drag_anchor = 0.0;
    }
}

fn pick_component_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, String, String)> {
    let (local_x, local_y, rect) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (rect.w / rect.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let granularity = state.granularity.as_str();
    match granularity {
        "vertex" => {
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    for (vertex_index, chunk) in mesh.positions.as_chunks::<3>().0.iter().enumerate() {
                        let world = instance.model.transform_point(Vec3::new(chunk[0], chunk[1], chunk[2]));
                        let Some(screen) = kernel_3d_scene::project_point(view_proj, world, rect.w, rect.h) else {
                            continue;
                        };
                        let dx = screen[0] - local_x;
                        let dy = screen[1] - local_y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= PICK_VERTEX_SCREEN_PX && best.as_ref().is_none_or(|(best_dist, _, _)| dist < *best_dist) {
                            let id = mesh.vertex_ids.get(vertex_index).map(|value| value.to_string()).unwrap_or_else(|| vertex_index.to_string());
                            best = Some((dist, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "edge" => {
            let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, f32, String, String)> = None;
            for draw in &state.draws {
                let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                if mesh.edge_positions.is_empty() {
                    continue;
                }
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    for (edge_index, chunk) in mesh.edge_positions.as_chunks::<6>().0.iter().enumerate() {
                        let a = instance.model.transform_point(Vec3::new(chunk[0], chunk[1], chunk[2]));
                        let b = instance.model.transform_point(Vec3::new(chunk[3], chunk[4], chunk[5]));
                        let (Some(screen_a), Some(screen_b)) = (kernel_3d_scene::project_point(view_proj, a, rect.w, rect.h), kernel_3d_scene::project_point(view_proj, b, rect.w, rect.h)) else {
                            continue;
                        };
                        let screen_dist = kernel_3d_scene::screen_segment_distance(local_x, local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                        if screen_dist > PICK_EDGE_SCREEN_PX {
                            continue;
                        }
                        let ray_dist = kernel_3d_scene::ray_segment_distance(origin, dir, a, b).unwrap_or(f32::INFINITY);
                        let depth = a.add(b).scale(0.5).sub(origin).dot(dir);
                        let better = match &best {
                            None => true,
                            Some((best_ray, best_depth, _, _)) => depth < *best_depth - 1e-4 || ((depth - *best_depth).abs() <= 1e-4 && ray_dist < *best_ray),
                        };
                        if better {
                            let id = mesh.edge_ids.get(edge_index).map(|value| value.to_string()).unwrap_or_else(|| edge_index.to_string());
                            best = Some((ray_dist, depth, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, _, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "face" => {
            let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    let Some(hit) = ray_pick_mesh_detail(origin, dir, mesh, instance) else {
                        continue;
                    };
                    if best.as_ref().is_none_or(|(best_depth, _, _)| hit.distance < *best_depth) {
                        let id = mesh_face_id(mesh, hit.triangle_index);
                        best = Some((hit.distance, id, instance.id.clone()));
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        _ => {}
    }
    None
}

fn pick_paint_hit(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, f32, f32)> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, f32, f32)> = None;
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(hit) = ray_pick_mesh_detail(origin, dir, mesh, instance) {
                if let Some((u, v)) = interpolate_mesh_uv(mesh, hit.triangle_index, hit.bary_u, hit.bary_v) {
                    if best.as_ref().is_none_or(|(best_dist, _, _, _)| hit.distance < *best_dist) {
                        best = Some((hit.distance, instance.id.clone(), u, v));
                    }
                }
            }
        }
    }
    best.map(|(_, object_id, u, v)| (object_id, u, v))
}

fn marquee_local_polygon(state: &World3dState, rect: Rect) -> (Vec<[f32; 2]>, bool, bool) {
    let rectangle = state.selection_method != "lasso";
    let crossing = marquee_is_crossing_from_path(&state.marquee_points, !rectangle);
    let global: Vec<[f32; 2]> = if rectangle {
        let start = state.marquee_points[0];
        let end = state.marquee_points[state.marquee_points.len() - 1];
        vec![start, end]
    } else {
        state.marquee_points.clone()
    };
    let local = global.iter().map(|point| [point[0] - rect.x, point[1] - rect.y]).collect();
    (local, rectangle, crossing)
}

fn update_marquee_preview(state: &mut World3dState, inner: Rect) {
    if state.marquee_points.len() < 2 {
        state.marquee_preview_ids.clear();
        return;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = camera.view_proj(aspect);
    let (polygon, rectangle, crossing) = marquee_local_polygon(state, inner);
    state.marquee_preview_ids = if component_mode_active(state) {
        screen_select_components(&state.meshes, &state.draws, view_proj, inner.w, inner.h, &polygon, rectangle, state.granularity.as_str(), state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&state.meshes, &state.draws, view_proj, inner.w, inner.h, &polygon, rectangle, crossing)
    };
}

fn pick_instance_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(distance) = ray_pick_instance(origin, dir, mesh, instance) {
                if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                    best = Some((distance, instance.id.clone()));
                }
            }
        }
    }
    best.map(|(_, id)| id)
}

//#region VortexArrow
const VORTEX_ARROW_SHAFT_MESH: &str = "cylinder";
const VORTEX_ARROW_HEAD_MESH: &str = "cone";

struct VortexArrowLayout {
    point_radius: f32,
    shaft_radius: f32,
    shaft_length: f32,
    head_length: f32,
    shaft_center: [f32; 3],
    head_base: [f32; 3],
    rotation: [f32; 4],
}

fn quat_normalize_f32(quat: [f32; 4]) -> [f32; 4] {
    let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
    if len < 1e-9 {
        [0.0, 0.0, 0.0, 1.0]
    } else {
        [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
    }
}

fn quat_from_unit_vectors(from: Vec3, to: Vec3) -> [f32; 4] {
    let from = from.normalize();
    let to = to.normalize();
    let r = from.dot(to) + 1.0;
    let quat = if r < 0.000_001 {
        if from.x.abs() > from.z.abs() {
            [-from.y, from.x, 0.0, r]
        } else {
            [0.0, -from.z, from.y, r]
        }
    } else {
        let cross = from.cross(to);
        [cross.x, cross.y, cross.z, r]
    };
    quat_normalize_f32(quat)
}

fn vortex_unit_direction(direction: Option<[f64; 3]>) -> Vec3 {
    let dir = direction.map(|value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32)).unwrap_or(Vec3::new(0.0, 0.0, -1.0));
    if dir.dot(dir) < 1e-12 {
        Vec3::new(0.0, 0.0, -1.0)
    } else {
        dir.normalize()
    }
}

fn vortex_arrow_layout(position: [f64; 3], direction: Option<[f64; 3]>, radius: f32, display_direction: Option<&str>) -> VortexArrowLayout {
    let dir = vortex_unit_direction(direction);
    let pos = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
    let arrow_length = radius;
    let head_length = radius * 0.28;
    let shaft_length = (arrow_length - head_length).max(radius * 0.2);
    let shaft_radius = radius * 0.055;
    let point_radius = radius * 0.18;
    let outward = !display_direction.is_some_and(|mode| mode == "inwards");
    let rotation = quat_from_unit_vectors(Vec3::new(0.0, 1.0, 0.0), dir);
    let (shaft_center, head_base) = if outward {
        (pos.add(dir.scale(shaft_length * 0.5)), pos.add(dir.scale(shaft_length)))
    } else {
        (pos.sub(dir.scale(head_length + shaft_length * 0.5)), pos.sub(dir.scale(head_length)))
    };
    VortexArrowLayout { point_radius, shaft_radius, shaft_length, head_length, shaft_center: shaft_center.to_array(), head_base: head_base.to_array(), rotation }
}

fn ensure_primitive_mesh(state: &mut World3dState, mesh_key: &str) {
    if state.meshes.contains_key(mesh_key) {
        return;
    }
    let primitive = mesh_from_kind(mesh_key);
    store_mesh(state, mesh_key.into(), Mesh3d::from_buffers(primitive.positions, primitive.normals, primitive.indices));
}

fn append_vortex_arrow_draws(state: &mut World3dState, gpu: &mut GpuContext, extra_draws: &mut Vec<SceneDraw3d>) {
    if state.vortices.is_empty() {
        return;
    }
    let mut point_instances = Vec::new();
    let mut shaft_instances = Vec::new();
    let mut head_instances = Vec::new();
    for vortex in &state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let layout = vortex_arrow_layout(position, vortex.direction, radius, vortex.display_direction.as_deref());
        let color = parse_color(vortex.color.as_deref().unwrap_or("#38bdf8"));
        let hovered = state.hovered_vortex_id.as_deref() == Some(vortex.full_id.as_str());
        let id = vortex.full_id.clone();
        point_instances.push(Instance3d {
            id: format!("{id}:point"),
            model: Instance3d::model_from_trs([position[0] as f32, position[1] as f32, position[2] as f32], [0.0, 0.0, 0.0, 1.0], [layout.point_radius, layout.point_radius, layout.point_radius]),
            color,
            selected: false,
            hovered,
        });
        shaft_instances.push(Instance3d {
            id: format!("{id}:shaft"),
            model: Instance3d::model_from_trs(layout.shaft_center, layout.rotation, [layout.shaft_radius * 2.0, layout.shaft_length, layout.shaft_radius * 2.0]),
            color,
            selected: false,
            hovered,
        });
        head_instances.push(Instance3d {
            id: format!("{id}:head"),
            model: Instance3d::model_from_trs(layout.head_base, layout.rotation, [layout.shaft_radius * 3.6, layout.head_length, layout.shaft_radius * 3.6]),
            color,
            selected: false,
            hovered,
        });
    }
    ensure_primitive_mesh(state, "vortex-marker");
    ensure_primitive_mesh(state, VORTEX_ARROW_SHAFT_MESH);
    ensure_primitive_mesh(state, VORTEX_ARROW_HEAD_MESH);
    for (mesh_key, instances) in [("vortex-marker", point_instances), (VORTEX_ARROW_SHAFT_MESH, shaft_instances), (VORTEX_ARROW_HEAD_MESH, head_instances)] {
        let mesh_version = *state.mesh_versions.get(mesh_key).unwrap_or(&0);
        if let Some(mesh) = state.meshes.get(mesh_key) {
            gpu.ensure_mesh(mesh_key, mesh_version, &mesh.positions, &mesh.normals, &mesh.indices);
        }
        extra_draws.push(SceneDraw3d { mesh_key: mesh_key.into(), mesh_version, instances });
    }
}
//#endregion VortexArrow

fn pick_vortex_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for vortex in &state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
        let min = [center.x - radius, center.y - radius, center.z - radius];
        let max = [center.x + radius, center.y + radius, center.z + radius];
        if let Some(distance) = ray_aabb_slab(origin, dir, min, max) {
            if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                best = Some((distance, vortex.full_id.clone()));
            }
        }
    }
    best.map(|(_, id)| id)
}

/// 🖼️ Ray-vs-quad hit test against visible reference-image planes, returning the closest hit
/// reference's `url` (used as its hover/context-menu identifier, since `WorldReferenceRecord` has
/// no separate id field). References are flat rectangles lying in the local XY plane (normal +Z,
/// width along X, height along Y) centered at `origin` — matching this renderer's Z-up convention.
fn pick_reference_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
    let plane_normal = Vec3::new(0.0, 0.0, 1.0);
    let mut best: Option<(f32, String)> = None;
    for reference in &state.references {
        if reference.hidden.unwrap_or(false) {
            continue;
        }
        let Some(url) = reference.url.as_deref() else {
            continue;
        };
        let plane_position = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let plane_origin = Vec3::new(plane_position[0] as f32, plane_position[1] as f32, plane_position[2] as f32);
        let width = reference.width_world.unwrap_or(1.0) as f32;
        let image_aspect = state.reference_aspect.get(url).copied().unwrap_or(1.0);
        let height = width / image_aspect.max(0.01);
        let Some(hit) = ray_plane_point(origin, dir, plane_origin, plane_normal) else {
            continue;
        };
        let offset = hit.sub(plane_origin);
        if offset.x.abs() > width * 0.5 || offset.y.abs() > height * 0.5 {
            continue;
        }
        let distance = origin.sub(hit).length();
        if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
            best = Some((distance, url.to_string()));
        }
    }
    best.map(|(_, url)| url)
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

fn ground_plane_pick(state: &World3dState, x: f32, y: f32, _inner: Rect, plane_z: f32) -> Option<[f32; 3]> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, viewport.w, viewport.h);
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
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0) as f32, values[1].as_f64().unwrap_or(1.0) as f32, values[2].as_f64().unwrap_or(1.0) as f32],
        _ => [1.0, 1.0, 1.0],
    }
}

/// 👻 Mesh key for `BrushPreviewGhost`: the real GLB's resolved id when a `meshUrl` is given
/// (loaded lazily, same as any other mesh), else the shared "box" primitive fallback ghost.
fn brush_preview_mesh_id(mesh_url: Option<&str>) -> String {
    mesh_url.map(mesh_id_from_url).unwrap_or_else(|| "box".to_string())
}

fn append_box_wireframe(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], orientation: [f64; 4], scale: [f64; 3], color: [f32; 4]) {
    let corners = [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]];
    let model = Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [orientation[0] as f32, orientation[1] as f32, orientation[2] as f32, orientation[3] as f32], [scale[0] as f32, scale[1] as f32, scale[2] as f32]);
    let world_corners: Vec<[f32; 3]> = corners.iter().map(|corner| model.transform_point(Vec3::new(corner[0], corner[1], corner[2])).to_array()).collect();
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    for (a, b) in edges {
        lines.push(LineVertex3d { position: world_corners[a], color });
        lines.push(LineVertex3d { position: world_corners[b], color });
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
            pending.push(PendingGlbFetch { surface_id: surface_id.clone(), url: url.clone() });
        }
    }
    pending
}

fn mesh_id_from_url(url: &str) -> String {
    let slug = url.trim_start_matches('/').rsplit('/').next().unwrap_or(url).trim_end_matches(".glb").trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

pub fn apply_glb_bytes(state: &mut World3dState, url: &str, bytes: &[u8]) {
    let mesh_id = mesh_id_from_url(url);
    state.mesh_source_urls.insert(mesh_id.clone(), url.to_string());
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
        let workspace_relative = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..").join(url.trim_start_matches('/'));
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
    let reader = image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok();
    let Some(reader) = reader else {
        return;
    };
    if let Ok(image) = reader.decode() {
        let rgba = image.to_rgba8();
        let aspect = rgba.width() as f32 / rgba.height().max(1) as f32;
        state.reference_aspect.insert(url.to_string(), aspect);
        state.reference_pixels.insert(url.to_string(), (rgba.width(), rgba.height(), rgba.into_raw()));
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

//#region TerrainFetch
#[derive(Clone, Debug)]
pub struct PendingTerrainTileFetch {
    pub surface_id: String,
    pub url: String,
    pub z: u32,
    pub x: u32,
    pub y: u32,
}

pub fn collect_pending_terrain_tile_fetches(states: &HashMap<String, World3dState>) -> Vec<PendingTerrainTileFetch> {
    let mut pending = Vec::new();
    for (surface_id, state) in states {
        for (url, &(z, x, y)) in &state.pending_terrain_tile_urls {
            pending.push(PendingTerrainTileFetch { surface_id: surface_id.clone(), url: url.clone(), z, x, y });
        }
    }
    pending
}

/// 🏔️📡 Byte-fetch companion to `sync_terrain`'s tile-visibility bookkeeping — mirrors
/// `fetch_pending_glb_meshes`/`fetch_pending_reference_images`. Needs a call from the renderer's
/// async poll loop (alongside those two) to actually run; see report for the exact wiring request.
pub async fn fetch_pending_terrain_tiles(states: &mut HashMap<String, World3dState>) {
    let pending = collect_pending_terrain_tile_fetches(states);
    for item in pending {
        let Some(bytes) = fetch_url_bytes(&item.url).await else {
            continue;
        };
        if let Some(state) = states.get_mut(&item.surface_id) {
            state.terrain_session.upload_elevation_tile(item.z, item.x, item.y, &bytes);
            state.pending_terrain_tile_urls.remove(&item.url);
        }
    }
}
//#endregion TerrainFetch

#[cfg(test)]
mod tests {
    use super::*;
    use ui_wgpu::{SurfaceKind, UiComponentSceneNode, UiPresence, World3dScene};

    #[test]
    fn world_orbit_view_gizmo_placement_matches_react_bottom_right_insets() {
        assert_eq!(world_orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 }), (32.0, 32.0));
        assert_eq!(world_orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 120.0, h: 160.0 }), (32.0, 32.0));
        assert_eq!(world_orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 40.0, h: 48.0 }), (22.0, 22.0));
    }

    fn topology_mesh() -> Mesh3d {
        let mut mesh = Mesh3d::from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2, 1, 3, 2]);
        mesh.face_ids = vec![10, 11];
        mesh.vertex_ids = vec![1, 2, 3, 4];
        mesh.edge_positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
        mesh.edge_ids = vec![5, 6];
        mesh
    }

    fn scene_with_selection(selection_json: &str) -> UiComponentSceneNode {
        UiComponentSceneNode {
            presence: UiPresence::default(),
            surface_id: "surface-1".into(),
            controller_id: "controller-1".into(),
            component_kind: SurfaceKind::World3d,
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
                engagement_preview_json: None,
                lod_json: None,
                chunking_json: None,
                context_menu_json: None,
                environment_json: None,
                frame_json: None,
                fit_json: None,
                terrain_json: None,
                points_json: None,
                status_json: None,
            }),
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
        }
    }

    #[test]
    fn sync_parses_selection_targets_and_active_object() {
        let selection = r#"{
            "granularity":"vertex",
            "targets":{"mesh":true,"vertex":true,"edge":false,"face":false},
            "activeObjectId":"obj-1"
        }"#;
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert!(state.selection_targets.vertex);
        assert!(!state.selection_targets.edge);
        assert_eq!(state.active_object_id.as_deref(), Some("obj-1"));
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
        sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert_eq!(state.granularity, "vertex");
        assert_eq!(state.component_ids, vec!["1".to_string(), "2".to_string()]);
        assert_eq!(state.hovered_component_id.as_deref(), Some("3"));
        assert_eq!(state.hovered_component_object_id.as_deref(), Some("obj-1"));
        assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));
        assert!(state.show_edges);
    }

    #[test]
    fn append_component_vertex_spheres_render_base_vertices() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.selection_targets.vertex = true;
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let instances = append_component_vertex_spheres(&mut state);
        assert_eq!(instances.len(), 0);

        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert_eq!(lines.len(), 28); // 4 from edges + 24 from 4 vertex crosses
    }

    #[test]
    fn append_component_overlays_highlights_only_hovered_edge() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "edge".into();
        state.selection_targets.edge = true;
        state.hovered_component_id = Some("5".into());
        state.hovered_component_object_id = Some("obj-1".into());
        state.hovered_component_mode = Some("edge".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(lines.len() >= 2);
        assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
    }

    #[test]
    fn append_component_overlays_highlights_selected_edge() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "edge".into();
        state.component_ids = vec!["6".into()];
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(lines.len() >= 2);
        assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
    }

    #[test]
    fn component_mode_does_not_apply_mesh_instance_hover() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.hovered_component_object_id = Some("obj-1".into());
        state.local_hover_id = None;
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        apply_runtime_draw_flags(&mut state);
        assert!(!state.draws[0].instances[0].hovered);
    }

    #[test]
    fn runtime_draw_flags_clear_stale_instance_selected_when_selection_is_empty() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.selected_ids.clear();
        state.local_hover_id = None;
        state.draws.push(SceneDraw3d {
            mesh_key: "mesh-1".into(),
            mesh_version: 0,
            instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: true }],
        });
        apply_runtime_draw_flags(&mut state);
        assert!(!state.draws[0].instances[0].selected, "empty selection must clear a stale instancesJson selected bit");
        assert!(!state.draws[0].instances[0].hovered, "empty hover must clear a stale instancesJson hovered bit");
    }

    #[test]
    fn merge_u32_ids_supports_add_and_toggle() {
        assert_eq!(merge_u32_ids(&["1".into()], &["2".into()], "add"), vec![1, 2]);
        assert_eq!(merge_u32_ids(&["1".into(), "2".into()], &["2".into(), "3".into()], "toggle"), vec![1, 3]);
    }

    #[test]
    fn pick_select_emits_numeric_world_pick_id() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let screen = kernel_3d_scene::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("vertex projects");
        let action = pick_select_action(&state, screen[0], screen[1], inner, false, false).expect("pick action");
        assert_eq!(action.action, "worldPick");
        let args = action.args.expect("args");
        assert_eq!(args["id"], json!(1));
    }

    #[test]
    fn marquee_preview_respects_pick_bounds_offset() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 100.0, y: 50.0, w: 400.0, h: 400.0 };
        state.pick_bounds = inner;
        state.marquee_points = vec![[110.0, 60.0], [490.0, 450.0]];
        update_marquee_preview(&mut state, inner);
        assert!(!state.marquee_preview_ids.is_empty(), "preview ids: {:?}", state.marquee_preview_ids);
    }

    #[test]
    fn marquee_crossing_includes_partial_overlap_window_does_not() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "mesh".into();
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let view_proj = camera.view_proj(1.0);
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        for corner in [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]] {
            let screen = kernel_3d_scene::project_point(view_proj, Vec3::from_array(corner), inner.w, inner.h).expect("screen");
            min_x = min_x.min(screen[0]);
            min_y = min_y.min(screen[1]);
            max_x = max_x.max(screen[0]);
            max_y = max_y.max(screen[1]);
        }
        let center_x = (min_x + max_x) * 0.5;
        let center_y = (min_y + max_y) * 0.5;
        state.marquee_points = vec![[inner.x + min_x, inner.y + min_y], [inner.x + center_x, inner.y + center_y]];
        update_marquee_preview(&mut state, inner);
        assert!(state.marquee_preview_ids.is_empty(), "window marquee should not select partially enclosed mesh");
        state.marquee_points = vec![[inner.x + center_x, inner.y + min_y], [inner.x + min_x, inner.y + center_y]];
        update_marquee_preview(&mut state, inner);
        assert!(!state.marquee_preview_ids.is_empty(), "crossing marquee should select partially enclosed mesh");
    }

    #[test]
    fn marquee_component_mode_emits_set_selection_with_numeric_ids() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.component_ids = vec!["1".into()];
        state.marquee_points = vec![[10.0, 10.0], [390.0, 390.0]];
        state.meshes.insert("mesh-1".into(), topology_mesh());
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let action = marquee_select_action(&mut state, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, true, false).expect("marquee action");
        assert_eq!(action.action, "setSelection");
        let args = action.args.expect("args");
        assert_eq!(args["mode"], json!("vertex"));
        assert!(args["ids"].as_array().is_some());
    }

    #[test]
    fn click_release_routes_to_pick_select_instead_of_empty_marquee() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.active_utility = "select".into();
        state.granularity = "mesh".into();
        state.marquee_active = true;
        state.marquee_points = vec![[120.0, 140.0]];
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let action = handle_world3d_pointer_button(&mut state, 120.0, 140.0, false, 0, &PointerModifiers::default()).expect("click should pick");
        assert_eq!(action.action, "worldPick");
        assert!(!state.marquee_active);
    }

    #[test]
    fn marquee_face_preview_and_overlay_use_logical_face_ids() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = bounds;
        state.pick_bounds = bounds;
        state.marquee_points = vec![[390.0, 10.0], [10.0, 390.0]];
        update_marquee_preview(&mut state, bounds);
        assert!(state.marquee_preview_ids.iter().any(|id| id == "10" || id == "11"), "preview ids: {:?}", state.marquee_preview_ids);
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(!lines.is_empty(), "face marquee preview should draw triangle edge lines");
    }

    #[test]
    fn hovered_component_preserved_when_selection_json_omits_hover_field() {
        let selection = r#"{"granularity":"face","componentIds":[10]}"#;
        let scene = scene_with_selection(selection);
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.hovered_component_id = Some("11".into());
        state.hovered_component_object_id = Some("obj-1".into());
        state.hovered_component_mode = Some("face".into());
        sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert_eq!(state.hovered_component_id.as_deref(), Some("11"));
    }

    #[test]
    fn apply_world_action_preview_updates_component_hover_and_selection() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "setHover".into(),
                args: Some(json!({
                    "objectId": "obj-1",
                    "mode": "vertex",
                    "id": 2,
                })),
            },
        );
        assert_eq!(state.hovered_component_id.as_deref(), Some("2"));
        assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));
        assert!(state.local_hover_id.is_none());

        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "worldPick".into(),
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
        sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        apply_world_action_preview(
            &mut state,
            &ActionDescriptor {
                controller_id: "controller-1".into(),
                action: "worldPick".into(),
                args: Some(json!({
                    "granularity": "vertex",
                    "id": 5,
                    "merge": "replace",
                })),
            },
        );
        sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
        assert_eq!(state.component_ids, vec!["5".to_string()]);
    }

    #[test]
    fn pick_viewport_uses_render_bounds_not_pick_clip_offset() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "vertex".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let bounds = Rect { x: 0.0, y: 50.0, w: 400.0, h: 400.0 };
        let clip = Rect { x: 0.0, y: 100.0, w: 400.0, h: 400.0 };
        state.bounds = bounds;
        state.pick_bounds = clip;
        let camera = state.orbit.to_camera();
        let screen = kernel_3d_scene::project_point(camera.view_proj(1.0), Vec3::ZERO, bounds.w, bounds.h).expect("vertex projects");
        let global_x = bounds.x + screen[0];
        let global_y = bounds.y + screen[1];
        let picked = pick_component_at(&state, global_x, global_y, bounds).expect("vertex pick respects render viewport");
        assert_eq!(picked.1, "1");
    }

    #[test]
    fn append_component_face_overlay_lines_include_hovered_face() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.hovered_component_mode = Some("face".into());
        state.hovered_component_id = Some("10".into());
        state.hovered_component_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let mut lines = Vec::new();
        append_component_overlays(&state, &mut lines);
        assert!(lines.len() >= 6, "hovered face should emit triangle edge lines, got {}", lines.len());
    }

    #[test]
    fn pick_component_at_face_mode_uses_ray_pick() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "face".into();
        state.active_object_id = Some("obj-1".into());
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let mesh_ref = state.meshes.get("mesh-1").expect("mesh");
        let tri = mesh_ref.indices.get(0..3).expect("triangle");
        let centroid = mesh_vertex(mesh_ref, tri[0]).add(mesh_vertex(mesh_ref, tri[1])).add(mesh_vertex(mesh_ref, tri[2])).scale(1.0 / 3.0);
        let screen = kernel_3d_scene::project_point(camera.view_proj(1.0), centroid, inner.w, inner.h).expect("face centroid projects");
        let picked = pick_component_at(&state, screen[0], screen[1], inner).expect("face pick");
        assert_eq!(picked.0, "face");
        assert_eq!(picked.2, "obj-1");
    }

    #[test]
    fn pick_component_at_edge_mode_uses_ray_pick() {
        let mesh = topology_mesh();
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.granularity = "edge".into();
        state.meshes.insert("mesh-1".into(), mesh);
        state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        let camera = state.orbit.to_camera();
        let chunk = state.meshes.get("mesh-1").and_then(|mesh| mesh.edge_positions.get(0..6)).expect("edge");
        let a = Vec3::new(chunk[0], chunk[1], chunk[2]);
        let b = Vec3::new(chunk[3], chunk[4], chunk[5]);
        let mid = a.add(b).scale(0.5);
        let screen = kernel_3d_scene::project_point(camera.view_proj(1.0), mid, inner.w, inner.h).expect("edge midpoint projects");
        let picked = pick_component_at(&state, screen[0], screen[1], inner).expect("edge pick");
        assert_eq!(picked.0, "edge");
        assert_eq!(picked.2, "obj-1");
    }

    #[test]
    fn chunk_key_indices_buckets_negative_coordinates() {
        assert_eq!(chunk_key_indices([-10.0, 5.0, 0.0], 256.0), (-1, 0, 0));
        assert_eq!(chunk_key_indices([300.0, 300.0, 0.0], 256.0), (1, 1, 0));
    }

    #[test]
    fn chunk_distance_visible_uses_hysteresis() {
        let center = chunk_center((0, 0, 0), 256.0);
        let cam_near = Vec3::new(0.0, 0.0, 0.0);
        assert!(chunk_distance_visible(cam_near, center, 256.0, 8000.0, false));
        let cam_far = Vec3::new(8400.0, 128.0, 0.0);
        assert!(!chunk_distance_visible(cam_far, center, 256.0, 8000.0, false));
        assert!(chunk_distance_visible(cam_far, center, 256.0, 8000.0, true));
    }

    #[test]
    fn mesh_pool_release_clears_at_zero_refcount() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.mesh_pool.acquire("mesh-1".into());
        assert!(state.mesh_pool.release("mesh-1".into()));
        assert!(!state.mesh_pool.contains(&"mesh-1".to_string()));
        state.mesh_pool.acquire("mesh-1".into());
        state.mesh_pool.acquire("mesh-1".into());
        assert!(!state.mesh_pool.release("mesh-1".into()));
        assert!(state.mesh_pool.contains(&"mesh-1".to_string()));
        assert!(state.mesh_pool.release("mesh-1".into()));
        assert!(!state.mesh_pool.contains(&"mesh-1".to_string()));
    }

    #[test]
    fn resolve_physical_mesh_id_picks_closest_lod_url() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.mesh_lod_catalog.insert("tower".into(), vec![WorldMeshLodEntry { lod: 1.0, url: "https://example.com/tower-high.glb".into() }, WorldMeshLodEntry { lod: 100.0, url: "https://example.com/tower-low.glb".into() }]);
        let detailed = resolve_physical_mesh_id(&state, "tower", 2.0);
        let coarse = resolve_physical_mesh_id(&state, "tower", 200.0);
        assert_eq!(detailed, "mesh:tower-high");
        assert_eq!(coarse, "mesh:tower-low");
    }

    //#region GlbAssetTests
    /// 📦 URL-backed meshes remain fetchable instead of becoming empty procedural primitives.
    #[test]
    fn rebuild_instance_draws_keeps_url_backed_mesh_pending() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.mesh_url_fallback.insert("mesh:capsule".into(), "/mesh/capsule.glb".into());
        state.pending_glb_urls.insert("/mesh/capsule.glb".into());
        state.parsed_instances = vec![WorldInstanceRecord { id: "capsule-1".into(), mesh_id: Some("mesh:capsule".into()), position: Some([0.0, 0.0, 0.0]), ..Default::default() }];

        rebuild_instance_draws(&mut state, 1.0);

        assert!(!state.meshes.contains_key("mesh:capsule"), "a URL-backed mesh must not be shadowed by an empty mesh_from_kind placeholder");
        let pending = collect_pending_glb_fetches(&HashMap::from([("surface-1".into(), state)]));
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].url, "/mesh/capsule.glb");
    }

    /// 🏗️ A dropped Puzzle object resolves its multi-primitive GLB into the same mesh key used by its persistent draw.
    #[test]
    fn dropped_puzzle_object_glb_becomes_renderable() {
        let url = "/mesh/capsule_J.glb";
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.mesh_url_fallback.insert("mesh:capsule_J".into(), url.into());
        state.pending_glb_urls.insert(url.into());
        state.parsed_instances = vec![WorldInstanceRecord { id: "capsule-1".into(), mesh_id: Some("mesh:capsule_J".into()), position: Some([0.0, 0.0, 0.0]), ..Default::default() }];
        rebuild_instance_draws(&mut state, 100.0);
        apply_glb_bytes(&mut state, url, include_bytes!("../../../../../../asset/metabolism/representation/capsule_J.glb"));
        let draw = state.draws.iter().find(|draw| draw.instances.iter().any(|instance| instance.id == "capsule-1")).expect("persistent draw");
        let mesh = state.meshes.get(&draw.mesh_key).expect("decoded mesh");
        assert_eq!(draw.mesh_key, "mesh:capsule_J");
        assert_eq!(mesh.positions.len() / 3, 1472);
        assert_eq!(mesh.indices.len() / 3, 1750);
        assert!(!state.pending_glb_urls.contains(url));
    }
    //#endregion GlbAssetTests

    #[test]
    fn lod_grid_lines_generate_for_near_camera() {
        let mut lines = Vec::new();
        append_lod_grid_lines(&mut lines, 2.0, 10.0, Vec3::ZERO, [0.5, 0.5, 0.5, 1.0]);
        assert!(!lines.is_empty());
    }

    //#region EnvironmentTests
    #[test]
    fn environment_clear_color_uses_opaque_background() {
        let environment = WorldEnvironmentRecord { background: Some("#112233".into()), ..Default::default() };
        let theme_clear = Rgba::new(0.0, 0.0, 0.0, 1.0);
        let clear = environment_clear_color(&environment, theme_clear);
        assert!((clear.r - (0x11 as f32 / 255.0)).abs() < 1e-3);
        assert!((clear.g - (0x22 as f32 / 255.0)).abs() < 1e-3);
        assert!((clear.b - (0x33 as f32 / 255.0)).abs() < 1e-3);
    }

    #[test]
    fn environment_clear_color_falls_back_when_transparent_or_absent() {
        let theme_clear = Rgba::new(0.1, 0.2, 0.3, 1.0);
        let transparent = WorldEnvironmentRecord { background: Some("transparent".into()), ..Default::default() };
        let absent = WorldEnvironmentRecord::default();
        assert_eq!(environment_clear_color(&transparent, theme_clear).r, theme_clear.r);
        assert_eq!(environment_clear_color(&absent, theme_clear).r, theme_clear.r);
    }

    #[test]
    fn environment_light_dir_uses_sun_direction_only_when_enabled() {
        let disabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(false), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
        assert_eq!(environment_light_dir(&disabled), [0.4, 0.6, 0.8]);

        let enabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(true), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
        let dir = environment_light_dir(&enabled);
        // azimuth=90, elevation=0 -> pure +Y direction (cos(0)*cos(90)=~0, cos(0)*sin(90)=1, sin(0)=0).
        assert!(dir[0].abs() < 1e-3);
        assert!((dir[1] - 1.0).abs() < 1e-3);
        assert!(dir[2].abs() < 1e-3);
    }

    #[test]
    fn rebuild_instance_draws_applies_environment_material_color_as_neutral_default() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.environment = WorldEnvironmentRecord { material: Some(WorldEnvironmentMaterialRecord { color: Some("#ff0000".into()), ..Default::default() }), ..Default::default() };
        state.parsed_instances = vec![WorldInstanceRecord { id: "obj-1".into(), mesh_id: Some("box".into()), position: Some([0.0, 0.0, 0.0]), ..Default::default() }];
        rebuild_instance_draws(&mut state, 1.0);
        let instance = &state.draws.iter().find(|draw| draw.mesh_key == "box").expect("box draw").instances[0];
        assert!((instance.color[0] - 1.0).abs() < 1e-3);
        assert!(instance.color[1].abs() < 1e-3);
    }
    //#endregion EnvironmentTests

    //#region TerrainTests
    #[test]
    fn hypsometric_color_matches_reference_stops() {
        let low = hypsometric_color(0.0);
        assert!((low[0] - 0x4b as f32 / 255.0).abs() < 1e-3);
        let peak = hypsometric_color(1.0);
        assert!((peak[0] - 1.0).abs() < 1e-3);
        assert!((peak[1] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn build_terrain_band_mesh_buckets_by_average_elevation() {
        // Two triangles, 6 verts (no sharing, to keep each triangle's average elevation exact):
        // triangle 0 is flat at elevation ratio 0.0, triangle 1 is flat at elevation ratio 1.0.
        let mesh = TerrainTileMeshPayload {
            positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 3.0, 0.0, 10.0, 2.0, 1.0, 10.0],
            normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
            indices: vec![0, 1, 2, 3, 4, 5],
            uvs: vec![0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0],
        };
        let low_band = build_terrain_band_mesh(&mesh, 0, TERRAIN_COLOR_BANDS);
        assert!(low_band.is_some(), "triangle 0 (all-zero elevation) should fall in band 0");
        let high_band = build_terrain_band_mesh(&mesh, TERRAIN_COLOR_BANDS - 1, TERRAIN_COLOR_BANDS);
        assert!(high_band.is_some(), "triangle 1 (all-one elevation) should fall in the top band");
        let empty_band = build_terrain_band_mesh(&mesh, 5, TERRAIN_COLOR_BANDS);
        assert!(empty_band.is_none(), "no triangle should land in a middle band for this fixture");
    }

    #[test]
    fn terrain_tile_url_substitutes_z_x_y() {
        assert_eq!(terrain_tile_url("/dem/{z}/{x}/{y}.png", 12, 34, 56), "/dem/12/34/56.png");
    }

    #[test]
    fn sync_terrain_state_queues_fetch_for_uncached_tile_and_builds_after_upload() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 9.7382, project_origin_lat: 52.3759, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
        apply_terrain_style_if_changed_state(&mut state);
        let camera = Camera3d { position: Vec3::new(0.0, 0.0, 300.0), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0 };
        let (band_draws, evicted) = sync_terrain_state(&mut state, &camera);
        assert!(band_draws.is_empty(), "no elevation data uploaded yet, nothing to draw");
        assert!(evicted.is_empty(), "nothing was cached yet, nothing to evict");
        assert!(!state.pending_terrain_tile_urls.is_empty(), "an uncached visible tile should be queued for byte-fetch");

        let (_, &(z, x, y)) = state.pending_terrain_tile_urls.iter().next().expect("a pending tile");
        let value = (100.0_f64 + 32768.0).round() as i64;
        let r = ((value >> 8) & 0xff) as u8;
        let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
        let mut image = image::RgbaImage::new(256, 256);
        for pixel in image.pixels_mut() {
            *pixel = image::Rgba([r, g, 0, 255]);
        }
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
        assert!(state.terrain_session.upload_elevation_tile(z, x, y, &bytes));

        let (band_draws_after_upload, _) = sync_terrain_state(&mut state, &camera);
        assert!(!band_draws_after_upload.is_empty(), "an uploaded tile should produce at least one banded draw");
    }

    #[test]
    fn apply_terrain_style_if_changed_state_purges_stale_meshes_on_origin_change() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 0.0, project_origin_lat: 0.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
        assert!(apply_terrain_style_if_changed_state(&mut state).is_empty(), "first application has nothing to purge");
        let mesh_key = terrain_band_mesh_key(&state.surface_id, 10, 1, 2, 0);
        store_mesh(&mut state, mesh_key, Mesh3d::from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]));
        state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 5.0, project_origin_lat: 5.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
        let purged = apply_terrain_style_if_changed_state(&mut state);
        assert_eq!(purged.len(), 1, "an origin change should purge the previously-cached terrain mesh");
        assert!(state.meshes.is_empty());
    }
    //#endregion TerrainTests

    //#region BrushPreviewTests
    #[test]
    fn brush_preview_mesh_id_falls_back_to_box_without_mesh_url() {
        assert_eq!(brush_preview_mesh_id(None), "box");
        assert_eq!(brush_preview_mesh_id(Some("/assets/tower.glb")), mesh_id_from_url("/assets/tower.glb"));
    }
    //#endregion BrushPreviewTests

    //#region ContextMenuTests
    #[test]
    fn resolve_world_context_menu_target_prioritizes_vortex_over_object_over_reference() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        state.local_hover_id = Some("reference:site-plan.png".into());
        assert_eq!(resolve_world_context_menu_target(&state), Some(("reference", "site-plan.png".to_string())));

        state.hovered_component_mode = Some("face".into());
        state.hovered_component_object_id = Some("obj-1".into());
        assert_eq!(resolve_world_context_menu_target(&state), Some(("object", "obj-1".to_string())));

        state.hovered_vortex_id = Some("vortex-1".into());
        assert_eq!(resolve_world_context_menu_target(&state), Some(("vortex", "vortex-1".to_string())));
    }

    #[test]
    fn resolve_world_context_menu_target_is_none_without_any_hover() {
        let state = World3dState::new("surface-1".into(), "controller-1".into());
        assert_eq!(resolve_world_context_menu_target(&state), None);
    }

    #[test]
    fn right_click_dispatches_context_menu_at_for_hovered_vortex() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        state.hovered_vortex_id = Some("vortex-1".into());
        handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
        let action = handle_world3d_pointer_button(&mut state, 200.0, 200.0, false, 2, &PointerModifiers::default()).expect("right click should dispatch");
        assert_eq!(action.action, "contextMenuAt");
        let args = action.args.expect("args");
        assert_eq!(args["kind"], json!("vortex"));
        assert_eq!(args["id"], json!("vortex-1"));
    }

    #[test]
    fn right_drag_does_not_dispatch_context_menu_even_with_a_hovered_vortex() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
        state.bounds = inner;
        state.pick_bounds = inner;
        state.hovered_vortex_id = Some("vortex-1".into());
        handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
        let action = handle_world3d_pointer_button(&mut state, 260.0, 260.0, false, 2, &PointerModifiers::default()).expect("right release should still sync camera");
        assert_eq!(action.action, "setCamera", "a right-drag should fall back to the orbit camera sync, not open a context menu");
    }
    //#endregion ContextMenuTests
}
