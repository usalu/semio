//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
use super::mutation::DeleteLoadCase;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteLoadCase, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.load_cases.iter().any(|case| case.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
