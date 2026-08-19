//! ↩️ Inverse for `UpdateAnalysisSettings` — recovers the pre-mutation settings from `base`.
use super::mutation::UpdateAnalysisSettings;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &UpdateAnalysisSettings, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    let _ = payload;
    vec![Fem3dMutation::UpdateAnalysisSettings(UpdateAnalysisSettings { settings: base.analysis.clone() })]
}
//#endregion 🔖️Inverse
