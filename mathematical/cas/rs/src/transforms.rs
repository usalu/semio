//! 🔄 Laplace transforms: linearity (`Add`, constant-factor pull-out) plus a table for `t^n`,
//! `exp/sin/cos/sinh/cosh(a*t)`. The inverse transform is scoped to its exact mirror image — linearity
//! plus the `1/(s-a) -> e^{at}` pattern — rather than a general rational-function inverse (that would
//! need to reuse `simplify::apart` and re-derive sign/branch handling per term; a documented follow-up).

use crate::expr::{Expr, Kind};
use crate::fnkind::FnKind;
use mathematical_number::Integer;

// #region 🔖Laplace
pub fn laplace_transform(f: &Expr, t: &Expr, s: &Expr) -> Option<Expr> {
    if !crate::visit::contains_symbol(f, t) {
        return Some(f.clone() * Expr::pow(s.clone(), Expr::integer(-1)));
    }
    if let Kind::Add(terms) = f.kind() {
        let mut parts = Vec::with_capacity(terms.len());
        for term in terms {
            parts.push(laplace_transform(term, t, s)?);
        }
        return Some(Expr::add(parts));
    }
    if let Kind::Mul(factors) = f.kind() {
        let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|fac| !crate::visit::contains_symbol(fac, t));
        if !const_factors.is_empty() && !var_factors.is_empty() {
            let rest = Expr::mul(var_factors);
            return Some(Expr::mul(const_factors) * laplace_transform(&rest, t, s)?);
        }
    }
    if f == t {
        return Some(Expr::pow(s.clone(), Expr::integer(-2)));
    }
    if let Kind::Pow(base, exp) = f.kind() {
        if base == t {
            if let Kind::Integer(n) = exp.kind() {
                if let Some(ev) = n.to_i64() {
                    if ev >= 0 {
                        return Some(Expr::from(factorial(ev)) * Expr::pow(s.clone(), Expr::integer(-(ev + 1))));
                    }
                }
            }
        }
    }
    if let Kind::Fn(kind, args) = f.kind() {
        if args.len() == 1 {
            if let Some(a) = linear_coeff_in(&args[0], t) {
                return laplace_table(kind, &a, s);
            }
        }
    }
    None
}

fn factorial(n: i64) -> Integer {
    let mut result = Integer::one();
    for k in 1..=n {
        result = result.mul(&Integer::from_i64(k));
    }
    result
}

/// 🔍 `arg == a * t` for some `a` free of `t`; `None` for anything with a constant offset or nonlinear
/// dependence (this pass's table entries only need the pure-scaling case).
fn linear_coeff_in(arg: &Expr, t: &Expr) -> Option<Expr> {
    if arg == t {
        return Some(Expr::integer(1));
    }
    if let Kind::Mul(factors) = arg.kind() {
        let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|f| f != t);
        if var_factors.len() == 1 && var_factors[0] == *t {
            return Some(Expr::mul(const_factors));
        }
    }
    None
}

fn laplace_table(kind: &FnKind, a: &Expr, s: &Expr) -> Option<Expr> {
    use FnKind::*;
    let s2 = Expr::pow(s.clone(), Expr::integer(2));
    let a2 = Expr::pow(a.clone(), Expr::integer(2));
    Some(match kind {
        Exp => Expr::pow(s.clone() - a.clone(), Expr::integer(-1)),
        Sin => a.clone() * Expr::pow(s2 + a2, Expr::integer(-1)),
        Cos => s.clone() * Expr::pow(s2 + a2, Expr::integer(-1)),
        Sinh => a.clone() * Expr::pow(s2 - a2, Expr::integer(-1)),
        Cosh => s.clone() * Expr::pow(s2 - a2, Expr::integer(-1)),
        _ => return None,
    })
}
// #endregion 🔖Laplace

// #region 🔖InverseLaplace
pub fn inverse_laplace_transform(f: &Expr, s: &Expr, t: &Expr) -> Option<Expr> {
    if let Kind::Add(terms) = f.kind() {
        let mut parts = Vec::with_capacity(terms.len());
        for term in terms {
            parts.push(inverse_laplace_transform(term, s, t)?);
        }
        return Some(Expr::add(parts));
    }
    if let Kind::Mul(factors) = f.kind() {
        let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|fac| !crate::visit::contains_symbol(fac, s));
        if !const_factors.is_empty() && !var_factors.is_empty() {
            let rest = Expr::mul(var_factors);
            return Some(Expr::mul(const_factors) * inverse_laplace_transform(&rest, s, t)?);
        }
    }
    if let Kind::Pow(base, exp) = f.kind() {
        if matches!(exp.kind(), Kind::Integer(n) if *n == Integer::from_i64(-1)) {
            if let Some(a) = extract_shift(base, s) {
                return Some(Expr::func(FnKind::Exp, vec![a * t.clone()]));
            }
        }
    }
    None
}

/// 🔍 `e == s - a` (or bare `s`, giving `a = 0`); `None` otherwise.
fn extract_shift(e: &Expr, s: &Expr) -> Option<Expr> {
    if e == s {
        return Some(Expr::integer(0));
    }
    if let Kind::Add(terms) = e.kind() {
        if terms.len() != 2 {
            return None;
        }
        let (s_terms, rest): (Vec<Expr>, Vec<Expr>) = terms.iter().cloned().partition(|term| term == s);
        if s_terms.len() == 1 {
            return Some(Expr::integer(-1) * Expr::add(rest));
        }
    }
    None
}
// #endregion 🔖InverseLaplace

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laplace_of_t_to_the_n() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        // L{t^2} = 2/s^3
        let e = Expr::pow(t.clone(), Expr::integer(2));
        let result = laplace_transform(&e, &t, &s).unwrap();
        assert_eq!(result, Expr::integer(2) * Expr::pow(s, Expr::integer(-3)));
    }

    #[test]
    fn laplace_of_exp() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        let e = Expr::func(FnKind::Exp, vec![Expr::integer(3) * t.clone()]);
        let result = laplace_transform(&e, &t, &s).unwrap();
        assert_eq!(result, Expr::pow(s - Expr::integer(3), Expr::integer(-1)));
    }

    #[test]
    fn laplace_linearity() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        let e = Expr::integer(2) * t.clone() + Expr::integer(3);
        let result = laplace_transform(&e, &t, &s).unwrap();
        let expected = Expr::integer(2) * Expr::pow(s.clone(), Expr::integer(-2)) + Expr::integer(3) * Expr::pow(s, Expr::integer(-1));
        assert_eq!(result, expected);
    }

    #[test]
    fn laplace_and_inverse_round_trip_for_exp() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        let e = Expr::func(FnKind::Exp, vec![Expr::integer(-2) * t.clone()]);
        let transformed = laplace_transform(&e, &t, &s).unwrap();
        let back = inverse_laplace_transform(&transformed, &s, &t).unwrap();
        assert_eq!(back, e);
    }
}
// #endregion 🔖Tests
