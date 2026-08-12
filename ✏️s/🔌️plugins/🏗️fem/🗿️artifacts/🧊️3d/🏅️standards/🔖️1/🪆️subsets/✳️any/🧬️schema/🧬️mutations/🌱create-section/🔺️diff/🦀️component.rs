//! 🔺️ Sparse diff builder for `CreateSection`.
use super::mutation::CreateSection;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSection, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { sections: Some(Fem3dSectionsDelta { added: vec![payload.section.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
