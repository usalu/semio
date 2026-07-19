//! ∫ Symbolic integration: linearity, a bare-variable antiderivative table, rational functions
//! (polynomial part + partial fractions, with the classical `ln`/`atan` split for irreducible
//! quadratic factors), `u`-substitution, and integration by parts (LIATE-ordered, depth-capped).
//! Returns `None` — never a wrong antiderivative — whenever no strategy applies.

use crate::expr::{Constant, Expr, Kind};
use crate::fnkind::FnKind;
use mathematical_number::{Integer, Rational};
use mathematical_polynomial::PolyU;

// #region 🔖Integrate
const MAX_BY_PARTS_DEPTH: u32 = 3;

pub fn integrate(e: &Expr, x: &Expr) -> Option<Expr> {
    integrate_depth(e, x, 0)
}

fn integrate_depth(e: &Expr, x: &Expr, depth: u32) -> Option<Expr> {
    if !crate::visit::contains_symbol(e, x) {
        return Some(e.clone() * x.clone());
    }
    if let Kind::Add(terms) = e.kind() {
        let mut parts = Vec::with_capacity(terms.len());
        for t in terms {
            parts.push(integrate_depth(t, x, depth)?);
        }
        return Some(Expr::add(parts));
    }
    if let Kind::Mul(factors) = e.kind() {
        let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|f| !crate::visit::contains_symbol(f, x));
        if !const_factors.is_empty() && !var_factors.is_empty() {
            let rest = Expr::mul(var_factors);
            let integral = integrate_depth(&rest, x, depth)?;
            return Some(Expr::mul(const_factors) * integral);
        }
    }
    if e == x {
        return Some(Expr::pow(x.clone(), Expr::integer(2)) * Expr::from(Rational::from_i64(1, 2).unwrap()));
    }
    if let Kind::Pow(base, exp) = e.kind() {
        if base == x {
            if let Kind::Integer(n) = exp.kind() {
                if let Some(ev) = n.to_i64() {
                    if ev != -1 {
                        return Some(Expr::pow(x.clone(), Expr::integer(ev + 1)) * Expr::from(Rational::new(Integer::one(), Integer::from_i64(ev + 1)).expect("ev + 1 != 0")));
                    }
                    return Some(Expr::func(FnKind::Ln, vec![Expr::func(FnKind::Abs, vec![x.clone()])]));
                }
            }
        }
    }
    if let Kind::Fn(kind, args) = e.kind() {
        if args.len() == 1 && &args[0] == x {
            if let Some(result) = bare_antiderivative(kind, x) {
                return Some(result);
            }
        }
    }
    if let Some(result) = integrate_rational(e, x) {
        return Some(result);
    }
    if let Some(result) = integrate_by_substitution(e, x) {
        return Some(result);
    }
    if depth < MAX_BY_PARTS_DEPTH {
        if let Some(result) = integrate_by_parts(e, x, depth) {
            return Some(result);
        }
    }
    None
}

fn bare_antiderivative(kind: &FnKind, x: &Expr) -> Option<Expr> {
    use FnKind::*;
    Some(match kind {
        Sin => Expr::integer(-1) * Expr::func(Cos, vec![x.clone()]),
        Cos => Expr::func(Sin, vec![x.clone()]),
        Exp => Expr::func(Exp, vec![x.clone()]),
        Sinh => Expr::func(Cosh, vec![x.clone()]),
        Cosh => Expr::func(Sinh, vec![x.clone()]),
        Ln => x.clone() * Expr::func(Ln, vec![x.clone()]) - x.clone(),
        Tan => Expr::integer(-1) * Expr::func(Ln, vec![Expr::func(Abs, vec![Expr::func(Cos, vec![x.clone()])])]),
        _ => return None,
    })
}

/// 🎯 `lim_{x -> x0} (x - x0) * e` — the residue at a *simple* pole; higher-order poles are a
/// documented gap (the underlying `limit` honestly returns `None` rather than a wrong value for those).
pub fn residue(e: &Expr, x: &Expr, x0: &Expr) -> Option<Expr> {
    let shifted = (x.clone() - x0.clone()) * e.clone();
    crate::limits::limit(&shifted, x, x0, crate::limits::Direction::Both)
}

