//! 🔥️ Composite column fire (C-32, C-46): the simple calculation model of EN 1994-1-2 (Annex H, partially encased sections) on a temperature field from a two-dimensional heat-conduction analysis, and the tabulated-data structure whose cells stay unverified until a human transcribes them.

use super::encased::{EncasedI, Encasement};
use super::{iso834_c, ky_theta, steel_specific_heat};
use semio_s_plugin_norm_compliance::{ndp_cell, ndp_common, AnnexChoice};
use std::f64::consts::PI;

const SIGMA: f64 = 5.67e-8;
const ALPHA_C: f64 = 25.0;
const EMISSIVITY: f64 = 0.7;
const RHO_STEEL: f64 = 7850.0;
const E_A: f64 = 210e9;
const E_S: f64 = 210e9;
const CELL_M: f64 = 0.01;
const FIRE_BUCKLING_ALPHA: f64 = 0.49;

fn interp(table: &[(f64, f64)], x: f64) -> f64 {
    let i = table.partition_point(|p| p.0 <= x);
    match (i.checked_sub(1).map(|k| table[k]), table.get(i)) {
        (Some(a), Some(b)) => a.1 + (b.1 - a.1) * (x - a.0) / (b.0 - a.0),
        (Some(a), None) => a.1,
        (None, Some(b)) => b.1,
        (None, None) => 0.0,
    }
}

const THETAS: [f64; 13] = [20.0, 100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0, 1100.0, 1200.0];

fn table(values: [f64; 13]) -> Vec<(f64, f64)> {
    THETAS.iter().copied().zip(values).collect()
}

/// 🔥️ Concrete strength reduction k_c,θ (unverified `en1994.1-2.kc-theta`).
pub fn kc_theta(theta: f64) -> f64 {
    interp(&table([1.0, 1.0, 0.95, 0.85, 0.75, 0.60, 0.45, 0.30, 0.15, 0.08, 0.04, 0.01, 0.0]), theta)
}

/// 🔥️ Strain ε_c1,θ at the peak concrete stress (unverified `en1994.1-2.eps-c1-theta`).
pub fn eps_c1_theta(theta: f64) -> f64 {
    interp(&table([2.5e-3, 4.0e-3, 5.5e-3, 7.0e-3, 10.0e-3, 15.0e-3, 25.0e-3, 25.0e-3, 25.0e-3, 25.0e-3, 25.0e-3, 25.0e-3, 25.0e-3]), theta)
}

/// 🔥️ Modulus reduction k_E,θ of steel and reinforcement (unverified `en1994.1-2.ke-theta`).
pub fn ke_theta(theta: f64) -> f64 {
    interp(&table([1.0, 1.0, 0.9, 0.8, 0.7, 0.6, 0.31, 0.13, 0.09, 0.0675, 0.045, 0.0225, 0.0]), theta)
}

/// 🔥️ Secant modulus E_c,sec,θ = k_c,θ·f_ck/ε_c1,θ of the concrete at the temperature θ.
pub fn concrete_secant_modulus_pa(theta: f64, fck_pa: f64) -> f64 {
    kc_theta(theta) * fck_pa / eps_c1_theta(theta)
}

/// 🧱 Unit mass of the concrete [kg/m³] (unverified `ndp.en1994.1-2.rho-concrete`).
pub(super) fn rho_concrete() -> f64 {
    ndp_common("ndp.en1994.1-2.rho-concrete")
}

pub(super) const KS_THETAS: [u32; 13] = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200];

/// 🔩 Production route of the reinforcing steel (EN 1992-1-2 4.2.4.3, Table 3.2a): the strength reduction k_s,θ of hot-rolled and cold-worked bars differ above 300 °C and never equal the structural-steel k_y,θ by definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RebarProduct {
    HotRolled,
    ColdWorked,
}

impl From<semio_s_artifact_norm_en1994_model::ReinforcementProduct> for RebarProduct {
    fn from(product: semio_s_artifact_norm_en1994_model::ReinforcementProduct) -> Self {
        match product {
            semio_s_artifact_norm_en1994_model::ReinforcementProduct::HotRolled => Self::HotRolled,
            semio_s_artifact_norm_en1994_model::ReinforcementProduct::ColdWorked => Self::ColdWorked,
        }
    }
}

