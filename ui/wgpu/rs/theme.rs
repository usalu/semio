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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassTier {
    Panel,
    Toolbar,
    Menu,
    WindowOptions,
}

#[derive(Clone, Copy, Debug)]
pub struct GlassStyle {
    pub tint: Rgba,
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
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
    pub temporary: Rgba,
    pub gap_standard: f32,
    pub padding_standard: f32,
    pub navbar_height: f32,
    pub panel_header_height: f32,
    pub control_height: f32,
    pub control_height_small: f32,
    pub glass_saturate: f32,
    pub font_size_body: f32,
    pub font_size_small: f32,
    pub font_size_emphasized: f32,
    pub footer_height: f32,
    pub panel_inset: f32,
    pub panel_min_width: f32,
    pub panel_max_width: f32,
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
        temporary: Rgba::from_chrome(&chrome.temporary),
        gap_standard: chrome_px(chrome_metrics::GAP_STANDARD_UI_SPACING),
        padding_standard: chrome_px(chrome_metrics::PADDING_STANDARD_UI_SPACING),
        navbar_height: chrome_px(chrome_metrics::NAVBAR_HEIGHT_UI_SPACING),
        panel_header_height: chrome_px(chrome_metrics::PANEL_HEADER_HEIGHT_UI_SPACING),
        control_height: chrome_px(chrome_metrics::CONTROL_HEIGHT_UI_SPACING),
        control_height_small: chrome_px(5.0),
        glass_saturate: chrome_metrics::GLASS_SATURATE as f32,
        font_size_body: typography::TEXT_SM_PX as f32,
        font_size_small: typography::TEXT_XS_PX as f32,
        font_size_emphasized: typography::TEXT_BASE_PX as f32,
        footer_height: chrome_px(chrome_metrics::FOOTER_HEIGHT_UI_SPACING),
        panel_inset: chrome_px(chrome_metrics::PANEL_INSET_UI_SPACING),
        panel_min_width: panel_width(dom::LAYOUT_PANEL_MIN_UI_SPACING),
        panel_max_width: panel_width(dom::LAYOUT_PANEL_MAX_UI_SPACING),
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

    pub fn glass(&self, tier: GlassTier) -> GlassStyle {
        match tier {
            GlassTier::Panel => GlassStyle {
                tint: self.panel,
                alpha: opacities::GLASS_PANEL_ALPHA as f32,
                blur_px: chrome_metrics::GLASS_PANEL_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
            GlassTier::Toolbar => GlassStyle {
                tint: self.panel,
                alpha: 0.3,
                blur_px: chrome_metrics::GLASS_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
            GlassTier::Menu => GlassStyle {
                tint: self.temporary,
                alpha: opacities::GLASS_MENU_ALPHA as f32,
                blur_px: chrome_metrics::GLASS_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
            GlassTier::WindowOptions => GlassStyle {
                tint: self.panel,
                alpha: opacities::GLASS_WINDOW_OPTIONS_ALPHA as f32,
                blur_px: chrome_metrics::GLASS_WINDOW_OPTIONS_BLUR_PX as f32,
                saturate: self.glass_saturate,
            },
        }
    }

    pub fn glass_mip_level(blur_px: f32, max_mip: u32) -> f32 {
        (blur_px / 4.0).log2().max(0.0).min(max_mip as f32)
    }
}

pub type ThemedRect = Rect;

#[cfg(test)]
mod tests {
    use super::{GlassTier, Theme};
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
    fn glass_panel_tier_matches_react_tokens() {
        let theme = Theme::light();
        let glass = theme.glass(GlassTier::Panel);
        let [r, g, b, _] = linear_to_rgba8(glass.tint.r, glass.tint.g, glass.tint.b, glass.tint.a);
        assert_eq!([r, g, b], [201, 200, 189]);
        assert!((glass.alpha - 0.58).abs() < f32::EPSILON);
        assert!((glass.blur_px - 40.0).abs() < f32::EPSILON);
        assert!((glass.saturate - 1.45).abs() < f32::EPSILON);
    }

    #[test]
    fn glass_menu_tier_uses_temporary_tint() {
        let theme = Theme::light();
        let glass = theme.glass(GlassTier::Menu);
        let [r, g, b, _] = linear_to_rgba8(glass.tint.r, glass.tint.g, glass.tint.b, glass.tint.a);
        assert_eq!([r, g, b], [151, 155, 148]);
        assert!((glass.alpha - 0.36).abs() < f32::EPSILON);
        assert!((glass.blur_px - 24.0).abs() < f32::EPSILON);
    }

    #[test]
    fn glass_window_options_tier_matches_react_tokens() {
        let theme = Theme::light();
        let glass = theme.glass(GlassTier::WindowOptions);
        assert!((glass.alpha - 0.22).abs() < f32::EPSILON);
        assert!((glass.blur_px - 14.0).abs() < f32::EPSILON);
    }

    #[test]
    fn chrome_item_default_is_transparent() {
        use crate::chrome::chrome_item_bg;
        let theme = Theme::light();
        let bg = chrome_item_bg(&theme, false, false);
        assert_eq!(bg.a, 0.0);
    }
}
