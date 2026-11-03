//! 🎨️ Colour of an entity: role palette, utilisation ramp and non-colour-redundant status colours; all values are linear 0..1 triples.

use super::config::{ColourRamp, Viewport3dWindowConfig};
use super::scene::{CheckStatus, ColourChannel, LoadKind, MaterialRole, SceneEntity, StyleState};

pub type Rgb = [f64; 3];

pub fn role_rgb(role: &MaterialRole) -> Rgb {
    match role {
        MaterialRole::Steel => [0.60, 0.64, 0.70],
        MaterialRole::Concrete => [0.72, 0.72, 0.70],
        MaterialRole::Rebar => [0.45, 0.30, 0.22],
        MaterialRole::Timber => [0.76, 0.58, 0.36],
        MaterialRole::Masonry => [0.72, 0.42, 0.33],
        MaterialRole::Soil { class } => match class.as_str() {
            "sand" => [0.85, 0.78, 0.55],
            "gravel" => [0.65, 0.65, 0.60],
            "rock" => [0.45, 0.45, 0.50],
            "clay" => [0.55, 0.45, 0.35],
            _ => [0.50, 0.40, 0.30],
        },
        MaterialRole::Aluminium => [0.78, 0.80, 0.84],
        MaterialRole::Glass => [0.60, 0.80, 0.90],
        MaterialRole::Weld => [0.90, 0.60, 0.20],
        MaterialRole::Bolt => [0.35, 0.37, 0.40],
        MaterialRole::Load { kind } => load_rgb(*kind),
        MaterialRole::Support => [0.30, 0.30, 0.30],
        MaterialRole::Neutral => [0.70, 0.70, 0.70],
    }
}

pub fn load_rgb(kind: LoadKind) -> Rgb {
    match kind {
        LoadKind::Permanent => [0.25, 0.25, 0.70],
        LoadKind::Variable => [0.20, 0.65, 0.30],
        LoadKind::Accidental => [0.90, 0.20, 0.20],
        LoadKind::Seismic => [0.60, 0.20, 0.70],
        LoadKind::Wind => [0.20, 0.70, 0.80],
        LoadKind::Snow => [0.75, 0.85, 0.95],
        LoadKind::Thermal => [0.95, 0.50, 0.10],
        LoadKind::Prestress => [0.85, 0.80, 0.10],
        LoadKind::Other => [0.50, 0.50, 0.50],
    }
}

pub fn status_rgb(status: CheckStatus) -> Rgb {
    match status {
        CheckStatus::Pass => [0.20, 0.65, 0.35],
        CheckStatus::Warning => [0.95, 0.70, 0.15],
        CheckStatus::Fail => [0.80, 0.15, 0.15],
        CheckStatus::NotApplicable => [0.60, 0.60, 0.60],
    }
}

/// 🌡️ Ramp colour of a utilisation ratio over `0..domain_max`, quantised to 1/32 of the domain so equal buckets share one mesh.
pub fn ramp_rgb(ramp: ColourRamp, ratio: f64, domain_max: f64) -> Rgb {
    let t = ((ratio / domain_max).clamp(0.0, 1.0) * 32.0).round() / 32.0;
    ramp.framework().sample(t).map(|c| f64::from(c) / 255.0)
}

fn mix(a: Rgb, b: Rgb, t: f64) -> Rgb {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// 🌈️ Ramp colour of a field value over `lo..hi`, quantised to 1/32 of the domain.
pub fn field_rgb(ramp: ColourRamp, value: f64, lo: f64, hi: f64) -> Rgb {
    let t = (((value - lo) / (hi - lo).max(1e-300)).clamp(0.0, 1.0) * 32.0).round() / 32.0;
    ramp.framework().sample(t).map(|c| f64::from(c) / 255.0)
}

/// 🔆️ Style-state tint of a colour: dimmed and stale towards grey, error towards red.
pub fn styled(entity: &SceneEntity, base: Rgb) -> Rgb {
    let stale = entity.utilisation.as_ref().is_some_and(|u| u.stale);
    match (entity.appearance.style, stale) {
        (StyleState::Error, _) => mix(base, [0.8, 0.1, 0.1], 0.6),
        (StyleState::Dimmed, _) => mix(base, [0.5, 0.5, 0.5], 0.6),
        (StyleState::Stale, _) | (_, true) => mix(base, [0.6, 0.6, 0.6], 0.5),
        _ => base,
    }
}

/// 🖍️ Flat base colour of an entity under the window configuration; entities with scalar fields are baked per vertex by the lowering and fall back to their role colour here.
pub fn entity_rgb(entity: &SceneEntity, config: &Viewport3dWindowConfig, role: &MaterialRole) -> Rgb {
    let base = match (&entity.appearance.channel, &entity.utilisation) {
        (ColourChannel::Utilisation, Some(u)) => ramp_rgb(config.ramp, u.ratio, config.domain_max),
        (ColourChannel::Status, Some(u)) => status_rgb(u.status),
        (ColourChannel::Custom { rgb }, _) => [f64::from(rgb.r) / 255.0, f64::from(rgb.g) / 255.0, f64::from(rgb.b) / 255.0],
        _ => role_rgb(role),
    };
    styled(entity, base)
}

/// 🖋️ Ink colour of hatch lines: the base colour at 40 percent brightness, so lines stay visible on every ramp and role colour.
pub fn ink_rgb(base: Rgb) -> Rgb {
    base.map(|c| c * 0.4)
}

pub fn rgb_hex(rgb: Rgb) -> String {
    format!("{:02x}{:02x}{:02x}", (rgb[0] * 255.0).round() as u8, (rgb[1] * 255.0).round() as u8, (rgb[2] * 255.0).round() as u8)
}
