//! ↪️ Inverse for `introduce-product-class`.

use crate::retire_product_class;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::introduce_product_class::mutation::IntroduceProductClass;

pub fn inverse<Op>(payload: &IntroduceProductClass, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_product_class::mutation::RetireProductClass> {
    Ok((|| {
    if base.catalogue.product_classes.iter().any(|item| item.id == payload.product_class.id) {
        return Vec::new();
    }
    vec![Op::from(retire_product_class::mutation::RetireProductClass { id: payload.product_class.id.clone() })]

    })())
}
