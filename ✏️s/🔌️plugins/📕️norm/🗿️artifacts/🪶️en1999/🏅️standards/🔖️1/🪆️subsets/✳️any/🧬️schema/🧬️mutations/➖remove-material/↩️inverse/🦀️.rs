//! ↩️ `remove-material` inverse.

use crate::remove_material::RemoveMaterial;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_material;

pub fn inverse<Op>(payload: &RemoveMaterial, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_material::InsertMaterial> {
    Ok((|| {
    if let Some((index, row)) = base.materials.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_material::InsertMaterial { index: index as u32, material: row.clone() })]
    } else { Vec::new() }

    })())
}
