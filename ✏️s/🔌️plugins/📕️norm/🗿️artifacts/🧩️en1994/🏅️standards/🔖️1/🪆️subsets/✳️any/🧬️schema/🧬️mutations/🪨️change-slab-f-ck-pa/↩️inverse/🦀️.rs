//! ↩️ Inverse for `change-slab-f-ck-pa`.
use crate::change_slab_f_ck_pa::ChangeSlabFCkPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabFCkPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_f_ck_pa::ChangeSlabFCkPa> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabFCkPa { id: payload.id.clone(), new_f_ck_pa: slab.f_ck_pa })])
}
