//! 🌐 World-3d scene host: mesh loading, orbit camera, picking, and marquee selection.

use crate::interpreter::FrameworkWidgetContext;
use semio_framework_core::{mesh_from_glb, mesh_from_kind, CommandDescriptor, MeshData, UiComponentSceneNode};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use ui_wgpu::{
    draw_text, Camera3d, HitKind, HitTarget, Instance3d, Mat4, Mesh3d, OrbitController, Rect, Rgba,
    SceneDraw3d, ScenePass3d, Vec3, point_in_polygon, project_point, ray_pick_instance,
    rect_contains, screen_select_instances,
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
//#endregion SceneRecords

//#region World3dState
pub struct World3dState {
    pub surface_id: String,
    pub controller_id: String,
    pub bounds: Rect,
    pub orbit: OrbitController,
    pub meshes: HashMap<String, Mesh3d>,
    pub draws: Vec<SceneDraw3d>,
    pub selection_method: String,
    pub local_hover_id: Option<String>,
    pub pending_glb_urls: HashSet<String>,
    pub marquee_points: Vec<[f32; 2]>,
    pub marquee_active: bool,
}

impl World3dState {
    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            surface_id,
            controller_id,
            bounds: Rect::default(),
            orbit: OrbitController::default(),
            meshes: HashMap::new(),
            draws: Vec::new(),
            selection_method: "rectangle".into(),
            local_hover_id: None,
            pending_glb_urls: HashSet::new(),
            marquee_points: Vec::new(),
            marquee_active: false,
        }
    }
}
//#endregion World3dState

pub fn sync_world3d_state(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    state.bounds = bounds;
    let Some(world) = &scene.world_3d else {
        state.draws.clear();
        return;
    };
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
            state.meshes.insert(mesh.id.clone(), Mesh3d::from_buffers(
                data.positions,
                data.normals,
                data.indices,
            ));
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
            state.meshes.insert(mesh_id.clone(), Mesh3d::from_buffers(
                mesh_from_kind(&mesh_id).positions,
                mesh_from_kind(&mesh_id).normals,
                mesh_from_kind(&mesh_id).indices,
            ));
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
        .map(|(mesh_key, instances)| SceneDraw3d { mesh_key, instances })
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
    for (mesh_key, mesh) in &state.meshes {
        gpu.ensure_mesh(mesh_key, &mesh.positions, &mesh.normals, &mesh.indices);
    }
    ctx.draw.push_scene_pass(ScenePass3d {
        viewport: [inner.x, inner.y, inner.w, inner.h],
        view_proj: view_proj.to_cols_array(),
        light_dir: [0.4, 0.6, 0.8],
        draws: state.draws.clone(),
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
    });
}

pub fn world3d_hit_target(scene: &UiComponentSceneNode, bounds: Rect) -> HitTarget<CommandDescriptor> {
    HitTarget {
        rect: bounds.inset(8.0),
        event: None,
        control_id: Some(scene.surface_id.clone()),
        kind: HitKind::World3d,
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
        return pick_select_command(state, x, y, inner, shift, ctrl);
    }
    None
}

pub fn handle_world3d_pointer_drag(
    state: &mut World3dState,
    dx: f32,
    dy: f32,
    button: i16,
    shift: bool,
) {
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
    state.meshes.insert(mesh_id, Mesh3d::from_buffers(
        mesh.positions,
        mesh.normals,
        mesh.indices,
    ));
}

fn pick_hover_command(state: &mut World3dState, x: f32, y: f32, inner: Rect) -> Option<CommandDescriptor> {
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

#[cfg(target_arch = "wasm32")]
pub async fn fetch_pending_glb_meshes(states: &mut HashMap<String, World3dState>) {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    for state in states.values_mut() {
        let urls: Vec<String> = state.pending_glb_urls.iter().cloned().collect();
        for url in urls {
            let mesh_id = format!("url:{}", url);
            if state.meshes.contains_key(&mesh_id) {
                state.pending_glb_urls.remove(&url);
                continue;
            }
            let Ok(window) = web_sys::window().ok_or("no window") else {
                continue;
            };
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            let Ok(request) = Request::new_with_str_and_init(&url, &opts) else {
                continue;
            };
            let Ok(response_value) = JsFuture::from(window.fetch_with_request(&request)).await else {
                continue;
            };
            let Ok(response) = response_value.dyn_into::<Response>() else {
                continue;
            };
            let Ok(buffer) = JsFuture::from(response.array_buffer().unwrap()).await else {
                continue;
            };
            let bytes = js_sys::Uint8Array::new(&buffer).to_vec();
            if let Ok(mesh) = mesh_from_glb(&bytes) {
                ingest_glb_mesh(state, &url, mesh, mesh_id);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_pending_glb_meshes(_states: &mut HashMap<String, World3dState>) {}
