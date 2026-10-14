//! 🗂️ 🗂️ Fem2d play app commands command — `remove-selection`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::delete_region;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

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

/// 🗂️ Each id is looked up against every collection in a fixed precedence (nodes, elements,
/// materials, sections, supports, load cases, regions, combinations) and removed from the first
/// one it matches — mirrors the pre-migration `handle_action`'s exact search order. Public so the
/// editor can route a delete keybinding's live interaction selection straight into it.
pub fn remove_ids(doc: &ArtifactView<'_, Fem2dSnapshot>, ids: &[String]) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let mut operations = Vec::new();
    for id in ids {
        if snapshot.nodes.iter().any(|n| &n.id == id) {
            operations.push(Fem2dMutation::DeleteNode(delete_node::DeleteNode { id: id.clone() }));
        } else if snapshot.elements.iter().any(|e| element_id(e) == id) {
            operations.push(Fem2dMutation::DeleteElement(delete_element::DeleteElement { id: id.clone() }));
        } else if snapshot.materials.iter().any(|m| &m.id == id) {
            operations.push(Fem2dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: id.clone() }));
        } else if snapshot.sections.iter().any(|s| &s.id == id) {
            operations.push(Fem2dMutation::DeleteSection(delete_section::DeleteSection { id: id.clone() }));
        } else if snapshot.supports.iter().any(|s| &s.id == id) {
            operations.push(Fem2dMutation::DeleteSupport(delete_support::DeleteSupport { id: id.clone() }));
        } else if snapshot.load_cases.iter().any(|l| &l.id == id) {
            operations.push(Fem2dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: id.clone() }));
        } else if snapshot.regions.iter().any(|r| &r.id == id) {
            operations.push(Fem2dMutation::DeleteRegion(delete_region::DeleteRegion { id: id.clone() }));
        } else if snapshot.combinations.iter().any(|c| &c.id == id) {
            operations.push(Fem2dMutation::DeleteCombination(delete_combination::DeleteCombination { id: id.clone() }));
        }
    }
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}

/// 🗂️ The subset of `ids` this command can actually delete — a framework selection may still carry a
/// stale id (a peer deleted it) or one this app owns no delete mutation for (a load, whose removal is
/// `remove-load` on its case), so the editor filters before it dispatches.
pub fn resolve_selection_ids(doc: &Fem2dSnapshot, ids: &[String]) -> Vec<String> {
    ids.iter()
        .filter(|id| {
            doc.nodes.iter().any(|n| &&n.id == id)
                || doc.elements.iter().any(|e| &element_id(e) == id)
                || doc.materials.iter().any(|m| &&m.id == id)
                || doc.sections.iter().any(|s| &&s.id == id)
                || doc.supports.iter().any(|s| &&s.id == id)
                || doc.load_cases.iter().any(|l| &&l.id == id)
                || doc.regions.iter().any(|r| &&r.id == id)
                || doc.combinations.iter().any(|c| &&c.id == id)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
