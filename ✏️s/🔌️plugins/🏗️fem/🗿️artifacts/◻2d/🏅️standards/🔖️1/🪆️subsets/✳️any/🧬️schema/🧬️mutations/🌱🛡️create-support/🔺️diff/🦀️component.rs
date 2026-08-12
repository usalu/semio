//! 🔺️ Sparse diff builder for `CreateSupport`.
use super::mutation::CreateSupport;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { supports: Some(Fem2dSupportsDelta { added: vec![payload.support.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
