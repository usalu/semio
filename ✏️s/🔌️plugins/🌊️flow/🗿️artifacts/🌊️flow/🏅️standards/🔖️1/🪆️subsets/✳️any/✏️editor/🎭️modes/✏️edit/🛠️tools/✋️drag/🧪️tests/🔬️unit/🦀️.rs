use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::set_node_label::SetNodeLabel;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowNode;

fn content() -> SemioFlowSnapshot {
    let node = |id: &str, x: f64, y: f64| FlowNode { id: id.into(), kind: "inputSlider".into(), label: id.into(), params: Vec::new(), position: SemioPoint2 { x, y } };
    SemioFlowSnapshot { schema: "stdio.semio.flow".into(), nodes: vec![node("a", 0.0, 0.0), node("b", 100.0, 50.0), node("c", 200.0, -20.0)], edges: Vec::new() }
}

fn record(node_ids: &[&str], dx: f64, dy: f64) -> NodeDragRecord {
    NodeDragRecord { gesture_id: "node-drag:7".into(), node_ids: node_ids.iter().map(|id| id.to_string()).collect(), dx, dy }
}

/// ⚖️ LAW: every gesture record that moves becomes ONE relative `drag-nodes` leaf over the nodes this flow holds — a
/// multi-select drag is one leaf; a stranger node is dropped from the targets, a record of strangers or of a zero offset
/// yields nothing.
#[test]
fn records_become_one_relative_leaf_each_over_the_nodes_the_flow_holds() {
    let base = content();
    let leaves = flow_drag_leaves(&base, &[record(&["a", "ghost", "b"], 30.0, 40.0), record(&["c"], 0.0, 0.0), record(&["ghost"], 1.0, 1.0), record(&["c"], -5.5, 2.0)]);
    assert_eq!(leaves, vec![DragNodes { targets: vec!["a".into(), "b".into()], dx: 30.0, dy: 40.0 }, DragNodes { targets: vec!["c".into()], dx: -5.5, dy: 2.0 }]);
    assert!(flow_drag_leaves(&base, &[]).is_empty());
}

/// ⚖️ LAW: one release commits as ONE transaction of the node-drag machine whose ref is scoped `<appId>#<verb>` and minted
/// from the authoring seed, prepared leaves first; a release that moves nothing commits nothing.
#[test]
fn a_release_commits_one_transaction_with_prepared_leaves_first() {
    let base = content();
    let label = SemioFlowMutation::SetNodeLabel(SetNodeLabel { id: "a".into(), label: "A".into() });
    let (transaction, leaves) = flow_drag_tool_commit("nodeGraphEdit", "seed-1", &base, vec![label.clone()], &[record(&["b"], 10.0, 0.0)]).expect("the release commits");
    assert_eq!(transaction.tool, format!("{FLOW_EDITOR_APP_ID}#nodeGraphEdit"));
    assert!(transaction.id.starts_with("tx-"), "{}", transaction.id);
    assert_eq!(leaves, vec![label, SemioFlowMutation::DragNodes(DragNodes { targets: vec!["b".into()], dx: 10.0, dy: 0.0 })]);
    assert!(flow_drag_tool_commit("nodeGraphEdit", "seed-1", &base, Vec::new(), &[record(&["b"], 0.0, 0.0), record(&["ghost"], 3.0, 3.0)]).is_none(), "a release that moves nothing leaves zero trace");
}

/// ⚖️ LAW (design §12): a seeded release is a composed-child tool transaction — one `content` child, its ops labelled
/// from the leaves, the parent untouched, no coalescing; a seedless view publishes the leaves plainly.
#[test]
fn a_seeded_release_is_a_child_transaction_and_a_seedless_one_is_plain() {
    let base = content();
    let emit = flow_drag_tool_emit("child-1", "moveMediaNode", "seed-1", &base, Vec::new(), &[record(&["c"], -20.0, 20.0)]);
    let transaction = emit.transaction.as_ref().expect("a seeded release carries its transaction");
    assert_eq!(transaction.tool, format!("{FLOW_EDITOR_APP_ID}#moveMediaNode"));
    assert!(emit.artifact_mutations.is_empty() && emit.coalesce_key.is_none() && emit.description.is_none());
    assert_eq!(emit.child_emits.len(), 1);
    let child = &emit.child_emits[0];
    assert_eq!((child.slot.as_str(), child.child_id.as_str(), child.ops.len()), ("content", "child-1", 1));
    assert_eq!(child.labels[0].resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 1 node by (-20, 20)");
    let plain = flow_drag_tool_emit("child-1", "moveMediaNode", "", &base, Vec::new(), &[record(&["c"], -20.0, 20.0)]);
    assert!(plain.transaction.is_none() && plain.child_emits.len() == 1);
    let nothing = flow_drag_tool_emit("child-1", "moveMediaNode", "seed-1", &base, Vec::new(), &[record(&["c"], 0.0, 0.0)]);
    assert!(nothing.transaction.is_none() && nothing.child_emits.is_empty());
}
