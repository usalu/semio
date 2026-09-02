//! 🧪️ `move-point` fixture — `rejects-moving-a-point-that-is-not-in-the-cloud`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `MathematicalSnapshot` keeps its graph and its point
//! cloud in three co-derived composed CHILDREN (`notation`/`results`/`computed`,
//! `🔖️WorkingScene`), and every APPLIED mathematical diff re-mints all three through
//! `mathematical_children_from_state`, whose `child_id` is a `DefaultHasher` digest of the child
//! content. Hand-authoring such an `➡️after` would mean hand-forging a value from `std`'s
//! deliberately unspecified default hasher. A committed snapshot therefore decodes to an
//! UNRESOLVED handle and `mathematical_scene` fails soft to an EMPTY point cloud — the state this
//! case pins, and the state in which `move-point`'s own `mutation.target-missing` fires.

use crate::artifacts::mathematical::standards::v1::subsets::geometry::schema::mutations::move_point::mutation::MovePoint;
use crate::artifacts::mathematical::{mathematical_geometry, MathematicalDiff, MathematicalMutation, MathematicalSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> MathematicalSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> MathematicalSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> MathematicalMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<MathematicalDiff> {
    <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `move-point` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(mathematical_geometry(&base).points.is_empty(), "rejects-moving-a-point-that-is-not-in-the-cloud's before-snapshot must decode to an unresolved, point-less cloud");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "move-point/rejects-moving-a-point-that-is-not-in-the-cloud: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected move must not mint a fresh notation/results/computed triple");
}

/// 🎯️ `move-point` owns TWO refusal branches and their ORDER is the contract: the base-state index
/// lookup runs first, so an absent point is the Error `mutation.target-missing` even when the
/// requested coordinates are themselves non-finite — the Fatal `mutation.invariant` is only
/// reachable once a point actually exists at that index.
#[semio_framework_async_macros::async_test]
async fn the_index_lookup_precedes_the_finiteness_invariant() {
    let base = before();
    let emitted = produced();
    assert_eq!(emitted.diff(), &MathematicalDiff::default(), "a rejecting move-point must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "an out-of-range index is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing point is an Error, not the Fatal reserved for a non-finite position");
    assert_eq!(messages[0].target, vec!["0".to_string()], "the diagnostic names the bare BASE-state index");

    let non_finite = MathematicalMutation::MovePoint(MovePoint { index: 0, x: f64::NAN, y: 0.0 });
    let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&non_finite, &base);
    assert_eq!(outcome.messages()[0].code.0, "mutation.target-missing", "with no point at the index, even a NaN position is refused as target-missing — the lookup guard wins");

    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("move", "point", "move-point", "MovedPoint"), "the fixture must be bound to move-point's own descriptor");
}

/// ↩️ `move-point` inverts to another `move-point` carrying the coordinates BASE held at that
/// index. With no such point the inverse is empty — base-derived, never payload-derived.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_prior_position_to_restore() {
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "move-point/rejects-moving-a-point-that-is-not-in-the-cloud: a rejected move must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `x`/`y` are `f64`, so a
/// whole coordinate always re-encodes with a `.0` — including the negative one.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "move-point/rejects-moving-a-point-that-is-not-in-the-cloud: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&(mutation()).to_value());
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "move-point/rejects-moving-a-point-that-is-not-in-the-cloud: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::JsonValue::as_str), Some("rejected"), "move-point/rejects-moving-a-point-that-is-not-in-the-cloud declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(pack::JsonValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(pack::JsonValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
