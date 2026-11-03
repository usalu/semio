//! 🔥️ Composite beam fire (EN 1994-1-2): the temperature field over the cross-section of the beam (the concrete slab by two-dimensional heat conduction from the soffit, the protected steel profile by the lumped-capacity model of EN 1993-1-2 Eq. 4.27, the hogging reinforcement read from the slab field) and the reduced plastic moment resistance over the fire duration. The property tables stay unverified (`ndp.en1994.1-2.{concrete-conduction,surface-exchange,kc-theta,tables}`); the independent oracle is `🧰️scripts/en1994-beam-fire-oracle.py`.

use super::column_fire::{concrete_cp, concrete_lambda, kc_theta, net_flux, rho_concrete, RebarProduct, KS_THETAS};
use semio_s_plugin_norm_compliance::ndp_common;
use super::constants::{BOARD_C, BOARD_LAMBDA, BOARD_RHO, HOGGING_REBAR_DEPTH_M};
use super::{iso834_c, ky_theta, plastic_full, protected_steel_history_c, ISection};

const CELL_X_M: f64 = 0.025;
const CELL_Y_M: f64 = 0.01;
const STEP_S: f64 = 1.0;

/// 🔖️ Registry ids (unverified) of the structural-steel strength reduction k_y,θ (EN 1993-1-2 Table 3.1) the fire bending resistance rests on.
pub fn steel_reduction_ndp_ids() -> Vec<String> {
    KS_THETAS.iter().map(|t| format!("ndp.en1993.1-2.t31.ky.{t}")).collect()
}

/// 🧱 The half cross-section of the slab over the beam axis (symmetry plane at `x = 0`): the fire acts on the soffit, the steel top flange (half width `flange_half_m`) shields its part of it at the steel temperature.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlabSection {
    pub thickness_m: f64,
    pub half_width_m: f64,
    pub flange_half_m: f64,
}

/// 🌡️ Slab temperatures [°C] on a grid of cells, row `j = 0` at the soffit and the last row at the slab top.
#[derive(Clone, Debug, PartialEq)]
pub struct SlabField {
    pub nx: usize,
    pub ny: usize,
    pub dx: f64,
    pub dy: f64,
    pub theta: Vec<f64>,
    pub minutes: f64,
}

impl SlabField {
    /// 🌡️ Temperature of cell `(i, j)`.
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.theta[j * self.nx + i]
    }

    /// 🌡️ Temperature at the distance `x_m` from the beam axis and the height `depth_m` above the soffit.
    pub fn sample(&self, x_m: f64, depth_m: f64) -> f64 {
        let i = ((x_m.abs() / self.dx) as usize).min(self.nx - 1);
        let j = ((depth_m.max(0.0) / self.dy) as usize).min(self.ny - 1);
        self.at(i, j)
    }

    /// 🌡️ Highest temperature of the field.
    pub fn max(&self) -> f64 {
        self.theta.iter().copied().fold(f64::MIN, f64::max)
    }

    /// 🌡️ Mean temperature over the width of row `j`.
    pub fn row_mean(&self, j: usize) -> f64 {
        (0..self.nx).map(|i| self.at(i, j.min(self.ny - 1))).sum::<f64>() / self.nx as f64
    }

    /// 🌡️ Temperature of the reinforcement layer at [`HOGGING_REBAR_DEPTH_M`] below the slab top (mean over the width).
    pub fn rebar_c(&self) -> f64 {
        self.row_mean(((self.ny as f64 * self.dy - HOGGING_REBAR_DEPTH_M).max(0.0) / self.dy) as usize)
    }

    /// 🌡️ Mean concrete strength factor k_c,θ over the top `x_m` of the slab (the compression zone of the sagging plastic moment).
    pub fn compression_factor(&self, x_m: f64) -> f64 {
        let rows = ((x_m / self.dy).ceil() as usize).clamp(1, self.ny);
        let factors: Vec<f64> = (self.ny - rows..self.ny).flat_map(|j| (0..self.nx).map(move |i| (i, j))).map(|(i, j)| kc_theta(self.at(i, j))).collect();
        factors.iter().sum::<f64>() / factors.len() as f64
    }
}

