//! ↩️ `change-form-title` — undo restores the BASE-state title. Always has an inverse (the document
//! always has a `title` field, even when `None`).

use super::mutation::ChangeFormTitle;
use crate::artifacts::forms::{FormMutation, FormsSnapshot};

//#region 🔖️Inverse
pub async fn inverse_change_form_title(_payload: &ChangeFormTitle, base: &FormsSnapshot) -> Vec<FormMutation> {
    vec![FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: base.title.clone() })]
}
//#endregion 🔖️Inverse
