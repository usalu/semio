//! ↩️ `change-form-title` — undo restores the BASE-state title. Always has an inverse (the document
//! always has a `title` field, even when `None`).

use crate::standards::v1::subsets::any::schema::mutations::change_form_title::mutation::ChangeFormTitle;
use crate::FormsSnapshot;

//#region 🔖️Inverse
pub fn inverse_change_form_title<Op>(_payload: &ChangeFormTitle, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_form_title::mutation::ChangeFormTitle>,
{
    Ok((|| {
    vec![Op::from(ChangeFormTitle { new_title: base.title.clone() })]

    })())
}
//#endregion 🔖️Inverse
