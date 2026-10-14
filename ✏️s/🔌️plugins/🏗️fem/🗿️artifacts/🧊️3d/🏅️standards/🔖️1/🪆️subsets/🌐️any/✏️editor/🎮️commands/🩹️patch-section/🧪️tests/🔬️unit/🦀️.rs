use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_section::*;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::replace_section;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem3d_demo_app};
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchSection) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_section_edits_one_property_on_the_live_demo_3d() {
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::PatchSection(PatchSection { id: "hea200".into(), field: "j".into(), value: "0.0000012".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let section = snapshot.sections.iter().find(|section| section.id == "hea200").expect("hea200");
    assert_eq!((section.j, section.area), (0.0000012, 0.00538));
    assert!(emit(&snapshot, PatchSection { id: "hea200".into(), field: "iy".into(), value: "wide".into() }).is_err());
    assert!(emit(&snapshot, PatchSection { id: "hea200".into(), field: "width".into(), value: "1".into() }).is_err());
    assert!(emit(&snapshot, PatchSection { id: "ghost".into(), field: "area".into(), value: "1".into() }).is_err());
}
