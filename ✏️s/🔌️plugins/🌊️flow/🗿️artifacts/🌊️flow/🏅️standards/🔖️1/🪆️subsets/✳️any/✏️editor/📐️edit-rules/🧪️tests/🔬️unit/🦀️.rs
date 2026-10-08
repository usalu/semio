use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{diff_semio_flow_mutation, inverse_semio_flow_mutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowParam;

fn node(id: &str, kind: &str, params: &[(&str, &str)], x: f64) -> FlowNode {
    FlowNode { id: id.into(), kind: kind.into(), label: id.into(), params: params.iter().map(|(key, value)| FlowParam { key: (*key).into(), value: (*value).into() }).collect(), position: SemioPoint2 { x, y: 0.0 } }
}

fn edge(id: &str, from: (&str, &str), to: (&str, &str)) -> FlowEdge {
    FlowEdge { id: id.into(), from: PortRef { node: from.0.into(), port: from.1.into() }, to: PortRef { node: to.0.into(), port: to.1.into() }, kind: "data".into() }
}

fn base() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        nodes: vec![node("a", "inputSlider", &[("value", "1")], 0.0), node("m", "math.add", &[("inputPorts", "[\"0\",\"1\"]")], 100.0), node("c", "outputPreview", &[], 200.0)],
        edges: vec![edge("e0", ("a", "out"), ("m", "0")), edge("e1", ("a", "out"), ("m", "1")), edge("e2", ("m", "out"), ("c", "in"))],
        ..SemioFlowSnapshot::default()
    }
}

fn forward(state: &SemioFlowSnapshot, leaf: &SemioFlowMutation) -> SemioFlowSnapshot {
    let outcome = diff_semio_flow_mutation(leaf, state);
    assert!(outcome.worst_level().is_none_or(|level| level < semio_framework_diagnostic::Severity::Error), "{leaf:?} refused: {:?}", outcome.messages());
    semio_s_artifact_stdio_semio::apply_diff(outcome.diff(), state).expect("a leaf diff applies to the state it was computed from")
}

fn by_id(mut snapshot: SemioFlowSnapshot) -> SemioFlowSnapshot {
    snapshot.nodes.sort_by(|a, b| a.id.cmp(&b.id));
    snapshot.edges.sort_by(|a, b| a.id.cmp(&b.id));
    snapshot
}

fn assert_leaves_reproduce_the_content_and_undo_restores_it(edit: &ContentEdit) {
    let (mut state, mut undo) = (base(), Vec::new());
    for leaf in &edit.leaves {
        undo.push(inverse_semio_flow_mutation(leaf, &state).expect("a leaf has a concrete inverse"));
        state = forward(&state, leaf);
    }
    assert_eq!(by_id(state.clone()), by_id(edit.content.clone()), "the leaves carry the base content to the gesture's working content");
    for rows in undo.iter().rev() {
        for row in rows.iter().rev() {
            state = forward(&state, row);
        }
    }
    assert_eq!(by_id(state), by_id(base()), "undo, last leaf first, restores the base content");
}

#[test]
fn removing_a_middle_node_severs_its_wires_first_and_undo_restores_it_in_place() {
    let mut edit = ContentEdit::new(base());
    assert!(edit.remove(&["m".to_string()], &[]));
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::RemoveEdge(_), SemioFlowMutation::RemoveEdge(_), SemioFlowMutation::RemoveEdge(_), SemioFlowMutation::RemoveNode(_)]), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_content_and_undo_restores_it(&edit);
    assert!(!ContentEdit::new(base()).remove(&["absent".to_string()], &["absent".to_string()]));
}

#[test]
fn renaming_a_middle_node_keeps_its_position_in_the_child_and_repoints_its_wires() {
    let mut edit = ContentEdit::new(base());
    assert!(edit.rename_node("m", "sum"));
    assert_eq!(edit.content.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), vec!["a", "sum", "c"]);
    assert!(edit.content.edges.iter().all(|edge| edge.from.node != "m" && edge.to.node != "m"));
    assert_leaves_reproduce_the_content_and_undo_restores_it(&edit);
}

#[test]
fn connecting_replaces_the_wire_entering_the_same_port() {
    let mut edit = ContentEdit::new(base());
    edit.connect(edge("e3", ("a", "out"), ("c", "in")));
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::RemoveEdge(removed), SemioFlowMutation::InsertEdge(_)] if removed.id == "e2"), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_content_and_undo_restores_it(&edit);
}

#[test]
fn inserting_an_input_port_in_the_middle_shifts_the_wires_at_or_past_it() {
    let mut edit = ContentEdit::new(base());
    edit.insert_port("m", true, 1, &["0".to_string(), "1".to_string(), "2".to_string()]);
    let targets: Vec<(&str, &str)> = edit.content.edges.iter().filter(|edge| edge.to.node == "m").map(|edge| (edge.id.as_str(), edge.to.port.as_str())).collect();
    assert_eq!(targets, vec![("e0", "0"), ("e1", "2")]);
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::SetNodeParam(ports), SemioFlowMutation::SetEdgeEndpoints(moved)] if ports.key == "inputPorts" && moved.id == "e1"), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_content_and_undo_restores_it(&edit);
}

#[test]
fn positions_and_params_are_single_absolute_leaves_and_restating_them_is_nothing() {
    let mut edit = ContentEdit::new(base());
    assert!(edit.set_position("a", 40.0, 0.0) && !edit.set_position("a", 40.0, 0.0));
    assert!(edit.set_param("a", "value", "6") && !edit.set_param("a", "value", "6") && edit.set_param("a", "max", "9") && !edit.set_param("absent", "k", "v"));
    assert_eq!(edit.leaves.len(), 3);
    assert_leaves_reproduce_the_content_and_undo_restores_it(&edit);
}
