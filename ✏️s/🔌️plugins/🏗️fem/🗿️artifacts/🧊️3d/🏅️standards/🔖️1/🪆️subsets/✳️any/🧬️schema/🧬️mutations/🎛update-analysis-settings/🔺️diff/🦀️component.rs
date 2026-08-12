//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
use super::mutation::UpdateAnalysisSettings;
use crate::artifacts::fem3d::diff::Fem3dDiff;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateAnalysisSettings, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { analysis: Some(payload.settings.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
