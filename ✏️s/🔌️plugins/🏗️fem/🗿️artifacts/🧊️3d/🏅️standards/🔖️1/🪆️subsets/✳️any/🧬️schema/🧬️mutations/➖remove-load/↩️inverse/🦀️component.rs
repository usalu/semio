//! ↩️ Inverse for `RemoveLoad` — recreates the captured load (via `add-load`) from `base`.
use super::mutation::RemoveLoad;
use crate::artifacts::fem3d::load_id;
use crate::artifacts::fem3d::mutations::{add_load, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &RemoveLoad, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.load_cases
        .iter()
        .find(|case| case.id == payload.case_id)
        .and_then(|case| case.loads.iter().find(|load| load_id(load) == payload.load_id).cloned())
        .map(|load| vec![Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: payload.case_id.clone(), load: Box::new(load) })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
