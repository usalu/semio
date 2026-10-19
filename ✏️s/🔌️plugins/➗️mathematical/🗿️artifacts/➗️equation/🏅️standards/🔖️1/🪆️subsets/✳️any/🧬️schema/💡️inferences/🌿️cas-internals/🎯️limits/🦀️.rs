//! 🎯️ Limit evaluation: direct substitution first, then L'Hopital's rule (differentiating a
//! detected numerator/denominator split, not the whole expression) up to a capped depth for `0/0` and
//! `∞/∞` indeterminate forms. `x -> ±∞` reduces to `t -> 0⁺` via the `x = 1/t` substitution.

use crate::cas::expr::{Constant, Expr, Kind};

// #region 🔖️Direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Both,
    FromAbove,
    FromBelow,
}
// #endregion 🔖️Direction

// #region 🔖️Limit
const MAX_LHOPITAL_DEPTH: u32 = 8;

/// 🎯️ A substituted value is a limit only when NO part of it is undefined or complex infinity.
///
/// 🐛️ Only the ROOT used to be checked, so `(2x + 1)/(x + 3)` at `x → ∞` — reduced to `t → 0` —
/// substituted to `(zoo + 1)/(zoo + 3)` and was returned as the limit instead of reaching the series
/// path, which reads `2` off the leading terms.
fn is_determinate(e: &Expr) -> bool {
    [Constant::Undefined, Constant::ComplexInf].into_iter().all(|constant| !crate::cas::visit::contains_symbol(e, &Expr::constant(constant)))
}

fn is_infinite(e: &Expr) -> bool {
    matches!(e.kind(), Kind::Constant(Constant::Inf) | Kind::Constant(Constant::NegInf) | Kind::Constant(Constant::ComplexInf))
}

/// 🎯️ `lim_{x -> at} e`, approaching from `dir` (only meaningful for a finite `at`; infinite limits
/// are always two-sided in the reduced `t -> 0` problem). Returns `None` when the limit can't be
/// resolved by direct substitution or a bounded L'Hopital chain — never a guessed or wrong value.
pub fn limit(e: &Expr, x: &Expr, at: &Expr, dir: Direction) -> Option<Expr> {
    if matches!(at.kind(), Kind::Constant(Constant::Inf)) {
        let t = Expr::symbol("§limit_t");
        let e_t = crate::cas::visit::subs(e, x, &Expr::pow(t.clone(), Expr::integer(-1)));
        return limit(&e_t, &t, &Expr::integer(0), Direction::FromAbove);
    }
    if matches!(at.kind(), Kind::Constant(Constant::NegInf)) {
        let t = Expr::symbol("§limit_t");
        let e_t = crate::cas::visit::subs(e, x, &(Expr::integer(-1) * Expr::pow(t.clone(), Expr::integer(-1))));
        return limit(&e_t, &t, &Expr::integer(0), Direction::FromAbove);
    }

    let direct = crate::cas::visit::subs(e, x, at);
    if is_determinate(&direct) {
        return Some(direct);
    }

    if let Some(series_result) = limit_via_series(e, x, at) {
        return Some(series_result);
    }

    let _ = dir;
    lhopital(e, x, at, 0)
}

/// 📶️ Series-based fast path: expand numerator and denominator around `at` and read off the ratio of
/// leading terms — handles `0/0` forms cleanly without repeated differentiation.
fn limit_via_series(e: &Expr, x: &Expr, at: &Expr) -> Option<Expr> {
    let (num, den) = extract_ratio(e, x);
    let num_series = crate::cas::series::taylor_series(&num, x, at, 6)?;
    let den_series = crate::cas::series::taylor_series(&den, x, at, 6)?;
    let (num_ord, num_coeff) = crate::cas::series::leading_term(&num_series)?;
    let (den_ord, den_coeff) = crate::cas::series::leading_term(&den_series)?;
    match num_ord.cmp(&den_ord) {
        std::cmp::Ordering::Greater => Some(Expr::integer(0)),
        std::cmp::Ordering::Equal => Some(num_coeff * Expr::pow(den_coeff, Expr::integer(-1))),
        std::cmp::Ordering::Less => None,
    }
}

fn lhopital(e: &Expr, x: &Expr, at: &Expr, depth: u32) -> Option<Expr> {
    if depth > MAX_LHOPITAL_DEPTH {
        return None;
    }
    let (num, den) = extract_ratio(e, x);
    let num_at = crate::cas::visit::subs(&num, x, at);
    let den_at = crate::cas::visit::subs(&den, x, at);
    let indeterminate = (num_at.is_zero_literal() && den_at.is_zero_literal()) || (is_infinite(&num_at) && is_infinite(&den_at));
    if !indeterminate {
        if !den_at.is_zero_literal() && is_determinate(&den_at) {
            return Some(num_at * Expr::pow(den_at, Expr::integer(-1)));
        }
        return None;
    }
    let dnum = crate::cas::diff::diff(&num, x)?;
    let dden = crate::cas::diff::diff(&den, x)?;
    let ratio = dnum * Expr::pow(dden, Expr::integer(-1));
    let direct = crate::cas::visit::subs(&ratio, x, at);
    if is_determinate(&direct) {
        return Some(direct);
    }
    lhopital(&ratio, x, at, depth + 1)
}

/// 🌉️ Splits `e` into a `num/den` pair via the poly bridge's rational-function detector (which treats
/// any non-polynomial subtree, including transcendental functions, as its own generator) — falls back
/// to `(e, 1)` when the bridge can't build a ratio at all.
fn extract_ratio(e: &Expr, x: &Expr) -> (Expr, Expr) {
    if let Some((num_m, den_m, map)) = crate::cas::polybridge::as_ratfunc_auto(e) {
        if map.gens.iter().any(|g| g == x) {
            return (crate::cas::polybridge::from_poly(&num_m, &map), crate::cas::polybridge::from_poly(&den_m, &map));
        }
    }
    (e.clone(), Expr::integer(1))
}
// #endregion 🔖️Limit

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️limits-unit/🦀️.rs");
// #endregion 🔖️Tests
