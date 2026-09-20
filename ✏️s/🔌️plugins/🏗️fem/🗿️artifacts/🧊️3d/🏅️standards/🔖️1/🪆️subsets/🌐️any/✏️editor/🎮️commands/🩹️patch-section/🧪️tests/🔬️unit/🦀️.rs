use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_demo_app};
use crate::editor::fem3d::Fem3dCommand;

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
