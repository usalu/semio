//! 🗂️ 🗂️ Fem2d play app commands command — `remove-selection`.

use crate::standards::v1::subsets::any::schema::mutations::{delete_combination, delete_element, delete_load_case, delete_material, delete_node, delete_region, delete_section, delete_support};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::{element_id, Fem2dSnapshot};
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-selection")]
pub struct RemoveSelection {
    pub ids: Vec<String>,
}

/// 🗂️ Each id is looked up against every collection in a fixed precedence (nodes, elements,
/// materials, sections, supports, load cases, regions, combinations) and removed from the first
/// one it matches — mirrors the pre-migration `handle_action`'s exact search order.
pub fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let mut operations = Vec::new();
    for id in &payload.ids {
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
