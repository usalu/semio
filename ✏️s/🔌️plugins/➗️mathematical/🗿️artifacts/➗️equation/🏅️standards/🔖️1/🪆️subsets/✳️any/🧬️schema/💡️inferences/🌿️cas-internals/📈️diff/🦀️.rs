//! 📈️ Table-driven symbolic differentiation: chain/product/power rules over the canonical tree, plus a
//! per-`FnKind` derivative table for elementary and special functions. Returns `None` — never a wrong
//! answer — whenever a subexpression involves a function without a known derivative rule (`zeta`,
//! `Hyp2F1`, a user-defined function, or an order/degree parameter that itself depends on the
//! differentiation variable).

use crate::cas::expr::{Constant, Expr, Kind};
use crate::cas::fnkind::FnKind;
use number::Rational;

// #region 🔖️Diff
/// 📉️ `d(e)/d(x)`, treating every other symbol as a constant (partial derivative).
pub fn diff(e: &Expr, x: &Expr) -> Option<Expr> {
    match e.kind() {
        Kind::Integer(_) | Kind::Rational(_) | Kind::Constant(_) | Kind::Bool(_) | Kind::RootOf { .. } => Some(Expr::integer(0)),
        Kind::Symbol(_) => Some(if e == x { Expr::integer(1) } else { Expr::integer(0) }),
        Kind::Add(terms) => {
            let mut parts = Vec::with_capacity(terms.len());
            for t in terms {
                parts.push(diff(t, x)?);
            }
            Some(Expr::add(parts))
        }
        Kind::Mul(factors) => {
            let mut sum_terms = Vec::with_capacity(factors.len());
            for i in 0..factors.len() {
                let d = diff(&factors[i], x)?;
                if d.is_zero_literal() {
                    continue;
                }
                let mut rest = factors.clone();
                rest.remove(i);
                let mut term_factors = vec![d];
                term_factors.extend(rest);
                sum_terms.push(Expr::mul(term_factors));
            }
            Some(Expr::add(sum_terms))
        }
        Kind::Pow(base, exp) => diff_pow(base, exp, x),
        Kind::Fn(kind, args) => diff_fn(kind, args, x),
        Kind::Piecewise(cases) => {
            let mut new_cases = Vec::with_capacity(cases.len());
            for (v, c) in cases {
                new_cases.push((diff(v, x)?, c.clone()));
            }
            Some(Expr::from_kind_unchecked(Kind::Piecewise(new_cases)))
        }
        Kind::Rel(..) | Kind::Wild(..) => None,
    }
}

/// 📉️ Multivariate: the vector of partial derivatives w.r.t. each of `vars`, in order; `None` as soon
/// as any single partial derivative is unknown.
pub fn gradient(e: &Expr, vars: &[Expr]) -> Option<Vec<Expr>> {
    vars.iter().map(|v| diff(e, v)).collect()
}

/// 🔗️ Implicit differentiation of `y` (a function of `x`) from the equation `lhs == rhs`: computes
/// `-diff(lhs-rhs, x) / diff(lhs-rhs, y)` (total derivative via the implicit function theorem), treating
/// `y` as an independent symbol in the equation and substituting nothing — the caller is expected to
/// already have `y` appearing explicitly wherever it's implicitly a function of `x`.
pub fn idiff(lhs: &Expr, rhs: &Expr, y: &Expr, x: &Expr) -> Option<Expr> {
    let f = lhs.clone() - rhs.clone();
    let dfdx = diff(&f, x)?;
    let dfdy = diff(&f, y)?;
    if dfdy.is_zero_literal() {
        return None;
    }
    Some(Expr::mul(vec![Expr::integer(-1), dfdx, Expr::pow(dfdy, Expr::integer(-1))]))
}
// #endregion 🔖️Diff

