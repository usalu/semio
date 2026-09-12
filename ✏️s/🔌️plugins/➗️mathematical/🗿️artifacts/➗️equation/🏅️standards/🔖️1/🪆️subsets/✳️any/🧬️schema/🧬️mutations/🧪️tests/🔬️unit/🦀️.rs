use super::*;
use crate::{equation_geometry, equation_graph, EquationGraph, EquationPoint};
use protocol::{Mutation, MutationDiff, SemanticMutation};

#[semio_framework_async_macros::async_test]
async fn replace_graph_diff_carries_the_whole_derived_triple() {
    // 🔎️ `notation`/`results`/`computed` are three co-derived projections of the SAME
    // `(graph, geometry)` pair, so a graph-scoped mutation always regenerates all three —
    // unlike the pre-migration single `graph` slot this test named before composition.
    let graph = EquationGraph { algorithm: "bfs".into(), ..EquationGraph::default() };
    let mutation = EquationMutation::ReplaceGraph(replace_graph::ReplaceGraph { graph });
    let base = EquationSnapshot::default();
    let outcome = Mutation::diff(&mutation, &base);
    let diff = outcome.diff();
    assert!(diff.notation.is_some());
    assert!(diff.results.is_some());
    assert!(diff.computed.is_some());
    let applied = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(equation_graph(&applied).algorithm, "bfs");
}

