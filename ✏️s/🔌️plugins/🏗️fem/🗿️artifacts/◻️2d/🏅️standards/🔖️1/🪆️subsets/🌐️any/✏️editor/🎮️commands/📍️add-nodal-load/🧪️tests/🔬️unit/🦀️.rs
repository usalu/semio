use crate::context::{dispatch, dispatch_editor_action, dispatch_refused, fem2d_app, fem2d_empty_app};
use crate::media_ports::editing_doc;
use semio_framework_value::DslValue;
use semio_plugin::{ConfigView, HistoryView, NoConfig};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{
    add_combination, add_load_case, add_nodal_load::{self, AddNodalLoad}, add_node, patch_combination, set_self_weight,
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
    let emitted = add_combination::handle(&add_combination::AddCombination { name: "ULS".into(), terms: format!("{dead_id}:1.35") }, &doc, &cfg).expect("lower the typed combination");
    let [Fem2dMutation::CreateCombination(mutation)] = emitted.artifact_mutations.as_slice() else { panic!("one create-combination intent") };
    assert_eq!(mutation.combination.terms, terms);
    dispatch_editor_action(&mut app, "addCombination", DslValue::from(&serde_json::json!({ "name": "ULS", "terms": format!("{dead_id}:1.35") }))).await;
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
async fn set_self_weight_unknown_case_is_refused_by_name_2d() {
    let mut app = fem2d_empty_app();
    with_dead_case(&mut app).await;
    let refusal = dispatch_refused(&mut app, Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true })).await;
    assert!(refusal.contains("fem2d.self-weight.case-unknown"), "{refusal}");
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
async fn a_named_missing_case_is_refused_not_redirected_to_another_case() {
    let app = fem2d_app();
    let history = HistoryView::empty();
    let doc = editing_doc(app.snapshot_ref(), &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let command = AddNodalLoad { node_id: "n1".into(), dof: FemDof::Tx, value: 1.0, case_id: Some("missing-case".into()) };
    let fault = add_nodal_load::handle(&command, &doc, &cfg).err().expect("a named case the document lacks is refused");
    assert_eq!(fault.code.0.as_str(), "fem2d.reference.load-case-unknown");
}

/// 🪪️ LAW: every 2D action that names an entity refuses an id the document does not hold with a coded fault (never a silent publication rejection), a member needs two different nodes, and each refusal reads as an EN and a DE notice that quotes the id.
#[test]
fn actions_naming_an_unknown_entity_are_refused_with_localized_notices_2d() {
    use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{add_area_load, add_bar, add_beam, add_member_udl, add_support};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🗂️remove-selection/🧪️tests/🔬️unit/🔣️.json")).expect("third-party JSON fixture parser");
    let before = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring::new(semio_framework_pack_json::from_json_str(&fixture["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("shared snapshot wire fixture"));
    let snapshot = &*before;
    let history = HistoryView::empty();
    let doc = editing_doc(snapshot, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let node = snapshot.nodes[0].id.as_str().to_owned();
    let other = snapshot.nodes[1].id.as_str().to_owned();
    let (material, section) = (snapshot.materials[0].id.as_str().to_owned(), snapshot.sections[0].id.as_str().to_owned());
    let beam = |start: &str, end: &str, material: &str, section: &str| add_beam::handle(&add_beam::AddBeam { start: start.into(), end: end.into(), material_id: material.into(), section_id: section.into() }, &doc, &cfg).err();
    let bar = |start: &str, end: &str| add_bar::handle(&add_bar::AddBar { start: start.into(), end: end.into(), material_id: material.clone(), section_id: section.clone() }, &doc, &cfg).err();
    let cases = vec![
        (add_support::handle(&add_support::AddSupport { node_id: "ghost".into(), fixed: Vec::new() }, &doc, &cfg).err(), "fem2d.reference.node-unknown", "ghost"),
        (add_nodal_load::handle(&AddNodalLoad { node_id: "ghost".into(), dof: FemDof::Ty, value: 1.0, case_id: None }, &doc, &cfg).err(), "fem2d.reference.node-unknown", "ghost"),
        (add_nodal_load::handle(&AddNodalLoad { node_id: node.clone(), dof: FemDof::Ty, value: 1.0, case_id: Some("ghost".into()) }, &doc, &cfg).err(), "fem2d.reference.load-case-unknown", "ghost"),
        (add_member_udl::handle(&add_member_udl::AddMemberUdl { element_id: "ghost".into(), wx: 0.0, wy: 1.0, case_id: None }, &doc, &cfg).err(), "fem2d.reference.element-unknown", "ghost"),
        (add_area_load::handle(&add_area_load::AddAreaLoad { region_id: "ghost".into(), pressure: 1.0, case_id: None }, &doc, &cfg).err(), "fem2d.reference.region-unknown", "ghost"),
        (bar(&node, "ghost"), "fem2d.reference.node-unknown", "ghost"),
        (bar(&node, &node), "fem2d.member.degenerate", node.as_str()),
        (beam(&node, &other, "ghost", &section), "fem2d.reference.material-unknown", "ghost"),
        (beam(&node, &other, &material, "ghost"), "fem2d.reference.section-unknown", "ghost"),
    ];
    let table = semio_framework::fault_notice_definitions(<semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp as semio_plugin::ArtifactEditor>::fault_notices());
    assert!(semio_framework::validate_fault_notices(&table).is_empty());
    for (fault, code, id) in cases {
        let fault = fault.unwrap_or_else(|| panic!("{code} for '{id}' is refused"));
        assert_eq!(fault.code.0.as_str(), code);
        for locale in [semio_ui_locale::Locale::En, semio_ui_locale::Locale::De] {
            let shown = semio_framework::fault_notice_text(&table, code, fault.params.as_deref(), semio_ui_locale::Terminology::Native, locale).unwrap_or_else(|| panic!("{code} has a {locale:?} notice"));
            assert!(!shown.contains('{') && (code.ends_with("degenerate") || shown.contains(id)), "{locale:?}: {shown}");
        }
    }
    assert!(add_bar::handle(&add_bar::AddBar { start: node, end: other, material_id: material, section_id: section }, &doc, &cfg).is_ok(), "a member between two known nodes with a known material and section is still lowered");
}
