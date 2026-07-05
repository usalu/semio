//! 🎨 Embeds GraphHost, FlowHost, and EditorHost via vello offscreen compositing.

use crate::interpreter::FrameworkWidgetContext;
use flow_core::FlowHost;
use framework_editor::EditorHost;
use framework_graph::GraphHost;
use infinite_cavas as cavas;
use semio_framework_core::{CommandDescriptor, UiComponentSceneNode};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ui_wgpu::{draw_text, FontAtlas, GpuContext, HitKind, HitTarget, Rect, Rgba, Theme};
use vello::peniko::Color;
use vello::wgpu;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions};

fn vello_clear(theme: &Theme) -> Color {
    let c = theme.canvas_clear;
    Color::new([c.r, c.g, c.b, c.a])
}

//#region Registry
enum NodeGraphEngine {
    Dag(GraphHost),
    Flow(FlowHost),
}

#[derive(Default)]
struct NodeGraphSyncCache {
    fixture_json: Option<String>,
    selection_json: Option<String>,
    preview_off_json: Option<String>,
    catalogue_json: Option<String>,
    operators_json: Option<String>,
    computing_json: Option<String>,
    lod_json: Option<String>,
    viewport_json: Option<String>,
    scene_json: Option<String>,
    is_dark: Option<bool>,
}

struct EngineSurface {
    node_graph: Option<NodeGraphEngine>,
    sync_cache: NodeGraphSyncCache,
    editor: Option<EditorHost>,
    vello: Renderer,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    width: u32,
    height: u32,
}

fn sync_field(cache: &mut Option<String>, value: &str) -> bool {
    if cache.as_deref() == Some(value) {
        false
    } else {
        *cache = Some(value.to_string());
        true
    }
}

fn sync_bool(cache: &mut Option<bool>, value: bool) -> bool {
    if cache == &Some(value) {
        false
    } else {
        *cache = Some(value);
        true
    }
}

fn theme_is_dark(theme: &Theme) -> bool {
    let c = theme.canvas_clear;
    let lum = f64::from(linear_to_rgba8_channel(c.r))
        * 0.299
        + f64::from(linear_to_rgba8_channel(c.g)) * 0.587
        + f64::from(linear_to_rgba8_channel(c.b)) * 0.114;
    lum < 128.0
}

fn linear_to_rgba8_channel(linear: f32) -> u8 {
    if linear <= 0.0031308 {
        (linear * 12.92 * 255.0).round() as u8
    } else {
        (1.055 * linear.powf(1.0 / 2.4) - 0.055).mul_add(255.0, 0.0).round() as u8
    }
}

fn sync_canvas_theme_dark(cache: &mut NodeGraphSyncCache, dark: bool, flow: &mut FlowHost) {
    if sync_bool(&mut cache.is_dark, dark) {
        flow.set_canvas_theme_dark(dark);
    }
}

fn sync_graph_canvas_theme_dark(cache: &mut NodeGraphSyncCache, dark: bool, graph: &mut GraphHost) {
    if sync_bool(&mut cache.is_dark, dark) {
        graph.set_canvas_theme_dark(dark);
    }
}

thread_local! {
    static ENGINE_SURFACES: RefCell<HashMap<String, EngineSurface>> = RefCell::new(HashMap::new());
}

fn raster_key(surface_id: &str) -> String {
    format!("engine:{surface_id}")
}

fn is_flow_graph(graph: &semio_framework_core::NodeGraphScene) -> bool {
    if graph
        .fixture_json
        .as_ref()
        .is_some_and(|json| !json.trim().is_empty())
    {
        return true;
    }
    graph
        .capabilities_json
        .as_deref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| value.get("engine").and_then(|engine| engine.as_str()).map(|id| id == "flow"))
        .unwrap_or(false)
}

fn scene_cmd(scene: &UiComponentSceneNode, command: &str, args: Value) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: scene.controller_id.clone(),
        command: command.to_string(),
        args: Some(args),
    }
}

