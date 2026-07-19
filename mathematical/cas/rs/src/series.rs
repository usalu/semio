//! 📶 Truncated Taylor series via repeated symbolic differentiation: `coeffs[k] = f^(k)(at) / k!`.
//! Simpler than direct series-arithmetic (composition/reversion aren't implemented in this pass) but
//! correct, and it reuses `diff` directly rather than duplicating a second derivative table.

use crate::expr::{Constant, Expr, Kind};
use mathematical_number::Integer;

// #region 🔖Series
/// 📶 A truncated Taylor expansion of some expression in `x` around `at`: `sum coeffs[k] * (x-at)^k`,
/// valid to `O((x-at)^(coeffs.len()))`.
#[derive(Clone, Debug, PartialEq)]
pub struct Series {
    pub x: Expr,
    pub at: Expr,
    pub coeffs: Vec<Expr>,
}

fn is_determinate(e: &Expr) -> bool {
    !matches!(e.kind(), Kind::Constant(Constant::Undefined) | Kind::Constant(Constant::ComplexInf))
}

/// 📶 Builds the order-`order` Taylor series of `e` in `x` around `at`; `None` if `e` (or any of its
/// first `order` derivatives) is undefined at `at` — e.g. `e` has a pole there, which this pass doesn't
/// handle as a genuine Laurent series (a documented first-pass limitation).
pub fn taylor_series(e: &Expr, x: &Expr, at: &Expr, order: usize) -> Option<Series> {
    let mut coeffs = Vec::with_capacity(order + 1);
    let mut current = e.clone();
    let mut factorial = Integer::one();
    for k in 0..=order {
        let value_at = crate::visit::subs(&current, x, at);
        if !is_determinate(&value_at) {
            return None;
        }
        let coeff = Expr::mul(vec![value_at, Expr::pow(Expr::from(factorial.clone()), Expr::integer(-1))]);
        coeffs.push(coeff);
        if k < order {
            current = crate::diff::diff(&current, x)?;
        }
        factorial = factorial.mul(&Integer::from_i64((k + 1) as i64));
    }
    Some(Series { x: x.clone(), at: at.clone(), coeffs })
}

/// ↩️ Reconstructs `sum coeffs[k] * (x-at)^k` as a plain `Expr`.
pub fn series_to_expr(s: &Series) -> Expr {
    let terms: Vec<Expr> = s
        .coeffs
        .iter()
        .enumerate()
        .map(|(k, c)| {
            if k == 0 {
                c.clone()
            } else {
                Expr::mul(vec![c.clone(), Expr::pow(s.x.clone() - s.at.clone(), Expr::integer(k as i64))])
            }
        })
        .collect();
    Expr::add(terms)
}

/// 🔎 The lowest-order term with a (structurally) nonzero coefficient, or `None` if every retained
/// coefficient is exactly zero — used by `limits` to read off the leading behavior near `at`.
pub fn leading_term(s: &Series) -> Option<(usize, Expr)> {
    s.coeffs.iter().enumerate().find(|(_, c)| !c.is_zero_literal()).map(|(k, c)| (k, c.clone()))
}

impl Series {
    /// ➕ Term-wise sum, truncated to the shorter of the two operands' orders.
    pub fn add(&self, other: &Self) -> Self {
        let n = self.coeffs.len().min(other.coeffs.len());
        let coeffs = (0..n).map(|k| self.coeffs[k].clone() + other.coeffs[k].clone()).collect();
        Self { x: self.x.clone(), at: self.at.clone(), coeffs }
    }

    /// ✖️ Cauchy product, truncated to the shorter of the two operands' orders.
    pub fn mul(&self, other: &Self) -> Self {
        let n = self.coeffs.len().min(other.coeffs.len());
        let coeffs = (0..n)
            .map(|k| {
                let terms: Vec<Expr> = (0..=k).map(|i| self.coeffs[i].clone() * other.coeffs[k - i].clone()).collect();
                Expr::add(terms)
            })
            .collect();
        Self { x: self.x.clone(), at: self.at.clone(), coeffs }
    }

    pub fn scale(&self, c: &Expr) -> Self {
        Self { x: self.x.clone(), at: self.at.clone(), coeffs: self.coeffs.iter().map(|k| k.clone() * c.clone()).collect() }
    }
}
// #endregion 🔖Series

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fnkind::FnKind;

    #[test]
    fn taylor_series_of_exp_matches_known_coefficients() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Exp, vec![x.clone()]);
        let s = taylor_series(&e, &x, &Expr::integer(0), 4).unwrap();
        // exp(x) = 1 + x + x^2/2 + x^3/6 + x^4/24
        assert_eq!(s.coeffs[0], Expr::integer(1));
        assert_eq!(s.coeffs[1], Expr::integer(1));
        assert_eq!(s.coeffs[2], Expr::from(mathematical_number::Rational::from_i64(1, 2).unwrap()));
        assert_eq!(s.coeffs[3], Expr::from(mathematical_number::Rational::from_i64(1, 6).unwrap()));
    }

    #[test]
    fn taylor_series_of_sin_around_zero_has_no_even_terms() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Sin, vec![x.clone()]);
        let s = taylor_series(&e, &x, &Expr::integer(0), 4).unwrap();
        assert_eq!(s.coeffs[0], Expr::integer(0));
        assert_eq!(s.coeffs[1], Expr::integer(1));
        assert_eq!(s.coeffs[2], Expr::integer(0));
    }

    #[test]
    fn taylor_series_fails_at_a_pole() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(-1));
        assert!(taylor_series(&e, &x, &Expr::integer(0), 2).is_none());
    }

    #[test]
    fn leading_term_skips_zero_coefficients() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Sin, vec![x.clone()]);
        let s = taylor_series(&e, &x, &Expr::integer(0), 3).unwrap();
        let (order, coeff) = leading_term(&s).unwrap();
        assert_eq!(order, 1);
        assert_eq!(coeff, Expr::integer(1));
    }

    #[test]
    fn series_to_expr_round_trips_a_polynomial() {
        let x = Expr::symbol("x");
        let s = Series { x: x.clone(), at: Expr::integer(0), coeffs: vec![Expr::integer(1), Expr::integer(2), Expr::integer(3)] };
        let e = series_to_expr(&s);
        let expected = Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(2), x.clone()]), Expr::mul(vec![Expr::integer(3), Expr::pow(x, Expr::integer(2))])]);
        assert_eq!(e, expected);
    }
}
// #endregion 🔖Tests
