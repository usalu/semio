//! ↩️ `create-product-group` — undo is `delete-product-group`, unless `base` already had this id
//! (then `create` was a no-op and there's nothing to undo).

use crate::artifacts::iso16757::mutations::delete_product_group;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::CreateProductGroup;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateProductGroup, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.product_groups.iter().any(|group| group.id == payload.product_group.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::DeleteProductGroup(delete_product_group::mutation::DeleteProductGroup { id: payload.product_group.id.clone() })]
}
//#endregion 🔖️Inverse
