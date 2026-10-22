use super::*;
use crate::context::dispatch;
use crate::context::fem3d_empty_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use protocol::{ColdMutation, Mutation};
use semio_framework_os_command::SemanticMutation;

use semio_framework_value::DslValue;
use semio_framework_value_derive::FromValue;
#[cfg(test)] use semio_framework_plugin::component::ArtifactView;
#[cfg(test)] use semio_framework_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
#[cfg(test)] use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;

fn vector() -> DslValue {
    semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

fn removal(base: &Fem3dSnapshot, ids: Vec<String>) -> Vec<Fem3dMutation> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(base, &history);
    let config = NoConfig::default();
    handle(&RemoveSelection { ids }, &doc, &ConfigView { snapshot: &config, window: None }).unwrap().artifact_mutations
}

#[test]
fn every_selected_entity_is_removed_after_its_selected_referrers_and_restored() {
    let vector = vector();
    let base = Fem3dSnapshot::from_value(vector.get("snapshot").unwrap().clone()).unwrap();
    let ids = Vec::<String>::from_value(vector.get("selection").unwrap().clone()).unwrap();
    let expected = Vec::<String>::from_value(vector.get("orderedKinds").unwrap().clone()).unwrap();
    let mutations = removal(&base, ids.clone());
    assert_eq!(mutations.iter().map(|event| event.semantics().kind.to_string()).collect::<Vec<_>>(), expected);
    let mut current = base.clone();
    let mut inverse = Vec::new();
    for event in &mutations {
        inverse.extend(event.inverse(&current).unwrap());
        let outcome = event.diff(&current);
        assert!(outcome.messages().iter().all(|message| message.level < semio_framework_diagnostic::Severity::Error), "{event:?}: {:?}", outcome.messages());
        let next = crate::vcs::apply_mutation(&current, event).unwrap().0;
        assert_ne!(current, next, "each distinct selected target changes once");
        current = next;
    }
    assert_eq!(current, Fem3dSnapshot::default());
    for event in inverse.iter().rev() { current = crate::vcs::apply_mutation(&current, event).unwrap().0; }
    assert_eq!(current, base, "undo restores original rows, order and references");
    let reversed = removal(&base, ids.into_iter().rev().collect());
    let reversed = reversed.iter().fold(base.clone(), |state, event| crate::vcs::apply_mutation(&state, event).unwrap().0);
    assert_eq!(reversed, Fem3dSnapshot::default(), "selection order cannot block selected dependencies");
    eprintln!("[DEBUG] FEM3d nine interaction granularities remove in referential order and undo exactly");
}

#[test]
fn selected_loads_remove_once_without_removing_their_case_or_geometry() {
    let vector = vector();
    let base = Fem3dSnapshot::from_value(vector.get("snapshot").unwrap().clone()).unwrap();
    let ids = Vec::<String>::from_value(vector.get("loadSelection").unwrap().clone()).unwrap();
    let mutations = removal(&base, ids);
    assert_eq!(mutations.len(), 3);
    let after = mutations.iter().fold(base.clone(), |state, event| crate::vcs::apply_mutation(&state, event).unwrap().0);
    assert!(after.load_cases[0].loads.is_empty());
    assert_eq!(after.nodes, base.nodes);
    assert_eq!(after.elements, base.elements);
    assert_eq!(after.solids, base.solids);
    assert_eq!(after.combinations, base.combinations);
    eprintln!("[DEBUG] FEM3d selected loads remove exactly once without touching carrier geometry");
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_solids_3d() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_material::AddMaterial { name: "Concrete".into(), e: 3.0e10, g: 1.25e10 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.clone();
    dispatch(&mut app, Fem3dCommand::AddSolid(semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_solid::AddSolid { x: 0.0, y: 0.0, width: 1.0, depth: 1.0, height: 1.0, material_id, base_z: None, layers: None, mesh_size: None, axis: None })).await;
    let solid_id = app.snapshot().expect("snapshot").solids[0].id.clone();
    dispatch(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec![fem3d_entity_target(FEM3D_GRANULARITY_SOLID, &solid_id).unwrap()] })).await;
    assert!(app.snapshot().expect("snapshot").solids.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_with_unknown_ids_is_a_no_op() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec![fem3d_entity_target(FEM3D_GRANULARITY_NODE, "missing").unwrap()] })).await;
    assert!(app.snapshot().expect("snapshot").nodes.is_empty());
}
