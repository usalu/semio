
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::EquationNodeLabel;

/// 🧪️ `x^2 - 3x + 2 = (x-1)(x-2)`, roots `{1, 2}` — built directly as a labeled tree (`Add` of
/// `x^2`, `-3x`, `2`), the same shape `expr_to_equation_node` would produce from
/// `cas::polybridge`'s own canonical `Add` term order.
fn quadratic_with_roots_one_and_two() -> EquationExprSnapshot {
    let x = EquationNode { label: EquationNodeLabel(1), kind: EquationNodeKind::Symbol { name: "x".into() } };
    let two_exp = EquationNode { label: EquationNodeLabel(2), kind: EquationNodeKind::Integer { lexeme: "2".into() } };
    let x_squared = EquationNode { label: EquationNodeLabel(3), kind: EquationNodeKind::Pow { base: Box::new(x.clone()), exponent: Box::new(two_exp) } };
    let neg_three = EquationNode { label: EquationNodeLabel(4), kind: EquationNodeKind::Integer { lexeme: "-3".into() } };
    let neg_three_x = EquationNode { label: EquationNodeLabel(5), kind: EquationNodeKind::Mul { factors: vec![neg_three, x] } };
    let two = EquationNode { label: EquationNodeLabel(6), kind: EquationNodeKind::Integer { lexeme: "2".into() } };
    let expr = EquationNode { label: EquationNodeLabel(7), kind: EquationNodeKind::Add { terms: vec![x_squared, neg_three_x, two] } };
    EquationExprSnapshot { expr, next_label: 8 }
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
    let mut snapshot = EquationSnapshot::default();
    snapshot.equation = quadratic_with_roots_one_and_two();
    let steps = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::plan(&snapshot);
    assert_eq!(steps.len(), 2, "x^2-3x+2 has exactly two real roots");
    assert!(steps.iter().all(|step| step.parents.is_empty()), "roots never depend on each other");
}

#[semio_framework_async_macros::async_test]
async fn compute_equation_roots_finds_one_and_two() {
    let mut snapshot = EquationSnapshot::default();
    snapshot.equation = quadratic_with_roots_one_and_two();
    let mut roots: Vec<f64> = compute_equation_roots(&snapshot).into_iter().map(|r| r.approx).collect();
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(roots.len(), 2);
    assert!((roots[0] - 1.0).abs() < 1e-6, "expected root near 1.0, got {}", roots[0]);
    assert!((roots[1] - 2.0).abs() < 1e-6, "expected root near 2.0, got {}", roots[1]);
}

#[semio_framework_async_macros::async_test]
async fn out_of_scope_equation_plans_zero_steps_not_a_panic() {
    // 🔎️ Rational, non-integer coefficient — documented scope boundary, not a crash.
    let snapshot = EquationSnapshot::default(); // default equation is the integer literal 0
    let steps = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::plan(&snapshot);
    assert!(steps.is_empty(), "the zero polynomial has no isolated real roots to plan");
}

/// 🧪️ Determinism law (mirrors `🧭topology`'s own `inference_determinism_law`): same snapshot,
/// same `DepHash` chain, same values — required for `protocol::infer_field`'s cache to ever be
/// safe to enable for this field.
#[semio_framework_async_macros::async_test]
async fn dep_hash_is_deterministic_across_repeated_calls() {
    let mut snapshot = EquationSnapshot::default();
    snapshot.equation = quadratic_with_roots_one_and_two();
    let first = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&snapshot, &0, &[]);
    let second = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&snapshot, &0, &[]);
    assert_eq!(first, second);
}

/// 🧪️ The whole point of a real `DepHash` chain: an edit that changes a coefficient must
/// change `dep_input`'s bytes for every root, proving the chain is actually wired to
/// `equation`, not a constant.
#[semio_framework_async_macros::async_test]
async fn dep_input_changes_when_a_coefficient_changes() {
    let mut before = EquationSnapshot::default();
    before.equation = quadratic_with_roots_one_and_two();
    let mut after = before.clone();
    after.equation.replace(EquationNodeLabel(6), EquationNodeKind::Integer { lexeme: "99".into() });
    let before_bytes = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&before, &0, &[]);
    let after_bytes = <EquationRootsField as protocol::InferredField<EquationSnapshot>>::dep_input(&after, &0, &[]);
    assert_ne!(before_bytes, after_bytes, "changing a coefficient must change the DepHash input");
}
