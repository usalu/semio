//! 🌱️ Bridges the kernel's `Kind::RootOf` leaf (a plain `Vec<Rational>` + index, so the kernel enum
//! never depends on `crate::polynomial`) to `crate::polynomial::AlgebraicReal` for the
//! numeric queries (sign, refinement, `f64` approximation) that need real algebra to answer.

use crate::cas::expr::{Expr, Kind};
use crate::polynomial::algebraic::AlgebraicReal;
use crate::polynomial::univariate::PolyU;
use number::{Integer, Natural, Rational};

// #region 🔖️Conversion
fn clear_denominators(coeffs: &[Rational]) -> PolyU<Integer> {
    let denom_lcm = coeffs.iter().fold(Natural::one(), |acc, c| {
        let g = acc.gcd(c.denom());
        acc.mul(c.denom()).div_rem(&g).0
    });
    let scale = Rational::from_integer(Integer::from_natural(denom_lcm));
    PolyU::from_coeffs(coeffs.iter().map(|c| c.mul(&scale).trunc()).collect())
}

fn to_algebraic(coeffs: &[Rational], index: u32) -> Option<AlgebraicReal> {
    let int_poly = clear_denominators(coeffs);
    AlgebraicReal::root_of(&int_poly, index as usize)
}
// #endregion 🔖️Conversion

// #region 🔖️Construction
pub fn root_of_expr(coeffs: Vec<Rational>, index: u32) -> Expr {
    Expr::from_kind_unchecked(Kind::RootOf { coeffs, index })
}

/// 🌱️ Builds one `RootOf` expression per real root of `poly` (ascending order).
pub fn real_roots_of(poly: &PolyU<Integer>) -> Vec<Expr> {
    let n_roots = crate::polynomial::roots::isolate_real_roots(poly).len();
    let coeffs: Vec<Rational> = poly.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect();
    (0..n_roots as u32).map(|i| root_of_expr(coeffs.clone(), i)).collect()
}
// #endregion 🔖️Construction

// #region 🔖️Queries
pub fn root_of_to_f64(e: &Expr) -> Option<f64> {
    let Kind::RootOf { coeffs, index } = e.kind() else { return None };
    let mut a = to_algebraic(coeffs, *index)?;
    a.refine(&Rational::from_i64(1, 1_000_000_000_000_000).unwrap());
    Some(a.to_f64())
}

pub fn root_of_sign(e: &Expr) -> Option<std::cmp::Ordering> {
    let Kind::RootOf { coeffs, index } = e.kind() else { return None };
    to_algebraic(coeffs, *index)?.sign()
}

pub fn root_of_refine(e: &Expr, width: &Rational) -> Option<(Rational, Rational)> {
    let Kind::RootOf { coeffs, index } = e.kind() else { return None };
    let mut a = to_algebraic(coeffs, *index)?;
    a.refine(width);
    Some(a.interval())
}
// #endregion 🔖️Queries

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️rootof-unit/🦀️.rs");
// #endregion 🔖️Tests
