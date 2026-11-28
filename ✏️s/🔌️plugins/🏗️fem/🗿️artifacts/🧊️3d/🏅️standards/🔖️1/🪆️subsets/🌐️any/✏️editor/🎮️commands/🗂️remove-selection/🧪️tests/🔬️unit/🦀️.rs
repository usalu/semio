use super::*;
use crate::context::dispatch;
use crate::context::dispatch_refused;
use crate::context::Retiring;
use crate::context::fem3d_empty_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use protocol::{ColdMutation, Mutation};
use semio_os_command::SemanticMutation;

use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_plugin::ConfigView;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::remove_selection::{handle, RemoveSelection};
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::{fem3d_entity_target, FEM3D_GRANULARITY_LOAD_CASE, FEM3D_GRANULARITY_MATERIAL, FEM3D_GRANULARITY_NODE, FEM3D_GRANULARITY_SECTION, FEM3D_GRANULARITY_SOLID};
#[cfg(test)] use semio_plugin::component::ArtifactView;
#[cfg(test)] use semio_plugin::component::NoConfig;
#[cfg(test)] use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::{Fem3dSnapshot, Fem3dSnapshotValue};
#[cfg(test)] use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;

fn vector() -> DslValue {
    semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

fn removal(base: &Fem3dSnapshot, ids: Vec<String>) -> Vec<Fem3dMutation> {
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(base, &history);
    let config = NoConfig::default();
    handle(&RemoveSelection { ids }, &doc, &ConfigView { snapshot: &config, window: None }).unwrap().artifact_mutations
}

fn replay(state: &Fem3dSnapshotValue, mutation: &Fem3dMutation) -> Fem3dSnapshotValue {
    let mut next = state.clone();
    semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation(&mut next, mutation).unwrap();
    next
}

fn base() -> (Fem3dSnapshotValue, Retiring<Fem3dSnapshot>) {
    let value = Fem3dSnapshotValue::from_value(vector().get("snapshot").unwrap().clone()).unwrap();
    let persistent = Retiring::new(Fem3dSnapshot::from_reference(value.clone()).unwrap());
    (value, persistent)
}

#[test]
fn every_selected_entity_is_removed_after_its_selected_referrers_and_restored() {
    let vector = vector();
    let (base_value, base) = base();
    let ids = Vec::<String>::from_value(vector.get("selection").unwrap().clone()).unwrap();
    let expected = Vec::<String>::from_value(vector.get("orderedKinds").unwrap().clone()).unwrap();
    let mutations = removal(&base, ids.clone());
    assert_eq!(mutations.iter().map(|event| event.semantics().kind.to_string()).collect::<Vec<_>>(), expected);
    let mut current = base_value.clone();
    let mut inverse = Vec::new();
    for event in &mutations {
        inverse.extend(ColdMutation::inverse(event, &current).unwrap());
        let outcome = ColdMutation::diff(event, &current);
        assert!(outcome.messages().iter().all(|message| message.level < semio_framework_diagnostic::Severity::Error), "{event:?}: {:?}", outcome.messages());
        let next = replay(&current, event);
        assert_ne!(current, next, "each distinct selected target changes once");
        current = next;
    }
    assert_eq!(current, Fem3dSnapshotValue::default());
    for event in inverse.iter().rev() { current = replay(&current, event); }
    assert_eq!(current, base_value, "undo restores original rows, order and references");
    let reversed = removal(&base, ids.into_iter().rev().collect());
    let reversed = reversed.iter().fold(base_value.clone(), |state, event| replay(&state, event));
    assert_eq!(reversed, Fem3dSnapshotValue::default(), "selection order cannot block selected dependencies");
}

#[test]
fn selected_loads_remove_once_without_removing_their_case_or_geometry() {
    let vector = vector();
    let (base_value, base) = base();
    let ids = Vec::<String>::from_value(vector.get("loadSelection").unwrap().clone()).unwrap();
    let mutations = removal(&base, ids);
    assert_eq!(mutations.len(), 3);
    let after = mutations.iter().fold(base_value.clone(), |state, event| replay(&state, event));
    assert!(after.load_cases[0].loads.is_empty());
    assert_eq!(after.nodes, base_value.nodes);
    assert_eq!(after.elements, base_value.elements);
    assert_eq!(after.solids, base_value.solids);
    assert_eq!(after.combinations, base_value.combinations);
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_solids_3d() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_material::AddMaterial { name: "Concrete".into(), e: 3.0e10, g: 1.25e10 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.to_string();
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

fn kinds_of(mutations: &[Fem3dMutation]) -> Vec<String> {
    mutations.iter().map(|event| event.semantics().kind.to_string()).collect()
}

fn replay_all(base: &Fem3dSnapshotValue, mutations: &[Fem3dMutation]) -> Fem3dSnapshotValue {
    let mut current = base.clone();
    let mut inverse = Vec::new();
    for event in mutations {
        inverse.extend(ColdMutation::inverse(event, &current).unwrap());
        let outcome = ColdMutation::diff(event, &current);
        assert!(outcome.messages().iter().all(|message| message.level < semio_framework_diagnostic::Severity::Error), "{event:?}: {:?}", outcome.messages());
        current = replay(&current, event);
    }
    let after = current.clone();
    for event in inverse.iter().rev() { current = replay(&current, event); }
    assert_eq!(&current, base, "undo restores the exact snapshot");
    after
}

/// 🗑️ LAW: removing a node cascades to the members, supports and loads that name it, dependency ordered, and undo restores the exact snapshot.
#[test]
fn remove_selection_of_a_node_cascades_its_dependents_and_restores_exact_snapshot_3d() {
    let (base_value, base) = base();
    let mutations = removal(&base, vec![fem3d_entity_target(FEM3D_GRANULARITY_NODE, "n1").unwrap()]);
    assert_eq!(kinds_of(&mutations), ["remove-load", "delete-support", "delete-element", "delete-node"]);
    let after = replay_all(&base_value, &mutations);
    assert!(after.nodes.iter().all(|node| node.id.as_str() != "n1") && after.supports.is_empty() && after.elements.is_empty());
    assert_eq!(after.load_cases[0].loads.len(), 2, "the member load goes with its member, the other loads stay");
    let n2 = removal(&base, vec![fem3d_entity_target(FEM3D_GRANULARITY_NODE, "n2").unwrap()]);
    assert_eq!(kinds_of(&n2), ["remove-load", "remove-load", "delete-element", "delete-node"]);
    assert!(replay_all(&base_value, &n2).supports.len() == 1);
}

/// 🗑️ LAW: removing a solid cascades to its area loads.
#[test]
fn remove_selection_of_a_solid_cascades_its_area_loads_3d() {
    let (base_value, base) = base();
    let mutations = removal(&base, vec![fem3d_entity_target(FEM3D_GRANULARITY_SOLID, "slab").unwrap()]);
    assert_eq!(kinds_of(&mutations), ["remove-load", "delete-solid"]);
    let after = replay_all(&base_value, &mutations);
    assert!(after.solids.is_empty() && after.load_cases[0].loads.len() == 2);
}

/// ⌨️ LAW: Delete and Backspace dispatch `removeSelection` without ids, and that means the live selection; ids a caller names are never overridden by it.
#[test]
fn remove_selection_without_ids_speaks_for_the_live_selection_3d() {
    let (_, base) = base();
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*base, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let selection = vec![fem3d_entity_target(FEM3D_GRANULARITY_NODE, "n1").unwrap()];
    let live = semio_s_artifact_fem_3d_editor::editor::fem3d::fem3d_route(&Fem3dCommand::RemoveSelection(RemoveSelection { ids: Vec::new() }), &doc, &cfg, || selection.clone(), None).expect("route");
    assert_eq!(live.artifact_mutations, removal(&base, selection.clone()));
    assert!(!live.artifact_mutations.is_empty(), "the key must remove the selected entity");
    let named = vec![fem3d_entity_target(FEM3D_GRANULARITY_SOLID, "slab").unwrap()];
    let routed = semio_s_artifact_fem_3d_editor::editor::fem3d::fem3d_route(&Fem3dCommand::RemoveSelection(RemoveSelection { ids: named.clone() }), &doc, &cfg, || selection.clone(), None).expect("route");
    assert_eq!(routed.artifact_mutations, removal(&base, named), "named ids win over the live selection");
}

/// 🗑️ LAW: the mounted publication is atomic: a node cascades to its member and support in one transaction, and a material an unselected member still names refuses the whole transaction and leaves the snapshot untouched.
#[semio_framework_async_macros::async_test]
async fn remove_selection_mounted_publication_is_atomic_for_cascades_and_protected_referrers_3d() {
    use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::{add_bar, add_material, add_node, add_section};
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0, z: 0.0 })).await;
    dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 4.0, y: 0.0, z: 0.0 })).await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 210e9, g: 81e9 })).await;
    dispatch(&mut app, Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.005, iy: 3.69e-5, iz: 1.34e-5, j: 2.1e-7 })).await;
    let (start, end, material, section) = {
        let snapshot = app.snapshot().expect("snapshot");
        (snapshot.nodes[0].id.get().clone(), snapshot.nodes[1].id.get().clone(), snapshot.materials[0].id.get().clone(), snapshot.sections[0].id.clone())
    };
    dispatch(&mut app, Fem3dCommand::AddBar(add_bar::AddBar { start: start.clone(), end, material_id: material.clone(), section_id: section })).await;
    let counts = |app: &crate::context::Fem3dApp| { let snapshot = app.snapshot().expect("snapshot"); (snapshot.nodes.len(), snapshot.elements.len(), snapshot.materials.len(), snapshot.sections.len()) };
    let before = counts(&app);
    let refusal = dispatch_refused(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec![fem3d_entity_target(FEM3D_GRANULARITY_MATERIAL, &material).unwrap()] })).await;
    assert!(refusal.contains("fem3d.remove-selection.material-referenced"), "{refusal}");
    assert_eq!(counts(&app), before, "an unselected referrer blocks the complete transaction");
    dispatch(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec![fem3d_entity_target(FEM3D_GRANULARITY_NODE, &start).unwrap()] })).await;
    let after = app.snapshot().expect("published snapshot");
    assert!(after.nodes.len() == 1 && after.elements.is_empty(), "the member goes with its node in one publication");
}

