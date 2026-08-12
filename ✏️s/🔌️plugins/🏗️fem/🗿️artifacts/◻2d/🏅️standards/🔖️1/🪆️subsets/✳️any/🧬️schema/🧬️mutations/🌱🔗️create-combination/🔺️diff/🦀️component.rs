//! 🔺️ Sparse diff builder for `CreateCombination`.
use super::mutation::CreateCombination;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dCombinationsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateCombination, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { added: vec![payload.combination.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
