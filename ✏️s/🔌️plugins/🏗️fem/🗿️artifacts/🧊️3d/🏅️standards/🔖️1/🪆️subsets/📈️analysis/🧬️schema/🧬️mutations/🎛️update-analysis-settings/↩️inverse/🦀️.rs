//! ↩️ Inverse for `UpdateAnalysisSettings` — recovers the pre-mutation settings from `base`.
use super::UpdateAnalysisSettings;
use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &UpdateAnalysisSettings, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    let _ = payload;
    vec![Fem3dMutation::UpdateAnalysisSettings(UpdateAnalysisSettings { settings: base.analysis.clone() })]
}
//#endregion 🔖️Inverse
