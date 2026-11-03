//! ↩️ Inverse for `change-slab-span-m`.
use crate::change_slab_span_m::ChangeSlabSpanM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabSpanM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_span_m::ChangeSlabSpanM> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabSpanM { id: payload.id.clone(), new_span_m: slab.span_m })])
}
