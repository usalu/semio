//! ↩️ Inverse for `ChangeLoadCaseSelfWeight` — recovers the pre-mutation flag from `base`.
use super::ChangeLoadCaseSelfWeight;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeLoadCaseSelfWeight, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.load_cases.iter().find(|case| case.id == payload.case_id).map(|case| vec![Fem3dMutation::ChangeLoadCaseSelfWeight(ChangeLoadCaseSelfWeight { case_id: payload.case_id.clone(), new_self_weight: case.self_weight })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
