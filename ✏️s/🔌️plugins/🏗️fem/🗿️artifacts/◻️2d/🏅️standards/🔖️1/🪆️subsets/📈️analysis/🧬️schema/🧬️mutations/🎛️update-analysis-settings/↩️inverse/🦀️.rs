//! ↩️ Inverse for `UpdateAnalysisSettings` — recovers the pre-mutation settings from `base`.
use super::UpdateAnalysisSettings;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &UpdateAnalysisSettings, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    let _ = payload;
    vec![Fem2dMutation::UpdateAnalysisSettings(UpdateAnalysisSettings { settings: base.analysis.clone() })]
}
//#endregion 🔖️Inverse
