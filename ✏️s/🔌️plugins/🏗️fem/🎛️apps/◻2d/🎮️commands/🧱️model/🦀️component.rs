//! 🧱️ Fem2d play app commands — structural model authoring: nodes, bars/beams, materials, sections,
//! supports, and meshed regions.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::{element_id, FemDof, FemElement, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️AddNode
pub mod add_node {
    use super::*;

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

    pub fn handle(payload: &AddBar, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.elements.iter().map(|e| element_id(e).to_string()), "e");
        let element = FemElement::Bar { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
        Ok(Emit::mutations(vec![Fem2dMutation::CreateElement(crate::artifacts::fem2d::mutations::create_element::mutation::CreateElement { element: Box::new(element) })]))
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

    pub fn handle(payload: &AddBeam, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.elements.iter().map(|e| element_id(e).to_string()), "e");
        let element = FemElement::Beam { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone() };
        Ok(Emit::mutations(vec![Fem2dMutation::CreateElement(crate::artifacts::fem2d::mutations::create_element::mutation::CreateElement { element: Box::new(element) })]))
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

    pub fn handle(payload: &AddMaterial, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.materials.iter().map(|m| m.id.clone()), "m");
        Ok(Emit::mutations(vec![Fem2dMutation::CreateMaterial(crate::artifacts::fem2d::mutations::create_material::mutation::CreateMaterial { material: FemMaterial { id, name: payload.name.clone(), e: payload.e, nu: 0.3, rho: 7850.0 } })]))
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

    pub fn handle(payload: &AddSection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.sections.iter().map(|s| s.id.clone()), "s");
        Ok(Emit::mutations(vec![Fem2dMutation::CreateSection(crate::artifacts::fem2d::mutations::create_section::mutation::CreateSection { section: FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy } })]))
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

    pub fn handle(payload: &AddSupport, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.supports.iter().map(|s| s.id.clone()), "sup");
        Ok(Emit::mutations(vec![Fem2dMutation::CreateSupport(crate::artifacts::fem2d::mutations::create_support::mutation::CreateSupport { support: FemSupport { id, node_id: payload.node_id.clone(), fixed: payload.fixed.clone() } })]))
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

    pub fn handle(payload: &AddRegion, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = crate::app_surface::next_id(snapshot.regions.iter().map(|r| r.id.clone()), "r");
        let outline = vec![[payload.x, payload.y], [payload.x + payload.width, payload.y], [payload.x + payload.width, payload.y + payload.height], [payload.x, payload.y + payload.height]];
        let region = FemRegion { id, name: "Region".into(), outline, holes: Vec::new(), thickness: payload.thickness.unwrap_or(0.02), material_id: payload.material_id.clone(), mesh_size: payload.mesh_size.unwrap_or(0.25) };
        Ok(Emit::mutations(vec![Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::mutation::CreateRegion { region })]))
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
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(app.snapshot().expect("snapshot").nodes.last().expect("node added").x, 1.0);
    }

    #[test]
    fn add_bar_and_add_beam_actions_emit_ops_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }));
        assert!(matches!(app.snapshot().expect("snapshot").elements.last(), Some(FemElement::Bar { .. })));
        dispatch(&mut app, Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }));
        assert!(matches!(app.snapshot().expect("snapshot").elements.last(), Some(FemElement::Beam { .. })));
    }

    #[test]
    fn add_material_action_emits_op_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }));
        let material = app.snapshot().expect("snapshot").materials.last().expect("material added").clone();
        assert_eq!(material.name, "Steel");
        assert_eq!(material.e, 2.1e11);
    }

    #[test]
    fn add_section_action_emits_op_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 }));
        assert_eq!(app.snapshot().expect("snapshot").sections.last().expect("section added").name, "HEA200");
    }

    #[test]
    fn add_support_action_emits_op_with_fixed_dofs_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }));
        assert_eq!(app.snapshot().expect("snapshot").supports.last().expect("support added").fixed, vec![FemDof::Tx, FemDof::Ty]);
    }

    #[test]
    fn add_region_action_emits_set_region_2d() {
        let mut app = fem2d_app();
        dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: None, mesh_size: None }));
        let region = app.snapshot().expect("snapshot").regions.last().expect("region added").clone();
        assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
        assert_eq!(region.thickness, 0.02);
        assert_eq!(region.mesh_size, 0.25);
    }
}
//#endregion 🧪️Tests