fn graph_cmd(controller_id: &str, surface_id: &str, command: &str, args: Value) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: controller_id.to_string(),
        command: command.to_string(),
        args: Some(args),
    }
}

fn graph_scene_json(graph: &semio_framework_core::NodeGraphScene) -> String {
    serde_json::to_string(graph).unwrap_or_else(|_| "{}".into())
}

fn editor_scene_json(editor: &semio_framework_core::TextEditorScene) -> String {
    serde_json::to_string(editor).unwrap_or_else(|_| "{}".into())
}

fn sync_flow_host(host: &mut FlowHost, graph: &semio_framework_core::NodeGraphScene, cache: &mut NodeGraphSyncCache) {
    if let Some(fixture_json) = &graph.fixture_json {
        if sync_field(&mut cache.fixture_json, fixture_json) {
            if let Ok(fixture) = FlowHost::parse_fixture_json(fixture_json) {
                host.replace_fixture(fixture);
            }
        }
    }
    if let Some(json) = &graph.catalogue_json {
        if sync_field(&mut cache.catalogue_json, json) {
            host.set_host_catalogue_json(json);
        }
    }
    if let Some(json) = &graph.operators_json {
        if sync_field(&mut cache.operators_json, json) {
            host.set_neuron_kind_infos_json(json);
        }
    }
    if let Some(json) = &graph.selection_json {
        if sync_field(&mut cache.selection_json, json) {
            host.set_selection_json(json);
        }
    }
    if let Some(json) = &graph.preview_off_json {
        if sync_field(&mut cache.preview_off_json, json) {
            host.set_preview_off_json(json);
        }
    }
    if let Some(json) = &graph.computing_json {
        if sync_field(&mut cache.computing_json, json) {
            if let Ok(value) = serde_json::from_str::<Value>(json) {
                let active = value.get("active").and_then(|v| v.as_str()).map(str::to_string);
                let stale: Vec<String> = value
                    .get("stale")
                    .and_then(|v| v.as_array())
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(|item| item.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                host.set_computing_progress(active.as_deref(), &stale);
            }
        }
    }
    if let Some(json) = &graph.lod_json {
        if sync_field(&mut cache.lod_json, json) {
            if let Ok(value) = serde_json::from_str::<Value>(json) {
                if let Some(automatic) = value.get("automatic").and_then(|v| v.as_bool()) {
                    host.set_automatic_lod(automatic);
                }
                if let Some(label) = value.get("forcedLabel").and_then(|v| v.as_str()) {
                    host.set_forced_draw_lod_label(label);
                }
            }
        }
    }
    if sync_field(&mut cache.viewport_json, &graph.viewport_json) {
        if let Ok(viewport) = serde_json::from_str::<Value>(&graph.viewport_json) {
            let x = viewport.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let y = viewport.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let zoom = viewport.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0);
            host.set_camera(x, y, zoom);
        }
    }
}

fn ensure_surface(
    gpu: &GpuContext,
    surface_id: &str,
    pw: u32,
    ph: u32,
) -> Result<(), String> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let needs_create = !map.contains_key(surface_id);
        let needs_resize = map
            .get(surface_id)
            .is_some_and(|entry| entry.width != pw.max(1) || entry.height != ph.max(1));
        if needs_create {
            let device = gpu.device();
            let vello = Renderer::new(
                device,
                RendererOptions {
                    use_cpu: false,
                    antialiasing_support: AaSupport::area_only(),
                    num_init_threads: std::num::NonZeroUsize::new(1),
                    pipeline_cache: None,
                },
            )
            .map_err(|err| format!("vello renderer: {err:?}"))?;
            let (texture, view) = create_target_texture(device, pw.max(1), ph.max(1));
            map.insert(
                surface_id.to_string(),
                EngineSurface {
                    node_graph: None,
                    sync_cache: NodeGraphSyncCache::default(),
                    editor: None,
                    vello,
                    texture,
                    view,
                    width: pw.max(1),
                    height: ph.max(1),
                },
            );
            return Ok(());
        }
        if needs_resize {
            let device = gpu.device();
            let entry = map.get_mut(surface_id).expect("surface");
            let (texture, view) = create_target_texture(device, pw.max(1), ph.max(1));
            entry.texture = texture;
            entry.view = view;
            entry.width = pw.max(1);
            entry.height = ph.max(1);
        }
        Ok(())
    })
}

