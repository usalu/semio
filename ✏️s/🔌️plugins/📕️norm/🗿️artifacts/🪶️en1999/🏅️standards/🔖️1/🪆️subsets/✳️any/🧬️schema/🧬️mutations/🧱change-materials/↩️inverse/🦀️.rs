//! ↩️ `change-materials` inverse.

use crate::change_materials::ChangeMaterials;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeMaterials, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_materials::ChangeMaterials> {
    Ok((|| {
    vec![Op::from(ChangeMaterials { materials: base.materials.clone() })]

    })())
}
