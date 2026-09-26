//! Inverse for `retire-product-index`.

use crate::mutations::introduce_product_index;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::RetireProductIndex;

pub fn inverse(payload: &RetireProductIndex, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some((index, item)) = base.catalogue.product_indexes.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::IntroduceProductIndex(introduce_product_index::mutation::IntroduceProductIndex {
        product_index: item.clone(),
        index: Some(index),
    })]
}
