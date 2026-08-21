//! 🧪️ `remove-point` fixture — `rejects-removing-a-point-from-an-empty-cloud`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `MathematicalSnapshot` keeps its graph and its point
//! cloud in three co-derived composed CHILDREN (`notation`/`results`/`computed`,
//! `🔖️WorkingScene`), and every APPLIED mathematical diff re-mints all three through
//! `mathematical_children_from_state`, whose `child_id` is a `DefaultHasher` digest of the child
//! content. Hand-authoring such an `➡️after` would mean hand-forging a value from `std`'s
//! deliberately unspecified default hasher. A committed snapshot therefore decodes to an
//! UNRESOLVED handle and `mathematical_scene` fails soft to an empty graph and an EMPTY point
//! cloud — the state this case pins, and the state in which `remove-point`'s own
//! `mutation.target-missing` fires.

use crate::artifacts::mathematical::{mathematical_geometry, MathematicalDiff, MathematicalMutation, MathematicalSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> MathematicalSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> MathematicalSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> MathematicalMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<MathematicalDiff> {
    <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `remove-point` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(mathematical_geometry(&base).points.is_empty(), "rejects-removing-a-point-from-an-empty-cloud's before-snapshot must decode to an unresolved, point-less cloud");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "remove-point/rejects-removing-a-point-from-an-empty-cloud: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected remove must not mint a fresh notation/results/computed triple");
}

/// ➖️ `remove-point` addresses an ANONYMOUS, index-keyed collection by its BASE-state index, so the
/// diagnostic target is that index rendered decimal — not the `["geometry", "points", "0"]` path
/// `MutationKind::target` renders for the undo stack. `>= len` is the whole test, which is why an
/// empty cloud rejects index 0.
#[semio_framework_async_macros::async_test]
async fn a_base_state_index_past_the_end_is_reported_as_a_decimal_string() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &MathematicalDiff::default(), "a rejecting remove-point must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "an out-of-range index is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "remove-point has no Fatal branch at all — an absent point is a miss, not an invariant breach");
    assert_eq!(messages[0].target, vec!["0".to_string()], "the diagnostic names the bare index, not a geometry/points path");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("remove", "point", "remove-point", "RemovedPoint"),
        "the fixture must be bound to remove-point's own descriptor — `remove`, the index-keyed counterpart of `delete`"
    );
}

/// ↩️ `remove-point` inverts by re-`insert`ing the exact point it captured from BASE. With no point
/// at that index there is nothing to capture, so the inverse is empty — base-derived, never
/// payload-derived.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_point_to_reinsert() {
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "remove-point/rejects-removing-a-point-from-an-empty-cloud: a rejected remove must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed
/// point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-point/rejects-removing-a-point-from-an-empty-cloud: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-point/rejects-removing-a-point-from-an-empty-cloud: committed mutation JSON is not canonical");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "remove-point/rejects-removing-a-point-from-an-empty-cloud declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}
