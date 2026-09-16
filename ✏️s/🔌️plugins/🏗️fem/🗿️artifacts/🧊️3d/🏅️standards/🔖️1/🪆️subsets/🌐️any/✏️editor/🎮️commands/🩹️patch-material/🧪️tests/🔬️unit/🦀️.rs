use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app};
use crate::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchMaterial) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_material_edits_one_property_on_the_live_demo_3d() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::PatchMaterial(PatchMaterial { id: "concrete".into(), field: "nu".into(), value: "0.25".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchMaterial(PatchMaterial { id: "concrete".into(), field: "name".into(), value: "C35/45".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let material = snapshot.materials.iter().find(|material| material.id == "concrete").expect("concrete");
    assert_eq!((material.nu, material.name.as_str(), material.e), (0.25, "C35/45", 33e9));
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "e".into(), value: "soft".into() }).is_err());
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "colour".into(), value: "grey".into() }).is_err());
    assert!(emit(&snapshot, PatchMaterial { id: "concrete".into(), field: "nu".into(), value: "0.25".into() }).expect("handle").artifact_mutations.is_empty());
}
