//! Inverse for `retire-product-class`.

use crate::introduce_product_class;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::retire_product_class::mutation::RetireProductClass;

pub fn inverse<Op>(payload: &RetireProductClass, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_product_class::mutation::IntroduceProductClass> {
    Ok((|| {
    let Some((index, item)) = base.catalogue.product_classes.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_product_class::mutation::IntroduceProductClass {
        product_class: item.clone(),
        index: Some(index),
    })]

    })())
}
