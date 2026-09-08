//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
use super::UpdateAnalysisSettings;
use crate::diff::Fem3dDiff;
use crate::mutations::{analysis_breach, invariant};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateAnalysisSettings, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if payload.settings == base.analysis {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Analysis settings already have that value.".to_string());
    }
    if let Some(breach) = analysis_breach(&payload.settings) {
        return invariant(breach, Vec::new());
    }
    protocol::MutationOutcome::new(Fem3dDiff { analysis: Some(payload.settings.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
