//! 🧪️ `update-stainless-inputs` fixture — `upsizes-the-stainless-section-to-a-duplex-grade`.
//!
//! EN 1993-1-4's three stainless fields move together: the design moment 40→56 kNm, the plastic modulus 300000→384000 mm3 and the proof strength 220→460 MPa (austenitic 1.4301 → duplex 1.4462). Nothing outside the stainless group is republished.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1993Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> En1993Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> En1993Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn applied() -> En1993Snapshot {
    let base = before();
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &base);
    <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(raised.diff(), &base).expect("update-stainless-inputs applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.stainless_f_y_mpa, 460.0, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the stainless proof strength must be the duplex 460 MPa");
    assert_eq!(snapshot.stainless_w_pl_mm3, 384000.0, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the stainless plastic modulus must be 384000 mm3");
    assert_eq!(snapshot.f_y_mpa, before().f_y_mpa, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the carbon-steel member's own fy must be untouched");
    assert_eq!(snapshot, expected_after(), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(step, &snapshot);
        snapshot = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1993Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `update-stainless-inputs`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: a rejected mutation must leave the snapshot untouched"),
        other => panic!("update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `update-stainless-inputs` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert_eq!(raised_diff.stainless_m_ed_knm, Some(56.0), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the diff must publish stainlessMEdKnm = 56");
    assert!(raised_diff.f_y_mpa.is_none(), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the base member's fy must NOT ride along in this diff");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `En1993Diff` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1993Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `update-stainless-inputs` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1993Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: committed diff did not carry before to after");
}
