//! 🗂️ 🗂️ FEM 3D app commands command — `remove-selection`.

use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{delete_combination, delete_element, delete_load_case, delete_material, delete_node, delete_section, delete_solid, delete_support};
use crate::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-selection")]
pub struct RemoveSelection {
    pub ids: Vec<String>,
}

/// 🗂️ Each id is looked up against every collection in a fixed precedence (nodes, elements,
/// materials, sections, supports, load cases, solids, combinations) and removed from the first one
/// it matches — mirrors the pre-migration `handle_action`'s exact search order.
pub fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let mut operations = Vec::new();
    for id in &payload.ids {
        if snapshot.nodes.iter().any(|n| &n.id == id) {
            operations.push(Fem3dMutation::DeleteNode(delete_node::DeleteNode { id: id.clone() }));
        } else if snapshot.elements.iter().any(|e| crate::element_id(e) == id) {
            operations.push(Fem3dMutation::DeleteElement(delete_element::DeleteElement { id: id.clone() }));
        } else if snapshot.materials.iter().any(|m| &m.id == id) {
            operations.push(Fem3dMutation::DeleteMaterial(delete_material::DeleteMaterial { id: id.clone() }));
        } else if snapshot.sections.iter().any(|s| &s.id == id) {
            operations.push(Fem3dMutation::DeleteSection(delete_section::DeleteSection { id: id.clone() }));
        } else if snapshot.supports.iter().any(|s| &s.id == id) {
            operations.push(Fem3dMutation::DeleteSupport(delete_support::DeleteSupport { id: id.clone() }));
        } else if snapshot.load_cases.iter().any(|l| &l.id == id) {
            operations.push(Fem3dMutation::DeleteLoadCase(delete_load_case::DeleteLoadCase { id: id.clone() }));
        } else if snapshot.solids.iter().any(|s| &s.id == id) {
            operations.push(Fem3dMutation::DeleteSolid(delete_solid::DeleteSolid { id: id.clone() }));
        } else if snapshot.combinations.iter().any(|c| &c.id == id) {
            operations.push(Fem3dMutation::DeleteCombination(delete_combination::DeleteCombination { id: id.clone() }));
        }
    }
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
