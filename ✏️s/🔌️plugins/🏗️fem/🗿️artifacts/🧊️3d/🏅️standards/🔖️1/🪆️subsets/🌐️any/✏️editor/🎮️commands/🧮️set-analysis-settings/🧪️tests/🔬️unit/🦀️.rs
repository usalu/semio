use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_analysis_settings::*;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::dispatch;
use crate::context::fem3d_empty_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_analysis_settings_partially_updates_and_keeps_the_rest() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::SetAnalysisSettings(SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: None, field: None, value: None, window_id: None })).await;
    let analysis = &app.snapshot().expect("snapshot").analysis;
    assert_eq!(analysis.modal_count, 5);
    assert_eq!(analysis.buckling_count, 3);
    assert_eq!(analysis.deformation_scale, 50.0);
}
