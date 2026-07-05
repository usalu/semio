//! 🎬 Native component scene hosts for canvas-2d, tables, graphs, and 3D views.

use crate::engine_canvas;
use crate::interpreter::FrameworkWidgetContext;
use crate::shell::{push_context_menu_item, push_find_item, ContextMenuItem, ShellFindItem};
use infinite_world::{render_world_3d, World3dState};
use base64::Engine;
use semio_framework_core::{CommandDescriptor, UiComponentSceneNode};
use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ui_wgpu::input::{DragAxis, KeyAction};
use ui_wgpu::{draw_text, HitKind, HitTarget, Rect, Rgba};

//#region SceneRuntime
#[derive(Clone, Copy, Debug, Default)]
struct Viewport {
    x: f32,
    y: f32,
    zoom: f32,
}

impl Viewport {
    fn from_json(raw: &str) -> Self {
        serde_json::from_str::<Value>(raw)
            .ok()
            .map(|value| Self {
                x: value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                y: value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                zoom: value
                    .get("zoom")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0) as f32,
            })
            .unwrap_or_default()
    }

    fn screen_to_world(&self, sx: f32, sy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        (
            (sx - cx) / self.zoom + self.x,
            (sy - cy) / self.zoom + self.y,
        )
    }

    fn world_to_screen(&self, wx: f32, wy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        (
            cx + (wx - self.x) * self.zoom,
            cy + (wy - self.y) * self.zoom,
        )
    }
}

#[derive(Clone, Debug)]
enum SceneDragMode {
    PanViewport,
    MoveNode { node_id: String, grab_x: f32, grab_y: f32 },
    ConnectPort {
        source_node_id: String,
        source_port_id: String,
        is_output: bool,
    },
    Marquee,
}

#[derive(Clone, Debug)]
struct SceneDrag {
    mode: SceneDragMode,
    button: i16,
}

#[derive(Clone, Debug, Default)]
struct SceneSurfaceState {
    scroll_offsets: HashMap<String, f32>,
    viewport: Viewport,
    drag: Option<SceneDrag>,
    pointer_was_down: bool,
    last_click_ms: f64,
    last_click_target: Option<String>,
    editor_cursor: usize,
    node_positions: HashMap<String, (f32, f32)>,
    selected_ids: HashSet<String>,
    hover_row_id: Option<String>,
    raster_digest: Option<u64>,
    pending_raster: Option<PendingRasterUpload>,
    pending_raster_uploads: Vec<PendingRasterUpload>,
    canvas_image_digests: HashMap<String, u64>,
    paint_stroke_active: bool,
    vfs_expanded_ids: HashSet<String>,
    vfs_selection_anchor: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PendingRasterUpload {
    pub key: String,
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

thread_local! {
    static SCENE_STATE: RefCell<HashMap<String, SceneSurfaceState>> = RefCell::new(HashMap::new());
    static GRAPH_NODE_CTX: RefCell<HashMap<String, Option<String>>> = RefCell::new(HashMap::new());
}

/** @emoji 🕸️ Clears per-frame graph node metadata used by context menus. */
pub fn clear_graph_node_context() {
    GRAPH_NODE_CTX.with(|cell| cell.borrow_mut().clear());
}

/** @emoji 🕸️ Registers a graph node instance mapping for context-menu dispatch. */
pub fn register_graph_node(node_id: &str, instance_id: Option<&str>) {
    GRAPH_NODE_CTX.with(|cell| {
        cell.borrow_mut().insert(
            node_id.to_string(),
            instance_id.map(str::to_string),
        );
    });
}

/** @emoji 🕸️ Resolves a graph node instance id for context-menu commands. */
pub fn graph_node_instance(node_id: &str) -> Option<String> {
    GRAPH_NODE_CTX.with(|cell| cell.borrow().get(node_id).cloned().flatten())
}

/** @emoji 📁 Toggles VFS row expand/collapse in scene-local state. */
pub fn toggle_vfs_row_expanded(surface_id: &str, row_id: &str) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.contains(row_id) {
            state.vfs_expanded_ids.remove(row_id);
        } else {
            state.vfs_expanded_ids.insert(row_id.to_string());
        }
    });
}

/** @emoji 📁 Seeds default expanded VFS roots on first render. */
pub fn seed_vfs_expanded(surface_id: &str, row_ids: &[String]) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.is_empty() {
            for id in row_ids {
                state.vfs_expanded_ids.insert(id.clone());
            }
        }
    });
}

/** @emoji 📁 Computes VFS multi-select ids for shift/meta click semantics. */
pub fn vfs_selection_for_click(
    surface_id: &str,
    row_id: &str,
    ordered_ids: &[String],
    shift: bool,
    additive: bool,
) -> Vec<String> {
    let mut state = scene_state(surface_id);
    if shift {
        let anchor = state.vfs_selection_anchor.clone().unwrap_or_else(|| row_id.to_string());
        let a = ordered_ids.iter().position(|id| id == &anchor);
        let b = ordered_ids.iter().position(|id| id == row_id);
        if let (Some(a), Some(b)) = (a, b) {
            let (start, end) = if a <= b { (a, b) } else { (b, a) };
            let ids: Vec<String> = ordered_ids[start..=end].to_vec();
            state.vfs_selection_anchor = Some(anchor);
            mutate_scene_state(surface_id, |state| {
                state.vfs_selection_anchor = Some(row_id.to_string());
            });
            return ids;
        }
    }
    mutate_scene_state(surface_id, |state| {
        state.vfs_selection_anchor = Some(row_id.to_string());
    });
    if additive {
        let mut ids: Vec<String> = scene_state(surface_id)
            .selected_ids
            .into_iter()
            .collect();
        if ids.iter().any(|id| id == row_id) {
            ids.retain(|id| id != row_id);
        } else {
            ids.push(row_id.to_string());
        }
        return ids;
    }
    vec![row_id.to_string()]
}

fn scene_state(surface_id: &str) -> SceneSurfaceState {
    SCENE_STATE.with(|cell| {
        cell.borrow_mut()
            .entry(surface_id.to_string())
            .or_default()
            .clone()
    })
}

fn mutate_scene_state(surface_id: &str, f: impl FnOnce(&mut SceneSurfaceState)) {
    SCENE_STATE.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.entry(surface_id.to_string()).or_default();
        f(entry);
    });
}

fn scene_cmd(scene: &UiComponentSceneNode, command: &str, args: Value) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: scene.controller_id.clone(),
        command: command.into(),
        args: Some(args),
    }
}

