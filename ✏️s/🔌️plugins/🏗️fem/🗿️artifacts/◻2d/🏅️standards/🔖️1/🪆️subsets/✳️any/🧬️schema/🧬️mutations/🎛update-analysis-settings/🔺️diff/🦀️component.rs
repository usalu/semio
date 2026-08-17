//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
use super::mutation::UpdateAnalysisSettings;
use crate::artifacts::fem2d::diff::Fem2dDiff;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateAnalysisSettings, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if payload.settings == base.analysis {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Analysis settings are unchanged.".to_string());
    }
    protocol::MutationOutcome::new(Fem2dDiff { analysis: Some(payload.settings.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
