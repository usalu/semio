//! 🔺️ Sparse diff builder for `ReplaceSection`.
use super::mutation::ReplaceSection;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta, Fem3dSectionsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSection, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { sections: Some(Fem3dSectionsDelta { patched: vec![Fem3dSectionsPatchEntry { id: payload.id.clone(), item: payload.new_section.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
