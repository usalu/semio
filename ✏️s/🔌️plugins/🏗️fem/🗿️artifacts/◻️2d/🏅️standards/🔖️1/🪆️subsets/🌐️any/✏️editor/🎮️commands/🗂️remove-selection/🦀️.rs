//! 🗂️ 🗂️ Fem2d play app commands command — `remove-selection`.

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
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::remove_load;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
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

/// 🗂️ Deletes the payload's ids; an empty list is refused with `fem2d.remove-selection.empty`. The keybinding path dispatches
/// `removeSelection` with no args and the CURRENT selection is framework-owned state only
/// `ArtifactEditor::handle` sees, so the editor resolves it there and calls [`remove_ids`] directly.
pub fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    if payload.ids.is_empty() { return Err(Fault::from("fem2d.remove-selection.empty")); }
    remove_ids(doc, &payload.ids)
}

/// 🔗️ The refusal of a removal that would leave `referrers` pointing at `id`: one localized notice per kind (`{id}` and the first five `{referrers}` filled from its params), raised before any mutation so nothing is removed and the shell says why.
fn referenced(code: &'static str, id: &str, referrers: &[&str]) -> Fault {
    let listed = referrers.iter().take(5).copied().collect::<Vec<_>>().join(", ");
    let listed = if referrers.len() > 5 { format!("{listed}, …") } else { listed };
    Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(code), format!("'{id}' is still referenced by {listed}")).with_param("id", id).with_param("referrers", listed)
}

/// 🧱️ The material id a member names.
fn element_material(element: &FemElement) -> &str {
    match element { FemElement::Bar { material_id, .. } | FemElement::Beam { material_id, .. } => material_id }
}

/// 📐️ The section id a member names.
fn element_section(element: &FemElement) -> &str {
    match element { FemElement::Bar { section_id, .. } | FemElement::Beam { section_id, .. } => section_id }
}

/// 🔗️ The two node ids a member connects.
fn element_ends(element: &FemElement) -> [&str; 2] {
    match element { FemElement::Bar { start, end, .. } | FemElement::Beam { start, end, .. } => [start, end] }
}

/// ⌨️ The payload the delete keybinding means: no ids of its own speaks for the live framework selection, ids it names win.
pub fn with_selection(payload: &RemoveSelection, selection: &[String]) -> RemoveSelection {
    if payload.ids.is_empty() { RemoveSelection { ids: selection.to_vec() } } else { payload.clone() }
}

/// 🗂️ Emits one dependency-ordered transaction over the selected entities, cascading to the supports, members and loads that name a removed node, member or region.
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
    let removed_nodes: HashSet<&str> = snapshot.nodes.iter().map(|row| row.id.as_str()).filter(|id| selected.contains(&("node", *id))).collect();
    let removed_elements: HashSet<&str> = snapshot.elements.iter().filter(|row| selected.contains(&("element", element_id(row))) || element_ends(row).iter().any(|end| removed_nodes.contains(end))).map(element_id).collect();
    let removed_regions: HashSet<&str> = snapshot.regions.iter().map(|row| row.id.as_str()).filter(|id| selected.contains(&("region", *id))).collect();
    for case in &snapshot.load_cases {
        if selected.contains(&("loadCase", case.id.as_str())) { continue; }
        for load in case.loads.iter().filter(|load| {
            let id = load_id(load);
            selected_loads.contains(&(case.id.as_str(), id)) || match load {
                FemLoad::Nodal { node_id, .. } => removed_nodes.contains(node_id.as_str()),
                FemLoad::MemberUdl { element_id, .. } => removed_elements.contains(element_id.as_str()),
                FemLoad::Area { region_id, .. } => removed_regions.contains(region_id.as_str()),
            }
        }) {
            operations.push(Fem2dMutation::RemoveLoad(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad { case_id: case.id.clone(), load_id: load_id(load).into() }));
        }
    }
    for row in snapshot.load_cases.iter().filter(|row| selected.contains(&("loadCase", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteLoadCase(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase { id: row.id.clone() }));
    }
    for row in snapshot.supports.iter().filter(|row| selected.contains(&("support", row.id.as_str())) || removed_nodes.contains(row.node_id.as_str())) {
        operations.push(Fem2dMutation::DeleteSupport(semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport { id: row.id.clone() }));
    }
    for row in snapshot.elements.iter().filter(|row| removed_elements.contains(element_id(row))) {
        operations.push(Fem2dMutation::DeleteElement(semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement { id: element_id(row).into() }));
    }
    for row in snapshot.regions.iter().filter(|row| selected.contains(&("region", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteRegion(semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::delete_region::DeleteRegion { id: row.id.clone() }));
    }
    for row in snapshot.nodes.iter().filter(|row| selected.contains(&("node", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteNode(semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode { id: row.id.as_str().to_owned() }));
    }
    for row in snapshot.sections.iter().filter(|row| selected.contains(&("section", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteSection(semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section::DeleteSection { id: row.id.clone() }));
    }
    for row in snapshot.materials.iter().filter(|row| selected.contains(&("material", row.id.as_str()))) {
        operations.push(Fem2dMutation::DeleteMaterial(semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial { id: row.id.as_str().to_owned() }));
    }
    for address in &addresses {
        let [kind, id] = address.as_slice() else { continue };
        let (code, referrers): (&str, Vec<&str>) = match *kind {
            "material" => ("fem2d.remove-selection.material-referenced", snapshot.elements.iter().filter(|row| !removed_elements.contains(element_id(row)) && element_material(row) == *id).map(element_id).chain(snapshot.regions.iter().filter(|row| !removed_regions.contains(row.id.as_str()) && row.material_id == *id).map(|row| row.id.as_str())).collect()),
            "section" => ("fem2d.remove-selection.section-referenced", snapshot.elements.iter().filter(|row| !removed_elements.contains(element_id(row)) && element_section(row) == *id).map(element_id).collect()),
            "loadCase" => ("fem2d.remove-selection.load-case-referenced", snapshot.combinations.iter().filter(|row| !selected.contains(&("combination", row.id.as_str())) && row.terms.iter().any(|term| term.case_id == *id)).map(|row| row.id.as_str()).collect()),
            _ => continue,
        };
        if !referrers.is_empty() { return Err(referenced(code, id, &referrers)); }
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
    for row in doc.nodes.iter() { live.insert(("node", "", row.id.as_str())); }
    for row in &doc.elements { live.insert(("element", "", element_id(row))); }
    for row in &doc.regions { live.insert(("region", "", row.id.as_str())); }
    for row in &doc.supports { live.insert(("support", "", row.id.as_str())); }
    for row in doc.materials.iter() { live.insert(("material", "", row.id.as_str())); }
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