fn surface_args(scene: &UiComponentSceneNode) -> Value {
    json!({ "surfaceId": scene.surface_id })
}

fn scroll_key(surface_id: &str, suffix: &str) -> String {
    format!("{surface_id}.{suffix}")
}

fn scroll_offset(surface_id: &str, suffix: &str) -> f32 {
    let key = scroll_key(surface_id, suffix);
    SCENE_STATE.with(|cell| {
        cell.borrow()
            .get(surface_id)
            .and_then(|state| state.scroll_offsets.get(&key).copied())
            .unwrap_or(0.0)
    })
    .max(0.0)
}

fn set_scroll_offset(surface_id: &str, suffix: &str, value: f32) {
    let key = scroll_key(surface_id, suffix);
    mutate_scene_state(surface_id, |state| {
        state.scroll_offsets.insert(key, value.max(0.0));
    });
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|perf| perf.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    0.0
}

fn digest_pixels(pixels: &[u8]) -> u64 {
    pixels.iter().fold(0u64, |acc, byte| acc.wrapping_mul(31).wrapping_add(*byte as u64))
}

pub fn drain_pending_raster_uploads() -> Vec<PendingRasterUpload> {
    let mut uploads = Vec::new();
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            if let Some(pending) = state.pending_raster.take() {
                uploads.push(pending);
            }
            uploads.append(&mut state.pending_raster_uploads);
        }
    });
    uploads
}
//#endregion SceneRuntime

fn canvas_world_pointer_json(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    extra: Value,
) -> Value {
    let state = scene_state(&scene.surface_id);
    let (wx, wy) = state.viewport.screen_to_world(x, y, inner);
    let mut payload = json!({
        "surfaceId": scene.surface_id,
        "x": wx,
        "y": wy,
    });
    if let (Some(base), Some(patch)) = (payload.as_object_mut(), extra.as_object()) {
        for (key, value) in patch {
            base.insert(key.clone(), value.clone());
        }
    }
    payload
}

//#region SceneInput
pub fn handle_scene_wheel(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    x: f32,
    y: f32,
    delta: f32,
    ctrl: bool,
) -> Vec<CommandDescriptor> {
    if !bounds.contains(x, y) {
        return Vec::new();
    }
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    match scene.component_kind.as_str() {
        "table" => {
            let current = scroll_offset(&scene.surface_id, "body");
            set_scroll_offset(&scene.surface_id, "body", current + delta * 0.5);
            Vec::new()
        }
        "text-editor" => engine_canvas::text_editor_wheel(scene, delta),
        "virtualFileSystem" => {
            let current = scroll_offset(&scene.surface_id, "vfs");
            set_scroll_offset(&scene.surface_id, "vfs", current + delta * 0.5);
            Vec::new()
        }
        "canvas-2d" => {
            mutate_scene_state(&scene.surface_id, |state| {
                let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.125, 8.0);
            });
            Vec::new()
        }
        "node-graph" => engine_canvas::node_graph_wheel(scene, inner, x, y, delta, ctrl),
        _ => Vec::new(),
    }
}

pub fn handle_scene_pointer_move(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    x: f32,
    y: f32,
    down: bool,
    _button: i16,
    drag_dx: f32,
    drag_dy: f32,
) -> Vec<CommandDescriptor> {
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    let mut commands = Vec::new();
    let state = scene_state(&scene.surface_id);
    if down {
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::PanViewport => {
                    let vp = state.viewport;
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.viewport.x -= drag_dx / vp.zoom.max(0.01);
                        state.viewport.y -= drag_dy / vp.zoom.max(0.01);
                    });
                }
                SceneDragMode::MoveNode { node_id, grab_x, grab_y } => {
                    let vp = state.viewport;
                    let (wx, wy) = vp.screen_to_world(x, y, inner);
                    let nx = wx - grab_x;
                    let ny = wy - grab_y;
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.node_positions.insert(node_id.clone(), (nx, ny));
                    });
                }
                SceneDragMode::ConnectPort { .. } => {}
                SceneDragMode::Marquee => {}
            }
        }
    }
    match scene.component_kind.as_str() {
        "canvas-2d" if down => {
            commands.push(scene_cmd(
                scene,
                "canvasPointerMove",
                canvas_world_pointer_json(scene, inner, x, y, json!({})),
            ));
        }
        "node-graph" if down => {
            commands.extend(engine_canvas::node_graph_pointer_move(
                scene, inner, x, y, false, false, false,
            ));
        }
        "text-editor" if down => {
            commands.extend(engine_canvas::text_editor_pointer_move(scene, inner, x, y));
        }
        "node-graph" | "text-editor" if !down => {
            commands.extend(match scene.component_kind.as_str() {
                "node-graph" => engine_canvas::node_graph_pointer_move(scene, inner, x, y, false, false, false),
                _ => engine_canvas::text_editor_pointer_move(scene, inner, x, y),
            });
        }
        _ => {}
    }
    commands
}