/// ∫ Definite integral via the fundamental theorem: `antideriv(hi) - antideriv(lo)`, with infinite
/// bounds routed through `limit`.
pub fn integrate_definite(e: &Expr, x: &Expr, lo: &Expr, hi: &Expr) -> Option<Expr> {
    let antideriv = integrate(e, x)?;
    let value_at = |bound: &Expr| -> Option<Expr> {
        if matches!(bound.kind(), Kind::Constant(Constant::Inf) | Kind::Constant(Constant::NegInf)) {
            crate::limits::limit(&antideriv, x, bound, crate::limits::Direction::Both)
        } else {
            let v = crate::visit::subs(&antideriv, x, bound);
            if matches!(v.kind(), Kind::Constant(Constant::Undefined) | Kind::Constant(Constant::ComplexInf)) {
                None
            } else {
                Some(v)
            }
        }
    };
    let at_hi = value_at(hi)?;
    let at_lo = value_at(lo)?;
    Some(at_hi - at_lo)
}
// #endregion 🔖Integrate

// #region 🔖RationalFunction
fn integrate_rational(e: &Expr, x: &Expr) -> Option<Expr> {
    let (num_m, den_m, map) = crate::polybridge::as_ratfunc_auto(e)?;
    if map.gens.len() != 1 || map.gens[0] != *x {
        return None;
    }
    let num = crate::polybridge::polym_to_polyu(&num_m, 0)?;
    let den = crate::polybridge::polym_to_polyu(&den_m, 0)?;
    if den.is_zero() {
        return None;
    }
    integrate_ratfunc(&num, &den, x)
}

fn integrate_ratfunc(num: &PolyU<Rational>, den: &PolyU<Rational>, x: &Expr) -> Option<Expr> {
    let (poly_part, remainder) = num.div_rem(den);
    let mut result_terms = Vec::new();
    for (i, c) in poly_part.coeffs().iter().enumerate() {
        if c.is_zero() {
            continue;
        }
        let new_exp = i as i64 + 1;
        result_terms.push(Expr::from(c.clone()) * Expr::pow(x.clone(), Expr::integer(new_exp)) * Expr::from(Rational::new(Integer::one(), Integer::from_i64(new_exp)).expect("new_exp != 0")));
    }
    if remainder.is_zero() {
        return Some(Expr::add(result_terms));
    }
    let den_expr = crate::polybridge::polyu_to_expr(den, x);
    let rem_expr = crate::polybridge::polyu_to_expr(&remainder, x);
    let rational_part = rem_expr * Expr::pow(den_expr, Expr::integer(-1));
    let apart_result = crate::simplify::apart(&rational_part, x);
    let terms: Vec<Expr> = match apart_result.kind() {
        Kind::Add(ts) => ts.clone(),
        _ => vec![apart_result.clone()],
    };
    for term in &terms {
        result_terms.push(integrate_partial_fraction_term(term, x)?);
    }
    Some(Expr::add(result_terms))
}

fn integrate_partial_fraction_term(term: &Expr, x: &Expr) -> Option<Expr> {
    if !crate::visit::contains_symbol(term, x) {
        return Some(term.clone() * x.clone());
    }
    let Kind::Mul(factors) = term.kind() else { return None };
    let pow_idx = factors.iter().position(|f| matches!(f.kind(), Kind::Pow(_, e) if matches!(e.kind(), Kind::Integer(n) if n.is_negative())))?;
    let Kind::Pow(factor_base, neg_exp) = factors[pow_idx].kind() else { unreachable!() };
    let Kind::Integer(neg_n) = neg_exp.kind() else { return None };
    let j = -neg_n.to_i64()?;
    let numerator = Expr::mul(factors.iter().enumerate().filter(|&(i, _)| i != pow_idx).map(|(_, f)| f.clone()).collect());
    integrate_over_factor_power(&numerator, factor_base, j, x)
}

fn integrate_over_factor_power(numerator: &Expr, factor: &Expr, j: i64, x: &Expr) -> Option<Expr> {
    let fp = crate::polybridge::as_poly_uni(factor, x)?;
    match fp.degree().unwrap_or(0) {
        1 => {
            let c1 = fp.coeff(1);
            let c0 = fp.coeff(0);
            let root = Expr::from(c0.neg().div(&c1)?);
            if j == 1 {
                Some(numerator.clone() * Expr::func(FnKind::Ln, vec![Expr::func(FnKind::Abs, vec![x.clone() - root])]))
            } else {
                let exp = 1 - j;
                Some(numerator.clone() * Expr::pow(x.clone() - root, Expr::integer(exp)) * Expr::from(Rational::new(Integer::one(), Integer::from_i64(exp)).expect("j != 1 here")))
            }
        }
        2 if j == 1 => integrate_linear_over_irreducible_quadratic(numerator, &fp, x),
        _ => None,
    }
}

