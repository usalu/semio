//! Inverse for `retire-product-index`.

use crate::introduce_product_index;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::retire_product_index::mutation::RetireProductIndex;

pub fn inverse<Op>(payload: &RetireProductIndex, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_product_index::mutation::IntroduceProductIndex> {
    Ok((|| {
    let Some((index, item)) = base.catalogue.product_indexes.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_product_index::mutation::IntroduceProductIndex {
        product_index: item.clone(),
        index: Some(index),
    })]

    })())
}
