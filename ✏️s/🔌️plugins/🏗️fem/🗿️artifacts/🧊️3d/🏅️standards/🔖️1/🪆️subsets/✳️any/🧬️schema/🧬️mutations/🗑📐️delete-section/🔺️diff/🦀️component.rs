//! 🔺️ Sparse diff builder for `DeleteSection`.
use super::mutation::DeleteSection;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSection, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { sections: Some(Fem3dSectionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
