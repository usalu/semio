//! ↩️ Inverse for `change-slab-m-factor`.
use crate::change_slab_m_factor::ChangeSlabMFactor;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabMFactor, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_m_factor::ChangeSlabMFactor> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabMFactor { id: payload.id.clone(), new_m_factor: slab.m_factor })])
}
