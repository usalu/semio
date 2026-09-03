//! 🧪️ `change-coefficient` fixture — `raises-the-leading-coefficient-to-three-halves`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ✅️ This is the ONE equation leaf that gets a fully hand-authored APPLIED case with a real
//! `🔺️diff`: `change-coefficient` edits `EquationSnapshot.equation`, the plain (non-`#[child]`)
//! persistent field, and therefore never calls `equation_children_from_state`. Every other verb
//! in this vocabulary re-mints the `notation`/`results`/`computed` triple, whose `child_id` is a
//! `DefaultHasher` digest, and so cannot have a hand-authored `➡️after`. Here the composed triple
//! is byte-identical across `⬅️before` and `➡️after` — an invariant this fixture asserts directly.
//!
//! 🌳 The committed equation is `2·x² + 7` with `EquationNodeLabel`s 0..6 and `nextLabel` 7; the
//! payload retargets the label-2 leading coefficient from the integer `2` to the rational `3/2`.

use crate::artifacts::equation::standards::v1::subsets::equation::schema::mutations::change_coefficient::mutation::ChangeCoefficient;
use crate::artifacts::equation::snapshot::schema::{EquationNodeKind, EquationNodeLabel};
use crate::artifacts::equation::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

/// 🪪 The label the committed payload addresses — the leading coefficient of `2·x²`.
const COEFFICIENT: EquationNodeLabel = EquationNodeLabel(2);

fn before() -> EquationSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The label-addressed replace carries `before` to exactly the committed `after`: the label-2
/// leaf becomes `Rational { 3, 2 }`, every other labelled node keeps its identity, and `next_label`
/// does NOT advance — a coefficient change replaces a node in place, it never mints one.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    assert_eq!(base.equation.find(COEFFICIENT).map(|node| node.kind.clone()), Some(EquationNodeKind::Integer { lexeme: "2".to_string() }), "raises-the-leading-coefficient-to-three-halves' base equation must carry the integer 2 at label 2");
    let applied = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("change-coefficient applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-coefficient/raises-the-leading-coefficient-to-three-halves: applied state differs from committed after-snapshot");
    assert_eq!(applied.equation.find(COEFFICIENT).map(|node| node.kind.clone()), Some(EquationNodeKind::Rational { numer: "3".to_string(), denom: "2".to_string() }), "a denominator other than 1 lands as the Rational variant, not Integer");
    assert_eq!(applied.equation.next_label, base.equation.next_label, "a replace-in-place must not advance the label allocator");
}

/// 🧩️ The load-bearing composition assertion for this leaf: `change-coefficient` is the only
/// equation verb that leaves the three co-derived composed children completely alone.
#[semio_framework_async_macros::async_test]
async fn the_composed_child_triple_is_never_re_minted() {
    let base = before();
    let after = expected_after();
    assert_eq!((&after.notation, &after.results, &after.computed), (&base.notation, &base.results, &base.computed), "change-coefficient/raises-the-leading-coefficient-to-three-halves: the equation edit must not disturb notation/results/computed");
    let emitted = produced();
    let diff = emitted.diff();
    assert!(diff.equation.is_some(), "change-coefficient fills the equation slot");
    assert!(diff.notation.is_none() && diff.results.is_none() && diff.computed.is_none(), "change-coefficient must leave every composed-child slot of the diff empty");
    assert!(diff.camera_x.is_none() && diff.camera_y.is_none() && diff.camera_zoom.is_none() && diff.locale.is_none(), "change-coefficient is artifact-lane only — it never writes a config-lane slot");
}

/// ↩️ The undo is reconstructed from BASE's own value at that label and collapses back to the
/// `Integer` variant because the captured denominator is `1`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(
        inverse,
        vec![EquationMutation::ChangeCoefficient(ChangeCoefficient { label: COEFFICIENT, numer: "2".to_string(), denom: "1".to_string() })],
        "change-coefficient inverts to BASE's own numer/denom at the same label, got {inverse:?}"
    );
    let mut snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &snapshot);
        snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-coefficient/raises-the-leading-coefficient-to-three-halves: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `EquationNodeKind` is
