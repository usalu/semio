//! ↩️ Inverse for `ChangeCounter` — the OLD counter value looked up from BASE.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeCounter, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    vec![super::change_counter(base.counter)]
}
//#endregion 🔖️Inverse