fn create_target_texture(device: &wgpu::Device, width: u32, height: u32) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("engine_canvas_target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

fn render_vello_scene(
    gpu: &mut GpuContext,
    surface_id: &str,
    scene: &cavas::Scene,
    clear: Color,
) -> Result<(), String> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id).ok_or_else(|| "missing engine surface".to_string())?;
        let params = RenderParams {
            base_color: clear,
            width: entry.width,
            height: entry.height,
            antialiasing_method: AaConfig::Area,
        };
        entry
            .vello
            .render_to_texture(gpu.device(), gpu.queue(), scene.vello_scene(), &entry.view, &params)
            .map_err(|err| format!("vello render: {err:?}"))?;
        let device = gpu.device();
        let published_view = entry.view.clone();
        let published_texture = std::mem::replace(
            &mut entry.texture,
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("engine_canvas_target"),
                size: wgpu::Extent3d {
                    width: entry.width,
                    height: entry.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }),
        );
        entry.view = entry.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let width = entry.width;
        let height = entry.height;
        gpu.register_engine_texture(
            &raster_key(surface_id),
            published_texture,
            &published_view,
            width,
            height,
        );
        Ok(())
    })
}
//#endregion Registry

//#region NodeGraph
pub fn paint_node_graph(
    gpu: &mut GpuContext,
    ctx: &mut FrameworkWidgetContext<'_>,
    scene: &UiComponentSceneNode,
    inner: Rect,
) {
    let Some(graph) = &scene.node_graph else {
        return;
    };
    let pw = inner.w.max(1.0) as u32;
    let ph = inner.h.max(1.0) as u32;
    let dpr = gpu.dpr() as f64;
    let flow = is_flow_graph(graph);
    if ensure_surface(gpu, &scene.surface_id, pw, ph).is_err() {
        return;
    }
    let clear = vello_clear(ctx.theme);
    let scene_json = graph_scene_json(graph);
    let dark = theme_is_dark(ctx.theme);
    let mut cavas_scene = cavas::Scene::new();
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(&scene.surface_id).expect("engine surface");
        if flow {
            let engine = match entry.node_graph.as_mut() {
                Some(NodeGraphEngine::Flow(host)) => host,
                _ => {
                    entry.node_graph = Some(NodeGraphEngine::Flow(FlowHost::default()));
                    entry.sync_cache = NodeGraphSyncCache::default();
                    match entry.node_graph.as_mut() {
                        Some(NodeGraphEngine::Flow(host)) => host,
                        _ => return,
                    }
                }
            };
            sync_flow_host(engine, graph, &mut entry.sync_cache);
            sync_canvas_theme_dark(&mut entry.sync_cache, dark, engine);
            engine.set_viewport(pw, ph, dpr);
            engine.paint_scene(&mut cavas_scene, pw, ph, dpr);
        } else {
            let engine = match entry.node_graph.as_mut() {
                Some(NodeGraphEngine::Dag(host)) => host,
                _ => {
                    entry.node_graph = Some(NodeGraphEngine::Dag(GraphHost::default()));
                    entry.sync_cache = NodeGraphSyncCache::default();
                    match entry.node_graph.as_mut() {
                        Some(NodeGraphEngine::Dag(host)) => host,
                        _ => return,
                    }
                }
            };
            if sync_field(&mut entry.sync_cache.scene_json, &scene_json) {
                let _ = engine.sync_from_scene_json(&scene_json);
            }
            sync_graph_canvas_theme_dark(&mut entry.sync_cache, dark, engine);
            engine.set_viewport(pw, ph, dpr);
            engine.paint_scene(&mut cavas_scene, pw, ph, dpr);
        }
    });
    if render_vello_scene(gpu, &scene.surface_id, &cavas_scene, clear).is_err() {
        return;
    }
    ctx.draw.push_raster_quad(
        &raster_key(&scene.surface_id),
        [inner.x, inner.y, inner.w, inner.h],
        [0.0, 0.0, 1.0, 1.0],
        1.0,
    );
    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: None,
        control_id: Some(format!("{}.pane", scene.surface_id)),
        kind: HitKind::ScrollRegion,
        drag_axis: Some(ui_wgpu::input::DragAxis::Both),
        drag_data: None,
    });
}

