
use super::*;
use crate::editor::fem2d::Fem2dCommand;
use crate::editor::fem2d::commands::{add_bar, add_beam, add_material, add_region, add_section, add_support};
use crate::editor::fem2d::testkit::{dispatch, fem2d_app};
use crate::{FemDof, FemElement};

#[semio_framework_async_macros::async_test]
async fn add_node_action_emits_op_2d() {
    let mut app = fem2d_app();
    let result = dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 1.0, y: 2.0 })).await;
    assert_eq!(result.mutations.len(), 1);
    assert_eq!(app.snapshot().expect("snapshot").nodes.last().expect("node added").x, 1.0);
}

#[semio_framework_async_macros::async_test]
async fn add_bar_and_add_beam_actions_emit_ops_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "Section".into(), area: 0.01, iy: 0.001 })).await;
    dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 0.0, y: 0.0 })).await;
    dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 1.0, y: 0.0 })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let (start, end) = (snapshot.nodes[0].id.clone(), snapshot.nodes[1].id.clone());
    let (material_id, section_id) = (snapshot.materials[0].id.clone(), snapshot.sections[0].id.clone());
    dispatch(&mut app, Fem2dCommand::AddBar(add_bar::AddBar { start: start.clone(), end: end.clone(), material_id: material_id.clone(), section_id: section_id.clone() })).await;
    assert!(matches!(app.snapshot().expect("snapshot").elements.last(), Some(FemElement::Bar { .. })));
    dispatch(&mut app, Fem2dCommand::AddBeam(add_beam::AddBeam { start, end, material_id, section_id })).await;
    assert!(matches!(app.snapshot().expect("snapshot").elements.last(), Some(FemElement::Beam { .. })));
}

#[semio_framework_async_macros::async_test]
async fn add_material_action_emits_op_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    let material = app.snapshot().expect("snapshot").materials.last().expect("material added").clone();
    assert_eq!(material.name, "Steel");
    assert_eq!(material.e, 2.1e11);
}

#[semio_framework_async_macros::async_test]
async fn add_section_action_emits_op_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 })).await;
    assert_eq!(app.snapshot().expect("snapshot").sections.last().expect("section added").name, "HEA200");
}

#[semio_framework_async_macros::async_test]
async fn add_support_action_emits_op_with_fixed_dofs_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 0.0, y: 0.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddSupport(add_support::AddSupport { node_id, fixed: vec![FemDof::Tx, FemDof::Ty] })).await;
    assert_eq!(app.snapshot().expect("snapshot").supports.last().expect("support added").fixed, vec![FemDof::Tx, FemDof::Ty]);
}

#[semio_framework_async_macros::async_test]
async fn add_region_action_emits_set_region_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id, thickness: None, mesh_size: None })).await;
    let region = app.snapshot().expect("snapshot").regions.last().expect("region added").clone();
    assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
    assert_eq!(region.thickness, 0.02);
    assert_eq!(region.mesh_size, 0.25);
}
