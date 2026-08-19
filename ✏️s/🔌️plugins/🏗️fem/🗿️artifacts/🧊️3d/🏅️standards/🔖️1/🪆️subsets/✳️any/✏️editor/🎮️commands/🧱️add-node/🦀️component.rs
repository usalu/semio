//! 🧱️ 🧱️ FEM 3D app commands command — `add-node`.

use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub async fn handle(payload: &AddNode, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.nodes.iter().map(|n| n.id.clone()), "n");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateNode(crate::artifacts::fem3d::mutations::create_node::mutation::CreateNode { node: crate::artifacts::fem3d::FemNode { id, x: payload.x, y: payload.y, z: payload.z } })]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem3d::commands::{add_frame, add_material, add_section, add_solid};
    use crate::editor::fem3d::testkit::{dispatch, fem3d_app, Fem3dApp};
    use crate::editor::fem3d::Fem3dCommand;

    #[semio_framework_async_macros::async_test]
    async fn add_node_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddNode(AddNode { x: 1.0, y: 2.0, z: 3.0 }));
        let snapshot = app.snapshot().expect("snapshot");
        let node = snapshot.nodes.last().expect("node added");
        assert_eq!((node.x, node.y, node.z), (1.0, 2.0, 3.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_material_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.1e10 }));
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.materials.last().expect("material added").g, 8.1e10);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_section_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 }));
        let snapshot = app.snapshot().expect("snapshot");
        assert_eq!(snapshot.sections.last().expect("section added").j, 0.0000006);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_frame_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddFrame(add_frame::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into(), roll: 0.5 }));
        let snapshot = app.snapshot().expect("snapshot");
        match snapshot.elements.last().expect("element added") {
            crate::artifacts::fem3d::FemElement::Frame { roll, .. } => assert_eq!(*roll, 0.5),
            _ => panic!("expected Frame"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn add_solid_action_emits_set_solid_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddSolid(add_solid::AddSolid { x: 0.0, y: 0.0, width: 2.0, depth: 1.0, height: 0.5, material_id: "concrete".into(), base_z: None, layers: None, mesh_size: None }));
        let snapshot = app.snapshot().expect("snapshot");
        let solid = snapshot.solids.last().expect("solid added");
        assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
        assert_eq!(solid.height, 0.5);
        assert_eq!(solid.layers, 1);
    }
}
