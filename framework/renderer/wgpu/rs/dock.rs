//! 🪟 Mode dock — multi-window layout tree with stack chrome and split resize.

use semio_framework_core::{
    layout::{
        WindowLayout, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode,
        WindowLayoutWindowNode,
    },
    AppDefinition, CommandDescriptor,
};
use std::collections::HashMap;
use ui_wgpu::{
    chrome_item_bg, chrome_item_text, draw_text, push_chrome_border, push_chrome_group_border,
    push_window_cap_border, DrawList, DragAxis, FontAtlas, HitKind, HitTarget, IconAtlas,
    InputState, Rect, Rgba, Theme,
};

pub type DockPath = Vec<usize>;

fn empty_path() -> DockPath {
    Vec::new()
}

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

pub struct DockRenderContext<'a> {
    pub draw: &'a mut DrawList,
    pub atlas: &'a mut FontAtlas,
    pub icons: &'a IconAtlas,
    pub input: &'a mut InputState<CommandDescriptor>,
    pub theme: &'a Theme,
    pub window_labels: &'a std::collections::HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DockDropZone {
    Tab { stack_path: DockPath, index: usize },
    Split { stack_path: DockPath, side: DockSide },
    RootSplit { side: DockSide },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DockDragKind {
    Tab,
    Stack,
}

#[derive(Clone, Debug)]
pub struct DockDragPayload {
    pub kind: DockDragKind,
    pub window_id: String,
    pub source_path: DockPath,
    pub tab_index: usize,
    pub ghost_label: String,
}

