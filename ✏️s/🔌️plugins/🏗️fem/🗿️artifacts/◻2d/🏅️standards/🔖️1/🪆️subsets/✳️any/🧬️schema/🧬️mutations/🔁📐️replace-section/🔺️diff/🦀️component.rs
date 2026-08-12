//! 🔺️ Sparse diff builder for `ReplaceSection`.
use super::mutation::ReplaceSection;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta, Fem2dSectionsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSection, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { sections: Some(Fem2dSectionsDelta { patched: vec![Fem2dSectionsPatchEntry { id: payload.id.clone(), item: payload.new_section.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
