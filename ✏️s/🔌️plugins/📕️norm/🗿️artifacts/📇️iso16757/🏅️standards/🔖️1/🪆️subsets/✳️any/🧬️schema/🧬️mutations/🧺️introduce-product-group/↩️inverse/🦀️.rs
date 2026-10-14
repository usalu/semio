//! ↩️ `introduce-product-group` — undo is `retire-product-group`, unless `base` already had this id
//! (then `create` was a no-op and there's nothing to undo).

use crate::retire_product_group;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::introduce_product_group::mutation::IntroduceProductGroup;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &IntroduceProductGroup, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_product_group::mutation::RetireProductGroup> {
    Ok((|| {
    if base.catalogue.product_groups.iter().any(|group| group.id == payload.product_group.id) {
        return Vec::new();
    }
    vec![Op::from(retire_product_group::mutation::RetireProductGroup { id: payload.product_group.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
