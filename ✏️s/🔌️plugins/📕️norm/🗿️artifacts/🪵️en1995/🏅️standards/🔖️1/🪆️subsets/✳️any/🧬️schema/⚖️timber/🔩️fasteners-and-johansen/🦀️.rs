//! 🔩️ Fasteners of the EN 1995 timber checks: the characteristic embedment and yield values, the Johansen load-carrying capacities, the effective number and the spacing minima. See [`super`].

use super::*;

pub fn residual_bh(b_m: f64, h_m: f64, d_ef: f64) -> (f64, f64) {
    ((b_m - 2.0 * d_ef).max(0.0), (h_m - 2.0 * d_ef).max(0.0))
}

/// 📏 Tables 8.2 – 8.5 minima `(a1, a3,t, a4,t)` in metres for a connection with the force parallel to the grain.
pub fn spacing_minima(c: &TimberConnection) -> (f64, f64, f64) {
    use semio_s_artifact_norm_en1995_model::FastenerType as F;
    let kind = match c.fastener_type { F::Nail | F::Screw => "nail", F::Bolt => "bolt", F::Dowel => "dowel" };
    let rho = properties_for_class(c.strength_class.id()).map(|p| p.rho_k).unwrap_or(350.0);
    let table = spacing_table(kind, c.diameter_m * 1000.0, 0.0, rho);
    let pick = |name: &str| table.iter().find(|r| r.0 == name).map(|r| r.1).unwrap_or(0.0) / 1000.0;
    (pick("a1"), pick("a3t"), pick("a4t"))
}
