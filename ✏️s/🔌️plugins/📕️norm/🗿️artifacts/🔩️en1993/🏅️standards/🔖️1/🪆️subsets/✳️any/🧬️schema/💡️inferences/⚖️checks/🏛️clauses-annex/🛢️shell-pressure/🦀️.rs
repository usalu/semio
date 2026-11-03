//! 🛢️ EN 1993-1-6 cylinders under external pressure: membrane hoop stress, the ring stiffener with its effective shell strip, ring general instability and the classical von Mises collapse pressure (mm, N, MPa). See [EN 1993-1-6 Annex D](https://eurocodes.jrc.ec.europa.eu/).

/// 🛢️ Membrane hoop compression σ_θ = p r/t of a cylinder under uniform external pressure.
pub fn hoop_stress(p: f64, r: f64, t: f64) -> f64 {
    p * r / t
}

/// 🛢️ Effective shell width acting with a ring stiffener, l_e = 1.56 √(r t) (unverified clause locator).
pub fn effective_shell_width(r: f64, t: f64) -> f64 {
    1.56 * (r * t).sqrt()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingSection {
    pub area: f64,
    pub centroid: f64,
    pub inertia: f64,
    pub radius: f64,
}

/// 🛢️ Ring with its effective shell strip: area, centroid offset from the shell mid-surface, second moment about the combined centroid and centroid radius.
pub fn ring_section(r: f64, t: f64, a_r: f64, i_r: f64, e_r: f64) -> RingSection {
    let l_e = effective_shell_width(r, t);
    let a_s = l_e * t;
    let area = a_r + a_s;
    let centroid = a_r * e_r / area;
    let inertia = i_r + a_r * (e_r - centroid).powi(2) + l_e * t.powi(3) / 12.0 + a_s * centroid * centroid;
    RingSection { area, centroid, inertia, radius: r + centroid }
}

/// 🛢️ Elastic critical external pressure of the ring general instability p_cr = 3 E I/(R³ l_r) (Bresse).
pub fn ring_p_cr(e: f64, inertia: f64, radius: f64, spacing: f64) -> f64 {
    3.0 * e * inertia / (radius.powi(3) * spacing)
}

/// 🛢️ Second moment a ring needs so that p_cr reaches `factor` times the design pressure.
pub fn ring_inertia_required(e: f64, p: f64, factor: f64, radius: f64, spacing: f64) -> f64 {
    factor * p * radius.powi(3) * spacing / (3.0 * e)
}

/// 🛢️ Classical von Mises elastic collapse pressure of a cylinder of length `l` between stiff rings (independent cross-check of the LS3 buckling stress).
pub fn von_mises_p_cr(e: f64, nu: f64, t: f64, d: f64, l: f64) -> f64 {
    let denominator = (1.0 - nu * nu).powf(0.75) * (l / d - 0.45 * (t / d).sqrt()).max(1e-9);
    2.42 * e * (t / d).powf(2.5) / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hoop_stress_is_the_boiler_formula() {
        assert!((hoop_stress(0.2, 1500.0, 10.0) - 30.0).abs() < 1e-12);
    }

    #[test]
    fn ring_section_matches_the_hand_computation() {
        let s = ring_section(1000.0, 8.0, 1500.0, 2.0e6, 80.0);
        let l_e = 1.56 * (8000.0_f64).sqrt();
        let a_s = l_e * 8.0;
        assert!((s.area - (1500.0 + a_s)).abs() < 1e-9);
        let c = 1500.0 * 80.0 / (1500.0 + a_s);
        assert!((s.centroid - c).abs() < 1e-9);
        let i = 2.0e6 + 1500.0 * (80.0 - c).powi(2) + l_e * 512.0 / 12.0 + a_s * c * c;
        assert!((s.inertia - i).abs() < 1e-6 * i);
        assert!((s.radius - (1000.0 + c)).abs() < 1e-12);
        assert!((l_e - 139.5307).abs() < 1e-4 && (s.centroid - 45.86726).abs() < 1e-5 && (s.inertia - 6_101_881.9).abs() < 1.0);
        assert!((ring_p_cr(210000.0, s.inertia, s.radius, 1500.0) - 2.240182).abs() < 1e-5);
    }

    #[test]
    fn bresse_pressure_scales_with_inertia_and_inverse_cube_of_radius() {
        let p = ring_p_cr(210000.0, 5.0e6, 1000.0, 1500.0);
        assert!((p - 3.0 * 210000.0 * 5.0e6 / (1.0e9 * 1500.0)).abs() < 1e-12);
        assert!((ring_p_cr(210000.0, 5.0e6, 2000.0, 1500.0) * 8.0 - p).abs() < 1e-12);
        let i = ring_inertia_required(210000.0, 0.1, 3.0, 1000.0, 1500.0);
        assert!((ring_p_cr(210000.0, i, 1000.0, 1500.0) - 0.3).abs() < 1e-12);
    }

    #[test]
    fn von_mises_pressure_decreases_with_length() {
        let short = von_mises_p_cr(210000.0, 0.3, 10.0, 2000.0, 3000.0);
        let long = von_mises_p_cr(210000.0, 0.3, 10.0, 2000.0, 6000.0);
        assert!(short > long && long > 0.0);
    }
}