#[derive(Clone, Debug)]
pub struct DockDragState {
    pub payload: DockDragPayload,
    pub x: f32,
    pub y: f32,
    pub drop_zone: Option<DockDropZone>,
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
        Self::apply_split_drag_on_node(
            &mut self.root,
            path,
            split_index,
            delta_px,
            axis_total,
            &self.split_resize_origin,
        );
    }

    pub fn apply_split_drag_with_origin(
        &mut self,
        path: &DockPath,
        split_index: usize,
        delta_px: f32,
        axis_total: f32,
        origin: &[f32],
    ) {
        Self::apply_split_drag_on_node(&mut self.root, path, split_index, delta_px, axis_total, origin);
    }

    fn apply_split_drag_on_node(
        root: &mut DockNode,
        path: &DockPath,
        split_index: usize,
        delta_px: f32,
        axis_total: f32,
        origin: &[f32],
    ) {
        let Some(node) = node_at_mut(root, path) else {
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
        let origin_left = origin.get(split_index).copied().unwrap_or(children[split_index].1);
        let origin_right = origin
            .get(split_index + 1)
            .copied()
            .unwrap_or(children[split_index + 1].1);
        let new_left = (origin_left + delta_frac).clamp(0.08, 0.92);
        let new_right = (origin_right - delta_frac).clamp(0.08, 0.92);
        children[split_index].1 = new_left;
        children[split_index + 1].1 = new_right;
        normalize_pair_sizes(children, split_index);
    }

    pub fn begin_split_drag(&mut self, path: &DockPath) -> Vec<f32> {
        let sizes = match node_at(&self.root, path) {
            Some(DockNode::Row(children) | DockNode::Column(children)) => {
                children.iter().map(|(_, s)| *s).collect()
            }
            _ => vec![],
        };
        self.split_resize_origin = sizes.clone();
        sizes
    }

    pub fn stack_body_rects(
        &self,
        bounds: Rect,
        theme: &Theme,
        window_labels: &HashMap<String, String>,
        atlas: &mut FontAtlas,
    ) -> Vec<(DockPath, Rect, String)> {
        let mut out = Vec::new();
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                let rect = bounds;
                if let DockNode::Stack { windows, active } = node {
                    out.push((
                        path.clone(),
                        stack_content_rect(
                            rect,
                            theme,
                            windows,
                            active,
                            window_labels,
                            atlas,
                            true,
                        ),
                        active.clone(),
                    ));
                }
            }
            return out;
        }
        collect_stack_bodies(
            &self.root,
            bounds,
            &empty_path(),
            theme,
            window_labels,
            atlas,
            self,
            &mut out,
        );
        out
    }

    pub fn stack_tab_bar_rects(&self, bounds: Rect, theme: &Theme) -> Vec<(DockPath, Rect)> {
        let mut out = Vec::new();
        if self.maximized_stack.is_some() {
            if let Some(path) = &self.maximized_stack {
                let rect = bounds;
                out.push((path.clone(), stack_tab_bar_rect(rect, theme)));
            }
            return out;
        }
        collect_stack_tab_bars(&self.root, bounds, &empty_path(), theme, &mut out);
        out
    }

    pub fn tab_index(&self, path: &DockPath, window_id: &str) -> Option<usize> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        windows.iter().position(|id| id == window_id)
    }

    pub fn apply_drop(&mut self, drag: &DockDragPayload, zone: &DockDropZone) -> bool {
        match zone {
            DockDropZone::Tab {
                stack_path,
                index,
            } => {
                if drag.source_path == *stack_path {
                    return self.reorder_tab(&drag.source_path, drag.tab_index, *index);
                }
                if !self.remove_window(&drag.window_id) {
                    return false;
                }
                self.insert_tab(stack_path, &drag.window_id, Some(*index))
            }
            DockDropZone::Split { stack_path, side } => {
                if !self.remove_window(&drag.window_id) {
                    return false;
                }
                self.split_stack_with_window(stack_path, &drag.window_id, *side)
            }
            DockDropZone::RootSplit { side } => {
                if !self.remove_window(&drag.window_id) {
                    return false;
                }
                self.split_root_with_window(&drag.window_id, *side)
            }
        }
    }

    pub fn reorder_tab(&mut self, path: &DockPath, from: usize, to: usize) -> bool {
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows, .. } = stack else {
            return false;
        };
        if from >= windows.len() || to > windows.len() || from == to {
            return false;
        }
        let window_id = windows.remove(from);
        let insert_at = if to > from { to.saturating_sub(1) } else { to };
        windows.insert(insert_at.min(windows.len()), window_id);
        true
    }

    pub fn remove_window(&mut self, window_id: &str) -> bool {
        let removed = remove_window_from_node(&mut self.root, window_id);
        if removed {
            collapse_empty(&mut self.root);
            if self
                .active_window_id
                .as_deref()
                .is_some_and(|id| id == window_id)
            {
                self.active_window_id = first_window_id(&self.root);
            }
        }
        removed
    }

    pub fn insert_tab(&mut self, path: &DockPath, window_id: &str, index: Option<usize>) -> bool {
        let Some(stack) = node_at_mut(&mut self.root, path) else {
            return false;
        };
        let DockNode::Stack { windows, active } = stack else {
            return false;
        };
        if windows.iter().any(|id| id == window_id) {
            return false;
        }
        let insert_at = index.unwrap_or(windows.len()).min(windows.len());
        windows.insert(insert_at, window_id.to_string());
        *active = window_id.to_string();
        self.active_window_id = Some(window_id.to_string());
        self.active_stack = Some(path.to_vec());
        true
    }

    pub fn split_stack_with_window(&mut self, path: &DockPath, window_id: &str, side: DockSide) -> bool {
        let Some(stack_node) = node_at(&self.root, path).cloned() else {
            return false;
        };
        let DockNode::Stack { .. } = stack_node else {
            return false;
        };
        let new_stack = DockNode::Stack {
            windows: vec![window_id.to_string()],
            active: window_id.to_string(),
        };
        let replacement = match side {
            DockSide::Left | DockSide::Top => axis_pair_from_stacks(&new_stack, &stack_node, side),
            DockSide::Right | DockSide::Bottom => axis_pair_from_stacks(&stack_node, &new_stack, side),
        };
        replace_node_at(&mut self.root, path, replacement);
        self.active_window_id = Some(window_id.to_string());
        self.active_stack = find_stack_path(&self.root, window_id, &mut vec![]);
        true
    }

    pub fn split_root_with_window(&mut self, window_id: &str, side: DockSide) -> bool {
        let current = std::mem::replace(
            &mut self.root,
            DockNode::Stack {
                windows: vec![],
                active: String::new(),
            },
        );
        if matches!(&current, DockNode::Stack { windows, .. } if windows.is_empty()) {
            self.root = DockNode::Stack {
                windows: vec![window_id.to_string()],
                active: window_id.to_string(),
            };
        } else {
            let new_stack = DockNode::Stack {
                windows: vec![window_id.to_string()],
                active: window_id.to_string(),
            };
            self.root = match side {
                DockSide::Left | DockSide::Top => axis_pair_from_stacks(&new_stack, &current, side),
                DockSide::Right | DockSide::Bottom => axis_pair_from_stacks(&current, &new_stack, side),
            };
        }
        self.active_window_id = Some(window_id.to_string());
        self.active_stack = find_stack_path(&self.root, window_id, &mut vec![]);
        true
    }

    pub fn stack_windows_at_path(&self, path: &DockPath) -> Option<Vec<String>> {
        let DockNode::Stack { windows, .. } = node_at(&self.root, path)? else {
            return None;
        };
        Some(windows.clone())
    }

    pub fn to_window_layout(&self) -> WindowLayout {
        WindowLayout {
            root: dock_node_to_layout_root(&self.root),
        }
    }

    pub fn register_hits(
        &self,
        ctx: &mut DockRenderContext<'_>,
        bounds: Rect,
    ) {
        ctx.draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.canvas_clear);
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                let rect = bounds;
                if let DockNode::Stack { .. } = node {
                    render_stack(self, ctx, path, node, rect, true, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(self, ctx, &self.root, bounds, &empty_path(), &mut |_, _| {}, None);
    }

    pub fn register_resize_hits(&self, ctx: &mut DockRenderContext<'_>, bounds: Rect) {
        if self.maximized_stack.is_some() {
            return;
        }
        walk_resize_hits(self, ctx, &self.root, bounds, &empty_path(), None);
    }

    pub fn split_axis_extent(&self, path: &DockPath, canvas: Rect) -> Option<f32> {
        let bounds = solve_node_bounds(&self.root, canvas, path, &empty_path())?;
        match node_at(&self.root, path)? {
            DockNode::Row(_) => Some(bounds.w.max(1.0)),
            DockNode::Column(_) => Some(bounds.h.max(1.0)),
            DockNode::Stack { .. } => None,
        }
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
    let active = stack
        .active_window_kind_id
        .clone()
        .filter(|id| windows.iter().any(|w| w == id))
        .or_else(|| windows.first().cloned())
        .unwrap_or_default();
    DockNode::Stack { windows, active }
}

pub fn dock_node_to_layout_root(node: &DockNode) -> WindowLayoutRoot {
    match node {
        DockNode::Stack { windows, active } => {
            WindowLayoutRoot::Stack(WindowLayoutStackNode {
                kind: "stack".into(),
                size: None,
                active_window_kind_id: Some(active.clone()),
                children: windows
                    .iter()
                    .map(|id| WindowLayoutWindowNode {
                        kind: "window".into(),
                        window_kind_id: id.clone(),
                        title: None,
                        instance_id: None,
                        template_id: None,
                    })
                    .collect(),
            })
        }
        DockNode::Row(children) => WindowLayoutRoot::Axis(semio_framework_core::layout::WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: children
                .iter()
                .map(|(child, size)| dock_child_from_node(child, *size))
                .collect(),
        }),
        DockNode::Column(children) => WindowLayoutRoot::Axis(semio_framework_core::layout::WindowLayoutAxisNode {
            kind: "column".into(),
            size: None,
            children: children
                .iter()
                .map(|(child, size)| dock_child_from_node(child, *size))
                .collect(),
        }),
    }
}

