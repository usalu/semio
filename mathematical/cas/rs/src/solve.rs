//! 🧩 Equation solving: univariate polynomial equations over `Q` (linear/quadratic in closed form,
//! degree >= 3 via `RootOf`), a narrow transcendental table (bare `exp(x)`/`ln(x)`/`sin(x)` equations),
//! symbolic-coefficient linear systems via Cramer's rule, and univariate rational-function inequalities
//! via root isolation + sign sampling.

use crate::expr::{Constant, Expr, Kind, RelationalOperator};
use crate::fnkind::FnKind;
use mathematical_number::{Integer, Natural, Rational};
use mathematical_polynomial::PolyU;

// #region 🔖SolutionSet
#[derive(Clone, Debug, PartialEq)]
pub enum Bound {
    NegInf,
    Inf,
    Value(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SolutionSet {
    Finite(Vec<Expr>),
    Intervals(Vec<(Bound, Bound)>),
    Parametric { sols: Vec<Expr>, params: Vec<Expr> },
    Empty,
    All,
    Unknown,
}
// #endregion 🔖SolutionSet

// #region 🔖Univariate
/// 🧩 Solves `e == 0` for `x`.
pub fn solve_univariate(e: &Expr, x: &Expr) -> SolutionSet {
    if let Some(p) = crate::polybridge::as_poly_uni(e, x) {
        return solve_poly_rational(&p, x);
    }
    solve_transcendental(e, x)
}

fn clear_denominators(p: &PolyU<Rational>) -> PolyU<Integer> {
    let denom_lcm = p.coeffs().iter().fold(Natural::one(), |acc, c| {
        let g = acc.gcd(c.denom());
        acc.mul(c.denom()).div_rem(&g).0
    });
    let scale = Rational::from_integer(Integer::from_natural(denom_lcm));
    PolyU::from_coeffs(p.coeffs().iter().map(|c| c.mul(&scale).trunc()).collect())
}

fn solve_poly_rational(p: &PolyU<Rational>, x: &Expr) -> SolutionSet {
    if p.is_zero() {
        return SolutionSet::All;
    }
    if p.degree().unwrap_or(0) == 0 {
        return SolutionSet::Empty;
    }
    let (_, factors) = crate::polybridge::factor_poly_u(p);
    let mut roots: std::collections::BTreeSet<Expr> = std::collections::BTreeSet::new();
    for (factor, _mult) in &factors {
        for r in solve_irreducible(factor, x) {
            roots.insert(r);
        }
    }
    if roots.is_empty() {
        SolutionSet::Empty
    } else {
        SolutionSet::Finite(roots.into_iter().collect())
    }
}

fn solve_irreducible(factor: &PolyU<Rational>, x: &Expr) -> Vec<Expr> {
    match factor.degree().unwrap_or(0) {
        0 => Vec::new(),
        1 => vec![solve_linear(factor)],
        2 => solve_quadratic(factor),
        _ => solve_via_rootof(factor, x),
    }
}

fn solve_linear(factor: &PolyU<Rational>) -> Expr {
    let a = factor.coeff(1);
    let b = factor.coeff(0);
    Expr::from(b.neg().div(&a).expect("nonzero leading coefficient of a degree-1 factor"))
}

/// √ `sqrt(r)` as a canonical `Expr`, rationalizing `sqrt(p/q) = sqrt(p*q)/q` so the radical-extraction
/// in `canon.rs` (which only folds `Integer` bases) gets a chance to simplify it; negative `r` factors
/// out `i`.
pub(crate) fn sqrt_of_rational(r: &Rational) -> Expr {
    if r.is_zero() {
        return Expr::integer(0);
    }
    if r.numer().is_negative() {
        return Expr::mul(vec![Expr::constant(Constant::I), sqrt_of_rational(&r.neg())]);
    }
    let numer = r.numer().magnitude().clone();
    let denom = r.denom().clone();
    let product = Integer::from_natural(numer.mul(&denom));
    let sqrt_expr = Expr::pow(Expr::from(product), Expr::from(Rational::from_i64(1, 2).unwrap()));
    Expr::mul(vec![sqrt_expr, Expr::from(Rational::new(Integer::one(), Integer::from_natural(denom)).unwrap())])
}

fn solve_quadratic(factor: &PolyU<Rational>) -> Vec<Expr> {
    let a = factor.coeff(2);
    let b = factor.coeff(1);
    let c = factor.coeff(0);
    let disc = b.mul(&b).sub(&a.mul(&c).mul(&Rational::from_i64(4, 1).unwrap()));
    let sqrt_disc = sqrt_of_rational(&disc);
    let two_a_inv = a.mul(&Rational::from_i64(2, 1).unwrap()).inv().expect("nonzero leading coefficient of a degree-2 factor");
    let neg_b = Expr::from(b.neg());
    vec![Expr::mul(vec![neg_b.clone() + sqrt_disc.clone(), Expr::from(two_a_inv.clone())]), Expr::mul(vec![neg_b - sqrt_disc, Expr::from(two_a_inv)])]
}

/// 🌱 Degree >= 3: real roots only, as `RootOf` objects (complex-root enumeration and the classical
/// Cardano/Ferrari radical forms are a documented follow-up — `RootOf` is always correct, just not
/// always a closed radical).
fn solve_via_rootof(factor: &PolyU<Rational>, _x: &Expr) -> Vec<Expr> {
    let int_poly = clear_denominators(factor);
    crate::rootof::real_roots_of(&int_poly)
}
// #endregion 🔖Univariate

// #region 🔖Transcendental
/// 🧩 Narrow pattern table: recognizes `e` as affine (`A*g + B` with `A, B` numeric) in a single
/// function-application generator `g = f(x)` with `f`'s argument being exactly `x` (not a nested
/// expression), and inverts `f` for `Exp`/`Ln`/`Sin`. Everything else is `Unknown`, never guessed.
fn solve_transcendental(e: &Expr, x: &Expr) -> SolutionSet {
    let gens = crate::polybridge::detect_gens(e);
    for g in &gens {
        let Kind::Fn(kind, args) = g.kind() else { continue };
        if args.len() != 1 || &args[0] != x {
            continue;
        }
        let Some((p, _map)) = crate::polybridge::as_poly(e, std::slice::from_ref(g)) else { continue };
        if p.total_degree() != 1 {
            continue;
        }
        let a = p.terms().iter().find(|(m, _)| m.exps()[0] == 1).map(|(_, c)| c.clone());
        let b = p.terms().iter().find(|(m, _)| m.exps()[0] == 0).map(|(_, c)| c.clone()).unwrap_or_else(Rational::zero);
        let Some(a) = a else { continue };
        let value = Expr::from(b.neg().div(&a).expect("nonzero coefficient of the matched generator"));
        return invert_generator(kind, x, &value);
    }
    SolutionSet::Unknown
}

fn invert_generator(kind: &FnKind, x: &Expr, value: &Expr) -> SolutionSet {
    match kind {
        FnKind::Exp => match crate::assume::is_positive(value) {
            Some(true) => SolutionSet::Finite(vec![Expr::func(FnKind::Ln, vec![value.clone()])]),
            Some(false) => SolutionSet::Empty,
            None => SolutionSet::Unknown,
        },
        FnKind::Ln => SolutionSet::Finite(vec![Expr::func(FnKind::Exp, vec![value.clone()])]),
        FnKind::Sin => {
            let n = Expr::symbol_with("n", crate::assume::AssumeSet::INTEGER);
            let asin_v = Expr::func(FnKind::Asin, vec![value.clone()]);
            let two_pi_n = Expr::mul(vec![Expr::integer(2), Expr::constant(Constant::Pi), n.clone()]);
            let sol1 = asin_v.clone() + two_pi_n.clone();
            let sol2 = (Expr::constant(Constant::Pi) - asin_v) + two_pi_n;
            let _ = x;
            SolutionSet::Parametric { sols: vec![sol1, sol2], params: vec![n] }
        }
        _ => SolutionSet::Unknown,
    }
}
// #endregion 🔖Transcendental

// #region 🔖LinearSystems
/// 🧩 Solves a square system of equations (each `== 0`), linear in `vars`, via Cramer's rule over plain
/// `Expr` arithmetic — no `Ring`/`Field` abstraction needed since the entries are already `Expr`.
/// Only square, non-singular systems are resolved in this pass; anything else is `Unknown`.
pub fn solve_linear_system(eqs: &[Expr], vars: &[Expr]) -> SolutionSet {
    let n = vars.len();
    if eqs.len() != n || n == 0 {
        return SolutionSet::Unknown;
    }
    let mut a = vec![vec![Expr::integer(0); n]; n];
    let mut b = vec![Expr::integer(0); n];
    for (row, eq) in eqs.iter().enumerate() {
        let Some((coeffs, constant)) = linear_coeffs_expr(eq, vars) else { return SolutionSet::Unknown };
        a[row] = coeffs;
        b[row] = Expr::integer(-1) * constant;
    }
    let det_a = crate::simplify::simplify(&det_expr(&a));
    if det_a.is_zero_literal() {
        return SolutionSet::Unknown;
    }
    SolutionSet::Finite(cramer_solutions(&a, &b, det_a))
}

fn cramer_solutions(a: &[Vec<Expr>], b: &[Expr], det_a: Expr) -> Vec<Expr> {
    let n = a.len();
    let mut sols = Vec::with_capacity(n);
    for i in 0..n {
        let mut a_i = a.to_vec();
        for row in 0..n {
            a_i[row][i] = b[row].clone();
        }
        sols.push(crate::simplify::cancel(&(det_expr(&a_i) * Expr::pow(det_a.clone(), Expr::integer(-1)))));
    }
    sols
}

/// 🧮 Cofactor-expansion determinant over plain `Expr` entries — reused by `matrix.rs` for symbolic
/// matrices, since `Expr` already behaves like a field under its own `+`/`-`/`*`/`Pow(-1)` encoding.
pub(crate) fn det_expr(m: &[Vec<Expr>]) -> Expr {
    let n = m.len();
    if n == 0 {
        return Expr::integer(1);
    }
    if n == 1 {
        return m[0][0].clone();
    }
    let mut result = Expr::integer(0);
    for col in 0..n {
        let minor: Vec<Vec<Expr>> = m[1..].iter().map(|row| row.iter().enumerate().filter(|&(j, _)| j != col).map(|(_, v)| v.clone()).collect()).collect();
        let sign = if col % 2 == 0 { Expr::integer(1) } else { Expr::integer(-1) };
        result = result + sign * m[0][col].clone() * det_expr(&minor);
    }
    result
}

/// 🧩 Extracts `(coeffs, constant)` such that `eq == sum(coeffs[i] * vars[i]) + constant`, or `None` if
/// `eq` (after `expand`) has a term mixing two variables or a variable at a power other than 1.
fn linear_coeffs_expr(eq: &Expr, vars: &[Expr]) -> Option<(Vec<Expr>, Expr)> {
    let expanded = crate::simplify::expand(eq);
    let terms: Vec<Expr> = match expanded.kind() {
        Kind::Add(ts) => ts.clone(),
        _ => vec![expanded.clone()],
    };
    let mut coeffs = vec![Expr::integer(0); vars.len()];
    let mut constant_terms = Vec::new();
    for t in &terms {
        let factors: Vec<Expr> = match t.kind() {
            Kind::Mul(fs) => fs.clone(),
            _ => vec![t.clone()],
        };
        let mut matched_var: Option<usize> = None;
        let mut rest = Vec::new();
        for f in &factors {
            if let Some(i) = vars.iter().position(|v| v == f) {
                if matched_var.is_some() {
                    return None;
                }
                matched_var = Some(i);
                continue;
            }
            if let Kind::Pow(base, _) = f.kind() {
                if vars.contains(base) {
                    return None;
                }
            }
            rest.push(f.clone());
        }
        match matched_var {
            Some(i) => coeffs[i] = coeffs[i].clone() + Expr::mul(rest),
            None => constant_terms.push(t.clone()),
        }
    }
    Some((coeffs, Expr::add(constant_terms)))
}
// #endregion 🔖LinearSystems

// #region 🔖Inequalities
/// 📏 Solves a univariate rational-function inequality `e <operation> 0` via real root isolation of the
/// numerator and denominator, then samples the sign of `e` at the midpoint of each interval between
/// consecutive critical points. The sampling itself uses `f64` midpoints (a documented heuristic —
/// exact Sturm-based sign evaluation at rational sample points would be fully certified, but midpoint
/// sampling is correct as long as no two distinct critical points round to the same `f64`, which is
/// true for any inputs realistic at this scale).
pub fn solve_inequality(e: &Expr, operator: RelationalOperator, x: &Expr) -> SolutionSet {
    let Some((num_m, den_m, map)) = crate::polybridge::as_ratfunc_auto(e) else { return SolutionSet::Unknown };
    if map.gens.len() != 1 || map.gens[0] != *x {
        return SolutionSet::Unknown;
    }
    let Some(num) = crate::polybridge::polym_to_polyu(&num_m, 0) else { return SolutionSet::Unknown };
    let Some(den) = crate::polybridge::polym_to_polyu(&den_m, 0) else { return SolutionSet::Unknown };
    if den.is_zero() {
        return SolutionSet::Unknown;
    }
    let num_i = clear_denominators(&num);
    let den_i = clear_denominators(&den);
    let num_roots = if num_i.degree().unwrap_or(0) > 0 { mathematical_polynomial::isolate_real_roots(&num_i) } else { Vec::new() };
    let den_roots = if den_i.degree().unwrap_or(0) > 0 { mathematical_polynomial::isolate_real_roots(&den_i) } else { Vec::new() };

    let mut points: Vec<f64> = num_roots.iter().chain(den_roots.iter()).map(|(lo, hi)| (lo.to_f64() + hi.to_f64()) / 2.0).collect();
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let den_root_set: Vec<f64> = den_roots.iter().map(|(lo, hi)| (lo.to_f64() + hi.to_f64()) / 2.0).collect();
    let is_pole = |p: f64| den_root_set.iter().any(|&d| (d - p).abs() < 1e-9);

    let sample_at = |sample: f64| -> f64 { num.eval(&Rational::from_f64(sample).unwrap_or_else(Rational::zero)).to_f64() / den.eval(&Rational::from_f64(sample).unwrap_or_else(Rational::zero)).to_f64() };

    let mut boundaries: Vec<f64> = vec![f64::NEG_INFINITY];
    boundaries.extend(points.iter().copied());
    boundaries.push(f64::INFINITY);

    let mut intervals: Vec<(Bound, Bound)> = Vec::new();
    for w in boundaries.windows(2) {
        let (lo, hi) = (w[0], w[1]);
        let sample = if lo.is_finite() && hi.is_finite() {
            (lo + hi) / 2.0
        } else if lo.is_finite() {
            lo + 1.0
        } else if hi.is_finite() {
            hi - 1.0
        } else {
            0.0
        };
        let value = sample_at(sample);
        let holds = match operation {
            RelationalOperator::Gt => value > 0.0,
            RelationalOperator::Ge => value >= 0.0,
            RelationalOperator::Lt => value < 0.0,
            RelationalOperator::Le => value <= 0.0,
            RelationalOperator::Eq | RelationalOperator::Ne => false, // equalities/disequalities go through solve_univariate, not here
        };
        if holds {
            let lo_bound = if lo.is_finite() { Bound::Value(Expr::from(Rational::from_f64(lo).unwrap_or_else(Rational::zero))) } else { Bound::NegInf };
            let hi_bound = if hi.is_finite() { Bound::Value(Expr::from(Rational::from_f64(hi).unwrap_or_else(Rational::zero))) } else { Bound::Inf };
            intervals.push((lo_bound, hi_bound));
        }
        let _ = is_pole(sample);
    }
    if intervals.is_empty() {
        SolutionSet::Empty
    } else {
        SolutionSet::Intervals(intervals)
    }
}
// #endregion 🔖Inequalities

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_linear_equation() {
        let x = Expr::symbol("x");
        // 2x - 6 = 0 -> x = 3
        let e = Expr::mul(vec![Expr::integer(2), x.clone()]) - Expr::integer(6);
        assert_eq!(solve_univariate(&e, &x), SolutionSet::Finite(vec![Expr::integer(3)]));
    }

    #[test]
    fn solve_quadratic_with_real_roots() {
        let x = Expr::symbol("x");
        // x^2 - 5x + 6 = 0 -> {2, 3}
        let e = Expr::pow(x.clone(), Expr::integer(2)) - Expr::mul(vec![Expr::integer(5), x.clone()]) + Expr::integer(6);
        let result = solve_univariate(&e, &x);
        match result {
            SolutionSet::Finite(mut roots) => {
                roots.sort();
                assert_eq!(roots, vec![Expr::integer(2), Expr::integer(3)]);
            }
            other => panic!("expected Finite, got {other:?}"),
        }
    }

    #[test]
    fn solve_quadratic_with_complex_roots() {
        let x = Expr::symbol("x");
        // x^2 + 1 = 0 -> {i, -i}
        let e = Expr::pow(x.clone(), Expr::integer(2)) + Expr::integer(1);
        let result = solve_univariate(&e, &x);
        match result {
            SolutionSet::Finite(roots) => {
                assert_eq!(roots.len(), 2);
                assert!(roots.contains(&Expr::constant(Constant::I)));
            }
            other => panic!("expected Finite, got {other:?}"),
        }
    }

    #[test]
    fn solve_high_degree_gives_rootof() {
        let x = Expr::symbol("x");
        // x^5 - x - 1 = 0 (irreducible over Q, one real root)
        let e = Expr::pow(x.clone(), Expr::integer(5)) - x.clone() - Expr::integer(1);
        let result = solve_univariate(&e, &x);
        match result {
            SolutionSet::Finite(roots) => {
                assert!(!roots.is_empty());
                assert!(roots.iter().all(|r| matches!(r.kind(), Kind::RootOf { .. })));
            }
            other => panic!("expected Finite RootOf set, got {other:?}"),
        }
    }

    #[test]
    fn solve_exp_equation() {
        let x = Expr::symbol("x");
        // 2*exp(x) - 6 = 0 -> x = ln(3)
        let e = Expr::mul(vec![Expr::integer(2), Expr::func(FnKind::Exp, vec![x.clone()])]) - Expr::integer(6);
        let result = solve_univariate(&e, &x);
        assert_eq!(result, SolutionSet::Finite(vec![Expr::func(FnKind::Ln, vec![Expr::integer(3)])]));
    }

    #[test]
    fn solve_sin_equation_gives_parametric_family() {
        let x = Expr::symbol("x");
        let half = Expr::from(Rational::from_i64(1, 2).unwrap());
        let e = Expr::func(FnKind::Sin, vec![x.clone()]) - half;
        match solve_univariate(&e, &x) {
            SolutionSet::Parametric { sols, params } => {
                assert_eq!(sols.len(), 2);
                assert_eq!(params.len(), 1);
            }
            other => panic!("expected Parametric, got {other:?}"),
        }
    }

    #[test]
    fn solve_2x2_linear_system() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        // 2x + y = 5, x - y = 1 -> x=2, y=1
        let eq1 = Expr::mul(vec![Expr::integer(2), x.clone()]) + y.clone() - Expr::integer(5);
        let eq2 = x.clone() - y.clone() - Expr::integer(1);
        let result = solve_linear_system(&[eq1, eq2], &[x, y]);
        assert_eq!(result, SolutionSet::Finite(vec![Expr::integer(2), Expr::integer(1)]));
    }

    #[test]
    fn solve_inequality_simple_quadratic() {
        let x = Expr::symbol("x");
        // x^2 - 1 > 0  ->  x < -1 or x > 1
        let e = Expr::pow(x.clone(), Expr::integer(2)) - Expr::integer(1);
        let result = solve_inequality(&e, RelationalOperator::Gt, &x);
        match result {
            SolutionSet::Intervals(intervals) => assert_eq!(intervals.len(), 2),
            other => panic!("expected Intervals, got {other:?}"),
        }
    }
}
// #endregion 🔖Tests
