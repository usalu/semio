//! 🧊 Raw wgpu WASM renderer for declarative framework UiNode trees.

pub mod dock {
// #region dock
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
// #endregion dock
}

pub mod engine_canvas {
// #region engine_canvas
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
// #endregion engine_canvas
}

pub mod interpreter {
// #region interpreter
//! 🧩 Maps framework UiNode trees to ui_wgpu widget nodes.

use crate::scenes::{render_component_scene, NodeGraphSurface};
use semio_framework_core::{CommandDescriptor, UiControlNode, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeSectionNode};
use ui_wgpu::{
    gap_for_token, layout_horizontal, layout_vertical, padding_for_token, ControlNode, KeyValueEntry, Rect, SelectItem,
    Theme, TreeItem, TreeItemAction, TreeSection, WidgetContext, WidgetInteractionMaps, WidgetNode, measure_widget,
    render_widget,
};

pub type FrameworkWidgetContext<'a> = WidgetContext<'a, CommandDescriptor>;

pub fn measure_ui_node(atlas: &mut ui_wgpu::FontAtlas, theme: &Theme, node: &UiNode) -> (f32, f32) {
    match node {
        UiNode::ComponentScene(_) => (320.0, 240.0),
        UiNode::Stack(stack) => {
            let gap = gap_for_token(theme, stack.gap.as_deref());
            let padding = padding_for_token(theme, stack.padding.as_deref()) * 2.0;
            let vertical = stack.direction != "horizontal";
            let mut total_main = 0.0f32;
            let mut max_cross = 0.0f32;
            for (index, child) in stack.children.iter().enumerate() {
                let (w, h) = measure_ui_node(atlas, theme, child);
                if vertical {
                    total_main += h;
                    max_cross = max_cross.max(w);
                    if index + 1 < stack.children.len() {
                        total_main += gap;
                    }
                } else {
                    total_main += w;
                    max_cross = max_cross.max(h);
                    if index + 1 < stack.children.len() {
                        total_main += gap;
                    }
                }
            }
            if vertical {
                (max_cross + padding, total_main + padding)
            } else {
                (total_main + padding, max_cross + padding)
            }
        }
        other => measure_widget(atlas, theme, &ui_node_to_widget(other)),
    }
}

pub fn render_ui_node(
    node: &UiNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    world3d_states: &mut std::collections::HashMap<String, infinite_world::World3dState>,
    node_graph_states: &mut std::collections::HashMap<String, NodeGraphSurface>,
) {
    match node {
        UiNode::ComponentScene(scene) => {
            render_component_scene(scene, bounds, ctx, gpu, world3d_states, node_graph_states)
        }
        UiNode::Stack(stack) => {
            let gap = gap_for_token(ctx.theme, stack.gap.as_deref());
            let padding = padding_for_token(ctx.theme, stack.padding.as_deref());
            let vertical = stack.direction != "horizontal";
            let sizes: Vec<f32> = stack
                .children
                .iter()
                .map(|child| {
                    let (w, h) = measure_ui_node(ctx.atlas, ctx.theme, child);
                    if vertical { h } else { w }
                })
                .collect();
            let rects = if vertical {
                layout_vertical(bounds, gap, padding, &sizes)
            } else {
                layout_horizontal(bounds, gap, padding, &sizes)
            };
            for (child, rect) in stack.children.iter().zip(rects.iter()) {
                render_ui_node(child, *rect, ctx, gpu, world3d_states, node_graph_states);
            }
        }
        other => render_widget(&ui_node_to_widget(other), bounds, ctx),
    }
}

pub fn ui_node_to_widget(node: &UiNode) -> WidgetNode<CommandDescriptor> {
    match node {
        UiNode::Stack(stack) => WidgetNode::Stack {
            direction: stack.direction.clone(),
            gap: stack.gap.clone(),
            padding: stack.padding.clone(),
            children: stack.children.iter().map(ui_node_to_widget).collect(),
        },
        UiNode::Text(text) => WidgetNode::Text {
            value: text.value.clone(),
            emphasize: text.emphasize.unwrap_or(false),
        },
        UiNode::Separator(_) => WidgetNode::Separator,
        UiNode::Button(button) => WidgetNode::Button {
            id: button.id.clone(),
            icon_id: Some(button.icon_id.clone()),
            label: button.label.clone(),
            event: Some(button.command.clone()),
        },
        UiNode::Input(input) => WidgetNode::Input {
            id: input.id.clone(),
            input_kind: input.input_kind.clone(),
            value: input.value.clone(),
            placeholder: input.placeholder.clone(),
            commit: input.commit.clone(),
            on_change: Some(input.on_change.clone()),
        },
        UiNode::Select(select) => WidgetNode::Select {
            id: select.id.clone(),
            value: select.value.clone(),
            items: select.items.iter().map(|i| SelectItem { value: i.value.clone(), label: i.label.clone() }).collect(),
            placeholder: select.placeholder.clone(),
            on_change: Some(select.on_change.clone()),
        },
        UiNode::Toggle(toggle) => WidgetNode::Toggle {
            id: toggle.id.clone(),
            icon_id: toggle.icon_id.clone(),
            pressed: toggle.pressed,
            text: toggle.text.clone(),
            on_change: Some(toggle.on_change.clone()),
        },
        UiNode::Vec3(vec3) => WidgetNode::Vec3 {
            id: vec3.id.clone(),
            value: vec3.value,
            on_change: Some(vec3.on_change.clone()),
        },
        UiNode::KeyValue(kv) => WidgetNode::KeyValue {
            entries: kv.entries.iter().map(|e| KeyValueEntry { label: e.label.clone(), value: e.value.clone() }).collect(),
        },
        UiNode::Slider(slider) => WidgetNode::Slider {
            id: slider.id.clone(),
            value: slider.value,
            min: slider.min,
            max: slider.max,
            step: slider.step,
            on_change: Some(slider.on_change.clone()),
        },
        UiNode::NumberStepper(stepper) => WidgetNode::NumberStepper {
            id: stepper.id.clone(),
            value: stepper.value,
            step: stepper.step,
            uniform: stepper.uniform,
            on_absolute: Some(stepper.on_absolute.clone()),
            on_delta: Some(stepper.on_delta.clone()),
        },
        UiNode::Ring(ring) => WidgetNode::Ring {
            id: ring.id.clone(),
            t: ring.t,
            disabled: ring.disabled.unwrap_or(false),
            on_change: Some(ring.on_change.clone()),
        },
        UiNode::IconSelect(icon) => WidgetNode::IconSelect {
            id: icon.id.clone(),
            value: icon.value.clone(),
            uniform: icon.uniform,
            classifier_kind: icon.classifier_kind.clone(),
            on_change: Some(icon.on_change.clone()),
        },
        UiNode::Field(field) => WidgetNode::Field {
            id: field.id.clone(),
            label: field.label.clone(),
            child: control_to_widget(&field.child),
        },
        UiNode::Section(section) => WidgetNode::Section {
            id: section.id.clone(),
            label: section.label.clone(),
            default_open: section.default_open.unwrap_or(true),
            children: section.children.iter().map(ui_node_to_widget).collect(),
        },
        UiNode::Tree(tree) => WidgetNode::Tree {
            sections: tree.sections.iter().map(tree_section_to_widget).collect(),
            selected_ids: tree.selected_ids.clone().unwrap_or_default(),
            highlighted_ids: tree.highlighted_ids.clone().unwrap_or_default(),
            selection_change: tree.selection_change.clone(),
        },
        UiNode::ComponentScene(_) => WidgetNode::Text {
            value: String::new(),
            emphasize: false,
        },
    }
}

fn control_to_widget(control: &UiControlNode) -> ControlNode<CommandDescriptor> {
    match control {
        UiControlNode::Button(n) => ControlNode::Button {
            id: n.id.clone(),
            icon_id: Some(n.icon_id.clone()),
            label: n.label.clone(),
            event: Some(n.command.clone()),
        },
        UiControlNode::Input(n) => ControlNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone(),
            commit: n.commit.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => ControlNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|i| SelectItem { value: i.value.clone(), label: i.label.clone() }).collect(),
            placeholder: n.placeholder.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => ControlNode::Toggle {
            id: n.id.clone(),
            icon_id: n.icon_id.clone(),
            pressed: n.pressed,
            text: n.text.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Vec3(n) => ControlNode::Vec3 {
            id: n.id.clone(),
            value: n.value,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::KeyValue(n) => ControlNode::KeyValue {
            entries: n.entries.iter().map(|e| KeyValueEntry { label: e.label.clone(), value: e.value.clone() }).collect(),
        },
        UiControlNode::Slider(n) => ControlNode::Slider {
            id: n.id.clone(),
            value: n.value,
            min: n.min,
            max: n.max,
            step: n.step,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::NumberStepper(n) => ControlNode::NumberStepper {
            id: n.id.clone(),
            value: n.value,
            step: n.step,
            uniform: n.uniform,
            on_absolute: Some(n.on_absolute.clone()),
            on_delta: Some(n.on_delta.clone()),
        },
        UiControlNode::Ring(n) => ControlNode::Ring {
            id: n.id.clone(),
            t: n.t,
            disabled: n.disabled.unwrap_or(false),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::IconSelect(n) => ControlNode::IconSelect {
            id: n.id.clone(),
            value: n.value.clone(),
            uniform: n.uniform,
            classifier_kind: n.classifier_kind.clone(),
            on_change: Some(n.on_change.clone()),
        },
    }
}

fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<CommandDescriptor> {
    TreeItemAction {
        icon_id: action.icon_id.clone(),
        label: action.label.clone(),
        event: action.command.clone(),
        reveal_on_hover: action.reveal_on_hover.unwrap_or(false),
    }
}

fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<CommandDescriptor> {
    TreeSection {
        id: section.id.clone(),
        label: section.label.clone(),
        default_open: section.default_open.unwrap_or(true),
        items: section.items.iter().map(tree_item_to_widget).collect(),
    }
}

fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<CommandDescriptor> {
    TreeItem {
        id: item.id.clone(),
        label: item.label.clone(),
        description: item.description.clone(),
        icon_id: item.icon_id.clone(),
        selected: item.selected.unwrap_or(false),
        highlighted: false,
        default_open: item.default_open.unwrap_or(false),
        is_hidden: item.is_hidden.unwrap_or(false),
        event: item.command.clone(),
        hover_event: item.hover_command.clone(),
        unhover_event: item.unhover_command.clone(),
        actions: item
            .actions
            .as_ref()
            .map(|actions| actions.iter().map(tree_action_to_widget).collect())
            .unwrap_or_default(),
        draggable: item.draggable.unwrap_or(false),
        drag_data: item.drag_data.clone().unwrap_or_default(),
        control: item
            .control
            .as_ref()
            .map(|control| Box::new(control_to_widget_node(control))),
        children: item
            .items
            .as_ref()
            .map(|items| items.iter().map(tree_item_to_widget).collect())
            .unwrap_or_default(),
    }
}

fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<CommandDescriptor> {
    match control {
        UiControlNode::Button(n) => WidgetNode::Button {
            id: n.id.clone(),
            icon_id: Some(n.icon_id.clone()),
            label: n.label.clone(),
            event: Some(n.command.clone()),
        },
        UiControlNode::Input(n) => WidgetNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone(),
            commit: n.commit.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => WidgetNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|i| SelectItem { value: i.value.clone(), label: i.label.clone() }).collect(),
            placeholder: n.placeholder.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => WidgetNode::Toggle {
            id: n.id.clone(),
            icon_id: n.icon_id.clone(),
            pressed: n.pressed,
            text: n.text.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Vec3(n) => WidgetNode::Vec3 {
            id: n.id.clone(),
            value: n.value,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::KeyValue(n) => WidgetNode::KeyValue {
            entries: n.entries.iter().map(|e| KeyValueEntry { label: e.label.clone(), value: e.value.clone() }).collect(),
        },
        UiControlNode::Slider(n) => WidgetNode::Slider {
            id: n.id.clone(),
            value: n.value,
            min: n.min,
            max: n.max,
            step: n.step,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::NumberStepper(n) => WidgetNode::NumberStepper {
            id: n.id.clone(),
            value: n.value,
            step: n.step,
            uniform: n.uniform,
            on_absolute: Some(n.on_absolute.clone()),
            on_delta: Some(n.on_delta.clone()),
        },
        UiControlNode::Ring(n) => WidgetNode::Ring {
            id: n.id.clone(),
            t: n.t,
            disabled: n.disabled.unwrap_or(false),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::IconSelect(n) => WidgetNode::IconSelect {
            id: n.id.clone(),
            value: n.value.clone(),
            uniform: n.uniform,
            classifier_kind: n.classifier_kind.clone(),
            on_change: Some(n.on_change.clone()),
        },
    }
}

pub fn framework_widget_context<'a>(
    draw: &'a mut ui_wgpu::DrawList,
    overlay: Option<&'a mut ui_wgpu::DrawList>,
    atlas: &'a mut ui_wgpu::FontAtlas,
    icons: Option<&'a ui_wgpu::IconAtlas>,
    input: &'a mut ui_wgpu::InputState<CommandDescriptor>,
    theme: &'a Theme,
    scroll_offsets: &'a mut std::collections::HashMap<String, f32>,
    collapsed_sections: &'a mut std::collections::HashMap<String, bool>,
    open_selects: &'a mut std::collections::HashMap<String, bool>,
    interaction_maps: Option<&'a mut WidgetInteractionMaps<CommandDescriptor>>,
) -> FrameworkWidgetContext<'a> {
    WidgetContext {
        draw,
        overlay,
        atlas,
        icons,
        input,
        theme,
        scroll_offsets,
        collapsed_sections,
        open_selects,
        interaction_maps,
    }
}
// #endregion interpreter
}

pub mod plugin_bridge {
// #region plugin_bridge
//! 🔌 JS bridge for wasm-bindgen plugin modules.

use js_sys::{Array, Function, Reflect};
use semio_framework_core::{PluginManifest, ToolNode, UiNode, ViewState, WindowEngagement};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone)]
pub struct PluginBridgeEntry {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    handle: Rc<JsValue>,
}

impl PluginBridgeEntry {
    pub fn from_js(plugin_id: String, handle: JsValue) -> Result<Self, String> {
        let manifest_fn = Reflect::get(&handle, &JsValue::from_str("manifest"))
            .map_err(|_| "missing manifest")?;
        let manifest_fn: Function = manifest_fn.dyn_into().map_err(|_| "manifest not fn")?;
        let manifest_json = manifest_fn
            .call0(&JsValue::NULL)
            .map_err(|_| "manifest call failed")?
            .as_string()
            .ok_or("manifest not string")?;
        let manifest: PluginManifest =
            serde_json::from_str(&manifest_json).map_err(|err| format!("manifest parse: {err}"))?;
        let _create_app = get_fn(&handle, "createApp")?;
        let _render = get_fn(&handle, "render")?;
        Ok(Self {
            plugin_id,
            manifest,
            handle: Rc::new(handle),
        })
    }

    pub async fn create_app(&self, app_id: &str) -> Result<u32, String> {
        let create_app = get_fn(self.handle.as_ref(), "createApp")?;
        let result = create_app
            .call1(&JsValue::NULL, &JsValue::from_str(app_id))
            .map_err(|_| "create_app failed")?;
        if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            let resolved = JsFuture::from(promise.clone())
                .await
                .map_err(|_| "create_app promise failed")?;
            resolved.as_f64().map(|v| v as u32).ok_or("create_app not number".into())
        } else {
            result.as_f64().map(|v| v as u32).ok_or("create_app not number".into())
        }
    }

    pub fn destroy_app(&self, instance_id: u32) {
        if let Ok(destroy) = Reflect::get(self.handle.as_ref(), &JsValue::from_str("destroyApp"))
            .and_then(|v| v.dyn_into::<Function>())
        {
            let _ = destroy.call1(&JsValue::NULL, &JsValue::from_f64(instance_id as f64));
        }
    }

    pub async fn handle_command(
        &self,
        instance_id: u32,
        command_json: &str,
        view_state: &ViewState,
    ) -> Result<Vec<String>, String> {
        let handle = Reflect::get(self.handle.as_ref(), &JsValue::from_str("handleCommand"))
            .ok()
            .and_then(|v| v.dyn_into::<Function>().ok());
        let Some(handle) = handle else {
            return Ok(Vec::new());
        };
        let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
        let result = handle
            .call3(
                &JsValue::NULL,
                &JsValue::from_f64(instance_id as f64),
                &JsValue::from_str(command_json),
                &JsValue::from_str(&view_json),
            )
            .map_err(|_| "handle_command failed")?;
        let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            JsFuture::from(promise.clone())
                .await
                .map_err(|_| "handle_command promise failed")?
        } else {
            result
        };
        if let Some(array) = resolved.dyn_ref::<Array>() {
            let mut ops = Vec::new();
            for index in 0..array.length() {
                if let Some(value) = array.get(index).as_string() {
                    ops.push(value);
                }
            }
            return Ok(ops);
        }
        if let Some(text) = resolved.as_string() {
            let parsed: Vec<String> = serde_json::from_str(&text).unwrap_or_default();
            return Ok(parsed);
        }
        Ok(Vec::new())
    }

    pub async fn render(
        &self,
        instance_id: u32,
        body_key: &str,
        view_state: &ViewState,
    ) -> Result<UiNode, String> {
        let render = get_fn(self.handle.as_ref(), "render")?;
        let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
        let result = render
            .call3(
                &JsValue::NULL,
                &JsValue::from_f64(instance_id as f64),
                &JsValue::from_str(body_key),
                &JsValue::from_str(&view_json),
            )
            .map_err(|_| "render failed")?;
        let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            JsFuture::from(promise.clone())
                .await
                .map_err(|_| "render promise failed")?
        } else {
            result
        };
        let json = resolved.as_string().ok_or("render not string")?;
        serde_json::from_str(&json).map_err(|err| format!("render parse: {err}"))
    }

    pub async fn tools(
        &self,
        instance_id: u32,
        view_state: &ViewState,
    ) -> Result<Vec<ToolNode>, String> {
        let tools = Reflect::get(self.handle.as_ref(), &JsValue::from_str("tools"))
            .ok()
            .and_then(|v| v.dyn_into::<Function>().ok());
        let Some(tools) = tools else {
            return Ok(Vec::new());
        };
        let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
        let result = tools
            .call2(
                &JsValue::NULL,
                &JsValue::from_f64(instance_id as f64),
                &JsValue::from_str(&view_json),
            )
            .map_err(|_| "tools failed")?;
        let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            JsFuture::from(promise.clone())
                .await
                .map_err(|_| "tools promise failed")?
        } else {
            result
        };
        let json = resolved.as_string().ok_or("tools not string")?;
        serde_json::from_str(&json).map_err(|err| format!("tools parse: {err}"))
    }

    pub async fn window_engagements(
        &self,
        instance_id: u32,
        view_state: &ViewState,
    ) -> Result<HashMap<String, WindowEngagement>, String> {
        let engagements = Reflect::get(self.handle.as_ref(), &JsValue::from_str("windowEngagements"))
            .ok()
            .and_then(|v| v.dyn_into::<Function>().ok());
        let Some(engagements) = engagements else {
            return Ok(HashMap::new());
        };
        let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
        let result = engagements
            .call2(
                &JsValue::NULL,
                &JsValue::from_f64(instance_id as f64),
                &JsValue::from_str(&view_json),
            )
            .map_err(|_| "window_engagements failed")?;
        let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
            JsFuture::from(promise.clone())
                .await
                .map_err(|_| "window_engagements promise failed")?
        } else {
            result
        };
        let json = resolved.as_string().ok_or("window_engagements not string")?;
        serde_json::from_str(&json).map_err(|err| format!("window_engagements parse: {err}"))
    }
}

fn get_fn(obj: &JsValue, key: &str) -> Result<Function, String> {
    Reflect::get(obj, &JsValue::from_str(key))
        .map_err(|_| format!("missing {key}"))?
        .dyn_into()
        .map_err(|_| format!("{key} not fn"))
}

pub fn parse_plugin_entries(plugins: JsValue) -> Result<Vec<PluginBridgeEntry>, String> {
    let array = plugins.dyn_into::<Array>().map_err(|_| "plugins not array")?;
    let mut entries = Vec::new();
    for index in 0..array.length() {
        let item = array.get(index);
        let plugin_id = Reflect::get(&item, &JsValue::from_str("pluginId"))
            .ok()
            .and_then(|v| v.as_string())
            .ok_or("pluginId missing")?;
        let handle = Reflect::get(&item, &JsValue::from_str("handle")).map_err(|_| "handle missing")?;
        entries.push(PluginBridgeEntry::from_js(plugin_id, handle)?);
    }
    Ok(entries)
}

pub fn is_studio_mode(plugin_filter: &str) -> bool {
    plugin_filter == "s"
}

pub fn filter_plugins(entries: Vec<PluginBridgeEntry>, plugin_filter: &str) -> Vec<PluginBridgeEntry> {
    if is_studio_mode(plugin_filter) {
        entries
    } else {
        entries
            .into_iter()
            .filter(|entry| entry.plugin_id == plugin_filter)
            .collect()
    }
}
// #endregion plugin_bridge
}