// #region 🔖️PowRule
fn diff_pow(base: &Expr, exp: &Expr, x: &Expr) -> Option<Expr> {
    let exp_depends = crate::cas::visit::contains_symbol(exp, x);
    let base_depends = crate::cas::visit::contains_symbol(base, x);
    if !exp_depends {
        let dbase = diff(base, x)?;
        if dbase.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        let new_exp = Expr::add(vec![exp.clone(), Expr::integer(-1)]);
        return Some(Expr::mul(vec![exp.clone(), Expr::pow(base.clone(), new_exp), dbase]));
    }
    if !base_depends {
        let dexp = diff(exp, x)?;
        if dexp.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        return Some(Expr::mul(vec![Expr::pow(base.clone(), exp.clone()), Expr::func(FnKind::Ln, vec![base.clone()]), dexp]));
    }
    let dbase = diff(base, x)?;
    let dexp = diff(exp, x)?;
    let term1 = Expr::mul(vec![dexp, Expr::func(FnKind::Ln, vec![base.clone()])]);
    let term2 = Expr::mul(vec![exp.clone(), dbase, Expr::pow(base.clone(), Expr::integer(-1))]);
    Some(Expr::mul(vec![Expr::pow(base.clone(), exp.clone()), Expr::add(vec![term1, term2])]))
}
// #endregion 🔖️PowRule

// #region 🔖️FnChainRule
/// 📉️ Per-function chain rule. `ln|u|` differentiates to `u′/u` directly: the generic chain gives the
/// equal `sign(u)·u′/|u|`, which no rational simplification can cancel against `1/u`, so every
/// logarithmic antiderivative (`∫ 1/((x−1)(x+1))`) failed its own derivative check.
fn diff_fn(kind: &FnKind, args: &[Expr], x: &Expr) -> Option<Expr> {
    match kind {
        FnKind::Ln if args.len() == 1 && matches!(args[0].kind(), Kind::Fn(FnKind::Abs, inner) if inner.len() == 1) => {
            let Kind::Fn(_, inner) = args[0].kind() else { return None };
            Some(Expr::mul(vec![diff(&inner[0], x)?, Expr::pow(inner[0].clone(), Expr::integer(-1))]))
        }
        FnKind::UserFn(_) | FnKind::Zeta => None,
        FnKind::BesselJ | FnKind::BesselY | FnKind::BesselI | FnKind::BesselK => diff_bessel(kind, args, x),
        FnKind::LegendreP => diff_legendre(args, x),
        FnKind::ChebyshevT => diff_chebyshev_t(args, x),
        FnKind::ChebyshevU => diff_chebyshev_u(args, x),
        FnKind::HermiteH => diff_hermite(args, x),
        FnKind::LaguerreL => diff_laguerre(args, x),
        _ if args.len() == 1 => {
            let inner_d = diff(&args[0], x)?;
            if inner_d.is_zero_literal() {
                return Some(Expr::integer(0));
            }
            let outer_d = unary_derivative(kind, &args[0])?;
            Some(Expr::mul(vec![outer_d, inner_d]))
        }
        _ => None,
    }
}

