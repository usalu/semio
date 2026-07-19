//! Σ Symbolic summation: closed forms for polynomial sums (via Lagrange interpolation of the partial
//! sum, which is itself always a polynomial one degree higher — this sidesteps deriving Bernoulli
//! numbers explicitly) and geometric sums, plus Fourier coefficients via definite integration.
//! Gosper/Zeilberger-style general hypergeometric summation is a documented follow-up, not attempted.

use crate::expr::{Constant, Expr, Kind};
use crate::fnkind::FnKind;
use mathematical_number::Rational;
use mathematical_polynomial::PolyU;

// #region 🔖ClosedForm
/// Σ `sum_{k=lo}^{hi} e(k)` in closed form, for `e` polynomial or geometric in `n`; `None` otherwise.
pub fn sum_closed(e: &Expr, n: &Expr, lo: &Expr, hi: &Expr) -> Option<Expr> {
    if let Some(p) = crate::polybridge::as_poly_uni(e, n) {
        let s = sum_polynomial_closed_form(&p, n)?;
        let at_hi = crate::visit::subs(&s, n, hi);
        let lo_minus_1 = lo.clone() - Expr::integer(1);
        let at_lo = crate::visit::subs(&s, n, &lo_minus_1);
        return Some(crate::simplify::simplify(&(at_hi - at_lo)));
    }
    sum_geometric(e, n, lo, hi)
}

/// Σ The polynomial `S(N) = sum_{k=0}^{N} p(k)`, recovered by evaluating the true partial sums at
/// `deg(p) + 2` integer points and interpolating (a degree-`d` polynomial's partial sum is always an
/// exact degree-`(d+1)` polynomial in `N`, so this is exact, not an approximation).
fn sum_polynomial_closed_form(p: &PolyU<Rational>, n: &Expr) -> Option<Expr> {
    let d = p.degree().unwrap_or(0);
    let num_points = d + 2;
    let mut cumulative = Rational::zero();
    let mut points = Vec::with_capacity(num_points);
    for k in 0..num_points {
        let k_r = Rational::from_i64(k as i64, 1).unwrap();
        cumulative = cumulative.add(&p.eval(&k_r));
        points.push((k_r, cumulative.clone()));
    }
    let s_poly = PolyU::interpolate(&points)?;
    Some(crate::polybridge::polyu_to_expr(&s_poly, n))
}

/// Σ `sum_{k=lo}^{hi} c * r^k` for `c`, `r` free of `n`, via the closed geometric-series formula
/// (special-cased at `r == 1`, where the sum is just `count * c`).
fn sum_geometric(e: &Expr, n: &Expr, lo: &Expr, hi: &Expr) -> Option<Expr> {
    let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = match e.kind() {
        Kind::Mul(factors) => factors.iter().cloned().partition(|f| !crate::visit::contains_symbol(f, n)),
        _ => (Vec::new(), vec![e.clone()]),
    };
    if var_factors.len() != 1 {
        return None;
    }
    let Kind::Pow(base, exp) = var_factors[0].kind() else { return None };
    if exp != n || crate::visit::contains_symbol(base, n) {
        return None;
    }
    let r = base.clone();
    let c = if const_factors.is_empty() { Expr::integer(1) } else { Expr::mul(const_factors) };
    let count = (hi.clone() - lo.clone()) + Expr::integer(1);
    if r.is_one_literal() {
        return Some(c * count);
    }
    let sum_r = Expr::pow(r.clone(), lo.clone()) * (Expr::pow(r.clone(), count) - Expr::integer(1)) * Expr::pow(r - Expr::integer(1), Expr::integer(-1));
    Some(crate::simplify::cancel(&(c * sum_r)))
}
// #endregion 🔖ClosedForm

// #region 🔖Fourier
/// 🌊 Fourier coefficients `(a_n, b_n)` of `f` on `[-L, L]` (`a_0` at index 0, `b_0` fixed at `0` since
/// the sine term vanishes there), via `integrate_definite` — correct whenever the underlying integrals
/// resolve, `None` for the whole pair otherwise (never a partial/wrong coefficient list).
pub fn fourier_coefficients(f: &Expr, x: &Expr, half_period: &Expr, n_terms: usize) -> Option<(Vec<Expr>, Vec<Expr>)> {
    let l = half_period.clone();
    let neg_l = Expr::integer(-1) * l.clone();
    let mut a = Vec::with_capacity(n_terms + 1);
    let mut b = Vec::with_capacity(n_terms + 1);
    for n in 0..=n_terms {
        let angle = Expr::mul(vec![Expr::integer(n as i64), Expr::constant(Constant::Pi), x.clone()]) * Expr::pow(l.clone(), Expr::integer(-1));
        let cos_term = Expr::func(FnKind::Cos, vec![angle.clone()]);
        let a_n = crate::integrate::integrate_definite(&(f.clone() * cos_term), x, &neg_l, &l)?;
        a.push(crate::simplify::cancel(&(a_n * Expr::pow(l.clone(), Expr::integer(-1)))));
        if n > 0 {
            let sin_term = Expr::func(FnKind::Sin, vec![angle]);
            let b_n = crate::integrate::integrate_definite(&(f.clone() * sin_term), x, &neg_l, &l)?;
            b.push(crate::simplify::cancel(&(b_n * Expr::pow(l.clone(), Expr::integer(-1)))));
        } else {
            b.push(Expr::integer(0));
        }
    }
    Some((a, b))
}
// #endregion 🔖Fourier

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_k_from_1_to_n_is_gauss_formula() {
        let n = Expr::symbol("n");
        let k = Expr::symbol("k");
        // sum_{k=1}^{n} k -- but sum_closed evaluates a polynomial in the SAME variable used for the
        // bound substitution, so pass `k` itself as both the summand's variable and the closed-form target.
        let result = sum_closed(&k, &k, &Expr::integer(1), &n).unwrap();
        let expected = crate::simplify::expand(&(n.clone() * (n.clone() + Expr::integer(1)) * Expr::from(Rational::from_i64(1, 2).unwrap())));
        assert_eq!(crate::simplify::expand(&result), expected);
    }

    #[test]
    fn sum_of_k_squared_matches_known_hand_values() {
        let k = Expr::symbol("k");
        // sum_{k=1}^{3} k^2 = 1+4+9 = 14
        let e = Expr::pow(k.clone(), Expr::integer(2));
        let result = sum_closed(&e, &k, &Expr::integer(1), &Expr::integer(3)).unwrap();
        assert_eq!(result, Expr::integer(14));
    }

    #[test]
    fn sum_geometric_series_hand_case() {
        let k = Expr::symbol("k");
        // sum_{k=0}^{3} 2^k = 1+2+4+8 = 15
        let e = Expr::pow(Expr::integer(2), k.clone());
        let result = sum_closed(&e, &k, &Expr::integer(0), &Expr::integer(3)).unwrap();
        assert_eq!(crate::simplify::simplify(&result), Expr::integer(15));
    }

    #[test]
    fn fourier_coefficients_of_a_polynomial_smoke_test() {
        let x = Expr::symbol("x");
        let l = Expr::constant(Constant::Pi);
        let f = x.clone();
        let result = fourier_coefficients(&f, &x, &l, 2);
        assert!(result.is_some());
    }
}
// #endregion 🔖Tests
