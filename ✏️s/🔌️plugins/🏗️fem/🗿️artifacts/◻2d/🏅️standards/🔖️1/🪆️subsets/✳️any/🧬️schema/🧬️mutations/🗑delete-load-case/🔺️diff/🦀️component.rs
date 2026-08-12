//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
use super::mutation::DeleteLoadCase;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLoadCase, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
