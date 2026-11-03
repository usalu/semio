//! ↩️ `insert-material` inverse.

use crate::insert_material::InsertMaterial;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_material;

pub fn inverse<Op>(payload: &InsertMaterial, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_material::RemoveMaterial> {
    Ok((|| {
    vec![Op::from(remove_material::RemoveMaterial { id: payload.material.id.clone() })]

    })())
}
