//! ↩️ Inverse for `RemoveLoad` — recreates the captured load (via `add-load`) from `base`.
use super::RemoveLoad;
use crate::artifacts::fem2d::load_id;
use crate::artifacts::fem2d::mutations::{add_load, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveLoad, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.load_cases
        .iter()
        .find(|case| case.id == payload.case_id)
        .and_then(|case| case.loads.iter().find(|load| load_id(load) == payload.load_id).cloned())
        .map(|load| vec![Fem2dMutation::AddLoad(add_load::AddLoad { case_id: payload.case_id.clone(), load: Box::new(load) })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
