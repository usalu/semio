use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_material::*;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::replace_material;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem3d_demo_app};
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchMaterial) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_material_edits_one_property_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchMaterial(PatchMaterial { id: "concrete".into(), field: "nu".into(), value: "0.25".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchMaterial(PatchMaterial { id: "concrete".into(), field: "name".into(), value: "C35/45".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let material = snapshot.materials.iter().find(|material| material.id == "concrete").expect("concrete");
    assert_eq!((material.nu, material.name.as_str(), material.e), (0.25, "C35/45", 33e9));
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "e".into(), value: "soft".into() }).is_err());
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "colour".into(), value: "grey".into() }).is_err());
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "nu".into(), value: "0.25".into() }).expect("handle").artifact_mutations.is_empty());
}
