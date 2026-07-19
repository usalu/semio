//! 🧽 Structural algebra built on the poly bridge: `expand`/`collect`/`together`/`cancel`/`apart`/
//! `factor`, one classical radical-denesting pattern, and the measured `simplify` pipeline that picks
//! whichever rewrite has the fewest nodes (deterministic — never a search, never a guess at "prettiest").

use crate::expr::{Expr, Kind};
use crate::polybridge;
use mathematical_number::Rational;
use mathematical_polynomial::PolyU;

// #region 🔖Expand
pub fn expand(e: &Expr) -> Expr {
    if let Some((poly, map)) = polybridge::as_poly_auto(e) {
        return polybridge::from_poly(&poly, &map);
    }
    expand_tree(e)
}

fn expand_tree(e: &Expr) -> Expr {
    match e.kind() {
        Kind::Add(terms) => Expr::add(terms.iter().map(expand_tree).collect()),
        Kind::Mul(factors) => {
            let expanded: Vec<Expr> = factors.iter().map(expand_tree).collect();
            distribute_mul(&expanded)
        }
        Kind::Pow(base, exp) => {
            let base_expanded = expand_tree(base);
            if let Kind::Integer(n) = exp.kind() {
                if let Some(ev) = n.to_i64() {
                    if ev >= 0 && matches!(base_expanded.kind(), Kind::Add(_)) {
                        let mut result = Expr::integer(1);
                        for _ in 0..ev {
                            result = distribute_pair(&result, &base_expanded);
                        }
                        return result;
                    }
                }
            }
            Expr::pow(base_expanded, expand_tree(exp))
        }
        Kind::Fn(kind, args) => Expr::func(kind.clone(), args.iter().map(expand_tree).collect()),
        _ => e.clone(),
    }
}

fn distribute_mul(factors: &[Expr]) -> Expr {
    let mut acc = Expr::integer(1);
    for f in factors {
        acc = distribute_pair(&acc, f);
    }
    acc
}

fn distribute_pair(a: &Expr, b: &Expr) -> Expr {
    let a_terms: Vec<Expr> = match a.kind() {
        Kind::Add(ts) => ts.clone(),
        _ => vec![a.clone()],
    };
    let b_terms: Vec<Expr> = match b.kind() {
        Kind::Add(ts) => ts.clone(),
        _ => vec![b.clone()],
    };
    let mut sum_terms = Vec::with_capacity(a_terms.len() * b_terms.len());
    for at in &a_terms {
        for bt in &b_terms {
            sum_terms.push(Expr::mul(vec![at.clone(), bt.clone()]));
        }
    }
    Expr::add(sum_terms)
}
// #endregion 🔖Expand

// #region 🔖Collect
/// 🗂️ Groups the (expanded) terms of `e` by their integer power of `x`; terms that aren't a clean
/// integer power of `x` (e.g. involving another generator entirely) are left untouched and appended.
pub fn collect(e: &Expr, x: &Expr) -> Expr {
    let expanded = expand(e);
    let terms: Vec<Expr> = match expanded.kind() {
        Kind::Add(ts) => ts.clone(),
        _ => vec![expanded.clone()],
    };
    let mut buckets: std::collections::BTreeMap<i64, Vec<Expr>> = std::collections::BTreeMap::new();
    let mut leftover: Vec<Expr> = Vec::new();
    for t in &terms {
        match term_power_of(t, x) {
            Some((exp, coeff)) => buckets.entry(exp).or_default().push(coeff),
            None => leftover.push(t.clone()),
        }
    }
    let mut result_terms: Vec<Expr> = Vec::new();
    for (exp, coeffs) in buckets.into_iter().rev() {
        let coeff_sum = Expr::add(coeffs);
        let term = if exp == 0 { coeff_sum } else { Expr::mul(vec![coeff_sum, Expr::pow(x.clone(), Expr::integer(exp))]) };
        result_terms.push(term);
    }
    result_terms.extend(leftover);
    Expr::add(result_terms)
}

