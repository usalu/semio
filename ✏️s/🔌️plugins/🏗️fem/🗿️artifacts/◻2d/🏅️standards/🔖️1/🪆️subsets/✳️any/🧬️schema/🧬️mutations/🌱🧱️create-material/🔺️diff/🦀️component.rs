//! 🔺️ Sparse diff builder for `CreateMaterial`.
use super::mutation::CreateMaterial;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dMaterialsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMaterial, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { materials: Some(Fem2dMaterialsDelta { added: vec![payload.material.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