pub fn handle_scene_pointer_button(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    shift: bool,
) -> Vec<CommandDescriptor> {
    let inner = bounds;
    if !inner.contains(x, y) {
        if !down {
            mutate_scene_state(&scene.surface_id, |state| {
                state.drag = None;
                state.pointer_was_down = false;
            });
        }
        return Vec::new();
    }
    let mut commands = Vec::new();
    if down {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = true;
        });
        match scene.component_kind.as_str() {
            "canvas-2d" => {
                if button == 0 {
                    mutate_scene_state(&scene.surface_id, |state| {
                        if !state.paint_stroke_active {
                            state.paint_stroke_active = true;
                        }
                    });
                    commands.push(scene_cmd(scene, "paintStrokeBegin", json!({ "surfaceId": scene.surface_id })));
                }
                commands.push(scene_cmd(
                    scene,
                    "canvasPointerDown",
                    canvas_world_pointer_json(
                        scene,
                        inner,
                        x,
                        y,
                        json!({ "button": button, "extend": shift }),
                    ),
                ));
                if button == 1 || button == 2 {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.drag = Some(SceneDrag {
                            mode: SceneDragMode::PanViewport,
                            button,
                        });
                    });
                }
            }
            "node-graph" => {
                commands.extend(engine_canvas::node_graph_pointer_down(
                    scene, inner, x, y, button, shift, false, false,
                ));
            }
            "text-editor" => {
                commands.extend(engine_canvas::text_editor_pointer_down(scene, inner, x, y, button));
            }
            _ => {}
        }
    } else {
        match scene.component_kind.as_str() {
            "canvas-2d" => {
                commands.push(scene_cmd(
                    scene,
                    "canvasPointerUp",
                    canvas_world_pointer_json(scene, inner, x, y, json!({})),
                ));
                mutate_scene_state(&scene.surface_id, |state| {
                    if state.paint_stroke_active {
                        state.paint_stroke_active = false;
                    }
                });
                commands.push(scene_cmd(scene, "paintStrokeEnd", json!({ "surfaceId": scene.surface_id })));
            }
            "node-graph" => {
                commands.extend(engine_canvas::node_graph_pointer_up(
                    scene, inner, x, y, shift, false, false,
                ));
            }
            "text-editor" => {
                commands.extend(engine_canvas::text_editor_pointer_up(scene, inner, x, y));
            }
            _ => {}
        }
        if let Some(target) = hit_double_click_target(scene, inner, x, y) {
            let now = now_ms();
            let prior = scene_state(&scene.surface_id);
            if prior.last_click_target.as_deref() == Some(target.as_str())
                && now - prior.last_click_ms < 400.0
            {
                if let Some(command) = double_click_command(scene, &target, inner, x, y) {
                    commands.push(command);
                }
            }
            mutate_scene_state(&scene.surface_id, |state| {
                state.last_click_target = Some(target);
                state.last_click_ms = now;
            });
        }
        mutate_scene_state(&scene.surface_id, |state| {
            if let Some(SceneDrag {
                mode: SceneDragMode::MoveNode { node_id, .. },
                ..
            }) = state.drag.as_ref()
            {
                if let Some((nx, ny)) = state.node_positions.get(node_id).copied() {
                    commands.push(scene_cmd(
                        scene,
                        "moveMediaNode",
                        json!({ "surfaceId": scene.surface_id, "nodeId": node_id, "x": nx, "y": ny }),
                    ));
                }
            }
            state.drag = None;
            state.pointer_was_down = false;
        });
    }
    commands
}

fn hit_double_click_target(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
) -> Option<String> {
    match scene.component_kind.as_str() {
        "virtualFileSystem" => {
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let body_y = inner.y + 24.0;
            let index = ((y - body_y + scroll) / row_h).floor() as i32;
            if index < 0 {
                return None;
            }
            Some(format!("{}.vfs.index.{index}", scene.surface_id))
        }
        "node-graph" => hit_graph_node(scene, inner, x, y)
            .map(|id| format!("{}.node.{}", scene.surface_id, id)),
        _ => None,
    }
}

fn double_click_command(
    scene: &UiComponentSceneNode,
    target: &str,
    inner: Rect,
    x: f32,
    y: f32,
) -> Option<CommandDescriptor> {
    match scene.component_kind.as_str() {
        "virtualFileSystem" => {
            let vfs = scene.virtual_file_system.as_ref()?;
            let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).ok()?;
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let index = ((y - inner.y - 24.0 + scroll) / row_h).floor() as usize;
            rows.get(index)
                .and_then(|row| vfs_double_click_command(scene, row))
        }
        "node-graph" => {
            let node_id = target.strip_prefix(&format!("{}.node.", scene.surface_id))?;
            let record = find_graph_node(scene, node_id)?;
            let instance_id = record.instance_id.as_deref()?;
            Some(scene_cmd(
                scene,
                "openInstance",
                json!({ "surfaceId": scene.surface_id, "instanceId": instance_id }),
            ))
        }
        _ => None,
    }
}
//#endregion SceneInput

//#region RenderEntry
pub fn render_component_scene(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    world3d_states: &mut HashMap<String, World3dState>,
) {
    let theme = ctx.theme;
    ctx.draw.set_screen_height(bounds.y + bounds.h);
    ctx.draw.push_rounded(
        [bounds.x, bounds.y, bounds.w, bounds.h],
        theme.panel,
        theme.border_radius,
    );
    match scene.component_kind.as_str() {
        "raster" => render_raster(scene, bounds, ctx, gpu),
        "table" => render_table(scene, bounds, ctx),
        "canvas-2d" => render_canvas_2d(scene, bounds, ctx),
        "node-graph" => render_node_graph(scene, bounds, ctx, gpu),
        "virtualFileSystem" => render_vfs(scene, bounds, ctx),
        "text-editor" => render_text_editor(scene, bounds, ctx, gpu),
        "world-3d" => {
            let state = world3d_states
                .entry(scene.surface_id.clone())
                .or_insert_with(|| World3dState::new(scene.surface_id.clone(), scene.controller_id.clone()));
            render_world_3d(scene, bounds, ctx, state, gpu);
        }
        _ => render_placeholder(&scene.component_kind, bounds, ctx),
    }
    apply_scene_wheel(scene, bounds, ctx);
}
//#endregion RenderEntry

fn apply_scene_wheel(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    if ctx.input.wheel_delta.abs() < 0.01 || !bounds.contains(ctx.input.pointer_x, ctx.input.pointer_y) {
        return;
    }
    let _ = handle_scene_wheel(
        scene,
        bounds,
        ctx.input.pointer_x,
        ctx.input.pointer_y,
        ctx.input.wheel_delta,
        ctx.input.modifiers.ctrl,
    );
}

fn render_placeholder(kind: &str, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    draw_text(
        ctx,
        &format!("{kind} host"),
        bounds.x + 12.0,
        bounds.y + 24.0,
        theme.font_size_body,
        theme.text_muted,
    );
}

//#region Raster
fn render_raster(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
) {
    let theme = ctx.theme;
    let Some(raster) = &scene.raster else {
        return render_placeholder("raster", bounds, ctx);
    };
    let inner = bounds;
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&raster.pixels_base64) else {
        draw_text(
            ctx,
            &format!("{}×{} raster", raster.width, raster.height),
            inner.x + 8.0,
            inner.y + 20.0,
            theme.font_size_small,
            theme.text_muted,
        );
        return;
    };
    let expected = (raster.width as usize).saturating_mul(raster.height as usize).saturating_mul(4);
    if bytes.len() < expected {
        draw_text(ctx, "Invalid raster payload", inner.x + 8.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
        return;
    }
    let digest = digest_pixels(&bytes[..expected]);
    let key = format!("raster:{}", scene.surface_id);
    mutate_scene_state(&scene.surface_id, |state| {
        if state.raster_digest != Some(digest) {
            state.raster_digest = Some(digest);
            state.pending_raster = Some(PendingRasterUpload {
                key: key.clone(),
                pixels: bytes[..expected].to_vec(),
                width: raster.width,
                height: raster.height,
            });
        }
    });
    let _ = gpu;
    let aspect = raster.width as f32 / raster.height.max(1) as f32;
    let (quad_w, quad_h) = if inner.w / inner.h > aspect {
        let h = inner.h;
        (h * aspect, h)
    } else {
        let w = inner.w;
        (w, w / aspect)
    };
    let qx = inner.x + (inner.w - quad_w) * 0.5;
    let qy = inner.y + (inner.h - quad_h) * 0.5;
    ctx.draw
        .push_raster_quad(&key, [qx, qy, quad_w, quad_h], [0.0, 0.0, 1.0, 1.0], 1.0);
    let quad = Rect::new(qx, qy, quad_w, quad_h);
    ctx.input.register_hit(HitTarget {
        rect: quad,
        event: Some(scene_cmd(scene, "rasterClick", surface_args(scene))),
        control_id: Some(scene.surface_id.clone()),
        kind: HitKind::Generic,
        drag_axis: None,
    drag_data: None,
    });
}
//#endregion Raster

