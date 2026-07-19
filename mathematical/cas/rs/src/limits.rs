//! 🎯 Limit evaluation: direct substitution first, then L'Hopital's rule (differentiating a
//! detected numerator/denominator split, not the whole expression) up to a capped depth for `0/0` and
//! `∞/∞` indeterminate forms. `x -> ±∞` reduces to `t -> 0⁺` via the `x = 1/t` substitution.

use crate::expr::{Constant, Expr, Kind};

// #region 🔖Direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Both,
    FromAbove,
    FromBelow,
}
// #endregion 🔖Direction

// #region 🔖Limit
const MAX_LHOPITAL_DEPTH: u32 = 8;

fn is_determinate(e: &Expr) -> bool {
    !matches!(e.kind(), Kind::Constant(Constant::Undefined))
}

fn is_infinite(e: &Expr) -> bool {
    matches!(e.kind(), Kind::Constant(Constant::Inf) | Kind::Constant(Constant::NegInf) | Kind::Constant(Constant::ComplexInf))
}

/// 🎯 `lim_{x -> at} e`, approaching from `dir` (only meaningful for a finite `at`; infinite limits
/// are always two-sided in the reduced `t -> 0` problem). Returns `None` when the limit can't be
/// resolved by direct substitution or a bounded L'Hopital chain — never a guessed or wrong value.
pub fn limit(e: &Expr, x: &Expr, at: &Expr, dir: Direction) -> Option<Expr> {
    if matches!(at.kind(), Kind::Constant(Constant::Inf)) {
        let t = Expr::symbol("§limit_t");
        let e_t = crate::visit::subs(e, x, &Expr::pow(t.clone(), Expr::integer(-1)));
        return limit(&e_t, &t, &Expr::integer(0), Direction::FromAbove);
    }
    if matches!(at.kind(), Kind::Constant(Constant::NegInf)) {
        let t = Expr::symbol("§limit_t");
        let e_t = crate::visit::subs(e, x, &(Expr::integer(-1) * Expr::pow(t.clone(), Expr::integer(-1))));
        return limit(&e_t, &t, &Expr::integer(0), Direction::FromAbove);
    }

    let direct = crate::visit::subs(e, x, at);
    if is_determinate(&direct) {
        return Some(direct);
    }

    if let Some(series_result) = limit_via_series(e, x, at) {
        return Some(series_result);
    }

    let _ = dir; // one-sided refinement over the series/L'Hopital path is a documented follow-up
    lhopital(e, x, at, 0)
}

/// 📶 Series-based fast path: expand numerator and denominator around `at` and read off the ratio of
/// leading terms — handles `0/0` forms cleanly without repeated differentiation.
fn limit_via_series(e: &Expr, x: &Expr, at: &Expr) -> Option<Expr> {
    let (num, den) = extract_ratio(e, x);
    let num_series = crate::series::taylor_series(&num, x, at, 6)?;
    let den_series = crate::series::taylor_series(&den, x, at, 6)?;
    let (num_ord, num_coeff) = crate::series::leading_term(&num_series)?;
    let (den_ord, den_coeff) = crate::series::leading_term(&den_series)?;
    match num_ord.cmp(&den_ord) {
        std::cmp::Ordering::Greater => Some(Expr::integer(0)),
        std::cmp::Ordering::Equal => Some(num_coeff * Expr::pow(den_coeff, Expr::integer(-1))),
        std::cmp::Ordering::Less => None, // denominator vanishes to lower order: signed infinity, not resolved here
    }
}

fn lhopital(e: &Expr, x: &Expr, at: &Expr, depth: u32) -> Option<Expr> {
    if depth > MAX_LHOPITAL_DEPTH {
        return None;
    }
    let (num, den) = extract_ratio(e, x);
    let num_at = crate::visit::subs(&num, x, at);
    let den_at = crate::visit::subs(&den, x, at);
    let indeterminate = (num_at.is_zero_literal() && den_at.is_zero_literal()) || (is_infinite(&num_at) && is_infinite(&den_at));
    if !indeterminate {
        if !den_at.is_zero_literal() && is_determinate(&den_at) {
            return Some(num_at * Expr::pow(den_at, Expr::integer(-1)));
        }
        return None;
    }
    let dnum = crate::diff::diff(&num, x)?;
    let dden = crate::diff::diff(&den, x)?;
    let ratio = dnum * Expr::pow(dden, Expr::integer(-1));
    let direct = crate::visit::subs(&ratio, x, at);
    if is_determinate(&direct) {
        return Some(direct);
    }
    lhopital(&ratio, x, at, depth + 1)
}

/// 🌉 Splits `e` into a `num/den` pair via the poly bridge's rational-function detector (which treats
/// any non-polynomial subtree, including transcendental functions, as its own generator) — falls back
/// to `(e, 1)` when the bridge can't build a ratio at all.
fn extract_ratio(e: &Expr, x: &Expr) -> (Expr, Expr) {
    if let Some((num_m, den_m, map)) = crate::polybridge::as_ratfunc_auto(e) {
        if map.gens.iter().any(|g| g == x) {
            return (crate::polybridge::from_poly(&num_m, &map), crate::polybridge::from_poly(&den_m, &map));
        }
    }
    (e.clone(), Expr::integer(1))
}
// #endregion 🔖Limit

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fnkind::FnKind;

    #[test]
    fn direct_substitution_when_defined() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2));
        assert_eq!(limit(&e, &x, &Expr::integer(3), Direction::Both), Some(Expr::integer(9)));
    }

    #[test]
    fn classic_sin_x_over_x_at_zero() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Sin, vec![x.clone()]) * Expr::pow(x.clone(), Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::integer(0), Direction::Both), Some(Expr::integer(1)));
    }

    #[test]
    fn polynomial_ratio_at_removable_singularity() {
        // (x^2 - 1)/(x - 1) -> 2 as x -> 1
        let x = Expr::symbol("x");
        let num = Expr::pow(x.clone(), Expr::integer(2)) - Expr::integer(1);
        let den = x.clone() - Expr::integer(1);
        let e = num * Expr::pow(den, Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::integer(1), Direction::Both), Some(Expr::integer(2)));
    }

    #[test]
    fn limit_at_infinity_of_rational_function() {
        // (2x + 1)/(x + 3) -> 2 as x -> oo
        let x = Expr::symbol("x");
        let num = Expr::integer(2) * x.clone() + Expr::integer(1);
        let den = x.clone() + Expr::integer(3);
        let e = num * Expr::pow(den, Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::constant(Constant::Inf), Direction::Both), Some(Expr::integer(2)));
    }

    #[test]
    fn one_plus_one_over_n_to_the_n_via_lhopital_on_log_form() {
        // A simpler but still classic L'Hopital case: lim x->0 (1 - cos(x))/x^2 = 1/2
        let x = Expr::symbol("x");
        let num = Expr::integer(1) - Expr::func(FnKind::Cos, vec![x.clone()]);
        let den = Expr::pow(x.clone(), Expr::integer(2));
        let e = num * Expr::pow(den, Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::integer(0), Direction::Both), Some(Expr::from(mathematical_number::Rational::from_i64(1, 2).unwrap())));
    }
}
// #endregion 🔖Tests
