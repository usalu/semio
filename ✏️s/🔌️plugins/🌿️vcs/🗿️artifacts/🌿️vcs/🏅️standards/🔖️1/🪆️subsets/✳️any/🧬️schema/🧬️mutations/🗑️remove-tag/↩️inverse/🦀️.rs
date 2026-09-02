//! ↩️ Inverse for `RemoveTag` — `add-tag` if BASE had it, else nothing to undo.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::VcsSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveTag, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        vec![super::super::add_tag::add_tag(payload.tag.clone())]
    } else {
        Vec::new()
    }
}
//#endregion 🔖️Inverse
