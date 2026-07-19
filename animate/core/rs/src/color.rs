//! 🎨 RGBA colors, named palette, and gradient interpolation.

use serde::{Deserialize, Serialize};

/// 🌈 Linear RGBA color with premultiplication left to the renderer.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    pub const ORANGE: Self = Self::rgb(1.0, 0.5, 0.0);
    pub const PURPLE: Self = Self::rgb(0.5, 0.0, 0.5);
    pub const TEAL: Self = Self::rgb(0.0, 0.5, 0.5);
    pub const GRAY: Self = Self::rgb(0.5, 0.5, 0.5);
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn hex(hex: &str) -> Self {
        let s = hex.trim_start_matches('#');
        let (r, g, b, a) = match s.len() {
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
                (r, g, b, a)
            }
            _ => (0, 0, 0, 255),
        };
        Self::rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0)
    }

    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.a = alpha;
        self
    }

    pub fn lerp(self, other: Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self { r: self.r + (other.r - self.r) * t, g: self.g + (other.g - self.g) * t, b: self.b + (other.b - self.b) * t, a: self.a + (other.a - self.a) * t }
    }

    pub fn to_array(self) -> [f64; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// 🌅 Multi-stop color gradient.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gradient {
    pub stops: Vec<(f64, Color)>,
}

impl Gradient {
    pub fn new(stops: Vec<(f64, Color)>) -> Self {
        let mut stops = stops;
        stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        Self { stops }
    }

    pub fn sample(&self, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        if self.stops.is_empty() {
            return Color::WHITE;
        }
        if self.stops.len() == 1 {
            return self.stops[0].1;
        }
        if t <= self.stops[0].0 {
            return self.stops[0].1;
        }
        if t >= self.stops[self.stops.len() - 1].0 {
            return self.stops[self.stops.len() - 1].1;
        }
        for pair in self.stops.windows(2) {
            let (t0, c0) = pair[0];
            let (t1, c1) = pair[1];
            if t >= t0 && t <= t1 {
                let u = if (t1 - t0).abs() < 1e-9 { 0.0 } else { (t - t0) / (t1 - t0) };
                return c0.lerp(c1, u);
            }
        }
        self.stops[0].1
    }
}

pub fn named_color(name: &str) -> Color {
    match name.to_ascii_lowercase().as_str() {
        "white" => Color::WHITE,
        "black" => Color::BLACK,
        "red" => Color::RED,
        "green" => Color::GREEN,
        "blue" => Color::BLUE,
        "yellow" => Color::YELLOW,
        "orange" => Color::ORANGE,
        "purple" => Color::PURPLE,
        "teal" => Color::TEAL,
        "gray" | "grey" => Color::GRAY,
        "manim_blue" | "semio_blue" => Color::hex("#58C4DD"),
        "manim_green" | "semio_green" => Color::hex("#83C167"),
        "manim_red" | "semio_red" => Color::hex("#FC6255"),
        "manim_yellow" | "semio_yellow" => Color::hex("#FFFF00"),
        other => Color::hex(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_midpoint_is_average() {
        let c = Color::BLACK.lerp(Color::WHITE, 0.5);
        assert!((c.r - 0.5).abs() < 1e-9);
    }

    #[test]
    fn gradient_samples_stops() {
        let g = Gradient::new(vec![(0.0, Color::RED), (1.0, Color::BLUE)]);
        let mid = g.sample(0.5);
        assert!(mid.r > 0.0 && mid.b > 0.0);
    }
}
