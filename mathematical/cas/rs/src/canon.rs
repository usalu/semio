//! 🧪 Smart constructors: the only place `Kind` values are ever built raw. Every rule here
//! (flattening, numeric folding, like-term/like-base collection, identity/absorber elimination, `Pow`
//! special cases, canonical sorting) is an invariant the rest of the crate is entitled to assume holds.

use crate::assume::{is_positive, AssumeSet};
use crate::expr::{Constant, Expr, Kind, Symbol};
use crate::fnkind::FnKind;
use mathematical_number::{primes, Integer, Natural, Rational};
use std::collections::BTreeMap;

// #region 🔖Leaves
pub(crate) fn make_symbol(name: &str, assumptions: AssumeSet) -> Expr {
    Expr::from_kind_unchecked(Kind::Symbol(Symbol::new(name, assumptions.close())))
}

pub(crate) fn make_integer(n: Integer) -> Expr {
    Expr::from_kind_unchecked(Kind::Integer(n))
}

/// ➗ Folds to `Integer` whenever the value is integral — `Kind::Rational` is only ever constructed
/// for genuinely non-integral values.
pub(crate) fn make_rational(r: Rational) -> Expr {
    if r.is_integer() {
        make_integer(r.trunc())
    } else {
        Expr::from_kind_unchecked(Kind::Rational(r))
    }
}

pub(crate) fn make_neg(e: Expr) -> Expr {
    make_mul(vec![make_integer(Integer::from_i64(-1)), e])
}
// #endregion 🔖Leaves

// #region 🔖NumberFolding
/// 🔢 An internal add/mul accumulator over the numeric subset of the kernel (`Integer`/`Rational`).
#[derive(Clone)]
enum Num {
    Int(Integer),
    Rat(Rational),
}

impl Num {
    fn zero() -> Self {
        Num::Int(Integer::zero())
    }
    fn one() -> Self {
        Num::Int(Integer::one())
    }
    fn from_expr(e: &Expr) -> Option<Self> {
        match e.kind() {
            Kind::Integer(n) => Some(Num::Int(n.clone())),
            Kind::Rational(r) => Some(Num::Rat(r.clone())),
            _ => None,
        }
    }
    fn to_rational(&self) -> Rational {
        match self {
            Num::Int(n) => Rational::from_integer(n.clone()),
            Num::Rat(r) => r.clone(),
        }
    }
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a.add(b)),
            _ => Num::Rat(self.to_rational().add(&other.to_rational())),
        }
    }
    fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a.mul(b)),
            _ => Num::Rat(self.to_rational().mul(&other.to_rational())),
        }
    }
    fn is_zero(&self) -> bool {
        match self {
            Num::Int(n) => n.is_zero(),
            Num::Rat(r) => r.is_zero(),
        }
    }
    fn is_one(&self) -> bool {
        match self {
            Num::Int(n) => *n == Integer::one(),
            Num::Rat(r) => *r == Rational::one(),
        }
    }
    fn is_negative(&self) -> bool {
        match self {
            Num::Int(n) => n.is_negative(),
            Num::Rat(r) => r.numer().is_negative(),
        }
    }
    fn into_expr(self) -> Expr {
        match self {
            Num::Int(n) => make_integer(n),
            Num::Rat(r) => make_rational(r),
        }
    }
}
// #endregion 🔖NumberFolding