/// 🗑️ LAW: removing a material, section or load case that something unselected still names is refused up front with a localized EN and DE notice naming it and its referrers, and the same removal together with those referrers is not refused.
#[test]
fn remove_selection_of_a_referenced_target_is_refused_with_a_localized_notice_3d() {
    use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_GRANULARITY_ELEMENT as ELEMENT;
    let (_, base) = base();
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*base, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let table = semio_framework::fault_notice_definitions(<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp as semio_plugin::ArtifactEditor>::fault_notices());
    assert!(semio_framework::validate_fault_notices(&table).is_empty());
    for (kind, id, code, english, german) in [
        (FEM3D_GRANULARITY_MATERIAL, "steel", "fem3d.remove-selection.material-referenced", "'steel' is still used by member, slab", "'steel' wird noch von member, slab verwendet"),
        (FEM3D_GRANULARITY_SECTION, "section", "fem3d.remove-selection.section-referenced", "'section' is still used by member", "'section' wird noch von member verwendet"),
        (FEM3D_GRANULARITY_LOAD_CASE, "case", "fem3d.remove-selection.load-case-referenced", "'case' is still used by the combinations combination", "'case' wird noch von den Kombinationen combination verwendet"),
    ] {
        let fault = handle(&RemoveSelection { ids: vec![fem3d_entity_target(kind, id).unwrap()] }, &doc, &cfg).err().expect("a referenced target is refused");
        assert_eq!(fault.code.0.as_str(), code);
        for (locale, expected) in [(semio_ui_locale::Locale::En, english), (semio_ui_locale::Locale::De, german)] {
            let shown = semio_framework::fault_notice_text(&table, code, fault.params.as_deref(), semio_ui_locale::Terminology::Native, locale).expect("notice");
            assert!(shown.contains(expected), "{locale:?}: {shown}");
        }
    }
    let together = vec![fem3d_entity_target(FEM3D_GRANULARITY_MATERIAL, "steel").unwrap(), fem3d_entity_target(ELEMENT, "member").unwrap(), fem3d_entity_target(FEM3D_GRANULARITY_SOLID, "slab").unwrap()];
    assert!(!handle(&RemoveSelection { ids: together }, &doc, &cfg).expect("removed with its referrers").artifact_mutations.is_empty());
}
