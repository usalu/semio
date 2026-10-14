//! Inverse for `change-slab-thickness-m`.
use crate::change_slab_thickness_m::ChangeSlabThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabThicknessM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_thickness_m::ChangeSlabThicknessM> {
    Ok((|| {
    let Some(slab) = base.slabs.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeSlabThicknessM { index: payload.index, new_concrete_thickness_m: slab.concrete_thickness_m })]

    })())
}
