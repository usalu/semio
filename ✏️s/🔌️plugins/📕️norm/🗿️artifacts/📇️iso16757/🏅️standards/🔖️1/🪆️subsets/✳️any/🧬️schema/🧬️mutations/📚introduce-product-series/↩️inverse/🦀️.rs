//! Inverse for `introduce-product-series`.

use crate::mutations::retire_product_series;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::IntroduceProductSeries;

pub fn inverse(payload: &IntroduceProductSeries, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.product_series.iter().any(|item| item.id == payload.product_series.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetireProductSeries(retire_product_series::mutation::RetireProductSeries { id: payload.product_series.id.clone() })]
}
