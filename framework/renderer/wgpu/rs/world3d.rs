//! 🌐 World-3d scene host: mesh loading, orbit camera, picking, and marquee selection.

use crate::interpreter::FrameworkWidgetContext;
use semio_framework_core::{mesh_from_glb, mesh_from_kind, CommandDescriptor, MeshData, UiComponentSceneNode};
use serde::Deserialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use ui_wgpu::{
    draw_text, mesh_content_version, Camera3d, HitKind, HitTarget, Instance3d, LineDraw3d, LineVertex3d,
    Mesh3d, OrbitController, Rect, Rgba, SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d,
    Vec3, aabb_intersects_frustum, frustum_planes, ray_aabb_slab, ray_pick_instance,
    screen_select_instances, transform_aabb,
};

//#region SceneRecords
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
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct WorldSelectionRecord {
    method: Option<String>,
    mode: Option<String>,
    ids: Option<Vec<String>>,
    hovered_id: Option<String>,
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
    pending_image_urls: HashSet<String>,
    reference_aspect: HashMap<String, f32>,
    reference_pixels: HashMap<String, (u32, u32, Vec<u8>)>,
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
            pending_image_urls: HashSet::new(),
            reference_aspect: HashMap::new(),
            reference_pixels: HashMap::new(),
        }
    }
}
//#endregion World3dState

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
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
            "[DEBUG] world3d sync skipped (scene unchanged)",
        ));
        return;
    }
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
    let meshes: Vec<WorldMeshRecord> =
        serde_json::from_str(&world.meshes_json).unwrap_or_default();
    for mesh in meshes {
        if let Some(data) = mesh.data {
            store_mesh(
                state,
                mesh.id.clone(),
                Mesh3d::from_buffers(data.positions, data.normals, data.indices),
            );
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
        let color = parse_color(instance.color.as_deref().unwrap_or("#94a3b8"));
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

pub fn render_world_3d(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    state: &mut World3dState,
    gpu: &mut ui_wgpu::GpuContext,
) {
    let theme = ctx.theme;
    sync_world3d_state(state, scene, bounds);
    let inner = bounds.inset(8.0);
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], Rgba::new(0.04, 0.05, 0.08, 1.0));
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
    if culled_count > 0 {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
            "[DEBUG] world3d culled {culled_count} instances",
        )));
    }
    let mut line_vertices = Vec::new();
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
        rect: bounds.inset(8.0),
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
    if !state.bounds.inset(8.0).contains(x, y) {
        return None;
    }
    let inner = state.bounds.inset(8.0);
    if down && button == 0 {
        if state.marquee_active {
            state.marquee_points.push([x, y]);
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

pub fn handle_world3d_pointer_button(
    state: &mut World3dState,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    shift: bool,
    ctrl: bool,
) -> Option<CommandDescriptor> {
    let inner = state.bounds.inset(8.0);
    if !inner.contains(x, y) {
        return None;
    }
    if down {
        if button == 0 {
            if state.active_tool == "brush" {
                if let Some(full_id) = pick_vortex_at(state, x, y, inner) {
                    return Some(CommandDescriptor {
                        controller_id: state.controller_id.clone(),
                        command: "worldVortexSelect".into(),
                        args: Some(json!({ "surfaceId": state.surface_id, "fullId": full_id })),
                    });
                }
            } else if state.active_tool == "select" {
                if let Some(object_id) = pick_instance_at(state, x, y, inner) {
                    state.drag_object_id = Some(object_id.clone());
                    state.drag_last_position = object_world_position(state, &object_id);
                    state.drag_object_z = state.drag_last_position.map(|p| p[2]).unwrap_or(0.0);
                    return None;
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
        return marquee_select_command(state, inner, shift, ctrl);
    }
    if button == 0 {
        if let Some(object_id) = state.drag_object_id.take() {
            if let Some(position) = state.drag_last_position {
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
    None
}

pub fn handle_world3d_pointer_drag(
    state: &mut World3dState,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    button: i16,
    shift: bool,
) {
    let inner = state.bounds.inset(8.0);
    if button == 0 && state.drag_object_id.is_some() && inner.contains(x, y) {
        if let Some(position) = ground_plane_pick(state, x, y, inner, state.drag_object_z) {
            state.drag_last_position = Some(position);
            if let Some(object_id) = state.drag_object_id.clone() {
                update_dragged_instance_position(state, &object_id, position);
            }
        }
        return;
    }
    if button == 2 {
        if shift {
            state.orbit.pan(dx, dy);
        } else {
            state.orbit.orbit(dx, dy);
        }
    } else if button == 1 || (button == 2 && shift) {
        state.orbit.pan(dx, dy);
    }
}

pub fn handle_world3d_wheel(state: &mut World3dState, delta: f32) {
    state.orbit.zoom(delta);
}

pub fn ingest_glb_mesh(state: &mut World3dState, url: &str, mesh: MeshData, mesh_id: String) {
    state.pending_glb_urls.remove(url);
    store_mesh(
        state,
        mesh_id,
        Mesh3d::from_buffers(mesh.positions, mesh.normals, mesh.indices),
    );
}

fn pick_hover_command(state: &mut World3dState, x: f32, y: f32, inner: Rect) -> Option<CommandDescriptor> {
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
    let hit = pick_instance_at(state, x, y, inner);
    let merge = if shift { "add" } else if ctrl { "toggle" } else { "replace" };
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
    let ids = screen_select_instances(
        &state.meshes,
        &state.draws,
        view_proj,
        inner.w,
        inner.h,
        &polygon,
        rectangle,
    );
    state.marquee_points.clear();
    let merge = if shift { "add" } else if ctrl { "toggle" } else { "replace" };
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

fn pick_instance_at(state: &World3dState, x: f32, y: f32, inner: Rect) -> Option<String> {
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = camera.ray_from_screen(aspect, local_x, local_y, inner.w, inner.h);
    let mut best: Option<(f32, String)> = None;
    for draw in &state.draws {
        let Some(mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(distance) = ray_pick_instance(origin, dir, mesh, instance) {
                if best.as_ref().map_or(true, |(best_distance, _)| distance < *best_distance) {
                    best = Some((distance, instance.id.clone()));
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

fn vec3_from_f64(values: [f64; 3]) -> Vec3 {
    Vec3::new(values[0] as f32, values[1] as f32, values[2] as f32)
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
pub async fn fetch_pending_glb_meshes(_states: &mut HashMap<String, World3dState>) {}

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
