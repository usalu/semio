//! ↩️ Inverse for `change-slab-k-factor`.
use crate::change_slab_k_factor::ChangeSlabKFactor;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabKFactor, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_k_factor::ChangeSlabKFactor> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabKFactor { id: payload.id.clone(), new_k_factor: slab.k_factor })])
}
