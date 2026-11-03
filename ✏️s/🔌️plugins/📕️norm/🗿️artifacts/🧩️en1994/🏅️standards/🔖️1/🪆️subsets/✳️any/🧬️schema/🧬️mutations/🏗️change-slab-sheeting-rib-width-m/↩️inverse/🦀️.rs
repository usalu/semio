//! ↩️ Inverse for `change-slab-sheeting-rib-width-m`.
use crate::change_slab_sheeting_rib_width_m::ChangeSlabSheetingRibWidthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabSheetingRibWidthM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_sheeting_rib_width_m::ChangeSlabSheetingRibWidthM> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabSheetingRibWidthM { id: payload.id.clone(), new_rib_width_m: slab.sheeting.rib_width_m })])
}
