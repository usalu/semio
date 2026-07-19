//! 🌱 Bridges the kernel's `Kind::RootOf` leaf (a plain `Vec<Rational>` + index, so the kernel enum
//! never depends on `mathematical_polynomial`) to `mathematical_polynomial::AlgebraicReal` for the
//! numeric queries (sign, refinement, `f64` approximation) that need real algebra to answer.

use crate::expr::{Expr, Kind};
use mathematical_number::{Integer, Natural, Rational};
use mathematical_polynomial::{AlgebraicReal, PolyU};

// #region 🔖Conversion
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
// #endregion 🔖Conversion

// #region 🔖Construction
pub fn root_of_expr(coeffs: Vec<Rational>, index: u32) -> Expr {
    Expr::from_kind_unchecked(Kind::RootOf { coeffs, index })
}

/// 🌱 Builds one `RootOf` expression per real root of `poly` (ascending order).
pub fn real_roots_of(poly: &PolyU<Integer>) -> Vec<Expr> {
    let n_roots = mathematical_polynomial::isolate_real_roots(poly).len();
    let coeffs: Vec<Rational> = poly.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect();
    (0..n_roots as u32).map(|i| root_of_expr(coeffs.clone(), i)).collect()
}
// #endregion 🔖Construction

// #region 🔖Queries
pub fn root_of_to_f64(e: &Expr) -> Option<f64> {
    let Kind::RootOf { coeffs, index } = e.kind() else { return None };
    to_algebraic(coeffs, *index).map(|a| a.to_f64())
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
// #endregion 🔖Queries

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_roots_of_quadratic_gives_two_rootofs() {
        // x^2 - 2, roots +-sqrt(2)
        let p = PolyU::from_coeffs(vec![Integer::from_i64(-2), Integer::from_i64(0), Integer::from_i64(1)]);
        let roots = real_roots_of(&p);
        assert_eq!(roots.len(), 2);
        let vals: Vec<f64> = roots.iter().map(|r| root_of_to_f64(r).unwrap()).collect();
        assert!(vals.iter().any(|v| (v - std::f64::consts::SQRT_2).abs() < 1e-9));
        assert!(vals.iter().any(|v| (v + std::f64::consts::SQRT_2).abs() < 1e-9));
    }

    #[test]
    fn root_of_sign_matches_isolation_interval() {
        let p = PolyU::from_coeffs(vec![Integer::from_i64(-2), Integer::from_i64(0), Integer::from_i64(1)]);
        let roots = real_roots_of(&p);
        let signs: Vec<_> = roots.iter().map(root_of_sign).collect();
        assert!(signs.contains(&Some(std::cmp::Ordering::Less)));
        assert!(signs.contains(&Some(std::cmp::Ordering::Greater)));
    }
}
// #endregion 🔖Tests
