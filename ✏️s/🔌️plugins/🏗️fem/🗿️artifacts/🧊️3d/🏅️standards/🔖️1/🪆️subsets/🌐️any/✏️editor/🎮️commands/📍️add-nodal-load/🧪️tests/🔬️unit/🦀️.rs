use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_nodal_load::*;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::add_load;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_bar;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_combination;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_load_case;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_material;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_member_udl;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_node;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_section;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_self_weight;
use crate::context::dispatch;
use crate::context::fem3d_empty_app;
use crate::context::Fem3dApp;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

async fn app_with_load_case() -> Fem3dApp {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Dead".into(), self_weight: false })).await;
    app
}


#[semio_framework_async_macros::async_test]
async fn add_nodal_load_with_no_existing_case_creates_one() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0, z: 0.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.get().clone();
    dispatch(&mut app, Fem3dCommand::AddNodalLoad(AddNodalLoad { node_id, dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tz, value: -5000.0, case_id: None })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.load_cases.len(), 1);
    assert!(snapshot.load_cases[0].id.starts_with("case-"));
    assert_eq!(snapshot.load_cases[0].id.len(), 21);
    assert!(matches!(snapshot.load_cases[0].loads[0], FemLoad::Nodal { .. }));
}

#[semio_framework_async_macros::async_test]
async fn add_member_udl_action_emits_op_3d() {
    let mut app = app_with_load_case().await;
    dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0, z: 0.0 })).await;
    dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 4.0, y: 0.0, z: 0.0 })).await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 210e9, g: 81e9 })).await;
    dispatch(&mut app, Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.005, iy: 3.69e-5, iz: 1.34e-5, j: 2.1e-7 })).await;
    let (start, end, material, section) = {
        let snapshot = app.snapshot().expect("snapshot");
        (snapshot.nodes[0].id.get().clone(), snapshot.nodes[1].id.get().clone(), snapshot.materials[0].id.get().clone(), snapshot.sections[0].id.clone())
    };
    dispatch(&mut app, Fem3dCommand::AddBar(add_bar::AddBar { start, end, material_id: material, section_id: section })).await;
    let element_id = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id(&app.snapshot().expect("snapshot").elements[0]).to_string();
    dispatch(&mut app, Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id, wx: 0.0, wy: 0.0, wz: -2000.0, case_id: None })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let load_case = &snapshot.load_cases[0];
    assert!(matches!(load_case.loads[0], FemLoad::MemberUdl { .. }));
}

#[semio_framework_async_macros::async_test]
async fn add_nodal_load_targets_named_case() {
    let mut app = app_with_load_case().await;
    dispatch(&mut app, Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })).await;
    let live_case_id = app.snapshot().expect("snapshot").load_cases[1].id.clone();
    dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.get().clone();
    dispatch(&mut app, Fem3dCommand::AddNodalLoad(AddNodalLoad { node_id, dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tz, value: -5000.0, case_id: Some(live_case_id) })).await;
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
async fn set_self_weight_unknown_case_is_refused_by_name() {
    let mut app = app_with_load_case().await;
    let refusal = crate::context::dispatch_refused(&mut app, Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "missing".into(), enabled: true })).await;
    assert!(refusal.contains("fem3d.self-weight.case-unknown"), "{refusal}");
    assert!(!app.snapshot().expect("snapshot").load_cases[0].self_weight);
}

#[semio_framework_async_macros::async_test]
async fn add_combination_parses_terms_json() {
    let mut app = app_with_load_case().await;
    let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();
    let terms = format!("[[\"{case_id}\",1.35]]");
    dispatch(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.combinations.len(), 1);
    assert_eq!(snapshot.combinations[0].terms.get(&case_id), Some(&1.35));
}

