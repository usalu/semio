//! 🗂️ 🗂️ FEM 3D app commands command — `remove-selection`.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::mutations::{delete_combination, delete_element, delete_load_case, delete_material, delete_node, delete_section, delete_solid, delete_support};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
            operations.push(Fem3dMutation::DeleteNode(delete_node::mutation::DeleteNode { id: id.clone() }));
        } else if snapshot.elements.iter().any(|e| crate::artifacts::fem3d::element_id(e) == id) {
            operations.push(Fem3dMutation::DeleteElement(delete_element::mutation::DeleteElement { id: id.clone() }));
        } else if snapshot.materials.iter().any(|m| &m.id == id) {
            operations.push(Fem3dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: id.clone() }));
        } else if snapshot.sections.iter().any(|s| &s.id == id) {
            operations.push(Fem3dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: id.clone() }));
        } else if snapshot.supports.iter().any(|s| &s.id == id) {
            operations.push(Fem3dMutation::DeleteSupport(delete_support::mutation::DeleteSupport { id: id.clone() }));
        } else if snapshot.load_cases.iter().any(|l| &l.id == id) {
            operations.push(Fem3dMutation::DeleteLoadCase(delete_load_case::mutation::DeleteLoadCase { id: id.clone() }));
        } else if snapshot.solids.iter().any(|s| &s.id == id) {
            operations.push(Fem3dMutation::DeleteSolid(delete_solid::mutation::DeleteSolid { id: id.clone() }));
        } else if snapshot.combinations.iter().any(|c| &c.id == id) {
            operations.push(Fem3dMutation::DeleteCombination(delete_combination::mutation::DeleteCombination { id: id.clone() }));
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
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app};
    use crate::apps::fem3d::Fem3dCommand;

    #[test]
    fn remove_selection_covers_solids_3d() {
        let mut app = fem3d_app();
        dispatch(
            &mut app,
            Fem3dCommand::AddSolid(crate::apps::fem3d::commands::add_solid::AddSolid { x: 0.0, y: 0.0, width: 1.0, depth: 1.0, height: 1.0, material_id: "concrete".into(), base_z: None, layers: None, mesh_size: None }),
        );
        let solid_id = app.snapshot().expect("snapshot").solids[0].id.clone();
        dispatch(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec![solid_id] }));
        assert!(app.snapshot().expect("snapshot").solids.is_empty());
    }

    #[test]
    fn remove_selection_with_unknown_ids_is_a_no_op() {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec!["missing".into()] }));
        assert!(app.snapshot().expect("snapshot").nodes.is_empty());
    }
}
