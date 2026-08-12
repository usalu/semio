//! 🔺️ Sparse diff builder for `CreateElement`.
use super::mutation::CreateElement;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateElement, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { elements: Some(Fem3dElementsDelta { added: vec![(*payload.element).clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
