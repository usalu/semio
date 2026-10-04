//! ↩️ Inverse for `ChangeNotes` — the OLD notes value looked up from BASE.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeNotes, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::change_notes(base.notes.clone())]

    })())
}
//#endregion 🔖️Inverse
