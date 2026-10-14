//! ↩️ `retire-product-group` — undo re-`create`s the group from BASE state, at its original
//! index; missing id ⇒ `Vec::new()`.

use crate::introduce_product_group;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::retire_product_group::mutation::RetireProductGroup;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RetireProductGroup, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_product_group::mutation::IntroduceProductGroup> {
    Ok((|| {
    let Some(position) = base.catalogue.product_groups.iter().position(|group| group.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_product_group::mutation::IntroduceProductGroup { product_group: base.catalogue.product_groups[position].clone(), index: Some(position) })]

    })())
}
//#endregion 🔖️Inverse
