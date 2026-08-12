//! 🔺️ Sparse diff builder for `CreateSupport`.
use super::mutation::CreateSupport;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { supports: Some(Fem3dSupportsDelta { added: vec![payload.support.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
