//! 🌱️ `roots` — the user's own named example ("things such as roots can be inferred"). First real
//! `impl InferredField<P>` in this codebase (grepped repo-wide: every other named inference —
//! `🧭topology`, `📦bounds`, `flat-position` — documents why it uses the plain whole-snapshot
//! `compute_X(snapshot) -> X` pattern instead; `roots` is the genuine fit the trait was designed
//! for, since real roots form a small indexed COLLECTION with no cross-root dependency, matching
//! `InferredField::Key`'s intent). `Key = usize` (index into the isolated-root list, ascending —
//! `polynomial::roots::isolate_real_roots`'s own order), no parents (roots don't depend on each
//! other). `compute()` delegates into `📈️polynomial-internals`' real Sturm-sequence isolation +
//! bisection refinement — none of that math is reimplemented here.
//!
//! Scope (honest limitation, matches `EquationNode`'s own): only equations that reduce to a
//! single-variable, INTEGER-coefficient polynomial (`Add`/`Mul`/`Pow(Symbol, IntegerLiteral)`/
//! `Integer`/`Rational` with `denom == 1`) produce roots; anything else (rational coefficients,
//! multiple variables, `Fn`/`Piecewise`/etc.) plans ZERO steps — an empty root list, not a panic
//! or a wrong answer. Extending this to the full `cas::rootof`/`cas::solve`/
//! `polynomial::algebraic` machinery (irrational/complex roots, symbolic closed forms) is future
//! work once the mutation/inference table grows past this vertical slice.

use crate::standards::v1::subsets::any::schema::snapshot::{EquationNode, EquationNodeKind, EquationExprSnapshot};
use crate::EquationSnapshot;
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::BTreeMap;

//#region 🔖️IntegerPolynomialExtraction
/// 🌉 Structural walk of `EquationNode` → `(variable name, degree -> integer coefficient)` — `None`
/// the instant the tree leaves this wave's scope (rational coefficient, second variable, non-integer
/// exponent, or any `Fn`/`Piecewise`/etc node — `EquationNode` can't even represent those yet).
fn extract_integer_polynomial(node: &EquationNode) -> Option<(String, BTreeMap<u32, number::Integer>)> {
    fn walk(node: &EquationNode, var: &mut Option<String>) -> Option<BTreeMap<u32, number::Integer>> {
        match &node.kind {
            EquationNodeKind::Integer { lexeme } => Some(BTreeMap::from([(0u32, lexeme.parse().ok()?)])),
            EquationNodeKind::Rational { numer, denom } => {
                if denom == "1" {
                    Some(BTreeMap::from([(0u32, numer.parse().ok()?)]))
                } else {
                    None
                }
            }
            EquationNodeKind::Symbol { name } => {
                match var {
                    None => *var = Some(name.clone()),
                    Some(existing) if existing != name => return None,
                    Some(_) => {}
                }
                Some(BTreeMap::from([(1u32, number::Integer::one())]))
            }
            EquationNodeKind::Add { terms } => {
                let mut acc: BTreeMap<u32, number::Integer> = BTreeMap::new();
                for term in terms {
                    for (degree, coeff) in walk(term, var)? {
                        let entry = acc.entry(degree).or_insert_with(number::Integer::zero);
                        *entry = entry.add(&coeff);
                    }
                }
                Some(acc)
            }
            EquationNodeKind::Mul { factors } => {
                let mut acc: BTreeMap<u32, number::Integer> = BTreeMap::from([(0u32, number::Integer::one())]);
                for factor in factors {
                    let rhs = walk(factor, var)?;
                    let mut next: BTreeMap<u32, number::Integer> = BTreeMap::new();
                    for (deg_a, coeff_a) in &acc {
                        for (deg_b, coeff_b) in &rhs {
                            let entry = next.entry(deg_a + deg_b).or_insert_with(number::Integer::zero);
                            *entry = entry.add(&coeff_a.mul(coeff_b));
                        }
                    }
                    acc = next;
                }
                Some(acc)
            }
            EquationNodeKind::Pow { base, exponent } => {
                let EquationNodeKind::Integer { lexeme } = &exponent.kind else { return None };
                let exp: u32 = lexeme.parse().ok()?;
                let EquationNodeKind::Symbol { name } = &base.kind else { return None };
                match var {
                    None => *var = Some(name.clone()),
                    Some(existing) if existing != name => return None,
                    Some(_) => {}
                }
                Some(BTreeMap::from([(exp, number::Integer::one())]))
            }
        }
    }
    let mut var = None;
    let coeffs = walk(node, &mut var)?;
    Some((var.unwrap_or_else(|| "x".to_string()), coeffs))
}

