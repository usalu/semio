//! 🪟 Mode dock — multi-window layout tree with stack chrome and split resize.

use semio_framework_core::{
    layout::{
        WindowLayout, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode,
        WindowLayoutWindowNode,
    },
    AppDefinition, CommandDescriptor,
};
use ui_wgpu::{
    draw_text, DrawList, DragAxis, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Rect,
    Rgba, Theme,
};

pub type DockPath = Vec<usize>;

//#region DockTypes
#[derive(Clone, Debug, PartialEq)]
pub enum DockNode {
    Row(Vec<(DockNode, f32)>),
    Column(Vec<(DockNode, f32)>),
    Stack { windows: Vec<String>, active: String },
}

impl Default for DockState {
    fn default() -> Self {
        Self {
            root: DockNode::Stack {
                windows: vec![],
                active: String::new(),
            },
            active_window_id: None,
            maximized_stack: None,
            active_stack: None,
            split_resize_origin: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct DockState {
    pub root: DockNode,
    pub active_window_id: Option<String>,
    pub maximized_stack: Option<DockPath>,
    pub active_stack: Option<DockPath>,
    pub split_resize_origin: Vec<f32>,
}

pub struct DockRenderContext<'a, 'b> {
    pub draw: &'a mut DrawList,
    pub atlas: &'a mut FontAtlas,
    pub icons: &'a IconAtlas,
    pub input: &'a mut InputState<CommandDescriptor>,
    pub theme: &'a Theme,
    pub window_labels: &'a std::collections::HashMap<String, String>,
}
//#endregion DockTypes

//#region DockLayout
impl DockState {
    pub fn from_app(app: &AppDefinition, active_window_id: Option<&str>) -> Self {
        let root = app
            .default_layout
            .as_ref()
            .map(|layout| dock_from_window_layout(&layout.root))
            .unwrap_or_else(|| even_layout(&app.window_kinds.iter().map(|k| k.id.clone()).collect::<Vec<_>>()));
        let active = active_window_id
            .map(str::to_string)
            .or_else(|| first_window_id(&root));
        let active_stack = active.as_ref().and_then(|id| find_stack_path(&root, id, &mut vec![]));
        Self {
            root,
            active_window_id: active,
            maximized_stack: None,
            active_stack,
            split_resize_origin: vec![],
        }
    }

    pub fn sync_active_window(&mut self, window_id: &str) {
        self.active_window_id = Some(window_id.to_string());
        self.active_stack = find_stack_path(&self.root, window_id, &mut vec![]);
    }

    pub fn set_stack_active(&mut self, path: &DockPath, window_id: &str) {
        if let Some(stack) = node_at_mut(&mut self.root, path) {
            if let DockNode::Stack { active, .. } = stack {
                *active = window_id.to_string();
            }
        }
        self.active_window_id = Some(window_id.to_string());
        self.active_stack = Some(path.clone());
    }

    pub fn toggle_maximize(&mut self, path: &DockPath) {
        if self.maximized_stack.as_ref() == Some(path) {
            self.maximized_stack = None;
        } else {
            self.maximized_stack = Some(path.clone());
        }
    }

    pub fn close_active_in_stack(&mut self, path: &DockPath) -> bool {
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows, active } = stack else {
            return false;
        };
        if windows.len() <= 1 {
            return false;
        }
        let idx = windows.iter().position(|id| id == active).unwrap_or(0);
        windows.remove(idx);
        let next = windows.get(idx.saturating_sub(1)).or_else(|| windows.first()).cloned();
        if let Some(id) = next {
            *active = id.clone();
            self.active_window_id = Some(id);
        }
        collapse_empty(&mut self.root);
        if self.maximized_stack.as_ref() == Some(path) && !path_exists(&self.root, path) {
            self.maximized_stack = None;
        }
        true
    }

    pub fn apply_split_drag(&mut self, path: &DockPath, split_index: usize, delta_px: f32, axis_total: f32) {
        let Some(node) = node_at_mut(&mut self.root, path) else {
            return;
        };
        let children = match node {
            DockNode::Row(children) | DockNode::Column(children) => children,
            DockNode::Stack { .. } => return,
        };
        if split_index + 1 >= children.len() || axis_total <= 0.0 {
            return;
        }
        let delta_frac = delta_px / axis_total;
        let left = &mut children[split_index].1;
        let right = &mut children[split_index + 1].1;
        let origin_left = self.split_resize_origin.get(split_index).copied().unwrap_or(*left);
        let origin_right = self
            .split_resize_origin
            .get(split_index + 1)
            .copied()
            .unwrap_or(*right);
        let new_left = (origin_left + delta_frac).clamp(0.08, 0.92);
        let new_right = (origin_right - delta_frac).clamp(0.08, 0.92);
        *left = new_left;
        *right = new_right;
        normalize_pair_sizes(children, split_index);
    }

    pub fn begin_split_drag(&mut self, path: &DockPath) {
        let Some(node) = node_at(&self.root, path) else {
            self.split_resize_origin.clear();
            return;
        };
        self.split_resize_origin = match node {
            DockNode::Row(children) | DockNode::Column(children) => {
                children.iter().map(|(_, s)| *s).collect()
            }
            DockNode::Stack { .. } => vec![],
        };
    }

    pub fn stack_body_rects(&self, bounds: Rect, theme: &Theme) -> Vec<(DockPath, Rect, String)> {
        let canvas = bounds.inset(theme.gap_standard);
        let mut out = Vec::new();
        if let Some(path) = &self.maximized_stack {
            if let Some((node, rect)) = solve_node_rect(&self.root, canvas, path, &[]) {
                if let DockNode::Stack { active, .. } = node {
                    out.push((
                        path.clone(),
                        stack_content_rect(rect, theme),
                        active.clone(),
                    ));
                }
            }
            return out;
        }
        collect_stack_bodies(&self.root, canvas, &[], theme, &mut out);
        out
    }

    pub fn register_hits(
        &self,
        ctx: &mut DockRenderContext<'_, '_>,
        bounds: Rect,
    ) {
        let canvas = bounds.inset(ctx.theme.gap_standard);
        ctx.draw.push_solid([canvas.x, canvas.y, canvas.w, canvas.h], ctx.theme.canvas_clear);
        if let Some(path) = &self.maximized_stack {
            if let Some((node, rect)) = solve_node_rect(&self.root, canvas, path, &[]) {
                if let DockNode::Stack { .. } = node {
                    render_stack(self, ctx, path, node, rect, true, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(self, ctx, &self.root, canvas, &[], &mut |_, _| {});
    }
}
//#endregion DockLayout

pub fn dock_from_window_layout(root: &WindowLayoutRoot) -> DockNode {
    match root {
        WindowLayoutRoot::Axis(axis) => axis_from_children(&axis.kind, &axis.children, axis.size),
        WindowLayoutRoot::Stack(stack) => stack_from_node(stack),
    }
}

fn axis_from_children(kind: &str, children: &[WindowLayoutChild], size: Option<f64>) -> DockNode {
    let parsed: Vec<(DockNode, f32)> = children
        .iter()
        .map(|child| match child {
            WindowLayoutChild::Axis(axis) => (
                axis_from_children(&axis.kind, &axis.children, axis.size),
                axis.size.map(|v| v as f32).unwrap_or(1.0),
            ),
            WindowLayoutChild::Stack(stack) => (stack_from_node(stack), stack.size.map(|v| v as f32).unwrap_or(1.0)),
        })
        .collect();
    let normalized = normalize_sizes(parsed, size.map(|v| v as f32));
    if kind == "column" {
        DockNode::Column(normalized)
    } else {
        DockNode::Row(normalized)
    }
}

fn stack_from_node(stack: &WindowLayoutStackNode) -> DockNode {
    let windows: Vec<String> = stack
        .children
        .iter()
        .map(|w| w.window_kind_id.clone())
        .collect();
    let active = windows.first().cloned().unwrap_or_default();
    DockNode::Stack { windows, active }
}

fn even_layout(window_ids: &[String]) -> DockNode {
    if window_ids.is_empty() {
        return DockNode::Stack {
            windows: vec![],
            active: String::new(),
        };
    }
    if window_ids.len() == 1 {
        return DockNode::Stack {
            windows: vec![window_ids[0].clone()],
            active: window_ids[0].clone(),
        };
    }
    let count = window_ids.len() as f32;
    DockNode::Row(
        window_ids
            .iter()
            .map(|id| {
                (
                    DockNode::Stack {
                        windows: vec![id.clone()],
                        active: id.clone(),
                    },
                    1.0 / count,
                )
            })
            .collect(),
    )
}

fn normalize_sizes(children: Vec<(DockNode, f32)>, axis_size: Option<f32>) -> Vec<(DockNode, f32)> {
    if children.is_empty() {
        return children;
    }
    let total: f32 = children.iter().map(|(_, s)| *s).sum();
    let scale = axis_size.unwrap_or(total).max(0.001);
    let sum = total.max(0.001);
    children
        .into_iter()
        .map(|(node, size)| (node, size / sum * scale))
        .collect()
}

fn normalize_pair_sizes(children: &mut [(DockNode, f32)], index: usize) {
    let pair_sum = children[index].1 + children[index + 1].1;
    let total: f32 = children.iter().map(|(_, s)| *s).sum();
    if total <= 0.0 {
        return;
    }
    let a = children[index].1 / pair_sum;
    let b = children[index + 1].1 / pair_sum;
    children[index].1 = a * pair_sum;
    children[index + 1].1 = b * pair_sum;
}

fn first_window_id(node: &DockNode) -> Option<String> {
    match node {
        DockNode::Stack { active, .. } if !active.is_empty() => Some(active.clone()),
        DockNode::Row(children) | DockNode::Column(children) => {
            children.iter().find_map(|(child, _)| first_window_id(child))
        }
    }
}

fn find_stack_path(node: &DockNode, window_id: &str, path: &mut DockPath) -> Option<DockPath> {
    match node {
        DockNode::Stack { windows, .. } if windows.iter().any(|id| id == window_id) => Some(path.clone()),
        DockNode::Row(children) | DockNode::Column(children) => {
            for (index, (child, _)) in children.iter().enumerate() {
                path.push(index);
                if let Some(found) = find_stack_path(child, window_id, path) {
                    return Some(found);
                }
                path.pop();
            }
            None
        }
    }
}

fn path_exists(node: &DockNode, path: &DockPath) -> bool {
    node_at(node, path).is_some()
}

fn node_at<'a>(node: &'a DockNode, path: &DockPath) -> Option<&'a DockNode> {
    let mut current = node;
    for index in path {
        current = match current {
            DockNode::Row(children) | DockNode::Column(children) => children.get(*index).map(|(n, _)| n)?,
            DockNode::Stack { .. } => return None,
        };
    }
    Some(current)
}

fn node_at_mut<'a>(node: &'a mut DockNode, path: &DockPath) -> Option<&'a mut DockNode> {
    if path.is_empty() {
        return Some(node);
    }
    let (head, tail) = path.split_first()?;
    let child = match node {
        DockNode::Row(children) | DockNode::Column(children) => children.get_mut(*head).map(|(n, _)| n)?,
        DockNode::Stack { .. } => return None,
    };
    node_at_mut(child, tail)
}

fn collapse_empty(node: &mut DockNode) {
    match node {
        DockNode::Row(children) | DockNode::Column(children) => {
            children.retain(|(child, _)| !is_empty_stack(child));
            for (child, _) in children.iter_mut() {
                collapse_empty(child);
            }
            children.retain(|(child, _)| !is_empty_node(child));
        }
        DockNode::Stack { .. } => {}
    }
}

fn is_empty_stack(node: &DockNode) -> bool {
    matches!(node, DockNode::Stack { windows, .. } if windows.is_empty())
}

fn is_empty_node(node: &DockNode) -> bool {
    match node {
        DockNode::Stack { windows, .. } => windows.is_empty(),
        DockNode::Row(children) | DockNode::Column(children) => children.is_empty(),
    }
}

fn solve_node_rect(
    node: &DockNode,
    bounds: Rect,
    target_path: &DockPath,
    current_path: &DockPath,
) -> Option<(&DockNode, Rect)> {
    if current_path == target_path {
        return Some((node, bounds));
    }
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut path = current_path.to_vec();
                path.push(index);
                if let Some(found) = solve_node_rect(child, Rect::new(x, bounds.y, w, bounds.h), target_path, &path) {
                    return Some(found);
                }
                x += w;
            }
            None
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut path = current_path.to_vec();
                path.push(index);
                if let Some(found) = solve_node_rect(child, Rect::new(bounds.x, y, bounds.w, h), target_path, &path) {
                    return Some(found);
                }
                y += h;
            }
            None
        }
        DockNode::Stack { .. } => None,
    }
}

