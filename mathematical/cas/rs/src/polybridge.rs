//! 🌉 The `Expr` <-> polynomial bridge — the workhorse every algebraic domain (`simplify`, `solve`,
//! rational `integrate`, `SymMatrix`) goes through: detect which subtrees behave as polynomial
//! "generators" (variables, in the Gröbner-basis sense), convert to/from `mathematical_polynomial`
//! types over those generators, and reconstruct canonical `Expr`s from the result.

use crate::expr::{Expr, Kind};
use mathematical_number::{Integer, Natural, Rational};
use mathematical_polynomial::{MonomialOrder, PolyM, PolyU};

// #region 🔖PolyMap
/// 🗺️ The ordered list of generators a conversion was performed against; `gens[i]` is polynomial
/// variable `i`.
#[derive(Clone, Debug)]
pub struct PolyMap {
    pub gens: Vec<Expr>,
}

fn gen_index(e: &Expr, map: &PolyMap) -> Option<usize> {
    map.gens.iter().position(|g| g == e)
}

fn push_unique(gens: &mut Vec<Expr>, e: Expr) {
    if !gens.contains(&e) {
        gens.push(e);
    }
}
// #endregion 🔖PolyMap

// #region 🔖GenDetection
/// 🔍 Collects the maximal non-polynomial subtrees of `e` as generators: symbols, function
/// applications, non-numeric constants, and any `Pow` node whose exponent isn't a plain non-negative
/// integer (fractional/negative/symbolic exponents can't be expressed as a polynomial power in the
/// base, so the whole `Pow` becomes its own opaque generator).
pub fn detect_gens(e: &Expr) -> Vec<Expr> {
    let mut gens = Vec::new();
    collect_gens(e, &mut gens);
    gens
}

fn collect_gens(e: &Expr, gens: &mut Vec<Expr>) {
    match e.kind() {
        Kind::Integer(_) | Kind::Rational(_) => {}
        Kind::Add(terms) | Kind::Mul(terms) => {
            for t in terms {
                collect_gens(t, gens);
            }
        }
        Kind::Pow(base, exp) => {
            if let Kind::Integer(n) = exp.kind() {
                if n.is_positive() || n.is_zero() {
                    collect_gens(base, gens);
                    return;
                }
            }
            push_unique(gens, e.clone());
        }
        _ => push_unique(gens, e.clone()),
    }
}
// #endregion 🔖GenDetection

// #region 🔖ExprToPoly
/// 🔁 Converts `e` to a `PolyM<Rational>` over the given (fixed, ordered) generator list; `None` if
/// `e` contains a subtree that isn't a polynomial combination of numbers and those generators (e.g. a
/// generator not in the list, or a negative/fractional power of one).
pub fn as_poly(e: &Expr, gens: &[Expr]) -> Option<(PolyM<Rational>, PolyMap)> {
    let map = PolyMap { gens: gens.to_vec() };
    let poly = expr_to_polym(e, &map)?;
    Some((poly, map))
}

pub fn as_poly_auto(e: &Expr) -> Option<(PolyM<Rational>, PolyMap)> {
    let gens = detect_gens(e);
    as_poly(e, &gens)
}

fn expr_to_polym(e: &Expr, map: &PolyMap) -> Option<PolyM<Rational>> {
    let nvars = map.gens.len().max(1);
    match e.kind() {
        Kind::Integer(n) => Some(PolyM::constant(Rational::from_integer(n.clone()), nvars, MonomialOrder::Lex)),
        Kind::Rational(r) => Some(PolyM::constant(r.clone(), nvars, MonomialOrder::Lex)),
        Kind::Add(terms) => {
            let mut acc = PolyM::zero(nvars, MonomialOrder::Lex);
            for t in terms {
                acc = acc.add(&expr_to_polym(t, map)?);
            }
            Some(acc)
        }
        Kind::Mul(factors) => {
            let mut acc = PolyM::constant(Rational::one(), nvars, MonomialOrder::Lex);
            for f in factors {
                acc = acc.mul(&expr_to_polym(f, map)?);
            }
            Some(acc)
        }
        Kind::Pow(base, exp) => {
            if let Kind::Integer(n) = exp.kind() {
                if let Some(ev) = n.to_i64() {
                    if ev >= 0 {
                        return Some(expr_to_polym(base, map)?.pow(ev as u64));
                    }
                }
            }
            gen_index(e, map).map(|idx| PolyM::var(idx, nvars, MonomialOrder::Lex))
        }
        _ => gen_index(e, map).map(|idx| PolyM::var(idx, nvars, MonomialOrder::Lex)),
    }
}

