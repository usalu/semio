//! 🔺️ Sparse diff builder for `ReplaceMaterial`.
use super::mutation::ReplaceMaterial;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dMaterialsDelta, Fem2dMaterialsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMaterial, _base: &Fem2dSnapshot) -> Fem2dDiff {
    Fem2dDiff { materials: Some(Fem2dMaterialsDelta { patched: vec![Fem2dMaterialsPatchEntry { id: payload.id.clone(), item: payload.new_material.clone() }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