pub fn node_graph_wheel(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    delta: f32,
    ctrl: bool,
) -> Vec<CommandDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) => {
                host.wheel_screen(sx, sy, 0.0, delta as f64, ctrl);
            }
            Some(NodeGraphEngine::Dag(host)) => {
                host.wheel_screen(sx, sy, delta as f64, true);
            }
            None => return Vec::new(),
        }
        graph_interaction_commands(surface_id, controller_id, entry)
    })
}

pub fn node_graph_pointer_down(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    button: i16,
    shift: bool,
    ctrl: bool,
    alt: bool,
) -> Vec<CommandDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) => {
                host.pointer_down_screen(sx, sy, button as u8, shift, ctrl, alt, button == 1);
            }
            Some(NodeGraphEngine::Dag(host)) => {
                host.pointer_down_screen(sx, sy, button as u8, shift, ctrl, alt);
            }
            None => return Vec::new(),
        }
        graph_interaction_commands(surface_id, controller_id, entry)
    })
}

pub fn node_graph_pointer_move(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl: bool,
    alt: bool,
) -> Vec<CommandDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) => {
                host.pointer_move_screen(sx, sy, shift, ctrl, alt);
            }
            Some(NodeGraphEngine::Dag(host)) => {
                host.pointer_move_screen(sx, sy, shift, ctrl, alt);
            }
            None => return Vec::new(),
        }
        graph_interaction_commands(surface_id, controller_id, entry)
    })
}

pub fn node_graph_pointer_up(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl: bool,
    alt: bool,
) -> Vec<CommandDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) => {
                host.pointer_up_screen(sx, sy, shift, ctrl, alt);
            }
            Some(NodeGraphEngine::Dag(host)) => {
                host.pointer_up_screen(sx, sy, shift, ctrl, alt);
            }
            None => return Vec::new(),
        }
        graph_interaction_commands(surface_id, controller_id, entry)
    })
}

fn graph_interaction_commands(
    surface_id: &str,
    controller_id: &str,
    entry: &EngineSurface,
) -> Vec<CommandDescriptor> {
    let (node_ids, hover_json, viewport_json) = match entry.node_graph.as_ref() {
        Some(NodeGraphEngine::Flow(host)) => {
            let ids: Vec<String> =
                serde_json::from_str(&host.selected_widget_ids_json()).unwrap_or_default();
            (
                ids,
                host.hovered_widget_id()
                    .map(|id| json!({ "nodeId": id }).to_string())
                    .unwrap_or_else(|| "null".into()),
                serde_json::to_string(&host.dag.fixture.camera).unwrap_or_else(|_| "{}".into()),
            )
        }
        Some(NodeGraphEngine::Dag(host)) => {
            let ids: Vec<String> =
                serde_json::from_str(&host.selected_node_ids_json()).unwrap_or_default();
            (
                ids,
                host.hovered_node_id()
                    .map(|id| json!({ "nodeId": id }).to_string())
                    .unwrap_or_else(|| "null".into()),
                host.camera_json(),
            )
        }
        None => return Vec::new(),
    };
    vec![
        graph_cmd(
            controller_id,
            surface_id,
            "nodeGraphSelect",
            json!({ "surfaceId": surface_id, "nodeIds": node_ids }),
        ),
        graph_cmd(
            controller_id,
            surface_id,
            "nodeGraphHover",
            json!({ "surfaceId": surface_id, "hoverJson": hover_json }),
        ),
        graph_cmd(
            controller_id,
            surface_id,
            "nodeGraphViewport",
            json!({ "surfaceId": surface_id, "viewportJson": viewport_json }),
        ),
    ]
}