// #region 🔖Add
pub(crate) fn make_add(terms: Vec<Expr>) -> Expr {
    let mut flat = Vec::with_capacity(terms.len());
    flatten_add(terms, &mut flat);

    if flat.iter().any(|t| matches!(t.kind(), Kind::Constant(Constant::Undefined))) {
        return Expr::constant(Constant::Undefined);
    }
    let inf_count = flat.iter().filter(|t| matches!(t.kind(), Kind::Constant(Constant::Inf))).count();
    let neg_inf_count = flat.iter().filter(|t| matches!(t.kind(), Kind::Constant(Constant::NegInf))).count();
    if inf_count > 0 && neg_inf_count > 0 {
        return Expr::constant(Constant::Undefined);
    }
    if inf_count > 0 {
        return Expr::constant(Constant::Inf);
    }
    if neg_inf_count > 0 {
        return Expr::constant(Constant::NegInf);
    }

    let mut numeric = Num::zero();
    let mut by_rest: BTreeMap<Expr, Num> = BTreeMap::new();
    for term in flat {
        if let Some(n) = Num::from_expr(&term) {
            numeric = numeric.add(&n);
            continue;
        }
        let (coeff, rest) = split_coefficient(&term);
        by_rest.entry(rest).and_modify(|c| *c = c.add(&coeff)).or_insert(coeff);
    }

    let mut result_terms: Vec<Expr> = Vec::new();
    if !numeric.is_zero() {
        result_terms.push(numeric.into_expr());
    }
    for (rest, coeff) in by_rest {
        if coeff.is_zero() {
            continue;
        }
        if coeff.is_one() {
            result_terms.push(rest);
        } else {
            result_terms.push(make_mul(vec![coeff.into_expr(), rest]));
        }
    }

    finalize_variadic(result_terms, Integer::zero(), Kind::Add)
}

fn flatten_add(terms: Vec<Expr>, out: &mut Vec<Expr>) {
    for t in terms {
        match t.kind() {
            Kind::Add(inner) => flatten_add(inner.clone(), out),
            _ => out.push(t),
        }
    }
}

/// ➗ Splits a term into `(numeric_coefficient, rest)` for like-term collection: a `Mul` with a
/// leading numeric factor contributes that factor as the coefficient and the remaining factors
/// (re-multiplied) as `rest`; anything else has an implicit coefficient of `1`.
fn split_coefficient(term: &Expr) -> (Num, Expr) {
    if let Kind::Mul(factors) = term.kind() {
        if let Some(first_num) = factors.first().and_then(Num::from_expr) {
            let rest_factors: Vec<Expr> = factors[1..].to_vec();
            let rest = make_mul(rest_factors);
            return (first_num, rest);
        }
    }
    (Num::one(), term.clone())
}
// #endregion 🔖Add

// #region 🔖Mul
pub(crate) fn make_mul(factors: Vec<Expr>) -> Expr {
    let mut flat = Vec::with_capacity(factors.len());
    flatten_mul(factors, &mut flat);

    if flat.iter().any(|t| matches!(t.kind(), Kind::Constant(Constant::Undefined))) {
        return Expr::constant(Constant::Undefined);
    }

    let mut numeric = Num::one();
    let mut symbolic: Vec<Expr> = Vec::new();
    for f in flat {
        if let Some(n) = Num::from_expr(&f) {
            numeric = numeric.mul(&n);
        } else {
            symbolic.push(f);
        }
    }

    // `zoo` (directionless complex infinity) swallows every finite factor's sign, so it must be
    // checked before the signed `oo`/`-oo` contagion below: `0 * zoo` is still `Undefined`, but
    // `zoo * -oo` (or any other signed infinity) collapses to `zoo`, not a signed infinity.
    if symbolic.iter().any(|t| matches!(t.kind(), Kind::Constant(Constant::ComplexInf))) {
        return if numeric.is_zero() { Expr::constant(Constant::Undefined) } else { Expr::constant(Constant::ComplexInf) };
    }

    let inf_terms: Vec<&Expr> = symbolic.iter().filter(|t| matches!(t.kind(), Kind::Constant(Constant::Inf) | Kind::Constant(Constant::NegInf))).collect();
    if !inf_terms.is_empty() {
        if numeric.is_zero() {
            return Expr::constant(Constant::Undefined);
        }
        let mut sign_negative = numeric.is_negative();
        for t in &inf_terms {
            if matches!(t.kind(), Kind::Constant(Constant::NegInf)) {
                sign_negative = !sign_negative;
            }
        }
        return Expr::constant(if sign_negative { Constant::NegInf } else { Constant::Inf });
    }

    if numeric.is_zero() {
        return Expr::integer(0);
    }

    // Combine like bases: x^a * x^b -> x^(a+b).
    let mut by_base: BTreeMap<Expr, Expr> = BTreeMap::new();
    let mut base_order: Vec<Expr> = Vec::new();
    for f in symbolic {
        let (base, exp) = match f.kind() {
            Kind::Pow(b, e) => (b.clone(), e.clone()),
            _ => (f.clone(), Expr::integer(1)),
        };
        if let Some(existing) = by_base.get(&base).cloned() {
            by_base.insert(base, make_add(vec![existing, exp]));
        } else {
            base_order.push(base.clone());
            by_base.insert(base, exp);
        }
    }

    let mut result_terms: Vec<Expr> = Vec::new();
    if !numeric.is_one() {
        result_terms.push(numeric.into_expr());
    }
    for base in base_order {
        let exp = by_base.remove(&base).unwrap();
        result_terms.push(make_pow(base, exp));
    }
    // Any newly-introduced numeric terms from pow folding (e.g. 2^3 -> 8) must be re-merged.
    if result_terms.iter().filter(|t| Num::from_expr(t).is_some()).count() > 1 {
        return make_mul(result_terms);
    }

    finalize_variadic(result_terms, Integer::one(), Kind::Mul)
}

