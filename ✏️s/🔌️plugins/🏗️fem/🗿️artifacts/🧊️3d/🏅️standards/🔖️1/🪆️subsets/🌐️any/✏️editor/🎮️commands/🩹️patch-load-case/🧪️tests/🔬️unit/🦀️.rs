use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app};
use crate::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchLoadCase) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_load_case_renames_and_toggles_self_weight_3d() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::PatchLoadCase(PatchLoadCase { id: "live".into(), field: "name".into(), value: "Imposed".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchLoadCase(PatchLoadCase { id: "live".into(), field: "selfWeight".into(), value: "true".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let live = snapshot.load_cases.iter().find(|case| case.id == "live").expect("live");
    assert_eq!((live.name.as_str(), live.self_weight), ("Imposed", true));
    assert!(matches!(emit(&snapshot, PatchLoadCase { id: "live".into(), field: "name".into(), value: "Imposed".into() }), Ok(emit) if emit.artifact_mutations.is_empty()));
    assert!(emit(&snapshot, PatchLoadCase { id: "live".into(), field: "selfWeight".into(), value: "maybe".into() }).is_err());
    assert!(emit(&snapshot, PatchLoadCase { id: "live".into(), field: "colour".into(), value: "x".into() }).is_err());
    assert!(emit(&snapshot, PatchLoadCase { id: "ghost".into(), field: "name".into(), value: "x".into() }).is_err());
}