fn to_poly_u(coeffs: &BTreeMap<u32, number::Integer>) -> crate::polynomial::univariate::PolyU<number::Integer> {
    let mut poly = crate::polynomial::univariate::PolyU::zero();
    for (degree, coeff) in coeffs {
        poly = poly.add(&crate::polynomial::univariate::PolyU::monomial(coeff.clone(), *degree as usize));
    }
    poly
}

/// 🌉 `None` when `equation` is outside this wave's scope (see module doc) — the ONE place
/// `plan`/`dep_input`/`compute` all funnel through, so all three always agree on scope.
fn equation_integer_polynomial(equation: &EquationExprSnapshot) -> Option<crate::polynomial::univariate::PolyU<number::Integer>> {
    let (_, coeffs) = extract_integer_polynomial(&equation.expr)?;
    if coeffs.is_empty() {
        return None;
    }
    Some(to_poly_u(&coeffs))
}
//#endregion 🔖️IntegerPolynomialExtraction

//#region 🔖️RootValue
/// 🌱️ One isolated-then-refined real root, as a decimal `f64` approximation — the refined rational
/// interval's midpoint, narrowed to `REFINE_WIDTH`.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EquationRoot {
    pub approx: f64,
}
//#endregion 🔖️RootValue

//#region 🔖️InferredField
/// 🌱️ The bisection target width every `compute()` call refines to — `1 / 10^9`, matching the
/// precision the migrated `polynomial::algebraic` tests already assert against (`1e-6`/`1e-9`
/// tolerances), so `roots`' output is at least as precise as what those tests already trust.
fn refine_width() -> number::Rational {
    number::Rational::new(number::Integer::one(), number::Integer::from_i64(1_000_000_000)).expect("1/10^9 is a valid rational")
}

pub struct EquationRootsField;

impl protocol::InferredField<EquationSnapshot> for EquationRootsField {
    type Key = usize;
    type Value = EquationRoot;

    const FIELD_ID: &'static str = "s.mathematical.equation.inference.roots";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["equation"]
    }

    /// 🧭️ Isolates once to learn how many real roots exist (Sturm-sequence sign-change counting —
    /// `polynomial::roots::isolate_real_roots`) and plans one step per index, no parents: roots of
    /// the same polynomial don't depend on each other's values.
    fn plan(snapshot: &EquationSnapshot) -> Vec<protocol::InferenceStep<Self::Key>> {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return Vec::new() };
        (0..crate::polynomial::roots::isolate_real_roots(&poly).len()).map(|index| protocol::InferenceStep { key: index, parents: vec![] }).collect()
    }

    /// 🔑️ The polynomial's own coefficients (so ANY edit to ANY coefficient invalidates every
    /// root's cache entry, not just the one nearest that coefficient — a real polynomial's roots
    /// are a global function of ALL coefficients, unlike `flat-position`'s local per-edge deps) plus
    /// this key's isolating interval (so a coefficient edit that shifts WHICH interval index `key`
    /// lands on also invalidates, even if the isolation count happens to stay the same).
    fn dep_input(snapshot: &EquationSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return Vec::new() };
        let mut bytes = Vec::new();
        for coeff in poly.coeffs() {
            bytes.extend_from_slice(coeff.to_string().as_bytes());
            bytes.push(0);
        }
        if let Some((lo, hi)) = crate::polynomial::roots::isolate_real_roots(&poly).get(*key) {
            bytes.extend_from_slice(lo.to_string().as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(hi.to_string().as_bytes());
        }
        bytes
    }

    /// 🧮️ Re-isolates (cheap relative to refinement — Sturm sequences over small integer
    /// polynomials) and bisects the `key`-th interval down to `refine_width()`, returning the
    /// refined interval's midpoint as `f64`.
    fn compute(snapshot: &EquationSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return EquationRoot::default() };
        let intervals = crate::polynomial::roots::isolate_real_roots(&poly);
        let Some((lo, hi)) = intervals.get(*key) else { return EquationRoot::default() };
        let (lo, hi) = crate::polynomial::roots::refine_root(&poly, lo, hi, &refine_width());
        let midpoint = lo.add(&hi).div(&number::Rational::from_i64(2, 1).expect("2/1 is valid")).unwrap_or_else(number::Rational::zero);
        EquationRoot { approx: midpoint.to_f64() }
    }
}

/// 🌱️ Assembles the whole `roots` field via `protocol::infer_field` — the real dependency-hash-
/// chained plan/compute orchestration `InferredField` exists for, not a hand-rolled loop over
/// `plan()`/`compute()`. Returns a plain ascending `Vec` (index order) for `EquationInference`.
pub fn compute_equation_roots(snapshot: &EquationSnapshot) -> Vec<EquationRoot> {
    let values = protocol::infer_field::<EquationSnapshot, EquationRootsField>(snapshot, None);
    values.into_values().collect()
}
//#endregion 🔖️InferredField

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
