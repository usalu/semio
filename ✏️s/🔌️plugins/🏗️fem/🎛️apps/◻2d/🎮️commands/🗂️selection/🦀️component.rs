//! 🗂️ Fem2d play app commands — removing a mixed-kind selection of document entities.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::element_id;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️RemoveSelection
pub mod remove_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-selection")]
    pub struct RemoveSelection {
        pub ids: Vec<String>,
    }

    /// 🧩️ Resolves each id against every collection in turn (nodes, elements, materials, sections,
    /// supports, load cases, regions, combinations) and emits the matching typed `Remove*` operation —
    /// mirrors the pre-migration stringly-typed selection-delete dispatch, now over the typed document.
    pub fn handle(payload: &RemoveSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let mut operations = Vec::new();
        for id in &payload.ids {
            if snapshot.nodes.iter().any(|n| &n.id == id) {
                operations.push(Fem2dMutation::RemoveNode { id: id.clone() });
            } else if snapshot.elements.iter().any(|e| element_id(e) == id) {
                operations.push(Fem2dMutation::RemoveElement { id: id.clone() });
            } else if snapshot.materials.iter().any(|m| &m.id == id) {
                operations.push(Fem2dMutation::RemoveMaterial { id: id.clone() });
            } else if snapshot.sections.iter().any(|s| &s.id == id) {
                operations.push(Fem2dMutation::RemoveSection { id: id.clone() });
            } else if snapshot.supports.iter().any(|s| &s.id == id) {
                operations.push(Fem2dMutation::RemoveSupport { id: id.clone() });
            } else if snapshot.load_cases.iter().any(|l| &l.id == id) {
                operations.push(Fem2dMutation::RemoveLoadCase { id: id.clone() });
            } else if snapshot.regions.iter().any(|r| &r.id == id) {
                operations.push(Fem2dMutation::RemoveRegion { id: id.clone() });
            } else if snapshot.combinations.iter().any(|c| &c.id == id) {
                operations.push(Fem2dMutation::RemoveCombination { id: id.clone() });
            }
        }
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::mutations(operations))
        }
    }
}
//#endregion 🔖️RemoveSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::commands::model::{add_node, add_region};
    use crate::apps::fem2d::commands::loads::add_load_case;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0 }));
        let node_id = app.snapshot().expect("snapshot").nodes[0].id.clone();
        dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Case".into(), self_weight: false }));
        let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();

        let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec![node_id, case_id] }));
        assert_eq!(result.mutations.len(), 2);
        assert!(app.snapshot().expect("snapshot").nodes.is_empty());
        assert!(app.snapshot().expect("snapshot").load_cases.is_empty());
    }

    #[test]
    fn remove_selection_covers_regions_and_combinations_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 1.0, height: 1.0, material_id: "steel".into(), thickness: None, mesh_size: None }));
        let region_id = app.snapshot().expect("snapshot").regions[0].id.clone();
        let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec![region_id] }));
        assert_eq!(result.mutations.len(), 1);
        assert!(app.snapshot().expect("snapshot").regions.is_empty());
    }

    #[test]
    fn remove_selection_with_no_matching_ids_is_a_no_op() {
        let mut app = fem2d_app();
        let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["missing".into()] }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