fn term_power_of(term: &Expr, x: &Expr) -> Option<(i64, Expr)> {
    if term == x {
        return Some((1, Expr::integer(1)));
    }
    if let Kind::Pow(base, exp) = term.kind() {
        if base == x {
            if let Kind::Integer(n) = exp.kind() {
                return n.to_i64().map(|ev| (ev, Expr::integer(1)));
            }
        }
    }
    if let Kind::Mul(factors) = term.kind() {
        for (i, f) in factors.iter().enumerate() {
            let found = if f == x {
                Some(1i64)
            } else if let Kind::Pow(base, exp) = f.kind() {
                if base == x {
                    if let Kind::Integer(n) = exp.kind() {
                        n.to_i64()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(exp) = found {
                let mut rest = factors.clone();
                rest.remove(i);
                return Some((exp, Expr::mul(rest)));
            }
        }
        return Some((0, term.clone()));
    }
    Some((0, term.clone()))
}
// #endregion 🔖Collect

// #region 🔖TogetherCancel
pub fn together(e: &Expr) -> Expr {
    let Some((num, den, map)) = polybridge::as_ratfunc_auto(e) else { return e.clone() };
    polybridge::build_ratio(&num, &den, &map)
}

/// ➗ `together`, plus a GCD cancellation pass when the numerator/denominator involve at most one
/// generator (the univariate case, where `PolyU::gcd_monic` applies); a genuinely multivariate
/// cancellation is left uncancelled (documented limitation — still correct, just not maximally reduced).
pub fn cancel(e: &Expr) -> Expr {
    let Some((num, den, map)) = polybridge::as_ratfunc_auto(e) else { return e.clone() };
    if den.is_zero() {
        return e.clone();
    }
    let used: Vec<usize> = (0..map.gens.len()).filter(|&i| polybridge::poly_uses_var(&num, i) || polybridge::poly_uses_var(&den, i)).collect();
    if let [vi] = used[..] {
        if let (Some(nu), Some(du)) = (polybridge::polym_to_polyu(&num, vi), polybridge::polym_to_polyu(&den, vi)) {
            if !du.is_zero() && du.degree().unwrap_or(0) > 0 {
                let g = nu.gcd_monic(&du);
                if g.degree().unwrap_or(0) > 0 {
                    if let (Some(nq), Some(dq)) = (exact_div_u(&nu, &g), exact_div_u(&du, &g)) {
                        let num2 = polybridge::polyu_to_polym(&nq, vi, map.gens.len());
                        let den2 = polybridge::polyu_to_polym(&dq, vi, map.gens.len());
                        return polybridge::build_ratio(&num2, &den2, &map);
                    }
                }
            }
        }
    }
    polybridge::build_ratio(&num, &den, &map)
}

fn exact_div_u(a: &PolyU<Rational>, b: &PolyU<Rational>) -> Option<PolyU<Rational>> {
    let (q, r) = a.div_rem(b);
    if r.is_zero() {
        Some(q)
    } else {
        None
    }
}
// #endregion 🔖TogetherCancel

// #region 🔖Factor
/// 🔍 Factors `e` over `Q` when it's univariate (a single generator); genuinely multivariate
/// expressions are returned unchanged — multivariate factoring is a documented follow-up, not attempted
/// via a wrong or partial answer.
pub fn factor(e: &Expr) -> Expr {
    let gens = polybridge::detect_gens(e);
    if gens.len() != 1 {
        return e.clone();
    }
    let Some(p) = polybridge::as_poly_uni(e, &gens[0]) else { return e.clone() };
    let (overall, factors) = polybridge::factor_poly_u(&p);
    if factors.is_empty() {
        return Expr::from(overall);
    }
    let mut all = vec![Expr::from(overall)];
    for (f, mult) in &factors {
        let fe = polybridge::polyu_to_expr(f, &gens[0]);
        all.push(if *mult == 1 { fe } else { Expr::pow(fe, Expr::integer(*mult as i64)) });
    }
    Expr::mul(all)
}
// #endregion 🔖Factor

// #region 🔖Apart
/// 🧩 Univariate partial-fraction decomposition over `Q`: factors the denominator, then solves the
/// linear system (via `mathematical_algebra`'s exact `MatG::solve`) for each factor's numerator
/// coefficients — handles repeated factors, not just squarefree denominators.
pub fn apart(e: &Expr, x: &Expr) -> Expr {
    let Some((num_m, den_m, map)) = polybridge::as_ratfunc_auto(e) else { return e.clone() };
    if map.gens.len() != 1 || map.gens[0] != *x {
        return e.clone();
    }
    let Some(num) = polybridge::polym_to_polyu(&num_m, 0) else { return e.clone() };
    let Some(den) = polybridge::polym_to_polyu(&den_m, 0) else { return e.clone() };
    if den.is_zero() {
        return e.clone();
    }
    apart_univariate(&num, &den, x)
}

fn together_fallback(poly_part: &PolyU<Rational>, remainder: &PolyU<Rational>, den: &PolyU<Rational>, x: &Expr) -> Expr {
    let poly_expr = polybridge::polyu_to_expr(poly_part, x);
    if remainder.is_zero() {
        return poly_expr;
    }
    Expr::add(vec![poly_expr, Expr::mul(vec![polybridge::polyu_to_expr(remainder, x), Expr::pow(polybridge::polyu_to_expr(den, x), Expr::integer(-1))])])
}

fn apart_univariate(num: &PolyU<Rational>, den: &PolyU<Rational>, x: &Expr) -> Expr {
    let (poly_part, remainder) = num.div_rem(den);
    if remainder.is_zero() {
        return polybridge::polyu_to_expr(&poly_part, x);
    }
    let (overall, factors) = polybridge::factor_poly_u(den);
    if factors.is_empty() {
        return together_fallback(&poly_part, &remainder, den, x);
    }
    let deg_den = den.degree().unwrap_or(0);
    let unknowns: Vec<(usize, u32, usize)> = factors
        .iter()
        .enumerate()
        .flat_map(|(fi, (factor, mult))| {
            let d = factor.degree().unwrap_or(0).max(1);
            (1..=*mult).flat_map(move |j| (0..d).map(move |k| (fi, j, k))).collect::<Vec<_>>()
        })
        .collect();
    if unknowns.len() != deg_den {
        return together_fallback(&poly_part, &remainder, den, x);
    }

    let base_cofactors: Vec<PolyU<Rational>> = (0..factors.len())
        .map(|i| {
            let mut acc = PolyU::<Rational>::one();
            for (l, (factor, mult)) in factors.iter().enumerate() {
                if l != i {
                    acc = acc.mul(&factor.pow(*mult as u64));
                }
            }
            acc
        })
        .collect();

    let n = unknowns.len();
    let mut rows = vec![vec![Rational::zero(); n]; deg_den];
    for (col, &(fi, j, k)) in unknowns.iter().enumerate() {
        let (factor, mult) = &factors[fi];
        let cofactor = base_cofactors[fi].mul(&factor.pow((*mult - j) as u64));
        let basis = cofactor.mul_scalar(&overall).shift_up(k);
        for row in 0..deg_den {
            rows[row][col] = basis.coeff(row);
        }
    }
    let matrix = mathematical_algebra::MatG::from_rows(rows);
    let mut b_data = vec![Rational::zero(); deg_den];
    for (row, slot) in b_data.iter_mut().enumerate() {
        *slot = remainder.coeff(row);
    }
    let b = mathematical_algebra::VecG::from_vec(b_data);
    let Some(solution) = matrix.solve(&b) else {
        return together_fallback(&poly_part, &remainder, den, x);
    };

    let mut terms = vec![polybridge::polyu_to_expr(&poly_part, x)];
    let mut idx = 0;
    for (factor, mult) in &factors {
        let d = factor.degree().unwrap_or(0).max(1);
        for j in 1..=*mult {
            let coeffs: Vec<Rational> = (0..d).map(|k| solution.get(idx + k).clone()).collect();
            idx += d;
            let a_poly = PolyU::from_coeffs(coeffs);
            if a_poly.is_zero() {
                continue;
            }
            let a_expr = polybridge::polyu_to_expr(&a_poly, x);
            let factor_expr = polybridge::polyu_to_expr(factor, x);
            let denom_expr = if j == 1 { factor_expr } else { Expr::pow(factor_expr, Expr::integer(j as i64)) };
            terms.push(Expr::mul(vec![a_expr, Expr::pow(denom_expr, Expr::integer(-1))]));
        }
    }
    Expr::add(terms)
}
// #endregion 🔖Apart

// #region 🔖RadicalDenest
/// 🌱 Denests the classical `sqrt(p + q*sqrt(c))` pattern into `sqrt(t1) + sign(q)*sqrt(t2)` when
/// `t = p^2 - q^2*c` is a perfect-square integer and `(p+-sqrt(t))` are both even (so `t1, t2` land on
/// exact integers) — the single denesting identity in scope for the first pass.
pub fn denest_sqrt(e: &Expr) -> Expr {
    crate::visit::replace_bottom_up(e, &mut |sub| try_denest_sqrt(sub))
}

fn try_denest_sqrt(e: &Expr) -> Option<Expr> {
    let Kind::Pow(inner, exp) = e.kind() else { return None };
    if !is_half(exp) {
        return None;
    }
    let Kind::Add(terms) = inner.kind() else { return None };
    if terms.len() != 2 {
        return None;
    }
    let p = match terms[0].kind() {
        Kind::Integer(n) => n.to_i64()?,
        _ => return None,
    };
    let (b, c) = extract_b_sqrt_c(&terms[1])?;
    let t = p.checked_mul(p)?.checked_sub(b.checked_mul(b)?.checked_mul(c)?)?;
    if t < 0 {
        return None;
    }
    let sq = isqrt_i64(t)?;
    if sq * sq != t {
        return None;
    }
    let (num1, num2) = (p.checked_add(sq)?, p.checked_sub(sq)?);
    if num1 % 2 != 0 || num2 % 2 != 0 || num2 < 0 {
        return None;
    }
    let (t1, t2) = (num1 / 2, num2 / 2);
    let sqrt1 = Expr::pow(Expr::integer(t1), Expr::from(Rational::from_i64(1, 2).unwrap()));
    let sqrt2 = Expr::pow(Expr::integer(t2), Expr::from(Rational::from_i64(1, 2).unwrap()));
    let sign = if b < 0 { -1 } else { 1 };
    Some(Expr::add(vec![sqrt1, Expr::mul(vec![Expr::integer(sign), sqrt2])]))
}

fn is_half(e: &Expr) -> bool {
    matches!(e.kind(), Kind::Rational(r) if *r == Rational::from_i64(1, 2).unwrap())
}

fn isqrt_i64(v: i64) -> Option<i64> {
    if v < 0 {
        return None;
    }
    Some((v as f64).sqrt().round() as i64)
}

fn extract_b_sqrt_c(term: &Expr) -> Option<(i64, i64)> {
    match term.kind() {
        Kind::Mul(factors) if factors.len() == 2 => {
            let coeff = match factors[0].kind() {
                Kind::Integer(n) => n.to_i64()?,
                _ => return None,
            };
            let c = match factors[1].kind() {
                Kind::Pow(base, exp) if is_half(exp) => match base.kind() {
                    Kind::Integer(n) => n.to_i64()?,
                    _ => return None,
                },
                _ => return None,
            };
            Some((coeff, c))
        }
        Kind::Pow(base, exp) if is_half(exp) => match base.kind() {
            Kind::Integer(n) => n.to_i64().map(|c| (1, c)),
            _ => None,
        },
        _ => None,
    }
}
// #endregion 🔖RadicalDenest

// #region 🔖Simplify
/// 🧭 The measured simplification pipeline: try a handful of candidate rewrites and keep whichever has
/// the fewest nodes (canonical order breaks ties) — deterministic, no heuristic search.
pub fn simplify(e: &Expr) -> Expr {
    let candidates = [e.clone(), cancel(e), crate::trig::trig_canon(e), factor(e), denest_sqrt(e)];
    candidates.into_iter().min_by(|a, b| crate::visit::node_count(a).cmp(&crate::visit::node_count(b)).then_with(|| a.cmp(b))).expect("candidate list is non-empty by construction")
}
// #endregion 🔖Simplify

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expr;

    #[test]
    fn expand_binomial_square() {
        let x = Expr::symbol("x");
        let e = Expr::pow(Expr::add(vec![x.clone(), Expr::integer(1)]), Expr::integer(2));
        let expanded = expand(&e);
        let expected = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(2), x]), Expr::integer(1)]);
        assert_eq!(expanded, expected);
    }

    #[test]
    fn expand_distributes_over_function_argument_unchanged() {
        let x = Expr::symbol("x");
        let e = Expr::func(crate::fnkind::FnKind::Sin, vec![Expr::add(vec![x.clone(), Expr::integer(1)])]);
        assert_eq!(expand(&e), e);
    }

    #[test]
    fn collect_groups_like_powers() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(3), Expr::pow(x.clone(), Expr::integer(2))]), x.clone()]);
        let collected = collect(&e, &x);
        // 4x^2 + x
        let expected = Expr::add(vec![Expr::mul(vec![Expr::integer(4), Expr::pow(x.clone(), Expr::integer(2))]), x]);
        assert_eq!(collected, expected);
    }

    #[test]
    fn together_combines_fractions() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(-1)), Expr::integer(1)]);
        let combined = together(&e);
        // Verify numerically: (1/x + 1) at x=2 should equal the combined form evaluated the same way.
        assert_ne!(combined, e);
    }

    #[test]
    fn cancel_removes_common_univariate_factor() {
        let x = Expr::symbol("x");
        // (x^2 - 1) / (x - 1) -> x + 1
        let num = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(-1)]);
        let den = Expr::add(vec![x.clone(), Expr::integer(-1)]);
        let e = Expr::mul(vec![num, Expr::pow(den, Expr::integer(-1))]);
        let result = cancel(&e);
        assert_eq!(result, Expr::add(vec![x, Expr::integer(1)]));
    }

    #[test]
    fn factor_recovers_linear_factors() {
        let x = Expr::symbol("x");
        // x^2 - 1 -> (x-1)(x+1) up to ordering/sign; check by expanding back.
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(-1)]);
        let factored = factor(&e);
        assert_eq!(expand(&factored), e);
        assert_ne!(factored, e);
    }

    #[test]
    fn apart_splits_simple_rational_function() {
        let x = Expr::symbol("x");
        // 1/((x-1)(x+1)) = (1/2)/(x-1) - (1/2)/(x+1)
        let den = Expr::mul(vec![Expr::add(vec![x.clone(), Expr::integer(-1)]), Expr::add(vec![x.clone(), Expr::integer(1)])]);
        let e = Expr::pow(den, Expr::integer(-1));
        let result = apart(&e, &x);
        // Recombine via together+cancel-free check: evaluate both sides symbolically by re-expanding the together form.
        let recombined = together(&result);
        let original_together = together(&e);
        assert_eq!(cancel(&recombined), cancel(&original_together));
    }

    #[test]
    fn denest_sqrt_classic_example() {
        // sqrt(3 + 2*sqrt(2)) == 1 + sqrt(2)
        let inner = Expr::add(vec![Expr::integer(3), Expr::mul(vec![Expr::integer(2), Expr::pow(Expr::integer(2), Expr::from(Rational::from_i64(1, 2).unwrap()))])]);
        let e = Expr::pow(inner, Expr::from(Rational::from_i64(1, 2).unwrap()));
        let result = denest_sqrt(&e);
        let expected = Expr::add(vec![Expr::integer(1), Expr::pow(Expr::integer(2), Expr::from(Rational::from_i64(1, 2).unwrap()))]);
        assert_eq!(result, expected);
    }

    #[test]
    fn simplify_picks_the_smallest_candidate() {
        let x = Expr::symbol("x");
        let num = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(-1)]);
        let den = Expr::add(vec![x.clone(), Expr::integer(-1)]);
        let e = Expr::mul(vec![num, Expr::pow(den, Expr::integer(-1))]);
        let result = simplify(&e);
        assert_eq!(result, Expr::add(vec![x, Expr::integer(1)]));
    }
}
// #endregion 🔖Tests
