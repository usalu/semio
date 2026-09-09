//! 🧮️ 🧮️ FEM 3D app commands command — `set-analysis-settings`.

use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use crate::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
    let settings = crate::FemAnalysisSettings {
        modal_count: payload.modal_count.map_or(current.modal_count, |value| value as usize),
        buckling_count: payload.buckling_count.map_or(current.buckling_count, |value| value as usize),
        deformation_scale: payload.deformation_scale.unwrap_or(current.deformation_scale),
    };
    Ok(Emit::mutations(vec![Fem3dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings })]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