#[semio_framework_async_macros::async_test]
async fn create_then_delete_node_round_trips() {
    let base = EquationSnapshot::default();
    let create = EquationMutation::CreateNode(create_node::CreateNode { id: "z".into(), label: "Z".into(), x: 1.0, y: 2.0 });
    let after_create = create.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(equation_graph(&after_create).nodes.iter().any(|node| node.id == "z"));

    let undo = create.inverse(&base);
    assert_eq!(undo, vec![EquationMutation::DeleteNode(delete_node::DeleteNode { id: "z".into() })]);
    let mut state = after_create.clone();
    for step in &undo {
        state = step.diff(&after_create).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(equation_graph(&state), equation_graph(&base));
    assert_eq!(equation_geometry(&state), equation_geometry(&base));
}

#[semio_framework_async_macros::async_test]
async fn delete_node_inverse_recreates_node_and_severed_edges() {
    let base = EquationSnapshot::default();
    let delete = EquationMutation::DeleteNode(delete_node::DeleteNode { id: "a".into() });
    let after_delete = delete.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(!equation_graph(&after_delete).nodes.iter().any(|node| node.id == "a"));
    assert!(!equation_graph(&after_delete).edges.iter().any(|edge| edge.source == "a" || edge.target == "a"));

    // 🔎️ Assert on the captured payload fields directly (not just a round-trip) — a
    // `create-node` whose id already exists is a documented no-op, which would silently mask
    // a wrong-field bug in a pure round-trip check.
    let base_graph = equation_graph(&base);
    let original_node = base_graph.nodes.iter().find(|node| node.id == "a").expect("fixture has node a");
    let first_step = delete.inverse(&base).into_iter().next().expect("inverse has at least a create step");
    match &first_step {
        EquationMutation::CreateNode(payload) => {
            assert_eq!(payload.id, original_node.id);
            assert_eq!(payload.label, original_node.label);
            assert_eq!((payload.x, payload.y), (original_node.x, original_node.y));
        }
        other => panic!("expected CreateNode as the first inverse step, got {other:?}"),
    }
    let severed_edge_ids: Vec<String> = base_graph.edges.iter().filter(|edge| edge.source == "a" || edge.target == "a").map(|edge| edge.id.clone()).collect();
    let reconnected_ids: Vec<String> = delete
        .inverse(&base)
        .into_iter()
        .skip(1)
        .map(|mutation| match mutation {
            EquationMutation::ConnectNodes(payload) => payload.id,
            other => panic!("expected ConnectNodes for every severed edge, got {other:?}"),
        })
        .collect();
    assert_eq!(reconnected_ids, severed_edge_ids);

    let undo = delete.inverse(&base);
    let mut state = after_delete;
    for step in &undo {
        state = step.diff(&base).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(equation_graph(&state), base_graph, "delete-node's inverse must restore the node and every severed edge");
}

#[semio_framework_async_macros::async_test]
async fn move_point_inverse_restores_old_position() {
    let base = EquationSnapshot::default();
    let original = equation_geometry(&base).points[0].clone();
    let mutation = EquationMutation::MovePoint(move_point::MovePoint { index: 0, x: 999.0, y: 999.0 });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(equation_geometry(&after).points[0], EquationPoint { x: 999.0, y: 999.0 });

    let undo = mutation.inverse(&base);
    let mut state = after;
    for step in &undo {
        state = step.diff(&base).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(equation_geometry(&state).points[0], original);
}

#[semio_framework_async_macros::async_test]
async fn insert_point_inverse_is_remove_point_at_same_index() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::InsertPoint(insert_point::InsertPoint { index: 1, x: 5.0, y: 6.0 });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(equation_geometry(&after).points.len(), equation_geometry(&base).points.len() + 1);

    let undo = mutation.inverse(&base);
    let mut state = after;
    for step in &undo {
        state = step.diff(&base).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(equation_geometry(&state), equation_geometry(&base));
}

#[semio_framework_async_macros::async_test]
async fn delete_nodes_plural_cascades_like_the_singular_form() {
    let base = EquationSnapshot::default();
    let ids = vec!["a".to_string(), "b".to_string()];
    let mutation = EquationMutation::DeleteNodes(delete_nodes::DeleteNodes { ids: ids.clone() });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(equation_graph(&after).nodes.iter().all(|node| !ids.contains(&node.id)));
    assert!(equation_graph(&after).edges.iter().all(|edge| !ids.contains(&edge.source) && !ids.contains(&edge.target)));

    let undo = mutation.inverse(&base);
    let mut state = after;
    for step in &undo {
        state = step.diff(&base).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(equation_graph(&state), equation_graph(&base));
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(EquationMutation::kinds().len(), 15);
    let mutation = EquationMutation::ChangeGraphDirected(change_graph_directed::ChangeGraphDirected { new_directed: false });
    assert_eq!(mutation.semantics().kind, "change-graph-directed");
    assert_eq!(mutation.semantics().record, "ChangedGraphDirected");
}

#[semio_framework_async_macros::async_test]
async fn connect_then_disconnect_nodes_round_trips() {
    let base = EquationSnapshot::default();
    let connect = EquationMutation::ConnectNodes(connect_nodes::ConnectNodes { id: "e-new".into(), source: "a".into(), target: "d".into() });
    let after_connect = connect.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert!(equation_graph(&after_connect).edges.iter().any(|edge| edge.id == "e-new"));

    let undo = connect.inverse(&base);
    let mut state = after_connect;
    for step in &undo {
        state = step.diff(&base).diff().apply(&state).expect("valid mutation diff");
    }
    assert_eq!(equation_graph(&state), equation_graph(&base));
}

//#region ⚖️SemanticLaws
/// ⚖️ `assert_mutation_inverse_law` (`protocol::os_spr::protocol_laws`) against the remaining kinds not
/// already covered by an explicit round-trip test above: the two document-root scalar
/// setters and the two remaining collection verbs (`change`/`remove`).
#[semio_framework_async_macros::async_test]
async fn change_graph_directed_obeys_the_inverse_law() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::ChangeGraphDirected(change_graph_directed::ChangeGraphDirected { new_directed: !equation_graph(&base).directed });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn update_graph_algorithm_obeys_the_inverse_law() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::UpdateGraphAlgorithm(update_graph_algorithm::UpdateGraphAlgorithm { new_algorithm: "dijkstra".into(), new_algorithm_seed: Some("seed-1".into()) });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn change_node_label_obeys_the_inverse_law() {
    let base = EquationSnapshot::default();
    let id = equation_graph(&base).nodes[0].id.clone();
    let mutation = EquationMutation::ChangeNodeLabel(change_node_label::ChangeNodeLabel { id, new_label: "Relabeled".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_point_obeys_the_inverse_law() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::RemovePoint(remove_point::RemovePoint { index: 0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn change_coefficient_obeys_the_inverse_law() {
    // 🔎️ Default `equation` is the integer literal `0` at label 0 — a numeric leaf, so this
    // exercises the real replace/restore path, not the no-op branch.
    let base = EquationSnapshot::default();
    let label = base.equation.expr.label;
    let mutation = EquationMutation::ChangeCoefficient(change_coefficient::ChangeCoefficient { label, numer: "5".into(), denom: "2".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn change_coefficient_sets_the_targeted_numeric_leaf() {
    let base = EquationSnapshot::default();
    let label = base.equation.expr.label;
    let mutation = EquationMutation::ChangeCoefficient(change_coefficient::ChangeCoefficient { label, numer: "7".into(), denom: "1".into() });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(after.equation.find(label).map(|node| node.kind.clone()), Some(crate::standards::v1::subsets::any::schema::snapshot::EquationNodeKind::Integer { lexeme: "7".to_string() }));
}

#[semio_framework_async_macros::async_test]
async fn change_coefficient_at_an_unknown_label_is_a_no_op() {
    // 🔎️ Diff computed from `(payload, base)` — a stale/foreign label leaves `equation`
    // byte-identical to `base`'s, never a panic or a silently-inserted wrong node.
    let base = EquationSnapshot::default();
    let unknown_label = crate::standards::v1::subsets::any::schema::snapshot::EquationNodeLabel(999);
    let mutation = EquationMutation::ChangeCoefficient(change_coefficient::ChangeCoefficient { label: unknown_label, numer: "7".into(), denom: "1".into() });
    let after = mutation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(after.equation, base.equation);
    assert_eq!(mutation.inverse(&base), Vec::new(), "no target ⇒ nothing to undo");
}
//#endregion ⚖️SemanticLaws

//#region 🔖️OutcomeLaws
/// 🪧 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS Pass 3 — one test per
/// verb family, calling the two test context laws landed under their frozen names
/// (`assert_missing_target_is_error`/`assert_fatal_never_applies`,
/// `📡️spr/🧪️test/🦀️kit.rs`). `assert_outcome_policy_matrix` is NOT landed under that
/// name — only the differently-shaped `assert_policy_matrix(rejects, is_applicable)` exists,
/// which asserts the frozen 3×4 policy table directly rather than one outcome per verb family,
/// so it is not a drop-in substitute here; see this lane's report to the coordinator.
#[semio_framework_async_macros::async_test]
async fn delete_node_missing_target_is_error() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::DeleteNode(delete_node::DeleteNode { id: "nonexistent".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_nodes_all_missing_targets_is_error() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::DeleteNodes(delete_nodes::DeleteNodes { ids: vec!["nonexistent-1".into(), "nonexistent-2".into()] });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn remove_point_missing_target_is_error() {
    let base = EquationSnapshot::default();
    let out_of_range = equation_geometry(&base).points.len();
    let mutation = EquationMutation::RemovePoint(remove_point::RemovePoint { index: out_of_range });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn move_node_missing_target_is_error() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::MoveNode(move_node::MoveNode { id: "nonexistent".into(), x: 1.0, y: 1.0 });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn change_node_label_missing_target_is_error() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::ChangeNodeLabel(change_node_label::ChangeNodeLabel { id: "nonexistent".into(), new_label: "X".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_nodes_missing_target_is_error() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::ConnectNodes(connect_nodes::ConnectNodes { id: "e-new".into(), source: "nonexistent".into(), target: "a".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn disconnect_nodes_missing_target_is_error() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::DisconnectNodes(disconnect_nodes::DisconnectNodes { id: "nonexistent".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_node_duplicate_id_fatal_never_applies() {
    let base = EquationSnapshot::default();
    let existing_id = equation_graph(&base).nodes[0].id.clone();
    let mutation = EquationMutation::CreateNode(create_node::CreateNode { id: existing_id, label: "dup".into(), x: 0.0, y: 0.0 });
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&Mutation::diff(&mutation, &base)).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_nodes_duplicate_id_fatal_never_applies() {
    let base = EquationSnapshot::default();
    let existing_edge_id = equation_graph(&base).edges[0].id.clone();
    let mutation = EquationMutation::ConnectNodes(connect_nodes::ConnectNodes { id: existing_edge_id, source: "a".into(), target: "d".into() });
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&Mutation::diff(&mutation, &base)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_point_non_finite_fatal_never_applies() {
    let base = EquationSnapshot::default();
    let mutation = EquationMutation::MovePoint(move_point::MovePoint { index: 0, x: f64::NAN, y: 0.0 });
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&Mutation::diff(&mutation, &base)).await;
}

#[semio_framework_async_macros::async_test]
async fn change_coefficient_zero_denominator_fatal_never_applies() {
    let base = EquationSnapshot::default();
    let label = base.equation.expr.label;
    let mutation = EquationMutation::ChangeCoefficient(change_coefficient::ChangeCoefficient { label, numer: "1".into(), denom: "0".into() });
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&Mutation::diff(&mutation, &base)).await;
}
//#endregion 🔖️OutcomeLaws
