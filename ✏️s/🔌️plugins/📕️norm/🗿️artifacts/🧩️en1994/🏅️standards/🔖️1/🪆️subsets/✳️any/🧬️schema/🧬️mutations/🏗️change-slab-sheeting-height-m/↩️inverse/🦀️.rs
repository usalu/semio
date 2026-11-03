//! ↩️ Inverse for `change-slab-sheeting-height-m`.
use crate::change_slab_sheeting_height_m::ChangeSlabSheetingHeightM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabSheetingHeightM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_sheeting_height_m::ChangeSlabSheetingHeightM> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabSheetingHeightM { id: payload.id.clone(), new_height_m: slab.sheeting.height_m })])
}
