//! ↩️ Inverse for `ChangeStatus` — the OLD status value looked up from BASE.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeStatus, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    vec![super::change_status(base.status.clone())]
}
//#endregion 🔖️Inverse
