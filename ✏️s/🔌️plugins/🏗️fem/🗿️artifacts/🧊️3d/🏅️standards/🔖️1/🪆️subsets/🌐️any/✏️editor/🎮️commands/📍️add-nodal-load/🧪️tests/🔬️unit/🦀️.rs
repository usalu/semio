
use super::*;
use crate::editor::fem3d::Fem3dCommand;
use crate::editor::fem3d::commands::{add_combination, add_load_case, add_member_udl, add_node, set_self_weight};
use crate::editor::fem3d::testkit::{Fem3dApp, dispatch, fem3d_empty_app};

async fn app_with_load_case() -> Fem3dApp {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false })).await;
    app
}

#[semio_framework_async_macros::async_test]
async fn resolve_load_case_returns_none_when_none_exist() {
    let snapshot = Fem3dSnapshot::default();
    assert!(resolve_load_case(&snapshot, None).is_none());
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_with_no_existing_case_creates_one() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0, z: 0.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.clone();
    dispatch(&mut app, Fem3dCommand::AddNodalLoad(AddNodalLoad { node_id, dof: crate::FemDof::Tz, value: -5000.0, case_id: None })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.load_cases.len(), 1);
    assert_eq!(snapshot.load_cases[0].id, "case-1");
    assert!(matches!(snapshot.load_cases[0].loads[0], FemLoad::Nodal { .. }));
}

#[semio_framework_async_macros::async_test]
async fn add_member_udl_action_emits_op_3d() {
    let mut app = app_with_load_case().await;
    dispatch(&mut app, Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -2000.0, case_id: None })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let load_case = &snapshot.load_cases[0];
    assert!(matches!(load_case.loads[0], FemLoad::MemberUdl { .. }));
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_targets_named_case() {
    let mut app = app_with_load_case().await;
    dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })).await;
    let live_case_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
    dispatch(&mut app, Fem3dCommand::AddNodalLoad(AddNodalLoad { node_id: "n2".into(), dof: crate::FemDof::Tz, value: -5000.0, case_id: Some(live_case_id) })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert!(snapshot.load_cases[1].loads.iter().any(|l| matches!(l, FemLoad::Nodal { .. })));
    assert!(snapshot.load_cases[0].loads.is_empty(), "the untargeted case must stay untouched");
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_toggles_existing_case() {
    let mut app = app_with_load_case().await;
    let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
    dispatch(&mut app, Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id, enabled: true })).await;
    assert!(app.snapshot().expect("snapshot").load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_unknown_case_is_a_no_op() {
    let mut app = app_with_load_case().await;
    dispatch(&mut app, Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true })).await;
    assert!(!app.snapshot().expect("snapshot").load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn add_combination_parses_terms_json() {
    let mut app = app_with_load_case().await;
    dispatch(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "[[\"case-0\",1.35]]".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.combinations.len(), 1);
    assert_eq!(snapshot.combinations[0].terms.get("case-0"), Some(&1.35));
}

#[semio_framework_async_macros::async_test]
async fn add_combination_invalid_terms_json_is_a_no_op() {
    let mut app = app_with_load_case().await;
    dispatch(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "not json".into() })).await;
    assert!(app.snapshot().expect("snapshot").combinations.is_empty());
}