pub mod scenes {
// #region scenes
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
        "node-graph" => engine_canvas::node_graph_wheel(
            &scene.surface_id,
            &scene.controller_id,
            inner,
            x,
            y,
            delta,
            ctrl,
        ),
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
                &scene.surface_id,
                &scene.controller_id,
                inner,
                x,
                y,
                false,
                false,
                false,
            ));
        }
        "text-editor" if down => {
            commands.extend(engine_canvas::text_editor_pointer_move(scene, inner, x, y));
        }
        "node-graph" | "text-editor" if !down => {
            commands.extend(match scene.component_kind.as_str() {
                "node-graph" => engine_canvas::node_graph_pointer_move(
                    &scene.surface_id,
                    &scene.controller_id,
                    inner,
                    x,
                    y,
                    false,
                    false,
                    false,
                ),
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
                    &scene.surface_id,
                    &scene.controller_id,
                    inner,
                    x,
                    y,
                    button,
                    shift,
                    false,
                    false,
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
                    &scene.surface_id,
                    &scene.controller_id,
                    inner,
                    x,
                    y,
                    shift,
                    false,
                    false,
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
    node_graph_states: &mut HashMap<String, NodeGraphSurface>,
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
        "node-graph" => render_node_graph(scene, bounds, ctx, gpu, node_graph_states),
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
#[derive(Clone, Debug)]
pub struct NodeGraphSurface {
    pub bounds: Rect,
    pub controller_id: String,
}
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
    node_graph_states: &mut HashMap<String, NodeGraphSurface>,
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
    node_graph_states.insert(
        scene.surface_id.clone(),
        NodeGraphSurface {
            bounds: inner,
            controller_id: scene.controller_id.clone(),
        },
    );
    engine_canvas::paint_node_graph(gpu, ctx, scene, inner);
    engine_canvas::paint_node_graph_labels(ctx, scene, inner);
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
// #endregion scenes
}

pub mod shell {
// #region shell
//! 🖥️ OS shell chrome — navbar, footer, floating panels, overlays, and studio mode.

use crate::dock::{
    compute_dock_drop_zone, dock_from_window_layout, drop_zone_indicator_rect, parse_path,
    DockDragKind, DockDragPayload, DockDragState, DockDropZone, DockRenderContext, DockState,
};
use crate::interpreter::{framework_widget_context, render_ui_node};
use crate::scenes::{clear_graph_node_context, resolve_graph_context_command, seed_vfs_expanded, toggle_vfs_row_expanded, vfs_selection_for_click, NodeGraphSurface};
use infinite_world::{
    fetch_pending_glb_meshes, fetch_pending_reference_images, handle_world3d_paint_commands,
    handle_world3d_pointer_button,
    handle_world3d_pointer_drag, handle_world3d_pointer_move, handle_world3d_wheel, World3dState,
};
use crate::plugin_bridge::{is_studio_mode, PluginBridgeEntry};
use semio_framework_core::{
    app_hierarchy_label, app_window_hierarchy_label, AppDefinition, CommandDescriptor, ExampleDefinition, ModeDefinition, PanelTabDefinition,
    ToolNode, UiButtonNode, UiNode, UiSelectItem, UiSelectNode, UiStackNode, UiTextNode, ViewState, WindowEngagement,
    WindowEngagementControl, WindowEngagementInput, WindowEngagementOption, WindowMeasure,
};
use semio_framework_core::layout::{
    WindowEngagementPossible, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ui_wgpu::{
    chrome_item_bg, chrome_item_text, draw_text, push_chrome_group_border, DrawList, DragAxis, FontAtlas, GlassTier,
    HitKind, HitTarget, IconAtlas, InputState, PointerModifiers, Rect, Rgba, Theme, TreeDragState, TreeDropPosition,
    WidgetInteractionMaps,
};

const S_HOME_APP_ID: &str = "home";
const S_PLAY_APP_ID: &str = "studio";
const S_PLAY_CONTROLLER_ID: &str = "s-play";
const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";
const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID: &str = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID: &str = "framework.display.layout";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID: &str = "framework.settings.general";
const DEFAULT_MEASURES_RAIL_WIDTH: f32 = 240.0;
const DEFAULT_ENGAGEMENT_RAIL_WIDTH: f32 = 280.0;
const CHROME_ICON_TINY: f32 = 14.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LeftPanelKind {
    #[default]
    Workbench,
    Display,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RightPanelKind {
    #[default]
    Details,
    Settings,
}

#[derive(Clone, Debug)]
pub struct SearchPaletteItem {
    pub id: String,
    pub label: String,
    pub group: String,
    pub command: Option<CommandDescriptor>,
    pub action: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ShellFindItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub surface_id: String,
    pub node_id: String,
}

thread_local! {
    static FIND_ITEM_SINK: std::cell::RefCell<Vec<ShellFindItem>> = std::cell::RefCell::new(Vec::new());
    static CONTEXT_MENU_SINK: std::cell::RefCell<Vec<ContextMenuItem>> = std::cell::RefCell::new(Vec::new());
}

pub fn push_find_item(item: ShellFindItem) {
    FIND_ITEM_SINK.with(|cell| cell.borrow_mut().push(item));
}

pub fn take_find_items() -> Vec<ShellFindItem> {
    FIND_ITEM_SINK.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

pub fn push_context_menu_item(item: ContextMenuItem) {
    CONTEXT_MENU_SINK.with(|cell| cell.borrow_mut().push(item));
}

pub fn take_context_menu_items() -> Vec<ContextMenuItem> {
    CONTEXT_MENU_SINK.with(|cell| std::mem::take(&mut *cell.borrow_mut()))
}

//#region ShellTypes
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioProgramEntry {
    pub plugin_id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub hierarchy: Vec<String>,
    pub yields: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedAppEntry {
    pub id: String,
    pub plugin_id: String,
    pub instance_id: u32,
    pub app_id: String,
    pub label: String,
    pub hierarchy: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioPanelState {
    pub active_panel_tab: String,
    pub programs: Vec<StudioProgramEntry>,
    pub spawned_apps: Vec<SpawnedAppEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_spawned_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
    pub command: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuState {
    pub x: f32,
    pub y: f32,
    pub items: Vec<ContextMenuItem>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum OverlayState {
    #[default]
    None,
    ThemeSelect,
    Search,
    Find,
    Dropdown(String),
}

#[derive(Clone, Debug, Default)]
pub struct RightClickState {
    pub pending: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct ActiveSession {
    pub plugin_id: String,
    pub instance_id: u32,
    pub app: AppDefinition,
    pub view_state: ViewState,
}

pub struct ShellState {
    pub plugins: Vec<PluginBridgeEntry>,
    pub plugin_filter: String,
    pub studio_mode: bool,
    pub session: Option<ActiveSession>,
    pub window_ui: HashMap<String, UiNode>,
    pub panel_ui: HashMap<String, UiNode>,
    pub spawned_ui: Option<UiNode>,
    pub active_window_id: Option<String>,
    pub left_panel_open: bool,
    pub right_panel_open: bool,
    pub left_panel_width: f32,
    pub right_panel_width: f32,
    pub scroll_offsets: HashMap<String, f32>,
    pub overlay_state: OverlayState,
    pub collapsed_sections: HashMap<String, bool>,
    pub open_selects: HashMap<String, bool>,
    pub active_right_tab: Option<String>,
    pub context_menu: Option<ContextMenuState>,
    pub search_open: bool,
    pub find_open: bool,
    pub theme_id: String,
    pub right_click: RightClickState,
    pub uri_history: Vec<String>,
    pub uri_index: usize,
    pub panel_resize_origin_width: f32,
    pub error: Option<String>,
    pub screen_w: f32,
    pub screen_h: f32,
    pub world3d_states: HashMap<String, World3dState>,
    pub node_graph_states: HashMap<String, NodeGraphSurface>,
    pub dock: DockState,
    pub active_left_kind: LeftPanelKind,
    pub active_right_kind: RightPanelKind,
    pub search_query: String,
    pub search_selected: usize,
    pub find_query: String,
    pub split_resize_path: Option<Vec<usize>>,
    pub split_resize_index: usize,
    pub split_resize_axis_total: f32,
    pub active_example_id: Option<String>,
    pub active_left_tab: Option<String>,
    pub find_items: Vec<ShellFindItem>,
    pub find_selected: usize,
    pub engagement_expanded: HashMap<String, bool>,
    pub engagement_activated: HashMap<String, bool>,
    pub measures_folded: HashMap<String, bool>,
    pub measures_expanded: HashMap<String, bool>,
    pub measures_width: HashMap<String, f32>,
    pub measures_resize_origin_width: f32,
    pub engagement_inputs: HashMap<String, String>,
    pub compact_mode: bool,
    pub expertise: String,
    pub tree_drag: Option<TreeDragState>,
    pub tree_hovered_id: Option<String>,
    pub widget_maps: WidgetInteractionMaps<CommandDescriptor>,
    pub pending_tree_drag: Option<(String, HashMap<String, String>)>,
    pub tree_drag_origin: (f32, f32),
    pub dock_drag: Option<DockDragState>,
    pub pending_dock_drag: Option<(DockDragPayload, (f32, f32))>,
    pub dock_drag_snapshot: Option<semio_framework_core::layout::WindowLayout>,
    pub dock_canvas_bounds: Rect,
    pub dock_drop_tab_bars: Vec<(Vec<usize>, Rect, Vec<f32>)>,
    pub dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    pub layout_override: Option<semio_framework_core::layout::WindowLayout>,
    pub split_resize_origin: Vec<f32>,
    pub split_resize_secondary_path: Option<Vec<usize>>,
    pub split_resize_secondary_index: usize,
    pub split_resize_secondary_axis_total: f32,
    pub split_resize_secondary_origin: Vec<f32>,
    pub measures_resize_window_id: Option<String>,
    pub deferred_commands: Vec<CommandDescriptor>,
    pub active_tools: Vec<ToolNode>,
    pub window_engagements: HashMap<String, WindowEngagement>,
    pub tool_collection_expanded: HashMap<String, bool>,
}
//#endregion ShellTypes

//#region ShellLifecycle
impl ShellState {
    pub fn new(plugins: Vec<PluginBridgeEntry>, plugin_filter: String) -> Self {
        let studio_mode = is_studio_mode(&plugin_filter);
        Self {
            plugins,
            plugin_filter,
            studio_mode,
            session: None,
            window_ui: HashMap::new(),
            panel_ui: HashMap::new(),
            spawned_ui: None,
            active_window_id: None,
            left_panel_open: true,
            right_panel_open: true,
            left_panel_width: 280.0,
            right_panel_width: 320.0,
            scroll_offsets: HashMap::new(),
            overlay_state: OverlayState::None,
            collapsed_sections: HashMap::new(),
            open_selects: HashMap::new(),
            active_right_tab: None,
            context_menu: None,
            search_open: false,
            find_open: false,
            theme_id: "system".into(),
            right_click: RightClickState::default(),
            uri_history: vec!["os://home".into()],
            uri_index: 0,
            panel_resize_origin_width: 280.0,
            error: None,
            screen_w: 1280.0,
            screen_h: 720.0,
            world3d_states: HashMap::new(),
            node_graph_states: HashMap::new(),
            dock: DockState::default(),
            active_left_kind: LeftPanelKind::Workbench,
            active_right_kind: RightPanelKind::Details,
            search_query: String::new(),
            search_selected: 0,
            find_query: String::new(),
            split_resize_path: None,
            split_resize_index: 0,
            split_resize_axis_total: 1.0,
            active_example_id: None,
            active_left_tab: None,
            find_items: Vec::new(),
            find_selected: 0,
            engagement_expanded: HashMap::new(),
            engagement_activated: HashMap::new(),
            measures_folded: HashMap::new(),
            measures_expanded: HashMap::new(),
            measures_width: HashMap::new(),
            measures_resize_origin_width: DEFAULT_MEASURES_RAIL_WIDTH,
            engagement_inputs: HashMap::new(),
            compact_mode: false,
            expertise: "standard".into(),
            tree_drag: None,
            tree_hovered_id: None,
            widget_maps: WidgetInteractionMaps::default(),
            pending_tree_drag: None,
            tree_drag_origin: (0.0, 0.0),
            dock_drag: None,
            pending_dock_drag: None,
            dock_drag_snapshot: None,
            dock_canvas_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            dock_drop_tab_bars: Vec::new(),
            dock_drop_bodies: Vec::new(),
            layout_override: None,
            split_resize_origin: Vec::new(),
            split_resize_secondary_path: None,
            split_resize_secondary_index: 0,
            split_resize_secondary_axis_total: 1.0,
            split_resize_secondary_origin: Vec::new(),
            measures_resize_window_id: None,
            deferred_commands: Vec::new(),
            active_tools: Vec::new(),
            window_engagements: HashMap::new(),
            tool_collection_expanded: HashMap::new(),
        }
    }

    pub fn build_studio_programs(&self) -> Vec<StudioProgramEntry> {
        self.plugins
            .iter()
            .flat_map(|plugin| {
                plugin.manifest.programs.iter().map(|program| StudioProgramEntry {
                    plugin_id: plugin.plugin_id.clone(),
                    program_id: program.program_id.clone(),
                    app_id: program.app_id.clone(),
                    label: program.label.clone(),
                    hierarchy: program.hierarchy.clone(),
                    yields: program.yields.clone(),
                })
            })
            .collect()
    }

    pub fn panel_state_from_view(view_state: &ViewState) -> Option<StudioPanelState> {
        view_state
            .panel_json
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok())
    }

    pub fn panel_json(state: &StudioPanelState) -> String {
        serde_json::to_string(state).unwrap_or_default()
    }

    pub async fn boot(&mut self) -> Result<(), String> {
        if self.studio_mode {
            let s_plugin = self
                .plugins
                .iter()
                .find(|p| p.plugin_id == "s")
                .ok_or("s studio plugin missing")?;
            let s_app = s_plugin
                .manifest
                .apps
                .iter()
                .find(|app| app.id == S_HOME_APP_ID)
                .or_else(|| s_plugin.manifest.apps.first())
                .ok_or("s home app missing")?
                .clone();
            let programs = self.build_studio_programs();
            let panel_state = StudioPanelState {
                active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
                programs,
                spawned_apps: vec![],
                active_spawned_id: None,
            };
            let instance_id = s_plugin.create_app(&s_app.id).await?;
            let view_state = ViewState {
                active_mode_id: s_app.default_mode_id.clone().or_else(|| s_app.modes.first().map(|m| m.id.clone())),
                active_window_kind_id: s_app.window_kinds.first().map(|w| w.id.clone()),
                selection_json: None,
                panel_json: Some(Self::panel_json(&panel_state)),
            };
            self.active_window_id = s_app.window_kinds.first().map(|w| w.id.clone());
            self.session = Some(ActiveSession {
                plugin_id: s_plugin.plugin_id.clone(),
                instance_id,
                app: s_app,
                view_state,
            });
        } else if let Some(plugin) = self.plugins.first() {
            let app = plugin
                .manifest
                .apps
                .first()
                .ok_or("plugin has no apps")?
                .clone();
            let instance_id = plugin.create_app(&app.id).await?;
            self.active_window_id = app.window_kinds.first().map(|w| w.id.clone());
            self.session = Some(ActiveSession {
                plugin_id: plugin.plugin_id.clone(),
                instance_id,
                app,
                view_state: ViewState {
                    active_mode_id: None,
                    active_window_kind_id: self.active_window_id.clone(),
                    selection_json: None,
                    panel_json: None,
                },
            });
        }
        self.sync_dock();
        self.sync_session_chrome();
        self.refresh_ui().await
    }

    fn sync_session_chrome(&mut self) {
        let Some(session) = &self.session else {
            return;
        };
        let examples = self
            .plugins
            .iter()
            .find(|p| p.plugin_id == session.plugin_id)
            .map(|p| p.manifest.examples.as_slice())
            .unwrap_or(&[]);
        if examples.is_empty() {
            self.active_example_id = None;
        } else {
            let current = self.active_example_id.clone();
            self.active_example_id = current
                .filter(|id| examples.iter().any(|ex| &ex.id == id))
                .or_else(|| examples.first().map(|ex| ex.id.clone()));
        }
        if let Some(mode_id) = session.view_state.active_mode_id.clone() {
            let _ = mode_id;
        }
    }

    fn active_plugin_examples(&self) -> Vec<ExampleDefinition> {
        let Some(session) = &self.session else {
            return Vec::new();
        };
        self.plugins
            .iter()
            .find(|p| p.plugin_id == session.plugin_id)
            .map(|p| p.manifest.examples.clone())
            .unwrap_or_default()
    }

    fn synthetic_panel_tab(id: &str, label: &str, group: &str) -> PanelTabDefinition {
        PanelTabDefinition {
            id: id.into(),
            label: label.into(),
            group: group.into(),
            body_key: String::new(),
        }
    }

    fn sync_dock(&mut self) {
        if let Some(session) = &self.session {
            if let Some(layout) = self.layout_override.clone() {
                self.dock.root = dock_from_window_layout(&layout.root);
                self.dock.active_window_id = self
                    .active_window_id
                    .clone()
                    .or_else(|| session.view_state.active_window_kind_id.clone());
            } else {
                self.dock = DockState::from_app(&session.app, self.active_window_id.as_deref());
            }
            if let Some(id) = &self.active_window_id {
                self.dock.sync_active_window(id);
            }
        }
    }

    fn persist_dock_layout(&mut self) {
        self.layout_override = Some(self.dock.to_window_layout());
        self.dock_drag_snapshot = None;
    }

    fn restore_dock_drag_snapshot(&mut self) {
        if let Some(layout) = self.dock_drag_snapshot.take() {
            self.layout_override = Some(layout);
            self.sync_dock();
        }
    }

    fn begin_pending_dock_drag(&mut self, payload: DockDragPayload, x: f32, y: f32) {
        self.dock_drag_snapshot = Some(self.dock.to_window_layout());
        self.pending_dock_drag = Some((payload, (x, y)));
    }

    fn dock_tab_bars_for_drop(
        &self,
        atlas: &mut FontAtlas,
        theme: &Theme,
        canvas: Rect,
        labels: &HashMap<String, String>,
    ) -> Vec<(Vec<usize>, Rect, Vec<f32>)> {
        self.dock
            .stack_tab_bar_rects(canvas, theme)
            .into_iter()
            .filter_map(|(path, rect)| {
                let windows = self.dock.stack_windows_at_path(&path)?;
                let widths: Vec<f32> = windows
                    .iter()
                    .map(|id| {
                        let label = labels.get(id).map(String::as_str).unwrap_or(id);
                        atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0
                    })
                    .collect();
                Some((path, rect, widths))
            })
            .collect()
    }

    pub async fn refresh_ui(&mut self) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        self.sync_dock();
        self.ensure_framework_panel_ui(&session);
        let plugin = self
            .plugins
            .iter()
            .find(|p| p.plugin_id == session.plugin_id)
            .ok_or("session plugin missing")?;
        self.window_ui.clear();
        for kind in &session.app.window_kinds {
            let node = plugin
                .render(session.instance_id, &kind.body_key, &session.view_state)
                .await?;
            self.window_ui.insert(kind.id.clone(), node);
        }
        self.panel_ui.clear();
        for tab in &session.app.panel_tabs {
            let node = plugin
                .render(session.instance_id, &tab.body_key, &session.view_state)
                .await?;
            self.panel_ui.insert(tab.id.clone(), node);
        }
        self.active_tools = plugin
            .tools(session.instance_id, &session.view_state)
            .await
            .unwrap_or_default();
        self.window_engagements = plugin
            .window_engagements(session.instance_id, &session.view_state)
            .await
            .unwrap_or_default();
        if self.studio_mode {
            if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
                if let Some(spawned) = panel
                    .active_spawned_id
                    .as_ref()
                    .and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id))
                {
                    if let Some(spawn_plugin) = self.plugins.iter().find(|p| p.plugin_id == spawned.plugin_id) {
                        let spawned_app = spawn_plugin
                            .manifest
                            .apps
                            .iter()
                            .find(|app| app.id == spawned.app_id);
                        if let Some(app) = spawned_app {
                            let body_key = app
                                .window_kinds
                                .first()
                                .map(|k| k.body_key.clone())
                                .unwrap_or_default();
                            let view_state = ViewState {
                                active_mode_id: app.default_mode_id.clone(),
                                active_window_kind_id: app.window_kinds.first().map(|w| w.id.clone()),
                                selection_json: None,
                                panel_json: None,
                            };
                            self.spawned_ui = Some(
                                spawn_plugin
                                    .render(spawned.instance_id, &body_key, &view_state)
                                    .await?,
                            );
                        }
                    }
                } else {
                    self.spawned_ui = None;
                }
            }
        }
        Ok(())
    }

    fn ensure_framework_panel_ui(&mut self, session: &ActiveSession) {
        let windows_ui = self.build_display_windows_ui(session);
        self.panel_ui
            .insert(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(), windows_ui);
        let layout_ui = self.build_display_layout_ui(session);
        self.panel_ui
            .insert(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID.into(), layout_ui);
        let settings_ui = self.build_settings_general_ui();
        self.panel_ui
            .insert(FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into(), settings_ui);
    }

    fn build_display_windows_ui(&self, session: &ActiveSession) -> UiNode {
        let items: Vec<UiNode> = session
            .app
            .window_kinds
            .iter()
            .map(|kind| {
                UiNode::Text(UiTextNode {
                    value: format!("{} — {}", kind.label, kind.id),
                    emphasize: None,
                    data_attributes: None,
                })
            })
            .collect();
        if items.is_empty() {
            return UiNode::Text(UiTextNode {
                value: "—".into(),
                emphasize: None,
                data_attributes: None,
            });
        }
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            children: items,
        })
    }

    fn build_display_layout_ui(&self, session: &ActiveSession) -> UiNode {
        let items: Vec<UiNode> = session
            .app
            .named_layouts
            .iter()
            .map(|layout| {
                UiNode::Button(UiButtonNode {
                    id: Some(format!("shell.layout.{}", layout.id)),
                    icon_id: layout.icon_id.clone().unwrap_or_else(|| "layout-grid".into()),
                    label: format!("{} ({})", layout.label, layout.origin),
                    command: CommandDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        command: "noop".into(),
                        args: None,
                    },
                    style: None,
                })
            })
            .collect();
        if items.is_empty() {
            return UiNode::Text(UiTextNode {
                value: "No saved layouts".into(),
                emphasize: None,
                data_attributes: None,
            });
        }
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            children: items,
        })
    }

    fn build_settings_general_ui(&self) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            children: vec![
                UiNode::Text(UiTextNode {
                    value: "General".into(),
                    emphasize: Some(true),
                    data_attributes: None,
                }),
                UiNode::Select(UiSelectNode {
                    id: "framework.settings.theme".into(),
                    value: self.theme_id.clone(),
                    items: vec![
                        UiSelectItem {
                            value: "system".into(),
                            label: "System".into(),
                        },
                        UiSelectItem {
                            value: "light".into(),
                            label: "Light".into(),
                        },
                        UiSelectItem {
                            value: "dark".into(),
                            label: "Dark".into(),
                        },
                    ],
                    placeholder: None,
                    on_change: CommandDescriptor {
                        controller_id: "framework".into(),
                        command: "setTheme".into(),
                        args: None,
                    },
                }),
                UiNode::Select(UiSelectNode {
                    id: "framework.settings.expertise".into(),
                    value: "standard".into(),
                    items: vec![
                        UiSelectItem {
                            value: "standard".into(),
                            label: "Standard".into(),
                        },
                        UiSelectItem {
                            value: "expert".into(),
                            label: "Expert".into(),
                        },
                    ],
                    placeholder: None,
                    on_change: CommandDescriptor {
                        controller_id: "framework".into(),
                        command: "setExpertise".into(),
                        args: None,
                    },
                }),
            ],
        })
    }
}
//#endregion ShellLifecycle

