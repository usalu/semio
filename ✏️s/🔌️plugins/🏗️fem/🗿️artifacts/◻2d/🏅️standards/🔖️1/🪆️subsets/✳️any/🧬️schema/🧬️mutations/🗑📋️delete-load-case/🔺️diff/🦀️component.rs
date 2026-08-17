//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
use super::mutation::DeleteLoadCase;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLoadCase, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.load_cases.iter().any(|case| case.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