fn flatten_mul(factors: Vec<Expr>, out: &mut Vec<Expr>) {
    for f in factors {
        match f.kind() {
            Kind::Mul(inner) => flatten_mul(inner.clone(), out),
            _ => out.push(f),
        }
    }
}

/// 🧹 Common tail for `Add`/`Mul`: drop the identity element (dead code by construction upstream is
/// avoided since numeric folding already strips it), sort canonically, and collapse
/// empty/singleton results.
fn finalize_variadic(mut terms: Vec<Expr>, identity: Integer, wrap: fn(Vec<Expr>) -> Kind) -> Expr {
    terms.retain(|t| !matches!(t.kind(), Kind::Integer(n) if *n == identity));
    if terms.is_empty() {
        return make_integer(identity);
    }
    if terms.len() == 1 {
        return terms.into_iter().next().unwrap();
    }
    terms.sort();
    Expr::from_kind_unchecked(wrap(terms))
}
// #endregion 🔖Mul

// #region 🔖Pow
pub(crate) fn make_pow(base: Expr, exp: Expr) -> Expr {
    if matches!(base.kind(), Kind::Constant(Constant::Undefined)) || matches!(exp.kind(), Kind::Constant(Constant::Undefined)) {
        return Expr::constant(Constant::Undefined);
    }
    if exp.is_zero_literal() {
        return Expr::integer(1);
    }
    if exp.is_one_literal() {
        return base;
    }
    if base.is_one_literal() {
        return Expr::integer(1);
    }
    if base.is_zero_literal() {
        return match exp.kind() {
            Kind::Integer(n) if n.is_positive() => Expr::integer(0),
            Kind::Integer(n) if n.is_negative() => Expr::constant(Constant::ComplexInf),
            Kind::Rational(r) if r.numer().is_positive() => Expr::integer(0),
            Kind::Rational(r) if r.numer().is_negative() => Expr::constant(Constant::ComplexInf),
            _ => Expr::from_kind_unchecked(Kind::Pow(base, exp)),
        };
    }

    // i^n cycles with period 4.
    if matches!(base.kind(), Kind::Constant(Constant::I)) {
        if let Kind::Integer(n) = exp.kind() {
            if let Some(e) = n.to_i64() {
                let m = e.rem_euclid(4);
                return match m {
                    0 => Expr::integer(1),
                    1 => Expr::constant(Constant::I),
                    2 => Expr::integer(-1),
                    _ => make_neg(Expr::constant(Constant::I)),
                };
            }
        }
    }

    // (x^a)^b -> x^(a*b) when b is an integer, or when x is known positive.
    if let Kind::Pow(inner_base, inner_exp) = base.kind() {
        let b_is_integer = matches!(exp.kind(), Kind::Integer(_));
        if b_is_integer || is_positive(&inner_base.clone()) == Some(true) {
            let combined_exp = make_mul(vec![inner_exp.clone(), exp.clone()]);
            return make_pow(inner_base.clone(), combined_exp);
        }
    }

    match (base.kind(), exp.kind()) {
        (Kind::Integer(b), Kind::Integer(e)) => fold_integer_pow(b.clone(), e.clone()),
        (Kind::Rational(b), Kind::Integer(e)) => {
            if let Some(ev) = e.to_i64() {
                if let Some(result) = b.pow(ev) {
                    return make_rational(result);
                }
            }
            Expr::from_kind_unchecked(Kind::Pow(base, exp))
        }
        (Kind::Integer(b), Kind::Rational(e)) => fold_radical(b.clone(), e.clone()).unwrap_or_else(|| Expr::from_kind_unchecked(Kind::Pow(base, exp))),
        _ => Expr::from_kind_unchecked(Kind::Pow(base, exp)),
    }
}

