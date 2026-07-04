//! 🎨 Theme colors for the wgpu shell chrome.

#[derive(Clone, Copy, Debug)]
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

    pub const BACKGROUND: Self = Self::new(0.07, 0.07, 0.08, 1.0);
    pub const PANEL: Self = Self::new(0.11, 0.11, 0.12, 1.0);
    pub const PANEL_BORDER: Self = Self::new(0.18, 0.18, 0.20, 1.0);
    pub const NAVBAR: Self = Self::new(0.09, 0.09, 0.10, 1.0);
    pub const TEXT: Self = Self::new(0.92, 0.92, 0.94, 1.0);
    pub const TEXT_MUTED: Self = Self::new(0.62, 0.62, 0.66, 1.0);
    pub const ACCENT: Self = Self::new(0.35, 0.55, 0.95, 1.0);
    pub const ACCENT_HOVER: Self = Self::new(0.42, 0.62, 1.0, 1.0);
    pub const BUTTON: Self = Self::new(0.16, 0.16, 0.18, 1.0);
    pub const BUTTON_HOVER: Self = Self::new(0.22, 0.22, 0.25, 1.0);
    pub const INPUT_BG: Self = Self::new(0.05, 0.05, 0.06, 1.0);
    pub const SEPARATOR: Self = Self::new(0.20, 0.20, 0.22, 1.0);
    pub const SELECTED: Self = Self::new(0.25, 0.40, 0.75, 0.35);
    pub const CANVAS_CLEAR: Self = Self::new(0.05, 0.05, 0.06, 1.0);
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
    }

    pub fn inset(&self, amount: f32) -> Self {
        Self {
            x: self.x + amount,
            y: self.y + amount,
            w: (self.w - amount * 2.0).max(0.0),
            h: (self.h - amount * 2.0).max(0.0),
        }
    }
}

pub const GAP_STANDARD: f32 = 8.0;
pub const PADDING_STANDARD: f32 = 12.0;
pub const NAVBAR_HEIGHT: f32 = 40.0;
pub const PANEL_HEADER_HEIGHT: f32 = 32.0;
pub const CONTROL_HEIGHT: f32 = 28.0;
pub const FONT_SIZE_BODY: f32 = 13.0;
pub const FONT_SIZE_SMALL: f32 = 11.0;
pub const BORDER_RADIUS: f32 = 6.0;