fn render_node(
    state: &DockState,
    ctx: &mut DockRenderContext<'_, '_>,
    node: &DockNode,
    bounds: Rect,
    path: &DockPath,
    render_body: &mut dyn FnMut(Rect, &str),
) {
    match node {
        DockNode::Row(children) => render_axis(state, ctx, children, bounds, path, true, render_body),
        DockNode::Column(children) => render_axis(state, ctx, children, bounds, path, false, render_body),
        DockNode::Stack { .. } => render_stack(state, ctx, path, node, bounds, false, render_body),
    }
}

fn render_axis(
    state: &DockState,
    ctx: &mut DockRenderContext<'_, '_>,
    children: &[(DockNode, f32)],
    bounds: Rect,
    path: &DockPath,
    horizontal: bool,
    render_body: &mut dyn FnMut(Rect, &str),
) {
    let total: f32 = children.iter().map(|(_, s)| *s).sum().max(0.001);
    let split_w = 6.0;
    if horizontal {
        let mut x = bounds.x;
        for (index, (child, size)) in children.iter().enumerate() {
            let w = bounds.w * (*size / total);
            let child_rect = Rect::new(x, bounds.y, w, bounds.h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            render_node(state, ctx, child, child_rect, &child_path, render_body);
            x += w;
            if index + 1 < children.len() {
                let handle = Rect::new(x - split_w * 0.5, bounds.y, split_w, bounds.h);
                register_split_hit(ctx, path, index, handle, DragAxis::Horizontal);
            }
        }
    } else {
        let mut y = bounds.y;
        for (index, (child, size)) in children.iter().enumerate() {
            let h = bounds.h * (*size / total);
            let child_rect = Rect::new(bounds.x, y, bounds.w, h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            render_node(state, ctx, child, child_rect, &child_path, render_body);
            y += h;
            if index + 1 < children.len() {
                let handle = Rect::new(bounds.x, y - split_w * 0.5, bounds.w, split_w);
                register_split_hit(ctx, path, index, handle, DragAxis::Vertical);
            }
        }
    }
}

fn register_split_hit(
    ctx: &mut DockRenderContext<'_, '_>,
    path: &DockPath,
    index: usize,
    rect: Rect,
    axis: DragAxis,
) {
    ctx.input.register_hit(HitTarget {
        rect,
        event: None,
        control_id: Some(format!("dock.split.{}.{index}", path_str(path))),
        kind: HitKind::ScrollRegion,
        drag_axis: Some(axis),
    });
}

fn render_stack(
    state: &DockState,
    ctx: &mut DockRenderContext<'_, '_>,
    path: &DockPath,
    node: &DockNode,
    bounds: Rect,
    maximized: bool,
    render_body: &mut dyn FnMut(Rect, &str),
) {
    let DockNode::Stack { windows, active } = node else {
        return;
    };
    if windows.is_empty() {
        return;
    }
    let theme = ctx.theme;
    let tab_h = theme.control_height;
    let stroke = theme.stroke_hairline;
    let globally_active = state.active_stack.as_ref() == Some(path);
    let border = if globally_active {
        theme.accent
    } else {
        theme.border_normal
    };
    let cap_y = bounds.y;
    let cap_rect = Rect::new(bounds.x, cap_y, bounds.w, tab_h);
    ctx.draw.push_solid([cap_rect.x, cap_rect.y, cap_rect.w, cap_rect.h], theme.panel);
    ctx.draw
        .push_solid([cap_rect.x, cap_rect.y, cap_rect.w, stroke], border);
    ctx.draw
        .push_solid([cap_rect.x, cap_rect.y, stroke, cap_rect.h], border);
    ctx.draw
        .push_solid([cap_rect.x + cap_rect.w - stroke, cap_rect.y, stroke, cap_rect.h], border);

    let controls_w = theme.control_height * 2.0 + theme.gap_standard;
    let mut tab_x = cap_rect.x + theme.gap_standard;
    for window_id in windows {
        let label = ctx
            .window_labels
            .get(window_id)
            .map(String::as_str)
            .unwrap_or(window_id);
        let tw = ctx.atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0;
        let tab_rect = Rect::new(tab_x, cap_y + theme.gap_standard * 0.5, tw, tab_h - theme.gap_standard);
        let is_active = window_id == active;
        let hovered = tab_rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
        let bg = if is_active {
            theme.selected
        } else if hovered {
            theme.button_hover
        } else {
            theme.panel
        };
        ctx.draw.push_solid([tab_rect.x, tab_rect.y, tab_rect.w, tab_rect.h], bg);
        if is_active {
            ctx.draw
                .push_solid([tab_rect.x, tab_rect.y, tab_rect.w, stroke], theme.accent);
            ctx.draw
                .push_solid([tab_rect.x, tab_rect.y, stroke, tab_rect.h], theme.accent);
            ctx.draw
                .push_solid([tab_rect.x + tab_rect.w - stroke, tab_rect.y, stroke, tab_rect.h], theme.accent);
        }
        dock_text(
            ctx,
            label,
            tab_rect.x + theme.padding_standard,
            tab_rect.y + (tab_rect.h + theme.font_size_small) * 0.5 - 1.0,
            theme.font_size_small,
            if is_active || hovered {
                theme.active_foreground
            } else {
                theme.text
            },
        );
        ctx.input.register_hit(HitTarget {
            rect: tab_rect,
            event: None,
            control_id: Some(format!("dock.tab.{}.{}", path_str(path), window_id)),
            kind: HitKind::Window,
            drag_axis: None,
        });
        tab_x += tw + theme.gap_standard * 0.5;
    }
    let gap_x = tab_x;
    let gap_w = (cap_rect.x + cap_rect.w - controls_w - theme.gap_standard - gap_x).max(theme.gap_standard);
    let gap_rect = Rect::new(gap_x, cap_y, gap_w, tab_h);
    ctx.draw.push_solid([gap_rect.x, gap_rect.y, gap_rect.w, gap_rect.h], theme.canvas_clear);
    ctx.draw
        .push_solid([gap_rect.x, gap_rect.y + tab_h - stroke, gap_rect.w, stroke], theme.border_normal);

    let focus_rect = Rect::new(
        cap_rect.x + cap_rect.w - controls_w,
        cap_y + theme.gap_standard * 0.5,
        theme.control_height,
        tab_h - theme.gap_standard,
    );
    let close_rect = Rect::new(
        focus_rect.x + theme.control_height + theme.gap_standard * 0.5,
        focus_rect.y,
        theme.control_height,
        focus_rect.h,
    );
    render_cap_button(ctx, focus_rect, if maximized { "▣" } else { "⛶" }, "dock.focus", path);
    render_cap_button(ctx, close_rect, "×", "dock.close", path);

    let body_y = cap_y + tab_h;
    let body_rect = Rect::new(bounds.x, body_y, bounds.w, bounds.h - tab_h);
    ctx.draw.push_solid([body_rect.x, body_rect.y, body_rect.w, body_rect.h], theme.canvas_clear);
    ctx.draw
        .push_solid([body_rect.x, body_rect.y, stroke, body_rect.h], border);
    ctx.draw
        .push_solid([body_rect.x + body_rect.w - stroke, body_rect.y, stroke, body_rect.h], border);
    ctx.draw
        .push_solid([body_rect.x, body_rect.y + body_rect.h - stroke, body_rect.w, stroke], border);

    let content = body_rect.inset(theme.gap_standard);
    render_body(content, active);
}

fn render_cap_button(ctx: &mut DockRenderContext<'_, '_>, rect: Rect, glyph: &str, prefix: &str, path: &DockPath) {
    let hovered = rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
    let bg = if hovered {
        ctx.theme.button_hover
    } else {
        ctx.theme.button
    };
    ctx.draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
    dock_text(
        ctx,
        glyph,
        rect.x + (rect.w - ctx.theme.font_size_small) * 0.5,
        rect.y + (rect.h + ctx.theme.font_size_small) * 0.5 - 1.0,
        ctx.theme.font_size_small,
        if hovered {
            ctx.theme.active_foreground
        } else {
            ctx.theme.text_muted
        },
    );
    ctx.input.register_hit(HitTarget {
        rect,
        event: None,
        control_id: Some(format!("{prefix}.{}", path_str(path))),
        kind: HitKind::Button,
        drag_axis: None,
    });
}

pub fn path_str(path: &DockPath) -> String {
    path.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn parse_path(value: &str) -> DockPath {
    if value.is_empty() {
        return vec![];
    }
    value
        .split(',')
        .filter_map(|part| part.parse().ok())
        .collect()
}

fn stack_content_rect(bounds: Rect, theme: &Theme) -> Rect {
    let tab_h = theme.control_height;
    let body = Rect::new(bounds.x, bounds.y + tab_h, bounds.w, (bounds.h - tab_h).max(0.0));
    body.inset(theme.gap_standard)
}

fn collect_stack_bodies(
    node: &DockNode,
    bounds: Rect,
    path: &DockPath,
    theme: &Theme,
    out: &mut Vec<(DockPath, Rect, String)>,
) {
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_bodies(child, Rect::new(x, bounds.y, w, bounds.h), &child_path, theme, out);
                x += w;
            }
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_bodies(child, Rect::new(bounds.x, y, bounds.w, h), &child_path, theme, out);
                y += h;
            }
        }
        DockNode::Stack { active, .. } => {
            out.push((path.to_vec(), stack_content_rect(bounds, theme), active.clone()));
        }
    }
}