impl RebarProduct {
    /// 🔖️ Wire value (`hot_rolled`, `cold_worked`).
    pub fn wire(self) -> &'static str {
        match self {
            Self::HotRolled => "hot_rolled",
            Self::ColdWorked => "cold_worked",
        }
    }

    fn row(self) -> &'static str {
        match self {
            Self::HotRolled => "hot",
            Self::ColdWorked => "cold",
        }
    }

    /// 🔖️ Registry ids (`ndp.en1994.1-2.ks.*`, unverified) of the reduction nodes of this product, for a check to cite.
    pub fn ndp_ids(self) -> Vec<String> {
        KS_THETAS.iter().map(|t| format!("ndp.en1994.1-2.ks.{}.{t}", self.row())).collect()
    }

    /// 📈 The reduction table of this product read from the registry once.
    pub fn reduction(self) -> RebarReduction {
        RebarReduction { ks: KS_THETAS.map(|t| ndp_cell("ndp.en1994.1-2.ks", &[self.row(), &t.to_string()], AnnexChoice::En)) }
    }
}

/// 📈 Strength reduction k_s,θ of reinforcing steel at the nodes of 100 K, linear in between.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RebarReduction {
    ks: [f64; 13],
}

impl RebarReduction {
    /// 📈 k_s,θ at the temperature `theta` [°C].
    pub fn at(&self, theta: f64) -> f64 {
        interp(&KS_THETAS.iter().map(|&t| f64::from(t)).zip(self.ks).collect::<Vec<_>>(), theta.clamp(20.0, 1200.0))
    }
}

pub(super) fn concrete_lambda(theta: f64) -> f64 {
    let t = theta.clamp(20.0, 1200.0) / 100.0;
    1.36 - 0.136 * t + 0.0057 * t * t
}

pub(super) fn concrete_cp(theta: f64) -> f64 {
    if theta <= 100.0 { 900.0 } else if theta <= 200.0 { 900.0 + (theta - 100.0) } else if theta <= 400.0 { 1000.0 + (theta - 200.0) / 2.0 } else { 1100.0 }
}

pub(super) fn steel_lambda(theta: f64) -> f64 {
    if theta < 800.0 { 54.0 - 3.33e-2 * theta } else { 27.3 }
}

pub(super) fn net_flux(gas: f64, theta: f64) -> f64 {
    ALPHA_C * (gas - theta) + EMISSIVITY * SIGMA * ((gas + 273.0).powi(4) - (theta + 273.0).powi(4))
}

/// 🌡️ Temperatures [°C] of one quadrant of the section (symmetry axes at `x = 0` and `y = 0`) on a grid of cells, with the material of each cell.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub nx: usize,
    pub ny: usize,
    pub dx: f64,
    pub dy: f64,
    pub theta: Vec<f64>,
    pub steel: Vec<bool>,
    pub minutes: f64,
}

impl Field {
    /// 🌡️ Temperature of cell `(i, j)`.
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.theta[j * self.nx + i]
    }

    /// 🌡️ Temperature of the cell containing the point `(x, y)` measured from the centre.
    pub fn sample(&self, x: f64, y: f64) -> f64 {
        let i = ((x.abs() / self.dx) as usize).min(self.nx - 1);
        let j = ((y.abs() / self.dy) as usize).min(self.ny - 1);
        self.at(i, j)
    }

    /// 🌡️ Highest temperature of the field.
    pub fn max(&self) -> f64 {
        self.theta.iter().copied().fold(f64::MIN, f64::max)
    }

    /// 🌡️ Mean temperature of the steel cells (0 without steel).
    pub fn steel_mean(&self) -> f64 {
        let cells: Vec<f64> = self.theta.iter().zip(&self.steel).filter(|(_, s)| **s).map(|(t, _)| *t).collect();
        if cells.is_empty() { 0.0 } else { cells.iter().sum::<f64>() / cells.len() as f64 }
    }
}

fn extent(g: &EncasedI) -> (f64, f64) {
    match g.kind {
        Encasement::Full => (g.b_c.max(g.b) / 2.0, g.h_c.max(g.h) / 2.0),
        Encasement::BetweenFlanges => (g.b / 2.0, g.h / 2.0),
    }
}

fn is_steel(g: &EncasedI, x: f64, y: f64, dy: f64) -> bool {
    let flange = y >= g.h / 2.0 - g.tf.max(dy / 2.0) && x <= g.b / 2.0 && y <= g.h / 2.0;
    let web = x <= g.tw / 2.0 && y < g.h / 2.0 - g.tf.max(dy / 2.0);
    flange || web
}

