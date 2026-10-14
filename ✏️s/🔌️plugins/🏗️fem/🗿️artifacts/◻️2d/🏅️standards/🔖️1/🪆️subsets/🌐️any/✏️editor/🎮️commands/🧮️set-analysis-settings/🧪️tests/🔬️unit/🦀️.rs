use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_analysis_settings::*;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::context::{dispatch, fem2d_app};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_analysis_settings_partial_args_keep_current_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetAnalysisSettings(SetAnalysisSettings { modal_count: Some(4), buckling_count: Some(6), deformation_scale: Some(50.0), field: None, value: None, window_id: None })).await;
    dispatch(&mut app, Fem2dCommand::SetAnalysisSettings(SetAnalysisSettings { modal_count: None, buckling_count: None, deformation_scale: Some(300.0), field: None, value: None, window_id: None })).await;
    let settings = app.snapshot().expect("snapshot").analysis.clone();
    assert_eq!(settings.modal_count, 4);
    assert_eq!(settings.buckling_count, 6);
    assert_eq!(settings.deformation_scale, 300.0);
}