/// ↩️ Rebuilds a canonical `Expr` from a `PolyM<Rational>` and the generator map it was built against.
pub fn from_poly(p: &PolyM<Rational>, map: &PolyMap) -> Expr {
    let mut terms = Vec::with_capacity(p.terms().len());
    for (m, c) in p.terms() {
        let mut factors = vec![Expr::from(c.clone())];
        for (i, &exp) in m.exps().iter().enumerate() {
            if exp > 0 {
                factors.push(Expr::pow(map.gens[i].clone(), Expr::integer(exp as i64)));
            }
        }
        terms.push(Expr::mul(factors));
    }
    if terms.is_empty() {
        Expr::integer(0)
    } else {
        Expr::add(terms)
    }
}

/// 🔁 Converts `e` to a dense univariate `PolyU<Rational>` in `x` alone; `None` if `e` involves any
/// other generator or a non-polynomial power of `x`.
pub fn as_poly_uni(e: &Expr, x: &Expr) -> Option<PolyU<Rational>> {
    let (poly, _map) = as_poly(e, std::slice::from_ref(x))?;
    let max_deg = poly.terms().iter().map(|(m, _)| m.exps()[0] as usize).max().unwrap_or(0);
    let mut coeffs = vec![Rational::zero(); max_deg + 1];
    for (m, c) in poly.terms() {
        coeffs[m.exps()[0] as usize] = c.clone();
    }
    Some(PolyU::from_coeffs(coeffs))
}

pub fn polyu_to_expr(p: &PolyU<Rational>, x: &Expr) -> Expr {
    let map = PolyMap { gens: vec![x.clone()] };
    let terms: Vec<(mathematical_polynomial::Monomial, Rational)> = p.coeffs().iter().enumerate().map(|(i, c)| (mathematical_polynomial::Monomial::new(vec![i as u32]), c.clone())).collect();
    from_poly(&PolyM::from_terms(terms, 1, MonomialOrder::Lex), &map)
}
// #endregion 🔖ExprToPoly

// #region 🔖RationalFunctionBridge
/// 🔍 Like [`detect_gens`], but recurses through integer powers of *either* sign (rational-function
/// generators are the base, not the whole `Pow`, since `x` and `1/x` should share one generator).
fn detect_gens_ratfunc(e: &Expr) -> Vec<Expr> {
    let mut gens = Vec::new();
    collect_gens_ratfunc(e, &mut gens);
    gens
}

fn collect_gens_ratfunc(e: &Expr, gens: &mut Vec<Expr>) {
    match e.kind() {
        Kind::Integer(_) | Kind::Rational(_) => {}
        Kind::Add(terms) | Kind::Mul(terms) => {
            for t in terms {
                collect_gens_ratfunc(t, gens);
            }
        }
        Kind::Pow(base, exp) => {
            if matches!(exp.kind(), Kind::Integer(_)) {
                collect_gens_ratfunc(base, gens);
                return;
            }
            push_unique(gens, e.clone());
        }
        _ => push_unique(gens, e.clone()),
    }
}

/// 🔁 Converts `e` into a single `num/den` rational-function form over its auto-detected generators —
/// the "together" operation at the polynomial level (no GCD cancellation; see `simplify::cancel` for that).
pub fn as_ratfunc_auto(e: &Expr) -> Option<(PolyM<Rational>, PolyM<Rational>, PolyMap)> {
    let gens = detect_gens_ratfunc(e);
    let map = PolyMap { gens };
    let (num, den) = expr_to_ratfunc(e, &map)?;
    Some((num, den, map))
}

fn ratfunc_one(nvars: usize) -> PolyM<Rational> {
    PolyM::constant(Rational::one(), nvars, MonomialOrder::Lex)
}

