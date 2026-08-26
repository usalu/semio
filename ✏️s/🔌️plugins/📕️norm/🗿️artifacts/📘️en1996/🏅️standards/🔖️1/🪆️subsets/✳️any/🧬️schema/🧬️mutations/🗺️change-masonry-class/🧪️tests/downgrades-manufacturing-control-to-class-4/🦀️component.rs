//! 🧪️ `change-masonry-class` fixture — `downgrades-manufacturing-control-to-class-4`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-masonry-class` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1996Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1996Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1996Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-masonry-class` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Moving the manufacturing-control class from `Class3` to `Class4` rewrites `masonry_class` alone — the EN
/// γ_M for class 4 is 2.2 against class 3's 2.0, but that lookup is an inference, so the annex selecting it
/// is untouched.
#[semio_framework_async_macros::async_test]
fn downgrades_manufacturing_control_to_class_4() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-masonry-class applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-masonry-class/downgrades-manufacturing-control-to-class-4: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.masonry_class, crate::artifacts::en1996::MasonryClass::Class4, "change-masonry-class/downgrades-manufacturing-control-to-class-4: masonry_class must read `MasonryClass::Class4` once the change lands");
    assert_eq!(applied.annex, before().annex, "change-masonry-class/downgrades-manufacturing-control-to-class-4: the annex decides WHETHER the class-dependent γ_M table is consulted at all and must not change with the class");
}

/// ↩️ `change-masonry-class`'s inverse reads the OLD `MasonryClass::Class3` out of BASE, so replaying it puts
/// the class-3 control level back on `masonry_class`.
#[semio_framework_async_macros::async_test]
fn returning_to_class_3_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-masonry-class applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-masonry-class/downgrades-manufacturing-control-to-class-4: the inverse of one change-masonry-class is exactly one change-masonry-class back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-masonry-class inverse step applies");
    }
    assert_eq!(snapshot.masonry_class, base.masonry_class, "change-masonry-class/downgrades-manufacturing-control-to-class-4: the inverse must put the class-3 control level back on `masonry_class`");
    assert_eq!(snapshot, base, "change-masonry-class/downgrades-manufacturing-control-to-class-4: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-masonry-class` payload are already canonical: decode →
/// encode is a fixed point, so `"Class4"` — `MasonryClass` carries no serde rename despite its
/// `dsl::DslScalar` derive is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-masonry-class/downgrades-manufacturing-control-to-class-4: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-masonry-class payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-masonry-class payload reparses");
    assert_eq!(reencoded, original, "change-masonry-class/downgrades-manufacturing-control-to-class-4: the committed change-masonry-class JSON is not canonical");
}

/// 🎯️ `Class4` differs from the committed `Class3`, so the equality guard — the only guard
/// `change-masonry-class` has — stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-masonry-class/downgrades-manufacturing-control-to-class-4: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-masonry-class/downgrades-manufacturing-control-to-class-4: `change-masonry-class` has no numeric-finiteness guard at all — only the equality guard — and `MasonryClass::Class4` differs from the committed committed `Class3`, so `mutation.no-op` must not fire");
    assert!(produced.messages().is_empty(), "change-masonry-class/downgrades-manufacturing-control-to-class-4: an accepted change-masonry-class emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-masonry-class` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `masonryClass` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-masonry-class diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-masonry-class/downgrades-manufacturing-control-to-class-4: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the masonry class and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-masonry-class diff decodes");
    assert_eq!(decoded.masonry_class, Some(crate::artifacts::en1996::MasonryClass::Class4), "change-masonry-class/downgrades-manufacturing-control-to-class-4: the committed diff must carry masonryClass = `MasonryClass::Class4`");
    assert!(decoded.annex.is_none(), "change-masonry-class/downgrades-manufacturing-control-to-class-4: change-masonry-class writes masonryClass and must leave `annex` untouched");
    assert!(decoded.f_k_mpa.is_none(), "change-masonry-class/downgrades-manufacturing-control-to-class-4: change-masonry-class writes masonryClass and must leave `f_k_mpa` untouched");
    assert!(decoded.artifact.is_none(), "change-masonry-class/downgrades-manufacturing-control-to-class-4: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-masonry-class/downgrades-manufacturing-control-to-class-4: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the control-class change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-masonry-class diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-masonry-class/downgrades-manufacturing-control-to-class-4: the committed diff did not carry before to after");
    assert_eq!(produced.masonry_class, crate::artifacts::en1996::MasonryClass::Class4, "change-masonry-class/downgrades-manufacturing-control-to-class-4: applying the committed diff must land masonry_class on `MasonryClass::Class4`");
}