/// ∫ `(p*x + q) / (a*x^2 + b*x + c) dx` for an irreducible quadratic (`c/a - (b/a)^2/4 > 0`), via the
/// classical split into a logarithmic part (from the derivative-matching half) and an `atan` part
/// (from completing the square).
fn integrate_linear_over_irreducible_quadratic(numerator: &Expr, fp: &PolyU<Rational>, x: &Expr) -> Option<Expr> {
    let np = crate::polybridge::as_poly_uni(numerator, x)?;
    if np.degree().unwrap_or(0) > 1 {
        return None;
    }
    let p = np.coeff(1);
    let q = np.coeff(0);
    let a = fp.coeff(2);
    let b = fp.coeff(1);
    let c = fp.coeff(0);
    let b_m = b.div(&a)?;
    let c_m = c.div(&a)?;
    let half = Rational::from_i64(1, 2).unwrap();
    let p_half = p.mul(&half);
    let remainder_const = q.sub(&p.mul(&b_m).mul(&half));
    let monic_factor_expr = crate::polybridge::polyu_to_expr(&PolyU::from_coeffs(vec![c_m.clone(), b_m.clone(), Rational::one()]), x);
    let d = c_m.sub(&b_m.mul(&b_m).mul(&Rational::from_i64(1, 4).unwrap()));
    if d.is_zero() || d.numer().is_negative() {
        return None; // not actually irreducible over R -- a documented refinement gap, not a wrong answer
    }
    let sqrt_d_expr = crate::solve::sqrt_of_rational(&d);
    let shift = b_m.mul(&half);
    let atan_arg = (x.clone() + Expr::from(shift)) * Expr::pow(sqrt_d_expr.clone(), Expr::integer(-1));
    let mut terms = Vec::new();
    if !p_half.is_zero() {
        terms.push(Expr::from(p_half) * Expr::func(FnKind::Ln, vec![monic_factor_expr]));
    }
    if !remainder_const.is_zero() {
        terms.push(Expr::from(remainder_const) * Expr::pow(sqrt_d_expr, Expr::integer(-1)) * Expr::func(FnKind::Atan, vec![atan_arg]));
    }
    let inv_a = Expr::from(Rational::one().div(&a)?);
    Some(inv_a * Expr::add(terms))
}
// #endregion 🔖RationalFunction

// #region 🔖Substitution
/// 🔄 `u`-substitution: for `e = f(inner) * rest`, if `rest / inner'` is free of `x` (a constant
/// multiplier), the integral is that constant times `F(inner)` (`F` from a small antiderivative table).
fn integrate_by_substitution(e: &Expr, x: &Expr) -> Option<Expr> {
    let Kind::Mul(factors) = e.kind() else { return None };
    for (i, f) in factors.iter().enumerate() {
        let Kind::Fn(kind, args) = f.kind() else { continue };
        if args.len() != 1 {
            continue;
        }
        let inner = &args[0];
        if !crate::visit::contains_symbol(inner, x) {
            continue;
        }
        let Some(inner_d) = crate::diff::diff(inner, x) else { continue };
        if inner_d.is_zero_literal() {
            continue;
        }
        let rest: Vec<Expr> = factors.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, g)| g.clone()).collect();
        let ratio = crate::simplify::cancel(&(Expr::mul(rest) * Expr::pow(inner_d, Expr::integer(-1))));
        if crate::visit::contains_symbol(&ratio, x) {
            continue;
        }
        if let Some(inner_antideriv) = antiderivative_table(kind, inner) {
            return Some(ratio * inner_antideriv);
        }
    }
    None
}

fn antiderivative_table(kind: &FnKind, inner: &Expr) -> Option<Expr> {
    use FnKind::*;
    Some(match kind {
        Sin => Expr::integer(-1) * Expr::func(Cos, vec![inner.clone()]),
        Cos => Expr::func(Sin, vec![inner.clone()]),
        Exp => Expr::func(Exp, vec![inner.clone()]),
        Sinh => Expr::func(Cosh, vec![inner.clone()]),
        Cosh => Expr::func(Sinh, vec![inner.clone()]),
        Tan => Expr::integer(-1) * Expr::func(Ln, vec![Expr::func(Abs, vec![Expr::func(Cos, vec![inner.clone()])])]),
        _ => return None,
    })
}
// #endregion 🔖Substitution

