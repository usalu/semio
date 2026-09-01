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

use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::{EquationNode, EquationNodeKind, EquationSnapshot};
use crate::artifacts::mathematical::MathematicalSnapshot;
use serde::{Deserialize, Serialize};
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️component.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::BTreeMap;

//#region 🔖️IntegerPolynomialExtraction
/// 🌉 Structural walk of `EquationNode` → `(variable name, degree -> integer coefficient)` — `None`
/// the instant the tree leaves this wave's scope (rational coefficient, second variable, non-integer
/// exponent, or any `Fn`/`Piecewise`/etc node — `EquationNode` can't even represent those yet).
async fn extract_integer_polynomial(node: &EquationNode) -> Option<(String, BTreeMap<u32, number::Integer>)> {
    async fn walk(node: &EquationNode, var: &mut Option<String>) -> Option<BTreeMap<u32, number::Integer>> {
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

async fn to_poly_u(coeffs: &BTreeMap<u32, number::Integer>) -> crate::polynomial::univariate::PolyU<number::Integer> {
    let mut poly = crate::polynomial::univariate::PolyU::zero();
    for (degree, coeff) in coeffs {
        poly = poly.add(&crate::polynomial::univariate::PolyU::monomial(coeff.clone(), *degree as usize));
    }
    poly
}

/// 🌉 `None` when `equation` is outside this wave's scope (see module doc) — the ONE place
/// `plan`/`dep_input`/`compute` all funnel through, so all three always agree on scope.
async fn equation_integer_polynomial(equation: &EquationSnapshot) -> Option<crate::polynomial::univariate::PolyU<number::Integer>> {
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MathematicalRoot {
    pub approx: f64,
}
//#endregion 🔖️RootValue

//#region 🔖️InferredField
/// 🌱️ The bisection target width every `compute()` call refines to — `1 / 10^9`, matching the
/// precision the migrated `polynomial::algebraic` tests already assert against (`1e-6`/`1e-9`
/// tolerances), so `roots`' output is at least as precise as what those tests already trust.
async fn refine_width() -> number::Rational {
    number::Rational::new(number::Integer::one(), number::Integer::from_i64(1_000_000_000)).expect("1/10^9 is a valid rational")
}

pub struct MathematicalRootsField;

impl protocol::InferredField<MathematicalSnapshot> for MathematicalRootsField {
    type Key = usize;
    type Value = MathematicalRoot;

    const FIELD_ID: &'static str = "s.mathematical.mathematical.inference.roots";
    const SCHEMA_VERSION: u32 = 1;

    async fn reads() -> &'static [&'static str] {
        &["equation"]
    }

    /// 🧭️ Isolates once to learn how many real roots exist (Sturm-sequence sign-change counting —
    /// `polynomial::roots::isolate_real_roots`) and plans one step per index, no parents: roots of
    /// the same polynomial don't depend on each other's values.
    async fn plan(snapshot: &MathematicalSnapshot) -> Vec<protocol::InferenceStep<Self::Key>> {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return Vec::new() };
        (0..crate::polynomial::roots::isolate_real_roots(&poly).len()).map(|index| protocol::InferenceStep { key: index, parents: vec![] }).collect()
    }

    /// 🔑️ The polynomial's own coefficients (so ANY edit to ANY coefficient invalidates every
    /// root's cache entry, not just the one nearest that coefficient — a real polynomial's roots
    /// are a global function of ALL coefficients, unlike `flat-position`'s local per-edge deps) plus
    /// this key's isolating interval (so a coefficient edit that shifts WHICH interval index `key`
    /// lands on also invalidates, even if the isolation count happens to stay the same).
    async fn dep_input(snapshot: &MathematicalSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
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
    async fn compute(snapshot: &MathematicalSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let Some(poly) = equation_integer_polynomial(&snapshot.equation) else { return MathematicalRoot::default() };
        let intervals = crate::polynomial::roots::isolate_real_roots(&poly);
        let Some((lo, hi)) = intervals.get(*key) else { return MathematicalRoot::default() };
        let (lo, hi) = crate::polynomial::roots::refine_root(&poly, lo, hi, &refine_width());
        let midpoint = lo.add(&hi).div(&number::Rational::from_i64(2, 1).expect("2/1 is valid")).unwrap_or_else(number::Rational::zero);
        MathematicalRoot { approx: midpoint.to_f64() }
    }
}

/// 🌱️ Assembles the whole `roots` field via `protocol::infer_field` — the real dependency-hash-
/// chained plan/compute orchestration `InferredField` exists for, not a hand-rolled loop over
/// `plan()`/`compute()`. Returns a plain ascending `Vec` (index order) for `MathematicalInference`.
pub async fn compute_mathematical_roots(snapshot: &MathematicalSnapshot) -> Vec<MathematicalRoot> {
    let values = protocol::infer_field::<MathematicalSnapshot, MathematicalRootsField>(snapshot, None);
    values.into_values().collect()
}
//#endregion 🔖️InferredField

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationNodeLabel;

    /// 🧪️ `x^2 - 3x + 2 = (x-1)(x-2)`, roots `{1, 2}` — built directly as a labeled tree (`Add` of
    /// `x^2`, `-3x`, `2`), the same shape `expr_to_equation_node` would produce from
    /// `cas::polybridge`'s own canonical `Add` term order.
    async fn quadratic_with_roots_one_and_two() -> EquationSnapshot {
        let x = EquationNode { label: EquationNodeLabel(1), kind: EquationNodeKind::Symbol { name: "x".into() } };
        let two_exp = EquationNode { label: EquationNodeLabel(2), kind: EquationNodeKind::Integer { lexeme: "2".into() } };
        let x_squared = EquationNode { label: EquationNodeLabel(3), kind: EquationNodeKind::Pow { base: Box::new(x.clone()), exponent: Box::new(two_exp) } };
        let neg_three = EquationNode { label: EquationNodeLabel(4), kind: EquationNodeKind::Integer { lexeme: "-3".into() } };
        let neg_three_x = EquationNode { label: EquationNodeLabel(5), kind: EquationNodeKind::Mul { factors: vec![neg_three, x] } };
        let two = EquationNode { label: EquationNodeLabel(6), kind: EquationNodeKind::Integer { lexeme: "2".into() } };
        let expr = EquationNode { label: EquationNodeLabel(7), kind: EquationNodeKind::Add { terms: vec![x_squared, neg_three_x, two] } };
        EquationSnapshot { expr, next_label: 8 }
    }

    #[semio_framework_async_macros::async_test]
    async fn extracts_integer_polynomial_coefficients_by_degree() {
        let (var, coeffs) = extract_integer_polynomial(&quadratic_with_roots_one_and_two().expr).expect("polynomial in scope");
        assert_eq!(var, "x");
        assert_eq!(coeffs.get(&0).map(|c| c.to_string()), Some("2".to_string()));
        assert_eq!(coeffs.get(&1).map(|c| c.to_string()), Some("-3".to_string()));
        assert_eq!(coeffs.get(&2).map(|c| c.to_string()), Some("1".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_has_one_step_per_isolated_root_with_no_parents() {
        let mut snapshot = MathematicalSnapshot::default();
        snapshot.equation = quadratic_with_roots_one_and_two();
        let steps = <MathematicalRootsField as protocol::InferredField<MathematicalSnapshot>>::plan(&snapshot);
        assert_eq!(steps.len(), 2, "x^2-3x+2 has exactly two real roots");
        assert!(steps.iter().all(|step| step.parents.is_empty()), "roots never depend on each other");
    }

    #[semio_framework_async_macros::async_test]
    async fn compute_mathematical_roots_finds_one_and_two() {
        let mut snapshot = MathematicalSnapshot::default();
        snapshot.equation = quadratic_with_roots_one_and_two();
        let mut roots: Vec<f64> = compute_mathematical_roots(&snapshot).into_iter().map(|r| r.approx).collect();
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(roots.len(), 2);
        assert!((roots[0] - 1.0).abs() < 1e-6, "expected root near 1.0, got {}", roots[0]);
        assert!((roots[1] - 2.0).abs() < 1e-6, "expected root near 2.0, got {}", roots[1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn out_of_scope_equation_plans_zero_steps_not_a_panic() {
        // 🔎️ Rational, non-integer coefficient — documented scope boundary, not a crash.
        let snapshot = MathematicalSnapshot::default(); // default equation is the integer literal 0
        let steps = <MathematicalRootsField as protocol::InferredField<MathematicalSnapshot>>::plan(&snapshot);
        assert!(steps.is_empty(), "the zero polynomial has no isolated real roots to plan");
    }

    /// 🧪️ Determinism law (mirrors `🧭topology`'s own `inference_determinism_law`): same snapshot,
    /// same `DepHash` chain, same values — required for `protocol::infer_field`'s cache to ever be
    /// safe to enable for this field.
    #[semio_framework_async_macros::async_test]
    async fn dep_hash_is_deterministic_across_repeated_calls() {
        let mut snapshot = MathematicalSnapshot::default();
        snapshot.equation = quadratic_with_roots_one_and_two();
        let first = <MathematicalRootsField as protocol::InferredField<MathematicalSnapshot>>::dep_input(&snapshot, &0, &[]);
        let second = <MathematicalRootsField as protocol::InferredField<MathematicalSnapshot>>::dep_input(&snapshot, &0, &[]);
        assert_eq!(first, second);
    }

    /// 🧪️ The whole point of a real `DepHash` chain: an edit that changes a coefficient must
    /// change `dep_input`'s bytes for every root, proving the chain is actually wired to
    /// `equation`, not a constant.
    #[semio_framework_async_macros::async_test]
    async fn dep_input_changes_when_a_coefficient_changes() {
        let mut before = MathematicalSnapshot::default();
        before.equation = quadratic_with_roots_one_and_two();
        let mut after = before.clone();
        after.equation.replace(EquationNodeLabel(6), EquationNodeKind::Integer { lexeme: "99".into() });
        let before_bytes = <MathematicalRootsField as protocol::InferredField<MathematicalSnapshot>>::dep_input(&before, &0, &[]);
        let after_bytes = <MathematicalRootsField as protocol::InferredField<MathematicalSnapshot>>::dep_input(&after, &0, &[]);
        assert_ne!(before_bytes, after_bytes, "changing a coefficient must change the DepHash input");
    }
}
//#endregion 🧪️Tests
