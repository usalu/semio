//! 🧪️ `set-node-positions` fixture — `🧪️reports` (placing every node where it already sits).
//!
//! Source of truth is the committed JSON quintet beside this file. A position-CHANGING `➡️after` is not hand-authorable for
//! this artifact (`diff_board_fixture` re-mints the composed child handle as an unspecified `DefaultHasher` digest), so the
//! committed vector is the leaf's NO-OP branch: every addressed node already sits at its payload position, the Warning `mutation.no-op`.

use crate::mutations::WiresMutation;
use crate::{materialize_wires_content, WiresDiff, WiresSnapshot};
use dsl::DslValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-node-positions/🧪️reports/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-node-positions/🧪️reports/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-node-positions/🧪️reports/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-node-positions/🧪️reports/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-node-positions/🧪️reports/🎯️outcome/🔣️.json");

fn board_entries(board: &DslValue, key: &str) -> Vec<DslValue> {
    board.get(key).and_then(|value| value.as_array()).map(|items| items.to_vec()).unwrap_or_default()
}

/// 🌱 The committed `⬅️before` with its composed content child resolved from the snapshot's own board mirror.
fn before() -> WiresSnapshot {
    let mut snapshot: WiresSnapshot = dsl::os_pack::from_json_str(BEFORE).expect("before snapshot decodes");
    let board = snapshot.wires_fixture.get("board").cloned().unwrap_or(DslValue::Null);
    materialize_wires_content(&mut snapshot.content, board_entries(&board, "nodes"), board_entries(&board, "edges"));
    snapshot
}
fn expected_after() -> WiresSnapshot {
    dsl::os_pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> WiresMutation {
    dsl::os_pack::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The no-op carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _messages) = store::apply_mutation(&before(), &mutation()).expect("the empty no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-node-positions/reports: applied state differs from committed after-snapshot");
}

/// ↩️ A no-op moves nothing, so it has nothing to undo.
#[semio_framework_async_macros::async_test]
async fn a_no_op_has_no_inverse() {
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "set-node-positions/reports: a no-op must have no inverse steps, got {inverse:?}");
}

/// 🎯️ The declared outcome — `no-op` with one `warning`/`mutation.no-op` — is what the diff builder emits, with the empty
/// diff the committed `🔺️diff` declares.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_and_diff_hold() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("no-op"));
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning);
    assert_eq!(produced.diff(), &WiresDiff::default(), "a no-op never re-mints the content child");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(produced.diff())).expect("produced diff encodes"), committed);
}

/// 🔣️ The committed snapshots and payload are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = dsl::os_pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "set-node-positions/reports: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "set-node-positions/reports: committed mutation JSON is not canonical");
}
