//! ↩️ `retire-product-group` — undo re-`create`s the group from BASE state, at its original
//! index; missing id ⇒ `Vec::new()`.

use crate::mutations::introduce_product_group;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::RetireProductGroup;

//#region 🔖️Inverse
pub fn inverse(payload: &RetireProductGroup, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(position) = base.catalogue.product_groups.iter().position(|group| group.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::IntroduceProductGroup(introduce_product_group::mutation::IntroduceProductGroup { product_group: base.catalogue.product_groups[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
