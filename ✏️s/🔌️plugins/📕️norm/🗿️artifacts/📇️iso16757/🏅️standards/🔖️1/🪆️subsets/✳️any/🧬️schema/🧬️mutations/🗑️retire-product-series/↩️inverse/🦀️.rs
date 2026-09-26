//! Inverse for `retire-product-series`.

use crate::mutations::introduce_product_series;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::RetireProductSeries;

pub fn inverse(payload: &RetireProductSeries, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some((index, item)) = base.catalogue.product_series.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::IntroduceProductSeries(introduce_product_series::mutation::IntroduceProductSeries {
        product_series: item.clone(),
        index: Some(index),
    })]
}
