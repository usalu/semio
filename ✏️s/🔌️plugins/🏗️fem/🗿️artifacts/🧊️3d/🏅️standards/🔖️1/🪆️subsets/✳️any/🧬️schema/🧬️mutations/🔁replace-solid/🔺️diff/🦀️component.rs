//! 🔺️ Sparse diff builder for `ReplaceSolid`.
use super::mutation::ReplaceSolid;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta, Fem3dSolidsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSolid, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { solids: Some(Fem3dSolidsDelta { patched: vec![Fem3dSolidsPatchEntry { id: payload.id.clone(), item: payload.new_solid.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
