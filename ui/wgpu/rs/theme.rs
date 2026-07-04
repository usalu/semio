//! 🎨 Theme colors and metrics for wgpu UI rendering.

use crate::geometry::Rect;

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
    pub font_size_body: f32,
    pub font_size_small: f32,
    pub border_radius: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: Rgba::new(0.07, 0.07, 0.08, 1.0),
            panel: Rgba::new(0.11, 0.11, 0.12, 1.0),
            panel_border: Rgba::new(0.18, 0.18, 0.20, 1.0),
            navbar: Rgba::new(0.09, 0.09, 0.10, 1.0),
            text: Rgba::new(0.92, 0.92, 0.94, 1.0),
            text_muted: Rgba::new(0.62, 0.62, 0.66, 1.0),
            accent: Rgba::new(0.35, 0.55, 0.95, 1.0),
            accent_hover: Rgba::new(0.42, 0.62, 1.0, 1.0),
            button: Rgba::new(0.16, 0.16, 0.18, 1.0),
            button_hover: Rgba::new(0.22, 0.22, 0.25, 1.0),
            input_bg: Rgba::new(0.05, 0.05, 0.06, 1.0),
            separator: Rgba::new(0.20, 0.20, 0.22, 1.0),
            selected: Rgba::new(0.25, 0.40, 0.75, 0.35),
            canvas_clear: Rgba::new(0.05, 0.05, 0.06, 1.0),
            gap_standard: 8.0,
            padding_standard: 12.0,
            navbar_height: 40.0,
            panel_header_height: 32.0,
            control_height: 28.0,
            font_size_body: 13.0,
            font_size_small: 11.0,
            border_radius: 6.0,
        }
    }
}

pub type ThemedRect = Rect;
