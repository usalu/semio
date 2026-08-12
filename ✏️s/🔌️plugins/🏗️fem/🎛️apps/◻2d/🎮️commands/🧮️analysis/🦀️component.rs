//! 🧮️ Fem2d play app commands — analysis settings (modal/buckling mode counts, deformation scale).

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::mutations::update_analysis_settings;
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::FemAnalysisSettings;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetAnalysisSettings
pub mod set_analysis_settings {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-analysis-settings")]
    pub struct SetAnalysisSettings {
        pub modal_count: Option<u32>,
        pub buckling_count: Option<u32>,
        pub deformation_scale: Option<f64>,
    }

    pub fn handle(payload: &SetAnalysisSettings, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
        let current = &doc.snapshot.analysis;
        let settings = FemAnalysisSettings {
            modal_count: payload.modal_count.map(|value| value as usize).unwrap_or(current.modal_count),
            buckling_count: payload.buckling_count.map(|value| value as usize).unwrap_or(current.buckling_count),
            deformation_scale: payload.deformation_scale.unwrap_or(current.deformation_scale),
        };
        Ok(Emit::mutations(vec![Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings })]))
    }
}
//#endregion 🔖️SetAnalysisSettings

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn set_analysis_settings_partial_args_keep_current_2d() {
        let mut app = fem2d_app();
        dispatch(
            &mut app,
            Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(4), buckling_count: Some(6), deformation_scale: Some(50.0) }),
        );
        dispatch(&mut app, Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: None, buckling_count: None, deformation_scale: Some(300.0) }));
        let settings = app.snapshot().expect("snapshot").analysis.clone();
        assert_eq!(settings.modal_count, 4);
        assert_eq!(settings.buckling_count, 6);
        assert_eq!(settings.deformation_scale, 300.0);
    }
}
//#endregion 🧪️Tests
