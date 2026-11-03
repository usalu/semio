//! 🌈️ Ramps: CVD-safe sequential, diverging and failure ramps per theme variant, linearly interpolated in sRGB.

use super::Variant;

pub type Stop = (f64, [u8; 3]);

/// 🧭️ Shape of a ramp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapKind {
    Sequential,
    Diverging,
    Failure,
}

/// 🗝️ Named ramp; ids are the wire ids referenced by `ColourMapRef`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RampId {
    Cool,
    Warm,
    Diverging,
    Failure,
}

pub const ALL_RAMPS: [RampId; 4] = [RampId::Cool, RampId::Warm, RampId::Diverging, RampId::Failure];

/// 🎚️ One resolved ramp.
#[derive(Clone, Copy, Debug)]
pub struct Ramp {
    pub id: RampId,
    pub variant: Variant,
    pub kind: MapKind,
    pub stops: &'static [Stop],
}

impl RampId {
    pub fn wire(self) -> &'static str {
        match self {
            Self::Cool => "cool",
            Self::Warm => "warm",
            Self::Diverging => "diverging",
            Self::Failure => "failure",
        }
    }

    pub fn parse(id: &str) -> Option<Self> {
        ALL_RAMPS.into_iter().find(|r| r.wire() == id)
    }

    fn kind(self) -> MapKind {
        match self {
            Self::Cool | Self::Warm => MapKind::Sequential,
            Self::Diverging => MapKind::Diverging,
            Self::Failure => MapKind::Failure,
        }
    }
}

const COOL_LIGHT: &[Stop] = &[(0.0, [0x0b, 0x1f, 0x4d]), (0.25, [0x1f, 0x5f, 0xa8]), (0.5, [0x3d, 0x8f, 0x8a]), (0.75, [0x8a, 0x8a, 0x1f]), (1.0, [0xc0, 0x8a, 0x00])];
const COOL_DARK: &[Stop] = &[(0.0, [0x5a, 0x8f, 0xe0]), (0.25, [0x4f, 0xa3, 0xb8]), (0.5, [0x9a, 0xb0, 0x7a]), (0.75, [0xd0, 0xc2, 0x4e]), (1.0, [0xff, 0xe4, 0x5c])];
const COOL_HC: &[Stop] = &[(0.0, [0x5a, 0xa0, 0xff]), (0.25, [0x4f, 0xc0, 0xd8]), (0.5, [0xa0, 0xd8, 0x90]), (0.75, [0xe0, 0xe0, 0x60]), (1.0, [0xff, 0xff, 0x80])];
const WARM_LIGHT: &[Stop] = &[(0.0, [0x3b, 0x0f, 0x70]), (0.33, [0x8c, 0x29, 0x81]), (0.66, [0xc4, 0x48, 0x2a]), (1.0, [0xb8, 0x74, 0x00])];
const WARM_DARK: &[Stop] = &[(0.0, [0x8f, 0x6a, 0xe0]), (0.33, [0xd0, 0x50, 0x8c]), (0.66, [0xf0, 0x8a, 0x3a]), (1.0, [0xff, 0xe0, 0x70])];
const WARM_HC: &[Stop] = &[(0.0, [0xa0, 0x80, 0xff]), (0.33, [0xff, 0x70, 0xb0]), (0.66, [0xff, 0xa0, 0x50]), (1.0, [0xff, 0xff, 0x90])];
const DIV_LIGHT: &[Stop] = &[(0.0, [0x21, 0x66, 0xac]), (0.25, [0x4a, 0x8f, 0xc4]), (0.5, [0xf2, 0xf2, 0xf2]), (0.75, [0xd9, 0x70, 0x3f]), (1.0, [0xb2, 0x18, 0x2b])];
const DIV_DARK: &[Stop] = &[(0.0, [0x6a, 0xa8, 0xff]), (0.25, [0x4a, 0x7f, 0xc0]), (0.5, [0x30, 0x30, 0x30]), (0.75, [0xc0, 0x6a, 0x4a]), (1.0, [0xff, 0x8a, 0x70])];
const DIV_HC: &[Stop] = &[(0.0, [0x70, 0xb0, 0xff]), (0.25, [0xa0, 0xd0, 0xff]), (0.5, [0xff, 0xff, 0xff]), (0.75, [0xff, 0xb0, 0x90]), (1.0, [0xff, 0x70, 0x60])];
const FAIL_LIGHT: &[Stop] = &[(0.0, [0xb3, 0x00, 0x1b]), (1.0, [0x5c, 0x00, 0x10])];
const FAIL_DARK: &[Stop] = &[(0.0, [0xff, 0x6b, 0x6b]), (1.0, [0xff, 0xb0, 0xb0])];
const FAIL_HC: &[Stop] = &[(0.0, [0xff, 0x70, 0x70]), (1.0, [0xff, 0xb8, 0xb8])];

/// 🎚️ The ramp of an id in a theme variant.
pub fn ramp(id: RampId, variant: Variant) -> Ramp {
    use Variant::{Dark, HighContrast, Light};
    let stops = match (id, variant) {
        (RampId::Cool, Light) => COOL_LIGHT,
        (RampId::Cool, Dark) => COOL_DARK,
        (RampId::Cool, HighContrast) => COOL_HC,
        (RampId::Warm, Light) => WARM_LIGHT,
        (RampId::Warm, Dark) => WARM_DARK,
        (RampId::Warm, HighContrast) => WARM_HC,
        (RampId::Diverging, Light) => DIV_LIGHT,
        (RampId::Diverging, Dark) => DIV_DARK,
        (RampId::Diverging, HighContrast) => DIV_HC,
        (RampId::Failure, Light) => FAIL_LIGHT,
        (RampId::Failure, Dark) => FAIL_DARK,
        (RampId::Failure, HighContrast) => FAIL_HC,
    };
    Ramp { id, variant, kind: id.kind(), stops }
}

impl Ramp {
    /// 🎨️ Colour at `t` in 0..1 (clamped; NaN samples the start).
    pub fn sample(&self, t: f64) -> [u8; 3] {
        let t = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) };
        let hit = self.stops.windows(2).find(|w| t <= w[1].0).unwrap_or(&self.stops[self.stops.len() - 2..]);
        let u = ((t - hit[0].0) / (hit[1].0 - hit[0].0)).clamp(0.0, 1.0);
        std::array::from_fn(|i| (f64::from(hit[0].1[i]) + (f64::from(hit[1].1[i]) - f64::from(hit[0].1[i])) * u + 0.5).floor() as u8)
    }

    /// 🎞️ `n` evenly spaced samples including both ends.
    pub fn samples(&self, n: usize) -> Vec<[u8; 3]> {
        (0..n).map(|i| self.sample(i as f64 / (n - 1).max(1) as f64)).collect()
    }
}