fn dock_child_from_node(node: &DockNode, size: f32) -> WindowLayoutChild {
    match node {
        DockNode::Stack { windows, active } => WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: Some(size as f64),
            active_window_kind_id: Some(active.clone()),
            children: windows
                .iter()
                .map(|id| WindowLayoutWindowNode {
                    kind: "window".into(),
                    window_kind_id: id.clone(),
                    title: None,
                    instance_id: None,
                    template_id: None,
                })
                .collect(),
        }),
        DockNode::Row(children) => WindowLayoutChild::Axis(semio_framework_core::layout::WindowLayoutAxisNode {
            kind: "row".into(),
            size: Some(size as f64),
            children: children
                .iter()
                .map(|(child, child_size)| dock_child_from_node(child, *child_size))
                .collect(),
        }),
        DockNode::Column(children) => WindowLayoutChild::Axis(semio_framework_core::layout::WindowLayoutAxisNode {
            kind: "column".into(),
            size: Some(size as f64),
            children: children
                .iter()
                .map(|(child, child_size)| dock_child_from_node(child, *child_size))
                .collect(),
        }),
    }
}

fn axis_pair_from_stacks(first: &DockNode, second: &DockNode, side: DockSide) -> DockNode {
    let pair = vec![(first.clone(), 0.5), (second.clone(), 0.5)];
    match side {
        DockSide::Left | DockSide::Right => DockNode::Row(pair),
        DockSide::Top | DockSide::Bottom => DockNode::Column(pair),
    }
}

fn replace_node_at(root: &mut DockNode, path: &[usize], replacement: DockNode) {
    if path.is_empty() {
        *root = replacement;
        return;
    }
    if let Some((head, tail)) = path.split_first() {
        let children = match root {
            DockNode::Row(children) | DockNode::Column(children) => children,
            DockNode::Stack { .. } => return,
        };
        if tail.is_empty() {
            if let Some((slot, _)) = children.get_mut(*head) {
                *slot = replacement;
            }
        } else if let Some((slot, _)) = children.get_mut(*head) {
            replace_node_at(slot, tail, replacement);
        }
    }
}

fn remove_window_from_node(node: &mut DockNode, window_id: &str) -> bool {
    match node {
        DockNode::Stack { windows, active } => {
            if let Some(index) = windows.iter().position(|id| id == window_id) {
                windows.remove(index);
                if active == window_id {
                    *active = windows
                        .get(index.saturating_sub(1))
                        .or_else(|| windows.first())
                        .cloned()
                        .unwrap_or_default();
                }
                return true;
            }
            false
        }
        DockNode::Row(children) | DockNode::Column(children) => children
            .iter_mut()
            .any(|(child, _)| remove_window_from_node(child, window_id)),
    }
}

pub fn resolve_split_side(local_x: f32, local_y: f32, width: f32, height: f32) -> DockSide {
    let mid_x = width * 0.5;
    let mid_y = height * 0.5;
    let dx = (local_x - mid_x).abs();
    let dy = (local_y - mid_y).abs();
    if dx >= dy {
        if local_x < mid_x {
            DockSide::Left
        } else {
            DockSide::Right
        }
    } else if local_y < mid_y {
        DockSide::Top
    } else {
        DockSide::Bottom
    }
}

pub fn compute_tab_insert_index(pointer_x: f32, tab_bar: Rect, tab_widths: &[f32], gap: f32) -> usize {
    let mut x = tab_bar.x + gap;
    for (index, width) in tab_widths.iter().enumerate() {
        if pointer_x < x + width * 0.5 {
            return index;
        }
        x += width + gap;
    }
    tab_widths.len()
}

pub fn compute_dock_drop_zone(
    pointer_x: f32,
    pointer_y: f32,
    tab_bars: &[(DockPath, Rect, Vec<f32>)],
    bodies: &[(DockPath, Rect, String)],
    canvas: Rect,
) -> Option<DockDropZone> {
    for (path, rect, widths) in tab_bars {
        if rect.contains(pointer_x, pointer_y) {
            let index = compute_tab_insert_index(pointer_x, *rect, widths, 4.0);
            return Some(DockDropZone::Tab {
                stack_path: path.clone(),
                index,
            });
        }
    }
    for (path, rect, _) in bodies {
        if rect.contains(pointer_x, pointer_y) {
            return Some(DockDropZone::Split {
                stack_path: path.clone(),
                side: resolve_split_side(
                    pointer_x - rect.x,
                    pointer_y - rect.y,
                    rect.w,
                    rect.h,
                ),
            });
        }
    }
    if canvas.contains(pointer_x, pointer_y) {
        return Some(DockDropZone::RootSplit {
            side: resolve_split_side(
                pointer_x - canvas.x,
                pointer_y - canvas.y,
                canvas.w,
                canvas.h,
            ),
        });
    }
    None
}

