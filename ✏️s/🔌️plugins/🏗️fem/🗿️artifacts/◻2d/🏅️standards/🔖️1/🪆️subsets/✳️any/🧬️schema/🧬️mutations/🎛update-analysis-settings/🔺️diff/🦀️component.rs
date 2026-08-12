//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
use super::mutation::UpdateAnalysisSettings;
use crate::artifacts::fem2d::diff::Fem2dDiff;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateAnalysisSettings, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { analysis: Some(payload.settings.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