#[semio_framework_async_macros::async_test]
async fn add_combination_invalid_terms_json_is_a_no_op() {
    let mut app = app_with_load_case().await;
    let refusal = crate::context::dispatch_refused(&mut app, Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "not json".into() })).await;
    assert!(refusal.contains("fem3d.combination.terms-unreadable"), "{refusal}");
    assert!(app.snapshot().expect("snapshot").combinations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_named_missing_case_is_refused_not_redirected_to_another_case() {
    let app = app_with_load_case().await;
    let history = semio_plugin::HistoryView::empty();
    let snapshot = app.snapshot().expect("snapshot");
    let view = ArtifactView::new(&snapshot, &history);
    let load = FemLoad::Nodal { id: "new-load".into(), node_id: "node".into(), dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tx, value: 1.0 };
    let fault = add_load_mutation(&view, Some("missing-case"), load).err().expect("a named case the document lacks is refused");
    assert_eq!(fault.code.0.as_str(), "fem3d.reference.load-case-unknown");
}

/// 🪪️ LAW: every 3D action that names an entity refuses an id the document does not hold with a coded fault (never a silent publication rejection), a member needs two different nodes, and each refusal reads as an EN and a DE notice that quotes the id.
#[test]
fn actions_naming_an_unknown_entity_are_refused_with_localized_notices_3d() {
    use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::{add_area_load, add_frame, add_support};
    let boot = crate::context::Retiring::new(semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::fem3d_boot_snapshot().expect("boot snapshot"));
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*boot, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let node = boot.nodes[0].id.as_str().to_owned();
    let other = boot.nodes[1].id.as_str().to_owned();
    let element = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id(&boot.elements[0]).to_owned();
    let (material, section) = (boot.materials[0].id.as_str().to_owned(), boot.sections[0].id.as_str().to_owned());
    let dof = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tz;
    let frame = |start: &str, end: &str, material: &str, section: &str| add_frame::handle(&add_frame::AddFrame { start: start.into(), end: end.into(), material_id: material.into(), section_id: section.into(), roll: 0.0 }, &doc, &cfg).err();
    let bar = |start: &str, end: &str| add_bar::handle(&add_bar::AddBar { start: start.into(), end: end.into(), material_id: material.clone(), section_id: section.clone() }, &doc, &cfg).err();
    let cases = vec![
        (add_support::handle(&add_support::AddSupport { node_id: "ghost".into(), fixed: Vec::new() }, &doc, &cfg).err(), "fem3d.reference.node-unknown", "ghost"),
        (handle(&AddNodalLoad { node_id: "ghost".into(), dof, value: 1.0, case_id: None }, &doc, &cfg).err(), "fem3d.reference.node-unknown", "ghost"),
        (add_member_udl::handle(&add_member_udl::AddMemberUdl { element_id: "ghost".into(), wx: 0.0, wy: 0.0, wz: 1.0, case_id: None }, &doc, &cfg).err(), "fem3d.reference.element-unknown", "ghost"),
        (add_area_load::handle(&add_area_load::AddAreaLoad { solid_id: "ghost".into(), pressure: 1.0, case_id: None }, &doc, &cfg).err(), "fem3d.reference.solid-unknown", "ghost"),
        (bar(&node, "ghost"), "fem3d.reference.node-unknown", "ghost"),
        (bar(&node, &node), "fem3d.member.degenerate", node.as_str()),
        (frame(&node, &other, "ghost", &section), "fem3d.reference.material-unknown", "ghost"),
        (frame(&node, &other, &material, "ghost"), "fem3d.reference.section-unknown", "ghost"),
    ];
    let table = semio_framework::fault_notice_definitions(<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp as semio_plugin::ArtifactEditor>::fault_notices());
    assert!(semio_framework::validate_fault_notices(&table).is_empty());
    for (fault, code, id) in cases {
        let fault = fault.unwrap_or_else(|| panic!("{code} for '{id}' is refused"));
        assert_eq!(fault.code.0.as_str(), code);
        for locale in [semio_ui_locale::Locale::En, semio_ui_locale::Locale::De] {
            let shown = semio_framework::fault_notice_text(&table, code, fault.params.as_deref(), semio_ui_locale::Terminology::Native, locale).unwrap_or_else(|| panic!("{code} has a {locale:?} notice"));
            assert!(!shown.contains('{') && (code.ends_with("degenerate") || shown.contains(id)), "{locale:?}: {shown}");
        }
    }
}
