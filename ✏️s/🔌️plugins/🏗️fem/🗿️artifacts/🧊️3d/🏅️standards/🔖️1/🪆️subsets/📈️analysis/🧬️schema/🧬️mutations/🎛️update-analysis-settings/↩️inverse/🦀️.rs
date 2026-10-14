//! ↩️ Inverse for `UpdateAnalysisSettings` — recovers the pre-mutation settings from `base`.
use super::UpdateAnalysisSettings;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<update_analysis_settings::UpdateAnalysisSettings>>(payload: &UpdateAnalysisSettings, base: &Fem3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = payload;
    vec![Op::from(UpdateAnalysisSettings { settings: base.analysis.clone() })]

    })())
}
//#endregion 🔖️Inverse
