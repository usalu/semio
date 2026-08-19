//! ↩️ Inverse for `ChangeLoadCaseSelfWeight` — recovers the pre-mutation flag from `base`.
use super::mutation::ChangeLoadCaseSelfWeight;
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeLoadCaseSelfWeight, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.load_cases
        .iter()
        .find(|case| case.id == payload.case_id)
        .map(|case| vec![Fem2dMutation::ChangeLoadCaseSelfWeight(ChangeLoadCaseSelfWeight { case_id: payload.case_id.clone(), new_self_weight: case.self_weight })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