fn expr_to_ratfunc(e: &Expr, map: &PolyMap) -> Option<(PolyM<Rational>, PolyM<Rational>)> {
    let nvars = map.gens.len().max(1);
    match e.kind() {
        Kind::Integer(n) => Some((PolyM::constant(Rational::from_integer(n.clone()), nvars, MonomialOrder::Lex), ratfunc_one(nvars))),
        Kind::Rational(r) => Some((PolyM::constant(r.clone(), nvars, MonomialOrder::Lex), ratfunc_one(nvars))),
        Kind::Add(terms) => {
            let mut num_acc = PolyM::zero(nvars, MonomialOrder::Lex);
            let mut den_acc = ratfunc_one(nvars);
            for t in terms {
                let (n, d) = expr_to_ratfunc(t, map)?;
                num_acc = num_acc.mul(&d).add(&n.mul(&den_acc));
                den_acc = den_acc.mul(&d);
            }
            Some((num_acc, den_acc))
        }
        Kind::Mul(factors) => {
            let mut num_acc = ratfunc_one(nvars);
            let mut den_acc = ratfunc_one(nvars);
            for f in factors {
                let (n, d) = expr_to_ratfunc(f, map)?;
                num_acc = num_acc.mul(&n);
                den_acc = den_acc.mul(&d);
            }
            Some((num_acc, den_acc))
        }
        Kind::Pow(base, exp) => {
            if let Kind::Integer(n) = exp.kind() {
                if let Some(ev) = n.to_i64() {
                    let (bn, bd) = expr_to_ratfunc(base, map)?;
                    return if ev >= 0 { Some((bn.pow(ev as u64), bd.pow(ev as u64))) } else { Some((bd.pow((-ev) as u64), bn.pow((-ev) as u64))) };
                }
            }
            gen_index(e, map).map(|idx| (PolyM::var(idx, nvars, MonomialOrder::Lex), ratfunc_one(nvars)))
        }
        _ => gen_index(e, map).map(|idx| (PolyM::var(idx, nvars, MonomialOrder::Lex), ratfunc_one(nvars))),
    }
}

pub fn poly_uses_var(p: &PolyM<Rational>, var: usize) -> bool {
    p.terms().iter().any(|(m, _)| m.exps()[var] > 0)
}

/// 🔁 Extracts `p` as a univariate polynomial in the single variable `var`, if none of `p`'s other
/// variables actually appear (`None` otherwise).
pub fn polym_to_polyu(p: &PolyM<Rational>, var: usize) -> Option<PolyU<Rational>> {
    let mut max_deg = 0usize;
    for (m, _) in p.terms() {
        for (i, &e) in m.exps().iter().enumerate() {
            if i != var && e > 0 {
                return None;
            }
        }
        max_deg = max_deg.max(m.exps()[var] as usize);
    }
    let mut coeffs = vec![Rational::zero(); max_deg + 1];
    for (m, c) in p.terms() {
        coeffs[m.exps()[var] as usize] = c.clone();
    }
    Some(PolyU::from_coeffs(coeffs))
}

pub fn polyu_to_polym(p: &PolyU<Rational>, var: usize, nvars: usize) -> PolyM<Rational> {
    let terms: Vec<(mathematical_polynomial::Monomial, Rational)> = p
        .coeffs()
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let mut exps = vec![0u32; nvars];
            exps[var] = i as u32;
            (mathematical_polynomial::Monomial::new(exps), c.clone())
        })
        .collect();
    PolyM::from_terms(terms, nvars, MonomialOrder::Lex)
}

/// 🔀 Rebuilds `num/den` as a canonical `Expr`, folding a constant denominator directly into `num`'s
/// coefficients rather than emitting a trivial `* 1` division.
pub fn build_ratio(num: &PolyM<Rational>, den: &PolyM<Rational>, map: &PolyMap) -> Expr {
    if den.is_zero() {
        return Expr::constant(crate::expr::Constant::ComplexInf);
    }
    if den.terms().len() == 1 && den.terms()[0].0.exps().iter().all(|&e| e == 0) {
        let c = den.terms()[0].1.clone();
        let inv = c.inv().expect("nonzero constant denominator");
        return from_poly(&num.mul_scalar(&inv), map);
    }
    Expr::mul(vec![from_poly(num, map), Expr::pow(from_poly(den, map), Expr::integer(-1))])
}
// #endregion 🔖RationalFunctionBridge

