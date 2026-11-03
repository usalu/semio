//! 📚️ EN 1993-1-5 Annex A critical plate buckling stress of longitudinally stiffened plates in mm, N, MPa: smeared equal stiffeners (A.1, three or more) and one stiffener (A.2.2.1). The formula structure is written from memory and is UNVERIFIED (registry rows `ndp.en1993.v-376.a1` and `ndp.en1993.v-376.a2`); the chart values of the standard are not encoded. See [EN 1993-1-5 Annex A](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

/// 📚️ Plate width, thickness and the equal longitudinal stiffeners: `area` and `inertia` per stiffener (own centroid) and `offset` the distance of the stiffener centroid from the plate mid-surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stiffened {
    pub b: f64,
    pub t: f64,
    pub count: usize,
    pub area: f64,
    pub inertia: f64,
    pub offset: f64,
}

/// 🔢️ Number of equally spaced stiffeners for a spacing `s` across the width `b` (zero when the spacing is not smaller than the width).
pub fn count(b: f64, spacing: f64) -> usize {
    if spacing > 1e-9 && spacing < b { ((b / spacing).round() as usize).saturating_sub(1) } else { 0 }
}

/// 📏️ Sub-panel width of `count` equally spaced stiffeners.
pub fn pitch(b: f64, count: usize) -> f64 {
    b / (count as f64 + 1.0)
}

/// 📚️ Elastic critical stress of the plate σ_E = π² E t² / (12 (1 − ν²) b²).
pub fn sigma_e(e: f64, nu: f64, t: f64, b: f64) -> f64 {
    PI * PI * e * t * t / (12.0 * (1.0 - nu * nu) * b * b)
}

/// 📚️ Plate bending inertia I_p = b t³ / 10.92.
pub fn i_p(b: f64, t: f64) -> f64 {
    b * t.powi(3) / 10.92
}

impl Stiffened {
    /// 📚️ Gross area ratio δ = ΣA_sl / (b t).
    pub fn delta(&self) -> f64 {
        self.count as f64 * self.area / (self.b * self.t)
    }

    /// 📚️ Second moment of the whole stiffened plate (plate plus all stiffeners) about its own neutral axis; for an unstiffened plate b t³ / 12.
    pub fn whole_inertia(&self) -> f64 {
        let (a_p, a_s) = (self.b * self.t, self.count as f64 * self.area);
        let shift = a_s * self.offset / (a_p + a_s);
        self.b * self.t.powi(3) / 12.0 + a_p * shift * shift + self.count as f64 * self.inertia + a_s * (self.offset - shift).powi(2)
    }

    /// 📚️ Stiffness ratio γ = I_sl / I_p of the whole stiffened plate.
    pub fn gamma(&self) -> f64 {
        self.whole_inertia() / i_p(self.b, self.t)
    }
}

/// 📚️ Result of the smeared-stiffener formula A.1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Smeared {
    pub alpha: f64,
    pub gamma: f64,
    pub delta: f64,
    pub k_sigma_p: f64,
    pub sigma_e: f64,
    pub sigma_cr_p: f64,
}

/// 📚️ A.1: k_σ,p = 2 ((1 + α²)² + γ − 1) / (α² (ψ + 1) (1 + δ)) for α ≤ 4 √γ, otherwise 4 (1 + √γ) / ((ψ + 1) (1 + δ)); σ_cr,p = k_σ,p σ_E. `psi` is the edge stress ratio (1 for uniform compression).
pub fn smeared(s: &Stiffened, a: f64, psi: f64, e: f64, nu: f64) -> Smeared {
    let (alpha, gamma, delta) = (a / s.b, s.gamma(), s.delta());
    let k_sigma_p = if alpha <= 4.0 * gamma.sqrt() {
        2.0 * ((1.0 + alpha * alpha).powi(2) + gamma - 1.0) / (alpha * alpha * (psi + 1.0) * (1.0 + delta))
    } else {
        4.0 * (1.0 + gamma.sqrt()) / ((psi + 1.0) * (1.0 + delta))
    };
    let sigma_e = sigma_e(e, nu, s.t, s.b);
    Smeared { alpha, gamma, delta, k_sigma_p, sigma_e, sigma_cr_p: k_sigma_p * sigma_e }
}

/// 📚️ Result of the single-stiffener formula A.2.2.1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Single {
    pub a_c: f64,
    pub sigma_cr_sl: f64,
    pub sigma_cr_sub: f64,
    pub sigma_cr_p: f64,
    pub k_sigma_p: f64,
    pub sigma_e: f64,
}

