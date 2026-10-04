//! ↩️ Inverse for `ChangeCounter` — the OLD counter value looked up from BASE.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeCounter, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::change_counter(base.counter)]

    })())
}
//#endregion 🔖️Inverse
