//! ↩️ Inverse for `AddLoad` — a `remove-load` of the just-added load id, only if the case existed.
use super::mutation::AddLoad;
use crate::artifacts::fem3d::load_id;
use crate::artifacts::fem3d::mutations::{remove_load, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &AddLoad, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    if base.load_cases.iter().any(|case| case.id == payload.case_id) {
        vec![Fem3dMutation::RemoveLoad(remove_load::mutation::RemoveLoad { case_id: payload.case_id.clone(), load_id: load_id(&payload.load).to_string() })]
    } else {
        Vec::new()
    }
}
//#endregion 🔖️Inverse
