//! @emoji 🎨 Default Vello canvas paint helpers from centralized styling tokens.

use crate::vello::peniko::Color;
use ui_styling::{theme::ThemeName, CANVAS_LIGHT};

/// @emoji 🌈 Maps a linear-sRGB token color to `peniko::Color`.
pub fn linear_color(rgba: [f32; 4]) -> Color {
    Color::new(rgba)
}

/// @emoji 🎨 Shared default clear color for graph board canvases.
pub fn default_raster_clear() -> Color {
    linear_color(CANVAS_LIGHT.raster_clear)
}

/// @emoji 🎨 Default themed icon foreground paint.
pub fn default_icon_fg() -> Color {
    linear_color(CANVAS_LIGHT.icon_fg)
}

/// @emoji 🎨 Default themed icon background paint.
pub fn default_icon_bg() -> Color {
    linear_color(CANVAS_LIGHT.icon_bg)
}

/// @emoji 🎨 Resolves canvas paints for a theme name.
pub fn canvas_clear_for(theme: ThemeName) -> Color {
    linear_color(theme.canvas().raster_clear)
}