/// INTERNALLY tagged on `kind` with camelCase variant names, and `EquationNodeLabel` is a
/// transparent `u64` newtype — so a coefficient address commits as a bare number.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-coefficient/raises-the-leading-coefficient-to-three-halves: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&(mutation()).to_value());
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-coefficient/raises-the-leading-coefficient-to-three-halves: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/ChangeCoefficient/label").and_then(pack::JsonValue::as_u64), Some(2), "the label commits as a bare u64, never as a wrapper object");
}

/// 🎯️ The declared outcome is a clean `applied` — a real coefficient change raises no diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::JsonValue::as_str), Some("applied"), "change-coefficient/raises-the-leading-coefficient-to-three-halves declares an applied outcome");
    let emitted = produced();
    assert!(emitted.messages().is_empty(), "a resolvable, non-identical, non-zero-denominator coefficient change is silent, got {:?}", emitted.messages());
    assert!(outcome.get("messages").is_none(), "a clean applied outcome commits no messages array");
}

/// 🔺️ The single most load-bearing assertion in this fixture: the produced delta is exactly the
/// committed one — `equation` replaced whole, everything else `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced_value = pack::json_from_dsl_value(&(produced().diff()).to_value());
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert!(pack::json::value_eq_ignoring_object_order(&produced_value, &committed), "change-coefficient/raises-the-leading-coefficient-to-three-halves: produced diff differs from the committed 🔺️diff/🔣️.json ({produced_value:?} vs {committed:?})");
}

/// 🔣️ The committed diff is canonical and decodes to `EquationDiff`, whose container
/// `#[serde(default)]` carries no per-field `skip_serializing_if` — all eight slots are present.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = pack::json_from_dsl_value(&decoded.to_value());
    let original = pack::parse_json(DIFF).expect("committed diff reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-coefficient/raises-the-leading-coefficient-to-three-halves: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 8, "EquationDiff emits all eight slots, `null` for the untouched ones");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the coefficient change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EquationDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced_snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "change-coefficient/raises-the-leading-coefficient-to-three-halves: committed diff did not carry before to after");
}

/// 🚨️ The three guards this verb owns, checked against the very same committed base: a label that
/// resolves to a non-numeric node and a label that resolves to nothing are both Error
/// `mutation.target-missing`, while a zero denominator is a Fatal `mutation.invariant`.
#[semio_framework_async_macros::async_test]
async fn a_non_numeric_target_and_a_zero_denominator_are_refused() {
    let base = before();
    let on_the_sum = EquationMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(0), numer: "1".to_string(), denom: "1".to_string() });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&on_the_sum, &base);
    assert_eq!(outcome.messages()[0].code.0, "mutation.target-missing", "label 0 is the Add root — a coefficient change cannot address a non-numeric node");
    assert_eq!(outcome.messages()[0].level, protocol::Severity::Error, "a non-numeric target is an Error, not a Fatal");

    let absent = EquationMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(999), numer: "1".to_string(), denom: "1".to_string() });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&absent, &base);
    assert_eq!(outcome.messages()[0].target, vec!["999".to_string()], "the diagnostic names the unresolved LABEL, rendered decimal");

    let zero_denominator = EquationMutation::ChangeCoefficient(ChangeCoefficient { label: COEFFICIENT, numer: "1".to_string(), denom: "0".to_string() });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&zero_denominator, &base);
    assert_eq!(outcome.messages()[0].code.0, "mutation.invariant", "a zero denominator breaches an invariant rather than missing a target");
    assert_eq!(outcome.messages()[0].level, protocol::Severity::Fatal, "a zero denominator is Fatal — no merge policy may absorb it");
    assert_eq!(outcome.diff(), &EquationDiff::default(), "every refused change-coefficient carries the empty diff");

    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "coefficient", "change-coefficient", "ChangedCoefficient"), "the fixture must be bound to change-coefficient's own descriptor");
}