//#region ShellCommands
impl ShellState {
    pub async fn dispatch_command(&mut self, command: CommandDescriptor) -> Result<(), String> {
        if command.controller_id == "framework" {
            match command.command.as_str() {
                "setTheme" => {
                    if let Some(value) = command
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_str())
                    {
                        self.theme_id = value.to_string();
                    }
                    return Ok(());
                }
                "setExpertise" => {
                    if let Some(value) = command
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_str())
                    {
                        self.expertise = value.to_string();
                    }
                    return Ok(());
                }
                "setCompact" => {
                    if let Some(value) = command
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_bool())
                    {
                        self.compact_mode = value;
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let plugin = self
            .plugins
            .iter()
            .find(|p| {
                p.manifest
                    .apps
                    .iter()
                    .any(|app| app.controller_id == command.controller_id)
            })
            .or_else(|| self.plugins.iter().find(|p| p.plugin_id == session.plugin_id))
            .ok_or("command plugin missing")?;
        let command_json = serde_json::to_string(&command).map_err(|err| err.to_string())?;
        let ops = plugin
            .handle_command(session.instance_id, &command_json, &session.view_state)
            .await?;
        self.apply_ops(&ops).await
    }

    pub async fn apply_ops(&mut self, ops: &[String]) -> Result<(), String> {
        let mut view_state = self.session.as_ref().map(|s| s.view_state.clone());
        for op_json in ops {
            let op: serde_json::Value = serde_json::from_str(op_json).unwrap_or(serde_json::Value::Null);
            if op.get("op").and_then(|v| v.as_str()) == Some("setPanel") {
                if let Some(panel) = op.get("panel") {
                    if let Some(mut vs) = view_state.take() {
                        vs.panel_json = Some(panel.to_string());
                        view_state = Some(vs);
                    }
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("downloadMediaExport") {
                if let (Some(filename), Some(mime_type), Some(data)) = (
                    op.get("filename").and_then(|v| v.as_str()),
                    op.get("mimeType").and_then(|v| v.as_str()),
                    op.get("data").and_then(|v| v.as_str()),
                ) {
                    download_media_export(filename, mime_type, data);
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("spawnProgram") {
                if let (Some(program_id), Some(session)) = (op.get("programId").and_then(|v| v.as_str()), &self.session) {
                    self.spawn_program(program_id, session.view_state.clone()).await?;
                }
            }
        }
        if let (Some(mut session), Some(vs)) = (self.session.take(), view_state) {
            session.view_state = vs;
            self.session = Some(session);
            self.sync_session_chrome();
            self.refresh_ui().await?;
        }
        Ok(())
    }

    async fn spawn_program(&mut self, program_id: &str, mut view_state: ViewState) -> Result<(), String> {
        let programs = self.build_studio_programs();
        let Some(program) = programs.iter().find(|p| p.program_id == program_id).cloned() else {
            return Ok(());
        };
        let plugin = self
            .plugins
            .iter()
            .find(|p| p.plugin_id == program.plugin_id)
            .ok_or("spawn plugin missing")?;
        let instance_id = plugin.create_app(&program.app_id).await?;
        let mut panel = Self::panel_state_from_view(&view_state).unwrap_or(StudioPanelState {
            active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
            programs: programs.clone(),
            spawned_apps: vec![],
            active_spawned_id: None,
        });
        let spawned_id = format!("{}-{}", program.plugin_id, instance_id);
        panel.spawned_apps.push(SpawnedAppEntry {
            id: spawned_id.clone(),
            plugin_id: program.plugin_id.clone(),
            instance_id,
            app_id: program.app_id.clone(),
            label: program.label.clone(),
            hierarchy: program.hierarchy.clone(),
        });
        panel.active_spawned_id = Some(spawned_id);
        view_state.panel_json = Some(Self::panel_json(&panel));
        if let Some(session) = self.session.as_mut() {
            session.view_state = view_state;
        }
        Ok(())
    }
}
//#endregion ShellCommands

//#region ShellInput
impl ShellState {
    pub async fn handle_pointer_button(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        input: &mut InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.pointer_button = button;
        if !down {
            if self.dock_drag.is_some() {
                self.finish_dock_drag(x, y, input).await?;
            } else if let Some((payload, _)) = self.pending_dock_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if let Some(rest) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("dock.tab.")) {
                        if let Some((path_str_value, window_id)) = rest.split_once('.') {
                            if window_id == payload.window_id {
                                let path = parse_path(path_str_value);
                                self.dock.set_stack_active(&path, window_id);
                                self.active_window_id = Some(window_id.to_string());
                            }
                        }
                    }
                }
                self.restore_dock_drag_snapshot();
            }
            if self.tree_drag.is_some() {
                self.finish_tree_drag(x, y, input).await?;
            } else if let Some((item_id, _)) = self.pending_tree_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if hit.control_id.as_deref() == Some(&format!("tree.label.{item_id}")) {
                        self.dispatch_tree_selection(&item_id).await?;
                        if let Some(command) = hit.event.clone() {
                            self.dispatch_command(command).await?;
                        }
                    }
                }
            }
            if input.drag.active {
                let drag_target = input.drag.target_id.clone();
                self.dispatch_widget_drag_values(input).await?;
                input.end_drag();
                if drag_target
                    .as_deref()
                    .is_some_and(|id| id.starts_with("dock.split.") || id.starts_with("dock.corner."))
                {
                    self.persist_dock_layout();
                }
            }
            return Ok(());
        }
        if button == 2 {
            let hit = input.hit_at(x, y).cloned();
            self.open_context_menu(x, y, hit);
            self.right_click = RightClickState { pending: true, x, y };
            return Ok(());
        }
        if self.dismiss_overlays(x, y, input) {
            return Ok(());
        }
        if let Some(hit) = input.hit_at(x, y).cloned() {
            if hit.kind == HitKind::PanelResize {
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(window_id) = id.strip_prefix("shell.measures.resize.") {
                        self.measures_resize_window_id = Some(window_id.to_string());
                        self.measures_resize_origin_width = *self
                            .measures_width
                            .get(window_id)
                            .unwrap_or(&DEFAULT_MEASURES_RAIL_WIDTH);
                        input.begin_drag(
                            x,
                            y,
                            button,
                            hit.control_id.clone(),
                            Some(DragAxis::Horizontal),
                            Some(hit.kind),
                        );
                        return Ok(());
                    }
                }
                let width = if hit.control_id.as_deref() == Some("panel.resize.left") {
                    self.left_panel_width
                } else {
                    self.right_panel_width
                };
                self.panel_resize_origin_width = width;
                input.begin_drag(
                    x,
                    y,
                    button,
                    hit.control_id.clone(),
                    Some(DragAxis::Horizontal),
                    Some(hit.kind),
                );
                return Ok(());
            }
            if matches!(hit.kind, HitKind::DockSplit | HitKind::DockJoinCorner) {
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(rest) = id.strip_prefix("dock.corner.r/") {
                        if let Some((row_part, col_part)) = rest.split_once("/c/") {
                            if let Some((row_path_str, row_index_str)) = row_part.rsplit_once('/') {
                                if let Some((col_path_str, col_index_str)) = col_part.rsplit_once('/') {
                                    let row_path = parse_path(row_path_str);
                                    let col_path = parse_path(col_path_str);
                                    self.split_resize_path = Some(row_path.clone());
                                    self.split_resize_index = row_index_str.parse().unwrap_or(0);
                                    self.split_resize_secondary_path = Some(col_path.clone());
                                    self.split_resize_secondary_index = col_index_str.parse().unwrap_or(0);
                                    self.split_resize_origin = self.dock.begin_split_drag(&row_path);
                                    self.split_resize_secondary_origin = self.dock.begin_split_drag(&col_path);
                                    self.split_resize_axis_total = self
                                        .dock
                                        .split_axis_extent(&row_path, self.dock_canvas_bounds)
                                        .unwrap_or(self.dock_canvas_bounds.w);
                                    self.split_resize_secondary_axis_total = self
                                        .dock
                                        .split_axis_extent(&col_path, self.dock_canvas_bounds)
                                        .unwrap_or(self.dock_canvas_bounds.h);
                                    input.begin_drag(
                                        x,
                                        y,
                                        button,
                                        Some(id.to_string()),
                                        Some(DragAxis::Both),
                                        Some(hit.kind),
                                    );
                                    return Ok(());
                                }
                            }
                        }
                    }
                    if let Some(rest) = id.strip_prefix("dock.split.") {
                        if let Some((path_str, index_str)) = rest.rsplit_once('.') {
                            let path = parse_path(path_str);
                            let index: usize = index_str.parse().unwrap_or(0);
                            self.split_resize_path = Some(path.clone());
                            self.split_resize_index = index;
                            self.split_resize_origin = self.dock.begin_split_drag(&path);
                            self.split_resize_axis_total = self
                                .dock
                                .split_axis_extent(&path, self.dock_canvas_bounds)
                                .unwrap_or_else(|| {
                                    match hit.drag_axis {
                                        Some(DragAxis::Vertical) => self.dock_canvas_bounds.h,
                                        _ => self.dock_canvas_bounds.w,
                                    }
                                });
                            input.begin_drag(
                                x,
                                y,
                                button,
                                Some(id.to_string()),
                                hit.drag_axis,
                                Some(hit.kind),
                            );
                            return Ok(());
                        }
                    }
                }
            }
            if self.handle_shell_hit(&hit).await? {
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if let Some(rest) = id.strip_prefix("dock.tab.") {
                    if let Some((path_str_value, window_id)) = rest.split_once('.') {
                        let path = parse_path(path_str_value);
                        let tab_index = self.dock.tab_index(&path, window_id).unwrap_or(0);
                        let ghost_label = self
                            .session
                            .as_ref()
                            .and_then(|s| {
                                s.app
                                    .window_kinds
                                    .iter()
                                    .find(|k| k.id == window_id)
                                    .map(|k| k.label.clone())
                            })
                            .unwrap_or_else(|| window_id.to_string());
                        self.begin_pending_dock_drag(
                            DockDragPayload {
                                kind: DockDragKind::Tab,
                                window_id: window_id.to_string(),
                                source_path: path,
                                tab_index,
                                ghost_label,
                            },
                            x,
                            y,
                        );
                        return Ok(());
                    }
                }
                if let Some(path_str_value) = id.strip_prefix("dock.stack.") {
                    let path = parse_path(path_str_value);
                    let windows = self.dock.stack_windows_at_path(&path).unwrap_or_default();
                    let active = windows
                        .iter()
                        .find(|wid| self.active_window_id.as_deref() == Some(wid.as_str()))
                        .or_else(|| windows.first())
                        .cloned()
                        .unwrap_or_default();
                    if !active.is_empty() {
                        let tab_index = self.dock.tab_index(&path, &active).unwrap_or(0);
                        let ghost_label = self
                            .session
                            .as_ref()
                            .and_then(|s| {
                                s.app
                                    .window_kinds
                                    .iter()
                                    .find(|k| k.id == active)
                                    .map(|k| k.label.clone())
                            })
                            .unwrap_or_else(|| active.clone());
                        self.begin_pending_dock_drag(
                            DockDragPayload {
                                kind: DockDragKind::Stack,
                                window_id: active,
                                source_path: path,
                                tab_index,
                                ghost_label,
                            },
                            x,
                            y,
                        );
                        return Ok(());
                    }
                }
            }
            if hit.kind == HitKind::Slider {
                if let Some(id) = hit.control_id.clone() {
                    input.begin_drag(x, y, button, Some(id), hit.drag_axis, Some(hit.kind));
                    return Ok(());
                }
            }
            if hit.kind == HitKind::Select || hit.kind == HitKind::Toggle || hit.kind == HitKind::DropdownItem {
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if id.contains(".vfs.") && !id.contains(".chevron.") {
                    if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.") {
                        let additive = input.modifiers.meta || input.modifiers.ctrl;
                        let shift = input.modifiers.shift;
                        let ordered = vec![row_id.to_string()];
                        let ids = vfs_selection_for_click(surface_id, row_id, &ordered, shift, additive);
                        self.dispatch_command(CommandDescriptor {
                            controller_id: self
                                .session
                                .as_ref()
                                .map(|s| s.app.controller_id.clone())
                                .unwrap_or_default(),
                            command: "selectRows".into(),
                            args: Some(serde_json::json!({ "surfaceId": surface_id, "ids": ids })),
                        })
                        .await?;
                        return Ok(());
                    }
                }
            }
            if let Some(drag_data) = hit.drag_data.clone() {
                if hit.control_id.as_deref().is_some_and(|id| id.starts_with("tree.label.")) {
                    if let Some(item_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                        self.tree_drag_origin = (x, y);
                        self.pending_tree_drag = Some((item_id.to_string(), drag_data));
                        return Ok(());
                    }
                }
            }
            if let Some(command) = hit.event.clone() {
                self.dispatch_command(command).await?;
            } else if hit.kind == HitKind::Input {
                if let Some(id) = &hit.control_id {
                    let seed = self
                        .widget_maps
                        .input_metas
                        .get(id)
                        .map(|meta| meta.value.as_str())
                        .unwrap_or("");
                    input.focus_input(id, seed);
                }
            }
        }
        self.flush_deferred_commands().await?;
        Ok(())
    }

    pub fn handle_pointer_move(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
    ) {
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.update_hover(x, y);
        self.update_tree_hover(input);
        if let Some((ref item_id, ref drag_data)) = self.pending_tree_drag {
            if down {
                let dx = x - self.tree_drag_origin.0;
                let dy = y - self.tree_drag_origin.1;
                if self.tree_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    self.tree_drag = Some(TreeDragState {
                        source_id: item_id.clone(),
                        drag_data: drag_data.clone(),
                        x,
                        y,
                        drop_target_id: None,
                        drop_position: TreeDropPosition::Inside,
                    });
                    self.pending_tree_drag = None;
                }
            }
        }
        if let Some(drag) = &mut self.tree_drag {
            drag.x = x;
            drag.y = y;
            if let Some(hit) = input.hit_at(x, y) {
                if let Some(target_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                    drag.drop_target_id = Some(target_id.to_string());
                    let rel = (y - hit.rect.y) / hit.rect.h.max(1.0);
                    drag.drop_position = if rel < 0.25 {
                        TreeDropPosition::Before
                    } else if rel > 0.75 {
                        TreeDropPosition::After
                    } else {
                        TreeDropPosition::Inside
                    };
                } else if hit.kind == HitKind::World3d || hit.kind == HitKind::Window {
                    drag.drop_target_id = hit.control_id.clone();
                    drag.drop_position = TreeDropPosition::Inside;
                } else {
                    drag.drop_target_id = None;
                }
            }
        }
        if let Some((payload, origin)) = &self.pending_dock_drag {
            if down {
                let dx = x - origin.0;
                let dy = y - origin.1;
                if self.dock_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    let payload = payload.clone();
                    let origin = *origin;
                    self.pending_dock_drag = None;
                    self.dock.remove_window(&payload.window_id);
                    self.dock_drag = Some(DockDragState {
                        payload,
                        x: origin.0,
                        y: origin.1,
                        drop_zone: None,
                    });
                }
            }
        }
        if let Some(drag) = &mut self.dock_drag {
            drag.x = x;
            drag.y = y;
            drag.drop_zone = compute_dock_drop_zone(
                x,
                y,
                &self.dock_drop_tab_bars,
                &self.dock_drop_bodies,
                self.dock_canvas_bounds,
            );
        }
        if input.drag.active {
            input.update_drag(x, y);
            if let Some(id) = input.drag.target_id.as_deref() {
                let dx = x - input.drag.start_x;
                let dy = y - input.drag.start_y;
                match id {
                    id if id.starts_with("shell.measures.resize.") => {
                        if let Some(window_id) = self.measures_resize_window_id.clone() {
                            let next = (self.measures_resize_origin_width - dx).clamp(160.0, 640.0);
                            self.measures_width.insert(window_id, next);
                        }
                    }
                    "panel.resize.left" => {
                        self.left_panel_width = (self.panel_resize_origin_width + dx)
                            .clamp(theme.panel_min_width, theme.panel_max_width);
                    }
                    "panel.resize.right" => {
                        self.right_panel_width = (self.panel_resize_origin_width - dx)
                            .clamp(theme.panel_min_width, theme.panel_max_width);
                    }
                    dock_id if dock_id.starts_with("dock.corner.") => {
                        if let Some(path) = self.split_resize_path.clone() {
                            self.dock.apply_split_drag_with_origin(
                                &path,
                                self.split_resize_index,
                                dx,
                                self.split_resize_axis_total,
                                &self.split_resize_origin,
                            );
                        }
                        if let Some(path) = self.split_resize_secondary_path.clone() {
                            self.dock.apply_split_drag_with_origin(
                                &path,
                                self.split_resize_secondary_index,
                                dy,
                                self.split_resize_secondary_axis_total,
                                &self.split_resize_secondary_origin,
                            );
                        }
                    }
                    dock_id if dock_id.starts_with("dock.split.") => {
                        if let (Some(path), axis) = (&self.split_resize_path, input.drag.axis) {
                            let delta = match axis {
                                Some(DragAxis::Horizontal) => dx,
                                Some(DragAxis::Vertical) => dy,
                                _ => dx,
                            };
                            self.dock.apply_split_drag_with_origin(
                                path,
                                self.split_resize_index,
                                delta,
                                self.split_resize_axis_total,
                                &self.split_resize_origin,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    async fn finish_dock_drag(
        &mut self,
        x: f32,
        y: f32,
        input: &InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        let Some(mut drag) = self.dock_drag.take() else {
            return Ok(());
        };
        drag.x = x;
        drag.y = y;
        if drag.drop_zone.is_none() {
            drag.drop_zone = compute_dock_drop_zone(
                x,
                y,
                &self.dock_drop_tab_bars,
                &self.dock_drop_bodies,
                self.dock_canvas_bounds,
            );
        }
        if let Some(zone) = drag.drop_zone {
            if self.dock.apply_drop(&drag.payload, &zone) {
                self.active_window_id = Some(drag.payload.window_id.clone());
                self.dock.sync_active_window(&drag.payload.window_id);
                self.persist_dock_layout();
            } else {
                self.restore_dock_drag_snapshot();
            }
        } else {
            self.restore_dock_drag_snapshot();
        }
        self.dock_drag_snapshot = None;
        let _ = input;
        Ok(())
    }

    pub fn handle_pointer_wheel(
        &mut self,
        x: f32,
        y: f32,
        delta: f32,
        input: &InputState<CommandDescriptor>,
    ) {
        if let Some(hit) = input.hit_at(x, y) {
            if hit.kind == HitKind::ScrollRegion {
                if let Some(id) = &hit.control_id {
                    let entry = self.scroll_offsets.entry(id.clone()).or_insert(0.0);
                    *entry = (*entry + delta * 24.0).max(0.0);
                }
            }
        }
    }

    pub async fn handle_world3d_input(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
        wheel_delta: f32,
        drag_dx: f32,
        drag_dy: f32,
    ) -> Result<(), String> {
        if wheel_delta.abs() > 0.0 {
            for state in self.world3d_states.values_mut() {
                if state.bounds.contains(x, y) {
                    handle_world3d_wheel(state, wheel_delta);
                }
            }
        }
        if (drag_dx.abs() > 0.0 || drag_dy.abs() > 0.0) && down {
            let modifiers = PointerModifiers { shift, ctrl, alt, meta };
            for state in self.world3d_states.values_mut() {
                if state.bounds.contains(x, y) {
                    handle_world3d_pointer_drag(state, x, y, drag_dx, drag_dy, button, &modifiers);
                }
            }
        }
        let mut commands = Vec::new();
        let modifiers = PointerModifiers { shift, ctrl, alt, meta };
        for state in self.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(command) = handle_world3d_pointer_button(state, x, y, down, button, &modifiers) {
                commands.push(command);
            }
            commands.extend(handle_world3d_paint_commands(state, x, y, down, button));
            if let Some(command) = handle_world3d_pointer_move(state, x, y, down, button) {
                commands.push(command);
            }
        }
        for command in commands {
            self.dispatch_command(command).await?;
        }
        Ok(())
    }

    pub async fn poll_world3d_assets(&mut self) {
        fetch_pending_glb_meshes(&mut self.world3d_states).await;
        fetch_pending_reference_images(&mut self.world3d_states).await;
    }

    async fn handle_shell_hit(&mut self, hit: &HitTarget<CommandDescriptor>) -> Result<bool, String> {
        let Some(id) = hit.control_id.as_deref() else {
            return Ok(false);
        };
        match id {
            "ui.nav.back" => {
                if self.uri_index > 0 {
                    self.uri_index -= 1;
                }
                return Ok(true);
            }
            "ui.nav.forward" => {
                if self.uri_index + 1 < self.uri_history.len() {
                    self.uri_index += 1;
                }
                return Ok(true);
            }
            "ui.nav.up" => {
                let uri = self.shell_uri();
                if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                    if !parent.is_empty() {
                        self.push_uri(parent);
                    }
                }
                return Ok(true);
            }
            "playground.navbar.fixture" => {
                self.overlay_state = OverlayState::Dropdown("example".to_string());
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.modes.") => {
                let mode_id = id.trim_start_matches("playground.navbar.modes.");
                if let Some(session) = self.session.as_mut() {
                    session.view_state.active_mode_id = Some(mode_id.to_string());
                }
                self.refresh_ui().await?;
                return Ok(true);
            }
            id if id.starts_with("framework.tool.collection.") => {
                let collection_id = id.trim_start_matches("framework.tool.collection.");
                let expanded = self
                    .tool_collection_expanded
                    .get(collection_id)
                    .copied()
                    .unwrap_or(false);
                self.tool_collection_expanded
                    .insert(collection_id.to_string(), !expanded);
                return Ok(true);
            }
            id if id.starts_with("shell.example.") => {
                let example_id = id.trim_start_matches("shell.example.");
                self.active_example_id = Some(example_id.to_string());
                self.overlay_state = OverlayState::None;
                if let Some(session) = &self.session {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        command: "setActiveExample".into(),
                        args: Some(serde_json::json!({ "exampleId": example_id })),
                    })
                    .await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.find.item.") => {
                let index: usize = id.trim_start_matches("shell.find.item.").parse().unwrap_or(0);
                self.activate_find_item(index).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.engagement.toggle.") => {
                let window_id = id.trim_start_matches("shell.engagement.toggle.");
                let activated = self
                    .engagement_activated
                    .get(window_id)
                    .copied()
                    .unwrap_or(false);
                self.engagement_activated
                    .insert(window_id.to_string(), !activated);
                self.engagement_expanded
                    .insert(window_id.to_string(), !activated);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.fold.") => {
                let window_id = id.trim_start_matches("shell.measures.fold.");
                self.measures_folded.insert(window_id.to_string(), true);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.unfold.") => {
                let window_id = id.trim_start_matches("shell.measures.unfold.");
                self.measures_folded.insert(window_id.to_string(), false);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.focus.") => {
                let window_id = id.trim_start_matches("shell.measures.focus.");
                let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
                self.measures_expanded
                    .insert(window_id.to_string(), !expanded);
                if !expanded {
                    self.engagement_activated.remove(window_id);
                    self.engagement_expanded.insert(window_id.to_string(), false);
                }
                return Ok(true);
            }
            "ui.search.toggle" => {
                self.search_open = !self.search_open;
                self.find_open = false;
                self.overlay_state = if self.search_open {
                    OverlayState::Search
                } else {
                    OverlayState::None
                };
                return Ok(true);
            }
            "ui.find.toggle" => {
                self.find_open = !self.find_open;
                self.search_open = false;
                self.overlay_state = if self.find_open {
                    OverlayState::Find
                } else {
                    OverlayState::None
                };
                return Ok(true);
            }
            "ui.panelToggle.display" => {
                if self.left_panel_open && self.active_left_kind == LeftPanelKind::Display {
                    self.left_panel_open = false;
                } else {
                    self.active_left_kind = LeftPanelKind::Display;
                    self.left_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.workbench" => {
                if self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench {
                    self.left_panel_open = false;
                } else {
                    self.active_left_kind = LeftPanelKind::Workbench;
                    self.left_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.details" => {
                if self.right_panel_open && self.active_right_kind == RightPanelKind::Details {
                    self.right_panel_open = false;
                } else {
                    self.active_right_kind = RightPanelKind::Details;
                    self.right_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.settings" => {
                if self.right_panel_open && self.active_right_kind == RightPanelKind::Settings {
                    self.right_panel_open = false;
                } else {
                    self.active_right_kind = RightPanelKind::Settings;
                    self.right_panel_open = true;
                }
                return Ok(true);
            }
            "ui.fullscreen.toggle" => {
                toggle_fullscreen();
                return Ok(true);
            }
            "studio.canvas.home" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "goHome".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            "studio.canvas.back" => {
                if let Some(session) = &self.session {
                    if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
                        if panel.active_spawned_id.is_some() {
                            self.dispatch_command(CommandDescriptor {
                                controller_id: S_PLAY_CONTROLLER_ID.into(),
                                command: "closeFocusedInstance".into(),
                                args: None,
                            })
                            .await?;
                        }
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("dock.focus.") => {
                let path = parse_path(id.trim_start_matches("dock.focus."));
                self.dock.toggle_maximize(&path);
                self.persist_dock_layout();
                return Ok(true);
            }
            id if id.starts_with("dock.close.") => {
                let path = parse_path(id.trim_start_matches("dock.close."));
                if self.dock.close_active_in_stack(&path) {
                    self.active_window_id = self.dock.active_window_id.clone();
                    self.persist_dock_layout();
                }
                return Ok(true);
            }
            id if id.starts_with("shell.layout.") => {
                let layout_id = id.trim_start_matches("shell.layout.");
                if let Some(session) = &self.session {
                    if let Some(named) = session.app.named_layouts.iter().find(|entry| entry.id == layout_id) {
                        self.layout_override = Some(named.layout.clone());
                        self.sync_dock();
                        self.active_window_id = self.dock.active_window_id.clone();
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("shell.mode.") => {
                let mode_id = id.trim_start_matches("shell.mode.");
                self.dispatch_command(CommandDescriptor {
                    controller_id: self
                        .session
                        .as_ref()
                        .map(|s| s.app.controller_id.clone())
                        .unwrap_or_default(),
                    command: "setMode".into(),
                    args: Some(serde_json::json!({ "modeId": mode_id })),
                })
                .await?;
                return Ok(true);
            }
            id if id.starts_with("framework.settings.theme.") => {
                self.theme_id = id.trim_start_matches("framework.settings.theme.").to_string();
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.left.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.left.");
                self.select_left_panel_tab(tab_id).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.right.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.right.");
                self.active_right_tab = Some(tab_id.to_string());
                if self.studio_mode {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        command: "setActivePanelTab".into(),
                        args: Some(serde_json::json!({ "tabId": tab_id })),
                    })
                    .await?;
                }
                return Ok(true);
            }
            "framework.footer.undo" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "undo".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            "framework.footer.redo" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "redo".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            "framework.footer.checkpoint" => {
                self.dispatch_command(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "commitCheckpoint".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            id if self.context_menu.as_ref().is_some_and(|menu| menu.items.iter().any(|item| item.id == id)) => {
                if let Some(menu) = &self.context_menu {
                    if let Some(item) = menu.items.iter().find(|item| item.id == id) {
                        if let Some(command) = item.command.clone() {
                            self.dispatch_command(command).await?;
                        }
                    }
                }
                self.context_menu = None;
                return Ok(true);
            }
            id if id.starts_with("shell.theme.") => {
                self.theme_id = id.trim_start_matches("shell.theme.").to_string();
                self.overlay_state = OverlayState::None;
                return Ok(true);
            }
            id if id.starts_with("section.chevron.") => {
                let section_id = id.trim_start_matches("section.chevron.");
                let key = format!("section.{section_id}");
                let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
                self.collapsed_sections.insert(key, !collapsed);
                return Ok(true);
            }
            id if id.starts_with("tree.chevron.") => {
                let item_id = id.trim_start_matches("tree.chevron.");
                let key = format!("tree.{item_id}");
                let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
                self.collapsed_sections.insert(key, !collapsed);
                return Ok(true);
            }
            id if id.contains(".vfs.chevron.") => {
                if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.chevron.") {
                    toggle_vfs_row_expanded(surface_id, row_id);
                    return Ok(true);
                }
            }
            id if self.widget_maps.select_metas.contains_key(id) => {
                let opening = !self.open_selects.get(id).copied().unwrap_or(false);
                for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
                    self.open_selects.insert(key, false);
                }
                self.open_selects.insert(id.to_string(), opening);
                return Ok(true);
            }
            id if id.contains(".item.") => {
                if let Some((select_id, value)) = id.rsplit_once(".item.") {
                    if let Some(cmd) = self.widget_maps.select_metas.get(select_id).cloned() {
                        self.open_selects.insert(select_id.to_string(), false);
                        self.dispatch_command(CommandDescriptor {
                            controller_id: cmd.controller_id,
                            command: cmd.command,
                            args: Some(serde_json::json!({ "value": value })),
                        })
                        .await?;
                        return Ok(true);
                    }
                }
            }
            id if self.widget_maps.toggle_metas.contains_key(id) => {
                if let Some((pressed, cmd)) = self.widget_maps.toggle_metas.get(id).cloned() {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: cmd.controller_id,
                        command: cmd.command,
                        args: Some(serde_json::json!({ "pressed": !pressed })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".minus") => {
                let base = id.trim_end_matches(".minus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: meta.on_delta.controller_id,
                        command: meta.on_delta.command,
                        args: Some(serde_json::json!({ "delta": -meta.step })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".plus") => {
                let base = id.trim_end_matches(".plus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: meta.on_delta.controller_id,
                        command: meta.on_delta.command,
                        args: Some(serde_json::json!({ "delta": meta.step })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.starts_with("tree.label.") => {
                let item_id = id.trim_start_matches("tree.label.");
                if hit.drag_data.is_some() {
                    return Ok(true);
                }
                self.queue_tree_selection(item_id);
                return Ok(false);
            }
            _ => {}
        }
        Ok(false)
    }

    async fn execute_search_item(&mut self, item_id: &str) -> Result<(), String> {
        if let Ok(index) = item_id.parse::<usize>() {
            self.activate_search_item(index).await?;
            return Ok(());
        }
        let items = self.filtered_search_items();
        if let Some(index) = items.iter().position(|item| item.id == item_id) {
            self.activate_search_item(index).await?;
        }
        Ok(())
    }

    fn update_tree_hover(&mut self, input: &InputState<CommandDescriptor>) {
        let hovered = input
            .hovered_id
            .as_deref()
            .and_then(|id| id.strip_prefix("tree.label."));
        if self.tree_hovered_id.as_deref() == hovered {
            return;
        }
        if let Some(prev) = self.tree_hovered_id.take() {
            if let Some(cmd) = self.widget_maps.tree_unhover_commands.get(&prev) {
                self.deferred_commands.push(cmd.clone());
            }
        }
        if let Some(id) = hovered {
            if let Some(cmd) = self.widget_maps.tree_hover_commands.get(id) {
                self.deferred_commands.push(cmd.clone());
            }
            self.tree_hovered_id = Some(id.to_string());
        }
    }

    fn queue_tree_selection(&mut self, item_id: &str) {
        let Some(cmd) = self.widget_maps.tree_selection_change.clone() else {
            return;
        };
        self.deferred_commands.push(CommandDescriptor {
            controller_id: cmd.controller_id,
            command: cmd.command,
            args: Some(serde_json::json!({ "ids": [item_id] })),
        });
    }

    async fn dispatch_tree_selection(&mut self, item_id: &str) -> Result<(), String> {
        self.queue_tree_selection(item_id);
        self.flush_deferred_commands().await
    }

    pub async fn flush_deferred_commands(&mut self) -> Result<(), String> {
        let commands = std::mem::take(&mut self.deferred_commands);
        for command in commands {
            self.dispatch_command(command).await?;
        }
        Ok(())
    }

    async fn dispatch_widget_drag_values(&mut self, input: &InputState<CommandDescriptor>) -> Result<(), String> {
        let Some(id) = input.drag.target_id.as_deref() else {
            return Ok(());
        };
        if let Some(value) = self.widget_maps.slider_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.slider_metas.get(id).cloned() {
                self.dispatch_command(CommandDescriptor {
                    controller_id: meta.on_change.controller_id,
                    command: meta.on_change.command,
                    args: Some(serde_json::json!({ "value": value })),
                })
                .await?;
            }
        } else if let Some(value) = self.widget_maps.ring_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.ring_metas.get(id).cloned() {
                self.dispatch_command(CommandDescriptor {
                    controller_id: meta.on_change.controller_id,
                    command: meta.on_change.command,
                    args: Some(serde_json::json!({ "value": value })),
                })
                .await?;
            }
        }
        Ok(())
    }

    async fn commit_focused_input(&mut self, input: &mut InputState<CommandDescriptor>) -> Result<(), String> {
        let Some(id) = input.focused_id.clone() else {
            return Ok(());
        };
        if let Some((vec3_id, axis)) = id.rsplit_once('.') {
            if let Ok(axis_index) = axis.parse::<usize>() {
                if axis_index < 3 {
                    if let Some(meta) = self.widget_maps.vec3_metas.get(vec3_id).cloned() {
                        let parsed = input.text_buffer.parse::<f64>().unwrap_or(0.0);
                        let mut value = meta.value;
                        value[axis_index] = parsed;
                        self.dispatch_command(CommandDescriptor {
                            controller_id: meta.on_change.controller_id,
                            command: meta.on_change.command,
                            args: Some(serde_json::json!({ "value": value })),
                        })
                        .await?;
                        input.blur_input();
                        return Ok(());
                    }
                }
            }
        }
        if id.ends_with(".input") {
            let base = id.trim_end_matches(".input");
            if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                let parsed = input.text_buffer.parse::<f64>().unwrap_or(meta.value);
                self.dispatch_command(CommandDescriptor {
                    controller_id: meta.on_absolute.controller_id,
                    command: meta.on_absolute.command,
                    args: Some(serde_json::json!({ "value": parsed })),
                })
                .await?;
                input.blur_input();
                return Ok(());
            }
        }
        if let Some(meta) = self.widget_maps.input_metas.get(&id).cloned() {
            self.dispatch_command(CommandDescriptor {
                controller_id: meta.on_change.controller_id,
                command: meta.on_change.command,
                args: Some(serde_json::json!({ "value": input.text_buffer })),
            })
            .await?;
            input.blur_input();
        }
        Ok(())
    }

    async fn finish_tree_drag(
        &mut self,
        x: f32,
        y: f32,
        input: &InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        let Some(drag) = self.tree_drag.take() else {
            return Ok(());
        };
        if let Some(hit) = input.hit_at(x, y) {
            if hit.kind == HitKind::World3d || hit.kind == HitKind::Window {
                if let Some(raw) = drag.drag_data.get("application/x-semio-catalogue-item") {
                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(raw) {
                        let program_id = payload.get("programId").and_then(|v| v.as_str());
                        let app_id = payload.get("appId").and_then(|v| v.as_str());
                        if let (Some(program_id), Some(app_id)) = (program_id, app_id) {
                            self.dispatch_command(CommandDescriptor {
                                controller_id: S_PLAY_CONTROLLER_ID.into(),
                                command: "spawnApp".into(),
                                args: Some(serde_json::json!({
                                    "programId": program_id,
                                    "appId": app_id,
                                    "position": { "x": x, "y": y },
                                })),
                            })
                            .await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render_tree_drag_overlay(&self, overlay: &mut DrawList, input: &InputState<CommandDescriptor>, theme: &Theme) {
        let Some(drag) = &self.tree_drag else {
            return;
        };
        overlay.push_solid(
            [drag.x - 60.0, drag.y - 12.0, 120.0, 24.0],
            theme.selected.with_alpha(0.85),
        );
        if let Some(hit) = input.hit_at(drag.x, drag.y) {
            if let Some(target_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                let _ = target_id;
                match drag.drop_position {
                    TreeDropPosition::Before => overlay.push_solid(
                        [hit.rect.x, hit.rect.y, hit.rect.w, 2.0],
                        theme.accent,
                    ),
                    TreeDropPosition::After => overlay.push_solid(
                        [hit.rect.x, hit.rect.y + hit.rect.h - 2.0, hit.rect.w, 2.0],
                        theme.accent,
                    ),
                    TreeDropPosition::Inside => overlay.push_rounded(
                        [hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h],
                        theme.accent.with_alpha(0.15),
                        theme.border_radius,
                    ),
                }
            }
        }
    }

    async fn select_left_panel_tab(&mut self, tab_id: &str) -> Result<(), String> {
        if self.studio_mode {
            if let Some(session) = &self.session {
                if session.app.id == S_PLAY_APP_ID {
                    self.dispatch_command(CommandDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        command: "setActivePanelTab".into(),
                        args: Some(serde_json::json!({ "tabId": tab_id })),
                    })
                    .await?;
                }
            }
        }
        Ok(())
    }

    fn dismiss_overlays(&mut self, x: f32, y: f32, input: &InputState<CommandDescriptor>) -> bool {
        let hit = input.hit_at(x, y);
        let on_overlay = hit.is_some_and(|h| {
            matches!(
                h.kind,
                HitKind::ContextMenu | HitKind::DropdownItem | HitKind::NavbarItem | HitKind::Select
            )
        });
        if self.open_selects.values().any(|open| *open) && !on_overlay {
            for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
                self.open_selects.insert(key, false);
            }
            return true;
        }
        if self.context_menu.is_some() && !on_overlay {
            self.context_menu = None;
            return true;
        }
        if self.overlay_state != OverlayState::None && !on_overlay {
            self.overlay_state = OverlayState::None;
            self.search_open = false;
            self.find_open = false;
            return true;
        }
        false
    }

    fn open_context_menu(&mut self, x: f32, y: f32, hit: Option<HitTarget<CommandDescriptor>>) {
        let node_id = hit.as_ref().and_then(|hit| {
            hit.control_id.as_deref().and_then(|id| {
                id.rsplit_once(".node.").map(|(_, node_id)| node_id.to_string())
            })
        });
        let mut items = take_context_menu_items()
            .into_iter()
            .map(|mut item| {
                if let Some(command) = item.command.take() {
                    item.command = Some(resolve_graph_context_command(&command, node_id.as_deref()));
                }
                item
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            if let (Some(node_id), Some(session)) = (node_id.as_deref(), &self.session) {
                items.push(ContextMenuItem {
                    id: format!("shell.context.node.select.{node_id}"),
                    label: "Select node".into(),
                    command: Some(CommandDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        command: "setMediaNodeSelection".into(),
                        args: Some(serde_json::json!({ "nodeIds": [node_id] })),
                    }),
                });
            }
        }
        if items.is_empty() {
            items = vec![
                ContextMenuItem {
                    id: "shell.context.copy".into(),
                    label: "Copy".into(),
                    command: None,
                },
                ContextMenuItem {
                    id: "shell.context.paste".into(),
                    label: "Paste".into(),
                    command: None,
                },
            ];
        }
        if self.studio_mode {
            items.push(ContextMenuItem {
                id: "shell.context.home".into(),
                label: "Go Home".into(),
                command: Some(CommandDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    command: "goHome".into(),
                    args: None,
                }),
            });
        }
        self.context_menu = Some(ContextMenuState { x, y, items });
        self.overlay_state = OverlayState::None;
    }

    fn build_search_items(&self) -> Vec<SearchPaletteItem> {
        let Some(session) = &self.session else {
            return Vec::new();
        };
        let mut items = Vec::new();
        for tab in &session.app.panel_tabs {
            items.push(SearchPaletteItem {
                id: format!("panel.{}", tab.id),
                label: tab.label.clone(),
                group: "Panels".into(),
                command: Some(CommandDescriptor {
                    controller_id: session.app.controller_id.clone(),
                    command: "setActivePanelTab".into(),
                    args: Some(serde_json::json!({ "tabId": tab.id })),
                }),
                action: None,
            });
        }
        for kind in &session.app.window_kinds {
            items.push(SearchPaletteItem {
                id: format!("window.{}", kind.id),
                label: kind.label.clone(),
                group: "Windows".into(),
                command: None,
                action: Some(format!("window:{}", kind.id)),
            });
        }
        for binding in &session.app.keybindings {
            items.push(SearchPaletteItem {
                id: format!("keybinding.{}", binding.keys),
                label: binding.command.command.clone(),
                group: "Commands".into(),
                command: Some(binding.command.clone()),
                action: None,
            });
        }
        if self.studio_mode {
            for cmd in ["undo", "redo", "commitCheckpoint"] {
                items.push(SearchPaletteItem {
                    id: format!("studio.{cmd}"),
                    label: cmd.into(),
                    group: "Studio".into(),
                    command: Some(CommandDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        command: cmd.into(),
                        args: None,
                    }),
                    action: None,
                });
            }
        }
        items
    }

    fn filtered_search_items(&self) -> Vec<SearchPaletteItem> {
        let query = self.search_query.to_lowercase();
        let items = self.build_search_items();
        if query.trim().is_empty() {
            return items.into_iter().take(20).collect();
        }
        items
            .into_iter()
            .filter(|item| {
                item.label.to_lowercase().contains(&query)
                    || item.group.to_lowercase().contains(&query)
            })
            .take(20)
            .collect()
    }

    fn filtered_find_items(&self) -> Vec<ShellFindItem> {
        let query = self.find_query.to_lowercase();
        if query.trim().is_empty() {
            return self.find_items.iter().take(20).cloned().collect();
        }
        self.find_items
            .iter()
            .filter(|item| {
                item.label.to_lowercase().contains(&query)
                    || item
                        .description
                        .as_ref()
                        .is_some_and(|d| d.to_lowercase().contains(&query))
            })
            .take(20)
            .cloned()
            .collect()
    }

    pub async fn activate_search_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.filtered_search_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(command) = item.command.clone() {
            self.dispatch_command(command).await?;
        } else if let Some(action) = &item.action {
            if let Some(window_id) = action.strip_prefix("window:") {
                self.active_window_id = Some(window_id.to_string());
            }
        }
        self.search_open = false;
        self.overlay_state = OverlayState::None;
        self.search_query.clear();
        self.search_selected = 0;
        Ok(())
    }

    pub async fn activate_find_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.filtered_find_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(session) = &self.session {
            self.dispatch_command(CommandDescriptor {
                controller_id: session.app.controller_id.clone(),
                command: "setMediaNodeSelection".into(),
                args: Some(serde_json::json!({
                    "surfaceId": item.surface_id,
                    "nodeIds": [item.node_id],
                })),
            })
            .await?;
        }
        self.find_open = false;
        self.overlay_state = OverlayState::None;
        self.find_query.clear();
        self.find_selected = 0;
        Ok(())
    }

    pub fn handle_keyboard(
        &mut self,
        action: ui_wgpu::KeyAction,
        modifiers: &ui_wgpu::PointerModifiers,
        input: &mut InputState<CommandDescriptor>,
    ) {
        if action == ui_wgpu::KeyAction::Escape {
            if self.dock_drag.take().is_some() || self.pending_dock_drag.take().is_some() {
                self.restore_dock_drag_snapshot();
                self.dock_drag_snapshot = None;
                return;
            }
        }
        let meta = modifiers.meta || modifiers.ctrl;
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("p")) {
            self.search_open = !self.search_open;
            self.find_open = false;
            self.overlay_state = if self.search_open {
                OverlayState::Search
            } else {
                OverlayState::None
            };
            if self.search_open {
                input.focused_id = Some("shell.search.input".into());
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("f")) {
            self.find_open = !self.find_open;
            self.search_open = false;
            self.overlay_state = if self.find_open {
                OverlayState::Find
            } else {
                OverlayState::None
            };
            if self.find_open {
                input.focused_id = Some("shell.find.input".into());
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c == "[") {
            if self.uri_index > 0 {
                self.uri_index -= 1;
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c == "]") {
            if self.uri_index + 1 < self.uri_history.len() {
                self.uri_index += 1;
            }
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::ArrowUp) {
            let uri = self.shell_uri();
            if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                if !parent.is_empty() {
                    self.push_uri(parent);
                }
            }
            return;
        }
        if meta && modifiers.shift && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("b")) {
            self.right_panel_open = !self.right_panel_open;
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("b")) {
            self.left_panel_open = !self.left_panel_open;
            return;
        }
        let palette_open = matches!(
            self.overlay_state,
            OverlayState::Search | OverlayState::Find
        );
        if palette_open {
            match action {
                ui_wgpu::KeyAction::Escape => {
                    self.overlay_state = OverlayState::None;
                    self.search_open = false;
                    self.find_open = false;
                    input.focused_id = None;
                }
                ui_wgpu::KeyAction::ArrowDown => {
                    if self.overlay_state == OverlayState::Search {
                        let len = self.filtered_search_items().len();
                        if len > 0 {
                            self.search_selected = (self.search_selected + 1).min(len - 1);
                        }
                    } else {
                        let len = self.filtered_find_items().len();
                        if len > 0 {
                            self.find_selected = (self.find_selected + 1).min(len - 1);
                        }
                    }
                }
                ui_wgpu::KeyAction::ArrowUp => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_selected = self.search_selected.saturating_sub(1);
                    } else {
                        self.find_selected = self.find_selected.saturating_sub(1);
                    }
                }
                ui_wgpu::KeyAction::Enter => {
                    let runtime = ();
                    let _ = runtime;
                }
                ui_wgpu::KeyAction::Char(key) => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_query.push_str(&key);
                        self.search_selected = 0;
                    } else {
                        self.find_query.push_str(&key);
                        self.find_selected = 0;
                    }
                }
                ui_wgpu::KeyAction::Backspace => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_query.pop();
                        self.search_selected = 0;
                    } else {
                        self.find_query.pop();
                        self.find_selected = 0;
                    }
                }
                _ => {}
            }
            return;
        }
        if input.focused_id.is_some() {
            match action {
                ui_wgpu::KeyAction::Char(key) => input.text_buffer.push_str(&key),
                ui_wgpu::KeyAction::Backspace => input.backspace(),
                ui_wgpu::KeyAction::Delete => input.delete_forward(),
                _ => {}
            }
        }
    }

    pub async fn handle_keyboard_async(
        &mut self,
        action: ui_wgpu::KeyAction,
        modifiers: &ui_wgpu::PointerModifiers,
        input: &mut InputState<CommandDescriptor>,
    ) -> Result<(), String> {
        if matches!(self.overlay_state, OverlayState::Search) && action == ui_wgpu::KeyAction::Enter {
            self.activate_search_item(self.search_selected).await?;
            return Ok(());
        }
        if matches!(self.overlay_state, OverlayState::Find) && action == ui_wgpu::KeyAction::Enter {
            self.activate_find_item(self.find_selected).await?;
            return Ok(());
        }
        if input.focused_id.is_some() {
            match action {
                ui_wgpu::KeyAction::Enter | ui_wgpu::KeyAction::Escape => {
                    self.commit_focused_input(input).await?;
                    return Ok(());
                }
                _ => {}
            }
        }
        self.handle_keyboard(action, modifiers, input);
        Ok(())
    }

    fn push_uri(&mut self, uri: String) {
        self.uri_history.truncate(self.uri_index + 1);
        self.uri_history.push(uri);
        self.uri_index = self.uri_history.len().saturating_sub(1);
    }
}
//#endregion ShellInput

fn chrome_text(
    target: &mut DrawList,
    atlas: &mut FontAtlas,
    input: &mut InputState<CommandDescriptor>,
    theme: &Theme,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Rgba,
) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = framework_widget_context(
        target,
        None,
        atlas,
        None,
        input,
        theme,
        &mut scroll,
        &mut collapsed,
        &mut selects,
        None,
    );
    draw_text(&mut ctx, text, x, y, size, color);
}

fn chrome_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

fn chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    push_chrome_group_border(draw, rect, theme);
}

struct ChromeGroupItem<'a> {
    control_id: &'a str,
    icon_id: Option<&'a str>,
    label: Option<&'a str>,
    active: bool,
    kind: HitKind,
}

fn measure_chrome_group_item(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> f32 {
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let text_w = item
        .label
        .map(|label| atlas.measure_text(label, theme.font_size_small).0)
        .unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

fn render_chrome_group(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<CommandDescriptor>,
    theme: &Theme,
    rect: Rect,
    items: &[ChromeGroupItem<'_>],
    register_hits: bool,
) {
    if items.is_empty() {
        return;
    }
    let hair = theme.stroke_hairline;
    let inner_y = rect.y + hair;
    let inner_h = (rect.h - hair * 2.0).max(0.0);
    let mut x = rect.x;
    for (index, item) in items.iter().enumerate() {
        let item_w = measure_chrome_group_item(atlas, theme, item);
        let item_rect = Rect::new(x, inner_y, item_w, inner_h);
        let hovered = item_rect.contains(input.pointer_x, input.pointer_y);
        let bg = chrome_item_bg(theme, item.active, hovered);
        if bg.a > 0.0 {
            draw.push_solid([item_rect.x, item_rect.y, item_rect.w, item_rect.h], bg);
        }
        let mut content_x = item_rect.x + theme.padding_standard;
        if let Some(icon_id) = item.icon_id {
            let icon_color = chrome_item_text(theme, item.active, hovered);
            chrome_icon(
                draw,
                icons,
                icon_id,
                content_x,
                item_rect.y + (item_rect.h - CHROME_ICON_TINY) * 0.5,
                CHROME_ICON_TINY,
                icon_color,
            );
            content_x += CHROME_ICON_TINY + theme.gap_standard;
        }
        if let Some(label) = item.label {
            chrome_text(
                draw,
                atlas,
                input,
                theme,
                label,
                content_x,
                item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                chrome_item_text(theme, item.active, hovered),
            );
        }
        if register_hits {
            input.register_hit(HitTarget {
                rect: item_rect,
                event: None,
                control_id: Some(item.control_id.into()),
                kind: item.kind.clone(),
                drag_axis: None,
                drag_data: None,
            });
        }
        x += item_w;
        if index + 1 < items.len() {
            draw.push_solid([x, inner_y, hair, inner_h], theme.border_normal);
        }
    }
    chrome_group_border(draw, rect, theme);
}

fn footer_tool_label<'a>(label: &'a Option<String>, text: &'a Option<String>, title: &'a Option<String>, id: &'a str) -> &'a str {
    title
        .as_deref()
        .or(label.as_deref())
        .or(text.as_deref())
        .unwrap_or(id)
}

fn render_footer_tool_nodes(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<CommandDescriptor>,
    theme: &Theme,
    mut x: f32,
    btn_y: f32,
    btn_h: f32,
    tools: &[ToolNode],
    collection_expanded: &HashMap<String, bool>,
) -> f32 {
    for tool in tools {
        match tool {
            ToolNode::Separator { .. } => {
                draw.push_solid(
                    [x + theme.gap_standard * 0.5, btn_y + 4.0, theme.stroke_hairline, btn_h - 8.0],
                    theme.border_normal,
                );
                x += theme.gap_standard;
            }
            ToolNode::Button {
                id,
                icon_id,
                label,
                text,
                title,
                disabled,
                on_press,
                ..
            } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = footer_tool_label(label, text, title, id);
                let item = ChromeGroupItem {
                    control_id: "framework.tool.button",
                    icon_id: Some(icon_id.as_str()),
                    label: Some(label_text),
                    active: false,
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                input.register_hit(HitTarget {
                    rect,
                    event: Some(on_press.clone()),
                    control_id: Some(format!("framework.tool.button.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
                x += item_w + theme.gap_standard * 0.5;
            }
            ToolNode::Toggle {
                id,
                icon_id,
                label,
                text,
                title,
                pressed,
                disabled,
                on_change,
                ..
            } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = footer_tool_label(label, text, title, id);
                let item = ChromeGroupItem {
                    control_id: "framework.tool.toggle",
                    icon_id: Some(icon_id.as_str()),
                    label: Some(label_text),
                    active: pressed.unwrap_or(false),
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                input.register_hit(HitTarget {
                    rect,
                    event: Some(on_change.clone()),
                    control_id: Some(format!("framework.tool.toggle.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
                x += item_w + theme.gap_standard * 0.5;
            }
            ToolNode::Collection {
                id,
                icon_id,
                label,
                text,
                title,
                disabled,
                children,
                ..
            } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let expanded = collection_expanded.get(id).copied().unwrap_or(false);
                let label_text = footer_tool_label(label, text, title, id);
                let item = ChromeGroupItem {
                    control_id: "framework.tool.collection",
                    icon_id: Some(icon_id.as_str()),
                    label: Some(label_text),
                    active: expanded,
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                input.register_hit(HitTarget {
                    rect,
                    event: None,
                    control_id: Some(format!("framework.tool.collection.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
                x += item_w + theme.gap_standard * 0.5;
                if expanded {
                    let leaves: Vec<ToolNode> = children
                        .iter()
                        .filter(|child| !matches!(child, ToolNode::Collection { .. }))
                        .cloned()
                        .collect();
                    x = render_footer_tool_nodes(
                        draw,
                        atlas,
                        icons,
                        input,
                        theme,
                        x,
                        btn_y,
                        btn_h,
                        &leaves,
                        collection_expanded,
                    );
                }
            }
        }
    }
    x
}

fn panel_tab_icon_id(tab: &PanelTabDefinition) -> &'static str {
    if tab.id == S_PLAY_CATALOGUE_TAB_ID || tab.group == "workbench" {
        return "library";
    }
    if tab.id.contains("parameters") {
        return "settings";
    }
    if tab.id.contains("inspector") || tab.id.contains("inspection") || tab.id == FRAMEWORK_PANEL_TAB_INSPECTION_ID {
        return "text-search";
    }
    if tab.id == FRAMEWORK_PANEL_TAB_HIERARCHY_ID {
        return "list-tree";
    }
    if tab.id == FRAMEWORK_DISPLAY_WINDOWS_TAB_ID {
        return "layout-grid";
    }
    if tab.id == FRAMEWORK_DISPLAY_LAYOUT_TAB_ID {
        return "layout";
    }
    if tab.id == FRAMEWORK_SETTINGS_GENERAL_TAB_ID {
        return "settings-2";
    }
    if tab.id == FRAMEWORK_PANEL_TAB_CATALOGUE_ID {
        return "library";
    }
    "circle-dot"
}

fn app_icon_id<'a>(app: &'a AppDefinition, icons: &IconAtlas) -> &'a str {
    if let Some(id) = app.icon_id.as_deref() {
        if icons.icon_uv(id).is_some() {
            return id;
        }
    }
    "component"
}

fn panel_toggle_icon_id(kind: &str, session: Option<&ActiveSession>) -> &'static str {
    match kind {
        "display" => "layout-grid",
        "workbench" => session
            .and_then(|s| s.app.panel_tabs.iter().find(|tab| tab.group == "left" || tab.group == "workbench"))
            .map(|tab| panel_tab_icon_id(tab))
            .unwrap_or("folder"),
        "details" => session
            .and_then(|s| s.app.panel_tabs.iter().find(|tab| tab.group == "right" || tab.group == "inspection"))
            .map(|tab| panel_tab_icon_id(tab))
            .unwrap_or("info"),
        "settings" => "settings-2",
        _ => "circle-dot",
    }
}

//#region ShellChrome
impl ShellState {
    pub fn render_chrome(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let w = self.screen_w;
        let h = self.screen_h;
        draw.set_screen_height(h);
        overlay.set_screen_height(h);
        overlay.clear();
        draw.push_solid([0.0, 0.0, w, h], theme.background);
        let body = self.body_rect(theme);
        FIND_ITEM_SINK.with(|cell| cell.borrow_mut().clear());
        CONTEXT_MENU_SINK.with(|cell| cell.borrow_mut().clear());
        clear_graph_node_context();
        self.node_graph_states.clear();
        self.widget_maps.clear_frame();
        let mut overlay_slot = Some(overlay);
        self.render_main_window(draw, &mut overlay_slot, atlas, icons, input, theme, body, gpu);
        self.find_items = take_find_items();
        if self.left_panel_open && self.has_left_tabs() {
            self.render_left_panel(draw, overlay_slot.as_deref_mut(), atlas, icons, input, theme, body, gpu);
        }
        if self.right_panel_open && self.has_right_tabs() {
            self.render_right_panel(draw, overlay_slot.as_deref_mut(), atlas, icons, input, theme, body, gpu);
        }
        self.render_navbar(draw, atlas, icons, input, theme, w);
        self.render_footer(draw, atlas, icons, input, theme, w, h);
        if let Some(overlay) = overlay_slot.as_deref_mut() {
            self.render_overlay(overlay, atlas, input, theme, w, h);
            self.render_tree_drag_overlay(overlay, input, theme);
        }
        if let Some(error) = &self.error {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let mut ctx = framework_widget_context(
                draw,
                None,
                atlas,
                Some(icons),
                input,
                theme,
                scroll_offsets,
                collapsed_sections,
                open_selects,
                None,
            );
            draw_text(
                &mut ctx,
                error,
                12.0,
                h - theme.footer_height - 24.0,
                theme.font_size_small,
                Rgba::new(0.95, 0.35, 0.35, 1.0),
            );
        }
    }

    fn body_rect(&self, theme: &Theme) -> Rect {
        Rect::new(
            0.0,
            theme.navbar_height,
            self.screen_w,
            self.screen_h - theme.navbar_height - theme.footer_height,
        )
    }

    fn shell_uri(&self) -> String {
        self.uri_history
            .get(self.uri_index)
            .cloned()
            .unwrap_or_else(|| {
                self.session.as_ref().map(|s| {
                    format!("os://{}/{}", s.plugin_id, s.app.id)
                }).unwrap_or_else(|| "os://home".into())
            })
    }

    fn panel_side_for_group(group: &str) -> &'static str {
        if group == "workbench" || group == "hierarchy" || group == "display" {
            "left"
        } else {
            "right"
        }
    }

    fn has_left_tabs(&self) -> bool {
        self.session.is_some()
    }

    fn has_right_tabs(&self) -> bool {
        self.session.is_some()
    }

    fn left_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
        match self.active_left_kind {
            LeftPanelKind::Display => vec![
                PanelTabDefinition {
                    id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(),
                    label: "Windows".into(),
                    group: "display".into(),
                    body_key: String::new(),
                },
                PanelTabDefinition {
                    id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID.into(),
                    label: "Layout".into(),
                    group: "display".into(),
                    body_key: String::new(),
                },
            ],
            LeftPanelKind::Workbench => {
                let mut tabs: Vec<PanelTabDefinition> = session
                    .app
                    .panel_tabs
                    .iter()
                    .filter(|tab| Self::panel_side_for_group(&tab.group) == "left")
                    .cloned()
                    .collect();
                let has_hierarchy = tabs.iter().any(|t| t.id == FRAMEWORK_PANEL_TAB_HIERARCHY_ID);
                if !has_hierarchy {
                    tabs.insert(
                        0,
                        PanelTabDefinition {
                            id: FRAMEWORK_PANEL_TAB_HIERARCHY_ID.into(),
                            label: "Hierarchy".into(),
                            group: "hierarchy".into(),
                            body_key: String::new(),
                        },
                    );
                }
                tabs
            }
        }
    }

    fn right_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
        match self.active_right_kind {
            RightPanelKind::Settings => vec![PanelTabDefinition {
                id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into(),
                label: "General".into(),
                group: "settings".into(),
                body_key: String::new(),
            }],
            RightPanelKind::Details => session
                .app
                .panel_tabs
                .iter()
                .filter(|tab| Self::panel_side_for_group(&tab.group) == "right")
                .cloned()
                .collect(),
        }
    }

    fn active_left_tab_id(&self, session: &ActiveSession) -> String {
        match self.active_left_kind {
            LeftPanelKind::Display => FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(),
            LeftPanelKind::Workbench => {
                if self.studio_mode && session.app.id == S_PLAY_APP_ID {
                    Self::panel_state_from_view(&session.view_state)
                        .map(|p| p.active_panel_tab)
                        .unwrap_or_else(|| S_PLAY_CATALOGUE_TAB_ID.into())
                } else {
                    self.left_tabs(session)
                        .first()
                        .map(|t| t.id.clone())
                        .unwrap_or_else(|| FRAMEWORK_PANEL_TAB_HIERARCHY_ID.into())
                }
            }
        }
    }

    fn active_right_tab_id(&self, session: &ActiveSession) -> String {
        if self.active_right_kind == RightPanelKind::Settings {
            return FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into();
        }
        if let Some(id) = &self.active_right_tab {
            return id.clone();
        }
        self.right_tabs(session)
            .first()
            .map(|t| t.id.clone())
            .unwrap_or_default()
    }

    fn has_display_tabs(&self) -> bool {
        self.session
            .as_ref()
            .is_some_and(|s| !s.app.window_kinds.is_empty())
    }

    fn floating_panel_rect(&self, left: bool, body: Rect, theme: &Theme) -> Rect {
        let inset = theme.panel_inset;
        if left {
            Rect::new(
                body.x + inset,
                body.y + inset,
                self.left_panel_width,
                body.h - inset * 2.0,
            )
        } else {
            Rect::new(
                body.x + body.w - inset - self.right_panel_width,
                body.y + inset,
                self.right_panel_width,
                body.h - inset * 2.0,
            )
        }
    }

    fn render_navbar(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        width: f32,
    ) {
        let navbar_rect = Rect::new(0.0, 0.0, width, theme.navbar_height);
        let navbar_hovered = navbar_rect.contains(input.pointer_x, input.pointer_y);
        draw.push_solid([0.0, 0.0, width, theme.navbar_height], theme.navbar);
        let border_color = if navbar_hovered {
            theme.border_emphasized
        } else {
            theme.border_normal
        };
        draw.push_solid(
            [0.0, theme.navbar_height - theme.stroke_hairline, width, theme.stroke_hairline],
            border_color,
        );
        let btn_h = theme.control_height;
        let btn_y = (theme.navbar_height - btn_h) * 0.5;
        let mut x = theme.padding_standard;
        let logo_size = btn_h - theme.gap_standard;
            chrome_icon(
                draw,
                icons,
                "semio-logo",
                x,
                btn_y + (btn_h - logo_size) * 0.5,
                logo_size,
                Rgba::new(1.0, 1.0, 1.0, 1.0),
            );
        x += logo_size + theme.gap_standard;
        let title = self
            .session
            .as_ref()
            .map(|s| app_hierarchy_label(&s.app.hierarchy))
            .unwrap_or_else(|| if self.studio_mode { "semio · s · studio".into() } else { "semio · os".into() });
        chrome_text(
            draw,
            atlas,
            input,
            theme,
            &title,
            x,
            btn_y + (btn_h + theme.font_size_body) * 0.5 - 2.0,
            theme.font_size_body,
            theme.text,
        );
        x += atlas.measure_text(&title, theme.font_size_body).0 + theme.gap_standard * 2.0;
        let examples = self.active_plugin_examples();
        if !examples.is_empty() && !self.studio_mode {
            let active_label = examples
                .iter()
                .find(|ex| Some(&ex.id) == self.active_example_id.as_ref())
                .map(|ex| ex.label.as_str())
                .unwrap_or("Example");
            let fixture_w = atlas.measure_text(active_label, theme.font_size_small).0
                + theme.padding_standard * 2.0
                + theme.gap_standard;
            let fixture_rect = Rect::new(x, btn_y, fixture_w.max(120.0), btn_h);
            render_chrome_group(
                draw,
                atlas,
                icons,
                input,
                theme,
                fixture_rect,
                &[ChromeGroupItem {
                    control_id: "playground.navbar.fixture",
                    icon_id: None,
                    label: Some(active_label),
                    active: self.overlay_state == OverlayState::Dropdown("example".to_string()),
                    kind: HitKind::NavbarItem,
                }],
                true,
            );
            x += fixture_rect.w + theme.gap_standard;
        }
        let mut rx = width - theme.padding_standard;
        let fullscreen_item = ChromeGroupItem {
            control_id: "ui.fullscreen.toggle",
            icon_id: Some("maximize-2"),
            label: Some("Fullscreen"),
            active: false,
            kind: HitKind::Toggle,
        };
        let fullscreen_w = measure_chrome_group_item(atlas, theme, &fullscreen_item);
        rx -= fullscreen_w;
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(rx, btn_y, fullscreen_w, btn_h),
            &[fullscreen_item],
            true,
        );
        rx -= theme.gap_standard;
        let mut toggle_items: Vec<ChromeGroupItem<'_>> = Vec::new();
        if self.has_display_tabs() {
            toggle_items.push(ChromeGroupItem {
                control_id: "ui.panelToggle.display",
                icon_id: Some(panel_toggle_icon_id("display", self.session.as_ref())),
                label: Some("Display"),
                active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Display,
                kind: HitKind::Toggle,
            });
        }
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.workbench",
            icon_id: Some(panel_toggle_icon_id("workbench", self.session.as_ref())),
            label: Some("Workbench"),
            active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.details",
            icon_id: Some(panel_toggle_icon_id("details", self.session.as_ref())),
            label: Some("Details"),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Details,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.settings",
            icon_id: Some(panel_toggle_icon_id("settings", self.session.as_ref())),
            label: Some("Settings"),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Settings,
            kind: HitKind::Toggle,
        });
        let toggle_w: f32 = toggle_items
            .iter()
            .map(|item| measure_chrome_group_item(atlas, theme, item))
            .sum();
        rx -= toggle_w;
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(rx, btn_y, toggle_w, btn_h),
            &toggle_items,
            true,
        );
        rx -= theme.gap_standard;
        if let Some(session) = &self.session {
            if session.app.modes.len() > 1 {
                let mode_control_ids: Vec<String> = session
                    .app
                    .modes
                    .iter()
                    .rev()
                    .map(|mode| format!("playground.navbar.modes.{}", mode.id))
                    .collect();
                let mode_items: Vec<ChromeGroupItem<'_>> = session
                    .app
                    .modes
                    .iter()
                    .rev()
                    .zip(mode_control_ids.iter())
                    .map(|(mode, control_id)| {
                        let active_mode = session
                            .view_state
                            .active_mode_id
                            .as_deref()
                            .or(session.app.default_mode_id.as_deref())
                            .unwrap_or(&mode.id);
                        ChromeGroupItem {
                            control_id: control_id.as_str(),
                            icon_id: None,
                            label: Some(mode.label.as_str()),
                            active: active_mode == mode.id,
                            kind: HitKind::NavbarItem,
                        }
                    })
                    .collect();
                let mode_w: f32 = mode_items
                    .iter()
                    .map(|item| measure_chrome_group_item(atlas, theme, item))
                    .sum();
                rx -= mode_w;
                render_chrome_group(
                    draw,
                    atlas,
                    icons,
                    input,
                    theme,
                    Rect::new(rx, btn_y, mode_w, btn_h),
                    &mode_items,
                    true,
                );
            }
        }
    }

    fn render_footer(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        width: f32,
        height: f32,
    ) {
        let y = height - theme.footer_height;
        let footer_rect = Rect::new(0.0, y, width, theme.footer_height);
        let footer_hovered = footer_rect.contains(input.pointer_x, input.pointer_y);
        draw.push_solid([0.0, y, width, theme.footer_height], theme.navbar);
        let border_color = if footer_hovered {
            theme.border_emphasized
        } else {
            theme.border_normal
        };
        draw.push_solid([0.0, y, width, theme.stroke_hairline], border_color);
        let session = match &self.session {
            Some(s) => s,
            None => return,
        };
        let btn_h = theme.control_height;
        let btn_y = y + (theme.footer_height - btn_h) * 0.5;
        let x = theme.padding_standard;
        let app_label = app_hierarchy_label(&session.app.hierarchy);
        let mut footer_items = vec![ChromeGroupItem {
            control_id: "framework.footer.app",
            icon_id: Some(app_icon_id(&session.app, icons)),
            label: Some(app_label.as_str()),
            active: false,
            kind: HitKind::Button,
        }];
        if self.studio_mode && session.app.controller_id == S_PLAY_CONTROLLER_ID {
            footer_items.extend([
                ChromeGroupItem {
                    control_id: "framework.footer.undo",
                    icon_id: Some("rotate-ccw"),
                    label: Some("Undo"),
                    active: false,
                    kind: HitKind::Button,
                },
                ChromeGroupItem {
                    control_id: "framework.footer.redo",
                    icon_id: Some("rotate-cw"),
                    label: Some("Redo"),
                    active: false,
                    kind: HitKind::Button,
                },
                ChromeGroupItem {
                    control_id: "framework.footer.checkpoint",
                    icon_id: Some("save"),
                    label: Some("Checkpoint"),
                    active: false,
                    kind: HitKind::Button,
                },
            ]);
        }
        let group_w: f32 = footer_items
            .iter()
            .map(|item| measure_chrome_group_item(atlas, theme, item))
            .sum();
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(x, btn_y, group_w, btn_h),
            &footer_items,
            true,
        );
        let mut tool_x = x + group_w + theme.gap_standard;
        tool_x = render_footer_tool_nodes(
            draw,
            atlas,
            icons,
            input,
            theme,
            tool_x,
            btn_y,
            btn_h,
            &self.active_tools,
            &self.tool_collection_expanded,
        );
        let _ = width;
        let _ = tool_x;
    }

    fn render_floating_panel(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        panel: Rect,
        tabs: &[PanelTabDefinition],
        active_tab_id: &str,
        side_left: bool,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        const PANEL_RESIZE_HIT_PX: f32 = 20.0;
        let resize_id = if side_left {
            "panel.resize.left"
        } else {
            "panel.resize.right"
        };
        let resize_edge_accent = input.drag.active
            && input.drag.target_id.as_deref() == Some(resize_id)
            || input
                .hit_at(input.pointer_x, input.pointer_y)
                .and_then(|hit| hit.control_id.as_deref())
                == Some(resize_id);
        let border = theme.border_normal;
        let hair = theme.stroke_hairline;
        draw.push_glass(
            [panel.x, panel.y, panel.w, panel.h],
            theme.border_radius,
            GlassTier::Panel,
            theme,
        );
        let left_stroke = if resize_edge_accent && !side_left {
            theme.accent
        } else {
            border
        };
        let right_stroke = if resize_edge_accent && side_left {
            theme.accent
        } else {
            border
        };
        draw.push_solid([panel.x, panel.y, panel.w, hair], border);
        draw.push_solid([panel.x, panel.y + panel.h - hair, panel.w, hair], border);
        draw.push_solid([panel.x, panel.y, hair, panel.h], left_stroke);
        draw.push_solid([panel.x + panel.w - hair, panel.y, hair, panel.h], right_stroke);
        let tab_bar_h = theme.panel_header_height;
        draw.push_solid(
            [panel.x, panel.y + tab_bar_h - hair, panel.w, hair],
            border,
        );
        let mut tab_x = panel.x;
        for (index, tab) in tabs.iter().enumerate() {
            let icon_id = panel_tab_icon_id(tab);
            let label_w = atlas.measure_text(&tab.label, theme.font_size_small).0;
            let tw = theme.padding_standard * 2.0 + CHROME_ICON_TINY + theme.gap_standard + label_w;
            let rect = Rect::new(tab_x, panel.y, tw, tab_bar_h);
            if index > 0 {
                draw.push_solid([tab_x, panel.y, hair, tab_bar_h], border);
            }
            let active = tab.id == active_tab_id;
            let hovered = rect.contains(input.pointer_x, input.pointer_y);
            let bg = if active {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.panel
            };
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
            let icon_x = rect.x + theme.padding_standard;
            let icon_y = rect.y + (rect.h - CHROME_ICON_TINY) * 0.5;
            chrome_icon(
                draw,
                icons,
                icon_id,
                icon_x,
                icon_y,
                CHROME_ICON_TINY,
                chrome_item_text(theme, active, hovered),
            );
            chrome_text(
                draw,
                atlas,
                input,
                theme,
                &tab.label,
                icon_x + CHROME_ICON_TINY + theme.gap_standard,
                rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                chrome_item_text(theme, active, hovered),
            );
            let prefix = if side_left {
                "shell.panel.tab.left."
            } else {
                "shell.panel.tab.right."
            };
            input.register_hit(HitTarget {
                rect,
                event: None,
                control_id: Some(format!("{prefix}{}", tab.id)),
                kind: HitKind::PanelTab,
                drag_axis: None,
            drag_data: None,
            });
            tab_x += tw;
        }
        let content = Rect::new(
            panel.x + theme.gap_standard,
            panel.y + tab_bar_h,
            panel.w - theme.gap_standard * 2.0,
            panel.h - tab_bar_h - theme.gap_standard,
        );
        let scroll_key = format!(
            "panel.{}.{}",
            if side_left { "left" } else { "right" },
            active_tab_id
        );
        let scroll_y = *self.scroll_offsets.get(&scroll_key).unwrap_or(&0.0);
        draw.push_scissor(content);
        input.register_hit(HitTarget {
            rect: content,
            event: None,
            control_id: Some(scroll_key.clone()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
        drag_data: None,
        });
        if let Some(ui) = self.panel_ui.get(active_tab_id).cloned() {
            let scrolled = Rect::new(content.x, content.y - scroll_y, content.w, content.h);
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let widget_maps = &mut self.widget_maps;
            let mut ctx = framework_widget_context(
                draw,
                overlay,
                atlas,
                Some(icons),
                input,
                theme,
                scroll_offsets,
                collapsed_sections,
                open_selects,
                Some(widget_maps),
            );
            render_ui_node(&ui, scrolled, &mut ctx, gpu, &mut self.world3d_states, &mut self.node_graph_states);
        }
        draw.pop_scissor();
        let handle = if side_left {
            Rect::new(
                panel.x + panel.w - PANEL_RESIZE_HIT_PX,
                panel.y,
                PANEL_RESIZE_HIT_PX,
                panel.h,
            )
        } else {
            Rect::new(panel.x, panel.y, PANEL_RESIZE_HIT_PX, panel.h)
        };
        input.register_hit(HitTarget {
            rect: handle,
            event: None,
            control_id: Some(resize_id.into()),
            kind: HitKind::PanelResize,
            drag_axis: Some(DragAxis::Horizontal),
            drag_data: None,
        });
    }

    fn render_left_panel(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        body: Rect,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let tabs = self.left_tabs(&session);
        if tabs.is_empty() {
            return;
        }
        let active = self.active_left_tab_id(&session);
        let panel = self.floating_panel_rect(true, body, theme);
        self.render_floating_panel(
            draw,
            overlay,
            atlas,
            icons,
            input,
            theme,
            panel,
            &tabs,
            &active,
            true,
            gpu,
        );
    }

    fn render_right_panel(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        body: Rect,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let tabs = self.right_tabs(&session);
        if tabs.is_empty() {
            return;
        }
        let active = self.active_right_tab_id(&session);
        let panel = self.floating_panel_rect(false, body, theme);
        self.render_floating_panel(
            draw,
            overlay,
            atlas,
            icons,
            input,
            theme,
            panel,
            &tabs,
            &active,
            false,
            gpu,
        );
    }

    fn render_main_window(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.background);
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let mut canvas = bounds.inset(theme.panel_inset);
        canvas = self.render_studio_canvas_bars(draw, atlas, icons, input, theme, canvas, &session);
        if self.studio_mode {
            if let Some(spawned_ui) = self.spawned_ui.clone() {
                self.render_window_content(
                    draw, overlay.as_deref_mut(), atlas, icons, input, theme, canvas, &spawned_ui, "spawned", gpu,
                );
                return;
            }
        }
        let window_labels: HashMap<String, String> = session
            .app
            .window_kinds
            .iter()
            .map(|kind| {
                (
                    kind.id.clone(),
                    app_window_hierarchy_label(&session.app, &kind.label),
                )
            })
            .collect();
        self.dock_canvas_bounds = canvas;
        self.dock_drop_tab_bars = self.dock_tab_bars_for_drop(atlas, theme, canvas, &window_labels);
        self.dock_drop_bodies = self
            .dock
            .stack_body_rects(canvas, theme, &window_labels, atlas)
            .into_iter()
            .map(|(path, rect, active)| (path, rect, active))
            .collect();
        {
            let mut dock_ctx = DockRenderContext {
                draw,
                atlas,
                icons,
                input,
                theme,
                window_labels: &window_labels,
            };
            self.dock.register_hits(&mut dock_ctx, canvas);
        }
        let placements = self.dock.stack_body_rects(canvas, theme, &window_labels, atlas);
        let show_fallback = placements.is_empty();
        for (_, mut content, window_id) in placements {
            let window_kind = session
                .app
                .window_kinds
                .iter()
                .find(|kind| kind.id == window_id)
                .cloned();
            let mut window_chip_hits: Vec<(Rect, String)> = Vec::new();
            if let Some(kind) = window_kind {
                if let Some(hit) = self.render_window_measures_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &mut content,
                    &window_id,
                    &kind,
                    gpu,
                ) {
                    window_chip_hits.push(hit);
                }
                if let Some(hit) = self.render_window_engagement_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &mut content,
                    &window_id,
                    &kind,
                    gpu,
                ) {
                    window_chip_hits.push(hit);
                }
            }
            if let Some(ui) = self.window_ui.get(&window_id).cloned() {
                self.render_window_content(
                    draw, overlay.as_deref_mut(), atlas, icons, input, theme, content, &ui, &window_id, gpu,
                );
            }
            for (rect, control_id) in window_chip_hits {
                input.register_hit(HitTarget {
                    rect,
                    event: None,
                    control_id: Some(control_id),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
            }
        }
        {
            let mut resize_ctx = DockRenderContext {
                draw,
                atlas,
                icons,
                input,
                theme,
                window_labels: &window_labels,
            };
            self.dock.register_resize_hits(&mut resize_ctx, canvas);
        }
        if show_fallback {
            chrome_text(
                draw,
                atlas,
                input,
                theme,
                &app_hierarchy_label(&session.app.hierarchy),
                canvas.x + 16.0,
                canvas.y + 32.0,
                theme.font_size_body,
                theme.text_muted,
            );
        }
        if let Some(drag) = &self.dock_drag {
            if let Some(zone) = &drag.drop_zone {
                if let Some(indicator) = drop_zone_indicator_rect(
                    zone,
                    &self.dock_drop_tab_bars,
                    &self.dock_drop_bodies,
                    self.dock_canvas_bounds,
                    theme.gap_standard,
                ) {
                    draw.push_rounded(
                        [indicator.x, indicator.y, indicator.w, indicator.h],
                        theme.accent.with_alpha(0.2),
                        theme.border_radius,
                    );
                    let hair = theme.stroke_hairline;
                    draw.push_solid([indicator.x, indicator.y, indicator.w, hair], theme.accent);
                    draw.push_solid(
                        [indicator.x, indicator.y + indicator.h - hair, indicator.w, hair],
                        theme.accent,
                    );
                    draw.push_solid([indicator.x, indicator.y, hair, indicator.h], theme.accent);
                    draw.push_solid(
                        [indicator.x + indicator.w - hair, indicator.y, hair, indicator.h],
                        theme.accent,
                    );
                }
            }
            let ghost = Rect::new(drag.x - 48.0, drag.y - 12.0, 120.0, theme.control_height);
            if !matches!(drag.drop_zone, Some(DockDropZone::Tab { .. })) {
                draw.push_rounded([ghost.x, ghost.y, ghost.w, ghost.h], theme.panel, theme.border_radius);
                chrome_text(
                    draw,
                    atlas,
                    input,
                    theme,
                    &drag.payload.ghost_label,
                    ghost.x + theme.padding_standard,
                    ghost.y + (ghost.h + theme.font_size_small) * 0.5 - 1.0,
                    theme.font_size_small,
                    theme.text,
                );
            }
        }
    }

    fn render_studio_canvas_bars(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        mut canvas: Rect,
        session: &ActiveSession,
    ) -> Rect {
        if !self.studio_mode || session.app.id != S_PLAY_APP_ID {
            return canvas;
        }
        let bar_h = theme.control_height;
        if self.spawned_ui.is_none() {
            let item = ChromeGroupItem {
                control_id: "studio.canvas.home",
                icon_id: Some("home"),
                label: Some("Home"),
                active: false,
                kind: HitKind::Button,
            };
            let bar_w = measure_chrome_group_item(atlas, theme, &item);
            let bar = Rect::new(canvas.x, canvas.y, bar_w, bar_h);
            render_chrome_group(draw, atlas, icons, input, theme, bar, &[item], true);
            canvas.y += bar_h + theme.gap_standard;
            canvas.h -= bar_h + theme.gap_standard;
            return canvas;
        }
        if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
            if let Some(spawned) = panel
                .active_spawned_id
                .as_ref()
                .and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id))
            {
                let label = format!(
                    "Back to Media Graph · {}",
                    app_hierarchy_label(&spawned.hierarchy)
                );
                let item = ChromeGroupItem {
                    control_id: "studio.canvas.back",
                    icon_id: Some("chevron-left"),
                    label: Some(&label),
                    active: false,
                    kind: HitKind::Button,
                };
                let bar_w = measure_chrome_group_item(atlas, theme, &item).min(canvas.w);
                let bar = Rect::new(canvas.x, canvas.y, bar_w, bar_h);
                render_chrome_group(draw, atlas, icons, input, theme, bar, &[item], true);
                canvas.y += bar_h + theme.gap_standard;
                canvas.h -= bar_h + theme.gap_standard;
            }
        }
        canvas
    }

    fn render_window_content(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        content: Rect,
        ui: &UiNode,
        window_id: &str,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let scroll_key = format!("window.{window_id}");
        let scroll_y = *self.scroll_offsets.get(&scroll_key).unwrap_or(&0.0);
        draw.push_scissor(content);
        input.register_hit(HitTarget {
            rect: content,
            event: None,
            control_id: Some(scroll_key.clone()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
        drag_data: None,
        });
        let scrolled = Rect::new(content.x, content.y - scroll_y, content.w, content.h);
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let widget_maps = &mut self.widget_maps;
        let mut ctx = framework_widget_context(
            draw,
            overlay,
            atlas,
            Some(icons),
            input,
            theme,
            scroll_offsets,
            collapsed_sections,
            open_selects,
            Some(widget_maps),
        );
        render_ui_node(ui, scrolled, &mut ctx, gpu, &mut self.world3d_states, &mut self.node_graph_states);
        draw.pop_scissor();
    }

    fn render_overlay(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        width: f32,
        height: f32,
    ) {
        if let Some(menu) = &self.context_menu {
            self.render_context_menu(overlay, atlas, input, theme, menu);
        }
        match &self.overlay_state {
            OverlayState::Search => {
                let items: Vec<(String, String, usize)> = self
                    .filtered_search_items()
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| (item.group, item.label, index))
                    .collect();
                self.render_command_list(
                    overlay,
                    atlas,
                    input,
                    theme,
                    width * 0.5 - 200.0,
                    theme.navbar_height + 8.0,
                    400.0,
                    height * 0.55,
                    "Search",
                    &self.search_query,
                    "shell.search.input",
                    self.search_selected,
                    &items,
                    "shell.search.item",
                );
            }
            OverlayState::Find => {
                let items: Vec<(String, String, usize)> = self
                    .filtered_find_items()
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| {
                        (
                            item.category.clone().unwrap_or_default(),
                            item.label.clone(),
                            index,
                        )
                    })
                    .collect();
                self.render_command_list(
                    overlay,
                    atlas,
                    input,
                    theme,
                    width * 0.5 - 200.0,
                    theme.navbar_height + 8.0,
                    400.0,
                    height * 0.55,
                    "Find in page",
                    &self.find_query,
                    "shell.find.input",
                    self.find_selected,
                    &items,
                    "shell.find.item",
                );
            }
            OverlayState::Dropdown(id) if id == "example" => {
                let examples = self.active_plugin_examples();
                let items: Vec<(String, String, usize)> = examples
                    .iter()
                    .enumerate()
                    .map(|(index, ex)| (String::new(), ex.label.clone(), index))
                    .collect();
                let id_items: Vec<(String, String, usize)> = examples
                    .iter()
                    .enumerate()
                    .map(|(index, ex)| (String::new(), ex.label.clone(), index))
                    .collect();
                let mapped: Vec<(String, String, usize)> = examples
                    .iter()
                    .enumerate()
                    .map(|(index, ex)| ("Examples".into(), ex.label.clone(), index))
                    .collect();
                self.render_example_dropdown(
                    overlay,
                    atlas,
                    input,
                    theme,
                    width * 0.25,
                    theme.navbar_height + 4.0,
                    220.0,
                    &mapped,
                    &examples,
                );
                let _ = (items, id_items);
            }
            OverlayState::Dropdown(_) => {}
            OverlayState::ThemeSelect => {}
            OverlayState::None => {}
        }
        for (id, open) in &self.open_selects {
            if *open {
                self.render_palette(overlay, atlas, input, theme, width * 0.4, theme.navbar_height + 40.0, 220.0, "Options", id);
            }
        }
    }

    fn render_example_dropdown(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
        w: f32,
        items: &[(String, String, usize)],
        examples: &[ExampleDefinition],
    ) {
        let row_h = theme.control_height;
        let h = items.len() as f32 * row_h + theme.padding_standard * 2.0;
        overlay.push_glass([x, y, w, h.max(row_h + 8.0)], theme.border_radius, GlassTier::Menu, theme);
        for (index, (_group, label, _)) in items.iter().enumerate() {
            let row = Rect::new(
                x + theme.gap_standard,
                y + theme.gap_standard + index as f32 * row_h,
                w - theme.gap_standard * 2.0,
                row_h,
            );
            let selected = examples
                .get(index)
                .is_some_and(|ex| self.active_example_id.as_deref() == Some(ex.id.as_str()));
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let bg = if selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                label,
                row.x + theme.padding_standard,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                if selected || hovered {
                    theme.active_foreground
                } else {
                    theme.text
                },
            );
            if let Some(example) = examples.get(index) {
                input.register_hit(HitTarget {
                    rect: row,
                    event: None,
                    control_id: Some(format!("shell.example.{}", example.id)),
                    kind: HitKind::DropdownItem,
                    drag_axis: None,
                drag_data: None,
                });
            }
        }
    }

    fn render_command_list(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        title: &str,
        query: &str,
        input_id: &str,
        selected: usize,
        items: &[(String, String, usize)],
        item_prefix: &str,
    ) {
        overlay.push_glass([x, y, w, h], theme.border_radius, GlassTier::Menu, theme);
        chrome_text(
            overlay,
            atlas,
            input,
            theme,
            title,
            x + 12.0,
            y + 20.0,
            theme.font_size_body,
            theme.text,
        );
        let filter_rect = Rect::new(x + 8.0, y + 32.0, w - 16.0, theme.control_height);
        overlay.push_rounded(
            [filter_rect.x, filter_rect.y, filter_rect.w, filter_rect.h],
            theme.input_bg,
            theme.border_radius,
        );
        let display_query = if query.is_empty() { "Type to filter…" } else { query };
        chrome_text(
            overlay,
            atlas,
            input,
            theme,
            display_query,
            filter_rect.x + 8.0,
            filter_rect.y + (filter_rect.h + theme.font_size_small) * 0.5 - 1.0,
            theme.font_size_small,
            if query.is_empty() {
                theme.text_muted
            } else {
                theme.text
            },
        );
        input.register_hit(HitTarget {
            rect: filter_rect,
            event: None,
            control_id: Some(input_id.into()),
            kind: HitKind::Input,
            drag_axis: None,
        drag_data: None,
        });
        let list_top = y + 32.0 + theme.control_height + 8.0;
        let list_h = h - (list_top - y) - 8.0;
        let mut row_y = list_top;
        let mut last_group = String::new();
        for (group, label, index) in items {
            if !group.is_empty() && group != &last_group {
                chrome_text(
                    overlay,
                    atlas,
                    input,
                    theme,
                    group,
                    x + 12.0,
                    row_y + 12.0,
                    theme.font_size_small,
                    theme.text_muted,
                );
                row_y += 18.0;
                last_group = group.clone();
            }
            let row = Rect::new(x + 8.0, row_y, w - 16.0, theme.control_height);
            if row_y + theme.control_height > list_top + list_h {
                break;
            }
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let is_selected = *index == selected;
            let bg = if is_selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                label,
                row.x + 8.0,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                if is_selected || hovered {
                    theme.active_foreground
                } else {
                    theme.text
                },
            );
            input.register_hit(HitTarget {
                rect: row,
                event: None,
                control_id: Some(format!("{item_prefix}.{index}")),
                kind: HitKind::DropdownItem,
                drag_axis: None,
            drag_data: None,
            });
            row_y += theme.control_height + 2.0;
        }
    }

    fn window_engagement_chrome_visible(
        engagement: &semio_framework_core::layout::WindowEngagement,
        window_id: &str,
        engagement_inputs: &HashMap<String, String>,
        activated: bool,
    ) -> bool {
        if engagement.session_active.unwrap_or(false) {
            return true;
        }
        let draft = engagement_inputs
            .get(window_id)
            .or_else(|| engagement.input.as_ref().and_then(|input| input.value.as_ref()))
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        if draft.is_some() {
            return true;
        }
        activated
    }

    fn render_window_measures_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        content: &mut Rect,
        window_id: &str,
        kind: &semio_framework_core::WindowKindDefinition,
        gpu: &mut ui_wgpu::GpuContext,
    ) -> Option<(Rect, String)> {
        if kind.measures.is_empty() {
            return None;
        }
        let folded = self.measures_folded.get(window_id).copied().unwrap_or(true);
        let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
        let rail_w = *self
            .measures_width
            .get(window_id)
            .unwrap_or(&DEFAULT_MEASURES_RAIL_WIDTH);
        if folded {
            let item = ChromeGroupItem {
                control_id: "",
                icon_id: Some("chevron-left"),
                label: Some("Window Options"),
                active: false,
                kind: HitKind::Button,
            };
            let chip_w = measure_chrome_group_item(atlas, theme, &item);
            let chip = Rect::new(
                content.x + content.w - chip_w,
                content.y + 8.0,
                chip_w,
                theme.control_height,
            );
            if let Some(chip_draw) = overlay.as_deref_mut() {
                render_chrome_group(chip_draw, atlas, icons, input, theme, chip, &[item], false);
            } else {
                render_chrome_group(draw, atlas, icons, input, theme, chip, &[item], false);
            }
            return Some((chip, format!("shell.measures.unfold.{window_id}")));
        }
        let width = if expanded { content.w * 0.45 } else { rail_w };
        let rail = Rect::new(content.x + content.w - width, content.y, width, content.h);
        draw.push_glass([rail.x, rail.y, rail.w, rail.h], theme.border_radius, GlassTier::WindowOptions, theme);
        let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
        draw.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
        let focus_label = if expanded { "Unfocus" } else { "Focus" };
        let focus_item = ChromeGroupItem {
            control_id: "shell.measures.focus",
            icon_id: Some(if expanded { "minimize-2" } else { "maximize-2" }),
            label: Some(focus_label),
            active: false,
            kind: HitKind::Button,
        };
        let fold_item = ChromeGroupItem {
            control_id: "shell.measures.fold",
            icon_id: Some("chevron-right"),
            label: Some("Window Options"),
            active: false,
            kind: HitKind::Button,
        };
        let focus_w = measure_chrome_group_item(atlas, theme, &focus_item);
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(header.x, header.y, focus_w, header.h),
            &[focus_item],
            true,
        );
        input.register_hit(HitTarget {
            rect: Rect::new(header.x, header.y, focus_w, header.h),
            event: None,
            control_id: Some(format!("shell.measures.focus.{window_id}")),
            kind: HitKind::Button,
            drag_axis: None,
        drag_data: None,
        });
        let fold_w = measure_chrome_group_item(atlas, theme, &fold_item);
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            Rect::new(header.x + header.w - fold_w, header.y, fold_w, header.h),
            &[fold_item],
            true,
        );
        input.register_hit(HitTarget {
            rect: Rect::new(header.x + header.w - fold_w, header.y, fold_w, header.h),
            event: None,
            control_id: Some(format!("shell.measures.fold.{window_id}")),
            kind: HitKind::Button,
            drag_axis: None,
        drag_data: None,
        });
        let body = Rect::new(
            rail.x + theme.gap_standard,
            rail.y + theme.panel_header_height + theme.gap_standard,
            rail.w - theme.gap_standard * 2.0,
            rail.h - theme.panel_header_height - theme.gap_standard * 2.0,
        );
        for measure in &kind.measures {
            self.render_window_measure(
                draw,
                overlay,
                atlas,
                icons,
                input,
                theme,
                body,
                measure,
                gpu,
            );
        }
        if !expanded {
            let resize = Rect::new(rail.x - 3.0, rail.y, 6.0, rail.h);
            input.register_hit(HitTarget {
                rect: resize,
                event: None,
                control_id: Some(format!("shell.measures.resize.{window_id}")),
                kind: HitKind::PanelResize,
                drag_axis: Some(DragAxis::Horizontal),
                drag_data: None,
            });
        }
        content.w -= width + theme.gap_standard;
        None
    }

    fn render_window_measure(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        measure: &WindowMeasure,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        use semio_framework_core::layout::MeasureSelectItem;
        use ui_wgpu::widgets::{render_widget, ControlNode, WidgetNode};
        let mut y = bounds.y;
        match measure {
            WindowMeasure::Group {
                id,
                label,
                default_open,
                children,
            } => {
                let open = !self.collapsed_sections.get(id).copied().unwrap_or(!default_open.unwrap_or(false));
                chrome_text(
                    draw,
                    atlas,
                    input,
                    theme,
                    &format!("{} {}", if open { "v" } else { ">" }, label),
                    bounds.x,
                    y + 14.0,
                    theme.font_size_small,
                    theme.text,
                );
                input.register_hit(HitTarget {
                    rect: Rect::new(bounds.x, y, bounds.w, theme.control_height),
                    event: None,
                    control_id: Some(format!("shell.measure.group.{id}")),
                    kind: HitKind::Button,
                    drag_axis: None,
                drag_data: None,
                });
                y += theme.control_height;
                if open {
                    for child in children {
                        self.render_window_measure(
                            draw, overlay, atlas, icons, input, theme,
                            Rect::new(bounds.x + 12.0, y, bounds.w - 12.0, bounds.h - (y - bounds.y)),
                            child, gpu,
                        );
                    }
                }
            }
            WindowMeasure::Select {
                id,
                label,
                value,
                items,
                on_change,
            } => {
                if let Some(label) = label {
                    chrome_text(draw, atlas, input, theme, label, bounds.x, y + 14.0, theme.font_size_small, theme.text_muted);
                }
                let node = WidgetNode::Select {
                    id: id.clone(),
                    value: value.clone(),
                    items: items
                        .iter()
                        .map(|item: &MeasureSelectItem| ui_wgpu::widgets::SelectItem {
                            value: item.value.clone(),
                            label: item.label.clone(),
                        })
                        .collect(),
                    placeholder: None,
                    on_change: Some(on_change.clone()),
                };
                let rect = Rect::new(bounds.x, y + 16.0, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(
                    draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
                    scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
                render_widget(&node, rect, &mut ctx);
            }
            WindowMeasure::Slider {
                id,
                label,
                value,
                min,
                max,
                step,
                on_change,
            } => {
                if let Some(label) = label {
                    chrome_text(draw, atlas, input, theme, label, bounds.x, y + 14.0, theme.font_size_small, theme.text_muted);
                }
                let node = WidgetNode::Slider {
                    id: id.clone(),
                    value: *value,
                    min: *min,
                    max: *max,
                    step: step.unwrap_or(0.01),
                    on_change: Some(on_change.clone()),
                };
                let rect = Rect::new(bounds.x, y + 16.0, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(
                    draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
                    scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
                render_widget(&node, rect, &mut ctx);
            }
            WindowMeasure::Toggle {
                id,
                icon_id,
                label,
                pressed,
                text,
                on_change,
            } => {
                let node = WidgetNode::Toggle {
                    id: id.clone(),
                    icon_id: icon_id.clone(),
                    pressed: *pressed,
                    text: text.clone().or(label.clone()),
                    on_change: Some(on_change.clone()),
                };
                let rect = Rect::new(bounds.x, y, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(
                    draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
                    scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
                render_widget(&node, rect, &mut ctx);
            }
        }
    }

    fn render_window_engagement_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        content: &mut Rect,
        window_id: &str,
        kind: &semio_framework_core::WindowKindDefinition,
        gpu: &mut ui_wgpu::GpuContext,
    ) -> Option<(Rect, String)> {
        let measures_expanded = self
            .measures_expanded
            .get(window_id)
            .copied()
            .unwrap_or(false);
        if measures_expanded {
            return None;
        }
        let window_active = self.active_window_id.as_deref() == Some(window_id);
        if !window_active {
            return None;
        }
        let engagement = self
            .window_engagements
            .get(&kind.id)
            .cloned()
            .or_else(|| kind.engagement.clone());
        let Some(engagement) = engagement else {
            return None;
        };
        let activated = self
            .engagement_activated
            .get(window_id)
            .copied()
            .unwrap_or(false);
        if !activated {
            let item = ChromeGroupItem {
                control_id: "",
                icon_id: Some("chevron-right"),
                label: Some("Command"),
                active: false,
                kind: HitKind::Button,
            };
            let chip_w = measure_chrome_group_item(atlas, theme, &item);
            let chip = Rect::new(content.x, content.y + 8.0, chip_w, theme.control_height);
            if let Some(chip_draw) = overlay.as_deref_mut() {
                render_chrome_group(chip_draw, atlas, icons, input, theme, chip, &[item], false);
            } else {
                render_chrome_group(draw, atlas, icons, input, theme, chip, &[item], false);
            }
            return Some((chip, format!("shell.engagement.toggle.{window_id}")));
        }
        let rail_w = DEFAULT_ENGAGEMENT_RAIL_WIDTH;
        let rail = Rect::new(content.x, content.y, rail_w, content.h);
        draw.push_glass([rail.x, rail.y, rail.w, rail.h], theme.border_radius, GlassTier::WindowOptions, theme);
        let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
        draw.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
        let toggle_item = ChromeGroupItem {
            control_id: "shell.engagement.toggle",
            icon_id: Some("chevron-left"),
            label: Some("Command"),
            active: false,
            kind: HitKind::Button,
        };
        let toggle_w = measure_chrome_group_item(atlas, theme, &toggle_item);
        let toggle_rect = Rect::new(header.x, header.y, toggle_w, header.h);
        render_chrome_group(
            draw,
            atlas,
            icons,
            input,
            theme,
            toggle_rect,
            &[toggle_item],
            true,
        );
        input.register_hit(HitTarget {
            rect: toggle_rect,
            event: None,
            control_id: Some(format!("shell.engagement.toggle.{window_id}")),
            kind: HitKind::Button,
            drag_axis: None,
            drag_data: None,
        });
        let mut y = rail.y + theme.panel_header_height;
        if let Some(options) = &engagement.options {
            for option in options {
                let label = option.label.clone().unwrap_or_else(|| option.id.clone());
                let pressed = option.pressed.unwrap_or(false);
                let item = ChromeGroupItem {
                    control_id: "shell.engagement.option",
                    icon_id: None,
                    label: Some(&label),
                    active: pressed,
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(rail.x + 8.0, y, item_w, theme.control_height);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                if let Some(command) = &option.command {
                    input.register_hit(HitTarget {
                        rect,
                        event: Some(command.clone()),
                        control_id: Some(format!("shell.engagement.option.{}.{}", window_id, option.id)),
                        kind: HitKind::Button,
                        drag_axis: None,
                    drag_data: None,
                    });
                }
                y += theme.control_height + 4.0;
            }
        }
        if let Some(input_spec) = &engagement.input {
            self.render_engagement_input(
                draw, overlay, atlas, icons, input, theme,
                Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height * 2.0),
                window_id, input_spec, gpu,
            );
            y += theme.control_height * 2.0 + 8.0;
        }
        if let Some(control) = &engagement.control {
            self.render_engagement_control(
                draw, overlay, atlas, icons, input, theme,
                Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height),
                control, gpu,
            );
        }
        if let Some(status_rows) = &engagement.status {
            for row in status_rows {
                y += theme.control_height;
                chrome_text(
                    draw, atlas, input, theme, &row.text,
                    rail.x + 8.0, y, theme.font_size_small, theme.text_muted,
                );
            }
        }
        if let Some(possibles) = &engagement.possible_engagements {
            for possible in possibles {
                y += theme.control_height + 2.0;
                let rect = Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height);
                draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius);
                chrome_text(
                    draw, atlas, input, theme, &possible.label,
                    rect.x + 8.0, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0,
                    theme.font_size_small, theme.text,
                );
                if let Some(command) = &possible.command {
                    input.register_hit(HitTarget {
                        rect,
                        event: Some(command.clone()),
                        control_id: Some(format!("shell.engagement.possible.{}.{}", window_id, possible.id)),
                        kind: HitKind::Button,
                        drag_axis: None,
                    drag_data: None,
                    });
                }
            }
        }
        content.x += rail_w + theme.gap_standard;
        content.w -= rail_w + theme.gap_standard;
        None
    }

    fn render_engagement_input(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        spec: &WindowEngagementInput,
        _gpu: &mut ui_wgpu::GpuContext,
    ) {
        let id = spec
            .id
            .clone()
            .unwrap_or_else(|| format!("engagement-input-{window_id}"));
        let value = self
            .engagement_inputs
            .get(&id)
            .cloned()
            .or_else(|| spec.value.clone())
            .unwrap_or_default();
        let node = ui_wgpu::widgets::WidgetNode::Input {
            id: id.clone(),
            input_kind: "text".into(),
            value,
            placeholder: spec.placeholder.clone(),
            commit: None,
            on_change: spec.on_change.clone(),
        };
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let mut ctx = framework_widget_context(
            draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
            scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
        ui_wgpu::widgets::render_widget(&node, bounds, &mut ctx);
    }

    fn render_engagement_control(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        bounds: Rect,
        control: &WindowEngagementControl,
        _gpu: &mut ui_wgpu::GpuContext,
    ) {
        use ui_wgpu::widgets::{render_widget, WidgetNode};
        let node = match control {
            WindowEngagementControl::Slider { id, value, min, max, step, on_change, .. } => {
                WidgetNode::Slider {
                    id: id.clone().unwrap_or_else(|| "engagement-slider".into()),
                    value: *value,
                    min: *min,
                    max: *max,
                    step: step.unwrap_or(0.01),
                    on_change: on_change.clone(),
                }
            }
            WindowEngagementControl::Stepper { id, value, step, on_change, .. } => {
                WidgetNode::NumberStepper {
                    id: id.clone().unwrap_or_else(|| "engagement-stepper".into()),
                    value: *value,
                    step: step.unwrap_or(1.0),
                    uniform: false,
                    on_absolute: on_change.clone(),
                    on_delta: on_change.clone(),
                }
            }
            WindowEngagementControl::Select { id, value, items, on_change, .. } => {
                WidgetNode::Select {
                    id: id.clone().unwrap_or_else(|| "engagement-select".into()),
                    value: value.clone().unwrap_or_default(),
                    items: items
                        .iter()
                        .map(|item| ui_wgpu::widgets::SelectItem {
                            value: item.value.clone(),
                            label: item.label.clone(),
                        })
                        .collect(),
                    placeholder: None,
                    on_change: on_change.clone(),
                }
            }
            WindowEngagementControl::Ring { id, value, on_select, .. } => {
                WidgetNode::Ring {
                    id: id.clone().unwrap_or_else(|| "engagement-ring".into()),
                    t: value
                        .as_ref()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.5),
                    disabled: false,
                    on_change: on_select.clone(),
                }
            }
            WindowEngagementControl::ToggleGroup { id, value, options, on_select, .. } => {
                let label = value
                    .clone()
                    .or_else(|| options.first().map(|o| o.id.clone()))
                    .unwrap_or_else(|| "toggle".into());
                WidgetNode::Toggle {
                    id: id.clone().unwrap_or_else(|| "engagement-toggle".into()),
                    icon_id: String::new(),
                    pressed: false,
                    text: Some(label),
                    on_change: on_select.clone(),
                }
            }
        };
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let mut ctx = framework_widget_context(
            draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme,
            scroll_offsets, collapsed_sections, open_selects,
                    None,
                );
        render_widget(&node, bounds, &mut ctx);
    }

    fn render_context_menu(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        menu: &ContextMenuState,
    ) {
        let row_h = theme.control_height;
        let w = 180.0;
        let h = menu.items.len() as f32 * row_h + 8.0;
        let rect = Rect::new(menu.x, menu.y, w, h);
        overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, GlassTier::Menu, theme);
        for (index, item) in menu.items.iter().enumerate() {
            let row = Rect::new(rect.x + 4.0, rect.y + 4.0 + index as f32 * row_h, w - 8.0, row_h);
            overlay.push_rounded([row.x, row.y, row.w, row.h], theme.button, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, &item.label,
                row.x + 8.0,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                theme.text);
            input.register_hit(HitTarget {
                rect: row,
                event: item.command.clone(),
                control_id: Some(item.id.clone()),
                kind: HitKind::ContextMenu,
                drag_axis: None,
            drag_data: None,
            });
        }
    }

    fn render_theme_dropdown(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
    ) {
        let options = [("system", "System"), ("light", "Light"), ("dark", "Dark")];
        let row_h = theme.control_height;
        let w = 112.0;
        let h = options.len() as f32 * row_h + theme.padding_standard * 2.0;
        overlay.push_glass([x, y, w, h], theme.border_radius, GlassTier::Menu, theme);
        for (index, (value, label)) in options.iter().enumerate() {
            let row = Rect::new(
                x + theme.gap_standard,
                y + theme.gap_standard + index as f32 * row_h,
                w - theme.gap_standard * 2.0,
                row_h,
            );
            let selected = *value == self.theme_id;
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let bg = if selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, label,
                row.x + theme.padding_standard,
                row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                if selected || hovered { theme.active_foreground } else { theme.text });
            input.register_hit(HitTarget {
                rect: row,
                event: None,
                control_id: Some(format!("shell.theme.{value}")),
                kind: HitKind::DropdownItem,
                drag_axis: None,
            drag_data: None,
            });
        }
    }

    fn render_palette(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<CommandDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
        w: f32,
        title: &str,
        hint: &str,
    ) {
        let h = 120.0;
        overlay.push_glass([x, y, w, h], theme.border_radius, GlassTier::Menu, theme);
        chrome_text(overlay, atlas, input, theme, title,
            x + 12.0,
            y + 24.0,
            theme.font_size_body,
            theme.text);
        if !hint.is_empty() {
            chrome_text(overlay, atlas, input, theme, hint,
                x + 12.0,
                y + 48.0,
                theme.font_size_small,
                theme.text_muted,);
        }
        let filter_rect = Rect::new(x + 8.0, y + h - theme.control_height - 8.0, w - 16.0, theme.control_height);
        overlay.push_rounded(
            [filter_rect.x, filter_rect.y, filter_rect.w, filter_rect.h],
            theme.input_bg,
            theme.border_radius,
        );
        input.register_hit(HitTarget {
            rect: filter_rect,
            event: None,
            control_id: Some(format!("shell.palette.{title}")),
            kind: HitKind::Input,
            drag_axis: None,
        drag_data: None,
        });
    }
}
//#endregion ShellChrome

