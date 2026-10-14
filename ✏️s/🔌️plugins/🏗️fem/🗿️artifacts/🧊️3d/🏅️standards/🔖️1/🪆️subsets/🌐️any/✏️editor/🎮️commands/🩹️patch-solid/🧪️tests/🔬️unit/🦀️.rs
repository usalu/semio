use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_solid::*;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::replace_solid;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemAxis;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem3d_demo_app};
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSolid) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_solid_edits_extrusion_axis_and_material_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchSolid(PatchSolid { id: "sol1".into(), field: "height".into(), value: "0.8".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchSolid(PatchSolid { id: "sol1".into(), field: "axis".into(), value: "y".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchSolid(PatchSolid { id: "sol1".into(), field: "layers".into(), value: "3".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let solid = snapshot.solids.iter().find(|solid| solid.id == "sol1").expect("sol1");
    assert_eq!((solid.height, solid.axis, solid.layers), (0.8, FemAxis::Y, 3));
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "axis".into(), value: "w".into() }).is_err());
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "meshSize".into(), value: "coarse".into() }).is_err());
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "colour".into(), value: "grey".into() }).is_err());
    assert!(emit(&snapshot, PatchSolid { id: "sol1".into(), field: "height".into(), value: "0.8".into() }).expect("handle").artifact_mutations.is_empty());
}
