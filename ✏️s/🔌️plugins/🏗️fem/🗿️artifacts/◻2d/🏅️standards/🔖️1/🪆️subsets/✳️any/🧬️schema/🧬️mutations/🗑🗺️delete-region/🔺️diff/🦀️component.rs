//! 🔺️ Sparse diff builder for `DeleteRegion`.
use super::mutation::DeleteRegion;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteRegion, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { regions: Some(Fem2dRegionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
