//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
use super::UpdateAnalysisSettings;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dAnalysisPatch, Fem3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::{analysis_breach,invariant};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateAnalysisSettings, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if payload.settings == base.analysis {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Analysis settings already have that value.".to_string());
    }
    if let Some(breach) = analysis_breach(&payload.settings) {
        return invariant(breach, Vec::new());
    }
    let settings = &payload.settings;
    let patch = Fem3dAnalysisPatch {
        modal_count: (settings.modal_count != base.analysis.modal_count).then_some(settings.modal_count),
        buckling_count: (settings.buckling_count != base.analysis.buckling_count).then_some(settings.buckling_count),
        deformation_scale: (settings.deformation_scale != base.analysis.deformation_scale).then_some(settings.deformation_scale),
    };
    protocol::MutationOutcome::new(Fem3dDiff { analysis: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
