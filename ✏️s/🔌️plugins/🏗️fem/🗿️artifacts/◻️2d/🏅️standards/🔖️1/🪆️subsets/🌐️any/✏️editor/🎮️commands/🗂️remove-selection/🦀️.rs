//! 🗂️ 🗂️ Fem2d play app commands command — `remove-selection`.

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
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::remove_load;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-selection")]
pub struct RemoveSelection {
    pub ids: Vec<String>,
}

/// 🗂️ Deletes the payload's ids. An empty list is a deliberate no-op: the keybinding path dispatches
/// `removeSelection` with no args and the CURRENT selection is framework-owned state only
/// `ArtifactEditor::handle` sees, so the editor resolves it there and calls [`remove_ids`] directly.
pub fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    remove_ids(doc, &payload.ids)
}

/// 🗂️ Emits one dependency-ordered transaction over the selected entities, including case-owned loads.
pub fn remove_ids(doc: &ArtifactView<'_, Fem2dSnapshot>, ids: &[String]) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let addresses: Vec<_> = ids.iter().filter_map(|id| protocol::wire::decode_scoped_identity(id).ok()).collect();
    let selected: HashSet<_> = addresses.iter().filter_map(|address| match address.as_slice() { [kind, id] => Some((*kind, *id)), _ => None }).collect();
    let selected_loads: HashSet<_> = addresses.iter().filter_map(|address| match address.as_slice() { ["load", case, id] => Some((*case, *id)), _ => None }).collect();
    let mut operations = Vec::new();
    let combinations: Vec<_> = snapshot.combinations.iter().filter(|row| selected.contains(&("combination", row.id.as_str()))).collect();
    let indices: HashMap<_, _> = combinations.iter().enumerate().map(|(index, row)| (row.id.as_str(), index)).collect();
    let mut references = vec![0; combinations.len()];
    for row in &combinations {
        for term in &row.terms {
            if let Some(&index) = indices.get(term.case_id.as_str()) { references[index] += 1; }
        }
    }
    let mut ready: VecDeque<_> = references.iter().enumerate().filter_map(|(index, &count)| (count == 0).then_some(index)).collect();
    let mut removed = 0;
    while let Some(index) = ready.pop_front() {
        let row = combinations[index];
        operations.push(Fem2dMutation::DeleteCombination(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination { id: row.id.clone() }));
        removed += 1;
        for term in &row.terms {
            if let Some(&target) = indices.get(term.case_id.as_str()) {
                references[target] -= 1;
                if references[target] == 0 { ready.push_back(target); }
            }
        }
    }
    if removed != combinations.len() { return Err(Fault::from("fem2d.remove-selection.combination-cycle")); }
    for case in &snapshot.load_cases {
        if selected.contains(&("loadCase", case.id.as_str())) { continue; }
        for load in case.loads.iter().filter(|load| selected_loads.contains(&(case.id.as_str(), semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load)))) {
            operations.push(Fem2dMutation::RemoveLoad(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad { case_id: case.id.clone(), load_id: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load).into() }));
        }
    }
    for row in snapshot.load_cases.iter().filter(|row| selected.contains(&("loadCase", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteLoadCase(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase { id: row.id.clone() }));
    }
    for row in snapshot.supports.iter().filter(|row| selected.contains(&("support", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteSupport(semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport { id: row.id.clone() }));
    }
    for row in snapshot.elements.iter().filter(|row| selected.contains(&("element", element_id(row)))) {
        operations.push(Fem2dMutation::DeleteElement(semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement { id: element_id(row).into() }));
    }
    for row in snapshot.regions.iter().filter(|row| selected.contains(&("region", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteRegion(semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::delete_region::DeleteRegion { id: row.id.clone() }));
    }
    for row in snapshot.nodes.iter().filter(|row| selected.contains(&("node", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteNode(semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode { id: row.id.clone() }));
    }
    for row in snapshot.sections.iter().filter(|row| selected.contains(&("section", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteSection(semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section::DeleteSection { id: row.id.clone() }));
    }
    for row in snapshot.materials.iter().filter(|row| selected.contains(&("material", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteMaterial(semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial { id: row.id.clone() }));
    }
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}

/// 🗂️ Filters stale selections and repeated ids while preserving the framework selection order.
pub fn resolve_selection_ids(doc: &Fem2dSnapshot, ids: &[String]) -> Vec<String> {
    let mut live = HashSet::new();
    for row in &doc.nodes { live.insert(("node", "", row.id.as_str())); }
    for row in &doc.elements { live.insert(("element", "", element_id(row))); }
    for row in &doc.regions { live.insert(("region", "", row.id.as_str())); }
    for row in &doc.supports { live.insert(("support", "", row.id.as_str())); }
    for row in &doc.materials { live.insert(("material", "", row.id.as_str())); }
    for row in &doc.sections { live.insert(("section", "", row.id.as_str())); }
    for row in &doc.combinations { live.insert(("combination", "", row.id.as_str())); }
    for case in &doc.load_cases {
        live.insert(("loadCase", "", case.id.as_str()));
        for load in &case.loads { live.insert(("load", case.id.as_str(), semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id(load))); }
    }
    let mut seen = HashSet::new();
    ids.iter().filter(|target| {
        if !seen.insert(target.as_str()) { return false; }
        let Ok(address) = protocol::wire::decode_scoped_identity(target) else { return false; };
        match address.as_slice() { [kind, id] => live.contains(&(*kind, "", *id)), ["load", case, id] => live.contains(&("load", *case, *id)), _ => false }
    }).cloned().collect()
}

