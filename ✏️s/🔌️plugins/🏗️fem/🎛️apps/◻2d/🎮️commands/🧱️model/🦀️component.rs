//! 🧱️ Fem2d play app commands — structural model authoring: nodes, bars/beams, materials, sections,
//! supports, and meshed regions.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigOperation};
use crate::artifacts::fem2d::op::Fem2dOperation;
use crate::artifacts::fem2d::{element_id, FemDof, FemElement, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dDocument = crate::artifacts::fem2d::Fem2dDocument;

//#region 🔖️AddNode
pub mod add_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-node")]
    pub struct AddNode {
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &AddNode, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.nodes.iter().map(|n| n.id.clone()), "n");
        let index = projection.nodes.len();
        Ok(Emit::operations(vec![Fem2dOperation::SetNode { index, node: FemNode { id, x: payload.x, y: payload.y } }]))
    }
}
//#endregion 🔖️AddNode

//#region 🔖️AddBar
pub mod add_bar {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-bar")]
    pub struct AddBar {
        pub start: String,
        pub end: String,
        pub material_id: String,
        pub section_id: String,
    }

    pub fn handle(payload: &AddBar, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.elements.iter().map(|e| element_id(e).to_string()), "e");
        let index = projection.elements.len();
        let element = FemElement::Bar { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
        Ok(Emit::operations(vec![Fem2dOperation::SetElement { index, element: Box::new(element) }]))
    }
}
//#endregion 🔖️AddBar

//#region 🔖️AddBeam
pub mod add_beam {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-beam")]
    pub struct AddBeam {
        pub start: String,
        pub end: String,
        pub material_id: String,
        pub section_id: String,
    }

    pub fn handle(payload: &AddBeam, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.elements.iter().map(|e| element_id(e).to_string()), "e");
        let index = projection.elements.len();
        let element = FemElement::Beam { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
        Ok(Emit::operations(vec![Fem2dOperation::SetElement { index, element: Box::new(element) }]))
    }
}
//#endregion 🔖️AddBeam

//#region 🔖️AddMaterial
pub mod add_material {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-material")]
    pub struct AddMaterial {
        pub name: String,
        pub e: f64,
    }

    pub fn handle(payload: &AddMaterial, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.materials.iter().map(|m| m.id.clone()), "m");
        let index = projection.materials.len();
        Ok(Emit::operations(vec![Fem2dOperation::SetMaterial { index, material: FemMaterial { id, name: payload.name.clone(), e: payload.e, nu: 0.3, rho: 7850.0 } }]))
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
    }

    pub fn handle(payload: &AddSection, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.sections.iter().map(|s| s.id.clone()), "s");
        let index = projection.sections.len();
        Ok(Emit::operations(vec![Fem2dOperation::SetSection { index, section: FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy } }]))
    }
}
//#endregion 🔖️AddSection

//#region 🔖️AddSupport
pub mod add_support {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-support")]
    pub struct AddSupport {
        pub node_id: String,
        pub fixed: Vec<FemDof>,
    }

    pub fn handle(payload: &AddSupport, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.supports.iter().map(|s| s.id.clone()), "sup");
        let index = projection.supports.len();
        Ok(Emit::operations(vec![Fem2dOperation::SetSupport { index, support: FemSupport { id, node_id: payload.node_id.clone(), fixed: payload.fixed.clone() } }]))
    }
}
//#endregion 🔖️AddSupport

//#region 🔖️AddRegion
pub mod add_region {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-region")]
    pub struct AddRegion {
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
        pub material_id: String,
        pub thickness: Option<f64>,
        pub mesh_size: Option<f64>,
    }

    pub fn handle(payload: &AddRegion, doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        let projection = doc.projection;
        let id = crate::core::shared::next_id(projection.regions.iter().map(|r| r.id.clone()), "r");
        let index = projection.regions.len();
        let outline = vec![[payload.x, payload.y], [payload.x + payload.width, payload.y], [payload.x + payload.width, payload.y + payload.height], [payload.x, payload.y + payload.height]];
        let region = FemRegion { id, name: "Region".into(), outline, holes: Vec::new(), thickness: payload.thickness.unwrap_or(0.02), material_id: payload.material_id.clone(), mesh_size: payload.mesh_size.unwrap_or(0.25) };
        Ok(Emit::operations(vec![Fem2dOperation::SetRegion { index, region }]))
    }
}
//#endregion 🔖️AddRegion

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn add_node_action_emits_op_2d() {
        let mut app = fem2d_app();
        let result = dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }));
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").nodes.last().expect("node added").x, 1.0);
    }

    #[test]
    fn add_bar_and_add_beam_actions_emit_ops_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }));
        assert!(matches!(app.projection().expect("projection").elements.last(), Some(FemElement::Bar { .. })));
        dispatch(&mut app, Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }));
        assert!(matches!(app.projection().expect("projection").elements.last(), Some(FemElement::Beam { .. })));
    }

    #[test]
    fn add_material_action_emits_op_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }));
        let material = app.projection().expect("projection").materials.last().expect("material added").clone();
        assert_eq!(material.name, "Steel");
        assert_eq!(material.e, 2.1e11);
    }

    #[test]
    fn add_section_action_emits_op_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 }));
        assert_eq!(app.projection().expect("projection").sections.last().expect("section added").name, "HEA200");
    }

    #[test]
    fn add_support_action_emits_op_with_fixed_dofs_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }));
        assert_eq!(app.projection().expect("projection").supports.last().expect("support added").fixed, vec![FemDof::Tx, FemDof::Ty]);
    }

    #[test]
    fn add_region_action_emits_set_region_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: None, mesh_size: None }));
        let region = app.projection().expect("projection").regions.last().expect("region added").clone();
        assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
        assert_eq!(region.thickness, 0.02);
        assert_eq!(region.mesh_size, 0.25);
    }
}
//#endregion 🧪️Tests
