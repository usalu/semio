//! 🔺️ Sparse diff builder for `ReplaceSupport`.
use super::mutation::ReplaceSupport;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta, Fem3dSupportsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSupport, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { supports: Some(Fem3dSupportsDelta { patched: vec![Fem3dSupportsPatchEntry { id: payload.id.clone(), item: payload.new_support.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
