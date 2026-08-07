//! 🧱️ FEM 3D app commands — model-building: nodes, bar/frame members, materials, sections, supports,
//! and meshed solid blocks.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigOperation};
use crate::artifacts::fem3d::op::Fem3dOperation;
use crate::artifacts::fem3d::Fem3dDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddNode
pub mod add_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-node")]
    pub struct AddNode {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    pub fn handle(payload: &AddNode, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::app_surface::next_id(projection.nodes.iter().map(|n| n.id.clone()), "n");
        let index = projection.nodes.len();
        Ok(Emit::operations(vec![Fem3dOperation::SetNode { index, node: crate::artifacts::fem3d::FemNode { id, x: payload.x, y: payload.y, z: payload.z } }]))
    }
}
//#endregion 🔖️AddNode

//#region 🔖️AddBar
pub mod add_bar {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "add-bar")]
    pub struct AddBar {
        pub start: String,
        pub end: String,
        pub material_id: String,
        pub section_id: String,
    }

    pub fn handle(payload: &AddBar, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::app_surface::next_id(projection.elements.iter().map(|e| crate::artifacts::fem3d::element_id(e).to_string()), "e");
        let index = projection.elements.len();
        let element = crate::artifacts::fem3d::FemElement::Bar { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
        Ok(Emit::operations(vec![Fem3dOperation::SetElement { index, element: Box::new(element) }]))
    }
}
//#endregion 🔖️AddBar

//#region 🔖️AddFrame
pub mod add_frame {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "add-frame")]
    pub struct AddFrame {
        pub start: String,
        pub end: String,
        pub material_id: String,
        pub section_id: String,
        pub roll: f64,
    }

    pub fn handle(payload: &AddFrame, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::app_surface::next_id(projection.elements.iter().map(|e| crate::artifacts::fem3d::element_id(e).to_string()), "e");
        let index = projection.elements.len();
        let element = crate::artifacts::fem3d::FemElement::Frame { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone(), roll: payload.roll };
        Ok(Emit::operations(vec![Fem3dOperation::SetElement { index, element: Box::new(element) }]))
    }
}
//#endregion 🔖️AddFrame

//#region 🔖️AddMaterial
pub mod add_material {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-material")]
    pub struct AddMaterial {
        pub name: String,
        pub e: f64,
        pub g: f64,
    }

    /// 🧱️ New materials default to `nu = 0.3`/`rho = 7850.0` (mild steel) — the manifest's `addMaterial`
    /// arg form only stages `name`/`e`/`g`, matching the pre-migration `handle_action` behavior verbatim.
    pub fn handle(payload: &AddMaterial, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::app_surface::next_id(projection.materials.iter().map(|m| m.id.clone()), "m");
        let index = projection.materials.len();
        Ok(Emit::operations(vec![Fem3dOperation::SetMaterial { index, material: crate::artifacts::fem3d::FemMaterial { id, name: payload.name.clone(), e: payload.e, g: payload.g, nu: 0.3, rho: 7850.0 } }]))
    }
}
//#endregion 🔖️AddMaterial

//#region 🔖️AddSection
pub mod add_section {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-section")]
    pub struct AddSection {
        pub name: String,
        pub area: f64,
        pub iy: f64,
        pub iz: f64,
        pub j: f64,
    }

    pub fn handle(payload: &AddSection, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::app_surface::next_id(projection.sections.iter().map(|s| s.id.clone()), "s");
        let index = projection.sections.len();
        Ok(Emit::operations(vec![Fem3dOperation::SetSection { index, section: crate::artifacts::fem3d::FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy, iz: payload.iz, j: payload.j } }]))
    }
}
//#endregion 🔖️AddSection

//#region 🔖️AddSupport
pub mod add_support {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "add-support")]
    pub struct AddSupport {
        pub node_id: String,
        pub fixed: Vec<crate::artifacts::fem3d::FemDof>,
    }

    pub fn handle(payload: &AddSupport, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::app_surface::next_id(projection.supports.iter().map(|s| s.id.clone()), "sup");
        let index = projection.supports.len();
        Ok(Emit::operations(vec![Fem3dOperation::SetSupport { index, support: crate::artifacts::fem3d::FemSupport { id, node_id: payload.node_id.clone(), fixed: payload.fixed.clone() } }]))
    }
}
//#endregion 🔖️AddSupport

//#region 🔖️AddSolid
pub mod add_solid {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "add-solid")]
    pub struct AddSolid {
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub depth: f64,
        pub height: f64,
        pub material_id: String,
        pub base_z: Option<f64>,
        pub layers: Option<u32>,
        pub mesh_size: Option<f64>,
    }

    /// 🧱️ Builds a rectangular footprint `[x,y]..[x+width,y+depth]` with `base_z`/`layers`/`mesh_size`
    /// defaulted to `0.0`/`1`/`0.5` when unspecified — mirrors the pre-migration `handle_action` defaults.
    pub fn handle(payload: &AddSolid, doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::app_surface::next_id(projection.solids.iter().map(|s| s.id.clone()), "sol");
        let index = projection.solids.len();
        let outline = vec![[payload.x, payload.y], [payload.x + payload.width, payload.y], [payload.x + payload.width, payload.y + payload.depth], [payload.x, payload.y + payload.depth]];
        let solid = crate::artifacts::fem3d::FemSolid {
            id,
            name: "Solid".into(),
            outline,
            holes: Vec::new(),
            base_z: payload.base_z.unwrap_or(0.0),
            height: payload.height,
            layers: payload.layers.map(|v| v as usize).unwrap_or(1),
            mesh_size: payload.mesh_size.unwrap_or(0.5),
            material_id: payload.material_id.clone(),
        };
        Ok(Emit::operations(vec![Fem3dOperation::SetSolid { index, solid }]))
    }
}
//#endregion 🔖️AddSolid

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app, Fem3dApp};
    use crate::apps::fem3d::Fem3dCommand;

    #[test]
    fn add_node_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }));
        let projection = app.projection().expect("projection");
        let node = projection.nodes.last().expect("node added");
        assert_eq!((node.x, node.y, node.z), (1.0, 2.0, 3.0));
    }

    #[test]
    fn add_material_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.1e10 }));
        let projection = app.projection().expect("projection");
        assert_eq!(projection.materials.last().expect("material added").g, 8.1e10);
    }

    #[test]
    fn add_section_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 }));
        let projection = app.projection().expect("projection");
        assert_eq!(projection.sections.last().expect("section added").j, 0.0000006);
    }

    #[test]
    fn add_frame_action_emits_op_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddFrame(add_frame::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into(), roll: 0.5 }));
        let projection = app.projection().expect("projection");
        match projection.elements.last().expect("element added") {
            crate::artifacts::fem3d::FemElement::Frame { roll, .. } => assert_eq!(*roll, 0.5),
            _ => panic!("expected Frame"),
        }
    }

    #[test]
    fn add_solid_action_emits_set_solid_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddSolid(add_solid::AddSolid { x: 0.0, y: 0.0, width: 2.0, depth: 1.0, height: 0.5, material_id: "concrete".into(), base_z: None, layers: None, mesh_size: None }));
        let projection = app.projection().expect("projection");
        let solid = projection.solids.last().expect("solid added");
        assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
        assert_eq!(solid.height, 0.5);
        assert_eq!(solid.layers, 1);
    }
}
// #endregion 🧪️Tests
