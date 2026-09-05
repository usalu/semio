//! 🧪️ `change-print-target` fixture — `🖨️sets-a-cmyk-print-target`.
//!
//! Proves the nullable `print_target` scalar goes from cleared to set.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    serde_json::from_str(BEFORE).expect("change-print-target/sets-a-cmyk-print-target: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("change-print-target/sets-a-cmyk-print-target: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("change-print-target/sets-a-cmyk-print-target: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-print-target applies to its committed before-snapshot")
}

/// ▶️ `change-print-target` writes the nullable root scalar and nothing else.
#[semio_framework_async_macros::async_test]
async fn fills_the_previously_cleared_print_target() {
    let after = applied();
    assert_eq!(after.print_target.as_deref(), Some("cmyk-coated"), "change-print-target must set the document print target");
    assert_eq!(after.name, "Fixture Layout", "change-print-target must not rename the document");
    assert!(after.data_fields_json.is_none(), "change-print-target must not touch the data-fields payload");
    assert_eq!(after, expected_after(), "change-print-target/sets-a-cmyk-print-target: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-clears the slot, because BASE had no print target.
#[semio_framework_async_macros::async_test]
async fn inverse_clears_the_print_target_again() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-print-target inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangePrintTarget(step) => assert!(step.new_print_target.is_none(), "the inverse must carry BASE's cleared print target"),
        other => panic!("change-print-target must invert to change-print-target, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-print-target/sets-a-cmyk-print-target: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-print-target/sets-a-cmyk-print-target: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-print-target/sets-a-cmyk-print-target: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-print-target/sets-a-cmyk-print-target: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-print-target`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-print-target/sets-a-cmyk-print-target: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-print-target/sets-a-cmyk-print-target: declared clean-applied but the diff builder reported {:?}", produced.messages());
    assert_eq!(produced.diff().print_target, Some(Some("cmyk-coated".to_string())), "change-print-target fills the doubly-optional `print_target` diff field");
    assert!(produced.diff().data_fields_json.is_none(), "change-print-target leaves `data_fields_json` untouched in the diff");
}

/// 🔺️ The sparse delta `change-print-target` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the ONLY populated field is `printTarget`, and it carries the string rather than the doubly-optional cleared arm.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-print-target/sets-a-cmyk-print-target: change-print-target must emit a diff whose sole populated field is `printTarget`");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-print-target/sets-a-cmyk-print-target: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-print-target` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-print-target/sets-a-cmyk-print-target: committed diff did not carry before to after");
}
