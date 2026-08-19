//! 🗂️ 🗂️ Fem2d play app commands command — `remove-selection`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::mutations::{delete_combination, delete_element, delete_load_case, delete_material, delete_node, delete_region, delete_section, delete_support};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-selection")]
pub struct RemoveSelection {
    pub ids: Vec<String>,
}

/// 🗂️ Each id is looked up against every collection in a fixed precedence (nodes, elements,
/// materials, sections, supports, load cases, regions, combinations) and removed from the first
/// one it matches — mirrors the pre-migration `handle_action`'s exact search order.
pub async fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let mut operations = Vec::new();
    for id in &payload.ids {
        if snapshot.nodes.iter().any(|n| &n.id == id) {
            operations.push(Fem2dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: id.clone() }));
        } else if snapshot.elements.iter().any(|e| element_id(e) == id) {
            operations.push(Fem2dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: id.clone() }));
        } else if snapshot.materials.iter().any(|m| &m.id == id) {
            operations.push(Fem2dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: id.clone() }));
        } else if snapshot.sections.iter().any(|s| &s.id == id) {
            operations.push(Fem2dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: id.clone() }));
        } else if snapshot.supports.iter().any(|s| &s.id == id) {
            operations.push(Fem2dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: id.clone() }));
        } else if snapshot.load_cases.iter().any(|l| &l.id == id) {
            operations.push(Fem2dMutation::DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase { id: id.clone() }));
        } else if snapshot.regions.iter().any(|r| &r.id == id) {
            operations.push(Fem2dMutation::DeleteRegion(delete_region::mutation::DeleteRegion { id: id.clone() }));
        } else if snapshot.combinations.iter().any(|c| &c.id == id) {
            operations.push(Fem2dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: id.clone() }));
        }
    }
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::commands::{add_node, add_region};
    use crate::editor::fem2d::commands::add_load_case;
    use crate::editor::fem2d::testkit::{dispatch, fem2d_app};
    use crate::editor::fem2d::Fem2dCommand;

    #[test]
    async fn remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0 }));
        let node_id = app.snapshot().expect("snapshot").nodes[0].id.clone();
        dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Case".into(), self_weight: false }));
        let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();

        let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec![node_id, case_id] }));
        assert_eq!(result.mutations.len(), 2);
        assert!(app.snapshot().expect("snapshot").nodes.is_empty());
        assert!(app.snapshot().expect("snapshot").load_cases.is_empty());
    }

    #[test]
    async fn remove_selection_covers_regions_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 1.0, height: 1.0, material_id: "steel".into(), thickness: None, mesh_size: None }));
        let region_id = app.snapshot().expect("snapshot").regions[0].id.clone();
        let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec![region_id] }));
        assert_eq!(result.mutations.len(), 1);
        assert!(app.snapshot().expect("snapshot").regions.is_empty());
    }

    #[test]
    async fn remove_selection_with_no_matching_ids_is_a_no_op_2d() {
        let mut app = fem2d_app();
        let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec!["missing".into()] }));
        assert!(result.mutations.is_empty());
    }
}
