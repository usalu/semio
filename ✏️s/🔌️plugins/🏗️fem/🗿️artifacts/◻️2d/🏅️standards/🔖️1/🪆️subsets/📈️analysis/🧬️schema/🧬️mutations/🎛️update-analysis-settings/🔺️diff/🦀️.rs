//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
//!
//! Guards, in the order they run: the shared `guards::analysis_bounds` (`mutation.invariant`,
//! Fatal) — at least one modal and one buckling mode, and a real positive deformation scale — then
//! `mutation.no-op` when the settings are already what the payload asks for. The bounds run FIRST
//! so a document already holding out-of-range settings cannot launder them through as a no-op.
use super::UpdateAnalysisSettings;
use crate::diff::Fem2dDiff;
use crate::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateAnalysisSettings, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if let Some(rejection) = guards::analysis_bounds(&payload.settings) {
        return rejection;
    }
    if payload.settings == base.analysis {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Analysis settings are unchanged.".to_string());
    }
    protocol::MutationOutcome::new(Fem2dDiff { analysis: Some(payload.settings.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
