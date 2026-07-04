//! 🧩 UiNode interpreter — lays out and draws declarative plugin UI trees.

use crate::draw::DrawList;
use crate::input::{HitKind, HitTarget, InputState};
use crate::layout_engine::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
use crate::scenes::render_component_scene;
use crate::text::FontAtlas;
use crate::theme::{
    Rect, Rgba, BORDER_RADIUS, CONTROL_HEIGHT, FONT_SIZE_BODY, FONT_SIZE_SMALL, GAP_STANDARD,
};
use semio_framework_core::{
    CommandDescriptor, UiButtonNode, UiControlNode, UiFieldNode, UiIconSelectNode, UiInputNode,
    UiKeyValueNode, UiNode, UiNumberStepperNode, UiRingNode, UiSectionNode, UiSelectNode,
    UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiVec3Node,
};

pub struct WidgetContext<'a> {
    pub draw: &'a mut DrawList,
    pub atlas: &'a mut FontAtlas,
    pub input: &'a mut InputState,
}

pub fn measure_node(atlas: &mut FontAtlas, node: &UiNode) -> (f32, f32) {
    match node {
        UiNode::Stack(stack) => measure_stack(atlas, stack),
        UiNode::Text(text) => {
            let size = if text.emphasize.unwrap_or(false) {
                FONT_SIZE_BODY + 1.0
            } else {
                FONT_SIZE_BODY
            };
            atlas.measure_text(&text.value, size)
        }
        UiNode::Separator(_) => (CONTROL_HEIGHT, 1.0),
        UiNode::Button(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiNode::Input(_) | UiNode::Select(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiNode::Toggle(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiNode::Vec3(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT * 3.0 + GAP_STANDARD * 2.0),
        UiNode::KeyValue(kv) => (CONTROL_HEIGHT, kv.entries.len() as f32 * 22.0),
        UiNode::Slider(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiNode::NumberStepper(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiNode::Ring(_) => (80.0, 80.0),
        UiNode::IconSelect(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiNode::Field(field) => {
            let (_, ch) = measure_control(atlas, &field.child);
            (ch + 18.0, ch + 18.0)
        }
        UiNode::Section(section) => measure_section(atlas, section),
        UiNode::Tree(tree) => (200.0, tree.sections.len() as f32 * 120.0),
        UiNode::ComponentScene(scene) => (320.0, 240.0),
    }
}

fn measure_stack(atlas: &mut FontAtlas, stack: &UiStackNode) -> (f32, f32) {
    let gap = gap_for_token(stack.gap.as_deref());
    let padding = padding_for_token(stack.padding.as_deref()) * 2.0;
    let vertical = stack.direction != "horizontal";
    let mut total_main = 0.0f32;
    let mut max_cross = 0.0f32;
    for (index, child) in stack.children.iter().enumerate() {
        let (w, h) = measure_node(atlas, child);
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

fn measure_control(atlas: &mut FontAtlas, control: &UiControlNode) -> (f32, f32) {
    match control {
        UiControlNode::Button(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiControlNode::Input(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiControlNode::Select(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiControlNode::Toggle(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiControlNode::Vec3(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT * 3.0),
        UiControlNode::KeyValue(kv) => (CONTROL_HEIGHT, kv.entries.len() as f32 * 22.0),
        UiControlNode::Slider(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiControlNode::NumberStepper(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
        UiControlNode::Ring(_) => (80.0, 80.0),
        UiControlNode::IconSelect(_) => (CONTROL_HEIGHT, CONTROL_HEIGHT),
    }
}

fn measure_section(atlas: &mut FontAtlas, section: &UiSectionNode) -> (f32, f32) {
    let mut height = PANEL_HEADER;
    for child in &section.children {
        let (_, h) = measure_node(atlas, child);
        height += h + GAP_STANDARD;
    }
    (200.0, height)
}

const PANEL_HEADER: f32 = 24.0;

pub fn render_ui_node(node: &UiNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    match node {
        UiNode::Stack(stack) => render_stack(stack, bounds, ctx),
        UiNode::Text(text) => render_text(text, bounds, ctx),
        UiNode::Separator(_) => render_separator(bounds, ctx),
        UiNode::Button(button) => render_button(button, bounds, ctx),
        UiNode::Input(input) => render_input(input, bounds, ctx),
        UiNode::Select(select) => render_select(select, bounds, ctx),
        UiNode::Toggle(toggle) => render_toggle(toggle, bounds, ctx),
        UiNode::Vec3(vec3) => render_vec3(vec3, bounds, ctx),
        UiNode::KeyValue(kv) => render_key_value(kv, bounds, ctx),
        UiNode::Slider(slider) => render_slider(slider, bounds, ctx),
        UiNode::NumberStepper(stepper) => render_number_stepper(stepper, bounds, ctx),
        UiNode::Ring(ring) => render_ring(ring, bounds, ctx),
        UiNode::IconSelect(icon) => render_icon_select(icon, bounds, ctx),
        UiNode::Field(field) => render_field(field, bounds, ctx),
        UiNode::Section(section) => render_section(section, bounds, ctx),
        UiNode::Tree(tree) => render_tree(tree, bounds, ctx),
        UiNode::ComponentScene(scene) => render_component_scene(scene, bounds, ctx),
    }
}

fn render_stack(stack: &UiStackNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let gap = gap_for_token(stack.gap.as_deref());
    let padding = padding_for_token(stack.padding.as_deref());
    let vertical = stack.direction != "horizontal";
    let sizes: Vec<f32> = stack
        .children
        .iter()
        .map(|child| {
            let (w, h) = measure_node(ctx.atlas, child);
            if vertical { h } else { w }
        })
        .collect();
    let rects = if vertical {
        layout_vertical(bounds, gap, padding, &sizes)
    } else {
        layout_horizontal(bounds, gap, padding, &sizes)
    };
    for (child, rect) in stack.children.iter().zip(rects.iter()) {
        render_ui_node(child, *rect, ctx);
    }
}

fn render_text(text: &UiTextNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let size = if text.emphasize.unwrap_or(false) {
        FONT_SIZE_BODY + 1.0
    } else {
        FONT_SIZE_BODY
    };
    let color = if text.emphasize.unwrap_or(false) {
        Rgba::TEXT
    } else {
        Rgba::TEXT_MUTED
    };
    draw_text(ctx, &text.value, bounds.x, bounds.y + size, size, color);
}

fn render_separator(bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let y = bounds.y + bounds.h * 0.5;
    ctx.draw.push_line(bounds.x, y, bounds.x + bounds.w, y, Rgba::SEPARATOR, 1.0);
}

fn render_button(button: &UiButtonNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let hovered = ctx
        .input
        .hit_at(ctx.input.pointer_x, ctx.input.pointer_y)
        .and_then(|h| h.control_id.as_deref())
        == button.id.as_deref().or(Some(&button.label));
    let bg = if hovered { Rgba::BUTTON_HOVER } else { Rgba::BUTTON };
    ctx.draw
        .push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], bg, BORDER_RADIUS);
    draw_text(
        ctx,
        &button.label,
        bounds.x + 8.0,
        bounds.y + (bounds.h + FONT_SIZE_BODY) * 0.5 - 2.0,
        FONT_SIZE_BODY,
        Rgba::TEXT,
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(button.command.clone()),
        control_id: button.id.clone().or_else(|| Some(button.label.clone())),
        kind: HitKind::Button,
    });
}

fn render_input(input: &UiInputNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    ctx.draw
        .push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], Rgba::INPUT_BG, BORDER_RADIUS);
    let (display, muted) = if ctx.input.focused_id.as_deref() == Some(input.id.as_str()) {
        (ctx.input.text_buffer.clone(), false)
    } else if input.value.is_empty() {
        (
            input.placeholder.clone().unwrap_or_default(),
            true,
        )
    } else {
        (input.value.clone(), false)
    };
    draw_text(
        ctx,
        &display,
        bounds.x + 8.0,
        bounds.y + (bounds.h + FONT_SIZE_BODY) * 0.5 - 2.0,
        FONT_SIZE_BODY,
        if muted { Rgba::TEXT_MUTED } else { Rgba::TEXT },
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: None,
        control_id: Some(input.id.clone()),
        kind: HitKind::Input,
    });
}

fn render_select(select: &UiSelectNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    ctx.draw
        .push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], Rgba::INPUT_BG, BORDER_RADIUS);
    let label = select
        .items
        .iter()
        .find(|item| item.value == select.value)
        .map(|item| item.label.as_str())
        .unwrap_or(select.placeholder.as_deref().unwrap_or("Select…"));
    draw_text(
        ctx,
        label,
        bounds.x + 8.0,
        bounds.y + (bounds.h + FONT_SIZE_BODY) * 0.5 - 2.0,
        FONT_SIZE_BODY,
        Rgba::TEXT,
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(select.on_change.clone()),
        control_id: Some(select.id.clone()),
        kind: HitKind::Select,
    });
}

fn render_toggle(toggle: &UiToggleNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let bg = if toggle.pressed { Rgba::ACCENT } else { Rgba::BUTTON };
    ctx.draw
        .push_rounded([bounds.x, bounds.y, bounds.w.min(36.0), bounds.h], bg, BORDER_RADIUS);
    if let Some(text) = &toggle.text {
        draw_text(
            ctx,
            text,
            bounds.x + 44.0,
            bounds.y + (bounds.h + FONT_SIZE_BODY) * 0.5 - 2.0,
            FONT_SIZE_BODY,
            Rgba::TEXT,
        );
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(toggle.on_change.clone()),
        control_id: Some(toggle.id.clone()),
        kind: HitKind::Toggle,
    });
}

fn render_vec3(vec3: &UiVec3Node, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let values = vec3.value.unwrap_or([0.0, 0.0, 0.0]);
    let labels = ["X", "Y", "Z"];
    for (index, label) in labels.iter().enumerate() {
        let y = bounds.y + index as f32 * (CONTROL_HEIGHT + 4.0);
        let row = Rect::new(bounds.x, y, bounds.w, CONTROL_HEIGHT);
        ctx.draw
            .push_rounded([row.x, row.y, row.w, row.h], Rgba::INPUT_BG, BORDER_RADIUS);
        let text = format!("{label}: {:.3}", values[index]);
        draw_text(ctx, &text, row.x + 8.0, row.y + 18.0, FONT_SIZE_SMALL, Rgba::TEXT);
    }
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(vec3.on_change.clone()),
        control_id: Some(vec3.id.clone()),
        kind: HitKind::Generic,
    });
}

fn render_key_value(kv: &UiKeyValueNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    for (index, entry) in kv.entries.iter().enumerate() {
        let y = bounds.y + index as f32 * 22.0;
        draw_text(ctx, &entry.label, bounds.x, y + FONT_SIZE_SMALL, FONT_SIZE_SMALL, Rgba::TEXT_MUTED);
        draw_text(
            ctx,
            &entry.value,
            bounds.x + bounds.w * 0.4,
            y + FONT_SIZE_SMALL,
            FONT_SIZE_SMALL,
            Rgba::TEXT,
        );
    }
}

fn render_slider(slider: &UiSliderNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let track_y = bounds.y + bounds.h * 0.5;
    ctx.draw
        .push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], Rgba::SEPARATOR, 2.0);
    let t = ((slider.value - slider.min) / (slider.max - slider.min)).clamp(0.0, 1.0);
    let knob_x = bounds.x + bounds.w * t as f32;
    ctx.draw
        .push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], Rgba::ACCENT, 6.0);
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(slider.on_change.clone()),
        control_id: Some(slider.id.clone()),
        kind: HitKind::Slider,
    });
}