fn fold_integer_pow(base: Integer, exp: Integer) -> Expr {
    let Some(ev) = exp.to_i64() else {
        return Expr::from_kind_unchecked(Kind::Pow(make_integer(base), make_integer(exp)));
    };
    if ev >= 0 {
        make_integer(base.pow(ev as u64))
    } else {
        let positive = base.pow((-ev) as u64);
        match Rational::new(Integer::one(), positive) {
            Some(r) => make_rational(r),
            None => Expr::constant(Constant::ComplexInf),
        }
    }
}

/// 🌱 Radical extraction for `base^(p/q)` (`base` a nonzero integer, `q >= 2`): factors `|base|` into
/// `outside^q * inside` (via prime factorization) with `inside` free of `q`-th-power factors, giving
/// `base^(1/q) = (+-outside) * inside^(1/q)`. Only the numerator-`1` case gets this partial extraction;
/// other numerators only fold when `base` is an *exact* `q`-th power (documented simplification).
fn fold_radical(base: Integer, exp: Rational) -> Option<Expr> {
    let q = exp.denom().to_u64()? as u32;
    let p = exp.numer().clone();
    if q < 2 {
        return None;
    }
    let is_neg = base.is_negative();
    if q % 2 == 0 && is_neg {
        return None; // even root of a negative number: not real, leave symbolic
    }
    let bm = base.abs();
    let factors = primes::factor(&bm);
    let mut outside = Natural::one();
    let mut inside = Natural::one();
    for (prime, e) in factors {
        let outside_exp = e / q;
        let inside_exp = e % q;
        outside = outside.mul(&prime.pow(outside_exp as u64));
        inside = inside.mul(&prime.pow(inside_exp as u64));
    }
    let p_abs = p.abs().to_u64()?;
    if inside == Natural::one() {
        // Exact q-th power: base^(p/q) folds fully to an integer/rational.
        let sign_factor = if is_neg && p_abs % 2 == 1 { -1i64 } else { 1i64 };
        let magnitude = outside.pow(p_abs);
        let signed = if sign_factor < 0 { Integer::from_natural(magnitude).neg() } else { Integer::from_natural(magnitude) };
        if p.is_negative() {
            return Rational::new(Integer::one(), signed).map(make_rational);
        }
        return Some(make_integer(signed));
    }
    if p_abs == 1 && outside != Natural::one() {
        let outside_signed = if is_neg { Integer::from_natural(outside).neg() } else { Integer::from_natural(outside) };
        let inside_expr = make_integer(Integer::from_natural(inside));
        let remainder_exp = if p.is_negative() { exp.neg() } else { exp };
        let remainder = make_pow(inside_expr, make_rational(remainder_exp));
        return Some(make_mul(vec![make_integer(outside_signed), remainder]));
    }
    None
}
// #endregion 🔖Pow

