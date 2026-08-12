//! 🔺️ Sparse diff builder for `DeleteMaterial`.
use super::mutation::DeleteMaterial;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dMaterialsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { materials: Some(Fem2dMaterialsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
