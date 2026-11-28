use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::remove_selection::*;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
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
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_load_case;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_material;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_node;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_region;
use crate::context::dispatch;
use crate::context::dispatch_refused;
use crate::context::fem2d_empty_app;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.as_str().to_owned();
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Case".into(), self_weight: false })).await;
    let case_id = app.snapshot().expect("snapshot").load_cases[0].id.as_str().to_owned();

    dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec![semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_target_id("node", &node_id).expect("node target"), semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_target_id("loadCase", &case_id).expect("case target")] })).await;
    assert!(app.snapshot().expect("snapshot").nodes.is_empty());
    assert!(app.snapshot().expect("snapshot").load_cases.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_regions_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.to_string();
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

fn selection_fixture() -> (serde_json::Value, Fem2dSnapshotRetiring) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("third-party JSON fixture parser");
    let before = Fem2dSnapshotRetiring::new(semio_framework_pack_json::from_json_str(&fixture["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("shared snapshot wire fixture"));
    (fixture, before)
}

#[test]
fn remove_selection_orders_dependencies_and_restores_exact_snapshot() {
    use protocol::{ColdMutation, Mutation};
    use semio_os_command::SemanticMutation;

    let (fixture, before) = selection_fixture();
    for (selection, kinds) in [("completeSelection", "completeKinds"), ("loadSelection", "loadKinds")] {
        let ids: Vec<String> = serde_json::from_value(fixture[selection].clone()).expect("selection ids");
        let history = semio_plugin::HistoryView::empty();
        let result = remove_ids(&ArtifactView::new(&*before, &history), &ids).expect("selection command");
        let emitted: Vec<_> = result.artifact_mutations.iter().map(|operation| operation.semantics().kind).collect();
        assert_eq!(emitted, serde_json::from_value::<Vec<String>>(fixture[kinds].clone()).expect("expected semantic kinds"));
        let mut cursor = Fem2dSnapshotRetiring::new((*before).clone());
        let mut undo = Vec::new();
        for operation in &result.artifact_mutations {
            undo.push(operation.inverse(&*cursor).expect("semantic inverse"));
            let outcome = operation.diff(&*cursor);
            assert!(outcome.messages().iter().all(|message| message.level < semio_framework_diagnostic::Severity::Error), "{outcome:?}");
            cursor = Fem2dSnapshotRetiring::new(protocol::apply_diff_cold(outcome.diff(), &*cursor).expect("central application"));
        }
        let mut oracle = fixture["before"].clone();
        if selection == "completeSelection" {
            for pointer in fixture["completeOraclePointers"].as_array().expect("oracle pointers") {
                *oracle.pointer_mut(pointer.as_str().expect("pointer")).expect("collection") = serde_json::Value::Array(Vec::new());
            }
        } else {
            *oracle.pointer_mut("/loadCases/0/loads").expect("load collection") = serde_json::Value::Array(Vec::new());
        }
        assert_eq!(*cursor, *Fem2dSnapshotRetiring::new(semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(&oracle.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("third-party reconstructed outcome")));
        for batch in undo.iter().rev() {
            for operation in batch {
                cursor = Fem2dSnapshotRetiring::new(protocol::apply_diff_cold(operation.diff(&*cursor).diff(), &*cursor).expect("inverse application"));
            }
        }
        assert_eq!(*cursor, *before);
    }
    assert_eq!(resolve_selection_ids(&*before, &["scope:1:4:load4:dead2:l1".into(), "scope:1:4:load4:dead2:l1".into(), "scope:1:4:node7:missing".into()]), vec!["scope:1:4:load4:dead2:l1"]);
    
}

/// 🔗️ LAW: removing a node cascades to every member, support and load that names it, in dependency order, and the transaction restores the exact snapshot when undone.
#[test]
fn remove_selection_of_a_node_cascades_its_dependents_and_restores_exact_snapshot() {
    use protocol::{ColdMutation, Mutation};
    use semio_os_command::SemanticMutation;

    let (fixture, before) = selection_fixture();
    let ids: Vec<String> = serde_json::from_value(fixture["cascadeSelection"].clone()).expect("selection ids");
    let history = semio_plugin::HistoryView::empty();
    let result = remove_ids(&ArtifactView::new(&*before, &history), &ids).expect("selection command");
    let emitted: Vec<_> = result.artifact_mutations.iter().map(|operation| operation.semantics().kind).collect();
    assert_eq!(emitted, serde_json::from_value::<Vec<String>>(fixture["cascadeKinds"].clone()).expect("expected semantic kinds"));
    let mut cursor = Fem2dSnapshotRetiring::new((*before).clone());
    let mut undo = Vec::new();
    for operation in &result.artifact_mutations {
        undo.push(operation.inverse(&*cursor).expect("semantic inverse"));
        let outcome = operation.diff(&*cursor);
        assert!(outcome.messages().iter().all(|message| message.level < semio_framework_diagnostic::Severity::Error), "{outcome:?}");
        cursor = Fem2dSnapshotRetiring::new(protocol::apply_diff_cold(outcome.diff(), &*cursor).expect("central application"));
    }
    let mut oracle = fixture["before"].clone();
    for pointer in fixture["cascadeOraclePointers"].as_array().expect("oracle pointers") {
        *oracle.pointer_mut(pointer.as_str().expect("pointer")).expect("collection") = serde_json::Value::Array(Vec::new());
    }
    *oracle.pointer_mut("/loadCases/0/loads").expect("load collection") = serde_json::Value::Array(Vec::new());
    oracle["nodes"].as_array_mut().expect("nodes").retain(|node| node["id"] != "n1");
    assert_eq!(*cursor, *Fem2dSnapshotRetiring::new(semio_framework_pack_json::from_json_str::<Fem2dSnapshot>(&oracle.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("third-party reconstructed outcome")));
    for batch in undo.iter().rev() {
        for operation in batch {
            cursor = Fem2dSnapshotRetiring::new(protocol::apply_diff_cold(operation.diff(&*cursor).diff(), &*cursor).expect("inverse application"));
        }
    }
    assert_eq!(*cursor, *before);
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_mounted_publication_is_atomic_for_selected_and_unselected_referrers() {
    mounted_delete_body().await;
}

async fn mounted_delete_body() {
    let (fixture, before) = selection_fixture();
    for selection in ["completeSelection", "loadSelection", "cascadeSelection", "protectedSelection"] {
        let mut app = fem2d_empty_app();
        let semio_framework::kernel::Effect::LoadDocument { pack, spr } = semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::reset_document_effect(&*before) else { panic!("document fixture effect") };
        semio_plugin::artifact_app_laws::load_document(&mut app.0, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("mount shared wire fixture");
        let ids: Vec<String> = serde_json::from_value(fixture[selection].clone()).expect("selection ids");
        if selection == "protectedSelection" {
            let refusal = dispatch_refused(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids })).await;
            assert!(refusal.contains("fem2d.remove-selection.material-referenced"), "{refusal}");
            assert_eq!(app.snapshot().expect("unchanged snapshot"), *before, "an unselected referrer blocks the complete transaction");
            continue;
        }
        dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids })).await;
        let after = app.snapshot().expect("published snapshot");
        if selection == "cascadeSelection" {
            assert!(after.nodes.len() == 1 && after.elements.is_empty() && after.supports.is_empty() && after.load_cases[0].loads.is_empty());
        } else if selection == "completeSelection" {
            assert!(after.nodes.is_empty() && after.elements.is_empty() && after.materials.is_empty() && after.sections.is_empty() && after.supports.is_empty() && after.load_cases.is_empty() && after.combinations.is_empty());
        } else {
            assert!(after.load_cases[0].loads.is_empty());
            assert_eq!(after.nodes, before.nodes);
        }
    }
    
}

/// ⌨️ LAW: Delete and Backspace dispatch `removeSelection` without ids, and that means the live selection; ids a caller names are never overridden by it.
#[test]
fn remove_selection_without_ids_speaks_for_the_live_selection() {
    let (fixture, before) = selection_fixture();
    let selection: Vec<String> = serde_json::from_value(fixture["cascadeSelection"].clone()).expect("selection ids");
    let key = with_selection(&RemoveSelection { ids: Vec::new() }, &selection);
    assert_eq!(key.ids, selection);
    let named = RemoveSelection { ids: vec!["scope:1:7:element2:e1".into()] };
    assert_eq!(with_selection(&named, &selection), named);
    let history = semio_plugin::HistoryView::empty();
    assert!(!remove_ids(&ArtifactView::new(&*before, &history), &key.ids).expect("selection command").artifact_mutations.is_empty(), "the key must remove the selected entity");
}

/// 🗑️ LAW: removing a material, section or load case that something unselected still names is refused up front with a localized EN and DE notice naming it and its referrers (read from the shared JSON fixture by a third-party parser), and the same removal together with those referrers is not refused.
#[test]
fn remove_selection_of_a_referenced_target_is_refused_with_a_localized_notice_2d() {
    let (fixture, before) = selection_fixture();
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*before, &history);
    let named = |key: &str, field: &str, id: &str| fixture["before"][key].as_array().unwrap().iter().filter(|row| row[field] == id || row[field].as_array().is_some_and(|terms| terms.iter().any(|term| term["caseId"] == id))).map(|row| row["id"].as_str().unwrap().to_owned()).collect::<Vec<_>>();
    let table = semio_framework::fault_notice_definitions(<semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp as semio_plugin::ArtifactEditor>::fault_notices());
    assert!(semio_framework::validate_fault_notices(&table).is_empty());
    let target = |kind: &str, id: &str| semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_target_id(kind, id).expect("target");
    let material = [named("elements", "materialId", "steel"), named("regions", "materialId", "steel")].concat().join(", ");
    for (kind, id, code, referrers, english, german) in [
        ("material", "steel", "fem2d.remove-selection.material-referenced", material, "'steel' is still used by", "'steel' wird noch von"),
        ("section", "section", "fem2d.remove-selection.section-referenced", named("elements", "sectionId", "section").join(", "), "'section' is still used by", "'section' wird noch von"),
        ("loadCase", "dead", "fem2d.remove-selection.load-case-referenced", named("combinations", "terms", "dead").join(", "), "'dead' is still used by the combinations", "'dead' wird noch von den Kombinationen"),
    ] {
        assert!(!referrers.is_empty(), "{kind}: the fixture names a referrer");
        let fault = remove_ids(&doc, &[target(kind, id)]).err().expect("a referenced target is refused");
        assert_eq!(fault.code.0.as_str(), code);
        for (locale, expected) in [(semio_ui_locale::Locale::En, format!("{english} {referrers}")), (semio_ui_locale::Locale::De, format!("{german} {referrers}"))] {
            let shown = semio_framework::fault_notice_text(&table, code, fault.params.as_deref(), semio_ui_locale::Terminology::Native, locale).expect("notice");
            assert!(shown.contains(&expected), "{locale:?}: {shown}");
        }
    }
    let together = [target("material", "steel"), target("element", "e1"), target("node", "n1")];
    assert!(!remove_ids(&doc, &together).expect("removed with its referrers").artifact_mutations.is_empty());
}
