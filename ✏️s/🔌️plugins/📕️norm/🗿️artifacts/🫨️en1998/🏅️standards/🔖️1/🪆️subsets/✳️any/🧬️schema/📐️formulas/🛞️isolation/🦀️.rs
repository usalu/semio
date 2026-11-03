//! 🛞 Seismic isolation by the equivalent linear method (EN 1998-2 chapter 7 for bridges, EN 1998-1 chapter 10 for buildings): the bilinear hysteresis of the isolator units, the effective stiffness K_eff, the effective period T_eff = 2π √(M/K_eff), the equivalent damping ξ_eff = E_D/(2π K_eff d²), the damping correction η and the design displacement d_cd from K_eff d_cd = M S_e(T_eff, ξ_eff). The limits (period ratio, period, damping) are unverified registry rows, see [`super`].

use super::part_1;

/// 🛞 Bilinear hysteresis of an isolation system: characteristic strength Q_d (force at zero displacement), post-yield stiffness K_d and yield displacement d_y, so the elastic stiffness is K_1 = K_d + Q_d/d_y.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bilinear {
    pub strength_n: f64,
    pub post_yield_n_per_m: f64,
    pub yield_displacement_m: f64,
}

/// 🌍️ Spectrum constants of the site with the importance factor in `a_g`, so that S_e = a_g S (…) of EN 1998-1 3.2.2.2.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spectrum {
    pub a_g: f64,
    pub s: f64,
    pub tb: f64,
    pub tc: f64,
    pub td: f64,
}

impl Spectrum {
    /// 📈 Elastic spectrum ordinate S_e(T) with the damping correction η.
    pub fn elastic(&self, t: f64, eta: f64) -> f64 {
        part_1::elastic_response_spectrum_eta(self.a_g, self.s, self.tb, self.tc, self.td, t, eta)
    }
}

/// 🛞 The equivalent linear state of the isolation system at the displacement d.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub displacement_m: f64,
    pub force_n: f64,
    pub effective_stiffness_n_per_m: f64,
    pub period_s: f64,
    pub damping: f64,
    pub eta: f64,
    pub dissipated_energy_nm: f64,
}

/// 🛞 Largest displacement searched for the equilibrium of the design displacement [m].
pub const SEARCH_LIMIT_M: f64 = 10.0;

impl Bilinear {
    /// 🛡️ Whether the three parameters describe a hysteresis loop: Q_d ≥ 0, K_d > 0 and d_y > 0, all finite.
    pub fn is_valid(&self) -> bool {
        [self.strength_n, self.post_yield_n_per_m, self.yield_displacement_m].iter().all(|v| v.is_finite()) && self.strength_n >= 0.0 && self.post_yield_n_per_m > 0.0 && self.yield_displacement_m > 0.0
    }

    /// 〰️ Elastic stiffness K_1 = K_d + Q_d/d_y.
    pub fn elastic_stiffness(&self) -> f64 {
        self.post_yield_n_per_m + self.strength_n / self.yield_displacement_m
    }

    /// 〰️ Force F(d) on the backbone curve: K_1 d up to the yield displacement, Q_d + K_d d beyond.
    pub fn force_n(&self, displacement_m: f64) -> f64 {
        let d = displacement_m.abs();
        if d <= self.yield_displacement_m { self.elastic_stiffness() * d } else { self.strength_n + self.post_yield_n_per_m * d }
    }

    /// 🔁 Energy E_D = 4 Q_d (d − d_y) dissipated in one cycle of amplitude d (zero while the system stays elastic).
    pub fn dissipated_energy_nm(&self, displacement_m: f64) -> f64 {
        4.0 * self.strength_n * (displacement_m.abs() - self.yield_displacement_m).max(0.0)
    }

    /// 🛞 State at the displacement d for the supported mass M: K_eff = F/d, T_eff, ξ_eff and η = √(10/(5 + ξ_eff[%])) ≥ 0.55.
    pub fn state(&self, displacement_m: f64, mass_kg: f64) -> State {
        let d = displacement_m.abs();
        let force_n = self.force_n(d);
        let effective_stiffness_n_per_m = force_n / d;
        let dissipated_energy_nm = self.dissipated_energy_nm(d);
        let damping = dissipated_energy_nm / (2.0 * std::f64::consts::PI * effective_stiffness_n_per_m * d * d);
        State { displacement_m: d, force_n, effective_stiffness_n_per_m, period_s: 2.0 * std::f64::consts::PI * (mass_kg / effective_stiffness_n_per_m).sqrt(), damping, eta: part_1::damping_correction(100.0 * damping), dissipated_energy_nm }
    }

    /// 🎯 Design displacement d_cd: the amplitude at which the force K_eff d equals the spectral force M S_e(T_eff, ξ_eff) (bisection between a micrometre and [`SEARCH_LIMIT_M`]); `None` when the parameters are no loop or the system cannot balance the spectral force inside the search range.
    pub fn design(&self, mass_kg: f64, spectrum: &Spectrum) -> Option<State> {
        if !self.is_valid() || !(mass_kg > 0.0) {
            return None;
        }
        let residual = |d: f64| {
            let state = self.state(d, mass_kg);
            state.force_n - mass_kg * spectrum.elastic(state.period_s, state.eta)
        };
        let (mut low, mut high) = (1.0e-6, SEARCH_LIMIT_M);
        if residual(low) > 0.0 || residual(high) < 0.0 {
            return None;
        }
        for _ in 0..200 {
            let mid = 0.5 * (low + high);
            if residual(mid) < 0.0 { low = mid } else { high = mid }
        }
        Some(self.state(0.5 * (low + high), mass_kg))
    }

    /// 🔁 Closed hysteresis loop of amplitude d as (displacement, force) vertices: the backbone point, the elastic unloading of 2 d_y, the opposite point and its unloading, so that the enclosed area is E_D.
    pub fn loop_points(&self, displacement_m: f64) -> Vec<(f64, f64)> {
        let d = displacement_m.abs().max(self.yield_displacement_m);
        let f = self.force_n(d);
        let (dy, drop) = (self.yield_displacement_m, 2.0 * self.elastic_stiffness() * self.yield_displacement_m);
        vec![(d, f), (d - 2.0 * dy, f - drop), (-d, -f), (-d + 2.0 * dy, -f + drop), (d, f)]
    }
}

/// ⏱ Effective period T_eff = 2π √(M/K_eff) of a mass on the effective stiffness.
pub fn effective_period_s(mass_kg: f64, effective_stiffness_n_per_m: f64) -> f64 {
    2.0 * std::f64::consts::PI * (mass_kg / effective_stiffness_n_per_m).sqrt()
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
