use crate::context::{dispatch, dispatch_editor_action, fem2d_app, fem2d_empty_app};
use crate::media_ports::editing_doc;
use semio_framework_value::DslValue;
use semio_plugin::{ConfigView, HistoryView, NoConfig};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{
    add_combination, add_load_case, add_nodal_load::{self, AddNodalLoad}, add_node, patch_combination,
};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::{FemCombinationTerm, FemDof, FemLoad};
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

async fn with_dead_case(app: &mut crate::context::Fem2dApp) {
    dispatch(app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false })).await;
}

#[semio_framework_async_macros::async_test]
async fn add_load_case_and_combination_emit_ops_2d() {
    let mut app = fem2d_empty_app();
    with_dead_case(&mut app).await;
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })).await;
    assert_eq!(app.snapshot_ref().load_cases.last().expect("case added").name, "Live");

    let dead_id = app.snapshot_ref().load_cases[0].id.clone();
    let terms = vec![FemCombinationTerm { case_id: dead_id.clone(), factor: 1.35 }];
    let history = HistoryView::empty();
    let doc = editing_doc(app.snapshot_ref(), &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emitted = add_combination::handle(&add_combination::AddCombination { name: "ULS".into(), terms: terms.clone() }, &doc, &cfg).expect("lower the typed combination");
    let [Fem2dMutation::CreateCombination(mutation)] = emitted.artifact_mutations.as_slice() else { panic!("one create-combination intent") };
    assert_eq!(mutation.combination.terms, terms);
    dispatch_editor_action(&mut app, "addCombination", DslValue::from(&serde_json::json!({ "name": "ULS" }))).await;
    let combination_id = app.snapshot_ref().combinations.last().expect("combination added").id.clone();
    dispatch(&mut app, Fem2dCommand::PatchCombination(patch_combination::PatchCombination {
        id: combination_id, field: format!("{}{dead_id}", patch_combination::TERM_FIELD_PREFIX), value: "1.35".into(),
    })).await;
    assert_eq!(app.snapshot_ref().combinations.last().expect("combination added").terms, terms);
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_with_no_existing_case_creates_one_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0 })).await;
    let node_id = app.snapshot_ref().nodes[0].id.as_str().to_owned();
    dispatch_editor_action(&mut app, "addNodalLoad", DslValue::from(&serde_json::json!({ "nodeId": node_id, "dof": "Ty", "value": -5000.0 }))).await;
    let snapshot = app.snapshot_ref();
    assert_eq!(snapshot.load_cases.len(), 1);
    assert!(snapshot.load_cases[0].id.starts_with("case-"));
    assert_eq!(snapshot.load_cases[0].id.len(), 21);
    assert!(matches!(snapshot.load_cases[0].loads[0], FemLoad::Nodal { .. }));
}

#[semio_framework_async_macros::async_test]
async fn add_area_load_targets_named_case_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Snow".into(), self_weight: false })).await;
    let snow_id = app.snapshot_ref().load_cases.last().expect("case added").id.clone();
    dispatch_editor_action(&mut app, "addAreaLoad", DslValue::from(&serde_json::json!({ "regionId": "r1", "pressure": 5000.0, "caseId": &snow_id }))).await;
    let snapshot = app.snapshot_ref();
    let load_case = snapshot.load_cases.iter().find(|case| case.id == snow_id).expect("the named case");
    assert!(matches!(load_case.loads.last(), Some(FemLoad::Area { region_id, .. }) if region_id == "r1"), "{:?}", load_case.loads);
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_toggles_case_2d() {
    let mut app = fem2d_empty_app();
    with_dead_case(&mut app).await;
    let case_id = app.snapshot_ref().load_cases[0].id.clone();
    dispatch_editor_action(&mut app, "setSelfWeight", DslValue::from(&serde_json::json!({ "caseId": case_id, "enabled": true }))).await;
    assert!(app.snapshot_ref().load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn set_self_weight_unknown_case_is_a_no_op_2d() {
    let mut app = fem2d_empty_app();
    with_dead_case(&mut app).await;
    dispatch_editor_action(&mut app, "setSelfWeight", DslValue::from(&serde_json::json!({ "caseId": "missing", "enabled": true }))).await;
    assert!(!app.snapshot_ref().load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_action_targets_named_case_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Wind".into(), self_weight: false })).await;
    let wind_id = app.snapshot_ref().load_cases.last().expect("case added").id.clone();
    dispatch_editor_action(&mut app, "addNodalLoad", DslValue::from(&serde_json::json!({ "nodeId": "n1", "dof": "Ty", "value": -5000.0, "caseId": &wind_id }))).await;
    let snapshot = app.snapshot_ref();
    let load_case = snapshot.load_cases.iter().find(|case| case.id == wind_id).expect("the named case");
    assert!(matches!(load_case.loads.last(), Some(FemLoad::Nodal { node_id, value, .. }) if node_id == "n1" && *value == -5000.0), "{:?}", load_case.loads);
}

#[semio_framework_async_macros::async_test]
async fn add_member_udl_action_emits_op_2d() {
    let mut app = fem2d_app();
    let before: usize = app.snapshot_ref().load_cases.iter().map(|case| case.loads.len()).sum();
    dispatch_editor_action(&mut app, "addMemberUdl", DslValue::from(&serde_json::json!({ "elementId": "e3", "wx": 0.0, "wy": -500.0 }))).await;
    let snapshot = app.snapshot_ref();
    let after: usize = snapshot.load_cases.iter().map(|case| case.loads.len()).sum();
    assert_eq!(after, before + 1, "one member load lands in one case");
    assert!(snapshot.load_cases.iter().flat_map(|case| case.loads.iter()).any(|load| matches!(load, FemLoad::MemberUdl { element_id, wy, .. } if element_id == "e3" && *wy == -500.0)));
}

#[semio_framework_async_macros::async_test]
async fn a_named_missing_case_is_not_redirected_to_another_case() {
    let app = fem2d_app();
    let history = HistoryView::empty();
    let doc = editing_doc(app.snapshot_ref(), &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let command = AddNodalLoad { node_id: "n1".into(), dof: FemDof::Tx, value: 1.0, case_id: Some("missing-case".into()) };
    let emitted = add_nodal_load::handle(&command, &doc, &cfg).expect("lower the nodal command with an explicit owner");
    let [Fem2dMutation::AddLoad(operation)] = emitted.artifact_mutations.as_slice() else { panic!("named target must stay explicit") };
    assert_eq!(operation.case_id, "missing-case");
}
