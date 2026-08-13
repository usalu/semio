//! 🧮️ 🧮️ FEM 3D app commands command — `set-analysis-settings`.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::mutations::update_analysis_settings;
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "set-analysis-settings")]
pub struct SetAnalysisSettings {
    pub modal_count: Option<u32>,
    pub buckling_count: Option<u32>,
    pub deformation_scale: Option<f64>,
}

/// ⚙️ Every field is optional and defaults to the document's current setting when omitted — a
/// partial update, not a whole-record replace.
pub fn handle(payload: &SetAnalysisSettings, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let current = &doc.snapshot.analysis;
    let settings = crate::artifacts::fem3d::FemAnalysisSettings {
        modal_count: payload.modal_count.map(|value| value as usize).unwrap_or(current.modal_count),
        buckling_count: payload.buckling_count.map(|value| value as usize).unwrap_or(current.buckling_count),
        deformation_scale: payload.deformation_scale.unwrap_or(current.deformation_scale),
    };
    Ok(Emit::mutations(vec![Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::mutation::UpdateAnalysisSettings { settings })]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app};
    use crate::apps::fem3d::Fem3dCommand;

    #[test]
    fn set_analysis_settings_partially_updates_and_keeps_the_rest() {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetAnalysisSettings(SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: None }));
        let analysis = &app.snapshot().expect("snapshot").analysis;
        assert_eq!(analysis.modal_count, 5);
        assert_eq!(analysis.buckling_count, 3);
        assert_eq!(analysis.deformation_scale, 50.0);
    }
}
