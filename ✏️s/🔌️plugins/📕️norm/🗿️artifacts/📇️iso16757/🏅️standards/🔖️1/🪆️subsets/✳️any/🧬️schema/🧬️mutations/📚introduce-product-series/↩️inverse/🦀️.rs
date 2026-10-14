//! Inverse for `introduce-product-series`.

use crate::retire_product_series;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::introduce_product_series::mutation::IntroduceProductSeries;

pub fn inverse<Op>(payload: &IntroduceProductSeries, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_product_series::mutation::RetireProductSeries> {
    Ok((|| {
    if base.catalogue.product_series.iter().any(|item| item.id == payload.product_series.id) {
        return Vec::new();
    }
    vec![Op::from(retire_product_series::mutation::RetireProductSeries { id: payload.product_series.id.clone() })]

    })())
}
