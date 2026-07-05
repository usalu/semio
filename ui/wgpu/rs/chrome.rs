//! 🎛 Bordered chrome primitives shared by widgets and shell renderers.

use crate::draw::DrawList;
use crate::draw::IconAtlas;
use crate::geometry::Rect;
use crate::text::FontAtlas;
use crate::theme::{Rgba, Theme};

pub const ICON_TINY: f32 = 14.0;

pub const TRANSPARENT: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);

pub fn push_chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    let hair = theme.stroke_hairline;
    push_chrome_border(draw, rect, hair, theme.border_normal, true, true, true, true);
}

pub fn push_chrome_border(
    draw: &mut DrawList,
    rect: Rect,
    stroke: f32,
    color: Rgba,
    top: bool,
    right: bool,
    bottom: bool,
    left: bool,
) {
    if top {
        draw.push_solid([rect.x, rect.y, rect.w, stroke], color);
    }
    if bottom {
        draw.push_solid([rect.x, rect.y + rect.h - stroke, rect.w, stroke], color);
    }
    if left {
        draw.push_solid([rect.x, rect.y, stroke, rect.h], color);
    }
    if right {
        draw.push_solid([rect.x + rect.w - stroke, rect.y, stroke, rect.h], color);
    }
}

pub fn push_window_cap_border(draw: &mut DrawList, rect: Rect, stroke: f32, color: Rgba) {
    push_chrome_border(draw, rect, stroke, color, true, true, false, true);
}

pub fn push_control_border(draw: &mut DrawList, rect: Rect, theme: &Theme, border: Rgba, bg: Rgba) {
    if bg.a > 0.0 {
        draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
    }
    let hair = theme.stroke_hairline;
    draw.push_solid([rect.x, rect.y, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y + rect.h - hair, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y, hair, rect.h], border);
    draw.push_solid([rect.x + rect.w - hair, rect.y, hair, rect.h], border);
}

pub fn push_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

pub fn measure_action_item(
    atlas: &mut FontAtlas,
    theme: &Theme,
    icon: bool,
    label: Option<&str>,
) -> f32 {
    let icon_w = if icon {
        ICON_TINY + theme.gap_standard
    } else {
        0.0
    };
    let text_w = label
        .map(|value| atlas.measure_text(value, theme.font_size_small).0)
        .unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

pub fn chrome_item_bg(theme: &Theme, active: bool, hovered: bool) -> Rgba {
    if active {
        if hovered {
            theme.accent_hover
        } else {
            theme.selected
        }
    } else if hovered {
        theme.button_hover
    } else {
        TRANSPARENT
    }
}

pub fn chrome_item_text(theme: &Theme, active: bool, hovered: bool) -> Rgba {
    if active {
        theme.active_foreground
    } else if hovered {
        theme.border_emphasized
    } else {
        theme.text_element
    }
}

pub fn item_bg(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    chrome_item_bg(theme, pressed, hovered)
}

pub fn item_text(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    chrome_item_text(theme, pressed, hovered)
}
