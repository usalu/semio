//! 🧩 Generic widget tree — layout, measurement, and drawing.

use crate::chrome::{item_bg, item_text, push_control_border, push_icon, ICON_TINY};
use crate::draw::{DrawList, IconAtlas};
use crate::geometry::Rect;
use crate::input::{DragAxis, HitKind, HitTarget, InputState};
use crate::layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
use crate::text::FontAtlas;
use crate::theme::{Rgba, Theme};
use std::collections::HashMap;

pub struct WidgetContext<'a, E> {
    pub draw: &'a mut DrawList,
    pub overlay: Option<&'a mut DrawList>,
    pub atlas: &'a mut FontAtlas,
    pub icons: Option<&'a IconAtlas>,
    pub input: &'a mut InputState<E>,
    pub theme: &'a Theme,
    pub scroll_offsets: &'a mut HashMap<String, f32>,
    pub collapsed_sections: &'a mut HashMap<String, bool>,
    pub open_selects: &'a mut HashMap<String, bool>,
    pub tree_hover_commands: Option<&'a mut HashMap<String, E>>,
    pub tree_unhover_commands: Option<&'a mut HashMap<String, E>>,
    pub tree_selection_change: Option<&'a mut Option<E>>,
}

#[derive(Clone, Debug)]
pub struct SelectItem {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct KeyValueEntry {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct TreeItemAction<E> {
    pub icon_id: String,
    pub label: Option<String>,
    pub event: E,
    pub reveal_on_hover: bool,
}

#[derive(Clone, Debug)]
pub struct TreeItem<E> {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub icon_id: Option<String>,
    pub selected: bool,
    pub highlighted: bool,
    pub default_open: bool,
    pub is_hidden: bool,
    pub event: Option<E>,
    pub hover_event: Option<E>,
    pub unhover_event: Option<E>,
    pub actions: Vec<TreeItemAction<E>>,
    pub draggable: bool,
    pub drag_data: HashMap<String, String>,
    pub control: Option<Box<WidgetNode<E>>>,
    pub children: Vec<TreeItem<E>>,
}

#[derive(Clone, Debug)]
pub struct TreeSection<E> {
    pub id: String,
    pub label: Option<String>,
    pub default_open: bool,
    pub items: Vec<TreeItem<E>>,
}

#[derive(Clone, Debug)]
pub enum ControlNode<E> {
    Button { id: Option<String>, label: String, event: Option<E> },
    Input { id: String, value: String, placeholder: Option<String> },
    Select { id: String, value: String, items: Vec<SelectItem>, placeholder: Option<String>, event: Option<E> },
    Toggle { id: String, pressed: bool, text: Option<String>, event: Option<E> },
    Vec3 { id: String, value: Option<[f64; 3]>, event: Option<E> },
    KeyValue { entries: Vec<KeyValueEntry> },
    Slider { id: String, value: f64, min: f64, max: f64, event: Option<E> },
    NumberStepper { id: String, value: f64, event: Option<E> },
    Ring { id: String, t: f64, event: Option<E> },
    IconSelect { id: String, value: String, event: Option<E> },
}

#[derive(Clone, Debug)]
pub enum WidgetNode<E> {
    Stack {
        direction: String,
        gap: Option<String>,
        padding: Option<String>,
        children: Vec<WidgetNode<E>>,
    },
    Text { value: String, emphasize: bool },
    Separator,
    Button { id: Option<String>, label: String, event: Option<E> },
    Input { id: String, value: String, placeholder: Option<String> },
    Select { id: String, value: String, items: Vec<SelectItem>, placeholder: Option<String>, event: Option<E> },
    Toggle { id: String, pressed: bool, text: Option<String>, event: Option<E> },
    Vec3 { id: String, value: Option<[f64; 3]>, event: Option<E> },
    KeyValue { entries: Vec<KeyValueEntry> },
    Slider { id: String, value: f64, min: f64, max: f64, event: Option<E> },
    NumberStepper { id: String, value: f64, event: Option<E> },
    Ring { id: String, t: f64, event: Option<E> },
    IconSelect { id: String, value: String, event: Option<E> },
    Field { id: String, label: String, child: ControlNode<E> },
    Section { id: String, label: Option<String>, children: Vec<WidgetNode<E>> },
    Tree {
        sections: Vec<TreeSection<E>>,
        selected_ids: Vec<String>,
        highlighted_ids: Vec<String>,
        selection_change: Option<E>,
    },
}

const PANEL_HEADER: f32 = 24.0;
const TREE_ROW_HEIGHT: f32 = 24.0;
const TREE_INDENT_PER_LEVEL: f32 = 10.0;
const TREE_TOGGLE_WIDTH: f32 = 14.0;
const TREE_ICON_SIZE: f32 = 14.0;
const TREE_SECTION_GAP: f32 = 8.0;

pub fn measure_widget<E>(atlas: &mut FontAtlas, theme: &Theme, node: &WidgetNode<E>) -> (f32, f32) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(theme, gap.as_deref());
            let padding = padding_for_token(theme, padding.as_deref()) * 2.0;
            let vertical = direction != "horizontal";
            let mut total_main = 0.0f32;
            let mut max_cross = 0.0f32;
            for (index, child) in children.iter().enumerate() {
                let (w, h) = measure_widget(atlas, theme, child);
                if vertical {
                    total_main += h;
                    max_cross = max_cross.max(w);
                    if index + 1 < children.len() {
                        total_main += gap;
                    }
                } else {
                    total_main += w;
                    max_cross = max_cross.max(h);
                    if index + 1 < children.len() {
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
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { theme.font_size_emphasized } else { theme.font_size_body };
            atlas.measure_text(value, size)
        }
        WidgetNode::Separator => (theme.control_height, 1.0),
        WidgetNode::Button { .. } | WidgetNode::Input { .. } | WidgetNode::Select { .. }
        | WidgetNode::Toggle { .. } | WidgetNode::Slider { .. } | WidgetNode::NumberStepper { .. }
        | WidgetNode::IconSelect { .. } => (theme.control_height, theme.control_height),
        WidgetNode::Vec3 { .. } => (theme.control_height, theme.control_height * 3.0 + theme.gap_standard * 2.0),
        WidgetNode::KeyValue { entries } => (theme.control_height, entries.len() as f32 * 22.0),
        WidgetNode::Ring { .. } => (80.0, 80.0),
        WidgetNode::Field { child, .. } => {
            let (_, ch) = measure_control(theme, child);
            (ch + 18.0, ch + 18.0)
        }
        WidgetNode::Section { children, .. } => {
            let mut height = PANEL_HEADER;
            for child in children {
                let (_, h) = measure_widget(atlas, theme, child);
                height += h + theme.gap_standard;
            }
            (200.0, height)
        }
        WidgetNode::Tree { sections, .. } => (200.0, measure_tree_sections(sections)),
    }
}

fn measure_control<E>(theme: &Theme, control: &ControlNode<E>) -> (f32, f32) {
    match control {
        ControlNode::Button { .. } | ControlNode::Input { .. } | ControlNode::Select { .. }
        | ControlNode::Toggle { .. } | ControlNode::Slider { .. } | ControlNode::NumberStepper { .. }
        | ControlNode::IconSelect { .. } => (theme.control_height, theme.control_height),
        ControlNode::Vec3 { .. } => (theme.control_height, theme.control_height * 3.0),
        ControlNode::KeyValue { entries } => (theme.control_height, entries.len() as f32 * 22.0),
        ControlNode::Ring { .. } => (80.0, 80.0),
    }
}

pub fn render_widget<E: Clone>(
    node: &WidgetNode<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(ctx.theme, gap.as_deref());
            let padding = padding_for_token(ctx.theme, padding.as_deref());
            let vertical = direction != "horizontal";
            let sizes: Vec<f32> = children
                .iter()
                .map(|child| {
                    let (w, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    if vertical { h } else { w }
                })
                .collect();
            let rects = if vertical {
                layout_vertical(bounds, gap, padding, &sizes)
            } else {
                layout_horizontal(bounds, gap, padding, &sizes)
            };
            for (child, rect) in children.iter().zip(rects.iter()) {
                render_widget(child, *rect, ctx);
            }
        }
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { ctx.theme.font_size_emphasized } else { ctx.theme.font_size_body };
            let color = if *emphasize { ctx.theme.text } else { ctx.theme.text_muted };
            draw_text(ctx, value, bounds.x, bounds.y + size, size, color);
        }
        WidgetNode::Separator => {
            let y = bounds.y + bounds.h * 0.5;
            ctx.draw.push_line(bounds.x, y, bounds.x + bounds.w, y, ctx.theme.separator, 1.0);
        }
        WidgetNode::Button { id, label, event } => render_button(id.clone(), label, event.clone(), bounds, ctx),
        WidgetNode::Input { id, value, placeholder } => render_input(id, value, placeholder.as_deref(), bounds, ctx),
        WidgetNode::Select { id, value, items, placeholder, event } => {
            render_select(id, value, items, placeholder.as_deref(), event.clone(), bounds, ctx)
        }
        WidgetNode::Toggle { id, pressed, text, event } => {
            render_toggle(id, *pressed, text.as_deref(), event.clone(), bounds, ctx)
        }
        WidgetNode::Vec3 { id, value, event } => render_vec3(id, *value, event.clone(), bounds, ctx),
        WidgetNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        WidgetNode::Slider { id, value, min, max, event } => {
            render_slider(id, *value, *min, *max, event.clone(), bounds, ctx)
        }
        WidgetNode::NumberStepper { id, value, event } => {
            render_number_stepper(id, *value, event.clone(), bounds, ctx)
        }
        WidgetNode::Ring { id, t, event } => render_ring(id, *t, event.clone(), bounds, ctx),
        WidgetNode::IconSelect { id, value, event } => render_icon_select(id, value, event.clone(), bounds, ctx),
        WidgetNode::Field { label, child, .. } => {
            draw_text(ctx, label, bounds.x, bounds.y + ctx.theme.font_size_small, ctx.theme.font_size_small, ctx.theme.text_muted);
            let child_bounds = Rect::new(bounds.x, bounds.y + 18.0, bounds.w, bounds.h - 18.0);
            render_control(child, child_bounds, ctx);
        }
        WidgetNode::Section { label, children, id } => {
            let collapsed = *ctx.collapsed_sections.get(id).unwrap_or(&false);
            if let Some(label) = label {
                let header = Rect::new(bounds.x, bounds.y, bounds.w, PANEL_HEADER);
                let chevron = if collapsed { "▸" } else { "▾" };
                draw_text(ctx, chevron, bounds.x, bounds.y + ctx.theme.font_size_body, ctx.theme.font_size_body, ctx.theme.text_muted);
                draw_text(ctx, label, bounds.x + 14.0, bounds.y + ctx.theme.font_size_body, ctx.theme.font_size_body, ctx.theme.text);
                ctx.input.register_hit(HitTarget {
                    rect: header,
                    event: None,
                    control_id: Some(format!("section.{id}")),
                    kind: HitKind::Generic,
                    drag_axis: None,
                    drag_data: None,
                });
            }
            if !collapsed {
                let mut y = bounds.y + PANEL_HEADER;
                for child in children {
                    let (_, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    let child_bounds = Rect::new(bounds.x, y, bounds.w, h);
                    render_widget(child, child_bounds, ctx);
                    y += h + ctx.theme.gap_standard;
                }
            }
        }
        WidgetNode::Tree {
            sections,
            selected_ids,
            highlighted_ids,
            selection_change,
        } => {
            if let Some(slot) = ctx.tree_selection_change.as_deref_mut() {
                *slot = selection_change.clone();
            }
            let scroll_id = format!("tree:{:.0}:{:.0}", bounds.x, bounds.y);
            let content_h = measure_tree_sections_state(sections, ctx.collapsed_sections);
            render_scroll_region(&scroll_id, bounds, content_h.max(bounds.h), ctx, |content, ctx| {
                render_tree(sections, selected_ids, highlighted_ids, content, ctx);
            });
        }
    }
}

fn render_control<E: Clone>(control: &ControlNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match control {
        ControlNode::Button { id, label, event } => render_button(id.clone(), label, event.clone(), bounds, ctx),
        ControlNode::Input { id, value, placeholder } => render_input(id, value, placeholder.as_deref(), bounds, ctx),
        ControlNode::Select { id, value, items, placeholder, event } => {
            render_select(id, value, items, placeholder.as_deref(), event.clone(), bounds, ctx)
        }
        ControlNode::Toggle { id, pressed, text, event } => {
            render_toggle(id, *pressed, text.as_deref(), event.clone(), bounds, ctx)
        }
        ControlNode::Vec3 { id, value, event } => render_vec3(id, *value, event.clone(), bounds, ctx),
        ControlNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        ControlNode::Slider { id, value, min, max, event } => {
            render_slider(id, *value, *min, *max, event.clone(), bounds, ctx)
        }
        ControlNode::NumberStepper { id, value, event } => {
            render_number_stepper(id, *value, event.clone(), bounds, ctx)
        }
        ControlNode::Ring { id, t, event } => render_ring(id, *t, event.clone(), bounds, ctx),
        ControlNode::IconSelect { id, value, event } => render_icon_select(id, value, event.clone(), bounds, ctx),
    }
}

fn render_button<E: Clone>(
    id: Option<String>,
    label: &str,
    event: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let control_id = id.clone().or_else(|| Some(label.to_string()));
    let hovered = ctx.input.hovered_id == control_id;
    let bg = item_bg(ctx.theme, false, hovered);
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let mut text_x = bounds.x + ctx.theme.padding_standard;
    if let Some(icons) = ctx.icons {
        if icons.icon_uv(label).is_some() {
            push_icon(ctx.draw, icons, label, text_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY);
            text_x += ICON_TINY + ctx.theme.gap_standard;
        }
    }
    draw_text(
        ctx,
        label,
        text_x,
        bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        item_text(ctx.theme, false, hovered),
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event,
        control_id,
        kind: HitKind::Button,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_input<E: Clone>(id: &str, value: &str, placeholder: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let focused = ctx.input.focused_id.as_deref() == Some(id);
    let border = if focused {
        ctx.theme.border_emphasized
    } else {
        ctx.theme.border_normal
    };
    push_control_border(ctx.draw, bounds, ctx.theme, border, ctx.theme.input_bg);
    let (display, muted) = if focused {
        (ctx.input.text_buffer.clone(), false)
    } else if value.is_empty() {
        (placeholder.unwrap_or("").to_string(), true)
    } else {
        (value.to_string(), false)
    };
    draw_text(
        ctx,
        &display,
        bounds.x + 8.0,
        bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        if muted { ctx.theme.text_muted } else { ctx.theme.text },
    );
    if focused {
        let cursor_x = bounds.x + 8.0 + measure_text_width(ctx, &display[..ctx.input.cursor_pos.min(display.len())], ctx.theme.font_size_body);
        ctx.draw.push_solid([cursor_x, bounds.y + 6.0, 1.0, bounds.h - 12.0], ctx.theme.text);
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(id.to_string()),
        kind: HitKind::Input,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_select<E: Clone>(
    id: &str,
    value: &str,
    items: &[SelectItem],
    placeholder: Option<&str>,
    event: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let open = *ctx.open_selects.get(id).unwrap_or(&false);
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = if hovered {
        ctx.theme.button_hover
    } else {
        ctx.theme.input_bg
    };
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let label = items
        .iter()
        .find(|item| item.value == value)
        .map(|item| item.label.as_str())
        .unwrap_or(placeholder.unwrap_or("Select…"));
    draw_text(
        ctx,
        label,
        bounds.x + ctx.theme.padding_standard,
        bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        ctx.theme.text,
    );
    if let Some(icons) = ctx.icons {
        push_icon(
            ctx.draw,
            icons,
            "chevron-down",
            bounds.x + bounds.w - ctx.theme.padding_standard - ICON_TINY,
            bounds.y + (bounds.h - ICON_TINY) * 0.5,
            ICON_TINY,
        );
    } else {
        draw_text(
            ctx,
            "▾",
            bounds.x + bounds.w - 16.0,
            bounds.y + (bounds.h + ctx.theme.font_size_small) * 0.5,
            ctx.theme.font_size_small,
            ctx.theme.text_muted,
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(id.to_string()),
        kind: HitKind::Select,
        drag_axis: None,
        drag_data: None,
    });
    if open {
        let item_h = ctx.theme.control_height;
        let menu_h = items.len() as f32 * item_h + 4.0;
        let menu = Rect::new(bounds.x, bounds.y + bounds.h + 2.0, bounds.w, menu_h);
        ctx.draw.push_rounded([menu.x, menu.y, menu.w, menu.h], ctx.theme.overlay_bg, ctx.theme.border_radius);
        for (index, item) in items.iter().enumerate() {
            let row = Rect::new(menu.x + 2.0, menu.y + 2.0 + index as f32 * item_h, menu.w - 4.0, item_h);
            let row_hovered = ctx.input.hit_at(ctx.input.pointer_x, ctx.input.pointer_y)
                .and_then(|h| h.control_id.as_deref()) == Some(&format!("{id}.item.{}", item.value));
            if row_hovered || item.value == value {
                ctx.draw.push_rounded([row.x, row.y, row.w, row.h], ctx.theme.row_hover, ctx.theme.border_radius);
            }
            draw_text(ctx, &item.label, row.x + 8.0, row.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
            ctx.input.register_hit(HitTarget {
                rect: row,
                event: event.clone(),
                control_id: Some(format!("{id}.item.{}", item.value)),
                kind: HitKind::DropdownItem,
                drag_axis: None,
                drag_data: None,
            });
        }
    }
}

fn render_toggle<E: Clone>(
    id: &str,
    pressed: bool,
    text: Option<&str>,
    event: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = item_bg(ctx.theme, pressed, hovered);
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let mut content_x = bounds.x + ctx.theme.padding_standard;
    if let Some(text) = text {
        draw_text(
            ctx,
            text,
            content_x,
            bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
            ctx.theme.font_size_body,
            item_text(ctx.theme, pressed, hovered),
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event,
        control_id: Some(id.to_string()),
        kind: HitKind::Toggle,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_vec3<E: Clone>(id: &str, value: Option<[f64; 3]>, event: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let values = value.unwrap_or([0.0, 0.0, 0.0]);
    let labels = ["X", "Y", "Z"];
    for (index, label) in labels.iter().enumerate() {
        let y = bounds.y + index as f32 * (ctx.theme.control_height + 4.0);
        let row = Rect::new(bounds.x, y, bounds.w, ctx.theme.control_height);
        ctx.draw.push_rounded([row.x, row.y, row.w, row.h], ctx.theme.input_bg, ctx.theme.border_radius);
        let text = format!("{label}: {:.3}", values[index]);
        draw_text(ctx, &text, row.x + 8.0, row.y + 18.0, ctx.theme.font_size_small, ctx.theme.text);
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event,
        control_id: Some(id.to_string()),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_key_value<E>(entries: &[KeyValueEntry], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    for (index, entry) in entries.iter().enumerate() {
        let y = bounds.y + index as f32 * 22.0;
        draw_text(ctx, &entry.label, bounds.x, y + ctx.theme.font_size_small, ctx.theme.font_size_small, ctx.theme.text_muted);
        draw_text(
            ctx,
            &entry.value,
            bounds.x + bounds.w * 0.4,
            y + ctx.theme.font_size_small,
            ctx.theme.font_size_small,
            ctx.theme.text,
        );
    }
}

fn render_slider<E: Clone>(
    id: &str,
    value: f64,
    min: f64,
    max: f64,
    event: Option<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let track_y = bounds.y + bounds.h * 0.5;
    ctx.draw.push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], ctx.theme.separator, 2.0);
    let range = (max - min).max(f64::EPSILON);
    let mut t = ((value - min) / range).clamp(0.0, 1.0);
    if ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
        let dx = ctx.input.drag.current_x - ctx.input.drag.start_x;
        t = (t as f32 + dx / bounds.w.max(1.0)).clamp(0.0, 1.0) as f64;
    }
    let knob_x = bounds.x + bounds.w * t as f32;
    ctx.draw.push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], ctx.theme.accent, 6.0);
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event,
        control_id: Some(id.to_string()),
        kind: HitKind::Slider,
        drag_axis: Some(DragAxis::Horizontal),
        drag_data: None,
    });
}

fn render_number_stepper<E: Clone>(id: &str, value: f64, event: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let seg = bounds.w / 3.0;
    let minus = Rect::new(bounds.x, bounds.y, seg, bounds.h);
    let center = Rect::new(bounds.x + seg, bounds.y, seg, bounds.h);
    let plus = Rect::new(bounds.x + seg * 2.0, bounds.y, seg, bounds.h);
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.input_bg, ctx.theme.border_radius);
    draw_text(ctx, "−", minus.x + seg * 0.5 - 4.0, minus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
    let text = format!("{value:.3}");
    draw_text(ctx, &text, center.x + 8.0, center.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
    draw_text(ctx, "+", plus.x + seg * 0.5 - 4.0, plus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
    ctx.input.register_hit(HitTarget {
        rect: minus,
        event: event.clone(),
        control_id: Some(format!("{id}.minus")),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
    ctx.input.register_hit(HitTarget {
        rect: center,
        event: None,
        control_id: Some(format!("{id}.input")),
        kind: HitKind::Input,
        drag_axis: None,
        drag_data: None,
    });
    ctx.input.register_hit(HitTarget {
        rect: plus,
        event,
        control_id: Some(format!("{id}.plus")),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}

fn render_ring<E: Clone>(id: &str, t: f64, event: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let cx = bounds.x + bounds.w * 0.5;
    let cy = bounds.y + bounds.h * 0.5;
    let radius = bounds.w.min(bounds.h) * 0.4;
    let segments = 48usize;
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let angle = std::f32::consts::TAU * i as f32 / segments as f32;
        points.push([cx + angle.cos() * radius, cy + angle.sin() * radius]);
    }
    for window in points.windows(2) {
        ctx.draw.push_line(
            window[0][0], window[0][1], window[1][0], window[1][1],
            ctx.theme.separator, 2.0,
        );
    }
    let mut knob_t = t;
    if ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
        let dx = ctx.input.drag.current_x - cx;
        let dy = ctx.input.drag.current_y - cy;
        knob_t = (dy.atan2(dx) as f64 / std::f64::consts::TAU).rem_euclid(1.0);
    }
    let knob_angle = std::f32::consts::TAU * knob_t as f32;
    let kx = cx + knob_angle.cos() * radius;
    let ky = cy + knob_angle.sin() * radius;
    ctx.draw.push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], ctx.theme.accent, 6.0);
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event,
        control_id: Some(id.to_string()),
        kind: HitKind::Slider,
        drag_axis: Some(DragAxis::Ring),
        drag_data: None,
    });
}

fn render_icon_select<E: Clone>(id: &str, value: &str, event: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.button, ctx.theme.border_radius);
    draw_text(
        ctx,
        value,
        bounds.x + 8.0,
        bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        ctx.theme.text,
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event,
        control_id: Some(id.to_string()),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}

fn measure_tree_sections<E>(sections: &[TreeSection<E>]) -> f32 {
    let collapsed = HashMap::new();
    measure_tree_sections_state(sections, &collapsed)
}

fn measure_tree_sections_state<E>(sections: &[TreeSection<E>], collapsed: &HashMap<String, bool>) -> f32 {
    let mut height = 0.0;
    for section in sections {
        height += TREE_ROW_HEIGHT;
        let section_key = format!("section.{}", section.id);
        let section_collapsed = collapsed.get(&section_key).copied().unwrap_or(!section.default_open);
        if !section_collapsed {
            for item in &section.items {
                height += measure_tree_item_height(item, collapsed, 0);
            }
            height += TREE_SECTION_GAP;
        }
    }
    height
}

fn measure_tree_item_height<E>(item: &TreeItem<E>, collapsed: &HashMap<String, bool>, depth: u32) -> f32 {
    let mut height = TREE_ROW_HEIGHT;
    let key = format!("tree.{}", item.id);
    let item_collapsed = collapsed.get(&key).copied().unwrap_or(!item.default_open);
    if !item_collapsed {
        for child in &item.children {
            height += measure_tree_item_height(child, collapsed, depth + 1);
        }
    }
    height
}

fn tree_gutter_width(depth: u32) -> f32 {
    depth as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH
}

fn tree_icon_id<E>(item: &TreeItem<E>, expandable: bool) -> &str {
    item.icon_id
        .as_deref()
        .unwrap_or(if expandable { "folder" } else { "file-text" })
}

fn tree_row_collapsed(collapsed: &HashMap<String, bool>, key: &str, default_open: bool) -> bool {
    collapsed.get(key).copied().unwrap_or(!default_open)
}

fn render_tree<E: Clone>(
    sections: &[TreeSection<E>],
    selected_ids: &[String],
    highlighted_ids: &[String],
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
) {
    let mut y = bounds.y;
    for section in sections {
    let section_key = format!("section.{}", section.id);
    if !ctx.collapsed_sections.contains_key(&section_key) {
        ctx.collapsed_sections.insert(section_key.clone(), !section.default_open);
    }
    let section_collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, section.default_open);
        render_tree_section_header(section, bounds, y, section_collapsed, ctx);
        y += TREE_ROW_HEIGHT;
        if !section_collapsed {
            for item in &section.items {
                y += render_tree_item(
                    item,
                    Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT),
                    ctx,
                    0,
                    selected_ids,
                    highlighted_ids,
                    &[],
                );
            }
            y += TREE_SECTION_GAP;
        }
    }
}

fn render_tree_section_header<E: Clone>(
    section: &TreeSection<E>,
    bounds: Rect,
    y: f32,
    collapsed: bool,
    ctx: &mut WidgetContext<'_, E>,
) {
    let row = Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT);
    let gutter_w = TREE_TOGGLE_WIDTH;
    let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
    let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
    let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
    tree_draw_chevron(ctx, chevron, gutter);
    ctx.input.register_hit(HitTarget {
        rect: gutter,
        event: None,
        control_id: Some(format!("section.chevron.{}", section.id)),
        kind: HitKind::TreeItem,
        drag_axis: None,
        drag_data: None,
    });
    if let Some(label) = &section.label {
        let text_color = if collapsed { ctx.theme.text_muted } else { ctx.theme.text };
        let label_x = content.x + ctx.theme.gap_standard;
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv("folder")) {
            draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
        }
        draw_text(
            ctx,
            label,
            label_x + TREE_ICON_SIZE + ctx.theme.gap_standard,
            content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0,
            ctx.theme.font_size_small,
            text_color,
        );
    }
}

