//! 🌉️ EN 1993-2 orthotropic steel deck: minimum thicknesses and the deck plate strip under a wheel patch, clamped over the trough walls (mm, N, MPa). See [EN 1993-2 §9.4](https://eurocodes.jrc.ec.europa.eu/).

/// 🌉️ Minimum deck plate thickness: 14 mm, or 12 mm below a surfacing of at least 70 mm (unverified clause locator).
pub fn min_deck_thickness(surfacing: f64) -> f64 {
    if surfacing >= 70.0 { 12.0 } else { 14.0 }
}

/// 🌉️ Minimum trough wall thickness of closed stiffeners (unverified clause locator).
pub const MIN_TROUGH_THICKNESS: f64 = 6.0;

/// 🌉️ Side of the wheel patch at the deck plate mid-surface: contact side + 2 t_s (45° through the surfacing) + t_d.
pub fn patch_side(patch: f64, surfacing: f64, deck: f64) -> f64 {
    patch + 2.0 * surfacing + deck
}

/// 🌉️ Fixed-end moment of a span `l` clamped at both ends under a central uniform line load `w` of length `c`.
pub fn fixed_end_moment(w: f64, l: f64, c: f64) -> f64 {
    let c = c.min(l);
    let (x1, x2) = ((l - c) / 2.0, (l + c) / 2.0);
    let primitive = |x: f64| l * l * x * x / 2.0 - 2.0 * l * x.powi(3) / 3.0 + x.powi(4) / 4.0;
    w / (l * l) * (primitive(x2) - primitive(x1))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeckStrip {
    pub m_hogging: f64,
    pub m_sagging: f64,
    pub sigma_hogging: f64,
    pub sigma_sagging: f64,
}

/// 🌉️ Strip of unit width clamped over the clear span `l` between the trough walls under the wheel force `q` spread over a square patch of side `c`; stresses σ = 6 M/t_d² (no longitudinal load spreading, no trough flexibility).
pub fn deck_strip(q: f64, c: f64, l: f64, t: f64) -> DeckStrip {
    let w = q / (c * c);
    let loaded = c.min(l);
    let m_end = fixed_end_moment(w, l, loaded);
    let m_free = w * loaded * (l / 4.0 - loaded / 8.0);
    let m_sag = m_free - m_end;
    DeckStrip { m_hogging: m_end, m_sagging: m_sag, sigma_hogging: 6.0 * m_end / (t * t), sigma_sagging: 6.0 * m_sag / (t * t) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_thickness_drops_below_a_thick_surfacing() {
        assert_eq!(min_deck_thickness(70.0), 12.0);
        assert_eq!(min_deck_thickness(40.0), 14.0);
    }

    #[test]
    fn full_span_load_gives_the_textbook_moments() {
        let (w, l) = (2.0, 300.0);
        assert!((fixed_end_moment(w, l, l) - w * l * l / 12.0).abs() < 1e-9);
        let s = deck_strip(w * l * l, l, l, 14.0);
        assert!((s.m_sagging - w * l * l / 24.0).abs() < 1e-9);
        assert!((s.sigma_hogging - 6.0 * w * l * l / 12.0 / 196.0).abs() < 1e-9);
    }

    #[test]
    fn narrow_patch_matches_the_point_load_limit() {
        let (l, w, c) = (300.0, 1.0, 1e-3);
        assert!((fixed_end_moment(w, l, c) / (w * c) - l / 8.0).abs() < 1e-6 * l);
    }

    #[test]
    fn wheel_on_a_clear_span_of_300_mm_gives_the_hand_stresses() {
        let s = deck_strip(60000.0, 554.0, 300.0, 14.0);
        assert!((s.m_hogging - 1466.1992).abs() < 1e-3);
        assert!((s.m_sagging - 733.0996).abs() < 1e-3);
        assert!((s.sigma_hogging - 44.8836).abs() < 1e-3);
        assert!((s.sigma_sagging - 22.4418).abs() < 1e-3);
    }

    #[test]
    fn patch_spreads_through_surfacing_and_plate() {
        assert!((patch_side(400.0, 70.0, 14.0) - 554.0).abs() < 1e-12);
    }
}