fn unary_derivative(kind: &FnKind, arg: &Expr) -> Option<Expr> {
    use FnKind::*;
    let half = Expr::from(Rational::from_i64(1, 2).unwrap());
    let neg_half = Expr::from(Rational::from_i64(-1, 2).unwrap());
    Some(match kind {
        Sin => Expr::func(Cos, vec![arg.clone()]),
        Cos => Expr::mul(vec![Expr::integer(-1), Expr::func(Sin, vec![arg.clone()])]),
        Tan => Expr::add(vec![Expr::integer(1), Expr::pow(Expr::func(Tan, vec![arg.clone()]), Expr::integer(2))]),
        Cot => Expr::mul(vec![Expr::integer(-1), Expr::add(vec![Expr::integer(1), Expr::pow(Expr::func(Cot, vec![arg.clone()]), Expr::integer(2))])]),
        Sec => Expr::mul(vec![Expr::func(Sec, vec![arg.clone()]), Expr::func(Tan, vec![arg.clone()])]),
        Csc => Expr::mul(vec![Expr::integer(-1), Expr::func(Csc, vec![arg.clone()]), Expr::func(Cot, vec![arg.clone()])]),
        Asin => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])]), neg_half),
        Acos => Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])]), neg_half)]),
        Atan => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::pow(arg.clone(), Expr::integer(2))]), Expr::integer(-1)),
        Acot => Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::add(vec![Expr::integer(1), Expr::pow(arg.clone(), Expr::integer(2))]), Expr::integer(-1))]),
        Asec => Expr::pow(Expr::mul(vec![Expr::func(Abs, vec![arg.clone()]), Expr::pow(Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]), half)]), Expr::integer(-1)),
        Acsc => Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::mul(vec![Expr::func(Abs, vec![arg.clone()]), Expr::pow(Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]), half)]), Expr::integer(-1))]),
        Sinh => Expr::func(Cosh, vec![arg.clone()]),
        Cosh => Expr::func(Sinh, vec![arg.clone()]),
        Tanh => Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::func(Tanh, vec![arg.clone()]), Expr::integer(2))])]),
        Asinh => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::pow(arg.clone(), Expr::integer(2))]), neg_half),
        Acosh => Expr::pow(Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]), neg_half),
        Atanh => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])]), Expr::integer(-1)),
        Exp => Expr::func(Exp, vec![arg.clone()]),
        Ln => Expr::pow(arg.clone(), Expr::integer(-1)),
        Abs => Expr::func(Sign, vec![arg.clone()]),
        Sign | Floor | Ceil => Expr::integer(0),
        Gamma => Expr::mul(vec![Expr::func(Gamma, vec![arg.clone()]), Expr::func(Digamma, vec![arg.clone()])]),
        LogGamma => Expr::func(Digamma, vec![arg.clone()]),
        Erf => Expr::mul(vec![Expr::integer(2), Expr::pow(Expr::constant(Constant::Pi), neg_half), Expr::func(Exp, vec![Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])])]),
        Erfc => Expr::mul(vec![Expr::integer(-2), Expr::pow(Expr::constant(Constant::Pi), neg_half), Expr::func(Exp, vec![Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])])]),
        LambertW => {
            let w = Expr::func(LambertW, vec![arg.clone()]);
            Expr::mul(vec![w.clone(), Expr::pow(Expr::mul(vec![arg.clone(), Expr::add(vec![Expr::integer(1), w])]), Expr::integer(-1))])
        }
        _ => return None,
    })
}
// #endregion 🔖️FnChainRule

// #region 🔖️SpecialFunctionRecurrences
fn diff_bessel(kind: &FnKind, args: &[Expr], x: &Expr) -> Option<Expr> {
    let [n, arg] = args else { return None };
    if crate::cas::visit::contains_symbol(n, x) {
        return None;
    }
    let inner_d = diff(arg, x)?;
    if inner_d.is_zero_literal() {
        return Some(Expr::integer(0));
    }
    let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
    let n_plus = Expr::add(vec![n.clone(), Expr::integer(1)]);
    let half = Expr::from(Rational::from_i64(1, 2).unwrap());
    let outer = match kind {
        FnKind::BesselJ => Expr::mul(vec![half, Expr::add(vec![Expr::func(FnKind::BesselJ, vec![n_minus, arg.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::BesselJ, vec![n_plus, arg.clone()])])])]),
        FnKind::BesselY => Expr::mul(vec![half, Expr::add(vec![Expr::func(FnKind::BesselY, vec![n_minus, arg.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::BesselY, vec![n_plus, arg.clone()])])])]),
        FnKind::BesselI => Expr::mul(vec![half, Expr::add(vec![Expr::func(FnKind::BesselI, vec![n_minus, arg.clone()]), Expr::func(FnKind::BesselI, vec![n_plus, arg.clone()])])]),
        FnKind::BesselK => Expr::mul(vec![Expr::integer(-1), half, Expr::add(vec![Expr::func(FnKind::BesselK, vec![n_minus, arg.clone()]), Expr::func(FnKind::BesselK, vec![n_plus, arg.clone()])])]),
        _ => unreachable!("diff_bessel only called for Bessel* kinds"),
    };
    Some(Expr::mul(vec![outer, inner_d]))
}

