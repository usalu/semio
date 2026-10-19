use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_load_case::*;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::dispatch;
use crate::context::fem3d_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

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
