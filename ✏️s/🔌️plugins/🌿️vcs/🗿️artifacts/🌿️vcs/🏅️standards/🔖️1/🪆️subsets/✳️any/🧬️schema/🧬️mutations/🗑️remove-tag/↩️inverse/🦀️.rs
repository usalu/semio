//! ↩️ Inverse for `RemoveTag` — `add-tag` at the tag's original index if BASE had it, else nothing to undo.
use crate::mutations::VcsDemoMutation;
use crate::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveTag, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.tags.iter().position(|existing| existing == &payload.tag) {
        Some(at) => vec![super::super::add_tag::add_tag_at(payload.tag.clone(), at as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
