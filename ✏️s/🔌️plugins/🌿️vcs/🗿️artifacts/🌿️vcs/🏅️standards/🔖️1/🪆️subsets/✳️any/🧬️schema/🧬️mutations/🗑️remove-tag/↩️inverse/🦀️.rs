//! ↩️ Inverse for `RemoveTag` — `add-tag` if BASE had it, else nothing to undo.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveTag, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        vec![super::super::add_tag::add_tag(payload.tag.clone())]
    } else {
        Vec::new()
    }

    })())
}
//#endregion 🔖️Inverse
