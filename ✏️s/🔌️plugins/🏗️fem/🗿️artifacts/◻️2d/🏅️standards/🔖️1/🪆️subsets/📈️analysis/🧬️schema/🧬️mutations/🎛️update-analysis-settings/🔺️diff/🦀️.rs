//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
//!
//! Guards, in the order they run: the shared `guards::analysis_bounds` (`mutation.invariant`,
//! Fatal) — at least one modal and one buckling mode, and a real positive deformation scale — then
//! `mutation.no-op` when the settings are already what the payload asks for. The bounds run FIRST
//! so a document already holding out-of-range settings cannot launder them through as a no-op.
use super::UpdateAnalysisSettings;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dAnalysisPatch, Fem2dDiff};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateAnalysisSettings, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if let Some(rejection) = guards::analysis_bounds(&payload.settings) {
        return rejection;
    }
    if payload.settings == base.analysis {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Analysis settings are unchanged.".to_string());
    }
    let settings = &payload.settings;
    let patch = Fem2dAnalysisPatch {
        modal_count: (settings.modal_count != base.analysis.modal_count).then_some(settings.modal_count),
        buckling_count: (settings.buckling_count != base.analysis.buckling_count).then_some(settings.buckling_count),
        deformation_scale: (settings.deformation_scale != base.analysis.deformation_scale).then_some(settings.deformation_scale),
    };
    protocol::MutationOutcome::new(Fem2dDiff { analysis: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
