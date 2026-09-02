//! 🧪️ `create-step` fixture — `accepts-a-rip-cut-step-and-changes-nothing`.
//!
//! `create-step` inserts the rip-cut step into `step_payloads` at `index` and re-mints `steps`/`tool_solids` from the edited timeline (`process3d_step_timeline_diff`), so the committed diff carries all three fields and the after-snapshot's timeline gains one entry.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Process3dSnapshot {
    semio_framework_os_kernel::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Process3dSnapshot {
    semio_framework_os_kernel::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Process3dMutation {
    semio_framework_os_kernel::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("create-step applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-step/accepts-a-rip-cut-step-and-changes-nothing: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "create-step/accepts-a-rip-cut-step-and-changes-nothing: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Process3dSnapshot = semio_framework_os_kernel::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&decoded));
        let original = semio_framework_os_kernel::json::parse(text).expect("snapshot reparses");
        assert!(semio_framework_os_kernel::json::value_eq_ignoring_object_order(&reencoded, &original), "create-step/accepts-a-rip-cut-step-and-changes-nothing: committed {side} JSON is not canonical");
    }
    let reencoded = semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&mutation()));
    let original = semio_framework_os_kernel::json::parse(MUTATION).expect("mutation reparses");
    assert!(semio_framework_os_kernel::json::value_eq_ignoring_object_order(&reencoded, &original), "create-step/accepts-a-rip-cut-step-and-changes-nothing: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_os_kernel::json::parse(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(semio_framework_os_kernel::json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(semio_framework_os_kernel::json::Value::as_array).map(|rows| rows.iter().map(|row| (row.get("level").and_then(semio_framework_os_kernel::json::Value::as_str).unwrap_or_default().to_string(), row.get("code").and_then(semio_framework_os_kernel::json::Value::as_str).unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = semio_framework_os_kernel::ToValue::to_value(&message.level);
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "create-step/accepts-a-rip-cut-step-and-changes-nothing: raised diagnostics differ from the committed 🎯️outcome messages");
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "create-step/accepts-a-rip-cut-step-and-changes-nothing: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "create-step/accepts-a-rip-cut-step-and-changes-nothing: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "create-step/accepts-a-rip-cut-step-and-changes-nothing: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "create-step/accepts-a-rip-cut-step-and-changes-nothing: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "create-step/accepts-a-rip-cut-step-and-changes-nothing: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-step/accepts-a-rip-cut-step-and-changes-nothing: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `create-step` is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation(), &base);
    let produced = semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(raised.diff()));
    let committed = semio_framework_os_kernel::json::parse(DIFF).expect("committed diff decodes");
    assert!(semio_framework_os_kernel::json::value_eq_ignoring_object_order(&produced, &committed), "create-step/accepts-a-rip-cut-step-and-changes-nothing: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Process3dDiff = semio_framework_os_kernel::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&decoded));
    let original = semio_framework_os_kernel::json::parse(DIFF).expect("committed diff reparses");
    assert!(semio_framework_os_kernel::json::value_eq_ignoring_object_order(&reencoded, &original), "create-step/accepts-a-rip-cut-step-and-changes-nothing: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `create-step` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Process3dDiff = semio_framework_os_kernel::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Process3dDiff as protocol::MutationDiff<Process3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-step/accepts-a-rip-cut-step-and-changes-nothing: committed diff did not carry before to after");
}