fn dock_text(
    ctx: &mut DockRenderContext<'_, '_>,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Rgba,
) {
    let mut scroll = std::collections::HashMap::new();
    let mut collapsed = std::collections::HashMap::new();
    let mut selects = std::collections::HashMap::new();
    let mut widget_ctx = crate::interpreter::framework_widget_context(
        ctx.draw,
        None,
        ctx.atlas,
        Some(ctx.icons),
        ctx.input,
        ctx.theme,
        &mut scroll,
        &mut collapsed,
        &mut selects,
    );
    draw_text(&mut widget_ctx, text, x, y, size, color);
}

//#region DockFreeFunctions

//#region DockTests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_core::layout::create_default_layout;
    use semio_framework_core::{AppDefinition, ModeDefinition, PanelTabDefinition, WindowKindDefinition};

    fn sample_app(window_ids: &[&str], layout: Option<WindowLayout>) -> AppDefinition {
        AppDefinition {
            id: "test".into(),
            label: "Test".into(),
            icon_id: None,
            controller_id: "test".into(),
            modes: vec![ModeDefinition {
                id: "default".into(),
                label: "Default".into(),
            }],
            default_mode_id: Some("default".into()),
            window_kinds: window_ids
                .iter()
                .map(|id| WindowKindDefinition {
                    id: (*id).into(),
                    label: (*id).into(),
                    body_key: format!("{id}.body"),
                    icon_id: None,
                    measures: vec![],
                    engagement: None,
                })
                .collect(),
            panel_tabs: vec![PanelTabDefinition {
                id: "tab".into(),
                label: "Tab".into(),
                group: "workbench".into(),
                body_key: "tab.body".into(),
            }],
            keybindings: vec![],
            named_layouts: vec![],
            default_layout: layout,
        }
    }

    #[test]
    fn even_layout_single_window() {
        let node = even_layout(&["main".into()]);
        assert!(matches!(node, DockNode::Stack { .. }));
        let dock = DockState::from_app(&sample_app(&["main"], None), None);
        assert_eq!(dock.active_window_id.as_deref(), Some("main"));
    }

    #[test]
    fn even_layout_multiple_windows() {
        let node = even_layout(&["a".into(), "b".into(), "c".into()]);
        assert!(matches!(node, DockNode::Row(_)));
        if let DockNode::Row(children) = node {
            assert_eq!(children.len(), 3);
            for (child, size) in children {
                assert!(matches!(child, DockNode::Stack { .. }));
                assert!((size - 1.0 / 3.0).abs() < 0.001);
            }
        }
    }

    #[test]
    fn parses_default_layout_row() {
        let layout = create_default_layout(&["a".into(), "b".into()], "row", None, None);
        let app = sample_app(&["a", "b"], Some(layout));
        let dock = DockState::from_app(&app, None);
        assert!(matches!(dock.root, DockNode::Row(_)));
    }

    #[test]
    fn close_window_collapses_stack_tabs() {
        let mut dock = DockState::from_app(
            &sample_app(&["a", "b"], None),
            Some("a"),
        );
        dock.root = DockNode::Stack {
            windows: vec!["a".into(), "b".into()],
            active: "a".into(),
        };
        let path = vec![];
        dock.close_active_in_stack(&path);
        if let DockNode::Stack { windows, active } = &dock.root {
            assert_eq!(windows, &vec!["b".to_string()]);
            assert_eq!(active, "b");
        } else {
            panic!("expected stack");
        }
    }
}
//#endregion DockTests
