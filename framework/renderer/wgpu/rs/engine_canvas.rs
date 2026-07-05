//! 🎨 Embeds GraphHost, FlowHost, and EditorHost via vello offscreen compositing.

use crate::interpreter::FrameworkWidgetContext;
use flow_core::FlowHost;
use framework_editor::EditorHost;
use framework_graph::GraphHost;
use infinite_cavas as cavas;
use semio_framework_core::{CommandDescriptor, UiComponentSceneNode};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use ui_wgpu::{GpuContext, HitKind, HitTarget, Rect, Theme};
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

struct EngineSurface {
    node_graph: Option<NodeGraphEngine>,
    editor: Option<EditorHost>,
    vello: Renderer,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    width: u32,
    height: u32,
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

fn graph_scene_json(graph: &semio_framework_core::NodeGraphScene) -> String {
    serde_json::to_string(graph).unwrap_or_else(|_| "{}".into())
}

fn editor_scene_json(editor: &semio_framework_core::TextEditorScene) -> String {
    serde_json::to_string(editor).unwrap_or_else(|_| "{}".into())
}

fn sync_flow_host(host: &mut FlowHost, graph: &semio_framework_core::NodeGraphScene) {
    if let Some(fixture_json) = &graph.fixture_json {
        if let Ok(fixture) = FlowHost::parse_fixture_json(fixture_json) {
            host.replace_fixture(fixture);
        }
    }
    if let Some(json) = &graph.catalogue_json {
        host.set_host_catalogue_json(json);
    }
    if let Some(json) = &graph.selection_json {
        host.set_selection_json(json);
    }
    if let Some(json) = &graph.preview_off_json {
        host.set_preview_off_json(json);
    }
    if let Some(json) = &graph.computing_json {
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
    if let Some(json) = &graph.lod_json {
        if let Ok(value) = serde_json::from_str::<Value>(json) {
            if let Some(automatic) = value.get("automatic").and_then(|v| v.as_bool()) {
                host.set_automatic_lod(automatic);
            }
            if let Some(label) = value.get("lod").and_then(|v| v.as_str()) {
                host.set_forced_draw_lod_label(label);
            }
        }
    }
    if let Ok(viewport) = serde_json::from_str::<Value>(&graph.viewport_json) {
        let x = viewport.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let y = viewport.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let zoom = viewport.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0);
        host.set_camera(x, y, zoom);
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
    let mut cavas_scene = cavas::Scene::new();
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(&scene.surface_id).expect("engine surface");
        if flow {
            let engine = match entry.node_graph.as_mut() {
                Some(NodeGraphEngine::Flow(host)) => host,
                _ => {
                    entry.node_graph = Some(NodeGraphEngine::Flow(FlowHost::default()));
                    match entry.node_graph.as_mut() {
                        Some(NodeGraphEngine::Flow(host)) => host,
                        _ => return,
                    }
                }
            };
            sync_flow_host(engine, graph);
            engine.set_viewport(pw, ph, dpr);
            engine.paint_scene(&mut cavas_scene, pw, ph, dpr);
        } else {
            let engine = match entry.node_graph.as_mut() {
                Some(NodeGraphEngine::Dag(host)) => host,
                _ => {
                    entry.node_graph = Some(NodeGraphEngine::Dag(GraphHost::default()));
                    match entry.node_graph.as_mut() {
                        Some(NodeGraphEngine::Dag(host)) => host,
                        _ => return,
                    }
                }
            };
            let _ = engine.sync_from_scene_json(&scene_json);
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
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    delta: f32,
    ctrl: bool,
) -> Vec<CommandDescriptor> {
    let Some(graph) = &scene.node_graph else {
        return Vec::new();
    };
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let flow = is_flow_graph(graph);
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) if flow => {
                host.wheel_screen(sx, sy, 0.0, delta as f64, ctrl);
            }
            Some(NodeGraphEngine::Dag(host)) if !flow => {
                host.wheel_screen(sx, sy, delta as f64, true);
            }
            _ => return Vec::new(),
        }
        graph_interaction_commands(scene, entry)
    })
}

pub fn node_graph_pointer_down(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    button: i16,
    shift: bool,
    ctrl: bool,
    alt: bool,
) -> Vec<CommandDescriptor> {
    let Some(graph) = &scene.node_graph else {
        return Vec::new();
    };
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let flow = is_flow_graph(graph);
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) if flow => {
                host.pointer_down_screen(sx, sy, button as u8, shift, ctrl, alt, button == 1);
            }
            Some(NodeGraphEngine::Dag(host)) if !flow => {
                host.pointer_down_screen(sx, sy, button as u8, shift, ctrl, alt);
            }
            _ => return Vec::new(),
        }
        graph_interaction_commands(scene, entry)
    })
}

pub fn node_graph_pointer_move(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl: bool,
    alt: bool,
) -> Vec<CommandDescriptor> {
    let Some(graph) = &scene.node_graph else {
        return Vec::new();
    };
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let flow = is_flow_graph(graph);
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) if flow => {
                host.pointer_move_screen(sx, sy, shift, ctrl, alt);
            }
            Some(NodeGraphEngine::Dag(host)) if !flow => {
                host.pointer_move_screen(sx, sy, shift, ctrl, alt);
            }
            _ => return Vec::new(),
        }
        graph_interaction_commands(scene, entry)
    })
}

pub fn node_graph_pointer_up(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl: bool,
    alt: bool,
) -> Vec<CommandDescriptor> {
    let Some(graph) = &scene.node_graph else {
        return Vec::new();
    };
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let flow = is_flow_graph(graph);
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) if flow => {
                host.pointer_up_screen(sx, sy, shift, ctrl, alt);
            }
            Some(NodeGraphEngine::Dag(host)) if !flow => {
                host.pointer_up_screen(sx, sy, shift, ctrl, alt);
            }
            _ => return Vec::new(),
        }
        graph_interaction_commands(scene, entry)
    })
}

fn graph_interaction_commands(
    scene: &UiComponentSceneNode,
    entry: &EngineSurface,
) -> Vec<CommandDescriptor> {
    let (selection_json, hover_json, viewport_json) = match entry.node_graph.as_ref() {
        Some(NodeGraphEngine::Flow(host)) => (
            host.selected_widget_ids_json(),
            host.hovered_widget_id()
                .map(|id| json!({ "nodeId": id }).to_string())
                .unwrap_or_else(|| "null".into()),
            serde_json::to_string(&host.dag.fixture.camera).unwrap_or_else(|_| "{}".into()),
        ),
        Some(NodeGraphEngine::Dag(host)) => (
            host.selected_node_ids_json(),
            host.hovered_node_id()
                .map(|id| json!({ "nodeId": id }).to_string())
                .unwrap_or_else(|| "null".into()),
            host.camera_json(),
        ),
        None => return Vec::new(),
    };
    vec![
        scene_cmd(
            scene,
            "nodeGraphSelect",
            json!({ "surfaceId": scene.surface_id, "selectionJson": selection_json }),
        ),
        scene_cmd(
            scene,
            "nodeGraphHover",
            json!({ "surfaceId": scene.surface_id, "hoverJson": hover_json }),
        ),
        scene_cmd(
            scene,
            "nodeGraphViewport",
            json!({ "surfaceId": scene.surface_id, "viewportJson": viewport_json }),
        ),
    ]
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
