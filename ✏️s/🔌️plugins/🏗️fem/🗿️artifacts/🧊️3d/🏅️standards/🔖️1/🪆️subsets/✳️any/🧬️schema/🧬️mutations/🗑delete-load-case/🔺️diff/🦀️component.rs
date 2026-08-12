//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
use super::mutation::DeleteLoadCase;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLoadCase, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
