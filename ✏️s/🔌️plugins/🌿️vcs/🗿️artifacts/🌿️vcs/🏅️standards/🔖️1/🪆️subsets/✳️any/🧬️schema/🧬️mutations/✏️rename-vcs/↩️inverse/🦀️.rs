//! ↩️ Inverse for `RenameVcs` — the OLD title looked up from BASE, never a captured value.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::RenameVcs, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    vec![super::rename_vcs(base.title.clone())]
}
//#endregion 🔖️Inverse
