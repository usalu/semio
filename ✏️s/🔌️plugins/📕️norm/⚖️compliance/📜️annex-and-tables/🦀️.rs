//! 📜️ National annex selection and table lookups of the norm compliance core. See [`super`].

use super::*;

// #region 🔖️Annex
/// 🇪️🇺️ National annex selection for Eurocode / DIN EN families.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum AnnexChoice {
    #[dsl(key = "en")]
    En,
    #[dsl(key = "de")]
    De,
}

impl AnnexChoice {
    pub fn label(self) -> &'static str {
        match self {
            Self::En => "EN",
            Self::De => "DE-NA",
        }
    }
}

/// 🗺️ Trait for national annex parameter overrides.
/// 🔀️ O1 de-dyn: closed set (`NaDe`/`NaEn`) — closes as `NationalAnnexes` via `dyn_enum_close!` in
/// `en1990`'s schema module, where the two impls live (`🎫️tickets/…/📓️terra-dedyn-fleet-norm-report.md`).
#[dyn_enum]
pub trait NationalAnnex {
    fn choice(&self) -> AnnexChoice;
    fn gamma_g(&self) -> f64;
    fn gamma_q(&self) -> f64;
    fn gamma_m(&self, _material: &str) -> f64 {
        1.0
    }
    fn gamma_r(&self) -> f64 {
        1.0
    }
    fn xi(&self, _category: &str) -> f64 {
        1.0
    }
    fn psi_0(&self, category: &str) -> f64;
    fn psi_1(&self, category: &str) -> f64;
    fn psi_2(&self, category: &str) -> f64;
}
// #endregion 🔖️Annex

// #region 🔖️Tables
/// 📊️ One-dimensional table entry for norm lookups.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableEntry1D {
    pub x: f64,
    pub y: f64,
}

/// 🔍️ Linear interpolation in a sorted 1D table.
pub fn table_lookup_linear(table: &[TableEntry1D], x: f64) -> f64 {
    if table.is_empty() {
        return 0.0;
    }
    if x <= table[0].x {
        return table[0].y;
    }
    if x >= table[table.len() - 1].x {
        return table[table.len() - 1].y;
    }
    for w in table.windows(2) {
        if x >= w[0].x && x <= w[1].x {
            let t = (x - w[0].x) / (w[1].x - w[0].x);
            return w[0].y + t * (w[1].y - w[0].y);
        }
    }
    table[table.len() - 1].y
}

/// 🔍️ Bilinear interpolation on a regular grid.
pub fn table_lookup_bilinear(x: f64, y: f64, x_vals: &[f64], y_vals: &[f64], z: &[f64]) -> f64 {
    let nx = x_vals.len();
    let ny = y_vals.len();
    if nx == 0 || ny == 0 || z.len() < nx * ny {
        return 0.0;
    }
    let xi = x_vals.iter().position(|&v| x <= v).unwrap_or(nx - 1).max(1);
    let yi = y_vals.iter().position(|&v| y <= v).unwrap_or(ny - 1).max(1);
    let x0 = x_vals[xi - 1];
    let x1 = x_vals[xi.min(nx - 1)];
    let y0 = y_vals[yi - 1];
    let y1 = y_vals[yi.min(ny - 1)];
    let tx = if (x1 - x0).abs() < f64::EPSILON { 0.0 } else { ((x - x0) / (x1 - x0)).clamp(0.0, 1.0) };
    let ty = if (y1 - y0).abs() < f64::EPSILON { 0.0 } else { ((y - y0) / (y1 - y0)).clamp(0.0, 1.0) };
    let z00 = z[(yi - 1) * nx + (xi - 1)];
    let z10 = z[(yi - 1) * nx + xi.min(nx - 1)];
    let z01 = z[yi.min(ny - 1) * nx + (xi - 1)];
    let z11 = z[yi.min(ny - 1) * nx + xi.min(nx - 1)];
    let z0 = z00 + tx * (z10 - z00);
    let z1 = z01 + tx * (z11 - z01);
    z0 + ty * (z1 - z0)
}
// #endregion 🔖️Tables
