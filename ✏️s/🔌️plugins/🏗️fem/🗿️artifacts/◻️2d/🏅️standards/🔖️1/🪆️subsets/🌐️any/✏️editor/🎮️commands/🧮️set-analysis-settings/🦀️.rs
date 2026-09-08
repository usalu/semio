//! 🧮️ 🧮️ Fem2d play app commands command — `set-analysis-settings`.

use crate::mutations::update_analysis_settings;
use crate::op::Fem2dMutation;
use crate::FemAnalysisSettings;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetAnalysisSettings
//#endregion 🔖️SetAnalysisSettings

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-analysis-settings")]
pub struct SetAnalysisSettings {
    pub modal_count: Option<u32>,
    pub buckling_count: Option<u32>,
    pub deformation_scale: Option<f64>,
}

pub fn handle(payload: &SetAnalysisSettings, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let current = &doc.snapshot.analysis;
    let settings = FemAnalysisSettings {
        modal_count: payload.modal_count.map_or(current.modal_count, |value| value as usize),
        buckling_count: payload.buckling_count.map_or(current.buckling_count, |value| value as usize),
        deformation_scale: payload.deformation_scale.unwrap_or(current.deformation_scale),
    };
    Ok(Emit::mutations(vec![Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{dispatch, fem2d_app};
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
}
//#endregion 🧪️Tests
