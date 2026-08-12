//! 🔺️ Sparse diff builder for `DeleteCombination`.
use super::mutation::DeleteCombination;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dCombinationsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteCombination, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
