//! 🔺️ Sparse diff builder for `DeleteSupport`.
use super::mutation::DeleteSupport;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSupport, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { supports: Some(Fem2dSupportsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
