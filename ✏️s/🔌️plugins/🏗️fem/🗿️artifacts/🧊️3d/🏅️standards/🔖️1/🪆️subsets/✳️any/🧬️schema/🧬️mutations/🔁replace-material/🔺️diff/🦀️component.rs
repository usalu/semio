//! 🔺️ Sparse diff builder for `ReplaceMaterial`.
use super::mutation::ReplaceMaterial;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta, Fem3dMaterialsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMaterial, _base: &Fem3dSnapshot) -> Fem3dDiff {
    Fem3dDiff { materials: Some(Fem3dMaterialsDelta { patched: vec![Fem3dMaterialsPatchEntry { id: payload.id.clone(), item: payload.new_material.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
