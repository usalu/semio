
use super::*;
use crate::editor::fem3d::Fem3dCommand;
use crate::editor::fem3d::testkit::{dispatch, fem3d_empty_app};

#[semio_framework_async_macros::async_test]
async fn set_analysis_settings_partially_updates_and_keeps_the_rest() {
    let mut app = fem3d_empty_app().await;
    dispatch(&mut app, Fem3dCommand::SetAnalysisSettings(SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: None })).await;
    let analysis = &app.snapshot().expect("snapshot").analysis;
    assert_eq!(analysis.modal_count, 5);
    assert_eq!(analysis.buckling_count, 3);
    assert_eq!(analysis.deformation_scale, 50.0);
}
