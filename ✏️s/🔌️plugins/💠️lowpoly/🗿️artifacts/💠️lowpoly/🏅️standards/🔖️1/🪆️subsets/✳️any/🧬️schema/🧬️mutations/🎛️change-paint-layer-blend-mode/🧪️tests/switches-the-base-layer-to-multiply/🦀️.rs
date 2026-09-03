//! 🧪️ `change-paint-layer-blend-mode` fixture — `switches-the-base-layer-to-multiply`.
//!
//! `change-paint-layer-blend-mode` treats the blend mode as an opaque string (no vocabulary check) and writes an indexed layer patch with only `blendMode` set.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::lowpoly::LowpolyDiff;
use crate::artifacts::lowpoly::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

/// 🔓️ Decodes committed fixture JSON through the artifact's own value codec — these types carry
/// `ToValue`/`FromValue`, never `serde`, because `LowpolyObject.mesh` is an `ArtifactChild` handle.
fn from_json<T: dsl::FromValue>(text: &str) -> T {
    let parsed: serde_json::Value = serde_json::from_str(text).expect("fixture json parses");
    dsl::FromValue::from_value(dsl::DslValue::from(parsed)).expect("fixture json decodes")
}

/// 🔒️ Re-encodes through the same codec so canonicality assertions compare like with like.
fn to_json<T: dsl::ToValue>(value: &T) -> serde_json::Value {
    dsl::ToValue::to_value(value).into()
}


fn before() -> LowpolySnapshot {
    from_json(BEFORE)
}
fn expected_after() -> LowpolySnapshot {
    from_json(AFTER)
}
fn mutation() -> LowpolyMutation {
    from_json(MUTATION)
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("change-paint-layer-blend-mode applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LowpolySnapshot = from_json(text);
        let reencoded = to_json(&decoded);
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: committed {side} JSON is not canonical");
    }
    let reencoded = to_json(&mutation());
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = to_json(&message.level);
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: raised diagnostics differ from the committed 🎯️outcome messages");
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `change-paint-layer-blend-mode` is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation(), &base);
    let produced = to_json(raised.diff());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: LowpolyDiff = from_json(DIFF);
    let reencoded = to_json(&decoded);
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-paint-layer-blend-mode` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: LowpolyDiff = from_json(DIFF);
    let produced = <LowpolyDiff as protocol::MutationDiff<LowpolySnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-paint-layer-blend-mode/switches-the-base-layer-to-multiply: committed diff did not carry before to after");
}
