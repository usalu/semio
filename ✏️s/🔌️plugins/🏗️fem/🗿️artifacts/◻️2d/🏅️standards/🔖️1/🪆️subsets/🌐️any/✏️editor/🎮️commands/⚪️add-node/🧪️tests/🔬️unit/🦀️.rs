use crate::context::{dispatch, dispatch_editor_action, fem2d_empty_app};
use semio_framework_value::DslValue;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{add_material, add_node::AddNode, add_section};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::{FemDof, FemElement};

#[semio_framework_async_macros::async_test]
async fn add_node_action_emits_op_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 1.0, y: 2.0 })).await;
    assert_eq!(app.snapshot_ref().nodes.iter().last().expect("node added").x, 1.0);
}

#[semio_framework_async_macros::async_test]
async fn add_bar_and_add_beam_actions_emit_ops_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "Section".into(), area: 0.01, iy: 0.001 })).await;
    dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 0.0, y: 0.0 })).await;
    dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 1.0, y: 0.0 })).await;
    let snapshot = app.snapshot_ref();
    let (start, end) = (snapshot.nodes[0].id.as_str().to_owned(), snapshot.nodes[1].id.as_str().to_owned());
    let (material_id, section_id) = (snapshot.materials[0].id.as_str().to_owned(), snapshot.sections[0].id.clone());
    dispatch_editor_action(&mut app, "addBar", DslValue::from(&serde_json::json!({ "start": &start, "end": &end, "materialId": &material_id, "sectionId": &section_id }))).await;
    assert!(matches!(app.snapshot_ref().elements.last(), Some(FemElement::Bar { .. })));
    dispatch_editor_action(&mut app, "addBeam", DslValue::from(&serde_json::json!({ "start": start, "end": end, "materialId": material_id, "sectionId": section_id }))).await;
    assert!(matches!(app.snapshot_ref().elements.last(), Some(FemElement::Beam { .. })));
}

#[semio_framework_async_macros::async_test]
async fn add_material_action_emits_op_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    let material = app.snapshot_ref().materials.iter().last().expect("material added");
    assert_eq!(material.name.as_str(), "Steel");
    assert_eq!(material.e, 2.1e11);
}

#[semio_framework_async_macros::async_test]
async fn add_section_action_emits_op_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 })).await;
    assert_eq!(app.snapshot_ref().sections.last().expect("section added").name, "HEA200");
}

#[semio_framework_async_macros::async_test]
async fn add_support_action_emits_op_with_fixed_dofs_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddNode(AddNode { x: 0.0, y: 0.0 })).await;
    let node_id = app.snapshot_ref().nodes[0].id.as_str().to_owned();
    dispatch_editor_action(&mut app, "addSupport", DslValue::from(&serde_json::json!({ "nodeId": node_id, "fixed": "Tx,Ty" }))).await;
    assert_eq!(app.snapshot_ref().supports.last().expect("support added").fixed, vec![FemDof::Tx, FemDof::Ty]);
}

#[semio_framework_async_macros::async_test]
async fn add_region_action_emits_set_region_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    let material_id = app.snapshot_ref().materials[0].id.as_str().to_owned();
    dispatch_editor_action(&mut app, "addRegion", DslValue::from(&serde_json::json!({ "x": 0.0, "y": 0.0, "width": 4.0, "height": 2.0, "materialId": material_id }))).await;
    let region = app.snapshot_ref().regions.last().expect("region added");
    assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
    assert_eq!(region.thickness, 0.02);
    assert_eq!(region.mesh_size, 0.25);
}
