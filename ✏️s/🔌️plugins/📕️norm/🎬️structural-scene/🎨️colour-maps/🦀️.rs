//! 🎨️ Colour maps: turn utilisation ratios and result scalars into colour plus redundant non-colour cues (glyph, hatch, text) per theme variant; the scene itself never carries colour.
//!
//! 🔗️ See `🎨️appearance` for `ColourMapRef`, `Hatch` and `CheckStatus`, and `visual-editor-architecture.md` §3.4 and §8.2.

use crate::{CheckStatus, Hatch, LocalizedCopy, QuantityKind, Rgb};

#[path = "👁️metrics/🦀️.rs"]
pub mod metrics;

#[path = "🌈️ramps/🦀️.rs"]
pub mod ramps;
pub use ramps::{ramp, MapKind, Ramp, RampId, ALL_RAMPS};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

/// 🌗️ Theme variant a colour map is resolved for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Light,
    Dark,
    HighContrast,
}

pub const ALL_VARIANTS: [Variant; 3] = [Variant::Light, Variant::Dark, Variant::HighContrast];

impl Variant {
    pub fn wire(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::HighContrast => "highContrast",
        }
    }

    pub fn parse(id: &str) -> Option<Self> {
        ALL_VARIANTS.into_iter().find(|v| v.wire() == id)
    }

    pub fn background(self) -> [u8; 3] {
        match self {
            Self::Light => [0xff, 0xff, 0xff],
            Self::Dark => [0x12, 0x12, 0x12],
            Self::HighContrast => [0x00, 0x00, 0x00],
        }
    }

    pub fn foreground(self) -> [u8; 3] {
        match self {
            Self::Light => [0x1a, 0x1a, 0x1a],
            Self::Dark | Self::HighContrast => [0xff, 0xff, 0xff],
        }
    }

    pub fn contrast_floor(self) -> f64 {
        match self {
            Self::Light | Self::Dark => 3.0,
            Self::HighContrast => 7.0,
        }
    }
}

/// 🧪️ Physical result field a scalar overlay shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultField {
    Stress,
    Temperature,
    Pressure,
    Settlement,
    Airflow,
    HeatFlow,
}

impl ResultField {
    pub fn ramp(self) -> RampId {
        match self {
            Self::Stress | Self::Pressure => RampId::Diverging,
            Self::Temperature | Self::HeatFlow => RampId::Warm,
            Self::Settlement | Self::Airflow => RampId::Cool,
        }
    }

    /// 🧪️ The result field a scalar of `kind` is shown as; kinds without an overlay have none.
    pub fn of(kind: QuantityKind) -> Option<Self> {
        match kind {
            QuantityKind::Stress => Some(Self::Stress),
            QuantityKind::Temperature => Some(Self::Temperature),
            QuantityKind::Pressure => Some(Self::Pressure),
            QuantityKind::VentilationRate => Some(Self::Airflow),
            QuantityKind::HeatTransferCoefficient | QuantityKind::Power => Some(Self::HeatFlow),
            _ => None,
        }
    }
}

/// 🚦️ Everything that tells a status apart besides its colour.
#[derive(Clone, Debug, PartialEq)]
pub struct Cue {
    pub rgb: Rgb,
    pub hatch: Option<Hatch>,
    pub glyph: &'static str,
    pub label: LocalizedCopy,
    pub status: CheckStatus,
    pub verified: bool,
}

pub const WARNING_RATIO: f64 = 0.9;
pub const FAILURE_RATIO: f64 = 1.0;
pub const DEFAULT_DOMAIN_MAX: f64 = 1.2;

fn rgb(c: [u8; 3]) -> Rgb {
    Rgb { r: c[0], g: c[1], b: c[2] }
}

/// 🌡️ Utilisation colour: safe ramp over 0..1, failure ramp over 1..`domain_max`, hard break at 1.
pub fn utilisation_rgb(variant: Variant, ratio: f64, domain_max: f64) -> [u8; 3] {
    if ratio.is_nan() {
        return variant_neutral(variant);
    }
    if ratio <= FAILURE_RATIO {
        return ramp(RampId::Cool, variant).sample(ratio.max(0.0));
    }
    ramp(RampId::Failure, variant).sample((ratio - FAILURE_RATIO) / (domain_max - FAILURE_RATIO).max(f64::EPSILON))
}

fn variant_neutral(variant: Variant) -> [u8; 3] {
    match variant {
        Variant::Light => [0x76, 0x76, 0x76],
        Variant::Dark => [0x9e, 0x9e, 0x9e],
        Variant::HighContrast => [0xc8, 0xc8, 0xc8],
    }
}

/// 🚦️ Colour, hatch, glyph and bilingual word for one utilisation; unverified bases get dots, failures diagonal hatching.
pub fn utilisation_cue(variant: Variant, ratio: f64, status: CheckStatus, verified: bool, domain_max: f64) -> Cue {
    let verified = verified && !ratio.is_nan();
    let failed = status == CheckStatus::Fail || (status != CheckStatus::NotApplicable && ratio > FAILURE_RATIO);
    let (status, glyph, hatch, en, de) = if status == CheckStatus::NotApplicable {
        (CheckStatus::NotApplicable, "–", None, "not applicable", "nicht anwendbar")
    } else if failed {
        (CheckStatus::Fail, "✗", Some(Hatch::Diagonal), "fails", "nicht erfüllt")
    } else if status == CheckStatus::Warning || ratio >= WARNING_RATIO {
        (CheckStatus::Warning, "!", Some(Hatch::Horizontal), "close to limit", "nahe am Grenzwert")
    } else {
        (CheckStatus::Pass, "✓", None, "passes", "erfüllt")
    };
    let (hatch, glyph, en, de) = if verified || status == CheckStatus::NotApplicable { (hatch, glyph, en.to_owned(), de.to_owned()) } else {
        let unverified = match glyph { "✗" => "✗?", "!" => "!?", _ => "✓?" };
        (Some(Hatch::Dots), unverified, format!("{en}, unverified basis"), format!("{de}, ungeprüfte Grundlage"))
    };
    let colour = if status == CheckStatus::NotApplicable { variant_neutral(variant) } else { utilisation_rgb(variant, ratio, domain_max) };
    Cue { rgb: rgb(colour), hatch, glyph, label: LocalizedCopy::new(en, de), status, verified }
}

/// 🌈️ Colour of a result scalar over `lo..hi`; stresses and pressures are centred on zero, so `lo` and `hi` are made symmetric.
pub fn scalar_rgb(field: ResultField, variant: Variant, value: f64, lo: f64, hi: f64) -> [u8; 3] {
    let ramp = ramp(field.ramp(), variant);
    let (lo, hi) = if ramp.kind == MapKind::Diverging { let m = lo.abs().max(hi.abs()); (-m, m) } else { (lo, hi) };
    ramp.sample((value - lo) / (hi - lo).max(f64::EPSILON))
}

/// 🏷️ Legend stops `(value, colour)` of a result field, ready for a framework colour ramp.
pub fn legend(field: ResultField, variant: Variant, lo: f64, hi: f64, steps: usize) -> Vec<(f64, [u8; 3])> {
    let n = steps.max(2);
    let (lo, hi) = if field.ramp() == RampId::Diverging { let m = lo.abs().max(hi.abs()); (-m, m) } else { (lo, hi) };
    (0..n).map(|i| { let v = lo + (hi - lo) * i as f64 / (n - 1) as f64; (v, scalar_rgb(field, variant, v, lo, hi)) }).collect()
}