fn render_number_stepper(stepper: &UiNumberStepperNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    ctx.draw
        .push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], Rgba::INPUT_BG, BORDER_RADIUS);
    let text = format!("{:.3}", stepper.value);
    draw_text(
        ctx,
        &text,
        bounds.x + 8.0,
        bounds.y + (bounds.h + FONT_SIZE_BODY) * 0.5 - 2.0,
        FONT_SIZE_BODY,
        Rgba::TEXT,
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(stepper.on_absolute.clone()),
        control_id: Some(stepper.id.clone()),
        kind: HitKind::Generic,
    });
}

fn render_ring(ring: &UiRingNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
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
        ctx.draw
            .push_line(window[0][0], window[0][1], window[1][0], window[1][1], Rgba::SEPARATOR, 2.0);
    }
    let knob_angle = std::f32::consts::TAU * ring.t as f32;
    let kx = cx + knob_angle.cos() * radius;
    let ky = cy + knob_angle.sin() * radius;
    ctx.draw
        .push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], Rgba::ACCENT, 6.0);
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(ring.on_change.clone()),
        control_id: Some(ring.id.clone()),
        kind: HitKind::Generic,
    });
}

fn render_icon_select(icon: &UiIconSelectNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    ctx.draw
        .push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], Rgba::BUTTON, BORDER_RADIUS);
    draw_text(
        ctx,
        &icon.value,
        bounds.x + 8.0,
        bounds.y + (bounds.h + FONT_SIZE_BODY) * 0.5 - 2.0,
        FONT_SIZE_BODY,
        Rgba::TEXT,
    );
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        command: Some(icon.on_change.clone()),
        control_id: Some(icon.id.clone()),
        kind: HitKind::Generic,
    });
}

