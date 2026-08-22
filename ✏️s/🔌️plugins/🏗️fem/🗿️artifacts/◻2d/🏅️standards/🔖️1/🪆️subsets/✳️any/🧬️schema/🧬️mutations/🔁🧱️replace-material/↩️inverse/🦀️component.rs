//! ↩️ Inverse for `ReplaceMaterial` — recovers the pre-mutation material from `base`.
use super::mutation::ReplaceMaterial;
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceMaterial, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.materials.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::ReplaceMaterial(ReplaceMaterial { id: payload.id.clone(), new_material: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
