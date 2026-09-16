//! 🧮️ Fem2d play app commands command — `set-analysis-settings`.
//!
//! The staged form sends the typed triple; a persistent `input(Number)` in the results panel sends
//! `{field, value}`, because the host merges a control's own scalar under the single key `value`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use crate::FemAnalysisSettings;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetAnalysisSettings
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-analysis-settings")]
pub struct SetAnalysisSettings {
    pub modal_count: Option<u32>,
    pub buckling_count: Option<u32>,
    pub deformation_scale: Option<f64>,
    pub field: Option<String>,
    pub value: Option<String>,
}

/// 🧮️ Applies ONE named analysis field, leaving the rest of the settings alone.
fn apply_field(settings: &mut FemAnalysisSettings, field: &str, value: &str) -> Result<(), Fault> {
    let number = value.trim().parse::<f64>().map_err(|_| Fault::from(format!("fem2d.analysis.value: '{value}' is not a number")))?;
    match field {
        "modalCount" => settings.modal_count = number.max(0.0) as usize,
        "bucklingCount" => settings.buckling_count = number.max(0.0) as usize,
        "deformationScale" => settings.deformation_scale = number,
        other => return Err(Fault::from(format!("fem2d.analysis.field: '{other}' is not an analysis field"))),
    }
    Ok(())
}

pub fn handle(payload: &SetAnalysisSettings, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let current = &doc.snapshot.analysis;
    let mut settings = FemAnalysisSettings {
        modal_count: payload.modal_count.map_or(current.modal_count, |value| value as usize),
        buckling_count: payload.buckling_count.map_or(current.buckling_count, |value| value as usize),
        deformation_scale: payload.deformation_scale.unwrap_or(current.deformation_scale),
    };
    if let Some(field) = payload.field.as_deref().filter(|field| !field.is_empty()) {
        apply_field(&mut settings, field, payload.value.as_deref().unwrap_or_default())?;
    }
    Ok(Emit::mutations(vec![Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings })]))
}
//#endregion 🔖️SetAnalysisSettings

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
