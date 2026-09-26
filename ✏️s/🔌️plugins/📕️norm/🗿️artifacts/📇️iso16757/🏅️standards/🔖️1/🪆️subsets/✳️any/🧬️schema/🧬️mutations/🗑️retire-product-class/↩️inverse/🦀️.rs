//! Inverse for `retire-product-class`.

use crate::mutations::introduce_product_class;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::RetireProductClass;

pub fn inverse(payload: &RetireProductClass, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some((index, item)) = base.catalogue.product_classes.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::IntroduceProductClass(introduce_product_class::mutation::IntroduceProductClass {
        product_class: item.clone(),
        index: Some(index),
    })]
}
