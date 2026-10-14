use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::remove_selection::*;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::delete_region;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_load_case;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::{add_material, add_node, add_region};
use crate::context::{dispatch, fem2d_empty_app};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 0.0, y: 0.0 })).await;
    let node_id = app.snapshot().expect("snapshot").nodes[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Case".into(), self_weight: false })).await;
    let case_id = app.snapshot().expect("snapshot").load_cases[0].id.clone();

    dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec![node_id, case_id] })).await;
    assert!(app.snapshot().expect("snapshot").nodes.is_empty());
    assert!(app.snapshot().expect("snapshot").load_cases.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_covers_regions_2d() {
    let mut app = fem2d_empty_app();
    dispatch(&mut app, Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 })).await;
    let material_id = app.snapshot().expect("snapshot").materials[0].id.clone();
    dispatch(&mut app, Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 1.0, height: 1.0, material_id, thickness: None, mesh_size: None })).await;
    let region_id = app.snapshot().expect("snapshot").regions[0].id.clone();
    dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec![region_id] })).await;
    assert!(app.snapshot().expect("snapshot").regions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn remove_selection_with_no_matching_ids_is_a_no_op_2d() {
    let mut app = fem2d_empty_app();
    let result = dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids: vec!["missing".into()] })).await;
    assert!(result.mutations.is_empty());
}
