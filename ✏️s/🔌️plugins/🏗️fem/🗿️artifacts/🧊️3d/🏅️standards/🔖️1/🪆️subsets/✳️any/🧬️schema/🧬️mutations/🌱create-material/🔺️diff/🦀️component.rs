//! 🔺️ Sparse diff builder for `CreateMaterial`.
use super::mutation::CreateMaterial;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { materials: Some(Fem3dMaterialsDelta { added: vec![payload.material.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
