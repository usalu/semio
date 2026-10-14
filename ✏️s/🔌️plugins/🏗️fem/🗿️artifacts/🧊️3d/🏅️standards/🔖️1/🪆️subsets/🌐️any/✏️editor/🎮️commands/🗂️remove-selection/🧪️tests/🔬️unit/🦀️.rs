use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::remove_selection::*;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem3d_empty_app};
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_solids_3d() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::AddMaterial(semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_material::AddMaterial { name: "Concrete".into(), e: 3.0e10, g: 1.25e10 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.clone();
    dispatch(&mut app, Fem3dCommand::AddSolid(semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_solid::AddSolid { x: 0.0, y: 0.0, width: 1.0, depth: 1.0, height: 1.0, material_id, base_z: None, layers: None, mesh_size: None, axis: None })).await;
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
