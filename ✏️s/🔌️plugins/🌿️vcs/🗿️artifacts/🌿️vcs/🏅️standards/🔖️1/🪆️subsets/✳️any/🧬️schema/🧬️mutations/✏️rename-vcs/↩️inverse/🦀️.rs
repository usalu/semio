//! ↩️ Inverse for `RenameVcs` — the OLD title looked up from BASE, never a captured value.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::RenameVcs, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::rename_vcs(base.title.clone())]

    })())
}
//#endregion 🔖️Inverse
