//! 🔺️ Sparse diff builder for `ReplaceElement`.
use super::mutation::ReplaceElement;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta, Fem3dElementsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceElement, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { elements: Some(Fem3dElementsDelta { patched: vec![Fem3dElementsPatchEntry { id: payload.id.clone(), item: (*payload.new_element).clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
