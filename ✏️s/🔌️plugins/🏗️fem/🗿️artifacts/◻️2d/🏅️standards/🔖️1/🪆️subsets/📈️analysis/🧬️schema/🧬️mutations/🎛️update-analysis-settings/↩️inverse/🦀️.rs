//! ↩️ Inverse for `UpdateAnalysisSettings` — recovers the pre-mutation settings from `base`.
use super::UpdateAnalysisSettings;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &UpdateAnalysisSettings, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = payload;
    vec![Fem2dMutation::UpdateAnalysisSettings(UpdateAnalysisSettings { settings: base.analysis.clone() })]

    })())
}
//#endregion 🔖️Inverse
