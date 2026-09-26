//! ↩️ `introduce-product-group` — undo is `retire-product-group`, unless `base` already had this id
//! (then `create` was a no-op and there's nothing to undo).

use crate::mutations::retire_product_group;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::IntroduceProductGroup;

//#region 🔖️Inverse
pub fn inverse(payload: &IntroduceProductGroup, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.product_groups.iter().any(|group| group.id == payload.product_group.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetireProductGroup(retire_product_group::mutation::RetireProductGroup { id: payload.product_group.id.clone() })]
}
//#endregion 🔖️Inverse