fn render_field(field: &UiFieldNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    draw_text(ctx, &field.label, bounds.x, bounds.y + FONT_SIZE_SMALL, FONT_SIZE_SMALL, Rgba::TEXT_MUTED);
    let child_bounds = Rect::new(bounds.x, bounds.y + 18.0, bounds.w, bounds.h - 18.0);
    match &field.child {
        UiControlNode::Button(n) => render_button(n, child_bounds, ctx),
        UiControlNode::Input(n) => render_input(n, child_bounds, ctx),
        UiControlNode::Select(n) => render_select(n, child_bounds, ctx),
        UiControlNode::Toggle(n) => render_toggle(n, child_bounds, ctx),
        UiControlNode::Vec3(n) => render_vec3(n, child_bounds, ctx),
        UiControlNode::KeyValue(n) => render_key_value(n, child_bounds, ctx),
        UiControlNode::Slider(n) => render_slider(n, child_bounds, ctx),
        UiControlNode::NumberStepper(n) => render_number_stepper(n, child_bounds, ctx),
        UiControlNode::Ring(n) => render_ring(n, child_bounds, ctx),
        UiControlNode::IconSelect(n) => render_icon_select(n, child_bounds, ctx),
    }
}

fn render_section(section: &UiSectionNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let label = section.label.as_deref().unwrap_or(&section.id);
    draw_text(ctx, label, bounds.x, bounds.y + FONT_SIZE_BODY, FONT_SIZE_BODY, Rgba::TEXT);
    let mut y = bounds.y + PANEL_HEADER;
    for child in &section.children {
        let (_, h) = measure_node(ctx.atlas, child);
        let child_bounds = Rect::new(bounds.x, y, bounds.w, h);
        render_ui_node(child, child_bounds, ctx);
        y += h + GAP_STANDARD;
    }
}