fn render_tree_item<E: Clone>(
    item: &TreeItem<E>,
    bounds: Rect,
    ctx: &mut WidgetContext<'_, E>,
    depth: u32,
    selected_ids: &[String],
    highlighted_ids: &[String],
    is_last_at_level: &[bool],
) -> f32 {
    if item.is_hidden {
        return TREE_ROW_HEIGHT;
    }
    let key = format!("tree.{}", item.id);
    if !ctx.collapsed_sections.contains_key(&key) {
        ctx.collapsed_sections.insert(key.clone(), !item.default_open);
    }
    let collapsed = tree_row_collapsed(ctx.collapsed_sections, &key, item.default_open);
    let expandable = !item.children.is_empty();
    let gutter_w = tree_gutter_width(depth);
    let row = Rect::new(bounds.x, bounds.y, bounds.w, TREE_ROW_HEIGHT);
    let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
    let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
    let hovered = ctx.input.hit_at(ctx.input.pointer_x, ctx.input.pointer_y)
        .and_then(|h| h.control_id.as_deref())
        .is_some_and(|id| id.strip_prefix("tree.label.").is_some_and(|v| v == item.id));
    let selected = item.selected || selected_ids.iter().any(|id| id == &item.id);
    let highlighted = item.highlighted || highlighted_ids.iter().any(|id| id == &item.id);
    tree_draw_guides(ctx, gutter, depth, is_last_at_level);
    if expandable {
        let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
        let chevron_rect = Rect::new(
            gutter.x + depth as f32 * TREE_INDENT_PER_LEVEL,
            gutter.y,
            TREE_TOGGLE_WIDTH,
            gutter.h,
        );
        tree_draw_chevron(ctx, chevron, chevron_rect);
        ctx.input.register_hit(HitTarget {
            rect: chevron_rect,
            event: None,
            control_id: Some(format!("tree.chevron.{}", item.id)),
            kind: HitKind::TreeItem,
            drag_axis: None,
            drag_data: None,
        });
    }
    if selected {
        ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.selected, ctx.theme.border_radius);
    } else if highlighted || hovered {
        ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.row_hover, ctx.theme.border_radius);
    }
    let mut label_x = content.x + ctx.theme.gap_standard;
    let icon_id = tree_icon_id(item, expandable);
    if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
        draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, ctx.theme.text);
        label_x += TREE_ICON_SIZE + ctx.theme.gap_standard;
    }
    let text_color = if selected || highlighted {
        ctx.theme.active_foreground
    } else if item.is_hidden {
        ctx.theme.text_muted
    } else {
        ctx.theme.text
    };
    draw_text(
        ctx,
        &item.label,
        label_x,
        content.y + (content.h + ctx.theme.font_size_body) * 0.5 - 2.0,
        ctx.theme.font_size_body,
        text_color,
    );
    if let Some(description) = &item.description {
        let label_w = measure_text_width(ctx, &item.label, ctx.theme.font_size_body);
        draw_text(
            ctx,
            description,
            label_x + label_w + ctx.theme.gap_standard,
            content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0,
            ctx.theme.font_size_small,
            ctx.theme.text_muted,
        );
    }
    let mut actions_x = content.x + content.w - ctx.theme.gap_standard;
    for (index, action) in item.actions.iter().enumerate().rev() {
        if action.reveal_on_hover && !hovered {
            continue;
        }
        let action_w = TREE_ICON_SIZE + ctx.theme.padding_standard;
        actions_x -= action_w;
        let action_rect = Rect::new(actions_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5 - 2.0, TREE_ICON_SIZE + 4.0, TREE_ICON_SIZE + 4.0);
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(&action.icon_id)) {
            draw_icon(ctx, uv, action_rect.x + 2.0, action_rect.y + 2.0, TREE_ICON_SIZE, ctx.theme.text);
        }
        ctx.input.register_hit(HitTarget {
            rect: action_rect,
            event: Some(action.event.clone()),
            control_id: Some(format!("tree.action.{}.{}", item.id, index)),
            kind: HitKind::Button,
            drag_axis: None,
            drag_data: None,
        });
    }
    if let Some(control) = &item.control {
        let control_w = 120.0;
        let control_rect = Rect::new(
            content.x + content.w - control_w - ctx.theme.gap_standard,
            content.y + (content.h - ctx.theme.control_height) * 0.5,
            control_w,
            ctx.theme.control_height,
        );
        render_widget(control, control_rect, ctx);
    }
    if let Some(hover) = &item.hover_event {
        if let Some(map) = ctx.tree_hover_commands.as_deref_mut() {
            map.insert(item.id.clone(), hover.clone());
        }
    }
    if let Some(unhover) = &item.unhover_event {
        if let Some(map) = ctx.tree_unhover_commands.as_deref_mut() {
            map.insert(item.id.clone(), unhover.clone());
        }
    }
    let label_rect = Rect::new(label_x, content.y, content.x + content.w - label_x - ctx.theme.gap_standard, content.h);
    ctx.input.register_hit(HitTarget {
        rect: label_rect,
        event: item.event.clone(),
        control_id: Some(format!("tree.label.{}", item.id)),
        kind: HitKind::TreeItem,
        drag_axis: if item.draggable { Some(DragAxis::Both) } else { None },
        drag_data: if item.draggable && !item.drag_data.is_empty() {
            Some(item.drag_data.clone())
        } else {
            None
        },
    });
    let mut height = TREE_ROW_HEIGHT;
    if !collapsed {
        for (index, child) in item.children.iter().enumerate() {
            let mut child_is_last = is_last_at_level.to_vec();
            child_is_last.push(index + 1 == item.children.len());
            let child_bounds = Rect::new(bounds.x, bounds.y + height, bounds.w, TREE_ROW_HEIGHT);
            height += render_tree_item(
                child,
                child_bounds,
                ctx,
                depth + 1,
                selected_ids,
                highlighted_ids,
                &child_is_last,
            );
        }
    }
    height
}

