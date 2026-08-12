//! 🔺️ Sparse diff builder for `DeleteSupport`.
use super::mutation::DeleteSupport;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSupport, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { supports: Some(Fem3dSupportsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
