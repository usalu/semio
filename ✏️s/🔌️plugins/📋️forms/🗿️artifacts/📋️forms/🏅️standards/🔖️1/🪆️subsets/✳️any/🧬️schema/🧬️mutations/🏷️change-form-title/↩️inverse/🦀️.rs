//! ↩️ `change-form-title` — undo restores the BASE-state title. Always has an inverse (the document
//! always has a `title` field, even when `None`).

use super::mutation::ChangeFormTitle;
use crate::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse_change_form_title(_payload: &ChangeFormTitle, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: base.title.clone() })]

    })())
}
//#endregion 🔖️Inverse
