//! 🗂️ FEM 3D app commands — bulk selection removal, dispatched across every id-keyed document
//! collection.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigOperation};
use crate::artifacts::fem3d::op::Fem3dOperation;
use crate::artifacts::fem3d::Fem3dDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️RemoveSelection
pub mod remove_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-selection")]
    pub struct RemoveSelection {
        pub ids: Vec<String>,
    }

    /// 🗂️ Each id is looked up against every collection in a fixed precedence (nodes, elements,
    /// materials, sections, supports, load cases, solids, combinations) and removed from the first one
    /// it matches — mirrors the pre-migration `handle_action`'s exact search order.
    pub fn handle(payload: &RemoveSelection, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let mut operations = Vec::new();
        for id in &payload.ids {
            if projection.nodes.iter().any(|n| &n.id == id) {
                operations.push(Fem3dOperation::RemoveNode { id: id.clone() });
            } else if projection.elements.iter().any(|e| crate::artifacts::fem3d::element_id(e) == id) {
                operations.push(Fem3dOperation::RemoveElement { id: id.clone() });
            } else if projection.materials.iter().any(|m| &m.id == id) {
                operations.push(Fem3dOperation::RemoveMaterial { id: id.clone() });
            } else if projection.sections.iter().any(|s| &s.id == id) {
                operations.push(Fem3dOperation::RemoveSection { id: id.clone() });
            } else if projection.supports.iter().any(|s| &s.id == id) {
                operations.push(Fem3dOperation::RemoveSupport { id: id.clone() });
            } else if projection.load_cases.iter().any(|l| &l.id == id) {
                operations.push(Fem3dOperation::RemoveLoadCase { id: id.clone() });
            } else if projection.solids.iter().any(|s| &s.id == id) {
                operations.push(Fem3dOperation::RemoveSolid { id: id.clone() });
            } else if projection.combinations.iter().any(|c| &c.id == id) {
                operations.push(Fem3dOperation::RemoveCombination { id: id.clone() });
            }
        }
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::operations(operations))
        }
    }
}
//#endregion 🔖️RemoveSelection

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app};
    use crate::apps::fem3d::Fem3dCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn remove_selection_covers_solids_3d() {
        let mut app = fem3d_app();
        dispatch(
            &mut app,
            Fem3dCommand::AddSolid(crate::apps::fem3d::commands::model::add_solid::AddSolid { x: 0.0, y: 0.0, width: 1.0, depth: 1.0, height: 1.0, material_id: "concrete".into(), base_z: None, layers: None, mesh_size: None }),
        );
        let solid_id = app.projection().expect("projection").solids[0].id.clone();
        dispatch(&mut app, Fem3dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec![solid_id] }));
        assert!(app.projection().expect("projection").solids.is_empty());
    }

    #[test]
    fn remove_selection_with_unknown_ids_is_a_no_op() {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["missing".into()] }));
        assert!(app.projection().expect("projection").nodes.is_empty());
    }
}
// #endregion 🧪️Tests