fn link(la: f64, lb: f64, ta: f64, tb: f64, face: f64, d: f64) -> f64 {
    2.0 * la * lb / (la + lb) * (tb - ta) / d * face
}

fn surface_flux(gas: f64, theta: f64, lambda: f64, half_cell: f64) -> f64 {
    let d = gas - theta;
    if d.abs() < 1e-9 { 0.0 } else { d / (d / net_flux(gas, theta) + half_cell / lambda) }
}

/// 🔥️ Steel temperature at `t_s` seconds from a one-second [`protected_steel_history_c`].
pub fn steel_at(history: &[f64], t_s: f64) -> f64 {
    history.get(t_s.max(0.0).round() as usize).or(history.last()).copied().unwrap_or(20.0)
}

/// 🌡️ Slab fields after each duration of `ends_s` (ascending seconds) of the ISO 834 fire by explicit finite-volume conduction with the surface exchange of EN 1991-1-2 on the soffit, the steel temperature `steel_c` (one entry per second) under the top flange and the unexposed exchange on the top.
pub fn slab_fields(s: &SlabSection, steel_c: &[f64], ends_s: &[f64]) -> Vec<SlabField> {
    let nx = ((s.half_width_m / CELL_X_M).ceil() as usize).clamp(4, 80);
    let ny = ((s.thickness_m / CELL_Y_M).ceil() as usize).clamp(4, 40);
    let (dx, dy) = (s.half_width_m / nx as f64, s.thickness_m / ny as f64);
    let (alpha_unexposed, rho_c) = (ndp_common("ndp.en1994.1-2.alpha-unexposed"), rho_concrete());
    let on_flange: Vec<bool> = (0..nx).map(|i| (i as f64 + 0.5) * dx <= s.flange_half_m).collect();
    let mut theta = vec![20.0; nx * ny];
    let (mut lambda, mut capacity, mut next) = (vec![0.0; nx * ny], vec![0.0; nx * ny], vec![0.0; nx * ny]);
    let mut t = 0.0;
    let mut out = Vec::new();
    for &end in ends_s {
        while t < end - 1e-9 {
            let step = STEP_S.min(end - t);
            let (gas, steel) = (iso834_c(t + step / 2.0), steel_at(steel_c, t + step / 2.0));
            for k in 0..nx * ny {
                (lambda[k], capacity[k]) = (concrete_lambda(theta[k]), rho_c * concrete_cp(theta[k]));
            }
            for j in 0..ny {
                for i in 0..nx {
                    let k = j * nx + i;
                    let mut q = 0.0;
                    if i + 1 < nx { q += link(lambda[k], lambda[k + 1], theta[k], theta[k + 1], dy, dx) }
                    if i > 0 { q += link(lambda[k], lambda[k - 1], theta[k], theta[k - 1], dy, dx) }
                    if j + 1 < ny { q += link(lambda[k], lambda[k + nx], theta[k], theta[k + nx], dx, dy) }
                    if j > 0 { q += link(lambda[k], lambda[k - nx], theta[k], theta[k - nx], dx, dy) }
                    if j == 0 { q += if on_flange[i] { lambda[k] * (steel - theta[k]) / (dy / 2.0) * dx } else { surface_flux(gas, theta[k], lambda[k], dy / 2.0) * dx } }
                    if j + 1 == ny { q += (20.0 - theta[k]) / (1.0 / alpha_unexposed + dy / 2.0 / lambda[k]) * dx }
                    next[k] = theta[k] + q * step / (capacity[k] * dx * dy);
                }
            }
            std::mem::swap(&mut theta, &mut next);
            t += step;
        }
        out.push(SlabField { nx, ny, dx, dy, theta: theta.clone(), minutes: end / 60.0 });
    }
    out
}

