//! 🗂️ 🗂️ FEM 3D app commands command — `remove-selection`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::*;
use std::collections::HashSet;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-selection")]
pub struct RemoveSelection {
    pub ids: Vec<String>,
}

/// 🗂️ Removes each interaction target once, with selected referrers before their selected parents.
pub fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let mut operations = Vec::new();
    let mut seen = HashSet::new();
    for identity in &payload.ids {
        if !seen.insert(identity.as_str()) { continue; }
        let id = fem3d_entity_id(identity).ok_or_else(|| Fault::from("fem3d.remove-selection.identity-refused"))?;
        let event = match fem3d_entity_kind(snapshot, identity) {
            Some(FEM3D_GRANULARITY_LOAD) => {
                let Some((case_id, load)) = fem3d_load_owner(snapshot, identity) else { continue };
                (0, Fem3dMutation::RemoveLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad { case_id: case_id.into(), load_id: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id(load).into() }))
            }
            Some(FEM3D_GRANULARITY_COMBINATION) => (1, Fem3dMutation::DeleteCombination(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination { id: id.into() })),
            Some(FEM3D_GRANULARITY_SUPPORT) => (2, Fem3dMutation::DeleteSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport { id: id.into() })),
            Some(FEM3D_GRANULARITY_ELEMENT) => (3, Fem3dMutation::DeleteElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement { id: id.into() })),
            Some(FEM3D_GRANULARITY_SOLID) => (4, Fem3dMutation::DeleteSolid(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_solid::DeleteSolid { id: id.into() })),
            Some(FEM3D_GRANULARITY_LOAD_CASE) => (5, Fem3dMutation::DeleteLoadCase(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase { id: id.into() })),
            Some(FEM3D_GRANULARITY_NODE) => (6, Fem3dMutation::DeleteNode(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode { id: id.into() })),
            Some(FEM3D_GRANULARITY_SECTION) => (7, Fem3dMutation::DeleteSection(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_section::DeleteSection { id: id.into() })),
            Some(FEM3D_GRANULARITY_MATERIAL) => (8, Fem3dMutation::DeleteMaterial(semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial { id: id.into() })),
            _ => continue,
        };
        operations.push(event);
    }
    operations.sort_by_key(|(rank, _)| *rank);
    let operations: Vec<_> = operations.into_iter().map(|(_, mutation)| mutation).collect();
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}

