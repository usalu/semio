use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::remove_selection::*;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::delete_region;
use semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_load_case;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_material;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_node;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_region;
use crate::context::dispatch;
use crate::context::fem2d_empty_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Case".into(), self_weight: false })).await;
    let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();

    dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec![semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_target_id("node", &node_id).expect("node target"), semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_target_id("loadCase", &case_id).expect("case target")] })).await;
    assert!(app.snapshot().expect("snapshot").nodes.is_empty());
    assert!(app.snapshot().expect("snapshot").load_cases.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_regions_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 1.0, height: 1.0, material_id, thickness: None, mesh_size: None })).await;
    let region_id = app.snapshot().expect("snapshot").regions[0].id.clone();
    dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec![semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_target_id("region", &region_id).expect("region target")] })).await;
    assert!(app.snapshot().expect("snapshot").regions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_with_no_matching_ids_is_a_no_op_2d() {
    let mut app = fem2d_empty_app();
    let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec!["missing".into()] })).await;
    assert!(result.mutations.is_empty());
}

fn selection_fixture() -> (serde_json::Value, Fem2dSnapshot) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("third-party JSON fixture parser");
    let before = semio_framework_pack_json::from_json_str(&fixture["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("shared snapshot wire fixture");
    (fixture, before)
}

#[test]
fn remove_selection_orders_dependencies_and_restores_exact_snapshot() {
    use protocol::{ColdMutation, Mutation};
    use semio_framework_os_command::SemanticMutation;

    let (fixture, before) = selection_fixture();
    for (selection, kinds) in [("completeSelection", "completeKinds"), ("loadSelection", "loadKinds")] {
        let ids: Vec<String> = serde_json::from_value(fixture[selection].clone()).expect("selection ids");
        let history = semio_framework_plugin::HistoryView::empty();
        let result = remove_ids(&ArtifactView::new(&before, &history), &ids).expect("selection command");
        let emitted: Vec<_> = result.artifact_mutations.iter().map(|operation| operation.semantics().kind).collect();
        assert_eq!(emitted, serde_json::from_value::<Vec<String>>(fixture[kinds].clone()).expect("expected semantic kinds"));
        let mut cursor = before.clone();
        let mut undo = Vec::new();
        for operation in &result.artifact_mutations {
            undo.push(operation.inverse(&cursor).expect("semantic inverse"));
            let outcome = operation.diff(&cursor);
            assert!(outcome.messages().iter().all(|message| message.level < semio_framework_diagnostic::Severity::Error), "{outcome:?}");
            cursor = protocol::apply_diff_cold(outcome.diff(), &cursor).expect("central application");
        }
        let mut oracle = fixture["before"].clone();
        if selection == "completeSelection" {
            for pointer in fixture["completeOraclePointers"].as_array().expect("oracle pointers") {
                *oracle.pointer_mut(pointer.as_str().expect("pointer")).expect("collection") = serde_json::Value::Array(Vec::new());
            }
        } else {
            *oracle.pointer_mut("/loadCases/0/loads").expect("load collection") = serde_json::Value::Array(Vec::new());
        }
        assert_eq!(cursor, semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(&oracle.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("third-party reconstructed outcome"));
        for batch in undo.iter().rev() {
            for operation in batch {
                cursor = protocol::apply_diff_cold(operation.diff(&cursor).diff(), &cursor).expect("inverse application");
            }
        }
        assert_eq!(cursor, before);
    }
    assert_eq!(resolve_selection_ids(&before, &["scope:1:4:load4:dead2:l1".into(), "scope:1:4:load4:dead2:l1".into(), "scope:1:4:node7:missing".into()]), vec!["scope:1:4:load4:dead2:l1"]);
    println!("[DEBUG] FEM2d selection deletion resolves loads, deduplicates targets, orders dependencies and replays exact inverses");
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_mounted_publication_is_atomic_for_selected_and_unselected_referrers() {
    mounted_delete_body().await;
}

#[test]
fn mounted_future_layout_probe() {
    fn bytes<F>(_factory: impl FnOnce() -> F) -> usize { std::mem::size_of::<F>() }
    fn manifest() -> semio_framework_plugin::App { semio_framework_plugin::App { definition: semio_s_artifact_fem_2d_editor::editor::fem2d::create_fem2d_app(), examples: Vec::new() } }
    let mut identity = semio_framework_plugin::artifact_app_laws::fixture_identity();
    let authority = &mut identity;
    let constructor_bytes = bytes(move || semio_framework_plugin::artifact_app_laws::new_app_with_registry::<semio_framework_plugin::EditorApp<semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp>>(manifest, semio_framework_os_kernel::ActorId("probe".into()), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), authority));
    log_phase(format!("[DEBUG] FEM2d mounted future layout: app={} constructor={} deletion={}", std::mem::size_of::<semio_s_artifact_fem_2d_editor::editor::fem2d::unit_tests::context::Fem2dApp>(), constructor_bytes, bytes(mounted_delete_body)));
}

fn log_phase(message: String) { std::io::Write::write_all(&mut std::io::stderr(), (message + "\n").as_bytes()).expect("debug phase"); }

async fn mounted_delete_body() {
    log_phase(format!("[DEBUG] FEM2d mounted delete entered; fixture app bytes={}", std::mem::size_of::<semio_s_artifact_fem_2d_editor::editor::fem2d::unit_tests::context::Fem2dApp>()));
    let (fixture, before) = selection_fixture();
    for selection in ["completeSelection", "loadSelection", "protectedSelection"] {
        log_phase(format!("[DEBUG] FEM2d mounted delete {selection}: constructing app"));
        let mut app = fem2d_empty_app();
        log_phase(format!("[DEBUG] FEM2d mounted delete {selection}: app constructed"));
        let semio_framework::kernel::Effect::LoadDocument { pack, spr } = semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::reset_document_effect(&before) else { panic!("document fixture effect") };
        semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("mount shared wire fixture");
        log_phase(format!("[DEBUG] FEM2d mounted delete {selection}: document mounted"));
        let ids: Vec<String> = serde_json::from_value(fixture[selection].clone()).expect("selection ids");
        dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids })).await;
        log_phase(format!("[DEBUG] FEM2d mounted delete {selection}: command settled"));
        let after = app.snapshot().expect("published snapshot");
        if selection == "completeSelection" {
            assert!(after.nodes.is_empty() && after.elements.is_empty() && after.materials.is_empty() && after.sections.is_empty() && after.supports.is_empty() && after.load_cases.is_empty() && after.combinations.is_empty());
        } else if selection == "loadSelection" {
            assert!(after.load_cases[0].loads.is_empty());
            assert_eq!(after.nodes, before.nodes);
        } else {
            assert_eq!(after, before, "an unselected referrer blocks the complete transaction");
        }
    }
    println!("[DEBUG] FEM2d mounted selection deletion publishes complete prepared transactions; referenced-node refusal preserves the full snapshot");
}
