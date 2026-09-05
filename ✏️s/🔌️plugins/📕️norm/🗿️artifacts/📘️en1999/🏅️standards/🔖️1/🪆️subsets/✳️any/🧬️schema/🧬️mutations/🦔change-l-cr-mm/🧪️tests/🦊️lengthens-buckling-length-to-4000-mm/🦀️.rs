//! 🧪️ `change-l-cr-mm` fixture — `🦊️lengthens-buckling-length-to-4000-mm` (EN 1999 aluminium).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1999::{En1999Diff, En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1999Snapshot {
    serde_json::from_str(BEFORE).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: mutation decodes")
}

/// ▶️ `change-l-cr-mm` carries `l_cr_mm` from 3200.0 to 4000.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.l_cr_mm, 4000.0, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: `l_cr_mm` must read 4000.0 after the mutation");
    assert_eq!(produced.theta_c, base.theta_c, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: `theta_c` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `l_cr_mm` (3200.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: inverse step applies");
    }
    assert_eq!(snapshot.l_cr_mm, base.l_cr_mm, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: inverse must put `l_cr_mm` back to 3200.0");
    assert_eq!(snapshot, base, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: snapshot reparses");
        assert_eq!(reencoded, original, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: mutation reparses");
    assert_eq!(reencoded, original, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 3200.0→4000.0 edit of `l_cr_mm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: changing `l_cr_mm` away from 3200.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `lCrMm` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().l_cr_mm, Some(4000.0), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: the diff must carry `l_cr_mm` = 4000.0");
    assert!(outcome.diff().theta_c.is_none(), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: the diff must leave `theta_c` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed diff decodes");
    assert_eq!(produced, committed, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed diff decodes");
    assert_eq!(decoded.l_cr_mm, Some(4000.0), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: the committed diff must name `l_cr_mm` = 4000.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed diff reparses");
    assert_eq!(reencoded, original, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 4000.0 `l_cr_mm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed diff applies to the before-snapshot");
    assert_eq!(produced.l_cr_mm, 4000.0, "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: the committed diff must set `l_cr_mm` to 4000.0");
    assert_eq!(produced, expected_after(), "change-l-cr-mm/lengthens-buckling-length-to-4000-mm: committed diff did not carry before to after");
}
