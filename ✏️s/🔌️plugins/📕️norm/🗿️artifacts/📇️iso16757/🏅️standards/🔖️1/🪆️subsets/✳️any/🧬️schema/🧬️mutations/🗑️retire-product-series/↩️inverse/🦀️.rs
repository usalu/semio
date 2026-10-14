//! Inverse for `retire-product-series`.

use crate::introduce_product_series;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::retire_product_series::mutation::RetireProductSeries;

pub fn inverse<Op>(payload: &RetireProductSeries, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_product_series::mutation::IntroduceProductSeries> {
    Ok((|| {
    let Some((index, item)) = base.catalogue.product_series.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_product_series::mutation::IntroduceProductSeries {
        product_series: item.clone(),
        index: Some(index),
    })]

    })())
}
