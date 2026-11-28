//! 🗂️ 🗂️ FEM 3D app commands command — `remove-selection`.

use semio_plugin::NoConfig;

use semio_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::delete_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use std::collections::HashSet;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-selection")]
pub struct RemoveSelection {
    pub ids: Vec<String>,
}

/// 🔗️ The refusal of a removal that would leave `referrers` pointing at `id`: one localized notice per kind (`{id}` and the first five `{referrers}` filled from its params), raised before any mutation so nothing is removed and the shell says why.
fn referenced(code: &'static str, id: &str, referrers: &[&str]) -> Fault {
    let listed = referrers.iter().take(5).copied().collect::<Vec<_>>().join(", ");
    let listed = if referrers.len() > 5 { format!("{listed}, …") } else { listed };
    Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(code), format!("'{id}' is still referenced by {listed}")).with_param("id", id).with_param("referrers", listed)
}

/// 🧱️ The material id a member names.
fn element_material(element: &FemElement) -> &str {
    match element { FemElement::Bar { material_id, .. } | FemElement::Frame { material_id, .. } => material_id }
}

/// 📐️ The section id a member names.
fn element_section(element: &FemElement) -> &str {
    match element { FemElement::Bar { section_id, .. } | FemElement::Frame { section_id, .. } => section_id }
}

/// 🗂️ Removes each interaction target once, with selected referrers before their selected parents, cascading to the
/// supports, members and loads that name a removed node, member or solid so no row is left dangling.
pub fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    if payload.ids.is_empty() { return Err(Fault::from("fem3d.remove-selection.empty")); }
    let snapshot = doc.snapshot;
    let mut operations = Vec::new();
    let mut seen = HashSet::new();
    let mut selected: HashSet<(&str, &str)> = HashSet::new();
    let mut removed_loads: HashSet<(&str, &str)> = HashSet::new();
    for identity in &payload.ids {
        if !seen.insert(identity.as_str()) { continue; }
        let id = fem3d_entity_id(identity).ok_or_else(|| Fault::from("fem3d.remove-selection.identity-refused"))?;
        let event = match fem3d_entity_kind(snapshot, identity) {
            Some(FEM3D_GRANULARITY_LOAD) => {
                let Some((case_id, load)) = fem3d_load_owner(snapshot, identity) else { continue };
                removed_loads.insert((case_id, semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(load)));
                (0, Fem3dMutation::RemoveLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad { case_id: case_id.into(), load_id: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(load).into() }))
            }
            Some(FEM3D_GRANULARITY_COMBINATION) => (1, Fem3dMutation::DeleteCombination(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination { id: id.into() })),
            Some(FEM3D_GRANULARITY_SUPPORT) => (2, Fem3dMutation::DeleteSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport { id: id.into() })),
            Some(FEM3D_GRANULARITY_ELEMENT) => (3, Fem3dMutation::DeleteElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement { id: id.into() })),
            Some(FEM3D_GRANULARITY_SOLID) => (4, Fem3dMutation::DeleteSolid(semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::delete_solid::DeleteSolid { id: id.into() })),
            Some(FEM3D_GRANULARITY_LOAD_CASE) => (5, Fem3dMutation::DeleteLoadCase(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase { id: id.into() })),
            Some(FEM3D_GRANULARITY_NODE) => (6, Fem3dMutation::DeleteNode(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode { id: id.into() })),
            Some(FEM3D_GRANULARITY_SECTION) => (7, Fem3dMutation::DeleteSection(semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section::DeleteSection { id: id.into() })),
            Some(FEM3D_GRANULARITY_MATERIAL) => (8, Fem3dMutation::DeleteMaterial(semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial { id: id.into() })),
            _ => continue,
        };
        if let Some(kind) = fem3d_entity_kind(snapshot, identity) { selected.insert((kind, id)); }
        operations.push(event);
    }
    let removed_nodes: HashSet<&str> = snapshot.nodes.iter().map(|row| row.id.as_str()).filter(|id| selected.contains(&(FEM3D_GRANULARITY_NODE, *id))).collect();
    let removed_elements: HashSet<&str> = snapshot.elements.iter().filter(|row| selected.contains(&(FEM3D_GRANULARITY_ELEMENT, element_id(row))) || { let (start, end) = fem3d_element_endpoints(row); removed_nodes.contains(start) || removed_nodes.contains(end) }).map(element_id).collect();
    let removed_solids: HashSet<&str> = snapshot.solids.iter().map(|row| row.id.as_str()).filter(|id| selected.contains(&(FEM3D_GRANULARITY_SOLID, *id))).collect();
    for case in &snapshot.load_cases {
        for load in case.loads.iter().filter(|load| match load {
            FemLoad::Nodal { node_id, .. } => removed_nodes.contains(node_id.as_str()),
            FemLoad::MemberUdl { element_id, .. } => removed_elements.contains(element_id.as_str()),
            FemLoad::Area { solid_id, .. } => removed_solids.contains(solid_id.as_str()),
        }) {
            if removed_loads.insert((case.id.as_str(), semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(load))) {
                operations.push((0, Fem3dMutation::RemoveLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad { case_id: case.id.as_str().into(), load_id: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(load).into() })));
            }
        }
    }
    for row in snapshot.supports.iter().filter(|row| removed_nodes.contains(row.node_id.as_str()) && !selected.contains(&(FEM3D_GRANULARITY_SUPPORT, row.id.as_str()))) {
        operations.push((2, Fem3dMutation::DeleteSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport { id: row.id.as_str().into() })));
    }
    for row in snapshot.elements.iter().filter(|row| removed_elements.contains(element_id(row)) && !selected.contains(&(FEM3D_GRANULARITY_ELEMENT, element_id(row)))) {
        operations.push((3, Fem3dMutation::DeleteElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement { id: element_id(row).into() })));
    }
    let removed_combinations: HashSet<&str> = snapshot.combinations.iter().map(|row| row.id.as_str()).filter(|id| selected.contains(&(FEM3D_GRANULARITY_COMBINATION, *id))).collect();
    for identity in &payload.ids {
        let (Some(kind), Some(id)) = (fem3d_entity_kind(snapshot, identity), fem3d_entity_id(identity)) else { continue };
        let (code, referrers): (&str, Vec<&str>) = match kind {
            FEM3D_GRANULARITY_MATERIAL => ("fem3d.remove-selection.material-referenced", snapshot.elements.iter().filter(|row| !removed_elements.contains(element_id(row)) && element_material(row) == id).map(element_id).chain(snapshot.solids.iter().filter(|row| !removed_solids.contains(row.id.as_str()) && row.material_id == id).map(|row| row.id.as_str())).collect()),
            FEM3D_GRANULARITY_SECTION => ("fem3d.remove-selection.section-referenced", snapshot.elements.iter().filter(|row| !removed_elements.contains(element_id(row)) && element_section(row) == id).map(element_id).collect()),
            FEM3D_GRANULARITY_LOAD_CASE => ("fem3d.remove-selection.load-case-referenced", snapshot.combinations.iter().filter(|row| !removed_combinations.contains(row.id.as_str()) && row.terms.contains_key(id)).map(|row| row.id.as_str()).collect()),
            _ => continue,
        };
        if !referrers.is_empty() { return Err(referenced(code, id, &referrers)); }
    }
    operations.sort_by_key(|(rank, _)| *rank);
    let operations: Vec<_> = operations.into_iter().map(|(_, mutation)| mutation).collect();
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}

