//! Inverse for `introduce-product-class`.

use crate::mutations::retire_product_class;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::IntroduceProductClass;

pub fn inverse(payload: &IntroduceProductClass, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.product_classes.iter().any(|item| item.id == payload.product_class.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetireProductClass(retire_product_class::mutation::RetireProductClass { id: payload.product_class.id.clone() })]
}
