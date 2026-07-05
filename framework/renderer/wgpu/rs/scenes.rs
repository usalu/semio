//! 🎬 Native component scene hosts for canvas-2d, tables, graphs, and 3D views.

use crate::interpreter::FrameworkWidgetContext;
use crate::world3d::{render_world_3d, World3dState};
use base64::Engine;
use semio_framework_core::UiComponentSceneNode;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use ui_wgpu::{draw_text, Rect, Rgba};

pub fn render_component_scene(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    world3d_states: &mut HashMap<String, World3dState>,
) {
    let theme = ctx.theme;
    ctx.draw.push_rounded(
        [bounds.x, bounds.y, bounds.w, bounds.h],
        theme.panel,
        theme.border_radius,
    );
    match scene.component_kind.as_str() {
        "raster" => render_raster(scene, bounds, ctx),
        "table" => render_table(scene, bounds, ctx),
        "canvas-2d" => render_canvas_2d(scene, bounds, ctx),
        "node-graph" => render_node_graph(scene, bounds, ctx),
        "flow-canvas" => render_flow_canvas(scene, bounds, ctx),
        "virtualFileSystem" => render_vfs(scene, bounds, ctx),
        "text-editor" => render_text_editor(scene, bounds, ctx),
        "world-3d" => {
            let state = world3d_states
                .entry(scene.surface_id.clone())
                .or_insert_with(|| World3dState::new(scene.surface_id.clone(), scene.controller_id.clone()));
            render_world_3d(scene, bounds, ctx, state, gpu);
        }
        _ => render_placeholder(&scene.component_kind, bounds, ctx),
    }
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

fn render_raster(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(raster) = &scene.raster else {
        return render_placeholder("raster", bounds, ctx);
    };
    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&raster.pixels_base64) {
        let inner = bounds.inset(8.0);
        ctx.draw.push_solid(
            [inner.x, inner.y, inner.w, inner.h],
            Rgba::new(0.2, 0.2, 0.22, 1.0),
        );
        draw_text(
            ctx,
            &format!("{}×{} raster", raster.width, raster.height),
            inner.x + 8.0,
            inner.y + 20.0,
            theme.font_size_small,
            theme.text_muted,
        );
        let _ = bytes;
    }
}

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
    let inner = bounds.inset(8.0);
    let col_w = if columns.is_empty() {
        inner.w
    } else {
        inner.w / columns.len() as f32
    };
    for (index, column) in columns.iter().enumerate() {
        let x = inner.x + index as f32 * col_w;
        draw_text(ctx, &column.label, x + 4.0, inner.y + 16.0, theme.font_size_small, theme.text_muted);
        ctx.draw
            .push_line(x, inner.y + 22.0, x, inner.y + inner.h, theme.separator, 1.0);
    }
    ctx.draw.push_line(
        inner.x,
        inner.y + 24.0,
        inner.x + inner.w,
        inner.y + 24.0,
        theme.separator,
        1.0,
    );
    for (row_index, row) in rows.iter().enumerate() {
        let y = inner.y + 28.0 + row_index as f32 * 20.0;
        for (col_index, column) in columns.iter().enumerate() {
            let x = inner.x + col_index as f32 * col_w;
            let value = row.get(&column.id).and_then(|v| v.as_str()).unwrap_or("—");
            draw_text(ctx, value, x + 4.0, y + 14.0, theme.font_size_small, theme.text);
        }
    }
}

#[derive(Deserialize)]
struct CanvasLayer {
    id: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn render_canvas_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(canvas) = &scene.canvas_2d else {
        return render_placeholder("canvas-2d", bounds, ctx);
    };
    let layers: Vec<CanvasLayer> = serde_json::from_str(&canvas.layers_json).unwrap_or_default();
    let inner = bounds.inset(8.0);
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let zoom = canvas.zoom as f32;
    let cam_x = canvas.camera_x as f32;
    let cam_y = canvas.camera_y as f32;
    for layer in &layers {
        let x = inner.x + (layer.x as f32 - cam_x) * zoom + inner.w * 0.5;
        let y = inner.y + (layer.y as f32 - cam_y) * zoom + inner.h * 0.5;
        let w = layer.width as f32 * zoom;
        let h = layer.height as f32 * zoom;
        ctx.draw
            .push_rounded([x, y, w.max(8.0), h.max(8.0)], Rgba::new(0.25, 0.35, 0.55, 0.8), 4.0);
        draw_text(ctx, &layer.name, x + 4.0, y + 14.0, theme.font_size_small, theme.text);
    }
}

