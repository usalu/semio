//! 🔺️ Sparse diff builder for `DeleteMaterial`.
use super::mutation::DeleteMaterial;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { materials: Some(Fem3dMaterialsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
