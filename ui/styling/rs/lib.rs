//! @emoji 🎨 Framework-neutral styling tokens generated from `ui/styling/tokens.json`.

#[allow(clippy::excessive_precision, reason = "🎨 float literals mirror ui/styling/tokens.json verbatim; truncating them by hand would drift from the source data on the next regeneration")]
#[path = "generated.rs"]
mod generated;

pub use generated::*;

// #region 🔖Color
pub mod color {
    /// @emoji 🌈 Converts sRGB8888 bytes to linear-sRGB `f32` components for GPU paints.
    pub fn rgba8_to_linear(r: u8, g: u8, b: u8, a: u8) -> [f32; 4] {
        fn ch(c: u8) -> f32 {
            let x = f64::from(c) / 255.0;
            let lin = if x <= 0.04045 { x / 12.92 } else { ((x + 0.055) / 1.055).powf(2.4) };
            lin as f32
        }
        [ch(r), ch(g), ch(b), f32::from(a) / 255.0]
    }

    /// @emoji 🌈 Converts linear-sRGB `f32` components to sRGB8888 bytes.
    pub fn linear_to_rgba8(lr: f32, lg: f32, lb: f32, la: f32) -> [u8; 4] {
        fn ch(l: f32) -> u8 {
            let x = l as f64;
            let s = if x <= 0.0031308 { x * 12.92 } else { 1.055 * x.powf(1.0 / 2.4) - 0.055 };
            (s * 255.0).round().clamp(0.0, 255.0) as u8
        }
        [ch(lr), ch(lg), ch(lb), (f64::from(la) * 255.0).round().clamp(0.0, 255.0) as u8]
    }
}
// #endregion 🔖Color

// #region 🔖Appearance
pub mod appearance {
    use super::generated::{BoardPalette, CanvasPalette, ChromePalette, MapPalette, BOARD_DARK, BOARD_LIGHT, CANVAS_DARK, CANVAS_LIGHT, CHROME_DARK, CHROME_LIGHT, MAP_DARK, MAP_LIGHT};

    /// @emoji 🎨 Active appearance (light/dark) for canvas hosts.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AppearanceName {
        Light,
        Dark,
    }

    impl AppearanceName {
        pub fn board(self) -> &'static BoardPalette {
            match self {
                Self::Light => &BOARD_LIGHT,
                Self::Dark => &BOARD_DARK,
            }
        }

        pub fn map(self) -> &'static MapPalette {
            match self {
                Self::Light => &MAP_LIGHT,
                Self::Dark => &MAP_DARK,
            }
        }

        pub fn canvas(self) -> &'static CanvasPalette {
            match self {
                Self::Light => &CANVAS_LIGHT,
                Self::Dark => &CANVAS_DARK,
            }
        }

        pub fn chrome(self) -> &'static ChromePalette {
            match self {
                Self::Light => &CHROME_LIGHT,
                Self::Dark => &CHROME_DARK,
            }
        }

        pub fn parse(s: &str) -> Self {
            if s.eq_ignore_ascii_case("dark") {
                Self::Dark
            } else {
                Self::Light
            }
        }
    }
}
// #endregion 🔖Appearance

#[cfg(test)]
mod tests {
    use super::appearance::AppearanceName;
    use super::{strokes, BOARD_LIGHT};

    #[test]
    fn board_light_raster_clear_is_opaque() {
        const { assert!(BOARD_LIGHT.raster_clear[3] > 0.9) };
    }

    #[test]
    fn stroke_widths_are_positive() {
        const { assert!(strokes::EDGE_BASE > 0.0) };
        const { assert!(strokes::WIRE_HIGHLIGHT > 0.0) };
    }

    #[test]
    fn appearance_name_parse() {
        assert_eq!(AppearanceName::parse("dark"), AppearanceName::Dark);
        assert_eq!(AppearanceName::parse("light"), AppearanceName::Light);
    }

    #[test]
    fn light_and_dark_palettes_differ() {
        use super::{BOARD_DARK, BOARD_LIGHT, CHROME_DARK, CHROME_LIGHT, MAP_DARK, MAP_LIGHT};
        assert_ne!(BOARD_LIGHT.raster_clear, BOARD_DARK.raster_clear);
        assert_ne!(MAP_LIGHT.surface_clear, MAP_DARK.surface_clear);
        assert_ne!(CHROME_LIGHT.canvas, CHROME_DARK.canvas);
    }

    #[test]
    fn grid_stroke_widths_match_tokens_json() {
        assert_eq!(strokes::GRID_LARGE, 1.0);
        assert_eq!(strokes::GRID_MEDIUM, 0.72);
        assert_eq!(strokes::GRID_SMALL, 0.48);
        assert_eq!(strokes::GRID_MICRO, 0.32);
    }
}
