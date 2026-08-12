//! 🔺️ Sparse diff builder for `DeleteSolid`.
use super::mutation::DeleteSolid;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSolid, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { solids: Some(Fem3dSolidsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