/// 🌡️ Temperature fields after each duration of `ends_s` (ascending seconds) of the ISO 834 standard fire by explicit finite-volume heat conduction on one quadrant, with the surface exchange of EN 1991-1-2 and perfect steel–concrete contact.
pub fn temperature_fields(g: &EncasedI, ends_s: &[f64]) -> Vec<Field> {
    let (wx, hy) = extent(g);
    let nx = ((wx / CELL_M).ceil() as usize).clamp(4, 60);
    let ny = ((hy / CELL_M).ceil() as usize).clamp(4, 60);
    let (dx, dy) = (wx / nx as f64, hy / ny as f64);
    let steel: Vec<bool> = (0..nx * ny).map(|k| is_steel(g, ((k % nx) as f64 + 0.5) * dx, ((k / nx) as f64 + 0.5) * dy, dy)).collect();
    let mut theta = vec![20.0; nx * ny];
    let rho_c = rho_concrete();
    let alpha_max = (steel_lambda(20.0) / (RHO_STEEL * steel_specific_heat(20.0))).max(concrete_lambda(20.0) / (rho_concrete() * concrete_cp(20.0)));
    let dt = (0.9 * dx.min(dy).powi(2) / (4.0 * alpha_max)).min(1.0);
    let mut out = Vec::new();
    let (mut t, mut lambda, mut capacity, mut next) = (0.0, vec![0.0; nx * ny], vec![0.0; nx * ny], vec![0.0; nx * ny]);
    for &end in ends_s {
        while t < end - 1e-9 {
            let step = dt.min(end - t);
            let gas = iso834_c(t + step / 2.0);
            for k in 0..nx * ny {
                (lambda[k], capacity[k]) = if steel[k] { (steel_lambda(theta[k]), RHO_STEEL * steel_specific_heat(theta[k])) } else { (concrete_lambda(theta[k]), rho_c * concrete_cp(theta[k])) };
            }
            for j in 0..ny {
                for i in 0..nx {
                    let k = j * nx + i;
                    let mut q = 0.0;
                    let mut link = |other: usize, face: f64, d: f64| q += 2.0 * lambda[k] * lambda[other] / (lambda[k] + lambda[other]) * (theta[other] - theta[k]) / d * face;
                    if i + 1 < nx { link(k + 1, dy, dx) }
                    if i > 0 { link(k - 1, dy, dx) }
                    if j + 1 < ny { link(k + nx, dx, dy) }
                    if j > 0 { link(k - nx, dx, dy) }
                    if i + 1 == nx { q += net_flux(gas, theta[k]) * dy }
                    if j + 1 == ny { q += net_flux(gas, theta[k]) * dx }
                    next[k] = theta[k] + q * step / (capacity[k] * dx * dy);
                }
            }
            std::mem::swap(&mut theta, &mut next);
            t += step;
        }
        out.push(Field { nx, ny, dx, dy, theta: theta.clone(), steel: steel.clone(), minutes: end / 60.0 });
    }
    out
}

/// 🌡️ Temperature field after `t_end_s` seconds of the standard fire.
pub fn temperature_field(g: &EncasedI, t_end_s: f64) -> Field {
    temperature_fields(g, &[t_end_s]).remove(0)
}

/// 🔥️ Characteristic strengths (γ_M,fi = 1.0) of the materials at 20 °C: steel f_y, concrete f_ck (no 0.85), bars f_sk.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FireStrengths {
    pub fy: f64,
    pub fck: f64,
    pub fsk: f64,
    pub ks: RebarReduction,
}

impl FireStrengths {
    /// 🔥️ The strengths with the reduction table of the reinforcement `product`.
    pub fn new(fy: f64, fck: f64, fsk: f64, product: RebarProduct) -> Self {
        Self { fy, fck, fsk, ks: product.reduction() }
    }
}

struct Piece {
    y: f64,
    area: f64,
    theta: f64,
    steel: bool,
}

fn pieces(g: &EncasedI, field: &Field) -> Vec<Piece> {
    let count = |steel: bool| field.steel.iter().filter(|s| **s == steel).count().max(1) as f64;
    let net = g.concrete_area() / 4.0;
    let steel_scale = g.steel_area() / 4.0 / (count(true) * field.dx * field.dy);
    let concrete_scale = net / (count(false) * field.dx * field.dy);
    let mut out = Vec::new();
    for j in 0..field.ny {
        for i in 0..field.nx {
            let steel = field.steel[j * field.nx + i];
            let area = 2.0 * field.dx * field.dy * if steel { steel_scale } else { concrete_scale };
            let (y, theta) = ((j as f64 + 0.5) * field.dy, field.at(i, j));
            out.push(Piece { y, area, theta, steel });
            out.push(Piece { y: -y, area, theta, steel });
        }
    }
    out
}

fn bar_pieces(g: &EncasedI, field: &Field) -> Vec<Piece> {
    let level = (field.ny as f64 * field.dy - g.cover).max(0.0);
    let x = (field.nx as f64 * field.dx - g.cover).max(0.0);
    let theta = field.sample(x, level);
    [level, -level].iter().map(|&y| Piece { y, area: g.as_m2 / 2.0, theta, steel: true }).collect()
}

/// 🔥️ Fire resistance of the column at one time: plastic axial resistance, effective stiffness, critical force, slenderness, buckling reduction and the resistance of the buckling-reduced section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resistance {
    pub n_pl: f64,
    pub ei: f64,
    pub n_cr: f64,
    pub lambda: f64,
    pub chi: f64,
    pub n_rd: f64,
    pub m_pl: f64,
}

