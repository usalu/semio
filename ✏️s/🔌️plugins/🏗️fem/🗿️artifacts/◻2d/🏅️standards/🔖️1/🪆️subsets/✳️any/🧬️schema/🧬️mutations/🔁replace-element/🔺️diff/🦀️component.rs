//! 🔺️ Sparse diff builder for `ReplaceElement`.
use super::mutation::ReplaceElement;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dElementsDelta, Fem2dElementsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceElement, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { elements: Some(Fem2dElementsDelta { patched: vec![Fem2dElementsPatchEntry { id: payload.id.clone(), item: (*payload.new_element).clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