// #region 🔖ByParts
/// 🧩 Integration by parts for a two-factor product, choosing `u` via a coarse LIATE ranking
/// (Logarithm < Inverse-trig < Algebraic < Trig/hyperbolic < Exponential), depth-capped so the
/// `v * du` recursion can't loop forever on a pair that doesn't actually simplify.
fn integrate_by_parts(e: &Expr, x: &Expr, depth: u32) -> Option<Expr> {
    let factors: Vec<Expr> = match e.kind() {
        Kind::Mul(fs) => fs.clone(),
        _ => vec![e.clone()],
    };
    if factors.len() > 2 {
        return None;
    }
    let (u, dv) = if factors.len() == 2 {
        if liate_rank(&factors[0], x) <= liate_rank(&factors[1], x) {
            (factors[0].clone(), factors[1].clone())
        } else {
            (factors[1].clone(), factors[0].clone())
        }
    } else {
        (e.clone(), Expr::integer(1))
    };
    let v = integrate_depth(&dv, x, depth + 1)?;
    let du = crate::diff::diff(&u, x)?;
    if du.is_zero_literal() {
        return Some(u * v);
    }
    let second_term = integrate_depth(&(v.clone() * du), x, depth + 1)?;
    Some(u * v - second_term)
}

fn liate_rank(f: &Expr, x: &Expr) -> i32 {
    match f.kind() {
        Kind::Fn(FnKind::Ln, _) => 0,
        Kind::Fn(FnKind::Asin | FnKind::Acos | FnKind::Atan, _) => 1,
        Kind::Fn(FnKind::Sin | FnKind::Cos | FnKind::Sinh | FnKind::Cosh, _) => 3,
        Kind::Fn(FnKind::Exp, _) => 4,
        _ if f == x || matches!(f.kind(), Kind::Pow(base, _) if base == x) => 2,
        _ => 2,
    }
}
// #endregion 🔖ByParts

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn diff_matches(e: &Expr, x: &Expr, antideriv: &Expr) -> bool {
        let d = crate::diff::diff(antideriv, x).unwrap();
        crate::simplify::simplify(&(d - e.clone())).is_zero_literal()
    }

    #[test]
    fn integrate_power_rule() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2));
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[test]
    fn integrate_reciprocal_gives_ln() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(-1));
        let result = integrate(&e, &x).unwrap();
        assert_eq!(result, Expr::func(FnKind::Ln, vec![Expr::func(FnKind::Abs, vec![x])]));
    }

    #[test]
    fn integrate_sin_and_cos() {
        let x = Expr::symbol("x");
        let sin_result = integrate(&Expr::func(FnKind::Sin, vec![x.clone()]), &x).unwrap();
        assert!(diff_matches(&Expr::func(FnKind::Sin, vec![x.clone()]), &x, &sin_result));
        let cos_result = integrate(&Expr::func(FnKind::Cos, vec![x.clone()]), &x).unwrap();
        assert!(diff_matches(&Expr::func(FnKind::Cos, vec![x.clone()]), &x, &cos_result));
    }

    #[test]
    fn integrate_polynomial_sum() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2)) + Expr::mul(vec![Expr::integer(3), x.clone()]) + Expr::integer(1);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[test]
    fn integrate_simple_partial_fraction() {
        let x = Expr::symbol("x");
        // 1/((x-1)(x+1)) integrates to (1/2)ln|x-1| - (1/2)ln|x+1| (up to grouping)
        let den = (x.clone() - Expr::integer(1)) * (x.clone() + Expr::integer(1));
        let e = Expr::pow(den, Expr::integer(-1));
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[test]
    fn integrate_u_substitution() {
        let x = Expr::symbol("x");
        // 2x * cos(x^2) -> sin(x^2)
        let inner = Expr::pow(x.clone(), Expr::integer(2));
        let e = Expr::mul(vec![Expr::integer(2), x.clone(), Expr::func(FnKind::Cos, vec![inner.clone()])]);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[test]
    fn integrate_by_parts_x_times_exp() {
        let x = Expr::symbol("x");
        let e = Expr::mul(vec![x.clone(), Expr::func(FnKind::Exp, vec![x.clone()])]);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[test]
    fn integrate_ln_by_parts() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Ln, vec![x.clone()]);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[test]
    fn integrate_irreducible_quadratic_denominator() {
        let x = Expr::symbol("x");
        // 1/(x^2+1) -> atan(x)
        let e = Expr::pow(Expr::pow(x.clone(), Expr::integer(2)) + Expr::integer(1), Expr::integer(-1));
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[test]
    fn definite_integral_of_power() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2));
        let result = integrate_definite(&e, &x, &Expr::integer(0), &Expr::integer(3)).unwrap();
        assert_eq!(result, Expr::integer(9));
    }

    #[test]
    fn residue_at_simple_pole() {
        let x = Expr::symbol("x");
        // 1/(x-2) has residue 1 at x=2
        let e = Expr::pow(x.clone() - Expr::integer(2), Expr::integer(-1));
        assert_eq!(residue(&e, &x, &Expr::integer(2)), Some(Expr::integer(1)));
    }
}
// #endregion 🔖Tests
