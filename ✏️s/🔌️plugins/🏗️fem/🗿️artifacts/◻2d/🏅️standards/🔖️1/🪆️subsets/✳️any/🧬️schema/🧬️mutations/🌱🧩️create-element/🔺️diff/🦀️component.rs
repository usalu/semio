//! 🔺️ Sparse diff builder for `CreateElement`.
use super::mutation::CreateElement;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dElementsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateElement, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { elements: Some(Fem2dElementsDelta { added: vec![(*payload.element).clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
