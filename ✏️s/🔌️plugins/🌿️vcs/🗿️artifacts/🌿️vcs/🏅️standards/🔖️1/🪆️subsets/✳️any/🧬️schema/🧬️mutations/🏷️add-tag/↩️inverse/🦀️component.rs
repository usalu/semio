//! ↩️ Inverse for `AddTag` — `remove-tag` if BASE didn't already have it, else nothing to undo.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::VcsSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::AddTag, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    if base.tags.iter().any(|existing| existing == &payload.tag) {
        Vec::new()
    } else {
        vec![super::super::remove_tag::mutation::remove_tag(payload.tag.clone())]
    }
}
//#endregion 🔖️Inverse
