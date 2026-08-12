//! 🔺️ Sparse diff builder for `DeleteSection`.
use super::mutation::DeleteSection;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSection, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { sections: Some(Fem2dSectionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
