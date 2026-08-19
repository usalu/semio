//! ↩️ Inverse for `UpdateAnalysisSettings` — recovers the pre-mutation settings from `base`.
use super::mutation::UpdateAnalysisSettings;
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &UpdateAnalysisSettings, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    let _ = payload;
    vec![Fem2dMutation::UpdateAnalysisSettings(UpdateAnalysisSettings { settings: base.analysis.clone() })]
}
//#endregion 🔖️Inverse
