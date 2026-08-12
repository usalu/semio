//! 🔺️ Sparse diff builder for `CreateSolid`.
use super::mutation::CreateSolid;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSolid, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { solids: Some(Fem3dSolidsDelta { added: vec![payload.solid.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
