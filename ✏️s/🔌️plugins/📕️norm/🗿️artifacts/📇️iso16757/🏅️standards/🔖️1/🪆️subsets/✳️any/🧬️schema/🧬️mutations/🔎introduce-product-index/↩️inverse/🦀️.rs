//! Inverse for `introduce-product-index`.

use crate::retire_product_index;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::introduce_product_index::mutation::IntroduceProductIndex;

pub fn inverse<Op>(payload: &IntroduceProductIndex, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_product_index::mutation::RetireProductIndex> {
    Ok((|| {
    if base.catalogue.product_indexes.iter().any(|item| item.id == payload.product_index.id) {
        return Vec::new();
    }
    vec![Op::from(retire_product_index::mutation::RetireProductIndex { id: payload.product_index.id.clone() })]

    })())
}
