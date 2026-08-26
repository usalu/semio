//! ↩️ Inverse for `ChangeStatus` — the OLD status value looked up from BASE.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeStatus, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    vec![super::mutation::change_status(base.status.clone())]
}
//#endregion 🔖️Inverse