/// @emoji 📐 Half-panel rectangle for split drop preview inside a stack body.
pub fn split_drop_preview_in_body(body: Rect, side: DockSide) -> Rect {
    let half_w = body.w * 0.5;
    let half_h = body.h * 0.5;
    match side {
        DockSide::Left => Rect::new(body.x, body.y, half_w, body.h),
        DockSide::Right => Rect::new(body.x + body.w - half_w, body.y, half_w, body.h),
        DockSide::Top => Rect::new(body.x, body.y, body.w, half_h),
        DockSide::Bottom => Rect::new(body.x, body.y + body.h - half_h, body.w, half_h),
    }
}

/// @emoji 🎯 Resolves the on-canvas indicator rect for an active dock drop zone.
pub fn drop_zone_indicator_rect(
    zone: &DockDropZone,
    tab_bars: &[(DockPath, Rect, Vec<f32>)],
    bodies: &[(DockPath, Rect, String)],
    canvas: Rect,
    gap: f32,
) -> Option<Rect> {
    match zone {
        DockDropZone::Tab { stack_path, index } => {
            let (_, tab_bar, widths) = tab_bars.iter().find(|(path, _, _)| path == stack_path)?;
            let mut x = tab_bar.x + gap;
            for width in widths.iter().take(*index) {
                x += width + gap;
            }
            let preview_w = widths.get(*index).copied().unwrap_or(88.0).clamp(48.0, 120.0);
            Some(Rect::new(
                x,
                tab_bar.y + gap * 0.5,
                preview_w,
                tab_bar.h - gap,
            ))
        }
        DockDropZone::Split { stack_path, side } => {
            let (_, body, _) = bodies.iter().find(|(path, _, _)| path == stack_path)?;
            Some(split_drop_preview_in_body(*body, *side))
        }
        DockDropZone::RootSplit { side } => Some(split_drop_preview_in_body(canvas, *side)),
    }
}

fn stack_tab_bar_rect(bounds: Rect, theme: &Theme) -> Rect {
    Rect::new(bounds.x, bounds.y, bounds.w, theme.control_height)
}

fn collect_stack_tab_bars(
    node: &DockNode,
    bounds: Rect,
    path: &[usize],
    theme: &Theme,
    out: &mut Vec<(DockPath, Rect)>,
) {
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_tab_bars(child, Rect::new(x, bounds.y, w, bounds.h), &child_path, theme, out);
                x += w;
            }
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_tab_bars(child, Rect::new(bounds.x, y, bounds.w, h), &child_path, theme, out);
                y += h;
            }
        }
        DockNode::Stack { .. } => {
            out.push((path.to_vec(), stack_tab_bar_rect(bounds, theme)));
        }
    }
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
        DockNode::Stack { .. } => None,
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
        DockNode::Stack { .. } => None,
    }
}

fn path_exists(node: &DockNode, path: &[usize]) -> bool {
    node_at(node, path).is_some()
}

fn node_at<'a>(node: &'a DockNode, path: &[usize]) -> Option<&'a DockNode> {
    let mut current = node;
    for index in path {
        current = match current {
            DockNode::Row(children) | DockNode::Column(children) => children.get(*index).map(|(n, _)| n)?,
            DockNode::Stack { .. } => return None,
        };
    }
    Some(current)
}

fn node_at_mut<'a>(node: &'a mut DockNode, path: &[usize]) -> Option<&'a mut DockNode> {
    if path.is_empty() {
        return Some(node);
    }
    let (head, tail) = path.split_first()?;
    let child = match node {
        DockNode::Row(children) | DockNode::Column(children) => children.get_mut(*head).map(|(n, _)| n)?,
        DockNode::Stack { .. } => return None,
    };
    node_at_mut(child, &tail.to_vec())
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

fn solve_node_bounds(
    node: &DockNode,
    bounds: Rect,
    target_path: &[usize],
    current_path: &[usize],
) -> Option<Rect> {
    if current_path == target_path {
        return Some(bounds);
    }
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut path = current_path.to_vec();
                path.push(index);
                if let Some(found) = solve_node_bounds(child, Rect::new(x, bounds.y, w, bounds.h), target_path, &path) {
                    return Some(found);
                }
                x += w;
            }
            None
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut path = current_path.to_vec();
                path.push(index);
                if let Some(found) = solve_node_bounds(child, Rect::new(bounds.x, y, bounds.w, h), target_path, &path) {
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
    ctx: &mut DockRenderContext<'_>,
    node: &DockNode,
    bounds: Rect,
    path: &[usize],
    render_body: &mut dyn FnMut(Rect, &str),
    outer_split: Option<(DockPath, usize, bool)>,
) {
    match node {
        DockNode::Row(children) => render_axis(state, ctx, children, bounds, path, true, render_body, outer_split),
        DockNode::Column(children) => render_axis(state, ctx, children, bounds, path, false, render_body, outer_split),
        DockNode::Stack { .. } => render_stack(state, ctx, path, node, bounds, false, render_body),
    }
}

const SPLIT_VIS_PX: f32 = 6.0;
const SPLIT_HIT_MIN_PX: f32 = 20.0;

