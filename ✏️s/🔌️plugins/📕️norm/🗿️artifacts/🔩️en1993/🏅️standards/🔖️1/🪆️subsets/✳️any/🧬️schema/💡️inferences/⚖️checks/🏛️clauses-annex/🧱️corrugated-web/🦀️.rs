//! 🧱️ EN 1993-1-5 Annex D corrugated web shear: fold geometry, local and global shear buckling stresses of the equivalent orthotropic plate and the shear resistance (mm, N, MPa). See [EN 1993-1-5 Annex D](https://eurocodes.jrc.ec.europa.eu/).

/// 🧱️ Trapezoidal fold: flat fold a_0, horizontal projection a_1 of the inclined fold and corrugation depth d.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corrugation {
    pub a0: f64,
    pub a1: f64,
    pub d: f64,
}

impl Corrugation {
    /// 🧱️ Length of the inclined fold a_2 = √(a_1² + d²).
    pub fn inclined(&self) -> f64 {
        self.a1.hypot(self.d)
    }

    /// 🧱️ Horizontal wave pitch w = 2 (a_0 + a_1).
    pub fn pitch(&self) -> f64 {
        2.0 * (self.a0 + self.a1)
    }

    /// 🧱️ Developed length of one wave s = 2 (a_0 + a_2).
    pub fn developed(&self) -> f64 {
        2.0 * (self.a0 + self.inclined())
    }

    /// 🧱️ Second moment of one wave of thickness `t` about its neutral axis, I = t d² (a_0/2 + a_2/6).
    pub fn wave_inertia(&self, t: f64) -> f64 {
        t * self.d * self.d * (self.a0 / 2.0 + self.inclined() / 6.0)
    }
}

/// 🧱️ Bending stiffness per unit width across the folds B_x = E I_wave/w.
pub fn b_across(e: f64, t: f64, g: &Corrugation) -> f64 {
    e * g.wave_inertia(t) / g.pitch()
}

/// 🧱️ Bending stiffness per unit width along the folds B_z = E t³/(12 (1 − ν²)) s/w.
pub fn b_along(e: f64, nu: f64, t: f64, g: &Corrugation) -> f64 {
    e * t.powi(3) / (12.0 * (1.0 - nu * nu)) * g.developed() / g.pitch()
}

/// 🧱️ Local shear buckling stress of the widest flat fold, τ_cr,l = k_l π² E/(12 (1 − ν²)) (t/a_max)² with k_l = 5.34 (unverified clause locator).
pub fn tau_cr_local(e: f64, nu: f64, t: f64, g: &Corrugation) -> f64 {
    let a_max = g.a0.max(g.inclined());
    5.34 * std::f64::consts::PI.powi(2) * e / (12.0 * (1.0 - nu * nu)) * (t / a_max).powi(2)
}

/// 🧱️ Global shear buckling stress τ_cr,g = 32.4/(t h_w²) (B_x B_z³)^¼ (unverified clause locator).
pub fn tau_cr_global(t: f64, hw: f64, bx: f64, bz: f64) -> f64 {
    32.4 / (t * hw * hw) * (bx * bz.powi(3)).powf(0.25)
}

/// 🧱️ Reduction for local buckling χ_c,l = 1.15/(0.9 + λ̄) ≤ 1 (unverified clause locator).
pub fn chi_local(lambda: f64) -> f64 {
    (1.15 / (0.9 + lambda)).min(1.0)
}

/// 🧱️ Reduction for global buckling χ_c,g = 1.5/(0.5 + λ̄²) ≤ 1 (unverified clause locator).
pub fn chi_global(lambda: f64) -> f64 {
    (1.5 / (0.5 + lambda * lambda)).min(1.0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorrugatedShear {
    pub tau_cr_local: f64,
    pub tau_cr_global: f64,
    pub lambda_local: f64,
    pub lambda_global: f64,
    pub chi: f64,
    pub v_b_rd: f64,
}

/// 🧱️ Shear resistance V_b,Rd = χ_c f_yw h_w t_w/(√3 γ_M1) with χ_c the smaller of the local and global reductions.
pub fn shear_resistance(g: &Corrugation, hw: f64, t: f64, fyw: f64, e: f64, nu: f64, gm1: f64) -> CorrugatedShear {
    let tau_l = tau_cr_local(e, nu, t, g);
    let tau_g = tau_cr_global(t, hw, b_across(e, t, g), b_along(e, nu, t, g));
    let tau_y = fyw / 3.0_f64.sqrt();
    let (lambda_local, lambda_global) = ((tau_y / tau_l).sqrt(), (tau_y / tau_g).sqrt());
    let chi = chi_local(lambda_local).min(chi_global(lambda_global));
    CorrugatedShear { tau_cr_local: tau_l, tau_cr_global: tau_g, lambda_local, lambda_global, chi, v_b_rd: chi * tau_y * hw * t / gm1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    const G: Corrugation = Corrugation { a0: 140.0, a1: 40.0, d: 50.0 };

    #[test]
    fn fold_geometry_follows_pythagoras() {
        assert!((G.inclined() - (40.0_f64 * 40.0 + 2500.0).sqrt()).abs() < 1e-12);
        assert!((G.pitch() - 360.0).abs() < 1e-12);
        assert!((G.developed() - 2.0 * (140.0 + G.inclined())).abs() < 1e-12);
        assert!(G.developed() > G.pitch());
    }

    #[test]
    fn wave_inertia_exceeds_a_flat_strip_of_the_same_pitch() {
        let t = 2.0;
        assert!(G.wave_inertia(t) / G.pitch() > t.powi(3) / 12.0);
        assert!((G.wave_inertia(t) - t * 2500.0 * (70.0 + G.inclined() / 6.0)).abs() < 1e-9);
    }

    #[test]
    fn worked_example_matches_the_independent_recomputation() {
        let r = shear_resistance(&G, 1000.0, 2.0, 355.0, 210000.0, 0.3, 1.1);
        assert!((b_across(210000.0, 2.0, &G) - 235_292_965.0).abs() < 10.0);
        assert!((b_along(210000.0, 0.3, 2.0, &G) - 174_385.677).abs() < 1e-2);
        assert!((r.tau_cr_local - 206.8434).abs() < 1e-3);
        assert!((r.tau_cr_global - 17.12184).abs() < 1e-4);
        assert!((r.lambda_local - 0.995435).abs() < 1e-5);
        assert!((r.lambda_global - 3.459861).abs() < 1e-5);
        assert!((r.chi - 0.1202825).abs() < 1e-6);
        assert!((r.v_b_rd - 44_823.68).abs() < 0.1);
    }

    #[test]
    fn shear_resistance_is_governed_by_the_smaller_reduction() {
        let r = shear_resistance(&G, 1000.0, 2.0, 355.0, 210000.0, 0.3, 1.1);
        assert!((r.chi - chi_local(r.lambda_local).min(chi_global(r.lambda_global))).abs() < 1e-15);
        let thicker = shear_resistance(&G, 1000.0, 3.0, 355.0, 210000.0, 0.3, 1.1);
        assert!(thicker.v_b_rd > r.v_b_rd);
    }

    #[test]
    fn reductions_are_capped_at_one() {
        assert!((chi_local(0.0) - 1.0).abs() < 1e-15 && (chi_global(0.1) - 1.0).abs() < 1e-15);
    }
}
