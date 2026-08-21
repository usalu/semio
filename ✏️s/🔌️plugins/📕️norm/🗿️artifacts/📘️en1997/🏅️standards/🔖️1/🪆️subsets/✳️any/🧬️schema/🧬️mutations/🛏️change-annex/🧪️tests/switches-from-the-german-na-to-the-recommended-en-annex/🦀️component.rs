//! 🧪️ `change-annex` fixture — `switches-from-the-german-na-to-the-recommended-en-annex`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-annex` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1997Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1997Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1997Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-annex` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Switching the national annex from `De` to `En` rewrites `annex` alone — and specifically NOT the design
/// approach, even though the German NA prescribes DA2* while the EN recommendation leaves the choice open.
/// Those are two independent document decisions.
#[semio_framework_async_macros::async_test]
async fn switches_from_the_german_na_to_the_recommended_en_annex() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-annex applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.annex, crate::document::AnnexChoice::En, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: annex must read `AnnexChoice::En` once the change lands");
    assert_eq!(applied.design_approach, before().design_approach, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the design approach is a separate `change-design-approach` decision and must not be re-pinned by an annex switch");
}

/// ↩️ `change-annex`'s inverse reads the OLD `AnnexChoice::De` out of BASE, so replaying it puts the German
/// national annex back on `annex`.
#[semio_framework_async_macros::async_test]
async fn switching_back_to_the_german_na_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-annex applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the inverse of one change-annex is exactly one change-annex back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-annex inverse step applies");
    }
    assert_eq!(snapshot.annex, base.annex, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the inverse must put the German national annex back on `annex`");
    assert_eq!(snapshot, base, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-annex` payload are already canonical: decode → encode
/// is a fixed point, so `"En"` — `AnnexChoice` carries no serde rename, so the wire spelling is the bare Rust
/// variant name is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-annex payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-annex payload reparses");
    assert_eq!(reencoded, original, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the committed change-annex JSON is not canonical");
}

/// 🎯️ `En` differs from the committed `De`, so `change-annex`'s equality guard stays shut and no
/// `mutation.no-op` warning is raised.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: an enum cannot be non-finite, so `change-annex` carries only an equality guard, and `AnnexChoice::En` differs from the committed `De`");
    assert!(produced.messages().is_empty(), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: an accepted change-annex emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-annex` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `annex` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-annex diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the annex choice and nothing
/// else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-annex diff decodes");
    assert_eq!(decoded.annex, Some(crate::document::AnnexChoice::En), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the committed diff must carry annex = `AnnexChoice::En`");
    assert!(decoded.design_approach.is_none(), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: change-annex writes annex and must leave `design_approach` untouched");
    assert!(decoded.settlement_limit_mm.is_none(), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: change-annex writes annex and must leave `settlement_limit_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the annex switch, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-annex diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: the committed diff did not carry before to after");
    assert_eq!(produced.annex, crate::document::AnnexChoice::En, "change-annex/switches-from-the-german-na-to-the-recommended-en-annex: applying the committed diff must land annex on `AnnexChoice::En`");
}