#[cfg(target_arch = "wasm32")]
fn download_media_export(filename: &str, mime_type: &str, data: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, HtmlAnchorElement, Url};

    let window = match web_sys::window() {
        Some(window) => window,
        None => return,
    };
    let document = match window.document() {
        Some(document) => document,
        None => return,
    };
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(data));
    let blob = Blob::new_with_str_sequence(&parts).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap();
    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.set_attribute("type", mime_type).ok();
    anchor.click();
    Url::revoke_object_url(&url).ok();
}

#[cfg(not(target_arch = "wasm32"))]
fn download_media_export(_filename: &str, _mime_type: &str, _data: &str) {}

#[cfg(target_arch = "wasm32")]
fn toggle_fullscreen() {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    if document.fullscreen_element().is_some() {
        let _ = document.exit_fullscreen();
    } else if let Some(element) = document.document_element() {
        let _ = element
            .dyn_ref::<web_sys::HtmlElement>()
            .map(|el| el.request_fullscreen());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_fullscreen() {}
// #endregion shell
}


use plugin_bridge::{filter_plugins, parse_plugin_entries};
use infinite_world::{
    apply_glb_bytes, apply_world_command_preview, collect_pending_glb_fetches, fetch_url_bytes,
    handle_world3d_paint_commands, handle_world3d_pointer_button, handle_world3d_pointer_drag,
    handle_world3d_pointer_move, handle_world3d_wheel,
};
use semio_framework_core::CommandDescriptor;
use shell::ShellState;
use std::cell::RefCell;
use std::rc::Rc;
use ui_wgpu::{
    apply_canvas_cursor, attach_dom_listeners, fetch_font_bytes, resolve_semio_cursor, schedule_frame,
    CursorDragState, DrawList, FontAtlas, GpuContext, IconAtlas, InputState, KeyAction, PointerCallbacks,
    PointerModifiers, SemioCursor, Theme,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

fn prefers_dark_scheme() -> bool {
    web_sys::window()
        .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|query| query.matches())
        .unwrap_or(true)
}

fn resolve_theme(theme_id: &str) -> Theme {
    match theme_id {
        "light" => Theme::light(),
        "dark" => Theme::dark(),
        _ if prefers_dark_scheme() => Theme::dark(),
        _ => Theme::light(),
    }
}

fn theme_is_dark(theme_id: &str) -> bool {
    match theme_id {
        "light" => false,
        "dark" => true,
        _ => prefers_dark_scheme(),
    }
}

struct AppRuntime {
    gpu: GpuContext,
    atlas: FontAtlas,
    icons: IconAtlas,
    shell: ShellState,
    draw: DrawList,
    overlay: DrawList,
    input: InputState<CommandDescriptor>,
    theme: Theme,
    canvas: web_sys::HtmlCanvasElement,
    theme_dark: bool,
    last_cursor: Option<(SemioCursor, bool)>,
    last_pointer_x: f32,
    last_pointer_y: f32,
    pointer_down: bool,
    pointer_button: i16,
    modifiers: PointerModifiers,
    wheel_delta: f32,
    asset_poll_pending: bool,
    self_weak: std::rc::Weak<RefCell<AppRuntime>>,
}

impl AppRuntime {
    fn frame(&mut self) {
        self.theme = resolve_theme(&self.shell.theme_id);
        self.theme_dark = theme_is_dark(&self.shell.theme_id);
        self.input.update_hover(self.last_pointer_x, self.last_pointer_y);
        self.input.clear_frame();
        self.draw.clear();
        self.overlay.clear();
        let wheel_delta = self.wheel_delta;
        self.wheel_delta = 0.0;
        if wheel_delta.abs() > 0.0 {
            let x = self.last_pointer_x;
            let y = self.last_pointer_y;
            let ctrl = self.modifiers.ctrl;
            self.shell
                .handle_pointer_wheel(x, y, wheel_delta, &self.input);
            for state in self.shell.world3d_states.values_mut() {
                if state.bounds.contains(x, y) {
                    handle_world3d_wheel(state, wheel_delta);
                }
            }
            let mut graph_commands = Vec::new();
            for (surface_id, surface) in &self.shell.node_graph_states {
                if surface.bounds.contains(x, y) {
                    graph_commands.extend(engine_canvas::node_graph_wheel(
                        surface_id,
                        &surface.controller_id,
                        surface.bounds,
                        x,
                        y,
                        wheel_delta,
                        ctrl,
                    ));
                }
            }
            if !graph_commands.is_empty() {
                let runtime = self.self_weak.clone();
                spawn_local(async move {
                    if let Some(runtime) = runtime.upgrade() {
                        if let Ok(mut app) = runtime.try_borrow_mut() {
                            app.dispatch_commands(graph_commands).await;
                        }
                    }
                });
            }
        }
        ICON_ATLAS_RUNTIME.with(|cell| {
            if let Some(atlas) = cell.borrow_mut().take() {
                self.icons = atlas;
                self.gpu.upload_icon_atlas(&self.icons);
            }
        });
        self.shell.render_chrome(
            &mut self.draw,
            &mut self.overlay,
            &mut self.atlas,
            &self.icons,
            &mut self.input,
            &self.theme,
            &mut self.gpu,
        );
        for upload in scenes::drain_pending_raster_uploads() {
            self.gpu.ensure_raster_texture(&upload.key, &upload.pixels, upload.width, upload.height);
        }
        if self.atlas.take_dirty() {
            self.gpu.upload_font_atlas(&self.atlas);
        }
        if let Err(err) = self.gpu.render_frame(&self.draw, Some(&self.overlay)) {
            web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] render frame: {err}")));
        }
        let hit = self
            .input
            .hit_at(self.last_pointer_x, self.last_pointer_y);
        let cursor = resolve_semio_cursor(
            hit,
            CursorDragState {
                tree_drag: self.shell.tree_drag.is_some(),
                dock_drag: self.shell.dock_drag.is_some(),
                pointer_drag_active: self.input.drag.active,
                pointer_drag_axis: self.input.drag.axis,
                pointer_drag_kind: self.input.drag.kind,
            },
        );
        apply_canvas_cursor(
            &self.canvas,
            cursor,
            self.theme_dark,
            &mut self.last_cursor,
        );
        if !self.asset_poll_pending {
            self.asset_poll_pending = true;
            let runtime = self.self_weak.clone();
            spawn_local(async move {
                let Some(runtime) = runtime.upgrade() else {
                    return;
                };
                let pending = {
                    let Ok(app) = runtime.try_borrow() else {
                        return;
                    };
                    collect_pending_glb_fetches(&app.shell.world3d_states)
                };
                let mut fetched = Vec::new();
                for item in pending {
                    if let Some(bytes) = fetch_url_bytes(&item.url).await {
                        fetched.push((item.surface_id, item.url, bytes));
                    }
                }
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    for (surface_id, url, bytes) in fetched {
                        if let Some(state) = app.shell.world3d_states.get_mut(&surface_id) {
                            apply_glb_bytes(state, &url, &bytes);
                        }
                    }
                    app.asset_poll_pending = false;
                };
            });
        }
    }

    fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.gpu.resize(css_width, css_height, dpr);
        self.shell.screen_w = (css_width * dpr).max(1.0);
        self.shell.screen_h = (css_height * dpr).max(1.0);
    }

    fn handle_key(&mut self, action: KeyAction, modifiers: PointerModifiers) {
        let activate_search = matches!(self.shell.overlay_state, shell::OverlayState::Search)
            && action == KeyAction::Enter;
        let activate_find = matches!(self.shell.overlay_state, shell::OverlayState::Find)
            && action == KeyAction::Enter;
        let search_index = self.shell.search_selected;
        let find_index = self.shell.find_selected;
        self.shell
            .handle_keyboard(action, &modifiers, &mut self.input);
        if activate_search {
            let runtime = self.self_weak.clone();
            spawn_local(async move {
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        let _ = app.shell.activate_search_item(search_index).await;
                    }
                }
            });
        } else if activate_find {
            let runtime = self.self_weak.clone();
            spawn_local(async move {
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        let _ = app.shell.activate_find_item(find_index).await;
                    }
                }
            });
        }
    }

    async fn dispatch_commands(&mut self, commands: Vec<CommandDescriptor>) {
        for command in commands {
            for state in self.shell.world3d_states.values_mut() {
                if state.controller_id == command.controller_id {
                    apply_world_command_preview(state, &command);
                }
            }
            if let Err(err) = self.shell.dispatch_command(command).await {
                web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] command failed: {err}")));
            }
        }
    }

    async fn handle_pointer_button(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        modifiers: PointerModifiers,
    ) {
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        let mut world_commands = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(command) = handle_world3d_pointer_button(
                state,
                x,
                y,
                down,
                button,
                &modifiers,
            ) {
                apply_world_command_preview(state, &command);
                world_commands.push(command);
            }
            for command in handle_world3d_paint_commands(state, x, y, down, button) {
                apply_world_command_preview(state, &command);
                world_commands.push(command);
            }
            if let Some(command) = handle_world3d_pointer_move(state, x, y, down, button) {
                apply_world_command_preview(state, &command);
                world_commands.push(command);
            }
        }
        if !world_commands.is_empty() {
            self.dispatch_commands(world_commands).await;
            return;
        }
        let mut graph_commands = Vec::new();
        for (surface_id, surface) in &self.shell.node_graph_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            if down {
                graph_commands.extend(engine_canvas::node_graph_pointer_down(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    button,
                    modifiers.shift,
                    modifiers.ctrl,
                    modifiers.alt,
                ));
            } else {
                graph_commands.extend(engine_canvas::node_graph_pointer_up(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    modifiers.shift,
                    modifiers.ctrl,
                    modifiers.alt,
                ));
            }
        }
        if !graph_commands.is_empty() {
            self.dispatch_commands(graph_commands).await;
        }
        if let Err(err) = self
            .shell
            .handle_pointer_button(x, y, down, button, &mut self.input)
            .await
        {
            web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] pointer failed: {err}")));
        }
    }

    async fn handle_pointer_move(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        modifiers: PointerModifiers,
    ) {
        let drag_dx = x - self.last_pointer_x;
        let drag_dy = y - self.last_pointer_y;
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        self.shell
            .handle_pointer_move(x, y, down, &mut self.input, &self.theme);
        if let Err(err) = self.shell.flush_deferred_commands().await {
            web_sys::console::warn_1(&JsValue::from_str(&format!("[DEBUG] deferred commands: {err}")));
        }
        if down && (button == 0 || button == 2 || button == 1) {
            for state in self.shell.world3d_states.values_mut() {
                if state.bounds.contains(x, y) {
                    handle_world3d_pointer_drag(
                        state,
                        x,
                        y,
                        drag_dx,
                        drag_dy,
                        button,
                        &modifiers,
                    );
                }
            }
        }
        let mut world_commands = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(command) = handle_world3d_pointer_move(state, x, y, down, button) {
                apply_world_command_preview(state, &command);
                world_commands.push(command);
            }
            for command in handle_world3d_paint_commands(state, x, y, down, button) {
                apply_world_command_preview(state, &command);
                world_commands.push(command);
            }
        }
        let mut graph_commands = Vec::new();
        for (surface_id, surface) in &self.shell.node_graph_states {
            if surface.bounds.contains(x, y) {
                graph_commands.extend(engine_canvas::node_graph_pointer_move(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    modifiers.shift,
                    modifiers.ctrl,
                    modifiers.alt,
                ));
            }
        }
        if !graph_commands.is_empty() {
            self.dispatch_commands(graph_commands).await;
        }
        if !world_commands.is_empty() {
            self.dispatch_commands(world_commands).await;
        }
    }

    async fn handle_context_menu(&mut self, x: f32, y: f32) {
        let _ = self
            .shell
            .handle_pointer_button(x, y, true, 2, &mut self.input)
            .await;
    }
}

