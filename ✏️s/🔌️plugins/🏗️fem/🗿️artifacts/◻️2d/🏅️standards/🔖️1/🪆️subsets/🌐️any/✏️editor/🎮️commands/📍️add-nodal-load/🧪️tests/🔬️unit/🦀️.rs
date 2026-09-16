use super::*;
use crate::editor::fem2d::commands::{add_area_load, add_combination, add_load_case, add_member_udl, add_node, set_self_weight};
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app, fem2d_empty_app};
use crate::editor::fem2d::Fem2dCommand;

async fn with_dead_case(app: &mut crate::editor::fem2d::unit_tests::context::Fem2dApp) {
    dispatch(app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false })).await;
}

#[semio_framework_async_macros::async_test]
async fn add_load_case_and_combination_emit_ops_2d() {
    let mut app = fem2d_empty_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })).await;
    assert_eq!(app.snapshot().expect("snapshot").load_cases.last().expect("case added").name, "Live");

    let dead_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: vec![crate::FemCombinationTerm { case_id: dead_id.clone(), factor: 1.35 }] })).await;
    assert_eq!(app.snapshot().expect("snapshot").combinations.last().expect("combination added").terms, vec![crate::FemCombinationTerm { case_id: dead_id, factor: 1.35 }]);
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_with_no_existing_case_creates_one_2d() {
    let mut app = fem2d_empty_app();
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
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Snow".into(), self_weight: false })).await;
    let snow_id = app.snapshot().expect("snapshot").load_cases.last().expect("case added").id.clone();
    dispatch(&mut app, Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some(snow_id.clone()) })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let load_case = snapshot.load_cases.iter().find(|case| case.id == snow_id).expect("the named case");
    assert!(matches!(load_case.loads.last(), Some(FemLoad::Area { region_id, .. }) if region_id == "r1"), "{:?}", load_case.loads);
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_toggles_case_2d() {
    let mut app = fem2d_empty_app();
    with_dead_case(&mut app).await;
    let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
    dispatch(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id, enabled: true })).await;
    assert!(app.snapshot().expect("snapshot").load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_unknown_case_is_a_no_op_2d() {
    let mut app = fem2d_empty_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true })).await;
    assert!(!app.snapshot().expect("snapshot").load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_action_targets_named_case_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Wind".into(), self_weight: false })).await;
    let wind_id = app.snapshot().expect("snapshot").load_cases.last().expect("case added").id.clone();
    dispatch(&mut app, Fem2dCommand::AddNodalLoad(AddNodalLoad { node_id: "n1".into(), dof: FemDof::Ty, value: -5000.0, case_id: Some(wind_id.clone()) })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let load_case = snapshot.load_cases.iter().find(|case| case.id == wind_id).expect("the named case");
    assert!(matches!(load_case.loads.last(), Some(FemLoad::Nodal { node_id, value, .. }) if node_id == "n1" && *value == -5000.0), "{:?}", load_case.loads);
}

#[semio_framework_async_macros::async_test]
async fn add_member_udl_action_emits_op_2d() {
    let mut app = fem2d_app();
    let before: usize = app.snapshot().expect("snapshot").load_cases.iter().map(|case| case.loads.len()).sum();
    dispatch(&mut app, Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e3".into(), wx: 0.0, wy: -500.0, case_id: None })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let after: usize = snapshot.load_cases.iter().map(|case| case.loads.len()).sum();
    assert_eq!(after, before + 1, "one member load lands in one case");
    assert!(snapshot.load_cases.iter().flat_map(|case| case.loads.iter()).any(|load| matches!(load, FemLoad::MemberUdl { element_id, wy, .. } if element_id == "e3" && *wy == -500.0)));
}