fn tree_draw_chevron<E>(ctx: &mut WidgetContext<'_, E>, icon_id: &str, rect: Rect) {
    if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
        draw_icon(
            ctx,
            uv,
            rect.x + (rect.w - TREE_ICON_SIZE) * 0.5,
            rect.y + (rect.h - TREE_ICON_SIZE) * 0.5,
            TREE_ICON_SIZE,
            ctx.theme.text_muted,
        );
    }
}

fn tree_draw_guides<E>(ctx: &mut WidgetContext<'_, E>, gutter: Rect, depth: u32, is_last_at_level: &[bool]) {
    let hair = ctx.theme.stroke_hairline.max(1.0);
    let guide_color = ctx.theme.border_normal;
    for level in 0..depth {
        if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
            continue;
        }
        let x = gutter.x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, gutter.h], guide_color);
    }
    if depth > 0 {
        let x = gutter.x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        let mid_y = gutter.y + gutter.h * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, mid_y - gutter.y], guide_color);
        ctx.draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
    }
}

pub fn render_scroll_region<E: Clone, F: FnOnce(Rect, &mut WidgetContext<'_, E>)>(
    scroll_id: &str,
    bounds: Rect,
    content_height: f32,
    ctx: &mut WidgetContext<'_, E>,
    render_content: F,
) {
    let max_scroll = (content_height - bounds.h).max(0.0);
    let offset = ctx
        .scroll_offsets
        .entry(scroll_id.to_string())
        .or_insert(0.0);
    *offset = offset.clamp(0.0, max_scroll);
    let scroll = *offset;
    ctx.draw.push_scissor(bounds);
    let content_bounds = Rect::new(bounds.x, bounds.y - scroll, bounds.w, content_height);
    render_content(content_bounds, ctx);
    ctx.draw.pop_scissor();
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        event: None,
        control_id: Some(scroll_id.to_string()),
        kind: HitKind::ScrollRegion,
        drag_axis: None,
        drag_data: None,
    });
}

