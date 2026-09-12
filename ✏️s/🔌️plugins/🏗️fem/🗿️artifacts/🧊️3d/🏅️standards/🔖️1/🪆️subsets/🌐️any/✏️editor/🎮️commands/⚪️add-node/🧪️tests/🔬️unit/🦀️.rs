use super::*;
use crate::editor::fem3d::commands::{add_frame, add_material, add_section, add_solid};
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_empty_app, Fem3dApp};
use crate::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn add_node_action_emits_op_3d() {
    let mut app: Fem3dApp = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddNode(AddNode { x: 1.0, y: 2.0, z: 3.0 })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = snapshot.nodes.last().expect("node added");
    assert_eq!((node.x, node.y, node.z), (1.0, 2.0, 3.0));
}

#[semio_framework_async_macros::async_test]
async fn add_material_action_emits_op_3d() {
    let mut app: Fem3dApp = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.1e10 })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.materials.last().expect("material added").g, 8.1e10);
}

#[semio_framework_async_macros::async_test]
async fn add_section_action_emits_op_3d() {
    let mut app: Fem3dApp = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 })).await;
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.sections.last().expect("section added").j, 0.0000006);
}

#[semio_framework_async_macros::async_test]
async fn add_frame_action_emits_op_3d() {
    let mut app: Fem3dApp = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.1e10 })).await;
    dispatch(&mut app, Fem3dCommand::AddSection(add_section::AddSection { name: "Section".into(), area: 0.01, iy: 0.001, iz: 0.001, j: 0.001 })).await;
    dispatch(&mut app, Fem3dCommand::AddNode(AddNode { x: 0.0, y: 0.0, z: 0.0 })).await;
    dispatch(&mut app, Fem3dCommand::AddNode(AddNode { x: 1.0, y: 0.0, z: 0.0 })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let (start, end) = (snapshot.nodes[0].id.clone(), snapshot.nodes[1].id.clone());
    let (material_id, section_id) = (snapshot.materials[0].id.clone(), snapshot.sections[0].id.clone());
    dispatch(&mut app, Fem3dCommand::AddFrame(add_frame::AddFrame { start, end, material_id, section_id, roll: 0.5 })).await;
    let snapshot = app.snapshot().expect("snapshot");
    match snapshot.elements.last().expect("element added") {
        crate::FemElement::Frame { roll, .. } => assert_eq!(*roll, 0.5),
        _ => panic!("expected Frame"),
    }
}

#[semio_framework_async_macros::async_test]
async fn add_solid_action_emits_set_solid_3d() {
    let mut app: Fem3dApp = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Concrete".into(), e: 3.0e10, g: 1.25e10 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.clone();
    dispatch(&mut app, Fem3dCommand::AddSolid(add_solid::AddSolid { x: 0.0, y: 0.0, width: 2.0, depth: 1.0, height: 0.5, material_id, base_z: None, layers: None, mesh_size: None })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let solid = snapshot.solids.last().expect("solid added");
    assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
    assert_eq!(solid.height, 0.5);
    assert_eq!(solid.layers, 1);
}
