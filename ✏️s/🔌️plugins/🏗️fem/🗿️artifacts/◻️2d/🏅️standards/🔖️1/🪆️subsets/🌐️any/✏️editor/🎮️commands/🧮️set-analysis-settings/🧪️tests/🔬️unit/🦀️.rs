use super::*;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app};
use crate::editor::fem2d::Fem2dCommand;

#[semio_framework_async_macros::async_test]
async fn set_analysis_settings_partial_args_keep_current_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetAnalysisSettings(SetAnalysisSettings { modal_count: Some(4), buckling_count: Some(6), deformation_scale: Some(50.0) })).await;
    dispatch(&mut app, Fem2dCommand::SetAnalysisSettings(SetAnalysisSettings { modal_count: None, buckling_count: None, deformation_scale: Some(300.0) })).await;
    let settings = app.snapshot().expect("snapshot").analysis.clone();
    assert_eq!(settings.modal_count, 4);
    assert_eq!(settings.buckling_count, 6);
    assert_eq!(settings.deformation_scale, 300.0);
}