pub fn draw_icon<E>(ctx: &mut WidgetContext<'_, E>, uv: [f32; 4], x: f32, y: f32, size: f32, _color: Rgba) {
    ctx.draw.push_textured([x, y, size, size], uv, 1.0);
}

fn measure_text_width<E>(ctx: &mut WidgetContext<'_, E>, text: &str, size: f32) -> f32 {
    let (w, _) = ctx.atlas.measure_text(text, size);
    w
}

pub fn draw_text_wrapped<E>(
    ctx: &mut WidgetContext<'_, E>,
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    size: f32,
    color: Rgba,
) -> f32 {
    let lines = wrap_text(ctx.atlas, text, max_width, size);
    let line_h = size * 1.35;
    for (index, line) in lines.iter().enumerate() {
        draw_text(ctx, line, x, y + line_h * index as f32 + size, size, color);
    }
    lines.len() as f32 * line_h
}

pub fn wrap_text(atlas: &mut FontAtlas, text: &str, max_width: f32, size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let trial = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };
        let (w, _) = atlas.measure_text(&trial, size);
        if w > max_width && !current.is_empty() {
            lines.push(current);
            current = word.to_string();
        } else {
            current = trial;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

pub fn draw_text<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let scale = size / 16.0;
    let atlas_w = ctx.atlas.width as f32;
    let atlas_h = ctx.atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = ctx.atlas.ensure_glyph(ch);
        let gw = glyph.width as f32 * scale;
        let gh = glyph.height as f32 * scale;
        let gx = cursor_x + glyph.bearing_x * scale;
        let gy = y - gh - glyph.bearing_y * scale;
        let uv_rect = [
            glyph.atlas_x as f32 / atlas_w,
            glyph.atlas_y as f32 / atlas_h,
            (glyph.atlas_x + glyph.width) as f32 / atlas_w,
            (glyph.atlas_y + glyph.height) as f32 / atlas_h,
        ];
        ctx.draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance * scale;
    }
}