fn render_axis(
    state: &DockState,
    ctx: &mut DockRenderContext<'_>,
    children: &[(DockNode, f32)],
    bounds: Rect,
    path: &[usize],
    horizontal: bool,
    render_body: &mut dyn FnMut(Rect, &str),
    outer_split: Option<(DockPath, usize, bool)>,
) {
    let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
    if horizontal {
        let mut x = bounds.x;
        for (index, (child, size)) in children.iter().enumerate() {
            let w = bounds.w * (*size / total);
            let child_rect = Rect::new(x, bounds.y, w, bounds.h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            render_node(
                state,
                ctx,
                child,
                child_rect,
                &child_path,
                render_body,
                Some((path.to_vec(), index, true)),
            );
            x += w;
        }
    } else {
        let mut y = bounds.y;
        for (index, (child, size)) in children.iter().enumerate() {
            let h = bounds.h * (*size / total);
            let child_rect = Rect::new(bounds.x, y, bounds.w, h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            render_node(
                state,
                ctx,
                child,
                child_rect,
                &child_path,
                render_body,
                Some((path.to_vec(), index, false)),
            );
            y += h;
        }
    }
    let _ = outer_split;
}

fn walk_resize_hits(
    state: &DockState,
    ctx: &mut DockRenderContext<'_>,
    node: &DockNode,
    bounds: Rect,
    path: &[usize],
    outer_split: Option<(DockPath, usize, bool)>,
) {
    match node {
        DockNode::Row(children) => walk_resize_axis(
            state,
            ctx,
            children,
            bounds,
            path,
            true,
            outer_split,
        ),
        DockNode::Column(children) => walk_resize_axis(
            state,
            ctx,
            children,
            bounds,
            path,
            false,
            outer_split,
        ),
        DockNode::Stack { .. } => {}
    }
}

fn walk_resize_axis(
    state: &DockState,
    ctx: &mut DockRenderContext<'_>,
    children: &[(DockNode, f32)],
    bounds: Rect,
    path: &[usize],
    horizontal: bool,
    outer_split: Option<(DockPath, usize, bool)>,
) {
    let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
    if horizontal {
        let mut x = bounds.x;
        for (index, (child, size)) in children.iter().enumerate() {
            let w = bounds.w * (*size / total);
            let child_rect = Rect::new(x, bounds.y, w, bounds.h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            walk_resize_hits(
                state,
                ctx,
                child,
                child_rect,
                &child_path,
                Some((path.to_vec(), index, true)),
            );
            x += w;
            if index + 1 < children.len() {
                let hit_w = SPLIT_HIT_MIN_PX.max(SPLIT_VIS_PX);
                let handle = Rect::new(x - hit_w * 0.5, bounds.y, hit_w, bounds.h);
                register_split_hit(ctx, path, index, handle, DragAxis::Horizontal);
                if let Some((parent_path, parent_index, parent_horizontal)) = &outer_split {
                    if *parent_horizontal != horizontal {
                        register_join_corner_hits(ctx, path, index, parent_path, *parent_index, handle, horizontal);
                    }
                }
            }
        }
    } else {
        let mut y = bounds.y;
        for (index, (child, size)) in children.iter().enumerate() {
            let h = bounds.h * (*size / total);
            let child_rect = Rect::new(bounds.x, y, bounds.w, h);
            let mut child_path = path.to_vec();
            child_path.push(index);
            walk_resize_hits(
                state,
                ctx,
                child,
                child_rect,
                &child_path,
                Some((path.to_vec(), index, false)),
            );
            y += h;
            if index + 1 < children.len() {
                let hit_h = SPLIT_HIT_MIN_PX.max(SPLIT_VIS_PX);
                let handle = Rect::new(bounds.x, y - hit_h * 0.5, bounds.w, hit_h);
                register_split_hit(ctx, path, index, handle, DragAxis::Vertical);
                if let Some((parent_path, parent_index, parent_horizontal)) = &outer_split {
                    if *parent_horizontal != horizontal {
                        register_join_corner_hits(ctx, path, index, parent_path, *parent_index, handle, horizontal);
                    }
                }
            }
        }
    }
}

fn register_join_corner_hits(
    ctx: &mut DockRenderContext<'_>,
    path: &[usize],
    split_index: usize,
    parent_path: &DockPath,
    parent_index: usize,
    handle: Rect,
    horizontal: bool,
) {
    let corner = 10.0;
    let corners = if horizontal {
        [
            Rect::new(handle.x - corner * 0.5, handle.y, corner, corner),
            Rect::new(handle.x - corner * 0.5, handle.y + handle.h - corner, corner, corner),
        ]
    } else {
        [
            Rect::new(handle.x, handle.y - corner * 0.5, corner, corner),
            Rect::new(handle.x + handle.w - corner, handle.y - corner * 0.5, corner, corner),
        ]
    };
    for (corner_slot, rect) in corners.iter().enumerate() {
        let _ = corner_slot;
        ctx.input.register_hit(HitTarget {
            rect: *rect,
            event: None,
            control_id: Some(format!(
                "dock.corner.r/{}/{}/c/{}/{}",
                path_str(path),
                split_index,
                path_str(parent_path),
                parent_index
            )),
            kind: HitKind::DockJoinCorner,
            drag_axis: Some(DragAxis::Both),
            drag_data: None,
        });
    }
}

fn register_split_hit(
    ctx: &mut DockRenderContext<'_>,
    path: &[usize],
    index: usize,
    rect: Rect,
    axis: DragAxis,
) {
    ctx.input.register_hit(HitTarget {
        rect,
        event: None,
        control_id: Some(format!("dock.split.{}.{index}", path_str(path))),
        kind: HitKind::DockSplit,
        drag_axis: Some(axis),
        drag_data: None,
    });
}

fn render_stack(
    state: &DockState,
    ctx: &mut DockRenderContext<'_>,
    path: &[usize],
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
    let globally_active = state.active_stack.as_ref().map(|p| p.as_slice()) == Some(path);
    let stack_hovered = bounds.contains(ctx.input.pointer_x, ctx.input.pointer_y);
    let border = if globally_active {
        theme.accent
    } else if stack_hovered {
        theme.border_emphasized
    } else {
        theme.border_normal
    };
    let cap_y = bounds.y;
    let cap_rect = Rect::new(bounds.x, cap_y, bounds.w, tab_h);
    ctx.draw.push_solid([cap_rect.x, cap_rect.y, cap_rect.w, cap_rect.h], theme.navbar);

    let focus_label = if maximized { "Unfocus" } else { "Focus" };
    let focus_icon = if maximized { "minimize-2" } else { "maximize-2" };
    let focus_w = measure_cap_button(ctx.atlas, theme, focus_icon, focus_label);
    let close_w = measure_cap_button(ctx.atlas, theme, "x", "Close");
    let controls_w = focus_w + close_w;
    let per_tab_chrome = windows.len() > 1;
    let mut tab_x = cap_rect.x;
    let mut active_tab_x = cap_rect.x;
    for (index, window_id) in windows.iter().enumerate() {
        let label = ctx
            .window_labels
            .get(window_id)
            .map(String::as_str)
            .unwrap_or(window_id);
        let tw = ctx.atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0;
        let tab_rect = Rect::new(tab_x, cap_y, tw, tab_h);
        let is_active = window_id == active;
        if is_active {
            active_tab_x = tab_x;
        }
        let stack_active_tab = is_active && globally_active;
        let is_last_before_gap = per_tab_chrome && index + 1 == windows.len();
        let hovered = tab_rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
        let bg = if stack_active_tab {
            theme.selected
        } else if is_active && hovered {
            theme.button_hover
        } else if hovered {
            theme.button_hover
        } else {
            theme.navbar
        };
        ctx.draw.push_solid([tab_rect.x, tab_rect.y, tab_rect.w, tab_rect.h], bg);
        if per_tab_chrome {
            let tab_border = if stack_active_tab {
                theme.accent
            } else {
                border
            };
            if stack_active_tab || is_last_before_gap {
                push_window_cap_border(ctx.draw, tab_rect, stroke, tab_border);
            } else {
                push_chrome_border(
                    ctx.draw,
                    tab_rect,
                    stroke,
                    tab_border,
                    true,
                    true,
                    true,
                    true,
                );
            }
        }
        dock_text(
            ctx,
            label,
            tab_rect.x + theme.padding_standard,
            tab_rect.y + (tab_rect.h + theme.font_size_small) * 0.5 - 1.0,
            theme.font_size_small,
            if stack_active_tab {
                theme.active_foreground
            } else if hovered {
                theme.border_emphasized
            } else {
                theme.text_element
            },
        );
        ctx.input.register_hit(HitTarget {
            rect: tab_rect,
            event: None,
            control_id: Some(format!("dock.tab.{}.{}", path_str(path), window_id)),
            kind: HitKind::Window,
            drag_axis: None,
            drag_data: None,
        });
        tab_x += tw;
    }
    if !per_tab_chrome {
        let tabs_cap = Rect::new(cap_rect.x, cap_y, tab_x - cap_rect.x, tab_h);
        push_window_cap_border(ctx.draw, tabs_cap, stroke, border);
    }
    let gap_x = tab_x;
    let controls_x = cap_rect.x + cap_rect.w - controls_w;
    let gap_w = (controls_x - gap_x).max(0.0);
    let gap_rect = Rect::new(gap_x, cap_y, gap_w, tab_h);
    ctx.draw.push_solid([gap_rect.x, gap_rect.y, gap_rect.w, gap_rect.h], theme.canvas_clear);
    push_chrome_border(
        ctx.draw,
        gap_rect,
        stroke,
        border,
        false,
        false,
        true,
        false,
    );
    ctx.input.register_hit(HitTarget {
        rect: gap_rect,
        event: None,
        control_id: Some(format!("dock.stack.{}", path_str(path))),
        kind: HitKind::Window,
        drag_axis: None,
        drag_data: None,
    });

    let controls_rect = Rect::new(controls_x, cap_y, controls_w, tab_h);
    ctx.draw.push_solid(
        [controls_rect.x, controls_rect.y, controls_rect.w, controls_rect.h],
        theme.navbar,
    );
    push_window_cap_border(ctx.draw, controls_rect, stroke, border);
    render_cap_action_group(
        ctx,
        controls_rect,
        &[
            ("dock.focus", focus_icon, focus_label),
            ("dock.close", "x", "Close"),
        ],
        path,
        false,
    );

    let body_y = cap_y + tab_h;
    let body_x = if per_tab_chrome { active_tab_x } else { bounds.x };
    let body_w = if per_tab_chrome {
        (gap_x + gap_w - active_tab_x).max(0.0)
    } else {
        bounds.w
    };
    let body_rect = Rect::new(body_x, body_y, body_w, bounds.h - tab_h);
    ctx.draw.push_solid([body_rect.x, body_rect.y, body_rect.w, body_rect.h], theme.canvas_clear);
    ctx.draw
        .push_solid([body_rect.x, body_rect.y, stroke, body_rect.h], border);
    ctx.draw
        .push_solid([body_rect.x + body_rect.w - stroke, body_rect.y, stroke, body_rect.h], border);
    ctx.draw
        .push_solid([body_rect.x, body_rect.y + body_rect.h - stroke, body_rect.w, stroke], border);

    let content = body_rect.inset(theme.padding_standard);
    render_body(content, active);
}

struct StackCapLayout {
    active_tab_x: f32,
    gap_x: f32,
    gap_w: f32,
}

fn layout_stack_cap(
    windows: &[String],
    active: &str,
    labels: &HashMap<String, String>,
    atlas: &mut FontAtlas,
    theme: &Theme,
    bounds: Rect,
    maximized: bool,
) -> StackCapLayout {
    let focus_label = if maximized { "Unfocus" } else { "Focus" };
    let focus_icon = if maximized { "minimize-2" } else { "maximize-2" };
    let focus_w = measure_cap_button(atlas, theme, focus_icon, focus_label);
    let close_w = measure_cap_button(atlas, theme, "x", "Close");
    let controls_w = focus_w + close_w;
    let mut tab_x = bounds.x;
    let mut active_tab_x = bounds.x;
    for window_id in windows {
        let label = labels
            .get(window_id)
            .map(String::as_str)
            .unwrap_or(window_id.as_str());
        let tw = atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0;
        if window_id == active {
            active_tab_x = tab_x;
        }
        tab_x += tw;
    }
    let gap_x = tab_x;
    let controls_x = bounds.x + bounds.w - controls_w;
    let gap_w = (controls_x - gap_x).max(0.0);
    StackCapLayout {
        active_tab_x,
        gap_x,
        gap_w,
    }
}

fn stack_body_chrome_rect(bounds: Rect, theme: &Theme, windows: &[String], layout: &StackCapLayout) -> Rect {
    let tab_h = theme.control_height;
    let per_tab_chrome = windows.len() > 1;
    let body_x = if per_tab_chrome {
        layout.active_tab_x
    } else {
        bounds.x
    };
    let body_w = if per_tab_chrome {
        (layout.gap_x + layout.gap_w - body_x).max(0.0)
    } else {
        bounds.w
    };
    Rect::new(
        body_x,
        bounds.y + tab_h,
        body_w,
        (bounds.h - tab_h).max(0.0),
    )
}

fn stack_content_rect(
    bounds: Rect,
    theme: &Theme,
    windows: &[String],
    active: &str,
    labels: &HashMap<String, String>,
    atlas: &mut FontAtlas,
    maximized: bool,
) -> Rect {
    let layout = layout_stack_cap(windows, active, labels, atlas, theme, bounds, maximized);
    stack_body_chrome_rect(bounds, theme, windows, &layout).inset(theme.padding_standard)
}

fn render_cap_action_group(
    ctx: &mut DockRenderContext<'_>,
    rect: Rect,
    buttons: &[(&str, &str, &str)],
    path: &[usize],
    draw_outer_border: bool,
) {
    let theme = ctx.theme;
    let hair = theme.stroke_hairline;
    let inner_y = rect.y + hair;
    let inner_h = (rect.h - hair * 2.0).max(0.0);
    let mut x = rect.x;
    for (index, (prefix, icon_id, label)) in buttons.iter().enumerate() {
        let item_w = measure_cap_button(ctx.atlas, theme, icon_id, label);
        let item_rect = Rect::new(x, inner_y, item_w, inner_h);
        let hovered = item_rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
        let bg = chrome_item_bg(theme, false, hovered);
        if bg.a > 0.0 {
            ctx.draw.push_solid([item_rect.x, item_rect.y, item_rect.w, item_rect.h], bg);
        }
        let icon_size = 14.0;
        let mut content_x = item_rect.x + theme.padding_standard;
        let icon_color = chrome_item_text(theme, false, hovered);
        if let Some(uv) = ctx.icons.icon_uv(icon_id) {
            ctx.draw.push_textured(
                [
                    content_x,
                    item_rect.y + (item_rect.h - icon_size) * 0.5,
                    icon_size,
                    icon_size,
                ],
                uv,
                icon_color,
            );
            content_x += icon_size + theme.gap_standard;
        }
        dock_text(
            ctx,
            label,
            content_x,
            item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0,
            theme.font_size_small,
            chrome_item_text(theme, false, hovered),
        );
        ctx.input.register_hit(HitTarget {
            rect: item_rect,
            event: None,
            control_id: Some(format!("{prefix}.{}", path_str(path))),
            kind: HitKind::Button,
            drag_axis: None,
            drag_data: None,
        });
        x += item_w;
        if index + 1 < buttons.len() {
            ctx.draw.push_solid([x, inner_y, hair, inner_h], theme.border_normal);
        }
    }
    if draw_outer_border {
        push_chrome_group_border(ctx.draw, rect, theme);
    }
}

fn measure_cap_button(atlas: &mut FontAtlas, theme: &Theme, _icon_id: &str, label: &str) -> f32 {
    let icon_w = 14.0 + theme.gap_standard;
    let text_w = atlas.measure_text(label, theme.font_size_small).0;
    theme.padding_standard * 2.0 + icon_w + text_w
}

pub fn path_str(path: &[usize]) -> String {
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

fn collect_stack_bodies(
    node: &DockNode,
    bounds: Rect,
    path: &[usize],
    theme: &Theme,
    window_labels: &HashMap<String, String>,
    atlas: &mut FontAtlas,
    state: &DockState,
    out: &mut Vec<(DockPath, Rect, String)>,
) {
    match node {
        DockNode::Row(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut x = bounds.x;
            for (index, (child, size)) in children.iter().enumerate() {
                let w = bounds.w * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_bodies(
                    child,
                    Rect::new(x, bounds.y, w, bounds.h),
                    &child_path,
                    theme,
                    window_labels,
                    atlas,
                    state,
                    out,
                );
                x += w;
            }
        }
        DockNode::Column(children) => {
            let total: f32 = children.iter().map(|(_, s)| *s).sum::<f32>().max(0.001);
            let mut y = bounds.y;
            for (index, (child, size)) in children.iter().enumerate() {
                let h = bounds.h * (*size / total);
                let mut child_path = path.to_vec();
                child_path.push(index);
                collect_stack_bodies(
                    child,
                    Rect::new(bounds.x, y, bounds.w, h),
                    &child_path,
                    theme,
                    window_labels,
                    atlas,
                    state,
                    out,
                );
                y += h;
            }
        }
        DockNode::Stack { windows, active } => {
            let maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            out.push((
                path.to_vec(),
                stack_content_rect(
                    bounds,
                    theme,
                    windows,
                    active,
                    window_labels,
                    atlas,
                    maximized,
                ),
                active.clone(),
            ));
        }
    }
}

fn dock_text(
    ctx: &mut DockRenderContext<'_>,
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
        None,
    );
    draw_text(&mut widget_ctx, text, x, y, size, color);
}

//#region DockFreeFunctions

//#region DockTests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use semio_framework_core::layout::create_default_layout;
    use semio_framework_core::{AppDefinition, ModeDefinition, PanelTabDefinition, WindowKindDefinition};

    fn sample_app(window_ids: &[&str], layout: Option<WindowLayout>) -> AppDefinition {
        AppDefinition {
            id: "test".into(),
            label: "Test".into(),
            hierarchy: vec!["semio".into(), "test".into()],
            icon_id: None,
            controller_id: "test".into(),
            modes: vec![ModeDefinition {
                id: "default".into(),
                label: "Default".into(),
                tools: vec![],
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
    fn split_axis_extent_uses_row_width_not_canvas_max() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = DockNode::Column(vec![
            (
                DockNode::Row(vec![
                    (stack_with("a"), 0.5),
                    (stack_with("b"), 0.5),
                ]),
                0.5,
            ),
            (stack_with("c"), 0.5),
        ]);
        let canvas = Rect::new(0.0, 0.0, 1000.0, 800.0);
        let row_extent = dock.split_axis_extent(&[], canvas).unwrap();
        assert!((row_extent - 1000.0).abs() < 0.1);
        let col_extent = dock.split_axis_extent(&[], canvas);
        assert!((col_extent.unwrap() - 800.0).abs() < 0.1);
        let nested_extent = dock.split_axis_extent(&[0], canvas).unwrap();
        assert!((nested_extent - 1000.0).abs() < 0.1);
    }

    #[test]
    fn resize_hits_win_over_later_scroll_region() {
        let dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = even_layout(&["a".into(), "b".into()]);
        let canvas = Rect::new(0.0, 0.0, 400.0, 300.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::default();
        let mut input = InputState::<()>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::from([
            ("a".into(), "A".into()),
            ("b".into(), "B".into()),
        ]);
        let mut ctx = DockRenderContext {
            draw: &mut draw,
            atlas: &mut atlas,
            icons: None,
            input: &mut input,
            theme: &theme,
            window_labels: &labels,
        };
        input.register_hit(HitTarget {
            rect: canvas,
            event: None,
            control_id: Some("content.scroll".into()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
            drag_data: None,
        });
        dock.register_resize_hits(&mut ctx, canvas);
        let hit = input.hit_at(200.0, 150.0).expect("split hit");
        assert_eq!(hit.kind, HitKind::DockSplit);
        assert_eq!(hit.drag_axis, Some(DragAxis::Horizontal));
        assert!(hit.rect.w >= 20.0);
    }

    fn stack_with(id: &str) -> DockNode {
        DockNode::Stack {
            windows: vec![id.into()],
            active: id.into(),
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

    #[test]
    fn reorder_tab_within_stack() {
        let mut dock = DockState::default();
        dock.root = DockNode::Stack {
            windows: vec!["a".into(), "b".into(), "c".into()],
            active: "a".into(),
        };
        assert!(dock.reorder_tab(&[], 0, 2));
        if let DockNode::Stack { windows, .. } = &dock.root {
            assert_eq!(windows, &vec!["b".to_string(), "c".to_string(), "a".to_string()]);
        } else {
            panic!("expected stack");
        }
    }

    #[test]
    fn maximized_stack_uses_full_canvas_bounds() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b", "c"], None), Some("a"));
        dock.root = even_layout(&["a".into(), "b".into(), "c".into()]);
        dock.toggle_maximize(&[1]);
        let canvas = Rect::new(0.0, 0.0, 900.0, 600.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::default();
        let bodies = dock.stack_body_rects(canvas, &theme, &HashMap::new(), &mut atlas);
        assert_eq!(bodies.len(), 1);
        let (_, body, _) = &bodies[0];
        assert!((body.w - canvas.w).abs() < 1.0);
        assert!((body.h - (canvas.h - theme.control_height)).abs() < 2.0);
    }

    #[test]
    fn split_drop_preview_covers_half_panel() {
        let body = Rect::new(10.0, 20.0, 400.0, 300.0);
        let left = split_drop_preview_in_body(body, DockSide::Left);
        assert_eq!(left.x, 10.0);
        assert_eq!(left.w, 200.0);
        assert_eq!(left.h, 300.0);
        let right = split_drop_preview_in_body(body, DockSide::Right);
        assert_eq!(right.x, 210.0);
        assert_eq!(right.w, 200.0);
    }
}
//#endregion DockTests
