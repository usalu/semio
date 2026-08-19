//! ↩️ Inverse for `ChangeNotes` — the OLD notes value looked up from BASE.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::VcsSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangeNotes, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    vec![super::mutation::change_notes(base.notes.clone())]
}
//#endregion 🔖️Inverse
