//! 🔺️ Sparse diff builder for `CreateLoadCase`.
use super::CreateLoadCase;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta};
use crate::artifacts::fem3d::mutations::resolve_load;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \"{}\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    for load in &payload.load_case.loads {
        if let Some(refusal) = resolve_load(base, load) {
            return refusal;
        }
    }
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { added: vec![payload.load_case.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