//#region Table
#[derive(Deserialize)]
struct TableColumn {
    id: String,
    label: String,
}

fn render_table(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(table) = &scene.table else {
        return render_placeholder("table", bounds, ctx);
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let inner = bounds;
    let header_h = theme.control_height * 1.33;
    let row_h = theme.control_height;
    let pad = theme.padding_standard;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    let col_w = if columns.is_empty() {
        inner.w
    } else {
        inner.w / columns.len() as f32
    };
    for (index, column) in columns.iter().enumerate() {
        let x = inner.x + index as f32 * col_w;
        draw_text(ctx, &column.label, x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    ctx.draw.push_line(
        inner.x,
        inner.y + header_h,
        inner.x + inner.w,
        inner.y + header_h,
        theme.separator,
        1.0,
    );
    let body = Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h);
    let scroll = scroll_offset(&scene.surface_id, "body");
    ctx.input.register_hit(HitTarget {
        rect: body,
        event: None,
        control_id: Some(scroll_key(&scene.surface_id, "body")),
        kind: HitKind::ScrollRegion,
        drag_axis: None,
        drag_data: None,
    });
    ctx.draw.push_scissor(body);
    let hovered_row = ctx.input.hovered_id.clone();
    if rows.is_empty() {
        let message = "No rows";
        draw_text(
            ctx,
            message,
            body.x + body.w * 0.5 - 40.0,
            body.y + body.h * 0.5,
            theme.font_size_small,
            theme.text_muted,
        );
    }
    for (row_index, row) in rows.iter().enumerate() {
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = row
            .get("id")
            .or_else(|| row.get("programId"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let control_id = format!("{}.row.{}", scene.surface_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        if hovered {
            ctx.draw
                .push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(
            row_rect.x,
            row_rect.y + row_rect.h - theme.stroke_hairline,
            row_rect.x + row_rect.w,
            row_rect.y + row_rect.h - theme.stroke_hairline,
            theme.separator,
            1.0,
        );
        for (col_index, column) in columns.iter().enumerate() {
            let x = body.x + col_index as f32 * col_w;
            let value = row
                .get(&column.id)
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .unwrap_or_else(|| "—".into());
            draw_text(
                ctx,
                &value,
                x + pad,
                y + row_h * 0.65,
                theme.font_size_small,
                if hovered { theme.active_foreground } else { theme.text },
            );
        }
        ctx.input.register_hit(HitTarget {
            rect: row_rect,
            event: Some(scene_cmd(
                scene,
                "selectRow",
                json!({ "surfaceId": scene.surface_id, "row": row }),
            )),
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data: None,
        });
    }
    ctx.draw.pop_scissor();
}
//#endregion Table

//#region Canvas2d
#[derive(Deserialize)]
struct CanvasLayer {
    #[serde(default)]
    kind: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    width: f64,
    #[serde(default)]
    height: f64,
    #[serde(default)]
    x0: Option<f64>,
    #[serde(default)]
    y0: Option<f64>,
    #[serde(default)]
    x1: Option<f64>,
    #[serde(default)]
    y1: Option<f64>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    target: Option<String>,
    #[serde(default, rename = "dataUrl")]
    data_url: Option<String>,
    #[serde(default)]
    points: Option<Vec<[f64; 2]>>,
    #[serde(default)]
    seams: Option<Vec<u8>>,
}

fn decode_canvas_image(data_url: &str) -> Option<(Vec<u8>, u32, u32)> {
    let payload = data_url
        .strip_prefix("data:image/png;base64,")
        .or_else(|| data_url.strip_prefix("data:image/jpeg;base64,"))
        .unwrap_or(data_url);
    let bytes = base64::engine::general_purpose::STANDARD.decode(payload).ok()?;
    let image = image::load_from_memory(&bytes).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some((rgba.into_raw(), width, height))
}

fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    let (pixels, width, height) = decode_canvas_image(data_url)?;
    let expected = (width as usize).saturating_mul(height as usize).saturating_mul(4);
    if pixels.len() < expected {
        return None;
    }
    let digest = digest_pixels(&pixels[..expected]);
    let key = format!("canvas-image:{surface_id}:{layer_id}");
    mutate_scene_state(surface_id, |state| {
        let prior = state.canvas_image_digests.get(&key).copied();
        if prior != Some(digest) {
            state.canvas_image_digests.insert(key.clone(), digest);
            state.pending_raster_uploads.push(PendingRasterUpload {
                key: key.clone(),
                pixels: pixels[..expected].to_vec(),
                width,
                height,
            });
        }
    });
    Some(key)
}

fn draw_checkerboard(
    draw: &mut ui_wgpu::DrawList,
    viewport: &Viewport,
    inner: Rect,
    theme: &ui_wgpu::Theme,
    extent: f32,
) {
    let cell = 16.0;
    let half = extent * 0.5;
    let light = Rgba::new(0.85, 0.85, 0.85, 1.0);
    let dark = Rgba::new(0.72, 0.72, 0.72, 1.0);
    let mut row = 0;
    let mut wy = -half;
    while wy < half {
        let mut col = 0;
        let mut wx = -half;
        while wx < half {
            let color = if (row + col) % 2 == 0 { light } else { dark };
            let (sx, sy) = viewport.world_to_screen(wx, wy, inner);
            let (sx1, sy1) = viewport.world_to_screen(wx + cell, wy + cell, inner);
            let w = (sx1 - sx).abs().max(1.0);
            let h = (sy1 - sy).abs().max(1.0);
            draw.push_solid([sx.min(sx1), sy.min(sy1), w, h], color);
            wx += cell;
            col += 1;
        }
        wy += cell;
        row += 1;
    }
    let _ = theme;
}

fn draw_dashed_line(
    draw: &mut ui_wgpu::DrawList,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: Rgba,
    width: f32,
) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let dash = 4.0f32;
    let gap = 4.0f32;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            let sx0 = x0 + ux * traveled;
            let sy0 = y0 + uy * traveled;
            let sx1 = x0 + ux * next;
            let sy1 = y0 + uy * next;
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
        }
        traveled = next;
        drawing = !drawing;
    }
}

fn render_canvas_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(canvas) = &scene.canvas_2d else {
        return render_placeholder("canvas-2d", bounds, ctx);
    };
    let layers: Vec<CanvasLayer> = serde_json::from_str(&canvas.layers_json).unwrap_or_default();
    let inner = bounds;
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let mut viewport = Viewport {
        x: canvas.camera_x as f32,
        y: canvas.camera_y as f32,
        zoom: canvas.zoom as f32,
    };
    let local = scene_state(&scene.surface_id);
    if local.viewport.zoom > 0.0 && scene.component_kind == "canvas-2d" {
        viewport = local.viewport;
    }
    if local.viewport.zoom > 0.0 && scene.component_kind == "canvas-2d" {
        viewport = local.viewport;
    }
    let has_polyline = layers.iter().any(|layer| layer.kind == "polyline");
    if has_polyline {
        draw_checkerboard(ctx.draw, &viewport, inner, ctx.theme, 1024.0);
    }
    for (index, layer) in layers.iter().enumerate() {
        if layer.kind == "image" {
            if let Some(data_url) = &layer.data_url {
                if let Some(key) = queue_canvas_image_upload(&scene.surface_id, &layer.id, data_url) {
                    let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
                    let w = layer.width as f32 * viewport.zoom;
                    let h = layer.height as f32 * viewport.zoom;
                    ctx.draw
                        .push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], 1.0);
                }
            }
            continue;
        }
        if layer.kind == "polyline" {
            if let Some(points) = &layer.points {
                let stroke = Rgba::new(0.2, 0.55, 0.95, 0.95);
                let seam_stroke = Rgba::new(0.95, 0.45, 0.2, 0.95);
                let width = (1.5 * viewport.zoom).max(1.0);
                for (edge_index, chunk) in points.chunks(2).enumerate() {
                    if chunk.len() < 2 {
                        continue;
                    }
                    let (x0, y0) = viewport.world_to_screen(chunk[0][0] as f32, chunk[0][1] as f32, inner);
                    let (x1, y1) = viewport.world_to_screen(chunk[1][0] as f32, chunk[1][1] as f32, inner);
                    let is_seam = layer
                        .seams
                        .as_ref()
                        .and_then(|seams| seams.get(edge_index))
                        .copied()
                        .unwrap_or(0)
                        != 0;
                    if is_seam {
                        draw_dashed_line(ctx.draw, x0, y0, x1, y1, seam_stroke, width);
                    } else {
                        ctx.draw.push_line(x0, y0, x1, y1, stroke, width);
                    }
                }
            }
            continue;
        }
        let hue = (index * 47 % 360) as f32;
        let stroke = Rgba::new(0.25 + hue / 720.0, 0.45, 0.65, 0.9);
        if layer.kind == "line" || layer.x0.is_some() {
            let x0 = layer.x0.unwrap_or(layer.x) as f32;
            let y0 = layer.y0.unwrap_or(layer.y) as f32;
            let x1 = layer.x1.unwrap_or(layer.x + layer.width) as f32;
            let y1 = layer.y1.unwrap_or(layer.y + layer.height) as f32;
            let (sx0, sy0) = viewport.world_to_screen(x0, y0, inner);
            let (sx1, sy1) = viewport.world_to_screen(x1, y1, inner);
            ctx.draw
                .push_line(sx0, sy0, sx1, sy1, stroke, (2.0 * viewport.zoom).max(1.0));
            continue;
        }
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let w = layer.width as f32 * viewport.zoom;
        let h = layer.height as f32 * viewport.zoom;
        ctx.draw.push_rounded(
            [sx, sy, w.max(8.0), h.max(8.0)],
            Rgba::new(0.25 + hue / 720.0, 0.35, 0.55, 0.8),
            4.0,
        );
        let label = if layer.name.is_empty() {
            layer.id.as_str()
        } else {
            layer.name.as_str()
        };
        if !label.is_empty() {
            draw_text(ctx, label, sx + 4.0, sy + 14.0, theme.font_size_small, theme.text);
        }
    }
    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: None,
        control_id: Some(scene.surface_id.clone()),
        kind: HitKind::Generic,
        drag_axis: Some(DragAxis::Both),
    drag_data: None,
    });
}
//#endregion Canvas2d

