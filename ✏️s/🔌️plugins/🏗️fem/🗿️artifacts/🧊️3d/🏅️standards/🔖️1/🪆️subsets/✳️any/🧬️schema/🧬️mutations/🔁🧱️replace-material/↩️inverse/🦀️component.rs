//! ↩️ Inverse for `ReplaceMaterial` — recovers the pre-mutation material from `base`.
use super::mutation::ReplaceMaterial;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceMaterial, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::ReplaceMaterial(ReplaceMaterial { id: payload.id.clone(), new_material: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
