//! ↩️ Inverse for `ChangeNotes` — the OLD notes value looked up from BASE.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeNotes, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    vec![super::change_notes(base.notes.clone())]
}
//#endregion 🔖️Inverse
