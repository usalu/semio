//! 🔺️ Sparse diff builder for `ReplaceRegion`.
use super::mutation::ReplaceRegion;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta, Fem2dRegionsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceRegion, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { regions: Some(Fem2dRegionsDelta { patched: vec![Fem2dRegionsPatchEntry { id: payload.id.clone(), item: payload.new_region.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
