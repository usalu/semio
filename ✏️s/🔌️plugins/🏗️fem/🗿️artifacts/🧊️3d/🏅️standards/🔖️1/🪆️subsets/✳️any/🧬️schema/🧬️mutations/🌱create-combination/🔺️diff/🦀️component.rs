//! 🔺️ Sparse diff builder for `CreateCombination`.
use super::mutation::CreateCombination;
use crate::artifacts::fem3d::diff::{Fem3dCombinationsDelta, Fem3dDiff};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateCombination, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { added: vec![payload.combination.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
