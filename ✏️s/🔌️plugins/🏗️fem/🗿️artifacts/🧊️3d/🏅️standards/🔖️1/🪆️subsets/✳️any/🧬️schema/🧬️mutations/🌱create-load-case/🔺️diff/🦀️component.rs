//! 🔺️ Sparse diff builder for `CreateLoadCase`.
use super::mutation::CreateLoadCase;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { added: vec![payload.load_case.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
