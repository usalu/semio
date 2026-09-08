
use super::*;
use crate::editor::fem3d::Fem3dCommand;
use crate::editor::fem3d::testkit::{dispatch, fem3d_empty_app};

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_solids_3d() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(crate::editor::fem3d::commands::add_material::AddMaterial { name: "Concrete".into(), e: 3.0e10, g: 1.25e10 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.clone();
    dispatch(&mut app, Fem3dCommand::AddSolid(crate::editor::fem3d::commands::add_solid::AddSolid { x: 0.0, y: 0.0, width: 1.0, depth: 1.0, height: 1.0, material_id, base_z: None, layers: None, mesh_size: None })).await;
    let solid_id = app.snapshot().expect("snapshot").solids[0].id.clone();
    dispatch(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec![solid_id] })).await;
    assert!(app.snapshot().expect("snapshot").solids.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_with_unknown_ids_is_a_no_op() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::RemoveSelection(RemoveSelection { ids: vec!["missing".into()] })).await;
    assert!(app.snapshot().expect("snapshot").nodes.is_empty());
}
