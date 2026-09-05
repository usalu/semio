//! 🧪️ `rename-benchmark-record` fixture — `🚫️absent-a`.
//!
//! Hand-authored source of truth is the JSON quartet beside this file, plus the empty `🔺️diff/🚫️.absent` marker that stands in for the diff a rejection never builds (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which reads the live `benchmarks` rows off the working-scene cache — which a fresh test process has never populated — finds no `benchmark-record-a`, and rejects with `mutation.target-missing`.
//!
//! That leaf's own contract line reads: ✏️ Sets the target row's `header.name` within the working-scene cache, then re-mints a fresh content-addressed `table` child handle. Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff).
//!
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from this JSON by `fixtures generate` and are asserted by the shared codec-matrix harness.

use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> ProgramSnapshot {
    serde_json::from_str(BEFORE).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    serde_json::from_str(AFTER).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: mutation decodes")
}

/// ▶️ rename-benchmark-record is rejected here, so the committed after-snapshot is the before-snapshot unchanged.
#[semio_framework_async_macros::async_test]
async fn rename_benchmark_record_leaves_the_before_snapshot_untouched() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: the empty rejection diff still applies");
    assert_eq!(applied, expected_after(), "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: a rejected rename-benchmark-record must leave the snapshot exactly as committed");
}

/// ↩️ A rejected rename-benchmark-record has nothing to undo: its inverse is empty.
#[semio_framework_async_macros::async_test]
async fn rename_benchmark_record_has_an_empty_inverse() {
    let base = before();
    assert!(mutation().inverse(&base).is_empty(), "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: a rejected rename-benchmark-record must record no inverse step");
}

/// 🔣️ Both committed snapshots and the committed rename-benchmark-record payload are canonical.
#[semio_framework_async_macros::async_test]
async fn rename_benchmark_record_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: snapshot reparses");
        assert_eq!(reencoded, original, "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: rename-benchmark-record payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: rename-benchmark-record payload reparses");
    assert_eq!(reencoded, original, "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: committed rename-benchmark-record payload JSON is not canonical");
}

/// 🎯️ The declared rejection holds, down to the code and the offending path.
#[semio_framework_async_macros::async_test]
async fn rename_benchmark_record_declared_rejection_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: this fixture declares a rejected outcome");
    let outcome = mutation().diff(&before());
    let raised =
        outcome.messages().iter().find(|message| message.level >= protocol::diagnostic::Severity::Error).expect("rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: rename-benchmark-record must raise an Error-level message here");
    assert_eq!(raised.code.0, "mutation.target-missing", "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: rejection code differs from the committed outcome");
    assert_eq!(raised.target, vec!["benchmark-record-a".to_string()], "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: rejection path differs from the committed outcome");
}

/// 🔺️ A rejection never carries a diff: rename-benchmark-record returns the default ProgramDiff untouched.
#[semio_framework_async_macros::async_test]
async fn rename_benchmark_record_produces_no_diff() {
    let outcome = mutation().diff(&before());
    assert_eq!(outcome.diff(), &ProgramDiff::default(), "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: a rejected rename-benchmark-record must not build any delta");
}

/// 🚫️ The case carries the absent-diff marker instead of an invented empty patch (contract D6).
#[semio_framework_async_macros::async_test]
async fn rename_benchmark_record_carries_the_absent_diff_marker() {
    assert!(ABSENT.is_empty(), "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: 🔺️diff/🚫️.absent must be an empty marker file");
}

/// 🪞 The committed after-snapshot repeats the before-snapshot byte for byte.
#[semio_framework_async_macros::async_test]
async fn rename_benchmark_record_after_snapshot_repeats_before() {
    assert_eq!(BEFORE, AFTER, "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: a rejected case must commit an after-snapshot identical to its before-snapshot");
    assert_eq!(before(), expected_after(), "rename-benchmark-record/rejects-renaming-absent-benchmark-record-a: decoded before and after snapshots must be equal");
}