fn render_tree(tree: &UiTreeNode, bounds: Rect, ctx: &mut WidgetContext<'_>) {
    let mut y = bounds.y;
    for section in &tree.sections {
        if let Some(label) = &section.label {
            draw_text(ctx, label, bounds.x, y + FONT_SIZE_BODY, FONT_SIZE_SMALL, Rgba::TEXT_MUTED);
            y += 20.0;
        }
        for item in &section.items {
            render_tree_item(item, Rect::new(bounds.x + 8.0, y, bounds.w - 8.0, 22.0), ctx, 0);
            y += 22.0;
        }
        y += 8.0;
    }
}

fn render_tree_item(item: &UiTreeItemNode, bounds: Rect, ctx: &mut WidgetContext<'_>, depth: u32) {
    let selected = item.selected.unwrap_or(false);
    if selected {
        ctx.draw
            .push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], Rgba::SELECTED, 4.0);
    }
    draw_text(
        ctx,
        &item.label,
        bounds.x + depth as f32 * 12.0,
        bounds.y + FONT_SIZE_BODY,
        FONT_SIZE_BODY,
        Rgba::TEXT,
    );
    if let Some(command) = &item.command {
        ctx.input.register_hit(HitTarget {
            rect: bounds,
            command: Some(command.clone()),
            control_id: Some(item.id.clone()),
            kind: HitKind::TreeItem,
        });
    }
    if let Some(children) = &item.items {
        let mut y = bounds.y + 22.0;
        for child in children {
            let child_bounds = Rect::new(bounds.x, y, bounds.w, 22.0);
            render_tree_item(child, child_bounds, ctx, depth + 1);
            y += 22.0;
        }
    }
}

pub fn draw_text(ctx: &mut WidgetContext<'_>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
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
        ctx.draw
            .push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance * scale;
    }
}