fn start_frame_loop(runtime: Rc<RefCell<AppRuntime>>) {
    let next = runtime.clone();
    schedule_frame(move || {
        if let Ok(mut app) = next.try_borrow_mut() {
            app.frame();
        }
        start_frame_loop(next.clone());
    });
}

#[wasm_bindgen(js_name = semioRendererBoot)]
pub async fn semio_renderer_boot(
    canvas: web_sys::HtmlCanvasElement,
    plugins: JsValue,
    plugin_filter: String,
) -> Result<(), JsValue> {
    let dpr = web_sys::window()
        .map(|w| w.device_pixel_ratio() as f32)
        .unwrap_or(1.0);
    let css_width = canvas.client_width().max(1) as f32;
    let css_height = canvas.client_height().max(1) as f32;
    canvas.set_width((css_width * dpr) as u32);
    canvas.set_height((css_height * dpr) as u32);

    const ANTA_LATIN: &[u8] = include_bytes!("../../../../ui/asset/font/anta/latin.ttf");
    let font_bytes = match fetch_font_bytes("/asset/font/anta/latin.ttf").await {
        Ok(bytes) if bytes.len() > 256 => bytes,
        _ => ANTA_LATIN.to_vec(),
    };
    let atlas = FontAtlas::from_bytes(&font_bytes)
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] atlas failed: {err}")))?;

    let mut gpu = GpuContext::from_canvas(canvas.clone(), dpr)
        .await
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] gpu init failed: {err}")))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);

    let entries = parse_plugin_entries(plugins)
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] plugin parse failed: {err}")))?;
    let filtered = filter_plugins(entries, &plugin_filter);
    let mut shell = ShellState::new(filtered, plugin_filter.clone());
    shell.screen_w = css_width * dpr;
    shell.screen_h = css_height * dpr;
    shell
        .boot()
        .await
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] shell boot failed: {err}")))?;

    let runtime = Rc::new(RefCell::new(AppRuntime {
        gpu,
        atlas,
        icons: IconAtlas::default(),
        shell,
        draw: DrawList::default(),
        overlay: DrawList::default(),
        input: InputState::default(),
        theme: Theme::default(),
        canvas: canvas.clone(),
        theme_dark: theme_is_dark("system"),
        last_cursor: None,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        modifiers: PointerModifiers::default(),
        wheel_delta: 0.0,
        asset_poll_pending: false,
        self_weak: std::rc::Weak::new(),
    }));
    runtime.borrow_mut().self_weak = Rc::downgrade(&runtime);

    start_frame_loop(runtime.clone());

    let runtime_pointer = runtime.clone();
    let runtime_move = runtime.clone();
    let runtime_wheel = runtime.clone();
    let runtime_keyboard = runtime.clone();
    let runtime_context = runtime.clone();

    attach_dom_listeners(
        &canvas,
        PointerCallbacks {
            on_move: Rc::new(move |x, y, down, button, modifiers| {
                let runtime = runtime_move.clone();
                spawn_local(async move {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.handle_pointer_move(x, y, down, button, modifiers).await;
                    }
                });
            }),
            on_button: Rc::new(move |x, y, down, button, modifiers| {
                let runtime = runtime_pointer.clone();
                spawn_local(async move {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.handle_pointer_button(x, y, down, button, modifiers).await;
                    }
                });
            }),
            on_wheel: Rc::new(move |delta, _x, _y, _modifiers| {
                if let Ok(mut app) = runtime_wheel.try_borrow_mut() {
                    app.wheel_delta += delta;
                }
            }),
            on_key: Rc::new(move |action, modifiers| {
                if let Ok(mut app) = runtime_keyboard.try_borrow_mut() {
                    app.handle_key(action, modifiers);
                }
            }),
            on_context_menu: Rc::new(move |x, y| {
                let runtime = runtime_context.clone();
                spawn_local(async move {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.handle_context_menu(x, y).await;
                    }
                });
            }),
        },
    );

    let runtime_resize = runtime.clone();
    let canvas_resize = canvas.clone();
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        let Ok(mut app) = runtime_resize.try_borrow_mut() else {
            return;
        };
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio() as f32)
            .unwrap_or(1.0);
        let w = canvas_resize.client_width().max(1) as f32;
        let h = canvas_resize.client_height().max(1) as f32;
        canvas_resize.set_width((w * dpr) as u32);
        canvas_resize.set_height((h * dpr) as u32);
        app.resize(w, h, dpr);
    }) as Box<dyn FnMut()>);
    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
    }
    closure.forget();

    web_sys::console::log_1(&JsValue::from_str("[DEBUG] wgpu renderer booted"));
    Ok(())
}

#[wasm_bindgen(js_name = uploadIconAtlas)]
pub fn upload_icon_atlas(width: u32, height: u32, pixels: &[u8], entries_json: &str) -> Result<(), JsValue> {
    let entries_map: std::collections::HashMap<String, [f32; 4]> = serde_json::from_str(entries_json)
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] icon entries parse: {err}")))?;
    let entries: Vec<(String, [f32; 4])> = entries_map.into_iter().collect();
    ICON_ATLAS_RUNTIME.with(|cell| {
        cell.borrow_mut().replace(IconAtlas::from_packed(width, height, pixels.to_vec(), entries));
    });
    Ok(())
}

thread_local! {
    static ICON_ATLAS_RUNTIME: RefCell<Option<IconAtlas>> = RefCell::new(None);
}
