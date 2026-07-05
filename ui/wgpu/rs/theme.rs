//! 🎨 Theme colors and metrics for wgpu UI rendering.

use crate::geometry::Rect;
use ui_styling::{
    metrics::{chrome as chrome_metrics, dom, typography},
    opacities, radii, strokes, ChromeTheme, CHROME_DARK, CHROME_LIGHT,
};
use ui_styling::theme::ThemeName;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_srgb8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let [lr, lg, lb, la] = ui_styling::color::rgba8_to_linear(r, g, b, a);
        Self::new(lr, lg, lb, la)
    }

    fn from_chrome(c: &[f32; 4]) -> Self {
        Self::new(c[0], c[1], c[2], c[3])
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub background: Rgba,
    pub panel: Rgba,
    pub panel_border: Rgba,
    pub navbar: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub active_foreground: Rgba,
    pub button: Rgba,
    pub button_hover: Rgba,
    pub input_bg: Rgba,
    pub separator: Rgba,
    pub selected: Rgba,
    pub canvas_clear: Rgba,
    pub gap_standard: f32,
    pub padding_standard: f32,
    pub navbar_height: f32,
    pub panel_header_height: f32,
    pub control_height: f32,
    pub control_height_small: f32,
    pub glass_panel_alpha: f32,
    pub font_size_body: f32,
    pub font_size_small: f32,
    pub font_size_emphasized: f32,
    pub footer_height: f32,
    pub panel_inset: f32,
    pub panel_min_width: f32,
    pub panel_max_width: f32,
    pub overlay_bg: Rgba,
    pub overlay_shadow: Rgba,
    pub focus_ring: Rgba,
    pub row_hover: Rgba,
    pub border_radius: f32,
    pub border_normal: Rgba,
    pub border_emphasized: Rgba,
    pub text_element: Rgba,
    pub stroke_hairline: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

fn chrome_px(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

fn panel_width(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

fn from_chrome(chrome: &ChromeTheme) -> Theme {
    Theme {
        background: Rgba::from_chrome(&chrome.canvas),
        panel: Rgba::from_chrome(&chrome.panel),
        panel_border: Rgba::from_chrome(&chrome.border_normal),
        navbar: Rgba::from_chrome(&chrome.window),
        text: Rgba::from_chrome(&chrome.foreground),
        text_muted: Rgba::from_chrome(&chrome.muted_foreground),
        accent: Rgba::from_chrome(&chrome.accent),
        accent_hover: Rgba::from_chrome(&chrome.active_hover),
        active_foreground: Rgba::from_chrome(&chrome.active_foreground),
        button: Rgba::from_chrome(&chrome.window),
        button_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
        input_bg: Rgba::from_chrome(&chrome.canvas),
        separator: Rgba::from_chrome(&chrome.border_normal),
        selected: Rgba::from_chrome(&chrome.active_base),
        canvas_clear: Rgba::from_chrome(&chrome.canvas),
        gap_standard: chrome_px(chrome_metrics::GAP_STANDARD_UI_SPACING),
        padding_standard: chrome_px(chrome_metrics::PADDING_STANDARD_UI_SPACING),
        navbar_height: chrome_px(chrome_metrics::NAVBAR_HEIGHT_UI_SPACING),
        panel_header_height: chrome_px(chrome_metrics::PANEL_HEADER_HEIGHT_UI_SPACING),
        control_height: chrome_px(chrome_metrics::CONTROL_HEIGHT_UI_SPACING),
        control_height_small: chrome_px(5.0),
        glass_panel_alpha: opacities::GLASS_PANEL_ALPHA as f32,
        font_size_body: typography::TEXT_SM_PX as f32,
        font_size_small: typography::TEXT_XS_PX as f32,
        font_size_emphasized: typography::TEXT_BASE_PX as f32,
        footer_height: chrome_px(chrome_metrics::FOOTER_HEIGHT_UI_SPACING),
        panel_inset: chrome_px(chrome_metrics::PANEL_INSET_UI_SPACING),
        panel_min_width: panel_width(dom::LAYOUT_PANEL_MIN_UI_SPACING),
        panel_max_width: panel_width(dom::LAYOUT_PANEL_MAX_UI_SPACING),
        overlay_bg: Rgba::from_chrome(&chrome.overlay_bg),
        overlay_shadow: Rgba::new(0.0, 0.0, 0.0, 0.0),
        focus_ring: Rgba::from_chrome(&chrome.accent).with_alpha(0.6),
        row_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
        border_radius: radii::CHROME as f32,
        border_normal: Rgba::from_chrome(&chrome.border_normal),
        border_emphasized: Rgba::from_chrome(&chrome.border_emphasized),
        text_element: Rgba::from_chrome(&chrome.border_element),
        stroke_hairline: strokes::CHROME_BORDER_HAIRLINE as f32,
    }
}

impl Theme {
    pub fn light() -> Self {
        from_chrome(&CHROME_LIGHT)
    }

    pub fn dark() -> Self {
        from_chrome(&CHROME_DARK)
    }

    pub fn for_name(name: ThemeName) -> Self {
        match name {
            ThemeName::Light => Self::light(),
            ThemeName::Dark => Self::dark(),
        }
    }

    pub fn glass_panel_fill(&self) -> Rgba {
        let a = self.glass_panel_alpha;
        let [pr, pg, pb, _] = ui_styling::color::linear_to_rgba8(self.panel.r, self.panel.g, self.panel.b, self.panel.a);
        let [cr, cg, cb, _] = ui_styling::color::linear_to_rgba8(
            self.canvas_clear.r,
            self.canvas_clear.g,
            self.canvas_clear.b,
            self.canvas_clear.a,
        );
        let mix = |panel: u8, canvas: u8| -> u8 {
            (f32::from(panel) * a + f32::from(canvas) * (1.0 - a)).round().clamp(0.0, 255.0) as u8
        };
        Rgba::from_srgb8(mix(pr, cr), mix(pg, cg), mix(pb, cb), 255)
    }
}

pub type ThemedRect = Rect;

#[cfg(test)]
mod tests {
    use super::{Rgba, Theme};
    use ui_styling::color::linear_to_rgba8;

    #[test]
    fn light_window_token_matches_react_navbar_hex() {
        let theme = Theme::light();
        let [r, g, b, _] = linear_to_rgba8(theme.navbar.r, theme.navbar.g, theme.navbar.b, theme.navbar.a);
        assert_eq!([r, g, b], [235, 232, 217]);
    }

    #[test]
    fn light_canvas_token_matches_react_canvas_hex() {
        let theme = Theme::light();
        let [r, g, b, _] = linear_to_rgba8(theme.canvas_clear.r, theme.canvas_clear.g, theme.canvas_clear.b, theme.canvas_clear.a);
        assert_eq!([r, g, b], [240, 236, 221]);
    }

    #[test]
    fn glass_panel_fill_matches_react_color_mix_over_canvas() {
        let theme = Theme::light();
        let fill = theme.glass_panel_fill();
        let [r, g, b, _] = linear_to_rgba8(fill.r, fill.g, fill.b, fill.a);
        assert_eq!([r, g, b], [217, 215, 202]);
    }

    #[test]
    fn chrome_item_default_is_transparent() {
        use crate::chrome::chrome_item_bg;
        let theme = Theme::light();
        let bg = chrome_item_bg(&theme, false, false);
        assert_eq!(bg.a, 0.0);
    }
}
