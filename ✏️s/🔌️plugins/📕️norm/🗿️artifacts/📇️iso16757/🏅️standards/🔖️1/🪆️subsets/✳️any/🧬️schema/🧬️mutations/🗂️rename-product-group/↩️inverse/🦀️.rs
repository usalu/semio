//! ↩️ `rename-product-group` — undo restores BASE's preferred name; missing id ⇒ `Vec::new()`.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

use crate::rename_product_group::mutation::RenameProductGroup;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RenameProductGroup, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_product_group::mutation::RenameProductGroup> {
    Ok((|| {
    let Some(group) = base.catalogue.product_groups.iter().find(|group| group.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(RenameProductGroup { id: payload.id.clone(), new_name: group.names.preferred.text.clone() })]

    })())
}
//#endregion 🔖️Inverse
