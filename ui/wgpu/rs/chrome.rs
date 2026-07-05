//! 🎛 Bordered chrome primitives shared by widgets and shell renderers.

use crate::draw::{DrawList, IconAtlas};
use crate::geometry::Rect;
use crate::text::FontAtlas;
use crate::theme::{Rgba, Theme};

pub const ICON_TINY: f32 = 14.0;

pub fn push_control_border(draw: &mut DrawList, rect: Rect, theme: &Theme, border: Rgba, bg: Rgba) {
    let hair = theme.stroke_hairline;
    draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
    draw.push_solid([rect.x, rect.y, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y + rect.h - hair, rect.w, hair], border);
    draw.push_solid([rect.x, rect.y, hair, rect.h], border);
    draw.push_solid([rect.x + rect.w - hair, rect.y, hair, rect.h], border);
}

pub fn push_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, 1.0);
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

pub fn item_bg(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    if pressed {
        if hovered {
            theme.accent_hover
        } else {
            theme.selected
        }
    } else if hovered {
        theme.button_hover
    } else {
        theme.button
    }
}

pub fn item_text(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
    if pressed || hovered {
        theme.active_foreground
    } else {
        theme.text
    }
}
