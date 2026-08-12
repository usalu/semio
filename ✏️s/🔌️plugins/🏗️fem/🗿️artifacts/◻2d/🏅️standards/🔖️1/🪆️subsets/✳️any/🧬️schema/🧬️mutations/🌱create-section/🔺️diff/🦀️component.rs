//! 🔺️ Sparse diff builder for `CreateSection`.
use super::mutation::CreateSection;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSection, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { sections: Some(Fem2dSectionsDelta { added: vec![payload.section.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
