//! 🧮️ 🧮️ Fem2d play app commands command — `set-analysis-settings`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use crate::FemAnalysisSettings;
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

pub fn handle(payload: &SetAnalysisSettings, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
