//! 🔺️ Sparse diff builder for `ReplaceSupport`.
use super::mutation::ReplaceSupport;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta, Fem2dSupportsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSupport, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { supports: Some(Fem2dSupportsDelta { patched: vec![Fem2dSupportsPatchEntry { id: payload.id.clone(), item: payload.new_support.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