// #region 🔖Func
pub(crate) fn make_func(kind: FnKind, args: Vec<Expr>) -> Expr {
    if let Some(arity) = kind.arity() {
        debug_assert_eq!(args.len(), arity, "make_func: wrong arity for {kind:?}");
    }
    if args.iter().any(|a| matches!(a.kind(), Kind::Constant(Constant::Undefined))) {
        return Expr::constant(Constant::Undefined);
    }
    if args.len() == 1 {
        if let Some(folded) = fold_unary_special_value(&kind, &args[0]) {
            return folded;
        }
    }
    Expr::from_kind_unchecked(Kind::Fn(kind, args))
}

fn fold_unary_special_value(kind: &FnKind, arg: &Expr) -> Option<Expr> {
    match kind {
        FnKind::Sin | FnKind::Tan | FnKind::Asin | FnKind::Atan | FnKind::Sinh | FnKind::Tanh | FnKind::Asinh | FnKind::Atanh if arg.is_zero_literal() => Some(Expr::integer(0)),
        FnKind::Cos | FnKind::Cosh if arg.is_zero_literal() => Some(Expr::integer(1)),
        FnKind::Exp if arg.is_zero_literal() => Some(Expr::integer(1)),
        FnKind::Ln if arg.is_one_literal() => Some(Expr::integer(0)),
        FnKind::Abs => match arg.kind() {
            Kind::Integer(n) => Some(make_integer(n.abs_integer())),
            Kind::Rational(r) => Some(make_rational(r.abs())),
            _ => None,
        },
        FnKind::Sign => match arg.kind() {
            Kind::Integer(n) => Some(Expr::integer(n.signum() as i64)),
            _ => None,
        },
        _ => None,
    }
}
// #endregion 🔖Func

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_folds_numeric_literals() {
        let e = make_add(vec![Expr::integer(2), Expr::integer(3)]);
        assert_eq!(e, Expr::integer(5));
    }

    #[test]
    fn add_collects_like_terms() {
        let x = Expr::symbol("x");
        let e = make_add(vec![x.clone(), x.clone()]);
        let expected = make_mul(vec![Expr::integer(2), x]);
        assert_eq!(e, expected);
    }

    #[test]
    fn add_drops_zero() {
        let x = Expr::symbol("x");
        let e = make_add(vec![x.clone(), Expr::integer(0)]);
        assert_eq!(e, x);
    }

    #[test]
    fn mul_folds_numeric_literals() {
        let e = make_mul(vec![Expr::integer(2), Expr::integer(3)]);
        assert_eq!(e, Expr::integer(6));
    }

    #[test]
    fn mul_combines_like_bases() {
        let x = Expr::symbol("x");
        let e = make_mul(vec![x.clone(), x.clone()]);
        let expected = make_pow(x, Expr::integer(2));
        assert_eq!(e, expected);
    }

    #[test]
    fn mul_by_zero_absorbs() {
        let x = Expr::symbol("x");
        let e = make_mul(vec![x, Expr::integer(0)]);
        assert_eq!(e, Expr::integer(0));
    }

    #[test]
    fn mul_zero_times_complex_infinity_is_undefined() {
        let e = make_mul(vec![Expr::integer(0), Expr::constant(Constant::ComplexInf)]);
        assert_eq!(e, Expr::constant(Constant::Undefined));
    }

    #[test]
    fn mul_nonzero_times_complex_infinity_is_complex_infinity() {
        let e = make_mul(vec![Expr::integer(5), Expr::constant(Constant::ComplexInf)]);
        assert_eq!(e, Expr::constant(Constant::ComplexInf));
    }

    #[test]
    fn pow_identities() {
        let x = Expr::symbol("x");
        assert_eq!(make_pow(x.clone(), Expr::integer(0)), Expr::integer(1));
        assert_eq!(make_pow(x.clone(), Expr::integer(1)), x);
        assert_eq!(make_pow(Expr::integer(1), x.clone()), Expr::integer(1));
    }

    #[test]
    fn pow_integer_folds_exactly() {
        assert_eq!(make_pow(Expr::integer(2), Expr::integer(10)), Expr::integer(1024));
    }

    #[test]
    fn pow_negative_integer_exponent_gives_rational() {
        let e = make_pow(Expr::integer(2), Expr::integer(-1));
        assert_eq!(e, make_rational(Rational::from_i64(1, 2).unwrap()));
    }

    #[test]
    fn radical_partial_extraction_matches_plan_example() {
        // 8^(1/2) -> 2 * 2^(1/2)
        let e = make_pow(Expr::integer(8), make_rational(Rational::from_i64(1, 2).unwrap()));
        let expected = make_mul(vec![Expr::integer(2), make_pow(Expr::integer(2), make_rational(Rational::from_i64(1, 2).unwrap()))]);
        assert_eq!(e, expected);
    }

    #[test]
    fn radical_exact_perfect_power_folds_fully() {
        // 4^(1/2) -> 2
        let e = make_pow(Expr::integer(4), make_rational(Rational::from_i64(1, 2).unwrap()));
        assert_eq!(e, Expr::integer(2));
    }

    #[test]
    fn radical_of_prime_stays_symbolic() {
        let e = make_pow(Expr::integer(2), make_rational(Rational::from_i64(1, 2).unwrap()));
        assert!(matches!(e.kind(), Kind::Pow(..)));
    }

    #[test]
    fn nested_pow_combines_exponents_for_integer_outer_exponent() {
        let x = Expr::symbol("x");
        let inner = make_pow(x.clone(), Expr::integer(2));
        let outer = make_pow(inner, Expr::integer(3));
        assert_eq!(outer, make_pow(x, Expr::integer(6)));
    }

    #[test]
    fn i_power_cycles_with_period_four() {
        let i = Expr::constant(Constant::I);
        assert_eq!(make_pow(i.clone(), Expr::integer(0)), Expr::integer(1));
        assert_eq!(make_pow(i.clone(), Expr::integer(1)), i);
        assert_eq!(make_pow(i.clone(), Expr::integer(2)), Expr::integer(-1));
        assert_eq!(make_pow(i.clone(), Expr::integer(4)), Expr::integer(1));
    }

    #[test]
    fn func_special_values_fold() {
        assert_eq!(make_func(FnKind::Sin, vec![Expr::integer(0)]), Expr::integer(0));
        assert_eq!(make_func(FnKind::Cos, vec![Expr::integer(0)]), Expr::integer(1));
        assert_eq!(make_func(FnKind::Exp, vec![Expr::integer(0)]), Expr::integer(1));
    }

    #[test]
    fn canonicalization_is_idempotent_on_a_small_corpus() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let corpus = vec![
            make_add(vec![x.clone(), y.clone(), Expr::integer(3)]),
            make_mul(vec![x.clone(), y.clone(), Expr::integer(2)]),
            make_pow(x.clone(), Expr::integer(5)),
            make_add(vec![make_mul(vec![Expr::integer(2), x.clone()]), make_mul(vec![Expr::integer(3), x])]),
        ];
        for e in corpus {
            // Rebuilding from the same top-level kind should reproduce exactly the same expression.
            let rebuilt = match e.kind() {
                Kind::Add(terms) => make_add(terms.clone()),
                Kind::Mul(factors) => make_mul(factors.clone()),
                Kind::Pow(b, ex) => make_pow(b.clone(), ex.clone()),
                _ => e.clone(),
            };
            assert_eq!(e, rebuilt);
        }
    }
}
// #endregion 🔖Tests
