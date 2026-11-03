//! 🌡️ Strain-compatibility N–M curve of a composite column: plane sections, parabola-rectangle concrete without tension (EN 1992-1-1, 3.1.7), elastic perfectly plastic steel and bars, strain plane from the tension edge through pivot B to pivot C. Reference only: the EN 1994-1-1 polygon (6.7.3.2) governs the verdict.

use super::constants::{E_A, E_S};
use super::ColSection;

/// 🌡️ Concrete strain at the end of the parabola, ε_c2 (EN 1992-1-1, Table 3.1, fck ≤ 50 MPa).
pub const EPS_C2: f64 = 0.002;
/// 🌡️ Ultimate concrete strain, ε_cu2 (EN 1992-1-1, Table 3.1, fck ≤ 50 MPa).
pub const EPS_CU: f64 = 0.0035;
const EPS_TENSION_LIMIT: f64 = 0.02;
const STRIPS: usize = 800;

/// 🧱️ A section as horizontal strips from the bottom edge up: steel width, concrete width and two bar layers `(level, area)` about the centroid.
#[derive(Clone, Debug)]
pub struct Strips {
    pub top: f64,
    pub steel: Vec<f64>,
    pub concrete: Vec<f64>,
    pub bars: [(f64, f64); 2],
    pub fyd: f64,
    pub fcd: f64,
    pub fsd: f64,
}

impl Strips {
    /// 🏛️ The equivalent square tube of the polygon (same steel ring, core and bar layers).
    pub fn tube(s: &ColSection) -> Self {
        let top = s.b / 2.0;
        let dy = s.b / STRIPS as f64;
        let (steel, concrete): (Vec<f64>, Vec<f64>) = (0..STRIPS)
            .map(|k| {
                let y = -top + (k as f64 + 0.5) * dy;
                if y.abs() >= top - s.t { (s.b, 0.0) } else { (2.0 * s.t, (s.b - 2.0 * s.t).max(0.0)) }
            })
            .unzip();
        let area = if s.as_m2 > 0.0 { s.as_m2 / 2.0 } else { 0.0 };
        Self { top, steel, concrete, bars: [(s.yr, area), (-s.yr, area)], fyd: s.fyd, fcd: s.fcd, fsd: s.fsd }
    }

    /// 📐️ Strain-compatible `(N, M)` points (compression positive, M about the centroid) from the tension-controlled limit to uniform compression ε_c2.
    pub fn curve(&self, points: usize) -> Vec<(f64, f64)> {
        let half = (points / 2).max(1);
        let h = 2.0 * self.top;
        let dy = h / self.steel.len() as f64;
        let tension = (0..half).map(|k| -EPS_TENSION_LIMIT + EPS_TENSION_LIMIT * k as f64 / half as f64);
        let compression = (0..=half).map(|k| EPS_C2 * k as f64 / half as f64);
        tension
            .chain(compression)
            .map(|bottom| {
                let e_top = if bottom <= 0.0 { EPS_CU } else { EPS_C2 + (EPS_C2 - bottom) * 0.75 };
                let strain = |depth: f64| e_top + (bottom - e_top) * depth / h;
                let (mut n, mut m) = (0.0, 0.0);
                for (k, (steel, concrete)) in self.steel.iter().zip(&self.concrete).enumerate() {
                    let y = -self.top + (k as f64 + 0.5) * dy;
                    let e = strain(self.top - y);
                    let ratio = (e / EPS_C2).clamp(0.0, 1.0);
                    let sigma_c = if e > 0.0 { self.fcd * (1.0 - (1.0 - ratio).powi(2)) } else { 0.0 };
                    let sigma_a = (E_A * e).clamp(-self.fyd, self.fyd);
                    let f = (sigma_c * concrete + sigma_a * steel) * dy;
                    n += f;
                    m += f * y;
                }
                for (y, area) in self.bars {
                    let f = area * (E_S * strain(self.top - y)).clamp(-self.fsd, self.fsd);
                    n += f;
                    m += f * y;
                }
                (n, m)
            })
            .collect()
    }
}

/// 📐️ The largest |M| of a curve (mirrored about M = 0) at the axial force `n`, 0 outside the curve.
pub fn moment_at(curve: &[(f64, f64)], n: f64) -> f64 {
    curve
        .windows(2)
        .filter(|w| w[0].0 != w[1].0 && w[0].0.min(w[1].0) <= n && n <= w[0].0.max(w[1].0))
        .map(|w| (w[0].1 + (n - w[0].0) / (w[1].0 - w[0].0) * (w[1].1 - w[0].1)).abs())
        .fold(0.0, f64::max)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
