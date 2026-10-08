use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{diff_semio_flow_mutation, inverse_semio_flow_mutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

fn base() -> SequenceWorkingScene {
    let host = neural_engine::ColdOwner::new(crate::snapshot::schema::default_host_snapshot());
    SequenceWorkingScene { steps: host.steps.clone(), edges: host.edges.clone() }
}

fn content(scene: &SequenceWorkingScene) -> SemioFlowSnapshot {
    crate::sequence_content_snapshot_from_working(&scene.steps, &scene.edges)
}

fn forward(state: &SemioFlowSnapshot, leaf: &SemioFlowMutation) -> SemioFlowSnapshot {
    let outcome = diff_semio_flow_mutation(leaf, state);
    assert!(outcome.worst_level().is_none_or(|level| level < semio_framework_diagnostic::Severity::Error), "{leaf:?} refused: {:?}", outcome.messages());
    semio_s_artifact_stdio_semio::apply_diff(outcome.diff(), state).expect("a leaf diff applies to the state it was computed from")
}

fn assert_leaves_reproduce_the_scene_and_undo_restores_it(before: &SequenceWorkingScene, edit: &SceneEdit) {
    let (mut state, mut undo) = (content(before), Vec::new());
    for leaf in &edit.leaves {
        undo.push(inverse_semio_flow_mutation(leaf, &state).expect("a leaf has a concrete inverse"));
        state = forward(&state, leaf);
    }
    assert_eq!(state, content(&edit.scene), "the leaves carry the base content to the gesture's working scene");
    for rows in undo.iter().rev() {
        for row in rows.iter().rev() {
            state = forward(&state, row);
        }
    }
    assert_eq!(state, content(before), "undo, last leaf first, restores the base content");
}

#[semio_framework_async_macros::async_test]
async fn adding_a_step_is_one_insert_node_with_a_fresh_id() {
    let scene = base();
    let mut edit = SceneEdit::new(scene.clone());
    let id = edit.add_step("log.print", 10.0, 20.0, None);
    assert!(scene.steps.iter().all(|step| step.id != id));
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::InsertNode(_)]), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn a_dropped_step_lands_in_the_picked_expanded_control_slot_only() {
    let mut scene = base();
    scene.steps.push(SequenceStep { id: "step-90".into(), kind: "control.if".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false });
    scene.steps.push(SequenceStep { id: "step-91".into(), kind: "control.while".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: true });
    let mut edit = SceneEdit::new(scene.clone());
    let inside = edit.add_step_dropped("log.print", 1.0, 1.0, Some("step-90"));
    let collapsed = edit.add_step_dropped("log.print", 1.0, 1.0, Some("step-91"));
    let slot = |id: &str| edit.scene.steps.iter().find(|step| step.id == id).and_then(|step| step.slot.clone());
    assert_eq!(slot(&inside), Some(SlotRef { owner: "step-90".into(), name: "then".into() }));
    assert_eq!(slot(&collapsed), None);
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn removing_a_middle_step_removes_its_edges_first_then_the_node_and_undo_restores_both() {
    let scene = base();
    let middle = scene.steps[scene.steps.len() / 2].id.clone();
    let mut edit = SceneEdit::new(scene.clone());
    edit.remove_steps(std::slice::from_ref(&middle));
    let kinds: Vec<&str> = edit.leaves.iter().map(|leaf| if matches!(leaf, SemioFlowMutation::RemoveEdge(_)) { "edge" } else if matches!(leaf, SemioFlowMutation::RemoveNode(_)) { "node" } else { "other" }).collect();
    assert_eq!(kinds.last(), Some(&"node"));
    assert!(kinds.iter().all(|kind| *kind != "other") && kinds.iter().filter(|kind| **kind == "node").count() == 1);
    assert!(edit.scene.edges.iter().all(|edge| edge.from != middle && edge.to != middle));
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn removing_a_control_takes_its_nested_members_transitively() {
    let mut scene = base();
    let member = |id: &str, kind: &str, owner: &str| SequenceStep { id: id.into(), kind: kind.into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: owner.into(), name: "body".into() }), collapsed: false };
    scene.steps.push(SequenceStep { id: "step-90".into(), kind: "control.while".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false });
    scene.steps.push(member("step-91", "control.repeat", "step-90"));
    scene.steps.push(member("step-92", "log.print", "step-91"));
    assert_eq!(removal_closure(&scene, ["step-90".to_string()]), vec!["step-90".to_string(), "step-91".to_string(), "step-92".to_string()]);
    let mut edit = SceneEdit::new(scene.clone());
    edit.remove_steps(&removal_closure(&scene, ["step-90".to_string()]));
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn a_drag_is_one_relative_drag_nodes_and_a_zero_or_absent_target_is_nothing() {
    let scene = base();
    let ids: Vec<String> = scene.steps.iter().take(2).map(|step| step.id.clone()).collect();
    let mut edit = SceneEdit::new(scene.clone());
    assert!(edit.drag(&ids, 40.0, -12.5));
    assert!(!edit.drag(&ids, 0.0, 0.0) && !edit.drag(&["absent".to_string()], 1.0, 1.0));
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::DragNodes(drag)] if drag.targets == ids && drag.dx == 40.0 && drag.dy == -12.5), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn a_layout_groups_equal_offsets_into_one_drag_each() {
    let scene = base();
    let target: Vec<(String, f64, f64)> = scene.steps.iter().enumerate().map(|(index, step)| (step.id.clone(), step.x + if index % 2 == 0 { 10.0 } else { 20.0 }, step.y + 5.0)).collect();
    let mut edit = SceneEdit::new(scene.clone());
    edit.move_to(&target);
    assert_eq!(edit.leaves.len(), if scene.steps.len() > 1 { 2 } else { 1 });
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn params_and_collapse_are_single_set_node_params_and_unchanged_values_are_nothing() {
    let mut scene = base();
    scene.steps.push(SequenceStep { id: "step-90".into(), kind: "control.if".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false });
    let id = scene.steps[0].id.clone();
    let mut edit = SceneEdit::new(scene.clone());
    let same = scene.steps[0].params.clone();
    assert!(edit.set_params(&id, same).is_err());
    let changed: StepParams = semio_framework_pack_json::from_json_str("{\"text\":\"changed\"}", semio_framework_pack_json::JsonMemberPolicy::Reject).expect("params");
    assert!(edit.set_params(&id, changed).is_ok());
    assert!(edit.toggle_collapsed("step-90") && !edit.toggle_collapsed(&id));
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::SetNodeParam(params), SemioFlowMutation::SetNodeParam(collapsed)] if params.key == "params" && collapsed.key == "collapsed" && collapsed.value == "true"), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn connecting_replaces_the_incoming_edge_and_refuses_cycles_self_loops_and_a_second_outgoing_edge() {
    let mut scene = base();
    scene.edges.clear();
    let ids: Vec<String> = scene.steps.iter().take(3).map(|step| step.id.clone()).collect();
    let mut edit = SceneEdit::new(scene.clone());
    assert!(edit.connect(&ids[0], &ids[1], true));
    assert!(!edit.connect(&ids[0], &ids[2], true), "one outgoing edge per step");
    assert!(!edit.connect(&ids[1], &ids[0], true), "no cycle");
    assert!(!edit.connect(&ids[1], &ids[1], true), "no self loop");
    assert!(edit.connect(&ids[2], &ids[1], true), "re-targeting an entered step replaces its incoming edge");
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::InsertEdge(_), SemioFlowMutation::RemoveEdge(_), SemioFlowMutation::InsertEdge(_)]), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}

#[semio_framework_async_macros::async_test]
async fn disconnecting_removes_exactly_the_named_edge() {
    let mut scene = base();
    let (first, second) = (scene.steps[0].id.clone(), scene.steps[1].id.clone());
    scene.edges = vec![SequenceEdge { id: "edge-101".into(), from: first.clone(), to: second.clone() }];
    let mut edit = SceneEdit::new(scene.clone());
    assert!(!edit.disconnect(&second, &first));
    assert!(edit.disconnect(&first, &second));
    assert!(matches!(edit.leaves.as_slice(), [SemioFlowMutation::RemoveEdge(edge)] if edge.id == "edge-101"), "{:?}", edit.leaves);
    assert_leaves_reproduce_the_scene_and_undo_restores_it(&scene, &edit);
}