#[derive(Deserialize)]
struct GraphNode {
    id: String,
    label: String,
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
struct GraphEdge {
    source: String,
    target: String,
}

fn render_node_graph(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(graph) = &scene.node_graph else {
        return render_placeholder("node-graph", bounds, ctx);
    };
    let nodes: Vec<GraphNode> = serde_json::from_str(&graph.nodes_json).unwrap_or_default();
    let edges: Vec<GraphEdge> = serde_json::from_str(&graph.edges_json).unwrap_or_default();
    let inner = bounds.inset(8.0);
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let node_map: std::collections::HashMap<&str, &GraphNode> =
        nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    for edge in &edges {
        if let (Some(src), Some(dst)) = (node_map.get(edge.source.as_str()), node_map.get(edge.target.as_str())) {
            let x0 = inner.x + src.x as f32 + 60.0;
            let y0 = inner.y + src.y as f32 + 20.0;
            let x1 = inner.x + dst.x as f32 + 60.0;
            let y1 = inner.y + dst.y as f32 + 20.0;
            ctx.draw.push_line(x0, y0, x1, y1, theme.accent, 2.0);
        }
    }
    for node in &nodes {
        let x = inner.x + node.x as f32;
        let y = inner.y + node.y as f32;
        ctx.draw
            .push_rounded([x, y, 120.0, 40.0], theme.button, theme.border_radius);
        draw_text(ctx, &node.label, x + 8.0, y + 24.0, theme.font_size_small, theme.text);
    }
}

#[derive(Deserialize)]
struct FlowWidget {
    id: String,
    kind: String,
}

#[derive(Deserialize)]
struct FlowFixture {
    widgets: Vec<FlowWidget>,
    layout: std::collections::HashMap<String, Value>,
}

fn render_flow_canvas(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(flow) = &scene.flow_canvas else {
        return render_placeholder("flow-canvas", bounds, ctx);
    };
    let fixture: FlowFixture = serde_json::from_str(&flow.fixture_json).unwrap_or(FlowFixture {
        widgets: vec![],
        layout: std::collections::HashMap::new(),
    });
    let inner = bounds.inset(8.0);
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    for widget in &fixture.widgets {
        let (x, y) = fixture
            .layout
            .get(&widget.id)
            .and_then(|pos| {
                let x = pos.get("x")?.as_f64()? as f32;
                let y = pos.get("y")?.as_f64()? as f32;
                Some((x, y))
            })
            .unwrap_or((40.0, 40.0));
        let px = inner.x + x;
        let py = inner.y + y;
        ctx.draw
            .push_rounded([px, py, 100.0, 36.0], theme.button, theme.border_radius);
        draw_text(ctx, &widget.kind, px + 8.0, py + 22.0, theme.font_size_small, theme.text);
    }
}

#[derive(Deserialize)]
struct VfsRow {
    id: String,
    name: String,
    level: u32,
}

fn render_vfs(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(vfs) = &scene.virtual_file_system else {
        return render_placeholder("virtualFileSystem", bounds, ctx);
    };
    let rows: Vec<VfsRow> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
    let inner = bounds.inset(8.0);
    for (index, row) in rows.iter().enumerate() {
        let y = inner.y + index as f32 * 20.0 + 16.0;
        draw_text(
            ctx,
            &row.name,
            inner.x + row.level as f32 * 14.0 + 8.0,
            y,
            theme.font_size_small,
            theme.text,
        );
    }
}

fn render_text_editor(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(editor) = &scene.text_editor else {
        return render_placeholder("text-editor", bounds, ctx);
    };
    let inner = bounds.inset(8.0);
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.input_bg);
    let lines: Vec<&str> = editor.buffer.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        draw_text(
            ctx,
            line,
            inner.x + 8.0,
            inner.y + 16.0 + index as f32 * 18.0,
            theme.font_size_small,
            theme.text,
        );
    }
}

