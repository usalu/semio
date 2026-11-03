//! ↩️ Inverse for `change-slab-sheeting-thickness-m`.
use crate::change_slab_sheeting_thickness_m::ChangeSlabSheetingThicknessM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabSheetingThicknessM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_sheeting_thickness_m::ChangeSlabSheetingThicknessM> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabSheetingThicknessM { id: payload.id.clone(), new_thickness_m: slab.sheeting.thickness_m })])
}
