//! 🔺️ Sparse diff builder for `CreateRegion`.
use super::mutation::CreateRegion;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateRegion, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { regions: Some(Fem2dRegionsDelta { added: vec![payload.region.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
