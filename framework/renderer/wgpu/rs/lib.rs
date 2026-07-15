//! 🧊 Raw wgpu WASM renderer for declarative framework UiNode trees.
//!
//! 🧭 Rough correspondence with the React shell (`framework/renderer/react/os-shell.tsx`), as a
//! discoverability breadcrumb rather than a rigorous mapping:
//! - this crate's top-level shell/state struct ~ React's `#region 🔖types` + `FrameworkOsShell`.
//! - the `dock` module below (window tree, stack chrome, split resize) ~ React's `Mode`
//!   component and the `WindowLayoutNode` tree helpers in `#region ShellHelpers`.
//! - `interpreter`/widget rendering ~ React's `UiNode` component tree rendering.

pub mod dock {
// #region dock
//! 🪟 Mode dock — multi-window layout tree with stack chrome and split resize.

use semio_framework_core::AppDefinition;
use std::collections::HashMap;
use ui_wgpu::{
    chrome_item_bg, chrome_item_text, draw_text, push_chrome_border, push_chrome_group_border,
    push_window_cap_border, even_window_layout, ActionDescriptor, DrawList, DragAxis, FontAtlas, GlassTier,
    HitKind, HitTarget, IconAtlas, InputState, Rect, Rgba, Theme, WindowLayout, WindowLayoutChild,
    WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
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
    pub input: &'a mut InputState<ActionDescriptor>,
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
        windows.insert(to.min(windows.len()), window_id);
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
                    render_stack(self, ctx, path, node, rect, true, true, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(self, ctx, &self.root, bounds, &empty_path(), true, &mut |_, _| {}, None);
    }

    pub fn paint_chrome(
        &self,
        ctx: &mut DockRenderContext<'_>,
        bounds: Rect,
        body_fill: bool,
    ) {
        if let Some(path) = &self.maximized_stack {
            if let Some(node) = node_at(&self.root, path) {
                if let DockNode::Stack { .. } = node {
                    render_stack(self, ctx, path, node, bounds, true, body_fill, &mut |_, _| {});
                    return;
                }
            }
        }
        render_node(
            self,
            ctx,
            &self.root,
            bounds,
            &empty_path(),
            body_fill,
            &mut |_, _| {},
            None,
        );
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
        DockNode::Row(children) => WindowLayoutRoot::Axis(ui_wgpu::WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: children
                .iter()
                .map(|(child, size)| dock_child_from_node(child, *size))
                .collect(),
        }),
        DockNode::Column(children) => WindowLayoutRoot::Axis(ui_wgpu::WindowLayoutAxisNode {
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
        DockNode::Row(children) => WindowLayoutChild::Axis(ui_wgpu::WindowLayoutAxisNode {
            kind: "row".into(),
            size: Some(size as f64),
            children: children
                .iter()
                .map(|(child, child_size)| dock_child_from_node(child, *child_size))
                .collect(),
        }),
        DockNode::Column(children) => WindowLayoutChild::Axis(ui_wgpu::WindowLayoutAxisNode {
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

/// 🪟 Wgpu-local adapter: builds the balanced fallback layout via
/// `ui_wgpu::even_window_layout` and converts it to a runtime `DockNode`.
fn even_layout(window_ids: &[String]) -> DockNode {
    dock_from_window_layout(&even_window_layout(window_ids).root)
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
    body_fill: bool,
    render_body: &mut dyn FnMut(Rect, &str),
    outer_split: Option<(DockPath, usize, bool)>,
) {
    match node {
        DockNode::Row(children) => render_axis(state, ctx, children, bounds, path, true, body_fill, render_body, outer_split),
        DockNode::Column(children) => render_axis(state, ctx, children, bounds, path, false, body_fill, render_body, outer_split),
        DockNode::Stack { .. } => {
            let maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            render_stack(state, ctx, path, node, bounds, maximized, body_fill, render_body)
        }
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
    body_fill: bool,
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
                body_fill,
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
                body_fill,
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
    body_fill: bool,
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
    let cap_glass = ctx.draw.push_glass(
        [cap_rect.x, cap_rect.y, cap_rect.w, cap_rect.h],
        0.0,
        GlassTier::Toolbar,
        theme,
    );
    ctx.draw.begin_glass_content(cap_glass);

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
        if stack_active_tab {
            ctx.draw.push_solid([tab_rect.x, tab_rect.y, tab_rect.w, tab_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([tab_rect.x, tab_rect.y, tab_rect.w, tab_rect.h], theme.button_hover);
        }
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
    ctx.draw.end_glass_content();

    let body_y = cap_y + tab_h;
    let body_x = if per_tab_chrome { active_tab_x } else { bounds.x };
    let body_w = if per_tab_chrome {
        (gap_x + gap_w - active_tab_x).max(0.0)
    } else {
        bounds.w
    };
    let body_rect = Rect::new(body_x, body_y, body_w, bounds.h - tab_h);
    if body_fill {
        ctx.draw.push_solid([body_rect.x, body_rect.y, body_rect.w, body_rect.h], theme.canvas_clear);
    }
    ctx.draw
        .push_solid([body_rect.x, body_rect.y, stroke, body_rect.h], border);
    ctx.draw
        .push_solid([body_rect.x + body_rect.w - stroke, body_rect.y, stroke, body_rect.h], border);
    ctx.draw
        .push_solid([body_rect.x, body_rect.y + body_rect.h - stroke, body_rect.w, stroke], border);

    if body_fill {
        let content = body_rect.inset(theme.padding_standard);
        render_body(content, active);
    }
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
    stack_body_chrome_rect(bounds, theme, windows, &layout)
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
    use ui_wgpu::{create_default_layout, WindowOptions};
    use crate::shell::ShellState;
    use semio_framework_core::{
        AppDefinition, ModeDefinition, PanelGroup, PanelTabDefinition, PanelTabKind, WindowKindDefinition,
    };

    fn sample_app(window_ids: &[&str], layout: Option<WindowLayout>) -> AppDefinition {
        AppDefinition {
            id: "test".into(),
            label: "Test".into(),
            document: vec!["semio".into(), "test".into()],
            icon_id: None,
            controller_id: "test".into(),
            modes: semio_framework_core::Modes::one(ModeDefinition {
                id: "default".into(),
                label: "Default".into(),
                tools: vec![],
                layout_id: None,
                actions: vec![],
            }),
            default_mode_id: "default".into(),
            window_kinds: semio_framework_core::WindowKinds::try_from(
                window_ids
                    .iter()
                    .map(|id| WindowKindDefinition {
                        id: (*id).into(),
                        label: (*id).into(),
                        body_key: format!("{id}.body"),
                        surface_kind: ui_wgpu::SurfaceKind::Canvas2d,
                        icon_id: None,
                        options: WindowOptions::default(),
                        actions: vec![],
                        tools: vec![],
                        params_schema: None,
                        document_projection_schema: None,
                        input_event_schema: None,
                        output_schema: None,
                        capabilities: vec![],
                    })
                    .collect::<Vec<_>>(),
            )
            .expect("sample_app tests always pass at least one window id"),
            panel_tabs: vec![PanelTabDefinition {
                kind: PanelTabKind::App("tab".into()),
                label: "Tab".into(),
                group: PanelGroup::Workbench,
                body_key: Some("tab.body".into()),
                children: vec![],
            }],
            keybindings: vec![],
            actions: vec![],
            tools: vec![],
            named_layouts: vec![],
            default_layout: layout,
            terminologies: vec![],
            introduction: None,
            dialogs: Vec::new(),
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
        let row_extent = dock.split_axis_extent(&vec![0], canvas).unwrap();
        assert!((row_extent - 1000.0).abs() < 0.1);
        let col_extent = dock.split_axis_extent(&vec![], canvas);
        assert!((col_extent.unwrap() - 800.0).abs() < 0.1);
        let nested_extent = dock.split_axis_extent(&vec![0], canvas).unwrap();
        assert!((nested_extent - 1000.0).abs() < 0.1);
    }

    #[test]
    fn panel_scroll_region_blocks_scene_wheel() {
        let panel_scroll = HitTarget {
            rect: Rect::new(0.0, 0.0, 200.0, 400.0),
            event: None,
            control_id: Some("panel.left.lowpoly".into()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
            drag_data: None,
        };
        assert!(!ShellState::wheel_propagates_to_scene_surface(Some(&panel_scroll)));
        let world = HitTarget {
            rect: Rect::new(0.0, 0.0, 800.0, 600.0),
            event: None,
            control_id: Some("world-surface".into()),
            kind: HitKind::World3d,
            drag_axis: None,
            drag_data: None,
        };
        assert!(ShellState::wheel_propagates_to_scene_surface(Some(&world)));
        let graph_pane = HitTarget {
            rect: Rect::new(0.0, 0.0, 800.0, 600.0),
            event: None,
            control_id: Some("graph-surface.pane".into()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
            drag_data: None,
        };
        assert!(ShellState::wheel_propagates_to_scene_surface(Some(&graph_pane)));
    }

    #[test]
    fn row_layout_stack_content_rects_match_per_window() {
        let layout = create_default_layout(
            &["flow".into(), "preview".into()],
            "row",
            Some(&[68.0, 32.0]),
            Some(&["Flow".into(), "Preview".into()]),
        );
        let app = sample_app(&["flow", "preview"], Some(layout));
        let dock = DockState::from_app(&app, Some("flow"));
        let canvas = Rect::new(0.0, 0.0, 1200.0, 800.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let labels = HashMap::from([
            ("flow".into(), "Flow".into()),
            ("preview".into(), "Preview".into()),
        ]);
        let placements = dock.stack_body_rects(canvas, &theme, &labels, &mut atlas);
        assert_eq!(placements.len(), 2);
        let flow_rect = placements.iter().find(|(_, _, id)| id == "flow").map(|(_, rect, _)| *rect);
        let preview_rect = placements.iter().find(|(_, _, id)| id == "preview").map(|(_, rect, _)| *rect);
        let flow_rect = flow_rect.expect("flow body rect");
        let preview_rect = preview_rect.expect("preview body rect");
        assert!(flow_rect.w > preview_rect.w);
        assert!((flow_rect.x + flow_rect.w - preview_rect.x).abs() < 1.0);
        assert!(flow_rect.h > 0.0 && preview_rect.h > 0.0);
    }

    #[test]
    fn resize_hits_win_over_later_scroll_region() {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = even_layout(&["a".into(), "b".into()]);
        let canvas = Rect::new(0.0, 0.0, 400.0, 300.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::from([
            ("a".into(), "A".into()),
            ("b".into(), "B".into()),
        ]);
        input.register_hit(HitTarget {
            rect: canvas,
            event: None,
            control_id: Some("content.scroll".into()),
            kind: HitKind::ScrollRegion,
            drag_axis: None,
            drag_data: None,
        });
        let mut ctx = DockRenderContext {
            draw: &mut draw,
            atlas: &mut atlas,
            icons: &IconAtlas::default(),
            input: &mut input,
            theme: &theme,
            window_labels: &labels,
        };
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
        assert!(dock.reorder_tab(&vec![], 0, 2));
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
        dock.toggle_maximize(&vec![1]);
        let canvas = Rect::new(0.0, 0.0, 900.0, 600.0);
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
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

    #[test]
    fn map_marquee_mode_matches_ui_react() {
        use crate::engine_canvas::map_marquee_mode;
        assert_eq!(map_marquee_mode(false, false), "default");
        assert_eq!(map_marquee_mode(true, false), "additive");
        assert_eq!(map_marquee_mode(false, true), "subtractive");
        assert_eq!(map_marquee_mode(true, true), "invertive");
    }

    //#region WindowActionsAndToolsTests
    use semio_framework_core::{
        ActionArgDef, ActionDefinition, ActionKind, ActionRef, ToolDefinition, ToolRef,
    };
    use ui_wgpu::{KeyAction, PointerModifiers};

    fn mods(meta: bool, ctrl: bool, shift: bool, alt: bool) -> PointerModifiers {
        PointerModifiers { meta, ctrl, shift, alt }
    }

    /// 🧰 Builds a two-window app: window `main` scopes `tool.a`, window `aux` scopes nothing; `tool.b`
    /// is an orphan (no window references it). Actions: `zeroArg` (no args) + `withArgs` (required text +
    /// defaulted toggle) scoped to `main`.
    fn actions_tools_app() -> AppDefinition {
        let mut app = sample_app(&["main", "aux"], None);
        app.controller_id = "ctrl".into();
        app.tools = vec![
            ToolDefinition::new("tool.a", "Tool A", "icon.a"),
            ToolDefinition {
                allows_actions_while_active: true,
                ..ToolDefinition::new("tool.b", "Tool B", "icon.b")
            },
        ];
        app.actions = vec![
            ActionDefinition::new("zeroArg", "Zero Arg", ActionKind::View),
            ActionDefinition {
                args: vec![
                    ActionArgDef::text("name", "Name").required(),
                    ActionArgDef {
                        default: Some(serde_json::json!(true)),
                        ..ActionArgDef::toggle("flag", "Flag")
                    },
                ],
                keys: Some("mod+e".into()),
                ..ActionDefinition::new("withArgs", "With Args", ActionKind::View)
            },
        ];
        // Scope tool.a + both actions to `main`; leave tool.b an orphan referenced by no window.
        for kind in app.window_kinds.iter_mut() {
            if kind.id == "main" {
                kind.tools = vec![ToolRef::new("tool.a")];
                kind.actions = vec![ActionRef::new("zeroArg"), ActionRef::new("withArgs")];
            }
        }
        app
    }

    fn shell() -> ShellState {
        ShellState::new(vec![], "test".into())
    }

    #[test]
    fn resolve_window_tools_scopes_explicit_and_orphans() {
        let app = actions_tools_app();
        let main = app.window_kinds.iter().find(|k| k.id == "main").unwrap();
        let aux = app.window_kinds.iter().find(|k| k.id == "aux").unwrap();
        let main_ids: Vec<&str> = crate::shell::resolve_window_tools(&app, main)
            .iter()
            .map(|t| t.id.as_str())
            .collect();
        let aux_ids: Vec<&str> = crate::shell::resolve_window_tools(&app, aux)
            .iter()
            .map(|t| t.id.as_str())
            .collect();
        // `main` gets its explicit tool.a first, then the orphan tool.b; `aux` only sees the orphan.
        assert_eq!(main_ids, vec!["tool.a", "tool.b"]);
        assert_eq!(aux_ids, vec!["tool.b"]);
    }

    #[test]
    fn key_event_matches_chord_respects_modifiers() {
        use crate::shell::key_event_matches_chord;
        let z = KeyAction::Char("z".into());
        assert!(key_event_matches_chord(&z, &mods(true, false, false, false), "mod+z"));
        assert!(key_event_matches_chord(&z, &mods(false, true, false, false), "mod+z"));
        // shift held but not declared → no match; declared shift required.
        assert!(!key_event_matches_chord(&z, &mods(true, false, true, false), "mod+z"));
        assert!(key_event_matches_chord(&z, &mods(true, false, true, false), "mod+shift+z"));
        // plain key must not fire while the accelerator is held.
        let k = KeyAction::Char("k".into());
        assert!(key_event_matches_chord(&k, &mods(false, false, false, false), "k"));
        assert!(!key_event_matches_chord(&k, &mods(true, false, false, false), "k"));
        assert!(key_event_matches_chord(&KeyAction::Escape, &mods(false, false, false, false), "escape"));
    }

    #[test]
    fn required_arg_gates_execution_and_merges_defaults() {
        let app = actions_tools_app();
        let defs = &app.actions.iter().find(|a| a.id == "withArgs").unwrap().args;
        // Nothing staged → required `name` missing → no executable args (P2 gate).
        assert!(ShellState::resolved_execute_args(defs, &serde_json::Map::new()).is_none());
        // Stage the required arg → executes, merging the defaulted `flag`.
        let mut staged = serde_json::Map::new();
        staged.insert("name".into(), serde_json::json!("hello"));
        let merged = ShellState::resolved_execute_args(defs, &staged).expect("executable");
        assert_eq!(merged.get("name"), Some(&serde_json::json!("hello")));
        assert_eq!(merged.get("flag"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn staging_stage_and_reset_roundtrip() {
        let mut shell = shell();
        shell.stage_arg("main", "withArgs", "name", serde_json::json!("x"));
        shell.stage_arg("main", "withArgs", "flag", serde_json::json!(false));
        let staged = shell.staged_map_for("main", "withArgs");
        assert_eq!(staged.get("name"), Some(&serde_json::json!("x")));
        assert_eq!(staged.get("flag"), Some(&serde_json::json!(false)));
        shell.reset_staged_args("main", "withArgs");
        assert!(shell.staged_map_for("main", "withArgs").is_empty());
    }

    #[test]
    fn tool_activation_toggles_and_switches() {
        let mut shell = shell();
        shell.apply_set_active_tool("main", "tool.a");
        assert_eq!(shell.active_tool_for_window("main"), Some("tool.a"));
        // Re-selecting the active tool deactivates it (the same update a re-click / Escape performs).
        shell.apply_set_active_tool("main", "tool.a");
        assert_eq!(shell.active_tool_for_window("main"), None);
        // Switching to a different tool activates it.
        shell.apply_set_active_tool("main", "tool.a");
        shell.apply_set_active_tool("main", "tool.b");
        assert_eq!(shell.active_tool_for_window("main"), Some("tool.b"));
    }

    #[test]
    fn active_tool_gates_actions_unless_allowed() {
        let app = actions_tools_app();
        let mut shell = shell();
        // No active tool → actions enabled.
        assert!(shell.actions_enabled_for_window(&app, "main"));
        // tool.a defaults to `allows_actions_while_active = false` → actions gated.
        shell.apply_set_active_tool("main", "tool.a");
        assert!(!shell.actions_enabled_for_window(&app, "main"));
        // tool.b sets the flag true → actions stay enabled.
        shell.apply_set_active_tool("main", "tool.a");
        shell.apply_set_active_tool("main", "tool.b");
        assert!(shell.actions_enabled_for_window(&app, "main"));
    }

    #[test]
    fn action_host_window_id_finds_scoping_window() {
        let app = actions_tools_app();
        assert_eq!(
            crate::shell::action_host_window_id(&app, "withArgs").as_deref(),
            Some("main")
        );
    }

    /// 🎯 The Tool Options rail (`render_window_tool_options_rail`) resolves its content through
    /// `partition_window_measures`: a tagged group surfaces only for its matching active tool, and is
    /// absent from BOTH buckets otherwise — untagged groups always stay in the general Measures rail.
    #[test]
    fn tool_options_partition_gates_tagged_group_by_active_tool() {
        use ui_wgpu::{partition_window_measures, ActionDescriptor, WindowMeasure};
        let measures = vec![
            WindowMeasure::Group {
                id: "brush-params".into(),
                label: "Brush".into(),
                default_open: Some(true),
                active_tool_id: Some("tool.a".into()),
                children: vec![WindowMeasure::Slider {
                    id: "size".into(),
                    label: Some("Size".into()),
                    value: 4.0,
                    min: 1.0,
                    max: 10.0,
                    step: Some(1.0),
                    on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "setSize".into(), args: None },
                }],
            },
            WindowMeasure::Group {
                id: "grid".into(),
                label: "Grid".into(),
                default_open: Some(true),
                active_tool_id: None,
                children: vec![],
            },
        ];
        let (general, tool_options) = partition_window_measures(&measures, Some("tool.a"));
        assert_eq!(tool_options.len(), 1, "matching tool surfaces the tagged group in tool options");
        assert!(matches!(&tool_options[0], WindowMeasure::Group { id, .. } if id == "brush-params"));
        assert_eq!(general.len(), 1, "untagged group stays in the general measures rail");
        assert!(matches!(&general[0], WindowMeasure::Group { id, .. } if id == "grid"));
        let (general_other, tool_options_other) = partition_window_measures(&measures, Some("tool.b"));
        assert!(tool_options_other.is_empty(), "wrong active tool drops the tagged group");
        assert_eq!(general_other.len(), 1, "untagged group unaffected by active tool");
        let (general_none, tool_options_none) = partition_window_measures(&measures, None);
        assert!(tool_options_none.is_empty(), "no active tool drops the tagged group");
        assert_eq!(general_none.len(), 1);
    }
    //#endregion WindowActionsAndToolsTests
}
//#endregion DockTests
// #endregion dock
}

pub mod engine_canvas {
// #region engine_canvas
//! 🎨 Embeds GraphHost, FlowHost, and EditorHost via vello offscreen compositing.

use crate::interpreter::FrameworkWidgetContext;
use flow_core::{dag::dag_screen_to_world, FlowFixture, FlowHost};
use framework_editor::EditorHost;
use framework_graph::GraphHost;
use infinite_cavas as cavas;
use ui_wgpu::{ActionDescriptor, SurfaceKind, UiComponentSceneNode};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ui_wgpu::{draw_text, draw_text_overlay, FontAtlas, GpuContext, HitKind, HitTarget, KeyAction, PointerModifiers, Rect, Rgba, Theme};
use vello::peniko::Color;
use vello::wgpu;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions};

#[cfg(target_arch = "wasm32")]
use js_sys;

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
    evaluated: bool,
}

fn flow_fixture_semantic_eq(left: &FlowFixture, right: &FlowFixture) -> bool {
    left.schema == right.schema
        && left.widgets == right.widgets
        && left.synapses == right.synapses
        && left.layout == right.layout
}

struct EngineSurface {
    node_graph: Option<NodeGraphEngine>,
    sync_cache: NodeGraphSyncCache,
    map_host: Option<gis_2d::MapHost>,
    map_sync_cache: MapSyncCache,
    board_host: Option<puzzle_2d::BoardHost>,
    board_sync_cache: BoardSyncCache,
    board_pending_events: Vec<BoardEventRow>,
    board_pointer_inside: bool,
    editor: Option<EditorHost>,
    vello: Renderer,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    width: u32,
    height: u32,
    last_note_click: Option<(String, f64)>,
}

#[derive(Default)]
struct MapSyncCache {
    map_fixture_json: Option<String>,
    camera_json: Option<String>,
    render_mode: Option<String>,
    vector_style: Option<String>,
    lod_mode: Option<String>,
    layer_visibility_json: Option<String>,
    layer_stroke_scale_json: Option<String>,
    selection_json: Option<String>,
    hover_json: Option<String>,
    theme_json: Option<String>,
    size_key: Option<String>,
}

#[derive(Default)]
struct BoardSyncCache {
    fixture_json: Option<String>,
    kind_catalogs_json: Option<String>,
    kind_compatibility_json: Option<String>,
    selection_json: Option<String>,
    camera_json: Option<String>,
    hovered_id: Option<String>,
    active_tool: Option<String>,
    selection_method: Option<String>,
    grid_snap_enabled: Option<bool>,
    grid_factor: Option<f64>,
    suggestion_offset: Option<f64>,
    brush_kind_weights_json: Option<String>,
    lod_mode: Option<String>,
    size_key: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PendingMapTileFetch {
    pub surface_id: String,
    pub key: String,
    pub url: String,
    pub vector: bool,
    pub z: u32,
    pub x: u32,
    pub y: u32,
}

thread_local! {
    static PENDING_MAP_TILE_FETCHES: RefCell<Vec<PendingMapTileFetch>> = RefCell::new(Vec::new());
    static MAP_TILE_MISS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

fn sync_field(cache: &mut Option<String>, value: &str) -> bool {
    if cache.as_deref() == Some(value) {
        false
    } else {
        *cache = Some(value.to_string());
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

fn sync_canvas_theme_dark(_cache: &mut NodeGraphSyncCache, dark: bool, flow: &mut FlowHost) {
    flow.set_canvas_theme_dark(dark);
}

fn sync_graph_canvas_theme_dark(_cache: &mut NodeGraphSyncCache, dark: bool, graph: &mut GraphHost) {
    graph.set_canvas_theme_dark(dark);
}

thread_local! {
    static ENGINE_SURFACES: RefCell<HashMap<String, EngineSurface>> = RefCell::new(HashMap::new());
}

fn raster_key(surface_id: &str) -> String {
    format!("engine:{surface_id}")
}

fn is_flow_graph(graph: &ui_wgpu::NodeGraphScene) -> bool {
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

fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: scene.controller_id.clone(),
        action: action.to_string(),
        args: Some(args),
    }
}

fn graph_action(controller_id: &str, surface_id: &str, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.to_string(),
        action: action.to_string(),
        args: Some(args),
    }
}

fn graph_scene_json(graph: &ui_wgpu::NodeGraphScene) -> String {
    serde_json::to_string(graph).unwrap_or_else(|_| "{}".into())
}

fn editor_scene_json(editor: &ui_wgpu::TextEditorScene) -> String {
    serde_json::to_string(editor).unwrap_or_else(|_| "{}".into())
}

fn sync_flow_host(host: &mut FlowHost, graph: &ui_wgpu::NodeGraphScene, cache: &mut NodeGraphSyncCache) {
    if let Some(json) = &graph.operators_json {
        if sync_field(&mut cache.operators_json, json) {
            host.set_neuron_kind_infos_json(json);
        }
    }
    if let Some(fixture_json) = &graph.fixture_json {
        if sync_field(&mut cache.fixture_json, fixture_json) {
            if let Ok(fixture) = FlowHost::parse_fixture_json(fixture_json) {
                if flow_fixture_semantic_eq(&host.fixture, &fixture) {
                    host.set_camera(fixture.camera.x, fixture.camera.y, fixture.camera.zoom);
                } else {
                    host.replace_fixture(fixture);
                    let _ = host.evaluate();
                }
            }
        } else if !cache.evaluated {
            let _ = host.evaluate();
        }
        if !cache.evaluated {
            let _ = host.evaluate();
            cache.evaluated = true;
        }
    }
    if let Some(json) = &graph.catalogue_json {
        if sync_field(&mut cache.catalogue_json, json) {
            host.set_host_catalogue_json(json);
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
                if let Some(distance) = value.get("proximityDistance").and_then(|v| v.as_f64()) {
                    host.set_proximity_distance(distance);
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
                    map_host: None,
                    map_sync_cache: MapSyncCache::default(),
                    board_host: None,
                    board_sync_cache: BoardSyncCache::default(),
                    board_pending_events: Vec::new(),
                    board_pointer_inside: false,
                    editor: None,
                    vello,
                    texture,
                    view,
                    width: pw.max(1),
                    height: ph.max(1),
                    last_note_click: None,
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

fn note_widget_hit_at_screen(host: &flow_core::FlowHost, sx: f64, sy: f64) -> Option<(String, f64, f64)> {
    use flow_core::dag::DagNodeKind;
    let (world_x, world_y) = dag_screen_to_world(&host.dag, sx, sy);
    let node = host.dag.fixture.nodes.iter().find(|node| {
        matches!(node.kind, DagNodeKind::Note { .. })
            && world_x >= node.x
            && world_x <= node.x + node.width
            && world_y >= node.y
            && world_y <= node.y + node.height
    })?;
    Some((node.id.clone(), world_x, world_y))
}

#[cfg(target_arch = "wasm32")]
fn engine_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn engine_now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

pub fn node_graph_apply_note_edit_key(action: KeyAction, modifiers: &PointerModifiers) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        for entry in map.values_mut() {
            let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() else {
                continue;
            };
            if host.dag.editing_note_id().is_none() {
                continue;
            }
            match action {
                KeyAction::Char(ch) if !modifiers.ctrl_or_meta() => host.note_insert_text(&ch),
                KeyAction::Backspace => host.note_backspace(),
                KeyAction::Delete => host.note_delete_forward(),
                KeyAction::ArrowLeft => {
                    let _ = host.note_move_caret("left", modifiers.shift);
                }
                KeyAction::ArrowRight => {
                    let _ = host.note_move_caret("right", modifiers.shift);
                }
                KeyAction::Enter | KeyAction::Escape => host.note_commit_edit(),
                _ => return false,
            }
            return true;
        }
        false
    })
}

pub fn node_graph_sync_caret_blink(visible: bool) {
    ENGINE_SURFACES.with(|cell| {
        for entry in cell.borrow_mut().values_mut() {
            if let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() {
                if host.dag.editing_note_id().is_some() {
                    host.set_note_caret_visible(visible);
                }
            }
        }
    });
}

fn node_graph_pan_gesture(button: i16, alt: bool, space_pressed: bool) -> bool {
    button == 1 || (button == 0 && (alt || space_pressed))
}

fn node_graph_set_wheel_zoom_active(entry: &mut EngineSurface, active: bool) {
    match entry.node_graph.as_mut() {
        Some(NodeGraphEngine::Flow(host)) => host.dag.set_wheel_zoom_active(active),
        Some(NodeGraphEngine::Dag(host)) => host.dag.set_wheel_zoom_active(active),
        None => {}
    }
}

pub fn node_graph_clear_wheel_zoom_active() {
    ENGINE_SURFACES.with(|cell| {
        for entry in cell.borrow_mut().values_mut() {
            node_graph_set_wheel_zoom_active(entry, false);
        }
    });
}

const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";

pub fn node_graph_clear_all_ghost_widgets() {
    ENGINE_SURFACES.with(|cell| {
        for entry in cell.borrow_mut().values_mut() {
            if let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() {
                host.clear_ghost_widget();
            }
        }
    });
}

pub fn node_graph_sync_flow_widget_ghost(
    x: f32,
    y: f32,
    drag_data: &HashMap<String, String>,
    surfaces: &[(&str, Rect)],
) {
    let Some(raw) = drag_data.get(FLOW_WIDGET_DRAG_MIME) else {
        node_graph_clear_all_ghost_widgets();
        return;
    };
    let mut over_graph = false;
    for (surface_id, bounds) in surfaces {
        if !bounds.contains(x, y) {
            continue;
        }
        let sx = (x - bounds.x) as f64;
        let sy = (y - bounds.y) as f64;
        ENGINE_SURFACES.with(|cell| {
            if let Some(entry) = cell.borrow_mut().get_mut(*surface_id) {
                if let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() {
                    let (world_x, world_y) = dag_screen_to_world(&host.dag, sx, sy);
                    let _ = host.set_ghost_widget(raw, world_x, world_y);
                    over_graph = true;
                }
            }
        });
        break;
    }
    if !over_graph {
        node_graph_clear_all_ghost_widgets();
    }
}

pub fn node_graph_flow_widget_drop_action(
    x: f32,
    y: f32,
    drag_data: &HashMap<String, String>,
    surfaces: &[(&str, Rect, &str)],
) -> Option<ActionDescriptor> {
    let raw = drag_data.get(FLOW_WIDGET_DRAG_MIME)?;
    let descriptor: Value = serde_json::from_str(raw).ok()?;
    for (surface_id, bounds, controller_id) in surfaces {
        if !bounds.contains(x, y) {
            continue;
        }
        let sx = (x - bounds.x) as f64;
        let sy = (y - bounds.y) as f64;
        let world = ENGINE_SURFACES.with(|cell| {
            cell.borrow().get(*surface_id).and_then(|entry| {
                let NodeGraphEngine::Flow(host) = entry.node_graph.as_ref()? else {
                    return None;
                };
                Some(dag_screen_to_world(&host.dag, sx, sy))
            })
        })?;
        return Some(ActionDescriptor {
            controller_id: (*controller_id).to_string(),
            action: "addWidget".into(),
            args: Some(json!({
                "kind": descriptor.get("kind").and_then(|value| value.as_str()).unwrap_or("inputSlider"),
                "neuronKind": descriptor.get("neuronKind").and_then(|value| value.as_str()),
                "x": world.0,
                "y": world.1,
            })),
        });
    }
    None
}

pub fn node_graph_wheel(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    delta: f32,
    _ctrl: bool,
) -> Vec<ActionDescriptor> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Vec::new();
        };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) => {
                host.dag.set_wheel_zoom_active(true);
                host.wheel_screen(sx, sy, 0.0, delta as f64, true);
            }
            Some(NodeGraphEngine::Dag(host)) => {
                host.dag.set_wheel_zoom_active(true);
                host.wheel_screen(sx, sy, delta as f64, true);
            }
            None => return Vec::new(),
        }
        graph_interaction_actions(surface_id, controller_id, entry)
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
    space_pressed: bool,
) -> Vec<ActionDescriptor> {
    let pan = node_graph_pan_gesture(button, alt, space_pressed);
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Vec::new();
        };
        if button == 0 && !pan && !shift && !ctrl {
            if let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() {
                if let Some((widget_id, world_x, world_y)) = note_widget_hit_at_screen(host, sx, sy) {
                    let now = engine_now_ms();
                    if let Some((last_id, last_ms)) = entry.last_note_click.clone() {
                        if last_id == widget_id && now - last_ms < 400.0 {
                            host.begin_note_edit(&widget_id, world_x, world_y);
                            entry.last_note_click = None;
                            return graph_interaction_actions(surface_id, controller_id, entry);
                        }
                    }
                    entry.last_note_click = Some((widget_id, now));
                } else {
                    entry.last_note_click = None;
                }
            }
        }
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Flow(host)) => {
                host.pointer_down_screen(sx, sy, button as u8, shift, ctrl, alt, pan);
            }
            Some(NodeGraphEngine::Dag(host)) => {
                host.pointer_down_screen(sx, sy, button as u8, shift, ctrl, alt, pan);
            }
            None => return Vec::new(),
        }
        graph_interaction_actions(surface_id, controller_id, entry)
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
) -> Vec<ActionDescriptor> {
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
        graph_interaction_actions(surface_id, controller_id, entry)
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
) -> Vec<ActionDescriptor> {
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
        graph_interaction_actions(surface_id, controller_id, entry)
    })
}

fn graph_interaction_actions(
    surface_id: &str,
    controller_id: &str,
    entry: &EngineSurface,
) -> Vec<ActionDescriptor> {
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
        graph_action(
            controller_id,
            surface_id,
            "nodeGraphSelect",
            json!({ "surfaceId": surface_id, "nodeIds": node_ids }),
        ),
        graph_action(
            controller_id,
            surface_id,
            "nodeGraphHover",
            json!({ "surfaceId": surface_id, "hoverJson": hover_json }),
        ),
        graph_action(
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
    draw_text_overlay(ctx, text, tx, ty, font_px, fill.with_alpha(fill.a * alpha));
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

struct NodeGraphOverlaySnapshot {
    preview_points_json: String,
    preview_crossing: bool,
    preview_method: String,
    selection_bounds_json: String,
}

fn node_graph_overlay_snapshot(surface_id: &str) -> Option<NodeGraphOverlaySnapshot> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => Some(NodeGraphOverlaySnapshot {
                preview_points_json: host.selection_preview_points_json(),
                preview_crossing: host.selection_preview_crossing(),
                preview_method: host.selection_preview_method().to_string(),
                selection_bounds_json: host.selection_union_bounds_screen_json(),
            }),
            Some(NodeGraphEngine::Dag(host)) => Some(NodeGraphOverlaySnapshot {
                preview_points_json: host.dag.selection_preview_points_json(),
                preview_crossing: host.dag.selection_preview_crossing(),
                preview_method: host.dag.selection_preview_method().to_string(),
                selection_bounds_json: host.dag.selection_union_bounds_screen_json(),
            }),
            None => None,
        }
    })
}

fn parse_selection_preview_points(json: &str) -> Vec<(f32, f32)> {
    serde_json::from_str::<Vec<[f64; 2]>>(json)
        .unwrap_or_default()
        .into_iter()
        .map(|point| (point[0] as f32, point[1] as f32))
        .collect()
}

fn paint_node_graph_selection_marquee(
    ctx: &mut FrameworkWidgetContext<'_>,
    inner: Rect,
    points: &[(f32, f32)],
    crossing: bool,
    method: &str,
    theme: &Theme,
) {
    if points.len() < 2 {
        return;
    }
    let lasso = method == "lasso";
    let global: Vec<[f32; 2]> = points
        .iter()
        .map(|(x, y)| [inner.x + x, inner.y + y])
        .collect();
    ui_wgpu::paint_selection_marquee(&mut ctx.draw, theme, crossing, lasso, &global, true);
}

fn paint_node_graph_selection_bounds(
    ctx: &mut FrameworkWidgetContext<'_>,
    inner: Rect,
    bounds_json: &str,
    theme: &Theme,
) {
    if bounds_json.trim() == "null" {
        return;
    }
    let Ok(value) = serde_json::from_str::<Value>(bounds_json) else {
        return;
    };
    let x = value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let y = value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let w = value.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let h = value.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let rx = inner.x + x;
    let ry = inner.y + y;
    let stroke = theme.text_element.with_alpha(0.95);
    ctx.draw.push_line_overlay(rx, ry, rx + w, ry, stroke, 1.0);
    ctx.draw.push_line_overlay(rx + w, ry, rx + w, ry + h, stroke, 1.0);
    ctx.draw.push_line_overlay(rx + w, ry + h, rx, ry + h, stroke, 1.0);
    ctx.draw.push_line_overlay(rx, ry + h, rx, ry, stroke, 1.0);
}

pub fn paint_node_graph_overlays(
    ctx: &mut FrameworkWidgetContext<'_>,
    scene: &UiComponentSceneNode,
    inner: Rect,
) {
    let Some(snapshot) = node_graph_overlay_snapshot(&scene.surface_id) else {
        return;
    };
    let points = parse_selection_preview_points(&snapshot.preview_points_json);
    paint_node_graph_selection_marquee(
        ctx,
        inner,
        &points,
        snapshot.preview_crossing,
        &snapshot.preview_method,
        ctx.theme,
    );
    paint_node_graph_selection_bounds(ctx, inner, &snapshot.selection_bounds_json, ctx.theme);
}
//#endregion NodeGraph

//#region GisMap
fn map_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
    template
        .replace("{z}", &z.to_string())
        .replace("{x}", &x.to_string())
        .replace("{y}", &y.to_string())
}

fn map_theme_json_from_ui_theme(theme: &Theme) -> String {
    let rgba = |color: Rgba| {
        let r = (color.r.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (color.g.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (color.b.clamp(0.0, 1.0) * 255.0).round() as u8;
        let a = (color.a.clamp(0.0, 1.0) * 255.0).round() as u8;
        [r, g, b, a]
    };
    json!({
        "surfaceClear": rgba(theme.canvas_clear),
        "landFill": rgba(theme.panel),
        "landStroke": [rgba(theme.separator)[0], rgba(theme.separator)[1], rgba(theme.separator)[2], 0],
        "labelFill": rgba(theme.text),
        "labelHalo": rgba(theme.canvas_clear),
        "regionFill": rgba(theme.selected.with_alpha(0.22)),
        "regionStroke": rgba(theme.accent),
        "routeStroke": rgba(theme.accent_hover),
        "positionFill": rgba(theme.accent),
        "positionStroke": rgba(theme.active_foreground),
        "selectionStroke": rgba(theme.accent),
        "hoverStroke": rgba(theme.accent_hover),
    })
    .to_string()
}

fn sync_map_host(
    host: &mut gis_2d::MapHost,
    scene: &ui_wgpu::GisMapScene,
    cache: &mut MapSyncCache,
    pw: u32,
    ph: u32,
    dpr: f64,
    theme_json: &str,
) {
    let size_key = format!("{pw}x{ph}@{dpr}");
    if sync_field(&mut cache.size_key, &size_key) {
        host.set_size(pw, ph, dpr);
    }
    if sync_field(&mut cache.map_fixture_json, &scene.map_fixture_json) {
        let _ = host.sync_map_json(&scene.map_fixture_json);
    }
    if sync_field(&mut cache.camera_json, &scene.camera_json) {
        if let Ok(camera) = serde_json::from_str::<Value>(&scene.camera_json) {
            let x = camera.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
            let y = camera.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
            let zoom = camera.get("zoom").and_then(|value| value.as_f64()).unwrap_or(1.0);
            host.set_camera(x, y, zoom);
        }
    }
    if sync_field(&mut cache.render_mode, &scene.render_mode) {
        host.set_render_mode(&scene.render_mode);
    }
    if sync_field(&mut cache.vector_style, &scene.vector_style) {
        host.set_vector_style(&scene.vector_style);
    }
    if sync_field(&mut cache.lod_mode, &scene.lod_mode) {
        host.set_lod_mode(&scene.lod_mode);
    }
    if sync_field(&mut cache.layer_visibility_json, &scene.layer_visibility_json) {
        let _ = host.set_layer_visibility_from_json(&scene.layer_visibility_json);
    }
    if sync_field(&mut cache.layer_stroke_scale_json, &scene.layer_stroke_scale_json) {
        let _ = host.set_layer_stroke_scale_from_json(&scene.layer_stroke_scale_json);
    }
    if sync_field(&mut cache.selection_json, &scene.selection_json) {
        let _ = host.set_selection_json(&scene.selection_json);
    }
    if sync_field(&mut cache.hover_json, &scene.hover_json) {
        let _ = host.set_hover_json(&scene.hover_json);
    }
    if sync_field(&mut cache.theme_json, theme_json) {
        let _ = host.set_map_theme_from_json(theme_json);
    }
}

fn queue_map_tile_fetches(surface_id: &str, scene: &ui_wgpu::GisMapScene, host: &mut gis_2d::MapHost) {
    host.prepare_visible_tiles();
    let needs_raster = scene.render_mode == "image" || scene.render_mode == "combined";
    let needs_vector = scene.render_mode == "vector" || scene.render_mode == "combined";
    PENDING_MAP_TILE_FETCHES.with(|pending| {
        let mut queue = pending.borrow_mut();
        if needs_raster {
            let rows: Vec<Value> = serde_json::from_str(&host.visible_tiles_json()).unwrap_or_default();
            for row in rows {
                let (Some(z), Some(x), Some(y), Some(key)) = (
                    row.get("z").and_then(|value| value.as_u64()).map(|value| value as u32),
                    row.get("x").and_then(|value| value.as_u64()).map(|value| value as u32),
                    row.get("y").and_then(|value| value.as_u64()).map(|value| value as u32),
                    row.get("key").and_then(|value| value.as_str()),
                ) else {
                    continue;
                };
                if host.has_tile(key) {
                    continue;
                }
                let miss_key = format!("raster:{key}");
                if MAP_TILE_MISS.with(|cell| cell.borrow().contains(&miss_key)) {
                    continue;
                }
                if queue.iter().any(|item| item.key == key && item.surface_id == surface_id) {
                    continue;
                }
                queue.push(PendingMapTileFetch {
                    surface_id: surface_id.to_string(),
                    key: key.to_string(),
                    url: map_tile_url(&scene.tile_url_template, z, x, y),
                    vector: false,
                    z,
                    x,
                    y,
                });
            }
        }
        if needs_vector {
            let rows: Vec<Value> = serde_json::from_str(&host.visible_vector_tiles_json()).unwrap_or_default();
            for row in rows {
                let (Some(z), Some(x), Some(y), Some(key)) = (
                    row.get("z").and_then(|value| value.as_u64()).map(|value| value as u32),
                    row.get("x").and_then(|value| value.as_u64()).map(|value| value as u32),
                    row.get("y").and_then(|value| value.as_u64()).map(|value| value as u32),
                    row.get("key").and_then(|value| value.as_str()),
                ) else {
                    continue;
                };
                if host.has_vector_tile(key) {
                    continue;
                }
                let miss_key = format!("vector:{key}");
                if MAP_TILE_MISS.with(|cell| cell.borrow().contains(&miss_key)) {
                    continue;
                }
                if queue.iter().any(|item| item.key == key && item.surface_id == surface_id) {
                    continue;
                }
                queue.push(PendingMapTileFetch {
                    surface_id: surface_id.to_string(),
                    key: key.to_string(),
                    url: map_tile_url(&scene.vector_tile_url_template, z, x, y),
                    vector: true,
                    z,
                    x,
                    y,
                });
            }
        }
    });
}

pub fn collect_pending_map_tile_fetches() -> Vec<PendingMapTileFetch> {
    PENDING_MAP_TILE_FETCHES.with(|cell| {
        let mut queue = cell.borrow_mut();
        let out = queue.clone();
        queue.clear();
        out
    })
}

pub fn apply_map_tile_bytes(surface_id: &str, fetch: &PendingMapTileFetch, bytes: &[u8]) {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return;
        };
        let Some(host) = entry.map_host.as_mut() else {
            return;
        };
        let result = if fetch.vector {
            host.upload_vector_tile(fetch.z, fetch.x, fetch.y, bytes)
        } else {
            host.upload_tile(fetch.z, fetch.x, fetch.y, bytes)
        };
        if result.is_err() {
            let miss_key = if fetch.vector {
                format!("vector:{}", fetch.key)
            } else {
                format!("raster:{}", fetch.key)
            };
            MAP_TILE_MISS.with(|cell| {
                cell.borrow_mut().insert(miss_key);
            });
        }
    });
}

pub fn paint_gis_map(
    gpu: &mut GpuContext,
    ctx: &mut FrameworkWidgetContext<'_>,
    scene: &UiComponentSceneNode,
    inner: Rect,
) {
    let Some(map_scene) = &scene.gis_map else {
        return;
    };
    let pw = inner.w.max(1.0) as u32;
    let ph = inner.h.max(1.0) as u32;
    let dpr = gpu.dpr() as f64;
    if ensure_surface(gpu, &scene.surface_id, pw, ph).is_err() {
        return;
    }
    let theme_json = map_theme_json_from_ui_theme(ctx.theme);
    let clear = vello_clear(ctx.theme);
    let cavas_scene = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(&scene.surface_id).expect("engine surface");
        if entry.map_host.is_none() {
            entry.map_host = Some(gis_2d::MapHost::new());
            entry.map_sync_cache = MapSyncCache::default();
        }
        let host = entry.map_host.as_mut().expect("map host");
        sync_map_host(host, map_scene, &mut entry.map_sync_cache, pw, ph, dpr, &theme_json);
        queue_map_tile_fetches(&scene.surface_id, map_scene, host);
        host.build_render_scene()
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
        control_id: Some(format!("{}.map", scene.surface_id)),
        kind: HitKind::ScrollRegion,
        drag_axis: Some(ui_wgpu::input::DragAxis::Both),
        drag_data: None,
    });
}

pub fn with_map_host_mut<R>(surface_id: &str, f: impl FnOnce(&mut gis_2d::MapHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        let host = entry.map_host.as_mut()?;
        Some(f(host))
    })
}

pub fn with_map_host<R>(surface_id: &str, f: impl FnOnce(&gis_2d::MapHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        let host = entry.map_host.as_ref()?;
        Some(f(host))
    })
}

pub fn map_action(controller_id: &str, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.to_string(),
        action: action.to_string(),
        args: Some(args),
    }
}

pub fn map_local_pointer(inner: Rect, x: f32, y: f32) -> (f64, f64) {
    ((x - inner.x) as f64, (y - inner.y) as f64)
}

pub fn map_marquee_mode(shift: bool, ctrl_or_meta: bool) -> &'static str {
    if shift && ctrl_or_meta {
        "invertive"
    } else if shift {
        "additive"
    } else if ctrl_or_meta {
        "subtractive"
    } else {
        "default"
    }
}

pub fn map_marquee_crossing(method: &str, start_x: f32, end_x: f32) -> bool {
    if method == "lasso" {
        end_x < start_x
    } else {
        end_x < start_x
    }
}

pub fn map_merge_selection(
    mode: &str,
    current_positions: &[String],
    current_routes: &[String],
    next_positions: &[String],
    next_routes: &[String],
) -> (Vec<String>, Vec<String>) {
    let mut positions: HashSet<String> = current_positions.iter().cloned().collect();
    let mut routes: HashSet<String> = current_routes.iter().cloned().collect();
    let next_pos: HashSet<String> = next_positions.iter().cloned().collect();
    let next_routes: HashSet<String> = next_routes.iter().cloned().collect();
    match mode {
        "additive" => {
            positions.extend(next_pos);
            routes.extend(next_routes);
        }
        "subtractive" => {
            positions.retain(|id| !next_pos.contains(id));
            routes.retain(|id| !next_routes.contains(id));
        }
        "invertive" => {
            for id in next_pos {
                if !positions.insert(id.clone()) {
                    positions.remove(&id);
                }
            }
            for id in next_routes {
                if !routes.insert(id.clone()) {
                    routes.remove(&id);
                }
            }
        }
        _ => {
            positions = next_pos;
            routes = next_routes;
        }
    }
    (
        positions.into_iter().collect(),
        routes.into_iter().collect(),
    )
}

pub fn parse_map_feature_hit(hit_json: &str) -> (Vec<String>, Vec<String>) {
    let hit: Value = serde_json::from_str(hit_json).unwrap_or(Value::Null);
    let positions = hit
        .get("positions")
        .and_then(|value| value.as_array())
        .map(|rows| {
            rows.iter()
                .filter_map(|row| row.as_str().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let routes = hit
        .get("routes")
        .and_then(|value| value.as_array())
        .map(|rows| {
            rows.iter()
                .filter_map(|row| row.as_str().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    (positions, routes)
}

pub fn parse_map_hover(hit_json: &str) -> Value {
    if hit_json == "null" {
        return Value::Null;
    }
    serde_json::from_str(hit_json).unwrap_or(Value::Null)
}

pub fn map_interaction_actions(
    surface_id: &str,
    controller_id: &str,
    host: &gis_2d::MapHost,
) -> Vec<ActionDescriptor> {
  let selection = json!({
      "positions": host.selected_positions_json(),
      "routes": host.selected_routes_json(),
  });
  let hover = if let (Some(kind), Some(id)) = (host.hovered_kind(), host.hovered_id()) {
      json!({ "kind": kind, "id": id })
  } else {
      Value::Null
  };
  vec![
      map_action(
          controller_id,
          ui_wgpu::gis_map_actions::SET_CAMERA,
          json!({ "surfaceId": surface_id, "camera": serde_json::from_str::<Value>(&host.camera_json()).unwrap_or(json!({})) }),
      ),
      map_action(
          controller_id,
          ui_wgpu::gis_map_actions::SET_FEATURE_SELECTION,
          json!({ "surfaceId": surface_id, "positions": selection["positions"], "routes": selection["routes"] }),
      ),
      map_action(
          controller_id,
          ui_wgpu::gis_map_actions::SET_HOVER,
          json!({ "surfaceId": surface_id, "hover": hover }),
      ),
  ]
}

pub fn gis_map_wheel(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    delta: f32,
    ctrl: bool,
) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Vec::new();
        };
        let Some(host) = entry.map_host.as_mut() else {
            return Vec::new();
        };
        let mut delta_y = delta as f64;
        if ctrl {
            delta_y *= 2.5;
        }
        host.wheel_screen(sx, sy, delta_y);
        map_interaction_actions(surface_id, controller_id, host)
    })
}
//#endregion GisMap

//#region Puzzle2dBoard
/// @emoji 🧩 Raw event row drained from {@link puzzle_2d::BoardHost::drain_events_json}; mirrors the TS `BoardEventRow` shape.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BoardEventRow {
    pub name: String,
    #[serde(default)]
    pub payload: Value,
}

pub struct CoalescedBoardEvents {
    pub flush_now: bool,
    pub events_json: String,
}

const PUZZLE2D_TRANSIENT_EVENT_NAMES: &[&str] = &["preselect", "brushPreview", "linkCompatibleNodes", "linkTargetRing"];
const PUZZLE2D_FLUSH_NOW_EVENT_NAMES: &[&str] = &["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"];

/// @emoji 📬 Drops transient rows, coalesces `camera` to its latest value and `nodeMove` to one row per id (unless a `nodeDragEnd` follows), and flags whether the buffer should flush immediately. Port of `coalescePuzzle2dBoardEvents` in the React host.
pub fn coalesce_puzzle2d_board_events(rows: &[BoardEventRow]) -> CoalescedBoardEvents {
    let has_drag_end = rows.iter().any(|row| row.name == "nodeDragEnd");
    let mut flush_now = false;
    let mut last_camera: Option<BoardEventRow> = None;
    let mut node_move_order: Vec<String> = Vec::new();
    let mut node_move_by_id: HashMap<String, BoardEventRow> = HashMap::new();
    let mut rest: Vec<BoardEventRow> = Vec::new();

    for row in rows {
        if PUZZLE2D_TRANSIENT_EVENT_NAMES.contains(&row.name.as_str()) {
            continue;
        }
        if row.name == "camera" {
            last_camera = Some(row.clone());
            continue;
        }
        if row.name == "nodeMove" {
            if has_drag_end {
                continue;
            }
            if let Some(id) = row.payload.get("id").and_then(Value::as_str) {
                if !node_move_by_id.contains_key(id) {
                    node_move_order.push(id.to_string());
                }
                node_move_by_id.insert(id.to_string(), row.clone());
                continue;
            }
        }
        if PUZZLE2D_FLUSH_NOW_EVENT_NAMES.contains(&row.name.as_str()) {
            flush_now = true;
        }
        rest.push(row.clone());
    }

    let mut coalesced: Vec<BoardEventRow> = Vec::new();
    if let Some(camera) = last_camera {
        coalesced.push(camera);
    }
    for id in &node_move_order {
        if let Some(row) = node_move_by_id.get(id) {
            coalesced.push(row.clone());
        }
    }
    coalesced.extend(rest);
    CoalescedBoardEvents {
        flush_now,
        events_json: serde_json::to_string(&coalesced).unwrap_or_else(|_| "[]".into()),
    }
}

fn parse_board_camera(json: &str) -> Option<(f64, f64, f64)> {
    let value: Value = serde_json::from_str(json).ok()?;
    Some((value.get("x")?.as_f64()?, value.get("y")?.as_f64()?, value.get("zoom")?.as_f64()?))
}

fn parse_board_selection_ids(json: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(json).unwrap_or_default()
}

/// @emoji 🔁 Applies scene fields onto `host`, diffing against `cache` so only changed fields re-sync. Mirrors `applyFixtureToSession` plus the independent per-field effects in the React host: reparsing the fixture resets selection/camera, so both are silently re-applied right after. Skips fixture/selection/camera sync entirely while `host` defers descriptor sync (mid-gesture), matching `pendingFixtureSceneRef`.
fn sync_board_host(host: &mut puzzle_2d::BoardHost, scene: &ui_wgpu::Puzzle2dBoardScene, cache: &mut BoardSyncCache, pw: u32, ph: u32, dpr: f64) {
    let size_key = format!("{pw}x{ph}@{dpr}");
    if sync_field(&mut cache.size_key, &size_key) {
        host.set_size(pw, ph, dpr);
    }
    let deferred = host.defers_descriptor_sync_from_js();
    if !deferred && sync_field(&mut cache.fixture_json, &scene.fixture_json) {
        if let Ok(raw) = serde_json::from_str::<Value>(&scene.fixture_json) {
            host.parse_fixture_v1(&raw);
        }
        host.set_selection_options(&scene.selection_method, "replace", true, true, true);
        host.set_selection_ids_silent(&parse_board_selection_ids(&scene.selection_json));
        cache.selection_json = Some(scene.selection_json.clone());
        if let Some((x, y, zoom)) = parse_board_camera(&scene.camera_json) {
            host.set_camera_silent(x, y, zoom);
        }
        cache.camera_json = Some(scene.camera_json.clone());
    }
    if sync_field(&mut cache.kind_catalogs_json, &scene.kind_catalogs_json) {
        let _ = host.set_board_kind_catalogs_from_json(&scene.kind_catalogs_json);
    }
    if sync_field(&mut cache.kind_compatibility_json, &scene.kind_compatibility_json) {
        let _ = host.set_handle_link_compat_from_json(&scene.kind_compatibility_json);
    }
    if !deferred && sync_field(&mut cache.selection_json, &scene.selection_json) {
        host.set_selection_ids_silent(&parse_board_selection_ids(&scene.selection_json));
    }
    if !deferred && sync_field(&mut cache.camera_json, &scene.camera_json) {
        if let Some((x, y, zoom)) = parse_board_camera(&scene.camera_json) {
            host.set_camera_silent(x, y, zoom);
        }
    }
    if cache.hovered_id != scene.hovered_id {
        cache.hovered_id = scene.hovered_id.clone();
        host.set_hovered_id_silent(scene.hovered_id.clone());
    }
    let active_tool = scene.active_tool.as_deref().unwrap_or("select");
    if cache.active_tool.as_deref() != Some(active_tool) {
        cache.active_tool = Some(active_tool.to_string());
        host.set_active_tool(active_tool);
    }
    if sync_field(&mut cache.selection_method, &scene.selection_method) {
        host.set_selection_options(&scene.selection_method, "replace", true, true, true);
    }
    if cache.grid_snap_enabled != Some(scene.grid_snap_enabled) {
        cache.grid_snap_enabled = Some(scene.grid_snap_enabled);
        host.set_grid_snap_enabled(scene.grid_snap_enabled);
    }
    if cache.grid_factor != Some(scene.grid_factor) {
        cache.grid_factor = Some(scene.grid_factor);
        let _ = host.set_grid_factor(scene.grid_factor);
    }
    if scene.suggestion_offset > 0.0 && cache.suggestion_offset != Some(scene.suggestion_offset) {
        cache.suggestion_offset = Some(scene.suggestion_offset);
        host.set_suggestion_offset(scene.suggestion_offset);
    }
    if sync_field(&mut cache.brush_kind_weights_json, &scene.brush_kind_weights_json) {
        host.set_brush_kind_weights(&scene.brush_kind_weights_json);
    }
    if sync_field(&mut cache.lod_mode, &scene.lod_mode) {
        if scene.lod_mode == "automatic" {
            host.set_automatic_lod(true);
        } else {
            host.set_automatic_lod(false);
            host.set_forced_draw_lod_label(&scene.lod_mode);
        }
    }
}

pub fn paint_puzzle_board(gpu: &mut GpuContext, ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, inner: Rect) {
    let Some(board_scene) = &scene.puzzle2d_board else {
        return;
    };
    let pw = inner.w.max(1.0) as u32;
    let ph = inner.h.max(1.0) as u32;
    let dpr = gpu.dpr() as f64;
    if ensure_surface(gpu, &scene.surface_id, pw, ph).is_err() {
        return;
    }
    let clear = vello_clear(ctx.theme);
    let cavas_scene = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(&scene.surface_id).expect("engine surface");
        if entry.board_host.is_none() {
            entry.board_host = Some(puzzle_2d::puzzle_board_host());
            entry.board_sync_cache = BoardSyncCache::default();
        }
        let host = entry.board_host.as_mut().expect("board host");
        sync_board_host(host, board_scene, &mut entry.board_sync_cache, pw, ph, dpr);
        host.build_vector_scene()
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
    if board_scene.interactive {
        ctx.input.register_hit(HitTarget {
            rect: inner,
            event: None,
            control_id: Some(format!("{}.board", scene.surface_id)),
            kind: HitKind::ScrollRegion,
            drag_axis: Some(ui_wgpu::input::DragAxis::Both),
            drag_data: None,
        });
    }
}

pub fn with_board_host_mut<R>(surface_id: &str, f: impl FnOnce(&mut puzzle_2d::BoardHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        let host = entry.board_host.as_mut()?;
        Some(f(host))
    })
}

pub fn with_board_host<R>(surface_id: &str, f: impl FnOnce(&puzzle_2d::BoardHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        let host = entry.board_host.as_ref()?;
        Some(f(host))
    })
}

pub fn board_action(controller_id: &str, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.to_string(),
        action: action.to_string(),
        args: Some(args),
    }
}

/// @emoji 🎯 Most-specific pick target at a screen point, mirroring `pickMostSpecificCanvasTarget`.
pub fn board_pick_best_target_id(surface_id: &str, sx: f64, sy: f64) -> Option<String> {
    with_board_host(surface_id, |host| {
        let json = host.pick_targets_at_screen_json(sx, sy);
        let targets: Vec<Value> = serde_json::from_str(&json).unwrap_or_default();
        targets
            .into_iter()
            .max_by_key(|t| t.get("generality").and_then(Value::as_u64).unwrap_or(0))
            .and_then(|t| t.get("id").and_then(|v| v.as_str()).map(str::to_string))
    })
    .flatten()
}

fn board_drain_into_buffer(surface_id: &str) {
    let rows = with_board_host_mut(surface_id, |host| {
        let json = host.drain_events_json();
        serde_json::from_str::<Vec<BoardEventRow>>(&json).unwrap_or_default()
    })
    .unwrap_or_default();
    if rows.is_empty() {
        return;
    }
    ENGINE_SURFACES.with(|cell| {
        if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
            entry.board_pending_events.extend(rows);
        }
    });
}

fn board_take_buffer_coalesced(surface_id: &str) -> Option<String> {
    let rows = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        map.get_mut(surface_id).map(|entry| std::mem::take(&mut entry.board_pending_events))
    })?;
    if rows.is_empty() {
        return None;
    }
    let coalesced = coalesce_puzzle2d_board_events(&rows);
    if coalesced.events_json == "[]" {
        None
    } else {
        Some(coalesced.events_json)
    }
}

/// @emoji 📤 Unconditional drain + coalesce + dispatch, mirroring `flushBoardEvents` (used after pointer-up, pointer-leave, and wheel).
fn board_flush_events_action(surface_id: &str, controller_id: &str) -> Option<ActionDescriptor> {
    board_drain_into_buffer(surface_id);
    let events_json = board_take_buffer_coalesced(surface_id)?;
    Some(board_action(controller_id, "applyBoardEvents", json!({ "eventsJson": events_json })))
}

/// @emoji 📤 Drains into the buffer and only dispatches if a flush-now event (select, brushPlace, edgeCreate, ...) is pending, mirroring `drainAndMaybeFlush` (used on pointer-move).
fn board_drain_and_maybe_flush(surface_id: &str, controller_id: &str) -> Vec<ActionDescriptor> {
    board_drain_into_buffer(surface_id);
    let flush_now = ENGINE_SURFACES.with(|cell| {
        cell.borrow()
            .get(surface_id)
            .map(|entry| coalesce_puzzle2d_board_events(&entry.board_pending_events).flush_now)
            .unwrap_or(false)
    });
    if !flush_now {
        return Vec::new();
    }
    match board_take_buffer_coalesced(surface_id) {
        Some(events_json) => vec![board_action(controller_id, "applyBoardEvents", json!({ "eventsJson": events_json }))],
        None => Vec::new(),
    }
}

fn board_camera_action(surface_id: &str, controller_id: &str) -> Option<ActionDescriptor> {
    with_board_host(surface_id, |host| {
        board_action(controller_id, "setCamera", json!({ "camera": { "x": host.camera.x, "y": host.camera.y, "zoom": host.camera.zoom } }))
    })
}

fn board_set_pointer_inside(surface_id: &str, inside: bool) {
    ENGINE_SURFACES.with(|cell| {
        if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
            entry.board_pointer_inside = inside;
        }
    });
}

pub fn puzzle_board_pointer_down(surface_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool) {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, button.max(0) as u8, shift, ctrl_or_meta));
    board_set_pointer_inside(surface_id, true);
}

pub fn puzzle_board_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt));
    board_set_pointer_inside(surface_id, true);
    board_drain_and_maybe_flush(surface_id, controller_id)
}

pub fn puzzle_board_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt));
    board_flush_events_action(surface_id, controller_id).into_iter().collect()
}

pub fn puzzle_board_pointer_leave(surface_id: &str, controller_id: &str, alt: bool) -> Vec<ActionDescriptor> {
    let was_inside = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return false;
        };
        let was = entry.board_pointer_inside;
        entry.board_pointer_inside = false;
        was
    });
    if !was_inside {
        return Vec::new();
    }
    with_board_host_mut(surface_id, |host| host.pointer_leave_screen(alt));
    board_flush_events_action(surface_id, controller_id).into_iter().collect()
}

/// @emoji 🖐️ True while a node drag or area-select gesture is in flight, so pointer-up outside the surface bounds still reaches the host (mirrors `gis_map_drag_active`).
pub fn board_drag_active(surface_id: &str) -> bool {
    with_board_host(surface_id, |host| host.defers_descriptor_sync_from_js() || host.is_dragging_area_select()).unwrap_or(false)
}

pub fn puzzle_board_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.wheel_screen(sx, sy, delta as f64));
    let mut actions = Vec::new();
    if let Some(camera_action) = board_camera_action(surface_id, controller_id) {
        actions.push(camera_action);
    }
    if let Some(events_action) = board_flush_events_action(surface_id, controller_id) {
        actions.push(events_action);
    }
    actions
}
//#endregion Puzzle2dBoard

#[cfg(test)]
mod puzzle2d_board_engine_tests {
    use super::*;

    fn row(name: &str, payload: Value) -> BoardEventRow {
        BoardEventRow { name: name.to_string(), payload }
    }

    #[test]
    fn coalesce_drops_transient_events() {
        let rows = vec![row("preselect", json!({})), row("brushPreview", json!({})), row("select", json!({ "ids": ["a"] }))];
        let result = coalesce_puzzle2d_board_events(&rows);
        assert!(result.flush_now);
        let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["name"], "select");
    }

    #[test]
    fn coalesce_keeps_only_latest_camera() {
        let rows = vec![row("camera", json!({ "x": 1 })), row("camera", json!({ "x": 2 }))];
        let result = coalesce_puzzle2d_board_events(&rows);
        assert!(!result.flush_now);
        let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["payload"]["x"], 2);
    }

    #[test]
    fn coalesce_collapses_node_move_to_one_row_per_id_preserving_order() {
        let rows = vec![
            row("nodeMove", json!({ "id": "a", "x": 1 })),
            row("nodeMove", json!({ "id": "b", "x": 2 })),
            row("nodeMove", json!({ "id": "a", "x": 3 })),
        ];
        let result = coalesce_puzzle2d_board_events(&rows);
        let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0]["payload"]["id"], "a");
        assert_eq!(parsed[0]["payload"]["x"], 3);
        assert_eq!(parsed[1]["payload"]["id"], "b");
    }

    #[test]
    fn coalesce_drops_node_move_entirely_when_drag_end_follows() {
        let rows = vec![row("nodeMove", json!({ "id": "a", "x": 1 })), row("nodeDragEnd", json!({ "moves": [] }))];
        let result = coalesce_puzzle2d_board_events(&rows);
        let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["name"], "nodeDragEnd");
    }

    #[test]
    fn coalesce_flags_flush_now_for_edge_and_brush_events() {
        for name in ["preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"] {
            let result = coalesce_puzzle2d_board_events(&[row(name, json!({}))]);
            assert!(result.flush_now, "{name} should flush immediately");
        }
    }

    #[test]
    fn coalesce_empty_input_produces_empty_array_and_no_flush() {
        let result = coalesce_puzzle2d_board_events(&[]);
        assert!(!result.flush_now);
        assert_eq!(result.events_json, "[]");
    }
}

//#region TextEditor
pub fn text_editor_apply_key(
    scene: &UiComponentSceneNode,
    key: ui_wgpu::KeyAction,
    modifiers: &ui_wgpu::PointerModifiers,
) -> Vec<ActionDescriptor> {
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
        text_editor_interaction_actions(scene, host)
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

pub fn text_editor_wheel(scene: &UiComponentSceneNode, delta: f32) -> Vec<ActionDescriptor> {
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
) -> Vec<ActionDescriptor> {
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
        text_editor_interaction_actions(scene, host)
    })
}

pub fn text_editor_pointer_move(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
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
        text_editor_interaction_actions(scene, host)
    })
}

pub fn text_editor_pointer_up(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
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
        text_editor_interaction_actions(scene, host)
    })
}

fn text_editor_interaction_actions(
    scene: &UiComponentSceneNode,
    host: &EditorHost,
) -> Vec<ActionDescriptor> {
    vec![
        scene_action(
            scene,
            "textSelect",
            json!({
                "surfaceId": scene.surface_id,
                "selectionJson": json!({ "start": host.anchor(), "end": host.caret() }).to_string(),
            }),
        ),
        scene_action(
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

use crate::scenes::{queue_canvas_image_upload, render_component_scene, GisMapSurface, NodeGraphSurface, Puzzle2dBoardSurface};
use ui_wgpu::{ActionDescriptor, UiComponentSceneNode, UiControlNode, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeSectionNode};
use serde_json::Value;
use ui_wgpu::{
    draw_text, gap_for_token, layout_horizontal, layout_vertical, padding_for_token, ControlNode, KeyValueEntry, Rect,
    SelectItem, Theme, TreeItem, TreeItemAction, TreeSection, WidgetContext, WidgetInteractionMaps, WidgetNode,
    measure_widget, render_widget,
};

pub type FrameworkWidgetContext<'a> = WidgetContext<'a, ActionDescriptor>;

//#region RenderPlanValidator
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderPlanLimits {
    pub max_tree_depth: usize,
    pub max_node_count: usize,
    pub max_json_payload_bytes: usize,
    pub max_texture_dimension: u32,
    pub max_mesh_count: usize,
}

impl Default for RenderPlanLimits {
    fn default() -> Self {
        Self {
            max_tree_depth: 64,
            max_node_count: 4096,
            max_json_payload_bytes: 4 * 1024 * 1024,
            max_texture_dimension: 8192,
            max_mesh_count: 2048,
        }
    }
}

pub const RENDER_PLAN_LIMITS: RenderPlanLimits = RenderPlanLimits {
    max_tree_depth: 64,
    max_node_count: 4096,
    max_json_payload_bytes: 4 * 1024 * 1024,
    max_texture_dimension: 8192,
    max_mesh_count: 2048,
};

fn check_json_payload(label: &str, payload: &str, limits: &RenderPlanLimits) -> Result<(), String> {
    if payload.len() > limits.max_json_payload_bytes {
        return Err(format!(
            "render plan limit exceeded: {label} has {} bytes (max {})",
            payload.len(),
            limits.max_json_payload_bytes
        ));
    }
    Ok(())
}

fn check_optional_json_payload(
    label: &str,
    payload: &Option<String>,
    limits: &RenderPlanLimits,
) -> Result<(), String> {
    if let Some(value) = payload {
        check_json_payload(label, value, limits)?;
    }
    Ok(())
}

pub fn validate_component_scene(scene: &UiComponentSceneNode, limits: &RenderPlanLimits) -> Result<(), String> {
    let scene_label = format!("component scene '{}'", scene.surface_id);
    if let Some(canvas) = &scene.canvas_2d {
        check_json_payload(&format!("{scene_label} canvas2d.layers"), &canvas.layers_json, limits)?;
    }
    if let Some(world) = &scene.world_3d {
        check_json_payload(&format!("{scene_label} world3d.camera"), &world.camera_json, limits)?;
        check_json_payload(&format!("{scene_label} world3d.meshes"), &world.meshes_json, limits)?;
        let mesh_count = serde_json::from_str::<Value>(&world.meshes_json)
            .ok()
            .and_then(|value| value.as_array().map(|array| array.len()))
            .unwrap_or(0);
        if mesh_count > limits.max_mesh_count {
            return Err(format!(
                "render plan limit exceeded: {scene_label} world3d mesh count {mesh_count} exceeds max {}",
                limits.max_mesh_count
            ));
        }
        check_json_payload(&format!("{scene_label} world3d.instances"), &world.instances_json, limits)?;
        check_json_payload(&format!("{scene_label} world3d.selection"), &world.selection_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.vortices"), &world.vortices_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.attractions"), &world.attractions_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} world3d.targetVolumes"),
            &world.target_volumes_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} world3d.references"), &world.references_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} world3d.brushPreview"),
            &world.brush_preview_json,
            limits,
        )?;
        check_optional_json_payload(
            &format!("{scene_label} world3d.interaction"),
            &world.interaction_json,
            limits,
        )?;
        check_optional_json_payload(
            &format!("{scene_label} world3d.engagementPreview"),
            &world.engagement_preview_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} world3d.lod"), &world.lod_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.chunking"), &world.chunking_json, limits)?;
    }
    if let Some(graph) = &scene.node_graph {
        check_json_payload(&format!("{scene_label} nodeGraph.nodes"), &graph.nodes_json, limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.edges"), &graph.edges_json, limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.viewport"), &graph.viewport_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.operators"), &graph.operators_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} nodeGraph.contextMenu"),
            &graph.context_menu_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.findItems"), &graph.find_items_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.selection"), &graph.selection_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.hover"), &graph.hover_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} nodeGraph.previewOff"),
            &graph.preview_off_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.lod"), &graph.lod_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.catalogue"), &graph.catalogue_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.controls"), &graph.controls_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.clusters"), &graph.clusters_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.computing"), &graph.computing_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} nodeGraph.capabilities"),
            &graph.capabilities_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.fixture"), &graph.fixture_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} nodeGraph.presencePeers"),
            &graph.presence_peers_json,
            limits,
        )?;
    }
    if let Some(editor) = &scene.text_editor {
        check_json_payload(&format!("{scene_label} textEditor.buffer"), &editor.buffer, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} textEditor.selection"),
            &editor.selection_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} textEditor.tokens"), &editor.tokens_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} textEditor.diagnostics"),
            &editor.diagnostics_json,
            limits,
        )?;
        check_optional_json_payload(
            &format!("{scene_label} textEditor.completions"),
            &editor.completions_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} textEditor.overlays"), &editor.overlays_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} textEditor.occurrences"),
            &editor.occurrences_json,
            limits,
        )?;
        check_optional_json_payload(
            &format!("{scene_label} textEditor.placeholders"),
            &editor.placeholders_json,
            limits,
        )?;
        check_optional_json_payload(
            &format!("{scene_label} textEditor.extraCarets"),
            &editor.extra_carets_json,
            limits,
        )?;
        check_optional_json_payload(
            &format!("{scene_label} textEditor.selectableSpans"),
            &editor.selectable_spans_json,
            limits,
        )?;
        check_optional_json_payload(&format!("{scene_label} textEditor.settings"), &editor.settings_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.camera"), &editor.camera_json, limits)?;
    }
    if let Some(table) = &scene.table {
        check_json_payload(&format!("{scene_label} table.columns"), &table.columns_json, limits)?;
        check_json_payload(&format!("{scene_label} table.rows"), &table.rows_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} table.selection"), &table.selection_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} table.sort"), &table.sort_json, limits)?;
    }
    if let Some(raster) = &scene.raster {
        check_json_payload(&format!("{scene_label} raster.documentSync"), &raster.document_sync_json, limits)?;
        check_json_payload(&format!("{scene_label} raster.assets"), &raster.assets_json, limits)?;
        check_json_payload(&format!("{scene_label} raster.camera"), &raster.camera_json, limits)?;
        check_json_payload(&format!("{scene_label} raster.selection"), &raster.selection_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} raster.compositeViewport"),
            &raster.composite_viewport_json,
            limits,
        )?;
    }
    if let Some(vfs) = &scene.virtual_file_system {
        check_json_payload(&format!("{scene_label} vfs.schema"), &vfs.schema_json, limits)?;
        check_json_payload(&format!("{scene_label} vfs.rows"), &vfs.rows_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} vfs.selectedRowIds"),
            &vfs.selected_row_ids_json,
            limits,
        )?;
    }
    if let Some(map) = &scene.gis_map {
        check_json_payload(&format!("{scene_label} gisMap.fixture"), &map.map_fixture_json, limits)?;
        check_json_payload(&format!("{scene_label} gisMap.camera"), &map.camera_json, limits)?;
        check_json_payload(
            &format!("{scene_label} gisMap.layerVisibility"),
            &map.layer_visibility_json,
            limits,
        )?;
        check_json_payload(
            &format!("{scene_label} gisMap.layerStrokeScale"),
            &map.layer_stroke_scale_json,
            limits,
        )?;
        check_json_payload(&format!("{scene_label} gisMap.selection"), &map.selection_json, limits)?;
        check_json_payload(&format!("{scene_label} gisMap.hover"), &map.hover_json, limits)?;
        check_optional_json_payload(
            &format!("{scene_label} gisMap.contextMenu"),
            &map.context_menu_json,
            limits,
        )?;
    }
    if let Some(board) = &scene.puzzle2d_board {
        check_json_payload(&format!("{scene_label} puzzle2dBoard.fixture"), &board.fixture_json, limits)?;
        check_json_payload(&format!("{scene_label} puzzle2dBoard.camera"), &board.camera_json, limits)?;
        check_json_payload(
            &format!("{scene_label} puzzle2dBoard.kindCatalogs"),
            &board.kind_catalogs_json,
            limits,
        )?;
        check_json_payload(
            &format!("{scene_label} puzzle2dBoard.selection"),
            &board.selection_json,
            limits,
        )?;
        check_json_payload(
            &format!("{scene_label} puzzle2dBoard.brushKindWeights"),
            &board.brush_kind_weights_json,
            limits,
        )?;
        check_json_payload(
            &format!("{scene_label} puzzle2dBoard.kindCompatibility"),
            &board.kind_compatibility_json,
            limits,
        )?;
    }
    Ok(())
}

struct RenderPlanWalkState {
    node_count: usize,
}

fn walk_ui_node(
    node: &UiNode,
    depth: usize,
    limits: &RenderPlanLimits,
    state: &mut RenderPlanWalkState,
) -> Result<(), String> {
    state.node_count += 1;
    if state.node_count > limits.max_node_count {
        return Err(format!(
            "render plan limit exceeded: node count {} exceeds max {}",
            state.node_count, limits.max_node_count
        ));
    }
    if depth > limits.max_tree_depth {
        return Err(format!(
            "render plan limit exceeded: tree depth {depth} exceeds max {}",
            limits.max_tree_depth
        ));
    }
    match node {
        UiNode::ComponentScene(scene) => validate_component_scene(scene, limits)?,
        UiNode::Stack(stack) => {
            for child in &stack.children {
                walk_ui_node(child, depth + 1, limits, state)?;
            }
        }
        UiNode::Section(section) => {
            for child in &section.children {
                walk_ui_node(child, depth + 1, limits, state)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn validate_ui_node(node: &UiNode, limits: &RenderPlanLimits) -> Result<(), String> {
    let mut state = RenderPlanWalkState { node_count: 0 };
    walk_ui_node(node, 1, limits, &mut state)
}

pub fn validate_window_body_surface(
    kind: &semio_framework_core::WindowKindDefinition,
    node: &UiNode,
) -> Result<(), String> {
    match node {
        UiNode::ComponentScene(scene) if scene.component_kind != kind.surface_kind => Err(format!(
            "window {} declared {} but plugin returned {}",
            kind.id,
            kind.surface_kind.as_str(),
            scene.component_kind.as_str()
        )),
        _ => Ok(()),
    }
}

fn render_plan_error_widget(message: &str, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    render_widget(
        &WidgetNode::Text {
            value: format!("Render plan rejected: {message}"),
            emphasize: true,
        },
        bounds,
        ctx,
    );
}
//#endregion RenderPlanValidator

pub fn measure_ui_node(atlas: &mut ui_wgpu::FontAtlas, theme: &Theme, node: &UiNode) -> (f32, f32) {
    match node {
        UiNode::ComponentScene(_) => (320.0, 240.0),
        UiNode::Image(_) => (128.0, 128.0),
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
    gis_map_states: &mut std::collections::HashMap<String, GisMapSurface>,
    icon_render_states: &mut std::collections::HashMap<String, infinite_world::World3dState>,
    puzzle2d_board_states: &mut std::collections::HashMap<String, Puzzle2dBoardSurface>,
) {
    if let Err(message) = validate_ui_node(node, &RENDER_PLAN_LIMITS) {
        return render_plan_error_widget(&message, bounds, ctx);
    }
    render_ui_node_inner(node, bounds, ctx, gpu, world3d_states, node_graph_states, gis_map_states, icon_render_states, puzzle2d_board_states);
}

fn render_ui_node_inner(
    node: &UiNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    world3d_states: &mut std::collections::HashMap<String, infinite_world::World3dState>,
    node_graph_states: &mut std::collections::HashMap<String, NodeGraphSurface>,
    gis_map_states: &mut std::collections::HashMap<String, GisMapSurface>,
    icon_render_states: &mut std::collections::HashMap<String, infinite_world::World3dState>,
    puzzle2d_board_states: &mut std::collections::HashMap<String, Puzzle2dBoardSurface>,
) {
    match node {
        UiNode::ComponentScene(scene) => render_component_scene(
            scene,
            bounds,
            ctx,
            gpu,
            world3d_states,
            node_graph_states,
            gis_map_states,
            icon_render_states,
            puzzle2d_board_states,
        ),
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
                render_ui_node_inner(child, *rect, ctx, gpu, world3d_states, node_graph_states, gis_map_states, icon_render_states, puzzle2d_board_states);
            }
        }
        UiNode::Section(section) => {
            let gap = gap_for_token(ctx.theme, None);
            let padding = padding_for_token(ctx.theme, None);
            let sizes: Vec<f32> = section
                .children
                .iter()
                .map(|child| measure_ui_node(ctx.atlas, ctx.theme, child).1)
                .collect();
            let rects = layout_vertical(bounds, gap, padding, &sizes);
            for (child, rect) in section.children.iter().zip(rects.iter()) {
                render_ui_node_inner(child, *rect, ctx, gpu, world3d_states, node_graph_states, gis_map_states, icon_render_states, puzzle2d_board_states);
            }
        }
        UiNode::Image(image) => render_ui_image(image, bounds, ctx),
        other => render_widget(&ui_node_to_widget(other), bounds, ctx),
    }
}

fn render_ui_image(image: &ui_wgpu::UiImageNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let Some(key) = queue_canvas_image_upload("ui-image", &image.id, &image.src) else {
        if let Some(alt) = &image.alt {
            draw_text(ctx, alt, bounds.x + 4.0, bounds.y + 16.0, ctx.theme.font_size_small, ctx.theme.text_muted);
        }
        return;
    };
    ctx.draw
        .push_raster_quad(&key, [bounds.x, bounds.y, bounds.w, bounds.h], [0.0, 0.0, 1.0, 1.0], 1.0);
}

pub fn ui_node_to_widget(node: &UiNode) -> WidgetNode<ActionDescriptor> {
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
            event: Some(button.action.clone()),
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
        UiNode::Field(field) => match ui_wgpu::ui_node_to_control(&field.child) {
            Some(control) => WidgetNode::Field {
                id: field.id.clone(),
                label: field.label.clone(),
                child: control_to_widget(&control),
            },
            None => WidgetNode::Section {
                id: field.id.clone(),
                label: Some(field.label.clone()),
                default_open: true,
                children: vec![ui_node_to_widget(&field.child)],
            },
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
        UiNode::ExternalSlot(slot) => WidgetNode::Text {
            value: format!("Extension: {} / {}", slot.plugin_id, slot.body_key),
            emphasize: false,
        },
        UiNode::Image(_) => WidgetNode::Text {
            value: String::new(),
            emphasize: false,
        },
    }
}

fn control_to_widget(control: &UiControlNode) -> ControlNode<ActionDescriptor> {
    match control {
        UiControlNode::Button(n) => ControlNode::Button {
            id: n.id.clone(),
            icon_id: Some(n.icon_id.clone()),
            label: n.label.clone(),
            event: Some(n.action.clone()),
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

fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<ActionDescriptor> {
    TreeItemAction {
        icon_id: action.icon_id.clone(),
        label: action.label.clone(),
        event: action.action.clone(),
        reveal_on_hover: action.reveal_on_hover.unwrap_or(false),
    }
}

fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<ActionDescriptor> {
    TreeSection {
        id: section.id.clone(),
        label: section.label.clone(),
        default_open: section.default_open.unwrap_or(true),
        items: section.items.iter().map(tree_item_to_widget).collect(),
    }
}

fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<ActionDescriptor> {
    TreeItem {
        id: item.id.clone(),
        label: item.label.clone(),
        description: item.description.clone(),
        icon_id: item.icon_id.clone(),
        selected: item.selected.unwrap_or(false),
        highlighted: false,
        default_open: item.default_open.unwrap_or(false),
        is_hidden: item.is_hidden.unwrap_or(false),
        event: item.action.clone(),
        hover_event: item.hover_action.clone(),
        unhover_event: item.unhover_action.clone(),
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

fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<ActionDescriptor> {
    match control {
        UiControlNode::Button(n) => WidgetNode::Button {
            id: n.id.clone(),
            icon_id: Some(n.icon_id.clone()),
            label: n.label.clone(),
            event: Some(n.action.clone()),
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
    input: &'a mut ui_wgpu::InputState<ActionDescriptor>,
    theme: &'a Theme,
    scroll_offsets: &'a mut std::collections::HashMap<String, f32>,
    collapsed_sections: &'a mut std::collections::HashMap<String, bool>,
    open_selects: &'a mut std::collections::HashMap<String, bool>,
    interaction_maps: Option<&'a mut WidgetInteractionMaps<ActionDescriptor>>,
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
        pick_clip: None,
    }
}

//#region RenderPlanValidatorTests
#[cfg(test)]
mod render_plan_validator_tests {
    use super::*;
    use ui_wgpu::{build_table_scene, build_world_3d_scene, TableScene, UiStackNode, World3dScene};

    #[test]
    fn validate_ui_node_rejects_oversized_json_payload() {
        let limits = RenderPlanLimits {
            max_json_payload_bytes: 16,
            ..RenderPlanLimits::default()
        };
        let node = build_table_scene(
            "table",
            "controller",
            TableScene::base("[]", "x".repeat(32)),
        );
        let error = validate_ui_node(&node, &limits).expect_err("oversized payload should be rejected");
        assert!(error.contains("table.rows"));
        assert!(error.contains("32 bytes"));
    }

    fn empty_stack(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: None,
            selected: None,
            activate: None,
            drop_action: None,
            children,
            loading: None,
        })
    }

    #[test]
    fn validate_component_scene_rejects_oversized_mesh_count() {
        let limits = RenderPlanLimits {
            max_mesh_count: 2,
            ..RenderPlanLimits::default()
        };
        let meshes_json = serde_json::to_string(&vec![serde_json::json!({"id": "m"}); 3]).unwrap();
        let node = build_world_3d_scene(
            "world",
            "controller",
            World3dScene {
                camera_json: "{}".into(),
                meshes_json,
                instances_json: "[]".into(),
                selection_json: "{}".into(),
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
            },
        );
        let error = validate_ui_node(&node, &limits).expect_err("oversized mesh count should be rejected");
        assert!(error.contains("mesh count 3 exceeds max 2"));
    }

    #[test]
    fn validate_ui_node_rejects_oversized_node_count() {
        let limits = RenderPlanLimits {
            max_node_count: 3,
            ..RenderPlanLimits::default()
        };
        let tree = empty_stack(vec![empty_stack(vec![]), empty_stack(vec![]), empty_stack(vec![])]);
        let error = validate_ui_node(&tree, &limits).expect_err("oversized node count should be rejected");
        assert!(error.contains("node count 4 exceeds max 3"));
    }

    #[test]
    fn validate_ui_node_rejects_oversized_tree_depth() {
        let limits = RenderPlanLimits {
            max_tree_depth: 2,
            ..RenderPlanLimits::default()
        };
        let mut tree = empty_stack(vec![]);
        for _ in 0..4 {
            tree = empty_stack(vec![tree]);
        }
        let error = validate_ui_node(&tree, &limits).expect_err("oversized tree depth should be rejected");
        assert!(error.contains("tree depth"));
        assert!(error.contains("exceeds max 2"));
    }
}
//#endregion RenderPlanValidatorTests
// #endregion interpreter
}

pub mod plugin_bridge {
// #region plugin_bridge
//! 🔌 Plugin bridge for wasm C-ABI modules (browser JS loader + wasmtime host).

use semio_framework_core::{PluginManifest, ViewState};
use ui_wgpu::{ToolNode, UiNode, WindowEngagement, WindowMeasure};
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::WasmPluginRuntime;

enum PluginBridgeBackend {
    #[cfg(target_arch = "wasm32")]
    Js(Rc<JsValue>),
    #[cfg(not(target_arch = "wasm32"))]
    Wasm(Arc<WasmPluginRuntime>),
}

impl Clone for PluginBridgeBackend {
    fn clone(&self) -> Self {
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Js(handle) => Self::Js(handle.clone()),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Wasm(runtime) => Self::Wasm(runtime.clone()),
        }
    }
}

#[derive(Clone)]
pub struct PluginBridgeEntry {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    backend: PluginBridgeBackend,
}

impl PluginBridgeEntry {
    #[cfg(target_arch = "wasm32")]
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
            backend: PluginBridgeBackend::Js(Rc::new(handle)),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_wasm(plugin_id: String, runtime: Arc<WasmPluginRuntime>) -> Result<Self, String> {
        Ok(Self {
            plugin_id,
            manifest: runtime.manifest.clone(),
            backend: PluginBridgeBackend::Wasm(runtime),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn wasm_runtime(&self) -> Option<Arc<WasmPluginRuntime>> {
        match &self.backend {
            PluginBridgeBackend::Wasm(runtime) => Some(runtime.clone()),
            #[cfg(target_arch = "wasm32")]
            _ => None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn wasm_artifact_path(&self) -> Option<&std::path::Path> {
        match &self.backend {
            PluginBridgeBackend::Wasm(runtime) => Some(runtime.path.as_path()),
            #[cfg(target_arch = "wasm32")]
            _ => None,
        }
    }

    pub async fn create_app(&self, app_id: &str) -> Result<u32, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            PluginBridgeBackend::Js(handle) => create_app_js(handle, app_id).await,
            #[cfg(not(target_arch = "wasm32"))]
            PluginBridgeBackend::Wasm(runtime) => runtime.create_app(app_id),
        }
    }

    pub fn destroy_app(&self, instance_id: u32) {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            PluginBridgeBackend::Js(handle) => destroy_app_js(handle, instance_id),
            #[cfg(not(target_arch = "wasm32"))]
            PluginBridgeBackend::Wasm(runtime) => runtime.destroy_app(instance_id),
        }
    }

    pub async fn handle_action(
        &self,
        instance_id: u32,
        action_json: &str,
        view_state: &ViewState,
    ) -> Result<semio_framework_core::kernel::ActionResult, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            PluginBridgeBackend::Js(handle) => handle_action_js(handle, instance_id, action_json, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            PluginBridgeBackend::Wasm(runtime) => runtime.handle_action(instance_id, action_json, view_state),
        }
    }

    pub async fn render(
        &self,
        instance_id: u32,
        body_key: &str,
        view_state: &ViewState,
    ) -> Result<UiNode, String> {
        self.render_with_document(instance_id, body_key, view_state, None)
            .await
    }

    pub async fn render_with_document(
        &self,
        instance_id: u32,
        body_key: &str,
        view_state: &ViewState,
        document_json: Option<&str>,
    ) -> Result<UiNode, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            PluginBridgeBackend::Js(handle) => {
                render_with_document_js(handle, instance_id, body_key, view_state, document_json).await
            }
            #[cfg(not(target_arch = "wasm32"))]
            PluginBridgeBackend::Wasm(runtime) => {
                runtime.render_with_document(instance_id, body_key, view_state, document_json)
            }
        }
    }

    pub async fn window_engagements(
        &self,
        instance_id: u32,
        view_state: &ViewState,
    ) -> Result<HashMap<String, WindowEngagement>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            PluginBridgeBackend::Js(handle) => window_engagements_js(handle, instance_id, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            PluginBridgeBackend::Wasm(runtime) => runtime.window_engagements(instance_id, view_state),
        }
    }

    pub async fn window_measures(
        &self,
        instance_id: u32,
        view_state: &ViewState,
    ) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            PluginBridgeBackend::Js(handle) => window_measures_js(handle, instance_id, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            PluginBridgeBackend::Wasm(runtime) => runtime.window_measures(instance_id, view_state),
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn create_app_js(handle: &Rc<JsValue>, app_id: &str) -> Result<u32, String> {
    let create_app = get_fn(handle.as_ref(), "createApp")?;
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

#[cfg(target_arch = "wasm32")]
fn destroy_app_js(handle: &Rc<JsValue>, instance_id: u32) {
    if let Ok(destroy) = Reflect::get(handle.as_ref(), &JsValue::from_str("destroyApp"))
        .and_then(|v| v.dyn_into::<Function>())
    {
        let _ = destroy.call1(&JsValue::NULL, &JsValue::from_f64(instance_id as f64));
    }
}

#[cfg(target_arch = "wasm32")]
async fn handle_action_js(
    handle: &Rc<JsValue>,
    instance_id: u32,
    action_json: &str,
    view_state: &ViewState,
) -> Result<semio_framework_core::kernel::ActionResult, String> {
    let action = Reflect::get(handle.as_ref(), &JsValue::from_str("handleAction"))
        .ok()
        .and_then(|v| v.dyn_into::<Function>().ok());
    let Some(action) = action else {
        return Ok(semio_framework_core::kernel::ActionResult {
            output: serde_json::Value::Null,
            operations: vec![],
            inverse_group: semio_framework_core::kernel::UndoGroup {
                action_id: semio_framework_core::kernel::ActionInvocationId(String::new()),
                operations: vec![],
                inverse_operations: vec![],
            },
            diagnostics: vec![],
            requested_effects: vec![],
            events: vec![],
            ui_scope: semio_framework_core::kernel::UiDirtyScope::default(),
        });
    };
    let context_json = serde_json::json!({
        "viewState": view_state,
        "actor": "local",
    })
    .to_string();
    let result = action
        .call3(
            &JsValue::NULL,
            &JsValue::from_f64(instance_id as f64),
            &JsValue::from_str(action_json),
            &JsValue::from_str(&context_json),
        )
        .map_err(|_| "handle_action failed")?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        JsFuture::from(promise.clone())
            .await
            .map_err(|_| "handle_action promise failed")?
    } else {
        result
    };
    if let Some(text) = resolved.as_string() {
        if let Ok(parsed) = serde_json::from_str::<semio_framework_core::kernel::ActionResult>(&text) {
            return Ok(parsed);
        }
        if let Ok(ops) = serde_json::from_str::<Vec<String>>(&text) {
            let descriptor: ui_wgpu::ActionDescriptor =
                serde_json::from_str(action_json).unwrap_or(ui_wgpu::ActionDescriptor {
                    controller_id: String::new(),
                    action: String::new(),
                    args: None,
                });
            return Ok(semio_framework_plugin::action_result_from_patch_ops(
                ops,
                &descriptor.action,
                instance_id,
                0,
                "local",
            ));
        }
    }
    Ok(semio_framework_plugin::action_result_from_patch_ops(
        Vec::new(),
        "",
        instance_id,
        0,
        "local",
    ))
}

#[cfg(target_arch = "wasm32")]
async fn render_js(
    handle: &Rc<JsValue>,
    instance_id: u32,
    body_key: &str,
    view_state: &ViewState,
) -> Result<UiNode, String> {
    render_with_document_js(handle, instance_id, body_key, view_state, None).await
}

#[cfg(target_arch = "wasm32")]
async fn render_with_document_js(
    handle: &Rc<JsValue>,
    instance_id: u32,
    body_key: &str,
    view_state: &ViewState,
    document_json: Option<&str>,
) -> Result<UiNode, String> {
    let render = if document_json.is_some() {
        Reflect::get(handle.as_ref(), &JsValue::from_str("renderWithDocument"))
            .ok()
            .and_then(|v| v.dyn_into::<Function>().ok())
            .or_else(|| get_fn(handle, "render").ok())
    } else {
        get_fn(handle, "render").ok()
    };
    let render = render.ok_or("render failed")?;
    let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
    let result = if let Some(document) = document_json {
        render.call4(
            &JsValue::NULL,
            &JsValue::from_f64(instance_id as f64),
            &JsValue::from_str(body_key),
            &JsValue::from_str(&view_json),
            &JsValue::from_str(document),
        )
    } else {
        render.call3(
            &JsValue::NULL,
            &JsValue::from_f64(instance_id as f64),
            &JsValue::from_str(body_key),
            &JsValue::from_str(&view_json),
        )
    }
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

#[cfg(target_arch = "wasm32")]
async fn window_engagements_js(
    handle: &Rc<JsValue>,
    instance_id: u32,
    view_state: &ViewState,
) -> Result<HashMap<String, WindowEngagement>, String> {
    let engagements = Reflect::get(handle.as_ref(), &JsValue::from_str("windowEngagements"))
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

#[cfg(target_arch = "wasm32")]
async fn window_measures_js(
    handle: &Rc<JsValue>,
    instance_id: u32,
    view_state: &ViewState,
) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
    let measures = Reflect::get(handle.as_ref(), &JsValue::from_str("windowMeasures"))
        .ok()
        .and_then(|v| v.dyn_into::<Function>().ok());
    let Some(measures) = measures else {
        return Ok(HashMap::new());
    };
    let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
    let result = measures
        .call2(
            &JsValue::NULL,
            &JsValue::from_f64(instance_id as f64),
            &JsValue::from_str(&view_json),
        )
        .map_err(|_| "window_measures failed")?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        JsFuture::from(promise.clone())
            .await
            .map_err(|_| "window_measures promise failed")?
    } else {
        result
    };
    let json = resolved.as_string().ok_or("window_measures not string")?;
    serde_json::from_str(&json).map_err(|err| format!("window_measures parse: {err}"))
}

#[cfg(target_arch = "wasm32")]
fn get_fn(obj: &JsValue, key: &str) -> Result<Function, String> {
    Reflect::get(obj, &JsValue::from_str(key))
        .map_err(|_| format!("missing {key}"))?
        .dyn_into()
        .map_err(|_| format!("{key} not fn"))
}

#[cfg(target_arch = "wasm32")]
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
        entries.push(PluginBridgeEntry::from_js(plugin_id.clone(), handle).map_err(|err| {
            format!("plugin {plugin_id}: {err}")
        })?);
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

#[cfg(not(target_arch = "wasm32"))]
pub fn load_wasm_plugins(plugin_filter: &str, modules_root: &std::path::Path) -> Result<Vec<PluginBridgeEntry>, String> {
    let plugin_ids: Vec<String> = if is_studio_mode(plugin_filter) {
        std::fs::read_dir(modules_root)
            .map_err(|error| error.to_string())?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
            .collect()
    } else {
        vec![plugin_filter.to_string()]
    };
    let mut entries = Vec::new();
    for plugin_id in plugin_ids {
        let plugin_dir = modules_root.join(&plugin_id);
        if !plugin_dir.is_dir() {
            continue;
        }
        let wasm_path = std::fs::read_dir(&plugin_dir)
            .map_err(|error| error.to_string())?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .find(|path| path.extension().is_some_and(|ext| ext == "wasm"));
        let Some(path) = wasm_path else {
            continue;
        };
        let runtime = Arc::new(WasmPluginRuntime::load(&path)?);
        entries.push(PluginBridgeEntry::from_wasm(plugin_id, runtime)?);
    }
    if entries.is_empty() {
        return Err(format!("[DEBUG] no wasm plugins found under {}", modules_root.display()));
    }
    Ok(entries)
}
// #endregion plugin_bridge
}

pub mod scenes {
//#region scenes
//! 🎬 Native component scene hosts for canvas-2d, tables, graphs, and 3D views.

use crate::engine_canvas;
use crate::interpreter::{validate_component_scene, FrameworkWidgetContext, RENDER_PLAN_LIMITS};
use crate::shell::{push_context_menu_item, push_find_item, ContextMenuItem, ShellFindItem, ShellState};
use infinite_world::{render_world_3d, World3dState};
use base64::Engine;
use ui_wgpu::{ActionDescriptor, SurfaceKind, UiComponentSceneNode};
use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ui_wgpu::input::{DragAxis, KeyAction};
use ui_wgpu::{draw_text, draw_text_wrapped, render_widget, HitKind, HitTarget, Rect, Rgba, Theme, WidgetNode};

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
    MapMarquee {
        start_x: f32,
        start_y: f32,
        method: String,
        merge_mode: String,
    },
    MapPan,
    NotePan { start_x: f32, start_y: f32, camera_x: f64, camera_y: f64, zoom: f64 },
    NoteMove { origins: HashMap<String, (f64, f64)>, start_x: f32, start_y: f32 },
    NoteResize { handle: String, from: NoteBoundsF, start_x: f32, start_y: f32, selected_ids: Vec<String> },
    NoteInk { block_id: String },
    NoteEraser { mode: String },
    NoteMarqueeDrag { start_x: f32, start_y: f32 },
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
    canvas_image_src_digests: HashMap<String, u64>,
    paint_stroke_active: bool,
    vfs_expanded_ids: HashSet<String>,
    vfs_selection_anchor: Option<String>,
    map_marquee_points: Vec<(f32, f32)>,
    map_marquee_active: bool,
    map_last_hover_json: Option<String>,
    note_camera: Option<(f64, f64, f64)>,
    note_overrides: HashMap<String, Value>,
    note_marquee_points: Vec<(f32, f32)>,
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

/** @emoji 🕸️ Resolves a graph node instance id for context-menu actions. */
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

fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: scene.controller_id.clone(),
        action: action.into(),
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
const MAP_MARQUEE_THRESHOLD_PX: f32 = 6.0;

pub fn handle_scene_wheel(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    x: f32,
    y: f32,
    delta: f32,
    ctrl: bool,
) -> Vec<ActionDescriptor> {
    if !bounds.contains(x, y) {
        return Vec::new();
    }
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    match scene.component_kind {
        SurfaceKind::Table => {
            let current = scroll_offset(&scene.surface_id, "body");
            set_scroll_offset(&scene.surface_id, "body", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::TextEditor => engine_canvas::text_editor_wheel(scene, delta),
        SurfaceKind::VirtualFileSystem => {
            let current = scroll_offset(&scene.surface_id, "vfs");
            set_scroll_offset(&scene.surface_id, "vfs", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::Canvas2d => {
            mutate_scene_state(&scene.surface_id, |state| {
                let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.125, 8.0);
            });
            Vec::new()
        }
        SurfaceKind::Raster => {
            if let Some(raster) = &scene.raster {
                let doc: RasterDocSyncJson = serde_json::from_str(&raster.document_sync_json).unwrap_or_default();
                mutate_scene_state(&scene.surface_id, |state| {
                    if state.viewport.zoom <= 0.0 {
                        state.viewport = Viewport {
                            x: doc.camera.x as f32,
                            y: doc.camera.y as f32,
                            zoom: doc.camera.zoom as f32,
                        };
                    }
                    let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                    state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.05, 32.0);
                });
            }
            Vec::new()
        }
        SurfaceKind::NodeGraph => engine_canvas::node_graph_wheel(
            &scene.surface_id,
            &scene.controller_id,
            inner,
            x,
            y,
            delta,
            ctrl,
        ),
        SurfaceKind::GisMap => engine_canvas::gis_map_wheel(
            &scene.surface_id,
            &scene.controller_id,
            inner,
            x,
            y,
            delta,
            ctrl,
        ),
        SurfaceKind::NoteCanvas => note_wheel(scene, inner, x, y, delta),
        SurfaceKind::VcsHistory => {
            let current = scroll_offset(&scene.surface_id, "history");
            set_scroll_offset(&scene.surface_id, "history", current + delta * 0.5);
            Vec::new()
        }
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
) -> Vec<ActionDescriptor> {
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    let mut actions = Vec::new();
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
                SceneDragMode::MapMarquee { start_x, start_y, method, .. } => {
                    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
                    let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(&scene.surface_id, |state| {
                        if distance >= MAP_MARQUEE_THRESHOLD_PX {
                            state.map_marquee_active = true;
                        }
                        if state.map_marquee_active {
                            if method == "lasso" {
                                state.map_marquee_points.push((sx as f32, sy as f32));
                            } else {
                                state.map_marquee_points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                            }
                        }
                    });
                }
                SceneDragMode::MapPan => {}
                SceneDragMode::NotePan { start_x, start_y, camera_x, camera_y, zoom } => {
                    let dx = (x - start_x) as f64;
                    let dy = (y - start_y) as f64;
                    let next = NoteCameraF { x: camera_x + dx, y: camera_y + dy, zoom: *zoom };
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.note_camera = Some((next.x, next.y, next.zoom));
                    });
                    actions.push(note_set_camera_action(scene, next));
                }
                SceneDragMode::NoteMove { origins, start_x, start_y } => {
                    let camera = note_current_camera(scene);
                    let dx = (x - start_x) as f64 / camera.zoom.max(0.0001);
                    let dy = (y - start_y) as f64 / camera.zoom.max(0.0001);
                    let doc: NoteDocumentJson = scene
                        .note_canvas
                        .as_ref()
                        .map(|n| serde_json::from_str(&n.document_json).unwrap_or_default())
                        .unwrap_or_default();
                    let mut events = Vec::new();
                    let mut new_overrides = Vec::new();
                    for (id, (ox, oy)) in origins.iter() {
                        if let Some(block) = find_note_block(&doc.blocks, id) {
                            let updated = note_block_with_position(block, ox + dx, oy + dy);
                            events.push(json!({ "op": "updateBlock", "blockId": id, "block": updated }));
                            new_overrides.push((id.clone(), updated));
                        }
                    }
                    if !events.is_empty() {
                        mutate_scene_state(&scene.surface_id, |state| {
                            for (id, block) in new_overrides {
                                state.note_overrides.insert(id, block);
                            }
                        });
                        actions.push(note_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::NoteResize { handle, from, start_x, start_y, selected_ids } => {
                    let camera = note_current_camera(scene);
                    let dx = (x - start_x) as f64 / camera.zoom.max(0.0001);
                    let dy = (y - start_y) as f64 / camera.zoom.max(0.0001);
                    let to = note_resize_bounds(*from, handle, dx, dy, 8.0);
                    let doc: NoteDocumentJson = scene
                        .note_canvas
                        .as_ref()
                        .map(|n| serde_json::from_str(&n.document_json).unwrap_or_default())
                        .unwrap_or_default();
                    let mut events = Vec::new();
                    let mut new_overrides = Vec::new();
                    for id in selected_ids {
                        if let Some(block) = find_note_block(&doc.blocks, id) {
                            let updated = note_scaled_block(block, *from, to);
                            events.push(json!({ "op": "updateBlock", "blockId": id, "block": updated }));
                            new_overrides.push((id.clone(), updated));
                        }
                    }
                    if !events.is_empty() {
                        mutate_scene_state(&scene.surface_id, |state| {
                            for (id, block) in new_overrides {
                                state.note_overrides.insert(id, block);
                            }
                        });
                        actions.push(note_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::NoteInk { block_id } => {
                    let camera = note_current_camera(scene);
                    let (world_x, world_y) = note_screen_to_world(camera, inner, x, y);
                    let doc: NoteDocumentJson = scene
                        .note_canvas
                        .as_ref()
                        .map(|n| serde_json::from_str(&n.document_json).unwrap_or_default())
                        .unwrap_or_default();
                    let current = state
                        .note_overrides
                        .get(block_id)
                        .cloned()
                        .or_else(|| find_note_block(&doc.blocks, block_id).cloned());
                    if let Some(mut block) = current {
                        let bx = note_block_num(&block, "x");
                        let by = note_block_num(&block, "y");
                        let local = json!([world_x - bx, world_y - by]);
                        if let Some(obj) = block.as_object_mut() {
                            let mut points = obj.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
                            points.push(local);
                            obj.insert("points".into(), Value::Array(points));
                        }
                        let block_id = block_id.clone();
                        mutate_scene_state(&scene.surface_id, |state| {
                            state.note_overrides.insert(block_id.clone(), block.clone());
                        });
                        actions.push(note_apply_events_action(
                            scene,
                            &[json!({ "op": "updateBlock", "blockId": block_id, "block": block })],
                            "live",
                            None,
                        ));
                    }
                }
                SceneDragMode::NoteEraser { mode } => {
                    let camera = note_current_camera(scene);
                    let (world_x, world_y) = note_screen_to_world(camera, inner, x, y);
                    let doc: NoteDocumentJson = scene
                        .note_canvas
                        .as_ref()
                        .map(|n| serde_json::from_str(&n.document_json).unwrap_or_default())
                        .unwrap_or_default();
                    let events = if mode == "eraserStroke" {
                        note_erase_ink_stroke_events(&doc.blocks, world_x, world_y, 8.0)
                    } else {
                        note_erase_ink_points_events(&doc.blocks, world_x, world_y, doc.eraser_radius.unwrap_or(12.0))
                    };
                    if !events.is_empty() {
                        actions.push(note_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::NoteMarqueeDrag { start_x, start_y } => {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.note_marquee_points = vec![(*start_x, *start_y), (x, y)];
                    });
                }
            }
        }
    }
    match scene.component_kind {
        SurfaceKind::NoteCanvas if !down => {
            actions.extend(note_hover_move(scene, inner, x, y));
        }
        SurfaceKind::Canvas2d if down => {
            actions.push(scene_action(
                scene,
                "canvasPointerMove",
                canvas_world_pointer_json(scene, inner, x, y, json!({})),
            ));
        }
        SurfaceKind::NodeGraph if down => {
            actions.extend(engine_canvas::node_graph_pointer_move(
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
        SurfaceKind::TextEditor if down => {
            actions.extend(engine_canvas::text_editor_pointer_move(scene, inner, x, y));
        }
        SurfaceKind::NodeGraph | SurfaceKind::TextEditor if !down => {
            actions.extend(match scene.component_kind {
                SurfaceKind::NodeGraph => engine_canvas::node_graph_pointer_move(
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
    actions
}

pub fn handle_scene_pointer_button(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    shift: bool,
) -> Vec<ActionDescriptor> {
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
    let mut actions = Vec::new();
    if down {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = true;
        });
        match scene.component_kind {
            SurfaceKind::Canvas2d => {
                if button == 0 {
                    mutate_scene_state(&scene.surface_id, |state| {
                        if !state.paint_stroke_active {
                            state.paint_stroke_active = true;
                        }
                    });
                    actions.push(scene_action(scene, "paintStrokeBegin", json!({ "surfaceId": scene.surface_id })));
                }
                actions.push(scene_action(
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
            SurfaceKind::Raster => {
                if button == 1 || button == 2 {
                    if let Some(raster) = &scene.raster {
                        let doc: RasterDocSyncJson = serde_json::from_str(&raster.document_sync_json).unwrap_or_default();
                        mutate_scene_state(&scene.surface_id, |state| {
                            if state.viewport.zoom <= 0.0 {
                                state.viewport = Viewport {
                                    x: doc.camera.x as f32,
                                    y: doc.camera.y as f32,
                                    zoom: doc.camera.zoom as f32,
                                };
                            }
                            state.drag = Some(SceneDrag {
                                mode: SceneDragMode::PanViewport,
                                button,
                            });
                        });
                    }
                }
            }
            SurfaceKind::NodeGraph => {
                actions.extend(engine_canvas::node_graph_pointer_down(
                    &scene.surface_id,
                    &scene.controller_id,
                    inner,
                    x,
                    y,
                    button,
                    shift,
                    false,
                    false,
                    false,
                ));
            }
            SurfaceKind::TextEditor => {
                actions.extend(engine_canvas::text_editor_pointer_down(scene, inner, x, y, button));
            }
            SurfaceKind::NoteCanvas => {
                actions.extend(note_pointer_down(scene, inner, x, y, button, shift));
            }
            _ => {}
        }
    } else {
        match scene.component_kind {
            SurfaceKind::NoteCanvas => {
                actions.extend(note_pointer_up(scene, inner, x, y));
            }
            SurfaceKind::Canvas2d => {
                actions.push(scene_action(
                    scene,
                    "canvasPointerUp",
                    canvas_world_pointer_json(scene, inner, x, y, json!({})),
                ));
                mutate_scene_state(&scene.surface_id, |state| {
                    if state.paint_stroke_active {
                        state.paint_stroke_active = false;
                    }
                });
                actions.push(scene_action(scene, "paintStrokeEnd", json!({ "surfaceId": scene.surface_id })));
            }
            SurfaceKind::NodeGraph => {
                actions.extend(engine_canvas::node_graph_pointer_up(
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
            SurfaceKind::TextEditor => {
                actions.extend(engine_canvas::text_editor_pointer_up(scene, inner, x, y));
            }
            _ => {}
        }
        if let Some(target) = hit_double_click_target(scene, inner, x, y) {
            let now = now_ms();
            let prior = scene_state(&scene.surface_id);
            if prior.last_click_target.as_deref() == Some(target.as_str())
                && now - prior.last_click_ms < 400.0
            {
                if let Some(action) = double_click_action(scene, &target, inner, x, y) {
                    actions.push(action);
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
                    actions.push(scene_action(
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
    actions
}

fn hit_double_click_target(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
) -> Option<String> {
    match scene.component_kind {
        SurfaceKind::VirtualFileSystem => {
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let body_y = inner.y + 24.0;
            let index = ((y - body_y + scroll) / row_h).floor() as i32;
            if index < 0 {
                return None;
            }
            Some(format!("{}.vfs.index.{index}", scene.surface_id))
        }
        SurfaceKind::NodeGraph => hit_graph_node(scene, inner, x, y)
            .map(|id| format!("{}.node.{}", scene.surface_id, id)),
        _ => None,
    }
}

fn double_click_action(
    scene: &UiComponentSceneNode,
    target: &str,
    inner: Rect,
    x: f32,
    y: f32,
) -> Option<ActionDescriptor> {
    match scene.component_kind {
        SurfaceKind::VirtualFileSystem => {
            let vfs = scene.virtual_file_system.as_ref()?;
            let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).ok()?;
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let index = ((y - inner.y - 24.0 + scroll) / row_h).floor() as usize;
            rows.get(index)
                .and_then(|row| vfs_double_click_action(scene, row))
        }
        SurfaceKind::NodeGraph => {
            let node_id = target.strip_prefix(&format!("{}.node.", scene.surface_id))?;
            let record = find_graph_node(scene, node_id)?;
            let instance_id = record.instance_id.as_deref()?;
            Some(scene_action(
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
    gis_map_states: &mut HashMap<String, GisMapSurface>,
    icon_render_states: &mut HashMap<String, World3dState>,
    puzzle2d_board_states: &mut HashMap<String, Puzzle2dBoardSurface>,
) {
    if let Err(message) = validate_component_scene(scene, &RENDER_PLAN_LIMITS) {
        let theme = ctx.theme;
        ctx.draw.set_screen_height(bounds.y + bounds.h);
        ctx.draw.push_rounded(
            [bounds.x, bounds.y, bounds.w, bounds.h],
            theme.panel,
            theme.border_radius,
        );
        draw_text(
            ctx,
            &format!("Render plan rejected: {message}"),
            bounds.x + 12.0,
            bounds.y + 24.0,
            theme.font_size_body,
            theme.text_muted,
        );
        return;
    }
    let theme = ctx.theme;
    ctx.draw.set_screen_height(bounds.y + bounds.h);
    ctx.draw.push_rounded(
        [bounds.x, bounds.y, bounds.w, bounds.h],
        theme.panel,
        theme.border_radius,
    );
    match scene.component_kind {
        SurfaceKind::Raster => render_raster(scene, bounds, ctx, gpu),
        SurfaceKind::Table => render_table(scene, bounds, ctx),
        SurfaceKind::Canvas2d => render_canvas_2d(scene, bounds, ctx),
        SurfaceKind::NodeGraph => render_node_graph(scene, bounds, ctx, gpu, node_graph_states),
        SurfaceKind::GisMap => render_gis_map(scene, bounds, ctx, gpu, gis_map_states),
        SurfaceKind::VirtualFileSystem => render_vfs(scene, bounds, ctx),
        SurfaceKind::TextEditor => render_text_editor(scene, bounds, ctx, gpu),
        SurfaceKind::NoteCanvas => render_note_canvas(scene, bounds, ctx, gpu),
        SurfaceKind::World3d => {
            let state = world3d_states
                .entry(scene.surface_id.clone())
                .or_insert_with(|| World3dState::new(scene.surface_id.clone(), scene.controller_id.clone()));
            render_world_3d(scene, bounds, ctx, state, gpu);
        }
        SurfaceKind::IconRender => render_icon_render(scene, bounds, ctx, gpu, icon_render_states),
        SurfaceKind::Puzzle2dBoard => render_puzzle_board(scene, bounds, ctx, gpu, puzzle2d_board_states),
        SurfaceKind::VcsHistory => render_vcs_history(scene, bounds, ctx),
        _ => render_placeholder(scene.component_kind.as_str(), bounds, ctx),
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
#[derive(Deserialize, Clone, Copy)]
struct RasterCameraFields {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "raster_default_one")]
    zoom: f64,
}

impl Default for RasterCameraFields {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct RasterTransformFields {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "raster_default_one")]
    scale_x: f64,
    #[serde(default = "raster_default_one")]
    scale_y: f64,
}

impl Default for RasterTransformFields {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0 }
    }
}

fn raster_default_one() -> f64 {
    1.0
}

fn raster_default_true() -> bool {
    true
}

fn raster_default_opacity() -> f32 {
    1.0
}

#[derive(Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum RasterLayerJson {
    #[serde(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        #[serde(default = "raster_default_true")]
        visible: bool,
        #[serde(default = "raster_default_opacity")]
        opacity: f32,
        #[serde(default)]
        transform: RasterTransformFields,
        width: Option<u32>,
        height: Option<u32>,
        image_key: Option<String>,
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        #[serde(default = "raster_default_true")]
        visible: bool,
        #[serde(default = "raster_default_opacity")]
        opacity: f32,
        #[serde(default)]
        transform: RasterTransformFields,
        #[serde(default)]
        children: Vec<RasterLayerJson>,
    },
    #[serde(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {},
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct RasterDocSyncJson {
    #[serde(default)]
    camera: RasterCameraFields,
    #[serde(default)]
    layers: Vec<RasterLayerJson>,
}

#[derive(Deserialize)]
struct RasterAssetJson {
    mime: String,
    data: String,
}

struct RasterFlatLayer {
    id: String,
    image_key: String,
    x: f64,
    y: f64,
    scale_x: f64,
    scale_y: f64,
    opacity: f32,
    width: u32,
    height: u32,
}

fn collect_raster_pixel_layers(
    layers: &[RasterLayerJson],
    parent_x: f64,
    parent_y: f64,
    parent_sx: f64,
    parent_sy: f64,
    parent_opacity: f32,
    out: &mut Vec<RasterFlatLayer>,
) {
    for layer in layers {
        match layer {
            RasterLayerJson::Pixel { id, visible, opacity, transform, width, height, image_key } => {
                if !*visible {
                    continue;
                }
                let Some(image_key) = image_key else {
                    continue;
                };
                out.push(RasterFlatLayer {
                    id: id.clone(),
                    image_key: image_key.clone(),
                    x: parent_x + transform.x * parent_sx,
                    y: parent_y + transform.y * parent_sy,
                    scale_x: parent_sx * transform.scale_x,
                    scale_y: parent_sy * transform.scale_y,
                    opacity: opacity * parent_opacity,
                    width: width.unwrap_or(0),
                    height: height.unwrap_or(0),
                });
            }
            RasterLayerJson::Group { visible, opacity, transform, children } => {
                if !*visible {
                    continue;
                }
                collect_raster_pixel_layers(
                    children,
                    parent_x + transform.x * parent_sx,
                    parent_y + transform.y * parent_sy,
                    parent_sx * transform.scale_x,
                    parent_sy * transform.scale_y,
                    opacity * parent_opacity,
                    out,
                );
            }
            RasterLayerJson::Adjustment { .. } => {}
        }
    }
}

/** 🖼️ Composites raster document layers as textured quads; blend modes, masks and adjustment layers are not yet applied (see FIX-LOWPOLY-DEV-BOOT sibling ticket 26/07/11/WGPU-RENDERER-FULL-PARITY for follow-up scope). */
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
    let _ = gpu;
    let inner = bounds;
    ctx.draw
        .push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let doc: RasterDocSyncJson = serde_json::from_str(&raster.document_sync_json).unwrap_or_default();
    let assets: HashMap<String, RasterAssetJson> = serde_json::from_str(&raster.assets_json).unwrap_or_default();
    let mut viewport = Viewport {
        x: doc.camera.x as f32,
        y: doc.camera.y as f32,
        zoom: doc.camera.zoom as f32,
    };
    let local = scene_state(&scene.surface_id);
    if local.viewport.zoom > 0.0 {
        viewport = local.viewport;
    }
    draw_checkerboard(ctx.draw, &viewport, inner, theme, 4096.0);
    let mut flat = Vec::new();
    collect_raster_pixel_layers(&doc.layers, 0.0, 0.0, 1.0, 1.0, 1.0, &mut flat);
    if flat.is_empty() {
        draw_text(ctx, "Empty raster document", inner.x + 8.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
    }
    for layer in &flat {
        let Some(asset) = assets.get(&layer.image_key) else {
            continue;
        };
        let data_url = format!("data:{};base64,{}", asset.mime, asset.data);
        let Some(key) = queue_canvas_image_upload(&scene.surface_id, &layer.id, &data_url) else {
            continue;
        };
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let w = layer.width as f32 * layer.scale_x as f32 * viewport.zoom;
        let h = layer.height as f32 * layer.scale_y as f32 * viewport.zoom;
        ctx.draw
            .push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], layer.opacity);
    }
    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: Some(scene_action(scene, "rasterClick", surface_args(scene))),
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

/// 🧾 Mirrors `ui_wgpu::TableCell` — a typed table cell value parsed out of a row's raw JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum TableCellPayload {
    Text { value: String },
    Number { value: f64 },
    Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
    Buttons { buttons: Vec<TableCellButtonPayload> },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableCellButtonPayload {
    icon_id: String,
    #[serde(default)]
    label: Option<String>,
    action: ActionDescriptor,
}

/// 🔗 Merges `patch` into `base`'s existing args (rather than replacing them), so a stepper/button cell keeps its row-identifying args (e.g. `objectId`) alongside the delta/click patch.
fn merge_action_args(base: &ActionDescriptor, patch: Value) -> ActionDescriptor {
    let mut args = match &base.args {
        Some(Value::Object(map)) => map.clone(),
        _ => serde_json::Map::new(),
    };
    if let Value::Object(patch_map) = patch {
        args.extend(patch_map);
    }
    ActionDescriptor {
        controller_id: base.controller_id.clone(),
        action: base.action.clone(),
        args: Some(Value::Object(args)),
    }
}

/// 🧾 Renders a table cell's interactive controls (stepper/buttons) directly, or returns the plain text to draw for text/number/legacy-string cells.
fn render_table_cell(cell: &Value, rect: Rect, ctx: &mut FrameworkWidgetContext<'_>) -> Option<String> {
    let Ok(payload) = serde_json::from_value::<TableCellPayload>(cell.clone()) else {
        return Some(match cell {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        });
    };
    match payload {
        TableCellPayload::Text { value } => Some(value),
        TableCellPayload::Number { value } => Some(value.to_string()),
        TableCellPayload::Stepper { value, min, max, step, action } => {
            let seg = rect.w / 3.0;
            let minus = Rect::new(rect.x, rect.y, seg, rect.h);
            let center = Rect::new(rect.x + seg, rect.y, seg, rect.h);
            let plus = Rect::new(rect.x + seg * 2.0, rect.y, seg, rect.h);
            render_widget(
                &WidgetNode::Button {
                    id: None,
                    icon_id: None,
                    label: "−".into(),
                    event: (value > min).then(|| merge_action_args(&action, json!({ "delta": -step }))),
                },
                minus,
                ctx,
            );
            render_widget(&WidgetNode::Text { value: format!("{value:.0}"), emphasize: false }, center, ctx);
            render_widget(
                &WidgetNode::Button {
                    id: None,
                    icon_id: None,
                    label: "+".into(),
                    event: (value < max).then(|| merge_action_args(&action, json!({ "delta": step }))),
                },
                plus,
                ctx,
            );
            None
        }
        TableCellPayload::Buttons { buttons } => {
            let seg = if buttons.is_empty() { rect.w } else { rect.w / buttons.len() as f32 };
            for (index, button) in buttons.iter().enumerate() {
                let button_rect = Rect::new(rect.x + index as f32 * seg, rect.y, seg, rect.h);
                render_widget(
                    &WidgetNode::Button {
                        id: None,
                        icon_id: Some(button.icon_id.clone()),
                        label: button.label.clone().unwrap_or_default(),
                        event: Some(button.action.clone()),
                    },
                    button_rect,
                    ctx,
                );
            }
            None
        }
    }
}

fn render_table(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(table) = &scene.table else {
        return render_placeholder("table", bounds, ctx);
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let selected_ids: Vec<String> = table
        .selection_json
        .as_deref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| value.get("selectedIds").cloned())
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default();
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
        let selected = selected_ids.iter().any(|id| id == &row_id);
        if selected {
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
        for (col_index, column) in columns.iter().enumerate() {
            let x = body.x + col_index as f32 * col_w;
            let cell_rect = Rect::new(x + pad, y, col_w - pad * 2.0, row_h);
            let text = match row.get(&column.id) {
                Some(value) => render_table_cell(value, cell_rect, ctx),
                None => Some("—".into()),
            };
            if let Some(text) = text {
                draw_text(
                    ctx,
                    &text,
                    x + pad,
                    y + row_h * 0.65,
                    theme.font_size_small,
                    if selected || hovered { theme.active_foreground } else { theme.text },
                );
            }
        }
        let drag_data = table.row_drag_mime.as_ref().and_then(|mime| {
            row.get("_drag")
                .map(|payload| HashMap::from([(mime.clone(), payload.to_string())]))
        });
        ctx.input.register_hit(HitTarget {
            rect: row_rect,
            event: Some(scene_action(
                scene,
                "selectRow",
                json!({ "surfaceId": scene.surface_id, "row": row }),
            )),
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data,
        });
    }
    ctx.draw.pop_scissor();
}
//#endregion Table

//#region VcsHistory
/** @emoji 🗄️ Mirrors `vcs::HistoryColumn` / React `HistoryColumn` (`ui/js/react/index.tsx:19116`). */
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryColumnAuthorJson {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryColumnJson {
    checkpoint_id: String,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    authors: Vec<HistoryColumnAuthorJson>,
    #[serde(default)]
    parent_checkpoint_id: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    lane: usize,
}

const HISTORY_LANE_PITCH: f32 = 16.0;
const HISTORY_LANE_PAD: f32 = 8.0;
const HISTORY_AUTHOR_SLOT: f32 = 40.0;

/** Ports `historyLaneCount` (`ui/js/react/index.tsx:19141`). */
fn history_lane_count(columns: &[HistoryColumnJson]) -> usize {
    columns.iter().map(|column| column.lane + 1).max().unwrap_or(1).max(1)
}

/** Ports `historyGraphWidth` (`ui/js/react/index.tsx:19145`). */
fn history_graph_width(lane_count: usize) -> f32 {
    (HISTORY_LANE_PAD * 2.0 + lane_count as f32 * HISTORY_LANE_PITCH).max(56.0)
}

/** Ports `historyLaneX` (`ui/js/react/index.tsx:19153`). */
fn history_lane_x(lane: usize, lane_count: usize, graph_width: f32) -> f32 {
    if lane_count <= 1 {
        return graph_width * 0.5;
    }
    HISTORY_LANE_PAD + lane as f32 * HISTORY_LANE_PITCH + HISTORY_LANE_PITCH * 0.5
}

/** Ports `historyRowLaneGuides` (`ui/js/react/index.tsx:19162`): per-row, per-lane guide-line
 * visibility, including the elbow-row propagation when a checkpoint's parent sits on another lane. */
fn history_row_lane_guides(columns: &[HistoryColumnJson], lane_count: usize) -> Vec<Vec<bool>> {
    let mut guides = vec![vec![false; lane_count]; columns.len()];
    let row_by_id: HashMap<&str, usize> = columns
        .iter()
        .enumerate()
        .map(|(index, column)| (column.checkpoint_id.as_str(), index))
        .collect();
    for (row_index, column) in columns.iter().enumerate() {
        if column.lane < lane_count {
            guides[row_index][column.lane] = true;
        }
        let Some(parent_row) = column.parent_checkpoint_id.as_deref().and_then(|id| row_by_id.get(id).copied()) else {
            continue;
        };
        let parent_lane = columns[parent_row].lane;
        if column.lane == parent_lane {
            for row in (row_index + 1)..parent_row {
                guides[row][column.lane] = true;
            }
            continue;
        }
        let elbow_row = if row_index + 1 < parent_row { row_index + 1 } else { parent_row };
        for row in (row_index + 1)..=elbow_row {
            if column.lane < lane_count {
                guides[row][column.lane] = true;
            }
        }
        for row in elbow_row..parent_row {
            if parent_lane < lane_count {
                guides[row][parent_lane] = true;
            }
        }
    }
    guides
}

fn render_vcs_history(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(history) = &scene.vcs_history else {
        return render_placeholder("vcs-history", bounds, ctx);
    };
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let inner = bounds;
    let row_h = theme.control_height * 1.33;
    let pad = theme.padding_standard;
    if columns.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }
    let lane_count = history_lane_count(&columns);
    let graph_width = history_graph_width(lane_count);
    let graph_col_w = graph_width + HISTORY_AUTHOR_SLOT;
    let labels_col_w = (inner.w * 0.28).max(96.0);
    let guides = history_row_lane_guides(&columns, lane_count);
    let row_by_id: HashMap<&str, usize> = columns
        .iter()
        .enumerate()
        .map(|(index, column)| (column.checkpoint_id.as_str(), index))
        .collect();

    let scroll = scroll_offset(&scene.surface_id, "history");
    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: None,
        control_id: Some(scroll_key(&scene.surface_id, "history")),
        kind: HitKind::ScrollRegion,
        drag_axis: None,
        drag_data: None,
    });
    ctx.draw.push_scissor(inner);
    let hovered_row = ctx.input.hovered_id.clone();
    let graph_x0 = inner.x + labels_col_w;
    let desc_x = inner.x + labels_col_w + graph_col_w;

    for (row_index, column) in columns.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        let control_id = format!("{}.history.{}", scene.surface_id, column.checkpoint_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, row_h);
        if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(
            row_rect.x,
            row_rect.y + row_rect.h - theme.stroke_hairline,
            row_rect.x + row_rect.w,
            row_rect.y + row_rect.h - theme.stroke_hairline,
            theme.separator,
            1.0,
        );

        let mut label_x = inner.x + pad;
        if column.labels.is_empty() {
            draw_text(ctx, "checkpoint", label_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        } else {
            for label in &column.labels {
                let chip_w = (label.len() as f32 * 6.0 + pad * 2.0).min((inner.x + labels_col_w - label_x).max(0.0));
                if chip_w <= 0.0 {
                    break;
                }
                ctx.draw.push_rounded([label_x, y + row_h * 0.5 - 9.0, chip_w, 18.0], theme.accent, 4.0);
                draw_text(ctx, label, label_x + 4.0, y + row_h * 0.5 + 4.0, theme.font_size_small, theme.active_foreground);
                label_x += chip_w + 4.0;
            }
        }

        for lane in 0..lane_count {
            if guides[row_index][lane] {
                let lx = graph_x0 + history_lane_x(lane, lane_count, graph_width);
                ctx.draw.push_line(lx, y, lx, y + row_h, theme.separator, 1.0);
            }
        }
        if let Some(parent_id) = column.parent_checkpoint_id.as_deref() {
            if let Some(&parent_row) = row_by_id.get(parent_id) {
                let x0 = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
                let parent_lane = columns[parent_row].lane;
                let x1 = graph_x0 + history_lane_x(parent_lane, lane_count, graph_width);
                let y0 = y + row_h * 0.5;
                let y1 = inner.y + parent_row as f32 * row_h - scroll + row_h * 0.5;
                if (x0 - x1).abs() < 0.5 {
                    ctx.draw.push_line(x0, y0, x1, y1, theme.separator, 1.5);
                } else {
                    let elbow_y = y + row_h;
                    ctx.draw.push_line(x0, y0, x0, elbow_y, theme.separator, 1.5);
                    ctx.draw.push_line(x0, elbow_y, x1, elbow_y, theme.separator, 1.5);
                    ctx.draw.push_line(x1, elbow_y, x1, y1, theme.separator, 1.5);
                }
            }
        }
        let dot_x = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([dot_x - 3.0, dot_y - 3.0, 6.0, 6.0], theme.text, 3.0);

        let avatar_size = 20.0;
        let avatar_x = graph_x0 + graph_width + 4.0;
        let avatar_y = y + row_h * 0.5 - avatar_size * 0.5;
        let initial = column
            .authors
            .first()
            .and_then(|author| author.name.chars().next())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".into());
        ctx.draw.push_rounded([avatar_x, avatar_y, avatar_size, avatar_size], theme.button, avatar_size * 0.5);
        draw_text(ctx, &initial, avatar_x + avatar_size * 0.32, avatar_y + avatar_size * 0.7, theme.font_size_small, theme.text);

        if let Some(description) = &column.description {
            draw_text(ctx, description, desc_x + pad, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        }

        ctx.input.register_hit(HitTarget {
            rect: row_rect,
            event: Some(scene_action(scene, "checkoutCheckpoint", json!({ "checkpointId": column.checkpoint_id }))),
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data: None,
        });
    }
    ctx.draw.pop_scissor();
}
//#endregion VcsHistory

//#region VcsHistoryTests
#[cfg(test)]
mod vcs_history_tests {
    use super::*;

    fn column(id: &str, lane: usize, parent: Option<&str>) -> HistoryColumnJson {
        HistoryColumnJson {
            checkpoint_id: id.to_string(),
            labels: Vec::new(),
            authors: Vec::new(),
            parent_checkpoint_id: parent.map(str::to_string),
            description: None,
            lane,
        }
    }

    #[test]
    fn lane_count_is_max_lane_plus_one() {
        let columns = vec![column("a", 0, None), column("b", 2, Some("a"))];
        assert_eq!(history_lane_count(&columns), 3);
    }

    #[test]
    fn lane_count_defaults_to_one_for_empty_columns() {
        assert_eq!(history_lane_count(&[]), 1);
    }

    #[test]
    fn lane_x_centers_graph_when_single_lane() {
        let width = history_graph_width(1);
        assert_eq!(history_lane_x(0, 1, width), width * 0.5);
    }

    #[test]
    fn linear_history_guides_stay_on_single_lane() {
        let columns = vec![column("c", 0, Some("b")), column("b", 0, Some("a")), column("a", 0, None)];
        let guides = history_row_lane_guides(&columns, 1);
        assert!(guides.iter().all(|row| row[0]), "a linear single-lane history must keep every row's lane-0 guide active");
    }

    #[test]
    fn fork_guides_propagate_through_elbow_row() {
        let columns = vec![column("c", 1, Some("a")), column("b", 0, None), column("a", 0, None)];
        let guides = history_row_lane_guides(&columns, 2);
        assert!(guides[0][1], "the forking checkpoint's own row must show its lane");
        assert!(guides[1][1] || guides[1][0], "the elbow row must carry a guide on at least one of the two connected lanes");
    }

    #[test]
    fn columns_json_tolerates_missing_optional_fields() {
        let json = r#"[{"checkpointId":"only-required"}]"#;
        let columns: Vec<HistoryColumnJson> = serde_json::from_str(json).unwrap();
        assert_eq!(columns.len(), 1);
        assert_eq!(columns[0].checkpoint_id, "only-required");
        assert_eq!(columns[0].lane, 0);
        assert!(columns[0].labels.is_empty());
        assert!(columns[0].authors.is_empty());
        assert!(columns[0].parent_checkpoint_id.is_none());
        assert!(columns[0].description.is_none());
    }
}
//#endregion VcsHistoryTests

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

/** Hashes the raw (still-encoded) `data_url` before touching base64/PNG decode, so an unchanged
 * image layer costs one cheap byte-hash per frame instead of a full decode — decode only runs when
 * the source string actually changes. Continuous rAF renderers (e.g. raster) would otherwise redo
 * base64+PNG decode every frame for every image layer regardless of whether anything changed. */
pub(crate) fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    let key = format!("canvas-image:{surface_id}:{layer_id}");
    let src_key = format!("canvas-image-src:{surface_id}:{layer_id}");
    let src_digest = digest_pixels(data_url.as_bytes());
    let unchanged = scene_state(surface_id).canvas_image_src_digests.get(&src_key).copied() == Some(src_digest);
    if unchanged {
        return Some(key);
    }
    let (pixels, width, height) = decode_canvas_image(data_url)?;
    let expected = (width as usize).saturating_mul(height as usize).saturating_mul(4);
    if pixels.len() < expected {
        return None;
    }
    let digest = digest_pixels(&pixels[..expected]);
    mutate_scene_state(surface_id, |state| {
        state.canvas_image_src_digests.insert(src_key.clone(), src_digest);
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

/** Clamps checkerboard cell iteration to the world-space rect actually visible through `inner`
 * (intersected with the full `±extent/2` grid) instead of always walking the whole grid — a
 * continuously-rendering surface (raster) was pushing up to `(extent/cell)^2` solid quads every
 * single frame regardless of zoom/pan, which starves headless WebGPU frame pacing. */
fn draw_checkerboard(
    draw: &mut ui_wgpu::DrawList,
    viewport: &Viewport,
    inner: Rect,
    theme: &ui_wgpu::Theme,
    extent: f32,
) {
    let cell = 16.0;
    let half = extent * 0.5;
    let light = theme.checker_light;
    let dark = theme.checker_dark;
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (-half, half, -half, half);
    if viewport.zoom > 0.0 {
        let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
        let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
        min_x = min_x.max(wx0.min(wx1) - cell);
        max_x = max_x.min(wx0.max(wx1) + cell);
        min_y = min_y.max(wy0.min(wy1) - cell);
        max_y = max_y.min(wy0.max(wy1) + cell);
    }
    let start_row = ((min_y - (-half)) / cell).floor().max(0.0) as i64;
    let start_col = ((min_x - (-half)) / cell).floor().max(0.0) as i64;
    let mut row = start_row;
    let mut wy = -half + start_row as f32 * cell;
    while wy < max_y {
        let mut col = start_col;
        let mut wx = -half + start_col as f32 * cell;
        while wx < max_x {
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
    if local.viewport.zoom > 0.0 && scene.component_kind == SurfaceKind::Canvas2d {
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
                let stroke = theme.diagram_stroke;
                let seam_stroke = theme.diagram_seam;
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
        let stroke = Rgba::new(
            theme.diagram_accent.r + hue / 720.0,
            theme.diagram_accent.g,
            theme.diagram_accent.b,
            theme.diagram_accent.a,
        );
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
            Rgba::new(
                theme.diagram_accent_fill.r + hue / 720.0,
                theme.diagram_accent_fill.g,
                theme.diagram_accent_fill.b,
                theme.diagram_accent_fill.a,
            ),
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
//#region NoteCanvas
// 📝 Direct DrawList painting for note-canvas, ported from note-canvas-host.tsx (framework/renderer/react).

//#region NoteCanvasModel
static NOTE_HOST_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn create_note_host_id(prefix: &str) -> String {
    let next = NOTE_HOST_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("{prefix}-host-{next}")
}

#[derive(Clone, Copy, Debug, Default)]
struct NoteCameraF {
    x: f64,
    y: f64,
    zoom: f64,
}

impl From<NoteCameraJson> for NoteCameraF {
    fn from(camera: NoteCameraJson) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoteCameraJson {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "note_default_zoom")]
    zoom: f64,
}

fn note_default_zoom() -> f64 {
    1.0
}

impl Default for NoteCameraJson {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct NoteDocumentJson {
    schema: String,
    id: String,
    camera: NoteCameraJson,
    blocks: Vec<Value>,
    active_tool: Option<String>,
    grid_visible: Option<bool>,
    grid_spacing: Option<f64>,
    grid_subdivisions: Option<f64>,
    grid_opacity: Option<f64>,
    snap_enabled: Option<bool>,
    snap_grid_spacing: Option<f64>,
    pencil_width: Option<f64>,
    eraser_radius: Option<f64>,
    assets: HashMap<String, Value>,
}

impl Default for NoteDocumentJson {
    fn default() -> Self {
        Self {
            schema: "note.document".into(),
            id: "empty".into(),
            camera: NoteCameraJson::default(),
            blocks: Vec::new(),
            active_tool: Some("selectDirect".into()),
            grid_visible: None,
            grid_spacing: None,
            grid_subdivisions: None,
            grid_opacity: None,
            snap_enabled: None,
            snap_grid_spacing: None,
            pencil_width: None,
            eraser_radius: None,
            assets: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct NoteBoundsF {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl NoteBoundsF {
    fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    fn intersects(&self, other: &NoteBoundsF) -> bool {
        self.x < other.x + other.w && self.x + self.w > other.x && self.y < other.y + other.h && self.y + self.h > other.y
    }
}

fn note_block_str<'a>(block: &'a Value, key: &str) -> &'a str {
    block.get(key).and_then(Value::as_str).unwrap_or("")
}

fn note_block_id(block: &Value) -> &str {
    note_block_str(block, "id")
}

fn note_block_kind(block: &Value) -> &str {
    note_block_str(block, "kind")
}

fn note_block_visible(block: &Value) -> bool {
    block.get("visible").and_then(Value::as_bool).unwrap_or(true)
}

fn note_block_locked(block: &Value) -> bool {
    block.get("locked").and_then(Value::as_bool).unwrap_or(false)
}

fn note_block_num(block: &Value, key: &str) -> f64 {
    block.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn note_block_bounds(block: &Value) -> NoteBoundsF {
    let x = note_block_num(block, "x");
    let y = note_block_num(block, "y");
    let w = note_block_num(block, "width");
    let h = note_block_num(block, "height");
    if note_block_kind(block) == "ink" {
        if let Some(points) = block.get("points").and_then(Value::as_array) {
            if !points.is_empty() {
                let mut min_x = f64::INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for point in points {
                    let px = point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    min_x = min_x.min(px);
                    min_y = min_y.min(py);
                    max_x = max_x.max(px);
                    max_y = max_y.max(py);
                }
                return NoteBoundsF {
                    x: x + min_x,
                    y: y + min_y,
                    w: (max_x - min_x).max(1.0),
                    h: (max_y - min_y).max(1.0),
                };
            }
        }
    }
    NoteBoundsF { x, y, w, h }
}

fn note_effective_bounds(block: &Value, overrides: &HashMap<String, Value>) -> NoteBoundsF {
    match overrides.get(note_block_id(block)) {
        Some(over) => note_block_bounds(over),
        None => note_block_bounds(block),
    }
}

fn flatten_note_blocks(blocks: &[Value]) -> Vec<&Value> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [Value], out: &mut Vec<&'a Value>) {
        for block in blocks {
            out.push(block);
            if note_block_kind(block) == "group" {
                if let Some(children) = block.get("children").and_then(Value::as_array) {
                    visit(children, out);
                }
            }
        }
    }
    visit(blocks, &mut out);
    out
}

fn find_note_block<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
    flatten_note_blocks(blocks).into_iter().find(|block| note_block_id(block) == id)
}

fn note_blocks_at_point<'a>(blocks: &'a [Value], overrides: &HashMap<String, Value>, x: f64, y: f64) -> Vec<&'a Value> {
    let mut flat = flatten_note_blocks(blocks);
    flat.reverse();
    flat.into_iter()
        .filter(|block| note_effective_bounds(block, overrides).contains_point(x, y))
        .collect()
}

fn note_blocks_intersecting_rect(blocks: &[Value], overrides: &HashMap<String, Value>, rect: NoteBoundsF) -> Vec<String> {
    flatten_note_blocks(blocks)
        .into_iter()
        .filter(|block| note_effective_bounds(block, overrides).intersects(&rect))
        .map(|block| note_block_id(block).to_string())
        .collect()
}

fn note_selection_bounds(blocks: &[Value], overrides: &HashMap<String, Value>, ids: &[String]) -> Option<NoteBoundsF> {
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let selected: Vec<NoteBoundsF> = flatten_note_blocks(blocks)
        .into_iter()
        .filter(|block| id_set.contains(note_block_id(block)))
        .map(|block| note_effective_bounds(block, overrides))
        .collect();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for bounds in &selected {
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.w);
        max_y = max_y.max(bounds.y + bounds.h);
    }
    Some(NoteBoundsF { x: min_x, y: min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) })
}

fn note_scale_value(v: f64, from_min: f64, from_size: f64, to_min: f64, to_size: f64) -> f64 {
    if from_size <= 0.0 {
        return to_min;
    }
    to_min + ((v - from_min) / from_size) * to_size
}

fn note_scaled_block(block: &Value, from: NoteBoundsF, to: NoteBoundsF) -> Value {
    let bounds = note_block_bounds(block);
    let next_x = note_scale_value(bounds.x, from.x, from.w, to.x, to.w);
    let next_y = note_scale_value(bounds.y, from.y, from.h, to.y, to.h);
    let next_w = (note_scale_value(bounds.x + bounds.w, from.x, from.w, to.x, to.w) - next_x).max(8.0);
    let next_h = (note_scale_value(bounds.y + bounds.h, from.y, from.h, to.y, to.h) - next_y).max(8.0);
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(next_x));
        obj.insert("y".into(), json!(next_y));
        obj.insert("width".into(), json!(next_w));
        obj.insert("height".into(), json!(next_h));
        if note_block_kind(block) == "ink" {
            let scale_x = if from.w > 0.0 { to.w / from.w } else { 1.0 };
            let scale_y = if from.h > 0.0 { to.h / from.h } else { 1.0 };
            if let Some(points) = block.get("points").and_then(Value::as_array) {
                let scaled: Vec<Value> = points
                    .iter()
                    .map(|p| {
                        let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0) * scale_x;
                        let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0) * scale_y;
                        json!([px, py])
                    })
                    .collect();
                obj.insert("points".into(), Value::Array(scaled));
            }
        }
    }
    cloned
}

fn note_resize_bounds(from: NoteBoundsF, handle: &str, dx: f64, dy: f64, min_size: f64) -> NoteBoundsF {
    let mut x = from.x;
    let mut y = from.y;
    let mut w = from.w;
    let mut h = from.h;
    if handle.contains('e') {
        w = (w + dx).max(min_size);
    }
    if handle.contains('w') {
        let next_w = (w - dx).max(min_size);
        x += w - next_w;
        w = next_w;
    }
    if handle.contains('s') {
        h = (h + dy).max(min_size);
    }
    if handle.contains('n') {
        let next_h = (h - dy).max(min_size);
        y += h - next_h;
        h = next_h;
    }
    NoteBoundsF { x, y, w, h }
}

fn note_snap_coordinate(v: f64, spacing: f64) -> f64 {
    if spacing <= 0.0 {
        v
    } else {
        (v / spacing).round() * spacing
    }
}

fn note_snap_point(x: f64, y: f64, spacing: f64) -> (f64, f64) {
    (note_snap_coordinate(x, spacing), note_snap_coordinate(y, spacing))
}

fn note_maybe_snap(doc: &NoteDocumentJson, x: f64, y: f64) -> (f64, f64) {
    if doc.snap_enabled.unwrap_or(false) {
        note_snap_point(x, y, doc.snap_grid_spacing.unwrap_or(8.0))
    } else {
        (x, y)
    }
}

fn note_block_with_position(block: &Value, x: f64, y: f64) -> Value {
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(x));
        obj.insert("y".into(), json!(y));
    }
    cloned
}

fn note_create_block(kind: &str, x: f64, y: f64) -> Value {
    let id = create_note_host_id(kind);
    match kind {
        "image" => json!({
            "id": id, "name": "Image", "kind": "image", "x": x, "y": y, "width": 240.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false, "imageKey": "placeholder",
        }),
        "table" => json!({
            "id": id, "name": "Table", "kind": "table", "x": x, "y": y, "width": 320.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "columns": ["A", "B", "C"],
            "rows": [
                [{"content": ""}, {"content": ""}, {"content": ""}],
                [{"content": ""}, {"content": ""}, {"content": ""}],
            ],
        }),
        "math" => json!({
            "id": id, "name": "Math", "kind": "math", "x": x, "y": y, "width": 200.0, "height": 80.0,
            "rotation": 0.0, "visible": true, "locked": false, "tex": "E = mc^2", "displayMode": true,
        }),
        "ink" => json!({
            "id": id, "name": "Ink", "kind": "ink", "x": x, "y": y, "width": 1.0, "height": 1.0,
            "rotation": 0.0, "visible": true, "locked": false, "points": [], "strokeWidth": 3.0, "color": [0.0, 0.0, 0.0, 1.0],
        }),
        "group" => json!({
            "id": id, "name": "Group", "kind": "group", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false, "children": [],
        }),
        _ => json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [{"runs": [{"text": ""}]}], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        }),
    }
}

fn note_text_plain(block: &Value) -> String {
    block
        .get("paragraphs")
        .and_then(Value::as_array)
        .map(|paragraphs| {
            paragraphs
                .iter()
                .map(|paragraph| {
                    paragraph
                        .get("runs")
                        .and_then(Value::as_array)
                        .map(|runs| runs.iter().filter_map(|run| run.get("text").and_then(Value::as_str)).collect::<String>())
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}

fn point_segment_distance(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    if dx == 0.0 && dy == 0.0 {
        return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
    }
    let t = (((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((px - (x1 + t * dx)).powi(2) + (py - (y1 + t * dy)).powi(2)).sqrt()
}

fn ink_points(block: &Value) -> Vec<(f64, f64)> {
    let bx = note_block_num(block, "x");
    let by = note_block_num(block, "y");
    block
        .get("points")
        .and_then(Value::as_array)
        .map(|points| {
            points
                .iter()
                .map(|p| {
                    let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    (bx + px, by + py)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn ink_hits_point(block: &Value, x: f64, y: f64, threshold: f64) -> bool {
    let points = ink_points(block);
    let stroke_width = note_block_num(block, "strokeWidth");
    if points.len() < 2 {
        return points.first().map(|p| ((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt() <= threshold).unwrap_or(false);
    }
    points
        .windows(2)
        .any(|w| point_segment_distance(x, y, w[0].0, w[0].1, w[1].0, w[1].1) <= threshold + stroke_width / 2.0)
}

fn note_erase_ink_stroke_events(blocks: &[Value], x: f64, y: f64, threshold: f64) -> Vec<Value> {
    flatten_note_blocks(blocks)
        .into_iter()
        .filter(|block| note_block_kind(block) == "ink" && ink_hits_point(block, x, y, threshold))
        .map(|block| json!({ "op": "removeBlock", "blockId": note_block_id(block) }))
        .collect()
}

fn note_erase_ink_points_in_block(block: &Value, x: f64, y: f64, radius: f64) -> Vec<Value> {
    let bx = note_block_num(block, "x");
    let by = note_block_num(block, "y");
    let points = block.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut kept_indices = Vec::new();
    for (index, point) in points.iter().enumerate() {
        let px = bx + point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
        let py = by + point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
        if ((px - x).powi(2) + (py - y).powi(2)).sqrt() > radius {
            kept_indices.push(index);
        }
    }
    if kept_indices.len() == points.len() {
        return vec![block.clone()];
    }
    if kept_indices.is_empty() {
        return Vec::new();
    }
    let mut runs: Vec<Vec<Value>> = Vec::new();
    let mut current: Vec<Value> = vec![points[kept_indices[0]].clone()];
    for window in kept_indices.windows(2) {
        if window[1] - window[0] > 1 {
            if current.len() >= 2 {
                runs.push(current);
            }
            current = vec![points[window[1]].clone()];
        } else {
            current.push(points[window[1]].clone());
        }
    }
    if current.len() >= 2 {
        runs.push(current);
    }
    let name = note_block_str(block, "name").to_string();
    runs.into_iter()
        .enumerate()
        .map(|(index, pts)| {
            let mut cloned = block.clone();
            if let Some(obj) = cloned.as_object_mut() {
                if index > 0 {
                    obj.insert("id".into(), json!(create_note_host_id("ink")));
                    obj.insert("name".into(), json!(format!("{name} fragment")));
                }
                obj.insert("points".into(), Value::Array(pts));
            }
            cloned
        })
        .collect()
}

fn note_erase_ink_points_events(blocks: &[Value], x: f64, y: f64, radius: f64) -> Vec<Value> {
    let mut events = Vec::new();
    for block in flatten_note_blocks(blocks) {
        if note_block_kind(block) != "ink" {
            continue;
        }
        let fragments = note_erase_ink_points_in_block(block, x, y, radius);
        if fragments.len() == 1 && fragments[0] == *block {
            continue;
        }
        events.push(json!({ "op": "removeBlock", "blockId": note_block_id(block) }));
        for fragment in fragments {
            events.push(json!({ "op": "addBlock", "block": fragment }));
        }
    }
    events
}

fn note_screen_to_world(camera: NoteCameraF, inner: Rect, sx: f32, sy: f32) -> (f64, f64) {
    let lx = (sx - inner.x) as f64;
    let ly = (sy - inner.y) as f64;
    ((lx - camera.x) / camera.zoom, (ly - camera.y) / camera.zoom)
}

fn note_world_to_screen(camera: NoteCameraF, inner: Rect, wx: f64, wy: f64) -> (f32, f32) {
    (inner.x + (wx * camera.zoom + camera.x) as f32, inner.y + (wy * camera.zoom + camera.y) as f32)
}

fn positive_mod_f32(v: f32, m: f32) -> f32 {
    if m <= 0.0 {
        0.0
    } else {
        ((v % m) + m) % m
    }
}
//#endregion NoteCanvasModel

//#region NoteCanvasState
fn note_current_camera(scene: &UiComponentSceneNode) -> NoteCameraF {
    let state = scene_state(&scene.surface_id);
    if let Some((x, y, zoom)) = state.note_camera {
        return NoteCameraF { x, y, zoom };
    }
    scene
        .note_canvas
        .as_ref()
        .and_then(|note| serde_json::from_str::<NoteDocumentJson>(&note.document_json).ok())
        .map(|doc| NoteCameraF::from(doc.camera))
        .unwrap_or_default()
}

fn note_events_json(events: &[Value]) -> String {
    Value::Array(events.to_vec()).to_string()
}

fn note_apply_events_action(scene: &UiComponentSceneNode, events: &[Value], phase: &str, select_ids: Option<&[String]>) -> ActionDescriptor {
    let mut args = json!({
        "surfaceId": scene.surface_id,
        "eventsJson": note_events_json(events),
        "phase": phase,
    });
    if let Some(ids) = select_ids {
        args["selectIds"] = json!(ids);
    }
    scene_action(scene, "applyNoteEvents", args)
}

fn note_set_selection_action(scene: &UiComponentSceneNode, ids: &[String]) -> ActionDescriptor {
    scene_action(scene, "setSelection", json!({ "surfaceId": scene.surface_id, "ids": ids }))
}

fn note_set_hover_action(scene: &UiComponentSceneNode, id: Option<&str>) -> ActionDescriptor {
    scene_action(scene, "setHover", json!({ "surfaceId": scene.surface_id, "id": id }))
}

fn note_set_camera_action(scene: &UiComponentSceneNode, camera: NoteCameraF) -> ActionDescriptor {
    scene_action(
        scene,
        "setCamera",
        json!({ "surfaceId": scene.surface_id, "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom } }),
    )
}

const NOTE_RESIZE_HANDLES: [&str; 8] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];

fn note_resize_handle_screen_pos(handle: &str, sx: f32, sy: f32, w: f32, h: f32, size: f32) -> (f32, f32) {
    let half = size * 0.5;
    let x = if handle.contains('w') {
        sx - half
    } else if handle.contains('e') {
        sx + w - half
    } else {
        sx + w * 0.5 - half
    };
    let y = if handle.contains('n') {
        sy - half
    } else if handle.contains('s') {
        sy + h - half
    } else {
        sy + h * 0.5 - half
    };
    (x, y)
}

fn note_resize_handle_at(bounds: NoteBoundsF, camera: NoteCameraF, inner: Rect, sx: f32, sy: f32, hit_radius: f32) -> Option<&'static str> {
    let (bx, by) = note_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    for handle in NOTE_RESIZE_HANDLES {
        let (hx, hy) = note_resize_handle_screen_pos(handle, bx, by, w, h, 8.0);
        let cx = hx + 4.0;
        let cy = hy + 4.0;
        if ((sx - cx).powi(2) + (sy - cy).powi(2)).sqrt() <= hit_radius {
            return Some(handle);
        }
    }
    None
}

/** @emoji 📝 Pointer-down entry point for note-canvas: mirrors handlePointerDown in note-canvas-host.tsx. */
fn note_pointer_down(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, shift: bool) -> Vec<ActionDescriptor> {
    let Some(note) = &scene.note_canvas else {
        return Vec::new();
    };
    if note.view_mode == "navigator" || !note.interactive {
        return Vec::new();
    }
    let doc: NoteDocumentJson = serde_json::from_str(&note.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&note.selection_json).unwrap_or_default();
    let state = scene_state(&scene.surface_id);
    let camera = state
        .note_camera
        .map(|(cx, cy, cz)| NoteCameraF { x: cx, y: cy, zoom: cz })
        .unwrap_or_else(|| NoteCameraF::from(doc.camera.clone()));
    let tool = doc.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
    let mut actions = Vec::new();

    let selection_bounds = note_selection_bounds(&doc.blocks, &state.note_overrides, &selected_ids);
    let show_handles = (tool == "selectDirect" || tool == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if button == 0 && show_handles {
        if let Some(bounds) = selection_bounds {
            if let Some(handle) = note_resize_handle_at(bounds, camera, inner, x, y, 8.0) {
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag {
                        mode: SceneDragMode::NoteResize {
                            handle: handle.to_string(),
                            from: bounds,
                            start_x: x,
                            start_y: y,
                            selected_ids: selected_ids.clone(),
                        },
                        button,
                    });
                });
                return actions;
            }
        }
    }

    if tool == "pan" || button == 1 {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag {
                mode: SceneDragMode::NotePan { start_x: x, start_y: y, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom },
                button,
            });
        });
        return actions;
    }

    if button != 0 {
        return actions;
    }

    let (world_x, world_y) = note_screen_to_world(camera, inner, x, y);

    if tool == "eraserStroke" || tool == "eraserPoint" {
        let events = if tool == "eraserStroke" {
            note_erase_ink_stroke_events(&doc.blocks, world_x, world_y, 8.0)
        } else {
            note_erase_ink_points_events(&doc.blocks, world_x, world_y, doc.eraser_radius.unwrap_or(12.0))
        };
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::NoteEraser { mode: tool.clone() }, button });
        });
        if !events.is_empty() {
            actions.push(note_apply_events_action(scene, &events, "begin", None));
        }
        return actions;
    }

    if tool == "selectMarquee" {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::NoteMarqueeDrag { start_x: x, start_y: y }, button });
            s.note_marquee_points = vec![(x, y)];
        });
        return actions;
    }

    if tool == "pencil" {
        let block = note_create_block("ink", world_x, world_y);
        let block_id = note_block_id(&block).to_string();
        mutate_scene_state(&scene.surface_id, |s| {
            s.note_overrides.insert(block_id.clone(), block.clone());
            s.drag = Some(SceneDrag { mode: SceneDragMode::NoteInk { block_id: block_id.clone() }, button });
        });
        actions.push(note_apply_events_action(scene, &[json!({ "op": "addBlock", "block": block })], "begin", Some(&[block_id])));
        return actions;
    }

    if tool == "text" || tool == "image" || tool == "table" || tool == "math" {
        let (px, py) = note_maybe_snap(&doc, world_x, world_y);
        let block = note_create_block(&tool, px, py);
        let block_id = note_block_id(&block).to_string();
        actions.push(note_apply_events_action(scene, &[json!({ "op": "addBlock", "block": block })], "atomic", Some(&[block_id])));
        return actions;
    }

    let hits = note_blocks_at_point(&doc.blocks, &state.note_overrides, world_x, world_y);
    let top = hits.first().copied();
    match top {
        Some(top_block) if !note_block_locked(top_block) => {
            if tool == "selectDirect" {
                let top_id = note_block_id(top_block).to_string();
                let next_selection = if shift {
                    let mut ids: Vec<String> = selected_ids.clone();
                    if !ids.contains(&top_id) {
                        ids.push(top_id.clone());
                    }
                    ids
                } else {
                    vec![top_id.clone()]
                };
                actions.push(note_set_selection_action(scene, &next_selection));
                let move_ids: Vec<String> = if selected_ids.contains(&top_id) { selected_ids.clone() } else { vec![top_id.clone()] };
                let mut origins = HashMap::new();
                for id in &move_ids {
                    if let Some(b) = find_note_block(&doc.blocks, id) {
                        let eff = state.note_overrides.get(id).unwrap_or(b);
                        origins.insert(id.clone(), (note_block_num(eff, "x"), note_block_num(eff, "y")));
                    }
                }
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag { mode: SceneDragMode::NoteMove { origins, start_x: x, start_y: y }, button });
                });
            }
        }
        _ => {
            if tool == "selectDirect" {
                actions.push(note_set_selection_action(scene, &[]));
            }
        }
    }
    actions
}

/** @emoji 📝 Pointer-up entry point for note-canvas: commits the active gesture and finalizes marquee selection. */
fn note_pointer_up(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    let state = scene_state(&scene.surface_id);
    let Some(drag) = state.drag.clone() else {
        return actions;
    };
    let doc: NoteDocumentJson = scene
        .note_canvas
        .as_ref()
        .map(|n| serde_json::from_str(&n.document_json).unwrap_or_default())
        .unwrap_or_default();
    match &drag.mode {
        SceneDragMode::NoteMove { origins, .. } => {
            let mut events = Vec::new();
            for id in origins.keys() {
                if let Some(block) = state.note_overrides.get(id).cloned().or_else(|| find_note_block(&doc.blocks, id).cloned()) {
                    let updated = if doc.snap_enabled.unwrap_or(false) {
                        let spacing = doc.snap_grid_spacing.unwrap_or(8.0);
                        let (sx, sy) = note_snap_point(note_block_num(&block, "x"), note_block_num(&block, "y"), spacing);
                        note_block_with_position(&block, sx, sy)
                    } else {
                        block
                    };
                    events.push(json!({ "op": "updateBlock", "blockId": id, "block": updated }));
                }
            }
            actions.push(note_apply_events_action(scene, &events, "commit", None));
        }
        SceneDragMode::NoteResize { selected_ids, .. } => {
            let mut events = Vec::new();
            for id in selected_ids {
                if let Some(block) = state.note_overrides.get(id).cloned() {
                    events.push(json!({ "op": "updateBlock", "blockId": id, "block": block }));
                }
            }
            actions.push(note_apply_events_action(scene, &events, "commit", None));
        }
        SceneDragMode::NoteInk { block_id } => {
            if let Some(block) = state.note_overrides.get(block_id).cloned() {
                actions.push(note_apply_events_action(scene, &[json!({ "op": "updateBlock", "blockId": block_id, "block": block })], "commit", None));
            } else {
                actions.push(note_apply_events_action(scene, &[], "commit", None));
            }
        }
        SceneDragMode::NoteEraser { .. } => {
            actions.push(note_apply_events_action(scene, &[], "commit", None));
        }
        SceneDragMode::NoteMarqueeDrag { start_x, start_y } => {
            let x0 = start_x.min(x);
            let y0 = start_y.min(y);
            let w = (x - start_x).abs();
            let h = (y - start_y).abs();
            if w >= 4.0 || h >= 4.0 {
                let camera = note_current_camera(scene);
                let (wx0, wy0) = note_screen_to_world(camera, inner, x0, y0);
                let (wx1, wy1) = note_screen_to_world(camera, inner, x0 + w, y0 + h);
                let world_rect = NoteBoundsF { x: wx0.min(wx1), y: wy0.min(wy1), w: (wx1 - wx0).abs(), h: (wy1 - wy0).abs() };
                let ids = note_blocks_intersecting_rect(&doc.blocks, &state.note_overrides, world_rect);
                actions.push(note_set_selection_action(scene, &ids));
            }
        }
        _ => {}
    }
    mutate_scene_state(&scene.surface_id, |s| {
        s.drag = None;
        s.note_marquee_points.clear();
    });
    actions
}

/** @emoji 📝 Pointer-move hover entry point for note-canvas: mirrors the `!dragState` hover branch of handlePointerMove. */
fn note_hover_move(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let Some(note) = &scene.note_canvas else {
        return Vec::new();
    };
    if note.view_mode == "navigator" || !note.interactive {
        return Vec::new();
    }
    let doc: NoteDocumentJson = serde_json::from_str(&note.document_json).unwrap_or_default();
    let camera = note_current_camera(scene);
    let (wx, wy) = note_screen_to_world(camera, inner, x, y);
    let state = scene_state(&scene.surface_id);
    let hits = note_blocks_at_point(&doc.blocks, &state.note_overrides, wx, wy);
    let top_id = hits.first().map(|block| note_block_id(block).to_string());
    if note.hovered_id.as_deref() == top_id.as_deref() {
        return Vec::new();
    }
    vec![note_set_hover_action(scene, top_id.as_deref())]
}

/** @emoji 📝 Wheel entry point for note-canvas: zoom-at-cursor, mirrors handleWheel in note-canvas-host.tsx. */
fn note_wheel(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    let Some(note) = &scene.note_canvas else {
        return Vec::new();
    };
    if note.view_mode == "navigator" {
        return Vec::new();
    }
    let camera = note_current_camera(scene);
    let zoom_factor: f64 = if delta < 0.0 { 1.08 } else { 0.92 };
    let next_zoom = (camera.zoom * zoom_factor).clamp(0.1, 8.0);
    let (wx, wy) = note_screen_to_world(camera, inner, x, y);
    let next = NoteCameraF {
        x: (x - inner.x) as f64 - wx * next_zoom,
        y: (y - inner.y) as f64 - wy * next_zoom,
        zoom: next_zoom,
    };
    mutate_scene_state(&scene.surface_id, |s| {
        s.note_camera = Some((next.x, next.y, next.zoom));
    });
    vec![note_set_camera_action(scene, next)]
}
//#endregion NoteCanvasState

//#region NoteCanvasRender
fn note_draw_rect_outline(draw: &mut ui_wgpu::DrawList, x: f32, y: f32, w: f32, h: f32, color: Rgba, width: f32) {
    draw.push_line(x, y, x + w, y, color, width);
    draw.push_line(x + w, y, x + w, y + h, color, width);
    draw.push_line(x + w, y + h, x, y + h, color, width);
    draw.push_line(x, y + h, x, y, color, width);
}

fn note_draw_grid(draw: &mut ui_wgpu::DrawList, camera: NoteCameraF, inner: Rect, theme: &Theme, spacing: f64, subdivisions: u32, opacity: f64) {
    let major_px = (spacing * camera.zoom) as f32;
    if major_px < 2.0 {
        return;
    }
    let minor_px = major_px / subdivisions.max(1) as f32;
    let offset_x = positive_mod_f32(camera.x as f32, major_px);
    let offset_y = positive_mod_f32(camera.y as f32, major_px);
    let color = theme.separator.with_alpha((theme.separator.a * opacity as f32).max(0.05));
    let minor_color = color.with_alpha(color.a * 0.55);

    let mut wx = inner.x + positive_mod_f32(offset_x, major_px) - major_px;
    while wx < inner.x + inner.w {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let mx = wx + s as f32 * minor_px;
                if mx >= inner.x && mx <= inner.x + inner.w {
                    draw.push_line(mx, inner.y, mx, inner.y + inner.h, minor_color, 0.5);
                }
            }
        }
        if wx >= inner.x && wx <= inner.x + inner.w {
            draw.push_line(wx, inner.y, wx, inner.y + inner.h, color, 1.0);
        }
        wx += major_px;
    }
    let mut wy = inner.y + positive_mod_f32(offset_y, major_px) - major_px;
    while wy < inner.y + inner.h {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let my = wy + s as f32 * minor_px;
                if my >= inner.y && my <= inner.y + inner.h {
                    draw.push_line(inner.x, my, inner.x + inner.w, my, minor_color, 0.5);
                }
            }
        }
        if wy >= inner.y && wy <= inner.y + inner.h {
            draw.push_line(inner.x, wy, inner.x + inner.w, wy, color, 1.0);
        }
        wy += major_px;
    }
}

fn note_draw_table(ctx: &mut FrameworkWidgetContext<'_>, block: &Value, sx: f32, sy: f32, w: f32, h: f32, theme: &Theme) {
    let columns: Vec<String> = block
        .get("columns")
        .and_then(Value::as_array)
        .map(|c| c.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    let rows: Vec<Vec<String>> = block
        .get("rows")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    row.as_array()
                        .map(|cells| cells.iter().map(|cell| cell.get("content").and_then(Value::as_str).unwrap_or("").to_string()).collect())
                        .unwrap_or_default()
                })
                .collect()
        })
        .unwrap_or_default();
    let col_count = columns.len().max(1);
    let row_count = rows.len() + 1;
    let col_w = w / col_count as f32;
    let row_h = h / row_count as f32;
    let font = theme.font_size_small.min(row_h * 0.6).max(6.0);
    for (index, label) in columns.iter().enumerate() {
        draw_text(ctx, label, sx + index as f32 * col_w + 3.0, sy + row_h * 0.7, font, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let ry = sy + (row_index + 1) as f32 * row_h;
        for (col_index, cell) in row.iter().enumerate() {
            draw_text(ctx, cell, sx + col_index as f32 * col_w + 3.0, ry + row_h * 0.7, font, theme.text);
        }
    }
    for index in 0..=col_count {
        let x = sx + index as f32 * col_w;
        ctx.draw.push_line(x, sy, x, sy + h, theme.separator, 0.5);
    }
    for index in 0..=row_count {
        let y = sy + index as f32 * row_h;
        ctx.draw.push_line(sx, y, sx + w, y, theme.separator, 0.5);
    }
}

fn note_draw_image(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, doc: &NoteDocumentJson, sx: f32, sy: f32, w: f32, h: f32) {
    let theme = ctx.theme;
    let image_key = note_block_str(block, "imageKey");
    if let Some(asset) = doc.assets.get(image_key) {
        let mime = asset.get("mime").and_then(Value::as_str).unwrap_or("image/png");
        let data = asset.get("data").and_then(Value::as_str).unwrap_or("");
        let data_url = if data.starts_with("data:") { data.to_string() } else { format!("data:{mime};base64,{data}") };
        if let Some(key) = queue_canvas_image_upload(&scene.surface_id, note_block_id(block), &data_url) {
            ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], 1.0);
            return;
        }
    }
    draw_text(ctx, image_key, sx + 6.0, sy + h * 0.5, theme.font_size_small, theme.text_muted);
}

fn note_draw_block(
    ctx: &mut FrameworkWidgetContext<'_>,
    scene: &UiComponentSceneNode,
    block: &Value,
    camera: NoteCameraF,
    inner: Rect,
    doc: &NoteDocumentJson,
    selected: bool,
    hovered: bool,
) {
    let theme = ctx.theme;
    let kind = note_block_kind(block);
    let bounds = note_block_bounds(block);
    let (sx, sy) = note_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;

    if kind == "ink" {
        let points = ink_points(block);
        if points.len() >= 2 {
            let color = block
                .get("color")
                .and_then(Value::as_array)
                .map(|c| {
                    let get = |i: usize| c.get(i).and_then(Value::as_f64).unwrap_or(0.0) as f32;
                    Rgba::new(get(0), get(1), get(2), get(3))
                })
                .unwrap_or(theme.text);
            let stroke_width = (note_block_num(block, "strokeWidth") as f32 * camera.zoom as f32).max(1.0);
            let screen_points: Vec<(f32, f32)> = points.iter().map(|p| note_world_to_screen(camera, inner, p.0, p.1)).collect();
            for pair in screen_points.windows(2) {
                ctx.draw.push_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, color, stroke_width);
            }
        }
        return;
    }

    let bg = theme.panel;
    ctx.draw.push_rounded([sx, sy, w.max(4.0), h.max(4.0)], bg.with_alpha(0.92), theme.border_radius.min(6.0));

    match kind {
        "text" => {
            let text = note_text_plain(block);
            let font_size = (note_block_num(block, "fontSize").max(8.0) as f32 * camera.zoom as f32).max(6.0);
            draw_text_wrapped(ctx, &text, sx + 6.0, sy + 4.0, (w - 12.0).max(1.0), font_size, theme.text);
        }
        "math" => {
            let tex = note_block_str(block, "tex");
            draw_text(ctx, tex, sx + 8.0, sy + h * 0.5 + 4.0, theme.font_size_body.max(8.0), theme.text);
        }
        "table" => note_draw_table(ctx, block, sx, sy, w.max(4.0), h.max(4.0), theme),
        "image" => note_draw_image(ctx, scene, block, doc, sx, sy, w.max(4.0), h.max(4.0)),
        "group" => {
            let children_len = block.get("children").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
            draw_text(ctx, &format!("Group · {children_len} children"), sx + 6.0, sy + 16.0, theme.font_size_small, theme.text_muted);
        }
        _ => {}
    }

    let border = if selected {
        theme.accent
    } else if hovered {
        theme.accent.with_alpha(theme.accent.a * 0.6)
    } else {
        theme.panel_border
    };
    let border_w = if selected { 2.0 } else { 1.0 };
    note_draw_rect_outline(ctx.draw, sx, sy, w.max(4.0), h.max(4.0), border, border_w);
}

fn note_draw_selection_chrome(draw: &mut ui_wgpu::DrawList, theme: &Theme, camera: NoteCameraF, inner: Rect, bounds: NoteBoundsF, show_handles: bool) {
    let (sx, sy) = note_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    note_draw_rect_outline(draw, sx, sy, w, h, theme.accent, 1.5);
    if !show_handles {
        return;
    }
    let handle_size = 8.0;
    for handle in NOTE_RESIZE_HANDLES {
        let (hx, hy) = note_resize_handle_screen_pos(handle, sx, sy, w, h, handle_size);
        draw.push_rounded([hx, hy, handle_size, handle_size], theme.background, 1.0);
        note_draw_rect_outline(draw, hx, hy, handle_size, handle_size, theme.accent, 1.0);
    }
}

fn render_note_canvas(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::GpuContext) {
    let _ = gpu;
    let theme = ctx.theme;
    let Some(note) = &scene.note_canvas else {
        return render_placeholder("note-canvas", bounds, ctx);
    };
    let doc: NoteDocumentJson = serde_json::from_str(&note.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&note.selection_json).unwrap_or_default();
    let selected_set: HashSet<&str> = selected_ids.iter().map(String::as_str).collect();
    let hovered_id = note.hovered_id.clone();
    let is_navigator = note.view_mode == "navigator";
    let inner = bounds;

    let state = scene_state(&scene.surface_id);
    let camera = state.note_camera.map(|(x, y, zoom)| NoteCameraF { x, y, zoom }).unwrap_or_else(|| NoteCameraF::from(doc.camera.clone()));

    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    ctx.draw.push_scissor(inner);

    if doc.grid_visible.unwrap_or(true) && !is_navigator {
        note_draw_grid(ctx.draw, camera, inner, theme, doc.grid_spacing.unwrap_or(32.0), doc.grid_subdivisions.unwrap_or(4.0).max(1.0) as u32, doc.grid_opacity.unwrap_or(0.35));
    }

    let overrides = state.note_overrides.clone();
    let blocks = flatten_note_blocks(&doc.blocks);
    for block in blocks.iter().copied() {
        let effective = overrides.get(note_block_id(block)).unwrap_or(block);
        if !note_block_visible(effective) {
            continue;
        }
        let id = note_block_id(block);
        let selected = selected_set.contains(id);
        let hovered = hovered_id.as_deref() == Some(id);
        note_draw_block(ctx, scene, effective, camera, inner, &doc, selected, hovered);
    }

    let selection_bounds = note_selection_bounds(&doc.blocks, &overrides, &selected_ids);
    let tool = doc.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
    let show_handles = !is_navigator && (tool == "selectDirect" || tool == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if let Some(sel) = selection_bounds {
        note_draw_selection_chrome(ctx.draw, theme, camera, inner, sel, show_handles);
    }

    if state.note_marquee_points.len() >= 2 {
        let points: Vec<[f32; 2]> = state.note_marquee_points.iter().map(|p| [p.0, p.1]).collect();
        ui_wgpu::paint_selection_marquee(ctx.draw, theme, false, false, &points, false);
    }

    ctx.draw.pop_scissor();

    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: None,
        control_id: Some(scene.surface_id.clone()),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}
//#endregion NoteCanvasRender

//#region RasterFrameCostTests
#[cfg(test)]
mod raster_frame_cost_tests {
    use super::*;

    fn count_solids(draw: &ui_wgpu::DrawList) -> usize {
        draw.layers.iter().map(|layer| layer.ui_instances.len()).sum()
    }

    fn tiny_png_data_url(r: u8, g: u8, b: u8) -> String {
        let img = image::RgbaImage::from_pixel(1, 1, image::Rgba([r, g, b, 255]));
        let mut bytes: Vec<u8> = Vec::new();
        image::DynamicImage::ImageRgba8(img)
            .write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
            .expect("encode tiny test png");
        format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(&bytes))
    }

    #[test]
    fn queue_canvas_image_upload_skips_decode_when_source_unchanged() {
        let surface_id = "raster-frame-cost-test-unchanged";
        let data_url = tiny_png_data_url(10, 20, 30);
        let first = queue_canvas_image_upload(surface_id, "layer-a", &data_url);
        assert!(first.is_some());
        mutate_scene_state(surface_id, |state| {
            state.pending_raster_uploads.clear();
        });
        let second = queue_canvas_image_upload(surface_id, "layer-a", &data_url);
        assert_eq!(first, second, "key must stay stable across frames");
        let pending = scene_state(surface_id).pending_raster_uploads.len();
        assert_eq!(pending, 0, "unchanged data_url must not re-decode/re-queue an upload");
    }

    #[test]
    fn queue_canvas_image_upload_redecodes_when_source_changes() {
        let surface_id = "raster-frame-cost-test-changed";
        let png_a = tiny_png_data_url(10, 20, 30);
        let png_b = tiny_png_data_url(200, 100, 50);
        queue_canvas_image_upload(surface_id, "layer-a", &png_a);
        mutate_scene_state(surface_id, |state| state.pending_raster_uploads.clear());
        queue_canvas_image_upload(surface_id, "layer-a", &png_b);
        let pending = scene_state(surface_id).pending_raster_uploads.len();
        assert_eq!(pending, 1, "changed data_url must re-decode and queue exactly one upload");
    }

    #[test]
    fn draw_checkerboard_clamps_to_visible_viewport() {
        let mut draw = ui_wgpu::DrawList::default();
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let theme = Theme::default();
        draw_checkerboard(&mut draw, &viewport, inner, &theme, 4096.0);
        let quads = count_solids(&draw);
        assert!(quads > 0, "checkerboard should still draw the visible cells");
        assert!(quads < 4000, "checkerboard must clamp to the viewport instead of the full ±extent/2 grid, got {quads}");
    }

    #[test]
    fn draw_checkerboard_falls_back_to_full_extent_when_zoom_is_zero() {
        let mut draw = ui_wgpu::DrawList::default();
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 0.0 };
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let theme = Theme::default();
        draw_checkerboard(&mut draw, &viewport, inner, &theme, 64.0);
        let quads = count_solids(&draw);
        assert_eq!(quads, 16, "degenerate zoom must fall back to the full extent grid (4x4 cells for a 64-unit extent)");
    }
}
//#endregion RasterFrameCostTests

//#region NoteCanvasTests
#[cfg(test)]
mod note_canvas_tests {
    use super::*;

    fn sample_block(id: &str, x: f64, y: f64, w: f64, h: f64) -> Value {
        json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": w, "height": h,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        })
    }

    #[test]
    fn hit_test_prefers_topmost_block() {
        let blocks = vec![sample_block("a", 0.0, 0.0, 100.0, 100.0), sample_block("b", 20.0, 20.0, 100.0, 100.0)];
        let overrides = HashMap::new();
        let hits = note_blocks_at_point(&blocks, &overrides, 50.0, 50.0);
        assert_eq!(note_block_id(hits[0]), "b");
    }

    #[test]
    fn hit_test_misses_outside_bounds() {
        let blocks = vec![sample_block("a", 0.0, 0.0, 10.0, 10.0)];
        let overrides = HashMap::new();
        assert!(note_blocks_at_point(&blocks, &overrides, 50.0, 50.0).is_empty());
    }

    #[test]
    fn resize_bounds_east_handle_grows_width_only() {
        let from = NoteBoundsF { x: 0.0, y: 0.0, w: 100.0, h: 50.0 };
        let to = note_resize_bounds(from, "e", 20.0, 0.0, 8.0);
        assert_eq!(to, NoteBoundsF { x: 0.0, y: 0.0, w: 120.0, h: 50.0 });
    }

    #[test]
    fn resize_bounds_northwest_handle_moves_origin() {
        let from = NoteBoundsF { x: 10.0, y: 10.0, w: 100.0, h: 100.0 };
        let to = note_resize_bounds(from, "nw", -10.0, -10.0, 8.0);
        assert_eq!(to, NoteBoundsF { x: 0.0, y: 0.0, w: 110.0, h: 110.0 });
    }

    #[test]
    fn resize_bounds_respects_minimum_size() {
        let from = NoteBoundsF { x: 0.0, y: 0.0, w: 20.0, h: 20.0 };
        let to = note_resize_bounds(from, "e", -100.0, 0.0, 8.0);
        assert_eq!(to.w, 8.0);
    }

    #[test]
    fn screen_world_roundtrip() {
        let camera = NoteCameraF { x: 12.0, y: -8.0, zoom: 1.5 };
        let inner = Rect::new(100.0, 40.0, 400.0, 300.0);
        let (wx, wy) = note_screen_to_world(camera, inner, 250.0, 150.0);
        let (sx, sy) = note_world_to_screen(camera, inner, wx, wy);
        assert!((sx - 250.0).abs() < 0.01);
        assert!((sy - 150.0).abs() < 0.01);
    }

    #[test]
    fn snap_rounds_to_nearest_grid_cell() {
        assert_eq!(note_snap_coordinate(13.0, 8.0), 16.0);
        assert_eq!(note_snap_coordinate(3.0, 8.0), 0.0);
    }

    #[test]
    fn ink_block_bounds_from_points() {
        let block = json!({
            "id": "i1", "kind": "ink", "x": 10.0, "y": 10.0, "width": 1.0, "height": 1.0,
            "points": [[0.0, 0.0], [5.0, 10.0], [-5.0, 2.0]], "strokeWidth": 3.0, "color": [0, 0, 0, 1],
        });
        let bounds = note_block_bounds(&block);
        assert_eq!(bounds.x, 5.0);
        assert_eq!(bounds.y, 10.0);
        assert_eq!(bounds.w, 10.0);
        assert_eq!(bounds.h, 10.0);
    }
}
//#endregion NoteCanvasTests
//#endregion NoteCanvas

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
    action: String,
    #[serde(default)]
    args: Option<Value>,
}

fn push_graph_context_menu(scene: &UiComponentSceneNode, graph: &ui_wgpu::NodeGraphScene) {
    let Some(raw) = graph.context_menu_json.as_deref() else {
        return;
    };
    let items: Vec<GraphContextMenuItem> = serde_json::from_str(raw).unwrap_or_default();
    for item in items {
        push_context_menu_item(ContextMenuItem {
            id: format!("{}.context.{}", scene.surface_id, item.id),
            label: item.label,
            action: Some(ActionDescriptor {
                controller_id: scene.controller_id.clone(),
                action: item.action,
                args: item.args,
            }),
        });
    }
}

/** @emoji 🕸️ Applies node-hit context to a scene context-menu action. */
pub fn resolve_graph_context_action(
    action: &ActionDescriptor,
    node_id: Option<&str>,
) -> ActionDescriptor {
    let Some(node_id) = node_id else {
        return action.clone();
    };
    let mut resolved = action.clone();
    match action.action.as_str() {
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
    engine_canvas::paint_node_graph_overlays(ctx, scene, inner);
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

//#region GisMap
#[derive(Clone, Debug)]
pub struct GisMapSurface {
    pub bounds: Rect,
    pub controller_id: String,
    pub selection_method: String,
}

fn query_map_feature_hits(
    host: &gis_2d::MapHost,
    method: &str,
    points: &[(f32, f32)],
    crossing: bool,
) -> (Vec<String>, Vec<String>) {
    if method == "lasso" && points.len() >= 3 {
        let payload: Vec<[f64; 2]> = points
            .iter()
            .map(|(x, y)| [*x as f64, *y as f64])
            .collect();
        let points_json = serde_json::to_string(&payload).unwrap_or_else(|_| "[]".into());
        engine_canvas::parse_map_feature_hit(&host.features_in_polygon_json(&points_json, crossing))
    } else if points.len() >= 2 {
        let (x0, y0) = points[0];
        let (x1, y1) = points[points.len() - 1];
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        engine_canvas::parse_map_feature_hit(&host.features_in_rect_json(
            min_x as f64,
            min_y as f64,
            max_x as f64,
            max_y as f64,
            crossing,
        ))
    } else {
        (Vec::new(), Vec::new())
    }
}

fn paint_gis_map_marquee(
    ctx: &mut FrameworkWidgetContext<'_>,
    surface_id: &str,
    inner: Rect,
    theme: &Theme,
) {
    let state = scene_state(surface_id);
    if !state.map_marquee_active {
        return;
    }
    let points = state.map_marquee_points;
    if points.len() < 2 {
        return;
    }
    let method = match &state.drag {
        Some(SceneDrag {
            mode: SceneDragMode::MapMarquee { method, .. },
            ..
        }) => method.as_str(),
        _ => "rectangle",
    };
    let lasso = method == "lasso" && points.len() >= 3;
    let global: Vec<[f32; 2]> = points
        .iter()
        .map(|(x, y)| [inner.x + x, inner.y + y])
        .collect();
    let crossing = ui_wgpu::marquee_is_crossing_from_path(&global, lasso);
    ui_wgpu::paint_selection_marquee(&mut ctx.draw, theme, crossing, lasso, &global, false);
}

/** @emoji 🗺️ Pushes GIS map context-menu items for a screen-space hit. */
pub fn push_gis_map_context_menu(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
) {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    let Some(hit_json) = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)) else {
        return;
    };
    let hit: Value = serde_json::from_str(&hit_json).unwrap_or(Value::Null);
    let (kind, id) = (
        hit.get("kind").and_then(|value| value.as_str()),
        hit.get("id").and_then(|value| value.as_str()),
    );
    if let (Some(kind), Some(id)) = (kind, id) {
        let selected = engine_canvas::with_map_host(surface_id, |host| {
            if kind == "position" {
                host.selected_positions_json().iter().any(|row| row == id)
            } else {
                host.selected_routes_json().iter().any(|row| row == id)
            }
        })
        .unwrap_or(false);
        push_context_menu_item(ContextMenuItem {
            id: format!("{surface_id}.context.select"),
            label: "Select".into(),
            action: Some(engine_canvas::map_action(
                controller_id,
                ui_wgpu::gis_map_actions::SET_FEATURE_SELECTION,
                json!({
                    "surfaceId": surface_id,
                    "positions": if kind == "position" { vec![id] } else { Vec::<&str>::new() },
                    "routes": if kind == "route" { vec![id] } else { Vec::<&str>::new() },
                    "mode": "default",
                }),
            )),
        });
        if selected {
            push_context_menu_item(ContextMenuItem {
                id: format!("{surface_id}.context.deselect"),
                label: "Deselect".into(),
                action: Some(engine_canvas::map_action(
                    controller_id,
                    ui_wgpu::gis_map_actions::DESELECT,
                    json!({ "surfaceId": surface_id, "featureId": id, "featureKind": kind }),
                )),
            });
        }
        push_context_menu_item(ContextMenuItem {
            id: format!("{surface_id}.context.focus"),
            label: "Focus / zoom to".into(),
            action: Some(engine_canvas::map_action(
                controller_id,
                ui_wgpu::gis_map_actions::FOCUS_FEATURE,
                json!({ "surfaceId": surface_id, "featureId": id, "featureKind": kind }),
            )),
        });
        if kind == "position" {
            let has_source = engine_canvas::with_map_host(surface_id, |host| {
                host.positions
                    .get(id)
                    .and_then(|row| row.source_url.as_deref())
                    .filter(|url| !url.is_empty())
                    .is_some()
            })
            .unwrap_or(false);
            if has_source {
                push_context_menu_item(ContextMenuItem {
                    id: format!("{surface_id}.context.source"),
                    label: "Open source".into(),
                    action: Some(engine_canvas::map_action(
                        controller_id,
                        ui_wgpu::gis_map_actions::OPEN_SOURCE,
                        json!({ "surfaceId": surface_id, "featureId": id }),
                    )),
                });
            }
        }
        return;
    }
    push_context_menu_item(ContextMenuItem {
        id: format!("{surface_id}.context.select-all"),
        label: "Select all".into(),
        action: Some(engine_canvas::map_action(
            controller_id,
            ui_wgpu::gis_map_actions::SELECT_ALL,
            json!({ "surfaceId": surface_id }),
        )),
    });
    let has_selection = engine_canvas::with_map_host(surface_id, |host| {
        !host.selected_positions_json().is_empty() || !host.selected_routes_json().is_empty()
    })
    .unwrap_or(false);
    push_context_menu_item(ContextMenuItem {
        id: format!("{surface_id}.context.clear"),
        label: "Clear selection".into(),
        action: if has_selection {
            Some(engine_canvas::map_action(
                controller_id,
                ui_wgpu::gis_map_actions::CLEAR_SELECTION,
                json!({ "surfaceId": surface_id }),
            ))
        } else {
            None
        },
    });
    push_context_menu_item(ContextMenuItem {
        id: format!("{surface_id}.context.fit-world"),
        label: "Fit world".into(),
        action: Some(engine_canvas::map_action(
            controller_id,
            ui_wgpu::gis_map_actions::FIT_WORLD,
            json!({ "surfaceId": surface_id }),
        )),
    });
}

pub fn gis_map_pointer_down(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    button: i16,
    shift: bool,
    ctrl_or_meta: bool,
    selection_method: &str,
) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if button == 0 {
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag {
                mode: SceneDragMode::MapMarquee {
                    start_x: sx as f32,
                    start_y: sy as f32,
                    method: selection_method.to_string(),
                    merge_mode: engine_canvas::map_marquee_mode(shift, ctrl_or_meta).to_string(),
                },
                button,
            });
            state.map_marquee_points = vec![(sx as f32, sy as f32)];
            state.map_marquee_active = false;
        });
        return Vec::new();
    }
    if button == 1 {
        engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, 1));
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag {
                mode: SceneDragMode::MapPan,
                button: 1,
            });
        });
        return engine_canvas::with_map_host_mut(surface_id, |host| {
            engine_canvas::map_interaction_actions(surface_id, controller_id, host)
        })
        .unwrap_or_default();
    }
    let _ = controller_id;
    Vec::new()
}

pub fn gis_map_pointer_move(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    down: bool,
) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if down {
        let state = scene_state(surface_id);
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::MapPan => {
                    engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_move_screen(sx, sy));
                    return engine_canvas::with_map_host_mut(surface_id, |host| {
                        engine_canvas::map_interaction_actions(surface_id, controller_id, host)
                    })
                    .unwrap_or_default();
                }
                SceneDragMode::MapMarquee {
                    start_x,
                    start_y,
                    method,
                    ..
                } => {
                    let distance =
                        ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(surface_id, |state| {
                        if distance >= MAP_MARQUEE_THRESHOLD_PX {
                            state.map_marquee_active = true;
                        }
                        if state.map_marquee_active {
                            if method == "lasso" {
                                if state.map_marquee_points.last().copied() != Some((sx as f32, sy as f32)) {
                                    state.map_marquee_points.push((sx as f32, sy as f32));
                                }
                            } else {
                                state.map_marquee_points =
                                    vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                            }
                        }
                    });
                }
                _ => {}
            }
        }
        return Vec::new();
    }
    let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy))
        .unwrap_or_else(|| "null".into());
    let hover = engine_canvas::parse_map_hover(&hit_json);
    let hover_json = if hover.is_null() {
        "null".into()
    } else {
        hover.to_string()
    };
    let prior = scene_state(surface_id).map_last_hover_json;
    if prior.as_deref() == Some(hover_json.as_str()) {
        return Vec::new();
    }
    mutate_scene_state(surface_id, |state| {
        state.map_last_hover_json = Some(hover_json.clone());
    });
    vec![engine_canvas::map_action(
        controller_id,
        ui_wgpu::gis_map_actions::SET_HOVER,
        json!({ "surfaceId": surface_id, "hover": hover }),
    )]
}

pub fn gis_map_pointer_up(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    let state = scene_state(surface_id);
    let Some(drag) = state.drag.clone() else {
        return Vec::new();
    };
    let mut actions = Vec::new();
    match drag.mode {
        SceneDragMode::MapPan => {
            engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_up_screen(sx, sy));
            actions.extend(
                engine_canvas::with_map_host_mut(surface_id, |host| {
                    engine_canvas::map_interaction_actions(surface_id, controller_id, host)
                })
                .unwrap_or_default(),
            );
        }
        SceneDragMode::MapMarquee {
            start_x,
            start_y,
            method,
            merge_mode,
        } => {
            let distance =
                ((sx as f32 - start_x).powi(2) + (sy as f32 - start_y).powi(2)).sqrt();
            if state.map_marquee_active && distance >= MAP_MARQUEE_THRESHOLD_PX {
                let mut points = state.map_marquee_points.clone();
                if method == "lasso" {
                    points.push((sx as f32, sy as f32));
                } else {
                    points = vec![(start_x, start_y), (sx as f32, sy as f32)];
                }
                let crossing = engine_canvas::map_marquee_crossing(&method, start_x, sx as f32);
                let (positions, routes) = engine_canvas::with_map_host(surface_id, |host| {
                    query_map_feature_hits(host, &method, &points, crossing)
                })
                .unwrap_or_default();
                actions.push(engine_canvas::map_action(
                    controller_id,
                    ui_wgpu::gis_map_actions::SET_FEATURE_SELECTION,
                    json!({
                        "surfaceId": surface_id,
                        "positions": positions,
                        "routes": routes,
                        "mode": merge_mode,
                    }),
                ));
            } else if distance < MAP_MARQUEE_THRESHOLD_PX {
                let hit_json = engine_canvas::with_map_host(surface_id, |host| {
                    host.hit_test_feature_json(sx, sy)
                })
                .unwrap_or_else(|| "null".into());
                let hit: Value = serde_json::from_str(&hit_json).unwrap_or(Value::Null);
                let (kind, id) = (
                    hit.get("kind").and_then(|value| value.as_str()),
                    hit.get("id").and_then(|value| value.as_str()),
                );
                if let (Some(kind), Some(id)) = (kind, id) {
                    actions.push(engine_canvas::map_action(
                        controller_id,
                        ui_wgpu::gis_map_actions::SET_FEATURE_SELECTION,
                        json!({
                            "surfaceId": surface_id,
                            "positions": if kind == "position" { vec![id] } else { Vec::<&str>::new() },
                            "routes": if kind == "route" { vec![id] } else { Vec::<&str>::new() },
                            "mode": merge_mode,
                        }),
                    ));
                }
            }
        }
        _ => {}
    }
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.map_marquee_points.clear();
        state.map_marquee_active = false;
    });
    actions
}

pub fn gis_map_drag_active(surface_id: &str) -> bool {
    scene_state(surface_id)
        .drag
        .as_ref()
        .is_some_and(|drag| matches!(drag.mode, SceneDragMode::MapMarquee { .. } | SceneDragMode::MapPan))
}

fn render_gis_map(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    gis_map_states: &mut HashMap<String, GisMapSurface>,
) {
    let Some(map_scene) = &scene.gis_map else {
        return render_placeholder("gis2d-map", bounds, ctx);
    };
    let inner = bounds;
    gis_map_states.insert(
        scene.surface_id.clone(),
        GisMapSurface {
            bounds: inner,
            controller_id: scene.controller_id.clone(),
            selection_method: map_scene.selection_method.clone(),
        },
    );
    engine_canvas::paint_gis_map(gpu, ctx, scene, inner);
    paint_gis_map_marquee(ctx, &scene.surface_id, inner, ctx.theme);
}
//#endregion GisMap

//#region IconRender
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderCameraFields {
    position: [f64; 3],
    target: [f64; 3],
    #[serde(default = "icon_render_default_zoom")]
    zoom: f64,
    #[serde(default)]
    fov: Option<f64>,
    #[serde(default)]
    up: Option<[f64; 3]>,
}

fn icon_render_default_zoom() -> f64 {
    1.0
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct IconRenderLightsFields {
    #[serde(default)]
    ambient_intensity: f64,
    #[serde(default)]
    ambient_color: Option<String>,
    #[serde(default)]
    sun_azimuth: f64,
    #[serde(default)]
    sun_elevation: f64,
    #[serde(default)]
    sun_intensity: f64,
    #[serde(default)]
    sun_color: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderMaterialFields {
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    metalness: Option<f64>,
    #[serde(default)]
    roughness: Option<f64>,
    #[serde(default)]
    emissive: Option<String>,
    #[serde(default)]
    emissive_intensity: Option<f64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderRequestFields {
    asset_url: String,
    camera: IconRenderCameraFields,
    #[serde(default)]
    lights: Option<IconRenderLightsFields>,
    width: f64,
    height: f64,
    #[serde(default)]
    shape: Option<String>,
    #[serde(default)]
    background: Option<String>,
    #[serde(default)]
    shadow_enabled: Option<bool>,
    #[serde(default)]
    material: Option<IconRenderMaterialFields>,
}

/** @emoji 🎥 Folds the request's three.js `zoom` into an equivalent vertical FOV, since the native orbit camera has no independent zoom factor, see https://threejs.org/docs/#api/en/cameras/PerspectiveCamera.zoom. */
fn icon_render_camera_json(camera: &IconRenderCameraFields) -> String {
    let fov = camera.fov.unwrap_or(50.0).max(1.0);
    let zoom = if camera.zoom.abs() > 1e-6 { camera.zoom } else { 1.0 };
    let effective_fov = if (zoom - 1.0).abs() > 1e-6 {
        let half = (fov * 0.5).to_radians();
        (2.0 * (half.tan() / zoom).atan()).to_degrees()
    } else {
        fov
    };
    let up = camera.up.unwrap_or([0.0, 0.0, 1.0]);
    json!({
        "position": camera.position,
        "target": camera.target,
        "up": up,
        "fov": effective_fov,
    })
    .to_string()
}

fn icon_render_environment_json(request: &IconRenderRequestFields) -> String {
    let lights = request.lights.clone().unwrap_or_default();
    let mut value = json!({
        "ambient": { "intensity": lights.ambient_intensity, "color": lights.ambient_color },
        "sun": {
            "azimuth": lights.sun_azimuth,
            "elevation": lights.sun_elevation,
            "intensity": lights.sun_intensity,
            "color": lights.sun_color,
        },
        "shadow": { "enabled": request.shadow_enabled.unwrap_or(false) },
    });
    if let Some(object) = value.as_object_mut() {
        if let Some(material) = &request.material {
            object.insert(
                "material".into(),
                json!({
                    "color": material.color,
                    "metalness": material.metalness,
                    "roughness": material.roughness,
                    "emissive": material.emissive,
                    "emissiveIntensity": material.emissive_intensity,
                }),
            );
        }
        if let Some(background) = &request.background {
            object.insert("background".into(), json!(background));
        }
    }
    value.to_string()
}

fn render_icon_render_empty(bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, message: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_body;
    let width = ctx.atlas.measure_text(message, size).0;
    draw_text(
        ctx,
        message,
        bounds.x + (bounds.w - width) * 0.5,
        bounds.y + bounds.h * 0.5,
        size,
        theme.text_muted,
    );
}

/** @emoji 🖼️ Native counterpart of framework/renderer/react/components/icon-render-host.tsx: reframes the request into a synthetic World3dScene and delegates the actual GLB draw to infinite_world::render_world_3d, then paints the aspect-fit frame/badge/footer chrome on top. */
fn render_icon_render(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    icon_render_states: &mut HashMap<String, World3dState>,
) {
    let Some(icon_render) = &scene.icon_render else {
        return render_icon_render_empty(bounds, ctx, "No shot");
    };
    let Ok(request) = serde_json::from_str::<IconRenderRequestFields>(&icon_render.request_json) else {
        return render_icon_render_empty(bounds, ctx, "No shot");
    };

    let theme = ctx.theme;
    let shape = request.shape.clone().unwrap_or_else(|| "rectangle".into());
    let width = request.width.max(1.0) as f32;
    let height = request.height.max(1.0) as f32;
    let fit_scale = (bounds.w / width).min(bounds.h / height).max(0.01);
    let frame_w = width * fit_scale;
    let frame_h = height * fit_scale;
    let frame = Rect::new(
        bounds.x + (bounds.w - frame_w) * 0.5,
        bounds.y + (bounds.h - frame_h) * 0.5,
        frame_w,
        frame_h,
    );

    let mesh_id = semio_framework_plugin::world3d_mesh_id_from_url(&request.asset_url);
    let instances_json = json!([{
        "id": "icon-render-subject",
        "meshId": mesh_id,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
    }])
    .to_string();
    let mut synthetic_world = semio_framework_plugin::world3d_scene(
        icon_render_camera_json(&request.camera),
        semio_framework_plugin::world3d_meshes_json_from_urls(std::slice::from_ref(&request.asset_url)),
        instances_json,
        ui_wgpu::world3d_default_selection_json(),
        &semio_framework_plugin::WorldSunConfig::default(),
    );
    synthetic_world.environment_json = Some(icon_render_environment_json(&request));

    let synthetic_scene = UiComponentSceneNode {
        surface_id: scene.surface_id.clone(),
        controller_id: scene.controller_id.clone(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(synthetic_world),
        node_graph: None,
        text_editor: None,
        table: None,
        raster: None,
        virtual_file_system: None,
        gis_map: None,
        puzzle2d_board: None,
        icon_render: None,
        note_canvas: None,
        vcs_history: None,
        protocol_list: None,
    };

    let state = icon_render_states
        .entry(scene.surface_id.clone())
        .or_insert_with(|| World3dState::new(scene.surface_id.clone(), scene.controller_id.clone()));
    render_world_3d(&synthetic_scene, frame, ctx, state, gpu);

    let hair = theme.stroke_hairline.max(1.0);
    ctx.draw.push_solid([frame.x, frame.y, frame.w, hair], theme.accent);
    ctx.draw
        .push_solid([frame.x, frame.y + frame.h - hair, frame.w, hair], theme.accent);
    ctx.draw.push_solid([frame.x, frame.y, hair, frame.h], theme.accent);
    ctx.draw
        .push_solid([frame.x + frame.w - hair, frame.y, hair, frame.h], theme.accent);

    let badge = format!("{}×{} · {}", request.width.round() as i64, request.height.round() as i64, shape);
    let badge_size = theme.font_size_small;
    let (badge_text_w, badge_text_h) = ctx.atlas.measure_text(&badge, badge_size);
    let pad = 4.0;
    let badge_w = badge_text_w + pad * 2.0;
    let badge_h = badge_text_h + pad * 2.0;
    let badge_x = frame.x + frame.w - badge_w - 4.0;
    let badge_y = frame.y + frame.h - badge_h - 4.0;
    ctx.draw
        .push_rounded([badge_x, badge_y, badge_w, badge_h], theme.panel.with_alpha(0.8), 2.0);
    draw_text(ctx, &badge, badge_x + pad, badge_y + pad + badge_text_h * 0.8, badge_size, theme.text_muted);

    if let Some(footer) = &icon_render.footer {
        let footer_size = theme.font_size_small;
        let footer_w = ctx.atlas.measure_text(footer, footer_size).0;
        draw_text(
            ctx,
            footer,
            bounds.x + (bounds.w - footer_w) * 0.5,
            bounds.y + bounds.h - 8.0,
            footer_size,
            theme.text_muted,
        );
    }
}
//#endregion IconRender

//#region Puzzle2dBoard
pub struct Puzzle2dBoardSurface {
    pub bounds: Rect,
    pub controller_id: String,
    pub fixture_json: String,
}

fn render_puzzle_board(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    puzzle2d_board_states: &mut HashMap<String, Puzzle2dBoardSurface>,
) {
    let Some(board_scene) = &scene.puzzle2d_board else {
        return render_placeholder("puzzle2d-board", bounds, ctx);
    };
    let inner = bounds;
    puzzle2d_board_states.insert(
        scene.surface_id.clone(),
        Puzzle2dBoardSurface {
            bounds: inner,
            controller_id: scene.controller_id.clone(),
            fixture_json: board_scene.fixture_json.clone(),
        },
    );
    engine_canvas::paint_puzzle_board(gpu, ctx, scene, inner);
}

pub fn puzzle_board_pointer_down(surface_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool) {
    engine_canvas::puzzle_board_pointer_down(surface_id, inner, x, y, button, shift, ctrl_or_meta);
}

pub fn puzzle_board_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_move(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt)
}

pub fn puzzle_board_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_up(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt)
}

pub fn puzzle_board_pointer_leave(surface_id: &str, controller_id: &str, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_leave(surface_id, controller_id, alt)
}

pub fn puzzle_board_drag_active(surface_id: &str) -> bool {
    engine_canvas::board_drag_active(surface_id)
}

pub fn puzzle_board_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_wheel(surface_id, controller_id, inner, x, y, delta)
}

//#region Puzzle2dSelectionMenu
pub struct Puzzle2dSelectionMenuItem {
    pub id: String,
    pub label: String,
    pub action: String,
    pub args: Option<Value>,
    pub disabled: bool,
}

fn puzzle2d_entity_flag(entity: &Value, key: &str) -> bool {
    entity.get(key).and_then(Value::as_bool).unwrap_or(false)
}

/// @emoji 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors `buildPuzzle2dSelectionMenuItems` in the React host.
pub fn build_puzzle2d_selection_menu_items(fixture_json: &str, selection_ids: &[String]) -> Vec<Puzzle2dSelectionMenuItem> {
    let fixture: Value = serde_json::from_str(fixture_json).unwrap_or(Value::Null);
    if selection_ids.is_empty() {
        return vec![Puzzle2dSelectionMenuItem { id: "selectAll".into(), label: "Select all".into(), action: "selectAll".into(), args: None, disabled: false }];
    }
    let selected: HashSet<&str> = selection_ids.iter().map(String::as_str).collect();
    let nodes = fixture.get("nodes").and_then(Value::as_array).cloned().unwrap_or_default();
    let edges = fixture.get("edges").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut selected_entities: Vec<Value> = Vec::new();
    let mut has_selected_node = false;
    for node in &nodes {
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(node.clone());
                has_selected_node = true;
            }
        }
        if let Some(handles) = node.get("handles").and_then(Value::as_array) {
            for handle in handles {
                if let Some(id) = handle.get("id").and_then(Value::as_str) {
                    if selected.contains(id) {
                        selected_entities.push(handle.clone());
                    }
                }
            }
        }
    }
    for edge in &edges {
        if let Some(id) = edge.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(edge.clone());
            }
        }
    }
    let any_visible = selected_entities.iter().any(|entity| !puzzle2d_entity_flag(entity, "hidden"));
    let any_unlocked = selected_entities.iter().any(|entity| !puzzle2d_entity_flag(entity, "locked"));
    vec![
        Puzzle2dSelectionMenuItem {
            id: "toggleHidden".into(),
            label: (if any_visible { "Hide" } else { "Show" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "hidden", "value": any_visible })),
            disabled: false,
        },
        Puzzle2dSelectionMenuItem {
            id: "toggleLocked".into(),
            label: (if any_unlocked { "Lock" } else { "Unlock" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "locked", "value": any_unlocked })),
            disabled: false,
        },
        Puzzle2dSelectionMenuItem { id: "duplicate".into(), label: "Duplicate".into(), action: "duplicateSelection".into(), args: None, disabled: !has_selected_node },
        Puzzle2dSelectionMenuItem { id: "selectSameKind".into(), label: "Select all of same kind".into(), action: "selectSameKind".into(), args: None, disabled: false },
        Puzzle2dSelectionMenuItem { id: "focusSelection".into(), label: "Zoom to selection".into(), action: "focusSelection".into(), args: None, disabled: false },
        Puzzle2dSelectionMenuItem { id: "deleteSelection".into(), label: "Delete".into(), action: "deleteSelection".into(), args: None, disabled: false },
    ]
}
//#endregion Puzzle2dSelectionMenu

/// @emoji 🧩 Pushes puzzle2d-board context-menu items for a screen-space hit, eagerly selecting the clicked target if it isn't already selected (mirrors the React host's `onContextMenu`).
pub async fn open_puzzle2d_board_context_menu(shell: &mut ShellState, surface_id: &str, controller_id: &str, fixture_json: &str, inner: Rect, x: f32, y: f32) {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    let best = engine_canvas::board_pick_best_target_id(surface_id, sx, sy);
    let current: Vec<String> = engine_canvas::with_board_host(surface_id, |host| host.selection.iter().cloned().collect()).unwrap_or_default();
    let mut effective = current.clone();
    if let Some(id) = &best {
        if !effective.contains(id) {
            effective = vec![id.clone()];
            engine_canvas::with_board_host_mut(surface_id, |host| host.set_selection_ids_silent(std::slice::from_ref(id)));
            let _ = shell
                .dispatch_action(engine_canvas::board_action(controller_id, "setSelection", json!({ "ids": effective })))
                .await;
        }
    }
    for item in build_puzzle2d_selection_menu_items(fixture_json, &effective) {
        push_context_menu_item(ContextMenuItem {
            id: format!("{surface_id}.context.{}", item.id),
            label: item.label,
            action: if item.disabled { None } else { Some(engine_canvas::board_action(controller_id, &item.action, item.args.unwrap_or(Value::Null))) },
        });
    }
}
//#endregion Puzzle2dBoard

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

fn vfs_double_click_action(scene: &UiComponentSceneNode, row: &Value) -> Option<ActionDescriptor> {
    let uri = row.get("navigateUri").and_then(|v| v.as_str())?;
    if uri.starts_with("os://instance/") {
        return Some(scene_action(
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
            return Some(scene_action(
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
        return Some(scene_action(
            scene,
            "navigateVirtualFileSystemNode",
            json!({ "surfaceId": scene.surface_id, "studioId": studio_id }),
        ));
    }
    if let Some(studio_id) = uri.strip_prefix("studio:") {
        return Some(scene_action(
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
                    ctx.input.queue_event(scene_action(
                        scene,
                        "submit",
                        json!({ "surfaceId": scene.surface_id, "document": editor.buffer }),
                    ));
                }
                KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("s") => {
                    ctx.input.queue_event(scene_action(
                        scene,
                        "formatDocument",
                        json!({ "surfaceId": scene.surface_id }),
                    ));
                }
                KeyAction::Enter | KeyAction::Escape => {
                    ctx.input.queue_event(scene_action(
                        scene,
                        "textEdit",
                        json!({ "surfaceId": scene.surface_id, "document": editor.buffer }),
                    ));
                    if matches!(key, KeyAction::Escape) {
                        ctx.input.blur_input();
                    }
                }
                KeyAction::Char(_) | KeyAction::Backspace | KeyAction::Delete => {
                    for action in engine_canvas::text_editor_apply_key(scene, key, &modifiers) {
                        ctx.input.queue_event(action);
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
//#endregion scenes
}

pub mod shell {
// #region shell
//! 🖥️ OS shell chrome — navbar, footer, floating panels, overlays, and studio mode.

use crate::dock::{
    compute_dock_drop_zone, dock_from_window_layout, drop_zone_indicator_rect, parse_path,
    DockDragKind, DockDragPayload, DockDragState, DockDropZone, DockRenderContext, DockState,
};
use crate::interpreter::{framework_widget_context, render_ui_node, validate_window_body_surface};
use crate::scenes::{
    clear_graph_node_context, open_puzzle2d_board_context_menu, push_gis_map_context_menu, resolve_graph_context_action, seed_vfs_expanded, toggle_vfs_row_expanded, vfs_selection_for_click, GisMapSurface, NodeGraphSurface,
    Puzzle2dBoardSurface,
};
use infinite_world::{
    fetch_pending_glb_meshes, fetch_pending_reference_images, handle_world3d_paint_actions,
    handle_world3d_pointer_button,
    handle_world3d_pointer_drag, handle_world3d_pointer_move, handle_world3d_wheel, World3dState,
};
use crate::plugin_bridge::{is_studio_mode, PluginBridgeEntry};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_sync::{
    DocumentActorMsg, DocumentEvent, DocumentHost, DocumentSyncStatus, PersistenceBinding, RemoteState,
};
use semio_framework_core::{
    app_document_label, app_window_document_label, AppDefinition, ExampleDefinition,
    ModeDefinition, PanelGroup, PanelTabDefinition, ViewState,
};
use ui_wgpu::component::layout::WindowEngagementPossible;
use ui_wgpu::{
    ActionDescriptor, ToolCategory, ToolNode, UiButtonNode, UiNode, UiSelectItem, UiSelectNode, UiStackNode,
    UiTextNode, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption,
    WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
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
    pub dispatch_action: Option<ActionDescriptor>,
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
    pub document: Vec<String>,
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
    pub document: Vec<String>,
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
    pub action: Option<ActionDescriptor>,
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

//#region 🔖NativeSyncChannel
/// @emoji 🧵 One open document's live `framework/sync` actor channel held by the native wgpu shell.
/// Mirrors `os-shell.tsx`'s `openDocumentSessionsRef` entry: the shell owns the `cmd_tx`/event
/// receiver while the sandboxed plugin instance's store pumps through the registered
/// `ChannelBackbone` (see `framework/product/os/core/rs`'s `host_runtime` canonical sequence).
#[cfg(not(target_arch = "wasm32"))]
pub struct ShellSyncChannel {
    pub document_id: String,
    pub actor_uri: String,
    pub instance_id: u32,
    pub plugin_id: String,
    pub cmd_tx: tokio::sync::mpsc::UnboundedSender<DocumentActorMsg>,
    pub events: tokio::sync::broadcast::Receiver<DocumentEvent>,
}
//#endregion 🔖NativeSyncChannel

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
    pub appearance_id: String,
    pub locale_id: String,
    pub terminology_id: String,
    pub right_click: RightClickState,
    pub uri_history: Vec<String>,
    pub uri_index: usize,
    pub open_studio_id: Option<String>,
    pub pending_shell_uri_apply: bool,
    pub panel_resize_origin_width: f32,
    pub error: Option<String>,
    pub screen_w: f32,
    pub screen_h: f32,
    pub world3d_states: HashMap<String, World3dState>,
    pub node_graph_states: HashMap<String, NodeGraphSurface>,
    pub gis_map_states: HashMap<String, GisMapSurface>,
    pub icon_render_states: HashMap<String, World3dState>,
    pub puzzle2d_board_states: HashMap<String, Puzzle2dBoardSurface>,
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
    pub widget_maps: WidgetInteractionMaps<ActionDescriptor>,
    pub pending_tree_drag: Option<(String, HashMap<String, String>)>,
    pub tree_drag_origin: (f32, f32),
    pub dock_drag: Option<DockDragState>,
    pub pending_dock_drag: Option<(DockDragPayload, (f32, f32))>,
    pub dock_drag_snapshot: Option<ui_wgpu::WindowLayout>,
    pub dock_canvas_bounds: Rect,
    pub dock_drop_tab_bars: Vec<(Vec<usize>, Rect, Vec<f32>)>,
    pub dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    pub layout_override: Option<ui_wgpu::WindowLayout>,
    pub split_resize_origin: Vec<f32>,
    pub split_resize_secondary_path: Option<Vec<usize>>,
    pub split_resize_secondary_index: usize,
    pub split_resize_secondary_axis_total: f32,
    pub split_resize_secondary_origin: Vec<f32>,
    pub measures_resize_window_id: Option<String>,
    pub deferred_actions: Vec<ActionDescriptor>,
    pub active_tools: Vec<ToolNode>,
    /// @emoji 🧰 Host-owned active tool per window kind (never a document field, never a VCS op).
    /// Replaces the deleted `active_tool_id`/`find_active_tool_id` "first pressed toggle" heuristic.
    pub active_tool_by_window: HashMap<String, String>,
    /// @emoji 📇 Per-window Actions-rail fold state (absent = folded, the default).
    pub action_panel_folded: HashMap<String, bool>,
    /// @emoji 📇 Per-window expanded action id (the accordion-open staged arg form).
    pub action_panel_expanded: HashMap<String, String>,
    /// @emoji 📝 Staged action argument values keyed `"{window_id}:{action_id}"` — edits buffer here
    /// and never dispatch until Execute (Architecture Decision 8, P2).
    pub staged_action_args: HashMap<String, serde_json::Map<String, serde_json::Value>>,
    pub sync_backbone_uri: Option<String>,
    pub sync_card_kind: Option<String>,
    pub sync_card_draft: String,
    pub sync_card_anchor: Option<(f32, f32)>,
    pub last_envelope_json: Option<String>,
    /// @emoji 🏛️ Shell-lifetime document-host actor registry (native only); the browser wgpu build
    /// has no native `DocumentHost` — its sync flows through the React shell's `backbone-worker.ts`.
    #[cfg(not(target_arch = "wasm32"))]
    pub document_host: DocumentHost,
    /// @emoji 🧵 The currently attached document's live actor channel (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub sync_channel: Option<ShellSyncChannel>,
    /// @emoji 🚦 Latest sync health for the active document's status badge (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub sync_status: Option<DocumentSyncStatus>,
    pub window_engagements: HashMap<String, WindowEngagement>,
    pub window_measures: HashMap<String, Vec<WindowMeasure>>,
    pub tool_collection_expanded: HashMap<String, bool>,
    pub contributor_instances: HashMap<String, u32>,
    /// 🖱️ Last-rendered window body rects per window id — used to apply the active tool's cursor while
    /// the pointer is over that window's content (Architecture Decision 8, P5).
    pub window_content_rects: HashMap<String, Rect>,
}
//#endregion ShellTypes

async fn resolve_external_slots_in_tree(
    node: UiNode,
    plugins: &[PluginBridgeEntry],
    contributor_instances: &mut HashMap<String, u32>,
    view_state: &ViewState,
) -> Result<UiNode, String> {
    match node {
        UiNode::ExternalSlot(slot) => {
            let plugin = plugins
                .iter()
                .find(|entry| entry.plugin_id == slot.plugin_id)
                .cloned()
                .ok_or_else(|| format!("contributor plugin missing: {}", slot.plugin_id))?;
            let instance_id = if let Some(id) = contributor_instances.get(&slot.plugin_id) {
                *id
            } else {
                let id = plugin.create_app(&slot.app_id).await?;
                contributor_instances.insert(slot.plugin_id.clone(), id);
                id
            };
            let rendered = plugin
                .render_with_document(
                    instance_id,
                    &slot.body_key,
                    view_state,
                    Some(slot.params_json.as_str()),
                )
                .await?;
            Box::pin(resolve_external_slots_in_tree(
                rendered,
                plugins,
                contributor_instances,
                view_state,
            ))
            .await
        }
        UiNode::Stack(mut stack) => {
            let mut children = Vec::with_capacity(stack.children.len());
            for child in stack.children {
                children.push(
                    Box::pin(resolve_external_slots_in_tree(
                        child,
                        plugins,
                        contributor_instances,
                        view_state,
                    ))
                    .await?,
                );
            }
            stack.children = children;
            Ok(UiNode::Stack(stack))
        }
        UiNode::Section(mut section) => {
            let mut children = Vec::with_capacity(section.children.len());
            for child in section.children {
                children.push(
                    Box::pin(resolve_external_slots_in_tree(
                        child,
                        plugins,
                        contributor_instances,
                        view_state,
                    ))
                    .await?,
                );
            }
            section.children = children;
            Ok(UiNode::Section(section))
        }
        other => Ok(other),
    }
}

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
            left_panel_open: false,
            right_panel_open: false,
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
            appearance_id: "system".into(),
            locale_id: "en".into(),
            terminology_id: "native".into(),
            right_click: RightClickState::default(),
            uri_history: vec!["/".into()],
            uri_index: 0,
            open_studio_id: None,
            pending_shell_uri_apply: false,
            panel_resize_origin_width: 280.0,
            error: None,
            screen_w: 1280.0,
            screen_h: 720.0,
            world3d_states: HashMap::new(),
            node_graph_states: HashMap::new(),
            gis_map_states: HashMap::new(),
            icon_render_states: HashMap::new(),
            puzzle2d_board_states: HashMap::new(),
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
            measures_resize_origin_width: 0.0,
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
            deferred_actions: Vec::new(),
            active_tools: Vec::new(),
            active_tool_by_window: HashMap::new(),
            action_panel_folded: HashMap::new(),
            action_panel_expanded: HashMap::new(),
            staged_action_args: HashMap::new(),
            sync_backbone_uri: None,
            sync_card_kind: None,
            sync_card_draft: String::new(),
            sync_card_anchor: None,
            last_envelope_json: None,
            #[cfg(not(target_arch = "wasm32"))]
            document_host: DocumentHost::new(),
            #[cfg(not(target_arch = "wasm32"))]
            sync_channel: None,
            #[cfg(not(target_arch = "wasm32"))]
            sync_status: None,
            window_engagements: HashMap::new(),
            window_measures: HashMap::new(),
            tool_collection_expanded: HashMap::new(),
            contributor_instances: HashMap::new(),
            window_content_rects: HashMap::new(),
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
                    document: program.document.clone(),
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

    pub fn prepare_hot_reload(&mut self, plugins: Vec<PluginBridgeEntry>) {
        if let Some(session) = self.session.take() {
            if let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                plugin.destroy_app(session.instance_id);
            }
        }
        self.plugins = plugins;
    }

    pub async fn hot_reload_plugins(&mut self, plugins: Vec<PluginBridgeEntry>) -> Result<(), String> {
        self.prepare_hot_reload(plugins);
        self.boot().await
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
                active_mode_id: Some(s_app.default_mode_id.clone()),
                active_window_kind_id: Some(s_app.window_kinds.first().id.clone()),
                active_tool_id: None,
                selection_json: None,
                panel_json: Some(Self::panel_json(&panel_state)),
                contributions_json: None,
                locale: Some(self.locale_id.clone()),
                terminology: Some(self.terminology_id.clone()),
            };
            self.active_window_id = Some(s_app.window_kinds.first().id.clone());
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
            self.active_window_id = Some(app.window_kinds.first().id.clone());
            self.session = Some(ActiveSession {
                plugin_id: plugin.plugin_id.clone(),
                instance_id,
                app: app.clone(),
                view_state: ViewState {
                    active_mode_id: Some(app.default_mode_id.clone()),
                    active_window_kind_id: self.active_window_id.clone(),
                    active_tool_id: None,
                    selection_json: None,
                    panel_json: None,
                    contributions_json: None,
                    locale: Some(self.locale_id.clone()),
                    terminology: Some(self.terminology_id.clone()),
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
            .map(|p| {
                p.manifest
                    .examples
                    .iter()
                    .filter(|example| example.app_id.is_empty() || example.app_id == session.app.id)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    fn flatten_panel_tab_leaves(tabs: &[PanelTabDefinition]) -> Vec<&PanelTabDefinition> {
        tabs.iter()
            .flat_map(|tab| {
                if tab.children.is_empty() {
                    vec![tab]
                } else {
                    Self::flatten_panel_tab_leaves(&tab.children)
                }
            })
            .collect()
    }

    fn synthetic_panel_tab(id: &str, label: &str, group: PanelGroup) -> PanelTabDefinition {
        PanelTabDefinition {
            kind: semio_framework_core::PanelTabKind::App(id.into()),
            label: label.into(),
            group,
            body_key: Some(String::new()),
            children: Vec::new(),
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

    fn contributions_json_from_plugins(plugins: &[PluginBridgeEntry]) -> String {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PluginContributionEntry<'a> {
            plugin_id: &'a str,
            contribution: &'a semio_framework_core::Contribution,
        }
        let entries: Vec<PluginContributionEntry<'_>> = plugins
            .iter()
            .flat_map(|plugin| {
                plugin
                    .manifest
                    .contributions
                    .iter()
                    .map(|contribution| PluginContributionEntry {
                        plugin_id: plugin.plugin_id.as_str(),
                        contribution,
                    })
            })
            .collect();
        serde_json::to_string(&entries).unwrap_or_else(|_| "[]".into())
    }

    async fn resolve_external_slots(
        &mut self,
        node: UiNode,
        view_state: &ViewState,
    ) -> Result<UiNode, String> {
        let plugins = self.plugins.clone();
        resolve_external_slots_in_tree(node, &plugins, &mut self.contributor_instances, view_state).await
    }

    pub async fn refresh_ui(&mut self) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        self.sync_dock();
        self.window_ui.clear();
        let mut view_state = session.view_state.clone();
        view_state.contributions_json = Some(Self::contributions_json_from_plugins(&self.plugins));
        {
            let plugin = self
                .plugins
                .iter()
                .find(|p| p.plugin_id == session.plugin_id)
                .cloned()
                .ok_or("session plugin missing")?;
            for kind in &session.app.window_kinds {
                // 🧰 Inject the host-owned active tool for this window kind so the plugin renders its
                // live-preview overlay for the right tool (Architecture Decision 4).
                view_state.active_tool_id = self.active_tool_by_window.get(&kind.id).cloned();
                let node = plugin
                    .render(session.instance_id, &kind.body_key, &view_state)
                    .await?;
                let resolved = self
                    .resolve_external_slots(node, &view_state)
                    .await?;
                let ui = match validate_window_body_surface(kind, &resolved) {
                    Ok(()) => resolved,
                    Err(message) => UiNode::Text(UiTextNode {
                        value: format!("Framework rejected render plan: {message}"),
                        emphasize: Some(true),
                        data_attributes: None,
                    }),
                };
                self.window_ui.insert(kind.id.clone(), ui);
            }
        }
        self.panel_ui.clear();
        self.ensure_framework_panel_ui(&session);
        let plugin = self
            .plugins
            .iter()
            .find(|p| p.plugin_id == session.plugin_id)
            .cloned()
            .ok_or("session plugin missing")?;
        for tab in Self::flatten_panel_tab_leaves(&session.app.panel_tabs) {
            let body_key = tab.body_key.as_deref().unwrap_or_default();
            let node = plugin
                .render(session.instance_id, body_key, &view_state)
                .await?;
            let resolved = self.resolve_external_slots(node, &view_state).await?;
            self.panel_ui.insert(tab.id().to_string(), resolved);
        }
        // 🧰 The toolbar is derived from the app's declared `AppDefinition.tools` (scoped to the active
        // window kind) via `ui_wgpu::derive_tool_nodes` — the old per-call `plugin.tools()` fetch and the
        // `find_active_tool_id` "first pressed toggle" heuristic are gone (Architecture Decision 5).
        self.active_tools = self.derive_toolbar_nodes(&session);
        self.active_tools.extend(framework_sync_tools(self.sync_backbone_uri.as_deref()));
        self.window_engagements = plugin
            .window_engagements(session.instance_id, &view_state)
            .await
            .unwrap_or_default();
        self.window_measures = plugin
            .window_measures(session.instance_id, &view_state)
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
                            let body_key = app.window_kinds.first().body_key.clone();
                            let view_state = ViewState {
                                active_mode_id: Some(app.default_mode_id.clone()),
                                active_window_kind_id: Some(app.window_kinds.first().id.clone()),
                                active_tool_id: None,
                                selection_json: None,
                                panel_json: None,
                                contributions_json: None,
                                locale: Some(self.locale_id.clone()),
                                terminology: Some(self.terminology_id.clone()),
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
            id: None,
            selected: None,
            activate: None,
            drop_action: None,
            loading: None,
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
                    action: ActionDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        action: "noop".into(),
                        args: None,
                    },
                    style: None,
                    disabled: None,
                    loading: None,
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
            id: None,
            selected: None,
            activate: None,
            drop_action: None,
            loading: None,
        })
    }

    fn build_settings_general_ui(&self) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: None,
            children: vec![
                UiNode::Text(UiTextNode {
                    value: "General".into(),
                    emphasize: Some(true),
                    data_attributes: None,
                }),
                UiNode::Select(UiSelectNode {
                    id: "framework.settings.appearance".into(),
                    value: self.appearance_id.clone(),
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
                    on_change: ActionDescriptor {
                        controller_id: "framework".into(),
                        action: "setAppearance".into(),
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
                    on_change: ActionDescriptor {
                        controller_id: "framework".into(),
                        action: "setExpertise".into(),
                        args: None,
                    },
                }),
                UiNode::Select(UiSelectNode {
                    id: "framework.settings.language".into(),
                    value: self.locale_id.clone(),
                    items: vec![
                        UiSelectItem {
                            value: "en".into(),
                            label: "English".into(),
                        },
                        UiSelectItem {
                            value: "de".into(),
                            label: "Deutsch".into(),
                        },
                    ],
                    placeholder: None,
                    on_change: ActionDescriptor {
                        controller_id: "framework".into(),
                        action: "setLocale".into(),
                        args: None,
                    },
                }),
                UiNode::Select(UiSelectNode {
                    id: "framework.settings.terminology".into(),
                    value: self.terminology_id.clone(),
                    items: self
                        .active_terminologies()
                        .into_iter()
                        .map(|id| UiSelectItem {
                            label: if id == "native" { "Native".into() } else { id.clone() },
                            value: id,
                        })
                        .collect(),
                    placeholder: None,
                    on_change: ActionDescriptor {
                        controller_id: "framework".into(),
                        action: "setTerminology".into(),
                        args: None,
                    },
                }),
            ],
            selected: None,
            activate: None,
            drop_action: None,
            loading: None,
        })
    }

    fn active_terminologies(&self) -> Vec<String> {
        let mut ids = vec!["native".to_string()];
        if let Some(session) = self.session.as_ref() {
            for id in &session.app.terminologies {
                if !ids.contains(id) {
                    ids.push(id.clone());
                }
            }
        }
        ids
    }
}
//#endregion ShellLifecycle

//#region ShellActions
fn patch_ops_from_action_result(result: &semio_framework_core::kernel::ActionResult) -> Vec<String> {
    result
        .operations
        .iter()
        .filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok())
        .collect()
}

impl ShellState {
    fn sync_document_id(&self) -> Option<String> {
        let session = self.session.as_ref()?;
        Some(format!("{}-{}", session.plugin_id, session.instance_id))
    }

    //#region 🔖NativeBackboneSync
    /// @emoji 🧭 Parses a shell sync-card uri into the `framework/sync` persistence bindings a
    /// document actor opens. `folder://` → the multi-document sqlite store; `file://x.json` → its
    /// parent folder's store (single-blob export demoted per the plan); `remote://host:port` → the
    /// hub over WebSocket. Superseded the fetch/CRUD `shell_backbone_read`/`write` pair.
    #[cfg(not(target_arch = "wasm32"))]
    fn parse_persistence_binding(uri: &str) -> Result<Vec<PersistenceBinding>, String> {
        if let Some(rest) = uri.strip_prefix("remote://") {
            let host_port = rest.split_once('/').map(|(host, _)| host).unwrap_or(rest);
            return Ok(vec![PersistenceBinding::Hub {
                base_url: format!("http://{host_port}"),
                token: None,
            }]);
        }
        if let Some(path) = uri.strip_prefix("folder://") {
            return Ok(vec![PersistenceBinding::Folder { path: std::path::PathBuf::from(path) }]);
        }
        if let Some(path) = uri.strip_prefix("file://") {
            let parent = std::path::Path::new(path)
                .parent()
                .map(std::path::Path::to_path_buf)
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            return Ok(vec![PersistenceBinding::Folder { path: parent }]);
        }
        Err(format!("unsupported backbone uri: {uri}"))
    }

    /// @emoji ✂️ Tears down the active document channel: detaches the plugin's backbone, deregisters
    /// the host channel end, and stops the actor (flushing pending outbound ops). Step 7 of the
    /// `host_runtime` canonical sequence.
    #[cfg(not(target_arch = "wasm32"))]
    fn detach_sync_backbone_internal(&mut self) {
        if let Some(channel) = self.sync_channel.take() {
            let _ = channel.cmd_tx.send(DocumentActorMsg::Detach);
            if let Some(runtime) = self
                .plugins
                .iter()
                .find(|entry| entry.plugin_id == channel.plugin_id)
                .and_then(|entry| entry.wasm_runtime())
            {
                let _ = runtime.detach_backbone(channel.instance_id);
                let _ = runtime.deregister_host_backbone(&channel.actor_uri);
            }
            self.document_host.close(&channel.document_id);
        }
        self.sync_status = None;
    }

    /// @emoji 📬 Drains the active document actor's event stream into the plugin store and the sync
    /// badge. Called once per native frame — the render loop already redraws continuously (winit
    /// `ControlFlow::Poll`), so a `try_recv` poll suffices and no `EventLoopProxy` wake is needed.
    /// `RemoteOps` are force-applied via `apply_operations` (idempotent by op id), which also covers
    /// idle frames where the sandboxed store never pumps its `ChannelBackbone` on its own. Returns
    /// whether anything changed (and a re-render was issued).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn pump_sync_events(&mut self) -> bool {
        use tokio::sync::broadcast::error::TryRecvError;
        let (instance_id, plugin_id, events) = {
            let Some(channel) = self.sync_channel.as_mut() else {
                return false;
            };
            let mut events: Vec<DocumentEvent> = Vec::new();
            loop {
                match channel.events.try_recv() {
                    Ok(event) => events.push(event),
                    Err(TryRecvError::Empty) | Err(TryRecvError::Closed) => break,
                    Err(TryRecvError::Lagged(_)) => continue,
                }
            }
            if events.is_empty() {
                return false;
            }
            (channel.instance_id, channel.plugin_id.clone(), events)
        };
        let runtime = self
            .plugins
            .iter()
            .find(|entry| entry.plugin_id == plugin_id)
            .and_then(|entry| entry.wasm_runtime());
        let mut changed = false;
        for event in events {
            match event {
                DocumentEvent::RemoteOps { envelopes } => {
                    if let (Some(runtime), Ok(json)) = (runtime.as_ref(), serde_json::to_string(&envelopes)) {
                        match runtime.apply_operations(instance_id, &json) {
                            Ok(()) => changed = true,
                            Err(error) => eprintln!("[DEBUG] wgpu shell apply_operations failed: {error}"),
                        }
                    }
                }
                DocumentEvent::SnapshotReplaced { envelope_json } => {
                    if let Some(runtime) = runtime.as_ref() {
                        match runtime.load_app_document(instance_id, &envelope_json) {
                            Ok(()) => changed = true,
                            Err(error) => eprintln!("[DEBUG] wgpu shell load_app_document failed: {error}"),
                        }
                    }
                }
                DocumentEvent::Status(status) => {
                    self.sync_status = Some(status);
                    changed = true;
                }
                DocumentEvent::Presence { .. } => {
                    // 👥 The Rust `semio_framework_core::ViewState` has no presence field yet (only the
                    // TS shell threads `presencePeersJson`); presence roster display in the native
                    // wgpu shell is a documented follow-up once core `ViewState` carries it.
                }
                DocumentEvent::Conflict(_) => {
                    self.sync_card_kind = Some("conflict".into());
                    changed = true;
                }
            }
        }
        if changed {
            let _ = self.refresh_ui().await;
        }
        changed
    }

    /// @emoji 🚦 Human-readable summary of a document's sync health for the attach card, mirroring
    /// the React shell's `syncStatusLabel`.
    #[cfg(not(target_arch = "wasm32"))]
    fn sync_status_label(status: &DocumentSyncStatus) -> String {
        let remote = match &status.remote {
            RemoteState::Live { peer_count } => {
                format!("live · {peer_count} peer{}", if *peer_count == 1 { "" } else { "s" })
            }
            RemoteState::Connecting => "connecting…".to_string(),
            RemoteState::Backoff { .. } => "reconnecting…".to_string(),
            RemoteState::Detached => "offline".to_string(),
        };
        let persisted = if status.persisted { "saved" } else { "unsaved" };
        let pending = if status.pending_ops > 0 {
            format!(" · {} pending", status.pending_ops)
        } else {
            String::new()
        };
        format!("{remote} · {persisted}{pending}")
    }
    //#endregion 🔖NativeBackboneSync

    /// @emoji 🔗 Opens the shell's active app document on a `framework/sync` `DocumentHost` actor and
    /// wires the sandboxed plugin store to it, following `framework/product/os/core/rs`'s
    /// `host_runtime` canonical sequence (open → subscribe → register host channel → plugin
    /// `attach-backbone`). The React shell's `openDocument` is the TS twin of this exact sequence.
    async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let runtime = self
                .plugins
                .iter()
                .find(|entry| entry.plugin_id == session.plugin_id)
                .ok_or("plugin missing")?
                .wasm_runtime()
                .ok_or("native plugin runtime missing")?;
            let document_id = self.sync_document_id().unwrap_or_else(|| "document".into());
            let schema = session.app.document.join(".");
            let bindings = Self::parse_persistence_binding(&uri)?;
            self.detach_sync_backbone_internal();
            let actor_uri = format!("actor://{document_id}");
            let channels = self.document_host.open(semio_framework_sync::DocumentActorConfig {
                document_id: document_id.clone(),
                schema,
                bindings,
                watch_external: true,
                actor: format!("wgpu-{}", session.instance_id),
            });
            let events = self.document_host.subscribe(&document_id);
            runtime
                .register_host_backbone(&actor_uri, Box::new(channels.channel_backbone))
                .map_err(|error| format!("register host backbone: {error}"))?;
            runtime
                .attach_backbone(session.instance_id, &actor_uri)
                .map_err(|error| format!("plugin attach backbone: {error}"))?;
            let cmd_tx = channels.cmd_tx.clone();
            let _ = cmd_tx.send(DocumentActorMsg::LocalOps { envelopes: Vec::new() });
            self.sync_channel = Some(ShellSyncChannel {
                document_id,
                actor_uri,
                instance_id: session.instance_id,
                plugin_id: session.plugin_id.clone(),
                cmd_tx,
                events,
            });
            self.sync_status = Some(DocumentSyncStatus::default());
            self.sync_backbone_uri = Some(uri);
            self.sync_card_kind = None;
            eprintln!(
                "[DEBUG] wgpu shell attached backbone {}",
                self.sync_backbone_uri.as_deref().unwrap_or_default()
            );
            self.refresh_ui().await?;
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &session;
            self.sync_backbone_uri = Some(uri);
            self.sync_card_kind = None;
            web_sys::console::log_1(&"[DEBUG] attached backbone (browser wgpu: relayed via host-shim)".into());
            Ok(())
        }
    }

    async fn handle_sync_action(&mut self, action: ActionDescriptor) -> Result<(), String> {
        match action.action.as_str() {
            "selectFile" => {
                self.sync_card_kind = Some("file".into());
                self.sync_card_draft = self
                    .sync_backbone_uri
                    .as_deref()
                    .filter(|uri| uri.starts_with("file://"))
                    .map(|uri| uri.trim_start_matches("file://").to_string())
                    .unwrap_or_default();
                Ok(())
            }
            "selectFolder" => {
                self.sync_card_kind = Some("folder".into());
                self.sync_card_draft = self
                    .sync_backbone_uri
                    .as_deref()
                    .filter(|uri| uri.starts_with("folder://"))
                    .map(|uri| uri.trim_start_matches("folder://").to_string())
                    .unwrap_or_default();
                Ok(())
            }
            "selectRemote" => {
                self.sync_card_kind = Some("remote".into());
                self.sync_card_draft = self
                    .sync_backbone_uri
                    .as_deref()
                    .filter(|uri| uri.starts_with("remote://"))
                    .map(|uri| uri.trim_start_matches("remote://").to_string())
                    .unwrap_or_default();
                Ok(())
            }
            "attach" => {
                let path = action
                    .args
                    .as_ref()
                    .and_then(|args| args.get("path"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(self.sync_card_draft.as_str());
                if path.trim().is_empty() {
                    return Ok(());
                }
                let kind = action
                    .args
                    .as_ref()
                    .and_then(|args| args.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(self.sync_card_kind.as_deref().unwrap_or("file"));
                let uri = match kind {
                    "folder" => format!("folder://{path}"),
                    "remote" => format!("remote://{path}"),
                    _ => format!("file://{path}"),
                };
                self.attach_sync_backbone(uri).await
            }
            "detach" => {
                #[cfg(not(target_arch = "wasm32"))]
                self.detach_sync_backbone_internal();
                self.sync_backbone_uri = None;
                self.sync_card_kind = None;
                self.last_envelope_json = None;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn dispatch_action(&mut self, action: ActionDescriptor) -> Result<(), String> {
        if action.controller_id == "framework" {
            match action.action.as_str() {
                "setAppearance" => {
                    if let Some(value) = action
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_str())
                    {
                        self.appearance_id = value.to_string();
                    }
                    return Ok(());
                }
                "setExpertise" => {
                    if let Some(value) = action
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
                    if let Some(value) = action
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_bool())
                    {
                        self.compact_mode = value;
                    }
                    return Ok(());
                }
                "setLocale" => {
                    if let Some(value) = action
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_str())
                    {
                        self.locale_id = value.to_string();
                    }
                    return Ok(());
                }
                "setTerminology" => {
                    if let Some(value) = action
                        .args
                        .as_ref()
                        .and_then(|args| args.get("value"))
                        .and_then(|v| v.as_str())
                    {
                        self.terminology_id = value.to_string();
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
        if action.controller_id == "framework.sync" {
            return self.handle_sync_action(action).await;
        }
        // 🧰 Intercept the framework `setActiveTool` View action to update the host-owned active-tool
        // map before forwarding to the plugin (which reacts by clearing its live-preview scratch). The
        // authoritative state is the shell map + the `ViewState.active_tool_id` it injects on render.
        if action.action == semio_framework_core::SET_ACTIVE_TOOL_ACTION_ID {
            if let Some(session) = self.session.clone() {
                if action.controller_id == session.app.controller_id {
                    if let Some(tool_id) = action
                        .args
                        .as_ref()
                        .and_then(|args| args.get("toolId"))
                        .and_then(|value| value.as_str())
                    {
                        let window_kind_id = action
                            .args
                            .as_ref()
                            .and_then(|args| args.get("windowKindId"))
                            .and_then(|value| value.as_str())
                            .map(String::from)
                            .unwrap_or_else(|| self.active_toolbar_window_kind(&session).id.clone());
                        self.apply_set_active_tool(&window_kind_id, tool_id);
                    }
                }
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
                    .any(|app| app.controller_id == action.controller_id)
            })
            .or_else(|| self.plugins.iter().find(|p| p.plugin_id == session.plugin_id))
            .ok_or("action plugin missing")?;
        let action_json = serde_json::to_string(&action).map_err(|err| err.to_string())?;
        let result = plugin
            .handle_action(session.instance_id, &action_json, &session.view_state)
            .await?;
        // 🧰 A plugin may programmatically switch the active tool via `HostEffect::SetActiveTool`
        // (Architecture Decision 4/9) — apply it to the host-owned map just like a user click.
        for effect in &result.requested_effects {
            if let semio_framework_core::kernel::HostEffect::SetActiveTool { window_kind_id, tool_id } = effect {
                self.active_tool_by_window
                    .insert(window_kind_id.clone(), tool_id.clone());
            }
        }
        let ops: Vec<String> = result
            .operations
            .iter()
            .filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok())
            .collect();
        self.apply_ops(&ops).await
    }

    pub async fn apply_ops(&mut self, ops: &[String]) -> Result<(), String> {
        self.apply_ops_inner(ops, true).await
    }

    async fn apply_ops_inner(&mut self, ops: &[String], allow_navigate: bool) -> Result<(), String> {
        let mut pending: Vec<String> = ops.to_vec();
        let mut view_state = self.session.as_ref().map(|s| s.view_state.clone());
        let mut document_changed = false;
        let mut navigate_uri: Option<String> = None;
        while !pending.is_empty() {
            let batch = std::mem::take(&mut pending);
            let mut follow_up_ops: Vec<String> = Vec::new();
            for op_json in batch {
            let op: serde_json::Value = serde_json::from_str(&op_json).unwrap_or(serde_json::Value::Null);
            if op.get("op").and_then(|v| v.as_str()) == Some("setDocument") {
                // 🔗 Document sync now flows through the `framework/sync` `DocumentHost` actor + the
                // plugin store's `ChannelBackbone` (see `attach_sync_backbone`), not a CRUD envelope
                // write on every `setDocument` — the old `shell_backbone_write` mirror is deleted.
                document_changed = true;
            }
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
                    let encoding = op.get("encoding").and_then(|v| v.as_str());
                    download_media_export(filename, mime_type, data, encoding);
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("requestFileOpen") {
                if let Some(import_action) = op.get("importAction").and_then(|v| v.as_str()) {
                    let accept = op
                        .get("accept")
                        .and_then(|v| v.as_str())
                        .unwrap_or(".json");
                    let read_as = op.get("readAs").and_then(|v| v.as_str());
                    if let (Some(session), Some(contents)) = (self.session.clone(), request_file_open(accept, read_as)) {
                        let payload = serde_json::from_str::<serde_json::Value>(&contents)
                            .unwrap_or_else(|_| serde_json::Value::String(contents.clone()));
                        let mut args = op.get("args").cloned().unwrap_or_else(|| serde_json::json!({}));
                        if let Some(obj) = args.as_object_mut() {
                            obj.insert("json".into(), serde_json::Value::String(contents));
                            obj.insert("payload".into(), payload);
                        }
                        let action = ActionDescriptor {
                            controller_id: session.app.controller_id.clone(),
                            action: import_action.to_string(),
                            args: Some(args),
                        };
                        if let Some(plugin) = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id) {
                            if let Ok(action_json) = serde_json::to_string(&action) {
                                if let Ok(import_result) = plugin
                                    .handle_action(session.instance_id, &action_json, &session.view_state)
                                    .await
                                {
                                    follow_up_ops.extend(patch_ops_from_action_result(&import_result));
                                }
                            }
                        }
                    }
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("requestFileSave") {
                #[cfg(not(target_arch = "wasm32"))]
                if let (Some(filename), Some(data), Some(studio_id)) = (
                    op.get("filename").and_then(|v| v.as_str()),
                    op.get("data").and_then(|v| v.as_str()),
                    op.get("studioId").and_then(|v| v.as_str()),
                ) {
                    if let Some(path) = request_file_save(filename) {
                        let _ = std::fs::write(&path, data.as_bytes());
                        if let Some(session) = self.session.clone() {
                            let action = ActionDescriptor {
                                controller_id: session.app.controller_id.clone(),
                                action: "bindStudioFile".into(),
                                args: Some(serde_json::json!({
                                    "studioId": studio_id,
                                    "filePath": path.display().to_string(),
                                })),
                            };
                            if let Some(plugin) = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id) {
                                if let Ok(action_json) = serde_json::to_string(&action) {
                                    if let Ok(bind_result) = plugin
                                        .handle_action(session.instance_id, &action_json, &session.view_state)
                                        .await
                                    {
                                        follow_up_ops.extend(patch_ops_from_action_result(&bind_result));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("requestFolderPick") {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(import_action) = op.get("importAction").and_then(|v| v.as_str()) {
                    if let Some(folder_path) = pick_folder() {
                        let mut args = op.get("args").cloned().unwrap_or_else(|| serde_json::json!({}));
                        if let Some(obj) = args.as_object_mut() {
                            obj.insert("folderPath".into(), serde_json::json!(folder_path));
                        }
                        if let Some(session) = self.session.clone() {
                            let action = ActionDescriptor {
                                controller_id: session.app.controller_id.clone(),
                                action: import_action.to_string(),
                                args: Some(args),
                            };
                            if let Some(plugin) = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id) {
                                if let Ok(action_json) = serde_json::to_string(&action) {
                                    if let Ok(folder_result) = plugin
                                        .handle_action(session.instance_id, &action_json, &session.view_state)
                                        .await
                                    {
                                        follow_up_ops.extend(patch_ops_from_action_result(&folder_result));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("spawnProgram") {
                if let (Some(program_id), Some(session)) = (op.get("programId").and_then(|v| v.as_str()), &self.session) {
                    self.spawn_program(program_id, session.view_state.clone()).await?;
                }
            }
            if op.get("op").and_then(|v| v.as_str()) == Some("navigate") {
                if let Some(uri) = op.get("uri").and_then(|v| v.as_str()) {
                    navigate_uri = Some(uri.to_string());
                }
            }
            }
            if !follow_up_ops.is_empty() {
                pending.extend(follow_up_ops);
                document_changed = true;
            }
        }
        if allow_navigate {
            if let Some(uri) = navigate_uri.take() {
                self.push_uri(uri.clone());
                self.apply_shell_uri(&uri).await?;
                if document_changed {
                    self.sync_session_chrome();
                }
                return Ok(());
            }
        }
        if let (Some(mut session), Some(vs)) = (self.session.take(), view_state) {
            session.view_state = vs;
            self.session = Some(session);
            self.sync_session_chrome();
            self.refresh_ui().await?;
        } else if document_changed {
            self.sync_session_chrome();
            self.refresh_ui().await?;
        }
        Ok(())
    }

    async fn switch_to_s_app(
        &mut self,
        app_id: &str,
        view_state: Option<ViewState>,
    ) -> Result<(), String> {
        let s_plugin = self
            .plugins
            .iter()
            .find(|plugin| plugin.plugin_id == "s")
            .ok_or("s plugin missing")?;
        let app = s_plugin
            .manifest
            .apps
            .iter()
            .find(|candidate| candidate.id == app_id)
            .ok_or("s app missing")?
            .clone();
        if let Some(session) = &self.session {
            if session.plugin_id == s_plugin.plugin_id && session.app.id == app_id {
                if let Some(next_view_state) = view_state {
                    if let Some(mut current) = self.session.take() {
                        current.view_state = next_view_state;
                        self.session = Some(current);
                        self.refresh_ui().await?;
                    }
                }
                return Ok(());
            }
        }
        let instance_id = s_plugin.create_app(&app.id).await?;
        let programs = self.build_studio_programs();
        let panel_state = StudioPanelState {
            active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
            programs,
            spawned_apps: vec![],
            active_spawned_id: None,
        };
        let next_view_state = view_state.unwrap_or_else(|| ViewState {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(app.window_kinds.first().id.clone()),
            active_tool_id: None,
            selection_json: None,
            panel_json: Some(Self::panel_json(&panel_state)),
            contributions_json: None,
            locale: Some(self.locale_id.clone()),
            terminology: Some(self.terminology_id.clone()),
        });
        self.active_window_id = Some(app.window_kinds.first().id.clone());
        if app_id == S_HOME_APP_ID {
            self.open_studio_id = None;
        }
        self.session = Some(ActiveSession {
            plugin_id: s_plugin.plugin_id.clone(),
            instance_id,
            app,
            view_state: next_view_state,
        });
        self.refresh_ui().await
    }

    async fn apply_shell_uri(&mut self, uri: &str) -> Result<(), String> {
        if !self.studio_mode {
            return Ok(());
        }
        let path = uri.split('?').next().unwrap_or(uri);
        let studio_id = path
            .strip_prefix("/studios/")
            .map(|value| value.trim_end_matches('/').to_string())
            .filter(|value| !value.is_empty());
        if studio_id.is_none() {
            self.open_studio_id = None;
            if self.session.as_ref().map(|session| session.app.id.as_str()) != Some(S_HOME_APP_ID) {
                self.switch_to_s_app(S_HOME_APP_ID, None).await?;
            }
            return Ok(());
        }
        let studio_id = studio_id.expect("studio id");
        self.switch_to_s_app(S_PLAY_APP_ID, None).await?;
        if self.open_studio_id.as_deref() == Some(studio_id.as_str()) {
            return Ok(());
        }
        self.open_studio_id = Some(studio_id.clone());
        let session = self.session.clone().ok_or("studio session missing")?;
        let plugin = self
            .plugins
            .iter()
            .find(|entry| entry.plugin_id == session.plugin_id)
            .ok_or("studio plugin missing")?;
        let action = ActionDescriptor {
            controller_id: S_PLAY_CONTROLLER_ID.into(),
            action: "openStudio".into(),
            args: Some(serde_json::json!({ "studioId": studio_id })),
        };
        let action_json = serde_json::to_string(&action).map_err(|err| err.to_string())?;
        let _ops = plugin
            .handle_action(session.instance_id, &action_json, &session.view_state)
            .await?;
        self.sync_session_chrome();
        self.refresh_ui().await
    }

    pub async fn apply_pending_shell_uri(&mut self) -> Result<(), String> {
        let uri = self.shell_uri();
        self.apply_shell_uri(&uri).await
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
            document: program.document.clone(),
        });
        panel.active_spawned_id = Some(spawned_id);
        view_state.panel_json = Some(Self::panel_json(&panel));
        if let Some(session) = self.session.as_mut() {
            session.view_state = view_state;
        }
        Ok(())
    }
}
//#endregion ShellActions

//#region ShellInput
impl ShellState {
    pub async fn handle_pointer_button(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
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
                        if let Some(action) = hit.event.clone() {
                            self.dispatch_action(action).await?;
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
            for (surface_id, surface) in &self.gis_map_states {
                if surface.bounds.contains(x, y) {
                    push_gis_map_context_menu(
                        surface_id,
                        &surface.controller_id,
                        surface.bounds,
                        x,
                        y,
                    );
                    break;
                }
            }
            let puzzle_board_hit = self
                .puzzle2d_board_states
                .iter()
                .find(|(_, surface)| surface.bounds.contains(x, y))
                .map(|(surface_id, surface)| (surface_id.clone(), surface.controller_id.clone(), surface.fixture_json.clone(), surface.bounds));
            if let Some((surface_id, controller_id, fixture_json, bounds)) = puzzle_board_hit {
                open_puzzle2d_board_context_menu(self, &surface_id, &controller_id, &fixture_json, bounds, x, y).await;
            }
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
                            .unwrap_or(&Theme::default().window_measures_default_width);
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
                let body = self.body_rect(theme);
                let width = if hit.control_id.as_deref() == Some("panel.resize.left") {
                    floating_panel_width(self.left_panel_width, body, theme)
                } else {
                    floating_panel_width(self.right_panel_width, body, theme)
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
            // 🧾 Flush an in-progress staged-arg edit before any Actions-rail interaction so Execute
            // merges it (Architecture Decision 8, P2 — "execute flushes any focused text buffer first").
            if hit.control_id.as_deref().is_some_and(|id| id.starts_with("shell.action."))
                && input.focused_id.as_deref().is_some_and(|id| {
                    id.starts_with("shell.action.arginput::") || id.starts_with("shell.action.argvec3::")
                })
            {
                self.commit_focused_input(input).await?;
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
                        self.dispatch_action(ActionDescriptor {
                            controller_id: self
                                .session
                                .as_ref()
                                .map(|s| s.app.controller_id.clone())
                                .unwrap_or_default(),
                            action: "selectRows".into(),
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
            if let Some(action) = hit.event.clone() {
                self.dispatch_action(action).await?;
            } else if hit.kind == HitKind::Input {
                if let Some(id) = &hit.control_id {
                    let seed = self
                        .widget_maps
                        .input_metas
                        .get(id)
                        .map(|meta| meta.value.clone())
                        .or_else(|| self.staged_input_seed(id))
                        .unwrap_or_default();
                    input.focus_input(id, &seed);
                }
            }
        }
        self.flush_deferred_actions().await?;
        Ok(())
    }

    pub fn handle_pointer_move(
        &mut self,
        x: f32,
        y: f32,
        down: bool,
        input: &mut InputState<ActionDescriptor>,
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
            crate::engine_canvas::node_graph_sync_flow_widget_ghost(
                x,
                y,
                &drag.drag_data,
                &self
                    .node_graph_states
                    .iter()
                    .map(|(id, surface)| (id.as_str(), surface.bounds))
                    .collect::<Vec<_>>(),
            );
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
        if input.drag.active && down {
            input.update_drag(x, y);
            if let Some(id) = input.drag.target_id.as_deref() {
                let dx = x - input.drag.start_x;
                let dy = y - input.drag.start_y;
                match id {
                    id if id.starts_with("shell.measures.resize.") => {
                        if let Some(window_id) = self.measures_resize_window_id.clone() {
                            let next = (self.measures_resize_origin_width - dx)
                                .clamp(theme.panel_min_width, theme.panel_max_width);
                            self.measures_width.insert(window_id, next);
                        }
                    }
                    "panel.resize.left" => {
                        let body = self.body_rect(theme);
                        self.left_panel_width = (self.panel_resize_origin_width + dx)
                            .clamp(theme.panel_min_width, floating_panel_max_width(body, theme));
                    }
                    "panel.resize.right" => {
                        let body = self.body_rect(theme);
                        self.right_panel_width = (self.panel_resize_origin_width - dx)
                            .clamp(theme.panel_min_width, floating_panel_max_width(body, theme));
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
        input: &InputState<ActionDescriptor>,
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

    fn scroll_region_is_scene_surface(control_id: &str) -> bool {
        control_id.ends_with(".pane") || control_id.ends_with(".map")
    }

    pub fn wheel_propagates_to_scene_surface(hit: Option<&HitTarget<ActionDescriptor>>) -> bool {
        let Some(hit) = hit else {
            return true;
        };
        match hit.kind {
            HitKind::World3d | HitKind::Window => true,
            HitKind::ScrollRegion => hit
                .control_id
                .as_deref()
                .is_some_and(Self::scroll_region_is_scene_surface),
            _ => false,
        }
    }

    pub fn handle_pointer_wheel(
        &mut self,
        x: f32,
        y: f32,
        delta: f32,
        input: &InputState<ActionDescriptor>,
    ) -> bool {
        let Some(hit) = input.hit_at(x, y) else {
            return false;
        };
        if hit.kind != HitKind::ScrollRegion {
            return false;
        }
        let Some(id) = &hit.control_id else {
            return false;
        };
        if Self::scroll_region_is_scene_surface(id) {
            return false;
        }
        let entry = self.scroll_offsets.entry(id.clone()).or_insert(0.0);
        *entry = (*entry + delta * 24.0).max(0.0);
        true
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
        let mut actions = Vec::new();
        let modifiers = PointerModifiers { shift, ctrl, alt, meta };
        for state in self.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(action) = handle_world3d_pointer_button(state, x, y, down, button, &modifiers) {
                actions.push(action);
            }
            actions.extend(handle_world3d_paint_actions(state, x, y, down, button));
            if let Some(action) = handle_world3d_pointer_move(state, x, y, down, button) {
                actions.push(action);
            }
        }
        for action in actions {
            self.dispatch_action(action).await?;
        }
        Ok(())
    }

    pub async fn poll_world3d_assets(&mut self) {
        fetch_pending_glb_meshes(&mut self.world3d_states).await;
        fetch_pending_reference_images(&mut self.world3d_states).await;
    }

    async fn handle_shell_hit(&mut self, hit: &HitTarget<ActionDescriptor>) -> Result<bool, String> {
        let Some(id) = hit.control_id.as_deref() else {
            return Ok(false);
        };
        match id {
            "ui.nav.back" => {
                if self.uri_index > 0 {
                    self.uri_index -= 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.forward" => {
                if self.uri_index + 1 < self.uri_history.len() {
                    self.uri_index += 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.up" => {
                let uri = self.shell_uri();
                if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                    if !parent.is_empty() {
                        self.push_uri(parent);
                    }
                }
                self.apply_pending_shell_uri().await?;
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
                    if let Some(layout) = semio_framework_core::resolve_layout_for_mode(&session.app, mode_id) {
                        self.layout_override = Some(layout);
                        self.sync_dock();
                        self.active_window_id = self.dock.active_window_id.clone();
                    }
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
                    self.dispatch_action(ActionDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        action: "setActiveExample".into(),
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
            id if id.starts_with("shell.action.fold.") => {
                let window_id = id.trim_start_matches("shell.action.fold.");
                let folded = self.action_panel_folded.get(window_id).copied().unwrap_or(true);
                self.action_panel_folded.insert(window_id.to_string(), !folded);
                return Ok(true);
            }
            id if id.starts_with("shell.action.expand::") => {
                if let Some((window_id, action_id)) =
                    id.trim_start_matches("shell.action.expand::").split_once("::")
                {
                    let open = self.action_panel_expanded.get(window_id).map(String::as_str) == Some(action_id);
                    if open {
                        self.action_panel_expanded.remove(window_id);
                    } else {
                        self.action_panel_expanded
                            .insert(window_id.to_string(), action_id.to_string());
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.reset::") => {
                if let Some((window_id, action_id)) =
                    id.trim_start_matches("shell.action.reset::").split_once("::")
                {
                    self.reset_staged_args(window_id, action_id);
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.argtoggle::") => {
                let parts: Vec<&str> = id.trim_start_matches("shell.action.argtoggle::").split("::").collect();
                if let [window_id, action_id, arg_id] = parts.as_slice() {
                    let current = self
                        .staged_map_for(window_id, action_id)
                        .get(*arg_id)
                        .and_then(|value| value.as_bool())
                        .or_else(|| self.arg_default(action_id, arg_id).and_then(|value| value.as_bool()))
                        .unwrap_or(false);
                    self.stage_arg(window_id, action_id, arg_id, serde_json::Value::Bool(!current));
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.argselect::") => {
                let parts: Vec<&str> = id.trim_start_matches("shell.action.argselect::").split("::").collect();
                if let [window_id, action_id, arg_id, value] = parts.as_slice() {
                    self.stage_arg(
                        window_id,
                        action_id,
                        arg_id,
                        serde_json::Value::String((*value).to_string()),
                    );
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.exec::") => {
                if let Some((window_id, action_id)) =
                    id.trim_start_matches("shell.action.exec::").split_once("::")
                {
                    let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
                    self.execute_staged_action(&window_id, &action_id).await?;
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
                self.dispatch_action(ActionDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    action: "goHome".into(),
                    args: None,
                })
                .await?;
                return Ok(true);
            }
            "studio.canvas.back" => {
                if let Some(session) = &self.session {
                    if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
                        if panel.active_spawned_id.is_some() {
                            self.dispatch_action(ActionDescriptor {
                                controller_id: S_PLAY_CONTROLLER_ID.into(),
                                action: "closeFocusedInstance".into(),
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
                self.dispatch_action(ActionDescriptor {
                    controller_id: self
                        .session
                        .as_ref()
                        .map(|s| s.app.controller_id.clone())
                        .unwrap_or_default(),
                    action: "setMode".into(),
                    args: Some(serde_json::json!({ "modeId": mode_id })),
                })
                .await?;
                return Ok(true);
            }
            id if id.starts_with("framework.settings.appearance.") => {
                self.appearance_id = id.trim_start_matches("framework.settings.appearance.").to_string();
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
                    self.dispatch_action(ActionDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        action: "setActivePanelTab".into(),
                        args: Some(serde_json::json!({ "tabId": tab_id })),
                    })
                    .await?;
                }
                return Ok(true);
            }
            id if self.context_menu.as_ref().is_some_and(|menu| menu.items.iter().any(|item| item.id == id)) => {
                if let Some(menu) = &self.context_menu {
                    if let Some(item) = menu.items.iter().find(|item| item.id == id) {
                        if let Some(action) = item.action.clone() {
                            self.dispatch_action(action).await?;
                        }
                    }
                }
                self.context_menu = None;
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
                    if let Some(action) = self.widget_maps.select_metas.get(select_id).cloned() {
                        self.open_selects.insert(select_id.to_string(), false);
                        self.dispatch_action(ActionDescriptor {
                            controller_id: action.controller_id,
                            action: action.action,
                            args: Some(serde_json::json!({ "value": value })),
                        })
                        .await?;
                        return Ok(true);
                    }
                }
            }
            id if self.widget_maps.toggle_metas.contains_key(id) => {
                if let Some((pressed, action)) = self.widget_maps.toggle_metas.get(id).cloned() {
                    self.dispatch_action(ActionDescriptor {
                        controller_id: action.controller_id,
                        action: action.action,
                        args: Some(serde_json::json!({ "pressed": !pressed })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".minus") => {
                let base = id.trim_end_matches(".minus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_action(ActionDescriptor {
                        controller_id: meta.on_delta.controller_id,
                        action: meta.on_delta.action,
                        args: Some(serde_json::json!({ "delta": -meta.step })),
                    })
                    .await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".plus") => {
                let base = id.trim_end_matches(".plus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_action(ActionDescriptor {
                        controller_id: meta.on_delta.controller_id,
                        action: meta.on_delta.action,
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

    fn update_tree_hover(&mut self, input: &InputState<ActionDescriptor>) {
        let hovered = input
            .hovered_id
            .as_deref()
            .and_then(|id| id.strip_prefix("tree.label."));
        if self.tree_hovered_id.as_deref() == hovered {
            return;
        }
        if let Some(prev) = self.tree_hovered_id.take() {
            if let Some(action) = self.widget_maps.tree_unhover_commands.get(&prev) {
                self.deferred_actions.push(action.clone());
            }
        }
        if let Some(id) = hovered {
            if let Some(action) = self.widget_maps.tree_hover_commands.get(id) {
                self.deferred_actions.push(action.clone());
            }
            self.tree_hovered_id = Some(id.to_string());
        }
    }

    fn queue_tree_selection(&mut self, item_id: &str) {
        let Some(action) = self.widget_maps.tree_selection_change.clone() else {
            return;
        };
        self.deferred_actions.push(ActionDescriptor {
            controller_id: action.controller_id,
            action: action.action,
            args: Some(serde_json::json!({ "ids": [item_id] })),
        });
    }

    async fn dispatch_tree_selection(&mut self, item_id: &str) -> Result<(), String> {
        self.queue_tree_selection(item_id);
        self.flush_deferred_actions().await
    }

    pub async fn flush_deferred_actions(&mut self) -> Result<(), String> {
        let actions = std::mem::take(&mut self.deferred_actions);
        for action in actions {
            self.dispatch_action(action).await?;
        }
        if self.pending_shell_uri_apply {
            self.pending_shell_uri_apply = false;
            self.apply_pending_shell_uri().await?;
        }
        Ok(())
    }

    async fn dispatch_widget_drag_values(&mut self, input: &InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.drag.target_id.as_deref() else {
            return Ok(());
        };
        if let Some(value) = self.widget_maps.slider_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.slider_metas.get(id).cloned() {
                self.dispatch_action(ActionDescriptor {
                    controller_id: meta.on_change.controller_id,
                    action: meta.on_change.action,
                    args: Some(serde_json::json!({ "value": value })),
                })
                .await?;
            }
        } else if let Some(value) = self.widget_maps.ring_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.ring_metas.get(id).cloned() {
                self.dispatch_action(ActionDescriptor {
                    controller_id: meta.on_change.controller_id,
                    action: meta.on_change.action,
                    args: Some(serde_json::json!({ "value": value })),
                })
                .await?;
            }
        }
        Ok(())
    }

    async fn commit_focused_input(&mut self, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.focused_id.clone() else {
            return Ok(());
        };
        // 📝 A staged action-arg input writes into the staging map (parsed per the arg's control kind)
        // instead of dispatching live — Architecture Decision 8, P2 (item 4).
        if self.commit_staged_input(&id, &input.text_buffer) {
            input.blur_input();
            return Ok(());
        }
        if let Some((vec3_id, axis)) = id.rsplit_once('.') {
            if let Ok(axis_index) = axis.parse::<usize>() {
                if axis_index < 3 {
                    if let Some(meta) = self.widget_maps.vec3_metas.get(vec3_id).cloned() {
                        let parsed = input.text_buffer.parse::<f64>().unwrap_or(0.0);
                        let mut value = meta.value;
                        value[axis_index] = parsed;
                        self.dispatch_action(ActionDescriptor {
                            controller_id: meta.on_change.controller_id,
                            action: meta.on_change.action,
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
                self.dispatch_action(ActionDescriptor {
                    controller_id: meta.on_absolute.controller_id,
                    action: meta.on_absolute.action,
                    args: Some(serde_json::json!({ "value": parsed })),
                })
                .await?;
                input.blur_input();
                return Ok(());
            }
        }
        if let Some(meta) = self.widget_maps.input_metas.get(&id).cloned() {
            self.dispatch_action(ActionDescriptor {
                controller_id: meta.on_change.controller_id,
                action: meta.on_change.action,
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
        input: &InputState<ActionDescriptor>,
    ) -> Result<(), String> {
        let Some(drag) = self.tree_drag.take() else {
            return Ok(());
        };
        if let Some(action) = crate::engine_canvas::node_graph_flow_widget_drop_action(
            x,
            y,
            &drag.drag_data,
            &self
                .node_graph_states
                .iter()
                .map(|(id, surface)| (id.as_str(), surface.bounds, surface.controller_id.as_str()))
                .collect::<Vec<_>>(),
        ) {
            crate::engine_canvas::node_graph_clear_all_ghost_widgets();
            self.dispatch_action(action).await?;
            return Ok(());
        }
        crate::engine_canvas::node_graph_clear_all_ghost_widgets();
        if let Some(hit) = input.hit_at(x, y) {
            if hit.kind == HitKind::World3d || hit.kind == HitKind::Window {
                if let Some(raw) = drag.drag_data.get("application/x-semio-catalogue-item") {
                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(raw) {
                        let program_id = payload.get("programId").and_then(|v| v.as_str());
                        let app_id = payload.get("appId").and_then(|v| v.as_str());
                        if let (Some(program_id), Some(app_id)) = (program_id, app_id) {
                            self.dispatch_action(ActionDescriptor {
                                controller_id: S_PLAY_CONTROLLER_ID.into(),
                                action: "spawnApp".into(),
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

    fn render_tree_drag_overlay(&self, overlay: &mut DrawList, input: &InputState<ActionDescriptor>, theme: &Theme) {
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
        self.active_left_tab = Some(tab_id.to_string());
        if self.studio_mode {
            if let Some(session) = &self.session {
                if session.app.id == S_PLAY_APP_ID {
                    self.dispatch_action(ActionDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        action: "setActivePanelTab".into(),
                        args: Some(serde_json::json!({ "tabId": tab_id })),
                    })
                    .await?;
                }
            }
        }
        Ok(())
    }

    fn dismiss_overlays(&mut self, x: f32, y: f32, input: &InputState<ActionDescriptor>) -> bool {
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

    fn open_context_menu(&mut self, x: f32, y: f32, hit: Option<HitTarget<ActionDescriptor>>) {
        let node_id = hit.as_ref().and_then(|hit| {
            hit.control_id.as_deref().and_then(|id| {
                id.rsplit_once(".node.").map(|(_, node_id)| node_id.to_string())
            })
        });
        let mut items = take_context_menu_items()
            .into_iter()
            .map(|mut item| {
                if let Some(action) = item.action.take() {
                    item.action = Some(resolve_graph_context_action(&action, node_id.as_deref()));
                }
                item
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            if let (Some(node_id), Some(session)) = (node_id.as_deref(), &self.session) {
                items.push(ContextMenuItem {
                    id: format!("shell.context.node.select.{node_id}"),
                    label: "Select node".into(),
                    action: Some(ActionDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        action: "setMediaNodeSelection".into(),
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
                    action: None,
                },
                ContextMenuItem {
                    id: "shell.context.paste".into(),
                    label: "Paste".into(),
                    action: None,
                },
            ];
        }
        if self.studio_mode {
            items.push(ContextMenuItem {
                id: "shell.context.home".into(),
                label: "Go Home".into(),
                action: Some(ActionDescriptor {
                    controller_id: S_PLAY_CONTROLLER_ID.into(),
                    action: "goHome".into(),
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
                id: format!("panel.{}", tab.id()),
                label: tab.label.clone(),
                group: "Panels".into(),
                dispatch_action: Some(ActionDescriptor {
                    controller_id: session.app.controller_id.clone(),
                    action: "setActivePanelTab".into(),
                    args: Some(serde_json::json!({ "tabId": tab.id() })),
                }),
                action: None,
            });
        }
        for kind in &session.app.window_kinds {
            items.push(SearchPaletteItem {
                id: format!("window.{}", kind.id),
                label: kind.label.clone(),
                group: "Windows".into(),
                dispatch_action: None,
                action: Some(format!("window:{}", kind.id)),
            });
        }
        for binding in &session.app.keybindings {
            items.push(SearchPaletteItem {
                id: format!("keybinding.{}", binding.keys),
                label: binding.action.action.clone(),
                group: "Actions".into(),
                dispatch_action: Some(binding.action.clone()),
                action: None,
            });
        }
        // 📇 Declared window-scoped actions (Architecture Decision 8, P3 — wgpu previously listed only
        // keybindings). Zero-arg actions dispatch directly; arg-carrying actions redirect to the hosting
        // window's Actions rail so they never fire with `args: None`.
        for action in &session.app.actions {
            if !action.in_palette
                || action.kind == semio_framework_core::ActionKind::History
                || action.id == semio_framework_core::SET_ACTIVE_TOOL_ACTION_ID
            {
                continue;
            }
            if action.args.is_empty() {
                items.push(SearchPaletteItem {
                    id: format!("action.{}", action.id),
                    label: action.label.clone(),
                    group: "Actions".into(),
                    dispatch_action: Some(ActionDescriptor {
                        controller_id: session.app.controller_id.clone(),
                        action: action.id.clone(),
                        args: None,
                    }),
                    action: None,
                });
            } else {
                let window_id = action_host_window_id(&session.app, &action.id)
                    .unwrap_or_else(|| session.app.window_kinds.first().id.clone());
                items.push(SearchPaletteItem {
                    id: format!("action.{}", action.id),
                    label: format!("{} …", action.label),
                    group: "Actions".into(),
                    dispatch_action: None,
                    action: Some(format!("action-panel:{window_id}:{}", action.id)),
                });
            }
        }
        if self.studio_mode {
            for action in ["undo", "redo", "commitCheckpoint"] {
                items.push(SearchPaletteItem {
                    id: format!("studio.{action}"),
                    label: action.into(),
                    group: "Studio".into(),
                    dispatch_action: Some(ActionDescriptor {
                        controller_id: S_PLAY_CONTROLLER_ID.into(),
                        action: action.into(),
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
        if let Some(action) = item.dispatch_action.clone() {
            self.dispatch_action(action).await?;
        } else if let Some(action) = &item.action {
            if let Some(window_id) = action.strip_prefix("window:") {
                self.active_window_id = Some(window_id.to_string());
            } else if let Some(rest) = action.strip_prefix("action-panel:") {
                // 📇 P3 redirect: focus the hosting window, unfold its Actions rail, expand the form.
                if let Some((window_id, action_id)) = rest.split_once(':') {
                    self.active_window_id = Some(window_id.to_string());
                    self.action_panel_folded.insert(window_id.to_string(), false);
                    self.action_panel_expanded
                        .insert(window_id.to_string(), action_id.to_string());
                }
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
            self.dispatch_action(ActionDescriptor {
                controller_id: session.app.controller_id.clone(),
                action: "setMediaNodeSelection".into(),
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
        input: &mut InputState<ActionDescriptor>,
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
            self.pending_shell_uri_apply = true;
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::Char(ref c) if c == "]") {
            if self.uri_index + 1 < self.uri_history.len() {
                self.uri_index += 1;
            }
            self.pending_shell_uri_apply = true;
            return;
        }
        if meta && matches!(action, ui_wgpu::KeyAction::ArrowUp) {
            let uri = self.shell_uri();
            if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                if !parent.is_empty() {
                    self.push_uri(parent);
                }
            }
            self.pending_shell_uri_apply = true;
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
        if self.sync_card_kind.is_some() {
            match action {
                ui_wgpu::KeyAction::Escape => {
                    self.sync_card_kind = None;
                    return;
                }
                ui_wgpu::KeyAction::Enter => {
                    self.deferred_actions.push(ActionDescriptor {
                        controller_id: "framework.sync".into(),
                        action: "attach".into(),
                        args: Some(serde_json::json!({
                            "path": self.sync_card_draft,
                            "kind": self.sync_card_kind,
                        })),
                    });
                    return;
                }
                ui_wgpu::KeyAction::Char(key) => {
                    self.sync_card_draft.push_str(&key);
                    return;
                }
                ui_wgpu::KeyAction::Backspace => {
                    self.sync_card_draft.pop();
                    return;
                }
                _ => {}
            }
        }
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
        input: &mut InputState<ActionDescriptor>,
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
        let idle = input.focused_id.is_none()
            && self.overlay_state == OverlayState::None
            && self.sync_card_kind.is_none()
            && self.dock_drag.is_none();
        // 🧰 Escape deactivates the active tool for the focused window (P5).
        if idle && action == ui_wgpu::KeyAction::Escape {
            if let Some(window_id) = self.active_window_id.clone() {
                if self.active_tool_by_window.remove(&window_id).is_some() {
                    self.refresh_ui().await?;
                    return Ok(());
                }
            }
        }
        // ⌨️ App-declared keybinding dispatch (Architecture Decision 8, P4) — NET-NEW for the wgpu
        // shell, which previously only handled hardcoded shell chords. Reserved shell chords still win.
        if idle && !is_reserved_shell_chord(&action, modifiers) {
            if let Some(descriptor) = self.match_app_keybinding(&action, modifiers) {
                self.dispatch_app_keybinding(descriptor).await?;
                return Ok(());
            }
        }
        self.handle_keyboard(action, modifiers, input);
        Ok(())
    }

    /// ⌨️ The app keybinding matching the current key event, if any.
    fn match_app_keybinding(
        &self,
        action: &ui_wgpu::KeyAction,
        modifiers: &ui_wgpu::PointerModifiers,
    ) -> Option<ActionDescriptor> {
        let session = self.session.as_ref()?;
        session
            .app
            .keybindings
            .iter()
            .find(|binding| key_event_matches_chord(action, modifiers, &binding.keys))
            .map(|binding| binding.action.clone())
    }

    /// ⌨️ Applies the P4 keybinding rule: arg-less actions dispatch directly; an arg-carrying action's
    /// hotkey opens its form, or — if that form is already expanded in the active window — executes it
    /// with the staged/validated args (never silent-fires defaults from a cold keystroke).
    async fn dispatch_app_keybinding(&mut self, descriptor: ActionDescriptor) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let action_def = session
            .app
            .actions
            .iter()
            .find(|action| action.id == descriptor.action)
            .cloned();
        let has_args = action_def.as_ref().is_some_and(|action| !action.args.is_empty());
        if !has_args {
            return self.dispatch_action(descriptor).await;
        }
        let action_id = action_def.expect("checked has_args").id;
        let window_id = action_host_window_id(&session.app, &action_id)
            .unwrap_or_else(|| self.active_toolbar_window_kind(&session).id.clone());
        let already_expanded = self.active_window_id.as_deref() == Some(window_id.as_str())
            && self.action_panel_expanded.get(&window_id).map(String::as_str) == Some(action_id.as_str());
        if already_expanded {
            self.execute_staged_action(&window_id, &action_id).await
        } else {
            self.active_window_id = Some(window_id.clone());
            self.action_panel_folded.insert(window_id.clone(), false);
            self.action_panel_expanded.insert(window_id, action_id);
            self.refresh_ui().await
        }
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
    input: &mut InputState<ActionDescriptor>,
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

/** @emoji 📑 Shared side-panel tab strip for floating panels. */
fn render_panel_tab_bar(
    panel_draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    panel: Rect,
    tabs: &[PanelTabDefinition],
    active_tab_id: &str,
    side_left: bool,
    inner_stroke: Rgba,
    hair: f32,
) -> f32 {
    let tab_bar_h = theme.panel_header_height;
    let tab_bar = Rect::new(
        panel.x + hair,
        panel.y,
        (panel.w - hair * 2.0).max(0.0),
        tab_bar_h,
    );
    panel_draw.push_scissor(tab_bar);
    panel_draw.push_solid(
        [tab_bar.x, tab_bar.y + tab_bar_h - hair, tab_bar.w, hair],
        inner_stroke,
    );
    let mut tab_x = tab_bar.x;
    for (index, tab) in tabs.iter().enumerate() {
        let icon_id = panel_tab_icon_id(tab);
        let label_w = atlas.measure_text(&tab.label, theme.font_size_small).0;
        let tw = theme.padding_standard * 2.0 + CHROME_ICON_TINY + theme.gap_standard + label_w;
        let rect = Rect::new(tab_x, tab_bar.y, tw, tab_bar_h);
        if index > 0 {
            panel_draw.push_solid([tab_x, tab_bar.y, hair, tab_bar_h], inner_stroke);
        }
        let active = tab.id() == active_tab_id;
        let hovered = rect.contains(input.pointer_x, input.pointer_y);
        if active {
            panel_draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.selected);
        } else if hovered {
            panel_draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.button_hover);
        }
        let icon_x = rect.x + theme.padding_standard;
        let icon_y = rect.y + (rect.h - CHROME_ICON_TINY) * 0.5;
        chrome_icon(
            panel_draw,
            icons,
            icon_id,
            icon_x,
            icon_y,
            CHROME_ICON_TINY,
            chrome_item_text(theme, active, hovered),
        );
        chrome_text(
            panel_draw,
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
            control_id: Some(format!("{prefix}{}", tab.id())),
            kind: HitKind::PanelTab,
            drag_axis: None,
            drag_data: None,
        });
        tab_x += tw;
    }
    panel_draw.pop_scissor();
    tab_bar_h
}

fn chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    push_chrome_group_border(draw, rect, theme);
}

struct ChromeGroupItem<'a> {
    control_id: &'a str,
    icon_id: Option<&'a str>,
    label: Option<&'a str>,
    active: bool,
    disabled: bool,
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

struct WindowMeasuresRailOutcome {
    chip_hit: Option<(Rect, String)>,
    reserve_width: f32,
}

fn window_overlay_max_width(content_w: f32, inset: f32) -> f32 {
    (content_w - inset * 2.0).max(0.0)
}

fn engagement_rail_width(theme: &Theme, content_w: f32, inset: f32, measures_reserve: f32) -> f32 {
    let available = content_w - inset * 2.0 - measures_reserve;
    theme
        .window_engagement_max_width
        .min(available.max(0.0))
}

fn floating_panel_available_width(body: Rect, theme: &Theme) -> f32 {
    (body.w - theme.panel_inset * 2.0).max(theme.panel_min_width)
}

fn floating_panel_max_width(body: Rect, theme: &Theme) -> f32 {
    theme
        .panel_max_width
        .min(floating_panel_available_width(body, theme))
        .max(theme.panel_min_width)
}

fn floating_panel_width(width: f32, body: Rect, theme: &Theme) -> f32 {
    width.clamp(theme.panel_min_width, floating_panel_max_width(body, theme))
}

fn measure_window_measure_height(
    theme: &Theme,
    collapsed_sections: &HashMap<String, bool>,
    measure: &WindowMeasure,
) -> f32 {
    match measure {
        WindowMeasure::Group {
            id,
            default_open,
            children,
            ..
        } => {
            let open =
                !collapsed_sections.get(id).copied().unwrap_or(!default_open.unwrap_or(false));
            let mut h = theme.control_height;
            if open {
                for child in children {
                    h += measure_window_measure_height(theme, collapsed_sections, child);
                }
            }
            h
        }
        WindowMeasure::Select { .. } | WindowMeasure::Slider { .. } => 16.0 + theme.control_height,
        WindowMeasure::Toggle { .. } => theme.control_height,
    }
}

fn measure_window_measures_body_height(
    theme: &Theme,
    collapsed_sections: &HashMap<String, bool>,
    measures: &[WindowMeasure],
) -> f32 {
    measures
        .iter()
        .map(|measure| measure_window_measure_height(theme, collapsed_sections, measure))
        .sum()
}

fn measure_engagement_body_height(theme: &Theme, engagement: &WindowEngagement) -> f32 {
    let mut h = 0.0f32;
    if let Some(options) = &engagement.options {
        h += options.len() as f32 * (theme.control_height + 4.0);
    }
    if engagement.input.is_some() {
        h += theme.control_height * 2.0 + 8.0;
    }
    if engagement.control.is_some() {
        h += theme.control_height;
    }
    if let Some(status_rows) = &engagement.status {
        h += status_rows.len() as f32 * theme.control_height;
    }
    if let Some(possibles) = &engagement.possible_engagements {
        h += possibles.len() as f32 * (theme.control_height + 2.0);
    }
    h
}

fn render_chrome_group(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
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
        let hovered = !item.disabled && item_rect.contains(input.pointer_x, input.pointer_y);
        let bg = if item.disabled {
            theme.overlay_shadow
        } else {
            chrome_item_bg(theme, item.active, hovered)
        };
        if bg.a > 0.0 {
            draw.push_solid([item_rect.x, item_rect.y, item_rect.w, item_rect.h], bg);
        }
        let text_color = if item.disabled {
            theme.text_muted
        } else {
            chrome_item_text(theme, item.active, hovered)
        };
        let mut content_x = item_rect.x + theme.padding_standard;
        if let Some(icon_id) = item.icon_id {
            chrome_icon(
                draw,
                icons,
                icon_id,
                content_x,
                item_rect.y + (item_rect.h - CHROME_ICON_TINY) * 0.5,
                CHROME_ICON_TINY,
                text_color,
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
                text_color,
            );
        }
        if register_hits && !item.disabled {
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

fn backbone_kind_from_uri(uri: &str) -> &'static str {
    if uri.starts_with("file://") {
        "file"
    } else if uri.starts_with("folder://") {
        "folder"
    } else if uri.starts_with("remote://") {
        "remote"
    } else {
        "unknown"
    }
}

fn framework_sync_tools(active_uri: Option<&str>) -> Vec<ToolNode> {
    let active_kind = active_uri.map(backbone_kind_from_uri);
    let pressed = |kind: &str| active_kind == Some(kind);
    vec![
        ToolNode::Toggle {
            id: "framework.sync.file".into(),
            icon_id: "file-json".into(),
            label: Some("File".into()),
            text: None,
            title: None,
            order: Some(0),
            pressed: Some(pressed("file")),
            disabled: None,
            category: Some(ToolCategory::Sync),
            on_change: ActionDescriptor {
                controller_id: "framework.sync".into(),
                action: "selectFile".into(),
                args: None,
            },
        },
        ToolNode::Toggle {
            id: "framework.sync.folder".into(),
            icon_id: "folder".into(),
            label: Some("Folder".into()),
            text: None,
            title: None,
            order: Some(1),
            pressed: Some(pressed("folder")),
            disabled: None,
            category: Some(ToolCategory::Sync),
            on_change: ActionDescriptor {
                controller_id: "framework.sync".into(),
                action: "selectFolder".into(),
                args: None,
            },
        },
        ToolNode::Toggle {
            id: "framework.sync.remote".into(),
            icon_id: "cloud".into(),
            label: Some("Remote".into()),
            text: None,
            title: None,
            order: Some(2),
            pressed: Some(pressed("remote")),
            disabled: None,
            category: Some(ToolCategory::Sync),
            on_change: ActionDescriptor {
                controller_id: "framework.sync".into(),
                action: "selectRemote".into(),
                args: None,
            },
        },
    ]
}

fn partition_tools_by_category(tools: &[ToolNode]) -> [Vec<ToolNode>; 4] {
    let mut buckets: [Vec<ToolNode>; 4] = [vec![], vec![], vec![], vec![]];
    for tool in tools {
        let idx = match tool.category() {
            ToolCategory::Selection => 0,
            ToolCategory::Tools => 1,
            ToolCategory::History => 2,
            ToolCategory::Sync => 3,
        };
        buckets[idx].push(tool.clone());
    }
    buckets
}

fn render_footer_section_divider(draw: &mut DrawList, theme: &Theme, x: f32, btn_y: f32, btn_h: f32) -> f32 {
    draw.push_solid(
        [x + theme.gap_standard * 0.5, btn_y + 4.0, theme.stroke_hairline, btn_h - 8.0],
        theme.border_normal,
    );
    x + theme.gap_standard
}

fn render_footer_tool_nodes(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
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
                x = render_footer_section_divider(draw, theme, x, btn_y, btn_h);
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
                    disabled: false,
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
                    disabled: false,
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
                    disabled: false,
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
    if tab.id() == S_PLAY_CATALOGUE_TAB_ID || tab.group == PanelGroup::Workbench {
        return "library";
    }
    if tab.id().contains("parameters") {
        return "settings";
    }
    if tab.id().contains("inspector") || tab.id().contains("inspection") || tab.id() == FRAMEWORK_PANEL_TAB_INSPECTION_ID {
        return "text-search";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_DOCUMENT_ID {
        return "file-text";
    }
    if tab.id() == FRAMEWORK_DISPLAY_WINDOWS_TAB_ID {
        return "layout-grid";
    }
    if tab.id() == FRAMEWORK_DISPLAY_LAYOUT_TAB_ID {
        return "layout";
    }
    if tab.id() == FRAMEWORK_SETTINGS_GENERAL_TAB_ID {
        return "settings-2";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_CATALOGUE_ID {
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

/// 🧭 This renderer only has a 2-panel (left/right) layout; fold the framework's 4-corner model back down.
fn group_side(group: PanelGroup) -> &'static str {
    if group.corner().ends_with("left") { "left" } else { "right" }
}

fn panel_toggle_icon_id(kind: &str, session: Option<&ActiveSession>) -> &'static str {
    match kind {
        "display" => "layout-grid",
        "workbench" => session
            .and_then(|s| s.app.panel_tabs.iter().find(|tab| group_side(tab.group) == "left"))
            .map(|tab| panel_tab_icon_id(tab))
            .unwrap_or("folder"),
        "details" => session
            .and_then(|s| s.app.panel_tabs.iter().find(|tab| group_side(tab.group) == "right"))
            .map(|tab| panel_tab_icon_id(tab))
            .unwrap_or("info"),
        "settings" => "settings-2",
        _ => "circle-dot",
    }
}

/// 🛡 Chrome content must always win over window bodies; route it to the
/// overlay compositing phase (guaranteed last) whenever one is available.
fn with_chrome_sink<F, R>(
    draw: &mut DrawList,
    overlay: &mut Option<&mut DrawList>,
    f: F,
) -> R
where
    F: FnOnce(&mut DrawList, &mut Option<&mut DrawList>) -> R,
{
    if let Some(chrome) = overlay.as_deref_mut() {
        let mut nested_overlay = None;
        f(chrome, &mut nested_overlay)
    } else {
        f(draw, overlay)
    }
}

//#region ActionPanelAndTools
/// 🧰 Resolves the tools a window kind presents in the toolbar — the tool mirror of
/// {@link semio_framework_core::resolve_window_actions}: explicit `window_kind.tools` refs in declared
/// order, plus any app tool referenced by no window kind (an "orphan" appearing on every window — the
/// scoping fallback that prevents blank toolbars mid-migration, Architecture Decision 8).
pub(crate) fn resolve_window_tools<'a>(
    app: &'a semio_framework_core::AppDefinition,
    window_kind: &semio_framework_core::WindowKindDefinition,
) -> Vec<&'a semio_framework_core::ToolDefinition> {
    use std::collections::HashSet;
    let referenced: HashSet<&str> = app
        .window_kinds
        .iter()
        .flat_map(|window| window.tools.iter().map(|tool_ref| tool_ref.as_str()))
        .collect();
    let mut resolved: Vec<&'a semio_framework_core::ToolDefinition> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for tool_ref in &window_kind.tools {
        if let Some(tool) = app.tools.iter().find(|tool| tool.id == tool_ref.as_str()) {
            if seen.insert(tool.id.as_str()) {
                resolved.push(tool);
            }
        }
    }
    for tool in &app.tools {
        if !referenced.contains(tool.id.as_str()) && seen.insert(tool.id.as_str()) {
            resolved.push(tool);
        }
    }
    resolved
}

/// 📇 The first window kind whose resolved actions include `action_id` — the window the palette/keybinding
/// redirect focuses to open an arg-carrying action's form (Architecture Decision 8, P3/P4).
pub(crate) fn action_host_window_id(
    app: &semio_framework_core::AppDefinition,
    action_id: &str,
) -> Option<String> {
    app.window_kinds
        .iter()
        .find(|kind| {
            semio_framework_core::resolve_window_actions(app, kind)
                .iter()
                .any(|action| action.id == action_id)
        })
        .map(|kind| kind.id.clone())
}

/// 🔢 Formats a number for a staged input/vec3 field — integers without a trailing `.0`.
fn fmt_num(value: f64) -> String {
    if value.is_finite() && value == value.trunc() && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// 🖱️ Maps a `ToolDefinition.cursor` CSS/winit cursor name onto the shell's {@link ui_wgpu::SemioCursor}.
fn semio_cursor_from_name(name: &str) -> ui_wgpu::SemioCursor {
    use ui_wgpu::SemioCursor;
    match name.trim().to_ascii_lowercase().as_str() {
        "pointer" => SemioCursor::Pointer,
        "text" => SemioCursor::Text,
        "grab" => SemioCursor::Grab,
        "grabbing" => SemioCursor::Grabbing,
        "move" => SemioCursor::Move,
        "crosshair" | "cross" => SemioCursor::Crosshair,
        "not-allowed" | "notallowed" | "forbidden" => SemioCursor::NotAllowed,
        "ew-resize" | "col-resize" | "e-resize" | "w-resize" => SemioCursor::EwResize,
        "ns-resize" | "row-resize" | "n-resize" | "s-resize" => SemioCursor::NsResize,
        "nwse-resize" | "nw-resize" | "se-resize" => SemioCursor::NwseResize,
        "nesw-resize" | "ne-resize" | "sw-resize" => SemioCursor::NeswResize,
        "cell" | "selectable" => SemioCursor::Selectable,
        _ => SemioCursor::Default,
    }
}

/// ⌨️ Whether a key event is one of the hardcoded shell chords (palette/find/panels/nav) that must win
/// over app-declared keybindings (P4 — "reserved shell chords still win").
pub(crate) fn is_reserved_shell_chord(action: &ui_wgpu::KeyAction, modifiers: &ui_wgpu::PointerModifiers) -> bool {
    let accelerator = modifiers.meta || modifiers.ctrl;
    if !accelerator {
        return false;
    }
    match action {
        ui_wgpu::KeyAction::Char(c) => matches!(c.to_ascii_lowercase().as_str(), "p" | "f" | "b" | "[" | "]"),
        ui_wgpu::KeyAction::ArrowUp => true,
        _ => false,
    }
}

/// ⌨️ Whether a key event matches a keybinding chord such as `"mod+shift+z"`, `"ctrl+k"`, or `"escape"`.
/// `"mod"` is the platform accelerator (meta OR ctrl). Declared modifiers must be present and no
/// undeclared accelerator/shift/alt may be held, so `mod+z` never fires for `mod+shift+z`.
pub(crate) fn key_event_matches_chord(
    action: &ui_wgpu::KeyAction,
    modifiers: &ui_wgpu::PointerModifiers,
    chord: &str,
) -> bool {
    let mut want_mod = false;
    let mut want_shift = false;
    let mut want_alt = false;
    let mut want_ctrl = false;
    let mut want_meta = false;
    let mut key_token = String::new();
    for token in chord.split('+') {
        match token.trim().to_ascii_lowercase().as_str() {
            "" => {}
            "mod" => want_mod = true,
            "shift" => want_shift = true,
            "alt" | "option" => want_alt = true,
            "ctrl" | "control" => want_ctrl = true,
            "cmd" | "meta" | "super" | "win" => want_meta = true,
            other => key_token = other.to_string(),
        }
    }
    if key_token.is_empty() {
        return false;
    }
    if modifiers.shift != want_shift || modifiers.alt != want_alt {
        return false;
    }
    let accelerator = modifiers.meta || modifiers.ctrl;
    let want_accelerator = want_mod || want_ctrl || want_meta;
    if want_accelerator != accelerator {
        return false;
    }
    match action {
        ui_wgpu::KeyAction::Char(c) => c.eq_ignore_ascii_case(&key_token),
        ui_wgpu::KeyAction::Enter => key_token == "enter" || key_token == "return",
        ui_wgpu::KeyAction::Escape => key_token == "escape" || key_token == "esc",
        ui_wgpu::KeyAction::Backspace => key_token == "backspace",
        ui_wgpu::KeyAction::Delete => key_token == "delete" || key_token == "del",
        ui_wgpu::KeyAction::Tab => key_token == "tab",
        ui_wgpu::KeyAction::ArrowLeft => key_token == "arrowleft" || key_token == "left",
        ui_wgpu::KeyAction::ArrowRight => key_token == "arrowright" || key_token == "right",
        ui_wgpu::KeyAction::ArrowUp => key_token == "arrowup" || key_token == "up",
        ui_wgpu::KeyAction::ArrowDown => key_token == "arrowdown" || key_token == "down",
        ui_wgpu::KeyAction::Space(_) => key_token == "space",
    }
}

impl ShellState {
    // #region tool-derivation
    /// 🧰 The window kind whose tools/actions the shell chrome currently scopes to (the focused window,
    /// else the view-state's active kind, else the app's first kind).
    fn active_toolbar_window_kind<'a>(
        &self,
        session: &'a ActiveSession,
    ) -> &'a semio_framework_core::WindowKindDefinition {
        let active_id = self
            .active_window_id
            .as_deref()
            .or(session.view_state.active_window_kind_id.as_deref());
        active_id
            .and_then(|id| session.app.window_kinds.iter().find(|kind| kind.id == id))
            .unwrap_or_else(|| session.app.window_kinds.first())
    }

    /// 🧰 Derives the footer toolbar `ToolNode`s from the app's declared tools scoped to the active
    /// window kind, marking the host-owned active tool pressed (Architecture Decision 5).
    fn derive_toolbar_nodes(&self, session: &ActiveSession) -> Vec<ToolNode> {
        if session.app.tools.is_empty() {
            return Vec::new();
        }
        let window_kind = self.active_toolbar_window_kind(session);
        let resolved = resolve_window_tools(&session.app, window_kind);
        if resolved.is_empty() {
            return Vec::new();
        }
        let specs: Vec<ui_wgpu::component::tools::DerivedToolSpec> = resolved
            .iter()
            .map(|tool| ui_wgpu::component::tools::DerivedToolSpec {
                id: tool.id.clone(),
                label: tool.label.clone(),
                icon_id: tool.icon_id.clone(),
                group: tool.group.clone(),
                category: tool.category,
            })
            .collect();
        let active = self
            .active_tool_by_window
            .get(&window_kind.id)
            .map(String::as_str);
        ui_wgpu::component::tools::derive_tool_nodes(&session.app.controller_id, &specs, active)
    }
    // #endregion

    // #region active-tool
    /// 🧰 Applies a user-driven `setActiveTool`: re-selecting the active tool deactivates it, otherwise
    /// it becomes the active tool for that window kind (Architecture Decision 4).
    pub(crate) fn apply_set_active_tool(&mut self, window_kind_id: &str, tool_id: &str) {
        let already = self
            .active_tool_by_window
            .get(window_kind_id)
            .map(String::as_str)
            == Some(tool_id);
        if already {
            self.active_tool_by_window.remove(window_kind_id);
        } else {
            self.active_tool_by_window
                .insert(window_kind_id.to_string(), tool_id.to_string());
        }
    }

    /// 🧰 The active tool id for a window kind, if any.
    pub(crate) fn active_tool_for_window(&self, window_kind_id: &str) -> Option<&str> {
        self.active_tool_by_window
            .get(window_kind_id)
            .map(String::as_str)
    }

    /// 🖱️ The cursor the active tool requests while the pointer is over the active window's body — maps
    /// `ToolDefinition.cursor` onto a {@link ui_wgpu::SemioCursor} (P5). `None` when no tool/cursor applies.
    pub(crate) fn tool_cursor_override(&self, x: f32, y: f32) -> Option<ui_wgpu::SemioCursor> {
        let session = self.session.as_ref()?;
        let window_id = self.active_window_id.as_deref()?;
        let tool_id = self.active_tool_for_window(window_id)?;
        let cursor_name = session
            .app
            .tools
            .iter()
            .find(|tool| tool.id == tool_id)?
            .cursor
            .as_deref()?;
        let rect = self.window_content_rects.get(window_id)?;
        rect.contains(x, y).then(|| semio_cursor_from_name(cursor_name))
    }

    /// 🚦 Whether window-scoped actions stay enabled: `true` when no tool is active or the active tool
    /// declares `allows_actions_while_active` (P5 — replaces the old `TOOL_ID_PREFIXES` whitelist).
    pub(crate) fn actions_enabled_for_window(
        &self,
        app: &semio_framework_core::AppDefinition,
        window_kind_id: &str,
    ) -> bool {
        match self.active_tool_for_window(window_kind_id) {
            None => true,
            Some(tool_id) => app
                .tools
                .iter()
                .find(|tool| tool.id == tool_id)
                .map(|tool| tool.allows_actions_while_active)
                .unwrap_or(true),
        }
    }
    // #endregion

    // #region staging
    fn staged_key(window_id: &str, action_id: &str) -> String {
        format!("{window_id}:{action_id}")
    }

    pub(crate) fn stage_arg(
        &mut self,
        window_id: &str,
        action_id: &str,
        arg_id: &str,
        value: serde_json::Value,
    ) {
        self.staged_action_args
            .entry(Self::staged_key(window_id, action_id))
            .or_default()
            .insert(arg_id.to_string(), value);
    }

    pub(crate) fn staged_map_for(
        &self,
        window_id: &str,
        action_id: &str,
    ) -> serde_json::Map<String, serde_json::Value> {
        self.staged_action_args
            .get(&Self::staged_key(window_id, action_id))
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn reset_staged_args(&mut self, window_id: &str, action_id: &str) {
        self.staged_action_args
            .remove(&Self::staged_key(window_id, action_id));
    }

    /// 📝 Parses a focused staged-arg input's buffer per the arg's control kind and writes it into the
    /// staging map. Returns `true` when `control_id` belongs to a staged input (so the caller stops).
    fn commit_staged_input(&mut self, control_id: &str, buffer: &str) -> bool {
        use semio_framework_core::ActionArgControl;
        let (is_vec3, rest) = if let Some(rest) = control_id.strip_prefix("shell.action.argvec3::") {
            (true, rest)
        } else if let Some(rest) = control_id.strip_prefix("shell.action.arginput::") {
            (false, rest)
        } else {
            return false;
        };
        let parts: Vec<&str> = rest.split("::").collect();
        let (Some(window_id), Some(action_id), Some(arg_id)) =
            (parts.first().copied(), parts.get(1).copied(), parts.get(2).copied())
        else {
            return true;
        };
        let (window_id, action_id, arg_id) =
            (window_id.to_string(), action_id.to_string(), arg_id.to_string());
        if is_vec3 {
            let axis: usize = parts.get(3).and_then(|token| token.parse().ok()).unwrap_or(0);
            let mut current: Vec<serde_json::Value> = self
                .staged_map_for(&window_id, &action_id)
                .get(&arg_id)
                .and_then(|value| value.as_array().cloned())
                .or_else(|| self.arg_default(&action_id, &arg_id).and_then(|value| value.as_array().cloned()))
                .unwrap_or_else(|| vec![serde_json::json!(0.0), serde_json::json!(0.0), serde_json::json!(0.0)]);
            while current.len() < 3 {
                current.push(serde_json::json!(0.0));
            }
            if axis < 3 {
                current[axis] = serde_json::json!(buffer.trim().parse::<f64>().unwrap_or(0.0));
            }
            self.stage_arg(&window_id, &action_id, &arg_id, serde_json::Value::Array(current));
            return true;
        }
        let control = self
            .session
            .as_ref()
            .and_then(|session| session.app.actions.iter().find(|action| action.id == action_id))
            .and_then(|action| action.args.iter().find(|arg| arg.id == arg_id))
            .map(|arg| arg.control.clone());
        let value = match control {
            Some(ActionArgControl::Number { .. }) | Some(ActionArgControl::Slider { .. }) => {
                serde_json::json!(buffer.trim().parse::<f64>().unwrap_or(0.0))
            }
            _ => serde_json::Value::String(buffer.to_string()),
        };
        self.stage_arg(&window_id, &action_id, &arg_id, value);
        true
    }

    /// 📝 The seed string used when focusing a staged-arg input — its current effective value.
    fn staged_input_seed(&self, control_id: &str) -> Option<String> {
        let (is_vec3, rest) = if let Some(rest) = control_id.strip_prefix("shell.action.argvec3::") {
            (true, rest)
        } else if let Some(rest) = control_id.strip_prefix("shell.action.arginput::") {
            (false, rest)
        } else {
            return None;
        };
        let parts: Vec<&str> = rest.split("::").collect();
        let window_id = parts.first().copied()?;
        let action_id = parts.get(1).copied()?;
        let arg_id = parts.get(2).copied()?;
        let session = self.session.as_ref()?;
        let arg = session
            .app
            .actions
            .iter()
            .find(|action| action.id == action_id)?
            .args
            .iter()
            .find(|arg| arg.id == arg_id)?;
        let effective = self.effective_arg_value(window_id, action_id, arg);
        if is_vec3 {
            let axis: usize = parts.get(3).and_then(|token| token.parse().ok()).unwrap_or(0);
            let number = effective
                .as_ref()
                .and_then(|value| value.as_array())
                .and_then(|array| array.get(axis))
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0);
            return Some(fmt_num(number));
        }
        Some(match effective {
            Some(serde_json::Value::String(text)) => text,
            Some(serde_json::Value::Number(num)) => num.to_string(),
            Some(serde_json::Value::Bool(flag)) => flag.to_string(),
            Some(other) => other.to_string(),
            None => String::new(),
        })
    }

    /// 🧮 Validated effective args for execution: `None` when a required arg is still unset — the P2
    /// gate that keeps arg-carrying actions from firing partially (delegates to the core-side pure
    /// {@link semio_framework_core::missing_required_args}).
    pub(crate) fn resolved_execute_args(
        defs: &[semio_framework_core::ActionArgDef],
        staged: &serde_json::Map<String, serde_json::Value>,
    ) -> Option<serde_json::Map<String, serde_json::Value>> {
        let effective = semio_framework_core::effective_action_args(defs, staged);
        if semio_framework_core::missing_required_args(defs, &effective).is_empty() {
            Some(effective)
        } else {
            None
        }
    }

    /// 🚀 Executes a staged action once (P2): validates required args, dispatches exactly one
    /// `ActionDescriptor` with the merged effective args, and keeps the staged values for tweak-and-
    /// repeat. No-ops when the active tool gates actions or a required arg is still unset.
    async fn execute_staged_action(&mut self, window_id: &str, action_id: &str) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        if !self.actions_enabled_for_window(&session.app, window_id) {
            return Ok(());
        }
        let Some(action) = session.app.actions.iter().find(|action| action.id == action_id).cloned() else {
            return Ok(());
        };
        let staged = self.staged_map_for(window_id, action_id);
        let Some(effective) = Self::resolved_execute_args(&action.args, &staged) else {
            return Ok(());
        };
        let args = if effective.is_empty() {
            None
        } else {
            Some(serde_json::Value::Object(effective))
        };
        self.dispatch_action(ActionDescriptor {
            controller_id: session.app.controller_id.clone(),
            action: action_id.to_string(),
            args,
        })
        .await
    }
    // #endregion
}
//#endregion ActionPanelAndTools

//#region ShellChrome
impl ShellState {
    pub fn render_chrome(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
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
        self.gis_map_states.clear();
        self.puzzle2d_board_states.clear();
        self.widget_maps.clear_frame();
        let mut overlay_slot = Some(overlay);
        self.render_main_window(draw, &mut overlay_slot, atlas, icons, input, theme, body, gpu);
        self.find_items = take_find_items();
        if self.left_panel_open && self.has_left_tabs() {
            if let Some(panel_draw) = overlay_slot.as_deref_mut() {
                self.render_left_panel(panel_draw, None, atlas, icons, input, theme, body, gpu);
            } else {
                self.render_left_panel(draw, None, atlas, icons, input, theme, body, gpu);
            }
        }
        if self.right_panel_open && self.has_right_tabs() {
            if let Some(panel_draw) = overlay_slot.as_deref_mut() {
                self.render_right_panel(panel_draw, None, atlas, icons, input, theme, body, gpu);
            } else {
                self.render_right_panel(draw, None, atlas, icons, input, theme, body, gpu);
            }
        }
        with_chrome_sink(draw, &mut overlay_slot, |chrome, _select_overlay| {
            self.render_navbar(chrome, atlas, icons, input, theme, w);
            self.render_footer(chrome, atlas, icons, input, theme, w, h);
        });
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
                theme.error,
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
                    kind: semio_framework_core::PanelTabKind::DisplayWindows,
                    label: "Windows".into(),
                    group: PanelGroup::Display,
                    body_key: Some(String::new()),
                    children: Vec::new(),
                },
                PanelTabDefinition {
                    kind: semio_framework_core::PanelTabKind::DisplayLayout,
                    label: "Layout".into(),
                    group: PanelGroup::Display,
                    body_key: Some(String::new()),
                    children: Vec::new(),
                },
            ],
            LeftPanelKind::Workbench => {
                let mut tabs: Vec<PanelTabDefinition> = session
                    .app
                    .panel_tabs
                    .iter()
                    .filter(|tab| group_side(tab.group) == "left")
                    .cloned()
                    .collect();
                let has_document = tabs.iter().any(|t| t.id() == FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
                if !has_document {
                    tabs.insert(
                        0,
                        PanelTabDefinition {
                            kind: semio_framework_core::PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
                            label: "Document".into(),
                            group: PanelGroup::Workbench,
                            body_key: Some(String::new()),
                            children: Vec::new(),
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
                kind: semio_framework_core::PanelTabKind::SettingsGeneral,
                label: "General".into(),
                group: PanelGroup::Settings,
                body_key: Some(String::new()),
                children: Vec::new(),
            }],
            RightPanelKind::Details => session
                .app
                .panel_tabs
                .iter()
                .filter(|tab| group_side(tab.group) == "right")
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
                    let tabs = self.left_tabs(session);
                    if let Some(id) = &self.active_left_tab {
                        if tabs.iter().any(|tab| tab.id() == *id) {
                            return id.clone();
                        }
                    }
                    tabs.first()
                        .map(|t| t.id().to_string())
                        .unwrap_or_else(|| FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into())
                }
            }
        }
    }

    fn active_right_tab_id(&self, session: &ActiveSession) -> String {
        if self.active_right_kind == RightPanelKind::Settings {
            return FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into();
        }
        let tabs = self.right_tabs(session);
        if let Some(id) = &self.active_right_tab {
            if tabs.iter().any(|tab| tab.id() == *id) {
                return id.clone();
            }
        }
        tabs.first()
            .map(|t| t.id().to_string())
            .unwrap_or_default()
    }

    fn has_display_tabs(&self) -> bool {
        self.session
            .as_ref()
            .is_some_and(|s| !s.app.window_kinds.is_empty())
    }

    fn floating_panel_rect(&self, left: bool, body: Rect, theme: &Theme) -> Rect {
        let inset = theme.panel_inset;
        let width = if left {
            floating_panel_width(self.left_panel_width, body, theme)
        } else {
            floating_panel_width(self.right_panel_width, body, theme)
        };
        if left {
            Rect::new(
                body.x + inset,
                body.y + inset,
                width,
                body.h - inset * 2.0,
            )
        } else {
            Rect::new(
                body.x + body.w - inset - width,
                body.y + inset,
                width,
                body.h - inset * 2.0,
            )
        }
    }

    fn render_navbar(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
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
                theme.text,
            );
        x += logo_size + theme.gap_standard;
        let title = self
            .session
            .as_ref()
            .map(|s| app_document_label(&s.app.document))
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
                    disabled: false,
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
            disabled: false,
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
                disabled: false,
                kind: HitKind::Toggle,
            });
        }
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.workbench",
            icon_id: Some(panel_toggle_icon_id("workbench", self.session.as_ref())),
            label: Some("Workbench"),
            active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench,
            disabled: false,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.details",
            icon_id: Some(panel_toggle_icon_id("details", self.session.as_ref())),
            label: Some("Details"),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Details,
            disabled: false,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.settings",
            icon_id: Some(panel_toggle_icon_id("settings", self.session.as_ref())),
            label: Some("Settings"),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Settings,
            disabled: false,
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
                // 🚧 `modes` is a `NonEmptyVec`, whose `iter()` yields an opaque non-double-ended
                // iterator — collect before reversing for the right-to-left navbar order.
                let modes: Vec<&semio_framework_core::ModeDefinition> = session.app.modes.iter().collect();
                let mode_control_ids: Vec<String> = modes
                    .iter()
                    .rev()
                    .map(|mode| format!("playground.navbar.modes.{}", mode.id))
                    .collect();
                let mode_items: Vec<ChromeGroupItem<'_>> = modes
                    .iter()
                    .rev()
                    .zip(mode_control_ids.iter())
                    .map(|(mode, control_id)| {
                        let active_mode = session
                            .view_state
                            .active_mode_id
                            .as_deref()
                            .unwrap_or(session.app.default_mode_id.as_str());
                        ChromeGroupItem {
                            control_id: control_id.as_str(),
                            icon_id: None,
                            label: Some(mode.label.as_str()),
                            active: active_mode == mode.id,
                            disabled: false,
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
        input: &mut InputState<ActionDescriptor>,
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
        if self.session.is_none() {
            return;
        }
        let btn_h = theme.control_height;
        let btn_y = y + (theme.footer_height - btn_h) * 0.5;
        // 🧰 Footer sections: Selection · Tools · History · Sync. The former `ToolCategory::Actions`
        // section is deleted — window-scoped actions now live in the per-window Actions rail
        // (Architecture Decision 8/9, P6).
        let partitions = partition_tools_by_category(&self.active_tools);
        let sections = [
            partitions[0].as_slice(),
            partitions[1].as_slice(),
            partitions[2].as_slice(),
            partitions[3].as_slice(),
        ];
        let mut tool_x = theme.padding_standard;
        let mut first_section = true;
        for tools in sections {
            if tools.is_empty() {
                continue;
            }
            if !first_section {
                tool_x = render_footer_section_divider(draw, theme, tool_x, btn_y, btn_h);
            }
            first_section = false;
            tool_x = render_footer_tool_nodes(
                draw,
                atlas,
                icons,
                input,
                theme,
                tool_x,
                btn_y,
                btn_h,
                tools,
                &self.tool_collection_expanded,
            );
        }
        let _ = tool_x;
    }

    fn render_floating_panel(
        &mut self,
        panel_draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
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
        let resize_handle = if side_left {
            Rect::new(
                panel.x + panel.w - PANEL_RESIZE_HIT_PX,
                panel.y,
                PANEL_RESIZE_HIT_PX,
                panel.h,
            )
        } else {
            Rect::new(panel.x, panel.y, PANEL_RESIZE_HIT_PX, panel.h)
        };
        let resize_active = input.drag.active && input.drag.target_id.as_deref() == Some(resize_id);
        let resize_handle_hovered = resize_handle.contains(input.pointer_x, input.pointer_y);
        let resize_edge_hot = resize_active || resize_handle_hovered;
        let panel_hovered = panel.contains(input.pointer_x, input.pointer_y);
        let hair = theme.stroke_hairline;
        let edge_color = |is_resize_edge: bool| {
            if is_resize_edge && resize_active {
                theme.accent
            } else if is_resize_edge && resize_handle_hovered {
                theme.border_emphasized
            } else if panel_hovered && !resize_edge_hot {
                theme.border_emphasized
            } else {
                theme.border_normal
            }
        };
        let top = edge_color(false);
        let bottom = edge_color(false);
        let left = edge_color(!side_left);
        let right = edge_color(side_left);
        let inner_stroke = if panel_hovered && !resize_edge_hot {
            theme.border_emphasized
        } else {
            theme.border_normal
        };
        let glass = panel_draw.push_glass(
            [panel.x, panel.y, panel.w, panel.h],
            theme.border_radius,
            GlassTier::Panel,
            theme,
        );
        panel_draw.begin_glass_content(glass);
        panel_draw.push_solid([panel.x, panel.y, panel.w, hair], top);
        panel_draw.push_solid([panel.x, panel.y + panel.h - hair, panel.w, hair], bottom);
        panel_draw.push_solid([panel.x, panel.y, hair, panel.h], left);
        panel_draw.push_solid([panel.x + panel.w - hair, panel.y, hair, panel.h], right);
        let tab_bar_h = render_panel_tab_bar(
            panel_draw,
            atlas,
            icons,
            input,
            theme,
            panel,
            tabs,
            active_tab_id,
            side_left,
            inner_stroke,
            hair,
        );
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
        panel_draw.push_scissor(content);
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
            panel_draw,
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
        ctx.pick_clip = Some(content);
        render_ui_node(&ui, scrolled, &mut ctx, gpu, &mut self.world3d_states, &mut self.node_graph_states, &mut self.gis_map_states, &mut self.icon_render_states, &mut self.puzzle2d_board_states);
        }
        panel_draw.pop_scissor();
        panel_draw.end_glass_content();
        input.register_hit(HitTarget {
            rect: resize_handle,
            event: None,
            control_id: Some(resize_id.into()),
            kind: HitKind::PanelResize,
            drag_axis: Some(DragAxis::Horizontal),
            drag_data: None,
        });
    }

    fn render_left_panel(
        &mut self,
        panel_draw: &mut DrawList,
        mut overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
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
            panel_draw,
            overlay.as_deref_mut(),
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
        panel_draw: &mut DrawList,
        mut overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
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
            panel_draw,
            overlay.as_deref_mut(),
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
        input: &mut InputState<ActionDescriptor>,
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
                    app_window_document_label(&session.app, &kind.label),
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
        self.window_content_rects.clear();
        for (_, content, window_id) in placements {
            self.window_content_rects.insert(window_id.clone(), content);
            let window_kind = session
                .app
                .window_kinds
                .iter()
                .find(|kind| kind.id == window_id)
                .cloned();
            let mut window_chip_hits: Vec<(Rect, String)> = Vec::new();
            if let Some(ui) = self.window_ui.get(&window_id).cloned() {
                self.render_window_content(
                    draw, overlay.as_deref_mut(), atlas, icons, input, theme, content, &ui, &window_id, gpu,
                );
            }
            if let Some(kind) = window_kind {
                let measures_outcome = self.render_window_measures_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &content,
                    &window_id,
                    &kind,
                    gpu,
                );
                if let Some(hit) = measures_outcome.chip_hit {
                    window_chip_hits.push(hit);
                }
                if let Some(hit) = self.render_window_engagement_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &content,
                    &window_id,
                    &kind,
                    measures_outcome.reserve_width,
                    gpu,
                ) {
                    window_chip_hits.push(hit);
                }
                if let Some(hit) = self.render_window_actions_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &content,
                    &window_id,
                    &session.app,
                    &kind,
                ) {
                    window_chip_hits.push(hit);
                }
                self.render_window_tool_options_rail(
                    draw,
                    overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    &content,
                    &window_id,
                    &kind,
                    gpu,
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
        with_chrome_sink(draw, overlay, |chrome, _select_overlay| {
            let mut dock_ctx = DockRenderContext {
                draw: chrome,
                atlas,
                icons,
                input,
                theme,
                window_labels: &window_labels,
            };
            self.dock.paint_chrome(&mut dock_ctx, canvas, false);
        });
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
                &app_document_label(&session.app.document),
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
        input: &mut InputState<ActionDescriptor>,
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
                disabled: false,
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
                    app_document_label(&spawned.document)
                );
                let item = ChromeGroupItem {
                    control_id: "studio.canvas.back",
                    icon_id: Some("chevron-left"),
                    label: Some(&label),
                    active: false,
                    disabled: false,
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
        input: &mut InputState<ActionDescriptor>,
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
        ctx.pick_clip = Some(content);
        render_ui_node(ui, scrolled, &mut ctx, gpu, &mut self.world3d_states, &mut self.node_graph_states, &mut self.gis_map_states, &mut self.icon_render_states, &mut self.puzzle2d_board_states);
        draw.pop_scissor();
    }

    fn render_overlay(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        width: f32,
        height: f32,
    ) {
        match &self.overlay_state {
            OverlayState::Search => {
                let items: Vec<(String, String, usize)> = self
                    .filtered_search_items()
                    .into_iter()
                    .enumerate()
                    .map(|(index, item)| (item.group, item.label, index))
                    .collect();
                self.render_action_list(
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
                self.render_action_list(
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
            }
            OverlayState::Dropdown(_) => {}
            OverlayState::None => {}
        }
        if let Some(kind) = self.sync_card_kind.as_deref() {
            let card_w = 320.0;
            let card_h = 132.0;
            let card_x = (width - card_w) * 0.5;
            let card_y = height - theme.footer_height - card_h - theme.gap_standard;
            overlay.push_solid([card_x, card_y, card_w, card_h], theme.panel);
            overlay.push_solid([card_x, card_y, card_w, theme.stroke_hairline], theme.border_normal);
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                &format!("{kind} backbone"),
                card_x + theme.padding_standard,
                card_y + theme.padding_standard,
                theme.font_size_small,
                theme.text,
            );
            if let Some(uri) = &self.sync_backbone_uri {
                chrome_text(
                    overlay,
                    atlas,
                    input,
                    theme,
                    uri,
                    card_x + theme.padding_standard,
                    card_y + theme.padding_standard + theme.font_size_small + 4.0,
                    theme.font_size_small,
                    theme.text_muted,
                );
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(status) = &self.sync_status {
                    chrome_text(
                        overlay,
                        atlas,
                        input,
                        theme,
                        &Self::sync_status_label(status),
                        card_x + theme.padding_standard,
                        card_y + theme.padding_standard + (theme.font_size_small + 4.0) * 2.0,
                        theme.font_size_small,
                        theme.text_muted,
                    );
                }
            }
            let input_y = card_y + 52.0;
            let input_h = theme.control_height;
            overlay.push_solid(
                [card_x + theme.padding_standard, input_y, card_w - theme.padding_standard * 2.0, input_h],
                theme.input_bg,
            );
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                if self.sync_card_draft.is_empty() {
                    "/absolute/path"
                } else {
                    &self.sync_card_draft
                },
                card_x + theme.padding_standard + 8.0,
                input_y + (input_h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                theme.text,
            );
            let attach_rect = Rect::new(
                card_x + theme.padding_standard,
                card_y + card_h - theme.control_height - theme.padding_standard,
                72.0,
                theme.control_height,
            );
            overlay.push_solid([attach_rect.x, attach_rect.y, attach_rect.w, attach_rect.h], theme.accent);
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                "Attach",
                attach_rect.x + 12.0,
                attach_rect.y + (attach_rect.h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                theme.active_foreground,
            );
            input.register_hit(HitTarget {
                rect: attach_rect,
                event: Some(ActionDescriptor {
                    controller_id: "framework.sync".into(),
                    action: "attach".into(),
                    args: Some(serde_json::json!({
                        "path": self.sync_card_draft,
                        "kind": kind,
                    })),
                }),
                control_id: Some("framework.sync.attach".into()),
                kind: HitKind::Button,
                drag_axis: None,
                drag_data: None,
            });
            if self.sync_backbone_uri.is_some() {
                let detach_rect = Rect::new(
                    attach_rect.x + attach_rect.w + theme.gap_standard,
                    attach_rect.y,
                    72.0,
                    theme.control_height,
                );
                overlay.push_solid([detach_rect.x, detach_rect.y, detach_rect.w, detach_rect.h], theme.button);
                chrome_text(
                    overlay,
                    atlas,
                    input,
                    theme,
                    "Detach",
                    detach_rect.x + 10.0,
                    detach_rect.y + (detach_rect.h + theme.font_size_small) * 0.5 - 1.0,
                    theme.font_size_small,
                    theme.text,
                );
                input.register_hit(HitTarget {
                    rect: detach_rect,
                    event: Some(ActionDescriptor {
                        controller_id: "framework.sync".into(),
                        action: "detach".into(),
                        args: None,
                    }),
                    control_id: Some("framework.sync.detach".into()),
                    kind: HitKind::Button,
                    drag_axis: None,
                    drag_data: None,
                });
            }
        }
        // render_palette removed
        if let Some(menu) = &self.context_menu {
            self.render_context_menu(overlay, atlas, input, theme, menu);
        }
    }

    fn render_example_dropdown(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<ActionDescriptor>,
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

    fn render_action_list(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<ActionDescriptor>,
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
        engagement: &ui_wgpu::WindowEngagement,
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

    fn measures_for_kind(&self, kind: &semio_framework_core::WindowKindDefinition) -> Vec<WindowMeasure> {
        self.window_measures
            .get(&kind.id)
            .filter(|measures| !measures.is_empty())
            .cloned()
            .unwrap_or_else(|| kind.options.measures.clone())
    }

    fn engagement_for_kind(&self, kind: &semio_framework_core::WindowKindDefinition) -> Option<WindowEngagement> {
        self.window_engagements
            .get(&kind.id)
            .cloned()
            .or_else(|| kind.options.engagement.as_option().cloned())
            .or_else(|| {
                if kind.surface_kind.is_viewport() {
                    Some(ui_wgpu::default_viewport_engagement())
                } else {
                    None
                }
            })
    }

    fn render_window_measures_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        kind: &semio_framework_core::WindowKindDefinition,
        gpu: &mut ui_wgpu::GpuContext,
    ) -> WindowMeasuresRailOutcome {
        let inset = theme.gap_standard;
        let active_tool = self.active_tool_by_window.get(window_id).cloned();
        let (measures, _tool_options) =
            ui_wgpu::partition_window_measures(&self.measures_for_kind(kind), active_tool.as_deref());
        if measures.is_empty() {
            return WindowMeasuresRailOutcome {
                chip_hit: None,
                reserve_width: 0.0,
            };
        }
        let folded = self.measures_folded.get(window_id).copied().unwrap_or(true);
        let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            if folded {
                let item = ChromeGroupItem {
                    control_id: "",
                    icon_id: Some("chevron-left"),
                    label: Some("Window Options"),
                    active: false,
                    disabled: false,
                    kind: HitKind::Button,
                };
                let chip_w = measure_chrome_group_item(atlas, theme, &item);
                let chip = Rect::new(
                    content.x + content.w - chip_w - inset,
                    content.y + inset,
                    chip_w,
                    theme.control_height,
                );
                render_chrome_group(chrome, atlas, icons, input, theme, chip, &[item], false);
                return WindowMeasuresRailOutcome {
                    chip_hit: Some((chip, format!("shell.measures.unfold.{window_id}"))),
                    reserve_width: chip_w + inset,
                };
            }
            let max_w = window_overlay_max_width(content.w, inset);
            let default_w = *self
                .measures_width
                .get(window_id)
                .unwrap_or(&theme.window_measures_default_width);
            let width = if expanded {
                content.w
            } else {
                default_w
                    .clamp(theme.panel_min_width, theme.panel_max_width)
                    .min(max_w)
            };
            let body_content_h =
                measure_window_measures_body_height(theme, &self.collapsed_sections, &measures);
            let rail_h = if expanded {
                content.h
            } else {
                let card_h = theme.panel_header_height + theme.gap_standard * 2.0 + body_content_h;
                card_h.min((content.h - inset * 2.0).max(theme.panel_header_height))
            };
            let (rail_x, rail_y) = if expanded {
                (content.x, content.y)
            } else {
                (
                    content.x + content.w - width - inset,
                    content.y + inset,
                )
            };
            let rail = Rect::new(rail_x, rail_y, width, rail_h);
            let glass = chrome.push_glass(
                [rail.x, rail.y, rail.w, rail.h],
                theme.border_radius,
                GlassTier::WindowOptions,
                theme,
            );
            chrome.begin_glass_content(glass);
            let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
            chrome.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
            let focus_label = if expanded { "Unfocus" } else { "Focus" };
            let focus_item = ChromeGroupItem {
                control_id: "shell.measures.focus",
                icon_id: Some(if expanded { "minimize-2" } else { "maximize-2" }),
                label: Some(focus_label),
                active: false,
                disabled: false,
                kind: HitKind::Button,
            };
            let fold_item = ChromeGroupItem {
                control_id: "shell.measures.fold",
                icon_id: Some("chevron-right"),
                label: Some("Window Options"),
                active: false,
                disabled: false,
                kind: HitKind::Button,
            };
            let focus_w = measure_chrome_group_item(atlas, theme, &focus_item);
            render_chrome_group(
                chrome,
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
                chrome,
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
            let mut y = body.y;
            for measure in &measures {
                let h = measure_window_measure_height(theme, &self.collapsed_sections, measure);
                self.render_window_measure(
                    chrome,
                    select_overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    Rect::new(body.x, y, body.w, h),
                    measure,
                    gpu,
                );
                y += h;
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
            chrome.end_glass_content();
            WindowMeasuresRailOutcome {
                chip_hit: None,
                reserve_width: if expanded { width } else { width + inset },
            }
        })(draw, overlay)
    }

    /// 🎯 Bottom-left "Tool Options" rail: the tool-scoped bucket of `partition_window_measures`,
    /// visually associated with the tool footer below it. Renders (no fold chip, no reserved space)
    /// only while the window's active tool has tagged measure groups; silent otherwise. Reuses
    /// [`Self::render_window_measure`] so Select/Slider/Toggle controls behave exactly as in the
    /// general Measures rail.
    fn render_window_tool_options_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        kind: &semio_framework_core::WindowKindDefinition,
        gpu: &mut ui_wgpu::GpuContext,
    ) {
        let inset = theme.gap_standard;
        let active_tool = self.active_tool_by_window.get(window_id).cloned();
        let (_general, tool_options) =
            ui_wgpu::partition_window_measures(&self.measures_for_kind(kind), active_tool.as_deref());
        if tool_options.is_empty() {
            return;
        }
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            let width = theme
                .window_measures_default_width
                .clamp(theme.panel_min_width, theme.panel_max_width)
                .min(window_overlay_max_width(content.w, inset));
            let body_content_h =
                measure_window_measures_body_height(theme, &self.collapsed_sections, &tool_options);
            let card_h = (theme.panel_header_height + theme.gap_standard * 2.0 + body_content_h)
                .min((content.h - inset * 2.0).max(theme.panel_header_height));
            let rail = Rect::new(
                content.x + inset,
                content.y + content.h - card_h - inset,
                width,
                card_h,
            );
            let glass = chrome.push_glass(
                [rail.x, rail.y, rail.w, rail.h],
                theme.border_radius,
                GlassTier::WindowOptions,
                theme,
            );
            chrome.begin_glass_content(glass);
            let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
            chrome.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
            chrome_text(
                chrome,
                atlas,
                input,
                theme,
                "Tool Options",
                header.x + theme.gap_standard,
                header.y + theme.panel_header_height * 0.5 + theme.font_size_small * 0.5,
                theme.font_size_small,
                theme.text_muted,
            );
            let body = Rect::new(
                rail.x + theme.gap_standard,
                rail.y + theme.panel_header_height + theme.gap_standard,
                rail.w - theme.gap_standard * 2.0,
                rail.h - theme.panel_header_height - theme.gap_standard * 2.0,
            );
            let mut y = body.y;
            for measure in &tool_options {
                let h = measure_window_measure_height(theme, &self.collapsed_sections, measure);
                self.render_window_measure(
                    chrome,
                    select_overlay,
                    atlas,
                    icons,
                    input,
                    theme,
                    Rect::new(body.x, y, body.w, h),
                    measure,
                    gpu,
                );
                y += h;
            }
            chrome.end_glass_content();
        })(draw, overlay)
    }

    fn render_window_measure(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        measure: &WindowMeasure,
        gpu: &mut ui_wgpu::GpuContext,
    ) -> f32 {
        use ui_wgpu::component::layout::MeasureSelectItem;
        use ui_wgpu::widgets::{render_widget, ControlNode, WidgetNode};
        let height = measure_window_measure_height(theme, &self.collapsed_sections, measure);
        let mut y = bounds.y;
        match measure {
            WindowMeasure::Group {
                id,
                label,
                default_open,
                children,
                ..
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
                        let child_h =
                            measure_window_measure_height(theme, &self.collapsed_sections, child);
                        self.render_window_measure(
                            draw, overlay, atlas, icons, input, theme,
                            Rect::new(bounds.x + 12.0, y, bounds.w - 12.0, child_h),
                            child, gpu,
                        );
                        y += child_h;
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
        height
    }

    fn render_window_engagement_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        kind: &semio_framework_core::WindowKindDefinition,
        measures_reserve: f32,
        gpu: &mut ui_wgpu::GpuContext,
    ) -> Option<(Rect, String)> {
        let inset = theme.gap_standard;
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
        let engagement = self.engagement_for_kind(kind);
        let Some(engagement) = engagement else {
            return None;
        };
        let activated = self
            .engagement_activated
            .get(window_id)
            .copied()
            .unwrap_or(false);
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            if !activated {
                let item = ChromeGroupItem {
                    control_id: "",
                    icon_id: Some("chevron-right"),
                    label: Some("Action"),
                    active: false,
                    disabled: false,
                    kind: HitKind::Button,
                };
                let chip_w = measure_chrome_group_item(atlas, theme, &item);
                let chip = Rect::new(
                    content.x + inset,
                    content.y + inset,
                    chip_w,
                    theme.control_height,
                );
                render_chrome_group(chrome, atlas, icons, input, theme, chip, &[item], false);
                return Some((chip, format!("shell.engagement.toggle.{window_id}")));
            }
            let rail_w = engagement_rail_width(theme, content.w, inset, measures_reserve);
            if rail_w <= 0.0 {
                return None;
            }
            let body_content_h = measure_engagement_body_height(theme, &engagement);
            let card_h = theme.panel_header_height + theme.gap_standard * 2.0 + body_content_h;
            let rail_h = card_h.min((content.h - inset * 2.0).max(theme.panel_header_height));
            let rail = Rect::new(content.x + inset, content.y + inset, rail_w, rail_h);
            let glass = chrome.push_glass(
                [rail.x, rail.y, rail.w, rail.h],
                theme.border_radius,
                GlassTier::WindowOptions,
                theme,
            );
            chrome.begin_glass_content(glass);
            let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
            chrome.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
            let toggle_item = ChromeGroupItem {
                control_id: "shell.engagement.toggle",
                icon_id: Some("chevron-left"),
                label: Some("Action"),
                active: false,
                disabled: false,
                kind: HitKind::Button,
            };
            let toggle_w = measure_chrome_group_item(atlas, theme, &toggle_item);
            let toggle_rect = Rect::new(header.x, header.y, toggle_w, header.h);
            render_chrome_group(
                chrome,
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
            let mut y = rail.y + theme.panel_header_height + theme.gap_standard;
            if let Some(options) = &engagement.options {
                for option in options {
                    let label = option.label.clone().unwrap_or_else(|| option.id.clone());
                    let pressed = option.pressed.unwrap_or(false);
                    let item = ChromeGroupItem {
                        control_id: "shell.engagement.option",
                        icon_id: None,
                        label: Some(&label),
                        active: pressed,
                        disabled: false,
                        kind: HitKind::Button,
                    };
                    let item_w = measure_chrome_group_item(atlas, theme, &item);
                    let rect = Rect::new(rail.x + 8.0, y, item_w, theme.control_height);
                    render_chrome_group(chrome, atlas, icons, input, theme, rect, &[item], true);
                    if let Some(action) = &option.action {
                        input.register_hit(HitTarget {
                            rect,
                            event: Some(action.clone()),
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
                    chrome, select_overlay, atlas, icons, input, theme,
                    Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height * 2.0),
                    window_id, input_spec, gpu,
                );
                y += theme.control_height * 2.0 + 8.0;
            }
            if let Some(control) = &engagement.control {
                self.render_engagement_control(
                    chrome, select_overlay, atlas, icons, input, theme,
                    Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height),
                    control, gpu,
                );
                y += theme.control_height;
            }
            if let Some(status_rows) = &engagement.status {
                for row in status_rows {
                    y += theme.control_height;
                    chrome_text(
                        chrome, atlas, input, theme, &row.text,
                        rail.x + 8.0, y, theme.font_size_small, theme.text_muted,
                    );
                }
            }
            if let Some(possibles) = &engagement.possible_engagements {
                for possible in possibles {
                    y += theme.control_height + 2.0;
                    let rect = Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height);
                    chrome.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius);
                    chrome_text(
                        chrome, atlas, input, theme, &possible.label,
                        rect.x + 8.0, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0,
                        theme.font_size_small, theme.text,
                    );
                    if let Some(action) = &possible.action {
                        input.register_hit(HitTarget {
                            rect,
                            event: Some(action.clone()),
                            control_id: Some(format!("shell.engagement.possible.{}.{}", window_id, possible.id)),
                            kind: HitKind::Button,
                            drag_axis: None,
                            drag_data: None,
                        });
                    }
                }
            }
            chrome.end_glass_content();
            None
        })(draw, overlay)
    }

    fn render_engagement_input(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
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
        input: &mut InputState<ActionDescriptor>,
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

    // #region ActionsRail
    /// 📇 Renders a window's Actions rail (Architecture Decision 8, P1) anchored bottom-right — the free
    /// corner (measures top-right, engagement top-left, toolbar bottom-left). Folded to a chip by
    /// default; unfolded it lists window-scoped actions in manifest order: zero-arg rows ARE the execute
    /// button, arg-carrying rows are accordion disclosures over a staged form. Returns the fold chip hit.
    fn render_window_actions_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        app: &semio_framework_core::AppDefinition,
        kind: &semio_framework_core::WindowKindDefinition,
    ) -> Option<(Rect, String)> {
        let actions: Vec<semio_framework_core::ActionDefinition> =
            semio_framework_core::resolve_window_actions(app, kind)
                .into_iter()
                .cloned()
                .collect();
        if actions.is_empty() {
            return None;
        }
        let inset = theme.gap_standard;
        let row_h = theme.control_height;
        let folded = self.action_panel_folded.get(window_id).copied().unwrap_or(true);
        let enabled = self.actions_enabled_for_window(app, window_id);
        let expanded_action = self.action_panel_expanded.get(window_id).cloned();
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            if folded {
                let item = ChromeGroupItem {
                    control_id: "",
                    icon_id: Some("chevron-up"),
                    label: Some("Actions"),
                    active: false,
                    disabled: false,
                    kind: HitKind::Button,
                };
                let chip_w = measure_chrome_group_item(atlas, theme, &item);
                let chip = Rect::new(
                    content.x + content.w - chip_w - inset,
                    content.y + content.h - row_h - inset,
                    chip_w,
                    row_h,
                );
                render_chrome_group(chrome, atlas, icons, input, theme, chip, &[item], false);
                return Some((chip, format!("shell.action.fold.{window_id}")));
            }
            let width = theme
                .window_measures_default_width
                .clamp(theme.panel_min_width, theme.panel_max_width)
                .min(window_overlay_max_width(content.w, inset));
            let mut body_h = theme.gap_standard;
            for action in &actions {
                body_h += row_h;
                if expanded_action.as_deref() == Some(action.id.as_str()) {
                    body_h += self.staged_form_height(theme, action);
                }
            }
            let card_h = (theme.panel_header_height + body_h + theme.gap_standard)
                .min((content.h - inset * 2.0).max(theme.panel_header_height));
            let rail = Rect::new(
                content.x + content.w - width - inset,
                content.y + content.h - card_h - inset,
                width,
                card_h,
            );
            let glass = chrome.push_glass(
                [rail.x, rail.y, rail.w, rail.h],
                theme.border_radius,
                GlassTier::WindowOptions,
                theme,
            );
            chrome.begin_glass_content(glass);
            let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
            chrome.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
            let fold_item = ChromeGroupItem {
                control_id: "shell.action.fold",
                icon_id: Some("chevron-down"),
                label: Some("Actions"),
                active: false,
                disabled: false,
                kind: HitKind::Button,
            };
            let fold_w = measure_chrome_group_item(atlas, theme, &fold_item);
            render_chrome_group(
                chrome,
                atlas,
                icons,
                input,
                theme,
                Rect::new(header.x, header.y, fold_w, header.h),
                &[fold_item],
                true,
            );
            input.register_hit(HitTarget {
                rect: Rect::new(header.x, header.y, fold_w, header.h),
                event: None,
                control_id: Some(format!("shell.action.fold.{window_id}")),
                kind: HitKind::Button,
                drag_axis: None,
                drag_data: None,
            });
            let mut y = rail.y + theme.panel_header_height + theme.gap_standard;
            let body_x = rail.x + theme.gap_standard;
            let body_w = rail.w - theme.gap_standard * 2.0;
            for action in &actions {
                let is_expanded = expanded_action.as_deref() == Some(action.id.as_str());
                let has_args = !action.args.is_empty();
                let row = Rect::new(body_x, y, body_w, row_h);
                let icon = if !has_args {
                    action.icon_id.as_deref()
                } else if is_expanded {
                    Some("chevron-down")
                } else {
                    Some("chevron-right")
                };
                let item = ChromeGroupItem {
                    control_id: "",
                    icon_id: icon,
                    label: Some(action.label.as_str()),
                    active: is_expanded,
                    disabled: !enabled,
                    kind: HitKind::Button,
                };
                render_chrome_group(chrome, atlas, icons, input, theme, row, &[item], false);
                if enabled {
                    let control_id = if has_args {
                        format!("shell.action.expand::{window_id}::{}", action.id)
                    } else {
                        format!("shell.action.exec::{window_id}::{}", action.id)
                    };
                    input.register_hit(HitTarget {
                        rect: row,
                        event: None,
                        control_id: Some(control_id),
                        kind: HitKind::Button,
                        drag_axis: None,
                        drag_data: None,
                    });
                }
                y += row_h;
                if is_expanded && has_args {
                    y += self.render_staged_form(
                        chrome, select_overlay, atlas, icons, input, theme,
                        Rect::new(body_x, y, body_w, self.staged_form_height(theme, action)),
                        window_id, action, enabled,
                    );
                }
            }
            chrome.end_glass_content();
            None
        })(draw, overlay)
    }

    /// 📝 Total height of one action's staged arg form (per-arg fields + the Execute/Reset row).
    fn staged_form_height(&self, theme: &Theme, action: &semio_framework_core::ActionDefinition) -> f32 {
        let mut h = theme.gap_standard;
        for arg in &action.args {
            h += self.staged_arg_height(theme, arg);
        }
        h + theme.control_height + theme.gap_standard
    }

    fn staged_arg_height(&self, theme: &Theme, arg: &semio_framework_core::ActionArgDef) -> f32 {
        match arg.control {
            semio_framework_core::ActionArgControl::Toggle => theme.control_height + theme.gap_standard,
            _ => theme.control_height * 2.0 + theme.gap_standard,
        }
    }

    /// 📝 The effective value of one arg (staged if present, else the declared default).
    fn effective_arg_value(
        &self,
        window_id: &str,
        action_id: &str,
        arg: &semio_framework_core::ActionArgDef,
    ) -> Option<serde_json::Value> {
        self.staged_action_args
            .get(&Self::staged_key(window_id, action_id))
            .and_then(|map| map.get(&arg.id).cloned())
            .or_else(|| arg.default.clone())
    }

    fn arg_default(&self, action_id: &str, arg_id: &str) -> Option<serde_json::Value> {
        self.session
            .as_ref()?
            .app
            .actions
            .iter()
            .find(|action| action.id == action_id)?
            .args
            .iter()
            .find(|arg| arg.id == arg_id)?
            .default
            .clone()
    }

    /// 📝 Renders the staged form for one expanded action and returns its consumed height. Every control
    /// writes to STAGING via shell-owned hit ids (never a live `on_change` dispatch) — P2.
    fn render_staged_form(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        action: &semio_framework_core::ActionDefinition,
        enabled: bool,
    ) -> f32 {
        let row_h = theme.control_height;
        let mut y = bounds.y + theme.gap_standard;
        for arg in &action.args {
            let arg_h = self.staged_arg_height(theme, arg);
            self.render_staged_arg(
                draw, overlay, atlas, icons, input, theme,
                Rect::new(bounds.x, y, bounds.w, arg_h),
                window_id, &action.id, arg, enabled,
            );
            y += arg_h;
        }
        // Execute / Reset row.
        let staged = self.staged_map_for(window_id, &action.id);
        let executable = Self::resolved_execute_args(&action.args, &staged).is_some();
        let exec_item = ChromeGroupItem {
            control_id: "",
            icon_id: Some("play"),
            label: Some("Execute"),
            active: false,
            disabled: !(enabled && executable),
            kind: HitKind::Button,
        };
        let exec_w = measure_chrome_group_item(atlas, theme, &exec_item);
        let exec_rect = Rect::new(bounds.x, y, exec_w, row_h);
        render_chrome_group(draw, atlas, icons, input, theme, exec_rect, &[exec_item], false);
        if enabled && executable {
            input.register_hit(HitTarget {
                rect: exec_rect,
                event: None,
                control_id: Some(format!("shell.action.exec::{window_id}::{}", action.id)),
                kind: HitKind::Button,
                drag_axis: None,
                drag_data: None,
            });
        }
        let reset_item = ChromeGroupItem {
            control_id: "",
            icon_id: Some("rotate-ccw"),
            label: Some("Reset"),
            active: false,
            disabled: false,
            kind: HitKind::Button,
        };
        let reset_w = measure_chrome_group_item(atlas, theme, &reset_item);
        let reset_rect = Rect::new(bounds.x + exec_w + theme.gap_standard, y, reset_w, row_h);
        render_chrome_group(draw, atlas, icons, input, theme, reset_rect, &[reset_item], false);
        input.register_hit(HitTarget {
            rect: reset_rect,
            event: None,
            control_id: Some(format!("shell.action.reset::{window_id}::{}", action.id)),
            kind: HitKind::Button,
            drag_axis: None,
            drag_data: None,
        });
        self.staged_form_height(theme, action)
    }

    fn render_staged_arg(
        &mut self,
        draw: &mut DrawList,
        _overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        action_id: &str,
        arg: &semio_framework_core::ActionArgDef,
        enabled: bool,
    ) {
        use semio_framework_core::ActionArgControl;
        let row_h = theme.control_height;
        let effective = self.effective_arg_value(window_id, action_id, arg);
        match &arg.control {
            ActionArgControl::Toggle => {
                let on = effective.as_ref().and_then(|v| v.as_bool()).unwrap_or(false);
                let label = format!("{} · {}", arg.label, if on { "on" } else { "off" });
                let item = ChromeGroupItem {
                    control_id: "",
                    icon_id: Some(if on { "check-square" } else { "square" }),
                    label: Some(label.as_str()),
                    active: on,
                    disabled: !enabled,
                    kind: HitKind::Button,
                };
                let item_w = measure_chrome_group_item(atlas, theme, &item).min(bounds.w);
                let rect = Rect::new(bounds.x, bounds.y, item_w, row_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], false);
                if enabled {
                    input.register_hit(HitTarget {
                        rect,
                        event: None,
                        control_id: Some(format!(
                            "shell.action.argtoggle::{window_id}::{action_id}::{}",
                            arg.id
                        )),
                        kind: HitKind::Button,
                        drag_axis: None,
                        drag_data: None,
                    });
                }
            }
            ActionArgControl::Select { options } => {
                chrome_text(draw, atlas, input, theme, &arg.label, bounds.x, bounds.y + 14.0, theme.font_size_small, theme.text_muted);
                let effective_str = effective.as_ref().and_then(|v| v.as_str()).map(String::from);
                let mut x = bounds.x;
                let chip_y = bounds.y + row_h;
                for option in options {
                    let active = effective_str.as_deref() == Some(option.value.as_str());
                    let item = ChromeGroupItem {
                        control_id: "",
                        icon_id: None,
                        label: Some(option.label.as_str()),
                        active,
                        disabled: !enabled,
                        kind: HitKind::Button,
                    };
                    let item_w = measure_chrome_group_item(atlas, theme, &item);
                    let rect = Rect::new(x, chip_y, item_w, row_h);
                    render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], false);
                    if enabled {
                        input.register_hit(HitTarget {
                            rect,
                            event: None,
                            control_id: Some(format!(
                                "shell.action.argselect::{window_id}::{action_id}::{}::{}",
                                arg.id, option.value
                            )),
                            kind: HitKind::Button,
                            drag_axis: None,
                            drag_data: None,
                        });
                    }
                    x += item_w + theme.gap_standard;
                }
            }
            ActionArgControl::IconSelect { .. } => {
                // Icon classifiers are not enumerable at manifest altitude; fall back to a text field.
                let value = self.staged_arg_display_string(window_id, action_id, arg, input, None);
                self.render_staged_text_field(
                    draw, atlas, icons, input, theme, bounds, window_id, action_id, arg, &value, enabled, None,
                );
            }
            ActionArgControl::Vec3 => {
                chrome_text(draw, atlas, input, theme, &arg.label, bounds.x, bounds.y + 14.0, theme.font_size_small, theme.text_muted);
                let arr = effective.as_ref().and_then(|v| v.as_array());
                let field_w = ((bounds.w - theme.gap_standard * 2.0) / 3.0).max(24.0);
                for axis in 0..3usize {
                    let control_id = format!(
                        "shell.action.argvec3::{window_id}::{action_id}::{}::{axis}",
                        arg.id
                    );
                    let focused = input.focused_id.as_deref() == Some(control_id.as_str());
                    let display = if focused {
                        input.text_buffer.clone()
                    } else {
                        fmt_num(arr.and_then(|a| a.get(axis)).and_then(|v| v.as_f64()).unwrap_or(0.0))
                    };
                    let rect = Rect::new(
                        bounds.x + axis as f32 * (field_w + theme.gap_standard),
                        bounds.y + row_h,
                        field_w,
                        row_h,
                    );
                    self.paint_staged_input_box(draw, atlas, input, theme, rect, &display, focused, enabled, &control_id);
                }
            }
            _ => {
                // Text / Number / Slider → a single focusable input, staged on commit.
                let control_id = format!("shell.action.arginput::{window_id}::{action_id}::{}", arg.id);
                let focused = input.focused_id.as_deref() == Some(control_id.as_str());
                let display = self.staged_arg_display_string(window_id, action_id, arg, input, Some(&control_id));
                self.render_staged_text_field(
                    draw, atlas, icons, input, theme, bounds, window_id, action_id, arg, &display, enabled, Some(focused),
                );
            }
        }
    }

    /// 📝 The current display string of a scalar arg — the live focus buffer if focused, else the
    /// effective staged/default value.
    fn staged_arg_display_string(
        &self,
        window_id: &str,
        action_id: &str,
        arg: &semio_framework_core::ActionArgDef,
        input: &InputState<ActionDescriptor>,
        control_id: Option<&str>,
    ) -> String {
        if let Some(control_id) = control_id {
            if input.focused_id.as_deref() == Some(control_id) {
                return input.text_buffer.clone();
            }
        }
        match self.effective_arg_value(window_id, action_id, arg) {
            Some(serde_json::Value::String(text)) => text,
            Some(serde_json::Value::Number(num)) => num.to_string(),
            Some(serde_json::Value::Bool(flag)) => flag.to_string(),
            Some(other) => other.to_string(),
            None => String::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_staged_text_field(
        &mut self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        action_id: &str,
        arg: &semio_framework_core::ActionArgDef,
        display: &str,
        enabled: bool,
        focused_override: Option<bool>,
    ) {
        let _ = icons;
        chrome_text(draw, atlas, input, theme, &arg.label, bounds.x, bounds.y + 14.0, theme.font_size_small, theme.text_muted);
        let control_id = format!("shell.action.arginput::{window_id}::{action_id}::{}", arg.id);
        let focused = focused_override.unwrap_or_else(|| input.focused_id.as_deref() == Some(control_id.as_str()));
        let rect = Rect::new(bounds.x, bounds.y + theme.control_height, bounds.w, theme.control_height);
        self.paint_staged_input_box(draw, atlas, input, theme, rect, display, focused, enabled, &control_id);
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_staged_input_box(
        &self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        rect: Rect,
        display: &str,
        focused: bool,
        enabled: bool,
        control_id: &str,
    ) {
        draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.input_bg, theme.border_radius);
        if focused {
            let hair = theme.stroke_hairline * 2.0;
            draw.push_solid([rect.x, rect.y + rect.h - hair, rect.w, hair], theme.accent);
        }
        let text_color = if enabled { theme.text } else { theme.text_muted };
        chrome_text(
            draw, atlas, input, theme, display,
            rect.x + theme.padding_standard,
            rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0,
            theme.font_size_small, text_color,
        );
        if enabled {
            input.register_hit(HitTarget {
                rect,
                event: None,
                control_id: Some(control_id.to_string()),
                kind: HitKind::Input,
                drag_axis: None,
                drag_data: None,
            });
        }
    }
    // #endregion

    fn render_context_menu(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<ActionDescriptor>,
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
                event: item.action.clone(),
                control_id: Some(item.id.clone()),
                kind: HitKind::ContextMenu,
                drag_axis: None,
            drag_data: None,
            });
        }
    }

    fn render_palette(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<ActionDescriptor>,
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
fn download_media_export(filename: &str, mime_type: &str, data: &str, _encoding: Option<&str>) {
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
fn download_media_export(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    if let Some(path) = rfd::FileDialog::new()
        .set_file_name(filename)
        .add_filter("export", &[mime_type.rsplit_once('/').map(|(_, ext)| ext).unwrap_or("dat")])
        .save_file()
    {
        use base64::Engine;
        let bytes = if encoding == Some("base64") {
            base64::engine::general_purpose::STANDARD.decode(data).unwrap_or_else(|_| data.as_bytes().to_vec())
        } else {
            data.as_bytes().to_vec()
        };
        let _ = std::fs::write(path, bytes);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn request_file_save(filename: &str) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_file_name(filename)
        .add_filter("studio", &["json"])
        .save_file()
}

#[cfg(target_arch = "wasm32")]
fn request_file_save(_filename: &str) -> Option<std::path::PathBuf> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn pick_folder() -> Option<String> {
    rfd::FileDialog::new().pick_folder().map(|path| path.display().to_string())
}

#[cfg(target_arch = "wasm32")]
fn pick_folder() -> Option<String> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn request_file_open(accept: &str, read_as: Option<&str>) -> Option<String> {
    let extensions: Vec<&str> = accept
        .split(',')
        .filter_map(|entry| entry.trim().strip_prefix('.'))
        .collect();
    let mut dialog = rfd::FileDialog::new();
    if !extensions.is_empty() {
        dialog = dialog.add_filter("import", &extensions);
    }
    let path = dialog.pick_file()?;
    if read_as == Some("dataUrl") {
        use base64::Engine;
        let bytes = std::fs::read(&path).ok()?;
        let mime = extensions.first().map(|ext| format!("application/{ext}")).unwrap_or_else(|| "application/octet-stream".into());
        return Some(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)));
    }
    std::fs::read_to_string(path).ok()
}

#[cfg(target_arch = "wasm32")]
fn request_file_open(_accept: &str, _read_as: Option<&str>) -> Option<String> {
    None
}

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

pub mod icon_atlas {
// #region icon_atlas
//! 🖼 CPU-rasterized Lucide icon atlas for native and web wgpu shells.

use ui_wgpu::IconAtlas;

const ICON_SIZE: u32 = 24;
const ATLAS_COLS: u32 = 16;
const ICON_ATLAS_TEXTURE_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/icons_generated.rs"));

fn rasterize_svg(svg: &str, tint_mask: bool) -> Option<Vec<u8>> {
    let mut options = usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg, &options).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(ICON_SIZE, ICON_SIZE)?;
    let scale = (ICON_SIZE as f32 / tree.size().width()).min(ICON_SIZE as f32 / tree.size().height());
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let mut pixels = pixmap.take();
    if tint_mask {
        for chunk in pixels.chunks_mut(4) {
            let alpha = chunk[3];
            chunk[0] = 255;
            chunk[1] = 255;
            chunk[2] = 255;
            chunk[3] = alpha;
        }
    }
    Some(pixels)
}

pub fn build_icon_atlas() -> IconAtlas {
    let mut loaded: Vec<(&str, Vec<u8>)> = Vec::new();
    for (id, svg) in ICON_SVGS {
        let Some(pixels) = rasterize_svg(svg, *id != "semio-logo") else {
            continue;
        };
        loaded.push((id, pixels));
    }
    if let Some(pixels) = rasterize_svg(SEMIO_LOGO_SVG, false) {
        loaded.push(("semio-logo", pixels));
    }
    let rows = loaded.len().div_ceil(ATLAS_COLS as usize);
    let width = ATLAS_COLS * ICON_SIZE;
    let height = (rows as u32).max(1) * ICON_SIZE;
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    let mut entries = Vec::new();
    for (index, (id, icon_pixels)) in loaded.into_iter().enumerate() {
        let col = (index as u32) % ATLAS_COLS;
        let row = (index as u32) / ATLAS_COLS;
        let ox = col * ICON_SIZE;
        let oy = row * ICON_SIZE;
        for y in 0..ICON_SIZE {
            for x in 0..ICON_SIZE {
                let src = ((y * ICON_SIZE + x) * 4) as usize;
                let dst = (((oy + y) * width + (ox + x)) * 4) as usize;
                pixels[dst] = icon_pixels[src];
                pixels[dst + 1] = icon_pixels[src + 1];
                pixels[dst + 2] = icon_pixels[src + 2];
                pixels[dst + 3] = icon_pixels[src + 3];
            }
        }
        let texture = ICON_ATLAS_TEXTURE_SIZE as f32;
        entries.push((
            id.to_string(),
            [
                ox as f32 / texture,
                oy as f32 / texture,
                (ox + ICON_SIZE) as f32 / texture,
                (oy + ICON_SIZE) as f32 / texture,
            ],
        ));
    }
    IconAtlas::from_packed(width, height, pixels, entries)
}
// #endregion icon_atlas
}


use plugin_bridge::filter_plugins;
#[cfg(target_arch = "wasm32")]
use plugin_bridge::parse_plugin_entries;
#[cfg(not(target_arch = "wasm32"))]
use plugin_bridge::load_wasm_plugins;
use infinite_world::{
    apply_glb_bytes, apply_world_action_preview, collect_pending_glb_fetches, fetch_url_bytes,
    handle_world3d_paint_actions, handle_world3d_pointer_button, handle_world3d_pointer_drag,
    handle_world3d_pointer_move, handle_world3d_wheel,
};
use ui_wgpu::ActionDescriptor;
use shell::ShellState;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use ui_wgpu::{
    apply_window_cursor, dispatch_window_event, fetch_font_bytes, resolve_semio_cursor, schedule_frame,
    CursorDragState, DrawList, FontAtlas, GpuContext, IconAtlas, InputState, KeyAction, PointerCallbacks,
    PointerModifiers, SemioCursor, Theme, WindowInputState,
};
#[cfg(target_arch = "wasm32")]
use ui_wgpu::apply_canvas_cursor;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowId};

fn spawn_app_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    #[cfg(target_arch = "wasm32")]
    spawn_local(future);
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(future);
}

#[cfg(target_arch = "wasm32")]
fn log_debug(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn log_debug(message: &str) {
    eprintln!("{message}");
}

#[cfg(target_arch = "wasm32")]
fn prefers_dark_scheme() -> bool {
    web_sys::window()
        .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|query| query.matches())
        .unwrap_or(true)
}

#[cfg(not(target_arch = "wasm32"))]
fn prefers_dark_scheme() -> bool {
    true
}

fn resolve_theme(appearance_id: &str) -> Theme {
    match appearance_id {
        "light" => Theme::light(),
        "dark" => Theme::dark(),
        _ if prefers_dark_scheme() => Theme::dark(),
        _ => Theme::light(),
    }
}

fn appearance_is_dark(appearance_id: &str) -> bool {
    match appearance_id {
        "light" => false,
        "dark" => true,
        _ => prefers_dark_scheme(),
    }
}

#[cfg(target_arch = "wasm32")]
fn app_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn app_now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

struct AppRuntime {
    gpu: GpuContext,
    atlas: FontAtlas,
    icons: IconAtlas,
    shell: ShellState,
    draw: DrawList,
    overlay: DrawList,
    input: InputState<ActionDescriptor>,
    theme: Theme,
    window: Arc<Window>,
    theme_dark: bool,
    last_cursor: Option<(SemioCursor, bool)>,
    last_pointer_x: f32,
    last_pointer_y: f32,
    pointer_down: bool,
    pointer_button: i16,
    modifiers: PointerModifiers,
    wheel_delta: f32,
    space_pressed: bool,
    wheel_zoom_deadline_ms: f64,
    caret_blink_at_ms: f64,
    caret_blink_visible: bool,
    asset_poll_pending: bool,
    self_weak: std::rc::Weak<RefCell<AppRuntime>>,
    #[cfg(not(target_arch = "wasm32"))]
    plugin_modules_root: std::path::PathBuf,
    #[cfg(not(target_arch = "wasm32"))]
    native_plugin_mtimes: std::collections::HashMap<std::path::PathBuf, std::time::SystemTime>,
    #[cfg(not(target_arch = "wasm32"))]
    native_reload_pending: bool,
}

#[cfg(not(target_arch = "wasm32"))]
fn fetch_map_tile_bytes_blocking(url: &str) -> Option<Vec<u8>> {
    let resolved = resolve_map_tile_fetch_url(url);
    if !resolved.starts_with("http://") && !resolved.starts_with("https://") {
        return None;
    }
    let mut response = ureq::get(&resolved).call().ok()?;
    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes).ok()?;
    Some(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
fn resolve_map_tile_fetch_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }
    if url.starts_with('/') {
        let base = std::env::var("SEMIO_GIS_MAP_TILE_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:6141".to_string());
        return format!("{}{}", base.trim_end_matches('/'), url);
    }
    url.to_string()
}

#[cfg(target_arch = "wasm32")]
fn fetch_map_tile_bytes_blocking(_url: &str) -> Option<Vec<u8>> {
    None
}

impl AppRuntime {
    #[cfg(not(target_arch = "wasm32"))]
    fn poll_native_plugin_hot_swap(&mut self) {
        let mut changed = false;
        for plugin in &self.shell.plugins {
            let Some(path) = plugin.wasm_artifact_path() else {
                continue;
            };
            let Ok(metadata) = std::fs::metadata(path) else {
                continue;
            };
            let Ok(mtime) = metadata.modified() else {
                continue;
            };
            let previous = self.native_plugin_mtimes.get(path);
            if previous.is_some_and(|previous| *previous != mtime) {
                changed = true;
            }
            self.native_plugin_mtimes.insert(path.to_path_buf(), mtime);
        }
        if changed {
            self.native_reload_pending = true;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn maybe_reload_native_plugins(&mut self) {
        if !self.native_reload_pending {
            return;
        }
        self.native_reload_pending = false;
        let plugin_filter = self.shell.plugin_filter.clone();
        let modules_root = self.plugin_modules_root.clone();
        let entries = match load_wasm_plugins(&plugin_filter, &modules_root) {
            Ok(entries) => filter_plugins(entries, &plugin_filter),
            Err(error) => {
                log_debug(&format!("[DEBUG] wasm plugin reload failed: {error}"));
                return;
            }
        };
        self.shell.prepare_hot_reload(entries);
        if let Err(error) = pollster::block_on(self.shell.boot()) {
            log_debug(&format!("[DEBUG] wasm plugin hot reload failed: {error}"));
        } else {
            log_debug("[DEBUG] wasm plugin hot reload complete");
        }
    }

    fn frame(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.poll_native_plugin_hot_swap();
            self.maybe_reload_native_plugins();
            pollster::block_on(self.shell.pump_sync_events());
        }
        self.theme = resolve_theme(&self.shell.appearance_id);
        self.theme_dark = appearance_is_dark(&self.shell.appearance_id);
        if !self.pointer_down && self.input.drag.active {
            self.input.end_drag();
        }
        self.input.update_hover(self.last_pointer_x, self.last_pointer_y);
        self.input.clear_frame();
        if self.wheel_zoom_deadline_ms > 0.0 && app_now_ms() >= self.wheel_zoom_deadline_ms {
            self.wheel_zoom_deadline_ms = 0.0;
            engine_canvas::node_graph_clear_wheel_zoom_active();
        }
        if app_now_ms() - self.caret_blink_at_ms >= 500.0 {
            self.caret_blink_at_ms = app_now_ms();
            self.caret_blink_visible = !self.caret_blink_visible;
            engine_canvas::node_graph_sync_caret_blink(self.caret_blink_visible);
        }
        self.draw.clear();
        self.overlay.clear();
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
        let wheel_delta = self.wheel_delta;
        self.wheel_delta = 0.0;
        if wheel_delta.abs() > 0.0 {
            let x = self.last_pointer_x;
            let y = self.last_pointer_y;
            let ctrl = self.modifiers.ctrl;
            self.shell
                .handle_pointer_wheel(x, y, wheel_delta, &self.input);
            if ShellState::wheel_propagates_to_scene_surface(self.input.hit_at(x, y)) {
                for state in self.shell.world3d_states.values_mut() {
                    if state.bounds.contains(x, y) {
                        handle_world3d_wheel(state, wheel_delta);
                    }
                }
                let mut graph_actions = Vec::new();
                for (surface_id, surface) in &self.shell.node_graph_states {
                    if surface.bounds.contains(x, y) {
                        graph_actions.extend(engine_canvas::node_graph_wheel(
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
                if !graph_actions.is_empty() {
                    self.wheel_zoom_deadline_ms = app_now_ms() + 120.0;
                    let runtime = self.self_weak.clone();
                    spawn_app_task(async move {
                        if let Some(runtime) = runtime.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.dispatch_actions(graph_actions).await;
                            }
                        }
                    });
                }
                let mut map_actions = Vec::new();
                for (surface_id, surface) in &self.shell.gis_map_states {
                    if surface.bounds.contains(x, y) {
                        map_actions.extend(engine_canvas::gis_map_wheel(
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
                if !map_actions.is_empty() {
                    let runtime = self.self_weak.clone();
                    spawn_app_task(async move {
                        if let Some(runtime) = runtime.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.dispatch_actions(map_actions).await;
                            }
                        }
                    });
                }
                let mut board_actions = Vec::new();
                for (surface_id, surface) in &self.shell.puzzle2d_board_states {
                    if surface.bounds.contains(x, y) {
                        board_actions.extend(scenes::puzzle_board_wheel(surface_id, &surface.controller_id, surface.bounds, x, y, wheel_delta));
                    }
                }
                if !board_actions.is_empty() {
                    let runtime = self.self_weak.clone();
                    spawn_app_task(async move {
                        if let Some(runtime) = runtime.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.dispatch_actions(board_actions).await;
                            }
                        }
                    });
                }
            }
        }
        for upload in scenes::drain_pending_raster_uploads() {
            self.gpu.ensure_raster_texture(&upload.key, &upload.pixels, upload.width, upload.height);
        }
        if self.atlas.take_dirty() {
            self.gpu.upload_font_atlas(&self.atlas);
        }
        let time_seconds = (app_now_ms() / 1000.0) as f32;
        if let Err(err) = self.gpu.render_frame(&self.draw, Some(&self.overlay), time_seconds) {
            log_debug(&format!("[DEBUG] render frame: {err}"));
        }
        let hit = self
            .input
            .hit_at(self.last_pointer_x, self.last_pointer_y);
        let base_cursor = resolve_semio_cursor(
            hit,
            CursorDragState {
                tree_drag: self.shell.tree_drag.is_some(),
                dock_drag: self.shell.dock_drag.is_some(),
                pointer_drag_active: self.input.drag.active,
                pointer_drag_axis: self.input.drag.axis,
                pointer_drag_kind: self.input.drag.kind,
            },
        );
        // 🖱️ The active tool's cursor overrides generic body cursors while the pointer is over the
        // window body (P5), but never a specific control cursor (text inputs, resize handles).
        let cursor = match self.shell.tool_cursor_override(self.last_pointer_x, self.last_pointer_y) {
            Some(tool_cursor)
                if matches!(
                    base_cursor,
                    SemioCursor::Default | SemioCursor::Grab | SemioCursor::Selectable | SemioCursor::Pointer
                ) =>
            {
                tool_cursor
            }
            _ => base_cursor,
        };
        apply_window_cursor(
            &self.window,
            cursor,
            self.theme_dark,
            &mut self.last_cursor,
        );
        if !self.asset_poll_pending {
            self.poll_pending_assets();
        }
    }

    fn poll_pending_assets(&mut self) {
        let mut glb = collect_pending_glb_fetches(&self.shell.world3d_states);
        glb.extend(collect_pending_glb_fetches(&self.shell.icon_render_states));
        let map = engine_canvas::collect_pending_map_tile_fetches();
        if glb.is_empty() && map.is_empty() {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            for item in map {
                let url = resolve_map_tile_fetch_url(&item.url);
                if let Some(bytes) = fetch_map_tile_bytes_blocking(&url) {
                    engine_canvas::apply_map_tile_bytes(&item.surface_id, &item, &bytes);
                }
            }
            for item in glb {
                if let Some(bytes) = pollster::block_on(fetch_url_bytes(&item.url)) {
                    if let Some(state) = self.shell.world3d_states.get_mut(&item.surface_id) {
                        apply_glb_bytes(state, &item.url, &bytes);
                    } else if let Some(state) = self.shell.icon_render_states.get_mut(&item.surface_id) {
                        apply_glb_bytes(state, &item.url, &bytes);
                    }
                }
            }
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.asset_poll_pending = true;
            let runtime = self.self_weak.clone();
            spawn_app_task(async move {
                struct AssetPollReset(std::rc::Weak<RefCell<AppRuntime>>);
                impl Drop for AssetPollReset {
                    fn drop(&mut self) {
                        if let Some(runtime) = self.0.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.asset_poll_pending = false;
                            }
                        }
                    }
                }
                let _reset = AssetPollReset(runtime.clone());
                let Some(runtime) = runtime.upgrade() else {
                    return;
                };
                let mut fetched_glb = Vec::new();
                for item in glb {
                    if let Some(bytes) = fetch_url_bytes(&item.url).await {
                        fetched_glb.push((item.surface_id, item.url, bytes));
                    }
                }
                let mut fetched_map = Vec::new();
                for item in map {
                    if let Some(bytes) = fetch_url_bytes(&item.url).await {
                        fetched_map.push((item, bytes));
                    }
                }
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    for (surface_id, url, bytes) in fetched_glb {
                        if let Some(state) = app.shell.world3d_states.get_mut(&surface_id) {
                            apply_glb_bytes(state, &url, &bytes);
                        } else if let Some(state) = app.shell.icon_render_states.get_mut(&surface_id) {
                            apply_glb_bytes(state, &url, &bytes);
                        }
                    }
                    for (fetch, bytes) in fetched_map {
                        engine_canvas::apply_map_tile_bytes(&fetch.surface_id, &fetch, &bytes);
                    }
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
        if let KeyAction::Space(pressed) = action {
            self.space_pressed = pressed;
            return;
        }
        if engine_canvas::node_graph_apply_note_edit_key(action.clone(), &modifiers) {
            return;
        }
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
            spawn_app_task(async move {
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        let _ = app.shell.activate_search_item(search_index).await;
                    }
                }
            });
        } else if activate_find {
            let runtime = self.self_weak.clone();
            spawn_app_task(async move {
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        let _ = app.shell.activate_find_item(find_index).await;
                    }
                }
            });
        }
    }

    async fn dispatch_actions(&mut self, actions: Vec<ActionDescriptor>) {
        for action in actions {
            for state in self.shell.world3d_states.values_mut() {
                if state.controller_id == action.controller_id {
                    apply_world_action_preview(state, &action);
                }
            }
            if let Err(err) = self.shell.dispatch_action(action).await {
                log_debug(&format!("[DEBUG] action failed: {err}"));
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
        if !down {
            let mut map_actions = Vec::new();
            let map_had_active_drag = self.shell.gis_map_states.keys().any(|surface_id| {
                scenes::gis_map_drag_active(surface_id)
            });
            for (surface_id, surface) in &self.shell.gis_map_states {
                if !surface.bounds.contains(x, y) && !scenes::gis_map_drag_active(surface_id) {
                    continue;
                }
                map_actions.extend(scenes::gis_map_pointer_up(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                ));
            }
            if !map_actions.is_empty() {
                self.dispatch_actions(map_actions).await;
            }
            let mut board_actions = Vec::new();
            let board_had_active_drag = self.shell.puzzle2d_board_states.keys().any(|surface_id| scenes::puzzle_board_drag_active(surface_id));
            for (surface_id, surface) in &self.shell.puzzle2d_board_states {
                if !surface.bounds.contains(x, y) && !scenes::puzzle_board_drag_active(surface_id) {
                    continue;
                }
                board_actions.extend(scenes::puzzle_board_pointer_up(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    modifiers.shift,
                    modifiers.ctrl_or_meta(),
                    modifiers.alt,
                ));
            }
            if !board_actions.is_empty() {
                self.dispatch_actions(board_actions).await;
            }
            let board_consumed = self
                .shell
                .puzzle2d_board_states
                .values()
                .any(|surface| surface.bounds.contains(x, y))
                || board_had_active_drag;
            let map_consumed = self
                .shell
                .gis_map_states
                .values()
                .any(|surface| surface.bounds.contains(x, y))
                || map_had_active_drag;
            if map_consumed || board_consumed {
                return;
            }
            if let Err(err) = self
                .shell
                .handle_pointer_button(x, y, down, button, &mut self.input, &self.theme)
                .await
            {
                log_debug(&format!("[DEBUG] pointer failed: {err}"));
            }
            let mut world_actions = Vec::new();
            for state in self.shell.world3d_states.values_mut() {
                if !state.bounds.contains(x, y) {
                    continue;
                }
                if let Some(action) = handle_world3d_pointer_button(
                    state,
                    x,
                    y,
                    down,
                    button,
                    &modifiers,
                ) {
                    apply_world_action_preview(state, &action);
                    world_actions.push(action);
                }
                for action in handle_world3d_paint_actions(state, x, y, down, button) {
                    apply_world_action_preview(state, &action);
                    world_actions.push(action);
                }
                if let Some(action) = handle_world3d_pointer_move(state, x, y, down, button) {
                    apply_world_action_preview(state, &action);
                    world_actions.push(action);
                }
            }
            if !world_actions.is_empty() {
                self.dispatch_actions(world_actions).await;
            }
            let mut graph_actions = Vec::new();
            for (surface_id, surface) in &self.shell.node_graph_states {
                if !surface.bounds.contains(x, y) {
                    continue;
                }
                graph_actions.extend(engine_canvas::node_graph_pointer_up(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    modifiers.shift,
                    modifiers.ctrl_or_meta(),
                    modifiers.alt,
                ));
            }
            if !graph_actions.is_empty() {
                self.dispatch_actions(graph_actions).await;
            }
            return;
        }
        let mut world_actions = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(action) = handle_world3d_pointer_button(
                state,
                x,
                y,
                down,
                button,
                &modifiers,
            ) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
            for action in handle_world3d_paint_actions(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
            if let Some(action) = handle_world3d_pointer_move(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
        }
        if !world_actions.is_empty() {
            self.dispatch_actions(world_actions).await;
            return;
        }
        let mut graph_actions = Vec::new();
        for (surface_id, surface) in &self.shell.node_graph_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            if down {
                graph_actions.extend(engine_canvas::node_graph_pointer_down(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    button,
                    modifiers.shift,
                    modifiers.ctrl_or_meta(),
                    modifiers.alt,
                    self.space_pressed,
                ));
            } else {
                graph_actions.extend(engine_canvas::node_graph_pointer_up(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    modifiers.shift,
                    modifiers.ctrl_or_meta(),
                    modifiers.alt,
                ));
            }
        }
        if !graph_actions.is_empty() {
            self.dispatch_actions(graph_actions).await;
        }
        let mut map_actions = Vec::new();
        let mut map_pointer_on_surface = false;
        for (surface_id, surface) in &self.shell.gis_map_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            map_pointer_on_surface = true;
            if down {
                map_actions.extend(scenes::gis_map_pointer_down(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    button,
                    modifiers.shift,
                    modifiers.ctrl_or_meta(),
                    &surface.selection_method,
                ));
            }
        }
        if !map_actions.is_empty() {
            self.dispatch_actions(map_actions).await;
            return;
        }
        if map_pointer_on_surface && (button == 0 || button == 1) {
            return;
        }
        let mut board_pointer_on_surface = false;
        for (surface_id, surface) in &self.shell.puzzle2d_board_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            board_pointer_on_surface = true;
            if down {
                scenes::puzzle_board_pointer_down(surface_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta());
            }
        }
        if board_pointer_on_surface && (button == 0 || button == 1) {
            return;
        }
        if let Err(err) = self
            .shell
            .handle_pointer_button(x, y, down, button, &mut self.input, &self.theme)
            .await
        {
            log_debug(&format!("[DEBUG] pointer failed: {err}"));
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
        if let Err(err) = self.shell.flush_deferred_actions().await {
            log_debug(&format!("[DEBUG] deferred actions: {err}"));
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
        let mut world_actions = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(action) = handle_world3d_pointer_move(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
            for action in handle_world3d_paint_actions(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
        }
        let mut graph_actions = Vec::new();
        for (surface_id, surface) in &self.shell.node_graph_states {
            if surface.bounds.contains(x, y) {
                graph_actions.extend(engine_canvas::node_graph_pointer_move(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    modifiers.shift,
                    modifiers.ctrl_or_meta(),
                    modifiers.alt,
                ));
            }
        }
        if !graph_actions.is_empty() {
            self.dispatch_actions(graph_actions).await;
        }
        let mut map_actions = Vec::new();
        for (surface_id, surface) in &self.shell.gis_map_states {
            if !surface.bounds.contains(x, y) && !scenes::gis_map_drag_active(surface_id) {
                continue;
            }
            map_actions.extend(scenes::gis_map_pointer_move(
                surface_id,
                &surface.controller_id,
                surface.bounds,
                x,
                y,
                down,
            ));
        }
        if !map_actions.is_empty() {
            self.dispatch_actions(map_actions).await;
        }
        let mut board_actions = Vec::new();
        for (surface_id, surface) in &self.shell.puzzle2d_board_states {
            let inside = surface.bounds.contains(x, y);
            if inside {
                board_actions.extend(scenes::puzzle_board_pointer_move(
                    surface_id,
                    &surface.controller_id,
                    surface.bounds,
                    x,
                    y,
                    modifiers.shift,
                    modifiers.ctrl_or_meta(),
                    modifiers.alt,
                ));
            } else {
                board_actions.extend(scenes::puzzle_board_pointer_leave(surface_id, &surface.controller_id, modifiers.alt));
            }
        }
        if !board_actions.is_empty() {
            self.dispatch_actions(board_actions).await;
        }
        if !world_actions.is_empty() {
            self.dispatch_actions(world_actions).await;
        }
    }

    async fn handle_context_menu(&mut self, x: f32, y: f32) {
        let _ = self
            .shell
            .handle_pointer_button(x, y, true, 2, &mut self.input, &self.theme)
            .await;
    }
}

fn start_frame_loop(window: Arc<Window>, runtime: Rc<RefCell<AppRuntime>>) {
    let next = runtime.clone();
    let window_next = window.clone();
    schedule_frame(&window, move || {
        if let Ok(mut app) = next.try_borrow_mut() {
            app.frame();
        }
        start_frame_loop(window_next.clone(), next.clone());
    });
}

enum HostUserEvent {
    RuntimeReady {
        runtime: Rc<RefCell<AppRuntime>>,
        callbacks: PointerCallbacks,
    },
}

struct SemioApp {
    proxy: EventLoopProxy<HostUserEvent>,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")]
    plugins: Option<wasm_bindgen::JsValue>,
    #[cfg(target_arch = "wasm32")]
    canvas: Option<web_sys::HtmlCanvasElement>,
    #[cfg(not(target_arch = "wasm32"))]
    plugin_modules_root: std::path::PathBuf,
    window: Option<Arc<Window>>,
    runtime: Option<Rc<RefCell<AppRuntime>>>,
    callbacks: Option<PointerCallbacks>,
    window_input: WindowInputState,
}

impl SemioApp {
    fn new(
        proxy: EventLoopProxy<HostUserEvent>,
        plugin_filter: String,
        #[cfg(target_arch = "wasm32")]
        plugins: Option<wasm_bindgen::JsValue>,
        #[cfg(target_arch = "wasm32")]
        canvas: Option<web_sys::HtmlCanvasElement>,
        #[cfg(not(target_arch = "wasm32"))]
        plugin_modules_root: std::path::PathBuf,
    ) -> Self {
        Self {
            proxy,
            plugin_filter,
            #[cfg(target_arch = "wasm32")]
            plugins,
            #[cfg(target_arch = "wasm32")]
            canvas,
            #[cfg(not(target_arch = "wasm32"))]
            plugin_modules_root,
            window: None,
            runtime: None,
            callbacks: None,
            window_input: WindowInputState::default(),
        }
    }
}

impl ApplicationHandler<HostUserEvent> for SemioApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let mut attributes = WindowAttributes::default().with_title("Semio");
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            if let Some(canvas) = self.canvas.clone() {
                let dpr = web_sys::window()
                    .map(|window| window.device_pixel_ratio() as f32)
                    .unwrap_or(1.0);
                let css_width = canvas.client_width().max(1) as f32;
                let css_height = canvas.client_height().max(1) as f32;
                let _ = canvas.style().set_property("width", "100%");
                let _ = canvas.style().set_property("height", "100vh");
                attributes = attributes
                    .with_inner_size(winit::dpi::LogicalSize::new(css_width, css_height))
                    .with_canvas(Some(canvas))
                    .with_append(true);
                let _ = dpr;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            attributes = attributes.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 800.0));
        }
        let window = Arc::new(event_loop.create_window(attributes).expect("create window"));
        self.window = Some(window.clone());
        let proxy = self.proxy.clone();
        let plugin_filter = self.plugin_filter.clone();
        #[cfg(target_arch = "wasm32")]
        let plugins = self.plugins.clone();
        #[cfg(not(target_arch = "wasm32"))]
        let plugin_modules_root = self.plugin_modules_root.clone();
        spawn_app_task(async move {
            let result = boot_runtime(
                window,
                plugin_filter,
                #[cfg(target_arch = "wasm32")]
                plugins,
                #[cfg(not(target_arch = "wasm32"))]
                plugin_modules_root,
            )
            .await;
            match result {
                Ok((runtime, callbacks)) => {
                    let _ = proxy.send_event(HostUserEvent::RuntimeReady { runtime, callbacks });
                }
                Err(error) => log_debug(&format!("[DEBUG] boot_runtime failed: {error}")),
            }
        });
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: HostUserEvent) {
        if let HostUserEvent::RuntimeReady { runtime, callbacks } = event {
            if let Some(window) = self.window.clone() {
                start_frame_loop(window, runtime.clone());
            }
            self.runtime = Some(runtime);
            self.callbacks = Some(callbacks);
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.clone() else {
            return;
        };
        match &event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(runtime) = self.runtime.as_ref() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        let dpr = window.scale_factor() as f32;
                        app.resize(size.width as f32 / dpr, size.height as f32 / dpr, dpr);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(runtime) = self.runtime.as_ref() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.frame();
                    }
                }
                window.request_redraw();
            }
            _ => {
                if let Some(callbacks) = self.callbacks.as_ref() {
                    dispatch_window_event(&window, &event, &mut self.window_input, callbacks);
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

async fn boot_runtime(
    window: Arc<Window>,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")] plugins: Option<wasm_bindgen::JsValue>,
    #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
) -> Result<(Rc<RefCell<AppRuntime>>, PointerCallbacks), String> {
    let dpr = window.scale_factor() as f32;
    let size = window.inner_size();
    #[cfg(target_arch = "wasm32")]
    let (css_width, css_height, dpr) = {
        use winit::platform::web::WindowExtWebSys;
        let dpr = web_sys::window()
            .map(|host| host.device_pixel_ratio() as f32)
            .unwrap_or(dpr);
        if let Some(canvas) = window.canvas() {
            let css_width = canvas.client_width().max(1) as f32;
            let css_height = canvas.client_height().max(1) as f32;
            canvas.set_width((css_width * dpr) as u32);
            canvas.set_height((css_height * dpr) as u32);
            (css_width, css_height, dpr)
        } else {
            (size.width as f32 / dpr, size.height as f32 / dpr, dpr)
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let css_width = size.width as f32 / dpr;
    #[cfg(not(target_arch = "wasm32"))]
    let css_height = size.height as f32 / dpr;

    const ANTA_LATIN: &[u8] = include_bytes!("../../../../ui/asset/font/anta/latin.ttf");
    let font_bytes = match fetch_font_bytes("/asset/font/anta/latin.ttf").await {
        Ok(bytes) if bytes.len() > 256 => bytes,
        _ => ANTA_LATIN.to_vec(),
    };
    let atlas = FontAtlas::from_bytes(&font_bytes).map_err(|err| format!("[DEBUG] atlas failed: {err}"))?;
    let icons = icon_atlas::build_icon_atlas();
    let mut gpu = GpuContext::from_window(window.clone())
        .await
        .map_err(|err| format!("[DEBUG] gpu init failed: {err}"))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);
    gpu.upload_icon_atlas(&icons);

    #[cfg(target_arch = "wasm32")]
    let entries = {
        let plugins = plugins.ok_or("missing wasm plugins")?;
        filter_plugins(parse_plugin_entries(plugins).map_err(|err| format!("[DEBUG] plugin parse failed: {err}"))?, &plugin_filter)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let entries = filter_plugins(
        load_wasm_plugins(&plugin_filter, &plugin_modules_root)?,
        &plugin_filter,
    );

    let mut shell = ShellState::new(entries, plugin_filter.clone());
    shell.screen_w = css_width * dpr;
    shell.screen_h = css_height * dpr;
    shell.boot().await.map_err(|err| format!("[DEBUG] shell boot failed: {err}"))?;

    let runtime = Rc::new(RefCell::new(AppRuntime {
        gpu,
        atlas,
        icons,
        shell,
        draw: DrawList::default(),
        overlay: DrawList::default(),
        input: InputState::default(),
        theme: Theme::default(),
        window: window.clone(),
        theme_dark: appearance_is_dark("system"),
        last_cursor: None,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        modifiers: PointerModifiers::default(),
        wheel_delta: 0.0,
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        caret_blink_at_ms: 0.0,
        caret_blink_visible: true,
        asset_poll_pending: false,
        self_weak: std::rc::Weak::new(),
        #[cfg(not(target_arch = "wasm32"))]
        plugin_modules_root: plugin_modules_root.clone(),
        #[cfg(not(target_arch = "wasm32"))]
        native_plugin_mtimes: std::collections::HashMap::new(),
        #[cfg(not(target_arch = "wasm32"))]
        native_reload_pending: false,
    }));
    runtime.borrow_mut().self_weak = Rc::downgrade(&runtime);

    let runtime_pointer = runtime.clone();
    let runtime_move = runtime.clone();
    let runtime_wheel = runtime.clone();
    let runtime_keyboard = runtime.clone();
    let runtime_context = runtime.clone();
    let callbacks = PointerCallbacks {
        on_move: Rc::new(move |x, y, down, button, modifiers| {
            let runtime = runtime_move.clone();
            spawn_app_task(async move {
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    app.handle_pointer_move(x, y, down, button, modifiers).await;
                }
            });
        }),
        on_button: Rc::new(move |x, y, down, button, modifiers| {
            let runtime = runtime_pointer.clone();
            spawn_app_task(async move {
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
            spawn_app_task(async move {
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    app.handle_context_menu(x, y).await;
                }
            });
        }),
    };

    log_debug("[DEBUG] wgpu renderer booted");
    Ok((runtime, callbacks))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) {
    let event_loop = EventLoop::<HostUserEvent>::with_user_event()
        .build()
        .expect("event loop");
    let proxy = event_loop.create_proxy();
    let mut app = SemioApp::new(proxy, plugin_filter.to_string(), plugin_modules_root);
    let _ = event_loop.run_app(&mut app);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioRendererBoot)]
pub async fn semio_renderer_boot(plugins: JsValue, plugin_filter: String) -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("[DEBUG] missing window"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("[DEBUG] missing document"))?;
    let root = document
        .get_element_by_id("root")
        .ok_or_else(|| JsValue::from_str("[DEBUG] missing #root"))?;
    let canvas = document
        .create_element("canvas")
        .map_err(|_| JsValue::from_str("[DEBUG] canvas create failed"))?;
    canvas.set_id("semio-wgpu-canvas");
    let style = canvas
        .dyn_ref::<web_sys::HtmlElement>()
        .map(|element| element.style())
        .ok_or_else(|| JsValue::from_str("[DEBUG] canvas style failed"))?;
    let _ = style.set_property("display", "block");
    let _ = style.set_property("width", "100%");
    let _ = style.set_property("height", "100%");
    let _ = style.set_property("touch-action", "none");
    let _ = style.set_property("outline", "none");
    root.set_inner_html("");
    root.append_child(&canvas)
        .map_err(|_| JsValue::from_str("[DEBUG] canvas append failed"))?;
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("[DEBUG] canvas cast failed"))?;

    let event_loop = EventLoop::<HostUserEvent>::with_user_event()
        .build()
        .map_err(|err| JsValue::from_str(&format!("[DEBUG] event loop: {err:?}")))?;
    let proxy = event_loop.create_proxy();
    let mut app = SemioApp::new(proxy, plugin_filter, Some(plugins), Some(canvas));
    use winit::platform::web::EventLoopExtWebSys;
    event_loop.spawn_app(app);
    Ok(())
}

#[cfg(target_arch = "wasm32")]
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

