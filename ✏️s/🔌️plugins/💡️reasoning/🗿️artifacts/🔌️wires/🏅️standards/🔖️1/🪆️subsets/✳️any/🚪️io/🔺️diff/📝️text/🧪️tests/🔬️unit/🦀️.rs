use super::*;
use crate::empty_wires_snapshot;

fn node(id: &str, text: &str) -> DslValue {
    dsl::to_dsl_value(&dsl::json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn apply_adds_node_via_board_fixture_delta() {
    let snapshot = empty_wires_snapshot();
    let diff = diff_board_fixture(&board_after_add_node(&snapshot, &node("node-1", "Alpha")));
    let after = diff.apply(&snapshot).expect("valid mutation diff");
    assert_eq!(crate::wires_working_board(&after).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn absorb_replace_wins() {
    let mut diff = diff_board_fixture(&board_after_add_node(&empty_wires_snapshot(), &node("node-1", "Alpha")));
    let replacement = empty_wires_snapshot();
    diff.absorb(diff_set_snapshot(&replacement));
    assert!(diff.content.is_none());
    assert!(diff.artifact.is_some());
}
