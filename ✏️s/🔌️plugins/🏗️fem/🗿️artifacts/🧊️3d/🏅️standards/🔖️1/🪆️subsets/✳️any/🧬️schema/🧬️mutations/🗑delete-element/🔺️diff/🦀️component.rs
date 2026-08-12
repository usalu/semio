//! 🔺️ Sparse diff builder for `DeleteElement`.
use super::mutation::DeleteElement;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteElement, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { elements: Some(Fem3dElementsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
