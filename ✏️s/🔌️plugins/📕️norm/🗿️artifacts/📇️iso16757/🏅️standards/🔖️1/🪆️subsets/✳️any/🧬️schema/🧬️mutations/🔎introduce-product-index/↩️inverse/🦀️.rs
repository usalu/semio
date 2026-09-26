//! Inverse for `introduce-product-index`.

use crate::mutations::retire_product_index;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::IntroduceProductIndex;

pub fn inverse(payload: &IntroduceProductIndex, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.product_indexes.iter().any(|item| item.id == payload.product_index.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetireProductIndex(retire_product_index::mutation::RetireProductIndex { id: payload.product_index.id.clone() })]
}
