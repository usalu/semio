//! ↩️ Inverse for `ChangeCounter` — the OLD counter value looked up from BASE.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeCounter, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    vec![super::change_counter(base.counter)]
}
//#endregion 🔖️Inverse