/// 📚️ A.2.2.1: a_c = 4.33 (I_sl b₁² b₂² / (t³ b))^¼; σ_cr,sl = 1.05 E √(I_sl t³ b) / (A_sl b₁ b₂) for a ≥ a_c, else π² E I_sl / (A_sl a²) + E t³ b a² / (4 π² (1 − ν²) A_sl b₁² b₂²). The governing stress is the smaller of σ_cr,sl and the local buckling of the wider sub-panel 4 σ_E(b_sub) (an engineering combination, unverified). `b1` is the stiffener distance from one edge.
pub fn single(s: &Stiffened, a: f64, b1: f64, e: f64, nu: f64) -> Single {
    let (b, t, i, area) = (s.b, s.t, s.inertia, s.area);
    let b1 = if b1 > 1e-9 && b1 < b { b1 } else { b / 2.0 };
    let b2 = b - b1;
    let a_c = 4.33 * (i * b1 * b1 * b2 * b2 / (t.powi(3) * b)).powf(0.25);
    let sigma_cr_sl = if a >= a_c {
        1.05 * e * (i * t.powi(3) * b).sqrt() / (area * b1 * b2)
    } else {
        PI * PI * e * i / (area * a * a) + e * t.powi(3) * b * a * a / (4.0 * PI * PI * (1.0 - nu * nu) * area * b1 * b1 * b2 * b2)
    };
    let sigma_cr_sub = 4.0 * sigma_e(e, nu, t, b1.max(b2));
    let sigma_cr_p = sigma_cr_sl.min(sigma_cr_sub);
    let sigma_e = sigma_e(e, nu, t, b);
    Single { a_c, sigma_cr_sl, sigma_cr_sub, sigma_cr_p, k_sigma_p: sigma_cr_p / sigma_e, sigma_e }
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: Stiffened = Stiffened { b: 1500.0, t: 10.0, count: 3, area: 1000.0, inertia: 1000.0 * 100.0 * 100.0 / 12.0, offset: 55.0 };

    #[test]
    fn counting_follows_the_spacing() {
        assert_eq!(count(1500.0, 375.0), 3);
        assert_eq!(count(1500.0, 750.0), 1);
        assert_eq!(count(1500.0, 1500.0), 0);
        assert_eq!(count(1500.0, 0.0), 0);
        assert!((pitch(1500.0, 3) - 375.0).abs() < 1e-12);
    }

    #[test]
    fn an_unstiffened_plate_stays_close_to_the_classical_coefficient_four() {
        let plain = Stiffened { count: 0, area: 0.0, inertia: 0.0, ..S };
        assert!((plain.gamma() - 10.92 / 12.0).abs() < 1e-12);
        let square = smeared(&plain, 1500.0, 1.0, 210000.0, 0.3);
        assert!((square.k_sigma_p - 2.0 * (4.0 + 10.92 / 12.0 - 1.0) / 2.0).abs() < 1e-9);
        assert!((square.k_sigma_p / 4.0 - 1.0).abs() < 0.03);
    }

    #[test]
    fn the_smeared_worked_example_matches_the_independent_recomputation() {
        let r = smeared(&S, 3000.0, 1.0, 210000.0, 0.3);
        assert!((S.whole_inertia() - 10_187_500.0).abs() < 1e-3);
        assert!((r.gamma - 74.165).abs() < 1e-3 && (r.delta - 0.2).abs() < 1e-12 && (r.alpha - 2.0).abs() < 1e-12);
        assert!((r.sigma_e - 8.435_559_317).abs() < 1e-8);
        assert!((r.k_sigma_p - 20.451_041_667).abs() < 1e-6);
        assert!((r.sigma_cr_p - 172.515_975_077).abs() < 1e-5);
    }

    #[test]
    fn a_long_panel_switches_to_the_second_branch() {
        let r = smeared(&S, 60_000.0, 1.0, 210000.0, 0.3);
        assert!((r.k_sigma_p - 4.0 * (1.0 + 74.165_f64.sqrt()) / (2.0 * 1.2)).abs() < 1e-3);
    }

    #[test]
    fn the_single_stiffener_worked_example_matches_the_independent_recomputation() {
        let one = Stiffened { count: 1, ..S };
        let long = single(&one, 3000.0, 750.0, 210000.0, 0.3);
        assert!((long.a_c - 2803.696_719).abs() < 1e-5);
        assert!((long.sigma_cr_sl - 438.269_323_590).abs() < 1e-5);
        assert!((long.sigma_cr_sub - 134.968_949_074).abs() < 1e-6);
        assert!((long.sigma_cr_p - long.sigma_cr_sub).abs() < 1e-12);
        let short = single(&one, 2000.0, 0.0, 210000.0, 0.3);
        assert!((short.sigma_cr_sl - 542.642_299_438).abs() < 1e-5);
    }
}