//#region NodeGraph
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphContextMenuItem {
    id: String,
    label: String,
    command: String,
    #[serde(default)]
    args: Option<Value>,
}

fn push_graph_context_menu(scene: &UiComponentSceneNode, graph: &semio_framework_core::NodeGraphScene) {
    let Some(raw) = graph.context_menu_json.as_deref() else {
        return;
    };
    let items: Vec<GraphContextMenuItem> = serde_json::from_str(raw).unwrap_or_default();
    for item in items {
        push_context_menu_item(ContextMenuItem {
            id: format!("{}.context.{}", scene.surface_id, item.id),
            label: item.label,
            command: Some(CommandDescriptor {
                controller_id: scene.controller_id.clone(),
                command: item.command,
                args: item.args,
            }),
        });
    }
}

/** @emoji 🕸️ Applies node-hit context to a scene context-menu command. */
pub fn resolve_graph_context_command(
    command: &CommandDescriptor,
    node_id: Option<&str>,
) -> CommandDescriptor {
    let Some(node_id) = node_id else {
        return command.clone();
    };
    let mut resolved = command.clone();
    match command.command.as_str() {
        "setMediaNodeSelection" => {
            resolved.args = Some(json!({ "nodeIds": [node_id] }));
        }
        "removeAppInstance" => {
            if let Some(instance_id) = graph_node_instance(node_id) {
                resolved.args = Some(json!({ "instanceId": instance_id }));
            }
        }
        "selectNode" => {
            resolved.args = Some(json!({ "nodeId": node_id }));
        }
        _ => {}
    }
    resolved
}