fn strength(piece: &Piece, s: &FireStrengths, bar: bool) -> f64 {
    if bar { s.ks.at(piece.theta) * s.fsk } else if piece.steel { ky_theta(piece.theta) * s.fy } else { kc_theta(piece.theta) * s.fck }
}

fn modulus(piece: &Piece, s: &FireStrengths, bar: bool) -> f64 {
    if bar { E_S * ke_theta(piece.theta) } else if piece.steel { E_A * ke_theta(piece.theta) } else { concrete_secant_modulus_pa(piece.theta, s.fck) }
}

fn plastic_moment(all: &[(&Piece, bool)], s: &FireStrengths, top: f64) -> f64 {
    let at = |y_na: f64| -> (f64, f64) {
        all.iter().fold((0.0, 0.0), |(n, m), (p, bar)| {
            let f = strength(p, s, *bar) * p.area;
            let signed = if p.y > y_na { f } else if p.steel || *bar { -f } else { 0.0 };
            (n + signed, m + signed * p.y)
        })
    };
    let (mut lo, mut hi) = (-top, top);
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if at(mid).0 > 0.0 { lo = mid } else { hi = mid }
    }
    at(0.5 * (lo + hi)).1
}

/// 🔥️ Resistance of the encased or partially encased column of buckling length `length_m` at the temperature field `field` by summing temperature-reduced fibres (simple calculation model, stiffness coefficients φ_i,θ = 1).
pub fn resistance(g: &EncasedI, field: &Field, s: &FireStrengths, length_m: f64) -> Resistance {
    let cells = pieces(g, field);
    let bars = bar_pieces(g, field);
    let all: Vec<(&Piece, bool)> = cells.iter().map(|p| (p, false)).chain(bars.iter().map(|p| (p, true))).collect();
    let n_pl = all.iter().map(|(p, bar)| strength(p, s, *bar) * p.area).sum::<f64>();
    let ei = all.iter().map(|(p, bar)| modulus(p, s, *bar) * p.area * p.y * p.y).sum::<f64>();
    let n_cr = PI * PI * ei / length_m.max(0.1).powi(2);
    let lambda = (n_pl / n_cr.max(1.0)).sqrt();
    let chi = super::chi_column(lambda, FIRE_BUCKLING_ALPHA);
    let m_pl = plastic_moment(&all, s, field.ny as f64 * field.dy);
    Resistance { n_pl, ei, n_cr, lambda, chi, n_rd: chi * n_pl, m_pl }
}

/// 📚️ One cell of a tabulated-data table: the transcribed value, if any, and whether a human verified it against the printed standard.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableCell {
    pub value: Option<f64>,
    pub verified: bool,
}

impl TableCell {
    /// 📚️ A cell that has not been transcribed.
    pub const MISSING: Self = Self { value: None, verified: false };
}

/// 📚️ One row of the tabulated minimum dimensions of a composite column for a fire resistance class and a load level η_fi.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColumnRow {
    pub rating_min: u32,
    pub load_level: f64,
    pub min_width_m: TableCell,
    pub min_axis_distance_m: TableCell,
    pub min_reinforcement_ratio: TableCell,
}

/// 📚️ The tabulated column data of EN 1994-1-2 (C-46); empty until a human transcribes the cells with `verified: false` and registers them (`en1994.1-2.tables`).
pub const COLUMN_TABLE: &[ColumnRow] = &[];

/// 📚️ Outcome of the tabulated-data check of one column.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tabulated {
    NoData,
    Evaluated { utilization: f64, verified: bool },
}

/// 📚️ Tabulated check against the first row of `rows` for `rating_min` whose load level is not below `eta_fi`; a missing cell means no data, never a guessed value.
pub fn check_tabulated(rows: &[ColumnRow], rating_min: u32, eta_fi: f64, width_m: f64, axis_distance_m: f64, ratio: f64) -> Tabulated {
    let Some(row) = rows.iter().filter(|r| r.rating_min == rating_min && r.load_level + 1e-9 >= eta_fi).min_by(|a, b| a.load_level.total_cmp(&b.load_level)) else { return Tabulated::NoData };
    let (Some(w), Some(a), Some(r)) = (row.min_width_m.value, row.min_axis_distance_m.value, row.min_reinforcement_ratio.value) else { return Tabulated::NoData };
    let utilization = (w / width_m.max(1e-9)).max(a / axis_distance_m.max(1e-9)).max(r / ratio.max(1e-9));
    Tabulated::Evaluated { utilization, verified: row.min_width_m.verified && row.min_axis_distance_m.verified && row.min_reinforcement_ratio.verified }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
