//! 🔺️ Sparse diff builder for `CreateLoadCase`.
use super::mutation::CreateLoadCase;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { added: vec![payload.load_case.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
