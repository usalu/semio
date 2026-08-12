//! 🔺️ Sparse diff builder for `DeleteElement`.
use super::mutation::DeleteElement;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dElementsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteElement, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { elements: Some(Fem2dElementsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
