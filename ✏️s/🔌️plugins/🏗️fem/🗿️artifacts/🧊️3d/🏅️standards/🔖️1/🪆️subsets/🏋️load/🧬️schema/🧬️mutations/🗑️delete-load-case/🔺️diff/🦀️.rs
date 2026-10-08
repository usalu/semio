//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
use super::DeleteLoadCase;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCaseRemoval};
use crate::standards::v1::subsets::any::schema::mutations::{load_case_referrers,target_referenced};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLoadCase, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.load_cases.iter().position(|case| case.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let referrers = load_case_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Load case", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { removed: vec![Fem3dLoadCaseRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