fn world_to_screen_inner(inner: Rect, cam_x: f64, cam_y: f64, zoom: f64, wx: f64, wy: f64) -> (f32, f32) {
    let zoom = zoom.max(0.05) as f32;
    let cx = inner.w * 0.5;
    let cy = inner.h * 0.5;
    let sx = inner.x + (wx - cam_x) as f32 * zoom + cx;
    let sy = inner.y + (wy - cam_y) as f32 * zoom + cy;
    (sx, sy)
}

const DAG_LABEL_SCREEN_PX: f32 = 11.0;
const LABEL_INSET: f32 = 0.88;

struct LabelInteractionChrome {
    selected_ids: HashSet<String>,
    highlighted_ids: HashSet<String>,
    hovered_id: Option<String>,
    dimmed_ids: Vec<String>,
}

fn label_chrome_from_flow(host: &FlowHost) -> LabelInteractionChrome {
    let selected: Vec<String> =
        serde_json::from_str(&host.selected_widget_ids_json()).unwrap_or_default();
    let preselect: Value =
        serde_json::from_str(&host.preselect_widget_ids_json()).unwrap_or(json!({}));
    let pre_ids: Vec<String> = preselect
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let removed: Vec<String> = preselect
        .get("removedIds")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let (selected_ids, highlighted_ids) = if pre_ids.is_empty() && removed.is_empty() {
        (selected.into_iter().collect(), HashSet::new())
    } else {
        (pre_ids.into_iter().collect(), removed.into_iter().collect())
    };
    LabelInteractionChrome {
        selected_ids,
        highlighted_ids,
        hovered_id: host.hovered_widget_id(),
        dimmed_ids: host.preview_off_widget_ids(),
    }
}

fn label_chrome_from_graph(host: &GraphHost) -> LabelInteractionChrome {
    let selected = host.dag.selected_node_ids();
    let pre_ids = host.dag.preselect_widget_ids();
    let removed = host.dag.preselect_removed_widget_ids();
    let (selected_ids, highlighted_ids) = if pre_ids.is_empty() && removed.is_empty() {
        (selected.into_iter().collect(), HashSet::new())
    } else {
        (pre_ids.into_iter().collect(), removed.into_iter().collect())
    };
    LabelInteractionChrome {
        selected_ids,
        highlighted_ids,
        hovered_id: host.dag.hovered_node_id(),
        dimmed_ids: Vec::new(),
    }
}

fn clamp_label_font_px(atlas: &mut FontAtlas, text: &str, target_px: f32, max_w: f32, max_h: f32) -> f32 {
    let mut px = target_px.max(4.0).round();
    let (w, h) = atlas.measure_text(text, px);
    if w <= max_w && h * 1.2 <= max_h {
        return px;
    }
    let mut low = 4.0_f32;
    let mut high = px;
    let mut best = 4.0_f32;
    while low <= high {
        let mid = ((low + high) * 0.5).floor();
        let (w, h) = atlas.measure_text(text, mid);
        if w <= max_w && h * 1.2 <= max_h {
            best = mid;
            low = mid + 1.0;
        } else {
            high = mid - 1.0;
        }
    }
    best
}

fn clamp_port_label_font_px(atlas: &mut FontAtlas, text: &str, target_px: f32, max_w: f32, max_h: f32) -> f32 {
    let mut px = target_px.max(8.0).round();
    let (w, _) = atlas.measure_text(text, px);
    if w <= max_w && px * 1.25 <= max_h {
        return px;
    }
    let mut low = 8.0_f32;
    let mut high = px;
    let mut best = 8.0_f32;
    while low <= high {
        let mid = ((low + high) * 0.5).floor();
        let (w, _) = atlas.measure_text(text, mid);
        if w <= max_w {
            best = mid;
            low = mid + 1.0;
        } else {
            high = mid - 1.0;
        }
    }
    best
}

