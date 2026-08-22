//! 🧱️ 🧱️ Fem2d play app commands command — `add-node`.

use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemNode;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️AddNode
//#endregion 🔖️AddNode

//#region 🔖️AddBar
//#endregion 🔖️AddBar

//#region 🔖️AddBeam
//#endregion 🔖️AddBeam

//#region 🔖️AddMaterial
//#endregion 🔖️AddMaterial

//#region 🔖️AddSection
//#endregion 🔖️AddSection

//#region 🔖️AddSupport
//#endregion 🔖️AddSupport

//#region 🔖️AddRegion
//#endregion 🔖️AddRegion

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.nodes.iter().map(|n| n.id.clone()), "n");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateNode(crate::artifacts::fem2d::mutations::create_node::mutation::CreateNode { node: FemNode { id, x: payload.x, y: payload.y } })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{FemDof, FemElement};
    use crate::editor::fem2d::commands::{add_bar, add_beam, add_material, add_region, add_section, add_support};
    use crate::editor::fem2d::testkit::{dispatch, fem2d_app};
    use crate::editor::fem2d::Fem2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn add_node_action_emits_op_2d() {
        let mut app = fem2d_app();
        let result = dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 1.0, y: 2.0 })).await;
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").nodes.last().expect("node added").x, 1.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_bar_and_add_beam_actions_emit_ops_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
        dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "Section".into(), area: 0.01, iy: 0.001 })).await;
        dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 0.0, y: 0.0 })).await;
        dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 1.0, y: 0.0 })).await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let (start, end) = (snapshot.nodes[0].id.clone(), snapshot.nodes[1].id.clone());
        let (material_id, section_id) = (snapshot.materials[0].id.clone(), snapshot.sections[0].id.clone());
        dispatch(&mut app, Fem2dCommand::AddBar(add_bar::AddBar { start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone() })).await;
        assert!(matches!(semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").elements.last(), Some(FemElement::Bar { .. })));
        dispatch(&mut app, Fem2dCommand::AddBeam(add_beam::AddBeam { start, end, material_id, section_id })).await;
        assert!(matches!(semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").elements.last(), Some(FemElement::Beam { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_material_action_emits_op_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
        let material = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").materials.last().expect("material added").clone();
        assert_eq!(material.name, "Steel");
        assert_eq!(material.e, 2.1e11);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_section_action_emits_op_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 })).await;
        assert_eq!(semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").sections.last().expect("section added").name, "HEA200");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_support_action_emits_op_with_fixed_dofs_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 0.0, y: 0.0 })).await;
        let node_id = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").nodes[0].id.clone();
        dispatch(&mut app, Fem2dCommand::AddSupport(add_support::AddSupport { node_id, fixed: vec![FemDof::Tx, FemDof::Ty] })).await;
        assert_eq!(semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").supports.last().expect("support added").fixed, vec![FemDof::Tx, FemDof::Ty]);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_region_action_emits_set_region_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
        let material_id = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").materials[0].id.clone();
        dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id, thickness: None, mesh_size: None })).await;
        let region = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot").regions.last().expect("region added").clone();
        assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
        assert_eq!(region.thickness, 0.02);
        assert_eq!(region.mesh_size, 0.25);
    }
}
//#endregion 🧪️Tests
