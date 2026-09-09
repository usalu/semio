use super::*;
use crate::editor::fem2d::commands::{add_area_load, add_combination, add_load_case, add_member_udl, add_node, set_self_weight};
use crate::editor::fem2d::testkit::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;

async fn with_dead_case(app: &mut crate::editor::fem2d::testkit::Fem2dApp) {
    dispatch(app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false })).await;
}

#[semio_framework_async_macros::async_test]
async fn add_load_case_and_combination_emit_ops_2d() {
    let mut app = fem2d_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })).await;
    assert_eq!(app.snapshot().expect("snapshot").load_cases.last().expect("case added").name, "Live");

    let dead_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![crate::FemCombinationTerm { case_id: dead_id.clone(), factor: 1.35 }] })).await;
    assert_eq!(app.snapshot().expect("snapshot").combinations.last().expect("combination added").terms, vec![crate::FemCombinationTerm { case_id: dead_id, factor: 1.35 }]);
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_with_no_existing_case_creates_one_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddNodalLoad(AddNodalLoad { node_id, dof: FemDof::Ty, value: -5000.0, case_id: None })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.load_cases.len(), 1);
    assert_eq!(snapshot.load_cases[0].id, "case-1");
    assert!(matches!(snapshot.load_cases[0].loads[0], FemLoad::Nodal { .. }));
}

#[semio_framework_async_macros::async_test]
async fn add_area_load_targets_named_case_2d() {
    let mut app = fem2d_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })).await;
    let live_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
    dispatch(&mut app, Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some(live_id.clone()) })).await;
    let load_case = app.snapshot().expect("snapshot").load_cases[1].clone();
    assert_eq!(load_case.id, live_id);
    assert!(matches!(load_case.loads[0], FemLoad::Area { .. }));
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_toggles_case_2d() {
    let mut app = fem2d_app();
    with_dead_case(&mut app).await;
    let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
    dispatch(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id, enabled: true })).await;
    assert!(app.snapshot().expect("snapshot").load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_unknown_case_is_a_no_op_2d() {
    let mut app = fem2d_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true })).await;
    assert!(!app.snapshot().expect("snapshot").load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_action_targets_named_case_2d() {
    let mut app = fem2d_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })).await;
    let live_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
    dispatch(&mut app, Fem2dCommand::AddNodalLoad(AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: Some(live_id.clone()) })).await;
    let load_case = app.snapshot().expect("snapshot").load_cases[1].clone();
    assert_eq!(load_case.id, live_id);
    assert!(matches!(load_case.loads[0], FemLoad::Nodal { .. }));
}

#[semio_framework_async_macros::async_test]
async fn add_member_udl_action_emits_op_2d() {
    let mut app = fem2d_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None })).await;
    assert!(matches!(app.snapshot().expect("snapshot").load_cases[0].loads[0], FemLoad::MemberUdl { .. }));
}
