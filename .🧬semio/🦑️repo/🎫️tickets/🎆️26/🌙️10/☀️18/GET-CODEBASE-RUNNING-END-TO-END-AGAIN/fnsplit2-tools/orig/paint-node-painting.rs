//! 🎨️ Paint node step with driver: the per-variant node painting of `paint_node_step_with_driver`. See [`super`].

use super::*;

// #region paint
#[allow(clippy::too_many_arguments, reason = "one retained paint context plus the canonical driver drag axis")]
pub fn paint_node_step_with_driver(
    tree: &UiTree,
    id: NodeId,
    origin_x: f32,
    origin_y: f32,
    theme: &Theme,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    has_scene_host: bool,
    driver_drag: UiDriverDrag,
    block_reversed: bool,
    inline: ui_contract::FlowInline,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    if cursor.node.is_none() {
        cursor.begin(id, origin_x, origin_y);
        return RetainedNodePaintStep::Pending;
    }
    if cursor.node != Some(id) || cursor.origin_x != origin_x || cursor.origin_y != origin_y {
        return RetainedNodePaintStep::Fault;
    }
    let Some(node) = tree.node(id) else { return RetainedNodePaintStep::Fault };
    let unscoped_body_font_size = theme.font_size_body;
    let mut scoped_theme = *theme;
    if semio_framework_ui_wgpu_mounted_layout::owning_tree_spec(tree, id).is_some_and(|owner| owner.presentation == ui_contract::TreePresentation::Compact) {
        scoped_theme.font_size_body = SIZE_TINY;
        scoped_theme.font_size_small = SIZE_TINY;
    }
    let theme = &scoped_theme;
    let Some(layout) = tree.accepted_layout(id) else { return RetainedNodePaintStep::Fault };
    let bounds = Rect::new(origin_x + layout.x, origin_y + layout.y, layout.width, layout.height);
    let presence = node.spec.0.presence();
    if !presence.visible() {
        return cursor.finish();
    }
    let mut flags = node.flags;
    if presence.state != UiState::Disabled {
        flags.set(NodeFlags::HOVERED, flags.contains(NodeFlags::HOVERED) || presence.hover);
    }
    // 🦴️ Busy elements draw their skeleton INSTEAD of their content, exactly like React's
    // `interpretUiNodeBusyShell` swaps the component for `elementSkeleton` — see 🦴️Skeleton.
    if skeleton_replaces_content(&node.spec.0, presence) {
        if cursor.phase == 0 {
            let step = retained_skeleton_step(&node.spec.0, bounds, theme, draw);
            cursor.advance(1);
            return step;
        }
        return match retained_presence_step_with_selection(draw, bounds, theme, presence, node_selection_outline(&node.spec.0)) {
            RetainedNodePaintStep::Complete => cursor.finish(),
            step => step,
        };
    }
    let step =
        match &node.spec.0 {
            UiNode::Text(text) => {
                if cursor.phase == 0 {
                    let emphasize = text.emphasize.unwrap_or(false);
                    // 🅰️ React's `TextView` is `text-foreground` in BOTH states and swaps `text-sm` for
                    // `font-semibold` (`🗣️Interpreter/🟦️.tsx:1168`), so plain body text is never muted
                    // here either, and an emphasized run is BOTH a step up the size ramp (it loses
                    // `text-sm`) and a real weight change. The weight half was a built-but-unwired
                    // primitive until ticket 26/09/17 packet W15a: `draw_text_weighted`/`TextWeight`
                    // existed with zero callers, so every emphasized `Text` painted as a size bump alone.
                    // No bold face ships on disk, so `Semibold` is the synthetic double strike
                    // `text::faux_bold_offset` describes — advances, and so wraps, unchanged.
                    let size = if emphasize { theme.font_size_emphasized } else { theme.font_size_body };
                    let color = theme.text;
                    let weight = semio_framework_ui_text::TextWeight::of(emphasize);
                    match retained_text_node_step_weighted(text.value.as_str(), bounds, size, color, weight, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(1);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::Stack(stack) => {
                if cursor.phase == 0 {
                    let owns_chrome = tree.authored_tree_item(id).is_none() && tree.authored_tree_section(id).is_none();
                    let result = retained_fixed_output(draw, |draw| {
                        if owns_chrome {
                            paint_stack_frame(stack, bounds, flags, theme, draw);
                        }
                    });
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::Separator(_) => {
                if cursor.phase == 0 {
                    let result = retained_fixed_output(draw, |draw| paint_separator(bounds, theme, draw));
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::Button(button) => match cursor.phase {
                0 => {
                    let hovered = flags.contains(NodeFlags::HOVERED);
                    let result = retained_fixed_output(draw, |draw| {
                        push_control_border(draw, bounds, theme, if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, item_bg(theme, false, hovered));
                        if let Some(icons) = icons {
                            push_icon(draw, icons, button.icon_id.as_str(), bounds.x + theme.padding_standard, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, false, hovered));
                        }
                    });
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                }
                1 => match retained_text_node_step(button.label.as_str(), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                },
                _ => retained_presence_step(draw, bounds, theme, presence),
            },
            UiNode::Input(input_node) => {
                let shown = input_node.display_factor.filter(|_| input_node.input_kind == "number").and_then(|factor| input_node.value.trim().parse::<f64>().ok().map(|stored| ui_contract::ui_number_display_text(stored, Some(factor), input_node.precision)));
                let refused = node.state.number_refusal.is_some() && node.state.edit.is_some();
                let display = node.state.edit.as_ref().map(|edit| edit.text.as_str()).filter(|text| !text.is_empty()).unwrap_or_else(|| {
                    if input_node.value.is_empty() {
                        input_node.placeholder.as_ref().map_or("", Label::as_str)
                    } else {
                        shown.as_deref().unwrap_or(input_node.value.as_str())
                    }
                });
                let long_text = input_node.input_kind == "longText";
                let display = if long_text { text_line_view(display, node.state.edit.as_ref().filter(|edit| !edit.text.is_empty()).map(|edit| edit.caret)).0 } else { display };
                let swatch = (input_node.input_kind == "color").then(|| color_input_swatch(bounds, display)).flatten();
                let text_bounds = swatch.map_or(bounds, |(_, _, text_bounds)| text_bounds);
                match cursor.phase {
                    0 => {
                        let result = retained_fixed_output(draw, |draw| {
                            push_control_border(draw, bounds, theme, if refused { theme.error } else if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, theme.input_bg);
                            if let Some((swatch, color, _)) = swatch {
                                draw.push_rounded([swatch.x, swatch.y, swatch.w, swatch.h], color, 2.0);
                            }
                        });
                        cursor.advance(1);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    1 => match retained_text_node_step(display, text_bounds, theme.font_size_body, if refused { theme.error } else if input_node.value.is_empty() { theme.text_muted } else { theme.text }, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(2);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    },
                    2 => {
                        let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                            cursor.advance(3);
                            return RetainedNodePaintStep::Pending;
                        };
                        let (caret_line, caret_at) = if long_text { text_line_view(&edit.text, Some(edit.caret)) } else { (edit.text.as_str(), edit.caret) };
                        match retained_caret_step(caret_line, caret_at, text_bounds, theme.font_size_body, RetainedCaretAlignment::Start, theme.accent, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.advance(3);
                                RetainedNodePaintStep::Pending
                            }
                            step => step,
                        }
                    }
                    _ => retained_presence_step(draw, bounds, theme, presence),
                }
            }
            UiNode::Select(select) => {
                if select.items.len() > RETAINED_NODE_COLLECTION_ITEMS {
                    return RetainedNodePaintStep::Fault;
                }
                match cursor.phase {
                    0 => {
                        let hovered = flags.contains(NodeFlags::HOVERED);
                        let result = retained_fixed_output(draw, |draw| {
                            push_control_border(draw, bounds, theme, if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, if hovered { theme.button_hover } else { theme.input_bg });
                            if let Some(icons) = icons {
                                push_icon(draw, icons, "chevron-down", bounds.x + bounds.w - theme.padding_standard - ICON_TINY, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, foreground_on_fill(theme, theme.text_element, false, hovered));
                            }
                        });
                        cursor.advance(1);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    1 if cursor.item < select.items.len() => {
                        if select.items[cursor.item].value == select.value {
                            cursor.selected = Some(cursor.item);
                        }
                        cursor.item += 1;
                        RetainedNodePaintStep::Pending
                    }
                    1 => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    2 => {
                        let label = cursor.selected.and_then(|index| select.items.get(index)).map(|item| item.label.as_str()).or_else(|| select.placeholder.as_ref().map(Label::as_str)).unwrap_or("Select…");
                        match retained_text_node_step(label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.advance(if node.state.open { 3 } else { 8 });
                                RetainedNodePaintStep::Pending
                            }
                            step => step,
                        }
                    }
                    3 => {
                        let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else { return RetainedNodePaintStep::Fault };
                        let mut opened = false;
                        let result = retained_fixed_output(draw, |draw| {
                            let glass = draw.push_glass([popup.menu.x, popup.menu.y, popup.menu.w, popup.menu.h], theme.border_radius, theme.glass(Level::Menu));
                            draw.begin_glass_content(glass);
                            push_chrome_border(draw, popup.menu, theme.stroke_hairline, theme.border_normal, true, true, true, true);
                            draw.push_scissor(semio_framework_ui_select_behavior::select_popup_viewport_rect(popup));
                            opened = true;
                        });
                        cursor.phase = 4;
                        cursor.item = popup.first_row;
                        cursor.glyph.reset();
                        cursor.popup_route = opened;
                        if result.is_err() {
                            let _ = retained_select_close_route(draw, cursor);
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    4 if node.state.select_popup.is_some_and(|popup| cursor.item < popup.last_row) => {
                        let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else {
                            let _ = retained_select_close_route(draw, cursor);
                            return RetainedNodePaintStep::Fault;
                        };
                        let row = semio_framework_ui_select_behavior::select_popup_row_rect(bounds, cursor.item, popup, theme);
                        let item = &select.items[cursor.item];
                        // ⌨️ The keyboard-highlighted row reads like a hovered one, as React's
                        // `data-highlighted` styling does — see `tree::WidgetState::highlighted`.
                        let highlighted = node.state.highlighted == Some(cursor.item);
                        let result = retained_fixed_output(draw, |draw| {
                            if highlighted || item.value == select.value {
                                draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
                            }
                        });
                        cursor.phase = 5;
                        cursor.glyph.reset();
                        if result.is_err() {
                            let _ = retained_select_close_route(draw, cursor);
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    4 => {
                        cursor.advance(6);
                        RetainedNodePaintStep::Pending
                    }
                    5 => {
                        let Some(item) = select.items.get(cursor.item) else {
                            let _ = retained_select_close_route(draw, cursor);
                            return RetainedNodePaintStep::Fault;
                        };
                        let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else {
                            let _ = retained_select_close_route(draw, cursor);
                            return RetainedNodePaintStep::Fault;
                        };
                        let painted = semio_framework_ui_select_behavior::select_popup_row_rect(bounds, cursor.item, popup, theme);
                        let row = Rect::new(painted.x + theme.padding_standard, painted.y, (painted.w - theme.padding_standard).max(0.0), painted.h);
                        match retained_text_node_step(item.label.as_str(), row, theme.font_size_body, theme.text, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.item += 1;
                                cursor.phase = 4;
                                cursor.glyph.reset();
                                RetainedNodePaintStep::Pending
                            }
                            RetainedNodePaintStep::Fault => {
                                let _ = retained_select_close_route(draw, cursor);
                                RetainedNodePaintStep::Fault
                            }
                            step => step,
                        }
                    }
                    6 => {
                        let Some(popup) = node.state.select_popup.and_then(|popup| retained_select_popup_at_bounds(tree, id, bounds, popup)) else {
                            let _ = retained_select_close_route(draw, cursor);
                            return RetainedNodePaintStep::Fault;
                        };
                        let result = retained_fixed_output(draw, |draw| {
                            draw.pop_scissor();
                            if let (Some(up), Some(down), Some(icons)) = (popup.up, popup.down, icons) {
                                let chevron = SIZE_TINY;
                                let center_x = popup.menu.x + (popup.menu.w - chevron) * 0.5;
                                push_icon(draw, icons, "chevron-up", center_x, up.y + (up.h - chevron) * 0.5, chevron, theme.text_muted);
                                push_icon(draw, icons, "chevron-down", center_x, down.y + (down.h - chevron) * 0.5, chevron, theme.text_muted);
                            }
                            draw.end_glass_content();
                        });
                        cursor.advance(7);
                        if result.is_err() {
                            let _ = retained_select_close_route(draw, cursor);
                            RetainedNodePaintStep::Fault
                        } else {
                            cursor.popup_route = false;
                            RetainedNodePaintStep::Pending
                        }
                    }
                    7 => {
                        let result = retained_select_close_route(draw, cursor);
                        cursor.advance(8);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    _ => retained_presence_step(draw, bounds, theme, presence),
                }
            }
            UiNode::Toggle(toggle) => match cursor.phase {
                0 => {
                    let pressed = toggle.presence.selected;
                    let hovered = flags.contains(NodeFlags::HOVERED);
                    let result = retained_fixed_output(draw, |draw| {
                        if toggle.appearance == ui_contract::ToggleAppearance::Checkbox {
                            paint_checkbox(toggle, bounds, flags, theme, inline, draw);
                            return;
                        }
                        push_control_border(draw, bounds, theme, if focus_ring_visible(flags) { theme.accent } else { theme.border_normal }, item_bg(theme, pressed, hovered));
                        if let Some(icons) = icons {
                            push_icon(draw, icons, toggle.icon_id.as_str(), bounds.x + theme.padding_standard, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, pressed, hovered));
                        }
                    });
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                }
                1 => {
                    if toggle.appearance == ui_contract::ToggleAppearance::Checkbox {
                        cursor.advance(2);
                        return RetainedNodePaintStep::Pending;
                    }
                    let label = toggle.text.as_ref().map_or("", Label::as_str);
                    match retained_text_node_step(label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(2);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                }
                _ => retained_presence_step_with_selection(draw, bounds, theme, presence, node_selection_outline(&node.spec.0)),
            },
            UiNode::KeyValue(key_value) => {
                if key_value.entries.len() > RETAINED_NODE_COLLECTION_ITEMS {
                    return RetainedNodePaintStep::Fault;
                }
                let Some(entry) = key_value.entries.get(cursor.item) else {
                    cursor.advance(2);
                    return match retained_presence_step(draw, bounds, theme, presence) {
                        RetainedNodePaintStep::Complete => cursor.finish(),
                        step => step,
                    };
                };
                let row = Rect::new(bounds.x, bounds.y + cursor.item as f32 * theme.control_height, bounds.w, theme.control_height);
                if cursor.phase == 0 {
                    match retained_text_node_step(entry.label.as_str(), row, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.phase = 1;
                            cursor.glyph.reset();
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                } else {
                    let value_bounds = Rect::new(row.x + row.w * 0.4, row.y, row.w * 0.6, row.h);
                    match retained_text_node_step(entry.value.as_str(), value_bounds, theme.font_size_small, theme.text, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.item += 1;
                            cursor.phase = 0;
                            cursor.glyph.reset();
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                }
            }
            UiNode::Slider(slider) => {
                let value = node.state.slider_draft_value.unwrap_or(slider.value);
                let unit_label = slider.unit_label();
                let control = semio_framework_ui_wgpu_layout::layout::slider_control_presentation(bounds, slider.axis_position(value), 0.0, 1.0, unit_label.as_ref().map(|_| layout.inline_suffix_width), theme.gap_standard, inline);
                let presentation = control.slider;
                let refusal = node.state.number_refusal.as_deref().filter(|_| node.state.edit.is_some());
                match cursor.phase {
                    0 if refusal.is_some_and(|text| !text.is_empty()) => {
                        cursor.advance(1);
                        RetainedNodePaintStep::Pending
                    }
                    0 => {
                        let result = retained_fixed_output_budgeted(draw, RETAINED_NODE_FIXED_OUTPUT_ITEMS.max(4 + slider.snaps.len()), |draw| paint_slider_travel(slider, value, presentation, theme, draw));
                        cursor.advance(2);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    1 => match retained_text_node_step(refusal.unwrap_or_default(), presentation.track_cell, theme.font_size_small, theme.error, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(2);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    },
                    2 => {
                        let formatted = slider.readout(value);
                        let label = node.state.edit.as_ref().map_or(formatted.as_str(), |edit| edit.text.as_str());
                        let color = if refusal.is_some() { theme.error } else if node.state.edit.is_some() { theme.text } else { theme.text_element };
                        let size = if node.state.edit.is_some() { unscoped_body_font_size } else { theme.font_size_small };
                        match retained_text_end_step(label, presentation.value_cell, size, color, inline, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.advance(3);
                                RetainedNodePaintStep::Pending
                            }
                            step => step,
                        }
                    }
                    3 => {
                        let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                            cursor.advance(4);
                            return RetainedNodePaintStep::Pending;
                        };
                        let alignment = if inline.is_rtl() { RetainedCaretAlignment::Start } else { RetainedCaretAlignment::End };
                        match retained_caret_step(&edit.text, edit.caret, presentation.value_cell, unscoped_body_font_size, alignment, theme.accent, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.advance(4);
                                RetainedNodePaintStep::Pending
                            }
                            step => step,
                        }
                    }
                    4 => {
                        let Some(label) = unit_label else {
                            cursor.advance(5);
                            return RetainedNodePaintStep::Pending;
                        };
                        let Some(unit_cell) = control.unit_cell else { return RetainedNodePaintStep::Fault };
                        match retained_tree_text_step(&label, unit_cell, unscoped_body_font_size, theme.text_muted, inline, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.advance(5);
                                RetainedNodePaintStep::Pending
                            }
                            step => step,
                        }
                    }
                    _ => retained_presence_step(draw, bounds, theme, presence),
                }
            }
            UiNode::NumberStepper(stepper) => {
                let [decrement, value_segment, increment] = semio_framework_ui_wgpu_layout::layout::number_stepper_segments(bounds, inline, theme.stroke_hairline);
                match cursor.phase {
                    0 => {
                        let result = retained_fixed_output(draw, |draw| draw.push_scissor(bounds));
                        cursor.advance(1);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    1 => {
                        let result = retained_fixed_output_budgeted(draw, RETAINED_NUMBER_STEPPER_CHROME_OUTPUT_ITEMS, |draw| {
                            push_stepper_chrome(draw, bounds, [decrement, value_segment, increment], inline, flags, node.state.stepper_hovered_segment, theme);
                        });
                        cursor.advance(2);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    2 => {
                        let result = retained_fixed_output(draw, |draw| push_stepper_icon(draw, icons, "minus", decrement, inline, theme));
                        cursor.advance(3);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    3 => {
                        let label = if let Some(edit) = node.state.edit.as_ref() {
                            let mut label = edit.text.clone();
                            if let Some(composition) = edit.composition.as_ref() {
                                label.insert_str(edit.caret, composition);
                            }
                            label
                        } else if stepper.uniform {
                            stepper.value_text(stepper.value)
                        } else {
                            UI_INSPECTOR_MIXED_PLACEHOLDER.to_string()
                        };
                        let color = if node.state.number_refusal.is_some() && node.state.edit.is_some() { theme.error } else if stepper.uniform || node.state.edit.is_some() { theme.text } else { theme.text_muted };
                        match retained_stepper_text_step(&label, value_segment, color, theme, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.advance(4);
                                RetainedNodePaintStep::Pending
                            }
                            step => step,
                        }
                    }
                    4 => {
                        let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                            cursor.advance(5);
                            return RetainedNodePaintStep::Pending;
                        };
                        let mut label = edit.text.clone();
                        let mut caret = edit.caret;
                        if let Some(composition) = edit.composition.as_ref() {
                            label.insert_str(edit.caret, composition);
                            caret += composition.len();
                        }
                        match retained_caret_step(&label, caret, value_segment, theme.font_size_body, RetainedCaretAlignment::Center, theme.accent, atlas, draw, cursor) {
                            RetainedNodePaintStep::Complete => {
                                cursor.advance(5);
                                RetainedNodePaintStep::Pending
                            }
                            step => step,
                        }
                    }
                    5 => {
                        let result = retained_fixed_output(draw, |draw| push_stepper_icon(draw, icons, "plus", increment, inline, theme));
                        cursor.advance(6);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    6 => {
                        let result = retained_fixed_output(draw, |draw| draw.pop_scissor());
                        cursor.advance(7);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    }
                    _ => retained_presence_step(draw, bounds, theme, presence),
                }
            }
            UiNode::Ring(ring) => {
                let segments = 48usize;
                if cursor.item < segments {
                    let cx = bounds.x + bounds.w * 0.5;
                    let cy = bounds.y + bounds.h * 0.5;
                    let radius = bounds.w.min(bounds.h) * 0.4;
                    let a0 = std::f32::consts::TAU * cursor.item as f32 / segments as f32;
                    let a1 = std::f32::consts::TAU * (cursor.item + 1) as f32 / segments as f32;
                    let result = retained_fixed_output(draw, |draw| draw.push_line(cx + a0.cos() * radius, cy + a0.sin() * radius, cx + a1.cos() * radius, cy + a1.sin() * radius, theme.separator, 2.0));
                    cursor.item += 1;
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else if cursor.phase == 0 {
                    let result = retained_fixed_output(draw, |draw| {
                        let cx = bounds.x + bounds.w * 0.5;
                        let cy = bounds.y + bounds.h * 0.5;
                        let radius = bounds.w.min(bounds.h) * 0.4;
                        let angle = std::f32::consts::TAU * ring.t as f32;
                        draw.push_rounded([cx + angle.cos() * radius - 6.0, cy + angle.sin() * radius - 6.0, 12.0, 12.0], theme.accent, 6.0);
                    });
                    cursor.phase = 1;
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::Progress(progress) => {
                if cursor.phase == 0 {
                    let result = retained_fixed_output(draw, |draw| paint_progress(progress, bounds, theme, draw));
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::IconSelect(select) => match cursor.phase {
                0 => {
                    let result = retained_fixed_output(draw, |draw| push_control_border(draw, bounds, theme, theme.border_normal, theme.input_bg));
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                }
                // ✍️ A focused `IconSelect` shows its LIVE buffer, not the declarative value —
                // `events`' `editable_value` seeds one for this kind exactly as it does for `Input`,
                // mirroring React's `IconSelector` editing its icon string in a textarea.
                1 => match retained_text_node_step(node.state.edit.as_ref().map_or(select.value.as_str(), |edit| edit.text.as_str()), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                },
                2 => {
                    let Some(edit) = node.state.edit.as_ref().filter(|edit| node.state.caret_visible && edit.anchor == edit.caret) else {
                        cursor.advance(3);
                        return RetainedNodePaintStep::Pending;
                    };
                    match retained_caret_step(&edit.text, edit.caret, bounds, theme.font_size_body, RetainedCaretAlignment::Start, theme.accent, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(3);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                }
                _ => retained_presence_step(draw, bounds, theme, presence),
            },
            UiNode::Field(field) => {
                let metrics = field_paint_metrics(tree, id, field, bounds, theme, atlas);
                let value = match cursor.phase {
                    0 => metrics.label.map(|band| (field.label.as_str(), chrome_band_rect(bounds, band), semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, theme.text, semio_framework_ui_text::TextWeight::Semibold, RetainedTextFlow::Clip)),
                    1 if field.required.unwrap_or(false) => metrics.label.map(|band| {
                        let label_width = atlas.measure_text(field.label.as_str(), semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE).0;
                        let mut rect = chrome_band_rect(bounds, band);
                        rect.x += label_width + theme.gap_standard * 0.5;
                        rect.w = (rect.w - label_width - theme.gap_standard * 0.5).max(0.0);
                        ("*", rect, semio_framework_ui_wgpu_layout::flex::FIELD_LABEL_FONT_SIZE, theme.error, semio_framework_ui_text::TextWeight::Semibold, RetainedTextFlow::Clip)
                    }),
                    2 => field
                        .description
                        .as_deref()
                        .zip(metrics.description)
                        .map(|(value, band)| (value, chrome_band_rect(bounds, band), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.text_muted, semio_framework_ui_text::TextWeight::Regular, RetainedTextFlow::Wrap)),
                    3 => field.error.as_deref().zip(metrics.error).map(|(value, band)| (value, chrome_band_rect(bounds, band), semio_framework_ui_wgpu_layout::flex::FIELD_DETAIL_FONT_SIZE, theme.error, semio_framework_ui_text::TextWeight::Regular, RetainedTextFlow::Wrap)),
                    _ => None,
                };
                if let Some((value, rect, size, color, weight, flow)) = value {
                    match paint_retained_glyph_step_weighted(value, rect, size, color, flow, weight, atlas, draw, &mut cursor.glyph) {
                        RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
                        RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
                        RetainedGlyphStep::Complete => {
                            cursor.advance(cursor.phase + 1);
                            RetainedNodePaintStep::Pending
                        }
                    }
                } else if cursor.phase < 4 {
                    cursor.advance(cursor.phase + 1);
                    RetainedNodePaintStep::Pending
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::Section(section) => {
                if cursor.phase == 0 {
                    let metrics = section_paint_metrics(tree, id, section, bounds, atlas);
                    let Some((label, title)) = section.label.as_ref().zip(metrics.title) else {
                        cursor.advance(1);
                        return RetainedNodePaintStep::Pending;
                    };
                    match retained_text_node_step_weighted(label.as_str(), chrome_band_rect(bounds, title), semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_FONT_SIZE, theme.text, semio_framework_ui_text::TextWeight::Semibold, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(1);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::Group(_) if semio_framework_ui_wgpu_mounted_layout::owning_tree_spec(tree, id).is_some() => retained_presence_step(draw, bounds, theme, presence),
            UiNode::Group(group) => {
                if cursor.phase == 0 {
                    let result = retained_fixed_output(draw, |draw| {
                        if let Some(icons) = icons {
                            push_icon(draw, icons, if group.default_open.unwrap_or(true) { "chevron-down" } else { "chevron-right" }, bounds.x, bounds.y, ICON_TINY, theme.text_element);
                        }
                    });
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else if cursor.phase == 1 {
                    match retained_text_node_step(group.label.as_str(), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(2);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::Tree(tree_node) => retained_tree_node_step(tree, id, tree_node, bounds, theme, driver_drag, block_reversed, inline, atlas, icons, draw, cursor),
            // 🖼️ No `SceneHost` this tick: decode what this process can (a `data:` PNG) and draw the real
            // bitmap as a `KIND_RASTER` quad at React's `object-contain` rect — see 🖼️UiImageSources. A
            // source only the host can fetch still falls back to the placeholder + `alt` chrome below,
            // which is what React shows for a broken/pending `<img>` too.
            UiNode::Image(image) => {
                // 🖼️ Admission is the production caller of `admit_ui_image` — without this line the
                // ledger stayed empty forever, `ui_image_natural_size` always answered `None`, and EVERY
                // `UiNode::Image` painted the `alt` placeholder however decodable its `src` was. Safe per
                // frame: an already-decoded source is a ledger lookup, never a second decode, and a
                // source only a host can fetch answers `Deferred`/`Unsupported` and falls through to the
                // same placeholder chrome React shows for a pending/broken `<img>`.
                let decoded = if has_scene_host { None } else { matches!(admit_ui_image(image.src.as_str()), UiImageAdmission::Ready).then(|| ui_image_natural_size(image.src.as_str())).flatten() };
                if has_scene_host {
                    retained_presence_step(draw, bounds, theme, presence)
                } else if let Some((natural_w, natural_h)) = decoded {
                    if cursor.phase == 0 {
                        let content = ui_image_content_rect(bounds, natural_w, natural_h);
                        let result = retained_fixed_output(draw, |draw| draw.push_raster_quad(image.src.as_str(), [content.x, content.y, content.w, content.h], [0.0, 0.0, 1.0, 1.0], 1.0));
                        cursor.advance(2);
                        if result.is_err() {
                            RetainedNodePaintStep::Fault
                        } else {
                            RetainedNodePaintStep::Pending
                        }
                    } else {
                        retained_presence_step(draw, bounds, theme, presence)
                    }
                } else if cursor.phase == 0 {
                    let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else if cursor.phase == 1 {
                    let label = image.alt.as_ref().map_or(image.id.as_str(), Label::as_str);
                    match retained_text_node_step(label, bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(2);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::ComponentScene(scene) => {
                if has_scene_host {
                    retained_presence_step(draw, bounds, theme, presence)
                } else if cursor.phase == 0 {
                    let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else if cursor.phase == 1 {
                    match retained_text_node_step(scene.surface_id.as_str(), bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(2);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
            UiNode::ExternalSlot(slot) => {
                if cursor.phase == 0 {
                    let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                } else if cursor.phase == 1 {
                    match retained_text_node_step(slot.host_status.as_deref().unwrap_or(slot.body_key.as_str()), bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(2);
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                } else {
                    retained_presence_step(draw, bounds, theme, presence)
                }
            }
        };
    match step {
        RetainedNodePaintStep::Complete => cursor.finish(),
        step => step,
    }
}
// #endregion paint
