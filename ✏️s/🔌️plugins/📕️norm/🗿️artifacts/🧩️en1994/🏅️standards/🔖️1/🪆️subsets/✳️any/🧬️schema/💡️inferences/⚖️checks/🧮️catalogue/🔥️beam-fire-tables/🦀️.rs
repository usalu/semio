//! 🔥 Tabulated resistance of a partially encased beam under standard fire, EN 1994-1-2:2005 Tables 4.1 and 4.2.

/// 📐 One geometry alternative from Table 4.1, with width in millimetres and dimensionless A_s/A_f by rating.
#[derive(Clone, Copy)]
pub struct BeamAlternative {
    pub load_level: f64,
    pub depth_factor: f64,
    pub rating_cells: [Option<(u16, f64)>; 5],
}

const fn beam_cell((width, ratio): (u16, f64)) -> Option<(u16, f64)> {
    if width == 0 { None } else { Some((width, ratio)) }
}

const fn alternative(load_level: f64, depth_factor: f64, cells: [(u16, f64); 5]) -> BeamAlternative {
    BeamAlternative {
        load_level,
        depth_factor,
        rating_cells: [beam_cell(cells[0]), beam_cell(cells[1]), beam_cell(cells[2]), beam_cell(cells[3]), beam_cell(cells[4])],
    }
}

/// 📘 Published minimum beam alternatives. Each row keeps its associated depth and reinforcement.
pub const BEAM_FIRE_ALTERNATIVES: [BeamAlternative; 11] = [
    alternative(0.30, 0.9, [(70, 0.0), (100, 0.0), (170, 0.0), (200, 0.0), (260, 0.0)]),
    alternative(0.30, 1.5, [(60, 0.0), (100, 0.0), (150, 0.0), (180, 0.0), (240, 0.0)]),
    alternative(0.30, 2.0, [(60, 0.0), (100, 0.0), (150, 0.0), (180, 0.0), (240, 0.0)]),
    alternative(0.50, 0.9, [(80, 0.0), (170, 0.0), (250, 0.4), (270, 0.5), (0, 0.0)]),
    alternative(0.50, 1.5, [(80, 0.0), (150, 0.0), (200, 0.2), (240, 0.3), (300, 0.5)]),
    alternative(0.50, 2.0, [(70, 0.0), (120, 0.0), (180, 0.2), (220, 0.3), (280, 0.3)]),
    alternative(0.50, 3.0, [(60, 0.0), (100, 0.0), (170, 0.2), (200, 0.3), (250, 0.3)]),
    alternative(0.70, 0.9, [(80, 0.0), (270, 0.4), (300, 0.6), (0, 0.0), (0, 0.0)]),
    alternative(0.70, 1.5, [(80, 0.0), (240, 0.3), (270, 0.4), (300, 0.6), (0, 0.0)]),
    alternative(0.70, 2.0, [(70, 0.0), (190, 0.3), (210, 0.4), (270, 0.5), (320, 1.0)]),
    alternative(0.70, 3.0, [(70, 0.0), (170, 0.2), (190, 0.4), (270, 0.5), (300, 0.8)]),
];

/// 📏 Table 4.2 with flange width and axis distances in millimetres, for R60, R90, R120 and R180.
pub const BEAM_FIRE_AXES_MM: &[(u16, [Option<(u16, u16)>; 4])] = &[
    (170, [Some((100, 45)), Some((120, 60)), None, None]),
    (200, [Some((80, 40)), Some((100, 55)), Some((120, 60)), None]),
    (250, [Some((60, 35)), Some((75, 50)), Some((90, 60)), Some((120, 60))]),
    (300, [Some((40, 25)), Some((50, 45)), Some((70, 60)), Some((90, 60))]),
];

/// 🔎 The next conservative listed load band of Table 4.1.
pub fn beam_load_level(load_ratio: f64) -> Option<f64> {
    if !load_ratio.is_finite() || load_ratio < 0.0 {
        return None;
    }
    [0.30, 0.50, 0.70].into_iter().find(|&level| load_ratio <= level)
}

/// 📐 Minimum reinforcement-axis distances in metres, with interpolation between published widths.
pub fn beam_reinforcement_axes(rating: u32, width_m: f64) -> Option<(f64, f64)> {
    let column = [60, 90, 120, 180].iter().position(|&class| class == rating)?;
    if !width_m.is_finite() || width_m < 0.17 {
        return None;
    }
    let actual = width_m.min(0.3);
    let upper = BEAM_FIRE_AXES_MM.iter().position(|(width, _)| actual <= f64::from(*width) / 1000.0)?;
    let (high, high_cells) = BEAM_FIRE_AXES_MM[upper];
    let (high_u1, high_u2) = high_cells[column]?;
    if actual == f64::from(high) / 1000.0 {
        return Some((f64::from(high_u1) / 1000.0, f64::from(high_u2) / 1000.0));
    }
    let (low, low_cells) = BEAM_FIRE_AXES_MM[upper.checked_sub(1)?];
    let (low_u1, low_u2) = low_cells[column]?;
    let fraction = (actual - f64::from(low) / 1000.0) / (f64::from(high - low) / 1000.0);
    let blend = |left: u16, right: u16| (f64::from(left) + fraction * (f64::from(right) - f64::from(left))) / 1000.0;
    Some((blend(low_u1, high_u1), blend(low_u2, high_u2)))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod reference;