/// 🔥️ The reduced sagging plastic moment at one steel temperature: `steel_only_nm` with k_y,θ on the steel (the model of the `mfi` check), `m_nm` additionally with the mean k_c,θ of the slab compression zone.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reduced {
    pub theta_a: f64,
    pub ky: f64,
    pub kc: f64,
    pub steel_only_nm: f64,
    pub m_nm: f64,
}

/// 🔥️ One time of the fire evolution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub minutes: f64,
    pub gas_c: f64,
    pub steel_c: f64,
    pub rebar_c: f64,
    pub rebar_ks: f64,
    pub slab_top_c: f64,
    pub soffit_c: f64,
    pub reduced: Reduced,
}

/// 🔥️ Temperature fields and resistances at the requested times.
#[derive(Clone, Debug, PartialEq)]
pub struct Evolution {
    pub points: Vec<Point>,
    pub fields: Vec<SlabField>,
}

/// 🧩 The fire inputs of one composite beam (SI; `fyd` and `fcc` with γ_M,fi = 1.0, `ys` the slab thickness, `thermal_m` the effective thickness `h_eff` = h_c + h_p·b_0/pitch of the profiled deck the temperature field is computed for, `hc` the concrete height above the ribs, `am_v` = A_p/V of the protected steel, `rebar` the production route of the hogging bars whose k_s,θ is read at the temperature of the reinforcement layer).
#[derive(Clone, Copy, Debug)]
pub struct Beam {
    pub sec: ISection,
    pub fyd: f64,
    pub ys: f64,
    pub thermal_m: f64,
    pub beff: f64,
    pub hc: f64,
    pub fcc: f64,
    pub am_v: f64,
    pub board_m: f64,
    pub rebar: RebarProduct,
}

impl Beam {
    /// 🧱 The slab half cross-section.
    pub fn slab(&self) -> SlabSection {
        SlabSection { thickness_m: self.thermal_m, half_width_m: self.beff / 2.0, flange_half_m: (self.sec.b / 2.0).min(self.beff / 2.0) }
    }

    /// 🔥️ Steel temperature after every second up to `t_end_s`.
    pub fn steel_history(&self, t_end_s: f64) -> Vec<f64> {
        protected_steel_history_c(t_end_s, self.am_v, self.board_m.max(1e-4), BOARD_LAMBDA, BOARD_C, BOARD_RHO)
    }

    /// 🔥️ Reduced plastic moment resistance at the steel temperature `theta_a`, with the slab `field` for k_c,θ of the compression zone.
    pub fn reduced(&self, theta_a: f64, field: Option<&SlabField>) -> Reduced {
        let ky = ky_theta(theta_a);
        let steel_only = plastic_full(&self.sec, self.fyd * ky, self.ys, self.beff, self.hc, self.fcc);
        let kc = field.map_or(1.0, |f| f.compression_factor(steel_only.x_m));
        let m_nm = if (kc - 1.0).abs() < 1e-12 { steel_only.m_nm } else { plastic_full(&self.sec, self.fyd * ky, self.ys, self.beff, self.hc, self.fcc * kc).m_nm };
        Reduced { theta_a, ky, kc, steel_only_nm: steel_only.m_nm, m_nm }
    }

    /// 🔥️ The evolution at the ascending `minutes`.
    pub fn evolve(&self, minutes: &[f64]) -> Evolution {
        let ends: Vec<f64> = minutes.iter().map(|m| m * 60.0).collect();
        let history = self.steel_history(ends.iter().copied().fold(0.0, f64::max));
        let fields = slab_fields(&self.slab(), &history, &ends);
        let ks = self.rebar.reduction();
        let points = minutes
            .iter()
            .zip(&fields)
            .map(|(&m, field)| {
                let steel_c = steel_at(&history, m * 60.0);
                Point { minutes: m, gas_c: iso834_c(m * 60.0), steel_c, rebar_c: field.rebar_c(), rebar_ks: ks.at(field.rebar_c()), slab_top_c: field.row_mean(field.ny - 1), soffit_c: field.row_mean(0), reduced: self.reduced(steel_c, Some(field)) }
            })
            .collect();
        Evolution { points, fields }
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
