//! 🧮️ Fem3d play app command — `set-analysis-settings`: a partial update of the document's analysis
//! settings. The staged form speaks the TYPED optional fields; a results-panel control speaks
//! `{field, value}` because the host merges a control's own scalar under `value`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use crate::{Fem3dSnapshot, FemAnalysisSettings};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-analysis-settings")]
pub struct SetAnalysisSettings {
    pub modal_count: Option<u32>,
    pub buckling_count: Option<u32>,
    pub deformation_scale: Option<f64>,
    pub field: Option<String>,
    pub value: Option<String>,
    /// 🪟️ The results window whose Analysis section authored this gesture. Analysis settings are
    /// DOCUMENT state, so nothing here is window-scoped — the tag is carried only so the one
    /// `{field, value}` convention is spelled identically on all three panel commands.
    pub window_id: Option<String>,
}

/// 🧮️ Applies ONE named analysis field, leaving the rest of the settings alone.
fn apply_field(settings: &mut FemAnalysisSettings, field: &str, value: &str) -> Result<(), Fault> {
    let number = value.trim().parse::<f64>().map_err(|_| Fault::from(format!("fem3d.analysis.value: '{value}' is not a number")))?;
    match field {
        "modalCount" => settings.modal_count = number.max(0.0) as usize,
        "bucklingCount" => settings.buckling_count = number.max(0.0) as usize,
        "deformationScale" => settings.deformation_scale = number,
        other => return Err(Fault::from(format!("fem3d.analysis.field: '{other}' is not an analysis field"))),
    }
    Ok(())
}

/// ⚙️ Every typed field is optional and defaults to the document's current setting when omitted — a
/// partial update, not a whole-record replace.
pub fn handle(payload: &SetAnalysisSettings, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let current = &doc.snapshot.analysis;
    let mut settings = FemAnalysisSettings {
        modal_count: payload.modal_count.map_or(current.modal_count, |value| value as usize),
        buckling_count: payload.buckling_count.map_or(current.buckling_count, |value| value as usize),
        deformation_scale: payload.deformation_scale.unwrap_or(current.deformation_scale),
    };
    if let Some(field) = payload.field.as_deref().filter(|field| !field.is_empty()) {
        apply_field(&mut settings, field, payload.value.as_deref().unwrap_or_default())?;
    }
    if settings == *current {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings })]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