fn label_overlay_fill(
    theme: &Theme,
    node_id: &str,
    ghost: bool,
    chrome: &LabelInteractionChrome,
) -> Rgba {
    if ghost {
        return theme.text_muted;
    }
    if chrome.dimmed_ids.iter().any(|id| id == node_id) {
        return theme.text_muted.with_alpha(0.5);
    }
    if chrome.selected_ids.contains(node_id) {
        return theme.active_foreground;
    }
    if chrome.highlighted_ids.contains(node_id) {
        return theme.text_muted;
    }
    if chrome.hovered_id.as_deref() == Some(node_id) {
        return theme.active_foreground;
    }
    theme.text_element
}

fn paint_label_overlay_row(
    ctx: &mut FrameworkWidgetContext<'_>,
    inner: Rect,
    cam_x: f64,
    cam_y: f64,
    zoom: f64,
    row: &Value,
    chrome: &LabelInteractionChrome,
) {
    let Some(text) = row.get("text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) else {
        return;
    };
    let wx = row.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let wy = row.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let node_w = row.get("nodeW").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let node_h = row.get("nodeH").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let align = row.get("align").and_then(|v| v.as_str());
    let ghost = row.get("ghost").and_then(|v| v.as_bool()).unwrap_or(false);
    let node_id = row.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let is_port = row.get("kind").and_then(|v| v.as_str()) == Some("port")
        || matches!(align, Some("left") | Some("right"));
    let zoom_f = zoom.max(0.05) as f32;
    let max_w = (node_w * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32;
    let max_h = if is_port {
        row.get("maxScreenH")
            .and_then(|v| v.as_f64())
            .filter(|h| *h > 0.0)
            .map(|h| h as f32)
            .unwrap_or((node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32)
    } else {
        (node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32
    };
    let target_px = row
        .get("fontScreenPx")
        .and_then(|v| v.as_f64())
        .filter(|px| *px > 0.0)
        .map(|px| px as f32)
        .unwrap_or(DAG_LABEL_SCREEN_PX);
    let font_px = if is_port {
        clamp_port_label_font_px(&mut ctx.atlas, text, target_px, max_w, max_h)
    } else {
        clamp_label_font_px(&mut ctx.atlas, text, target_px, max_w, max_h)
    };
    let (anchor_x, anchor_y) = world_to_screen_inner(inner, cam_x, cam_y, zoom, wx, wy);
    let (text_w, text_h) = ctx.atlas.measure_text(text, font_px);
    let tx = match align {
        Some("left") => anchor_x,
        Some("right") => anchor_x - text_w,
        _ => anchor_x - text_w * 0.5,
    };
    let ty = anchor_y + text_h * 0.5;
    let fill = label_overlay_fill(ctx.theme, node_id, ghost, chrome);
    let alpha = if ghost {
        0.85
    } else if chrome.dimmed_ids.iter().any(|id| id == node_id) {
        0.5
    } else {
        1.0
    };
    draw_text(ctx, text, tx, ty, font_px, fill.with_alpha(fill.a * alpha));
}

pub fn paint_node_graph_labels(
    ctx: &mut FrameworkWidgetContext<'_>,
    scene: &UiComponentSceneNode,
    inner: Rect,
) {
    let snapshot = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(&scene.surface_id)?;
        match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => {
                let state_json = host.label_overlay_paint_state_json().ok()?;
                Some((state_json, label_chrome_from_flow(host)))
            }
            Some(NodeGraphEngine::Dag(host)) => {
                let state_json = host.label_overlay_paint_state_json().ok()?;
                Some((state_json, label_chrome_from_graph(host)))
            }
            None => None,
        }
    });
    let Some((state_json, chrome)) = snapshot else {
        return;
    };
    let Ok(state) = serde_json::from_str::<Value>(&state_json) else {
        return;
    };
    let cam = state.get("camera").cloned().unwrap_or(json!({}));
    let cam_x = cam.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let cam_y = cam.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let zoom = cam.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let labels = state
        .get("labels")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    for row in &labels {
        paint_label_overlay_row(ctx, inner, cam_x, cam_y, zoom, row, &chrome);
    }
}
//#endregion NodeGraph

