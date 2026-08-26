//! ↩️ `rename-product-group` — undo restores BASE's preferred name; missing id ⇒ `Vec::new()`.

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::RenameProductGroup;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameProductGroup, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(group) = base.catalogue.product_groups.iter().find(|group| group.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::RenameProductGroup(RenameProductGroup { id: payload.id.clone(), new_name: group.names.preferred.text.clone() })]
}
//#endregion 🔖️Inverse
