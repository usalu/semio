//! 🔺️ Sparse diff builder for `DeleteCombination`.
use super::mutation::DeleteCombination;
use crate::artifacts::fem3d::diff::{Fem3dCombinationsDelta, Fem3dDiff};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteCombination, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