// #region 🔖RationalFactor
/// 🔍 Factors a `Rational`-coefficient univariate polynomial by clearing denominators (multiplying by
/// the LCM of every coefficient's denominator), factoring the resulting integer polynomial, and
/// converting each irreducible factor back to a monic `Rational` polynomial — folding its former
/// leading coefficient (and the clearing scale) into the returned overall constant, so
/// `overall * prod(factor_i ^ mult_i) == self` exactly.
pub fn factor_poly_u(p: &PolyU<Rational>) -> (Rational, Vec<(PolyU<Rational>, u32)>) {
    if p.is_zero() {
        return (Rational::zero(), Vec::new());
    }
    let denom_lcm = p.coeffs().iter().fold(Natural::one(), |acc, c| {
        let g = acc.gcd(c.denom());
        acc.mul(c.denom()).div_rem(&g).0
    });
    let scale = Rational::from_integer(Integer::from_natural(denom_lcm));
    let int_coeffs: Vec<Integer> = p.coeffs().iter().map(|c| c.mul(&scale).trunc()).collect();
    let int_poly = PolyU::from_coeffs(int_coeffs);
    let (content, factors) = mathematical_polynomial::factor_integer_poly(&int_poly);
    let mut overall = Rational::from_integer(content).div(&scale).expect("clearing scale is nonzero by construction");
    let mut result = Vec::with_capacity(factors.len());
    for (f, mult) in factors {
        let rat_f = PolyU::from_coeffs(f.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
        let lc = rat_f.leading_coeff().cloned().unwrap_or_else(Rational::one);
        let monic = rat_f.make_monic();
        overall = overall.mul(&lc.pow(mult as i64).expect("nonzero leading coefficient raised to a non-negative power"));
        result.push((monic, mult));
    }
    (overall, result)
}
// #endregion 🔖RationalFactor

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_gens_finds_symbols_and_functions() {
        let x = Expr::symbol("x");
        let s = Expr::func(crate::fnkind::FnKind::Sin, vec![x.clone()]);
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), s.clone()]);
        let gens = detect_gens(&e);
        assert!(gens.contains(&x));
        assert!(gens.contains(&s));
    }

    #[test]
    fn as_poly_roundtrips_through_from_poly() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(3), x.clone()]), Expr::integer(1)]);
        let (poly, map) = as_poly_auto(&e).unwrap();
        let rebuilt = from_poly(&poly, &map);
        assert_eq!(rebuilt, e);
    }

    #[test]
    fn as_poly_uni_extracts_univariate_polynomial() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(3)), Expr::integer(2)]);
        let p = as_poly_uni(&e, &x).unwrap();
        assert_eq!(p.coeff(3), Rational::one());
        assert_eq!(p.coeff(0), Rational::from_i64(2, 1).unwrap());
    }

    #[test]
    fn as_poly_uni_fails_for_other_generators() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::add(vec![x.clone(), y]);
        assert!(as_poly_uni(&e, &x).is_none());
    }

    #[test]
    fn as_ratfunc_auto_recovers_together_form() {
        let x = Expr::symbol("x");
        // 1/x + 1 -> (x + 1)/x  (structurally: num has x-degree-1 term, den has x^1 term)
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(-1)), Expr::integer(1)]);
        let (num, den, map) = as_ratfunc_auto(&e).unwrap();
        assert!(poly_uses_var(&den, gen_index(&x, &map).unwrap()));
        assert!(!num.is_zero());
    }

    #[test]
    fn factor_poly_u_recombines_to_the_original() {
        // (2x - 1)(x + 3) = 2x^2 + 5x - 3, with a rational (non-integer) leading structure once made monic.
        let f = PolyU::from_coeffs(vec![Rational::from_i64(-3, 1).unwrap(), Rational::from_i64(5, 1).unwrap(), Rational::from_i64(2, 1).unwrap()]);
        let (overall, factors) = factor_poly_u(&f);
        let mut recombined = PolyU::constant(overall);
        for (factor, mult) in &factors {
            recombined = recombined.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(recombined, f);
    }

    #[test]
    fn build_ratio_folds_constant_denominator() {
        let x = Expr::symbol("x");
        let (num, _map) = as_poly(&x, &[x.clone()]).unwrap();
        let den = PolyM::constant(Rational::from_i64(2, 1).unwrap(), 1, MonomialOrder::Lex);
        let map = PolyMap { gens: vec![x.clone()] };
        let result = build_ratio(&num, &den, &map);
        assert_eq!(result, Expr::mul(vec![Expr::from(Rational::from_i64(1, 2).unwrap()), x]));
    }
}
// #endregion 🔖Tests