fn diff_legendre(args: &[Expr], x: &Expr) -> Option<Expr> {
    let [n, arg] = args else { return None };
    if crate::cas::visit::contains_symbol(n, x) {
        return None;
    }
    let inner_d = diff(arg, x)?;
    if inner_d.is_zero_literal() {
        return Some(Expr::integer(0));
    }
    let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
    let denom = Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]);
    let numer = Expr::add(vec![Expr::mul(vec![arg.clone(), Expr::func(FnKind::LegendreP, vec![n.clone(), arg.clone()])]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::LegendreP, vec![n_minus, arg.clone()])])]);
    let outer = Expr::mul(vec![n.clone(), numer, Expr::pow(denom, Expr::integer(-1))]);
    Some(Expr::mul(vec![outer, inner_d]))
}

fn diff_chebyshev_t(args: &[Expr], x: &Expr) -> Option<Expr> {
    let [n, arg] = args else { return None };
    if crate::cas::visit::contains_symbol(n, x) {
        return None;
    }
    let inner_d = diff(arg, x)?;
    if inner_d.is_zero_literal() {
        return Some(Expr::integer(0));
    }
    let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
    let outer = Expr::mul(vec![n.clone(), Expr::func(FnKind::ChebyshevU, vec![n_minus, arg.clone()])]);
    Some(Expr::mul(vec![outer, inner_d]))
}

fn diff_chebyshev_u(args: &[Expr], x: &Expr) -> Option<Expr> {
    let [n, arg] = args else { return None };
    if crate::cas::visit::contains_symbol(n, x) {
        return None;
    }
    let inner_d = diff(arg, x)?;
    if inner_d.is_zero_literal() {
        return Some(Expr::integer(0));
    }
    let n_plus = Expr::add(vec![n.clone(), Expr::integer(1)]);
    let denom = Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]);
    let numer = Expr::add(vec![Expr::mul(vec![n_plus.clone(), Expr::func(FnKind::ChebyshevT, vec![n_plus, arg.clone()])]), Expr::mul(vec![Expr::integer(-1), arg.clone(), Expr::func(FnKind::ChebyshevU, vec![n.clone(), arg.clone()])])]);
    let outer = Expr::mul(vec![numer, Expr::pow(denom, Expr::integer(-1))]);
    Some(Expr::mul(vec![outer, inner_d]))
}

fn diff_hermite(args: &[Expr], x: &Expr) -> Option<Expr> {
    let [n, arg] = args else { return None };
    if crate::cas::visit::contains_symbol(n, x) {
        return None;
    }
    let inner_d = diff(arg, x)?;
    if inner_d.is_zero_literal() {
        return Some(Expr::integer(0));
    }
    let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
    let outer = Expr::mul(vec![Expr::integer(2), n.clone(), Expr::func(FnKind::HermiteH, vec![n_minus, arg.clone()])]);
    Some(Expr::mul(vec![outer, inner_d]))
}

fn diff_laguerre(args: &[Expr], x: &Expr) -> Option<Expr> {
    let [n, arg] = args else { return None };
    if crate::cas::visit::contains_symbol(n, x) {
        return None;
    }
    let inner_d = diff(arg, x)?;
    if inner_d.is_zero_literal() {
        return Some(Expr::integer(0));
    }
    let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
    let diff_l = Expr::add(vec![Expr::func(FnKind::LaguerreL, vec![n.clone(), arg.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::LaguerreL, vec![n_minus, arg.clone()])])]);
    let outer = Expr::mul(vec![n.clone(), diff_l, Expr::pow(arg.clone(), Expr::integer(-1))]);
    Some(Expr::mul(vec![outer, inner_d]))
}
// #endregion 🔖️SpecialFunctionRecurrences

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️diff-unit/🦀️.rs");
// #endregion 🔖️Tests
