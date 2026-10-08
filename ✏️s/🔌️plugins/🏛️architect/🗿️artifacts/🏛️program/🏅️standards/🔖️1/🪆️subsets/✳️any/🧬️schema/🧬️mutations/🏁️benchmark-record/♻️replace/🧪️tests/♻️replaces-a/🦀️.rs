//! 🧪️ `replace-benchmark-record` fixture — `♻️replaces`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which replaces the target row wholesale in the composed table's row delta — `removed = [id]`, `added = [payload row]`, `reordered` unless the row was last; the child handle is re-derived by the central applier.
//!
//! That leaf's own contract line reads: 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns: `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
//!
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from this JSON by `fixtures generate` and are asserted by the shared codec-matrix harness.

use crate::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🏁️benchmark-record/♻️replace/♻️replaces/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🏁️benchmark-record/♻️replace/♻️replaces/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🏁️benchmark-record/♻️replace/♻️replaces/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🏁️benchmark-record/♻️replace/♻️replaces/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🏁️benchmark-record/♻️replace/♻️replaces/🎯️outcome/🔣️.json");

fn before() -> ProgramSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("replace-benchmark-record/replaces-benchmark-record-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("replace-benchmark-record/replaces-benchmark-record-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("replace-benchmark-record/replaces-benchmark-record-a: mutation decodes")
}

/// ▶️ replace-benchmark-record carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("replace-benchmark-record/replaces-benchmark-record-a: replace-benchmark-record applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-benchmark-record/replaces-benchmark-record-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying replace-benchmark-record and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base).expect("valid retained mutation inverse fixture");
    undo.reverse();
    let mut state = protocol::apply_diff(forward.diff(&base).diff(), &base).expect("replace-benchmark-record/replaces-benchmark-record-a: forward diff applies");
    for step in &undo {
        state = protocol::apply_diff(step.diff(&state).diff(), &state).expect("replace-benchmark-record/replaces-benchmark-record-a: inverse step applies");
    }
    assert_eq!(state, base, "replace-benchmark-record/replaces-benchmark-record-a: replace-benchmark-record (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed replace-benchmark-record payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("replace-benchmark-record/replaces-benchmark-record-a: snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("replace-benchmark-record/replaces-benchmark-record-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("replace-benchmark-record/replaces-benchmark-record-a: snapshot reparses");
        assert_eq!(reencoded, original, "replace-benchmark-record/replaces-benchmark-record-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("replace-benchmark-record/replaces-benchmark-record-a: replace-benchmark-record payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("replace-benchmark-record/replaces-benchmark-record-a: replace-benchmark-record payload reparses");
    assert_eq!(reencoded, original, "replace-benchmark-record/replaces-benchmark-record-a: committed replace-benchmark-record payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: replace-benchmark-record applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("replace-benchmark-record/replaces-benchmark-record-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-benchmark-record/replaces-benchmark-record-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "replace-benchmark-record/replaces-benchmark-record-a: replace-benchmark-record raised a diagnostic on a fixture that declares a clean apply");
    assert!(protocol::apply_diff(outcome.diff(), &base).is_ok(), "replace-benchmark-record/replaces-benchmark-record-a: replace-benchmark-record was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta replace-benchmark-record produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("replace-benchmark-record/replaces-benchmark-record-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("replace-benchmark-record/replaces-benchmark-record-a: committed diff decodes");
    assert_eq!(produced, committed, "replace-benchmark-record/replaces-benchmark-record-a: the diff replace-benchmark-record builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_committed_diff_is_canonical() {
    let decoded: ProgramDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("replace-benchmark-record/replaces-benchmark-record-a: committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("replace-benchmark-record/replaces-benchmark-record-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("replace-benchmark-record/replaces-benchmark-record-a: committed diff reparses");
    assert_eq!(reencoded, original, "replace-benchmark-record/replaces-benchmark-record-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what replace-benchmark-record does, not a summary.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("replace-benchmark-record/replaces-benchmark-record-a: committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("replace-benchmark-record/replaces-benchmark-record-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-benchmark-record/replaces-benchmark-record-a: the committed diff did not carry before to after");
}

/// 🧮️ Law L3: the diffs of replace-benchmark-record's inverse mutations, summed with `absorb`, equal the negative of its forward diff and carry the committed after-snapshot back to the before-snapshot.
#[semio_framework_async_macros::async_test]
async fn replace_benchmark_record_inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