//#region TextEditor
pub fn text_editor_apply_key(
    scene: &UiComponentSceneNode,
    key: ui_wgpu::KeyAction,
    modifiers: &ui_wgpu::PointerModifiers,
) -> Vec<CommandDescriptor> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        match key {
            ui_wgpu::KeyAction::Char(ch) if !(modifiers.meta || modifiers.ctrl) => {
                host.insert_text(&ch.to_string());
            }
            ui_wgpu::KeyAction::Backspace => host.backspace(),
            ui_wgpu::KeyAction::Delete => host.delete_forward(),
            ui_wgpu::KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("a") => {
                host.select_all();
            }
            _ => return Vec::new(),
        }
        text_editor_interaction_commands(scene, host)
    })
}

pub fn paint_text_editor(
    gpu: &mut GpuContext,
    ctx: &mut FrameworkWidgetContext<'_>,
    scene: &UiComponentSceneNode,
    inner: Rect,
) {
    let Some(editor) = &scene.text_editor else {
        return;
    };
    let pw = inner.w.max(1.0) as u32;
    let ph = inner.h.max(1.0) as u32;
    let dpr = gpu.dpr() as f64;
    if ensure_surface(gpu, &scene.surface_id, pw, ph).is_err() {
        return;
    }
    let clear = vello_clear(ctx.theme);
    let scene_json = editor_scene_json(editor);
    let cavas_scene = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(&scene.surface_id).expect("engine surface");
        if entry.editor.is_none() {
            entry.editor = Some(EditorHost::new());
        }
        let host = entry.editor.as_mut().expect("editor host");
        let _ = host.sync_from_scene_json(&scene_json);
        host.set_size(pw, ph, dpr);
        host.build_scene()
    });
    if render_vello_scene(gpu, &scene.surface_id, &cavas_scene, clear).is_err() {
        return;
    }
    ctx.draw.push_raster_quad(
        &raster_key(&scene.surface_id),
        [inner.x, inner.y, inner.w, inner.h],
        [0.0, 0.0, 1.0, 1.0],
        1.0,
    );
    let editor_id = format!("{}.editor", scene.surface_id);
    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: None,
        control_id: Some(editor_id),
        kind: HitKind::Input,
        drag_axis: None,
        drag_data: None,
    });
}

pub fn text_editor_wheel(scene: &UiComponentSceneNode, delta: f32) -> Vec<CommandDescriptor> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.wheel_scroll_screen(delta as f64);
        Vec::new()
    })
}

pub fn text_editor_pointer_down(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    button: i16,
) -> Vec<CommandDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.pointer_down_screen(sx, sy, button as i32);
        text_editor_interaction_commands(scene, host)
    })
}

pub fn text_editor_pointer_move(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<CommandDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.pointer_move_screen(sx, sy, 0);
        text_editor_interaction_commands(scene, host)
    })
}

pub fn text_editor_pointer_up(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<CommandDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.editor.as_mut() else {
            return Vec::new();
        };
        host.pointer_up_screen(sx, sy, 0);
        text_editor_interaction_commands(scene, host)
    })
}

fn text_editor_interaction_commands(
    scene: &UiComponentSceneNode,
    host: &EditorHost,
) -> Vec<CommandDescriptor> {
    vec![
        scene_cmd(
            scene,
            "textSelect",
            json!({
                "surfaceId": scene.surface_id,
                "selectionJson": json!({ "start": host.anchor(), "end": host.caret() }).to_string(),
            }),
        ),
        scene_cmd(
            scene,
            "textEdit",
            json!({ "surfaceId": scene.surface_id, "document": host.text() }),
        ),
    ]
}
//#endregion TextEditor