#[derive(Clone, Debug, Deserialize)]
struct GraphPort {
    id: String,
    #[serde(default)]
    label: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphNode {
    id: String,
    label: Option<String>,
    instance_id: Option<String>,
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    inputs: Option<Vec<GraphPort>>,
    outputs: Option<Vec<GraphPort>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphEdge {
    id: Option<String>,
    source: Option<String>,
    target: Option<String>,
    source_node_id: Option<String>,
    target_node_id: Option<String>,
    source_port_id: Option<String>,
    target_port_id: Option<String>,
}

fn parse_graph_nodes(json: &str) -> Vec<GraphNode> {
    serde_json::from_str(json).unwrap_or_default()
}

fn parse_graph_edges(json: &str) -> Vec<GraphEdge> {
    serde_json::from_str(json).unwrap_or_default()
}

fn find_graph_node(scene: &UiComponentSceneNode, node_id: &str) -> Option<GraphNode> {
    scene
        .node_graph
        .as_ref()
        .and_then(|graph| parse_graph_nodes(&graph.nodes_json).into_iter().find(|n| n.id == node_id))
}

fn hit_graph_node(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Option<String> {
    let graph = scene.node_graph.as_ref()?;
    let nodes = parse_graph_nodes(&graph.nodes_json);
    let state = scene_state(&scene.surface_id);
    let viewport = if state.viewport.zoom > 0.0 {
        state.viewport
    } else {
        Viewport::from_json(&graph.viewport_json)
    };
    for node in nodes.iter().rev() {
        let (nx, ny) = state
            .node_positions
            .get(&node.id)
            .copied()
            .unwrap_or((node.x.unwrap_or(0.0) as f32, node.y.unwrap_or(0.0) as f32));
        let (sx, sy) = viewport.world_to_screen(nx, ny, inner);
        let w = node.width.unwrap_or(180.0) as f32 * viewport.zoom;
        let h = node.height.unwrap_or(72.0) as f32 * viewport.zoom;
        let rect = Rect::new(sx, sy, w, h);
        if rect.contains(x, y) {
            return Some(node.id.clone());
        }
    }
    None
}

fn push_bezier(
    ctx: &mut FrameworkWidgetContext<'_>,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: Rgba,
    width: f32,
) {
    let cx0 = x0 + (x1 - x0) * 0.5;
    let cy0 = y0;
    let cx1 = x0 + (x1 - x0) * 0.5;
    let cy1 = y1;
    let segments = 16usize;
    let mut last = (x0, y0);
    for step in 1..=segments {
        let t = step as f32 / segments as f32;
        let u = 1.0 - t;
        let px = u * u * u * x0 + 3.0 * u * u * t * cx0 + 3.0 * u * t * t * cx1 + t * t * t * x1;
        let py = u * u * u * y0 + 3.0 * u * u * t * cy0 + 3.0 * u * t * t * cy1 + t * t * t * y1;
        ctx.draw.push_line(last.0, last.1, px, py, color, width);
        last = (px, py);
    }
}

fn render_node_graph(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
) {
    let Some(graph) = &scene.node_graph else {
        return render_placeholder("node-graph", bounds, ctx);
    };
    let nodes = parse_graph_nodes(&graph.nodes_json);
    push_graph_context_menu(scene, graph);
    for node in &nodes {
        register_graph_node(&node.id, node.instance_id.as_deref());
        let label = node
            .label
            .as_deref()
            .or(node.instance_id.as_deref())
            .unwrap_or(&node.id);
        push_find_item(ShellFindItem {
            id: node.id.clone(),
            label: label.to_string(),
            description: node.instance_id.clone(),
            category: Some("Nodes".into()),
            surface_id: scene.surface_id.clone(),
            node_id: node.id.clone(),
        });
    }
    let inner = bounds;
    engine_canvas::paint_node_graph(gpu, ctx, scene, inner);
}

fn node_screen_pos(node: &GraphNode, state: &SceneSurfaceState, viewport: &Viewport, inner: Rect) -> (f32, f32) {
    let (nx, ny) = state
        .node_positions
        .get(&node.id)
        .copied()
        .unwrap_or((node.x.unwrap_or(0.0) as f32, node.y.unwrap_or(0.0) as f32));
    viewport.world_to_screen(nx, ny, inner)
}
//#endregion NodeGraph

//#region VirtualFileSystem
#[derive(Deserialize)]
struct VfsDescriptorKind {
    #[serde(default)]
    presentation: String,
}

#[derive(Deserialize)]
struct VfsFileNodeKind {
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    descriptors: Vec<VfsDescriptorColumn>,
}

#[derive(Deserialize)]
struct VfsDescriptorColumn {
    id: String,
    #[serde(default)]
    label: String,
    #[serde(rename = "descriptorKindId", default)]
    descriptor_kind_id: String,
}

#[derive(Deserialize)]
struct VfsSchema {
    #[serde(rename = "descriptorColumnIds", default)]
    descriptor_column_ids: Vec<String>,
    #[serde(rename = "descriptorKinds", default)]
    descriptor_kinds: HashMap<String, VfsDescriptorKind>,
    #[serde(rename = "fileNodeKinds", default)]
    file_node_kinds: HashMap<String, VfsFileNodeKind>,
}

#[derive(Clone)]
struct VfsVisibleRow {
    row: Value,
    level: u32,
    has_children: bool,
    expanded: bool,
}

fn vfs_children_by_parent(rows: &[Value]) -> HashMap<String, Vec<Value>> {
    let mut map: HashMap<String, Vec<Value>> = HashMap::new();
    for row in rows {
        let parent = row
            .get("parentId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        map.entry(parent).or_default().push(row.clone());
    }
    map
}

fn build_vfs_visible_rows(rows: &[Value], expanded_ids: &HashSet<String>) -> Vec<VfsVisibleRow> {
    let children_by_parent = vfs_children_by_parent(rows);
    let mut visible = Vec::new();
    fn visit(
        node: &Value,
        level: u32,
        out: &mut Vec<VfsVisibleRow>,
        children_by_parent: &HashMap<String, Vec<Value>>,
        expanded_ids: &HashSet<String>,
    ) {
        let id = node.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let has_children = node.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or_else(|| {
            children_by_parent.get(&id).is_some_and(|c| !c.is_empty())
        });
        let expanded = has_children && expanded_ids.contains(&id);
        out.push(VfsVisibleRow {
            row: node.clone(),
            level,
            has_children,
            expanded,
        });
        if !expanded {
            return;
        }
        if let Some(children) = children_by_parent.get(&id) {
            for child in children {
                visit(child, level + 1, out, children_by_parent, expanded_ids);
            }
        }
    }
    let roots: Vec<Value> = rows
        .iter()
        .filter(|row| {
            row.get("parentId")
                .map(|v| v.is_null() || v.as_str() == Some(""))
                .unwrap_or(true)
        })
        .cloned()
        .collect();
    for root in roots {
        if root.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or(false) {
            let root_id = root.get("id").and_then(|v| v.as_str()).unwrap_or("");
            if let Some(children) = children_by_parent.get(root_id) {
                for child in children {
                    visit(child, 0, &mut visible, &children_by_parent, expanded_ids);
                }
            }
        } else {
            visit(&root, 0, &mut visible, &children_by_parent, expanded_ids);
        }
    }
    visible
}

fn vfs_glyph_icon(schema: &VfsSchema, row: &Value) -> &'static str {
    let kind_id = row.get("fileNodeKindId").and_then(|v| v.as_str()).unwrap_or("file");
    if schema.file_node_kinds.get(kind_id).and_then(|k| k.icon.as_deref()).is_some() {
        return "folder";
    }
    match kind_id {
        "root" | "studio" | "folder" => "folder",
        "instance" => "box",
        _ => "file-text",
    }
}

fn vfs_descriptor_label(schema: &VfsSchema, column_id: &str) -> String {
    for kind in schema.file_node_kinds.values() {
        if let Some(col) = kind.descriptors.iter().find(|c| c.id == column_id) {
            if !col.label.is_empty() {
                return col.label.clone();
            }
        }
    }
    column_id.to_string()
}

fn vfs_descriptor_value(schema: &VfsSchema, row: &Value, column_id: &str) -> String {
    let raw = row
        .get("descriptorValues")
        .and_then(|values| values.get(column_id))
        .map(|v| match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .unwrap_or_default();
    let kind_id = schema
        .file_node_kinds
        .values()
        .flat_map(|kind| kind.descriptors.iter())
        .find(|col| col.id == column_id)
        .map(|col| col.descriptor_kind_id.as_str())
        .unwrap_or("text");
    let presentation = schema
        .descriptor_kinds
        .get(kind_id)
        .map(|k| k.presentation.as_str())
        .unwrap_or("text");
    if presentation == "time" {
        if let Ok(ms) = raw.parse::<f64>() {
            let secs = (ms / 1000.0) as i64;
            let mins = secs / 60;
            let hours = mins / 60;
            return format!("{:02}:{:02}:{:02}", hours, mins % 60, secs % 60);
        }
    }
    raw
}

fn render_vfs(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(vfs) = &scene.virtual_file_system else {
        return render_placeholder("virtualFileSystem", bounds, ctx);
    };
    let schema: VfsSchema = serde_json::from_str(&vfs.schema_json).unwrap_or(VfsSchema {
        descriptor_column_ids: vec![],
        descriptor_kinds: HashMap::new(),
        file_node_kinds: HashMap::new(),
    });
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
    let root_expand_ids: Vec<String> = rows
        .iter()
        .filter(|row| row.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or(false))
        .filter_map(|row| row.get("id").and_then(|v| v.as_str()).map(str::to_string))
        .collect();
    seed_vfs_expanded(&scene.surface_id, &root_expand_ids);
    let selected: HashSet<String> = vfs
        .selected_row_ids_json
        .as_deref()
        .and_then(|json| serde_json::from_str::<Vec<String>>(json).ok())
        .unwrap_or_default()
        .into_iter()
        .collect();
    let state = scene_state(&scene.surface_id);
    let expanded_ids = state.vfs_expanded_ids;
    let visible_rows = build_vfs_visible_rows(&rows, &expanded_ids);
    let inner = bounds;
    let header_h = theme.control_height * 1.33;
    let row_h = theme.control_height;
    let pad = theme.padding_standard;
    let name_col_w = inner.w * 0.32;
    let descriptor_ids: Vec<String> = if schema.descriptor_column_ids.is_empty() {
        vec![]
    } else {
        schema.descriptor_column_ids.clone()
    };
    let descriptor_col_w = if descriptor_ids.is_empty() {
        0.0
    } else {
        (inner.w - name_col_w) / descriptor_ids.len() as f32
    };
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    draw_text(ctx, "Name", inner.x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    for (index, column_id) in descriptor_ids.iter().enumerate() {
        let x = inner.x + name_col_w + index as f32 * descriptor_col_w;
        draw_text(
            ctx,
            &vfs_descriptor_label(&schema, column_id),
            x + pad,
            inner.y + header_h * 0.65,
            theme.font_size_small,
            theme.text_muted,
        );
    }
    let body = Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h);
    let scroll = scroll_offset(&scene.surface_id, "vfs");
    ctx.input.register_hit(HitTarget {
        rect: body,
        event: None,
        control_id: Some(scroll_key(&scene.surface_id, "vfs")),
        kind: HitKind::ScrollRegion,
        drag_axis: None,
        drag_data: None,
    });
    ctx.draw.push_scissor(body);
    let hovered_row = vfs
        .hovered_row_id
        .clone()
        .or_else(|| ctx.input.hovered_id.clone());
    if visible_rows.is_empty() {
        let message = vfs.empty_message.as_deref().unwrap_or("No file system nodes");
        draw_text(ctx, message, body.x + pad, body.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    for entry in &visible_rows {
        let row = &entry.row;
        let row_index = visible_rows.iter().position(|v| v.row.get("id") == row.get("id")).unwrap_or(0);
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let control_id = format!("{}.vfs.{}", scene.surface_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let selected_row = selected.contains(&row_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        if selected_row {
            ctx.draw
                .push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw
                .push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(
            row_rect.x,
            row_rect.y + row_rect.h - theme.stroke_hairline,
            row_rect.x + row_rect.w,
            row_rect.y + row_rect.h - theme.stroke_hairline,
            theme.separator,
            1.0,
        );
        let indent = entry.level as f32 * 14.0;
        let mut name_x = body.x + pad + indent;
        if entry.has_children {
            let chevron_rect = Rect::new(name_x, y, 14.0, row_h);
            let chevron = if entry.expanded { "chevron-down" } else { "chevron-right" };
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(chevron) {
                    ctx.draw.push_textured(
                        [chevron_rect.x, y + (row_h - 14.0) * 0.5, 14.0, 14.0],
                        uv,
                        ctx.theme.text_element,
                    );
                }
            }
            ctx.input.register_hit(HitTarget {
                rect: chevron_rect,
                event: None,
                control_id: Some(format!("{}.vfs.chevron.{}", scene.surface_id, row_id)),
                kind: HitKind::Generic,
                drag_axis: None,
                drag_data: None,
            });
            name_x += 14.0;
        }
        let icon_id = vfs_glyph_icon(&schema, row);
        if let Some(icons) = ctx.icons {
            if let Some(uv) = icons.icon_uv(icon_id) {
                ctx.draw.push_textured(
                    [name_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0],
                    uv,
                    ctx.theme.text_element,
                );
            }
        }
        name_x += 18.0;
        let name = row.get("name").and_then(|v| v.as_str()).unwrap_or("—");
        draw_text(
            ctx,
            name,
            name_x,
            y + row_h * 0.65,
            theme.font_size_small,
            if selected_row || hovered {
                theme.active_foreground
            } else {
                theme.text
            },
        );
        for (col_index, column_id) in descriptor_ids.iter().enumerate() {
            let x = body.x + name_col_w + col_index as f32 * descriptor_col_w;
            let value = vfs_descriptor_value(&schema, row, column_id);
            draw_text(
                ctx,
                &value,
                x + pad,
                y + row_h * 0.65,
                theme.font_size_small,
                if selected_row { theme.active_foreground } else { theme.text_muted },
            );
        }
        let drag_data = if vfs.drag_drop_enabled.unwrap_or(false) {
            let mut data = HashMap::new();
            data.insert(
                "application/x-semio-vfs-node".into(),
                serde_json::to_string(row).unwrap_or_default(),
            );
            Some(data)
        } else {
            None
        };
        ctx.input.register_hit(HitTarget {
            rect: row_rect,
            event: None,
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data,
        });
    }
    ctx.draw.pop_scissor();
}

fn vfs_double_click_command(scene: &UiComponentSceneNode, row: &Value) -> Option<CommandDescriptor> {
    let uri = row.get("navigateUri").and_then(|v| v.as_str())?;
    if uri.starts_with("os://instance/") {
        return Some(scene_cmd(
            scene,
            "openInstance",
            json!({
                "surfaceId": scene.surface_id,
                "instanceId": uri.trim_start_matches("os://instance/"),
            }),
        ));
    }
    if uri.starts_with("os://export/") {
        let parts: Vec<&str> = uri.split('/').collect();
        if parts.len() >= 5 {
            return Some(scene_cmd(
                scene,
                "exportMedia",
                json!({
                    "surfaceId": scene.surface_id,
                    "instanceId": parts[2],
                    "format": parts[4],
                }),
            ));
        }
    }
    if uri.starts_with("/studios/") {
        let studio_id = uri.split('/').nth(2)?;
        return Some(scene_cmd(
            scene,
            "navigateVirtualFileSystemNode",
            json!({ "surfaceId": scene.surface_id, "studioId": studio_id }),
        ));
    }
    if let Some(studio_id) = uri.strip_prefix("studio:") {
        return Some(scene_cmd(
            scene,
            "navigateVirtualFileSystemNode",
            json!({ "surfaceId": scene.surface_id, "studioId": studio_id }),
        ));
    }
    None
}
//#endregion VirtualFileSystem

//#region TextEditor
fn cursor_from_click(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    scroll: f32,
) -> usize {
    let Some(editor) = &scene.text_editor else {
        return 0;
    };
    let line_h = 18.0;
    let line_index = ((y - inner.y - 8.0 + scroll) / line_h).max(0.0) as usize;
    let lines: Vec<&str> = editor.buffer.lines().collect();
    let line = lines.get(line_index).copied().unwrap_or("");
    let rel_x = (x - inner.x - 8.0).max(0.0);
    let mut cursor = 0usize;
    let mut width = 0.0f32;
    for (index, ch) in line.chars().enumerate() {
        let advance = if ch == '\t' { 8.0 } else { 7.0 };
        if width + advance * 0.5 > rel_x {
            cursor = index;
            break;
        }
        width += advance;
        cursor = index + 1;
    }
    lines.iter().take(line_index).map(|l| l.len() + 1).sum::<usize>() + cursor
}

fn render_text_editor(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
) {
    let Some(editor) = &scene.text_editor else {
        return render_placeholder("text-editor", bounds, ctx);
    };
    let inner = bounds;
    engine_canvas::paint_text_editor(gpu, ctx, scene, inner);
    let editor_id = format!("{}.editor", scene.surface_id);
    let focused = ctx.input.focused_id.as_deref() == Some(editor_id.as_str());
    if focused && ctx.input.text_buffer.is_empty() && !editor.buffer.is_empty() {
        ctx.input.focus_input(&editor_id, &editor.buffer);
    }
    if focused {
        let modifiers = ctx.input.modifiers.clone();
        for key in ctx.input.drain_keys() {
            match key {
                KeyAction::Enter if modifiers.meta || modifiers.ctrl => {
                    ctx.input.queue_event(scene_cmd(
                        scene,
                        "submit",
                        json!({ "surfaceId": scene.surface_id, "document": editor.buffer }),
                    ));
                }
                KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("s") => {
                    ctx.input.queue_event(scene_cmd(
                        scene,
                        "formatDocument",
                        json!({ "surfaceId": scene.surface_id }),
                    ));
                }
                KeyAction::Enter | KeyAction::Escape => {
                    ctx.input.queue_event(scene_cmd(
                        scene,
                        "textEdit",
                        json!({ "surfaceId": scene.surface_id, "document": editor.buffer }),
                    ));
                    if matches!(key, KeyAction::Escape) {
                        ctx.input.blur_input();
                    }
                }
                KeyAction::Char(_) | KeyAction::Backspace | KeyAction::Delete => {
                    for command in engine_canvas::text_editor_apply_key(scene, key, &modifiers) {
                        ctx.input.queue_event(command);
                    }
                }
                _ => {}
            }
        }
    }
    if ctx.input.pointer_down
        && inner.contains(ctx.input.pointer_x, ctx.input.pointer_y)
        && ctx.input.pointer_button == 0
    {
        ctx.input.focus_input(&editor_id, &editor.buffer);
    }
}

fn line_col_at(text: &str, cursor: usize) -> (usize, usize) {
    let mut index = 0usize;
    for (line_index, line) in text.lines().enumerate() {
        let next = index + line.len() + 1;
        if cursor < next {
            return (line_index, cursor.saturating_sub(index));
        }
        index = next;
    }
    let line_count = text.lines().count();
    (line_count.saturating_sub(1), 0)
}
//#endregion TextEditor
